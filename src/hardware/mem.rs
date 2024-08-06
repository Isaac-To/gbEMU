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
}

impl MemoryAccess for CPU {
    fn mem_read_8(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }
    fn mem_read_16(&self, addr: u16) -> u16 {
        let low = self.mem_read_8(addr);
        let high = self.mem_read_8(addr + 1);
        ((high as u16) << 8) | low as u16
    }
    fn mem_write_8(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }
    fn mem_write_16(&mut self, addr: u16, value: u16) {
        let low = value as u8;
        let high = (value >> 8) as u8;
        self.mem_write_8(addr, low);
        self.mem_write_8(addr + 1, high);
    }
    fn mem_stack_push_8 (&mut self, value: u8) {
        let sp = self.reg_get_16(Reg16b::SP);
        self.mem_write_8(sp, value);
        self.reg_set_16(Reg16b::SP, sp - 1);
    }
    fn mem_stack_push_16(&mut self, value: u16) {
        let sp = self.reg_get_16(Reg16b::SP);
        self.mem_write_16(sp, value);
        self.reg_set_16(Reg16b::SP, sp - 2);
    }
    fn mem_stack_pop_8(&mut self) -> u8 {
        let sp = self.reg_get_16(Reg16b::SP);
        let value = self.mem_read_8(sp + 1);
        self.reg_set_16(Reg16b::SP, sp + 1);
        value
    }
    fn mem_stack_pop_16(&mut self) -> u16 {
        let sp = self.reg_get_16(Reg16b::SP);
        let value = self.mem_read_16(sp + 1);
        self.reg_set_16(Reg16b::SP, sp + 2);
        value
    }
}