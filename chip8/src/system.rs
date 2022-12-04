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
    pub fn new(game: &str) {
        let mut memory = [0; 4096];
        let cpu = Cpu::new();

        let mut f = File::open(game).expect("Unable to locate ROM");
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).expect("Unable to read ROM data");

        for i in 0..buffer.len() {
            memory[i + cpu.program as usize] = buffer[i];
        }

        System {
            cpu: cpu,
            memory: memory,
            keypad: Keypad::new(),
            display: Display::new(),
        };

        pub static FONTS: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        for i in 0..80 {
            memory[i] = FONTS[i];
        }
    }
}
