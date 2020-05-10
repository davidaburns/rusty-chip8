use super::cpu::Chip8;
use super::ram::Ram;
use std::fs::File;
use std::io::Read;

pub struct Emulator<'a> {
    cpu: Chip8<'a>,
    memory: Ram,
    vram: Ram
}

impl<'a> Emulator<'a> {
    pub fn new() -> Emulator<'a> {
        Emulator {
            cpu: Chip8::new(),
            memory: Ram::new(4096),
            vram: Ram::new(64 * 32)
        }
    }
    
    pub fn initialize(&'a mut self) {
        self.cpu.initialize();
        self.cpu.memory = Some(&mut self.memory);
        self.cpu.vram = Some(&mut self.vram);
    }

    pub fn load_rom(&mut self, file_path: String) {
        // Load binary data from rom file into a buffer
        let mut f = File::open(&file_path).expect("Specified rom file not found");
        let metadata = std::fs::metadata(&file_path).expect("Unable to read rom file metadata");
        let mut buffer = vec![0 as u8; metadata.len() as usize];

        f.read(&mut buffer).expect("ERR: Buffer overflow occured while attempting to load rom file");

        // Load the rom buffer into chip8 ram with the offset of 0x0200
        let mem = self.cpu.memory.as_deref_mut().unwrap();
        for (pos, data) in buffer.iter().enumerate() {
            mem.write(pos + 0x200, *data);
        }
    }

    pub fn load_bytes(&mut self, bytes: [u8; 4096]) {
        // Load direct bytes directly into memory.
    }

    pub fn reset(&'a mut self) {
        self.initialize();
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.cycle();
        }
    }
}