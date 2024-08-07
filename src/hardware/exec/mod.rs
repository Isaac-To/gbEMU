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

/*
Functions are named as follows:
_  - prepending the function name means it is a helper function
a  - prepending an argument means it is accessing a memory address
*/


impl CPU {
    // ADC - Add with Carry to A
    fn _adc_a(&mut self, val: u8) {
        let flags = self.reg_get_flags();
        let a = self.reg_get_8(&Reg8b::A);
        let (result, carry) = a.overflowing_add(val);
        let (result, carry2) = result.overflowing_add(flags.3);
        let half_carry = (a & 0xF) + (val & 0xF) + flags.3 > 0xF;
        self.reg_set_8(&Reg8b::A, result);
        self.reg_set_flags((
            if result == 0 { 1 } else { 0 },
            0,
            half_carry as u8,
            if carry || carry2 { 1 } else { 0 },
        ));
    }
    pub fn adc_a_r8(&mut self, reg: &Reg8b) {
        self._adc_a(self.reg_get_8(reg));
    }
    pub fn adc_a_ahl(&mut self) {
        self._adc_a(self.mem_read_8(self.reg_get_16(&Reg16b::HL)));
    }
    pub fn adc_a_n8(&mut self, val: u8) {
        self._adc_a(val);
    }
    // ADD A - Add to A
    fn _add_a(&mut self, val: u8) {
        let a = self.reg_get_8(&Reg8b::A);
        let (result, carry) = a.overflowing_add(val);
        let half_carry = (a & 0xF) + (val & 0xF) > 0xF;
        self.reg_set_8(&Reg8b::A, result);
        self.reg_set_flags((
            if result == 0 { 1 } else { 0 },
            0,
            half_carry as u8,
            if carry { 1 } else { 0 },
        ));
    }
    pub fn add_a_r8(&mut self, reg: &Reg8b) {
        self._add_a(self.reg_get_8(reg));
    }
    pub fn add_a_ahl(&mut self) {
        self._add_a(self.mem_read_8(self.reg_get_16(&Reg16b::HL)));
    }
    pub fn add_a_n8(&mut self, val: u8) {
        self._add_a(val);
    }
    // ADD HL - Add to HL
    fn _add_hl(&mut self, val: u16) {
        let hl = self.reg_get_16(&Reg16b::HL);
        let (result, carry) = hl.overflowing_add(val);
        let half_carry = (hl & 0xFFF) + (val & 0xFFF) > 0xFFF;
        self.reg_set_16(&Reg16b::HL, result);
        self.reg_set_flags((
            self.reg_get_flags().0,
            0,
            half_carry as u8,
            if carry { 1 } else { 0 },
        ));
    }
    pub fn add_hl_r16(&mut self, reg: &Reg16b) {
        self._add_hl(self.reg_get_16(reg));
    }
    pub fn add_hl_sp(&mut self) {
        self._add_hl(self.reg_get_16(&Reg16b::SP));
    }
    // ADD SP
    pub fn add_sp_e8(&mut self) {
        let sp = self.reg_get_16(&Reg16b::SP);
        let e8 = self.mem_pc_read_8() as i8 as i16;
        let result = sp.wrapping_add_signed(e8);
        let flags = (
            0,
            0,
            if (sp & 0xF).wrapping_add_signed(e8 & 0xF) > 0xF {
                1
            } else {
                0
            },
            if (sp & 0xFF).wrapping_add_signed(e8 & 0xFF) > 0xFF {
                1
            } else {
                0
            },
        );
        self.reg_set_16(&Reg16b::SP, result);
        self.reg_set_flags(flags);
    }
    // AND A - Bitwise AND to A
    fn _and_a(&mut self, val: u8) {
        let a = self.reg_get_8(&Reg8b::A);
        let result = a & val;
        self.reg_set_8(&Reg8b::A, result);
        self.reg_set_flags((if result == 0 { 1 } else { 0 }, 0, 1, 0));
    }
    pub fn and_a_r8(&mut self, reg: &Reg8b) {
        self._and_a(self.reg_get_8(reg));
    }
    pub fn and_a_ahl(&mut self) {
        self._and_a(self.mem_read_8(self.reg_get_16(&Reg16b::HL)));
    }
    pub fn and_a_n8(&mut self, val: u8) {
        self._and_a(val);
    }
    // BIT u3 - Test Bit
    pub fn _bit_u3(&mut self, bit: u8, val: u8) {
        let result = val & (1 << bit);
        self.reg_set_flags((
            if result == 0 { 1 } else { 0 },
            0,
            1,
            self.reg_get_flags().3,
        ));
    }
    pub fn bit_u3_r8(&mut self, bit: u8, reg: &Reg8b) {
        self._bit_u3(bit, self.reg_get_8(reg));
    }
    pub fn bit_u3_ahl(&mut self, bit: u8) {
        self._bit_u3(bit, self.mem_read_8(self.reg_get_16(&Reg16b::HL)));
    }
    // CALL - Place address of next instruction on stack and jump to address
    pub fn call_n16(&mut self, addr: u16) {
        let pc = self.reg_get_16(&Reg16b::PC);
        self.mem_stack_push_16(pc);
        self.reg_set_16(&Reg16b::PC, addr);
    }
    pub fn call_cc_n16(&mut self, addr: u16, flag: u8) {
        if self.reg_get_flags().3 == flag {
            self.call_n16(addr);
        }
    }
    // CCF - Complement Carry Flag
    pub fn ccf(&mut self) {
        let flags = self.reg_get_flags();
        self.reg_set_flags((flags.0, 0, 0, if flags.3 == 0 { 1 } else { 0 }));
    }
    // CP A - Compare A
    fn _cp_a(&mut self, val: u8) {
        let a = self.reg_get_8(&Reg8b::A);
        let result = a.wrapping_sub(val);
        let half_carry = (a & 0xF) < (val & 0xF);
        self.reg_set_flags((
            if result == 0 { 1 } else { 0 },
            1,
            half_carry as u8,
            if a < val { 1 } else { 0 },
        ));
    }
    pub fn cp_a_r8(&mut self, reg: &Reg8b) {
        self._cp_a(self.reg_get_8(reg));
    }
    pub fn cp_a_ahl(&mut self) {
        self._cp_a(self.mem_read_8(self.reg_get_16(&Reg16b::HL)));
    }
    pub fn cp_a_n8(&mut self, val: u8) {
        self._cp_a(val);
    }
    // CPL - Complement A
    pub fn cpl(&mut self) {
        let a = self.reg_get_8(&Reg8b::A);
        self.reg_set_8(&Reg8b::A, !a);
        self.reg_set_flags((self.reg_get_flags().0, 1, 1, self.reg_get_flags().3));
    }
    // DAA - Decimal Adjust A
    pub fn daa(&mut self) {
        let mut flags = self.reg_get_flags();
        let mut a = self.reg_get_8(&Reg8b::A);
        let mut carry = 0;
        let mut offset = 0;
        // 6 is derived from the fact 16 - 10 = 6
        let lower_nibble = a & 0xF;
        let upper_nibble = a >> 4;
        if flags.2 == 1 || (!(flags.1 == 1) && lower_nibble > 9) {
            offset |= 0x6;
        }
        if flags.3 == 1 || (!(flags.1 == 1) && upper_nibble > 9) {
            offset |= 0x60;
            carry = 1;
        }
        if flags.1 == 0 {
            a = a.wrapping_add(offset);
        } else {
            a = a.wrapping_sub(offset);
        }
        flags = (if a == 0 { 1 } else { 0 }, flags.1, 0, carry);
        self.reg_set_8(&Reg8b::A, a);
        self.reg_set_flags(flags);
    }
    // DEC - Decrement
    pub fn dec_r8(&mut self, reg: &Reg8b) {
        let val = self.reg_get_8(reg).wrapping_sub(1);
        let half_carry = (val & 0xF) == 0xF;
        self.reg_set_8(reg, val);
        self.reg_set_flags((
            if val == 0 { 1 } else { 0 },
            1,
            half_carry as u8,
            self.reg_get_flags().3,
        ));
    }
    pub fn dec_ahl(&mut self) {
        let val = self
            .mem_read_8(self.reg_get_16(&Reg16b::HL))
            .wrapping_sub(1);
        let half_carry = (val & 0xF) == 0xF;
        self.mem_write_8(self.reg_get_16(&Reg16b::HL), val);
        self.reg_set_flags((
            if val == 0 { 1 } else { 0 },
            1,
            half_carry as u8,
            self.reg_get_flags().3,
        ));
    }
    pub fn dec_r16(&mut self, reg: &Reg16b) {
        let val = self.reg_get_16(reg).wrapping_sub(1);
        self.reg_set_16(reg, val);
    }
    pub fn dec_sp(&mut self) {
        let val = self.reg_get_16(&Reg16b::SP).wrapping_sub(1);
        self.reg_set_16(&Reg16b::SP, val);
    }
    // DI - Disable Interrupts
    pub fn di(&mut self) {
        self._ime = 0;
    }
    // EI - Enable Interrupts
    pub fn ei(&mut self) {
        self._interrupt_iminent = 1;
    }
    // HALT - Halt CPU
    pub fn halt(&mut self) {
        if self._ime == 1 {
            // IME set
            self._low_power = 1;
        } else {
            // IME not set
            if self._interrupt_iminent == 1 {
                // Interrupt iminent
                self._low_power = 1;
            } else {
                // No interrupt iminent
                self._low_power = 0;
                // Odd but this is how it is in documentation
                self.mem_read_16(self.reg_get_16(&Reg16b::PC) + 1);
                self.mem_read_16(self.reg_get_16(&Reg16b::PC) + 1);
            }
        }
    }
    // INC - Increment
    pub fn inc_r8(&mut self, reg: &Reg8b) {
        let val = self.reg_get_8(reg).wrapping_add(1);
        let half_carry = (val & 0xF) == 0;
        self.reg_set_8(reg, val);
        self.reg_set_flags((
            if val == 0 { 1 } else { 0 },
            0,
            half_carry as u8,
            self.reg_get_flags().3,
        ));
    }
    pub fn inc_ahl(&mut self) {
        let val = self
            .mem_read_8(self.reg_get_16(&Reg16b::HL))
            .wrapping_add(1);
        let half_carry = (val & 0xF) == 0;
        self.mem_write_8(self.reg_get_16(&Reg16b::HL), val);
        self.reg_set_flags((
            if val == 0 { 1 } else { 0 },
            0,
            half_carry as u8,
            self.reg_get_flags().3,
        ));
    }
    pub fn inc_r16(&mut self, reg: &Reg16b) {
        let val = self.reg_get_16(reg).wrapping_add(1);
        self.reg_set_16(reg, val);
    }
    pub fn inc_sp(&mut self) {
        let val = self.reg_get_16(&Reg16b::SP).wrapping_add(1);
        self.reg_set_16(&Reg16b::SP, val);
    }
    // JP - Jump
    pub fn jp_n16(&mut self, addr: u16) {
        self.reg_set_16(&Reg16b::PC, addr);
    }
    pub fn jp_cc_n16(&mut self, addr: u16, flag: u8) {
        if self.reg_get_flags().3 == flag {
            self.jp_n16(addr);
        }
    }
    pub fn jp_hl(&mut self) {
        self.reg_set_16(&Reg16b::PC, self.reg_get_16(&Reg16b::HL));
    }
    // JR - Jump Relative
    pub fn jr_n16(&mut self) {
        let s8 = self.mem_pc_read_8() as i8 as i16;
        let pc = self.reg_get_16(&Reg16b::PC);
        self.jp_n16(pc.wrapping_add_signed(s8));
    }
    pub fn jr_cc_n16(&mut self, flag: u8) {
        if self.reg_get_flags().3 == flag {
            self.jr_n16();
        }
    }
    // LD - Load
    pub fn ld_r8_r8(&mut self, dest: &Reg8b, src: &Reg8b) {
        self.reg_set_8(dest, self.reg_get_8(src));
    }
    pub fn ld_r8_n8(&mut self, dest: &Reg8b, val: u8) {
        self.reg_set_8(dest, val);
    }
    pub fn ld_r16_n16(&mut self, dest: &Reg16b, val: u16) {
        self.reg_set_16(dest, val);
    }
    pub fn ld_ahl_r8(&mut self, reg: &Reg8b) {
        self.mem_write_8(self.reg_get_16(&Reg16b::HL), self.reg_get_8(reg));
    }
    pub fn ld_ahl_n8(&mut self, val: u8) {
        self.mem_write_8(self.reg_get_16(&Reg16b::HL), val);
    }
    pub fn ld_r8_ahl(&mut self, reg: &Reg8b) {
        self.reg_set_8(reg, self.mem_read_8(self.reg_get_16(&Reg16b::HL)));
    }
    pub fn ld_ar16_a(&mut self, reg: &Reg16b) {
        self.mem_write_8(self.reg_get_16(reg), self.reg_get_8(&Reg8b::A));
    }
    pub fn ld_an16_a(&mut self, addr: u16) {
        self.mem_write_8(addr, self.reg_get_8(&Reg8b::A));
    }
    // LDH
    // NOP
    // OR A
    // POP
    // PUSH
    // RES u3
    // RET
    // RETI
    // RL
    // RLA
    // RLC
    // RLCA
    // RR
    // RRA
    // RRC
    // RRCA
    // RST
    // SBC
    // SCF
    // SET u3
    // SLA
    // SRA
    // SRL
    // STOP
    // SUB
    // SWAP
    // XOR
}
