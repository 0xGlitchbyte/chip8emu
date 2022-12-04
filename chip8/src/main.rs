mod cpu;
mod display;
mod keypad;

mod system;
use system::System;

fn main() {
    let mut system = System::new();

    loop {
        system.cycle();
    }
}
