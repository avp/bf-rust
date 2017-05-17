use std::io;
use std::io::Write;
use std::collections::HashMap;

const TAPE_LEN: usize = 30000;

#[derive(PartialEq, Eq, Hash, Debug)]
enum Bracket {
  Open(usize),
  Close(usize),
}

fn make_mapping(prog: &String) -> HashMap<Bracket, Bracket> {
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
  match stack.pop() {
    Some(i) => {
      panic!("Mismatched [ at {}", i);
    }
    None => {}
  }
  return result;
}

pub fn interpret(prog: &String) {
  let chars: Vec<char> = prog.chars().collect();
  let prog_len = prog.len();

  let mut tape: [u8; TAPE_LEN] = [0; TAPE_LEN];
  let mut cursor: usize = 0;
  let mut ip: usize = 0;

  let matches = make_mapping(&prog);

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
        tape[cursor] = tape[cursor] + 1;
      }
      '-' => {
        tape[cursor] = tape[cursor] - 1;
      }
      '.' => {
        print!("{}", tape[cursor] as char);
        io::stdout().flush().ok().expect("Could not flush stdout");
      }
      ',' => {
        print!(", operator unimplemented");
      }
      '[' => {
        if tape[cursor] == 0 {
          match matches.get(&Bracket::Open(ip)).unwrap() {
            &Bracket::Close(next) => ip = next,
            _ => panic!("Invalid bracket match"),
          }
        }
      }
      ']' => {
        if tape[cursor] != 0 {
          match matches.get(&Bracket::Close(ip)).unwrap() {
            &Bracket::Open(next) => ip = next,
            _ => panic!("Invalid bracket match"),
          }
        }
      }
      _ => {}
    };
    ip += 1;
  }
}
