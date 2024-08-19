pub struct Opcodes {
    pub mnemonic: &'static str,
    pub cycles: [u8; 2],
    pub operands: [Operands; 3],
    pub immediate: bool,
    pub flags: Flags,
}

pub struct Operands {
    pub name: &'static str,
    pub bytes: u8,
    pub immediate: bool,
}

pub struct Flags {
    pub z: &'static str,
    pub n: &'static str,
    pub h: &'static str,
    pub c: &'static str,
}

pub const H0X00: Opcodes = Opcodes {
    mnemonic: "NOP",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X01: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [12, 0],
    operands: [
    Operands {
        name: "BC",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "n16",
        bytes: 2,
        immediate: true
    },
    Operands {
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
};

pub const H0X02: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "BC",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X03: Opcodes = Opcodes {
    mnemonic: "INC",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "BC",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X04: Opcodes = Opcodes {
    mnemonic: "INC",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "B",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X05: Opcodes = Opcodes {
    mnemonic: "DEC",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "B",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X06: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "B",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "n8",
        bytes: 1,
        immediate: true
    },
    Operands {
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
};

pub const H0X07: Opcodes = Opcodes {
    mnemonic: "RLCA",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X08: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [20, 0],
    operands: [
    Operands {
        name: "a16",
        bytes: 2,
        immediate: false
    },
    Operands {
        name: "SP",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X09: Opcodes = Opcodes {
    mnemonic: "ADD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "HL",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "BC",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X0A: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "BC",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X0B: Opcodes = Opcodes {
    mnemonic: "DEC",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "BC",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X0C: Opcodes = Opcodes {
    mnemonic: "INC",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X0D: Opcodes = Opcodes {
    mnemonic: "DEC",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X0E: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "n8",
        bytes: 1,
        immediate: true
    },
    Operands {
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
};

pub const H0X0F: Opcodes = Opcodes {
    mnemonic: "RRCA",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X10: Opcodes = Opcodes {
    mnemonic: "STOP",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "n8",
        bytes: 1,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X11: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [12, 0],
    operands: [
    Operands {
        name: "DE",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "n16",
        bytes: 2,
        immediate: true
    },
    Operands {
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
};

pub const H0X12: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "DE",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X13: Opcodes = Opcodes {
    mnemonic: "INC",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "DE",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X14: Opcodes = Opcodes {
    mnemonic: "INC",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "D",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X15: Opcodes = Opcodes {
    mnemonic: "DEC",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "D",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X16: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "D",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "n8",
        bytes: 1,
        immediate: true
    },
    Operands {
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
};

pub const H0X17: Opcodes = Opcodes {
    mnemonic: "RLA",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X18: Opcodes = Opcodes {
    mnemonic: "JR",
    cycles: [12, 0],
    operands: [
    Operands {
        name: "e8",
        bytes: 1,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X19: Opcodes = Opcodes {
    mnemonic: "ADD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "HL",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "DE",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X1A: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "DE",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X1B: Opcodes = Opcodes {
    mnemonic: "DEC",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "DE",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X1C: Opcodes = Opcodes {
    mnemonic: "INC",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "E",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X1D: Opcodes = Opcodes {
    mnemonic: "DEC",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "E",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X1E: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "E",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "n8",
        bytes: 1,
        immediate: true
    },
    Operands {
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
};

pub const H0X1F: Opcodes = Opcodes {
    mnemonic: "RRA",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X20: Opcodes = Opcodes {
    mnemonic: "JR",
    cycles: [12, 8],
    operands: [
    Operands {
        name: "NZ",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "e8",
        bytes: 1,
        immediate: true
    },
    Operands {
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
};

pub const H0X21: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [12, 0],
    operands: [
    Operands {
        name: "HL",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "n16",
        bytes: 2,
        immediate: true
    },
    Operands {
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
};

pub const H0X22: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "HL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X23: Opcodes = Opcodes {
    mnemonic: "INC",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "HL",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X24: Opcodes = Opcodes {
    mnemonic: "INC",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "H",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X25: Opcodes = Opcodes {
    mnemonic: "DEC",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "H",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X26: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "H",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "n8",
        bytes: 1,
        immediate: true
    },
    Operands {
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
};

pub const H0X27: Opcodes = Opcodes {
    mnemonic: "DAA",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X28: Opcodes = Opcodes {
    mnemonic: "JR",
    cycles: [12, 8],
    operands: [
    Operands {
        name: "Z",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "e8",
        bytes: 1,
        immediate: true
    },
    Operands {
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
};

pub const H0X29: Opcodes = Opcodes {
    mnemonic: "ADD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "HL",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "HL",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X2A: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "HL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X2B: Opcodes = Opcodes {
    mnemonic: "DEC",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "HL",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X2C: Opcodes = Opcodes {
    mnemonic: "INC",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "L",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X2D: Opcodes = Opcodes {
    mnemonic: "DEC",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "L",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X2E: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "L",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "n8",
        bytes: 1,
        immediate: true
    },
    Operands {
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
};

pub const H0X2F: Opcodes = Opcodes {
    mnemonic: "CPL",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X30: Opcodes = Opcodes {
    mnemonic: "JR",
    cycles: [12, 8],
    operands: [
    Operands {
        name: "NC",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "e8",
        bytes: 1,
        immediate: true
    },
    Operands {
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
};

pub const H0X31: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [12, 0],
    operands: [
    Operands {
        name: "SP",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "n16",
        bytes: 2,
        immediate: true
    },
    Operands {
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
};

pub const H0X32: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "HL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X33: Opcodes = Opcodes {
    mnemonic: "INC",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "SP",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X34: Opcodes = Opcodes {
    mnemonic: "INC",
    cycles: [12, 0],
    operands: [
    Operands {
        name: "HL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X35: Opcodes = Opcodes {
    mnemonic: "DEC",
    cycles: [12, 0],
    operands: [
    Operands {
        name: "HL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X36: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [12, 0],
    operands: [
    Operands {
        name: "HL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "n8",
        bytes: 1,
        immediate: true
    },
    Operands {
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
};

pub const H0X37: Opcodes = Opcodes {
    mnemonic: "SCF",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X38: Opcodes = Opcodes {
    mnemonic: "JR",
    cycles: [12, 8],
    operands: [
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "e8",
        bytes: 1,
        immediate: true
    },
    Operands {
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
};

pub const H0X39: Opcodes = Opcodes {
    mnemonic: "ADD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "HL",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "SP",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X3A: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "HL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X3B: Opcodes = Opcodes {
    mnemonic: "DEC",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "SP",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X3C: Opcodes = Opcodes {
    mnemonic: "INC",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X3D: Opcodes = Opcodes {
    mnemonic: "DEC",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X3E: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "n8",
        bytes: 1,
        immediate: true
    },
    Operands {
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
};

pub const H0X3F: Opcodes = Opcodes {
    mnemonic: "CCF",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X40: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "B",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "B",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X41: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "B",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X42: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "B",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "D",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X43: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "B",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "E",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X44: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "B",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "H",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X45: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "B",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "L",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X46: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "B",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "HL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X47: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "B",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X48: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "B",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X49: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X4A: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "D",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X4B: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "E",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X4C: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "H",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X4D: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "L",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X4E: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "HL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X4F: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X50: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "D",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "B",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X51: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "D",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X52: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "D",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "D",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X53: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "D",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "E",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X54: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "D",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "H",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X55: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "D",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "L",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X56: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "D",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "HL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X57: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "D",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X58: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "E",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "B",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X59: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "E",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X5A: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "E",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "D",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X5B: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "E",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "E",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X5C: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "E",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "H",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X5D: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "E",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "L",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X5E: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "E",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "HL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X5F: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "E",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X60: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "H",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "B",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X61: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "H",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X62: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "H",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "D",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X63: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "H",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "E",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X64: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "H",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "H",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X65: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "H",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "L",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X66: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "H",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "HL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X67: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "H",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X68: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "L",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "B",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X69: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "L",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X6A: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "L",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "D",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X6B: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "L",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "E",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X6C: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "L",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "H",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X6D: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "L",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "L",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X6E: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "L",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "HL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X6F: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "L",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X70: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "HL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "B",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X71: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "HL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X72: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "HL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "D",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X73: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "HL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "E",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X74: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "HL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "H",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X75: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "HL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "L",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X76: Opcodes = Opcodes {
    mnemonic: "HALT",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X77: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "HL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X78: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "B",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X79: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X7A: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "D",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X7B: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "E",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X7C: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "H",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X7D: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "L",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X7E: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "HL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X7F: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X80: Opcodes = Opcodes {
    mnemonic: "ADD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "B",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X81: Opcodes = Opcodes {
    mnemonic: "ADD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X82: Opcodes = Opcodes {
    mnemonic: "ADD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "D",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X83: Opcodes = Opcodes {
    mnemonic: "ADD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "E",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X84: Opcodes = Opcodes {
    mnemonic: "ADD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "H",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X85: Opcodes = Opcodes {
    mnemonic: "ADD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "L",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X86: Opcodes = Opcodes {
    mnemonic: "ADD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "HL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X87: Opcodes = Opcodes {
    mnemonic: "ADD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X88: Opcodes = Opcodes {
    mnemonic: "ADC",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "B",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X89: Opcodes = Opcodes {
    mnemonic: "ADC",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X8A: Opcodes = Opcodes {
    mnemonic: "ADC",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "D",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X8B: Opcodes = Opcodes {
    mnemonic: "ADC",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "E",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X8C: Opcodes = Opcodes {
    mnemonic: "ADC",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "H",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X8D: Opcodes = Opcodes {
    mnemonic: "ADC",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "L",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X8E: Opcodes = Opcodes {
    mnemonic: "ADC",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "HL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X8F: Opcodes = Opcodes {
    mnemonic: "ADC",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X90: Opcodes = Opcodes {
    mnemonic: "SUB",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "B",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X91: Opcodes = Opcodes {
    mnemonic: "SUB",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X92: Opcodes = Opcodes {
    mnemonic: "SUB",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "D",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X93: Opcodes = Opcodes {
    mnemonic: "SUB",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "E",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X94: Opcodes = Opcodes {
    mnemonic: "SUB",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "H",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X95: Opcodes = Opcodes {
    mnemonic: "SUB",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "L",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X96: Opcodes = Opcodes {
    mnemonic: "SUB",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "HL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X97: Opcodes = Opcodes {
    mnemonic: "SUB",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X98: Opcodes = Opcodes {
    mnemonic: "SBC",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "B",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X99: Opcodes = Opcodes {
    mnemonic: "SBC",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X9A: Opcodes = Opcodes {
    mnemonic: "SBC",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "D",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X9B: Opcodes = Opcodes {
    mnemonic: "SBC",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "E",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X9C: Opcodes = Opcodes {
    mnemonic: "SBC",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "H",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X9D: Opcodes = Opcodes {
    mnemonic: "SBC",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "L",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0X9E: Opcodes = Opcodes {
    mnemonic: "SBC",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "HL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0X9F: Opcodes = Opcodes {
    mnemonic: "SBC",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XA0: Opcodes = Opcodes {
    mnemonic: "AND",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "B",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XA1: Opcodes = Opcodes {
    mnemonic: "AND",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XA2: Opcodes = Opcodes {
    mnemonic: "AND",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "D",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XA3: Opcodes = Opcodes {
    mnemonic: "AND",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "E",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XA4: Opcodes = Opcodes {
    mnemonic: "AND",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "H",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XA5: Opcodes = Opcodes {
    mnemonic: "AND",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "L",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XA6: Opcodes = Opcodes {
    mnemonic: "AND",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "HL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XA7: Opcodes = Opcodes {
    mnemonic: "AND",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XA8: Opcodes = Opcodes {
    mnemonic: "XOR",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "B",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XA9: Opcodes = Opcodes {
    mnemonic: "XOR",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XAA: Opcodes = Opcodes {
    mnemonic: "XOR",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "D",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XAB: Opcodes = Opcodes {
    mnemonic: "XOR",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "E",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XAC: Opcodes = Opcodes {
    mnemonic: "XOR",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "H",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XAD: Opcodes = Opcodes {
    mnemonic: "XOR",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "L",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XAE: Opcodes = Opcodes {
    mnemonic: "XOR",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "HL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XAF: Opcodes = Opcodes {
    mnemonic: "XOR",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XB0: Opcodes = Opcodes {
    mnemonic: "OR",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "B",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XB1: Opcodes = Opcodes {
    mnemonic: "OR",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XB2: Opcodes = Opcodes {
    mnemonic: "OR",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "D",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XB3: Opcodes = Opcodes {
    mnemonic: "OR",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "E",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XB4: Opcodes = Opcodes {
    mnemonic: "OR",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "H",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XB5: Opcodes = Opcodes {
    mnemonic: "OR",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "L",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XB6: Opcodes = Opcodes {
    mnemonic: "OR",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "HL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XB7: Opcodes = Opcodes {
    mnemonic: "OR",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XB8: Opcodes = Opcodes {
    mnemonic: "CP",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "B",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XB9: Opcodes = Opcodes {
    mnemonic: "CP",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XBA: Opcodes = Opcodes {
    mnemonic: "CP",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "D",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XBB: Opcodes = Opcodes {
    mnemonic: "CP",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "E",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XBC: Opcodes = Opcodes {
    mnemonic: "CP",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "H",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XBD: Opcodes = Opcodes {
    mnemonic: "CP",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "L",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XBE: Opcodes = Opcodes {
    mnemonic: "CP",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "HL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XBF: Opcodes = Opcodes {
    mnemonic: "CP",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XC0: Opcodes = Opcodes {
    mnemonic: "RET",
    cycles: [20, 8],
    operands: [
    Operands {
        name: "NZ",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XC1: Opcodes = Opcodes {
    mnemonic: "POP",
    cycles: [12, 0],
    operands: [
    Operands {
        name: "BC",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XC2: Opcodes = Opcodes {
    mnemonic: "JP",
    cycles: [16, 12],
    operands: [
    Operands {
        name: "NZ",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "a16",
        bytes: 2,
        immediate: true
    },
    Operands {
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
};

pub const H0XC3: Opcodes = Opcodes {
    mnemonic: "JP",
    cycles: [16, 0],
    operands: [
    Operands {
        name: "a16",
        bytes: 2,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XC4: Opcodes = Opcodes {
    mnemonic: "CALL",
    cycles: [24, 12],
    operands: [
    Operands {
        name: "NZ",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "a16",
        bytes: 2,
        immediate: true
    },
    Operands {
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
};

pub const H0XC5: Opcodes = Opcodes {
    mnemonic: "PUSH",
    cycles: [16, 0],
    operands: [
    Operands {
        name: "BC",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XC6: Opcodes = Opcodes {
    mnemonic: "ADD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "n8",
        bytes: 1,
        immediate: true
    },
    Operands {
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
};

pub const H0XC7: Opcodes = Opcodes {
    mnemonic: "RST",
    cycles: [16, 0],
    operands: [
    Operands {
        name: "$00",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XC8: Opcodes = Opcodes {
    mnemonic: "RET",
    cycles: [20, 8],
    operands: [
    Operands {
        name: "Z",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XC9: Opcodes = Opcodes {
    mnemonic: "RET",
    cycles: [16, 0],
    operands: [
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XCA: Opcodes = Opcodes {
    mnemonic: "JP",
    cycles: [16, 12],
    operands: [
    Operands {
        name: "Z",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "a16",
        bytes: 2,
        immediate: true
    },
    Operands {
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
};

pub const H0XCB: Opcodes = Opcodes {
    mnemonic: "PREFIX",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XCC: Opcodes = Opcodes {
    mnemonic: "CALL",
    cycles: [24, 12],
    operands: [
    Operands {
        name: "Z",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "a16",
        bytes: 2,
        immediate: true
    },
    Operands {
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
};

pub const H0XCD: Opcodes = Opcodes {
    mnemonic: "CALL",
    cycles: [24, 0],
    operands: [
    Operands {
        name: "a16",
        bytes: 2,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XCE: Opcodes = Opcodes {
    mnemonic: "ADC",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "n8",
        bytes: 1,
        immediate: true
    },
    Operands {
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
};

pub const H0XCF: Opcodes = Opcodes {
    mnemonic: "RST",
    cycles: [16, 0],
    operands: [
    Operands {
        name: "$08",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XD0: Opcodes = Opcodes {
    mnemonic: "RET",
    cycles: [20, 8],
    operands: [
    Operands {
        name: "NC",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XD1: Opcodes = Opcodes {
    mnemonic: "POP",
    cycles: [12, 0],
    operands: [
    Operands {
        name: "DE",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XD2: Opcodes = Opcodes {
    mnemonic: "JP",
    cycles: [16, 12],
    operands: [
    Operands {
        name: "NC",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "a16",
        bytes: 2,
        immediate: true
    },
    Operands {
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
};

pub const H0XD3: Opcodes = Opcodes {
    mnemonic: "ILLEGAL_D3",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XD4: Opcodes = Opcodes {
    mnemonic: "CALL",
    cycles: [24, 12],
    operands: [
    Operands {
        name: "NC",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "a16",
        bytes: 2,
        immediate: true
    },
    Operands {
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
};

pub const H0XD5: Opcodes = Opcodes {
    mnemonic: "PUSH",
    cycles: [16, 0],
    operands: [
    Operands {
        name: "DE",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XD6: Opcodes = Opcodes {
    mnemonic: "SUB",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "n8",
        bytes: 1,
        immediate: true
    },
    Operands {
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
};

pub const H0XD7: Opcodes = Opcodes {
    mnemonic: "RST",
    cycles: [16, 0],
    operands: [
    Operands {
        name: "$10",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XD8: Opcodes = Opcodes {
    mnemonic: "RET",
    cycles: [20, 8],
    operands: [
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XD9: Opcodes = Opcodes {
    mnemonic: "RETI",
    cycles: [16, 0],
    operands: [
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XDA: Opcodes = Opcodes {
    mnemonic: "JP",
    cycles: [16, 12],
    operands: [
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "a16",
        bytes: 2,
        immediate: true
    },
    Operands {
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
};

pub const H0XDB: Opcodes = Opcodes {
    mnemonic: "ILLEGAL_DB",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XDC: Opcodes = Opcodes {
    mnemonic: "CALL",
    cycles: [24, 12],
    operands: [
    Operands {
        name: "C",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "a16",
        bytes: 2,
        immediate: true
    },
    Operands {
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
};

pub const H0XDD: Opcodes = Opcodes {
    mnemonic: "ILLEGAL_DD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XDE: Opcodes = Opcodes {
    mnemonic: "SBC",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "n8",
        bytes: 1,
        immediate: true
    },
    Operands {
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
};

pub const H0XDF: Opcodes = Opcodes {
    mnemonic: "RST",
    cycles: [16, 0],
    operands: [
    Operands {
        name: "$18",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XE0: Opcodes = Opcodes {
    mnemonic: "LDH",
    cycles: [12, 0],
    operands: [
    Operands {
        name: "a8",
        bytes: 1,
        immediate: false
    },
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XE1: Opcodes = Opcodes {
    mnemonic: "POP",
    cycles: [12, 0],
    operands: [
    Operands {
        name: "HL",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XE2: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "C",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XE3: Opcodes = Opcodes {
    mnemonic: "ILLEGAL_E3",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XE4: Opcodes = Opcodes {
    mnemonic: "ILLEGAL_E4",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XE5: Opcodes = Opcodes {
    mnemonic: "PUSH",
    cycles: [16, 0],
    operands: [
    Operands {
        name: "HL",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XE6: Opcodes = Opcodes {
    mnemonic: "AND",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "n8",
        bytes: 1,
        immediate: true
    },
    Operands {
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
};

pub const H0XE7: Opcodes = Opcodes {
    mnemonic: "RST",
    cycles: [16, 0],
    operands: [
    Operands {
        name: "$20",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XE8: Opcodes = Opcodes {
    mnemonic: "ADD",
    cycles: [16, 0],
    operands: [
    Operands {
        name: "SP",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "e8",
        bytes: 1,
        immediate: true
    },
    Operands {
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
};

pub const H0XE9: Opcodes = Opcodes {
    mnemonic: "JP",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "HL",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XEA: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [16, 0],
    operands: [
    Operands {
        name: "a16",
        bytes: 2,
        immediate: false
    },
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XEB: Opcodes = Opcodes {
    mnemonic: "ILLEGAL_EB",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XEC: Opcodes = Opcodes {
    mnemonic: "ILLEGAL_EC",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XED: Opcodes = Opcodes {
    mnemonic: "ILLEGAL_ED",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XEE: Opcodes = Opcodes {
    mnemonic: "XOR",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "n8",
        bytes: 1,
        immediate: true
    },
    Operands {
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
};

pub const H0XEF: Opcodes = Opcodes {
    mnemonic: "RST",
    cycles: [16, 0],
    operands: [
    Operands {
        name: "$28",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XF0: Opcodes = Opcodes {
    mnemonic: "LDH",
    cycles: [12, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "a8",
        bytes: 1,
        immediate: false
    },
    Operands {
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
};

pub const H0XF1: Opcodes = Opcodes {
    mnemonic: "POP",
    cycles: [12, 0],
    operands: [
    Operands {
        name: "AF",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XF2: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "C",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XF3: Opcodes = Opcodes {
    mnemonic: "DI",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XF4: Opcodes = Opcodes {
    mnemonic: "ILLEGAL_F4",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XF5: Opcodes = Opcodes {
    mnemonic: "PUSH",
    cycles: [16, 0],
    operands: [
    Operands {
        name: "AF",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XF6: Opcodes = Opcodes {
    mnemonic: "OR",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "n8",
        bytes: 1,
        immediate: true
    },
    Operands {
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
};

pub const H0XF7: Opcodes = Opcodes {
    mnemonic: "RST",
    cycles: [16, 0],
    operands: [
    Operands {
        name: "$30",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XF8: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [12, 0],
    operands: [
    Operands {
        name: "HL",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "SP",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XF9: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "SP",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "HL",
        bytes: 0,
        immediate: true
    },
    Operands {
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
};

pub const H0XFA: Opcodes = Opcodes {
    mnemonic: "LD",
    cycles: [16, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "a16",
        bytes: 2,
        immediate: false
    },
    Operands {
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
};

pub const H0XFB: Opcodes = Opcodes {
    mnemonic: "EI",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XFC: Opcodes = Opcodes {
    mnemonic: "ILLEGAL_FC",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XFD: Opcodes = Opcodes {
    mnemonic: "ILLEGAL_FD",
    cycles: [4, 0],
    operands: [
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

pub const H0XFE: Opcodes = Opcodes {
    mnemonic: "CP",
    cycles: [8, 0],
    operands: [
    Operands {
        name: "A",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "n8",
        bytes: 1,
        immediate: true
    },
    Operands {
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
};

pub const H0XFF: Opcodes = Opcodes {
    mnemonic: "RST",
    cycles: [16, 0],
    operands: [
    Operands {
        name: "$38",
        bytes: 0,
        immediate: true
    },
    Operands {
        name: "NULL",
        bytes: 0,
        immediate: false
    },
    Operands {
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
};

