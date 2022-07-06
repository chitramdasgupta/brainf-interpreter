use std::{env, fs};

mod interpreter;

fn main() -> Result<(), interpreter::Error> {
    let args: Vec<String> = env::args().collect();

    let file_name = &args.get(1).unwrap();

    let cell_count = match &args.get(2) {
        Some(count) => count.parse::<usize>().unwrap(),
        None => 30_000,
    };

    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    let mut machine = interpreter::Machine::new(cell_count);

    machine.parse_instructions(contents)?;

    machine.execute_instructions()?;

    Ok(())
}
