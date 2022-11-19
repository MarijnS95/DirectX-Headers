#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    clippy::all
)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_COLOR_SPACE_TYPE(pub i32);
pub const DXGI_COLOR_SPACE_RGB_FULL_G22_NONE_P709: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(0i32);
pub const DXGI_COLOR_SPACE_RGB_FULL_G10_NONE_P709: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(1i32);
pub const DXGI_COLOR_SPACE_RGB_STUDIO_G22_NONE_P709: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(2i32);
pub const DXGI_COLOR_SPACE_RGB_STUDIO_G22_NONE_P2020: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(3i32);
pub const DXGI_COLOR_SPACE_RESERVED: DXGI_COLOR_SPACE_TYPE = DXGI_COLOR_SPACE_TYPE(4i32);
pub const DXGI_COLOR_SPACE_YCBCR_FULL_G22_NONE_P709_X601: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(5i32);
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P601: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(6i32);
pub const DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P601: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(7i32);
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P709: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(8i32);
pub const DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P709: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(9i32);
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P2020: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(10i32);
pub const DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P2020: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(11i32);
pub const DXGI_COLOR_SPACE_RGB_FULL_G2084_NONE_P2020: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(12i32);
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G2084_LEFT_P2020: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(13i32);
pub const DXGI_COLOR_SPACE_RGB_STUDIO_G2084_NONE_P2020: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(14i32);
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_TOPLEFT_P2020: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(15i32);
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G2084_TOPLEFT_P2020: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(16i32);
pub const DXGI_COLOR_SPACE_RGB_FULL_G22_NONE_P2020: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(17i32);
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_GHLG_TOPLEFT_P2020: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(18i32);
pub const DXGI_COLOR_SPACE_YCBCR_FULL_GHLG_TOPLEFT_P2020: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(19i32);
pub const DXGI_COLOR_SPACE_RGB_STUDIO_G24_NONE_P709: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(20i32);
pub const DXGI_COLOR_SPACE_RGB_STUDIO_G24_NONE_P2020: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(21i32);
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G24_LEFT_P709: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(22i32);
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G24_LEFT_P2020: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(23i32);
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G24_TOPLEFT_P2020: DXGI_COLOR_SPACE_TYPE =
    DXGI_COLOR_SPACE_TYPE(24i32);
pub const DXGI_COLOR_SPACE_CUSTOM: DXGI_COLOR_SPACE_TYPE = DXGI_COLOR_SPACE_TYPE(-1i32);
impl ::core::marker::Copy for DXGI_COLOR_SPACE_TYPE {}
impl ::core::clone::Clone for DXGI_COLOR_SPACE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_COLOR_SPACE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXGI_COLOR_SPACE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXGI_COLOR_SPACE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_COLOR_SPACE_TYPE")
            .field(&self.0)
            .finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DXGI_FORMAT(pub u32);
pub const DXGI_FORMAT_UNKNOWN: DXGI_FORMAT = DXGI_FORMAT(0u32);
pub const DXGI_FORMAT_R32G32B32A32_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(1u32);
pub const DXGI_FORMAT_R32G32B32A32_FLOAT: DXGI_FORMAT = DXGI_FORMAT(2u32);
pub const DXGI_FORMAT_R32G32B32A32_UINT: DXGI_FORMAT = DXGI_FORMAT(3u32);
pub const DXGI_FORMAT_R32G32B32A32_SINT: DXGI_FORMAT = DXGI_FORMAT(4u32);
pub const DXGI_FORMAT_R32G32B32_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(5u32);
pub const DXGI_FORMAT_R32G32B32_FLOAT: DXGI_FORMAT = DXGI_FORMAT(6u32);
pub const DXGI_FORMAT_R32G32B32_UINT: DXGI_FORMAT = DXGI_FORMAT(7u32);
pub const DXGI_FORMAT_R32G32B32_SINT: DXGI_FORMAT = DXGI_FORMAT(8u32);
pub const DXGI_FORMAT_R16G16B16A16_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(9u32);
pub const DXGI_FORMAT_R16G16B16A16_FLOAT: DXGI_FORMAT = DXGI_FORMAT(10u32);
pub const DXGI_FORMAT_R16G16B16A16_UNORM: DXGI_FORMAT = DXGI_FORMAT(11u32);
pub const DXGI_FORMAT_R16G16B16A16_UINT: DXGI_FORMAT = DXGI_FORMAT(12u32);
pub const DXGI_FORMAT_R16G16B16A16_SNORM: DXGI_FORMAT = DXGI_FORMAT(13u32);
pub const DXGI_FORMAT_R16G16B16A16_SINT: DXGI_FORMAT = DXGI_FORMAT(14u32);
pub const DXGI_FORMAT_R32G32_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(15u32);
pub const DXGI_FORMAT_R32G32_FLOAT: DXGI_FORMAT = DXGI_FORMAT(16u32);
pub const DXGI_FORMAT_R32G32_UINT: DXGI_FORMAT = DXGI_FORMAT(17u32);
pub const DXGI_FORMAT_R32G32_SINT: DXGI_FORMAT = DXGI_FORMAT(18u32);
pub const DXGI_FORMAT_R32G8X24_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(19u32);
pub const DXGI_FORMAT_D32_FLOAT_S8X24_UINT: DXGI_FORMAT = DXGI_FORMAT(20u32);
pub const DXGI_FORMAT_R32_FLOAT_X8X24_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(21u32);
pub const DXGI_FORMAT_X32_TYPELESS_G8X24_UINT: DXGI_FORMAT = DXGI_FORMAT(22u32);
pub const DXGI_FORMAT_R10G10B10A2_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(23u32);
pub const DXGI_FORMAT_R10G10B10A2_UNORM: DXGI_FORMAT = DXGI_FORMAT(24u32);
pub const DXGI_FORMAT_R10G10B10A2_UINT: DXGI_FORMAT = DXGI_FORMAT(25u32);
pub const DXGI_FORMAT_R11G11B10_FLOAT: DXGI_FORMAT = DXGI_FORMAT(26u32);
pub const DXGI_FORMAT_R8G8B8A8_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(27u32);
pub const DXGI_FORMAT_R8G8B8A8_UNORM: DXGI_FORMAT = DXGI_FORMAT(28u32);
pub const DXGI_FORMAT_R8G8B8A8_UNORM_SRGB: DXGI_FORMAT = DXGI_FORMAT(29u32);
pub const DXGI_FORMAT_R8G8B8A8_UINT: DXGI_FORMAT = DXGI_FORMAT(30u32);
pub const DXGI_FORMAT_R8G8B8A8_SNORM: DXGI_FORMAT = DXGI_FORMAT(31u32);
pub const DXGI_FORMAT_R8G8B8A8_SINT: DXGI_FORMAT = DXGI_FORMAT(32u32);
pub const DXGI_FORMAT_R16G16_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(33u32);
pub const DXGI_FORMAT_R16G16_FLOAT: DXGI_FORMAT = DXGI_FORMAT(34u32);
pub const DXGI_FORMAT_R16G16_UNORM: DXGI_FORMAT = DXGI_FORMAT(35u32);
pub const DXGI_FORMAT_R16G16_UINT: DXGI_FORMAT = DXGI_FORMAT(36u32);
pub const DXGI_FORMAT_R16G16_SNORM: DXGI_FORMAT = DXGI_FORMAT(37u32);
pub const DXGI_FORMAT_R16G16_SINT: DXGI_FORMAT = DXGI_FORMAT(38u32);
pub const DXGI_FORMAT_R32_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(39u32);
pub const DXGI_FORMAT_D32_FLOAT: DXGI_FORMAT = DXGI_FORMAT(40u32);
pub const DXGI_FORMAT_R32_FLOAT: DXGI_FORMAT = DXGI_FORMAT(41u32);
pub const DXGI_FORMAT_R32_UINT: DXGI_FORMAT = DXGI_FORMAT(42u32);
pub const DXGI_FORMAT_R32_SINT: DXGI_FORMAT = DXGI_FORMAT(43u32);
pub const DXGI_FORMAT_R24G8_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(44u32);
pub const DXGI_FORMAT_D24_UNORM_S8_UINT: DXGI_FORMAT = DXGI_FORMAT(45u32);
pub const DXGI_FORMAT_R24_UNORM_X8_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(46u32);
pub const DXGI_FORMAT_X24_TYPELESS_G8_UINT: DXGI_FORMAT = DXGI_FORMAT(47u32);
pub const DXGI_FORMAT_R8G8_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(48u32);
pub const DXGI_FORMAT_R8G8_UNORM: DXGI_FORMAT = DXGI_FORMAT(49u32);
pub const DXGI_FORMAT_R8G8_UINT: DXGI_FORMAT = DXGI_FORMAT(50u32);
pub const DXGI_FORMAT_R8G8_SNORM: DXGI_FORMAT = DXGI_FORMAT(51u32);
pub const DXGI_FORMAT_R8G8_SINT: DXGI_FORMAT = DXGI_FORMAT(52u32);
pub const DXGI_FORMAT_R16_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(53u32);
pub const DXGI_FORMAT_R16_FLOAT: DXGI_FORMAT = DXGI_FORMAT(54u32);
pub const DXGI_FORMAT_D16_UNORM: DXGI_FORMAT = DXGI_FORMAT(55u32);
pub const DXGI_FORMAT_R16_UNORM: DXGI_FORMAT = DXGI_FORMAT(56u32);
pub const DXGI_FORMAT_R16_UINT: DXGI_FORMAT = DXGI_FORMAT(57u32);
pub const DXGI_FORMAT_R16_SNORM: DXGI_FORMAT = DXGI_FORMAT(58u32);
pub const DXGI_FORMAT_R16_SINT: DXGI_FORMAT = DXGI_FORMAT(59u32);
pub const DXGI_FORMAT_R8_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(60u32);
pub const DXGI_FORMAT_R8_UNORM: DXGI_FORMAT = DXGI_FORMAT(61u32);
pub const DXGI_FORMAT_R8_UINT: DXGI_FORMAT = DXGI_FORMAT(62u32);
pub const DXGI_FORMAT_R8_SNORM: DXGI_FORMAT = DXGI_FORMAT(63u32);
pub const DXGI_FORMAT_R8_SINT: DXGI_FORMAT = DXGI_FORMAT(64u32);
pub const DXGI_FORMAT_A8_UNORM: DXGI_FORMAT = DXGI_FORMAT(65u32);
pub const DXGI_FORMAT_R1_UNORM: DXGI_FORMAT = DXGI_FORMAT(66u32);
pub const DXGI_FORMAT_R9G9B9E5_SHAREDEXP: DXGI_FORMAT = DXGI_FORMAT(67u32);
pub const DXGI_FORMAT_R8G8_B8G8_UNORM: DXGI_FORMAT = DXGI_FORMAT(68u32);
pub const DXGI_FORMAT_G8R8_G8B8_UNORM: DXGI_FORMAT = DXGI_FORMAT(69u32);
pub const DXGI_FORMAT_BC1_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(70u32);
pub const DXGI_FORMAT_BC1_UNORM: DXGI_FORMAT = DXGI_FORMAT(71u32);
pub const DXGI_FORMAT_BC1_UNORM_SRGB: DXGI_FORMAT = DXGI_FORMAT(72u32);
pub const DXGI_FORMAT_BC2_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(73u32);
pub const DXGI_FORMAT_BC2_UNORM: DXGI_FORMAT = DXGI_FORMAT(74u32);
pub const DXGI_FORMAT_BC2_UNORM_SRGB: DXGI_FORMAT = DXGI_FORMAT(75u32);
pub const DXGI_FORMAT_BC3_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(76u32);
pub const DXGI_FORMAT_BC3_UNORM: DXGI_FORMAT = DXGI_FORMAT(77u32);
pub const DXGI_FORMAT_BC3_UNORM_SRGB: DXGI_FORMAT = DXGI_FORMAT(78u32);
pub const DXGI_FORMAT_BC4_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(79u32);
pub const DXGI_FORMAT_BC4_UNORM: DXGI_FORMAT = DXGI_FORMAT(80u32);
pub const DXGI_FORMAT_BC4_SNORM: DXGI_FORMAT = DXGI_FORMAT(81u32);
pub const DXGI_FORMAT_BC5_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(82u32);
pub const DXGI_FORMAT_BC5_UNORM: DXGI_FORMAT = DXGI_FORMAT(83u32);
pub const DXGI_FORMAT_BC5_SNORM: DXGI_FORMAT = DXGI_FORMAT(84u32);
pub const DXGI_FORMAT_B5G6R5_UNORM: DXGI_FORMAT = DXGI_FORMAT(85u32);
pub const DXGI_FORMAT_B5G5R5A1_UNORM: DXGI_FORMAT = DXGI_FORMAT(86u32);
pub const DXGI_FORMAT_B8G8R8A8_UNORM: DXGI_FORMAT = DXGI_FORMAT(87u32);
pub const DXGI_FORMAT_B8G8R8X8_UNORM: DXGI_FORMAT = DXGI_FORMAT(88u32);
pub const DXGI_FORMAT_R10G10B10_XR_BIAS_A2_UNORM: DXGI_FORMAT = DXGI_FORMAT(89u32);
pub const DXGI_FORMAT_B8G8R8A8_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(90u32);
pub const DXGI_FORMAT_B8G8R8A8_UNORM_SRGB: DXGI_FORMAT = DXGI_FORMAT(91u32);
pub const DXGI_FORMAT_B8G8R8X8_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(92u32);
pub const DXGI_FORMAT_B8G8R8X8_UNORM_SRGB: DXGI_FORMAT = DXGI_FORMAT(93u32);
pub const DXGI_FORMAT_BC6H_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(94u32);
pub const DXGI_FORMAT_BC6H_UF16: DXGI_FORMAT = DXGI_FORMAT(95u32);
pub const DXGI_FORMAT_BC6H_SF16: DXGI_FORMAT = DXGI_FORMAT(96u32);
pub const DXGI_FORMAT_BC7_TYPELESS: DXGI_FORMAT = DXGI_FORMAT(97u32);
pub const DXGI_FORMAT_BC7_UNORM: DXGI_FORMAT = DXGI_FORMAT(98u32);
pub const DXGI_FORMAT_BC7_UNORM_SRGB: DXGI_FORMAT = DXGI_FORMAT(99u32);
pub const DXGI_FORMAT_AYUV: DXGI_FORMAT = DXGI_FORMAT(100u32);
pub const DXGI_FORMAT_Y410: DXGI_FORMAT = DXGI_FORMAT(101u32);
pub const DXGI_FORMAT_Y416: DXGI_FORMAT = DXGI_FORMAT(102u32);
pub const DXGI_FORMAT_NV12: DXGI_FORMAT = DXGI_FORMAT(103u32);
pub const DXGI_FORMAT_P010: DXGI_FORMAT = DXGI_FORMAT(104u32);
pub const DXGI_FORMAT_P016: DXGI_FORMAT = DXGI_FORMAT(105u32);
pub const DXGI_FORMAT_420_OPAQUE: DXGI_FORMAT = DXGI_FORMAT(106u32);
pub const DXGI_FORMAT_YUY2: DXGI_FORMAT = DXGI_FORMAT(107u32);
pub const DXGI_FORMAT_Y210: DXGI_FORMAT = DXGI_FORMAT(108u32);
pub const DXGI_FORMAT_Y216: DXGI_FORMAT = DXGI_FORMAT(109u32);
pub const DXGI_FORMAT_NV11: DXGI_FORMAT = DXGI_FORMAT(110u32);
pub const DXGI_FORMAT_AI44: DXGI_FORMAT = DXGI_FORMAT(111u32);
pub const DXGI_FORMAT_IA44: DXGI_FORMAT = DXGI_FORMAT(112u32);
pub const DXGI_FORMAT_P8: DXGI_FORMAT = DXGI_FORMAT(113u32);
pub const DXGI_FORMAT_A8P8: DXGI_FORMAT = DXGI_FORMAT(114u32);
pub const DXGI_FORMAT_B4G4R4A4_UNORM: DXGI_FORMAT = DXGI_FORMAT(115u32);
pub const DXGI_FORMAT_P208: DXGI_FORMAT = DXGI_FORMAT(130u32);
pub const DXGI_FORMAT_V208: DXGI_FORMAT = DXGI_FORMAT(131u32);
pub const DXGI_FORMAT_V408: DXGI_FORMAT = DXGI_FORMAT(132u32);
pub const DXGI_FORMAT_SAMPLER_FEEDBACK_MIN_MIP_OPAQUE: DXGI_FORMAT = DXGI_FORMAT(189u32);
pub const DXGI_FORMAT_SAMPLER_FEEDBACK_MIP_REGION_USED_OPAQUE: DXGI_FORMAT = DXGI_FORMAT(190u32);
pub const DXGI_FORMAT_FORCE_UINT: DXGI_FORMAT = DXGI_FORMAT(4294967295u32);
impl ::core::marker::Copy for DXGI_FORMAT {}
impl ::core::clone::Clone for DXGI_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXGI_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXGI_FORMAT {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXGI_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_FORMAT").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct DXGI_RATIONAL {
    pub Numerator: u32,
    pub Denominator: u32,
}
impl ::core::marker::Copy for DXGI_RATIONAL {}
impl ::core::clone::Clone for DXGI_RATIONAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_RATIONAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_RATIONAL")
            .field("Numerator", &self.Numerator)
            .field("Denominator", &self.Denominator)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for DXGI_RATIONAL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DXGI_RATIONAL {
    fn eq(&self, other: &Self) -> bool {
        self.Numerator == other.Numerator && self.Denominator == other.Denominator
    }
}
impl ::core::cmp::Eq for DXGI_RATIONAL {}
impl ::core::default::Default for DXGI_RATIONAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DXGI_SAMPLE_DESC {
    pub Count: u32,
    pub Quality: u32,
}
impl ::core::marker::Copy for DXGI_SAMPLE_DESC {}
impl ::core::clone::Clone for DXGI_SAMPLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DXGI_SAMPLE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_SAMPLE_DESC")
            .field("Count", &self.Count)
            .field("Quality", &self.Quality)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for DXGI_SAMPLE_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DXGI_SAMPLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Quality == other.Quality
    }
}
impl ::core::cmp::Eq for DXGI_SAMPLE_DESC {}
impl ::core::default::Default for DXGI_SAMPLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
