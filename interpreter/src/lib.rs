use scanner::scan;
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use log;

/// Structure that holds interpreter information.
struct Interpreter {}

/// REPL mode
pub fn repl() {
    // All the interpreter steps go here
    todo!()
}

/// Execute a file.
pub fn execute(path: &Path) {
    log::debug!("About to execute file {}", path.display());
    // TODO: Error handling
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let tokens = scan(reader);
    // All the interpreter steps go here
}
