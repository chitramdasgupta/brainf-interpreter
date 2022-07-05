use std::{fmt, io::Read};

const TAPE_LENGTH: usize = 30_000;

#[derive(Clone, Hash, Debug, PartialEq)]
enum Instr {
    IncrDataByte,
    DecrDataByte,
    IncrDataPointer,
    DecrDataPointer,
    Print,
    Input,
    JumpForward,
    JumpBackward,
}

#[derive(Clone, Hash)]
pub struct Machine {
    instruction_tape: Vec<Instr>,
    data_tape: [u8; TAPE_LENGTH],
    data_pointer: usize,
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            instruction_tape: Vec::new(),
            data_tape: [0; TAPE_LENGTH],
            data_pointer: 0,
        }
    }
}

impl fmt::Debug for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Machine")
            .field("instruction_tape", &self.instruction_tape)
            .field("data_tape", &self.data_tape)
            .field("data_pointer", &self.data_pointer)
            .finish()
    }
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Machine (instruction_tape: {:?}, data_tape: {:?}, data_ponter: {})",
            self.instruction_tape, self.data_tape, self.data_pointer
        )
    }
}

impl Machine {
    pub fn parse_instructions(&mut self, contents: String) {
        contents.chars().for_each(|c| {
            match c {
                '+' => self.instruction_tape.push(Instr::IncrDataByte),
                '-' => self.instruction_tape.push(Instr::DecrDataByte),
                '>' => self.instruction_tape.push(Instr::IncrDataPointer),
                '<' => self.instruction_tape.push(Instr::DecrDataPointer),
                '.' => self.instruction_tape.push(Instr::Print),
                ',' => self.instruction_tape.push(Instr::Input),
                '[' => self.instruction_tape.push(Instr::JumpForward),
                ']' => self.instruction_tape.push(Instr::JumpBackward),
                _ => {}
            };
        });
    }

    pub fn execute_instructions(&mut self) {
        let mut i = 0;
        while i < self.instruction_tape.len() {
            match self.instruction_tape[i] {
                Instr::IncrDataByte => {
                    self.data_tape[self.data_pointer] =
                        self.data_tape[self.data_pointer].wrapping_add(1)
                }

                Instr::DecrDataByte => {
                    self.data_tape[self.data_pointer] =
                        self.data_tape[self.data_pointer].wrapping_sub(1)
                }

                Instr::IncrDataPointer => {
                    self.data_pointer = (self.data_pointer + 1).rem_euclid(TAPE_LENGTH)
                }

                Instr::DecrDataPointer => {
                    self.data_pointer = (self.data_pointer - 1).rem_euclid(TAPE_LENGTH)
                }

                Instr::Print => print!("{}", self.data_tape[self.data_pointer] as char),

                Instr::Input => {
                    self.data_tape[self.data_pointer] = std::io::stdin()
                        .bytes()
                        .next()
                        .and_then(|result| result.ok())
                        .expect("Could not read input");
                }

                Instr::JumpForward => {
                    if self.data_tape[self.data_pointer] == 0 {
                        let mut nested_bracket = 0;
                        i += 1;
                        while nested_bracket > 0 || self.instruction_tape[i] != Instr::JumpBackward
                        {
                            if self.instruction_tape[i] == Instr::JumpForward {
                                nested_bracket += 1;
                            } else if self.instruction_tape[i] == Instr::JumpBackward {
                                nested_bracket -= 1;
                            }
                            i += 1;
                        }
                    }
                }

                Instr::JumpBackward => {
                    if self.data_tape[self.data_pointer] != 0 {
                        let mut nested_bracket = 0;
                        i -= 1;
                        while nested_bracket > 0 || self.instruction_tape[i] != Instr::JumpForward {
                            if self.instruction_tape[i] == Instr::JumpBackward {
                                nested_bracket += 1;
                            } else if self.instruction_tape[i] == Instr::JumpForward {
                                nested_bracket -= 1;
                            }
                            i -= 1;
                        }
                    }
                }
            }
            i += 1;
        }
    }
}
