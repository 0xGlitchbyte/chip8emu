use cpu::Cpu;
use display::Display;
use keypad::Keypad;

pub struct System {
    cpu: Cpu,
    memory: [u8; 4096],
    keypad: Keypad,
    display: Display,
}

impl System {
    pub fn new() -> System {
        System {
            cpu: Cpu::new(),
            memory: [0; 4096],
            keypad: Keypad::new(),
            display: Display::new(),
        }
    }

    pub fn cycle(&mut self) {
        //! TODO
    }
}
