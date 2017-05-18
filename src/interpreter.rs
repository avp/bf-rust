use std::io;
use std::io::Write;
use std::collections::HashMap;
use std::io::Read;

const TAPE_LEN: usize = 30000;

#[derive(PartialEq, Eq, Hash, Debug)]
enum Bracket {
  Open(usize),
  Close(usize),
}

fn make_mapping(prog: &str) -> HashMap<Bracket, Bracket> {
  let mut stack: Vec<usize> = vec![];
  let mut result: HashMap<Bracket, Bracket> = HashMap::new();
  for (i, c) in prog.chars().enumerate() {
    match c {
      '[' => {
        stack.push(i);
      }
      ']' => {
        match stack.pop() {
          Some(open) => {
            result.insert(Bracket::Open(open), Bracket::Close(i));
            result.insert(Bracket::Close(i), Bracket::Open(open));
          }
          None => panic!("Mismatched ] at {}", i),
        }
      }
      _ => {}
    }
  }
  if let Some(i) = stack.pop() {
    panic!("Mismatched [ at {}", i);
  }
  result
}

pub fn interpret(prog: &str) {
  let chars: Vec<char> = prog.chars().collect();
  let prog_len = prog.len();

  let mut tape: [u8; TAPE_LEN] = [0; TAPE_LEN];
  let mut cursor: usize = 0;
  let mut ip: usize = 0;

  let matches = make_mapping(prog);

  while ip < prog_len {
    match chars[ip] {
      '>' => {
        if cursor == TAPE_LEN - 1 {
          panic!("Exceeded length of tape");
        }
        cursor += 1;
      }
      '<' => {
        if cursor == 0 {
          panic!("Attempted to move off the left side of the tape");
        }
        cursor -= 1;
      }
      '+' => {
        tape[cursor] = if tape[cursor] == 0xff {
          0
        } else {
          tape[cursor] + 1
        }
      }
      '-' => {
        tape[cursor] = if tape[cursor] == 0 {
          0xff
        } else {
          tape[cursor] - 1
        }
      }
      '.' => {
        print!("{}", tape[cursor] as char);
        io::stdout().flush().expect("Could not flush stdout");
      }
      ',' => {
        match io::stdin().bytes().next() {
          Some(Ok(c)) => tape[cursor] = c,
          _ => panic!("Failed to read input"),
        }
      }
      '[' => {
        if tape[cursor] == 0 {
          match matches[&Bracket::Open(ip)] {
            Bracket::Close(next) => ip = next,
            _ => panic!("Invalid bracket match"),
          }
        }
      }
      ']' => {
        if tape[cursor] != 0 {
          match matches[&Bracket::Close(ip)] {
            Bracket::Open(next) => ip = next,
            _ => panic!("Invalid bracket match"),
          }
        }
      }
      _ => {}
    };
    ip += 1;
  }
}
