#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "DirectX_Dxgi_Common")]
pub mod Common;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
