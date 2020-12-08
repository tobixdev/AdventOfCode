use super::program::Program;
use super::program::Interpreter;

pub fn run() {
    let program = Program::read();
    let mut interpreter = Interpreter::new(&program);
    println!("Accumulator: {}", interpreter.execute().unwrap_err())
}