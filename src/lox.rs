use super::Scanner;
use std::path::PathBuf;
use std::io::{BufReader, BufRead, self};

pub struct Lox {
  had_error: bool
}

impl Lox {
  pub fn new() -> Lox {
      Lox {
          had_error: false
      }
  }

  pub fn start_from_path(&self, path: PathBuf) {
      let contents = std::fs::read_to_string(path).unwrap();
      println!("{}", contents);
  }

  pub fn start_interactive(&mut self) {
      let mut reader = BufReader::new(io::stdin());
      let mut line = String::new();

      loop {
          line.clear();

          print!("> ");
          let _ = reader.read_line(&mut line);
          println!("{}", line);

          self.run(&line);

          if self.had_error {
              self.had_error = false;
          }
      }
  }

  fn run(&self, source: &str) {
      let scanner = Scanner::new(source);
      let tokens = scanner.scan_tokens();
  
      for token in tokens {
          println!("{:?}", token);
      }
  }

//   fn error(&self, line: u32, message: String) {
//       self.report(line, "", &message);
//   }
  
//   fn report(&self, line: u32, location: &str, message: &str) {
//       println!("[line {}] Error {}: {}", line, location, message);
//       std::process::exit(65);
//   }
}