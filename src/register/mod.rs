//! RISC-V Hypervisor Extension Registers
//!
//! This module provides access to Control and Status Registers (CSRs) defined
//! in the RISC-V Hypervisor Extension. These registers enable virtualization
//! support by providing hypervisor-level control and virtual machine state management.
//!
//! ## Register Categories
//!
//! ### Hypervisor Control Registers
//! These registers control hypervisor behavior and guest execution:
//! - `hstatus` - Hypervisor status register
//! - `hedeleg` - Hypervisor exception delegation register
//! - `hideleg` - Hypervisor interrupt delegation register
//! - `hie` - Hypervisor interrupt enable register
//! - `hip` - Hypervisor interrupt pending register
//! - `hvip` - Hypervisor virtual interrupt pending register
//! - `hcounteren` - Hypervisor counter enable register
//! - `hgatp` - Hypervisor guest address translation and protection register
//! - `htimedelta` - Hypervisor time delta register
//! - `htimedeltah` - Hypervisor time delta high register
//! - `htval` - Hypervisor trap value register
//! - `htinst` - Hypervisor trap instruction register
//! - `hgeie` - Hypervisor guest external interrupt enable register
//! - `hgeip` - Hypervisor guest external interrupt pending register
//!
//! ### Virtual Supervisor Registers
//! These registers manage virtual machine supervisor-level state:
//! - `vsstatus` - Virtual supervisor status register
//! - `vsie` - Virtual supervisor interrupt enable register
//! - `vsip` - Virtual supervisor interrupt pending register
//! - `vsepc` - Virtual supervisor exception program counter
//! - `vscause` - Virtual supervisor cause register
//! - `vstval` - Virtual supervisor trap value register
//! - `vstvec` - Virtual supervisor trap vector register
//! - `vsscratch` - Virtual supervisor scratch register
//! - `vsatp` - Virtual supervisor address translation and protection register

// Hypervisor Extension Registers
/// Hypervisor x64 register implementations
mod hypervisorx64;
pub use self::hypervisorx64::*;

// TODO: Debug/Trace Registers (shared with Debug Mode)
// TODO: Debug Mode Registers
