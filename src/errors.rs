
#[derive(Debug, Clone)]
pub struct InstructionErrorPosition {
    pub pos: usize,
    pub character: char
}

#[derive(Debug, Clone)]
pub enum InstructionSetParseError {
    UnknownInstruction(InstructionErrorPosition)
}

#[derive(Debug, Clone, PartialEq)]
pub enum InterpreterError {
    IncorrectBehavior(String),
    MissingGoBackInstruction,
}