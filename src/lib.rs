//! Detects the runtimes a Windows desktop program depends on, and installs missing
//! ones through the Microsoft Store install service. See `docs/design.md`.
#![cfg(windows)]

#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[rustfmt::skip]
mod bindings;
mod dependency;
mod packages;
pub mod store;
mod version;

pub use dependency::{DiagnoseError, FrameworkDependency};
pub use version::{ParseVersionError, Version};
