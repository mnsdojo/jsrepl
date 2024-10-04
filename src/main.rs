mod repl;

fn main() {
    match repl::cli::start_repl() {
        Ok(_) => {}
        Err(e) => eprintln!("Error starting repl : {:?}", e),
    }
}
