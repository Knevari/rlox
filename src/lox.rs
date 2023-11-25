use crate::Scanner;
use std::path::PathBuf;
use std::io::{BufReader, BufRead, self};

pub struct Lox {
}

impl Lox {
  pub fn new() -> Lox {
      Lox {}
  }

  pub fn start_from_path(&self, path: PathBuf) {
    // Read entire program into memory
    let source = std::fs::read_to_string(path).unwrap();
    self.run(source);
  }

  pub fn start_interactive(&mut self) {
    // Batch before reading
    let mut reader = BufReader::new(io::stdin());
    let mut line = String::new();

    loop {
        line.clear();
        let _ = reader.read_line(&mut line);
        
        self.run(line.to_string());
    }
  }

  fn run(&self, source: String) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("{}", *token);
    }
  }
}

// PLEASE VALIDATE THE FUCKING ERRORS