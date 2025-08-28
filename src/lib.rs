//! RISC-V Hypervisor Extension Support
//!
//! This crate provides low-level access to RISC-V hypervisor extension registers.
//! It implements the hypervisor CSRs defined in the RISC-V Hypervisor Extension
//! specification, enabling virtualization support on RISC-V processors.
//!
//!
//! # Usage
//! ```rust,no_run
//! use riscv_h::register::hstatus;
//!
//! // Read hypervisor status register
//! let hstatus = hstatus::read();
//!
//! // Check if virtualization mode is enabled
//! if hstatus.spv() {
//!     // Handle virtualized context
//! }
//! ```

#![no_std]
#![allow(missing_docs)]

/// RISC-V hypervisor extension register definitions and access functions
pub mod register;
