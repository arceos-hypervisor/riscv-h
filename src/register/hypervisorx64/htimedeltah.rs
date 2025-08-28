//! Hypervisor Time Delta Register.
use riscv::{read_csr_as_usize, write_csr_as_usize};

read_csr_as_usize!(0x615);
write_csr_as_usize!(0x615);
