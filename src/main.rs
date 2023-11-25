use clap::Parser;
use std::path::PathBuf;

mod scanner;
mod tokens;
mod lox;
mod error;

use scanner::Scanner;
use lox::Lox;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    path: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let mut lox = Lox::new();

    match args.path {
        Some(path) => lox.start_from_path(path),
        None => lox.start_interactive()
    }
}

// Add error handling urgent!