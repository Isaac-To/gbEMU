use crate::hardware::CPU;

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

pub enum Reg16b {
    AF,
    BC,
    DE,
    HL, // Typically used for memory addressing
    SP, // Stack Pointer
    PC, // Program Counter
}

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

// Flag Register
// STORED IN F REGISTER
// BIT 0-3: Unused
// BIT 4: Carry Flag
// BIT 5: Half Carry Flag
// BIT 6: Subtract Flag
// BIT 7: Zero Flag

#[derive(Debug, PartialEq)]
pub enum Flag {
    Zero = 7, // Z
    Subtract = 6, // N
    HalfCarry = 5, // H
    Carry = 4, // C
}

pub trait RegisterAccess {
    fn reg_get_8(&self, reg: Reg8b) -> u8;
    fn reg_set_8(&mut self, reg: Reg8b, value: u8);
    fn reg_get_16(&self, reg: Reg16b) -> u16;
    fn reg_set_16(&mut self, reg: Reg16b, value: u16);
    fn reg_get_flags(&self) -> (bool, bool, bool, bool);
    fn reg_set_flags(&mut self, zero: bool, subtract: bool, half_carry: bool, carry: bool);
}

impl RegisterAccess for CPU {
    fn reg_get_8(&self, reg: Reg8b) -> u8 {
        self.registers[reg as usize]
    }
    fn reg_set_8(&mut self, reg: Reg8b, value: u8) {
        self.registers[reg as usize] = value;
    }
    fn reg_get_16(&self, reg: Reg16b) -> u16 {
        (self.reg_get_8(reg.value().0) as u16) << 8 | self.reg_get_8(reg.value().1) as u16
    }
    fn reg_set_16(&mut self, reg: Reg16b, value: u16) {
        self.registers[reg.value().0 as usize] = (value >> 8) as u8;
        self.registers[reg.value().1 as usize] = value as u8;
    }
    fn reg_get_flags(&self)-> (bool, bool, bool, bool) {
        let f = self.reg_get_8(Reg8b::F);
        (
            f & (1 << Flag::Zero as u8) != 0,
            f & (1 << Flag::Subtract as u8) != 0,
            f & (1 << Flag::HalfCarry as u8) != 0,
            f & (1 << Flag::Carry as u8) != 0,
        )
    }
    fn reg_set_flags(&mut self, zero: bool, subtract: bool, half_carry: bool, carry: bool) {
        let mut f = 0;
        if zero {
            f |= 1 << Flag::Zero as u8;
        }
        if subtract {
            f |= 1 << Flag::Subtract as u8;
        }
        if half_carry {
            f |= 1 << Flag::HalfCarry as u8;
        }
        if carry {
            f |= 1 << Flag::Carry as u8;
        }
        self.reg_set_8(Reg8b::F, f);
    }
}
