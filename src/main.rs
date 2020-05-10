// Turn off warnings while debugging, removing these
#![allow(dead_code, unused_variables, unused_parens, unused_imports, arithmetic_overflow)]
pub mod chip8;

fn main() {
    let mut emulator = chip8::emulator::Emulator::new();
    emulator.load_rom("./pong.ch8".to_string());
    emulator.run();
}
