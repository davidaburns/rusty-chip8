// Turn off warnings while debugging, removing these
#![allow(dead_code, unused_variables, unused_parens, unused_imports, arithmetic_overflow)]
pub mod chip8;

use chip8::emulator::Emulator;

fn main() {
    let mut emulator: Emulator = Emulator::new();

    emulator.initialize();
    emulator.load_rom("./pong.ch8".to_string());
    emulator.run();
}
