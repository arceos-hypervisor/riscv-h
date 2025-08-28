//! Virtual Supevisor Interrupt Enable Register.

use bit_field::BitField;
use riscv::{clear, read_csr_as, set, set_clear_csr, write_csr};

/// Virtual Supervisor Interrupt Enable Register.
#[derive(Copy, Clone, Debug)]
pub struct Vsie {
    bits: usize,
}

impl Vsie {
    /// Returns the raw bits of the register.
    #[inline]
    pub fn bits(&self) -> usize {
        return self.bits;
    }
    /// Creates a register value from raw bits.
    #[inline]
    pub fn from_bits(x: usize) -> Self {
        return Vsie { bits: x };
    }
    /// Writes the register value to the CSR.
    #[inline]
    pub unsafe fn write(&self) {
        _write(self.bits);
    }
    /// Returns the supervisor software interrupt enable.
    #[inline]
    pub fn ssie(&self) -> bool {
        self.bits.get_bit(1)
    }
    /// Sets the supervisor software interrupt enable.
    #[inline]
    pub fn set_ssie(&mut self, val: bool) {
        self.bits.set_bit(1, val);
    }
    /// Returns the supervisor timer interrupt enable.
    #[inline]
    pub fn stie(&self) -> bool {
        self.bits.get_bit(5)
    }
    /// Sets the supervisor timer interrupt enable.
    #[inline]
    pub fn set_stie(&mut self, val: bool) {
        self.bits.set_bit(5, val);
    }
    /// Returns the supervisor external interrupt enable.
    #[inline]
    pub fn seie(&self) -> bool {
        self.bits.get_bit(9)
    }
    /// Sets the supervisor external interrupt enable.
    #[inline]
    pub fn set_seie(&mut self, val: bool) {
        self.bits.set_bit(9, val);
    }
}

read_csr_as!(Vsie, 0x204);
write_csr!(0x204);
set!(0x204);
clear!(0x204);
// bit ops
set_clear_csr!(
    /// Supervisor software interrupt enable.
    , set_ssie, clear_ssie, 1 << 1);
set_clear_csr!(
    /// Supervisor timer interrupt enable.
    , set_stie, clear_stie, 1 << 5);
set_clear_csr!(
    /// Supervisor external interrupt enable.
    , set_seie, clear_seie, 1 << 9);

// enums
