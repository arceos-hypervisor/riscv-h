//! Hypervisor Time Delta Register.
use riscv::{read_composite_csr, read_csr_as_usize, write_csr_as_usize};

read_composite_csr!(super::htimedeltah::read(), read());
read_csr_as_usize!(0x605);
write_csr_as_usize!(0x605);
