// Operations are listed here: https://gbdev.io/pandocs/CPU_Instruction_Set.html
// Individually explained here: https://rgbds.gbdev.io/docs/v0.8.0/gbz80.7
use crate::hardware::{
    mem::MemoryAccess,
    reg::{Reg16b, Reg8b, RegisterAccess},
    CPU,
};

pub trait Execution {
    fn exec(&mut self);
}

impl Execution for CPU {
    fn exec(&mut self) {}
}

impl CPU {
    // Returns (cycles, bytes)

    // Add with carry helper function
    fn adc_8_a(&mut self, a: u8, b: u8) {
        let flags = self.reg_get_flags();
        let (result, carry_0) = a.overflowing_add(b);
        let (result, carry_1) = result.overflowing_add(flags.3 as u8);
        self.reg_set_8(Reg8b::A, result);
        self.reg_set_flags(
            result == 0,
            false,
            (a & 0x0F) + (b & 0x0F) + (flags.3 as u8) > 0x0F,
            carry_0 | carry_1,
        );
    }
    // ADC A, r8
    pub fn adc_a_r8(&mut self, reg: Reg8b) -> (u8, u8) {
        let a = self.reg_get_8(Reg8b::A);
        let b = self.reg_get_8(reg);
        self.adc_8_a(a, b);
        (1, 1)
    }
    // ADC A, (HL)
    pub fn adc_a_hl(&mut self) -> (u8, u8) {
        let a = self.reg_get_8(Reg8b::A);
        let b = self.mem_read_8(self.reg_get_16(Reg16b::HL));
        self.adc_8_a(a, b);
        (2, 1)

    }
    // ADC A, n8
    pub fn adc_a_n8(&mut self, n: u8) -> (u8, u8) {
        let a = self.reg_get_8(Reg8b::A);
        self.adc_8_a(a, n);
        (2, 2)
    }
    // Add_8 helper function
    fn add_8_a(&mut self, a: u8, b: u8) {
        let (result, carry) = a.overflowing_add(b);
        self.reg_set_8(Reg8b::A, result);
        let flags = (
            result == 0,
            false,
            (a & 0x0F) + (b & 0x0F) > 0x0F,
            carry,
        );
        self.reg_set_flags(flags.0, flags.1, flags.2, flags.3);
    }
    // ADD A, r8
    pub fn add_a_r8(&mut self, reg: Reg8b) -> (u8, u8) {
        let a = self.reg_get_8(Reg8b::A);
        let b = self.reg_get_8(reg);
        self.add_8_a(a, b);
        (1, 1)
    }
    // ADD A, (HL)
    pub fn add_a_hl(&mut self) -> (u8, u8) {
        let a = self.reg_get_8(Reg8b::A);
        let b = self.mem_read_8(self.reg_get_16(Reg16b::HL));
        self.add_8_a(a, b);
        (2, 1)
    }
    // ADD A, n8
    pub fn add_a_n8(&mut self, n: u8) -> (u8, u8) {
        let a = self.reg_get_8(Reg8b::A);
        self.add_8_a(a, n);
        (2, 2)
    }
    // Add_16 helper function
    fn add_16_hl(&mut self, a: u16, b: u16) {
        let (result, carry) = a.overflowing_add(b);
        self.reg_set_16(Reg16b::HL, result);
        let mut flags = self.reg_get_flags();
        flags = (
            flags.0,
            false,
            (a & 0x0FFF) + (b & 0x0FFF) > 0x0FFF,
            carry,
        );
        self.reg_set_flags(flags.0, flags.1, flags.2, flags.3);
    }
    // ADD HL, r16
    pub fn add_hl_r16(&mut self, reg: Reg16b) -> (u8, u8) {
        let a = self.reg_get_16(Reg16b::HL);
        let b = self.reg_get_16(reg);
        self.add_16_hl(a, b);
        (2, 1)
    }
    // ADD HL, SP
    pub fn add_hl_sp(&mut self) -> (u8, u8) {
        let a = self.reg_get_16(Reg16b::HL);
        let b = self.reg_get_16(Reg16b::SP);
        self.add_16_hl(a, b);
        (2, 1)
    }
    // ADD SP, e8
    pub fn add_sp_e8(&mut self, n: i8) -> (u8, u8) {
        let a = self.reg_get_16(Reg16b::SP);
        let b = n as u16;
        let (result, carry) = a.overflowing_add(b);
        self.reg_set_16(Reg16b::SP, result);
        let flags = (
            false,
            false,
            (a & 0x0F) + (b & 0x0F) > 0x0F,
            carry,
        );
        self.reg_set_flags(flags.0, flags.1, flags.2, flags.3);
        (4, 2)
    }
    // AND_8_A helper function
    fn and_8_a(&mut self, a: u8, b: u8) {
        let result = a & b;
        self.reg_set_8(Reg8b::A, result);
        let flags = (
            result == 0,
            false,
            true,
            false,
        );
        self.reg_set_flags(flags.0, flags.1, flags.2, flags.3);
    }
    // AND A, r8
    pub fn and_a_r8(&mut self, reg: Reg8b) -> (u8, u8) {
        let a = self.reg_get_8(Reg8b::A);
        let b = self.reg_get_8(reg);
        self.and_8_a(a, b);
        (1, 1)
    }
    // AND A, (HL)
    pub fn and_a_hl(&mut self) -> (u8, u8) {
        let a = self.reg_get_8(Reg8b::A);
        let b = self.mem_read_8(self.reg_get_16(Reg16b::HL));
        self.and_8_a(a, b);
        (2, 1)
    }
    // AND A, n8
    pub fn and_a_n8(&mut self, n: u8) -> (u8, u8) {
        let a = self.reg_get_8(Reg8b::A);
        self.and_8_a(a, n);
        (2, 2)
    }
    // BIT_8_u3 helper function
    fn bit_8_u3(&mut self, bit: u8, value: u16) {
        let result = value & (1 << bit) != 0;
        let mut flags = self.reg_get_flags();
        flags = (
            !result,
            false,
            true,
            self.reg_get_flags().3,
        );
        self.reg_set_flags(flags.0, flags.1, flags.2, flags.3);
    }
    // BIT u3, r8
    pub fn bit_u3_r8(&mut self, bit: u8, reg: Reg8b) -> (u8, u8) {
        let value = self.reg_get_8(reg) as u16;
        self.bit_8_u3(bit, value);
        (2, 2)
    }
    // BIT u3, (HL)
    pub fn bit_u3_hl(&mut self, bit: u8) -> (u8, u8) {
        let value = self.mem_read_8(self.reg_get_16(Reg16b::HL)) as u16;
        self.bit_8_u3(bit, value);
        (3, 2)
    }
    // CALL n16
    pub fn call_n16(&mut self, n: u16) -> (u8, u8) {
        let next_instruction_addr = self.reg_get_16(Reg16b::PC);
        self.mem_stack_push_16(next_instruction_addr);
        self.reg_set_16(Reg16b::PC, n);
        (6, 3)
    }
    // CALL cc, n16
    pub fn call_cc_n16(&mut self, cc: u8, n: u16) -> (u8, u8) {
        if self.reg_get_flags().0 == (cc != 0) {
            self.call_n16(n);
            (6, 3)
        } else {
            (3, 3)
        }
    }
    // CCF
    pub fn ccf(&mut self) -> (u8, u8) {
        let mut flags = self.reg_get_flags();
        flags = (
            flags.0,
            false,
            false,
            !flags.3,
        );
        self.reg_set_flags(flags.0, flags.1, flags.2, flags.3);
        (1, 1)
    }
}
