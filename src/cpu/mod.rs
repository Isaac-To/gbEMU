mod exec;
mod reg;

pub struct CPU {
    // Registers
    // PC and SP are 8,9 and 10,11 respectively
    registers: [u8; 12],

    // Memory
    memory: [u8; 8192],

    // Video memory
    vram: [u8; 8192],
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: [0; 12],
            memory: [0; 8192],
            vram: [0; 8192],
        }
    }
}
