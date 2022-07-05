use std::collections::HashMap;
use std::io::Read;


const TAPE_LENGTH: usize = 30_000;


#[derive(Debug)]
pub enum Error {
    UnmatchedClosedBracket(usize),
    UnmatchedOpenBracket(usize),
}


pub struct Machine {
    instruction_tape: Vec<char>,
    data_tape: Vec<u8>,
    data_pointer: usize,
    brackets: HashMap<usize, usize>,
}


impl Machine {
    pub fn from(tape_length: usize) -> Self {
        Self {
            instruction_tape: vec![],
            data_tape: vec![0; tape_length],
            data_pointer: 0,
            brackets: HashMap::new(),
        }
    }


    pub fn new() -> Self {
        Self::from(TAPE_LENGTH)
    }

    pub fn parse_instructions(&mut self, contents: String) -> Result<(), Error> {
        self.instruction_tape = contents
            .chars()
            .filter(|c| matches!(*c, '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']'))
            .collect();
        self.brackets.clear();
        self.check_brackets()?;
        self.data_tape = vec![0; self.data_tape.len()];
        self.data_pointer = 0;
        Ok(())
    }

    fn check_brackets(&mut self) -> Result<(), Error> {
        let mut stack = vec![];
        for (i, c) in self.instruction_tape.iter().enumerate() {
            match c {
                '[' => {
                    stack.push(i);
                }
                ']' => {
                    if stack.is_empty() {
                        return Err(Error::UnmatchedClosedBracket(i));
                    }
                    let start = stack.pop().unwrap();
                    self.brackets.insert(start, i);
                    self.brackets.insert(i, start);
                }
                _ => {}
            }
        }
        if stack.is_empty() {
            Ok(())
        } else {
            Err(Error::UnmatchedOpenBracket(stack.pop().unwrap()))
        }
    }

    pub fn execute_instructions(&mut self) {
        let mut i = 0;
        while i < self.instruction_tape.len() {
            match self.instruction_tape[i] {
                '+' => self.data_tape[self.data_pointer] = self.data_tape[self.data_pointer].wrapping_add(1),
                '-' => self.data_tape[self.data_pointer] = self.data_tape[self.data_pointer].wrapping_sub(1),

                '>' => self.data_pointer = self.data_pointer.wrapping_add(1),
                '<' => self.data_pointer = self.data_pointer.wrapping_sub(1),

                '.' => print!("{}", self.data_tape[self.data_pointer] as char),
                ',' => self.data_tape[self.data_pointer] = std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .unwrap(),

                '[' => if self.data_tape[self.data_pointer] == 0 {
                    i = self.brackets[&i];
                },
                ']' => if self.data_tape[self.data_pointer] != 0 {
                    i = self.brackets[&i];
                },

                _ => {}
            }
            i += 1;
        }
    }
}
