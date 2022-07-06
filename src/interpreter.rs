use std::{fmt, io::Read};


const TAPE_LEN: usize = 30_000;


#[derive(Clone, Hash, Debug, PartialEq)]
enum Instr {
    IncrDataByte,
    DecrDataByte,
    IncrDataPointer,
    DecrDataPointer,
    Print,
    Input,
    JumpForward(usize),
    JumpBackward(usize),
}


#[derive(Debug)]
pub enum Error {
    UnmatchedClosedBracket(usize),
    UnmatchedOpenBracket(usize),
}


#[derive(Clone)]
pub struct Machine {
    instruction_tape: Vec<Instr>,
    data_tape: Vec<u8>,
    data_pointer: usize,
}


impl Default for Machine {
    fn default() -> Self {
        Self {
            instruction_tape: Default::default(),
            data_tape: vec![0; TAPE_LEN],
            data_pointer: Default::default(),
        }
    }
}


impl fmt::Debug for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Machine")
         .field("instr_tape", &self.instruction_tape)
         .field("data", &self.data_tape)
         .field("pointer", &self.data_pointer)
         .finish()
    }
}


impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Machine (instr_tape: {:?}, data: {:?}, pointer: {})",
            self.instruction_tape, self.data_tape, self.data_pointer
        )
    }
}


impl Machine {
    pub fn from(tape_length: usize) -> Self {
        Self {
            instruction_tape: vec![],
            data_tape: vec![0; tape_length],
            data_pointer: 0,
        }
    }


    pub fn new() -> Self {
        Self::from(TAPE_LEN)
    }

    pub fn parse_instructions(&mut self, contents: String) -> Result<(), Error> {
        self.instruction_tape = contents
            .chars()
            .filter_map(|c| match c {
                '+' => Some(Instr::IncrDataByte),
                '-' => Some(Instr::DecrDataByte),
                '>' => Some(Instr::IncrDataPointer),
                '<' => Some(Instr::DecrDataPointer),
                '.' => Some(Instr::Print),
                ',' => Some(Instr::Input),
                '[' => Some(Instr::JumpForward(0)),
                ']' => Some(Instr::JumpBackward(0)),
                _ => None
            })
            .collect();
        self.check_brackets()?;
        self.data_tape = vec![0; self.data_tape.len()];
        self.data_pointer = 0;
        Ok(())
    }

    fn check_brackets(&mut self) -> Result<(), Error> {
        let mut stack = vec![];
        for (i, c) in self.instruction_tape.clone().iter().enumerate() {
            match c {
                Instr::JumpForward(0) => {
                    stack.push(i);
                }
                Instr::JumpBackward(0) => {
                    if stack.is_empty() {
                        return Err(Error::UnmatchedClosedBracket(i));
                    }
                    let start = stack.pop().unwrap();
                    self.instruction_tape[i] = Instr::JumpBackward(start);
                    self.instruction_tape[start] = Instr::JumpForward(i);
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
                Instr::IncrDataByte => self.data_tape[self.data_pointer] = self.data_tape[self.data_pointer].wrapping_add(1),
                Instr::DecrDataByte => self.data_tape[self.data_pointer] = self.data_tape[self.data_pointer].wrapping_sub(1),

                Instr::IncrDataPointer => self.data_pointer = self.data_pointer.wrapping_add(1),
                Instr::DecrDataPointer => self.data_pointer = self.data_pointer.wrapping_sub(1),

                Instr::Print => print!("{}", self.data_tape[self.data_pointer] as char),
                Instr::Input => self.data_tape[self.data_pointer] = std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .unwrap(),

                Instr::JumpForward(jump) => if self.data_tape[self.data_pointer] == 0 {
                    i = jump;
                },
                Instr::JumpBackward(jump) => if self.data_tape[self.data_pointer] != 0 {
                    i = jump;
                },
            }
            i += 1;
        }
    }
}
