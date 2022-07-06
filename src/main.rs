use std::{env, fs};

mod interpreter;

fn main() -> Result<(), interpreter::Error> {
    let args: Vec<String> = env::args().collect();

    let file_name = &args[1];

    let mut cell_count: usize = 30_000;
    if args.len() == 3 {
        cell_count = (&args[2]).parse::<usize>().unwrap_or(30_000);
    }

    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    let mut machine = interpreter::Machine::new(cell_count);

    machine.parse_instructions(contents)?;

    machine.execute_instructions()?;

    Ok(())
}
