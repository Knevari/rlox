pub struct Error {}

impl Error {
  pub fn complain(line: usize, message: String) {
    Error::report(line, "", &message);
  }

  fn report(line: usize, location: &str, message: &str) {
      println!("[line {}] Error {}: {}", line, location, message);
      // std::process::exit(65);
  }
}