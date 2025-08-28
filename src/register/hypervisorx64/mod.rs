//! RISC-V Hypervisor Extension Registers for 64-bit Systems
//!
//! This module contains implementations of all hypervisor and virtual supervisor
//! registers for 64-bit RISC-V systems with the hypervisor extension.

/// Hypervisor counter enable register
pub mod hcounteren;
/// Hypervisor exception delegation register  
pub mod hedeleg;
/// Hypervisor guest address translation and protection register
pub mod hgatp;
/// Hypervisor guest external interrupt enable register
pub mod hgeie;
/// Hypervisor guest external interrupt pending register
pub mod hgeip;
/// Hypervisor interrupt delegation register
pub mod hideleg;
/// Hypervisor interrupt enable register
pub mod hie;
/// Hypervisor interrupt pending register
pub mod hip;
/// Hypervisor status register
pub mod hstatus;
/// Hypervisor time delta register
pub mod htimedelta;
/// Hypervisor time delta high register (for RV32)
pub mod htimedeltah;
/// Hypervisor trap instruction register
pub mod htinst;
/// Hypervisor trap value register
pub mod htval;
/// Hypervisor virtual interrupt pending register
pub mod hvip;
/// Virtual supervisor address translation and protection register
pub mod vsatp;
/// Virtual supervisor cause register
pub mod vscause;
/// Virtual supervisor exception program counter
pub mod vsepc;
/// Virtual supervisor interrupt enable register
pub mod vsie;
/// Virtual supervisor interrupt pending register
pub mod vsip;
/// Virtual supervisor scratch register
pub mod vsscratch;
/// Virtual supervisor status register
pub mod vsstatus;
/// Virtual supervisor trap value register
pub mod vstval;
/// Virtual supervisor trap vector register
pub mod vstvec;
