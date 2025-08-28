//! Hypervisor Interrupt Enable Register.
//!
//! The `hie` register controls which virtual supervisor interrupts can be taken
//! when executing in VS-mode or VU-mode. It contains enable bits for:
//! - Virtual supervisor software interrupts (VSSIE)
//! - Virtual supervisor timer interrupts (VSTIE)  
//! - Virtual supervisor external interrupts (VSEIE)
//!
//! This register works in conjunction with the `hvip` register (interrupt pending)
//! and guest interrupt delegation to manage virtualized interrupt delivery.

use bit_field::BitField;
use riscv::{clear, read_csr_as, set, set_clear_csr, write_csr};

/// Hypervisor Interrupt Enable Register.
#[derive(Copy, Clone, Debug)]
pub struct Hie {
    bits: usize,
}

impl Hie {
    #[inline]
    pub fn bits(&self) -> usize {
        return self.bits;
    }
    #[inline]
    pub fn from_bits(x: usize) -> Self {
        return Hie { bits: x };
    }
    #[inline]
    pub unsafe fn write(&self) {
        _write(self.bits);
    }
    /// Returns the status of the virtual supervisor software interrupt enable.
    #[inline]
    pub fn vssie(&self) -> bool {
        self.bits.get_bit(2)
    }
    /// Sets the status of the virtual supervisor software interrupt enable.
    #[inline]
    pub fn set_vssie(&mut self, val: bool) {
        self.bits.set_bit(2, val);
    }
    /// Returns the status of the virtual supervisor timer interrupt enable.
    #[inline]
    pub fn vstie(&self) -> bool {
        self.bits.get_bit(6)
    }
    /// Sets the status of the virtual supervisor timer interrupt enable.
    #[inline]
    pub fn set_vstie(&mut self, val: bool) {
        self.bits.set_bit(6, val);
    }
    /// Returns the status of the virtual supervisor external interrupt enable.
    #[inline]
    pub fn vseie(&self) -> bool {
        self.bits.get_bit(10)
    }
    /// Sets the status of the virtual supervisor external interrupt enable.
    #[inline]
    pub fn set_vseie(&mut self, val: bool) {
        self.bits.set_bit(10, val);
    }
    /// Returns the status of the supervisor guest external interrupt enable.
    #[inline]
    pub fn sgeie(&self) -> bool {
        self.bits.get_bit(12)
    }
    /// Sets the status of the supervisor guest external interrupt enable.
    #[inline]
    pub fn set_sgeie(&mut self, val: bool) {
        self.bits.set_bit(12, val);
    }
}

read_csr_as!(Hie, 0x604);
write_csr!(0x604);
set!(0x604);
clear!(0x604);

// bit ops
set_clear_csr!(
    /// Virtual supervisor software interrupt enable.
    , set_vssie, clear_vssie, 1 << 2);
set_clear_csr!(
    /// Virtual supervisor timer interrupt enable.
    , set_vstie, clear_vstie, 1 << 6);
set_clear_csr!(
    /// Virtual supervisor external interrupt enable.
    , set_vseie, clear_vseie, 1 << 10);
set_clear_csr!(
    /// Supervisor guest external interrupt enable.
    , set_sgeie, clear_sgeie, 1 << 12);

// enums
