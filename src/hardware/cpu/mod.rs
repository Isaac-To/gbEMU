pub mod opcodes;
pub mod reg;

use super::System;

pub struct CPU {
    /// Registers
    /// PC and SP are 8,9 and 10,11 respectively
    registers: [u8; 12],
}

impl CPU {
    /// Create a new CPU
    /// All registers are initialized to 0
    pub fn new() -> CPU {
        CPU { registers: [0; 12] }
    }
}

pub trait Execution {
    fn exec(&mut self);
}

impl Execution for System {
    fn exec(&mut self) {}
}
