//! Virtual Supervisor Scratch Register.

use riscv::{read_csr_as_usize, write_csr_as_usize};

read_csr_as_usize!(0x240);
write_csr_as_usize!(0x240);
