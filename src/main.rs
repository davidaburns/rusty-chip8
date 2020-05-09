#![allow(dead_code, unused_variables, unused_parens)]

/*
    CHIP-8 Specification Notes

    CPU:
        - 4kb of ram
        - addressable from 0x000 -> 0xFFF

        - programs start at:
            - 0x200 -> Normal
            - 0x600 -> ETI 660 Chip-8 program start location
        
        - 16 8bit registers (v[0] -> v[15])
            16th register is used for the carry flag

        - 8bit delay timer
        - 8bit sound timer
            - When these registers are non-zero, decrement them at the rate of 60hz till they reach 0

        - Keyboard was a 16-key hex keypad

        - Chip8 System has 8kb of vram in 128x64 pixel mode (Original has 64x32 pixels)

        - Chip8 draws graphics through the use of sprites (groups of pixel bytes)
            - Each sprites pixel bytes are in binary representation of the graphic it is displaying
            - Sprites are comprised of 15 bytes of graphics making the max size of 8x15 pixels 
*/

struct Chip8Emulator<'a> {
    cpu: Chip8<'a>
}

impl<'a> Chip8Emulator<'a> {
    fn new() -> Chip8Emulator<'a> {
        Chip8Emulator {
            cpu: Chip8::new()
        }
    }
    
    fn initialize(&self) {

    }

    fn load_file(&self, file_path: String) {

    }

    fn load_bytes(&self, bytes: [u8; 4096]) {

    }

    fn reset(&self) {

    }

    fn run(&mut self) {
        loop {
            self.cpu.cycle();
        }
    }
}

struct Chip8<'a> {
    memory:  [u8; 4096], // TODO: Implement a 'Ram' struct to house the ram + read and write functionality
    v: [u8; 16],
    delay_timer: u8,
    sound_timer: u8,
    program_counter: u16,
    stack_pointer: u8,
    stack: [u16; 16],
    vram: [u8; 64 * 32], // TODO: Implement a 'Ram' struct to house the ram + read and write functionality
    opcodes: [fn(&mut Chip8<'a>); 32]
}

// Basic CPU functionality
impl<'a> Chip8<'a> {
    fn new() -> Chip8<'a> {
        Chip8 {
            memory: [0x00; 4096],
            v: [0x00; 16],
            delay_timer: 0,
            sound_timer: 0,
            program_counter: 0x0000,
            stack_pointer: 0x00,
            stack: [0x0000; 16],
            vram: [0x00; 64 * 32],
            opcodes: [Chip8::noop; 32]
        }
    }

    fn cycle(&mut self) {
        self.fetch();
        self.decode();
        self.execute();
        self.timers();
    }

    fn fetch(&mut self) {

    }

    fn decode(&mut self) {

    }

    fn execute(&mut self) {

    }

    fn timers(&mut self) {

    }
}

impl<'a> Chip8<'a> {
    fn noop(&mut self) {
        // Empty instruction for initialization purposes
        println!("noop");
    }
}

fn main() {
    let mut emulator = Chip8Emulator::new();
    emulator.run();
}
