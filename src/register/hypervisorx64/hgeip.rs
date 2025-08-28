//! Hypervisor Guest External Interrupt Pending Register.
use riscv::{read_csr_as_usize, write_csr_as_usize};

read_csr_as_usize!(0xE12);
write_csr_as_usize!(0xE12);
