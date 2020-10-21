use super::cpu::Chip8;
use super::cpu::ProgramCounterOp;
use super::opcode::Opcode;

use rand::{thread_rng, Rng};

pub type Chip8Instruction<'a> = fn(&mut Chip8<'a>, &Opcode) -> ProgramCounterOp;

impl<'a> Chip8<'a> {
    pub fn noop(&mut self, op: &Opcode) -> ProgramCounterOp {
        // Empty instruction for initialization purposes
        print!("_noop");
        ProgramCounterOp::Next
    }

    pub fn _0nnn(&mut self, op: &Opcode) -> ProgramCounterOp {
        // *NOT NEEDED ONLY LEGACY MACHINES*
        print!("_0nnn");
        ProgramCounterOp::Next
    }

    pub fn _00e0(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_00e0");

        self.bus.vram.clear();
        ProgramCounterOp::Next
    }
 
    pub fn _00ee(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_00ee");

        let addr = self.stack[self.stack_pointer];
        self.stack_pointer -= 1;

        ProgramCounterOp::Jump(addr)
    }

    pub fn _1nnn(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_1nnn");
        ProgramCounterOp::Jump(op.nnn)
    }

    pub fn _2nnn(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_2nnn");

        self.stack_pointer += 1;
        self.stack[self.stack_pointer] = self.program_counter;

        ProgramCounterOp::Next
    }

    pub fn _3xkk(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_3xkk");
        if (self.v[op.x] == op.kk) {
            return ProgramCounterOp::Next;
        }

        ProgramCounterOp::Next
    }

    pub fn _4xkk(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_4xkk");
        if (self.v[op.x] != op.kk) {
            return ProgramCounterOp::SkipNext;
        }

        ProgramCounterOp::Next
    }

    pub fn _5xy0(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_5xy0");
        if (self.v[op.x] != self.v[op.y]) {
            return ProgramCounterOp::SkipNext;
        }

        ProgramCounterOp::Next
    }

    pub fn _6xkk(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_6xkk");

        self.v[op.x] = op.kk;
        ProgramCounterOp::Next
    }

    pub fn _7xkk(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_7xkk");

        self.v[op.x] = self.v[op.x].wrapping_add(op.kk);
        ProgramCounterOp::Next
    }

    pub fn _8xy0(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_8xy0");

        self.v[op.x] = self.v[op.y];
        ProgramCounterOp::Next
    }

    pub fn _8xy1(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_8xy1");

        self.v[op.x] = (self.v[op.x] | self.v[op.y]);
        ProgramCounterOp::Next
    }

    pub fn _8xy2(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_8xy2");

        self.v[op.x] = (self.v[op.x] & self.v[op.y]);
        ProgramCounterOp::Next
    }

    pub fn _8xy3(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_8xy3");
                
        self.v[op.x] = (self.v[op.x] ^ self.v[op.y]);
        ProgramCounterOp::Next
    }

    pub fn _8xy4(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_8xy4");
        let result = (self.v[op.x] + self.v[op.y]) as u16;
        self.v[0x0F] = if (result > 255) {
            1
        } else {
            0
        };

        self.v[op.x] = (result & 0x00FF) as u8;
        ProgramCounterOp::Next
    }

    pub fn _8xy5(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_8xy5");
        self.v[0x0F] = if (self.v[op.x] > self.v[op.y]) {
            1
        } else {
            0
        };

        self.v[op.x] = self.v[op.x].wrapping_sub(self.v[op.y]);
        ProgramCounterOp::Next
    }

    pub fn _8xy6(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_8xy6");

        self.v[0x0F] = self.v[op.x] & 1;
        self.v[op.x] = self.v[op.x] / 2;

        ProgramCounterOp::Next
    }

    pub fn _8xy7(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_8xy7");
        self.v[0x0F] = if (self.v[op.x] > self.v[op.y]) {
            1
        } else {
            0
        };

        self.v[op.x] = self.v[op.y].wrapping_sub(self.v[op.x]);
        ProgramCounterOp::Next
    }

    pub fn _8xye(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_8xye");

        self.v[0x0F] = (self.v[op.x] >> 7) & 1;
        self.v[op.x] = self.v[op.x].wrapping_mul(2);

        ProgramCounterOp::Next
    }

    pub fn _9xy0(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_9xy0");
        if (self.v[op.x] != self.v[op.y]) {
            return ProgramCounterOp::SkipNext;
        }

        ProgramCounterOp::Next
    }

    pub fn _annn(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_annn");

        self.i = op.nnn as u16;
        ProgramCounterOp::Next
    }

    pub fn _bnnn(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_bnnn");
        ProgramCounterOp::Jump(op.nnn  + (self.v[0] as usize))
    }

    pub fn _cxkk(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_cxkk");

        let rnd: u8 = thread_rng().gen_range(0, 255);
        self.v[op.x] = rnd & op.kk;

        ProgramCounterOp::Next
    }

    pub fn _dxyn(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_dxyn");

        for byte in 0..op.n {
            let y = (self.v[op.y] as usize + byte) % self.bus.display.height;
            for bit in 0..8 {
                let x = (self.v[op.x] as usize + bit) % self.bus.display.width;
                let color = (self.bus.ram.read(self.i as usize + byte) >> (7 - bit)) & 1;

                let vram_idx = y * x;
                let vram_value = self.bus.vram.read(vram_idx);

                self.v[0x0F] |= color & vram_value;
                self.bus.vram.write(vram_idx, vram_value ^ color);
            }
        }

        ProgramCounterOp::Next
    }

    pub fn _ex9e(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_ex9e");
        
        let key = self.v[op.x];
        if (self.bus.keyboard.is_pressed(key)) {
            return ProgramCounterOp::SkipNext;
        }

        ProgramCounterOp::Next
    }

    pub fn _exa1(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_exa1");

        let key = self.v[op.x];
        if (!self.bus.keyboard.is_pressed(key)) {
            return ProgramCounterOp::SkipNext;
        }

        ProgramCounterOp::Next
    }

    pub fn _fx07(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_fx07");

        self.v[op.x] = self.delay_timer;
        ProgramCounterOp::Next
    }

    pub fn _fx0a(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_fx0a");

        self.v[op.x] = self.bus.keyboard.wait_for_key();
        ProgramCounterOp::Next
    }

    pub fn _fx15(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_fx15");

        self.delay_timer = self.v[op.x];
        ProgramCounterOp::Next
    }

    pub fn _fx18(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_fx18");

        self.sound_timer = self.v[op.x];
        ProgramCounterOp::Next
    }

    pub fn _fx1e(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_fx1e");

        self.i += self.v[op.x] as u16;
        ProgramCounterOp::Next
    }

    pub fn _fx29(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_fx29");

        self.i = (self.v[op.x] * 5) as u16;
        ProgramCounterOp::Next
    }

    pub fn _fx33(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_fx33");

        let vx = self.v[op.x];
        self.bus.ram.write(self.i as usize, vx / 100);
        self.bus.ram.write((self.i + 1) as usize, (vx % 100) / 10);
        self.bus.ram.write((self.i + 2) as usize, vx % 10);

        ProgramCounterOp::Next
    }

    pub fn _fx55(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_fx55");
        for idx in 0..op.x {
            self.bus.ram.write(self.i as usize + idx, self.v[idx]);
        }

        ProgramCounterOp::Next
    }

    pub fn _fx65(&mut self, op: &Opcode) -> ProgramCounterOp {
        print!("_fx65");
        for idx in 0..op.x {
            self.v[idx] = self.bus.ram.read(self.i as usize + idx);
        }

        ProgramCounterOp::Next
    }
}
