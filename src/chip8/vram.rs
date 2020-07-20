pub struct VRam {
    memory: [u8; 64 * 32]
}

impl VRam {
    pub fn new() -> VRam {
        VRam {
            memory: [0x00; 64 * 32]
        }
    }

    pub fn clear(&mut self) {
        self.memory = [0x00; 64 * 32];
    }

    pub fn read(&self, i: usize) -> u8 {
        self.memory[i]
    }

    pub fn write(&mut self, i: usize, value: u8) {
        self.memory[i] = value;
    }
}