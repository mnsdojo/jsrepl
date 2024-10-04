use rustyline::{error::ReadlineError, DefaultEditor};

use crate::repl::color::Colorize;

pub fn start_repl() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "jsRepl".green().bold());
    println!("{}", "Type 'exit' to quit".green().italic());

    //init the line editor
    let mut editor = DefaultEditor::new()?;

    if editor.load_history("history.txt").is_err() {
        println!("{}", "No previous history".yellow());
    }


    loop {
        let readline = editor.readline(">> ");
        match readline {
            Ok(line) => {
                let input = line.trim();
                if input.eq_ignore_ascii_case("exit") {
                    println!("Exiting...");
                    break;
                }

                editor.add_history_entry(input)?;
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
