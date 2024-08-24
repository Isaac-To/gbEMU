# This converts the opcode.json file into a python dictionary and outputs it into rust code at src/hardware/cpu/opcodes.rs for use in the CPU
# opcode.json is from https://gbdev.io/gb-opcodes/Opcodes.json

import json

def write_to_rust_file(f, key, type):
    operands = type[key].get("operands")
    while len(operands) < 3:
        operands.append({"name": "NULL", "bytes": 0, "immediate": False, "value": 0})
    cycles = type[key]["cycles"]
    while len(cycles) < 2:
        cycles.append(0)
    op = f'''
        operands: [{
    ",".join([f"""
            Operand {{
                name: "{operand["name"]}",
                bytes: {int(0 if operand.get("bytes") is None else operand.get("bytes"))},
                immediate: {str(operand["immediate"]).lower()},
                value: 0
            }}""" for operand in operands
    ])
    }],'''
    isa_name = f"ISA::{type[key]["mnemonic"].lower()}"
    for operand in operands:
        opname = ""
        if operand["name"] == "NULL":
            break
        if operand["name"] in ["AF", "BC", "DE", "PC"]:
            opname = "r16"
        elif operand["name"] in ["A", "B", "C", "D", "E", "H", "L", "F"]:
            opname = "r8"
        else:
            opname = operand['name'].lower()
        if operand["immediate"]:
            isa_name += f"_{opname}"
        else:
            isa_name += f"_a{opname}"
    f.write(f'''({key}, Opcode {{
        mnemonic: "{type[key]["mnemonic"]}",
        cycles: {type[key]["cycles"]},{op}
        immediate: {str(type[key]["immediate"]).lower()},
        flags: Flags {{
            z: "{type[key]["flags"]["Z"]}",
            n: "{type[key]["flags"]["N"]}",
            h: "{type[key]["flags"]["H"]}",
            c: "{type[key]["flags"]["C"]}",
        }},
        isa_call: {isa_name}
    }}),
    ''')

if __name__ == "__main__":
    json_file = open("resources/opcode/opcodes.json")
    data = json.load(json_file)
    json_file.close()
    f = open("src/hardware/cpu/opcodes.rs", "w")
    unprefixed = data["unprefixed"]
    cbprefixed = data["cbprefixed"]
    f.write("""use super::{
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
    fn to_n8(&self) -> u8;
    fn to_n16(&self) -> u16;
    fn to_e8(&self) -> i8;
    fn to_e16(&self) -> i16;
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
    fn to_n8(&self) -> u8 {
        match self.name {
            "n8" => self.value as u8,
            _ => panic!("Invalid operand"),
        }
    }

    /// Converts an operand to an u16
    fn to_n16(&self) -> u16 {
        match self.name {
            "n16" => self.value,
            _ => panic!("Invalid operand"),
        }
    }

    /// Converts an operand to an i8
    fn to_e8(&self) -> i8 {
        match self.name {
            "e8" => self.value as i8,
            _ => panic!("Invalid operand"),
        }
    }

    /// Converts an operand to an i16
    fn to_e16(&self) -> i16 {
        match self.name {
            "e16" => self.value as i16,
            _ => panic!("Invalid operand"),
        }
    }
}

pub static UNPREFIXED_OPCODES: &[(u8, Opcode)] = &[
\t""")
for key in unprefixed:
    write_to_rust_file(f, key, unprefixed)
f.write("""
];
pub static CB_PREFIXED_OPCODES: &[(u8, Opcode)] = &[
\t""")
for key in cbprefixed:
    write_to_rust_file(f, key, unprefixed)
f.write("""
];""")
