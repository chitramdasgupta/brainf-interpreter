# Brainf Interpreter

An interpreter for the esoteric programming language [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) written in rust.

## Compilation

**NOTE:** To be able to compile the project, you will need to have
rust installed. Check how to do it [here](https://www.rust-lang.org/tools/install).

```
git clone https://github.com/chitramdasgupta/brainf-interpreter.git
cd brainf-interpreter
cargo build --release
```

## Running

The repository includes a number of code samples written in Brainfuck in `examples/`

To run one of them:

```
./target/release/brainf_interpreter ./examples/hello.bf
```

This will print *Hello World!*
