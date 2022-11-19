#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    clippy::all
)]
pub mod Common;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
