#![stable(feature = "rust1", since = "1.0.0")]
#![doc(cfg(target_os = "xous"))]
>>>>>>>> 19b41569e69ccc3111029ba7c53788f2cc4df06a:library/std/src/os/xous/mod.rs
=======

#[allow(unused_extern_crates)]
#[stable(feature = "rust1", since = "1.0.0")]
pub extern crate hermit_abi;

pub mod ffi;
pub mod io;

/// A prelude for conveniently writing platform-specific code.
///
/// Includes all extension traits, and some important type definitions.
#[stable(feature = "rust1", since = "1.0.0")]
pub mod prelude {
    #[doc(no_inline)]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub use super::ffi::{OsStrExt, OsStringExt};
}
