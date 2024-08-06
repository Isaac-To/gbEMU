pub mod exec;
pub mod reg;
pub mod mem;

pub(crate) struct CPU {
    // Registers
    // PC and SP are 8,9 and 10,11 respectively
    registers: [u8; 12],

    // Memory
    memory: [u8; 8192],

    // Video memory
    vram: [u8; 8192],

    // Internal Flags
    _ime: u8,
    _interrupt_iminent: u8,
    _low_power: u8
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: [0; 12],
            memory: [0; 8192],
            vram: [0; 8192],
            _ime: 0,
            _interrupt_iminent: 0,
            _low_power: 0
        }
    }
}
