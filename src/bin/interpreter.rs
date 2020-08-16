use std::str::FromStr;

use brainfuckrs::lang::InstructionSet;
use brainfuckrs::interpreter::BrainfuckInterpreter;

fn main() {
    // Should print 4
    let instruction_set = InstructionSet::from_str("++[>++<-]>.").unwrap();

    let mut interpreter = BrainfuckInterpreter::new(instruction_set, 10);
    interpreter.interpret();
}