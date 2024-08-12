
/// Operations are listed here: https://gbdev.io/pandocs/CPU_Instruction_Set.html
/// Individually explained here: https://rgbds.gbdev.io/docs/v0.8.0/gbz80.7

use crate::hardware::{
    mem::MemoryAccess,
    reg::{Flag, Reg16b, Reg8b, RegisterAccess},
    CPU,
};

/// Functions are named as follows:
/// 
/// _  - prepending the function name means it is a helper function
/// 
/// a  - prepending an argument means it is accessing a memory address

pub trait ISA {
    fn _xor_a(&mut self, val: u8);
    fn _swap(&mut self, val: u8) -> u8;
    fn _sub_a(&mut self, val: u8);
    fn _srl(&mut self, val: u8) -> u8;
    fn _sra(&mut self, val: u8) -> u8;
    fn _sla(&mut self, val: u8) -> u8;
    fn _set_u3(&mut self, val: u8, bit: u8) -> u8;
    fn _sbc_a(&mut self, val: u8);
    fn _rrc(&mut self, val: u8) -> u8;
    fn _rr(&mut self, val: u8) -> u8;
    fn _rlc(&mut self, val: u8) -> u8;
    fn _rl(&mut self, val: u8) -> u8;
    fn push_af(&mut self);
    fn _condition(&self, flag: Flag) -> bool;
    fn _adc_a(&mut self, val: u8);
    fn adc_a_r8(&mut self, reg: &Reg8b);
    fn adc_a_ahl(&mut self);
    fn adc_a_n8(&mut self, val: u8);
    fn _add_a(&mut self, val: u8);
    fn add_a_r8(&mut self, reg: &Reg8b);
    fn add_a_ahl(&mut self);
    fn add_a_n8(&mut self, val: u8);
    fn _add_hl(&mut self, val: u16);
    fn add_hl_r16(&mut self, reg: &Reg16b);
    fn add_hl_sp(&mut self);
    fn add_sp_e8(&mut self);
    fn _and_a(&mut self, val: u8);
    fn and_a_r8(&mut self, reg: &Reg8b);
    fn and_a_ahl(&mut self);
    fn and_a_n8(&mut self, val: u8);
    fn _bit_u3(&mut self, bit: u8, val: u8);
    fn bit_u3_r8(&mut self, bit: u8, reg: &Reg8b);
    fn bit_u3_ahl(&mut self, bit: u8);
    fn call_n16(&mut self, addr: u16);
    fn call_cc_n16(&mut self, addr: u16, flag: Flag);
    fn ccf(&mut self);
    fn _cp_a(&mut self, val: u8);
    fn cp_a_r8(&mut self, reg: &Reg8b);
    fn cp_a_ahl(&mut self);
    fn cp_a_n8(&mut self, val: u8);
    fn cpl(&mut self);
    fn daa(&mut self);
    fn dec_r8(&mut self, reg: &Reg8b);
    fn dec_ahl(&mut self);
    fn dec_r16(&mut self, reg: &Reg16b);
    fn dec_sp(&mut self);
    fn di(&mut self);
    fn ei(&mut self);
    fn halt(&mut self);
    fn inc_r8(&mut self, reg: &Reg8b);
    fn inc_ahl(&mut self);
    fn inc_r16(&mut self, reg: &Reg16b);
    fn inc_sp(&mut self);
    fn jp_n16(&mut self, addr: u16);
    fn jp_cc_n16(&mut self, addr: u16, flag: Flag);
    fn jp_hl(&mut self);
    fn jr_n16(&mut self);
    fn jr_cc_n16(&mut self, flag: Flag);
    fn ld_r8_r8(&mut self, dest: &Reg8b, src: &Reg8b);
    fn ld_r8_n8(&mut self, dest: &Reg8b, val: u8);
    fn ld_r16_n16(&mut self, dest: &Reg16b, val: u16);
    fn ld_ahl_r8(&mut self, reg: &Reg8b);
    fn ld_ahl_n8(&mut self, val: u8);
    fn ld_r8_ahl(&mut self, reg: &Reg8b);
    fn ld_ar16_a(&mut self, reg: &Reg16b);
    fn ld_an16_a(&mut self, addr: u16);
    fn ldh_an16_a(&mut self, addr: u16);
    fn ldh_ac_a(&mut self);
    fn ld_a_ar16(&mut self, reg: &Reg16b);
    fn ld_a_an16(&mut self, addr: u16);
    fn ldh_a_an16(&mut self, addr: u16);
    fn ldh_a_ac(&mut self);
    fn ld_ahli_a(&mut self);
    fn ld_ahld_a(&mut self);
    fn ld_a_ahld(&mut self);
    fn ld_a_ahli(&mut self);
    fn ld_sp_n16(&mut self, val: u16);
    fn ld_sp_hl(&mut self);
    fn ld_an16_sp(&mut self, addr: u16);
    fn ld_hl_sppe8(&mut self);
    fn nop(&mut self);
    fn _or_a(&mut self, val: u8);
    fn or_a_r8(&mut self, reg: &Reg8b);
    fn or_a_ahl(&mut self);
    fn or_a_n8(&mut self, val: u8);
    fn pop_af(&mut self);
    fn pop_r16(&mut self, reg: &Reg16b);
    fn push_r16(&mut self, reg: &Reg16b);
    fn res_u3_r8(&mut self, bit: u8, reg: &Reg8b);
    fn res_u3_ahl(&mut self, bit: u8);
    fn ret(&mut self);
    fn ret_cc(&mut self, flag: Flag);
    fn reti(&mut self);
    fn rl_r8(&mut self, reg: &Reg8b);
    fn rl_ahl(&mut self);
    fn rla(&mut self);
    fn rlc_r8(&mut self, reg: &Reg8b);
    fn rlc_ahl(&mut self);
    fn rlca(&mut self);
    fn rr_r8(&mut self, reg: &Reg8b);
    fn rr_ahl(&mut self);
    fn rra(&mut self);
    fn rrc_r8(&mut self, reg: &Reg8b);
    fn rrc_ahl(&mut self);
    fn rrca(&mut self);
    fn rst(&mut self, addr: u16);
    fn sbc_a_r8(&mut self, reg: &Reg8b);
    fn sbc_a_ahl(&mut self);
    fn sbc_a_n8(&mut self, val: u8);
    fn scf(&mut self);
    fn set_u3_r8(&mut self, reg: &Reg8b, bit: u8);
    fn set_u3_ahl(&mut self, bit: u8);
    fn sla_r8(&mut self, reg: &Reg8b);
    fn sla_ahl(&mut self);
    fn sra_r8(&mut self, reg: &Reg8b);
    fn sra_ahl(&mut self);
    fn srl_r8(&mut self, reg: &Reg8b);
    fn srl_ahl(&mut self);
    fn stop(&mut self);
    fn sub_a_r8(&mut self, reg: &Reg8b);
    fn sub_a_ahl(&mut self);
    fn sub_a_n8(&mut self, val: u8);
    fn swap_r8(&mut self, reg: &Reg8b);
    fn swap_ahl(&mut self);
    fn xor_a_r8(&mut self, reg: &Reg8b);
    fn xor_a_ahl(&mut self);
    fn xor_a_n8(&mut self, val: u8);
}

impl ISA for CPU {
    /// A helper function to get the value of a flag
    /// 
    /// Intended for use in conditional instructions
    fn _condition(&self, flag: Flag) -> bool {
        let flags = self.reg_get_flags();
        match flag {
            Flag::Zero => flags.0 == 1,
            Flag::Subtract => flags.1 == 1,
            Flag::HalfCarry => flags.2 == 1,
            Flag::Carry => flags.3 == 1,
        }
    }
    /// Helper function for ADC A - Add with Carry to A
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
    /// Add with Carry to A from 8-bit register
    fn adc_a_r8(&mut self, reg: &Reg8b) {
        self._adc_a(self.reg_get_8(reg));
    }
    /// Add with Carry to A from memory address in HL
    fn adc_a_ahl(&mut self) {
        self._adc_a(self.mem_read_8(self.reg_get_16(&Reg16b::HL)));
    }
    /// Add with Carry to A from 8-bit immediate value
    fn adc_a_n8(&mut self, val: u8) {
        self._adc_a(val);
    }
    /// Helper function for ADD A - Add to A
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
    /// Add to A from 8-bit register
    fn add_a_r8(&mut self, reg: &Reg8b) {
        self._add_a(self.reg_get_8(reg));
    }
    /// Add to A from memory address in HL
    fn add_a_ahl(&mut self) {
        self._add_a(self.mem_read_8(self.reg_get_16(&Reg16b::HL)));
    }
    /// Add to A from 8-bit immediate value
    fn add_a_n8(&mut self, val: u8) {
        self._add_a(val);
    }
    // Helper function for ADD HL - Add to HL
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
    /// Add to HL from 16-bit register
    fn add_hl_r16(&mut self, reg: &Reg16b) {
        self._add_hl(self.reg_get_16(reg));
    }
    /// Add to HL from SP
    fn add_hl_sp(&mut self) {
        self._add_hl(self.reg_get_16(&Reg16b::SP));
    }
    /// Add to SP with signed 8-bit immediate value
    fn add_sp_e8(&mut self) {
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
    /// Helper function for AND A - Logical AND with A
    fn _and_a(&mut self, val: u8) {
        let a = self.reg_get_8(&Reg8b::A);
        let result = a & val;
        self.reg_set_8(&Reg8b::A, result);
        self.reg_set_flags((if result == 0 { 1 } else { 0 }, 0, 1, 0));
    }
    /// Logical AND with A from 8-bit register
    fn and_a_r8(&mut self, reg: &Reg8b) {
        self._and_a(self.reg_get_8(reg));
    }
    /// Logical AND with A from memory address in HL
    fn and_a_ahl(&mut self) {
        self._and_a(self.mem_read_8(self.reg_get_16(&Reg16b::HL)));
    }
    /// Logical AND with A from 8-bit immediate value
    fn and_a_n8(&mut self, val: u8) {
        self._and_a(val);
    }
    /// Helper function for BIT u3 - Test bit
    fn _bit_u3(&mut self, bit: u8, val: u8) {
        let result = val & (1 << bit);
        self.reg_set_flags((
            if result == 0 { 1 } else { 0 },
            0,
            1,
            self.reg_get_flags().3,
        ));
    }
    /// Test bit u3 in 8-bit register
    fn bit_u3_r8(&mut self, bit: u8, reg: &Reg8b) {
        self._bit_u3(bit, self.reg_get_8(reg));
    }
    /// Test bit u3 in memory address in HL
    fn bit_u3_ahl(&mut self, bit: u8) {
        self._bit_u3(bit, self.mem_read_8(self.reg_get_16(&Reg16b::HL)));
    }
    /// Place address of next instruction on stack and jump to address
    fn call_n16(&mut self, addr: u16) {
        let pc = self.reg_get_16(&Reg16b::PC);
        self.mem_stack_push_16(pc);
        self.reg_set_16(&Reg16b::PC, addr);
    }
    /// Conditional CALL
    fn call_cc_n16(&mut self, addr: u16, flag: Flag) {
        if self._condition(flag) {
            self.call_n16(addr);
        }
    }
    /// Complement Carry Flag
    fn ccf(&mut self) {
        let flags = self.reg_get_flags();
        self.reg_set_flags((flags.0, 0, 0, if flags.3 == 0 { 1 } else { 0 }));
    }
    /// Helper function for CP A - Compare A
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
    /// Compare A with 8-bit register
    fn cp_a_r8(&mut self, reg: &Reg8b) {
        self._cp_a(self.reg_get_8(reg));
    }
    /// Compare A with memory address in HL
    fn cp_a_ahl(&mut self) {
        self._cp_a(self.mem_read_8(self.reg_get_16(&Reg16b::HL)));
    }
    /// Compare A with 8-bit immediate value
    fn cp_a_n8(&mut self, val: u8) {
        self._cp_a(val);
    }
    /// Complement A
    fn cpl(&mut self) {
        let a = self.reg_get_8(&Reg8b::A);
        self.reg_set_8(&Reg8b::A, !a);
        self.reg_set_flags((self.reg_get_flags().0, 1, 1, self.reg_get_flags().3));
    }
    /// Decimal Adjust A
    fn daa(&mut self) {
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
    /// Decrement 8-bit register
    fn dec_r8(&mut self, reg: &Reg8b) {
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
    /// Decrement memory address in HL
    fn dec_ahl(&mut self) {
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
    /// Decrement 16-bit register
    fn dec_r16(&mut self, reg: &Reg16b) {
        let val = self.reg_get_16(reg).wrapping_sub(1);
        self.reg_set_16(reg, val);
    }
    /// Decrement Stack Pointer
    fn dec_sp(&mut self) {
        let val = self.reg_get_16(&Reg16b::SP).wrapping_sub(1);
        self.reg_set_16(&Reg16b::SP, val);
    }
    /// Disable Interrupts
    fn di(&mut self) {
        self._ime = 0;
    }
    /// Enable Interrupts
    fn ei(&mut self) {
        self._interrupt_iminent = 1;
    }
    /// Halt CPU
    fn halt(&mut self) {
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
    /// Increment 8-bit register
    fn inc_r8(&mut self, reg: &Reg8b) {
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
    /// Increment memory address in HL
    fn inc_ahl(&mut self) {
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
    /// Increment 16-bit register
    fn inc_r16(&mut self, reg: &Reg16b) {
        let val = self.reg_get_16(reg).wrapping_add(1);
        self.reg_set_16(reg, val);
    }
    /// Increment Stack Pointer
    fn inc_sp(&mut self) {
        let val = self.reg_get_16(&Reg16b::SP).wrapping_add(1);
        self.reg_set_16(&Reg16b::SP, val);
    }
    /// Jump
    fn jp_n16(&mut self, addr: u16) {
        self.reg_set_16(&Reg16b::PC, addr);
    }
    /// Conditional Jump
    fn jp_cc_n16(&mut self, addr: u16, flag: Flag) {
        if self._condition(flag) {
            self.jp_n16(addr);
        }
    }
    /// Jump to address in HL
    fn jp_hl(&mut self) {
        self.reg_set_16(&Reg16b::PC, self.reg_get_16(&Reg16b::HL));
    }
    /// Jump Relative
    fn jr_n16(&mut self) {
        let s8 = self.mem_pc_read_8() as i8 as i16;
        let pc = self.reg_get_16(&Reg16b::PC);
        self.jp_n16(pc.wrapping_add_signed(s8));
    }
    /// Conditional Jump Relative
    fn jr_cc_n16(&mut self, flag: Flag) {
        if self._condition(flag) {
            self.jr_n16();
        }
    }
    /// Load 8-bit register to 8-bit register
    fn ld_r8_r8(&mut self, dest: &Reg8b, src: &Reg8b) {
        self.reg_set_8(dest, self.reg_get_8(src));
    }
    /// Load immediate 8-bit value to 8-bit register
    fn ld_r8_n8(&mut self, dest: &Reg8b, val: u8) {
        self.reg_set_8(dest, val);
    }
    /// Load immediate 16-bit value to 16-bit register
    fn ld_r16_n16(&mut self, dest: &Reg16b, val: u16) {
        self.reg_set_16(dest, val);
    }
    /// Load 8-bit register to memory address in HL
    fn ld_ahl_r8(&mut self, reg: &Reg8b) {
        self.mem_write_8(self.reg_get_16(&Reg16b::HL), self.reg_get_8(reg));
    }
    /// Load 8-bit immediate value to memory address in HL
    fn ld_ahl_n8(&mut self, val: u8) {
        self.mem_write_8(self.reg_get_16(&Reg16b::HL), val);
    }
    /// Load memory address in HL to 8-bit register
    fn ld_r8_ahl(&mut self, reg: &Reg8b) {
        self.reg_set_8(reg, self.mem_read_8(self.reg_get_16(&Reg16b::HL)));
    }
    /// Load register A to memory address in 16-bit register
    fn ld_ar16_a(&mut self, reg: &Reg16b) {
        self.mem_write_8(self.reg_get_16(reg), self.reg_get_8(&Reg8b::A));
    }
    /// Load register A to memory address in 16-bit immediate value
    fn ld_an16_a(&mut self, addr: u16) {
        self.mem_write_8(addr, self.reg_get_8(&Reg8b::A));
    }
    /// Load register A to memory address in 16-bit immediate value (High RAM)
    fn ldh_an16_a(&mut self, addr: u16) {
        if 0xFF00 < addr && addr < 0xFFFF {
            self.mem_write_8(addr, self.reg_get_8(&Reg8b::A));
        }
    }
    /// Load register A to memory address in 0xFF00 + register C
    fn ldh_ac_a(&mut self) {
        let addr = 0xFF00 + self.reg_get_8(&Reg8b::C) as u16;
        self.mem_write_8(addr, self.reg_get_8(&Reg8b::A));
    }
    /// Load memory address in 16-bit register to register A
    fn ld_a_ar16(&mut self, reg: &Reg16b) {
        self.reg_set_8(&Reg8b::A, self.mem_read_8(self.reg_get_16(reg)));
    }
    /// Load memory address in 16-bit immediate value to register A
    fn ld_a_an16(&mut self, addr: u16) {
        self.reg_set_8(&Reg8b::A, self.mem_read_8(addr));
    }
    /// Load memory address in 16-bit immediate value to register A (High RAM)
    fn ldh_a_an16(&mut self, addr: u16) {
        if 0xFF00 < addr && addr < 0xFFFF {
            self.reg_set_8(&Reg8b::A, self.mem_read_8(addr));
        }
    }
    /// Load memory address in 0xFF00 + register C to register A
    fn ldh_a_ac(&mut self) {
        let addr = 0xFF00 + self.reg_get_8(&Reg8b::C) as u16;
        self.reg_set_8(&Reg8b::A, self.mem_read_8(addr));
    }
    /// Load register A to memory address in HL and increment HL
    fn ld_ahli_a(&mut self) {
        let hl = self.reg_get_16(&Reg16b::HL);
        self.reg_set_8(&Reg8b::A, self.mem_read_8(hl));
        self.reg_set_16(&Reg16b::HL, hl.wrapping_add(1));
    }
    /// Load register A to memory address in HL and decrement HL
    fn ld_ahld_a(&mut self) {
        let hl = self.reg_get_16(&Reg16b::HL);
        self.reg_set_8(&Reg8b::A, self.mem_read_8(hl));
        self.reg_set_16(&Reg16b::HL, hl.wrapping_sub(1));
    }
    /// Load memory address in HL to register A and decrement HL
    fn ld_a_ahld(&mut self) {
        let hl = self.reg_get_16(&Reg16b::HL);
        self.reg_set_8(&Reg8b::A, self.mem_read_8(hl));
        self.reg_set_16(&Reg16b::HL, hl.wrapping_sub(1));
    }
    /// Load memory address in HL to register A and increment HL
    fn ld_a_ahli(&mut self) {
        let hl = self.reg_get_16(&Reg16b::HL);
        self.reg_set_8(&Reg8b::A, self.mem_read_8(hl));
        self.reg_set_16(&Reg16b::HL, hl.wrapping_add(1));
    }
    /// Load 16-bit immediate value to SP
    fn ld_sp_n16(&mut self, val: u16) {
        self.reg_set_16(&Reg16b::SP, val);
    }
    /// Load register SP to memory address in 16-bit immediate value
    fn ld_an16_sp(&mut self, addr: u16) {
        self.mem_write_16(addr, self.reg_get_16(&Reg16b::SP));
    }
    /// Load register SP with added signed 8-bit immediate value to HL
    fn ld_hl_sppe8(&mut self) {
        let sp = self.reg_get_16(&Reg16b::SP);
        let e8 = self.mem_pc_read_8() as i8 as i16;
        let result = sp.wrapping_add_signed(e8);
        self.reg_set_16(&Reg16b::HL, result);
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
        self.reg_set_flags(flags);
    }
    /// Load HL to SP
    fn ld_sp_hl(&mut self) {
        self.reg_set_16(&Reg16b::SP, self.reg_get_16(&Reg16b::HL));
    }
    /// No Operation
    fn nop(&mut self) {}
    /// Helper function for OR A - Logical OR with A
    fn _or_a(&mut self, val: u8) {
        let a = self.reg_get_8(&Reg8b::A);
        let result = a | val;
        self.reg_set_8(&Reg8b::A, result);
        self.reg_set_flags((if result == 0 { 1 } else { 0 }, 0, 0, 0));
    }
    /// Logical OR with A from 8-bit register
    fn or_a_r8(&mut self, reg: &Reg8b) {
        self._or_a(self.reg_get_8(reg));
    }
    /// Logical OR with A from memory address in HL
    fn or_a_ahl(&mut self) {
        self._or_a(self.mem_read_8(self.reg_get_16(&Reg16b::HL)));
    }
    /// Logical OR with A from 8-bit immediate value
    fn or_a_n8(&mut self, val: u8) {
        self._or_a(val);
    }
    /// Pop 16-bit value from stack to register AF
    fn pop_af(&mut self) {
        let val = self.mem_stack_pop_16();
        self.reg_set_16(&Reg16b::AF, val);
    }
    /// Pop 16-bit value from stack to 16-bit register
    fn pop_r16(&mut self, reg: &Reg16b) {
        let val = self.mem_stack_pop_16();
        self.reg_set_16(reg, val);
    }
    /// Push 16-bit value from register AF to stack
    fn push_af(&mut self) {
        let val = self.reg_get_16(&Reg16b::AF);
        self.mem_stack_push_16(val);
    }
    /// Push 16-bit value from 16-bit register to stack
    fn push_r16(&mut self, reg: &Reg16b) {
        let val = self.reg_get_16(reg);
        self.mem_stack_push_16(val);
    }
    /// Reset bit u3 in 8-bit register
    fn res_u3_r8(&mut self, bit: u8, reg: &Reg8b) {
        let val = self.reg_get_8(reg) & !(1 << bit);
        self.reg_set_8(reg, val);
    }
    /// Reset bit u3 in memory address in HL
    fn res_u3_ahl(&mut self, bit: u8) {
        let val = self.mem_read_8(self.reg_get_16(&Reg16b::HL)) & !(1 << bit);
        self.mem_write_8(self.reg_get_16(&Reg16b::HL), val);
    }
    /// Return from subroutine
    /// Something akin to POP PC
    fn ret(&mut self) {
        self.pop_r16(&Reg16b::PC);
    }
    /// Conditional Return
    fn ret_cc(&mut self, flag: Flag) {
        if self._condition(flag) {
            self.ret();
        }
    }
    /// Return from interrupt
    /// Enable interrupts and return from interrupt
    fn reti(&mut self) {
        self._ime = 1;
        self.ret();
    }
    /// Helper function for RL - Rotate left through Carry Flag
    fn _rl(&mut self, val: u8) -> u8 {
        let flags = self.reg_get_flags();
        let carry = (val & 0x80) >> 7;
        let result = (val << 1) | flags.3;
        self.reg_set_flags((if result == 0 { 1 } else { 0 }, 0, 0, carry));
        result
    }
    /// Rotate left 8-bit register through Carry Flag
    fn rl_r8(&mut self, reg: &Reg8b) {
        let val = self.reg_get_8(reg);
        let result = self._rl(val);
        self.reg_set_8(reg, result);
    }
    /// Rotate left memory address in HL through Carry Flag
    fn rl_ahl(&mut self) {
        let val = self.mem_read_8(self.reg_get_16(&Reg16b::HL));
        let result = self._rl(val);
        self.mem_write_8(self.reg_get_16(&Reg16b::HL), result);
    }
    /// Rotate register A left through Carry Flag
    fn rla(&mut self) {
        let val = self.reg_get_8(&Reg8b::A);
        let result = self._rl(val);
        self.reg_set_8(&Reg8b::A, result);
    }
    /// Helper function for RLC - Rotate left circular
    fn _rlc(&mut self, val: u8) -> u8 {
        let carry = (val & 0x80) >> 7;
        let result = (val << 1) | carry;
        self.reg_set_flags((if result == 0 { 1 } else { 0 }, 0, 0, carry));
        result
    }
    /// Rotate 8-bit register left circular
    fn rlc_r8(&mut self, reg: &Reg8b) {
        let val = self._rlc(self.reg_get_8(reg));
        self.reg_set_8(reg, val);
    }
    /// Rotate memory address in HL left circular
    fn rlc_ahl(&mut self) {
        let loc = self.reg_get_16(&Reg16b::HL);
        let val = self._rlc(self.mem_read_8(loc));
        self.mem_write_8(loc, val);
    }
    /// Rotate register A left circular
    fn rlca(&mut self) {
        let val = self._rlc(self.reg_get_8(&Reg8b::A));
        self.reg_set_8(&Reg8b::A, val);
    }
    /// Helper function for RR - Rotate Right through Carry Flag
    fn _rr(&mut self, val: u8) -> u8 {
        let flags = self.reg_get_flags();
        let carry = val & 1;
        let result = (val >> 1) | (flags.3 << 7);
        self.reg_set_flags((if result == 0 { 1 } else { 0 }, 0, 0, carry));
        result
    }
    /// Rotate 8-bit register right through Carry Flag
    fn rr_r8(&mut self, reg: &Reg8b) {
        let val = self.reg_get_8(reg);
        let result = self._rr(val);
        self.reg_set_8(reg, result);
    }
    /// Rotate memory address in HL right through Carry Flag
    fn rr_ahl(&mut self) {
        let val = self.mem_read_8(self.reg_get_16(&Reg16b::HL));
        let result = self._rr(val);
        self.mem_write_8(self.reg_get_16(&Reg16b::HL), result);
    }
    /// Rotate register A right through Carry Flag
    fn rra(&mut self) {
        let val = self.reg_get_8(&Reg8b::A);
        let result = self._rr(val);
        self.reg_set_8(&Reg8b::A, result);
    }
    /// Helper function for RRC - Rotate Right Circular
    fn _rrc(&mut self, val: u8) -> u8 {
        let carry = val & 1;
        let result = (val >> 1) | (carry << 7);
        self.reg_set_flags((if result == 0 { 1 } else { 0 }, 0, 0, carry));
        result
    }
    /// Rotate 8-bit register right circular
    fn rrc_r8(&mut self, reg: &Reg8b) {
        let val = self._rrc(self.reg_get_8(reg));
        self.reg_set_8(reg, val);
    }
    /// Rotate memory address in HL right circular
    fn rrc_ahl(&mut self) {
        let loc = self.reg_get_16(&Reg16b::HL);
        let val = self._rrc(self.mem_read_8(loc));
        self.mem_write_8(loc, val);
    }
    /// Rotate register A right circular
    fn rrca(&mut self) {
        let val = self._rrc(self.reg_get_8(&Reg8b::A));
        self.reg_set_8(&Reg8b::A, val);
    }
    /// RST - Restart
    /// Something like "CALL" for vecs but faster
    fn rst(&mut self, vec: u16) {
        self.push_r16(&Reg16b::PC);
        self.reg_set_16(&Reg16b::PC, vec);
    }
    /// Helper function for SBC A - Subtract with Carry from A
    fn _sbc_a(&mut self, val: u8) {
        let a = self.reg_get_8(&Reg8b::A);
        let carry = self.reg_get_flags().3;
        let result = a.wrapping_sub(val).wrapping_sub(carry);
        let flags = (
            if result == 0 { 1 } else { 0 },
            1,
            ((a & 0xf) < ((val & 0xf) + carry)) as u8,
            ((a as u16) < ((val as u16) + (carry as u16))) as u8,
        );
        self.reg_set_flags(flags);
        self.reg_set_8(&Reg8b::A, result);
    }
    /// Subtract with Carry from A with 8-bit register
    fn sbc_a_r8(&mut self, reg: &Reg8b) {
        self._sbc_a(self.reg_get_8(reg));
    }
    /// Subtract with Carry from A with memory address in HL
    fn sbc_a_ahl(&mut self) {
        self._sbc_a(self.mem_read_8(self.reg_get_16(&Reg16b::HL)));
    }
    /// Subtract with Carry from A with 8-bit immediate value
    fn sbc_a_n8(&mut self, val: u8) {
        self._sbc_a(val);
    }
    /// Set Carry Flag
    fn scf(&mut self) {
        let flags = self.reg_get_flags();
        self.reg_set_flags((flags.0, 0, 0, 1));
    }
    /// Helper function for SET u3 - Set bit u3
    fn _set_u3(&mut self, val: u8, bit: u8) -> u8 {
        val | (1 << bit)
    }
    /// Set bit u3 in 8-bit register
    fn set_u3_r8(&mut self, reg: &Reg8b, bit: u8) {
        let val = self._set_u3(self.reg_get_8(reg), bit);
        self.reg_set_8(reg, val);
    }
    /// Set bit u3 in memory address in HL
    fn set_u3_ahl(&mut self, bit: u8) {
        let loc = self.reg_get_16(&Reg16b::HL);
        let val = self._set_u3(self.mem_read_8(loc), bit);
        self.mem_write_8(loc, val);
    }
    /// Helper function for SLA - Shift Left Arithmetic
    fn _sla(&mut self, val: u8) -> u8 {
        let carry = (val & 0x80) >> 7;
        let result = val << 1;
        self.reg_set_flags((if result == 0 { 1 } else { 0 }, 0, 0, carry));
        result
    }
    /// Shift Left Arithmetic 8-bit register
    fn sla_r8(&mut self, reg: &Reg8b) {
        let val = self._sla(self.reg_get_8(reg));
        self.reg_set_8(reg, val);
    }
    /// Shift Left Arithmetic memory address in HL
    fn sla_ahl(&mut self) {
        let loc = self.reg_get_16(&Reg16b::HL);
        let val = self._sla(self.mem_read_8(loc));
        self.mem_write_8(loc, val);
    }
    /// Helper function for SRA - Shift Right Arithmetic
    fn _sra(&mut self, val: u8) -> u8 {
        let carry = val & 1;
        let result = (val >> 1) | (val & 0x80);
        self.reg_set_flags((if result == 0 { 1 } else { 0 }, 0, 0, carry));
        result
    }
    /// Shift Right Arithmetic 8-bit register
    fn sra_r8(&mut self, reg: &Reg8b) {
        let val = self._sra(self.reg_get_8(reg));
        self.reg_set_8(reg, val);
    }
    /// Shift Right Arithmetic memory address in HL
    fn sra_ahl(&mut self) {
        let loc = self.reg_get_16(&Reg16b::HL);
        let val = self._sra(self.mem_read_8(loc));
        self.mem_write_8(loc, val);
    }
    /// Helper function for SRL - Shift Right Logical
    fn _srl(&mut self, val: u8) -> u8 {
        let carry = val & 1;
        let result = val >> 1;
        self.reg_set_flags((if result == 0 { 1 } else { 0 }, 0, 0, carry));
        result
    }
    /// Shift Right Logical 8-bit register
    fn srl_r8(&mut self, reg: &Reg8b) {
        let val = self._srl(self.reg_get_8(reg));
        self.reg_set_8(reg, val);
    }
    /// Shift Right Logical memory address in HL
    fn srl_ahl(&mut self) {
        let loc = self.reg_get_16(&Reg16b::HL);
        let val = self._srl(self.mem_read_8(loc));
        self.mem_write_8(loc, val);
    }
    /// Halt CPU until button press
    /// Enters low power mode, also used for double and normal speed modes
    fn stop(&mut self) {
        self._low_power = 1;
    }
    /// Helper function for SUB A - Subtract from A
    fn _sub_a(&mut self, val: u8) {
        let a = self.reg_get_8(&Reg8b::A);
        let result = a.wrapping_sub(val);
        self.reg_set_flags((
            if result == 0 { 1 } else { 0 },
            0,
            (a & 0xf < val & 0xf) as u8,
            (a < val) as u8,
        ));
        self.reg_set_8(&Reg8b::A, result);
    }
    /// Subtract from A with 8-bit register
    fn sub_a_r8(&mut self, reg: &Reg8b) {
        self._sub_a(self.reg_get_8(reg));
    }
    /// Subtract from A with memory address in HL
    fn sub_a_ahl(&mut self) {
        self._sub_a(self.mem_read_8(self.reg_get_16(&Reg16b::HL)));
    }
    /// Subtract from A with 8-bit immediate value
    fn sub_a_n8(&mut self, val: u8) {
        self._sub_a(val);
    }
    /// Helper function for SWAP - Swap nibbles
    fn _swap(&mut self, val: u8) -> u8 {
        let val = (val << 4) | (val >> 4);
        let flags = (if val == 0 { 1 } else { 0 }, 0, 0, 0);
        self.reg_set_flags(flags);
        val
    }
    /// Swap nibbles in 8-bit register
    fn swap_r8(&mut self, reg: &Reg8b) {
        let val = self._swap(self.reg_get_8(reg));
        self.reg_set_8(reg, val);
    }
    /// Swap nibbles in memory address in HL
    fn swap_ahl(&mut self) {
        let loc = self.reg_get_16(&Reg16b::HL);
        let val = self._swap(self.mem_read_8(loc));
        self.mem_write_8(loc, val);
    }
    // Helper function for XOR A - Logical XOR with A
    fn _xor_a(&mut self, val: u8) {
        let a = self.reg_get_8(&Reg8b::A);
        let result = a ^ val;
        let flags = (if result == 0 { 1 } else { 0 }, 0, 0, 0);
        self.reg_set_flags(flags);
        self.reg_set_8(&Reg8b::A, result);
    }
    /// Logical XOR with A from 8-bit register
    fn xor_a_r8(&mut self, reg: &Reg8b) {
        self._xor_a(self.reg_get_8(reg));
    }
    /// Logical XOR with A from memory address in HL
    fn xor_a_ahl(&mut self) {
        self._xor_a(self.mem_read_8(self.reg_get_16(&Reg16b::HL)));
    }
    /// Logical XOR with A from 8-bit immediate value
    fn xor_a_n8(&mut self, val: u8) {
        self._xor_a(val);
    }
}
