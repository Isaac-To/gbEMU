trait RegisterAccess {
    fn get_register_8(&self, reg: Reg8b) -> u8;
    fn set_register_8(&mut self, reg: Reg8b, value: u8);
    fn get_register_16(&self, reg: Reg16b) -> u16;
    fn set_register_16(&mut self, reg: Reg16b, value: u16);
}

enum Reg8b {
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

enum Reg16b {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
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

impl RegisterAccess for CPU {
    fn get_register_8(&self, reg: Reg8b) -> u8 {
        self.registers[reg as usize]
    }
    fn set_register_8(&mut self, reg: Reg8b, value: u8) {
        self.registers[reg as usize] = value;
    }
    fn get_register_16(&self, reg: Reg16b) -> u16 {
        (self.get_register_8(reg.value().0) as u16) << 8 | self.get_register_8(reg.value().1) as u16
    }
    fn set_register_16(&mut self, reg: Reg16b, value: u16) {
        self.registers[reg.value().0 as usize] = (value >> 8) as u8;
        self.registers[reg.value().1 as usize] = value as u8;
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
enum Flag {
    Carry = 4,
    HalfCarry = 5,
    Subtract = 6,
    Zero = 7,
    None = 0,
}

trait FlagRegister {
    fn get_flag(&self) -> Flag;
    fn set_flag(&mut self, flag: Flag);
}

impl FlagRegister for CPU {
    fn get_flag(&self) -> Flag {
        let content = self.get_register_8(Reg8b::F);
        {
            if content >> Flag::Carry as u8 == 1 {
                return Flag::Carry;
            } else if content >> Flag::HalfCarry as u8 == 1 {
                return Flag::HalfCarry;
            } else if content >> Flag::Subtract as u8 == 1 {
                return Flag::Subtract;
            } else if content >> Flag::Zero as u8 == 1 {
                return Flag::Zero;
            } else {
                return Flag::None;
            }
        }
    }
    fn set_flag(&mut self, flag: Flag) {
        self.set_register_8(Reg8b::F, 1 << flag as u8);
    }
}
