/// Registers are 8-bit and 16-bit values that are used to store data and perform operations.

/// Reg8b is an enum that represents the 8-bit registers of the CPU.
pub enum Reg8b {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    F,
    S,
    P,
    P2,
    C2,
}

/// Implement a method to get the value of the register this is used to index the registers array.
impl Reg8b {
    fn value(&self) -> u8 {
        match self {
            Reg8b::A => 0,
            Reg8b::B => 1,
            Reg8b::C => 2,
            Reg8b::D => 3,
            Reg8b::E => 4,
            Reg8b::H => 5,
            Reg8b::L => 6,
            Reg8b::F => 7,
            Reg8b::S => 8,
            Reg8b::P => 9,
            Reg8b::P2 => 10,
            Reg8b::C2 => 11,
        }
    }
}

/// Reg16b is an enum that represents the 16-bit registers of the CPU.
pub enum Reg16b {
    AF,
    BC,
    DE,
    HL, // Typically used for memory addressing
    SP, // Stack Pointer
    PC, // Program Counter
}

/// Implement a method to get the value of the register this is used to index the registers array.
impl Reg16b {
    fn value(&self) -> (Reg8b, Reg8b) {
        match self {
            Reg16b::AF => (Reg8b::A, Reg8b::F),
            Reg16b::BC => (Reg8b::B, Reg8b::C),
            Reg16b::DE => (Reg8b::D, Reg8b::E),
            Reg16b::HL => (Reg8b::H, Reg8b::L),
            Reg16b::SP => (Reg8b::S, Reg8b::P),
            Reg16b::PC => (Reg8b::P2, Reg8b::C2),
        }
    }
}

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

/// RegisterAccess trait for CPU to allow read write access to registers
pub trait RegisterAccess {
    fn get_8(&self, reg: &Reg8b) -> u8;
    fn set_8(&mut self, reg: &Reg8b, value: u8);
    fn get_16(&self, reg: &Reg16b) -> u16;
    fn set_16(&mut self, reg: &Reg16b, value: u16);
    fn get_flags(&self) -> (u8, u8, u8, u8);
    fn set_flags(&mut self, flags: (u8, u8, u8, u8));
}

pub struct Registers {
    registers: [u8; 12],
}

impl Registers {
    /// Create a new Registers struct
    /// All registers are initialized to 0
    pub fn new() -> Registers {
        let mut reg = Registers { registers: [0; 12] };
        reg.set_16(&Reg16b::PC, 0x100);
        reg
    }
}

/// Implement RegisterAccess for CPU
impl RegisterAccess for Registers {
    /// Read 8-bit value from register
    fn get_8(&self, reg: &Reg8b) -> u8 {
        self.registers[reg.value() as usize]
    }
    /// Write 8-bit value to register
    fn set_8(&mut self, reg: &Reg8b, value: u8) {
        self.registers[reg.value() as usize] = value;
    }
    /// Read 16-bit value from register
    fn get_16(&self, reg: &Reg16b) -> u16 {
        (self.get_8(&reg.value().0) as u16) << 8 | self.get_8(&reg.value().1) as u16
    }
    /// Write 16-bit value to register
    fn set_16(&mut self, reg: &Reg16b, value: u16) {
        self.set_8(&reg.value().0, (value >> 8) as u8);
        self.set_8(&reg.value().1, value as u8);
    }
    /// Read flags from F register
    fn get_flags(&self) -> (u8, u8, u8, u8) {
        let f = self.get_8(&Reg8b::F);
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
        self.set_8(&Reg8b::F, f);
    }
}
