//! Hypervisor Trap Instruction Register.
use riscv::{read_csr_as_usize, write_csr_as_usize};

read_csr_as_usize!(0x64A);
write_csr_as_usize!(0x64A);
