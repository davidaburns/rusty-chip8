#![allow(dead_code, unused_variables, unused_parens, arithmetic_overflow)]

/*
    CHIP-8 Specification Notes

    CPU:
        - 4kb of ram
        - addressable from 0x000 -> 0xFFF

        - programs start at:
            - 0x200 -> Normal
            - 0x600 -> ETI 660 Chip-8 program start location
        
        - 16 8bit registers (v[0] -> v[15]) (Data Registers)
            - 16th register is used for the carry flag
            - accepted values are 0x00 -> 0xFF

        - 16bit 'I' register used to stored memory addresses for later use

        - 8bit delay timer
        - 8bit sound timer
            - When these registers are non-zero, decrement them at the rate of 60hz till they reach 0

        - Keyboard was a 16-key hex keypad

        - Chip8 System has 2kb of vram to represent a 64x32 screen of pixels

        - Chip8 draws graphics through the use of sprites (groups of pixel bytes)
            - Each sprites pixel bytes are in binary representation of the graphic it is displaying
            - Sprites are comprised of 15 bytes of graphics making the max size of 8x15 pixels 

        - Each instruction is 2bytes wide with the data encoded directly in the instruction
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
    
    fn initialize(&mut self) {
        self.load_font_glyphs();
        self.cpu.program_counter = 0x200;
    }

    fn load_font_glyphs(&mut self) {

    }

    fn load_file(&mut self, file_path: String) {

    }

    fn load_bytes(&mut self, bytes: [u8; 4096]) {

    }

    fn reset(&mut self) {
        self.initialize();
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
    i: u16,
    delay_timer: u8,
    sound_timer: u8,
    program_counter: usize,
    stack_pointer: u8,
    stack: [u16; 16],
    vram: [u8; 64 * 32], // TODO: Implement a 'Ram' struct to house the ram + read and write functionality
    instructions: [fn(&mut Chip8<'a>); 32],
    opcode: u16
}

// Basic CPU functionality
impl<'a> Chip8<'a> {
    fn new() -> Chip8<'a> {
        Chip8 {
            memory: [0x00; 4096],
            v: [0x00; 16],
            i: 0x0000,
            delay_timer: 0,
            sound_timer: 0,
            program_counter: 0x0000,
            stack_pointer: 0x00,
            stack: [0x0000; 16],
            vram: [0x00; 64 * 32],
            instructions: [Chip8::noop; 32],
            opcode: 0x0000
        }
    }

    fn cycle(&mut self) {
        self.fetch();
        self.decode();
        self.execute();
        self.timers();

        //DEBUG PURPOSES
        if (self.program_counter + 2 > 0x0FFF) {
            self.program_counter = 0x200
        } else {
            self.program_counter += 2;
        }        
    }

    fn fetch(&mut self) {
        let pc = self.program_counter;
        self.opcode = ((self.memory[pc] as usize) << 8 | self.memory[pc + 1] as usize) as u16;
    }

    fn decode(&mut self) {

    }

    fn execute(&mut self) {
        println!("${:04x} {:04x}", self.program_counter, self.opcode);
    }

    fn timers(&mut self) {
        if (self.delay_timer > 0) {
            self.delay_timer -= 1;
        }

        if (self.sound_timer > 0) {
            self.sound_timer -= 1;
            if (self.sound_timer == 0) {
                println!("BEEP");
            }
        }
    }
}

impl<'a> Chip8<'a> {
    fn noop(&mut self) {
        // Empty instruction for initialization purposes
        println!("noop");
    }

    fn _0nnn(&mut self) {
        println!("_0nnn");
    }

    fn _00e0(&mut self) {
        println!("_00e0");
    }

    fn _00ee(&mut self) {
        println!("_00ee");
    }

    fn _1nnn(&mut self) {
        println!("_1nnn");
    }

    fn _2nnn(&mut self) {
        println!("_2nnn");
    }

    fn _3xnn(&mut self) {
        println!("_3xnn");
    }

    fn _4xnn(&mut self) {
        println!("_4xnn");
    }

    fn _5xy0(&mut self) {
        println!("_4xnn");
    }

    fn _6xnn(&mut self) {
        println!("_6xnn");
    }

    fn _7xnn(&mut self) {
        println!("_7xnn");
    }

    fn _8xy0(&mut self) {
        println!("_8xy0");
    }

    fn _8xy1(&mut self) {
        println!("_8xy1");
    }

    fn _8xy2(&mut self) {
        println!("_8xy2");
    }

    fn _8xy3(&mut self) {
        println!("_8xy3");
    }

    fn _8xy4(&mut self) {
        println!("_8xy4");
    }

    fn _8xy5(&mut self) {
        println!("_8xy5");
    }

    fn _8xy6(&mut self) {
        println!("_8xy6");
    }

    fn _8xy7(&mut self) {
        println!("_8xy7");
    }

    fn _8xye(&mut self) {
        println!("_8xye");
    }

    fn _9xy0(&mut self) {
        println!("_9xy0");
    }

    fn _annn(&mut self) {
        println!("_annn");
    }

    fn _bnnn(&mut self) {
        println!("_bnnn");
    }

    fn _cxnn(&mut self) {
        println!("_cxnn");
    }

    fn _dxyn(&mut self) {
        println!("_dxyn");
    }

    fn _ex9e(&mut self) {
        println!("_ex9e");
    }

    fn _exa1(&mut self) {
        println!("_exa1");
    }

    fn _fx07(&mut self) {
        println!("_fx07");
    }

    fn _fx0a(&mut self) {
        println!("_fx0a");
    }

    fn _fx15(&mut self) {
        println!("_fx15");
    }

    fn _fx18(&mut self) {
        println!("_fx18");
    }

    fn _fx1e(&mut self) {
        println!("_fx1e");
    }

    fn _fx29(&mut self) {
        println!("_fx29");
    }

    fn _fx33(&mut self) {
        println!("_fx33");
    }

    fn _fx55(&mut self) {
        println!("_fx55");
    }

    fn _fx65(&mut self) {
        println!("_fx65");
    }
}

fn main() {
    let mut emulator = Chip8Emulator::new();

    emulator.initialize();
    emulator.load_file("pong.ch8".to_string());
    emulator.run();
}
