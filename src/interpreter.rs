use std::io::Read;

const TAPE_LENGTH: usize = 30_000;

pub struct Machine {
    instruction_tape: Vec<char>,
    data_tape: [u8; TAPE_LENGTH],
    data_pointer: usize,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            instruction_tape: Vec::new(),
            data_tape: [0; TAPE_LENGTH],
            data_pointer: 0,
        }
    }

    pub fn parse_instructions(&mut self, contents: String) {
        self.instruction_tape = contents
            .chars()
            .filter(|c| matches!(*c, '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']'))
            .collect();
    }

    pub fn execute_instructions(&mut self) {
        let mut i = 0;
        while i < self.instruction_tape.len() {
            match self.instruction_tape[i] {
                '+' => {
                    self.data_tape[self.data_pointer] = if self.data_tape[self.data_pointer] == 255
                    {
                        0
                    } else {
                        self.data_tape[self.data_pointer] + 1
                    }
                }

                '-' => {
                    self.data_tape[self.data_pointer] = if self.data_tape[self.data_pointer] == 0 {
                        255
                    } else {
                        self.data_tape[self.data_pointer] - 1
                    }
                }

                '>' => {
                    self.data_pointer += 1;
                    self.data_pointer = if self.data_pointer == TAPE_LENGTH {
                        0
                    } else {
                        self.data_pointer
                    }
                }

                '<' => {
                    self.data_pointer = if self.data_pointer == 0 {
                        TAPE_LENGTH - 1
                    } else {
                        self.data_pointer - 1
                    }
                }

                '.' => print!("{}", self.data_tape[self.data_pointer] as char),

                ',' => {
                    self.data_tape[self.data_pointer] = std::io::stdin()
                        .bytes()
                        .next()
                        .and_then(|result| result.ok())
                        .unwrap();
                }

                '[' => {
                    if self.data_tape[self.data_pointer] == 0 {
                        let mut nested_bracket = 0;
                        i += 1;
                        while nested_bracket > 0 || self.instruction_tape[i] != ']' {
                            if self.instruction_tape[i] == '[' {
                                nested_bracket += 1;
                            } else if self.instruction_tape[i] == ']' {
                                nested_bracket -= 1;
                            }
                            i += 1;
                        }
                    }
                }

                ']' => {
                    if self.data_tape[self.data_pointer] != 0 {
                        let mut nested_bracket = 0;
                        i -= 1;
                        while nested_bracket > 0 || self.instruction_tape[i] != '[' {
                            if self.instruction_tape[i] == ']' {
                                nested_bracket += 1;
                            } else if self.instruction_tape[i] == '[' {
                                nested_bracket -= 1;
                            }
                            i -= 1;
                        }
                    }
                }

                _ => {}
            }
            i += 1;
        }
    }
}
