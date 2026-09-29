//! Headless browser policy and an optional, synthetic-only Windows embedding spike.
#![deny(unsafe_op_in_unsafe_fn)]

mod dev_profile;
mod page_policy;
mod retention;
mod surface;
pub use dev_profile::*;
pub use page_policy::*;
pub use retention::*;
pub use surface::*;
#[cfg(all(target_os = "windows", feature = "windows-child"))]
pub mod windows_child;
