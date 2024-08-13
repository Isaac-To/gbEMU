pub mod cpu;
pub mod mem;
pub mod rom;

use cpu::reg::{Reg16b, RegisterAccess};

/// System struct
pub struct System {
    /// CPU
    cpu: cpu::CPU,

    /// Memory
    memory: [u8; 8192],

    /// Video memory
    vram: [u8; 8192],

    /// Internal Flags
    _ime: u8,
    _interrupt_iminent: u8,
    _low_power: u8
}

impl System {
    /// Create a new System
    /// All registers are initialized to 0 except SP which is set to 8191 to match the end of memory
    pub fn new() -> System {
        let mut sys = System {
            cpu: cpu::CPU::new(),
            memory: [0; 8192],
            vram: [0; 8192],
            _ime: 0,
            _interrupt_iminent: 0,
            _low_power: 0
        };
        sys.cpu.reg_set_16(&Reg16b::SP, 8191);
        return sys;
    }
}
