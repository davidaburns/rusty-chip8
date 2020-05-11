use super::cpu::Chip8;
use super::cpu::ProgramCounterOp;

pub type Chip8Instruction<'a> = fn(&mut Chip8<'a>) -> ProgramCounterOp;

impl<'a> Chip8<'a> {
    pub fn noop(&mut self) -> ProgramCounterOp {
        // Empty instruction for initialization purposes
        print!("_noop");
        ProgramCounterOp::Next
    }

    pub fn _0nnn(&mut self) -> ProgramCounterOp {
        print!("_0nnn");
        ProgramCounterOp::Next
    }

    // CLS: Clear the screen
    pub fn _00e0(&mut self) -> ProgramCounterOp {
        print!("_00e0");

        let vram_ref = self.vram.as_deref_mut();
        for pixel in vram_ref.unwrap().iter_mut() {
            *pixel = 0x00;
        }

        ProgramCounterOp::Next
    }
 
    pub fn _00ee(&mut self) -> ProgramCounterOp {
        print!("_00ee");
        ProgramCounterOp::Next
    }

    pub fn _1nnn(&mut self) -> ProgramCounterOp {
        print!("_1nnn");
        ProgramCounterOp::Next
    }

    pub fn _2nnn(&mut self) -> ProgramCounterOp {
        print!("_2nnn");
        ProgramCounterOp::Next
    }

    pub fn _3xnn(&mut self) -> ProgramCounterOp {
        print!("_3xnn");
        ProgramCounterOp::Next
    }

    pub fn _4xnn(&mut self) -> ProgramCounterOp {
        print!("_4xnn");
        ProgramCounterOp::Next
    }

    pub fn _5xy0(&mut self) -> ProgramCounterOp {
        print!("_5xy0");
        ProgramCounterOp::Next
    }

    pub fn _6xnn(&mut self) -> ProgramCounterOp {
        print!("_6xnn");

        let nibbles = self.deconstruct_opcode();
        let x = nibbles.1 as usize;
        let nn = ((nibbles.2) << 4 | nibbles.3 as u16) as u8;

        self.v[x] = nn;

        ProgramCounterOp::Next
    }

    pub fn _7xnn(&mut self) -> ProgramCounterOp {
        print!("_7xnn");
        ProgramCounterOp::Next
    }

    pub fn _8xy0(&mut self) -> ProgramCounterOp {
        print!("_8xy0");
        ProgramCounterOp::Next
    }

    pub fn _8xy1(&mut self) -> ProgramCounterOp {
        print!("_8xy1");
        ProgramCounterOp::Next
    }

    pub fn _8xy2(&mut self) -> ProgramCounterOp {
        print!("_8xy2");
        ProgramCounterOp::Next
    }

    pub fn _8xy3(&mut self) -> ProgramCounterOp {
        print!("_8xy3");
        ProgramCounterOp::Next
    }

    pub fn _8xy4(&mut self) -> ProgramCounterOp {
        print!("_8xy4");
        ProgramCounterOp::Next
    }

    pub fn _8xy5(&mut self) -> ProgramCounterOp {
        print!("_8xy5");
        ProgramCounterOp::Next
    }

    pub fn _8xy6(&mut self) -> ProgramCounterOp {
        print!("_8xy6");
        ProgramCounterOp::Next
    }

    pub fn _8xy7(&mut self) -> ProgramCounterOp {
        print!("_8xy7");
        ProgramCounterOp::Next
    }

    pub fn _8xye(&mut self) -> ProgramCounterOp {
        print!("_8xye");
        ProgramCounterOp::Next
    }

    pub fn _9xy0(&mut self) -> ProgramCounterOp {
        print!("_9xy0");
        ProgramCounterOp::Next
    }

    pub fn _annn(&mut self) -> ProgramCounterOp {
        print!("_annn");
        ProgramCounterOp::Next
    }

    pub fn _bnnn(&mut self) -> ProgramCounterOp {
        print!("_bnnn");
        ProgramCounterOp::Next
    }

    pub fn _cxnn(&mut self) -> ProgramCounterOp {
        print!("_cxnn");
        ProgramCounterOp::Next
    }

    pub fn _dxyn(&mut self) -> ProgramCounterOp {
        print!("_dxyn");
        ProgramCounterOp::Next
    }

    pub fn _ex9e(&mut self) -> ProgramCounterOp {
        print!("_ex9e");
        ProgramCounterOp::Next
    }

    pub fn _exa1(&mut self) -> ProgramCounterOp {
        print!("_exa1");
        ProgramCounterOp::Next
    }

    pub fn _fx07(&mut self) -> ProgramCounterOp {
        print!("_fx07");
        ProgramCounterOp::Next
    }

    pub fn _fx0a(&mut self) -> ProgramCounterOp {
        print!("_fx0a");
        ProgramCounterOp::Next
    }

    pub fn _fx15(&mut self) -> ProgramCounterOp {
        print!("_fx15");
        ProgramCounterOp::Next
    }

    pub fn _fx18(&mut self) -> ProgramCounterOp {
        print!("_fx18");
        ProgramCounterOp::Next
    }

    pub fn _fx1e(&mut self) -> ProgramCounterOp {
        print!("_fx1e");
        ProgramCounterOp::Next
    }

    pub fn _fx29(&mut self) -> ProgramCounterOp {
        print!("_fx29");
        ProgramCounterOp::Next
    }

    pub fn _fx33(&mut self) -> ProgramCounterOp {
        print!("_fx33");
        ProgramCounterOp::Next
    }

    pub fn _fx55(&mut self) -> ProgramCounterOp {
        print!("_fx55");
        ProgramCounterOp::Next
    }

    pub fn _fx65(&mut self) -> ProgramCounterOp {
        print!("_fx65");
        ProgramCounterOp::Next
    }
}
