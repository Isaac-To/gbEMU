use crate::cpu::CPU;

pub trait MemoryAccess {
    fn read_mem(&self, addr: u16) -> u8;
    fn write_mem(&mut self, addr: u16, value: u8);
}

impl MemoryAccess for CPU {
    fn read_mem(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn write_mem(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }
}