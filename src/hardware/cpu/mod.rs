pub mod opcodes;
pub mod reg;

use self::opcodes::Opcode;
use self::reg::*;
use super::super::hardware::mem::Memory;
use super::super::hardware::mem::ROMAccess;
use super::super::hardware::rw::RWAccess;

pub struct CPU {
    /// Registers
    /// PC and SP are 8,9 and 10,11 respectively
    registers: reg::Registers,
    clock_debt: u8,
    mem_ptr: *mut Memory,
}

impl CPU {
    /// Create a new CPU
    /// All registers are initialized to 0
    pub fn new(mem_ptr: &mut Memory) -> CPU {
        CPU {
            registers: reg::Registers::new(),
            clock_debt: 0,
            mem_ptr,
        }
    }
}

impl CPU {
    fn peek_pc(&mut self) -> Opcode {
        unsafe { (*self.mem_ptr).decode(self.registers.read_16(reg::PC)).1 }
    }
    fn pop_pc(&mut self) -> Opcode {
        let (addr, opcode) = unsafe { (*self.mem_ptr).decode(self.registers.read_16(PC)) };
        self.registers.write_16(reg::PC, addr);
        opcode
    }
}

pub trait Execution {
    fn step(&mut self);
    fn exec(&mut self, opcode: &Opcode);
}

impl Execution for CPU {
    fn step(&mut self) {
        if self.clock_debt == 0 {
            // Do something
            let opcode = self.pop_pc();
            self.exec(&opcode);
            println!("{}", opcode);
        }
        if self.clock_debt > 0 {
            self.clock_debt -= 1;
        }
    }
    fn exec(&mut self, opcode: &Opcode) {
        match opcode.mnemonic {
            "nop" => {}
            "ld" => {}
            _ => {
                println!("Instruction not implemented: {}", opcode)
            }
        }
    }
}
