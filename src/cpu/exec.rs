use super::CPU;

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

    }
}
