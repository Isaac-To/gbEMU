mod hardware;

use std::fmt::Display;

use crate::hardware::{
    CPU,
    reg::{RegisterAccess, Reg16b, Reg8b},
    mem::MemoryAccess,
    rom::{ROM, ROMAccess},
};

fn main() {
    let mut cpu = CPU::new();
}
