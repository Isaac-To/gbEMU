use super::{
        isa::ISA,
        reg::{Reg8b, Reg16b}
    };
use super::super::System;

#[derive(Clone, Debug)]
pub struct Opcode {
    pub mnemonic: &'static str,
    pub cycles: [u8; 2],
    pub operands: [Operand; 3],
    pub immediate: bool,
    pub flags: Flags,
    pub isa_call: for<'a> fn(&'a mut System, args: Vec<Operand>)
}

#[derive(Clone, Debug)]
pub struct Operand {
    pub name: &'static str,
    pub bytes: u8,
    pub immediate: bool,
    pub value: u16,
}

#[derive(Clone, Debug)]
pub struct Flags {
    pub z: &'static str,
    pub n: &'static str,
    pub h: &'static str,
    pub c: &'static str,
}

pub fn unprefixed_opcode_get(opcode: &u8) -> Opcode {
    for (key, value) in UNPREFIXED_OPCODES.iter() {
        if opcode == key {
            return value.clone();
        }
    }
    panic!("Opcode not found: {}", opcode);
}

pub fn cb_prefixed_opcode_get(opcode: &u8) -> Opcode {
    for (key, value) in CB_PREFIXED_OPCODES.iter() {
        if opcode == key {
            return value.clone();
        }
    }
    panic!("Opcode not found: {}", opcode);
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut output = self.mnemonic.to_string();
        output.push_str(" ");
        for operand in &self.operands {
            if operand.name == "NULL" {
                break;
            }
            if operand.bytes != 0 {
                if operand.immediate == true {
                    output.push_str(&format!("{} ", format!("0x{:x}", operand.value)));
                } else {
                    output.push_str(&format!("{} ", format!("(0x{:x})", operand.value)));
                }
            } else {
                if operand.immediate == true {
                    output.push_str(&format!("{} ", format!("{}", operand.name)));
                } else {
                    output.push_str(&format!("{} ", format!("({})", operand.name)));
                }
            }
        }
        write!(f, "{}", output)
    }
}

pub trait OperandTypeConversions {
    fn to_reg8b(&self) -> &Reg8b;
    fn to_reg16b(&self) -> &Reg16b;
    fn to_u8(&self) -> u8;
    fn to_u16(&self) -> u16;
    fn to_i8(&self) -> i8;
    fn to_i16(&self) -> i16;
}

impl OperandTypeConversions for Operand {
    /// Helper functions

    /// Converts an operand to a Reg8b
    fn to_reg8b(&self) -> &Reg8b {
        match self.name {
            "A" => &Reg8b::A,
            "B" => &Reg8b::B,
            "C" => &Reg8b::C,
            "D" => &Reg8b::D,
            "E" => &Reg8b::E,
            "H" => &Reg8b::H,
            "L" => &Reg8b::L,
            "F" => &Reg8b::F,
            "S" => &Reg8b::S,
            "P" => &Reg8b::P,
            "P2" => &Reg8b::P2,
            "C2" => &Reg8b::C2,
            _ => panic!("Invalid operand"),
        }
    }

    /// Converts an operand to a Reg16b
    fn to_reg16b(&self) -> &Reg16b {
        match self.name {
            "AF" => &Reg16b::AF,
            "BC" => &Reg16b::BC,
            "DE" => &Reg16b::DE,
            "HL" => &Reg16b::HL,
            "SP" => &Reg16b::SP,
            "PC" => &Reg16b::PC,
            _ => panic!("Invalid operand"),
        }
    }

    /// Converts an operand to an u8
    fn to_u8(&self) -> u8 {
        match self.name {
            "n8" => self.value as u8,
            _ => panic!("Invalid operand"),
        }
    }

    /// Converts an operand to an u16
    fn to_u16(&self) -> u16 {
        match self.name {
            "n16" => self.value,
            _ => panic!("Invalid operand"),
        }
    }

    /// Converts an operand to an i8
    fn to_i8(&self) -> i8 {
        match self.name {
            "e8" => self.value as i8,
            _ => panic!("Invalid operand"),
        }
    }

    /// Converts an operand to an i16
    fn to_i16(&self) -> i16 {
        match self.name {
            "e16" => self.value as i16,
            _ => panic!("Invalid operand"),
        }
    }
}

pub static UNPREFIXED_OPCODES: &[(u8, Opcode)] = &[
	(0x00, Opcode {
        mnemonic: "NOP",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::nop
    }),
    (0x01, Opcode {
        mnemonic: "LD",
        cycles: [12, 0],
        operands: [
            Operand {
                name: "BC",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n16",
                bytes: 2,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r16_n16
    }),
    (0x02, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "BC",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_ar16_r8
    }),
    (0x03, Opcode {
        mnemonic: "INC",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "BC",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::inc_r16
    }),
    (0x04, Opcode {
        mnemonic: "INC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "-",
        },
        isa_call: ISA::inc_r8
    }),
    (0x05, Opcode {
        mnemonic: "DEC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "-",
        },
        isa_call: ISA::dec_r8
    }),
    (0x06, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_n8
    }),
    (0x07, Opcode {
        mnemonic: "RLCA",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "0",
            n: "0",
            h: "0",
            c: "C",
        },
        isa_call: ISA::rlca
    }),
    (0x08, Opcode {
        mnemonic: "LD",
        cycles: [20, 0],
        operands: [
            Operand {
                name: "a16",
                bytes: 2,
                immediate: false,
                value: 0
            },
            Operand {
                name: "SP",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_aa16_sp
    }),
    (0x09, Opcode {
        mnemonic: "ADD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "BC",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::add_hl_r16
    }),
    (0x0A, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "BC",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_ar16
    }),
    (0x0B, Opcode {
        mnemonic: "DEC",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "BC",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::dec_r16
    }),
    (0x0C, Opcode {
        mnemonic: "INC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "-",
        },
        isa_call: ISA::inc_r8
    }),
    (0x0D, Opcode {
        mnemonic: "DEC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "-",
        },
        isa_call: ISA::dec_r8
    }),
    (0x0E, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_n8
    }),
    (0x0F, Opcode {
        mnemonic: "RRCA",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "0",
            n: "0",
            h: "0",
            c: "C",
        },
        isa_call: ISA::rrca
    }),
    (0x10, Opcode {
        mnemonic: "STOP",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::stop_n8
    }),
    (0x11, Opcode {
        mnemonic: "LD",
        cycles: [12, 0],
        operands: [
            Operand {
                name: "DE",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n16",
                bytes: 2,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r16_n16
    }),
    (0x12, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "DE",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_ar16_r8
    }),
    (0x13, Opcode {
        mnemonic: "INC",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "DE",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::inc_r16
    }),
    (0x14, Opcode {
        mnemonic: "INC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "-",
        },
        isa_call: ISA::inc_r8
    }),
    (0x15, Opcode {
        mnemonic: "DEC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "-",
        },
        isa_call: ISA::dec_r8
    }),
    (0x16, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_n8
    }),
    (0x17, Opcode {
        mnemonic: "RLA",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "0",
            n: "0",
            h: "0",
            c: "C",
        },
        isa_call: ISA::rla
    }),
    (0x18, Opcode {
        mnemonic: "JR",
        cycles: [12, 0],
        operands: [
            Operand {
                name: "e8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::jr_e8
    }),
    (0x19, Opcode {
        mnemonic: "ADD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "DE",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::add_hl_r16
    }),
    (0x1A, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "DE",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_ar16
    }),
    (0x1B, Opcode {
        mnemonic: "DEC",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "DE",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::dec_r16
    }),
    (0x1C, Opcode {
        mnemonic: "INC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "-",
        },
        isa_call: ISA::inc_r8
    }),
    (0x1D, Opcode {
        mnemonic: "DEC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "-",
        },
        isa_call: ISA::dec_r8
    }),
    (0x1E, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_n8
    }),
    (0x1F, Opcode {
        mnemonic: "RRA",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "0",
            n: "0",
            h: "0",
            c: "C",
        },
        isa_call: ISA::rra
    }),
    (0x20, Opcode {
        mnemonic: "JR",
        cycles: [12, 8],
        operands: [
            Operand {
                name: "NZ",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "e8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::jr_nz_e8
    }),
    (0x21, Opcode {
        mnemonic: "LD",
        cycles: [12, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n16",
                bytes: 2,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_hl_n16
    }),
    (0x22, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_ahl_r8
    }),
    (0x23, Opcode {
        mnemonic: "INC",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::inc_hl
    }),
    (0x24, Opcode {
        mnemonic: "INC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "-",
        },
        isa_call: ISA::inc_r8
    }),
    (0x25, Opcode {
        mnemonic: "DEC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "-",
        },
        isa_call: ISA::dec_r8
    }),
    (0x26, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_n8
    }),
    (0x27, Opcode {
        mnemonic: "DAA",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "-",
            h: "0",
            c: "C",
        },
        isa_call: ISA::daa
    }),
    (0x28, Opcode {
        mnemonic: "JR",
        cycles: [12, 8],
        operands: [
            Operand {
                name: "Z",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "e8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::jr_z_e8
    }),
    (0x29, Opcode {
        mnemonic: "ADD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::add_hl_hl
    }),
    (0x2A, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_ahl
    }),
    (0x2B, Opcode {
        mnemonic: "DEC",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::dec_hl
    }),
    (0x2C, Opcode {
        mnemonic: "INC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "-",
        },
        isa_call: ISA::inc_r8
    }),
    (0x2D, Opcode {
        mnemonic: "DEC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "-",
        },
        isa_call: ISA::dec_r8
    }),
    (0x2E, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_n8
    }),
    (0x2F, Opcode {
        mnemonic: "CPL",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "1",
            h: "1",
            c: "-",
        },
        isa_call: ISA::cpl
    }),
    (0x30, Opcode {
        mnemonic: "JR",
        cycles: [12, 8],
        operands: [
            Operand {
                name: "NC",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "e8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::jr_nc_e8
    }),
    (0x31, Opcode {
        mnemonic: "LD",
        cycles: [12, 0],
        operands: [
            Operand {
                name: "SP",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n16",
                bytes: 2,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_sp_n16
    }),
    (0x32, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_ahl_r8
    }),
    (0x33, Opcode {
        mnemonic: "INC",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "SP",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::inc_sp
    }),
    (0x34, Opcode {
        mnemonic: "INC",
        cycles: [12, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "-",
        },
        isa_call: ISA::inc_ahl
    }),
    (0x35, Opcode {
        mnemonic: "DEC",
        cycles: [12, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "-",
        },
        isa_call: ISA::dec_ahl
    }),
    (0x36, Opcode {
        mnemonic: "LD",
        cycles: [12, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_ahl_n8
    }),
    (0x37, Opcode {
        mnemonic: "SCF",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "0",
            h: "0",
            c: "1",
        },
        isa_call: ISA::scf
    }),
    (0x38, Opcode {
        mnemonic: "JR",
        cycles: [12, 8],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "e8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::jr_r8_e8
    }),
    (0x39, Opcode {
        mnemonic: "ADD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "SP",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::add_hl_sp
    }),
    (0x3A, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_ahl
    }),
    (0x3B, Opcode {
        mnemonic: "DEC",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "SP",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::dec_sp
    }),
    (0x3C, Opcode {
        mnemonic: "INC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "-",
        },
        isa_call: ISA::inc_r8
    }),
    (0x3D, Opcode {
        mnemonic: "DEC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "-",
        },
        isa_call: ISA::dec_r8
    }),
    (0x3E, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_n8
    }),
    (0x3F, Opcode {
        mnemonic: "CCF",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "0",
            h: "0",
            c: "C",
        },
        isa_call: ISA::ccf
    }),
    (0x40, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x41, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x42, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x43, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x44, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x45, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x46, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_ahl
    }),
    (0x47, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x48, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x49, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x4A, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x4B, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x4C, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x4D, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x4E, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_ahl
    }),
    (0x4F, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x50, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x51, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x52, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x53, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x54, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x55, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x56, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_ahl
    }),
    (0x57, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x58, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x59, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x5A, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x5B, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x5C, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x5D, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x5E, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_ahl
    }),
    (0x5F, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x60, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x61, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x62, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x63, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x64, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x65, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x66, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_ahl
    }),
    (0x67, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x68, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x69, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x6A, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x6B, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x6C, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x6D, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x6E, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_ahl
    }),
    (0x6F, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x70, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_ahl_r8
    }),
    (0x71, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_ahl_r8
    }),
    (0x72, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_ahl_r8
    }),
    (0x73, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_ahl_r8
    }),
    (0x74, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_ahl_r8
    }),
    (0x75, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_ahl_r8
    }),
    (0x76, Opcode {
        mnemonic: "HALT",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::halt
    }),
    (0x77, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_ahl_r8
    }),
    (0x78, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x79, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x7A, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x7B, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x7C, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x7D, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x7E, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_ahl
    }),
    (0x7F, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x80, Opcode {
        mnemonic: "ADD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::add_r8_r8
    }),
    (0x81, Opcode {
        mnemonic: "ADD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::add_r8_r8
    }),
    (0x82, Opcode {
        mnemonic: "ADD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::add_r8_r8
    }),
    (0x83, Opcode {
        mnemonic: "ADD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::add_r8_r8
    }),
    (0x84, Opcode {
        mnemonic: "ADD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::add_r8_r8
    }),
    (0x85, Opcode {
        mnemonic: "ADD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::add_r8_r8
    }),
    (0x86, Opcode {
        mnemonic: "ADD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::add_r8_ahl
    }),
    (0x87, Opcode {
        mnemonic: "ADD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::add_r8_r8
    }),
    (0x88, Opcode {
        mnemonic: "ADC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::adc_r8_r8
    }),
    (0x89, Opcode {
        mnemonic: "ADC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::adc_r8_r8
    }),
    (0x8A, Opcode {
        mnemonic: "ADC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::adc_r8_r8
    }),
    (0x8B, Opcode {
        mnemonic: "ADC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::adc_r8_r8
    }),
    (0x8C, Opcode {
        mnemonic: "ADC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::adc_r8_r8
    }),
    (0x8D, Opcode {
        mnemonic: "ADC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::adc_r8_r8
    }),
    (0x8E, Opcode {
        mnemonic: "ADC",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::adc_r8_ahl
    }),
    (0x8F, Opcode {
        mnemonic: "ADC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::adc_r8_r8
    }),
    (0x90, Opcode {
        mnemonic: "SUB",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sub_r8_r8
    }),
    (0x91, Opcode {
        mnemonic: "SUB",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sub_r8_r8
    }),
    (0x92, Opcode {
        mnemonic: "SUB",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sub_r8_r8
    }),
    (0x93, Opcode {
        mnemonic: "SUB",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sub_r8_r8
    }),
    (0x94, Opcode {
        mnemonic: "SUB",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sub_r8_r8
    }),
    (0x95, Opcode {
        mnemonic: "SUB",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sub_r8_r8
    }),
    (0x96, Opcode {
        mnemonic: "SUB",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sub_r8_ahl
    }),
    (0x97, Opcode {
        mnemonic: "SUB",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "1",
            n: "1",
            h: "0",
            c: "0",
        },
        isa_call: ISA::sub_r8_r8
    }),
    (0x98, Opcode {
        mnemonic: "SBC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sbc_r8_r8
    }),
    (0x99, Opcode {
        mnemonic: "SBC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sbc_r8_r8
    }),
    (0x9A, Opcode {
        mnemonic: "SBC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sbc_r8_r8
    }),
    (0x9B, Opcode {
        mnemonic: "SBC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sbc_r8_r8
    }),
    (0x9C, Opcode {
        mnemonic: "SBC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sbc_r8_r8
    }),
    (0x9D, Opcode {
        mnemonic: "SBC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sbc_r8_r8
    }),
    (0x9E, Opcode {
        mnemonic: "SBC",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sbc_r8_ahl
    }),
    (0x9F, Opcode {
        mnemonic: "SBC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "-",
        },
        isa_call: ISA::sbc_r8_r8
    }),
    (0xA0, Opcode {
        mnemonic: "AND",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "1",
            c: "0",
        },
        isa_call: ISA::and_r8_r8
    }),
    (0xA1, Opcode {
        mnemonic: "AND",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "1",
            c: "0",
        },
        isa_call: ISA::and_r8_r8
    }),
    (0xA2, Opcode {
        mnemonic: "AND",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "1",
            c: "0",
        },
        isa_call: ISA::and_r8_r8
    }),
    (0xA3, Opcode {
        mnemonic: "AND",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "1",
            c: "0",
        },
        isa_call: ISA::and_r8_r8
    }),
    (0xA4, Opcode {
        mnemonic: "AND",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "1",
            c: "0",
        },
        isa_call: ISA::and_r8_r8
    }),
    (0xA5, Opcode {
        mnemonic: "AND",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "1",
            c: "0",
        },
        isa_call: ISA::and_r8_r8
    }),
    (0xA6, Opcode {
        mnemonic: "AND",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "1",
            c: "0",
        },
        isa_call: ISA::and_r8_ahl
    }),
    (0xA7, Opcode {
        mnemonic: "AND",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "1",
            c: "0",
        },
        isa_call: ISA::and_r8_r8
    }),
    (0xA8, Opcode {
        mnemonic: "XOR",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::xor_r8_r8
    }),
    (0xA9, Opcode {
        mnemonic: "XOR",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::xor_r8_r8
    }),
    (0xAA, Opcode {
        mnemonic: "XOR",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::xor_r8_r8
    }),
    (0xAB, Opcode {
        mnemonic: "XOR",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::xor_r8_r8
    }),
    (0xAC, Opcode {
        mnemonic: "XOR",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::xor_r8_r8
    }),
    (0xAD, Opcode {
        mnemonic: "XOR",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::xor_r8_r8
    }),
    (0xAE, Opcode {
        mnemonic: "XOR",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::xor_r8_ahl
    }),
    (0xAF, Opcode {
        mnemonic: "XOR",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "1",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::xor_r8_r8
    }),
    (0xB0, Opcode {
        mnemonic: "OR",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::or_r8_r8
    }),
    (0xB1, Opcode {
        mnemonic: "OR",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::or_r8_r8
    }),
    (0xB2, Opcode {
        mnemonic: "OR",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::or_r8_r8
    }),
    (0xB3, Opcode {
        mnemonic: "OR",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::or_r8_r8
    }),
    (0xB4, Opcode {
        mnemonic: "OR",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::or_r8_r8
    }),
    (0xB5, Opcode {
        mnemonic: "OR",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::or_r8_r8
    }),
    (0xB6, Opcode {
        mnemonic: "OR",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::or_r8_ahl
    }),
    (0xB7, Opcode {
        mnemonic: "OR",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::or_r8_r8
    }),
    (0xB8, Opcode {
        mnemonic: "CP",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::cp_r8_r8
    }),
    (0xB9, Opcode {
        mnemonic: "CP",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::cp_r8_r8
    }),
    (0xBA, Opcode {
        mnemonic: "CP",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::cp_r8_r8
    }),
    (0xBB, Opcode {
        mnemonic: "CP",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::cp_r8_r8
    }),
    (0xBC, Opcode {
        mnemonic: "CP",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::cp_r8_r8
    }),
    (0xBD, Opcode {
        mnemonic: "CP",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::cp_r8_r8
    }),
    (0xBE, Opcode {
        mnemonic: "CP",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::cp_r8_ahl
    }),
    (0xBF, Opcode {
        mnemonic: "CP",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "1",
            n: "1",
            h: "0",
            c: "0",
        },
        isa_call: ISA::cp_r8_r8
    }),
    (0xC0, Opcode {
        mnemonic: "RET",
        cycles: [20, 8],
        operands: [
            Operand {
                name: "NZ",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ret_nz
    }),
    (0xC1, Opcode {
        mnemonic: "POP",
        cycles: [12, 0],
        operands: [
            Operand {
                name: "BC",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::pop_r16
    }),
    (0xC2, Opcode {
        mnemonic: "JP",
        cycles: [16, 12],
        operands: [
            Operand {
                name: "NZ",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "a16",
                bytes: 2,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::jp_nz_a16
    }),
    (0xC3, Opcode {
        mnemonic: "JP",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "a16",
                bytes: 2,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::jp_a16
    }),
    (0xC4, Opcode {
        mnemonic: "CALL",
        cycles: [24, 12],
        operands: [
            Operand {
                name: "NZ",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "a16",
                bytes: 2,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::call_nz_a16
    }),
    (0xC5, Opcode {
        mnemonic: "PUSH",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "BC",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::push_r16
    }),
    (0xC6, Opcode {
        mnemonic: "ADD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::add_r8_n8
    }),
    (0xC7, Opcode {
        mnemonic: "RST",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "$00",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::rst_$00
    }),
    (0xC8, Opcode {
        mnemonic: "RET",
        cycles: [20, 8],
        operands: [
            Operand {
                name: "Z",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ret_z
    }),
    (0xC9, Opcode {
        mnemonic: "RET",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ret
    }),
    (0xCA, Opcode {
        mnemonic: "JP",
        cycles: [16, 12],
        operands: [
            Operand {
                name: "Z",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "a16",
                bytes: 2,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::jp_z_a16
    }),
    (0xCB, Opcode {
        mnemonic: "PREFIX",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::prefix
    }),
    (0xCC, Opcode {
        mnemonic: "CALL",
        cycles: [24, 12],
        operands: [
            Operand {
                name: "Z",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "a16",
                bytes: 2,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::call_z_a16
    }),
    (0xCD, Opcode {
        mnemonic: "CALL",
        cycles: [24, 0],
        operands: [
            Operand {
                name: "a16",
                bytes: 2,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::call_a16
    }),
    (0xCE, Opcode {
        mnemonic: "ADC",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::adc_r8_n8
    }),
    (0xCF, Opcode {
        mnemonic: "RST",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "$08",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::rst_$08
    }),
    (0xD0, Opcode {
        mnemonic: "RET",
        cycles: [20, 8],
        operands: [
            Operand {
                name: "NC",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ret_nc
    }),
    (0xD1, Opcode {
        mnemonic: "POP",
        cycles: [12, 0],
        operands: [
            Operand {
                name: "DE",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::pop_r16
    }),
    (0xD2, Opcode {
        mnemonic: "JP",
        cycles: [16, 12],
        operands: [
            Operand {
                name: "NC",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "a16",
                bytes: 2,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::jp_nc_a16
    }),
    (0xD3, Opcode {
        mnemonic: "ILLEGAL_D3",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::illegal_d3
    }),
    (0xD4, Opcode {
        mnemonic: "CALL",
        cycles: [24, 12],
        operands: [
            Operand {
                name: "NC",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "a16",
                bytes: 2,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::call_nc_a16
    }),
    (0xD5, Opcode {
        mnemonic: "PUSH",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "DE",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::push_r16
    }),
    (0xD6, Opcode {
        mnemonic: "SUB",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sub_r8_n8
    }),
    (0xD7, Opcode {
        mnemonic: "RST",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "$10",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::rst_$10
    }),
    (0xD8, Opcode {
        mnemonic: "RET",
        cycles: [20, 8],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ret_r8
    }),
    (0xD9, Opcode {
        mnemonic: "RETI",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::reti
    }),
    (0xDA, Opcode {
        mnemonic: "JP",
        cycles: [16, 12],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "a16",
                bytes: 2,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::jp_r8_a16
    }),
    (0xDB, Opcode {
        mnemonic: "ILLEGAL_DB",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::illegal_db
    }),
    (0xDC, Opcode {
        mnemonic: "CALL",
        cycles: [24, 12],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "a16",
                bytes: 2,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::call_r8_a16
    }),
    (0xDD, Opcode {
        mnemonic: "ILLEGAL_DD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::illegal_dd
    }),
    (0xDE, Opcode {
        mnemonic: "SBC",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sbc_r8_n8
    }),
    (0xDF, Opcode {
        mnemonic: "RST",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "$18",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::rst_$18
    }),
    (0xE0, Opcode {
        mnemonic: "LDH",
        cycles: [12, 0],
        operands: [
            Operand {
                name: "a8",
                bytes: 1,
                immediate: false,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ldh_aa8_r8
    }),
    (0xE1, Opcode {
        mnemonic: "POP",
        cycles: [12, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::pop_hl
    }),
    (0xE2, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_ar8_r8
    }),
    (0xE3, Opcode {
        mnemonic: "ILLEGAL_E3",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::illegal_e3
    }),
    (0xE4, Opcode {
        mnemonic: "ILLEGAL_E4",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::illegal_e4
    }),
    (0xE5, Opcode {
        mnemonic: "PUSH",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::push_hl
    }),
    (0xE6, Opcode {
        mnemonic: "AND",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "1",
            c: "0",
        },
        isa_call: ISA::and_r8_n8
    }),
    (0xE7, Opcode {
        mnemonic: "RST",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "$20",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::rst_$20
    }),
    (0xE8, Opcode {
        mnemonic: "ADD",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "SP",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "e8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "0",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::add_sp_e8
    }),
    (0xE9, Opcode {
        mnemonic: "JP",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::jp_hl
    }),
    (0xEA, Opcode {
        mnemonic: "LD",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "a16",
                bytes: 2,
                immediate: false,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_aa16_r8
    }),
    (0xEB, Opcode {
        mnemonic: "ILLEGAL_EB",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::illegal_eb
    }),
    (0xEC, Opcode {
        mnemonic: "ILLEGAL_EC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::illegal_ec
    }),
    (0xED, Opcode {
        mnemonic: "ILLEGAL_ED",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::illegal_ed
    }),
    (0xEE, Opcode {
        mnemonic: "XOR",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::xor_r8_n8
    }),
    (0xEF, Opcode {
        mnemonic: "RST",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "$28",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::rst_$28
    }),
    (0xF0, Opcode {
        mnemonic: "LDH",
        cycles: [12, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "a8",
                bytes: 1,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ldh_r8_aa8
    }),
    (0xF1, Opcode {
        mnemonic: "POP",
        cycles: [12, 0],
        operands: [
            Operand {
                name: "AF",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "N",
            h: "H",
            c: "C",
        },
        isa_call: ISA::pop_r16
    }),
    (0xF2, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_ar8
    }),
    (0xF3, Opcode {
        mnemonic: "DI",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::di
    }),
    (0xF4, Opcode {
        mnemonic: "ILLEGAL_F4",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::illegal_f4
    }),
    (0xF5, Opcode {
        mnemonic: "PUSH",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "AF",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::push_r16
    }),
    (0xF6, Opcode {
        mnemonic: "OR",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::or_r8_n8
    }),
    (0xF7, Opcode {
        mnemonic: "RST",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "$30",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::rst_$30
    }),
    (0xF8, Opcode {
        mnemonic: "LD",
        cycles: [12, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "SP",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "e8",
                bytes: 1,
                immediate: true,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "0",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::ld_hl_sp_e8
    }),
    (0xF9, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "SP",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_sp_hl
    }),
    (0xFA, Opcode {
        mnemonic: "LD",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "a16",
                bytes: 2,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_aa16
    }),
    (0xFB, Opcode {
        mnemonic: "EI",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ei
    }),
    (0xFC, Opcode {
        mnemonic: "ILLEGAL_FC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::illegal_fc
    }),
    (0xFD, Opcode {
        mnemonic: "ILLEGAL_FD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::illegal_fd
    }),
    (0xFE, Opcode {
        mnemonic: "CP",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::cp_r8_n8
    }),
    (0xFF, Opcode {
        mnemonic: "RST",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "$38",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::rst_$38
    }),
    
];
pub static CB_PREFIXED_OPCODES: &[(u8, Opcode)] = &[
	(0x00, Opcode {
        mnemonic: "NOP",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::nop
    }),
    (0x01, Opcode {
        mnemonic: "LD",
        cycles: [12, 0],
        operands: [
            Operand {
                name: "BC",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n16",
                bytes: 2,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r16_n16
    }),
    (0x02, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "BC",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_ar16_r8
    }),
    (0x03, Opcode {
        mnemonic: "INC",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "BC",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::inc_r16
    }),
    (0x04, Opcode {
        mnemonic: "INC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "-",
        },
        isa_call: ISA::inc_r8
    }),
    (0x05, Opcode {
        mnemonic: "DEC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "-",
        },
        isa_call: ISA::dec_r8
    }),
    (0x06, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_n8
    }),
    (0x07, Opcode {
        mnemonic: "RLCA",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "0",
            n: "0",
            h: "0",
            c: "C",
        },
        isa_call: ISA::rlca
    }),
    (0x08, Opcode {
        mnemonic: "LD",
        cycles: [20, 0],
        operands: [
            Operand {
                name: "a16",
                bytes: 2,
                immediate: false,
                value: 0
            },
            Operand {
                name: "SP",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_aa16_sp
    }),
    (0x09, Opcode {
        mnemonic: "ADD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "BC",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::add_hl_r16
    }),
    (0x0A, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "BC",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_ar16
    }),
    (0x0B, Opcode {
        mnemonic: "DEC",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "BC",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::dec_r16
    }),
    (0x0C, Opcode {
        mnemonic: "INC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "-",
        },
        isa_call: ISA::inc_r8
    }),
    (0x0D, Opcode {
        mnemonic: "DEC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "-",
        },
        isa_call: ISA::dec_r8
    }),
    (0x0E, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_n8
    }),
    (0x0F, Opcode {
        mnemonic: "RRCA",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "0",
            n: "0",
            h: "0",
            c: "C",
        },
        isa_call: ISA::rrca
    }),
    (0x10, Opcode {
        mnemonic: "STOP",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::stop_n8
    }),
    (0x11, Opcode {
        mnemonic: "LD",
        cycles: [12, 0],
        operands: [
            Operand {
                name: "DE",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n16",
                bytes: 2,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r16_n16
    }),
    (0x12, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "DE",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_ar16_r8
    }),
    (0x13, Opcode {
        mnemonic: "INC",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "DE",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::inc_r16
    }),
    (0x14, Opcode {
        mnemonic: "INC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "-",
        },
        isa_call: ISA::inc_r8
    }),
    (0x15, Opcode {
        mnemonic: "DEC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "-",
        },
        isa_call: ISA::dec_r8
    }),
    (0x16, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_n8
    }),
    (0x17, Opcode {
        mnemonic: "RLA",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "0",
            n: "0",
            h: "0",
            c: "C",
        },
        isa_call: ISA::rla
    }),
    (0x18, Opcode {
        mnemonic: "JR",
        cycles: [12, 0],
        operands: [
            Operand {
                name: "e8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::jr_e8
    }),
    (0x19, Opcode {
        mnemonic: "ADD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "DE",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::add_hl_r16
    }),
    (0x1A, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "DE",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_ar16
    }),
    (0x1B, Opcode {
        mnemonic: "DEC",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "DE",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::dec_r16
    }),
    (0x1C, Opcode {
        mnemonic: "INC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "-",
        },
        isa_call: ISA::inc_r8
    }),
    (0x1D, Opcode {
        mnemonic: "DEC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "-",
        },
        isa_call: ISA::dec_r8
    }),
    (0x1E, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_n8
    }),
    (0x1F, Opcode {
        mnemonic: "RRA",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "0",
            n: "0",
            h: "0",
            c: "C",
        },
        isa_call: ISA::rra
    }),
    (0x20, Opcode {
        mnemonic: "JR",
        cycles: [12, 8],
        operands: [
            Operand {
                name: "NZ",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "e8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::jr_nz_e8
    }),
    (0x21, Opcode {
        mnemonic: "LD",
        cycles: [12, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n16",
                bytes: 2,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_hl_n16
    }),
    (0x22, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_ahl_r8
    }),
    (0x23, Opcode {
        mnemonic: "INC",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::inc_hl
    }),
    (0x24, Opcode {
        mnemonic: "INC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "-",
        },
        isa_call: ISA::inc_r8
    }),
    (0x25, Opcode {
        mnemonic: "DEC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "-",
        },
        isa_call: ISA::dec_r8
    }),
    (0x26, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_n8
    }),
    (0x27, Opcode {
        mnemonic: "DAA",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "-",
            h: "0",
            c: "C",
        },
        isa_call: ISA::daa
    }),
    (0x28, Opcode {
        mnemonic: "JR",
        cycles: [12, 8],
        operands: [
            Operand {
                name: "Z",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "e8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::jr_z_e8
    }),
    (0x29, Opcode {
        mnemonic: "ADD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::add_hl_hl
    }),
    (0x2A, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_ahl
    }),
    (0x2B, Opcode {
        mnemonic: "DEC",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::dec_hl
    }),
    (0x2C, Opcode {
        mnemonic: "INC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "-",
        },
        isa_call: ISA::inc_r8
    }),
    (0x2D, Opcode {
        mnemonic: "DEC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "-",
        },
        isa_call: ISA::dec_r8
    }),
    (0x2E, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_n8
    }),
    (0x2F, Opcode {
        mnemonic: "CPL",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "1",
            h: "1",
            c: "-",
        },
        isa_call: ISA::cpl
    }),
    (0x30, Opcode {
        mnemonic: "JR",
        cycles: [12, 8],
        operands: [
            Operand {
                name: "NC",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "e8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::jr_nc_e8
    }),
    (0x31, Opcode {
        mnemonic: "LD",
        cycles: [12, 0],
        operands: [
            Operand {
                name: "SP",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n16",
                bytes: 2,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_sp_n16
    }),
    (0x32, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_ahl_r8
    }),
    (0x33, Opcode {
        mnemonic: "INC",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "SP",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::inc_sp
    }),
    (0x34, Opcode {
        mnemonic: "INC",
        cycles: [12, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "-",
        },
        isa_call: ISA::inc_ahl
    }),
    (0x35, Opcode {
        mnemonic: "DEC",
        cycles: [12, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "-",
        },
        isa_call: ISA::dec_ahl
    }),
    (0x36, Opcode {
        mnemonic: "LD",
        cycles: [12, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_ahl_n8
    }),
    (0x37, Opcode {
        mnemonic: "SCF",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "0",
            h: "0",
            c: "1",
        },
        isa_call: ISA::scf
    }),
    (0x38, Opcode {
        mnemonic: "JR",
        cycles: [12, 8],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "e8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::jr_r8_e8
    }),
    (0x39, Opcode {
        mnemonic: "ADD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "SP",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::add_hl_sp
    }),
    (0x3A, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_ahl
    }),
    (0x3B, Opcode {
        mnemonic: "DEC",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "SP",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::dec_sp
    }),
    (0x3C, Opcode {
        mnemonic: "INC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "-",
        },
        isa_call: ISA::inc_r8
    }),
    (0x3D, Opcode {
        mnemonic: "DEC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "-",
        },
        isa_call: ISA::dec_r8
    }),
    (0x3E, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_n8
    }),
    (0x3F, Opcode {
        mnemonic: "CCF",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "0",
            h: "0",
            c: "C",
        },
        isa_call: ISA::ccf
    }),
    (0x40, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x41, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x42, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x43, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x44, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x45, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x46, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_ahl
    }),
    (0x47, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x48, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x49, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x4A, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x4B, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x4C, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x4D, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x4E, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_ahl
    }),
    (0x4F, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x50, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x51, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x52, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x53, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x54, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x55, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x56, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_ahl
    }),
    (0x57, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x58, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x59, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x5A, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x5B, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x5C, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x5D, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x5E, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_ahl
    }),
    (0x5F, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x60, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x61, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x62, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x63, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x64, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x65, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x66, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_ahl
    }),
    (0x67, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x68, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x69, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x6A, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x6B, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x6C, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x6D, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x6E, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_ahl
    }),
    (0x6F, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x70, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_ahl_r8
    }),
    (0x71, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_ahl_r8
    }),
    (0x72, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_ahl_r8
    }),
    (0x73, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_ahl_r8
    }),
    (0x74, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_ahl_r8
    }),
    (0x75, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_ahl_r8
    }),
    (0x76, Opcode {
        mnemonic: "HALT",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::halt
    }),
    (0x77, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_ahl_r8
    }),
    (0x78, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x79, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x7A, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x7B, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x7C, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x7D, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x7E, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_ahl
    }),
    (0x7F, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_r8
    }),
    (0x80, Opcode {
        mnemonic: "ADD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::add_r8_r8
    }),
    (0x81, Opcode {
        mnemonic: "ADD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::add_r8_r8
    }),
    (0x82, Opcode {
        mnemonic: "ADD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::add_r8_r8
    }),
    (0x83, Opcode {
        mnemonic: "ADD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::add_r8_r8
    }),
    (0x84, Opcode {
        mnemonic: "ADD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::add_r8_r8
    }),
    (0x85, Opcode {
        mnemonic: "ADD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::add_r8_r8
    }),
    (0x86, Opcode {
        mnemonic: "ADD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::add_r8_ahl
    }),
    (0x87, Opcode {
        mnemonic: "ADD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::add_r8_r8
    }),
    (0x88, Opcode {
        mnemonic: "ADC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::adc_r8_r8
    }),
    (0x89, Opcode {
        mnemonic: "ADC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::adc_r8_r8
    }),
    (0x8A, Opcode {
        mnemonic: "ADC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::adc_r8_r8
    }),
    (0x8B, Opcode {
        mnemonic: "ADC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::adc_r8_r8
    }),
    (0x8C, Opcode {
        mnemonic: "ADC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::adc_r8_r8
    }),
    (0x8D, Opcode {
        mnemonic: "ADC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::adc_r8_r8
    }),
    (0x8E, Opcode {
        mnemonic: "ADC",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::adc_r8_ahl
    }),
    (0x8F, Opcode {
        mnemonic: "ADC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::adc_r8_r8
    }),
    (0x90, Opcode {
        mnemonic: "SUB",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sub_r8_r8
    }),
    (0x91, Opcode {
        mnemonic: "SUB",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sub_r8_r8
    }),
    (0x92, Opcode {
        mnemonic: "SUB",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sub_r8_r8
    }),
    (0x93, Opcode {
        mnemonic: "SUB",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sub_r8_r8
    }),
    (0x94, Opcode {
        mnemonic: "SUB",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sub_r8_r8
    }),
    (0x95, Opcode {
        mnemonic: "SUB",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sub_r8_r8
    }),
    (0x96, Opcode {
        mnemonic: "SUB",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sub_r8_ahl
    }),
    (0x97, Opcode {
        mnemonic: "SUB",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "1",
            n: "1",
            h: "0",
            c: "0",
        },
        isa_call: ISA::sub_r8_r8
    }),
    (0x98, Opcode {
        mnemonic: "SBC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sbc_r8_r8
    }),
    (0x99, Opcode {
        mnemonic: "SBC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sbc_r8_r8
    }),
    (0x9A, Opcode {
        mnemonic: "SBC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sbc_r8_r8
    }),
    (0x9B, Opcode {
        mnemonic: "SBC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sbc_r8_r8
    }),
    (0x9C, Opcode {
        mnemonic: "SBC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sbc_r8_r8
    }),
    (0x9D, Opcode {
        mnemonic: "SBC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sbc_r8_r8
    }),
    (0x9E, Opcode {
        mnemonic: "SBC",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sbc_r8_ahl
    }),
    (0x9F, Opcode {
        mnemonic: "SBC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "-",
        },
        isa_call: ISA::sbc_r8_r8
    }),
    (0xA0, Opcode {
        mnemonic: "AND",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "1",
            c: "0",
        },
        isa_call: ISA::and_r8_r8
    }),
    (0xA1, Opcode {
        mnemonic: "AND",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "1",
            c: "0",
        },
        isa_call: ISA::and_r8_r8
    }),
    (0xA2, Opcode {
        mnemonic: "AND",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "1",
            c: "0",
        },
        isa_call: ISA::and_r8_r8
    }),
    (0xA3, Opcode {
        mnemonic: "AND",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "1",
            c: "0",
        },
        isa_call: ISA::and_r8_r8
    }),
    (0xA4, Opcode {
        mnemonic: "AND",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "1",
            c: "0",
        },
        isa_call: ISA::and_r8_r8
    }),
    (0xA5, Opcode {
        mnemonic: "AND",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "1",
            c: "0",
        },
        isa_call: ISA::and_r8_r8
    }),
    (0xA6, Opcode {
        mnemonic: "AND",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "1",
            c: "0",
        },
        isa_call: ISA::and_r8_ahl
    }),
    (0xA7, Opcode {
        mnemonic: "AND",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "1",
            c: "0",
        },
        isa_call: ISA::and_r8_r8
    }),
    (0xA8, Opcode {
        mnemonic: "XOR",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::xor_r8_r8
    }),
    (0xA9, Opcode {
        mnemonic: "XOR",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::xor_r8_r8
    }),
    (0xAA, Opcode {
        mnemonic: "XOR",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::xor_r8_r8
    }),
    (0xAB, Opcode {
        mnemonic: "XOR",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::xor_r8_r8
    }),
    (0xAC, Opcode {
        mnemonic: "XOR",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::xor_r8_r8
    }),
    (0xAD, Opcode {
        mnemonic: "XOR",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::xor_r8_r8
    }),
    (0xAE, Opcode {
        mnemonic: "XOR",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::xor_r8_ahl
    }),
    (0xAF, Opcode {
        mnemonic: "XOR",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "1",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::xor_r8_r8
    }),
    (0xB0, Opcode {
        mnemonic: "OR",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::or_r8_r8
    }),
    (0xB1, Opcode {
        mnemonic: "OR",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::or_r8_r8
    }),
    (0xB2, Opcode {
        mnemonic: "OR",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::or_r8_r8
    }),
    (0xB3, Opcode {
        mnemonic: "OR",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::or_r8_r8
    }),
    (0xB4, Opcode {
        mnemonic: "OR",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::or_r8_r8
    }),
    (0xB5, Opcode {
        mnemonic: "OR",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::or_r8_r8
    }),
    (0xB6, Opcode {
        mnemonic: "OR",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::or_r8_ahl
    }),
    (0xB7, Opcode {
        mnemonic: "OR",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::or_r8_r8
    }),
    (0xB8, Opcode {
        mnemonic: "CP",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "B",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::cp_r8_r8
    }),
    (0xB9, Opcode {
        mnemonic: "CP",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::cp_r8_r8
    }),
    (0xBA, Opcode {
        mnemonic: "CP",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "D",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::cp_r8_r8
    }),
    (0xBB, Opcode {
        mnemonic: "CP",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "E",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::cp_r8_r8
    }),
    (0xBC, Opcode {
        mnemonic: "CP",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "H",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::cp_r8_r8
    }),
    (0xBD, Opcode {
        mnemonic: "CP",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "L",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::cp_r8_r8
    }),
    (0xBE, Opcode {
        mnemonic: "CP",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::cp_r8_ahl
    }),
    (0xBF, Opcode {
        mnemonic: "CP",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "1",
            n: "1",
            h: "0",
            c: "0",
        },
        isa_call: ISA::cp_r8_r8
    }),
    (0xC0, Opcode {
        mnemonic: "RET",
        cycles: [20, 8],
        operands: [
            Operand {
                name: "NZ",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ret_nz
    }),
    (0xC1, Opcode {
        mnemonic: "POP",
        cycles: [12, 0],
        operands: [
            Operand {
                name: "BC",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::pop_r16
    }),
    (0xC2, Opcode {
        mnemonic: "JP",
        cycles: [16, 12],
        operands: [
            Operand {
                name: "NZ",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "a16",
                bytes: 2,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::jp_nz_a16
    }),
    (0xC3, Opcode {
        mnemonic: "JP",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "a16",
                bytes: 2,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::jp_a16
    }),
    (0xC4, Opcode {
        mnemonic: "CALL",
        cycles: [24, 12],
        operands: [
            Operand {
                name: "NZ",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "a16",
                bytes: 2,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::call_nz_a16
    }),
    (0xC5, Opcode {
        mnemonic: "PUSH",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "BC",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::push_r16
    }),
    (0xC6, Opcode {
        mnemonic: "ADD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::add_r8_n8
    }),
    (0xC7, Opcode {
        mnemonic: "RST",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "$00",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::rst_$00
    }),
    (0xC8, Opcode {
        mnemonic: "RET",
        cycles: [20, 8],
        operands: [
            Operand {
                name: "Z",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ret_z
    }),
    (0xC9, Opcode {
        mnemonic: "RET",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ret
    }),
    (0xCA, Opcode {
        mnemonic: "JP",
        cycles: [16, 12],
        operands: [
            Operand {
                name: "Z",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "a16",
                bytes: 2,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::jp_z_a16
    }),
    (0xCB, Opcode {
        mnemonic: "PREFIX",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::prefix
    }),
    (0xCC, Opcode {
        mnemonic: "CALL",
        cycles: [24, 12],
        operands: [
            Operand {
                name: "Z",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "a16",
                bytes: 2,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::call_z_a16
    }),
    (0xCD, Opcode {
        mnemonic: "CALL",
        cycles: [24, 0],
        operands: [
            Operand {
                name: "a16",
                bytes: 2,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::call_a16
    }),
    (0xCE, Opcode {
        mnemonic: "ADC",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::adc_r8_n8
    }),
    (0xCF, Opcode {
        mnemonic: "RST",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "$08",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::rst_$08
    }),
    (0xD0, Opcode {
        mnemonic: "RET",
        cycles: [20, 8],
        operands: [
            Operand {
                name: "NC",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ret_nc
    }),
    (0xD1, Opcode {
        mnemonic: "POP",
        cycles: [12, 0],
        operands: [
            Operand {
                name: "DE",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::pop_r16
    }),
    (0xD2, Opcode {
        mnemonic: "JP",
        cycles: [16, 12],
        operands: [
            Operand {
                name: "NC",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "a16",
                bytes: 2,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::jp_nc_a16
    }),
    (0xD3, Opcode {
        mnemonic: "ILLEGAL_D3",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::illegal_d3
    }),
    (0xD4, Opcode {
        mnemonic: "CALL",
        cycles: [24, 12],
        operands: [
            Operand {
                name: "NC",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "a16",
                bytes: 2,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::call_nc_a16
    }),
    (0xD5, Opcode {
        mnemonic: "PUSH",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "DE",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::push_r16
    }),
    (0xD6, Opcode {
        mnemonic: "SUB",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sub_r8_n8
    }),
    (0xD7, Opcode {
        mnemonic: "RST",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "$10",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::rst_$10
    }),
    (0xD8, Opcode {
        mnemonic: "RET",
        cycles: [20, 8],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ret_r8
    }),
    (0xD9, Opcode {
        mnemonic: "RETI",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::reti
    }),
    (0xDA, Opcode {
        mnemonic: "JP",
        cycles: [16, 12],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "a16",
                bytes: 2,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::jp_r8_a16
    }),
    (0xDB, Opcode {
        mnemonic: "ILLEGAL_DB",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::illegal_db
    }),
    (0xDC, Opcode {
        mnemonic: "CALL",
        cycles: [24, 12],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "a16",
                bytes: 2,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::call_r8_a16
    }),
    (0xDD, Opcode {
        mnemonic: "ILLEGAL_DD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::illegal_dd
    }),
    (0xDE, Opcode {
        mnemonic: "SBC",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::sbc_r8_n8
    }),
    (0xDF, Opcode {
        mnemonic: "RST",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "$18",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::rst_$18
    }),
    (0xE0, Opcode {
        mnemonic: "LDH",
        cycles: [12, 0],
        operands: [
            Operand {
                name: "a8",
                bytes: 1,
                immediate: false,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ldh_aa8_r8
    }),
    (0xE1, Opcode {
        mnemonic: "POP",
        cycles: [12, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::pop_hl
    }),
    (0xE2, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "C",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_ar8_r8
    }),
    (0xE3, Opcode {
        mnemonic: "ILLEGAL_E3",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::illegal_e3
    }),
    (0xE4, Opcode {
        mnemonic: "ILLEGAL_E4",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::illegal_e4
    }),
    (0xE5, Opcode {
        mnemonic: "PUSH",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::push_hl
    }),
    (0xE6, Opcode {
        mnemonic: "AND",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "1",
            c: "0",
        },
        isa_call: ISA::and_r8_n8
    }),
    (0xE7, Opcode {
        mnemonic: "RST",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "$20",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::rst_$20
    }),
    (0xE8, Opcode {
        mnemonic: "ADD",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "SP",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "e8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "0",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::add_sp_e8
    }),
    (0xE9, Opcode {
        mnemonic: "JP",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::jp_hl
    }),
    (0xEA, Opcode {
        mnemonic: "LD",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "a16",
                bytes: 2,
                immediate: false,
                value: 0
            },
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_aa16_r8
    }),
    (0xEB, Opcode {
        mnemonic: "ILLEGAL_EB",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::illegal_eb
    }),
    (0xEC, Opcode {
        mnemonic: "ILLEGAL_EC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::illegal_ec
    }),
    (0xED, Opcode {
        mnemonic: "ILLEGAL_ED",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::illegal_ed
    }),
    (0xEE, Opcode {
        mnemonic: "XOR",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::xor_r8_n8
    }),
    (0xEF, Opcode {
        mnemonic: "RST",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "$28",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::rst_$28
    }),
    (0xF0, Opcode {
        mnemonic: "LDH",
        cycles: [12, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "a8",
                bytes: 1,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ldh_r8_aa8
    }),
    (0xF1, Opcode {
        mnemonic: "POP",
        cycles: [12, 0],
        operands: [
            Operand {
                name: "AF",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "N",
            h: "H",
            c: "C",
        },
        isa_call: ISA::pop_r16
    }),
    (0xF2, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "C",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_ar8
    }),
    (0xF3, Opcode {
        mnemonic: "DI",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::di
    }),
    (0xF4, Opcode {
        mnemonic: "ILLEGAL_F4",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::illegal_f4
    }),
    (0xF5, Opcode {
        mnemonic: "PUSH",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "AF",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::push_r16
    }),
    (0xF6, Opcode {
        mnemonic: "OR",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        },
        isa_call: ISA::or_r8_n8
    }),
    (0xF7, Opcode {
        mnemonic: "RST",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "$30",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::rst_$30
    }),
    (0xF8, Opcode {
        mnemonic: "LD",
        cycles: [12, 0],
        operands: [
            Operand {
                name: "HL",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "SP",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "e8",
                bytes: 1,
                immediate: true,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "0",
            n: "0",
            h: "H",
            c: "C",
        },
        isa_call: ISA::ld_hl_sp_e8
    }),
    (0xF9, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "SP",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "HL",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_sp_hl
    }),
    (0xFA, Opcode {
        mnemonic: "LD",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "a16",
                bytes: 2,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ld_r8_aa16
    }),
    (0xFB, Opcode {
        mnemonic: "EI",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::ei
    }),
    (0xFC, Opcode {
        mnemonic: "ILLEGAL_FC",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::illegal_fc
    }),
    (0xFD, Opcode {
        mnemonic: "ILLEGAL_FD",
        cycles: [4, 0],
        operands: [
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::illegal_fd
    }),
    (0xFE, Opcode {
        mnemonic: "CP",
        cycles: [8, 0],
        operands: [
            Operand {
                name: "A",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "n8",
                bytes: 1,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        },
        isa_call: ISA::cp_r8_n8
    }),
    (0xFF, Opcode {
        mnemonic: "RST",
        cycles: [16, 0],
        operands: [
            Operand {
                name: "$38",
                bytes: 0,
                immediate: true,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            },
            Operand {
                name: "NULL",
                bytes: 0,
                immediate: false,
                value: 0
            }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        },
        isa_call: ISA::rst_$38
    }),
    
];