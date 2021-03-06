#![cfg_attr(not(feature = "stable"), no_std)]
#![cfg_attr(all(not(feature = "stable"), any(target_arch = "x86", target_arch = "x86_64")), feature(llvm_asm))]
#![cfg_attr(not(feature = "stable"), feature(const_fn))]

pub use self::io::*;
pub use self::mmio::*;
pub use self::pio::*;

mod io;
mod mmio;
mod pio;
