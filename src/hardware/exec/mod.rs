pub mod isa;

use crate::hardware::{
    exec::isa::ISA,
    mem::MemoryAccess,
    reg::{Reg16b, Reg8b, RegisterAccess},
    CPU,
};

pub trait Execution {
    fn exec(&mut self);
}

impl Execution for CPU {
    fn exec(&mut self) {}
}
