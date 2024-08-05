use crate::cpu::CPU;
use crate::cpu::reg::{
    Reg8b,
    Reg16b,
    RegisterAccess,
};

enum Operation {
    // Operations are documented here: https://bgb.bircd.org/pandocs.htm#cpuinstructionset
    // 8-bit loads
    LdRR,
    LdRN,
    LdRHl,
    LdHlN,
    LdABc,
    LdADe,
    LdANn,
    LdBcA,
    LdDeA,
    LdNnA,
    LdAFf00N,
    LdFf00NA,
    LdAFf00C,
    LdFf00CA,
    LdHlAP1,
    LdAHlP1,
    LdHlAS1,
    LdAHlS1,
    // 16-bit loads
    LdRrNn,
    LdSpHl,
    PushRr,
    PopRr,
    // 8-bit Arithmetic/Logical
    AddAR,
    AddAN,
    AddAHl,
    AdcAR,
    AdcAN,
    AdcAHl,
    SubR,
    SubN,
    SubHl,
    SbcAR,
    SbcAN,
    SbcAHl,
    AndR,
    AndN,
    AndHl,
    XorR,
    XorN,
    OrR,
    OrN,
    OrHl,
    CpR,
    CpN,
    CpHl,
    IncR,
    IncHl,
    DecR,
    DecHl,
    Daa,
    Cpl,
    // 16-bit Arithmetic/Logical
    AddHlRr,
    IncRr,
    DecRr,
    AddSpDd,
    LdHlSppdd,
    // Rotates and Shifts
    Rcla,
    Rla,
    Rrca,
    Rra,
    RlcR,
    RlcHl,
    RlR,
    RlHl,
    RrcR,
    RrcHl,
    RrR,
    RrHl,
    SlaR,
    SlaHl,
    SwapR,
    SwapHl,
    SraR,
    SraHl,
    SrlR,
    SrlHl,
    // Single Bit Operations
    BitNR,
    BitNHl,
    SetNR,
    SetNHl,
    ResNR,
    ResNHl,
    // CPU Control
    Ccf,
    Scf,
    Nop,
    Halt,
    Stop,
    Di,
    Ei,
    // Jump
    JpNn,
    JpHl,
    JpFNn,
    JrPcDd,
    JrFPcDd,
    CallNn,
    CallFNn,
    Ret,
    RetF,
    Reti,
    RstN
}

struct INS {
    op: Operation,
    arg1: u8,
    arg2: u8,
    arg3: u8,
}

impl INS {
    pub fn new(op: Operation, arg1: u8, arg2: u8, arg3: u8) -> INS {
        INS { op, arg1, arg2, arg3 }
    }
}

impl CPU {
    fn exec(&mut self, ins: INS) {
        // TODO: Implement the instruction set
    }
}

trait Instructions {
    fn ld_r_r(&mut self, reg0: Reg8b, reg1: Reg8b);
    fn ld_r_n(&mut self, reg: Reg8b, val: u8);
    fn ld_r_hl(&mut self, reg: Reg8b);
    fn ld_hl_r(&mut self, reg: Reg8b);
    fn ld_hl_n(&mut self, val: u8);
    fn ld_a_bc(&mut self);
    fn ld_a_de(&mut self);
    fn ld_a_nn(&mut self, val: u16);
    fn ld_bc_a(&mut self);
    fn ld_de_a(&mut self);
    fn ld_nn_a(&mut self, val: u16);
    fn ld_a_ff00_n(&mut self, val: u8);
    fn ld_ff00_n_a(&mut self, val: u8);
    fn ld_a_ff00_c(&mut self);
    fn ld_ff00_c_a(&mut self);
    fn ld_hl_a_p1(&mut self);
    fn ld_a_hl_p1(&mut self);
    fn ld_hl_a_s1(&mut self);
    fn ld_a_hl_s1(&mut self);
}

impl Instructions for CPU {
    fn ld_r_r(&mut self, reg0: Reg8b, reg1: Reg8b) {
        // Load 8 bit register to 8 bit register
        self.set_register_8(reg1, self.get_register_8(reg0));
    }
    fn ld_r_n(&mut self, reg: Reg8b, val: u8) {
        // Load immediate 8 bit value to 8 bit register
        self.set_register_8(reg, val);
    }
    fn ld_r_hl(&mut self, reg: Reg8b) {
        // Load memory location pointed by HL to 8 bit register
        let addr = self.get_register_16(Reg16b::HL);
        self.set_register_8(reg, self.memory[addr as usize]);
    }
    fn ld_hl_r(&mut self, reg: Reg8b) {
        // Load 8 bit register to memory location pointed by HL
        let addr = self.get_register_16(Reg16b::HL);
        self.memory[addr as usize] = self.get_register_8(reg);
    }
    fn ld_hl_n(&mut self, val: u8) {
        // Load immediate 8 bit value to memory location pointed by HL
        let addr = self.get_register_16(Reg16b::HL);
        self.memory[addr as usize] = val;
    }
    fn ld_a_bc(&mut self) {
        // Load memory location pointed by BC to A
        let addr = self.get_register_16(Reg16b::BC);
        self.set_register_8(Reg8b::A, self.memory[addr as usize]);
    }
    fn ld_a_de(&mut self) {
        // Load memory location pointed by DE to A
        let addr = self.get_register_16(Reg16b::DE);
        self.set_register_8(Reg8b::A, self.memory[addr as usize]);
    }
    fn ld_a_nn(&mut self, val: u16) {
        // Load memory location pointed by immediate 16 bit value to A
        self.set_register_8(Reg8b::A, self.memory[val as usize]);
    }
    fn ld_bc_a(&mut self) {
        // Load A to memory location pointed by BC
        let addr = self.get_register_16(Reg16b::BC);
        self.memory[addr as usize] = self.get_register_8(Reg8b::A);
    }
    fn ld_de_a(&mut self) {
        // Load A to memory location pointed by DE
        let addr = self.get_register_16(Reg16b::DE);
        self.memory[addr as usize] = self.get_register_8(Reg8b::A);
    }
    fn ld_nn_a(&mut self, val: u16) {
        // Load A to memory location pointed by immediate 16 bit value
        self.memory[val as usize] = self.get_register_8(Reg8b::A);
    }
    fn ld_a_ff00_n(&mut self, val: u8) {
        // Load memory location pointed by 0xFF00 + immediate 8 bit value to A
        let addr = 0xFF00 + val as u16;
        self.set_register_8(Reg8b::A, self.memory[addr as usize]);
    }
    fn ld_ff00_n_a(&mut self, val: u8) {
        // Load A to memory location pointed by 0xFF00 + immediate 8 bit value
        let addr = 0xFF00 + val as u16;
        self.memory[addr as usize] = self.get_register_8(Reg8b::A);
    }
    fn ld_a_ff00_c(&mut self) {
        // Load memory location pointed by 0xFF00 + C to A
        let addr = 0xFF00 + self.get_register_8(Reg8b::C2) as u16;
        self.set_register_8(Reg8b::A, self.memory[addr as usize]);
    }
    fn ld_ff00_c_a(&mut self) {
        // Load A to memory location pointed by 0xFF00 + C
        let addr = 0xFF00 + self.get_register_8(Reg8b::C2) as u16;
        self.memory[addr as usize] = self.get_register_8(Reg8b::A);
    }
    fn ld_hl_a_p1(&mut self) {
        // Load A to memory location pointed by HL and increment HL
        let addr = self.get_register_16(Reg16b::HL);
        self.memory[addr as usize] = self.get_register_8(Reg8b::A);
        let value = self.get_register_16(Reg16b::HL) + 1;
        self.set_register_16(Reg16b::HL, value);
    }
    fn ld_a_hl_p1(&mut self) {
        // Load memory location pointed by HL to A and increment HL
        let addr = self.get_register_16(Reg16b::HL);
        self.set_register_8(Reg8b::A, self.memory[addr as usize]);
        let value = self.get_register_16(Reg16b::HL) + 1;
        self.set_register_16(Reg16b::HL, value);
    }
    fn ld_hl_a_s1(&mut self) {
        // Load A to memory location pointed by HL and decrement HL
        let addr = self.get_register_16(Reg16b::HL);
        self.memory[addr as usize] = self.get_register_8(Reg8b::A);
        let value = self.get_register_16(Reg16b::HL) - 1;
        self.set_register_16(Reg16b::HL, value);
    }
    fn ld_a_hl_s1(&mut self) {
        // Load memory location pointed by HL to A and decrement HL
        let addr = self.get_register_16(Reg16b::HL);
        self.set_register_8(Reg8b::A, self.memory[addr as usize]);
        let value = self.get_register_16(Reg16b::HL) - 1;
        self.set_register_16(Reg16b::HL, value);
    }

}
