pub struct Ram {
    memory: [u8; 4096]
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            memory: [0x00; 4096]
        }
    }

    pub fn clear(&mut self) {
        self.memory = [0x00; 4096];
    }

    pub fn read(&self, i: usize) -> u8 {
        self.memory[i]
    }

    pub fn write(&mut self, i: usize, value: u8) {
        self.memory[i] = value;
    }
}