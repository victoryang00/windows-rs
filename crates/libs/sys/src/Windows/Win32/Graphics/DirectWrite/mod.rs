#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    pub fn DWriteCreateFactory(factorytype: DWRITE_FACTORY_TYPE, iid: *const ::windows_sys::core::GUID, factory: *mut *mut *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_ALPHA_MAX: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_AUTOMATIC_FONT_AXES = u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_AUTOMATIC_FONT_AXES_NONE: DWRITE_AUTOMATIC_FONT_AXES = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_AUTOMATIC_FONT_AXES_OPTICAL_SIZE: DWRITE_AUTOMATIC_FONT_AXES = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_BASELINE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_BASELINE_DEFAULT: DWRITE_BASELINE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_BASELINE_ROMAN: DWRITE_BASELINE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_BASELINE_CENTRAL: DWRITE_BASELINE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_BASELINE_MATH: DWRITE_BASELINE = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_BASELINE_HANGING: DWRITE_BASELINE = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_BASELINE_IDEOGRAPHIC_BOTTOM: DWRITE_BASELINE = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_BASELINE_IDEOGRAPHIC_TOP: DWRITE_BASELINE = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_BASELINE_MINIMUM: DWRITE_BASELINE = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_BASELINE_MAXIMUM: DWRITE_BASELINE = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_BREAK_CONDITION = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_BREAK_CONDITION_NEUTRAL: DWRITE_BREAK_CONDITION = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_BREAK_CONDITION_CAN_BREAK: DWRITE_BREAK_CONDITION = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_BREAK_CONDITION_MAY_NOT_BREAK: DWRITE_BREAK_CONDITION = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_BREAK_CONDITION_MUST_BREAK: DWRITE_BREAK_CONDITION = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_CARET_METRICS {
    pub slopeRise: i16,
    pub slopeRun: i16,
    pub offset: i16,
}
impl ::core::marker::Copy for DWRITE_CARET_METRICS {}
impl ::core::clone::Clone for DWRITE_CARET_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_CLUSTER_METRICS {
    pub width: f32,
    pub length: u16,
    pub _bitfield: u16,
}
impl ::core::marker::Copy for DWRITE_CLUSTER_METRICS {}
impl ::core::clone::Clone for DWRITE_CLUSTER_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_COLOR_F {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl ::core::marker::Copy for DWRITE_COLOR_F {}
impl ::core::clone::Clone for DWRITE_COLOR_F {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_COLOR_GLYPH_RUN {
    pub glyphRun: DWRITE_GLYPH_RUN,
    pub glyphRunDescription: *mut DWRITE_GLYPH_RUN_DESCRIPTION,
    pub baselineOriginX: f32,
    pub baselineOriginY: f32,
    pub runColor: DWRITE_COLOR_F,
    pub paletteIndex: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DWRITE_COLOR_GLYPH_RUN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DWRITE_COLOR_GLYPH_RUN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_COLOR_GLYPH_RUN1 {
    pub Base: DWRITE_COLOR_GLYPH_RUN,
    pub glyphImageFormat: DWRITE_GLYPH_IMAGE_FORMATS,
    pub measuringMode: DWRITE_MEASURING_MODE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DWRITE_COLOR_GLYPH_RUN1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DWRITE_COLOR_GLYPH_RUN1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_CONTAINER_TYPE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_CONTAINER_TYPE_UNKNOWN: DWRITE_CONTAINER_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_CONTAINER_TYPE_WOFF: DWRITE_CONTAINER_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_CONTAINER_TYPE_WOFF2: DWRITE_CONTAINER_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_ERR_BASE: u32 = 20480u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_E_DOWNLOADCANCELLED: ::windows_sys::core::HRESULT = -2003283954i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_E_DOWNLOADFAILED: ::windows_sys::core::HRESULT = -2003283953i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_E_REMOTEFONT: ::windows_sys::core::HRESULT = -2003283955i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_E_TOOMANYDOWNLOADS: ::windows_sys::core::HRESULT = -2003283952i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_FACTORY_TYPE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FACTORY_TYPE_SHARED: DWRITE_FACTORY_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FACTORY_TYPE_ISOLATED: DWRITE_FACTORY_TYPE = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_FILE_FRAGMENT {
    pub fileOffset: u64,
    pub fragmentSize: u64,
}
impl ::core::marker::Copy for DWRITE_FILE_FRAGMENT {}
impl ::core::clone::Clone for DWRITE_FILE_FRAGMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_FLOW_DIRECTION = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FLOW_DIRECTION_TOP_TO_BOTTOM: DWRITE_FLOW_DIRECTION = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FLOW_DIRECTION_BOTTOM_TO_TOP: DWRITE_FLOW_DIRECTION = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FLOW_DIRECTION_LEFT_TO_RIGHT: DWRITE_FLOW_DIRECTION = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FLOW_DIRECTION_RIGHT_TO_LEFT: DWRITE_FLOW_DIRECTION = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_FONT_AXIS_ATTRIBUTES = u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_AXIS_ATTRIBUTES_NONE: DWRITE_FONT_AXIS_ATTRIBUTES = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_AXIS_ATTRIBUTES_VARIABLE: DWRITE_FONT_AXIS_ATTRIBUTES = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_AXIS_ATTRIBUTES_HIDDEN: DWRITE_FONT_AXIS_ATTRIBUTES = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_FONT_AXIS_RANGE {
    pub axisTag: DWRITE_FONT_AXIS_TAG,
    pub minValue: f32,
    pub maxValue: f32,
}
impl ::core::marker::Copy for DWRITE_FONT_AXIS_RANGE {}
impl ::core::clone::Clone for DWRITE_FONT_AXIS_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_FONT_AXIS_TAG = u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_AXIS_TAG_WEIGHT: DWRITE_FONT_AXIS_TAG = 1952999287u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_AXIS_TAG_WIDTH: DWRITE_FONT_AXIS_TAG = 1752458359u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_AXIS_TAG_SLANT: DWRITE_FONT_AXIS_TAG = 1953393779u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_AXIS_TAG_OPTICAL_SIZE: DWRITE_FONT_AXIS_TAG = 2054385775u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_AXIS_TAG_ITALIC: DWRITE_FONT_AXIS_TAG = 1818326121u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_FONT_AXIS_VALUE {
    pub axisTag: DWRITE_FONT_AXIS_TAG,
    pub value: f32,
}
impl ::core::marker::Copy for DWRITE_FONT_AXIS_VALUE {}
impl ::core::clone::Clone for DWRITE_FONT_AXIS_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_FONT_FACE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FACE_TYPE_CFF: DWRITE_FONT_FACE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FACE_TYPE_TRUETYPE: DWRITE_FONT_FACE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FACE_TYPE_OPENTYPE_COLLECTION: DWRITE_FONT_FACE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FACE_TYPE_TYPE1: DWRITE_FONT_FACE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FACE_TYPE_VECTOR: DWRITE_FONT_FACE_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FACE_TYPE_BITMAP: DWRITE_FONT_FACE_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FACE_TYPE_UNKNOWN: DWRITE_FONT_FACE_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FACE_TYPE_RAW_CFF: DWRITE_FONT_FACE_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FACE_TYPE_TRUETYPE_COLLECTION: DWRITE_FONT_FACE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_FONT_FAMILY_MODEL = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FAMILY_MODEL_TYPOGRAPHIC: DWRITE_FONT_FAMILY_MODEL = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FAMILY_MODEL_WEIGHT_STRETCH_STYLE: DWRITE_FONT_FAMILY_MODEL = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_FONT_FEATURE {
    pub nameTag: DWRITE_FONT_FEATURE_TAG,
    pub parameter: u32,
}
impl ::core::marker::Copy for DWRITE_FONT_FEATURE {}
impl ::core::clone::Clone for DWRITE_FONT_FEATURE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_FONT_FEATURE_TAG = u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_ALTERNATIVE_FRACTIONS: DWRITE_FONT_FEATURE_TAG = 1668441697u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_PETITE_CAPITALS_FROM_CAPITALS: DWRITE_FONT_FEATURE_TAG = 1668297315u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_SMALL_CAPITALS_FROM_CAPITALS: DWRITE_FONT_FEATURE_TAG = 1668493923u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_ALTERNATES: DWRITE_FONT_FEATURE_TAG = 1953259875u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_CASE_SENSITIVE_FORMS: DWRITE_FONT_FEATURE_TAG = 1702060387u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_GLYPH_COMPOSITION_DECOMPOSITION: DWRITE_FONT_FEATURE_TAG = 1886217059u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_LIGATURES: DWRITE_FONT_FEATURE_TAG = 1734962275u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_CAPITAL_SPACING: DWRITE_FONT_FEATURE_TAG = 1886613603u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_SWASH: DWRITE_FONT_FEATURE_TAG = 1752658787u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_CURSIVE_POSITIONING: DWRITE_FONT_FEATURE_TAG = 1936880995u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_DEFAULT: DWRITE_FONT_FEATURE_TAG = 1953261156u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_DISCRETIONARY_LIGATURES: DWRITE_FONT_FEATURE_TAG = 1734962276u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_EXPERT_FORMS: DWRITE_FONT_FEATURE_TAG = 1953527909u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_FRACTIONS: DWRITE_FONT_FEATURE_TAG = 1667330662u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_FULL_WIDTH: DWRITE_FONT_FEATURE_TAG = 1684633446u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_HALF_FORMS: DWRITE_FONT_FEATURE_TAG = 1718378856u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_HALANT_FORMS: DWRITE_FONT_FEATURE_TAG = 1852596584u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_ALTERNATE_HALF_WIDTH: DWRITE_FONT_FEATURE_TAG = 1953259880u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_HISTORICAL_FORMS: DWRITE_FONT_FEATURE_TAG = 1953720680u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_HORIZONTAL_KANA_ALTERNATES: DWRITE_FONT_FEATURE_TAG = 1634626408u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_HISTORICAL_LIGATURES: DWRITE_FONT_FEATURE_TAG = 1734962280u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_HALF_WIDTH: DWRITE_FONT_FEATURE_TAG = 1684633448u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_HOJO_KANJI_FORMS: DWRITE_FONT_FEATURE_TAG = 1869246312u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_JIS04_FORMS: DWRITE_FONT_FEATURE_TAG = 875589738u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_JIS78_FORMS: DWRITE_FONT_FEATURE_TAG = 943157354u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_JIS83_FORMS: DWRITE_FONT_FEATURE_TAG = 859336810u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_JIS90_FORMS: DWRITE_FONT_FEATURE_TAG = 809070698u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_KERNING: DWRITE_FONT_FEATURE_TAG = 1852990827u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_STANDARD_LIGATURES: DWRITE_FONT_FEATURE_TAG = 1634167148u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_LINING_FIGURES: DWRITE_FONT_FEATURE_TAG = 1836412524u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_LOCALIZED_FORMS: DWRITE_FONT_FEATURE_TAG = 1818455916u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_MARK_POSITIONING: DWRITE_FONT_FEATURE_TAG = 1802658157u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_MATHEMATICAL_GREEK: DWRITE_FONT_FEATURE_TAG = 1802659693u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_MARK_TO_MARK_POSITIONING: DWRITE_FONT_FEATURE_TAG = 1802333037u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_ALTERNATE_ANNOTATION_FORMS: DWRITE_FONT_FEATURE_TAG = 1953259886u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_NLC_KANJI_FORMS: DWRITE_FONT_FEATURE_TAG = 1801677934u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_OLD_STYLE_FIGURES: DWRITE_FONT_FEATURE_TAG = 1836412527u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_ORDINALS: DWRITE_FONT_FEATURE_TAG = 1852076655u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_PROPORTIONAL_ALTERNATE_WIDTH: DWRITE_FONT_FEATURE_TAG = 1953259888u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_PETITE_CAPITALS: DWRITE_FONT_FEATURE_TAG = 1885430640u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_PROPORTIONAL_FIGURES: DWRITE_FONT_FEATURE_TAG = 1836412528u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_PROPORTIONAL_WIDTHS: DWRITE_FONT_FEATURE_TAG = 1684633456u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_QUARTER_WIDTHS: DWRITE_FONT_FEATURE_TAG = 1684633457u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_REQUIRED_LIGATURES: DWRITE_FONT_FEATURE_TAG = 1734962290u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_RUBY_NOTATION_FORMS: DWRITE_FONT_FEATURE_TAG = 2036495730u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_ALTERNATES: DWRITE_FONT_FEATURE_TAG = 1953259891u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_SCIENTIFIC_INFERIORS: DWRITE_FONT_FEATURE_TAG = 1718511987u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_SMALL_CAPITALS: DWRITE_FONT_FEATURE_TAG = 1885564275u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_SIMPLIFIED_FORMS: DWRITE_FONT_FEATURE_TAG = 1819307379u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_1: DWRITE_FONT_FEATURE_TAG = 825258867u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_2: DWRITE_FONT_FEATURE_TAG = 842036083u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_3: DWRITE_FONT_FEATURE_TAG = 858813299u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_4: DWRITE_FONT_FEATURE_TAG = 875590515u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_5: DWRITE_FONT_FEATURE_TAG = 892367731u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_6: DWRITE_FONT_FEATURE_TAG = 909144947u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_7: DWRITE_FONT_FEATURE_TAG = 925922163u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_8: DWRITE_FONT_FEATURE_TAG = 942699379u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_9: DWRITE_FONT_FEATURE_TAG = 959476595u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_10: DWRITE_FONT_FEATURE_TAG = 808547187u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_11: DWRITE_FONT_FEATURE_TAG = 825324403u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_12: DWRITE_FONT_FEATURE_TAG = 842101619u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_13: DWRITE_FONT_FEATURE_TAG = 858878835u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_14: DWRITE_FONT_FEATURE_TAG = 875656051u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_15: DWRITE_FONT_FEATURE_TAG = 892433267u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_16: DWRITE_FONT_FEATURE_TAG = 909210483u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_17: DWRITE_FONT_FEATURE_TAG = 925987699u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_18: DWRITE_FONT_FEATURE_TAG = 942764915u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_19: DWRITE_FONT_FEATURE_TAG = 959542131u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_20: DWRITE_FONT_FEATURE_TAG = 808612723u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_SUBSCRIPT: DWRITE_FONT_FEATURE_TAG = 1935832435u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_SUPERSCRIPT: DWRITE_FONT_FEATURE_TAG = 1936749939u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_SWASH: DWRITE_FONT_FEATURE_TAG = 1752397683u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_TITLING: DWRITE_FONT_FEATURE_TAG = 1819568500u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_TRADITIONAL_NAME_FORMS: DWRITE_FONT_FEATURE_TAG = 1835101812u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_TABULAR_FIGURES: DWRITE_FONT_FEATURE_TAG = 1836412532u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_TRADITIONAL_FORMS: DWRITE_FONT_FEATURE_TAG = 1684107892u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_THIRD_WIDTHS: DWRITE_FONT_FEATURE_TAG = 1684633460u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_UNICASE: DWRITE_FONT_FEATURE_TAG = 1667853941u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_VERTICAL_WRITING: DWRITE_FONT_FEATURE_TAG = 1953654134u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_VERTICAL_ALTERNATES_AND_ROTATION: DWRITE_FONT_FEATURE_TAG = 846492278u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FEATURE_TAG_SLASHED_ZERO: DWRITE_FONT_FEATURE_TAG = 1869768058u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_FONT_FILE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FILE_TYPE_UNKNOWN: DWRITE_FONT_FILE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FILE_TYPE_CFF: DWRITE_FONT_FILE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FILE_TYPE_TRUETYPE: DWRITE_FONT_FILE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FILE_TYPE_OPENTYPE_COLLECTION: DWRITE_FONT_FILE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FILE_TYPE_TYPE1_PFM: DWRITE_FONT_FILE_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FILE_TYPE_TYPE1_PFB: DWRITE_FONT_FILE_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FILE_TYPE_VECTOR: DWRITE_FONT_FILE_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FILE_TYPE_BITMAP: DWRITE_FONT_FILE_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_FILE_TYPE_TRUETYPE_COLLECTION: DWRITE_FONT_FILE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_FONT_LINE_GAP_USAGE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_LINE_GAP_USAGE_DEFAULT: DWRITE_FONT_LINE_GAP_USAGE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_LINE_GAP_USAGE_DISABLED: DWRITE_FONT_LINE_GAP_USAGE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_LINE_GAP_USAGE_ENABLED: DWRITE_FONT_LINE_GAP_USAGE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_FONT_METRICS {
    pub designUnitsPerEm: u16,
    pub ascent: u16,
    pub descent: u16,
    pub lineGap: i16,
    pub capHeight: u16,
    pub xHeight: u16,
    pub underlinePosition: i16,
    pub underlineThickness: u16,
    pub strikethroughPosition: i16,
    pub strikethroughThickness: u16,
}
impl ::core::marker::Copy for DWRITE_FONT_METRICS {}
impl ::core::clone::Clone for DWRITE_FONT_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_FONT_METRICS1 {
    pub __AnonymousBase_DWrite_1_L627_C38: DWRITE_FONT_METRICS,
    pub glyphBoxLeft: i16,
    pub glyphBoxTop: i16,
    pub glyphBoxRight: i16,
    pub glyphBoxBottom: i16,
    pub subscriptPositionX: i16,
    pub subscriptPositionY: i16,
    pub subscriptSizeX: i16,
    pub subscriptSizeY: i16,
    pub superscriptPositionX: i16,
    pub superscriptPositionY: i16,
    pub superscriptSizeX: i16,
    pub superscriptSizeY: i16,
    pub hasTypographicMetrics: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DWRITE_FONT_METRICS1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DWRITE_FONT_METRICS1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_FONT_PROPERTY {
    pub propertyId: DWRITE_FONT_PROPERTY_ID,
    pub propertyValue: ::windows_sys::core::PCWSTR,
    pub localeName: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for DWRITE_FONT_PROPERTY {}
impl ::core::clone::Clone for DWRITE_FONT_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_FONT_PROPERTY_ID = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_PROPERTY_ID_NONE: DWRITE_FONT_PROPERTY_ID = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_PROPERTY_ID_WEIGHT_STRETCH_STYLE_FAMILY_NAME: DWRITE_FONT_PROPERTY_ID = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_PROPERTY_ID_TYPOGRAPHIC_FAMILY_NAME: DWRITE_FONT_PROPERTY_ID = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_PROPERTY_ID_WEIGHT_STRETCH_STYLE_FACE_NAME: DWRITE_FONT_PROPERTY_ID = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_PROPERTY_ID_FULL_NAME: DWRITE_FONT_PROPERTY_ID = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_PROPERTY_ID_WIN32_FAMILY_NAME: DWRITE_FONT_PROPERTY_ID = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_PROPERTY_ID_POSTSCRIPT_NAME: DWRITE_FONT_PROPERTY_ID = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_PROPERTY_ID_DESIGN_SCRIPT_LANGUAGE_TAG: DWRITE_FONT_PROPERTY_ID = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_PROPERTY_ID_SUPPORTED_SCRIPT_LANGUAGE_TAG: DWRITE_FONT_PROPERTY_ID = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_PROPERTY_ID_SEMANTIC_TAG: DWRITE_FONT_PROPERTY_ID = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_PROPERTY_ID_WEIGHT: DWRITE_FONT_PROPERTY_ID = 10i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_PROPERTY_ID_STRETCH: DWRITE_FONT_PROPERTY_ID = 11i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_PROPERTY_ID_STYLE: DWRITE_FONT_PROPERTY_ID = 12i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_PROPERTY_ID_TYPOGRAPHIC_FACE_NAME: DWRITE_FONT_PROPERTY_ID = 13i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_PROPERTY_ID_TOTAL: DWRITE_FONT_PROPERTY_ID = 13i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_PROPERTY_ID_TOTAL_RS3: DWRITE_FONT_PROPERTY_ID = 14i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_PROPERTY_ID_PREFERRED_FAMILY_NAME: DWRITE_FONT_PROPERTY_ID = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_PROPERTY_ID_FAMILY_NAME: DWRITE_FONT_PROPERTY_ID = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_PROPERTY_ID_FACE_NAME: DWRITE_FONT_PROPERTY_ID = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_FONT_SIMULATIONS = u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_SIMULATIONS_NONE: DWRITE_FONT_SIMULATIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_SIMULATIONS_BOLD: DWRITE_FONT_SIMULATIONS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_SIMULATIONS_OBLIQUE: DWRITE_FONT_SIMULATIONS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_FONT_SOURCE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_SOURCE_TYPE_UNKNOWN: DWRITE_FONT_SOURCE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_SOURCE_TYPE_PER_MACHINE: DWRITE_FONT_SOURCE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_SOURCE_TYPE_PER_USER: DWRITE_FONT_SOURCE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_SOURCE_TYPE_APPX_PACKAGE: DWRITE_FONT_SOURCE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_SOURCE_TYPE_REMOTE_FONT_PROVIDER: DWRITE_FONT_SOURCE_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_FONT_STRETCH = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_STRETCH_UNDEFINED: DWRITE_FONT_STRETCH = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_STRETCH_ULTRA_CONDENSED: DWRITE_FONT_STRETCH = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_STRETCH_EXTRA_CONDENSED: DWRITE_FONT_STRETCH = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_STRETCH_CONDENSED: DWRITE_FONT_STRETCH = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_STRETCH_SEMI_CONDENSED: DWRITE_FONT_STRETCH = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_STRETCH_NORMAL: DWRITE_FONT_STRETCH = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_STRETCH_MEDIUM: DWRITE_FONT_STRETCH = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_STRETCH_SEMI_EXPANDED: DWRITE_FONT_STRETCH = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_STRETCH_EXPANDED: DWRITE_FONT_STRETCH = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_STRETCH_EXTRA_EXPANDED: DWRITE_FONT_STRETCH = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_STRETCH_ULTRA_EXPANDED: DWRITE_FONT_STRETCH = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_FONT_STYLE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_STYLE_NORMAL: DWRITE_FONT_STYLE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_STYLE_OBLIQUE: DWRITE_FONT_STYLE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_STYLE_ITALIC: DWRITE_FONT_STYLE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_FONT_WEIGHT = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_WEIGHT_THIN: DWRITE_FONT_WEIGHT = 100i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_WEIGHT_EXTRA_LIGHT: DWRITE_FONT_WEIGHT = 200i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_WEIGHT_ULTRA_LIGHT: DWRITE_FONT_WEIGHT = 200i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_WEIGHT_LIGHT: DWRITE_FONT_WEIGHT = 300i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_WEIGHT_SEMI_LIGHT: DWRITE_FONT_WEIGHT = 350i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_WEIGHT_NORMAL: DWRITE_FONT_WEIGHT = 400i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_WEIGHT_REGULAR: DWRITE_FONT_WEIGHT = 400i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_WEIGHT_MEDIUM: DWRITE_FONT_WEIGHT = 500i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_WEIGHT_DEMI_BOLD: DWRITE_FONT_WEIGHT = 600i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_WEIGHT_SEMI_BOLD: DWRITE_FONT_WEIGHT = 600i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_WEIGHT_BOLD: DWRITE_FONT_WEIGHT = 700i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_WEIGHT_EXTRA_BOLD: DWRITE_FONT_WEIGHT = 800i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_WEIGHT_ULTRA_BOLD: DWRITE_FONT_WEIGHT = 800i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_WEIGHT_BLACK: DWRITE_FONT_WEIGHT = 900i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_WEIGHT_HEAVY: DWRITE_FONT_WEIGHT = 900i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_WEIGHT_EXTRA_BLACK: DWRITE_FONT_WEIGHT = 950i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_FONT_WEIGHT_ULTRA_BLACK: DWRITE_FONT_WEIGHT = 950i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub struct DWRITE_GLYPH_IMAGE_DATA {
    pub imageData: *const ::core::ffi::c_void,
    pub imageDataSize: u32,
    pub uniqueDataId: u32,
    pub pixelsPerEm: u32,
    pub pixelSize: super::Direct2D::Common::D2D_SIZE_U,
    pub horizontalLeftOrigin: super::super::Foundation::POINT,
    pub horizontalRightOrigin: super::super::Foundation::POINT,
    pub verticalTopOrigin: super::super::Foundation::POINT,
    pub verticalBottomOrigin: super::super::Foundation::POINT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::marker::Copy for DWRITE_GLYPH_IMAGE_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::clone::Clone for DWRITE_GLYPH_IMAGE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_GLYPH_IMAGE_FORMATS = u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_GLYPH_IMAGE_FORMATS_NONE: DWRITE_GLYPH_IMAGE_FORMATS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_GLYPH_IMAGE_FORMATS_TRUETYPE: DWRITE_GLYPH_IMAGE_FORMATS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_GLYPH_IMAGE_FORMATS_CFF: DWRITE_GLYPH_IMAGE_FORMATS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_GLYPH_IMAGE_FORMATS_COLR: DWRITE_GLYPH_IMAGE_FORMATS = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_GLYPH_IMAGE_FORMATS_SVG: DWRITE_GLYPH_IMAGE_FORMATS = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_GLYPH_IMAGE_FORMATS_PNG: DWRITE_GLYPH_IMAGE_FORMATS = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_GLYPH_IMAGE_FORMATS_JPEG: DWRITE_GLYPH_IMAGE_FORMATS = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_GLYPH_IMAGE_FORMATS_TIFF: DWRITE_GLYPH_IMAGE_FORMATS = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_GLYPH_IMAGE_FORMATS_PREMULTIPLIED_B8G8R8A8: DWRITE_GLYPH_IMAGE_FORMATS = 128u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_GLYPH_METRICS {
    pub leftSideBearing: i32,
    pub advanceWidth: u32,
    pub rightSideBearing: i32,
    pub topSideBearing: i32,
    pub advanceHeight: u32,
    pub bottomSideBearing: i32,
    pub verticalOriginY: i32,
}
impl ::core::marker::Copy for DWRITE_GLYPH_METRICS {}
impl ::core::clone::Clone for DWRITE_GLYPH_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_GLYPH_OFFSET {
    pub advanceOffset: f32,
    pub ascenderOffset: f32,
}
impl ::core::marker::Copy for DWRITE_GLYPH_OFFSET {}
impl ::core::clone::Clone for DWRITE_GLYPH_OFFSET {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_GLYPH_ORIENTATION_ANGLE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_GLYPH_ORIENTATION_ANGLE_0_DEGREES: DWRITE_GLYPH_ORIENTATION_ANGLE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_GLYPH_ORIENTATION_ANGLE_90_DEGREES: DWRITE_GLYPH_ORIENTATION_ANGLE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_GLYPH_ORIENTATION_ANGLE_180_DEGREES: DWRITE_GLYPH_ORIENTATION_ANGLE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_GLYPH_ORIENTATION_ANGLE_270_DEGREES: DWRITE_GLYPH_ORIENTATION_ANGLE = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_GLYPH_RUN {
    pub fontFace: *mut *mut *mut *mut IDWriteFontFace,
    pub fontEmSize: f32,
    pub glyphCount: u32,
    pub glyphIndices: *const u16,
    pub glyphAdvances: *const f32,
    pub glyphOffsets: *const DWRITE_GLYPH_OFFSET,
    pub isSideways: super::super::Foundation::BOOL,
    pub bidiLevel: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DWRITE_GLYPH_RUN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DWRITE_GLYPH_RUN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_GLYPH_RUN_DESCRIPTION {
    pub localeName: ::windows_sys::core::PCWSTR,
    pub string: ::windows_sys::core::PCWSTR,
    pub stringLength: u32,
    pub clusterMap: *const u16,
    pub textPosition: u32,
}
impl ::core::marker::Copy for DWRITE_GLYPH_RUN_DESCRIPTION {}
impl ::core::clone::Clone for DWRITE_GLYPH_RUN_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_GRID_FIT_MODE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_GRID_FIT_MODE_DEFAULT: DWRITE_GRID_FIT_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_GRID_FIT_MODE_DISABLED: DWRITE_GRID_FIT_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_GRID_FIT_MODE_ENABLED: DWRITE_GRID_FIT_MODE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_HIT_TEST_METRICS {
    pub textPosition: u32,
    pub length: u32,
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
    pub bidiLevel: u32,
    pub isText: super::super::Foundation::BOOL,
    pub isTrimmed: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DWRITE_HIT_TEST_METRICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DWRITE_HIT_TEST_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_INFORMATIONAL_STRING_ID = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_INFORMATIONAL_STRING_NONE: DWRITE_INFORMATIONAL_STRING_ID = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_INFORMATIONAL_STRING_COPYRIGHT_NOTICE: DWRITE_INFORMATIONAL_STRING_ID = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_INFORMATIONAL_STRING_VERSION_STRINGS: DWRITE_INFORMATIONAL_STRING_ID = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_INFORMATIONAL_STRING_TRADEMARK: DWRITE_INFORMATIONAL_STRING_ID = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_INFORMATIONAL_STRING_MANUFACTURER: DWRITE_INFORMATIONAL_STRING_ID = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_INFORMATIONAL_STRING_DESIGNER: DWRITE_INFORMATIONAL_STRING_ID = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_INFORMATIONAL_STRING_DESIGNER_URL: DWRITE_INFORMATIONAL_STRING_ID = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_INFORMATIONAL_STRING_DESCRIPTION: DWRITE_INFORMATIONAL_STRING_ID = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_INFORMATIONAL_STRING_FONT_VENDOR_URL: DWRITE_INFORMATIONAL_STRING_ID = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_INFORMATIONAL_STRING_LICENSE_DESCRIPTION: DWRITE_INFORMATIONAL_STRING_ID = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_INFORMATIONAL_STRING_LICENSE_INFO_URL: DWRITE_INFORMATIONAL_STRING_ID = 10i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_INFORMATIONAL_STRING_WIN32_FAMILY_NAMES: DWRITE_INFORMATIONAL_STRING_ID = 11i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_INFORMATIONAL_STRING_WIN32_SUBFAMILY_NAMES: DWRITE_INFORMATIONAL_STRING_ID = 12i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_INFORMATIONAL_STRING_TYPOGRAPHIC_FAMILY_NAMES: DWRITE_INFORMATIONAL_STRING_ID = 13i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_INFORMATIONAL_STRING_TYPOGRAPHIC_SUBFAMILY_NAMES: DWRITE_INFORMATIONAL_STRING_ID = 14i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_INFORMATIONAL_STRING_SAMPLE_TEXT: DWRITE_INFORMATIONAL_STRING_ID = 15i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_INFORMATIONAL_STRING_FULL_NAME: DWRITE_INFORMATIONAL_STRING_ID = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_NAME: DWRITE_INFORMATIONAL_STRING_ID = 17i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_CID_NAME: DWRITE_INFORMATIONAL_STRING_ID = 18i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_INFORMATIONAL_STRING_WEIGHT_STRETCH_STYLE_FAMILY_NAME: DWRITE_INFORMATIONAL_STRING_ID = 19i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_INFORMATIONAL_STRING_DESIGN_SCRIPT_LANGUAGE_TAG: DWRITE_INFORMATIONAL_STRING_ID = 20i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_INFORMATIONAL_STRING_SUPPORTED_SCRIPT_LANGUAGE_TAG: DWRITE_INFORMATIONAL_STRING_ID = 21i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_INFORMATIONAL_STRING_PREFERRED_FAMILY_NAMES: DWRITE_INFORMATIONAL_STRING_ID = 13i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_INFORMATIONAL_STRING_PREFERRED_SUBFAMILY_NAMES: DWRITE_INFORMATIONAL_STRING_ID = 14i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_INFORMATIONAL_STRING_WWS_FAMILY_NAME: DWRITE_INFORMATIONAL_STRING_ID = 19i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_INLINE_OBJECT_METRICS {
    pub width: f32,
    pub height: f32,
    pub baseline: f32,
    pub supportsSideways: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DWRITE_INLINE_OBJECT_METRICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DWRITE_INLINE_OBJECT_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_JUSTIFICATION_OPPORTUNITY {
    pub expansionMinimum: f32,
    pub expansionMaximum: f32,
    pub compressionMaximum: f32,
    pub _bitfield: u32,
}
impl ::core::marker::Copy for DWRITE_JUSTIFICATION_OPPORTUNITY {}
impl ::core::clone::Clone for DWRITE_JUSTIFICATION_OPPORTUNITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_LINE_BREAKPOINT {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for DWRITE_LINE_BREAKPOINT {}
impl ::core::clone::Clone for DWRITE_LINE_BREAKPOINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_LINE_METRICS {
    pub length: u32,
    pub trailingWhitespaceLength: u32,
    pub newlineLength: u32,
    pub height: f32,
    pub baseline: f32,
    pub isTrimmed: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DWRITE_LINE_METRICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DWRITE_LINE_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_LINE_METRICS1 {
    pub Base: DWRITE_LINE_METRICS,
    pub leadingBefore: f32,
    pub leadingAfter: f32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DWRITE_LINE_METRICS1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DWRITE_LINE_METRICS1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_LINE_SPACING {
    pub method: DWRITE_LINE_SPACING_METHOD,
    pub height: f32,
    pub baseline: f32,
    pub leadingBefore: f32,
    pub fontLineGapUsage: DWRITE_FONT_LINE_GAP_USAGE,
}
impl ::core::marker::Copy for DWRITE_LINE_SPACING {}
impl ::core::clone::Clone for DWRITE_LINE_SPACING {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_LINE_SPACING_METHOD = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_LINE_SPACING_METHOD_DEFAULT: DWRITE_LINE_SPACING_METHOD = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_LINE_SPACING_METHOD_UNIFORM: DWRITE_LINE_SPACING_METHOD = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_LINE_SPACING_METHOD_PROPORTIONAL: DWRITE_LINE_SPACING_METHOD = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_LOCALITY = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_LOCALITY_REMOTE: DWRITE_LOCALITY = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_LOCALITY_PARTIAL: DWRITE_LOCALITY = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_LOCALITY_LOCAL: DWRITE_LOCALITY = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_MATRIX {
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
    pub dx: f32,
    pub dy: f32,
}
impl ::core::marker::Copy for DWRITE_MATRIX {}
impl ::core::clone::Clone for DWRITE_MATRIX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_MEASURING_MODE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_MEASURING_MODE_NATURAL: DWRITE_MEASURING_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_MEASURING_MODE_GDI_CLASSIC: DWRITE_MEASURING_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_MEASURING_MODE_GDI_NATURAL: DWRITE_MEASURING_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_NUMBER_SUBSTITUTION_METHOD = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_NUMBER_SUBSTITUTION_METHOD_FROM_CULTURE: DWRITE_NUMBER_SUBSTITUTION_METHOD = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_NUMBER_SUBSTITUTION_METHOD_CONTEXTUAL: DWRITE_NUMBER_SUBSTITUTION_METHOD = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_NUMBER_SUBSTITUTION_METHOD_NONE: DWRITE_NUMBER_SUBSTITUTION_METHOD = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_NUMBER_SUBSTITUTION_METHOD_NATIONAL: DWRITE_NUMBER_SUBSTITUTION_METHOD = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_NUMBER_SUBSTITUTION_METHOD_TRADITIONAL: DWRITE_NUMBER_SUBSTITUTION_METHOD = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_OPTICAL_ALIGNMENT = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_OPTICAL_ALIGNMENT_NONE: DWRITE_OPTICAL_ALIGNMENT = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_OPTICAL_ALIGNMENT_NO_SIDE_BEARINGS: DWRITE_OPTICAL_ALIGNMENT = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_OUTLINE_THRESHOLD = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_OUTLINE_THRESHOLD_ANTIALIASED: DWRITE_OUTLINE_THRESHOLD = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_OUTLINE_THRESHOLD_ALIASED: DWRITE_OUTLINE_THRESHOLD = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_OVERHANG_METRICS {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
impl ::core::marker::Copy for DWRITE_OVERHANG_METRICS {}
impl ::core::clone::Clone for DWRITE_OVERHANG_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub union DWRITE_PANOSE {
    pub values: [u8; 10],
    pub familyKind: u8,
    pub text: DWRITE_PANOSE_3,
    pub script: DWRITE_PANOSE_1,
    pub decorative: DWRITE_PANOSE_0,
    pub symbol: DWRITE_PANOSE_2,
}
impl ::core::marker::Copy for DWRITE_PANOSE {}
impl ::core::clone::Clone for DWRITE_PANOSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_PANOSE_0 {
    pub familyKind: u8,
    pub decorativeClass: u8,
    pub weight: u8,
    pub aspect: u8,
    pub contrast: u8,
    pub serifVariant: u8,
    pub fill: u8,
    pub lining: u8,
    pub decorativeTopology: u8,
    pub characterRange: u8,
}
impl ::core::marker::Copy for DWRITE_PANOSE_0 {}
impl ::core::clone::Clone for DWRITE_PANOSE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_PANOSE_1 {
    pub familyKind: u8,
    pub toolKind: u8,
    pub weight: u8,
    pub spacing: u8,
    pub aspectRatio: u8,
    pub contrast: u8,
    pub scriptTopology: u8,
    pub scriptForm: u8,
    pub finials: u8,
    pub xAscent: u8,
}
impl ::core::marker::Copy for DWRITE_PANOSE_1 {}
impl ::core::clone::Clone for DWRITE_PANOSE_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_PANOSE_2 {
    pub familyKind: u8,
    pub symbolKind: u8,
    pub weight: u8,
    pub spacing: u8,
    pub aspectRatioAndContrast: u8,
    pub aspectRatio94: u8,
    pub aspectRatio119: u8,
    pub aspectRatio157: u8,
    pub aspectRatio163: u8,
    pub aspectRatio211: u8,
}
impl ::core::marker::Copy for DWRITE_PANOSE_2 {}
impl ::core::clone::Clone for DWRITE_PANOSE_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_PANOSE_3 {
    pub familyKind: u8,
    pub serifStyle: u8,
    pub weight: u8,
    pub proportion: u8,
    pub contrast: u8,
    pub strokeVariation: u8,
    pub armStyle: u8,
    pub letterform: u8,
    pub midline: u8,
    pub xHeight: u8,
}
impl ::core::marker::Copy for DWRITE_PANOSE_3 {}
impl ::core::clone::Clone for DWRITE_PANOSE_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_PANOSE_ARM_STYLE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ARM_STYLE_ANY: DWRITE_PANOSE_ARM_STYLE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ARM_STYLE_NO_FIT: DWRITE_PANOSE_ARM_STYLE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_HORIZONTAL: DWRITE_PANOSE_ARM_STYLE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_WEDGE: DWRITE_PANOSE_ARM_STYLE = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_VERTICAL: DWRITE_PANOSE_ARM_STYLE = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_SINGLE_SERIF: DWRITE_PANOSE_ARM_STYLE = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_DOUBLE_SERIF: DWRITE_PANOSE_ARM_STYLE = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_HORIZONTAL: DWRITE_PANOSE_ARM_STYLE = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_WEDGE: DWRITE_PANOSE_ARM_STYLE = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_VERTICAL: DWRITE_PANOSE_ARM_STYLE = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_SINGLE_SERIF: DWRITE_PANOSE_ARM_STYLE = 10i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_DOUBLE_SERIF: DWRITE_PANOSE_ARM_STYLE = 11i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_HORZ: DWRITE_PANOSE_ARM_STYLE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_VERT: DWRITE_PANOSE_ARM_STYLE = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ARM_STYLE_BENT_ARMS_HORZ: DWRITE_PANOSE_ARM_STYLE = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ARM_STYLE_BENT_ARMS_WEDGE: DWRITE_PANOSE_ARM_STYLE = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ARM_STYLE_BENT_ARMS_VERT: DWRITE_PANOSE_ARM_STYLE = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ARM_STYLE_BENT_ARMS_SINGLE_SERIF: DWRITE_PANOSE_ARM_STYLE = 10i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ARM_STYLE_BENT_ARMS_DOUBLE_SERIF: DWRITE_PANOSE_ARM_STYLE = 11i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_PANOSE_ASPECT = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ASPECT_ANY: DWRITE_PANOSE_ASPECT = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ASPECT_NO_FIT: DWRITE_PANOSE_ASPECT = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ASPECT_SUPER_CONDENSED: DWRITE_PANOSE_ASPECT = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ASPECT_VERY_CONDENSED: DWRITE_PANOSE_ASPECT = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ASPECT_CONDENSED: DWRITE_PANOSE_ASPECT = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ASPECT_NORMAL: DWRITE_PANOSE_ASPECT = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ASPECT_EXTENDED: DWRITE_PANOSE_ASPECT = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ASPECT_VERY_EXTENDED: DWRITE_PANOSE_ASPECT = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ASPECT_SUPER_EXTENDED: DWRITE_PANOSE_ASPECT = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ASPECT_MONOSPACED: DWRITE_PANOSE_ASPECT = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_PANOSE_ASPECT_RATIO = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ASPECT_RATIO_ANY: DWRITE_PANOSE_ASPECT_RATIO = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ASPECT_RATIO_NO_FIT: DWRITE_PANOSE_ASPECT_RATIO = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ASPECT_RATIO_VERY_CONDENSED: DWRITE_PANOSE_ASPECT_RATIO = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ASPECT_RATIO_CONDENSED: DWRITE_PANOSE_ASPECT_RATIO = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ASPECT_RATIO_NORMAL: DWRITE_PANOSE_ASPECT_RATIO = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ASPECT_RATIO_EXPANDED: DWRITE_PANOSE_ASPECT_RATIO = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_ASPECT_RATIO_VERY_EXPANDED: DWRITE_PANOSE_ASPECT_RATIO = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_PANOSE_CHARACTER_RANGES = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_CHARACTER_RANGES_ANY: DWRITE_PANOSE_CHARACTER_RANGES = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_CHARACTER_RANGES_NO_FIT: DWRITE_PANOSE_CHARACTER_RANGES = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_CHARACTER_RANGES_EXTENDED_COLLECTION: DWRITE_PANOSE_CHARACTER_RANGES = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_CHARACTER_RANGES_LITERALS: DWRITE_PANOSE_CHARACTER_RANGES = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_CHARACTER_RANGES_NO_LOWER_CASE: DWRITE_PANOSE_CHARACTER_RANGES = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_CHARACTER_RANGES_SMALL_CAPS: DWRITE_PANOSE_CHARACTER_RANGES = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_PANOSE_CONTRAST = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_CONTRAST_ANY: DWRITE_PANOSE_CONTRAST = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_CONTRAST_NO_FIT: DWRITE_PANOSE_CONTRAST = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_CONTRAST_NONE: DWRITE_PANOSE_CONTRAST = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_CONTRAST_VERY_LOW: DWRITE_PANOSE_CONTRAST = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_CONTRAST_LOW: DWRITE_PANOSE_CONTRAST = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_CONTRAST_MEDIUM_LOW: DWRITE_PANOSE_CONTRAST = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_CONTRAST_MEDIUM: DWRITE_PANOSE_CONTRAST = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_CONTRAST_MEDIUM_HIGH: DWRITE_PANOSE_CONTRAST = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_CONTRAST_HIGH: DWRITE_PANOSE_CONTRAST = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_CONTRAST_VERY_HIGH: DWRITE_PANOSE_CONTRAST = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_CONTRAST_HORIZONTAL_LOW: DWRITE_PANOSE_CONTRAST = 10i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_CONTRAST_HORIZONTAL_MEDIUM: DWRITE_PANOSE_CONTRAST = 11i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_CONTRAST_HORIZONTAL_HIGH: DWRITE_PANOSE_CONTRAST = 12i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_CONTRAST_BROKEN: DWRITE_PANOSE_CONTRAST = 13i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_PANOSE_DECORATIVE_CLASS = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_DECORATIVE_CLASS_ANY: DWRITE_PANOSE_DECORATIVE_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_DECORATIVE_CLASS_NO_FIT: DWRITE_PANOSE_DECORATIVE_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_DECORATIVE_CLASS_DERIVATIVE: DWRITE_PANOSE_DECORATIVE_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_DECORATIVE_CLASS_NONSTANDARD_TOPOLOGY: DWRITE_PANOSE_DECORATIVE_CLASS = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_DECORATIVE_CLASS_NONSTANDARD_ELEMENTS: DWRITE_PANOSE_DECORATIVE_CLASS = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_DECORATIVE_CLASS_NONSTANDARD_ASPECT: DWRITE_PANOSE_DECORATIVE_CLASS = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_DECORATIVE_CLASS_INITIALS: DWRITE_PANOSE_DECORATIVE_CLASS = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_DECORATIVE_CLASS_CARTOON: DWRITE_PANOSE_DECORATIVE_CLASS = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_DECORATIVE_CLASS_PICTURE_STEMS: DWRITE_PANOSE_DECORATIVE_CLASS = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_DECORATIVE_CLASS_ORNAMENTED: DWRITE_PANOSE_DECORATIVE_CLASS = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_DECORATIVE_CLASS_TEXT_AND_BACKGROUND: DWRITE_PANOSE_DECORATIVE_CLASS = 10i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_DECORATIVE_CLASS_COLLAGE: DWRITE_PANOSE_DECORATIVE_CLASS = 11i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_DECORATIVE_CLASS_MONTAGE: DWRITE_PANOSE_DECORATIVE_CLASS = 12i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_PANOSE_DECORATIVE_TOPOLOGY = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_ANY: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_NO_FIT: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_STANDARD: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_SQUARE: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_MULTIPLE_SEGMENT: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_ART_DECO: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_UNEVEN_WEIGHTING: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_DIVERSE_ARMS: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_DIVERSE_FORMS: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_LOMBARDIC_FORMS: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_UPPER_CASE_IN_LOWER_CASE: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 10i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_IMPLIED_TOPOLOGY: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 11i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_HORSESHOE_E_AND_A: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 12i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_CURSIVE: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 13i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_BLACKLETTER: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 14i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_SWASH_VARIANCE: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 15i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_PANOSE_FAMILY = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_FAMILY_ANY: DWRITE_PANOSE_FAMILY = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_FAMILY_NO_FIT: DWRITE_PANOSE_FAMILY = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_FAMILY_TEXT_DISPLAY: DWRITE_PANOSE_FAMILY = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_FAMILY_SCRIPT: DWRITE_PANOSE_FAMILY = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_FAMILY_DECORATIVE: DWRITE_PANOSE_FAMILY = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_FAMILY_SYMBOL: DWRITE_PANOSE_FAMILY = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_FAMILY_PICTORIAL: DWRITE_PANOSE_FAMILY = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_PANOSE_FILL = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_FILL_ANY: DWRITE_PANOSE_FILL = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_FILL_NO_FIT: DWRITE_PANOSE_FILL = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_FILL_STANDARD_SOLID_FILL: DWRITE_PANOSE_FILL = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_FILL_NO_FILL: DWRITE_PANOSE_FILL = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_FILL_PATTERNED_FILL: DWRITE_PANOSE_FILL = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_FILL_COMPLEX_FILL: DWRITE_PANOSE_FILL = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_FILL_SHAPED_FILL: DWRITE_PANOSE_FILL = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_FILL_DRAWN_DISTRESSED: DWRITE_PANOSE_FILL = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_PANOSE_FINIALS = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_FINIALS_ANY: DWRITE_PANOSE_FINIALS = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_FINIALS_NO_FIT: DWRITE_PANOSE_FINIALS = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_FINIALS_NONE_NO_LOOPS: DWRITE_PANOSE_FINIALS = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_FINIALS_NONE_CLOSED_LOOPS: DWRITE_PANOSE_FINIALS = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_FINIALS_NONE_OPEN_LOOPS: DWRITE_PANOSE_FINIALS = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_FINIALS_SHARP_NO_LOOPS: DWRITE_PANOSE_FINIALS = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_FINIALS_SHARP_CLOSED_LOOPS: DWRITE_PANOSE_FINIALS = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_FINIALS_SHARP_OPEN_LOOPS: DWRITE_PANOSE_FINIALS = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_FINIALS_TAPERED_NO_LOOPS: DWRITE_PANOSE_FINIALS = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_FINIALS_TAPERED_CLOSED_LOOPS: DWRITE_PANOSE_FINIALS = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_FINIALS_TAPERED_OPEN_LOOPS: DWRITE_PANOSE_FINIALS = 10i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_FINIALS_ROUND_NO_LOOPS: DWRITE_PANOSE_FINIALS = 11i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_FINIALS_ROUND_CLOSED_LOOPS: DWRITE_PANOSE_FINIALS = 12i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_FINIALS_ROUND_OPEN_LOOPS: DWRITE_PANOSE_FINIALS = 13i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_PANOSE_LETTERFORM = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_LETTERFORM_ANY: DWRITE_PANOSE_LETTERFORM = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_LETTERFORM_NO_FIT: DWRITE_PANOSE_LETTERFORM = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_CONTACT: DWRITE_PANOSE_LETTERFORM = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_WEIGHTED: DWRITE_PANOSE_LETTERFORM = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_BOXED: DWRITE_PANOSE_LETTERFORM = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_FLATTENED: DWRITE_PANOSE_LETTERFORM = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_ROUNDED: DWRITE_PANOSE_LETTERFORM = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_OFF_CENTER: DWRITE_PANOSE_LETTERFORM = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_SQUARE: DWRITE_PANOSE_LETTERFORM = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_CONTACT: DWRITE_PANOSE_LETTERFORM = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_WEIGHTED: DWRITE_PANOSE_LETTERFORM = 10i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_BOXED: DWRITE_PANOSE_LETTERFORM = 11i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_FLATTENED: DWRITE_PANOSE_LETTERFORM = 12i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_ROUNDED: DWRITE_PANOSE_LETTERFORM = 13i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_OFF_CENTER: DWRITE_PANOSE_LETTERFORM = 14i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_SQUARE: DWRITE_PANOSE_LETTERFORM = 15i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_PANOSE_LINING = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_LINING_ANY: DWRITE_PANOSE_LINING = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_LINING_NO_FIT: DWRITE_PANOSE_LINING = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_LINING_NONE: DWRITE_PANOSE_LINING = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_LINING_INLINE: DWRITE_PANOSE_LINING = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_LINING_OUTLINE: DWRITE_PANOSE_LINING = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_LINING_ENGRAVED: DWRITE_PANOSE_LINING = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_LINING_SHADOW: DWRITE_PANOSE_LINING = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_LINING_RELIEF: DWRITE_PANOSE_LINING = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_LINING_BACKDROP: DWRITE_PANOSE_LINING = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_PANOSE_MIDLINE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_MIDLINE_ANY: DWRITE_PANOSE_MIDLINE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_MIDLINE_NO_FIT: DWRITE_PANOSE_MIDLINE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_MIDLINE_STANDARD_TRIMMED: DWRITE_PANOSE_MIDLINE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_MIDLINE_STANDARD_POINTED: DWRITE_PANOSE_MIDLINE = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_MIDLINE_STANDARD_SERIFED: DWRITE_PANOSE_MIDLINE = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_MIDLINE_HIGH_TRIMMED: DWRITE_PANOSE_MIDLINE = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_MIDLINE_HIGH_POINTED: DWRITE_PANOSE_MIDLINE = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_MIDLINE_HIGH_SERIFED: DWRITE_PANOSE_MIDLINE = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_MIDLINE_CONSTANT_TRIMMED: DWRITE_PANOSE_MIDLINE = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_MIDLINE_CONSTANT_POINTED: DWRITE_PANOSE_MIDLINE = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_MIDLINE_CONSTANT_SERIFED: DWRITE_PANOSE_MIDLINE = 10i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_MIDLINE_LOW_TRIMMED: DWRITE_PANOSE_MIDLINE = 11i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_MIDLINE_LOW_POINTED: DWRITE_PANOSE_MIDLINE = 12i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_MIDLINE_LOW_SERIFED: DWRITE_PANOSE_MIDLINE = 13i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_PANOSE_PROPORTION = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_PROPORTION_ANY: DWRITE_PANOSE_PROPORTION = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_PROPORTION_NO_FIT: DWRITE_PANOSE_PROPORTION = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_PROPORTION_OLD_STYLE: DWRITE_PANOSE_PROPORTION = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_PROPORTION_MODERN: DWRITE_PANOSE_PROPORTION = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_PROPORTION_EVEN_WIDTH: DWRITE_PANOSE_PROPORTION = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_PROPORTION_EXPANDED: DWRITE_PANOSE_PROPORTION = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_PROPORTION_CONDENSED: DWRITE_PANOSE_PROPORTION = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_PROPORTION_VERY_EXPANDED: DWRITE_PANOSE_PROPORTION = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_PROPORTION_VERY_CONDENSED: DWRITE_PANOSE_PROPORTION = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_PROPORTION_MONOSPACED: DWRITE_PANOSE_PROPORTION = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_PANOSE_SCRIPT_FORM = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SCRIPT_FORM_ANY: DWRITE_PANOSE_SCRIPT_FORM = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SCRIPT_FORM_NO_FIT: DWRITE_PANOSE_SCRIPT_FORM = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SCRIPT_FORM_UPRIGHT_NO_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SCRIPT_FORM_UPRIGHT_SOME_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SCRIPT_FORM_UPRIGHT_MORE_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SCRIPT_FORM_UPRIGHT_EXTREME_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SCRIPT_FORM_OBLIQUE_NO_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SCRIPT_FORM_OBLIQUE_SOME_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SCRIPT_FORM_OBLIQUE_MORE_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SCRIPT_FORM_OBLIQUE_EXTREME_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SCRIPT_FORM_EXAGGERATED_NO_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = 10i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SCRIPT_FORM_EXAGGERATED_SOME_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = 11i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SCRIPT_FORM_EXAGGERATED_MORE_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = 12i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SCRIPT_FORM_EXAGGERATED_EXTREME_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = 13i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_PANOSE_SCRIPT_TOPOLOGY = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_ANY: DWRITE_PANOSE_SCRIPT_TOPOLOGY = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_NO_FIT: DWRITE_PANOSE_SCRIPT_TOPOLOGY = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_ROMAN_DISCONNECTED: DWRITE_PANOSE_SCRIPT_TOPOLOGY = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_ROMAN_TRAILING: DWRITE_PANOSE_SCRIPT_TOPOLOGY = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_ROMAN_CONNECTED: DWRITE_PANOSE_SCRIPT_TOPOLOGY = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_CURSIVE_DISCONNECTED: DWRITE_PANOSE_SCRIPT_TOPOLOGY = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_CURSIVE_TRAILING: DWRITE_PANOSE_SCRIPT_TOPOLOGY = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_CURSIVE_CONNECTED: DWRITE_PANOSE_SCRIPT_TOPOLOGY = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_BLACKLETTER_DISCONNECTED: DWRITE_PANOSE_SCRIPT_TOPOLOGY = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_BLACKLETTER_TRAILING: DWRITE_PANOSE_SCRIPT_TOPOLOGY = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_BLACKLETTER_CONNECTED: DWRITE_PANOSE_SCRIPT_TOPOLOGY = 10i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_PANOSE_SERIF_STYLE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SERIF_STYLE_ANY: DWRITE_PANOSE_SERIF_STYLE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SERIF_STYLE_NO_FIT: DWRITE_PANOSE_SERIF_STYLE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SERIF_STYLE_COVE: DWRITE_PANOSE_SERIF_STYLE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SERIF_STYLE_OBTUSE_COVE: DWRITE_PANOSE_SERIF_STYLE = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SERIF_STYLE_SQUARE_COVE: DWRITE_PANOSE_SERIF_STYLE = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SERIF_STYLE_OBTUSE_SQUARE_COVE: DWRITE_PANOSE_SERIF_STYLE = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SERIF_STYLE_SQUARE: DWRITE_PANOSE_SERIF_STYLE = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SERIF_STYLE_THIN: DWRITE_PANOSE_SERIF_STYLE = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SERIF_STYLE_OVAL: DWRITE_PANOSE_SERIF_STYLE = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SERIF_STYLE_EXAGGERATED: DWRITE_PANOSE_SERIF_STYLE = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SERIF_STYLE_TRIANGLE: DWRITE_PANOSE_SERIF_STYLE = 10i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SERIF_STYLE_NORMAL_SANS: DWRITE_PANOSE_SERIF_STYLE = 11i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SERIF_STYLE_OBTUSE_SANS: DWRITE_PANOSE_SERIF_STYLE = 12i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SERIF_STYLE_PERPENDICULAR_SANS: DWRITE_PANOSE_SERIF_STYLE = 13i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SERIF_STYLE_FLARED: DWRITE_PANOSE_SERIF_STYLE = 14i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SERIF_STYLE_ROUNDED: DWRITE_PANOSE_SERIF_STYLE = 15i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SERIF_STYLE_SCRIPT: DWRITE_PANOSE_SERIF_STYLE = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SERIF_STYLE_PERP_SANS: DWRITE_PANOSE_SERIF_STYLE = 13i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SERIF_STYLE_BONE: DWRITE_PANOSE_SERIF_STYLE = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_PANOSE_SPACING = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SPACING_ANY: DWRITE_PANOSE_SPACING = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SPACING_NO_FIT: DWRITE_PANOSE_SPACING = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SPACING_PROPORTIONAL_SPACED: DWRITE_PANOSE_SPACING = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SPACING_MONOSPACED: DWRITE_PANOSE_SPACING = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_PANOSE_STROKE_VARIATION = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_STROKE_VARIATION_ANY: DWRITE_PANOSE_STROKE_VARIATION = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_STROKE_VARIATION_NO_FIT: DWRITE_PANOSE_STROKE_VARIATION = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_STROKE_VARIATION_NO_VARIATION: DWRITE_PANOSE_STROKE_VARIATION = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_STROKE_VARIATION_GRADUAL_DIAGONAL: DWRITE_PANOSE_STROKE_VARIATION = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_STROKE_VARIATION_GRADUAL_TRANSITIONAL: DWRITE_PANOSE_STROKE_VARIATION = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_STROKE_VARIATION_GRADUAL_VERTICAL: DWRITE_PANOSE_STROKE_VARIATION = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_STROKE_VARIATION_GRADUAL_HORIZONTAL: DWRITE_PANOSE_STROKE_VARIATION = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_STROKE_VARIATION_RAPID_VERTICAL: DWRITE_PANOSE_STROKE_VARIATION = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_STROKE_VARIATION_RAPID_HORIZONTAL: DWRITE_PANOSE_STROKE_VARIATION = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_STROKE_VARIATION_INSTANT_VERTICAL: DWRITE_PANOSE_STROKE_VARIATION = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_STROKE_VARIATION_INSTANT_HORIZONTAL: DWRITE_PANOSE_STROKE_VARIATION = 10i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_ANY: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_NO_FIT: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_NO_WIDTH: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_EXCEPTIONALLY_WIDE: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_SUPER_WIDE: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_VERY_WIDE: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_WIDE: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_NORMAL: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_NARROW: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_VERY_NARROW: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_PANOSE_SYMBOL_KIND = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SYMBOL_KIND_ANY: DWRITE_PANOSE_SYMBOL_KIND = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SYMBOL_KIND_NO_FIT: DWRITE_PANOSE_SYMBOL_KIND = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SYMBOL_KIND_MONTAGES: DWRITE_PANOSE_SYMBOL_KIND = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SYMBOL_KIND_PICTURES: DWRITE_PANOSE_SYMBOL_KIND = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SYMBOL_KIND_SHAPES: DWRITE_PANOSE_SYMBOL_KIND = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SYMBOL_KIND_SCIENTIFIC: DWRITE_PANOSE_SYMBOL_KIND = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SYMBOL_KIND_MUSIC: DWRITE_PANOSE_SYMBOL_KIND = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SYMBOL_KIND_EXPERT: DWRITE_PANOSE_SYMBOL_KIND = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SYMBOL_KIND_PATTERNS: DWRITE_PANOSE_SYMBOL_KIND = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SYMBOL_KIND_BOARDERS: DWRITE_PANOSE_SYMBOL_KIND = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SYMBOL_KIND_ICONS: DWRITE_PANOSE_SYMBOL_KIND = 10i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SYMBOL_KIND_LOGOS: DWRITE_PANOSE_SYMBOL_KIND = 11i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_SYMBOL_KIND_INDUSTRY_SPECIFIC: DWRITE_PANOSE_SYMBOL_KIND = 12i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_PANOSE_TOOL_KIND = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_TOOL_KIND_ANY: DWRITE_PANOSE_TOOL_KIND = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_TOOL_KIND_NO_FIT: DWRITE_PANOSE_TOOL_KIND = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_TOOL_KIND_FLAT_NIB: DWRITE_PANOSE_TOOL_KIND = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_TOOL_KIND_PRESSURE_POINT: DWRITE_PANOSE_TOOL_KIND = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_TOOL_KIND_ENGRAVED: DWRITE_PANOSE_TOOL_KIND = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_TOOL_KIND_BALL: DWRITE_PANOSE_TOOL_KIND = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_TOOL_KIND_BRUSH: DWRITE_PANOSE_TOOL_KIND = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_TOOL_KIND_ROUGH: DWRITE_PANOSE_TOOL_KIND = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_TOOL_KIND_FELT_PEN_BRUSH_TIP: DWRITE_PANOSE_TOOL_KIND = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_TOOL_KIND_WILD_BRUSH: DWRITE_PANOSE_TOOL_KIND = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_PANOSE_WEIGHT = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_WEIGHT_ANY: DWRITE_PANOSE_WEIGHT = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_WEIGHT_NO_FIT: DWRITE_PANOSE_WEIGHT = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_WEIGHT_VERY_LIGHT: DWRITE_PANOSE_WEIGHT = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_WEIGHT_LIGHT: DWRITE_PANOSE_WEIGHT = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_WEIGHT_THIN: DWRITE_PANOSE_WEIGHT = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_WEIGHT_BOOK: DWRITE_PANOSE_WEIGHT = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_WEIGHT_MEDIUM: DWRITE_PANOSE_WEIGHT = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_WEIGHT_DEMI: DWRITE_PANOSE_WEIGHT = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_WEIGHT_BOLD: DWRITE_PANOSE_WEIGHT = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_WEIGHT_HEAVY: DWRITE_PANOSE_WEIGHT = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_WEIGHT_BLACK: DWRITE_PANOSE_WEIGHT = 10i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_WEIGHT_EXTRA_BLACK: DWRITE_PANOSE_WEIGHT = 11i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_WEIGHT_NORD: DWRITE_PANOSE_WEIGHT = 11i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_PANOSE_XASCENT = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_XASCENT_ANY: DWRITE_PANOSE_XASCENT = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_XASCENT_NO_FIT: DWRITE_PANOSE_XASCENT = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_XASCENT_VERY_LOW: DWRITE_PANOSE_XASCENT = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_XASCENT_LOW: DWRITE_PANOSE_XASCENT = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_XASCENT_MEDIUM: DWRITE_PANOSE_XASCENT = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_XASCENT_HIGH: DWRITE_PANOSE_XASCENT = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_XASCENT_VERY_HIGH: DWRITE_PANOSE_XASCENT = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_PANOSE_XHEIGHT = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_XHEIGHT_ANY: DWRITE_PANOSE_XHEIGHT = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_XHEIGHT_NO_FIT: DWRITE_PANOSE_XHEIGHT = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_XHEIGHT_CONSTANT_SMALL: DWRITE_PANOSE_XHEIGHT = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_XHEIGHT_CONSTANT_STANDARD: DWRITE_PANOSE_XHEIGHT = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_XHEIGHT_CONSTANT_LARGE: DWRITE_PANOSE_XHEIGHT = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_XHEIGHT_DUCKING_SMALL: DWRITE_PANOSE_XHEIGHT = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_XHEIGHT_DUCKING_STANDARD: DWRITE_PANOSE_XHEIGHT = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_XHEIGHT_DUCKING_LARGE: DWRITE_PANOSE_XHEIGHT = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_XHEIGHT_CONSTANT_STD: DWRITE_PANOSE_XHEIGHT = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PANOSE_XHEIGHT_DUCKING_STD: DWRITE_PANOSE_XHEIGHT = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_PARAGRAPH_ALIGNMENT = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PARAGRAPH_ALIGNMENT_NEAR: DWRITE_PARAGRAPH_ALIGNMENT = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PARAGRAPH_ALIGNMENT_FAR: DWRITE_PARAGRAPH_ALIGNMENT = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PARAGRAPH_ALIGNMENT_CENTER: DWRITE_PARAGRAPH_ALIGNMENT = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_PIXEL_GEOMETRY = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PIXEL_GEOMETRY_FLAT: DWRITE_PIXEL_GEOMETRY = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PIXEL_GEOMETRY_RGB: DWRITE_PIXEL_GEOMETRY = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_PIXEL_GEOMETRY_BGR: DWRITE_PIXEL_GEOMETRY = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_READING_DIRECTION = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_READING_DIRECTION_LEFT_TO_RIGHT: DWRITE_READING_DIRECTION = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_READING_DIRECTION_RIGHT_TO_LEFT: DWRITE_READING_DIRECTION = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_READING_DIRECTION_TOP_TO_BOTTOM: DWRITE_READING_DIRECTION = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_READING_DIRECTION_BOTTOM_TO_TOP: DWRITE_READING_DIRECTION = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_RENDERING_MODE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_RENDERING_MODE_DEFAULT: DWRITE_RENDERING_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_RENDERING_MODE_ALIASED: DWRITE_RENDERING_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_RENDERING_MODE_GDI_CLASSIC: DWRITE_RENDERING_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_RENDERING_MODE_GDI_NATURAL: DWRITE_RENDERING_MODE = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_RENDERING_MODE_NATURAL: DWRITE_RENDERING_MODE = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_RENDERING_MODE_NATURAL_SYMMETRIC: DWRITE_RENDERING_MODE = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_RENDERING_MODE_OUTLINE: DWRITE_RENDERING_MODE = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_RENDERING_MODE_CLEARTYPE_GDI_CLASSIC: DWRITE_RENDERING_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_RENDERING_MODE_CLEARTYPE_GDI_NATURAL: DWRITE_RENDERING_MODE = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_RENDERING_MODE_CLEARTYPE_NATURAL: DWRITE_RENDERING_MODE = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_RENDERING_MODE_CLEARTYPE_NATURAL_SYMMETRIC: DWRITE_RENDERING_MODE = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_RENDERING_MODE1 = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_RENDERING_MODE1_DEFAULT: DWRITE_RENDERING_MODE1 = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_RENDERING_MODE1_ALIASED: DWRITE_RENDERING_MODE1 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_RENDERING_MODE1_GDI_CLASSIC: DWRITE_RENDERING_MODE1 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_RENDERING_MODE1_GDI_NATURAL: DWRITE_RENDERING_MODE1 = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_RENDERING_MODE1_NATURAL: DWRITE_RENDERING_MODE1 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_RENDERING_MODE1_NATURAL_SYMMETRIC: DWRITE_RENDERING_MODE1 = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_RENDERING_MODE1_OUTLINE: DWRITE_RENDERING_MODE1 = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_RENDERING_MODE1_NATURAL_SYMMETRIC_DOWNSAMPLED: DWRITE_RENDERING_MODE1 = 7i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_SCRIPT_ANALYSIS {
    pub script: u16,
    pub shapes: DWRITE_SCRIPT_SHAPES,
}
impl ::core::marker::Copy for DWRITE_SCRIPT_ANALYSIS {}
impl ::core::clone::Clone for DWRITE_SCRIPT_ANALYSIS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_SCRIPT_PROPERTIES {
    pub isoScriptCode: u32,
    pub isoScriptNumber: u32,
    pub clusterLookahead: u32,
    pub justificationCharacter: u32,
    pub _bitfield: u32,
}
impl ::core::marker::Copy for DWRITE_SCRIPT_PROPERTIES {}
impl ::core::clone::Clone for DWRITE_SCRIPT_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_SCRIPT_SHAPES = u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_SCRIPT_SHAPES_DEFAULT: DWRITE_SCRIPT_SHAPES = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_SCRIPT_SHAPES_NO_VISUAL: DWRITE_SCRIPT_SHAPES = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_SHAPING_GLYPH_PROPERTIES {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for DWRITE_SHAPING_GLYPH_PROPERTIES {}
impl ::core::clone::Clone for DWRITE_SHAPING_GLYPH_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_SHAPING_TEXT_PROPERTIES {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for DWRITE_SHAPING_TEXT_PROPERTIES {}
impl ::core::clone::Clone for DWRITE_SHAPING_TEXT_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_STRIKETHROUGH {
    pub width: f32,
    pub thickness: f32,
    pub offset: f32,
    pub readingDirection: DWRITE_READING_DIRECTION,
    pub flowDirection: DWRITE_FLOW_DIRECTION,
    pub localeName: ::windows_sys::core::PCWSTR,
    pub measuringMode: DWRITE_MEASURING_MODE,
}
impl ::core::marker::Copy for DWRITE_STRIKETHROUGH {}
impl ::core::clone::Clone for DWRITE_STRIKETHROUGH {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_TEXTURE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_TEXTURE_ALIASED_1x1: DWRITE_TEXTURE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_TEXTURE_CLEARTYPE_3x1: DWRITE_TEXTURE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_TEXT_ALIGNMENT = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_TEXT_ALIGNMENT_LEADING: DWRITE_TEXT_ALIGNMENT = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_TEXT_ALIGNMENT_TRAILING: DWRITE_TEXT_ALIGNMENT = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_TEXT_ALIGNMENT_CENTER: DWRITE_TEXT_ALIGNMENT = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_TEXT_ALIGNMENT_JUSTIFIED: DWRITE_TEXT_ALIGNMENT = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_TEXT_ANTIALIAS_MODE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_TEXT_ANTIALIAS_MODE_CLEARTYPE: DWRITE_TEXT_ANTIALIAS_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_TEXT_ANTIALIAS_MODE_GRAYSCALE: DWRITE_TEXT_ANTIALIAS_MODE = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_TEXT_METRICS {
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub widthIncludingTrailingWhitespace: f32,
    pub height: f32,
    pub layoutWidth: f32,
    pub layoutHeight: f32,
    pub maxBidiReorderingDepth: u32,
    pub lineCount: u32,
}
impl ::core::marker::Copy for DWRITE_TEXT_METRICS {}
impl ::core::clone::Clone for DWRITE_TEXT_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_TEXT_METRICS1 {
    pub Base: DWRITE_TEXT_METRICS,
    pub heightIncludingTrailingWhitespace: f32,
}
impl ::core::marker::Copy for DWRITE_TEXT_METRICS1 {}
impl ::core::clone::Clone for DWRITE_TEXT_METRICS1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_TEXT_RANGE {
    pub startPosition: u32,
    pub length: u32,
}
impl ::core::marker::Copy for DWRITE_TEXT_RANGE {}
impl ::core::clone::Clone for DWRITE_TEXT_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_TRIMMING {
    pub granularity: DWRITE_TRIMMING_GRANULARITY,
    pub delimiter: u32,
    pub delimiterCount: u32,
}
impl ::core::marker::Copy for DWRITE_TRIMMING {}
impl ::core::clone::Clone for DWRITE_TRIMMING {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_TRIMMING_GRANULARITY = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_TRIMMING_GRANULARITY_NONE: DWRITE_TRIMMING_GRANULARITY = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_TRIMMING_GRANULARITY_CHARACTER: DWRITE_TRIMMING_GRANULARITY = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_TRIMMING_GRANULARITY_WORD: DWRITE_TRIMMING_GRANULARITY = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_TYPOGRAPHIC_FEATURES {
    pub features: *mut DWRITE_FONT_FEATURE,
    pub featureCount: u32,
}
impl ::core::marker::Copy for DWRITE_TYPOGRAPHIC_FEATURES {}
impl ::core::clone::Clone for DWRITE_TYPOGRAPHIC_FEATURES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_UNDERLINE {
    pub width: f32,
    pub thickness: f32,
    pub offset: f32,
    pub runHeight: f32,
    pub readingDirection: DWRITE_READING_DIRECTION,
    pub flowDirection: DWRITE_FLOW_DIRECTION,
    pub localeName: ::windows_sys::core::PCWSTR,
    pub measuringMode: DWRITE_MEASURING_MODE,
}
impl ::core::marker::Copy for DWRITE_UNDERLINE {}
impl ::core::clone::Clone for DWRITE_UNDERLINE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub struct DWRITE_UNICODE_RANGE {
    pub first: u32,
    pub last: u32,
}
impl ::core::marker::Copy for DWRITE_UNICODE_RANGE {}
impl ::core::clone::Clone for DWRITE_UNICODE_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_VERTICAL_GLYPH_ORIENTATION = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_VERTICAL_GLYPH_ORIENTATION_DEFAULT: DWRITE_VERTICAL_GLYPH_ORIENTATION = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_VERTICAL_GLYPH_ORIENTATION_STACKED: DWRITE_VERTICAL_GLYPH_ORIENTATION = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub type DWRITE_WORD_WRAPPING = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_WORD_WRAPPING_WRAP: DWRITE_WORD_WRAPPING = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_WORD_WRAPPING_NO_WRAP: DWRITE_WORD_WRAPPING = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_WORD_WRAPPING_EMERGENCY_BREAK: DWRITE_WORD_WRAPPING = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_WORD_WRAPPING_WHOLE_WORD: DWRITE_WORD_WRAPPING = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const DWRITE_WORD_WRAPPING_CHARACTER: DWRITE_WORD_WRAPPING = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
pub const FACILITY_DWRITE: u32 = 2200u32;
#[repr(C)]
pub struct IDWriteAsyncResult {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWaitHandle: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::HANDLE,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWaitHandle: usize,
    pub GetResult: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteAsyncResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3458595069, data2: 34363, data3: 19731, data4: [150, 81, 193, 248, 141, 199, 63, 226] };
}
#[repr(C)]
pub struct IDWriteBitmapRenderTarget {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub DrawGlyphRun: unsafe extern "system" fn(this: *mut *mut Self, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, renderingparams: *mut ::core::ffi::c_void, textcolor: u32, blackboxrect: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DrawGlyphRun: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetMemoryDC: unsafe extern "system" fn(this: *mut *mut Self) -> super::Gdi::HDC,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetMemoryDC: usize,
    pub GetPixelsPerDip: unsafe extern "system" fn(this: *mut *mut Self) -> f32,
    pub SetPixelsPerDip: unsafe extern "system" fn(this: *mut *mut Self, pixelsperdip: f32) -> ::windows_sys::core::HRESULT,
    pub GetCurrentTransform: unsafe extern "system" fn(this: *mut *mut Self, transform: *mut DWRITE_MATRIX) -> ::windows_sys::core::HRESULT,
    pub SetCurrentTransform: unsafe extern "system" fn(this: *mut *mut Self, transform: *const DWRITE_MATRIX) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSize: unsafe extern "system" fn(this: *mut *mut Self, size: *mut super::super::Foundation::SIZE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSize: usize,
    pub Resize: unsafe extern "system" fn(this: *mut *mut Self, width: u32, height: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteBitmapRenderTarget {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1582969507, data2: 36351, data3: 18291, data4: [159, 246, 6, 150, 234, 183, 114, 103] };
}
#[repr(C)]
pub struct IDWriteBitmapRenderTarget1 {
    pub base__: IDWriteBitmapRenderTarget,
    pub GetTextAntialiasMode: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_TEXT_ANTIALIAS_MODE,
    pub SetTextAntialiasMode: unsafe extern "system" fn(this: *mut *mut Self, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteBitmapRenderTarget1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2032042648, data2: 16115, data3: 16944, data4: [152, 128, 201, 189, 236, 196, 32, 100] };
}
#[repr(C)]
pub struct IDWriteColorGlyphRunEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, hasrun: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCurrentRun: unsafe extern "system" fn(this: *mut *mut Self, colorglyphrun: *mut *mut DWRITE_COLOR_GLYPH_RUN) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCurrentRun: usize,
}
impl ::windows_sys::core::Interface for IDWriteColorGlyphRunEnumerator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3542072855, data2: 61783, data3: 16802, data4: [141, 36, 203, 119, 158, 5, 96, 232] };
}
#[repr(C)]
pub struct IDWriteColorGlyphRunEnumerator1 {
    pub base__: IDWriteColorGlyphRunEnumerator,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCurrentRun2: unsafe extern "system" fn(this: *mut *mut Self, colorglyphrun: *mut *mut DWRITE_COLOR_GLYPH_RUN1) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCurrentRun2: usize,
}
impl ::windows_sys::core::Interface for IDWriteColorGlyphRunEnumerator1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2086635226, data2: 51105, data3: 20229, data4: [184, 225, 85, 161, 121, 254, 90, 53] };
}
#[repr(C)]
pub struct IDWriteFactory {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSystemFontCollection: unsafe extern "system" fn(this: *mut *mut Self, fontcollection: *mut *mut ::core::ffi::c_void, checkforupdates: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSystemFontCollection: usize,
    pub CreateCustomFontCollection: unsafe extern "system" fn(this: *mut *mut Self, collectionloader: *mut ::core::ffi::c_void, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32, fontcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RegisterFontCollectionLoader: unsafe extern "system" fn(this: *mut *mut Self, fontcollectionloader: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UnregisterFontCollectionLoader: unsafe extern "system" fn(this: *mut *mut Self, fontcollectionloader: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateFontFileReference: unsafe extern "system" fn(this: *mut *mut Self, filepath: ::windows_sys::core::PCWSTR, lastwritetime: *const super::super::Foundation::FILETIME, fontfile: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateFontFileReference: usize,
    pub CreateCustomFontFileReference: unsafe extern "system" fn(this: *mut *mut Self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfileloader: *mut ::core::ffi::c_void, fontfile: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFontFace: unsafe extern "system" fn(this: *mut *mut Self, fontfacetype: DWRITE_FONT_FACE_TYPE, numberoffiles: u32, fontfiles: *const *mut ::core::ffi::c_void, faceindex: u32, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS, fontface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateRenderingParams: unsafe extern "system" fn(this: *mut *mut Self, renderingparams: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CreateMonitorRenderingParams: unsafe extern "system" fn(this: *mut *mut Self, monitor: super::Gdi::HMONITOR, renderingparams: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CreateMonitorRenderingParams: usize,
    pub CreateCustomRenderingParams: unsafe extern "system" fn(this: *mut *mut Self, gamma: f32, enhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, renderingparams: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RegisterFontFileLoader: unsafe extern "system" fn(this: *mut *mut Self, fontfileloader: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UnregisterFontFileLoader: unsafe extern "system" fn(this: *mut *mut Self, fontfileloader: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateTextFormat: unsafe extern "system" fn(this: *mut *mut Self, fontfamilyname: ::windows_sys::core::PCWSTR, fontcollection: *mut ::core::ffi::c_void, fontweight: DWRITE_FONT_WEIGHT, fontstyle: DWRITE_FONT_STYLE, fontstretch: DWRITE_FONT_STRETCH, fontsize: f32, localename: ::windows_sys::core::PCWSTR, textformat: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateTypography: unsafe extern "system" fn(this: *mut *mut Self, typography: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetGdiInterop: unsafe extern "system" fn(this: *mut *mut Self, gdiinterop: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateTextLayout: unsafe extern "system" fn(this: *mut *mut Self, string: ::windows_sys::core::PCWSTR, stringlength: u32, textformat: *mut ::core::ffi::c_void, maxwidth: f32, maxheight: f32, textlayout: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateGdiCompatibleTextLayout: unsafe extern "system" fn(this: *mut *mut Self, string: ::windows_sys::core::PCWSTR, stringlength: u32, textformat: *mut ::core::ffi::c_void, layoutwidth: f32, layoutheight: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL, textlayout: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateGdiCompatibleTextLayout: usize,
    pub CreateEllipsisTrimmingSign: unsafe extern "system" fn(this: *mut *mut Self, textformat: *mut ::core::ffi::c_void, trimmingsign: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateTextAnalyzer: unsafe extern "system" fn(this: *mut *mut Self, textanalyzer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateNumberSubstitution: unsafe extern "system" fn(this: *mut *mut Self, substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD, localename: ::windows_sys::core::PCWSTR, ignoreuseroverride: super::super::Foundation::BOOL, numbersubstitution: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateNumberSubstitution: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateGlyphRunAnalysis: unsafe extern "system" fn(this: *mut *mut Self, glyphrun: *const DWRITE_GLYPH_RUN, pixelsperdip: f32, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, baselineoriginx: f32, baselineoriginy: f32, glyphrunanalysis: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateGlyphRunAnalysis: usize,
}
impl ::windows_sys::core::Interface for IDWriteFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3092901466, data2: 55352, data3: 19291, data4: [162, 232, 26, 220, 125, 147, 219, 72] };
}
#[repr(C)]
pub struct IDWriteFactory1 {
    pub base__: IDWriteFactory,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEudcFontCollection: unsafe extern "system" fn(this: *mut *mut Self, fontcollection: *mut *mut ::core::ffi::c_void, checkforupdates: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEudcFontCollection: usize,
    pub CreateCustomRenderingParams2: unsafe extern "system" fn(this: *mut *mut Self, gamma: f32, enhancedcontrast: f32, enhancedcontrastgrayscale: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, renderingparams: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteFactory1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 811020185, data2: 56006, data3: 16859, data4: [161, 110, 4, 134, 48, 126, 96, 106] };
}
#[repr(C)]
pub struct IDWriteFactory2 {
    pub base__: IDWriteFactory1,
    pub GetSystemFontFallback: unsafe extern "system" fn(this: *mut *mut Self, fontfallback: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFontFallbackBuilder: unsafe extern "system" fn(this: *mut *mut Self, fontfallbackbuilder: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TranslateColorGlyphRun: unsafe extern "system" fn(this: *mut *mut Self, baselineoriginx: f32, baselineoriginy: f32, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, measuringmode: DWRITE_MEASURING_MODE, worldtodevicetransform: *const DWRITE_MATRIX, colorpaletteindex: u32, colorlayers: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TranslateColorGlyphRun: usize,
    pub CreateCustomRenderingParams3: unsafe extern "system" fn(this: *mut *mut Self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, renderingparams: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateGlyphRunAnalysis2: unsafe extern "system" fn(this: *mut *mut Self, glyphrun: *const DWRITE_GLYPH_RUN, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32, glyphrunanalysis: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateGlyphRunAnalysis2: usize,
}
impl ::windows_sys::core::Interface for IDWriteFactory2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 70909024, data2: 51780, data3: 18836, data4: [141, 238, 58, 154, 247, 183, 50, 236] };
}
#[repr(C)]
pub struct IDWriteFactory3 {
    pub base__: IDWriteFactory2,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateGlyphRunAnalysis3: unsafe extern "system" fn(this: *mut *mut Self, glyphrun: *const DWRITE_GLYPH_RUN, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE1, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32, glyphrunanalysis: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateGlyphRunAnalysis3: usize,
    pub CreateCustomRenderingParams4: unsafe extern "system" fn(this: *mut *mut Self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE1, gridfitmode: DWRITE_GRID_FIT_MODE, renderingparams: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFontFaceReference: unsafe extern "system" fn(this: *mut *mut Self, fontfile: *mut ::core::ffi::c_void, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateFontFaceReference2: unsafe extern "system" fn(this: *mut *mut Self, filepath: ::windows_sys::core::PCWSTR, lastwritetime: *const super::super::Foundation::FILETIME, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateFontFaceReference2: usize,
    pub GetSystemFontSet: unsafe extern "system" fn(this: *mut *mut Self, fontset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFontSetBuilder: unsafe extern "system" fn(this: *mut *mut Self, fontsetbuilder: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFontCollectionFromFontSet: unsafe extern "system" fn(this: *mut *mut Self, fontset: *mut ::core::ffi::c_void, fontcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSystemFontCollection2: unsafe extern "system" fn(this: *mut *mut Self, includedownloadablefonts: super::super::Foundation::BOOL, fontcollection: *mut *mut ::core::ffi::c_void, checkforupdates: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSystemFontCollection2: usize,
    pub GetFontDownloadQueue: unsafe extern "system" fn(this: *mut *mut Self, fontdownloadqueue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteFactory3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2585477571, data2: 54203, data3: 18026, data4: [135, 252, 254, 103, 85, 106, 59, 101] };
}
#[repr(C)]
pub struct IDWriteFactory4 {
    pub base__: IDWriteFactory3,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub TranslateColorGlyphRun2: unsafe extern "system" fn(this: *mut *mut Self, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, desiredglyphimageformats: DWRITE_GLYPH_IMAGE_FORMATS, measuringmode: DWRITE_MEASURING_MODE, worldanddpitransform: *const DWRITE_MATRIX, colorpaletteindex: u32, colorlayers: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common")))]
    TranslateColorGlyphRun2: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub ComputeGlyphOrigins: unsafe extern "system" fn(this: *mut *mut Self, glyphrun: *const DWRITE_GLYPH_RUN, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, glyphorigins: *mut super::Direct2D::Common::D2D_POINT_2F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common")))]
    ComputeGlyphOrigins: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub ComputeGlyphOrigins2: unsafe extern "system" fn(this: *mut *mut Self, glyphrun: *const DWRITE_GLYPH_RUN, measuringmode: DWRITE_MEASURING_MODE, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, worldanddpitransform: *const DWRITE_MATRIX, glyphorigins: *mut super::Direct2D::Common::D2D_POINT_2F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common")))]
    ComputeGlyphOrigins2: usize,
}
impl ::windows_sys::core::Interface for IDWriteFactory4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1259035603, data2: 1943, data3: 17737, data4: [138, 197, 254, 145, 92, 197, 56, 86] };
}
#[repr(C)]
pub struct IDWriteFactory5 {
    pub base__: IDWriteFactory4,
    pub CreateFontSetBuilder2: unsafe extern "system" fn(this: *mut *mut Self, fontsetbuilder: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateInMemoryFontFileLoader: unsafe extern "system" fn(this: *mut *mut Self, newloader: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateHttpFontFileLoader: unsafe extern "system" fn(this: *mut *mut Self, referrerurl: ::windows_sys::core::PCWSTR, extraheaders: ::windows_sys::core::PCWSTR, newloader: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AnalyzeContainerType: unsafe extern "system" fn(this: *mut *mut Self, filedata: *const ::core::ffi::c_void, filedatasize: u32) -> DWRITE_CONTAINER_TYPE,
    pub UnpackFontFile: unsafe extern "system" fn(this: *mut *mut Self, containertype: DWRITE_CONTAINER_TYPE, filedata: *const ::core::ffi::c_void, filedatasize: u32, unpackedfontstream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteFactory5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2509093274, data2: 48682, data3: 20233, data4: [175, 125, 101, 24, 152, 3, 209, 211] };
}
#[repr(C)]
pub struct IDWriteFactory6 {
    pub base__: IDWriteFactory5,
    pub CreateFontFaceReference3: unsafe extern "system" fn(this: *mut *mut Self, fontfile: *mut ::core::ffi::c_void, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFontResource: unsafe extern "system" fn(this: *mut *mut Self, fontfile: *mut ::core::ffi::c_void, faceindex: u32, fontresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSystemFontSet2: unsafe extern "system" fn(this: *mut *mut Self, includedownloadablefonts: super::super::Foundation::BOOL, fontset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSystemFontSet2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSystemFontCollection3: unsafe extern "system" fn(this: *mut *mut Self, includedownloadablefonts: super::super::Foundation::BOOL, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, fontcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSystemFontCollection3: usize,
    pub CreateFontCollectionFromFontSet2: unsafe extern "system" fn(this: *mut *mut Self, fontset: *mut ::core::ffi::c_void, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, fontcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFontSetBuilder3: unsafe extern "system" fn(this: *mut *mut Self, fontsetbuilder: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateTextFormat2: unsafe extern "system" fn(this: *mut *mut Self, fontfamilyname: ::windows_sys::core::PCWSTR, fontcollection: *mut ::core::ffi::c_void, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontsize: f32, localename: ::windows_sys::core::PCWSTR, textformat: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteFactory6 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4084485504, data2: 8695, data3: 17131, data4: [179, 93, 153, 91, 199, 47, 194, 35] };
}
#[repr(C)]
pub struct IDWriteFactory7 {
    pub base__: IDWriteFactory6,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSystemFontSet3: unsafe extern "system" fn(this: *mut *mut Self, includedownloadablefonts: super::super::Foundation::BOOL, fontset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSystemFontSet3: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSystemFontCollection4: unsafe extern "system" fn(this: *mut *mut Self, includedownloadablefonts: super::super::Foundation::BOOL, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, fontcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSystemFontCollection4: usize,
}
impl ::windows_sys::core::Interface for IDWriteFactory7 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 902881459, data2: 36982, data3: 19758, data4: [160, 22, 169, 27, 86, 138, 6, 180] };
}
#[repr(C)]
pub struct IDWriteFont {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetFontFamily: unsafe extern "system" fn(this: *mut *mut Self, fontfamily: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetWeight: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_FONT_WEIGHT,
    pub GetStretch: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_FONT_STRETCH,
    pub GetStyle: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_FONT_STYLE,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSymbolFont: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSymbolFont: usize,
    pub GetFaceNames: unsafe extern "system" fn(this: *mut *mut Self, names: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetInformationalStrings: unsafe extern "system" fn(this: *mut *mut Self, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut *mut ::core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetInformationalStrings: usize,
    pub GetSimulations: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_FONT_SIMULATIONS,
    pub GetMetrics: unsafe extern "system" fn(this: *mut *mut Self, fontmetrics: *mut DWRITE_FONT_METRICS),
    #[cfg(feature = "Win32_Foundation")]
    pub HasCharacter: unsafe extern "system" fn(this: *mut *mut Self, unicodevalue: u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasCharacter: usize,
    pub CreateFontFace: unsafe extern "system" fn(this: *mut *mut Self, fontface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteFont {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2899404438, data2: 35860, data3: 20317, data4: [135, 126, 254, 63, 193, 211, 39, 55] };
}
#[repr(C)]
pub struct IDWriteFont1 {
    pub base__: IDWriteFont,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMetrics2: unsafe extern "system" fn(this: *mut *mut Self, fontmetrics: *mut DWRITE_FONT_METRICS1),
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMetrics2: usize,
    pub GetPanose: unsafe extern "system" fn(this: *mut *mut Self, panose: *mut DWRITE_PANOSE),
    pub GetUnicodeRanges: unsafe extern "system" fn(this: *mut *mut Self, maxrangecount: u32, unicoderanges: *mut DWRITE_UNICODE_RANGE, actualrangecount: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsMonospacedFont: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsMonospacedFont: usize,
}
impl ::windows_sys::core::Interface for IDWriteFont1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2899404438, data2: 35860, data3: 20317, data4: [135, 126, 254, 63, 193, 211, 39, 56] };
}
#[repr(C)]
pub struct IDWriteFont2 {
    pub base__: IDWriteFont1,
    #[cfg(feature = "Win32_Foundation")]
    pub IsColorFont: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsColorFont: usize,
}
impl ::windows_sys::core::Interface for IDWriteFont2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 695504598, data2: 35996, data3: 19050, data4: [190, 11, 217, 18, 232, 83, 137, 68] };
}
#[repr(C)]
pub struct IDWriteFont3 {
    pub base__: IDWriteFont2,
    pub CreateFontFace2: unsafe extern "system" fn(this: *mut *mut Self, fontface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Equals: unsafe extern "system" fn(this: *mut *mut Self, font: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    Equals: usize,
    pub GetFontFaceReference: unsafe extern "system" fn(this: *mut *mut Self, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HasCharacter2: unsafe extern "system" fn(this: *mut *mut Self, unicodevalue: u32) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasCharacter2: usize,
    pub GetLocality: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_LOCALITY,
}
impl ::windows_sys::core::Interface for IDWriteFont3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 695504598, data2: 35996, data3: 19050, data4: [190, 11, 217, 18, 232, 83, 137, 68] };
}
#[repr(C)]
pub struct IDWriteFontCollection {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetFontFamilyCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetFontFamily: unsafe extern "system" fn(this: *mut *mut Self, index: u32, fontfamily: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FindFamilyName: unsafe extern "system" fn(this: *mut *mut Self, familyname: ::windows_sys::core::PCWSTR, index: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindFamilyName: usize,
    pub GetFontFromFontFace: unsafe extern "system" fn(this: *mut *mut Self, fontface: *mut ::core::ffi::c_void, font: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteFontCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2823613954, data2: 16106, data3: 20206, data4: [168, 39, 135, 193, 160, 42, 15, 204] };
}
#[repr(C)]
pub struct IDWriteFontCollection1 {
    pub base__: IDWriteFontCollection,
    pub GetFontSet: unsafe extern "system" fn(this: *mut *mut Self, fontset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFontFamily2: unsafe extern "system" fn(this: *mut *mut Self, index: u32, fontfamily: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteFontCollection1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1398296897, data2: 55800, data3: 16533, data4: [131, 33, 215, 60, 246, 189, 17, 108] };
}
#[repr(C)]
pub struct IDWriteFontCollection2 {
    pub base__: IDWriteFontCollection1,
    pub GetFontFamily3: unsafe extern "system" fn(this: *mut *mut Self, index: u32, fontfamily: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetMatchingFonts: unsafe extern "system" fn(this: *mut *mut Self, familyname: ::windows_sys::core::PCWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontlist: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFontFamilyModel: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_FONT_FAMILY_MODEL,
    pub GetFontSet2: unsafe extern "system" fn(this: *mut *mut Self, fontset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteFontCollection2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1363163590, data2: 17943, data3: 16484, data4: [191, 139, 146, 234, 131, 229, 6, 224] };
}
#[repr(C)]
pub struct IDWriteFontCollection3 {
    pub base__: IDWriteFontCollection2,
    #[cfg(feature = "Win32_Foundation")]
    pub GetExpirationEvent: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::HANDLE,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetExpirationEvent: usize,
}
impl ::windows_sys::core::Interface for IDWriteFontCollection3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2765116838, data2: 63971, data3: 20005, data4: [147, 183, 158, 48, 159, 58, 248, 233] };
}
#[repr(C)]
pub struct IDWriteFontCollectionLoader {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateEnumeratorFromKey: unsafe extern "system" fn(this: *mut *mut Self, factory: *mut ::core::ffi::c_void, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32, fontfileenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteFontCollectionLoader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3433636068, data2: 21232, data3: 18731, data4: [191, 168, 41, 199, 46, 224, 164, 104] };
}
#[repr(C)]
pub struct IDWriteFontDownloadListener {
    pub base__: ::windows_sys::core::IUnknown,
    pub DownloadCompleted: unsafe extern "system" fn(this: *mut *mut Self, downloadqueue: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void, downloadresult: ::windows_sys::core::HRESULT),
}
impl ::windows_sys::core::Interface for IDWriteFontDownloadListener {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2960123321, data2: 17388, data3: 17299, data4: [136, 27, 219, 228, 220, 114, 253, 167] };
}
#[repr(C)]
pub struct IDWriteFontDownloadQueue {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddListener: unsafe extern "system" fn(this: *mut *mut Self, listener: *mut ::core::ffi::c_void, token: *mut u32) -> ::windows_sys::core::HRESULT,
    pub RemoveListener: unsafe extern "system" fn(this: *mut *mut Self, token: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEmpty: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEmpty: usize,
    pub BeginDownload: unsafe extern "system" fn(this: *mut *mut Self, context: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CancelDownload: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetGenerationCount: unsafe extern "system" fn(this: *mut *mut Self) -> u64,
}
impl ::windows_sys::core::Interface for IDWriteFontDownloadQueue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3072221266, data2: 23274, data3: 20387, data4: [131, 46, 246, 13, 67, 31, 126, 145] };
}
#[repr(C)]
pub struct IDWriteFontFace {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetType: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_FONT_FACE_TYPE,
    pub GetFiles: unsafe extern "system" fn(this: *mut *mut Self, numberoffiles: *mut u32, fontfiles: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetIndex: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetSimulations: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_FONT_SIMULATIONS,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSymbolFont: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSymbolFont: usize,
    pub GetMetrics: unsafe extern "system" fn(this: *mut *mut Self, fontfacemetrics: *mut DWRITE_FONT_METRICS),
    pub GetGlyphCount: unsafe extern "system" fn(this: *mut *mut Self) -> u16,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDesignGlyphMetrics: unsafe extern "system" fn(this: *mut *mut Self, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDesignGlyphMetrics: usize,
    pub GetGlyphIndices: unsafe extern "system" fn(this: *mut *mut Self, codepoints: *const u32, codepointcount: u32, glyphindices: *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TryGetFontTable: unsafe extern "system" fn(this: *mut *mut Self, opentypetabletag: u32, tabledata: *mut *mut ::core::ffi::c_void, tablesize: *mut u32, tablecontext: *mut *mut ::core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TryGetFontTable: usize,
    pub ReleaseFontTable: unsafe extern "system" fn(this: *mut *mut Self, tablecontext: *const ::core::ffi::c_void),
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub GetGlyphRunOutline: unsafe extern "system" fn(this: *mut *mut Self, emsize: f32, glyphindices: *const u16, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphcount: u32, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, geometrysink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common")))]
    GetGlyphRunOutline: usize,
    pub GetRecommendedRenderingMode: unsafe extern "system" fn(this: *mut *mut Self, emsize: f32, pixelsperdip: f32, measuringmode: DWRITE_MEASURING_MODE, renderingparams: *mut ::core::ffi::c_void, renderingmode: *mut DWRITE_RENDERING_MODE) -> ::windows_sys::core::HRESULT,
    pub GetGdiCompatibleMetrics: unsafe extern "system" fn(this: *mut *mut Self, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, fontfacemetrics: *mut DWRITE_FONT_METRICS) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetGdiCompatibleGlyphMetrics: unsafe extern "system" fn(this: *mut *mut Self, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetGdiCompatibleGlyphMetrics: usize,
}
impl ::windows_sys::core::Interface for IDWriteFontFace {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1598652493, data2: 28708, data3: 19779, data4: [191, 169, 210, 89, 132, 245, 56, 73] };
}
#[repr(C)]
pub struct IDWriteFontFace1 {
    pub base__: IDWriteFontFace,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMetrics2: unsafe extern "system" fn(this: *mut *mut Self, fontmetrics: *mut DWRITE_FONT_METRICS1),
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMetrics2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetGdiCompatibleMetrics2: unsafe extern "system" fn(this: *mut *mut Self, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, fontmetrics: *mut DWRITE_FONT_METRICS1) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetGdiCompatibleMetrics2: usize,
    pub GetCaretMetrics: unsafe extern "system" fn(this: *mut *mut Self, caretmetrics: *mut DWRITE_CARET_METRICS),
    pub GetUnicodeRanges: unsafe extern "system" fn(this: *mut *mut Self, maxrangecount: u32, unicoderanges: *mut DWRITE_UNICODE_RANGE, actualrangecount: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsMonospacedFont: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsMonospacedFont: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDesignGlyphAdvances: unsafe extern "system" fn(this: *mut *mut Self, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32, issideways: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDesignGlyphAdvances: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetGdiCompatibleGlyphAdvances: unsafe extern "system" fn(this: *mut *mut Self, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL, issideways: super::super::Foundation::BOOL, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetGdiCompatibleGlyphAdvances: usize,
    pub GetKerningPairAdjustments: unsafe extern "system" fn(this: *mut *mut Self, glyphcount: u32, glyphindices: *const u16, glyphadvanceadjustments: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HasKerningPairs: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasKerningPairs: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRecommendedRenderingMode2: unsafe extern "system" fn(this: *mut *mut Self, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const DWRITE_MATRIX, issideways: super::super::Foundation::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingmode: *mut DWRITE_RENDERING_MODE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRecommendedRenderingMode2: usize,
    pub GetVerticalGlyphVariants: unsafe extern "system" fn(this: *mut *mut Self, glyphcount: u32, nominalglyphindices: *const u16, verticalglyphindices: *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HasVerticalGlyphVariants: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasVerticalGlyphVariants: usize,
}
impl ::windows_sys::core::Interface for IDWriteFontFace1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2803826100, data2: 40923, data3: 18488, data4: [173, 144, 207, 195, 190, 140, 61, 175] };
}
#[repr(C)]
pub struct IDWriteFontFace2 {
    pub base__: IDWriteFontFace1,
    #[cfg(feature = "Win32_Foundation")]
    pub IsColorFont: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsColorFont: usize,
    pub GetColorPaletteCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetPaletteEntryCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetPaletteEntries: unsafe extern "system" fn(this: *mut *mut Self, colorpaletteindex: u32, firstentryindex: u32, entrycount: u32, paletteentries: *mut DWRITE_COLOR_F) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRecommendedRenderingMode3: unsafe extern "system" fn(this: *mut *mut Self, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const DWRITE_MATRIX, issideways: super::super::Foundation::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: *mut ::core::ffi::c_void, renderingmode: *mut DWRITE_RENDERING_MODE, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRecommendedRenderingMode3: usize,
}
impl ::windows_sys::core::Interface for IDWriteFontFace2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3635898623, data2: 25788, data3: 20070, data4: [152, 43, 236, 142, 135, 246, 147, 247] };
}
#[repr(C)]
pub struct IDWriteFontFace3 {
    pub base__: IDWriteFontFace2,
    pub GetFontFaceReference: unsafe extern "system" fn(this: *mut *mut Self, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPanose: unsafe extern "system" fn(this: *mut *mut Self, panose: *mut DWRITE_PANOSE),
    pub GetWeight: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_FONT_WEIGHT,
    pub GetStretch: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_FONT_STRETCH,
    pub GetStyle: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_FONT_STYLE,
    pub GetFamilyNames: unsafe extern "system" fn(this: *mut *mut Self, names: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFaceNames: unsafe extern "system" fn(this: *mut *mut Self, names: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetInformationalStrings: unsafe extern "system" fn(this: *mut *mut Self, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut *mut ::core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetInformationalStrings: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub HasCharacter: unsafe extern "system" fn(this: *mut *mut Self, unicodevalue: u32) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasCharacter: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRecommendedRenderingMode4: unsafe extern "system" fn(this: *mut *mut Self, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const DWRITE_MATRIX, issideways: super::super::Foundation::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: *mut ::core::ffi::c_void, renderingmode: *mut DWRITE_RENDERING_MODE1, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRecommendedRenderingMode4: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsCharacterLocal: unsafe extern "system" fn(this: *mut *mut Self, unicodevalue: u32) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsCharacterLocal: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsGlyphLocal: unsafe extern "system" fn(this: *mut *mut Self, glyphid: u16) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsGlyphLocal: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AreCharactersLocal: unsafe extern "system" fn(this: *mut *mut Self, characters: ::windows_sys::core::PCWSTR, charactercount: u32, enqueueifnotlocal: super::super::Foundation::BOOL, islocal: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AreCharactersLocal: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AreGlyphsLocal: unsafe extern "system" fn(this: *mut *mut Self, glyphindices: *const u16, glyphcount: u32, enqueueifnotlocal: super::super::Foundation::BOOL, islocal: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AreGlyphsLocal: usize,
}
impl ::windows_sys::core::Interface for IDWriteFontFace3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3548214680, data2: 2494, data3: 16930, data4: [162, 54, 32, 129, 52, 28, 193, 242] };
}
#[repr(C)]
pub struct IDWriteFontFace4 {
    pub base__: IDWriteFontFace3,
    pub GetGlyphImageFormats: unsafe extern "system" fn(this: *mut *mut Self, glyphid: u16, pixelsperemfirst: u32, pixelsperemlast: u32, glyphimageformats: *mut DWRITE_GLYPH_IMAGE_FORMATS) -> ::windows_sys::core::HRESULT,
    pub GetGlyphImageFormats2: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_GLYPH_IMAGE_FORMATS,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub GetGlyphImageData: unsafe extern "system" fn(this: *mut *mut Self, glyphid: u16, pixelsperem: u32, glyphimageformat: DWRITE_GLYPH_IMAGE_FORMATS, glyphdata: *mut DWRITE_GLYPH_IMAGE_DATA, glyphdatacontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common")))]
    GetGlyphImageData: usize,
    pub ReleaseGlyphImageData: unsafe extern "system" fn(this: *mut *mut Self, glyphdatacontext: *mut ::core::ffi::c_void),
}
impl ::windows_sys::core::Interface for IDWriteFontFace4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 670214404, data2: 20152, data3: 17437, data4: [150, 120, 5, 99, 245, 62, 62, 47] };
}
#[repr(C)]
pub struct IDWriteFontFace5 {
    pub base__: IDWriteFontFace4,
    pub GetFontAxisValueCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetFontAxisValues: unsafe extern "system" fn(this: *mut *mut Self, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HasVariations: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasVariations: usize,
    pub GetFontResource: unsafe extern "system" fn(this: *mut *mut Self, fontresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Equals: unsafe extern "system" fn(this: *mut *mut Self, fontface: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    Equals: usize,
}
impl ::windows_sys::core::Interface for IDWriteFontFace5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2565862309, data2: 46695, data3: 18330, data4: [177, 69, 226, 250, 91, 159, 220, 41] };
}
#[repr(C)]
pub struct IDWriteFontFace6 {
    pub base__: IDWriteFontFace5,
    pub GetFamilyNames2: unsafe extern "system" fn(this: *mut *mut Self, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, names: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFaceNames2: unsafe extern "system" fn(this: *mut *mut Self, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, names: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteFontFace6 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3299999259, data2: 28292, data3: 18389, data4: [181, 76, 165, 151, 152, 27, 6, 173] };
}
#[repr(C)]
pub struct IDWriteFontFaceReference {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateFontFace: unsafe extern "system" fn(this: *mut *mut Self, fontface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFontFaceWithSimulations: unsafe extern "system" fn(this: *mut *mut Self, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS, fontface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Equals: unsafe extern "system" fn(this: *mut *mut Self, fontfacereference: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    Equals: usize,
    pub GetFontFaceIndex: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetSimulations: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_FONT_SIMULATIONS,
    pub GetFontFile: unsafe extern "system" fn(this: *mut *mut Self, fontfile: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetLocalFileSize: unsafe extern "system" fn(this: *mut *mut Self) -> u64,
    pub GetFileSize: unsafe extern "system" fn(this: *mut *mut Self) -> u64,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFileTime: unsafe extern "system" fn(this: *mut *mut Self, lastwritetime: *mut super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFileTime: usize,
    pub GetLocality: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_LOCALITY,
    pub EnqueueFontDownloadRequest: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub EnqueueCharacterDownloadRequest: unsafe extern "system" fn(this: *mut *mut Self, characters: ::windows_sys::core::PCWSTR, charactercount: u32) -> ::windows_sys::core::HRESULT,
    pub EnqueueGlyphDownloadRequest: unsafe extern "system" fn(this: *mut *mut Self, glyphindices: *const u16, glyphcount: u32) -> ::windows_sys::core::HRESULT,
    pub EnqueueFileFragmentDownloadRequest: unsafe extern "system" fn(this: *mut *mut Self, fileoffset: u64, fragmentsize: u64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteFontFaceReference {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1585424330, data2: 56803, data3: 16972, data4: [137, 240, 159, 205, 111, 237, 88, 205] };
}
#[repr(C)]
pub struct IDWriteFontFaceReference1 {
    pub base__: IDWriteFontFaceReference,
    pub CreateFontFace2: unsafe extern "system" fn(this: *mut *mut Self, fontface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFontAxisValueCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetFontAxisValues: unsafe extern "system" fn(this: *mut *mut Self, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteFontFaceReference1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3229744759, data2: 12241, data3: 16812, data4: [165, 163, 52, 152, 60, 75, 166, 26] };
}
#[repr(C)]
pub struct IDWriteFontFallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub MapCharacters: unsafe extern "system" fn(this: *mut *mut Self, analysissource: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, basefontcollection: *mut ::core::ffi::c_void, basefamilyname: ::windows_sys::core::PCWSTR, baseweight: DWRITE_FONT_WEIGHT, basestyle: DWRITE_FONT_STYLE, basestretch: DWRITE_FONT_STRETCH, mappedlength: *mut u32, mappedfont: *mut *mut ::core::ffi::c_void, scale: *mut f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteFontFallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4020242681, data2: 63393, data3: 18623, data4: [176, 92, 242, 36, 113, 60, 192, 255] };
}
#[repr(C)]
pub struct IDWriteFontFallback1 {
    pub base__: IDWriteFontFallback,
    pub MapCharacters2: unsafe extern "system" fn(this: *mut *mut Self, analysissource: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, basefontcollection: *mut ::core::ffi::c_void, basefamilyname: ::windows_sys::core::PCWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, mappedlength: *mut u32, scale: *mut f32, mappedfontface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteFontFallback1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 597121437, data2: 56589, data3: 18049, data4: [189, 106, 244, 243, 30, 170, 222, 119] };
}
#[repr(C)]
pub struct IDWriteFontFallbackBuilder {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddMapping: unsafe extern "system" fn(this: *mut *mut Self, ranges: *const DWRITE_UNICODE_RANGE, rangescount: u32, targetfamilynames: *const *const u16, targetfamilynamescount: u32, fontcollection: *mut ::core::ffi::c_void, localename: ::windows_sys::core::PCWSTR, basefamilyname: ::windows_sys::core::PCWSTR, scale: f32) -> ::windows_sys::core::HRESULT,
    pub AddMappings: unsafe extern "system" fn(this: *mut *mut Self, fontfallback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFontFallback: unsafe extern "system" fn(this: *mut *mut Self, fontfallback: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteFontFallbackBuilder {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4253560070, data2: 35514, data3: 20408, data4: [184, 73, 139, 232, 183, 62, 20, 222] };
}
#[repr(C)]
pub struct IDWriteFontFamily {
    pub base__: IDWriteFontList,
    pub GetFamilyNames: unsafe extern "system" fn(this: *mut *mut Self, names: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFirstMatchingFont: unsafe extern "system" fn(this: *mut *mut Self, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE, matchingfont: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetMatchingFonts: unsafe extern "system" fn(this: *mut *mut Self, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE, matchingfonts: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteFontFamily {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3659585775, data2: 33066, data3: 19523, data4: [152, 2, 98, 236, 74, 189, 122, 221] };
}
#[repr(C)]
pub struct IDWriteFontFamily1 {
    pub base__: IDWriteFontFamily,
    pub GetFontLocality: unsafe extern "system" fn(this: *mut *mut Self, listindex: u32) -> DWRITE_LOCALITY,
    pub GetFont2: unsafe extern "system" fn(this: *mut *mut Self, listindex: u32, font: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFontFaceReference: unsafe extern "system" fn(this: *mut *mut Self, listindex: u32, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteFontFamily1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3659585775, data2: 33066, data3: 19523, data4: [152, 2, 98, 236, 74, 189, 122, 223] };
}
#[repr(C)]
pub struct IDWriteFontFamily2 {
    pub base__: IDWriteFontFamily1,
    pub GetMatchingFonts2: unsafe extern "system" fn(this: *mut *mut Self, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, matchingfonts: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFontSet: unsafe extern "system" fn(this: *mut *mut Self, fontset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteFontFamily2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1054121591, data2: 41880, data3: 16993, data4: [185, 207, 193, 38, 194, 19, 30, 243] };
}
#[repr(C)]
pub struct IDWriteFontFile {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetReferenceKey: unsafe extern "system" fn(this: *mut *mut Self, fontfilereferencekey: *mut *mut ::core::ffi::c_void, fontfilereferencekeysize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetLoader: unsafe extern "system" fn(this: *mut *mut Self, fontfileloader: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Analyze: unsafe extern "system" fn(this: *mut *mut Self, issupportedfonttype: *mut super::super::Foundation::BOOL, fontfiletype: *mut DWRITE_FONT_FILE_TYPE, fontfacetype: *mut DWRITE_FONT_FACE_TYPE, numberoffaces: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Analyze: usize,
}
impl ::windows_sys::core::Interface for IDWriteFontFile {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1939703914, data2: 52981, data3: 18396, data4: [135, 105, 26, 139, 65, 190, 187, 176] };
}
#[repr(C)]
pub struct IDWriteFontFileEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, hascurrentfile: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    pub GetCurrentFontFile: unsafe extern "system" fn(this: *mut *mut Self, fontfile: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteFontFileEnumerator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1920290889, data2: 24567, data3: 17245, data4: [131, 72, 75, 233, 124, 250, 108, 124] };
}
#[repr(C)]
pub struct IDWriteFontFileLoader {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateStreamFromKey: unsafe extern "system" fn(this: *mut *mut Self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfilestream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteFontFileLoader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1920773454, data2: 54959, data3: 19614, data4: [138, 8, 214, 149, 177, 28, 170, 73] };
}
#[repr(C)]
pub struct IDWriteFontFileStream {
    pub base__: ::windows_sys::core::IUnknown,
    pub ReadFileFragment: unsafe extern "system" fn(this: *mut *mut Self, fragmentstart: *mut *mut ::core::ffi::c_void, fileoffset: u64, fragmentsize: u64, fragmentcontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ReleaseFileFragment: unsafe extern "system" fn(this: *mut *mut Self, fragmentcontext: *mut ::core::ffi::c_void),
    pub GetFileSize: unsafe extern "system" fn(this: *mut *mut Self, filesize: *mut u64) -> ::windows_sys::core::HRESULT,
    pub GetLastWriteTime: unsafe extern "system" fn(this: *mut *mut Self, lastwritetime: *mut u64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteFontFileStream {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1833461246, data2: 2744, data3: 19857, data4: [143, 98, 93, 214, 190, 52, 163, 224] };
}
#[repr(C)]
pub struct IDWriteFontList {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetFontCollection: unsafe extern "system" fn(this: *mut *mut Self, fontcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFontCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetFont: unsafe extern "system" fn(this: *mut *mut Self, index: u32, font: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteFontList {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 437093432, data2: 7575, data3: 20161, data4: [174, 249, 162, 251, 134, 237, 106, 203] };
}
#[repr(C)]
pub struct IDWriteFontList1 {
    pub base__: IDWriteFontList,
    pub GetFontLocality: unsafe extern "system" fn(this: *mut *mut Self, listindex: u32) -> DWRITE_LOCALITY,
    pub GetFont2: unsafe extern "system" fn(this: *mut *mut Self, listindex: u32, font: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFontFaceReference: unsafe extern "system" fn(this: *mut *mut Self, listindex: u32, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteFontList1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3659585775, data2: 33066, data3: 19523, data4: [152, 2, 98, 236, 74, 189, 122, 222] };
}
#[repr(C)]
pub struct IDWriteFontList2 {
    pub base__: IDWriteFontList1,
    pub GetFontSet: unsafe extern "system" fn(this: *mut *mut Self, fontset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteFontList2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3228973620, data2: 30639, data3: 17498, data4: [183, 53, 8, 195, 123, 10, 91, 245] };
}
#[repr(C)]
pub struct IDWriteFontResource {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetFontFile: unsafe extern "system" fn(this: *mut *mut Self, fontfile: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFontFaceIndex: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetFontAxisCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetDefaultFontAxisValues: unsafe extern "system" fn(this: *mut *mut Self, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows_sys::core::HRESULT,
    pub GetFontAxisRanges: unsafe extern "system" fn(this: *mut *mut Self, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32) -> ::windows_sys::core::HRESULT,
    pub GetFontAxisAttributes: unsafe extern "system" fn(this: *mut *mut Self, axisindex: u32) -> DWRITE_FONT_AXIS_ATTRIBUTES,
    pub GetAxisNames: unsafe extern "system" fn(this: *mut *mut Self, axisindex: u32, names: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAxisValueNameCount: unsafe extern "system" fn(this: *mut *mut Self, axisindex: u32) -> u32,
    pub GetAxisValueNames: unsafe extern "system" fn(this: *mut *mut Self, axisindex: u32, axisvalueindex: u32, fontaxisrange: *mut DWRITE_FONT_AXIS_RANGE, names: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HasVariations: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasVariations: usize,
    pub CreateFontFace: unsafe extern "system" fn(this: *mut *mut Self, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFontFaceReference: unsafe extern "system" fn(this: *mut *mut Self, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteFontResource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 528497270, data2: 26737, data3: 18664, data4: [152, 127, 185, 117, 85, 28, 80, 242] };
}
#[repr(C)]
pub struct IDWriteFontSet {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetFontCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetFontFaceReference: unsafe extern "system" fn(this: *mut *mut Self, listindex: u32, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FindFontFaceReference: unsafe extern "system" fn(this: *mut *mut Self, fontfacereference: *mut ::core::ffi::c_void, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindFontFaceReference: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FindFontFace: unsafe extern "system" fn(this: *mut *mut Self, fontface: *mut ::core::ffi::c_void, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindFontFace: usize,
    pub GetPropertyValues: unsafe extern "system" fn(this: *mut *mut Self, propertyid: DWRITE_FONT_PROPERTY_ID, values: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPropertyValues2: unsafe extern "system" fn(this: *mut *mut Self, propertyid: DWRITE_FONT_PROPERTY_ID, preferredlocalenames: ::windows_sys::core::PCWSTR, values: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPropertyValues3: unsafe extern "system" fn(this: *mut *mut Self, listindex: u32, propertyid: DWRITE_FONT_PROPERTY_ID, exists: *mut super::super::Foundation::BOOL, values: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPropertyValues3: usize,
    pub GetPropertyOccurrenceCount: unsafe extern "system" fn(this: *mut *mut Self, property: *const DWRITE_FONT_PROPERTY, propertyoccurrencecount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetMatchingFonts: unsafe extern "system" fn(this: *mut *mut Self, familyname: ::windows_sys::core::PCWSTR, fontweight: DWRITE_FONT_WEIGHT, fontstretch: DWRITE_FONT_STRETCH, fontstyle: DWRITE_FONT_STYLE, filteredset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetMatchingFonts2: unsafe extern "system" fn(this: *mut *mut Self, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, filteredset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteFontSet {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1398296897, data2: 55800, data3: 16533, data4: [131, 33, 215, 60, 246, 189, 17, 107] };
}
#[repr(C)]
pub struct IDWriteFontSet1 {
    pub base__: IDWriteFontSet,
    pub GetMatchingFonts3: unsafe extern "system" fn(this: *mut *mut Self, fontproperty: *const DWRITE_FONT_PROPERTY, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, matchingfonts: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFirstFontResources: unsafe extern "system" fn(this: *mut *mut Self, filteredfontset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFilteredFonts: unsafe extern "system" fn(this: *mut *mut Self, indices: *const u32, indexcount: u32, filteredfontset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFilteredFonts2: unsafe extern "system" fn(this: *mut *mut Self, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, selectanyrange: super::super::Foundation::BOOL, filteredfontset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFilteredFonts2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFilteredFonts3: unsafe extern "system" fn(this: *mut *mut Self, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, selectanyproperty: super::super::Foundation::BOOL, filteredfontset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFilteredFonts3: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFilteredFontIndices: unsafe extern "system" fn(this: *mut *mut Self, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, selectanyrange: super::super::Foundation::BOOL, indices: *mut u32, maxindexcount: u32, actualindexcount: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFilteredFontIndices: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFilteredFontIndices2: unsafe extern "system" fn(this: *mut *mut Self, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, selectanyproperty: super::super::Foundation::BOOL, indices: *mut u32, maxindexcount: u32, actualindexcount: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFilteredFontIndices2: usize,
    pub GetFontAxisRanges: unsafe extern "system" fn(this: *mut *mut Self, listindex: u32, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, maxfontaxisrangecount: u32, actualfontaxisrangecount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetFontAxisRanges2: unsafe extern "system" fn(this: *mut *mut Self, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, maxfontaxisrangecount: u32, actualfontaxisrangecount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetFontFaceReference2: unsafe extern "system" fn(this: *mut *mut Self, listindex: u32, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFontResource: unsafe extern "system" fn(this: *mut *mut Self, listindex: u32, fontresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFontFace: unsafe extern "system" fn(this: *mut *mut Self, listindex: u32, fontface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFontLocality: unsafe extern "system" fn(this: *mut *mut Self, listindex: u32) -> DWRITE_LOCALITY,
}
impl ::windows_sys::core::Interface for IDWriteFontSet1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2124405381, data2: 27794, data3: 16467, data4: [188, 71, 122, 227, 83, 13, 180, 211] };
}
#[repr(C)]
pub struct IDWriteFontSet2 {
    pub base__: IDWriteFontSet1,
    #[cfg(feature = "Win32_Foundation")]
    pub GetExpirationEvent: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::HANDLE,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetExpirationEvent: usize,
}
impl ::windows_sys::core::Interface for IDWriteFontSet2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3699289369, data2: 58700, data3: 17327, data4: [178, 218, 78, 43, 121, 186, 63, 127] };
}
#[repr(C)]
pub struct IDWriteFontSet3 {
    pub base__: IDWriteFontSet2,
    pub GetFontSourceType: unsafe extern "system" fn(this: *mut *mut Self, fontindex: u32) -> DWRITE_FONT_SOURCE_TYPE,
    pub GetFontSourceNameLength: unsafe extern "system" fn(this: *mut *mut Self, listindex: u32) -> u32,
    pub GetFontSourceName: unsafe extern "system" fn(this: *mut *mut Self, listindex: u32, stringbuffer: ::windows_sys::core::PWSTR, stringbuffersize: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteFontSet3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2080849650, data2: 42996, data3: 16453, data4: [140, 50, 138, 184, 174, 100, 15, 144] };
}
#[repr(C)]
pub struct IDWriteFontSetBuilder {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddFontFaceReference: unsafe extern "system" fn(this: *mut *mut Self, fontfacereference: *mut ::core::ffi::c_void, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32) -> ::windows_sys::core::HRESULT,
    pub AddFontFaceReference2: unsafe extern "system" fn(this: *mut *mut Self, fontfacereference: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddFontSet: unsafe extern "system" fn(this: *mut *mut Self, fontset: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFontSet: unsafe extern "system" fn(this: *mut *mut Self, fontset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteFontSetBuilder {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 795093758, data2: 40040, data3: 20288, data4: [184, 190, 69, 116, 1, 175, 203, 61] };
}
#[repr(C)]
pub struct IDWriteFontSetBuilder1 {
    pub base__: IDWriteFontSetBuilder,
    pub AddFontFile: unsafe extern "system" fn(this: *mut *mut Self, fontfile: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteFontSetBuilder1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1073181023, data2: 15580, data3: 19910, data4: [155, 114, 236, 86, 33, 220, 202, 253] };
}
#[repr(C)]
pub struct IDWriteFontSetBuilder2 {
    pub base__: IDWriteFontSetBuilder1,
    pub AddFont: unsafe extern "system" fn(this: *mut *mut Self, fontfile: *mut ::core::ffi::c_void, fontfaceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32) -> ::windows_sys::core::HRESULT,
    pub AddFontFile2: unsafe extern "system" fn(this: *mut *mut Self, filepath: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteFontSetBuilder2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3998983698, data2: 45361, data3: 17980, data4: [143, 79, 49, 137, 185, 64, 30, 69] };
}
#[repr(C)]
pub struct IDWriteGdiInterop {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CreateFontFromLOGFONT: unsafe extern "system" fn(this: *mut *mut Self, logfont: *const super::Gdi::LOGFONTW, font: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CreateFontFromLOGFONT: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub ConvertFontToLOGFONT: unsafe extern "system" fn(this: *mut *mut Self, font: *mut ::core::ffi::c_void, logfont: *mut super::Gdi::LOGFONTW, issystemfont: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    ConvertFontToLOGFONT: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub ConvertFontFaceToLOGFONT: unsafe extern "system" fn(this: *mut *mut Self, font: *mut ::core::ffi::c_void, logfont: *mut super::Gdi::LOGFONTW) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    ConvertFontFaceToLOGFONT: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CreateFontFaceFromHdc: unsafe extern "system" fn(this: *mut *mut Self, hdc: super::Gdi::HDC, fontface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CreateFontFaceFromHdc: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CreateBitmapRenderTarget: unsafe extern "system" fn(this: *mut *mut Self, hdc: super::Gdi::HDC, width: u32, height: u32, rendertarget: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CreateBitmapRenderTarget: usize,
}
impl ::windows_sys::core::Interface for IDWriteGdiInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 517837969, data2: 38995, data3: 17049, data4: [137, 143, 100, 50, 152, 59, 111, 58] };
}
#[repr(C)]
pub struct IDWriteGdiInterop1 {
    pub base__: IDWriteGdiInterop,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CreateFontFromLOGFONT2: unsafe extern "system" fn(this: *mut *mut Self, logfont: *const super::Gdi::LOGFONTW, fontcollection: *mut ::core::ffi::c_void, font: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CreateFontFromLOGFONT2: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetFontSignature: unsafe extern "system" fn(this: *mut *mut Self, fontface: *mut ::core::ffi::c_void, fontsignature: *mut super::super::Globalization::FONTSIGNATURE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetFontSignature: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetFontSignature2: unsafe extern "system" fn(this: *mut *mut Self, font: *mut ::core::ffi::c_void, fontsignature: *mut super::super::Globalization::FONTSIGNATURE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetFontSignature2: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub GetMatchingFontsByLOGFONT: unsafe extern "system" fn(this: *mut *mut Self, logfont: *const super::Gdi::LOGFONTA, fontset: *mut ::core::ffi::c_void, filteredset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    GetMatchingFontsByLOGFONT: usize,
}
impl ::windows_sys::core::Interface for IDWriteGdiInterop1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1163312752, data2: 15037, data3: 20336, data4: [144, 190, 66, 23, 128, 166, 245, 21] };
}
#[repr(C)]
pub struct IDWriteGlyphRunAnalysis {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAlphaTextureBounds: unsafe extern "system" fn(this: *mut *mut Self, texturetype: DWRITE_TEXTURE_TYPE, texturebounds: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAlphaTextureBounds: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateAlphaTexture: unsafe extern "system" fn(this: *mut *mut Self, texturetype: DWRITE_TEXTURE_TYPE, texturebounds: *const super::super::Foundation::RECT, alphavalues: *mut u8, buffersize: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateAlphaTexture: usize,
    pub GetAlphaBlendParams: unsafe extern "system" fn(this: *mut *mut Self, renderingparams: *mut ::core::ffi::c_void, blendgamma: *mut f32, blendenhancedcontrast: *mut f32, blendcleartypelevel: *mut f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteGlyphRunAnalysis {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2107104247, data2: 57477, data3: 17108, data4: [129, 227, 106, 136, 59, 222, 209, 24] };
}
#[repr(C)]
pub struct IDWriteInMemoryFontFileLoader {
    pub base__: IDWriteFontFileLoader,
    pub CreateInMemoryFontFileReference: unsafe extern "system" fn(this: *mut *mut Self, factory: *mut ::core::ffi::c_void, fontdata: *const ::core::ffi::c_void, fontdatasize: u32, ownerobject: *mut ::core::ffi::c_void, fontfile: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFileCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
}
impl ::windows_sys::core::Interface for IDWriteInMemoryFontFileLoader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3692048199, data2: 41261, data3: 19228, data4: [130, 45, 158, 17, 126, 51, 4, 63] };
}
#[repr(C)]
pub struct IDWriteInlineObject {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Draw: unsafe extern "system" fn(this: *mut *mut Self, clientdrawingcontext: *const ::core::ffi::c_void, renderer: *mut ::core::ffi::c_void, originx: f32, originy: f32, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Draw: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMetrics: unsafe extern "system" fn(this: *mut *mut Self, metrics: *mut DWRITE_INLINE_OBJECT_METRICS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMetrics: usize,
    pub GetOverhangMetrics: unsafe extern "system" fn(this: *mut *mut Self, overhangs: *mut DWRITE_OVERHANG_METRICS) -> ::windows_sys::core::HRESULT,
    pub GetBreakConditions: unsafe extern "system" fn(this: *mut *mut Self, breakconditionbefore: *mut DWRITE_BREAK_CONDITION, breakconditionafter: *mut DWRITE_BREAK_CONDITION) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteInlineObject {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2201615843, data2: 4207, data3: 18347, data4: [131, 115, 28, 98, 149, 235, 16, 179] };
}
#[repr(C)]
pub struct IDWriteLocalFontFileLoader {
    pub base__: IDWriteFontFileLoader,
    pub GetFilePathLengthFromKey: unsafe extern "system" fn(this: *mut *mut Self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, filepathlength: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetFilePathFromKey: unsafe extern "system" fn(this: *mut *mut Self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, filepath: ::windows_sys::core::PWSTR, filepathsize: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLastWriteTimeFromKey: unsafe extern "system" fn(this: *mut *mut Self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, lastwritetime: *mut super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLastWriteTimeFromKey: usize,
}
impl ::windows_sys::core::Interface for IDWriteLocalFontFileLoader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3000628204, data2: 51710, data3: 18961, data4: [162, 236, 216, 98, 8, 247, 192, 162] };
}
#[repr(C)]
pub struct IDWriteLocalizedStrings {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub FindLocaleName: unsafe extern "system" fn(this: *mut *mut Self, localename: ::windows_sys::core::PCWSTR, index: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindLocaleName: usize,
    pub GetLocaleNameLength: unsafe extern "system" fn(this: *mut *mut Self, index: u32, length: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetLocaleName: unsafe extern "system" fn(this: *mut *mut Self, index: u32, localename: ::windows_sys::core::PWSTR, size: u32) -> ::windows_sys::core::HRESULT,
    pub GetStringLength: unsafe extern "system" fn(this: *mut *mut Self, index: u32, length: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetString: unsafe extern "system" fn(this: *mut *mut Self, index: u32, stringbuffer: ::windows_sys::core::PWSTR, size: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteLocalizedStrings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 136667657, data2: 2458, data3: 19252, data4: [184, 109, 194, 43, 17, 14, 119, 113] };
}
#[repr(C)]
pub struct IDWriteNumberSubstitution {
    pub base__: ::windows_sys::core::IUnknown,
}
impl ::windows_sys::core::Interface for IDWriteNumberSubstitution {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 344480969, data2: 47792, data3: 20368, data4: [182, 237, 92, 54, 106, 44, 208, 61] };
}
#[repr(C)]
pub struct IDWritePixelSnapping {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPixelSnappingDisabled: unsafe extern "system" fn(this: *mut *mut Self, clientdrawingcontext: *const ::core::ffi::c_void, isdisabled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPixelSnappingDisabled: usize,
    pub GetCurrentTransform: unsafe extern "system" fn(this: *mut *mut Self, clientdrawingcontext: *const ::core::ffi::c_void, transform: *mut DWRITE_MATRIX) -> ::windows_sys::core::HRESULT,
    pub GetPixelsPerDip: unsafe extern "system" fn(this: *mut *mut Self, clientdrawingcontext: *const ::core::ffi::c_void, pixelsperdip: *mut f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWritePixelSnapping {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3941835482, data2: 60660, data3: 19748, data4: [182, 68, 179, 79, 104, 66, 2, 75] };
}
#[repr(C)]
pub struct IDWriteRemoteFontFileLoader {
    pub base__: IDWriteFontFileLoader,
    pub CreateRemoteStreamFromKey: unsafe extern "system" fn(this: *mut *mut Self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfilestream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetLocalityFromKey: unsafe extern "system" fn(this: *mut *mut Self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, locality: *mut DWRITE_LOCALITY) -> ::windows_sys::core::HRESULT,
    pub CreateFontFileReferenceFromUrl: unsafe extern "system" fn(this: *mut *mut Self, factory: *mut ::core::ffi::c_void, baseurl: ::windows_sys::core::PCWSTR, fontfileurl: ::windows_sys::core::PCWSTR, fontfile: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteRemoteFontFileLoader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1751420035, data2: 28382, data3: 18112, data4: [171, 70, 32, 8, 58, 136, 127, 222] };
}
#[repr(C)]
pub struct IDWriteRemoteFontFileStream {
    pub base__: IDWriteFontFileStream,
    pub GetLocalFileSize: unsafe extern "system" fn(this: *mut *mut Self, localfilesize: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFileFragmentLocality: unsafe extern "system" fn(this: *mut *mut Self, fileoffset: u64, fragmentsize: u64, islocal: *mut super::super::Foundation::BOOL, partialsize: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFileFragmentLocality: usize,
    pub GetLocality: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_LOCALITY,
    pub BeginDownload: unsafe extern "system" fn(this: *mut *mut Self, downloadoperationid: *const ::windows_sys::core::GUID, filefragments: *const DWRITE_FILE_FRAGMENT, fragmentcount: u32, asyncresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteRemoteFontFileStream {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1303606650, data2: 11378, data3: 20185, data4: [178, 182, 26, 186, 190, 26, 255, 156] };
}
#[repr(C)]
pub struct IDWriteRenderingParams {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetGamma: unsafe extern "system" fn(this: *mut *mut Self) -> f32,
    pub GetEnhancedContrast: unsafe extern "system" fn(this: *mut *mut Self) -> f32,
    pub GetClearTypeLevel: unsafe extern "system" fn(this: *mut *mut Self) -> f32,
    pub GetPixelGeometry: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_PIXEL_GEOMETRY,
    pub GetRenderingMode: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_RENDERING_MODE,
}
impl ::windows_sys::core::Interface for IDWriteRenderingParams {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 789423418, data2: 10973, data3: 18381, data4: [130, 238, 217, 236, 52, 104, 142, 117] };
}
#[repr(C)]
pub struct IDWriteRenderingParams1 {
    pub base__: IDWriteRenderingParams,
    pub GetGrayscaleEnhancedContrast: unsafe extern "system" fn(this: *mut *mut Self) -> f32,
}
impl ::windows_sys::core::Interface for IDWriteRenderingParams1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2487303412, data2: 42748, data3: 16968, data4: [139, 80, 102, 116, 52, 143, 202, 211] };
}
#[repr(C)]
pub struct IDWriteRenderingParams2 {
    pub base__: IDWriteRenderingParams1,
    pub GetGridFitMode: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_GRID_FIT_MODE,
}
impl ::windows_sys::core::Interface for IDWriteRenderingParams2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4191621571, data2: 38775, data3: 16558, data4: [135, 232, 62, 90, 249, 191, 9, 72] };
}
#[repr(C)]
pub struct IDWriteRenderingParams3 {
    pub base__: IDWriteRenderingParams2,
    pub GetRenderingMode1: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_RENDERING_MODE1,
}
impl ::windows_sys::core::Interface for IDWriteRenderingParams3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3079818154, data2: 14619, data3: 16682, data4: [140, 92, 228, 76, 194, 216, 103, 220] };
}
#[repr(C)]
pub struct IDWriteStringList {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetLocaleNameLength: unsafe extern "system" fn(this: *mut *mut Self, listindex: u32, length: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetLocaleName: unsafe extern "system" fn(this: *mut *mut Self, listindex: u32, localename: ::windows_sys::core::PWSTR, size: u32) -> ::windows_sys::core::HRESULT,
    pub GetStringLength: unsafe extern "system" fn(this: *mut *mut Self, listindex: u32, length: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetString: unsafe extern "system" fn(this: *mut *mut Self, listindex: u32, stringbuffer: ::windows_sys::core::PWSTR, stringbuffersize: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteStringList {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3488493888, data2: 4439, data3: 18378, data4: [139, 133, 49, 191, 207, 63, 45, 14] };
}
#[repr(C)]
pub struct IDWriteTextAnalysisSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetScriptAnalysis: unsafe extern "system" fn(this: *mut *mut Self, textposition: u32, textlength: u32, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS) -> ::windows_sys::core::HRESULT,
    pub SetLineBreakpoints: unsafe extern "system" fn(this: *mut *mut Self, textposition: u32, textlength: u32, linebreakpoints: *const DWRITE_LINE_BREAKPOINT) -> ::windows_sys::core::HRESULT,
    pub SetBidiLevel: unsafe extern "system" fn(this: *mut *mut Self, textposition: u32, textlength: u32, explicitlevel: u8, resolvedlevel: u8) -> ::windows_sys::core::HRESULT,
    pub SetNumberSubstitution: unsafe extern "system" fn(this: *mut *mut Self, textposition: u32, textlength: u32, numbersubstitution: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteTextAnalysisSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1477496132, data2: 3232, data3: 18177, data4: [179, 250, 190, 197, 24, 42, 228, 246] };
}
#[repr(C)]
pub struct IDWriteTextAnalysisSink1 {
    pub base__: IDWriteTextAnalysisSink,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGlyphOrientation: unsafe extern "system" fn(this: *mut *mut Self, textposition: u32, textlength: u32, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, adjustedbidilevel: u8, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGlyphOrientation: usize,
}
impl ::windows_sys::core::Interface for IDWriteTextAnalysisSink1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2967028128, data2: 34279, data3: 19851, data4: [159, 211, 92, 237, 153, 52, 72, 42] };
}
#[repr(C)]
pub struct IDWriteTextAnalysisSource {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetTextAtPosition: unsafe extern "system" fn(this: *mut *mut Self, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetTextBeforePosition: unsafe extern "system" fn(this: *mut *mut Self, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetParagraphReadingDirection: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_READING_DIRECTION,
    pub GetLocaleName: unsafe extern "system" fn(this: *mut *mut Self, textposition: u32, textlength: *mut u32, localename: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    pub GetNumberSubstitution: unsafe extern "system" fn(this: *mut *mut Self, textposition: u32, textlength: *mut u32, numbersubstitution: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteTextAnalysisSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1754143320, data2: 20628, data3: 18376, data4: [173, 200, 251, 206, 166, 10, 233, 43] };
}
#[repr(C)]
pub struct IDWriteTextAnalysisSource1 {
    pub base__: IDWriteTextAnalysisSource,
    pub GetVerticalGlyphOrientation: unsafe extern "system" fn(this: *mut *mut Self, textposition: u32, textlength: *mut u32, glyphorientation: *mut DWRITE_VERTICAL_GLYPH_ORIENTATION, bidilevel: *mut u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteTextAnalysisSource1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1671232216, data2: 4020, data3: 19233, data4: [165, 138, 6, 121, 32, 18, 0, 9] };
}
#[repr(C)]
pub struct IDWriteTextAnalyzer {
    pub base__: ::windows_sys::core::IUnknown,
    pub AnalyzeScript: unsafe extern "system" fn(this: *mut *mut Self, analysissource: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, analysissink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AnalyzeBidi: unsafe extern "system" fn(this: *mut *mut Self, analysissource: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, analysissink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AnalyzeNumberSubstitution: unsafe extern "system" fn(this: *mut *mut Self, analysissource: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, analysissink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AnalyzeLineBreakpoints: unsafe extern "system" fn(this: *mut *mut Self, analysissource: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, analysissink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetGlyphs: unsafe extern "system" fn(this: *mut *mut Self, textstring: ::windows_sys::core::PCWSTR, textlength: u32, fontface: *mut ::core::ffi::c_void, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: ::windows_sys::core::PCWSTR, numbersubstitution: *mut ::core::ffi::c_void, features: *const *const DWRITE_TYPOGRAPHIC_FEATURES, featurerangelengths: *const u32, featureranges: u32, maxglyphcount: u32, clustermap: *mut u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, glyphindices: *mut u16, glyphprops: *mut DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetGlyphs: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetGlyphPlacements: unsafe extern "system" fn(this: *mut *mut Self, textstring: ::windows_sys::core::PCWSTR, clustermap: *const u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, textlength: u32, glyphindices: *const u16, glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES, glyphcount: u32, fontface: *mut ::core::ffi::c_void, fontemsize: f32, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: ::windows_sys::core::PCWSTR, features: *const *const DWRITE_TYPOGRAPHIC_FEATURES, featurerangelengths: *const u32, featureranges: u32, glyphadvances: *mut f32, glyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetGlyphPlacements: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetGdiCompatibleGlyphPlacements: unsafe extern "system" fn(
        this: *mut *mut Self,
        textstring: ::windows_sys::core::PCWSTR,
        clustermap: *const u16,
        textprops: *const DWRITE_SHAPING_TEXT_PROPERTIES,
        textlength: u32,
        glyphindices: *const u16,
        glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES,
        glyphcount: u32,
        fontface: *mut ::core::ffi::c_void,
        fontemsize: f32,
        pixelsperdip: f32,
        transform: *const DWRITE_MATRIX,
        usegdinatural: super::super::Foundation::BOOL,
        issideways: super::super::Foundation::BOOL,
        isrighttoleft: super::super::Foundation::BOOL,
        scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS,
        localename: ::windows_sys::core::PCWSTR,
        features: *const *const DWRITE_TYPOGRAPHIC_FEATURES,
        featurerangelengths: *const u32,
        featureranges: u32,
        glyphadvances: *mut f32,
        glyphoffsets: *mut DWRITE_GLYPH_OFFSET,
    ) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetGdiCompatibleGlyphPlacements: usize,
}
impl ::windows_sys::core::Interface for IDWriteTextAnalyzer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3085309502, data2: 32582, data3: 17332, data4: [132, 179, 228, 230, 36, 156, 54, 93] };
}
#[repr(C)]
pub struct IDWriteTextAnalyzer1 {
    pub base__: IDWriteTextAnalyzer,
    pub ApplyCharacterSpacing: unsafe extern "system" fn(this: *mut *mut Self, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textlength: u32, glyphcount: u32, clustermap: *const u16, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBaseline: unsafe extern "system" fn(this: *mut *mut Self, fontface: *mut ::core::ffi::c_void, baseline: DWRITE_BASELINE, isvertical: super::super::Foundation::BOOL, issimulationallowed: super::super::Foundation::BOOL, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, localename: ::windows_sys::core::PCWSTR, baselinecoordinate: *mut i32, exists: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBaseline: usize,
    pub AnalyzeVerticalGlyphOrientation: unsafe extern "system" fn(this: *mut *mut Self, analysissource: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, analysissink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetGlyphOrientationTransform: unsafe extern "system" fn(this: *mut *mut Self, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: super::super::Foundation::BOOL, transform: *mut DWRITE_MATRIX) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetGlyphOrientationTransform: usize,
    pub GetScriptProperties: unsafe extern "system" fn(this: *mut *mut Self, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, scriptproperties: *mut DWRITE_SCRIPT_PROPERTIES) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTextComplexity: unsafe extern "system" fn(this: *mut *mut Self, textstring: ::windows_sys::core::PCWSTR, textlength: u32, fontface: *mut ::core::ffi::c_void, istextsimple: *mut super::super::Foundation::BOOL, textlengthread: *mut u32, glyphindices: *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTextComplexity: usize,
    pub GetJustificationOpportunities: unsafe extern "system" fn(this: *mut *mut Self, fontface: *mut ::core::ffi::c_void, fontemsize: f32, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, textstring: ::windows_sys::core::PCWSTR, clustermap: *const u16, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, justificationopportunities: *mut DWRITE_JUSTIFICATION_OPPORTUNITY) -> ::windows_sys::core::HRESULT,
    pub JustifyGlyphAdvances: unsafe extern "system" fn(this: *mut *mut Self, linewidth: f32, glyphcount: u32, justificationopportunities: *const DWRITE_JUSTIFICATION_OPPORTUNITY, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, justifiedglyphadvances: *mut f32, justifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows_sys::core::HRESULT,
    pub GetJustifiedGlyphs: unsafe extern "system" fn(this: *mut *mut Self, fontface: *mut ::core::ffi::c_void, fontemsize: f32, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, maxglyphcount: u32, clustermap: *const u16, glyphindices: *const u16, glyphadvances: *const f32, justifiedglyphadvances: *const f32, justifiedglyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32, modifiedclustermap: *mut u16, modifiedglyphindices: *mut u16, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteTextAnalyzer1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2161825792, data2: 57887, data3: 20099, data4: [150, 206, 191, 204, 229, 0, 219, 124] };
}
#[repr(C)]
pub struct IDWriteTextAnalyzer2 {
    pub base__: IDWriteTextAnalyzer1,
    #[cfg(feature = "Win32_Foundation")]
    pub GetGlyphOrientationTransform2: unsafe extern "system" fn(this: *mut *mut Self, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: super::super::Foundation::BOOL, originx: f32, originy: f32, transform: *mut DWRITE_MATRIX) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetGlyphOrientationTransform2: usize,
    pub GetTypographicFeatures: unsafe extern "system" fn(this: *mut *mut Self, fontface: *mut ::core::ffi::c_void, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, localename: ::windows_sys::core::PCWSTR, maxtagcount: u32, actualtagcount: *mut u32, tags: *mut DWRITE_FONT_FEATURE_TAG) -> ::windows_sys::core::HRESULT,
    pub CheckTypographicFeature: unsafe extern "system" fn(this: *mut *mut Self, fontface: *mut ::core::ffi::c_void, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, localename: ::windows_sys::core::PCWSTR, featuretag: DWRITE_FONT_FEATURE_TAG, glyphcount: u32, glyphindices: *const u16, featureapplies: *mut u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteTextAnalyzer2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1429905395, data2: 22163, data3: 19959, data4: [181, 43, 116, 128, 111, 127, 46, 185] };
}
#[repr(C)]
pub struct IDWriteTextFormat {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetTextAlignment: unsafe extern "system" fn(this: *mut *mut Self, textalignment: DWRITE_TEXT_ALIGNMENT) -> ::windows_sys::core::HRESULT,
    pub SetParagraphAlignment: unsafe extern "system" fn(this: *mut *mut Self, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> ::windows_sys::core::HRESULT,
    pub SetWordWrapping: unsafe extern "system" fn(this: *mut *mut Self, wordwrapping: DWRITE_WORD_WRAPPING) -> ::windows_sys::core::HRESULT,
    pub SetReadingDirection: unsafe extern "system" fn(this: *mut *mut Self, readingdirection: DWRITE_READING_DIRECTION) -> ::windows_sys::core::HRESULT,
    pub SetFlowDirection: unsafe extern "system" fn(this: *mut *mut Self, flowdirection: DWRITE_FLOW_DIRECTION) -> ::windows_sys::core::HRESULT,
    pub SetIncrementalTabStop: unsafe extern "system" fn(this: *mut *mut Self, incrementaltabstop: f32) -> ::windows_sys::core::HRESULT,
    pub SetTrimming: unsafe extern "system" fn(this: *mut *mut Self, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetLineSpacing: unsafe extern "system" fn(this: *mut *mut Self, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> ::windows_sys::core::HRESULT,
    pub GetTextAlignment: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_TEXT_ALIGNMENT,
    pub GetParagraphAlignment: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_PARAGRAPH_ALIGNMENT,
    pub GetWordWrapping: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_WORD_WRAPPING,
    pub GetReadingDirection: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_READING_DIRECTION,
    pub GetFlowDirection: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_FLOW_DIRECTION,
    pub GetIncrementalTabStop: unsafe extern "system" fn(this: *mut *mut Self) -> f32,
    pub GetTrimming: unsafe extern "system" fn(this: *mut *mut Self, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetLineSpacing: unsafe extern "system" fn(this: *mut *mut Self, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> ::windows_sys::core::HRESULT,
    pub GetFontCollection: unsafe extern "system" fn(this: *mut *mut Self, fontcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFontFamilyNameLength: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetFontFamilyName: unsafe extern "system" fn(this: *mut *mut Self, fontfamilyname: ::windows_sys::core::PWSTR, namesize: u32) -> ::windows_sys::core::HRESULT,
    pub GetFontWeight: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_FONT_WEIGHT,
    pub GetFontStyle: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_FONT_STYLE,
    pub GetFontStretch: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_FONT_STRETCH,
    pub GetFontSize: unsafe extern "system" fn(this: *mut *mut Self) -> f32,
    pub GetLocaleNameLength: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetLocaleName: unsafe extern "system" fn(this: *mut *mut Self, localename: ::windows_sys::core::PWSTR, namesize: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteTextFormat {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2626709528, data2: 12759, data3: 20435, data4: [161, 81, 124, 94, 34, 93, 181, 90] };
}
#[repr(C)]
pub struct IDWriteTextFormat1 {
    pub base__: IDWriteTextFormat,
    pub SetVerticalGlyphOrientation: unsafe extern "system" fn(this: *mut *mut Self, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> ::windows_sys::core::HRESULT,
    pub GetVerticalGlyphOrientation: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_VERTICAL_GLYPH_ORIENTATION,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLastLineWrapping: unsafe extern "system" fn(this: *mut *mut Self, islastlinewrappingenabled: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLastLineWrapping: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLastLineWrapping: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLastLineWrapping: usize,
    pub SetOpticalAlignment: unsafe extern "system" fn(this: *mut *mut Self, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> ::windows_sys::core::HRESULT,
    pub GetOpticalAlignment: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_OPTICAL_ALIGNMENT,
    pub SetFontFallback: unsafe extern "system" fn(this: *mut *mut Self, fontfallback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFontFallback: unsafe extern "system" fn(this: *mut *mut Self, fontfallback: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteTextFormat1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1595362121, data2: 3467, data3: 19707, data4: [139, 202, 241, 204, 233, 208, 108, 103] };
}
#[repr(C)]
pub struct IDWriteTextFormat2 {
    pub base__: IDWriteTextFormat1,
    pub SetLineSpacing2: unsafe extern "system" fn(this: *mut *mut Self, linespacingoptions: *const DWRITE_LINE_SPACING) -> ::windows_sys::core::HRESULT,
    pub GetLineSpacing2: unsafe extern "system" fn(this: *mut *mut Self, linespacingoptions: *mut DWRITE_LINE_SPACING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteTextFormat2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4135456477, data2: 40509, data3: 20172, data4: [140, 50, 65, 131, 37, 61, 254, 112] };
}
#[repr(C)]
pub struct IDWriteTextFormat3 {
    pub base__: IDWriteTextFormat2,
    pub SetFontAxisValues: unsafe extern "system" fn(this: *mut *mut Self, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows_sys::core::HRESULT,
    pub GetFontAxisValueCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetFontAxisValues: unsafe extern "system" fn(this: *mut *mut Self, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows_sys::core::HRESULT,
    pub GetAutomaticFontAxes: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_AUTOMATIC_FONT_AXES,
    pub SetAutomaticFontAxes: unsafe extern "system" fn(this: *mut *mut Self, automaticfontaxes: DWRITE_AUTOMATIC_FONT_AXES) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteTextFormat3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1832605249, data2: 58704, data3: 17165, data4: [168, 91, 183, 191, 72, 169, 52, 39] };
}
#[repr(C)]
pub struct IDWriteTextLayout {
    pub base__: IDWriteTextFormat,
    pub SetMaxWidth: unsafe extern "system" fn(this: *mut *mut Self, maxwidth: f32) -> ::windows_sys::core::HRESULT,
    pub SetMaxHeight: unsafe extern "system" fn(this: *mut *mut Self, maxheight: f32) -> ::windows_sys::core::HRESULT,
    pub SetFontCollection: unsafe extern "system" fn(this: *mut *mut Self, fontcollection: *mut ::core::ffi::c_void, textrange: DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    pub SetFontFamilyName: unsafe extern "system" fn(this: *mut *mut Self, fontfamilyname: ::windows_sys::core::PCWSTR, textrange: DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    pub SetFontWeight: unsafe extern "system" fn(this: *mut *mut Self, fontweight: DWRITE_FONT_WEIGHT, textrange: DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    pub SetFontStyle: unsafe extern "system" fn(this: *mut *mut Self, fontstyle: DWRITE_FONT_STYLE, textrange: DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    pub SetFontStretch: unsafe extern "system" fn(this: *mut *mut Self, fontstretch: DWRITE_FONT_STRETCH, textrange: DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    pub SetFontSize: unsafe extern "system" fn(this: *mut *mut Self, fontsize: f32, textrange: DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUnderline: unsafe extern "system" fn(this: *mut *mut Self, hasunderline: super::super::Foundation::BOOL, textrange: DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUnderline: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetStrikethrough: unsafe extern "system" fn(this: *mut *mut Self, hasstrikethrough: super::super::Foundation::BOOL, textrange: DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetStrikethrough: usize,
    pub SetDrawingEffect: unsafe extern "system" fn(this: *mut *mut Self, drawingeffect: *mut ::core::ffi::c_void, textrange: DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    pub SetInlineObject: unsafe extern "system" fn(this: *mut *mut Self, inlineobject: *mut ::core::ffi::c_void, textrange: DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    pub SetTypography: unsafe extern "system" fn(this: *mut *mut Self, typography: *mut ::core::ffi::c_void, textrange: DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    pub SetLocaleName: unsafe extern "system" fn(this: *mut *mut Self, localename: ::windows_sys::core::PCWSTR, textrange: DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    pub GetMaxWidth: unsafe extern "system" fn(this: *mut *mut Self) -> f32,
    pub GetMaxHeight: unsafe extern "system" fn(this: *mut *mut Self) -> f32,
    pub GetFontCollection2: unsafe extern "system" fn(this: *mut *mut Self, currentposition: u32, fontcollection: *mut *mut ::core::ffi::c_void, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    pub GetFontFamilyNameLength2: unsafe extern "system" fn(this: *mut *mut Self, currentposition: u32, namelength: *mut u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    pub GetFontFamilyName2: unsafe extern "system" fn(this: *mut *mut Self, currentposition: u32, fontfamilyname: ::windows_sys::core::PWSTR, namesize: u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    pub GetFontWeight2: unsafe extern "system" fn(this: *mut *mut Self, currentposition: u32, fontweight: *mut DWRITE_FONT_WEIGHT, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    pub GetFontStyle2: unsafe extern "system" fn(this: *mut *mut Self, currentposition: u32, fontstyle: *mut DWRITE_FONT_STYLE, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    pub GetFontStretch2: unsafe extern "system" fn(this: *mut *mut Self, currentposition: u32, fontstretch: *mut DWRITE_FONT_STRETCH, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    pub GetFontSize2: unsafe extern "system" fn(this: *mut *mut Self, currentposition: u32, fontsize: *mut f32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetUnderline: unsafe extern "system" fn(this: *mut *mut Self, currentposition: u32, hasunderline: *mut super::super::Foundation::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetUnderline: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetStrikethrough: unsafe extern "system" fn(this: *mut *mut Self, currentposition: u32, hasstrikethrough: *mut super::super::Foundation::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetStrikethrough: usize,
    pub GetDrawingEffect: unsafe extern "system" fn(this: *mut *mut Self, currentposition: u32, drawingeffect: *mut *mut ::core::ffi::c_void, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    pub GetInlineObject: unsafe extern "system" fn(this: *mut *mut Self, currentposition: u32, inlineobject: *mut *mut ::core::ffi::c_void, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    pub GetTypography: unsafe extern "system" fn(this: *mut *mut Self, currentposition: u32, typography: *mut *mut ::core::ffi::c_void, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    pub GetLocaleNameLength2: unsafe extern "system" fn(this: *mut *mut Self, currentposition: u32, namelength: *mut u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    pub GetLocaleName2: unsafe extern "system" fn(this: *mut *mut Self, currentposition: u32, localename: ::windows_sys::core::PWSTR, namesize: u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    pub Draw: unsafe extern "system" fn(this: *mut *mut Self, clientdrawingcontext: *const ::core::ffi::c_void, renderer: *mut ::core::ffi::c_void, originx: f32, originy: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLineMetrics: unsafe extern "system" fn(this: *mut *mut Self, linemetrics: *mut DWRITE_LINE_METRICS, maxlinecount: u32, actuallinecount: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLineMetrics: usize,
    pub GetMetrics: unsafe extern "system" fn(this: *mut *mut Self, textmetrics: *mut DWRITE_TEXT_METRICS) -> ::windows_sys::core::HRESULT,
    pub GetOverhangMetrics: unsafe extern "system" fn(this: *mut *mut Self, overhangs: *mut DWRITE_OVERHANG_METRICS) -> ::windows_sys::core::HRESULT,
    pub GetClusterMetrics: unsafe extern "system" fn(this: *mut *mut Self, clustermetrics: *mut DWRITE_CLUSTER_METRICS, maxclustercount: u32, actualclustercount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub DetermineMinWidth: unsafe extern "system" fn(this: *mut *mut Self, minwidth: *mut f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HitTestPoint: unsafe extern "system" fn(this: *mut *mut Self, pointx: f32, pointy: f32, istrailinghit: *mut super::super::Foundation::BOOL, isinside: *mut super::super::Foundation::BOOL, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HitTestPoint: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub HitTestTextPosition: unsafe extern "system" fn(this: *mut *mut Self, textposition: u32, istrailinghit: super::super::Foundation::BOOL, pointx: *mut f32, pointy: *mut f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HitTestTextPosition: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub HitTestTextRange: unsafe extern "system" fn(this: *mut *mut Self, textposition: u32, textlength: u32, originx: f32, originy: f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS, maxhittestmetricscount: u32, actualhittestmetricscount: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HitTestTextRange: usize,
}
impl ::windows_sys::core::Interface for IDWriteTextLayout {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1400074295, data2: 27924, data3: 16651, data4: [155, 254, 11, 24, 43, 183, 9, 97] };
}
#[repr(C)]
pub struct IDWriteTextLayout1 {
    pub base__: IDWriteTextLayout,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPairKerning: unsafe extern "system" fn(this: *mut *mut Self, ispairkerningenabled: super::super::Foundation::BOOL, textrange: DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPairKerning: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPairKerning: unsafe extern "system" fn(this: *mut *mut Self, currentposition: u32, ispairkerningenabled: *mut super::super::Foundation::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPairKerning: usize,
    pub SetCharacterSpacing: unsafe extern "system" fn(this: *mut *mut Self, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textrange: DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    pub GetCharacterSpacing: unsafe extern "system" fn(this: *mut *mut Self, currentposition: u32, leadingspacing: *mut f32, trailingspacing: *mut f32, minimumadvancewidth: *mut f32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteTextLayout1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2422528034, data2: 32935, data3: 18012, data4: [169, 134, 223, 101, 247, 139, 143, 235] };
}
#[repr(C)]
pub struct IDWriteTextLayout2 {
    pub base__: IDWriteTextLayout1,
    pub GetMetrics2: unsafe extern "system" fn(this: *mut *mut Self, textmetrics: *mut DWRITE_TEXT_METRICS1) -> ::windows_sys::core::HRESULT,
    pub SetVerticalGlyphOrientation: unsafe extern "system" fn(this: *mut *mut Self, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> ::windows_sys::core::HRESULT,
    pub GetVerticalGlyphOrientation: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_VERTICAL_GLYPH_ORIENTATION,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLastLineWrapping: unsafe extern "system" fn(this: *mut *mut Self, islastlinewrappingenabled: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLastLineWrapping: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLastLineWrapping: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLastLineWrapping: usize,
    pub SetOpticalAlignment: unsafe extern "system" fn(this: *mut *mut Self, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> ::windows_sys::core::HRESULT,
    pub GetOpticalAlignment: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_OPTICAL_ALIGNMENT,
    pub SetFontFallback: unsafe extern "system" fn(this: *mut *mut Self, fontfallback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFontFallback: unsafe extern "system" fn(this: *mut *mut Self, fontfallback: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteTextLayout2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 278118799, data2: 36190, data3: 17392, data4: [176, 100, 9, 23, 49, 27, 82, 94] };
}
#[repr(C)]
pub struct IDWriteTextLayout3 {
    pub base__: IDWriteTextLayout2,
    pub InvalidateLayout: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetLineSpacing2: unsafe extern "system" fn(this: *mut *mut Self, linespacingoptions: *const DWRITE_LINE_SPACING) -> ::windows_sys::core::HRESULT,
    pub GetLineSpacing2: unsafe extern "system" fn(this: *mut *mut Self, linespacingoptions: *mut DWRITE_LINE_SPACING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLineMetrics2: unsafe extern "system" fn(this: *mut *mut Self, linemetrics: *mut DWRITE_LINE_METRICS1, maxlinecount: u32, actuallinecount: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLineMetrics2: usize,
}
impl ::windows_sys::core::Interface for IDWriteTextLayout3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 131976530, data2: 526, data3: 19944, data4: [172, 51, 108, 149, 61, 131, 249, 45] };
}
#[repr(C)]
pub struct IDWriteTextLayout4 {
    pub base__: IDWriteTextLayout3,
    pub SetFontAxisValues: unsafe extern "system" fn(this: *mut *mut Self, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, textrange: DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    pub GetFontAxisValueCount: unsafe extern "system" fn(this: *mut *mut Self, currentposition: u32) -> u32,
    pub GetFontAxisValues: unsafe extern "system" fn(this: *mut *mut Self, currentposition: u32, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_sys::core::HRESULT,
    pub GetAutomaticFontAxes: unsafe extern "system" fn(this: *mut *mut Self) -> DWRITE_AUTOMATIC_FONT_AXES,
    pub SetAutomaticFontAxes: unsafe extern "system" fn(this: *mut *mut Self, automaticfontaxes: DWRITE_AUTOMATIC_FONT_AXES) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteTextLayout4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 95010626, data2: 8767, data3: 17473, data4: [181, 251, 130, 99, 104, 95, 85, 233] };
}
#[repr(C)]
pub struct IDWriteTextRenderer {
    pub base__: IDWritePixelSnapping,
    #[cfg(feature = "Win32_Foundation")]
    pub DrawGlyphRun: unsafe extern "system" fn(this: *mut *mut Self, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DrawGlyphRun: usize,
    pub DrawUnderline: unsafe extern "system" fn(this: *mut *mut Self, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, underline: *const DWRITE_UNDERLINE, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DrawStrikethrough: unsafe extern "system" fn(this: *mut *mut Self, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, strikethrough: *const DWRITE_STRIKETHROUGH, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DrawInlineObject: unsafe extern "system" fn(this: *mut *mut Self, clientdrawingcontext: *const ::core::ffi::c_void, originx: f32, originy: f32, inlineobject: *mut ::core::ffi::c_void, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DrawInlineObject: usize,
}
impl ::windows_sys::core::Interface for IDWriteTextRenderer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4018831669, data2: 23750, data3: 17918, data4: [136, 37, 197, 160, 114, 78, 184, 25] };
}
#[repr(C)]
pub struct IDWriteTextRenderer1 {
    pub base__: IDWriteTextRenderer,
    #[cfg(feature = "Win32_Foundation")]
    pub DrawGlyphRun2: unsafe extern "system" fn(this: *mut *mut Self, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DrawGlyphRun2: usize,
    pub DrawUnderline2: unsafe extern "system" fn(this: *mut *mut Self, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, underline: *const DWRITE_UNDERLINE, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DrawStrikethrough2: unsafe extern "system" fn(this: *mut *mut Self, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, strikethrough: *const DWRITE_STRIKETHROUGH, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DrawInlineObject2: unsafe extern "system" fn(this: *mut *mut Self, clientdrawingcontext: *const ::core::ffi::c_void, originx: f32, originy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, inlineobject: *mut ::core::ffi::c_void, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DrawInlineObject2: usize,
}
impl ::windows_sys::core::Interface for IDWriteTextRenderer1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3554732340, data2: 8864, data3: 17022, data4: [170, 228, 125, 149, 116, 181, 157, 177] };
}
#[repr(C)]
pub struct IDWriteTypography {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddFontFeature: unsafe extern "system" fn(this: *mut *mut Self, fontfeature: DWRITE_FONT_FEATURE) -> ::windows_sys::core::HRESULT,
    pub GetFontFeatureCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetFontFeature: unsafe extern "system" fn(this: *mut *mut Self, fontfeatureindex: u32, fontfeature: *mut DWRITE_FONT_FEATURE) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDWriteTypography {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1441861931, data2: 7618, data3: 19260, data4: [149, 65, 244, 104, 148, 237, 133, 182] };
}
