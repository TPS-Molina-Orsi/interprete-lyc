use std::path::PathBuf;

use clap::Parser;

// NOTA: Usamos clap solo para el parseo de la CLI.
#[derive(Parser)]
struct CliInput {
    /// File with
    file: Option<PathBuf>,
}

impl CliInput {
    fn handle(&self) {
        if let Some(path) = &self.file {
            // Read and execute file
            interpreter::execute(path);
        } else {
            // REPL mode
            todo!("Implementar REPL");
        }
    }
}

// Wanna run me?
// make build ; RUST_LOG=trace ./target/debug/cli tests/data/factorial.scm
fn main() {
    env_logger::init();

    let cli = CliInput::parse();
    cli.handle();
}
