pub struct Cpu {
    pub program: u16, // program counter
    stack: [u16; 16], // stack storage
    pointer: u8,      // stack pointer
    delay: u8,        // delay timer
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            program: 0x200,
            stack: [0; 16],
            pointer: 0,
            delay: 0,
        }
    }
}
