use super::memory::Memory;
use super::keyboard::Keyboard;
use super::display::Display;

pub struct Bus {
    pub ram: Memory,
    pub vram: Memory,
    pub keyboard: Keyboard,
    pub display: Display
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            ram: Memory::new(4096),
            vram: Memory::new(64 * 32),
            keyboard: Keyboard::new(),
            display: Display::new(64, 32)
        }
    }
}