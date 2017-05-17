use std::env;
use std::fs::File;
use std::io::Read;

mod interpreter;

fn get_prog() -> String {
  let filename = match env::args().nth(1) {
    Some(n) => n,
    _ => panic!("Please provide a file to load from"),
  };
  let mut file = match File::open(filename) {
    Ok(f) => f,
    _ => panic!("That file doesn't exist"),
  };

  let mut prog = String::new();
  if let Err(e) = file.read_to_string(&mut prog) {
    println!("{}", e.to_string());
  }
  prog
}

fn main() {
  let prog = get_prog();
  interpreter::interpret(&prog);
}
