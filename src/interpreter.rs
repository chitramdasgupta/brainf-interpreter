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
    JumpForward,
    JumpBackward,
}

#[derive(Clone, Hash)]
pub struct Machine {
    instr_tape: Vec<Instr>,
    data: [u8; TAPE_LEN],
    pointer: usize,
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            instr_tape: Vec::new(),
            data: [0; TAPE_LEN],
            pointer: 0,
        }
    }
}

impl fmt::Debug for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Machine")
            .field("instr_tape", &self.instr_tape)
            .field("data", &self.data)
            .field("pointer", &self.pointer)
            .finish()
    }
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Machine (instr_tape: {:?}, data: {:?}, pointer: {})",
            self.instr_tape, self.data, self.pointer
        )
    }
}

impl Machine {
    pub fn parse_instructions(&mut self, contents: String) {
        contents.chars().for_each(|c| {
            match c {
                '+' => self.instr_tape.push(Instr::IncrDataByte),
                '-' => self.instr_tape.push(Instr::DecrDataByte),
                '>' => self.instr_tape.push(Instr::IncrDataPointer),
                '<' => self.instr_tape.push(Instr::DecrDataPointer),
                '.' => self.instr_tape.push(Instr::Print),
                ',' => self.instr_tape.push(Instr::Input),
                '[' => self.instr_tape.push(Instr::JumpForward),
                ']' => self.instr_tape.push(Instr::JumpBackward),
                _ => {}
            };
        });
    }

    pub fn execute_instructions(&mut self) {
        let mut i = 0;
        while i < self.instr_tape.len() {
            match self.instr_tape[i] {
                Instr::IncrDataByte => {
                    self.data[self.pointer] = self.data[self.pointer].wrapping_add(1)
                }

                Instr::DecrDataByte => {
                    self.data[self.pointer] = self.data[self.pointer].wrapping_sub(1)
                }

                Instr::IncrDataPointer => self.pointer = (self.pointer + 1).rem_euclid(TAPE_LEN),

                Instr::DecrDataPointer => self.pointer = (self.pointer - 1).rem_euclid(TAPE_LEN),

                Instr::Print => print!("{}", self.data[self.pointer] as char),

                Instr::Input => {
                    self.data[self.pointer] = std::io::stdin()
                        .bytes()
                        .next()
                        .and_then(|result| result.ok())
                        .expect("Could not read input");
                }

                Instr::JumpForward => {
                    if self.data[self.pointer] == 0 {
                        let mut open_braces = 1;
                        while open_braces > 0 {
                            i += 1;
                            if self.instr_tape[i] == Instr::JumpForward {
                                open_braces += 1;
                            } else if self.instr_tape[i] == Instr::JumpBackward {
                                open_braces -= 1;
                            }
                        }
                    }
                }

                Instr::JumpBackward => {
                    if self.data[self.pointer] != 0 {
                        let mut open_braces = 1;
                        while open_braces > 0 {
                            i -= 1;
                            if self.instr_tape[i] == Instr::JumpBackward {
                                open_braces += 1;
                            } else if self.instr_tape[i] == Instr::JumpForward {
                                open_braces -= 1;
                            }
                        }
                    }
                }
            }
            i += 1;
        }
    }
}
