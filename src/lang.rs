use crate::errors::{
    InstructionSetParseError, 
    InstructionErrorPosition
};
use std::str::FromStr;


#[derive(Debug, Clone)]
pub enum Instruction {
    IncrementPointer,
    DecrementPointer,
    IncrementByte,
    DecrementByte,
    Print,
    Read,
    GoTo,
    GoBack
}

#[derive(Debug, Clone)]
pub struct InstructionSet {
    pub inner: Vec<Instruction>
}

impl InstructionSet {
    pub fn get(&self, index: usize) -> Option<&Instruction> {
        if index < self.inner.len() {
            return Some(&self.inner[index])
        }
        None
    }

    pub fn size(&self) -> usize {
        self.inner.len()
    }
}

pub type MaybeInstruction = Result<Instruction, String>;

fn match_instruction(c: char) -> MaybeInstruction {

    match c {
        '>' => Ok(Instruction::IncrementPointer),
        '<' => Ok(Instruction::DecrementPointer),
        '+' => Ok(Instruction::IncrementByte),
        '-' => Ok(Instruction::DecrementByte),
        '[' => Ok(Instruction::GoTo),
        ']' => Ok(Instruction::GoBack),
        '.' => Ok(Instruction::Print),
        ',' => Ok(Instruction::Read),
        _ => Err("Louche".to_string())
    }
}

impl FromStr for InstructionSet {

    type Err = InstructionSetParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grammar = s.to_string();
        let mut inner = Vec::new();
        for (i, v) in grammar.chars().enumerate() {
            match match_instruction(v) {
                Ok(instruction) => inner.push(instruction),
                Err(_) => return Err(InstructionSetParseError::UnknownInstruction(InstructionErrorPosition{
                    pos: i,
                    character: v
                }))
            }
        }
        Ok(InstructionSet{
            inner: inner
        })
    }
}