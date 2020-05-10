pub struct Ram {
    memory: Vec<u8>
}

impl Ram {
    pub fn new(size: usize) -> Ram {
        Ram {
            memory: vec![0x00; size]
        }
    }
    
    pub fn read(&self, addr: usize) -> u8 {
        self.memory[addr]
    }

    pub fn write(&mut self, addr: usize, val: u8) {
        self.memory[addr] = val;
    }
}