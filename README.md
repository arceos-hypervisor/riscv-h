<h1 align="center">riscv-h</h1>

<p align="center">RISC-V Hypervisor Extension Register Support</p>

<div align="center">

[![Crates.io](https://img.shields.io/crates/v/riscv-h.svg)](https://crates.io/crates/riscv-h)
[![Docs.rs](https://docs.rs/riscv-h/badge.svg)](https://docs.rs/riscv-h)
[![Rust](https://img.shields.io/badge/edition-2024-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](https://github.com/arceos-hypervisor/riscv-h/blob/main/LICENSE)

</div>

English | [中文](README_CN.md)

# Introduction

RISC-V Hypervisor Extension Register Support Library, providing low-level access to Control and Status Registers (CSRs) defined in the RISC-V Hypervisor Extension specification. Supports `#![no_std]`, suitable for bare-metal and OS kernel development.

This crate exports the following core modules:

- **`register::hstatus`** — Hypervisor status register
- **`register::hgatp`** — Hypervisor guest address translation and protection register
- **`register::hvip`** — Hypervisor virtual interrupt pending register
- **`register::vsstatus`** — Virtual supervisor status register
- **`register::vsatp`** — Virtual supervisor address translation and protection register

All register types implement `Copy`, `Clone`, `Debug` traits and provide type-safe bitfield access methods.

## Quick Start

### Prerequisites

- Rust nightly toolchain
- Rust components: rust-src, clippy, rustfmt, llvm-tools
- Target platform: riscv64gc-unknown-none-elf

```bash
# Install rustup (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install nightly toolchain and components
rustup install nightly
rustup component add rust-src clippy rustfmt llvm-tools --toolchain nightly

# Add RISC-V target
rustup target add riscv64gc-unknown-none-elf --toolchain nightly
```

### Running Checks and Tests

```bash
# 1. Clone the repository
git clone https://github.com/arceos-hypervisor/riscv-h.git
cd riscv-h

# 2. Code check (format check + clippy + build + doc generation)
./scripts/check.sh

# 3. Run tests
# Run all tests (unit tests + integration tests)
./scripts/test.sh

# Run unit tests only
./scripts/test.sh unit

# Run integration tests only
./scripts/test.sh integration

# List all available test suites
./scripts/test.sh list
```

## Integration

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
riscv-h = "0.2.0"
```

### Usage Example

```rust,ignore
#![no_std]

use riscv_h::register::{hstatus, hgatp, hvip};

// Read hypervisor status register
let hstatus = hstatus::read();

// Check if in virtualized mode
if hstatus.spv() {
    // Access individual fields
    let vsxl = hstatus.vsxl();      // Virtual supervisor XLEN
    let vtw = hstatus.vtw();        // Trap WFI
    let vtsr = hstatus.vtsr();      // Trap SRET
    let vgein = hstatus.vgein();    // Virtual guest external interrupt number
    
    setup_guest_translation();
}

// Configure virtual interrupt pending
let mut hvip_val = hvip::Hvip::from_bits(0);
hvip_val.set_vssip(true);  // Set VS-mode software interrupt pending
hvip_val.set_vstip(true);  // Set VS-mode timer interrupt pending
hvip_val.set_vseip(true);  // Set VS-mode external interrupt pending

unsafe {
    hvip_val.write();
}

fn setup_guest_translation() {
    unsafe {
        // Configure guest address translation
        let mut hgatp = hgatp::Hgatp::from_bits(0);
        hgatp.set_mode(hgatp::HgatpValues::Sv48x4);  // Use Sv48x4 mode
        hgatp.set_vmid(1);                            // Set VMID
        hgatp.set_ppn(0x1000);                        // Set root page table PPN
        hgatp.write();
    }
}
```

### Exception and Interrupt Delegation

```rust,ignore
use riscv_h::register::{hedeleg, hideleg};

unsafe {
    // Delegate common exceptions to VS-mode
    hedeleg::set_ex2(true);   // Illegal instruction
    hedeleg::set_ex8(true);   // Environment call from U-mode
    hedeleg::set_ex12(true);  // Instruction page fault
    hedeleg::set_ex13(true);  // Load page fault
    hedeleg::set_ex15(true);  // Store page fault
    
    // Delegate timer and software interrupts to VS-mode
    hideleg::set_vstie(true);  // VS-mode timer interrupt
    hideleg::set_vssie(true);  // VS-mode software interrupt
}
```

## Supported Registers

### Hypervisor Control Registers

| Register | Description | CSR Address |
|----------|-------------|-------------|
| `hstatus` | Hypervisor status register | 0x600 |
| `hedeleg` | Hypervisor exception delegation | 0x602 |
| `hideleg` | Hypervisor interrupt delegation | 0x603 |
| `hie` | Hypervisor interrupt enable | 0x604 |
| `hcounteren` | Hypervisor counter enable | 0x606 |
| `hgatp` | Hypervisor guest address translation and protection | 0x680 |

### Virtual Supervisor Registers

| Register | Description | CSR Address |
|----------|-------------|-------------|
| `vsstatus` | Virtual supervisor status | 0x200 |
| `vsie` | Virtual supervisor interrupt enable | 0x204 |
| `vstvec` | Virtual supervisor trap vector | 0x205 |
| `vsscratch` | Virtual supervisor scratch | 0x240 |
| `vsepc` | Virtual supervisor exception PC | 0x241 |
| `vscause` | Virtual supervisor cause | 0x242 |
| `vstval` | Virtual supervisor trap value | 0x243 |
| `vsatp` | Virtual supervisor address translation and protection | 0x280 |

### Additional Registers

- **Interrupt Management**: `hip`, `hvip`, `hgeie`, `hgeip`
- **Time Management**: `htimedelta`, `htimedeltah` 
- **Trap Information**: `htval`, `htinst`
- **Virtual Supervisor Interrupts**: `vsip`

### Documentation

Generate and view API documentation:

```bash
cargo doc --no-deps --open
```

Online documentation: [docs.rs/riscv-h](https://docs.rs/riscv-h)

# Contributing

1. Fork the repository and create a branch
2. Run local checks: `./scripts/check.sh`
3. Run local tests: `./scripts/test.sh`
4. Submit a PR and pass CI checks

# License

This project is licensed under the Apache License, Version 2.0. See the \[LICENSE\](LICENSE) file for details.
