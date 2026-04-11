#[cfg(feature = "tpm1_2")]
pub mod tpm12;
#[cfg(all(feature = "tpm1_2", not(feature = "tpm2_0")))]
pub use tpm12::*;

#[cfg(feature = "tpm2_0")]
pub mod tpm20;
#[cfg(feature = "tpm2_0")]
pub use tpm20::*;

#[cfg(all(not(feature = "tpm1_2"), not(feature = "tpm2_0")))]
pub mod stubs;
#[cfg(all(not(feature = "tpm1_2"), not(feature = "tpm2_0")))]
pub use stubs::*;
