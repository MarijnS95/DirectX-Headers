#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "DirectX_Direct3D")]
pub mod Direct3D;
#[cfg(feature = "DirectX_Direct3D12")]
pub mod Direct3D12;
#[cfg(feature = "DirectX_Dxgi")]
pub mod Dxgi;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
