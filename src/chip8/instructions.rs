use super::cpu::Chip8;

impl<'a> Chip8<'a> {
    pub fn noop(&mut self) {
        // Empty instruction for initialization purposes
        print!("noop");
    }

    pub fn _0nnn(&mut self) {
        print!("_0nnn");
    }

    pub fn _00e0(&mut self) {
        print!("_00e0");
    }

    pub fn _00ee(&mut self) {
        print!("_00ee");
    }

    pub fn _1nnn(&mut self) {
        print!("_1nnn");
    }

    pub fn _2nnn(&mut self) {
        print!("_2nnn");
    }

    pub fn _3xnn(&mut self) {
        print!("_3xnn");
    }

    pub fn _4xnn(&mut self) {
        print!("_4xnn");
    }

    pub fn _5xy0(&mut self) {
        print!("_4xnn");
    }

    pub fn _6xnn(&mut self) {
        print!("_6xnn");
    }

    pub fn _7xnn(&mut self) {
        print!("_7xnn");
    }

    pub fn _8xy0(&mut self) {
        print!("_8xy0");
    }

    pub fn _8xy1(&mut self) {
        print!("_8xy1");
    }

    pub fn _8xy2(&mut self) {
        print!("_8xy2");
    }

    pub fn _8xy3(&mut self) {
        print!("_8xy3");
    }

    pub fn _8xy4(&mut self) {
        print!("_8xy4");
    }

    pub fn _8xy5(&mut self) {
        print!("_8xy5");
    }

    pub fn _8xy6(&mut self) {
        print!("_8xy6");
    }

    pub fn _8xy7(&mut self) {
        print!("_8xy7");
    }

    pub fn _8xye(&mut self) {
        print!("_8xye");
    }

    pub fn _9xy0(&mut self) {
        print!("_9xy0");
    }

    pub fn _annn(&mut self) {
        print!("_annn");
    }

    pub fn _bnnn(&mut self) {
        print!("_bnnn");
    }

    pub fn _cxnn(&mut self) {
        print!("_cxnn");
    }

    pub fn _dxyn(&mut self) {
        print!("_dxyn");
    }

    pub fn _ex9e(&mut self) {
        print!("_ex9e");
    }

    pub fn _exa1(&mut self) {
        print!("_exa1");
    }

    pub fn _fx07(&mut self) {
        print!("_fx07");
    }

    pub fn _fx0a(&mut self) {
        print!("_fx0a");
    }

    pub fn _fx15(&mut self) {
        print!("_fx15");
    }

    pub fn _fx18(&mut self) {
        print!("_fx18");
    }

    pub fn _fx1e(&mut self) {
        print!("_fx1e");
    }

    pub fn _fx29(&mut self) {
        print!("_fx29");
    }

    pub fn _fx33(&mut self) {
        print!("_fx33");
    }

    pub fn _fx55(&mut self) {
        print!("_fx55");
    }

    pub fn _fx65(&mut self) {
        print!("_fx65");
    }
}
