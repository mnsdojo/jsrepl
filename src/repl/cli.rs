use std::process::Command;

use deno_core::JsRuntime;
use rustyline::{error::ReadlineError, DefaultEditor};

use crate::repl::{color::Colorize, executor::execute_js, syntax};

pub fn start_repl() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "jsRepl".green().bold());
    println!("{}", "Type 'exit' to quit".green().italic());

    let mut editor = DefaultEditor::new()?;

    if editor.load_history("history.txt").is_err() {
        println!("{}", "No previous history".yellow());
    }

    let highlighter = syntax::Highlighter::new()?;
    let mut js_runtime = JsRuntime::new(deno_core::RuntimeOptions::default());

    loop {
        let readline = editor.readline(">> ");
        match readline {
            Ok(line) => {
                let input = line.trim();
                highlighter.highlight(input)?;
                if input.eq_ignore_ascii_case("clear") {
                    clear_screen();
                    continue;
                }
                if input.eq_ignore_ascii_case("exit") {
                    println!("Exiting...");
                    break;
                }

                editor.add_history_entry(input)?;

                // execute the js code
                match execute_js(&mut js_runtime, input) {
                    Ok(result) => println!("{}", result.green()),
                    Err(err) => eprintln!("Error: {}", err),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl-C pressed, exiting...");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Ctrl-D pressed, existing ...");
                break;
            }
            Err(err) => {
                eprintln!("Error : {:?}", err);
                break;
            }
        }
    }

    editor.save_history("history.txt")?;
    Ok(())
}

fn clear_screen() {
    let _ = Command::new("clear")
        .status()
        .or_else(|_| Command::new("cls").status());
}
