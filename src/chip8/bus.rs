use super::ram::Ram;
use super::vram::VRam;
use super::keyboard::Keyboard;

pub struct Bus {
    pub ram: Ram,
    pub vram: VRam,
    pub keyboard: Keyboard
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            ram: Ram::new(),
            vram: VRam::new(),
            keyboard: Keyboard::new()
        }
    }
}