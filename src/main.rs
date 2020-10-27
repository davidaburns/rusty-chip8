// Turn off warnings while debugging, removing these
#![allow(
    dead_code,
    unused_variables,
    unused_parens,
    unused_imports,
    unused_mut,
    unused_attributes,
    arithmetic_overflow
)]

pub mod app;
use app::application;

fn main() {
    let app = application::Application::new("ImGUI - Chip8 Emulator");
    app.run();
}
