use libheif_sys as lh;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Chroma {
    C420,
    C422,
    C444,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RgbChroma {
    C444,
    // Interleaved
    Rgb,
    Rgba,
    HdrRgbBe,
    HdrRgbaBe,
    HdrRgbLe,
    HdrRgbaLe,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ColorSpace {
    Undefined,
    YCbCr(Chroma),
    Rgb(RgbChroma),
    Monochrome,
}

impl ColorSpace {
    pub(crate) fn from_libheif(
        color_space: lh::heif_colorspace,
        chroma: lh::heif_chroma,
    ) -> Option<Self> {
        match color_space {
            lh::heif_colorspace_heif_colorspace_undefined => Some(ColorSpace::Undefined),
            lh::heif_colorspace_heif_colorspace_monochrome => Some(ColorSpace::Monochrome),
            lh::heif_colorspace_heif_colorspace_YCbCr => match chroma {
                lh::heif_chroma_heif_chroma_420 => Some(ColorSpace::YCbCr(Chroma::C420)),
                lh::heif_chroma_heif_chroma_422 => Some(ColorSpace::YCbCr(Chroma::C422)),
                lh::heif_chroma_heif_chroma_444 => Some(ColorSpace::YCbCr(Chroma::C444)),
                _ => None,
            },
            lh::heif_colorspace_heif_colorspace_RGB => match chroma {
                lh::heif_chroma_heif_chroma_444 => Some(ColorSpace::Rgb(RgbChroma::C444)),
                lh::heif_chroma_heif_chroma_interleaved_RGB => {
                    Some(ColorSpace::Rgb(RgbChroma::Rgb))
                }
                lh::heif_chroma_heif_chroma_interleaved_RGBA => {
                    Some(ColorSpace::Rgb(RgbChroma::Rgba))
                }
                lh::heif_chroma_heif_chroma_interleaved_RRGGBB_BE => {
                    Some(ColorSpace::Rgb(RgbChroma::HdrRgbBe))
                }
                lh::heif_chroma_heif_chroma_interleaved_RRGGBB_LE => {
                    Some(ColorSpace::Rgb(RgbChroma::HdrRgbLe))
                }
                lh::heif_chroma_heif_chroma_interleaved_RRGGBBAA_BE => {
                    Some(ColorSpace::Rgb(RgbChroma::HdrRgbaBe))
                }
                lh::heif_chroma_heif_chroma_interleaved_RRGGBBAA_LE => {
                    Some(ColorSpace::Rgb(RgbChroma::HdrRgbaLe))
                }
                _ => None,
            },
            _ => None,
        }
    }

    pub(crate) fn heif_color_space(self) -> lh::heif_colorspace {
        match self {
            ColorSpace::YCbCr(_) => lh::heif_colorspace_heif_colorspace_YCbCr,
            ColorSpace::Rgb(_) => lh::heif_colorspace_heif_colorspace_RGB,
            ColorSpace::Monochrome => lh::heif_colorspace_heif_colorspace_monochrome,
            ColorSpace::Undefined => lh::heif_colorspace_heif_colorspace_undefined,
        }
    }

    pub(crate) fn heif_chroma(self) -> lh::heif_chroma {
        match self {
            ColorSpace::YCbCr(chroma) => match chroma {
                Chroma::C420 => lh::heif_chroma_heif_chroma_420,
                Chroma::C422 => lh::heif_chroma_heif_chroma_422,
                Chroma::C444 => lh::heif_chroma_heif_chroma_444,
            },
            ColorSpace::Rgb(chroma) => match chroma {
                RgbChroma::C444 => lh::heif_chroma_heif_chroma_444,
                RgbChroma::Rgb => lh::heif_chroma_heif_chroma_interleaved_RGB,
                RgbChroma::Rgba => lh::heif_chroma_heif_chroma_interleaved_RGBA,
                RgbChroma::HdrRgbBe => lh::heif_chroma_heif_chroma_interleaved_RRGGBB_BE,
                RgbChroma::HdrRgbLe => lh::heif_chroma_heif_chroma_interleaved_RRGGBB_LE,
                RgbChroma::HdrRgbaBe => lh::heif_chroma_heif_chroma_interleaved_RRGGBBAA_BE,
                RgbChroma::HdrRgbaLe => lh::heif_chroma_heif_chroma_interleaved_RRGGBBAA_LE,
            },
            ColorSpace::Undefined => lh::heif_chroma_heif_chroma_undefined,
            ColorSpace::Monochrome => lh::heif_chroma_heif_chroma_monochrome,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum Channel {
    Y = lh::heif_channel_heif_channel_Y as _,
    Cb = lh::heif_channel_heif_channel_Cb as _,
    Cr = lh::heif_channel_heif_channel_Cr as _,
    R = lh::heif_channel_heif_channel_R as _,
    G = lh::heif_channel_heif_channel_G as _,
    B = lh::heif_channel_heif_channel_B as _,
    Alpha = lh::heif_channel_heif_channel_Alpha as _,
    Interleaved = lh::heif_channel_heif_channel_interleaved as _,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, enumn::N)]
#[repr(C)]
pub enum CompressionFormat {
    Undefined = lh::heif_compression_format_heif_compression_undefined as _,
    Hevc = lh::heif_compression_format_heif_compression_HEVC as _,
    Avc = lh::heif_compression_format_heif_compression_AVC as _,
    Jpeg = lh::heif_compression_format_heif_compression_JPEG as _,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, enumn::N)]
#[repr(C)]
pub enum EncoderParameterType {
    Int = lh::heif_encoder_parameter_type_heif_encoder_parameter_type_integer as _,
    Bool = lh::heif_encoder_parameter_type_heif_encoder_parameter_type_boolean as _,
    String = lh::heif_encoder_parameter_type_heif_encoder_parameter_type_string as _,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EncoderParameterValue {
    Int(i32),
    Bool(bool),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EncoderQuality {
    LossLess,
    Lossy(u8),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ReaderGrowStatus {
    SizeReached = lh::heif_reader_grow_status_heif_reader_grow_status_size_reached as _,
    Timeout = lh::heif_reader_grow_status_heif_reader_grow_status_timeout as _,
    SizeBeyondEof = lh::heif_reader_grow_status_heif_reader_grow_status_size_beyond_eof as _,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, enumn::N)]
#[repr(C)]
pub enum FileTypeResult {
    No = lh::heif_filetype_result_heif_filetype_no as _,
    /// It is HEIF and can be read by libheif
    Supported = lh::heif_filetype_result_heif_filetype_yes_supported as _,
    /// It is HEIF, but cannot be read by libheif
    Unsupported = lh::heif_filetype_result_heif_filetype_yes_unsupported as _,
    /// Not sure whether it is an HEIF, try detection with more input data
    MayBe = lh::heif_filetype_result_heif_filetype_maybe as _,
}
