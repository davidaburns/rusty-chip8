use super::cpu::Chip8;
use std::fs::File;
use std::io::Read;

pub struct Emulator<'a> {
    cpu: Chip8<'a>,
    memory: [u8; 4096],
    vram: [u8; 64 * 32],
}

impl<'a> Emulator<'a> {
    pub fn new() -> Emulator<'a> {
        Emulator {
            cpu: Chip8::new(),
            memory: [0x00; 4096],
            vram: [0x00; 64 * 32],
        }
    }

    pub fn load(&mut self, file_path: String) {
        // Load binary data from rom file into a buffer
        let mut f = File::open(&file_path)
            .expect("Specified rom file not found");

        let metadata = std::fs::metadata(&file_path)
            .expect("Unable to read rom file metadata");

        let mut buffer = vec![0 as u8; metadata.len() as usize];
        f.read(&mut buffer)
            .expect("ERR: Buffer overflow occured while attempting to load rom file");

        // Load the rom buffer into chip8 ram with the offset of 0x0200
        for (pos, data) in buffer.iter().enumerate() {
            self.cpu.bus.ram.write(0x0200 + pos, *data);
        }
    }

    pub fn run(&'a mut self) {
        self.cpu.initialize();
        loop {
            self.cpu.cycle();
        }
    }
}
