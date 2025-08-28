//! Hypervisor Guest External Interrupt Enable Register.
use riscv::{read_csr_as_usize, write_csr_as_usize};

read_csr_as_usize!(0x607);
write_csr_as_usize!(0x607);
