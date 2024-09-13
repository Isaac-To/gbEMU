use super::super::super::hardware::rw::RWAccess;

/// Registers are 8-bit and 16-bit values that are used to store data and perform operations.

/// The 8-bit registers of the CPU.
pub const A: u16 = 0;
pub const B: u16 = 1;
pub const C: u16 = 2;
pub const D: u16 = 3;
pub const E: u16 = 4;
pub const H: u16 = 5;
pub const L: u16 = 6;
pub const F: u16 = 7;
pub const S: u16 = 8;
pub const P: u16 = 9;
pub const P2: u16 = 10;
pub const C2: u16 = 11;

/// The 16-bit registers of the CPU.
pub const AF: u16 = 1;
pub const BC: u16 = 1;
pub const DE: u16 = 2;
pub const HL: u16 = 3; // Typically used for memory addressing
pub const SP: u16 = 4; // Stack Pointer
pub const PC: u16 = 5; // Program Counter

/// Flag Register
/// STORED IN F REGISTER
/// BIT 0-3: Unused
/// BIT 4: Carry Flag
/// BIT 5: Half Carry Flag
/// BIT 6: Subtract Flag
/// BIT 7: Zero Flag

#[derive(Debug, PartialEq)]
/// Flags that can be set in the F register
pub enum Flag {
    Zero = 7,      // Z
    Subtract = 6,  // N
    HalfCarry = 5, // H
    Carry = 4,     // C
}

pub struct Registers {
    registers: [u8; 12],
}

impl Registers {
    /// Create a new Registers struct
    /// All registers are initialized to 0
    pub fn new() -> Registers {
        let mut reg = Registers { registers: [0; 12] };
        reg.write_16(PC, 0x100);
        reg
    }
    /// Read flags from F register
    fn get_flags(&self) -> (u8, u8, u8, u8) {
        let f = self.read_8(F);
        (
            (f >> Flag::Zero as u8) & 1,
            (f >> Flag::Subtract as u8) & 1,
            (f >> Flag::HalfCarry as u8) & 1,
            (f >> Flag::Carry as u8) & 1,
        )
    }
    /// Write flags to F register
    fn set_flags(&mut self, (zero, subtract, half_carry, carry): (u8, u8, u8, u8)) {
        let mut f = 0;
        f |= zero << Flag::Zero as u8;
        f |= subtract << Flag::Subtract as u8;
        f |= half_carry << Flag::HalfCarry as u8;
        f |= carry << Flag::Carry as u8;
        self.write_8(F, f);
    }
}

/// Implement RegisterAccess for CPU
impl RWAccess for Registers {
    /// Read 8-bit value from register
    fn read_8(&self, addr: u16) -> u8 {
        self.registers[addr as usize]
    }
    /// Write 8-bit value to register
    fn write_8(&mut self, addr: u16, val: u8) {
        self.registers[addr as usize] = val;
    }
    /// Read 16-bit value from register
    fn read_16(&self, addr: u16) -> u16 {
        let repr = match addr {
            0 => (0, 7),
            1 => (1, 2),
            2 => (3, 4),
            3 => (5, 6),
            4 => (8, 9),
            5 => (10, 11),
            _ => panic!("Invalid register"),
        };
        (self.read_8(repr.0) as u16) << 8 | self.read_8(repr.1) as u16
    }
    /// Write 16-bit value to register
    fn write_16(&mut self, addr: u16, val: u16) {
        let repr = match addr {
            0 => (0, 7),
            1 => (1, 2),
            2 => (3, 4),
            3 => (5, 6),
            4 => (8, 9),
            5 => (10, 11),
            _ => panic!("Invalid register"),
        };
        self.write_8(repr.0, (val >> 8) as u8);
        self.write_8(repr.1, val as u8);
    }
}
