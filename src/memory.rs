use std::fmt;
use std::io::{self, Read};

#[derive(Clone, Debug)]
pub struct BrainfuckMemory {
    pub memory: Vec<u8>,
    pub cursor: usize,
    pub size: usize
}

impl BrainfuckMemory {
    pub fn new(size: usize) -> BrainfuckMemory {
        BrainfuckMemory{
            memory: vec![0u8; size as usize],
            cursor: 0,
            size: size            
        }
    }

    pub fn increment(&mut self) {
        self.memory[self.cursor] += 1;
    }

    pub fn decrement(&mut self) {
        self.memory[self.cursor] -= 1;
    }

    pub fn forward(&mut self) {
        self.cursor += 1;
    }

    pub fn move_back(&mut self) {
        self.cursor -= 1;
    }

    pub fn read(&mut self) {
        let mut buffer = ['0' as u8; 1];
        io::stdin().read_exact(&mut buffer).unwrap();
        self.memory[self.cursor] = buffer[0];
    }

    pub fn print(&mut self) {
        print!("{}", self.memory[self.cursor]);
    }
}

impl fmt::Display for BrainfuckMemory {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BF(size: {} - cur: {}) {:?}", self.size, self.cursor, self.memory)
    }
}