pub struct Chip8<'a> {
    pub memory:  [u8; 4096], // TODO: Implement a 'Ram' struct to house the ram + read and write functionality
    pub vram: [u8; 64 * 32], // TODO: Implement a 'Ram' struct to house the ram + read and write functionality
    pub v: [u8; 16],
    pub i: u16,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub program_counter: usize,
    pub stack_pointer: u8,
    pub stack: [u16; 16],
    
    // Emulator specific
    pub instructions: [fn(&mut Chip8<'a>); 36],
    pub opcode: u16,
    pub opcode_index: usize,
    pub increment_program_counter: bool
}

// CPU functionality
impl<'a> Chip8<'a> {
    pub fn new() -> Chip8<'a> {
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
            opcode_index: 0,
            increment_program_counter: true
        }
    }

    pub fn initialize(&mut self) {
        self.v = [0x00; 16];
        self.i = 0x0000;
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.program_counter = 0x0200;
        self.stack_pointer = 0x00;
        self.stack = [0x000; 16];
        self.opcode = 0x0000;
        self.opcode_index = 0;
        self.increment_program_counter = true;

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

    pub fn cycle(&mut self) {
        self.fetch();
        self.decode();
        self.execute();
        self.timers();

        // Increment program counter if there has been no jumps or subroutine calls
        // TODO: Add flag to determine if jump/subroutine call has occured
        if (self.program_counter + 2 > 0x0FFF) {
            self.program_counter = 0x200
        } else {
            self.program_counter += 2;
        }
        
        // DEBUG PURPOSES FOR CONSOLE OUTPUT
        print!("\n");
    }

    pub fn fetch(&mut self) {
        let pc = self.program_counter;
        self.opcode = ((self.memory[pc] as usize) << 8 | self.memory[pc + 1] as usize) as u16;
    }

    pub fn decode(&mut self) {
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

    pub fn execute(&mut self) {
        print!("${:04x} {:04x} [{}] ", self.program_counter, self.opcode, self.opcode_index);
        self.instructions[self.opcode_index](self);      
    }

    pub fn timers(&mut self) {
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