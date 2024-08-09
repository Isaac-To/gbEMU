pub mod exec;
pub mod reg;
pub mod mem;
use crate::hardware::{
    reg::{Reg16b, Reg8b, RegisterAccess},
    mem::MemoryAccess,
};

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
        let mut cpu = CPU {
            registers: [0; 12],
            memory: [0; 8192],
            vram: [0; 8192],
            _ime: 0,
            _interrupt_iminent: 0,
            _low_power: 0
        };
        cpu.reg_set_16(&Reg16b::SP, 8191);
        return cpu;
    }
}
