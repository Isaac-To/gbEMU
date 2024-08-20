#[derive(Clone, Debug)]
    pub struct Opcode {
    pub mnemonic: &'static str,
    pub cycles: [u8; 2],
    pub operands: [Operand; 3],
    pub immediate: bool,
    pub flags: Flags,
}


#[derive(Clone, Debug)]
pub struct Operand {
    pub name: &'static str,
    pub bytes: u8,
    pub immediate: bool,
}


#[derive(Clone, Debug)]
pub struct Flags {
    pub z: &'static str,
    pub n: &'static str,
    pub h: &'static str,
    pub c: &'static str,
}

pub static OPCODES: &[(u8, Opcode)] = &[ 
(0x00, Opcode {
        mnemonic: "NOP",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x01, Opcode {
        mnemonic: "LD",
        cycles: [12, 0],
        operands: [
        Operand {
            name: "BC",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "n16",
            bytes: 2,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x02, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "BC",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x03, Opcode {
        mnemonic: "INC",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "BC",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x04, Opcode {
        mnemonic: "INC",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "B",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "-",
        }
    }),
    (0x05, Opcode {
        mnemonic: "DEC",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "B",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "-",
        }
    }),
    (0x06, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "B",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "n8",
            bytes: 1,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x07, Opcode {
        mnemonic: "RLCA",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "0",
            n: "0",
            h: "0",
            c: "C",
        }
    }),
    (0x08, Opcode {
        mnemonic: "LD",
        cycles: [20, 0],
        operands: [
        Operand {
            name: "a16",
            bytes: 2,
            immediate: false
        },
        Operand {
            name: "SP",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x09, Opcode {
        mnemonic: "ADD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "HL",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "BC",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "0",
            h: "H",
            c: "C",
        }
    }),
    (0x0A, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "BC",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x0B, Opcode {
        mnemonic: "DEC",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "BC",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x0C, Opcode {
        mnemonic: "INC",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "-",
        }
    }),
    (0x0D, Opcode {
        mnemonic: "DEC",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "-",
        }
    }),
    (0x0E, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "n8",
            bytes: 1,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x0F, Opcode {
        mnemonic: "RRCA",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "0",
            n: "0",
            h: "0",
            c: "C",
        }
    }),
    (0x10, Opcode {
        mnemonic: "STOP",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "n8",
            bytes: 1,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x11, Opcode {
        mnemonic: "LD",
        cycles: [12, 0],
        operands: [
        Operand {
            name: "DE",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "n16",
            bytes: 2,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x12, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "DE",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x13, Opcode {
        mnemonic: "INC",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "DE",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x14, Opcode {
        mnemonic: "INC",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "D",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "-",
        }
    }),
    (0x15, Opcode {
        mnemonic: "DEC",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "D",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "-",
        }
    }),
    (0x16, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "D",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "n8",
            bytes: 1,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x17, Opcode {
        mnemonic: "RLA",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "0",
            n: "0",
            h: "0",
            c: "C",
        }
    }),
    (0x18, Opcode {
        mnemonic: "JR",
        cycles: [12, 0],
        operands: [
        Operand {
            name: "e8",
            bytes: 1,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x19, Opcode {
        mnemonic: "ADD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "HL",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "DE",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "0",
            h: "H",
            c: "C",
        }
    }),
    (0x1A, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "DE",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x1B, Opcode {
        mnemonic: "DEC",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "DE",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x1C, Opcode {
        mnemonic: "INC",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "E",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "-",
        }
    }),
    (0x1D, Opcode {
        mnemonic: "DEC",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "E",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "-",
        }
    }),
    (0x1E, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "E",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "n8",
            bytes: 1,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x1F, Opcode {
        mnemonic: "RRA",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "0",
            n: "0",
            h: "0",
            c: "C",
        }
    }),
    (0x20, Opcode {
        mnemonic: "JR",
        cycles: [12, 8],
        operands: [
        Operand {
            name: "NZ",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "e8",
            bytes: 1,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x21, Opcode {
        mnemonic: "LD",
        cycles: [12, 0],
        operands: [
        Operand {
            name: "HL",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "n16",
            bytes: 2,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x22, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "HL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x23, Opcode {
        mnemonic: "INC",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "HL",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x24, Opcode {
        mnemonic: "INC",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "H",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "-",
        }
    }),
    (0x25, Opcode {
        mnemonic: "DEC",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "H",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "-",
        }
    }),
    (0x26, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "H",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "n8",
            bytes: 1,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x27, Opcode {
        mnemonic: "DAA",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "-",
            h: "0",
            c: "C",
        }
    }),
    (0x28, Opcode {
        mnemonic: "JR",
        cycles: [12, 8],
        operands: [
        Operand {
            name: "Z",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "e8",
            bytes: 1,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x29, Opcode {
        mnemonic: "ADD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "HL",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "HL",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "0",
            h: "H",
            c: "C",
        }
    }),
    (0x2A, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "HL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x2B, Opcode {
        mnemonic: "DEC",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "HL",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x2C, Opcode {
        mnemonic: "INC",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "L",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "-",
        }
    }),
    (0x2D, Opcode {
        mnemonic: "DEC",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "L",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "-",
        }
    }),
    (0x2E, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "L",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "n8",
            bytes: 1,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x2F, Opcode {
        mnemonic: "CPL",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "1",
            h: "1",
            c: "-",
        }
    }),
    (0x30, Opcode {
        mnemonic: "JR",
        cycles: [12, 8],
        operands: [
        Operand {
            name: "NC",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "e8",
            bytes: 1,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x31, Opcode {
        mnemonic: "LD",
        cycles: [12, 0],
        operands: [
        Operand {
            name: "SP",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "n16",
            bytes: 2,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x32, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "HL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x33, Opcode {
        mnemonic: "INC",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "SP",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x34, Opcode {
        mnemonic: "INC",
        cycles: [12, 0],
        operands: [
        Operand {
            name: "HL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "-",
        }
    }),
    (0x35, Opcode {
        mnemonic: "DEC",
        cycles: [12, 0],
        operands: [
        Operand {
            name: "HL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "-",
        }
    }),
    (0x36, Opcode {
        mnemonic: "LD",
        cycles: [12, 0],
        operands: [
        Operand {
            name: "HL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "n8",
            bytes: 1,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x37, Opcode {
        mnemonic: "SCF",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "0",
            h: "0",
            c: "1",
        }
    }),
    (0x38, Opcode {
        mnemonic: "JR",
        cycles: [12, 8],
        operands: [
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "e8",
            bytes: 1,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x39, Opcode {
        mnemonic: "ADD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "HL",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "SP",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "0",
            h: "H",
            c: "C",
        }
    }),
    (0x3A, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "HL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x3B, Opcode {
        mnemonic: "DEC",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "SP",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x3C, Opcode {
        mnemonic: "INC",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "-",
        }
    }),
    (0x3D, Opcode {
        mnemonic: "DEC",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "-",
        }
    }),
    (0x3E, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "n8",
            bytes: 1,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x3F, Opcode {
        mnemonic: "CCF",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "0",
            h: "0",
            c: "C",
        }
    }),
    (0x40, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "B",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "B",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x41, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "B",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x42, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "B",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "D",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x43, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "B",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "E",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x44, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "B",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "H",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x45, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "B",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "L",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x46, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "B",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "HL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x47, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "B",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x48, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "B",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x49, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x4A, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "D",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x4B, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "E",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x4C, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "H",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x4D, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "L",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x4E, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "HL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x4F, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x50, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "D",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "B",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x51, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "D",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x52, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "D",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "D",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x53, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "D",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "E",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x54, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "D",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "H",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x55, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "D",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "L",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x56, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "D",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "HL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x57, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "D",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x58, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "E",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "B",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x59, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "E",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x5A, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "E",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "D",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x5B, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "E",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "E",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x5C, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "E",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "H",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x5D, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "E",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "L",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x5E, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "E",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "HL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x5F, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "E",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x60, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "H",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "B",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x61, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "H",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x62, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "H",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "D",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x63, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "H",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "E",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x64, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "H",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "H",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x65, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "H",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "L",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x66, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "H",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "HL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x67, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "H",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x68, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "L",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "B",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x69, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "L",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x6A, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "L",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "D",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x6B, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "L",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "E",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x6C, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "L",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "H",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x6D, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "L",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "L",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x6E, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "L",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "HL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x6F, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "L",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x70, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "HL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "B",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x71, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "HL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x72, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "HL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "D",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x73, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "HL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "E",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x74, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "HL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "H",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x75, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "HL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "L",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x76, Opcode {
        mnemonic: "HALT",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x77, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "HL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x78, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "B",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x79, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x7A, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "D",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x7B, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "E",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x7C, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "H",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x7D, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "L",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x7E, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "HL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x7F, Opcode {
        mnemonic: "LD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0x80, Opcode {
        mnemonic: "ADD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "B",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        }
    }),
    (0x81, Opcode {
        mnemonic: "ADD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        }
    }),
    (0x82, Opcode {
        mnemonic: "ADD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "D",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        }
    }),
    (0x83, Opcode {
        mnemonic: "ADD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "E",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        }
    }),
    (0x84, Opcode {
        mnemonic: "ADD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "H",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        }
    }),
    (0x85, Opcode {
        mnemonic: "ADD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "L",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        }
    }),
    (0x86, Opcode {
        mnemonic: "ADD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "HL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        }
    }),
    (0x87, Opcode {
        mnemonic: "ADD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        }
    }),
    (0x88, Opcode {
        mnemonic: "ADC",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "B",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        }
    }),
    (0x89, Opcode {
        mnemonic: "ADC",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        }
    }),
    (0x8A, Opcode {
        mnemonic: "ADC",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "D",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        }
    }),
    (0x8B, Opcode {
        mnemonic: "ADC",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "E",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        }
    }),
    (0x8C, Opcode {
        mnemonic: "ADC",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "H",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        }
    }),
    (0x8D, Opcode {
        mnemonic: "ADC",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "L",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        }
    }),
    (0x8E, Opcode {
        mnemonic: "ADC",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "HL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        }
    }),
    (0x8F, Opcode {
        mnemonic: "ADC",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        }
    }),
    (0x90, Opcode {
        mnemonic: "SUB",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "B",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        }
    }),
    (0x91, Opcode {
        mnemonic: "SUB",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        }
    }),
    (0x92, Opcode {
        mnemonic: "SUB",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "D",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        }
    }),
    (0x93, Opcode {
        mnemonic: "SUB",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "E",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        }
    }),
    (0x94, Opcode {
        mnemonic: "SUB",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "H",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        }
    }),
    (0x95, Opcode {
        mnemonic: "SUB",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "L",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        }
    }),
    (0x96, Opcode {
        mnemonic: "SUB",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "HL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        }
    }),
    (0x97, Opcode {
        mnemonic: "SUB",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "1",
            n: "1",
            h: "0",
            c: "0",
        }
    }),
    (0x98, Opcode {
        mnemonic: "SBC",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "B",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        }
    }),
    (0x99, Opcode {
        mnemonic: "SBC",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        }
    }),
    (0x9A, Opcode {
        mnemonic: "SBC",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "D",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        }
    }),
    (0x9B, Opcode {
        mnemonic: "SBC",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "E",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        }
    }),
    (0x9C, Opcode {
        mnemonic: "SBC",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "H",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        }
    }),
    (0x9D, Opcode {
        mnemonic: "SBC",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "L",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        }
    }),
    (0x9E, Opcode {
        mnemonic: "SBC",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "HL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        }
    }),
    (0x9F, Opcode {
        mnemonic: "SBC",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "-",
        }
    }),
    (0xA0, Opcode {
        mnemonic: "AND",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "B",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "1",
            c: "0",
        }
    }),
    (0xA1, Opcode {
        mnemonic: "AND",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "1",
            c: "0",
        }
    }),
    (0xA2, Opcode {
        mnemonic: "AND",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "D",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "1",
            c: "0",
        }
    }),
    (0xA3, Opcode {
        mnemonic: "AND",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "E",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "1",
            c: "0",
        }
    }),
    (0xA4, Opcode {
        mnemonic: "AND",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "H",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "1",
            c: "0",
        }
    }),
    (0xA5, Opcode {
        mnemonic: "AND",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "L",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "1",
            c: "0",
        }
    }),
    (0xA6, Opcode {
        mnemonic: "AND",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "HL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "1",
            c: "0",
        }
    }),
    (0xA7, Opcode {
        mnemonic: "AND",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "1",
            c: "0",
        }
    }),
    (0xA8, Opcode {
        mnemonic: "XOR",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "B",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        }
    }),
    (0xA9, Opcode {
        mnemonic: "XOR",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        }
    }),
    (0xAA, Opcode {
        mnemonic: "XOR",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "D",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        }
    }),
    (0xAB, Opcode {
        mnemonic: "XOR",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "E",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        }
    }),
    (0xAC, Opcode {
        mnemonic: "XOR",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "H",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        }
    }),
    (0xAD, Opcode {
        mnemonic: "XOR",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "L",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        }
    }),
    (0xAE, Opcode {
        mnemonic: "XOR",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "HL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        }
    }),
    (0xAF, Opcode {
        mnemonic: "XOR",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "1",
            n: "0",
            h: "0",
            c: "0",
        }
    }),
    (0xB0, Opcode {
        mnemonic: "OR",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "B",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        }
    }),
    (0xB1, Opcode {
        mnemonic: "OR",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        }
    }),
    (0xB2, Opcode {
        mnemonic: "OR",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "D",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        }
    }),
    (0xB3, Opcode {
        mnemonic: "OR",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "E",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        }
    }),
    (0xB4, Opcode {
        mnemonic: "OR",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "H",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        }
    }),
    (0xB5, Opcode {
        mnemonic: "OR",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "L",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        }
    }),
    (0xB6, Opcode {
        mnemonic: "OR",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "HL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        }
    }),
    (0xB7, Opcode {
        mnemonic: "OR",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        }
    }),
    (0xB8, Opcode {
        mnemonic: "CP",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "B",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        }
    }),
    (0xB9, Opcode {
        mnemonic: "CP",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        }
    }),
    (0xBA, Opcode {
        mnemonic: "CP",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "D",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        }
    }),
    (0xBB, Opcode {
        mnemonic: "CP",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "E",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        }
    }),
    (0xBC, Opcode {
        mnemonic: "CP",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "H",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        }
    }),
    (0xBD, Opcode {
        mnemonic: "CP",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "L",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        }
    }),
    (0xBE, Opcode {
        mnemonic: "CP",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "HL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        }
    }),
    (0xBF, Opcode {
        mnemonic: "CP",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "1",
            n: "1",
            h: "0",
            c: "0",
        }
    }),
    (0xC0, Opcode {
        mnemonic: "RET",
        cycles: [20, 8],
        operands: [
        Operand {
            name: "NZ",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xC1, Opcode {
        mnemonic: "POP",
        cycles: [12, 0],
        operands: [
        Operand {
            name: "BC",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xC2, Opcode {
        mnemonic: "JP",
        cycles: [16, 12],
        operands: [
        Operand {
            name: "NZ",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "a16",
            bytes: 2,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xC3, Opcode {
        mnemonic: "JP",
        cycles: [16, 0],
        operands: [
        Operand {
            name: "a16",
            bytes: 2,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xC4, Opcode {
        mnemonic: "CALL",
        cycles: [24, 12],
        operands: [
        Operand {
            name: "NZ",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "a16",
            bytes: 2,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xC5, Opcode {
        mnemonic: "PUSH",
        cycles: [16, 0],
        operands: [
        Operand {
            name: "BC",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xC6, Opcode {
        mnemonic: "ADD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "n8",
            bytes: 1,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        }
    }),
    (0xC7, Opcode {
        mnemonic: "RST",
        cycles: [16, 0],
        operands: [
        Operand {
            name: "$00",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xC8, Opcode {
        mnemonic: "RET",
        cycles: [20, 8],
        operands: [
        Operand {
            name: "Z",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xC9, Opcode {
        mnemonic: "RET",
        cycles: [16, 0],
        operands: [
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xCA, Opcode {
        mnemonic: "JP",
        cycles: [16, 12],
        operands: [
        Operand {
            name: "Z",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "a16",
            bytes: 2,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xCB, Opcode {
        mnemonic: "PREFIX",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xCC, Opcode {
        mnemonic: "CALL",
        cycles: [24, 12],
        operands: [
        Operand {
            name: "Z",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "a16",
            bytes: 2,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xCD, Opcode {
        mnemonic: "CALL",
        cycles: [24, 0],
        operands: [
        Operand {
            name: "a16",
            bytes: 2,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xCE, Opcode {
        mnemonic: "ADC",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "n8",
            bytes: 1,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "H",
            c: "C",
        }
    }),
    (0xCF, Opcode {
        mnemonic: "RST",
        cycles: [16, 0],
        operands: [
        Operand {
            name: "$08",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xD0, Opcode {
        mnemonic: "RET",
        cycles: [20, 8],
        operands: [
        Operand {
            name: "NC",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xD1, Opcode {
        mnemonic: "POP",
        cycles: [12, 0],
        operands: [
        Operand {
            name: "DE",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xD2, Opcode {
        mnemonic: "JP",
        cycles: [16, 12],
        operands: [
        Operand {
            name: "NC",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "a16",
            bytes: 2,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xD3, Opcode {
        mnemonic: "ILLEGAL_D3",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xD4, Opcode {
        mnemonic: "CALL",
        cycles: [24, 12],
        operands: [
        Operand {
            name: "NC",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "a16",
            bytes: 2,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xD5, Opcode {
        mnemonic: "PUSH",
        cycles: [16, 0],
        operands: [
        Operand {
            name: "DE",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xD6, Opcode {
        mnemonic: "SUB",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "n8",
            bytes: 1,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        }
    }),
    (0xD7, Opcode {
        mnemonic: "RST",
        cycles: [16, 0],
        operands: [
        Operand {
            name: "$10",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xD8, Opcode {
        mnemonic: "RET",
        cycles: [20, 8],
        operands: [
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xD9, Opcode {
        mnemonic: "RETI",
        cycles: [16, 0],
        operands: [
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xDA, Opcode {
        mnemonic: "JP",
        cycles: [16, 12],
        operands: [
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "a16",
            bytes: 2,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xDB, Opcode {
        mnemonic: "ILLEGAL_DB",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xDC, Opcode {
        mnemonic: "CALL",
        cycles: [24, 12],
        operands: [
        Operand {
            name: "C",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "a16",
            bytes: 2,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xDD, Opcode {
        mnemonic: "ILLEGAL_DD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xDE, Opcode {
        mnemonic: "SBC",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "n8",
            bytes: 1,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        }
    }),
    (0xDF, Opcode {
        mnemonic: "RST",
        cycles: [16, 0],
        operands: [
        Operand {
            name: "$18",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xE0, Opcode {
        mnemonic: "LDH",
        cycles: [12, 0],
        operands: [
        Operand {
            name: "a8",
            bytes: 1,
            immediate: false
        },
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xE1, Opcode {
        mnemonic: "POP",
        cycles: [12, 0],
        operands: [
        Operand {
            name: "HL",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xE2, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "C",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xE3, Opcode {
        mnemonic: "ILLEGAL_E3",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xE4, Opcode {
        mnemonic: "ILLEGAL_E4",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xE5, Opcode {
        mnemonic: "PUSH",
        cycles: [16, 0],
        operands: [
        Operand {
            name: "HL",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xE6, Opcode {
        mnemonic: "AND",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "n8",
            bytes: 1,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "1",
            c: "0",
        }
    }),
    (0xE7, Opcode {
        mnemonic: "RST",
        cycles: [16, 0],
        operands: [
        Operand {
            name: "$20",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xE8, Opcode {
        mnemonic: "ADD",
        cycles: [16, 0],
        operands: [
        Operand {
            name: "SP",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "e8",
            bytes: 1,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "0",
            n: "0",
            h: "H",
            c: "C",
        }
    }),
    (0xE9, Opcode {
        mnemonic: "JP",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "HL",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xEA, Opcode {
        mnemonic: "LD",
        cycles: [16, 0],
        operands: [
        Operand {
            name: "a16",
            bytes: 2,
            immediate: false
        },
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xEB, Opcode {
        mnemonic: "ILLEGAL_EB",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xEC, Opcode {
        mnemonic: "ILLEGAL_EC",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xED, Opcode {
        mnemonic: "ILLEGAL_ED",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xEE, Opcode {
        mnemonic: "XOR",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "n8",
            bytes: 1,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        }
    }),
    (0xEF, Opcode {
        mnemonic: "RST",
        cycles: [16, 0],
        operands: [
        Operand {
            name: "$28",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xF0, Opcode {
        mnemonic: "LDH",
        cycles: [12, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "a8",
            bytes: 1,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xF1, Opcode {
        mnemonic: "POP",
        cycles: [12, 0],
        operands: [
        Operand {
            name: "AF",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "N",
            h: "H",
            c: "C",
        }
    }),
    (0xF2, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "C",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xF3, Opcode {
        mnemonic: "DI",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xF4, Opcode {
        mnemonic: "ILLEGAL_F4",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xF5, Opcode {
        mnemonic: "PUSH",
        cycles: [16, 0],
        operands: [
        Operand {
            name: "AF",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xF6, Opcode {
        mnemonic: "OR",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "n8",
            bytes: 1,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "0",
            h: "0",
            c: "0",
        }
    }),
    (0xF7, Opcode {
        mnemonic: "RST",
        cycles: [16, 0],
        operands: [
        Operand {
            name: "$30",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xF8, Opcode {
        mnemonic: "LD",
        cycles: [12, 0],
        operands: [
        Operand {
            name: "HL",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "SP",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "e8",
            bytes: 1,
            immediate: true
        }],
        immediate: true,
        flags: Flags {
            z: "0",
            n: "0",
            h: "H",
            c: "C",
        }
    }),
    (0xF9, Opcode {
        mnemonic: "LD",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "SP",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "HL",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xFA, Opcode {
        mnemonic: "LD",
        cycles: [16, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "a16",
            bytes: 2,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: false,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xFB, Opcode {
        mnemonic: "EI",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xFC, Opcode {
        mnemonic: "ILLEGAL_FC",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xFD, Opcode {
        mnemonic: "ILLEGAL_FD",
        cycles: [4, 0],
        operands: [
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    (0xFE, Opcode {
        mnemonic: "CP",
        cycles: [8, 0],
        operands: [
        Operand {
            name: "A",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "n8",
            bytes: 1,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "Z",
            n: "1",
            h: "H",
            c: "C",
        }
    }),
    (0xFF, Opcode {
        mnemonic: "RST",
        cycles: [16, 0],
        operands: [
        Operand {
            name: "$38",
            bytes: 0,
            immediate: true
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        },
        Operand {
            name: "NULL",
            bytes: 0,
            immediate: false
        }],
        immediate: true,
        flags: Flags {
            z: "-",
            n: "-",
            h: "-",
            c: "-",
        }
    }),
    
];

pub fn opcode_get(opcode: &u8) -> Opcode {
    for (key, value) in OPCODES.iter() {
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
            output.push_str(&format!("{} ", operand.name));
        }
        write!(f, "{}", output)
    }
}
