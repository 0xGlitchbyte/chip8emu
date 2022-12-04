mod cpu;
mod display;
mod keypad;

mod system;
use system::System;

//use eframe::{epi::App, run_native};

struct Chip8;

impl App for Chip8 {}

fn main() {
    let mut system = System::new();

    //run_native("chip8",, native_options);

    loop {
        system.cycle();
    }
}
