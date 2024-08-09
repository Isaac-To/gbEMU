use crate::hardware::{
    CPU,
    reg::{Reg16b, Reg8b, RegisterAccess},
};

pub trait MemoryAccess {
    fn mem_read_8(&self, addr: u16) -> u8;
    fn mem_read_16(&self, addr: u16) -> u16;
    fn mem_write_8(&mut self, addr: u16, value: u8);
    fn mem_write_16(&mut self, addr: u16, value: u16);
    fn mem_stack_push_8(&mut self, value: u8);
    fn mem_stack_push_16(&mut self, value: u16);
    fn mem_stack_pop_8(&mut self) -> u8;
    fn mem_stack_pop_16(&mut self) -> u16;
    fn mem_pc_read_8(&mut self) -> u8;
}

impl MemoryAccess for CPU {
    fn mem_read_8(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }
    fn mem_read_16(&self, addr: u16) -> u16 {
        self.mem_read_8(addr) as u16 | (self.mem_read_8(addr + 1) as u16) << 8
    }
    fn mem_write_8(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }
    fn mem_write_16(&mut self, addr: u16, value: u16) {
        self.mem_write_8(addr, value as u8);
        self.mem_write_8(addr + 1, (value >> 8) as u8);
    }
    fn mem_stack_push_8 (&mut self, value: u8) {
        let sp = self.reg_get_16(&Reg16b::SP);
        self.mem_write_8(sp - 1, value);
        self.reg_set_16(&Reg16b::SP, sp - 1);
    }
    fn mem_stack_push_16(&mut self, value: u16) {
        self.mem_stack_push_8((value >> 8) as u8);
        self.mem_stack_push_8(value as u8);
    }
    fn mem_stack_pop_8(&mut self) -> u8 {
        let sp = self.reg_get_16(&Reg16b::SP);
        let value = self.mem_read_8(sp);
        self.reg_set_16(&Reg16b::SP, sp + 1);
        value
    }
    fn mem_stack_pop_16(&mut self) -> u16 {
        self.mem_stack_pop_8() as u16 | (self.mem_stack_pop_8() as u16) << 8
    }
    fn mem_pc_read_8(&mut self) -> u8 {
        let pc = self.reg_get_16(&Reg16b::PC);
        self.reg_set_16(&Reg16b::PC, pc.wrapping_add(1));
        self.mem_read_8(pc)
    }
}