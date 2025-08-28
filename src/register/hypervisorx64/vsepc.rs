//! Virtual Supervisor Exception Program Counter.

use riscv::{read_csr_as_usize, write_csr_as_usize};
read_csr_as_usize!(0x241);
write_csr_as_usize!(0x241);
