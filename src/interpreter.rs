use crate::lang::{InstructionSet, Instruction};
use crate::memory::BrainfuckMemory;
use crate::errors::InterpreterError;

#[derive(Debug, Clone)]
pub struct BrainfuckInterpreter {
    instruction_set: InstructionSet,
    memory: BrainfuckMemory,
    head: usize,
    loops: Vec<usize>
}

impl BrainfuckInterpreter {
    pub fn new(instruction_set: InstructionSet, memory_size: usize) -> BrainfuckInterpreter {
        BrainfuckInterpreter {
            loops: Vec::new(),  
            instruction_set: instruction_set,
            memory: BrainfuckMemory::new(memory_size),
            head: 0
        }
    }

    pub fn interpret(&mut self) {    

        while self.head != self.instruction_set.inner.len() {
            match self.instruction_set.inner[self.head] {
                Instruction::IncrementByte => {
                    self.memory.increment();
                    self.head += 1;
                }
                Instruction::DecrementByte => {
                    self.memory.decrement();
                    self.head += 1;
                }
                Instruction::IncrementPointer => {
                    self.memory.forward();
                    self.head += 1;
                }
                Instruction::DecrementPointer => {
                    self.memory.move_back();
                    self.head += 1;
                }
                Instruction::Print => {
                    self.memory.print();
                    self.head += 1;
                }
                Instruction::Read => {
                    self.memory.read();
                    self.head += 1
                }
                Instruction::GoBack => {
                    let start_loop_index = self.loops.pop().expect("Syntax error");
                    self.head = start_loop_index;
                }
                Instruction::GoTo => {
                    if self.memory.memory[self.memory.cursor] == 0 {
                        let next_closure = next_closure_index(&self.instruction_set, self.head).expect("Interpreter error");
                        // The rule is to jump next to the next bracket. 
                        self.head = next_closure + 1;
                    } else {
                        self.loops.push(self.head);
                        self.head += 1;
                    }
                }
            }
            println!("\n{}", self.memory);
        }
    }
}




// Search for the next closure
fn next_closure_index(instruction_set: &InstructionSet, position: usize) -> Result<usize, InterpreterError>  {
    let vec_size: usize = instruction_set.inner.len();
    if position >= vec_size {
        return Err(InterpreterError::IncorrectBehavior(
            String::from("Position should be at least eq to the last instruction position"))
        );
    }
    let mut estimated_closure_position: usize = position + 1 ;
    for v in position + 1..vec_size  {
        match instruction_set.inner[v] {
            Instruction::GoBack => {
                return Ok(estimated_closure_position);
            }
            _ => estimated_closure_position += 1
        }
    }
    Err(InterpreterError::MissingGoBackInstruction)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_next_closure_index() {
        let instruction_set = InstructionSet{
            inner: vec![
                Instruction::IncrementByte,
                Instruction::IncrementByte,
                Instruction::GoTo,
                Instruction::IncrementPointer,
                Instruction::IncrementByte,
                Instruction::DecrementPointer,
                Instruction::DecrementByte,
                Instruction::DecrementPointer,
                Instruction::GoBack
            ]
        };
    
        let index = next_closure_index(&instruction_set, 3 as usize).unwrap();
        assert_eq!(index, 8);
    }
    
    #[test]
    fn test_next_closure_index_missing() {
        let instruction_set = InstructionSet{inner: vec![Instruction::GoTo]};
        let res = next_closure_index(&instruction_set, 0 as usize);
        assert_eq!(res.unwrap_err(), InterpreterError::MissingGoBackInstruction);
    }
}

