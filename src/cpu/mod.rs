pub struct CPU {
    // Registers
    registers: [u8; 12],
}

impl CPU {
    pub fn new() -> CPU {
        CPU { registers: [0; 12] }
    }
}
