//! Virtual Supervisor Trap Value Register.

use riscv::{read_csr_as_usize, write_csr_as_usize};

read_csr_as_usize!(0x243);
write_csr_as_usize!(0x243);
