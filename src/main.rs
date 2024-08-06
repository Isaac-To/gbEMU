mod hardware;
use crate::hardware::{
    CPU,
    reg::{RegisterAccess, Reg16b, Reg8b},
    mem::MemoryAccess,
};

fn main() {
    let mut cpu = CPU::new();
}
