// Turn off warnings while debugging, removing these
#![allow(dead_code, unused_variables, unused_parens, unused_imports, arithmetic_overflow)]

use std::fs::File;
use std::io::Read;

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
            - Instructions will be represented by an array of function pointers to their respective opcode
            - Need to determine how to translate 2byte instruction into an index representing the opcode to call
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
        self.cpu.initialize();
    }

    fn load_font_glyphs(&mut self) {

    }

    fn load_rom(&mut self, file_path: String) {
        // Load binary data from rom file into a buffer
        let mut f = File::open(&file_path).expect("Specified rom file not found");
        let metadata = std::fs::metadata(&file_path).expect("Unable to read rom file metadata");
        let mut buffer = vec![0 as u8; metadata.len() as usize];

        f.read(&mut buffer).expect("ERR: Buffer overflow occured while attempting to load rom file");

        // Load the rom buffer into chip8 ram with the offset of 0x0200
        for (pos, data) in buffer.iter().enumerate() {
            self.cpu.memory[pos + 0x200] = *data
        }
    }

    fn load_bytes(&mut self, bytes: [u8; 4096]) {
        // Load direct bytes directly into memory.
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
    vram: [u8; 64 * 32], // TODO: Implement a 'Ram' struct to house the ram + read and write functionality
    v: [u8; 16],
    i: u16,
    delay_timer: u8,
    sound_timer: u8,
    program_counter: usize,
    stack_pointer: u8,
    stack: [u16; 16],
    
    // Emulator specific
    instructions: [fn(&mut Chip8<'a>); 36],
    opcode: u16,
    opcode_index: usize
}

// CPU functionality
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
            instructions: [Chip8::noop; 36],
            opcode: 0x0000,
            opcode_index: 0
        }
    }

    fn initialize(&mut self) {
        self.v = [0x00; 16];
        self.i = 0x0000;
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.program_counter = 0x0200;
        self.stack_pointer = 0x00;
        self.stack = [0x000; 16];
        self.opcode = 0x0000;
        self.opcode_index = 0;

        self.instructions = [
            Chip8::noop, Chip8::_0nnn, Chip8::_00e0, Chip8::_00ee,
            Chip8::_1nnn, Chip8::_2nnn, Chip8::_3xnn, Chip8::_4xnn,
            Chip8::_5xy0, Chip8::_6xnn, Chip8::_7xnn, Chip8::_8xy0,
            Chip8::_8xy1, Chip8::_8xy2, Chip8::_8xy3, Chip8::_8xy4,
            Chip8::_8xy5, Chip8::_8xy6, Chip8::_8xy7, Chip8::_8xye,
            Chip8::_9xy0, Chip8::_annn, Chip8::_bnnn, Chip8::_cxnn,
            Chip8::_dxyn, Chip8::_ex9e, Chip8::_exa1, Chip8::_fx07,
            Chip8::_fx0a, Chip8::_fx15, Chip8::_fx18, Chip8::_fx1e,
            Chip8::_fx29, Chip8::_fx33, Chip8::_fx55, Chip8::_fx65, 
        ]
    }

    fn cycle(&mut self) {
        self.fetch();
        self.decode();
        self.execute();
        self.timers();

        // DEBUG PURPOSES
        if (self.program_counter + 2 > 0x0FFF) {
            self.program_counter = 0x200
        } else {
            self.program_counter += 2;
        }
        
        // DEBUG PURPOSES FOR CONSOLE OUTPUT
        print!("\n");
    }

    fn fetch(&mut self) {
        let pc = self.program_counter;
        self.opcode = ((self.memory[pc] as usize) << 8 | self.memory[pc + 1] as usize) as u16;
    }

    fn decode(&mut self) {
        match (self.opcode & 0xF000) {
            0x0 => {
               match (self.opcode & 0x00FF) {
                   0x0000 => self.opcode_index = 0,
                   0x00E0 => self.opcode_index = 2,
                   0x00EE => self.opcode_index = 3, 
                   _ => self.opcode_index = 1
               } 
            },
            0x1000 => self.opcode_index = 4,
            0x2000 => self.opcode_index = 5,
            0x3000 => self.opcode_index = 6,
            0x4000 => self.opcode_index = 7,
            0x5000 => self.opcode_index = 8,
            0x6000 => self.opcode_index = 9,
            0x7000 => self.opcode_index = 10,
            0x8000 => {
                match (self.opcode & 0x000F) {
                    0x0000 => self.opcode_index = 11,
                    0x0001 => self.opcode_index = 12,
                    0x0002 => self.opcode_index = 13,
                    0x0003 => self.opcode_index = 14,
                    0x0004 => self.opcode_index = 15,
                    0x0005 => self.opcode_index = 16,
                    0x0006 => self.opcode_index = 17,
                    0x0007 => self.opcode_index = 18,
                    0x000E => self.opcode_index = 19,
                    _ => self.opcode_index = 0
                } 
            },
            0x9000 => self.opcode_index = 20,
            0xA000 => self.opcode_index = 21,
            0xB000 => self.opcode_index = 22,
            0xC000 => self.opcode_index = 23,
            0xD000 => self.opcode_index = 24,
            0xE000 => {
                match (self.opcode & 0x00FF) {
                    0x009E => self.opcode_index = 25,
                    0x00A1 => self.opcode_index = 26,
                    _ => self.opcode_index = 0
                }
            },
            0xF000 => {
                match (self.opcode & 0x00FF) {
                    0x0007 => self.opcode_index = 27,
                    0x000A => self.opcode_index = 28,
                    0x0015 => self.opcode_index = 29,
                    0x0018 => self.opcode_index = 30,
                    0x001E => self.opcode_index = 31,
                    0x0029 => self.opcode_index = 32,
                    0x0033 => self.opcode_index = 33,
                    0x0055 => self.opcode_index = 34,
                    0x0065 => self.opcode_index = 35,
                    _ => self.opcode_index = 0
                }
            },
            _ => self.opcode_index = 0
        }
    }

    fn execute(&mut self) {
        print!("${:04x} {:04x} [{}] ", self.program_counter, self.opcode, self.opcode_index);
        self.instructions[self.opcode_index](self);      
    }

    fn timers(&mut self) {
        if (self.delay_timer > 0) {
            self.delay_timer -= 1;
        }

        if (self.sound_timer > 0) {
            self.sound_timer -= 1;
            if (self.sound_timer == 0) {
                print!(" BEEP");
            }
        }
    }
}

impl<'a> Chip8<'a> {
    fn noop(&mut self) {
        // Empty instruction for initialization purposes
        print!("noop");
    }

    fn _0nnn(&mut self) {
        print!("_0nnn");
    }

    fn _00e0(&mut self) {
        print!("_00e0");
    }

    fn _00ee(&mut self) {
        print!("_00ee");
    }

    fn _1nnn(&mut self) {
        print!("_1nnn");
    }

    fn _2nnn(&mut self) {
        print!("_2nnn");
    }

    fn _3xnn(&mut self) {
        print!("_3xnn");
    }

    fn _4xnn(&mut self) {
        print!("_4xnn");
    }

    fn _5xy0(&mut self) {
        print!("_4xnn");
    }

    fn _6xnn(&mut self) {
        print!("_6xnn");
    }

    fn _7xnn(&mut self) {
        print!("_7xnn");
    }

    fn _8xy0(&mut self) {
        print!("_8xy0");
    }

    fn _8xy1(&mut self) {
        print!("_8xy1");
    }

    fn _8xy2(&mut self) {
        print!("_8xy2");
    }

    fn _8xy3(&mut self) {
        print!("_8xy3");
    }

    fn _8xy4(&mut self) {
        print!("_8xy4");
    }

    fn _8xy5(&mut self) {
        print!("_8xy5");
    }

    fn _8xy6(&mut self) {
        print!("_8xy6");
    }

    fn _8xy7(&mut self) {
        print!("_8xy7");
    }

    fn _8xye(&mut self) {
        print!("_8xye");
    }

    fn _9xy0(&mut self) {
        print!("_9xy0");
    }

    fn _annn(&mut self) {
        print!("_annn");
    }

    fn _bnnn(&mut self) {
        print!("_bnnn");
    }

    fn _cxnn(&mut self) {
        print!("_cxnn");
    }

    fn _dxyn(&mut self) {
        print!("_dxyn");
    }

    fn _ex9e(&mut self) {
        print!("_ex9e");
    }

    fn _exa1(&mut self) {
        print!("_exa1");
    }

    fn _fx07(&mut self) {
        print!("_fx07");
    }

    fn _fx0a(&mut self) {
        print!("_fx0a");
    }

    fn _fx15(&mut self) {
        print!("_fx15");
    }

    fn _fx18(&mut self) {
        print!("_fx18");
    }

    fn _fx1e(&mut self) {
        print!("_fx1e");
    }

    fn _fx29(&mut self) {
        print!("_fx29");
    }

    fn _fx33(&mut self) {
        print!("_fx33");
    }

    fn _fx55(&mut self) {
        print!("_fx55");
    }

    fn _fx65(&mut self) {
        print!("_fx65");
    }
}

fn main() {
    let mut emulator = Chip8Emulator::new();

    emulator.initialize();
    emulator.load_rom("./pong.ch8".to_string());
    emulator.run();
}
