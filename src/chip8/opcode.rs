#[derive(Default, Copy, Clone)]
pub struct Opcode {
    pub op: u16,
    pub nibbles: (u16, u16, u16, u8),
    pub x: usize,
    pub y: usize,
    pub n: usize,
    pub kk: u8,
    pub nnn: usize
}

impl Opcode {
    pub fn new(op: u16) -> Opcode {
        let mut ret: Opcode = Opcode {..Default::default()};
        ret.nibbles = (
            (op & 0xF000) >> 12 as u8,
            (op & 0x0F00) >> 8 as u8,
            (op & 0x00F0) >> 4 as u8,
            (op & 0x000F) as u8,
        );

        ret.op = op;
        ret.nnn = (op & 0x0FFF) as usize;
        ret.kk = (op & 0x00FF) as u8;
        ret.x = ret.nibbles.1 as usize;
        ret.y = ret.nibbles.2 as usize;
        ret.n = ret.nibbles.3 as usize;

        ret
    }
}