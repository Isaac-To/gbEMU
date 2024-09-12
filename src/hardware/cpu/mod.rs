pub mod opcodes;
pub mod reg;
use self::{opcodes::Opcode, reg::RegisterAccess};

use super::mem::{self, MemoryAccess, ROMAccess};

pub struct CPU {
    /// Registers
    /// PC and SP are 8,9 and 10,11 respectively
    registers: reg::Registers,
    clock_debt: u8,
    mem_ptr: *mut mem::Memory,
}

impl CPU {
    /// Create a new CPU
    /// All registers are initialized to 0
    pub fn new(mem_ptr: &mut mem::Memory) -> CPU {
        CPU {
            registers: reg::Registers::new(),
            clock_debt: 0,
            mem_ptr,
        }
    }
}

pub trait Execution {
    fn exec(&mut self);
}

impl Execution for CPU {
    fn exec(&mut self) {
        if self.clock_debt == 0 {
            // Do something
            println!("{}", self.pop_pc());
        }
        if self.clock_debt > 0 {
            self.clock_debt -= 1;
        }
    }
}

impl CPU {
    fn peek_pc(&mut self) -> Opcode {
        unsafe {
            (*self.mem_ptr)
                .decode(self.registers.get_16(&reg::Reg16b::PC))
                .1
        }
    }
    fn pop_pc(&mut self) -> Opcode {
        let (addr, opcode) =
            unsafe { (*self.mem_ptr).decode(self.registers.get_16(&reg::Reg16b::PC)) };
        self.registers.set_16(&reg::Reg16b::PC, addr);
        opcode
    }
}
