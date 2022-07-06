use std::{fmt, io::Read};

#[derive(Clone, Hash, Debug, PartialEq, Copy)]
enum Instr {
    IncrDataByte(u8),
    DecrDataByte(u8),
    IncrDataPointer(usize),
    DecrDataPointer(usize),
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

#[derive(Clone, Default)]
pub struct Machine {
    instruction_tape: Vec<Instr>,
    data_tape: Vec<u8>,
    data_pointer: usize,
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
    pub fn new(tape_length: usize) -> Self {
        Self {
            instruction_tape: vec![],
            data_tape: vec![0; tape_length],
            data_pointer: 0,
        }
    }

    pub fn parse_instructions(&mut self, contents: String) -> Result<(), Error> {
        self.instruction_tape = contents
            .chars()
            .filter_map(|c| match c {
                '+' => Some(Instr::IncrDataByte(1)),
                '-' => Some(Instr::DecrDataByte(1)),
                '>' => Some(Instr::IncrDataPointer(1)),
                '<' => Some(Instr::DecrDataPointer(1)),
                '.' => Some(Instr::Print),
                ',' => Some(Instr::Input),
                '[' => Some(Instr::JumpForward(0)),
                ']' => Some(Instr::JumpBackward(0)),
                _ => None,
            })
            .collect();
        if let Err(e) = self.check_brackets() {
            self.instruction_tape.clear();
            return Err(e);
        };
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

    fn condense(&mut self) -> Vec<Instr> {
        let mut condensed = Vec::new();
        let mut counter = None;
        for instr in &self.instruction_tape {
            match instr {
                Instr::IncrDataByte(1) => {
                    if let Some(Instr::IncrDataByte(incr)) = counter {
                        counter = Some(Instr::IncrDataByte(incr + 1));
                    }
                }
                _ => {
                    if let Some(instr) = counter {
                        condensed.push(instr);
                        counter = None;
                    }
                }
            }
        }
        condensed
    }

    pub fn execute_instructions(&mut self) {
        let mut i = 0;
        while i < self.instruction_tape.len() {
            match self.instruction_tape[i] {
                Instr::IncrDataByte(incr) => {
                    self.data_tape[self.data_pointer] =
                        self.data_tape[self.data_pointer].wrapping_add(incr)
                }
                Instr::DecrDataByte(incr) => {
                    self.data_tape[self.data_pointer] =
                        self.data_tape[self.data_pointer].wrapping_sub(incr)
                }

                Instr::IncrDataPointer(incr) => {
                    self.data_pointer = self.data_pointer.wrapping_add(incr)
                }

                Instr::DecrDataPointer(incr) => {
                    self.data_pointer = self.data_pointer.wrapping_sub(incr)
                }

                Instr::Print => print!("{}", self.data_tape[self.data_pointer] as char),
                Instr::Input => {
                    self.data_tape[self.data_pointer] = std::io::stdin()
                        .bytes()
                        .next()
                        .and_then(|result| result.ok())
                        .unwrap()
                }

                Instr::JumpForward(jump) => {
                    if self.data_tape[self.data_pointer] == 0 {
                        i = jump;
                    }
                }
                Instr::JumpBackward(jump) => {
                    if self.data_tape[self.data_pointer] != 0 {
                        i = jump;
                    }
                }
            }
            i += 1;
        }
    }
}
