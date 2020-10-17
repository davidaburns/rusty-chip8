use super::memory::Memory;
use super::keyboard::Keyboard;

pub struct Bus {
    pub ram: Memory,
    pub vram: Memory,
    pub keyboard: Keyboard
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            ram: Memory::new(4096),
            vram: Memory::new(64 * 32),
            keyboard: Keyboard::new()
        }
    }
}