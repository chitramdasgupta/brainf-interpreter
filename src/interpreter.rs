use std::{fmt, io::Read};

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
    TapeOverflowed,
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
                '+' => Some(Instr::IncrDataByte),
                '-' => Some(Instr::DecrDataByte),
                '>' => Some(Instr::IncrDataPointer),
                '<' => Some(Instr::DecrDataPointer),
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

    pub fn execute_instructions(&mut self) -> Result<(), Error> {
        let mut i = 0;
        while i < self.instruction_tape.len() {
            if self.data_tape.get(self.data_pointer).is_none() {
                eprintln!("\nEnd of tape reached.");
                eprintln!("Consider using a larger number of cells");
                eprintln!("eg: brainf_interpreter source.bf 60000");
                return Err(Error::TapeOverflowed);
            }
            match self.instruction_tape[i] {
                Instr::IncrDataByte => {
                    self.data_tape[self.data_pointer] =
                        self.data_tape[self.data_pointer].wrapping_add(1)
                }
                Instr::DecrDataByte => {
                    self.data_tape[self.data_pointer] =
                        self.data_tape[self.data_pointer].wrapping_sub(1)
                }

                Instr::IncrDataPointer => self.data_pointer = self.data_pointer.wrapping_add(1),
                Instr::DecrDataPointer => self.data_pointer = self.data_pointer.wrapping_sub(1),

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
        Ok(())
    }
}
