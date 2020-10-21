use super::opcode::Opcode;
use super::instructions::Chip8Instruction;
use super::bus::Bus;

pub enum ProgramCounterOp {
    Next,
    SkipNext,
    Jump(usize)
}

pub struct Chip8<'a>  {
    pub bus: Bus,
    pub v: [u8; 16],
    pub i: u16,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub program_counter: usize,
    pub stack_pointer: usize,
    pub stack: [usize; 16],
    pub opcode: Option<Opcode>,
    opcode_inst: Chip8Instruction<'a>,
}

// CPU functionality
impl<'a> Chip8<'a> {
    pub fn new() -> Chip8<'a> {
        Chip8 {
            bus: Bus::new(),
            v: [0x00; 16],
            i: 0x0000,
            delay_timer: 0,
            sound_timer: 0,
            program_counter: 0x0000,
            stack_pointer: 0x00,
            stack: [0x0000; 16],
            opcode: Option::None,
            opcode_inst: Chip8::noop
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
        self.opcode = Option::None;
        self.opcode_inst = Chip8::noop;
    }

    pub fn cycle(&mut self) { 
        self.fetch();
        self.decode();
        self.execute();
        self.timers();

        print!("\n");
    }

    pub fn fetch(&mut self) {
        let pc = self.program_counter;
        let op = ((self.bus.ram.read(pc) as usize) << 8 | (self.bus.ram.read(pc + 1) as usize)) as u16;

        self.opcode = Some(Opcode::new(op));
    }

    pub fn decode(&mut self) {
        match self.opcode.unwrap().nibbles { 
            (0x00, 0x00, 0x0e, 0x00) => self.opcode_inst = Chip8::_00e0,
            (0x00, 0x00, 0x0e, 0x0e) => self.opcode_inst = Chip8::_00ee,
            (0x01, _, _, _) =>          self.opcode_inst = Chip8::_1nnn,
            (0x02, _, _, _) =>          self.opcode_inst = Chip8::_2nnn,
            (0x03, _, _, _) =>          self.opcode_inst = Chip8::_3xkk,
            (0x04, _, _, _) =>          self.opcode_inst = Chip8::_4xkk,
            (0x05, _, _, 0x00) =>       self.opcode_inst = Chip8::_5xy0,
            (0x06, _, _, _) =>          self.opcode_inst = Chip8::_6xkk,
            (0x07, _, _, _) =>          self.opcode_inst = Chip8::_7xkk,
            (0x08, _, _, 0x00) =>       self.opcode_inst = Chip8::_8xy0,
            (0x08, _, _, 0x01) =>       self.opcode_inst = Chip8::_8xy1,
            (0x08, _, _, 0x02) =>       self.opcode_inst = Chip8::_8xy2,
            (0x08, _, _, 0x03) =>       self.opcode_inst = Chip8::_8xy3,
            (0x08, _, _, 0x04) =>       self.opcode_inst = Chip8::_8xy4,
            (0x08, _, _, 0x05) =>       self.opcode_inst = Chip8::_8xy5,
            (0x08, _, _, 0x06) =>       self.opcode_inst = Chip8::_8xy6,
            (0x08, _, _, 0x07) =>       self.opcode_inst = Chip8::_8xy7,
            (0x08, _, _, 0x0e) =>       self.opcode_inst = Chip8::_8xye,
            (0x09, _, _, 0x00) =>       self.opcode_inst = Chip8::_9xy0,
            (0x0a, _, _, _) =>          self.opcode_inst = Chip8::_annn,
            (0x0b, _, _, _) =>          self.opcode_inst = Chip8::_bnnn,
            (0x0c, _, _, _) =>          self.opcode_inst = Chip8::_cxkk,
            (0x0d, _, _, _) =>          self.opcode_inst = Chip8::_dxyn,
            (0x0e, _, 0x09, 0x0e) =>    self.opcode_inst = Chip8::_ex9e,
            (0x0e, _, 0x0a, 0x01) =>    self.opcode_inst = Chip8::_exa1,
            (0x0f, _, 0x00, 0x07) =>    self.opcode_inst = Chip8::_fx07,
            (0x0f, _, 0x00, 0x0a) =>    self.opcode_inst = Chip8::_fx0a,
            (0x0f, _, 0x01, 0x05) =>    self.opcode_inst = Chip8::_fx15,
            (0x0f, _, 0x01, 0x08) =>    self.opcode_inst = Chip8::_fx18,
            (0x0f, _, 0x01, 0x0e) =>    self.opcode_inst = Chip8::_fx1e,
            (0x0f, _, 0x02, 0x09) =>    self.opcode_inst = Chip8::_fx29,
            (0x0f, _, 0x03, 0x03) =>    self.opcode_inst = Chip8::_fx33,
            (0x0f, _, 0x05, 0x05) =>    self.opcode_inst = Chip8::_fx55,
            (0x0f, _, 0x06, 0x05) =>    self.opcode_inst = Chip8::_fx65,
            _ => self.opcode_inst = Chip8::noop,
        };
    }

    pub fn execute(&mut self) {
        let opcode = self.opcode.unwrap();
        print!("${:04x} 0x{:04x} ",self.program_counter, opcode.op);

        let pc_op = (self.opcode_inst)(self, &opcode);
        if (self.program_counter + 2 > 4095) {
            self.program_counter = 0x0200;
        } else {
            match pc_op {
                ProgramCounterOp::Next => self.program_counter += 2,
                ProgramCounterOp::SkipNext => self.program_counter += 4,
                ProgramCounterOp::Jump(addr) => self.program_counter = addr
            }
        }

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
