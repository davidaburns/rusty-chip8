// Turn off warnings while debugging, removing these
#![allow(
    dead_code,
    unused_variables,
    unused_parens,
    unused_imports,
    unused_mut,
    arithmetic_overflow
)]

pub mod chip8;
use chip8::emulator::Emulator;

fn main() {
    let mut emulator = Emulator::new();
    emulator.load("./pong.ch8".to_string());
    emulator.run();
}
