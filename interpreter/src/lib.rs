use scanner::scan;
use parser::Parser;
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};
use evaluator::Evaluator;
use log;

/// Structure that holds interpreter information.
struct Interpreter {}

/// REPL mode
pub fn repl() {
    // All the interpreter steps go here
    todo!()
}

/// Execute a file.
pub fn execute(path: PathBuf) {
    log::debug!("About to execute file {}", path.display());
    // TODO: Error handling
    let input = scanner::Input::File { path: path.clone() };
    let text = std::fs::read_to_string(path).expect("Failed to read filed");
    let tokens = scan(input, text);
    // All the interpreter steps go here
    let expressions = Parser::new(tokens).parse();
    let result = Evaluator::new(expressions).evaluate();
    println!("{}", result);
    log::trace!("Parsed expression: {:#?}", result);
}
