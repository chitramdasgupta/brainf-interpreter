use std::{env, fs};
mod interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    let mut machine = interpreter::Machine::new();
    machine.parse_instructions(contents);
    machine.execute_instructions();
}
