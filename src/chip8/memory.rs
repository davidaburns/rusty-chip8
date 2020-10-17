use std::vec::Vec;
pub struct Memory{
    size: usize,
    ram: Vec<u8>
}

impl Memory {
    pub fn new(size: usize) -> Memory {
        Memory {
            size: size,
            ram: vec![0x00; size]
        }
    }

    pub fn clear(&mut self) {
        self.ram = vec![0x00; self.size];
    }

    pub fn read(&self, i: usize) -> u8 {
        self.ram[i]
    }

    pub fn write(&mut self, i: usize, value: u8) {
        self.ram[i] = value;
    }
}