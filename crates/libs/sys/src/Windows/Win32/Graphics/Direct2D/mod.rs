#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub mod Common;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn D2D1ComputeMaximumScaleFactor(matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> f32;
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub fn D2D1ConvertColorSpace(sourcecolorspace: D2D1_COLOR_SPACE, destinationcolorspace: D2D1_COLOR_SPACE, color: *const Common::D2D1_COLOR_F) -> Common::D2D1_COLOR_F;
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub fn D2D1CreateDevice(dxgidevice: *mut *mut super::Dxgi::IDXGIDevice, creationproperties: *const D2D1_CREATION_PROPERTIES, d2ddevice: *mut *mut *mut ID2D1Device) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub fn D2D1CreateDeviceContext(dxgisurface: *mut *mut super::Dxgi::IDXGISurface, creationproperties: *const D2D1_CREATION_PROPERTIES, d2ddevicecontext: *mut *mut *mut ID2D1DeviceContext) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
    pub fn D2D1CreateFactory(factorytype: D2D1_FACTORY_TYPE, riid: *const ::windows_sys::core::GUID, pfactoryoptions: *const D2D1_FACTORY_OPTIONS, ppifactory: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub fn D2D1GetGradientMeshInteriorPointsFromCoonsPatch(ppoint0: *const Common::D2D_POINT_2F, ppoint1: *const Common::D2D_POINT_2F, ppoint2: *const Common::D2D_POINT_2F, ppoint3: *const Common::D2D_POINT_2F, ppoint4: *const Common::D2D_POINT_2F, ppoint5: *const Common::D2D_POINT_2F, ppoint6: *const Common::D2D_POINT_2F, ppoint7: *const Common::D2D_POINT_2F, ppoint8: *const Common::D2D_POINT_2F, ppoint9: *const Common::D2D_POINT_2F, ppoint10: *const Common::D2D_POINT_2F, ppoint11: *const Common::D2D_POINT_2F, ptensorpoint11: *mut Common::D2D_POINT_2F, ptensorpoint12: *mut Common::D2D_POINT_2F, ptensorpoint21: *mut Common::D2D_POINT_2F, ptensorpoint22: *mut Common::D2D_POINT_2F);
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation"))]
    pub fn D2D1InvertMatrix(matrix: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation"))]
    pub fn D2D1IsMatrixInvertible(matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub fn D2D1MakeRotateMatrix(angle: f32, center: Common::D2D_POINT_2F, matrix: *mut super::super::super::Foundation::Numerics::Matrix3x2);
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub fn D2D1MakeSkewMatrix(anglex: f32, angley: f32, center: Common::D2D_POINT_2F, matrix: *mut super::super::super::Foundation::Numerics::Matrix3x2);
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
    pub fn D2D1SinCos(angle: f32, s: *mut f32, c: *mut f32);
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
    pub fn D2D1Tan(angle: f32) -> f32;
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
    pub fn D2D1Vec3Length(x: f32, y: f32, z: f32) -> f32;
}
pub const CLSID_D2D12DAffineTransform: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1789490309, data2: 25428, data3: 19708, data4: [144, 140, 228, 167, 79, 98, 201, 108] };
pub const CLSID_D2D13DPerspectiveTransform: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3263450379, data2: 15750, data3: 18151, data4: [133, 186, 82, 108, 146, 64, 243, 251] };
pub const CLSID_D2D13DTransform: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3896933124, data2: 60513, data3: 19338, data4: [181, 222, 212, 215, 61, 235, 234, 90] };
pub const CLSID_D2D1AlphaMask: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3356413936, data2: 16341, data3: 20229, data4: [131, 40, 197, 209, 114, 75, 79, 10] };
pub const CLSID_D2D1ArithmeticComposite: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4229239863, data2: 1178, data3: 18308, data4: [162, 74, 241, 196, 218, 242, 9, 135] };
pub const CLSID_D2D1Atlas: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2436770788, data2: 64975, data3: 20450, data4: [165, 240, 36, 84, 241, 79, 244, 8] };
pub const CLSID_D2D1BitmapSource: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1605812813, data2: 50909, data3: 16945, data4: [148, 4, 80, 244, 213, 195, 37, 45] };
pub const CLSID_D2D1Blend: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2177218427, data2: 5112, data3: 19677, data4: [173, 32, 200, 144, 84, 122, 198, 93] };
pub const CLSID_D2D1Border: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 707611072, data2: 19151, data3: 17351, data4: [140, 106, 124, 74, 39, 135, 77, 39] };
pub const CLSID_D2D1Brightness: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2364181790, data2: 30640, data3: 18822, data4: [179, 185, 47, 12, 14, 174, 120, 135] };
pub const CLSID_D2D1ChromaKey: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1958747995, data2: 10765, data3: 16524, data4: [136, 226, 199, 163, 199, 25, 119, 66] };
pub const CLSID_D2D1ColorManagement: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 438850124, data2: 64982, data3: 19108, data4: [174, 143, 131, 126, 184, 38, 123, 55] };
pub const CLSID_D2D1ColorMatrix: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2451506134, data2: 25628, data3: 18399, data4: [133, 45, 180, 187, 97, 83, 174, 17] };
pub const CLSID_D2D1Composite: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1224515409, data2: 63148, data3: 18673, data4: [139, 88, 59, 40, 172, 70, 247, 109] };
pub const CLSID_D2D1Contrast: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3058214794, data2: 3797, data3: 20352, data4: [169, 74, 142, 130, 90, 202, 107, 119] };
pub const CLSID_D2D1ConvolveMatrix: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1082100744, data2: 21811, data3: 17201, data4: [163, 65, 35, 204, 56, 119, 132, 62] };
pub const CLSID_D2D1Crop: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3795808528, data2: 3738, data3: 17188, data4: [175, 71, 106, 44, 12, 70, 243, 91] };
pub const CLSID_D2D1CrossFade: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 318076392, data2: 19889, data3: 18527, data4: [154, 132, 3, 160, 125, 211, 130, 159] };
pub const CLSID_D2D1DirectionalBlur: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 390273446, data2: 22761, data3: 18866, data4: [187, 99, 202, 242, 200, 17, 163, 219] };
pub const CLSID_D2D1DiscreteTransfer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2424729549, data2: 18574, data3: 17739, data4: [175, 6, 229, 4, 27, 102, 195, 108] };
pub const CLSID_D2D1DisplacementMap: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3989078884, data2: 1047, data3: 16657, data4: [148, 80, 67, 132, 95, 169, 248, 144] };
pub const CLSID_D2D1DistantDiffuse: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1048509794, data2: 41773, data3: 18132, data4: [168, 60, 82, 120, 136, 154, 201, 84] };
pub const CLSID_D2D1DistantSpecular: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1116479205, data2: 30648, data3: 17488, data4: [138, 181, 114, 33, 156, 33, 171, 218] };
pub const CLSID_D2D1DpiCompensation: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1814480327, data2: 13536, data3: 18172, data4: [156, 253, 229, 130, 55, 6, 226, 40] };
pub const CLSID_D2D1EdgeDetection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4025844682, data2: 51975, data3: 19113, data4: [172, 93, 44, 196, 76, 118, 70, 15] };
pub const CLSID_D2D1Emboss: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2982538027, data2: 840, data3: 17392, data4: [129, 7, 73, 87, 202, 203, 162, 174] };
pub const CLSID_D2D1Exposure: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3043790074, data2: 63028, data3: 16878, data4: [190, 224, 255, 166, 23, 16, 96, 4] };
pub const CLSID_D2D1Flood: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1640119328, data2: 44649, data3: 19854, data4: [148, 207, 80, 7, 141, 246, 56, 242] };
pub const CLSID_D2D1GammaTransfer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1083458756, data2: 50201, data3: 16800, data4: [176, 193, 140, 208, 192, 161, 142, 66] };
pub const CLSID_D2D1GaussianBlur: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 535522665, data2: 12262, data3: 19145, data4: [140, 88, 29, 127, 147, 231, 166, 165] };
pub const CLSID_D2D1Grayscale: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 920510699, data2: 14117, data3: 17120, data4: [131, 109, 82, 251, 32, 174, 230, 68] };
pub const CLSID_D2D1HdrToneMap: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2064348301, data2: 17936, data3: 17542, data4: [169, 12, 153, 157, 154, 46, 43, 17] };
pub const CLSID_D2D1HighlightsShadows: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3403449220, data2: 12863, data3: 19582, data4: [163, 97, 46, 43, 36, 223, 110, 228] };
pub const CLSID_D2D1Histogram: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2283648976, data2: 63470, data3: 19789, data4: [166, 210, 70, 151, 172, 198, 110, 232] };
pub const CLSID_D2D1HueRotation: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 256137452, data2: 19250, data3: 18715, data4: [158, 133, 189, 115, 244, 77, 62, 182] };
pub const CLSID_D2D1HueToRgb: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2071504573, data2: 321, data3: 19951, data4: [138, 82, 99, 86, 238, 12, 189, 213] };
pub const CLSID_D2D1Invert: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3770906701, data2: 52025, data3: 20100, data4: [182, 253, 107, 114, 240, 129, 2, 99] };
pub const CLSID_D2D1LinearTransfer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2907162877, data2: 25583, data3: 19148, data4: [155, 81, 103, 151, 156, 3, 108, 6] };
pub const CLSID_D2D1LookupTable3D: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 882773722, data2: 136, data3: 19065, data4: [156, 163, 199, 227, 0, 32, 32, 32] };
pub const CLSID_D2D1LuminanceToAlpha: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1092950711, data2: 3051, data3: 18168, data4: [157, 167, 89, 233, 63, 204, 229, 222] };
pub const CLSID_D2D1Morphology: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3940992013, data2: 25194, data3: 19501, data4: [191, 203, 57, 16, 1, 171, 226, 2] };
pub const CLSID_D2D1Opacity: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2166192548, data2: 56872, data3: 17492, data4: [128, 148, 198, 70, 133, 248, 189, 76] };
pub const CLSID_D2D1OpacityMetadata: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1817378922, data2: 17488, data3: 16793, data4: [170, 91, 173, 22, 86, 254, 206, 94] };
pub const CLSID_D2D1PointDiffuse: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3118662595, data2: 49292, data3: 20369, data4: [139, 123, 56, 101, 107, 196, 140, 32] };
pub const CLSID_D2D1PointSpecular: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 163826214, data2: 15074, data3: 20233, data4: [158, 188, 237, 56, 101, 213, 63, 34] };
pub const CLSID_D2D1Posterize: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 562599006, data2: 13219, data3: 17254, data4: [183, 188, 8, 107, 208, 45, 8, 132] };
pub const CLSID_D2D1Premultiply: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 116044825, data2: 57069, data3: 16408, data4: [128, 210, 62, 29, 71, 26, 222, 178] };
pub const CLSID_D2D1RgbToHue: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 603186668, data2: 37352, data3: 19773, data4: [173, 10, 175, 173, 193, 0, 74, 161] };
pub const CLSID_D2D1Saturation: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1555225039, data2: 12925, data3: 17823, data4: [160, 206, 64, 192, 178, 8, 107, 247] };
pub const CLSID_D2D1Scale: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2645529449, data2: 14406, data3: 19726, data4: [164, 78, 12, 96, 121, 52, 165, 215] };
pub const CLSID_D2D1Sepia: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 974844944, data2: 24349, data3: 19902, data4: [132, 223, 145, 93, 167, 155, 113, 83] };
pub const CLSID_D2D1Shadow: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3330188129, data2: 6243, data3: 20073, data4: [137, 219, 105, 93, 62, 154, 91, 107] };
pub const CLSID_D2D1Sharpen: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3384313803, data2: 50687, data3: 19909, data4: [151, 121, 39, 61, 207, 65, 124, 125] };
pub const CLSID_D2D1SpotDiffuse: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2173309189, data2: 31026, data3: 17652, data4: [170, 134, 8, 174, 123, 47, 44, 147] };
pub const CLSID_D2D1SpotSpecular: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3987620382, data2: 30292, data3: 18999, data4: [157, 184, 113, 172, 193, 190, 179, 193] };
pub const CLSID_D2D1Straighten: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1302625042, data2: 31139, data3: 20400, data4: [130, 55, 187, 195, 178, 164, 222, 8] };
pub const CLSID_D2D1TableTransfer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1542985923, data2: 24131, data3: 18635, data4: [182, 49, 134, 131, 150, 214, 161, 212] };
pub const CLSID_D2D1TemperatureTint: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2300010631, data2: 35577, data3: 18952, data4: [174, 177, 137, 95, 56, 219, 23, 102] };
pub const CLSID_D2D1Tile: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2960671032, data2: 15222, data3: 19397, data4: [177, 59, 15, 162, 173, 2, 101, 159] };
pub const CLSID_D2D1Tint: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 909191959, data2: 63453, data3: 16404, data4: [145, 93, 255, 202, 118, 140, 242, 17] };
pub const CLSID_D2D1Turbulence: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3475748526, data2: 34970, data3: 19159, data4: [186, 41, 162, 253, 115, 44, 159, 201] };
pub const CLSID_D2D1UnPremultiply: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4221224073, data2: 44429, data3: 16877, data4: [153, 153, 187, 99, 71, 209, 16, 247] };
pub const CLSID_D2D1Vignette: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3222028478, data2: 24167, data3: 19619, data4: [149, 180, 244, 176, 44, 17, 81, 53] };
pub const CLSID_D2D1WhiteLevelAdjustment: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1151453915, data2: 27869, data3: 18456, data4: [143, 244, 38, 193, 207, 233, 91, 219] };
pub const CLSID_D2D1YCbCr: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2572172481, data2: 26311, data3: 17865, data4: [168, 117, 138, 216, 167, 145, 68, 1] };
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_2DAFFINETRANSFORM_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_2DAFFINETRANSFORM_PROP_INTERPOLATION_MODE: D2D1_2DAFFINETRANSFORM_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_2DAFFINETRANSFORM_PROP_BORDER_MODE: D2D1_2DAFFINETRANSFORM_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_2DAFFINETRANSFORM_PROP_TRANSFORM_MATRIX: D2D1_2DAFFINETRANSFORM_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_2DAFFINETRANSFORM_PROP_SHARPNESS: D2D1_2DAFFINETRANSFORM_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_2DAFFINETRANSFORM_PROP_FORCE_DWORD: D2D1_2DAFFINETRANSFORM_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_LINEAR: D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_CUBIC: D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR: D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_ANISOTROPIC: D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_FORCE_DWORD: D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_3DPERSPECTIVETRANSFORM_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_INTERPOLATION_MODE: D2D1_3DPERSPECTIVETRANSFORM_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_BORDER_MODE: D2D1_3DPERSPECTIVETRANSFORM_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_DEPTH: D2D1_3DPERSPECTIVETRANSFORM_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_PERSPECTIVE_ORIGIN: D2D1_3DPERSPECTIVETRANSFORM_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_LOCAL_OFFSET: D2D1_3DPERSPECTIVETRANSFORM_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_GLOBAL_OFFSET: D2D1_3DPERSPECTIVETRANSFORM_PROP = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_ROTATION_ORIGIN: D2D1_3DPERSPECTIVETRANSFORM_PROP = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_ROTATION: D2D1_3DPERSPECTIVETRANSFORM_PROP = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_FORCE_DWORD: D2D1_3DPERSPECTIVETRANSFORM_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_3DTRANSFORM_INTERPOLATION_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DTRANSFORM_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_3DTRANSFORM_INTERPOLATION_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DTRANSFORM_INTERPOLATION_MODE_LINEAR: D2D1_3DTRANSFORM_INTERPOLATION_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DTRANSFORM_INTERPOLATION_MODE_CUBIC: D2D1_3DTRANSFORM_INTERPOLATION_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DTRANSFORM_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR: D2D1_3DTRANSFORM_INTERPOLATION_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DTRANSFORM_INTERPOLATION_MODE_ANISOTROPIC: D2D1_3DTRANSFORM_INTERPOLATION_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DTRANSFORM_INTERPOLATION_MODE_FORCE_DWORD: D2D1_3DTRANSFORM_INTERPOLATION_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_3DTRANSFORM_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DTRANSFORM_PROP_INTERPOLATION_MODE: D2D1_3DTRANSFORM_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DTRANSFORM_PROP_BORDER_MODE: D2D1_3DTRANSFORM_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DTRANSFORM_PROP_TRANSFORM_MATRIX: D2D1_3DTRANSFORM_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DTRANSFORM_PROP_FORCE_DWORD: D2D1_3DTRANSFORM_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_ANTIALIAS_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ANTIALIAS_MODE_PER_PRIMITIVE: D2D1_ANTIALIAS_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ANTIALIAS_MODE_ALIASED: D2D1_ANTIALIAS_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ANTIALIAS_MODE_FORCE_DWORD: D2D1_ANTIALIAS_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_APPEND_ALIGNED_ELEMENT: u32 = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_ARC_SEGMENT {
    pub point: Common::D2D_POINT_2F,
    pub size: Common::D2D_SIZE_F,
    pub rotationAngle: f32,
    pub sweepDirection: D2D1_SWEEP_DIRECTION,
    pub arcSize: D2D1_ARC_SIZE,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_ARC_SEGMENT {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_ARC_SEGMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_ARC_SIZE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ARC_SIZE_SMALL: D2D1_ARC_SIZE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ARC_SIZE_LARGE: D2D1_ARC_SIZE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ARC_SIZE_FORCE_DWORD: D2D1_ARC_SIZE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_ARITHMETICCOMPOSITE_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ARITHMETICCOMPOSITE_PROP_COEFFICIENTS: D2D1_ARITHMETICCOMPOSITE_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ARITHMETICCOMPOSITE_PROP_CLAMP_OUTPUT: D2D1_ARITHMETICCOMPOSITE_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ARITHMETICCOMPOSITE_PROP_FORCE_DWORD: D2D1_ARITHMETICCOMPOSITE_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_ATLAS_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ATLAS_PROP_INPUT_RECT: D2D1_ATLAS_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ATLAS_PROP_INPUT_PADDING_RECT: D2D1_ATLAS_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ATLAS_PROP_FORCE_DWORD: D2D1_ATLAS_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_BITMAPSOURCE_ALPHA_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ALPHA_MODE_PREMULTIPLIED: D2D1_BITMAPSOURCE_ALPHA_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ALPHA_MODE_STRAIGHT: D2D1_BITMAPSOURCE_ALPHA_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ALPHA_MODE_FORCE_DWORD: D2D1_BITMAPSOURCE_ALPHA_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_BITMAPSOURCE_INTERPOLATION_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_BITMAPSOURCE_INTERPOLATION_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_INTERPOLATION_MODE_LINEAR: D2D1_BITMAPSOURCE_INTERPOLATION_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_INTERPOLATION_MODE_CUBIC: D2D1_BITMAPSOURCE_INTERPOLATION_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_INTERPOLATION_MODE_FANT: D2D1_BITMAPSOURCE_INTERPOLATION_MODE = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_INTERPOLATION_MODE_MIPMAP_LINEAR: D2D1_BITMAPSOURCE_INTERPOLATION_MODE = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_INTERPOLATION_MODE_FORCE_DWORD: D2D1_BITMAPSOURCE_INTERPOLATION_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_BITMAPSOURCE_ORIENTATION = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ORIENTATION_DEFAULT: D2D1_BITMAPSOURCE_ORIENTATION = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ORIENTATION_FLIP_HORIZONTAL: D2D1_BITMAPSOURCE_ORIENTATION = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE180: D2D1_BITMAPSOURCE_ORIENTATION = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE180_FLIP_HORIZONTAL: D2D1_BITMAPSOURCE_ORIENTATION = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE270_FLIP_HORIZONTAL: D2D1_BITMAPSOURCE_ORIENTATION = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE90: D2D1_BITMAPSOURCE_ORIENTATION = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE90_FLIP_HORIZONTAL: D2D1_BITMAPSOURCE_ORIENTATION = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE270: D2D1_BITMAPSOURCE_ORIENTATION = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ORIENTATION_FORCE_DWORD: D2D1_BITMAPSOURCE_ORIENTATION = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_BITMAPSOURCE_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_PROP_WIC_BITMAP_SOURCE: D2D1_BITMAPSOURCE_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_PROP_SCALE: D2D1_BITMAPSOURCE_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_PROP_INTERPOLATION_MODE: D2D1_BITMAPSOURCE_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_PROP_ENABLE_DPI_CORRECTION: D2D1_BITMAPSOURCE_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_PROP_ALPHA_MODE: D2D1_BITMAPSOURCE_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_PROP_ORIENTATION: D2D1_BITMAPSOURCE_PROP = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_PROP_FORCE_DWORD: D2D1_BITMAPSOURCE_PROP = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_BITMAP_BRUSH_PROPERTIES {
    pub extendModeX: D2D1_EXTEND_MODE,
    pub extendModeY: D2D1_EXTEND_MODE,
    pub interpolationMode: D2D1_BITMAP_INTERPOLATION_MODE,
}
impl ::core::marker::Copy for D2D1_BITMAP_BRUSH_PROPERTIES {}
impl ::core::clone::Clone for D2D1_BITMAP_BRUSH_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_BITMAP_BRUSH_PROPERTIES1 {
    pub extendModeX: D2D1_EXTEND_MODE,
    pub extendModeY: D2D1_EXTEND_MODE,
    pub interpolationMode: D2D1_INTERPOLATION_MODE,
}
impl ::core::marker::Copy for D2D1_BITMAP_BRUSH_PROPERTIES1 {}
impl ::core::clone::Clone for D2D1_BITMAP_BRUSH_PROPERTIES1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_BITMAP_INTERPOLATION_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_BITMAP_INTERPOLATION_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAP_INTERPOLATION_MODE_LINEAR: D2D1_BITMAP_INTERPOLATION_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAP_INTERPOLATION_MODE_FORCE_DWORD: D2D1_BITMAP_INTERPOLATION_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_BITMAP_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAP_OPTIONS_NONE: D2D1_BITMAP_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAP_OPTIONS_TARGET: D2D1_BITMAP_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAP_OPTIONS_CANNOT_DRAW: D2D1_BITMAP_OPTIONS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAP_OPTIONS_CPU_READ: D2D1_BITMAP_OPTIONS = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAP_OPTIONS_GDI_COMPATIBLE: D2D1_BITMAP_OPTIONS = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAP_OPTIONS_FORCE_DWORD: D2D1_BITMAP_OPTIONS = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D2D1_BITMAP_PROPERTIES {
    pub pixelFormat: Common::D2D1_PIXEL_FORMAT,
    pub dpiX: f32,
    pub dpiY: f32,
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D2D1_BITMAP_PROPERTIES {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D2D1_BITMAP_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D2D1_BITMAP_PROPERTIES1 {
    pub pixelFormat: Common::D2D1_PIXEL_FORMAT,
    pub dpiX: f32,
    pub dpiY: f32,
    pub bitmapOptions: D2D1_BITMAP_OPTIONS,
    pub colorContext: *mut *mut *mut *mut ID2D1ColorContext,
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D2D1_BITMAP_PROPERTIES1 {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D2D1_BITMAP_PROPERTIES1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_BLEND = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_ZERO: D2D1_BLEND = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_ONE: D2D1_BLEND = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_SRC_COLOR: D2D1_BLEND = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_INV_SRC_COLOR: D2D1_BLEND = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_SRC_ALPHA: D2D1_BLEND = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_INV_SRC_ALPHA: D2D1_BLEND = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_DEST_ALPHA: D2D1_BLEND = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_INV_DEST_ALPHA: D2D1_BLEND = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_DEST_COLOR: D2D1_BLEND = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_INV_DEST_COLOR: D2D1_BLEND = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_SRC_ALPHA_SAT: D2D1_BLEND = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_BLEND_FACTOR: D2D1_BLEND = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_INV_BLEND_FACTOR: D2D1_BLEND = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_FORCE_DWORD: D2D1_BLEND = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_BLEND_DESCRIPTION {
    pub sourceBlend: D2D1_BLEND,
    pub destinationBlend: D2D1_BLEND,
    pub blendOperation: D2D1_BLEND_OPERATION,
    pub sourceBlendAlpha: D2D1_BLEND,
    pub destinationBlendAlpha: D2D1_BLEND,
    pub blendOperationAlpha: D2D1_BLEND_OPERATION,
    pub blendFactor: [f32; 4],
}
impl ::core::marker::Copy for D2D1_BLEND_DESCRIPTION {}
impl ::core::clone::Clone for D2D1_BLEND_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_BLEND_OPERATION = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_OPERATION_ADD: D2D1_BLEND_OPERATION = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_OPERATION_SUBTRACT: D2D1_BLEND_OPERATION = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_OPERATION_REV_SUBTRACT: D2D1_BLEND_OPERATION = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_OPERATION_MIN: D2D1_BLEND_OPERATION = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_OPERATION_MAX: D2D1_BLEND_OPERATION = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_OPERATION_FORCE_DWORD: D2D1_BLEND_OPERATION = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_BLEND_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_PROP_MODE: D2D1_BLEND_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_PROP_FORCE_DWORD: D2D1_BLEND_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_BORDER_EDGE_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BORDER_EDGE_MODE_CLAMP: D2D1_BORDER_EDGE_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BORDER_EDGE_MODE_WRAP: D2D1_BORDER_EDGE_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BORDER_EDGE_MODE_MIRROR: D2D1_BORDER_EDGE_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BORDER_EDGE_MODE_FORCE_DWORD: D2D1_BORDER_EDGE_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_BORDER_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BORDER_PROP_EDGE_MODE_X: D2D1_BORDER_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BORDER_PROP_EDGE_MODE_Y: D2D1_BORDER_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BORDER_PROP_FORCE_DWORD: D2D1_BORDER_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_BRIGHTNESS_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BRIGHTNESS_PROP_WHITE_POINT: D2D1_BRIGHTNESS_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BRIGHTNESS_PROP_BLACK_POINT: D2D1_BRIGHTNESS_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BRIGHTNESS_PROP_FORCE_DWORD: D2D1_BRIGHTNESS_PROP = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Foundation_Numerics\"`*"]
#[cfg(feature = "Foundation_Numerics")]
pub struct D2D1_BRUSH_PROPERTIES {
    pub opacity: f32,
    pub transform: super::super::super::Foundation::Numerics::Matrix3x2,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for D2D1_BRUSH_PROPERTIES {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for D2D1_BRUSH_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_BUFFER_PRECISION = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BUFFER_PRECISION_UNKNOWN: D2D1_BUFFER_PRECISION = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BUFFER_PRECISION_8BPC_UNORM: D2D1_BUFFER_PRECISION = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BUFFER_PRECISION_8BPC_UNORM_SRGB: D2D1_BUFFER_PRECISION = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BUFFER_PRECISION_16BPC_UNORM: D2D1_BUFFER_PRECISION = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BUFFER_PRECISION_16BPC_FLOAT: D2D1_BUFFER_PRECISION = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BUFFER_PRECISION_32BPC_FLOAT: D2D1_BUFFER_PRECISION = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BUFFER_PRECISION_FORCE_DWORD: D2D1_BUFFER_PRECISION = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_CAP_STYLE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CAP_STYLE_FLAT: D2D1_CAP_STYLE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CAP_STYLE_SQUARE: D2D1_CAP_STYLE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CAP_STYLE_ROUND: D2D1_CAP_STYLE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CAP_STYLE_TRIANGLE: D2D1_CAP_STYLE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CAP_STYLE_FORCE_DWORD: D2D1_CAP_STYLE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_CHANGE_TYPE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANGE_TYPE_NONE: D2D1_CHANGE_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANGE_TYPE_PROPERTIES: D2D1_CHANGE_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANGE_TYPE_CONTEXT: D2D1_CHANGE_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANGE_TYPE_GRAPH: D2D1_CHANGE_TYPE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANGE_TYPE_FORCE_DWORD: D2D1_CHANGE_TYPE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_CHANNEL_DEPTH = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANNEL_DEPTH_DEFAULT: D2D1_CHANNEL_DEPTH = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANNEL_DEPTH_1: D2D1_CHANNEL_DEPTH = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANNEL_DEPTH_4: D2D1_CHANNEL_DEPTH = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANNEL_DEPTH_FORCE_DWORD: D2D1_CHANNEL_DEPTH = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_CHANNEL_SELECTOR = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANNEL_SELECTOR_R: D2D1_CHANNEL_SELECTOR = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANNEL_SELECTOR_G: D2D1_CHANNEL_SELECTOR = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANNEL_SELECTOR_B: D2D1_CHANNEL_SELECTOR = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANNEL_SELECTOR_A: D2D1_CHANNEL_SELECTOR = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANNEL_SELECTOR_FORCE_DWORD: D2D1_CHANNEL_SELECTOR = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_CHROMAKEY_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHROMAKEY_PROP_COLOR: D2D1_CHROMAKEY_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHROMAKEY_PROP_TOLERANCE: D2D1_CHROMAKEY_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHROMAKEY_PROP_INVERT_ALPHA: D2D1_CHROMAKEY_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHROMAKEY_PROP_FEATHER: D2D1_CHROMAKEY_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHROMAKEY_PROP_FORCE_DWORD: D2D1_CHROMAKEY_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_COLORMANAGEMENT_ALPHA_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_ALPHA_MODE_PREMULTIPLIED: D2D1_COLORMANAGEMENT_ALPHA_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_ALPHA_MODE_STRAIGHT: D2D1_COLORMANAGEMENT_ALPHA_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_ALPHA_MODE_FORCE_DWORD: D2D1_COLORMANAGEMENT_ALPHA_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_COLORMANAGEMENT_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_PROP_SOURCE_COLOR_CONTEXT: D2D1_COLORMANAGEMENT_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_PROP_SOURCE_RENDERING_INTENT: D2D1_COLORMANAGEMENT_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_PROP_DESTINATION_COLOR_CONTEXT: D2D1_COLORMANAGEMENT_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_PROP_DESTINATION_RENDERING_INTENT: D2D1_COLORMANAGEMENT_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_PROP_ALPHA_MODE: D2D1_COLORMANAGEMENT_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_PROP_QUALITY: D2D1_COLORMANAGEMENT_PROP = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_PROP_FORCE_DWORD: D2D1_COLORMANAGEMENT_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_COLORMANAGEMENT_QUALITY = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_QUALITY_PROOF: D2D1_COLORMANAGEMENT_QUALITY = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_QUALITY_NORMAL: D2D1_COLORMANAGEMENT_QUALITY = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_QUALITY_BEST: D2D1_COLORMANAGEMENT_QUALITY = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_QUALITY_FORCE_DWORD: D2D1_COLORMANAGEMENT_QUALITY = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_COLORMANAGEMENT_RENDERING_INTENT = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_RENDERING_INTENT_PERCEPTUAL: D2D1_COLORMANAGEMENT_RENDERING_INTENT = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_RENDERING_INTENT_RELATIVE_COLORIMETRIC: D2D1_COLORMANAGEMENT_RENDERING_INTENT = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_RENDERING_INTENT_SATURATION: D2D1_COLORMANAGEMENT_RENDERING_INTENT = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_RENDERING_INTENT_ABSOLUTE_COLORIMETRIC: D2D1_COLORMANAGEMENT_RENDERING_INTENT = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_RENDERING_INTENT_FORCE_DWORD: D2D1_COLORMANAGEMENT_RENDERING_INTENT = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_COLORMATRIX_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMATRIX_PROP_COLOR_MATRIX: D2D1_COLORMATRIX_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMATRIX_PROP_ALPHA_MODE: D2D1_COLORMATRIX_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMATRIX_PROP_CLAMP_OUTPUT: D2D1_COLORMATRIX_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMATRIX_PROP_FORCE_DWORD: D2D1_COLORMATRIX_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION_DEFAULT: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION_DISABLE: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION_FORCE_DWORD: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_COLOR_CONTEXT_TYPE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_CONTEXT_TYPE_ICC: D2D1_COLOR_CONTEXT_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_CONTEXT_TYPE_SIMPLE: D2D1_COLOR_CONTEXT_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_CONTEXT_TYPE_DXGI: D2D1_COLOR_CONTEXT_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_CONTEXT_TYPE_FORCE_DWORD: D2D1_COLOR_CONTEXT_TYPE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_COLOR_INTERPOLATION_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_INTERPOLATION_MODE_STRAIGHT: D2D1_COLOR_INTERPOLATION_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_INTERPOLATION_MODE_PREMULTIPLIED: D2D1_COLOR_INTERPOLATION_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_INTERPOLATION_MODE_FORCE_DWORD: D2D1_COLOR_INTERPOLATION_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_COLOR_SPACE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_SPACE_CUSTOM: D2D1_COLOR_SPACE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_SPACE_SRGB: D2D1_COLOR_SPACE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_SPACE_SCRGB: D2D1_COLOR_SPACE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_SPACE_FORCE_DWORD: D2D1_COLOR_SPACE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_COMBINE_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COMBINE_MODE_UNION: D2D1_COMBINE_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COMBINE_MODE_INTERSECT: D2D1_COMBINE_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COMBINE_MODE_XOR: D2D1_COMBINE_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COMBINE_MODE_EXCLUDE: D2D1_COMBINE_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COMBINE_MODE_FORCE_DWORD: D2D1_COMBINE_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_NONE: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_GDI_COMPATIBLE: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_FORCE_DWORD: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_COMPOSITE_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COMPOSITE_PROP_MODE: D2D1_COMPOSITE_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COMPOSITE_PROP_FORCE_DWORD: D2D1_COMPOSITE_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_CONTRAST_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONTRAST_PROP_CONTRAST: D2D1_CONTRAST_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONTRAST_PROP_CLAMP_INPUT: D2D1_CONTRAST_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONTRAST_PROP_FORCE_DWORD: D2D1_CONTRAST_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_CONVOLVEMATRIX_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_KERNEL_UNIT_LENGTH: D2D1_CONVOLVEMATRIX_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_SCALE_MODE: D2D1_CONVOLVEMATRIX_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_KERNEL_SIZE_X: D2D1_CONVOLVEMATRIX_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_KERNEL_SIZE_Y: D2D1_CONVOLVEMATRIX_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_KERNEL_MATRIX: D2D1_CONVOLVEMATRIX_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_DIVISOR: D2D1_CONVOLVEMATRIX_PROP = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_BIAS: D2D1_CONVOLVEMATRIX_PROP = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_KERNEL_OFFSET: D2D1_CONVOLVEMATRIX_PROP = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_PRESERVE_ALPHA: D2D1_CONVOLVEMATRIX_PROP = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_BORDER_MODE: D2D1_CONVOLVEMATRIX_PROP = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_CLAMP_OUTPUT: D2D1_CONVOLVEMATRIX_PROP = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_FORCE_DWORD: D2D1_CONVOLVEMATRIX_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_CONVOLVEMATRIX_SCALE_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_CONVOLVEMATRIX_SCALE_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_SCALE_MODE_LINEAR: D2D1_CONVOLVEMATRIX_SCALE_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_SCALE_MODE_CUBIC: D2D1_CONVOLVEMATRIX_SCALE_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_CONVOLVEMATRIX_SCALE_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_SCALE_MODE_ANISOTROPIC: D2D1_CONVOLVEMATRIX_SCALE_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_SCALE_MODE_HIGH_QUALITY_CUBIC: D2D1_CONVOLVEMATRIX_SCALE_MODE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_SCALE_MODE_FORCE_DWORD: D2D1_CONVOLVEMATRIX_SCALE_MODE = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_CREATION_PROPERTIES {
    pub threadingMode: D2D1_THREADING_MODE,
    pub debugLevel: D2D1_DEBUG_LEVEL,
    pub options: D2D1_DEVICE_CONTEXT_OPTIONS,
}
impl ::core::marker::Copy for D2D1_CREATION_PROPERTIES {}
impl ::core::clone::Clone for D2D1_CREATION_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_CROP_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CROP_PROP_RECT: D2D1_CROP_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CROP_PROP_BORDER_MODE: D2D1_CROP_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CROP_PROP_FORCE_DWORD: D2D1_CROP_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_CROSSFADE_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CROSSFADE_PROP_WEIGHT: D2D1_CROSSFADE_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CROSSFADE_PROP_FORCE_DWORD: D2D1_CROSSFADE_PROP = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES {
    pub shaderBufferWithInputSignature: *const u8,
    pub shaderBufferSize: u32,
    pub inputElements: *const D2D1_INPUT_ELEMENT_DESC,
    pub elementCount: u32,
    pub stride: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_DASH_STYLE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DASH_STYLE_SOLID: D2D1_DASH_STYLE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DASH_STYLE_DASH: D2D1_DASH_STYLE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DASH_STYLE_DOT: D2D1_DASH_STYLE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DASH_STYLE_DASH_DOT: D2D1_DASH_STYLE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DASH_STYLE_DASH_DOT_DOT: D2D1_DASH_STYLE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DASH_STYLE_CUSTOM: D2D1_DASH_STYLE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DASH_STYLE_FORCE_DWORD: D2D1_DASH_STYLE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_DC_INITIALIZE_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DC_INITIALIZE_MODE_COPY: D2D1_DC_INITIALIZE_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DC_INITIALIZE_MODE_CLEAR: D2D1_DC_INITIALIZE_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DC_INITIALIZE_MODE_FORCE_DWORD: D2D1_DC_INITIALIZE_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_DEBUG_LEVEL = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DEBUG_LEVEL_NONE: D2D1_DEBUG_LEVEL = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DEBUG_LEVEL_ERROR: D2D1_DEBUG_LEVEL = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DEBUG_LEVEL_WARNING: D2D1_DEBUG_LEVEL = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DEBUG_LEVEL_INFORMATION: D2D1_DEBUG_LEVEL = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DEBUG_LEVEL_FORCE_DWORD: D2D1_DEBUG_LEVEL = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DEFAULT_FLATTENING_TOLERANCE: f32 = 0.25f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_DEVICE_CONTEXT_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DEVICE_CONTEXT_OPTIONS_NONE: D2D1_DEVICE_CONTEXT_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DEVICE_CONTEXT_OPTIONS_ENABLE_MULTITHREADED_OPTIMIZATIONS: D2D1_DEVICE_CONTEXT_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DEVICE_CONTEXT_OPTIONS_FORCE_DWORD: D2D1_DEVICE_CONTEXT_OPTIONS = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_DIRECTIONALBLUR_OPTIMIZATION = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DIRECTIONALBLUR_OPTIMIZATION_SPEED: D2D1_DIRECTIONALBLUR_OPTIMIZATION = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DIRECTIONALBLUR_OPTIMIZATION_BALANCED: D2D1_DIRECTIONALBLUR_OPTIMIZATION = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DIRECTIONALBLUR_OPTIMIZATION_QUALITY: D2D1_DIRECTIONALBLUR_OPTIMIZATION = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DIRECTIONALBLUR_OPTIMIZATION_FORCE_DWORD: D2D1_DIRECTIONALBLUR_OPTIMIZATION = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_DIRECTIONALBLUR_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DIRECTIONALBLUR_PROP_STANDARD_DEVIATION: D2D1_DIRECTIONALBLUR_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DIRECTIONALBLUR_PROP_ANGLE: D2D1_DIRECTIONALBLUR_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DIRECTIONALBLUR_PROP_OPTIMIZATION: D2D1_DIRECTIONALBLUR_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DIRECTIONALBLUR_PROP_BORDER_MODE: D2D1_DIRECTIONALBLUR_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DIRECTIONALBLUR_PROP_FORCE_DWORD: D2D1_DIRECTIONALBLUR_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_DISCRETETRANSFER_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISCRETETRANSFER_PROP_RED_TABLE: D2D1_DISCRETETRANSFER_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISCRETETRANSFER_PROP_RED_DISABLE: D2D1_DISCRETETRANSFER_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISCRETETRANSFER_PROP_GREEN_TABLE: D2D1_DISCRETETRANSFER_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISCRETETRANSFER_PROP_GREEN_DISABLE: D2D1_DISCRETETRANSFER_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISCRETETRANSFER_PROP_BLUE_TABLE: D2D1_DISCRETETRANSFER_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISCRETETRANSFER_PROP_BLUE_DISABLE: D2D1_DISCRETETRANSFER_PROP = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISCRETETRANSFER_PROP_ALPHA_TABLE: D2D1_DISCRETETRANSFER_PROP = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISCRETETRANSFER_PROP_ALPHA_DISABLE: D2D1_DISCRETETRANSFER_PROP = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISCRETETRANSFER_PROP_CLAMP_OUTPUT: D2D1_DISCRETETRANSFER_PROP = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISCRETETRANSFER_PROP_FORCE_DWORD: D2D1_DISCRETETRANSFER_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_DISPLACEMENTMAP_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISPLACEMENTMAP_PROP_SCALE: D2D1_DISPLACEMENTMAP_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISPLACEMENTMAP_PROP_X_CHANNEL_SELECT: D2D1_DISPLACEMENTMAP_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISPLACEMENTMAP_PROP_Y_CHANNEL_SELECT: D2D1_DISPLACEMENTMAP_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISPLACEMENTMAP_PROP_FORCE_DWORD: D2D1_DISPLACEMENTMAP_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_DISTANTDIFFUSE_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_PROP_AZIMUTH: D2D1_DISTANTDIFFUSE_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_PROP_ELEVATION: D2D1_DISTANTDIFFUSE_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_PROP_DIFFUSE_CONSTANT: D2D1_DISTANTDIFFUSE_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_PROP_SURFACE_SCALE: D2D1_DISTANTDIFFUSE_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_PROP_COLOR: D2D1_DISTANTDIFFUSE_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_PROP_KERNEL_UNIT_LENGTH: D2D1_DISTANTDIFFUSE_PROP = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_PROP_SCALE_MODE: D2D1_DISTANTDIFFUSE_PROP = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_PROP_FORCE_DWORD: D2D1_DISTANTDIFFUSE_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_DISTANTDIFFUSE_SCALE_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_DISTANTDIFFUSE_SCALE_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_SCALE_MODE_LINEAR: D2D1_DISTANTDIFFUSE_SCALE_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_SCALE_MODE_CUBIC: D2D1_DISTANTDIFFUSE_SCALE_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_DISTANTDIFFUSE_SCALE_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_SCALE_MODE_ANISOTROPIC: D2D1_DISTANTDIFFUSE_SCALE_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_SCALE_MODE_HIGH_QUALITY_CUBIC: D2D1_DISTANTDIFFUSE_SCALE_MODE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_SCALE_MODE_FORCE_DWORD: D2D1_DISTANTDIFFUSE_SCALE_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_DISTANTSPECULAR_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_PROP_AZIMUTH: D2D1_DISTANTSPECULAR_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_PROP_ELEVATION: D2D1_DISTANTSPECULAR_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_PROP_SPECULAR_EXPONENT: D2D1_DISTANTSPECULAR_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_PROP_SPECULAR_CONSTANT: D2D1_DISTANTSPECULAR_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_PROP_SURFACE_SCALE: D2D1_DISTANTSPECULAR_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_PROP_COLOR: D2D1_DISTANTSPECULAR_PROP = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_PROP_KERNEL_UNIT_LENGTH: D2D1_DISTANTSPECULAR_PROP = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_PROP_SCALE_MODE: D2D1_DISTANTSPECULAR_PROP = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_PROP_FORCE_DWORD: D2D1_DISTANTSPECULAR_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_DISTANTSPECULAR_SCALE_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_DISTANTSPECULAR_SCALE_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_SCALE_MODE_LINEAR: D2D1_DISTANTSPECULAR_SCALE_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_SCALE_MODE_CUBIC: D2D1_DISTANTSPECULAR_SCALE_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_DISTANTSPECULAR_SCALE_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_SCALE_MODE_ANISOTROPIC: D2D1_DISTANTSPECULAR_SCALE_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_SCALE_MODE_HIGH_QUALITY_CUBIC: D2D1_DISTANTSPECULAR_SCALE_MODE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_SCALE_MODE_FORCE_DWORD: D2D1_DISTANTSPECULAR_SCALE_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_DPICOMPENSATION_INTERPOLATION_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_DPICOMPENSATION_INTERPOLATION_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_INTERPOLATION_MODE_LINEAR: D2D1_DPICOMPENSATION_INTERPOLATION_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_INTERPOLATION_MODE_CUBIC: D2D1_DPICOMPENSATION_INTERPOLATION_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR: D2D1_DPICOMPENSATION_INTERPOLATION_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_INTERPOLATION_MODE_ANISOTROPIC: D2D1_DPICOMPENSATION_INTERPOLATION_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC: D2D1_DPICOMPENSATION_INTERPOLATION_MODE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_INTERPOLATION_MODE_FORCE_DWORD: D2D1_DPICOMPENSATION_INTERPOLATION_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_DPICOMPENSATION_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_PROP_INTERPOLATION_MODE: D2D1_DPICOMPENSATION_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_PROP_BORDER_MODE: D2D1_DPICOMPENSATION_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_PROP_INPUT_DPI: D2D1_DPICOMPENSATION_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_PROP_FORCE_DWORD: D2D1_DPICOMPENSATION_PROP = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Foundation_Numerics\"`*"]
#[cfg(feature = "Foundation_Numerics")]
pub struct D2D1_DRAWING_STATE_DESCRIPTION {
    pub antialiasMode: D2D1_ANTIALIAS_MODE,
    pub textAntialiasMode: D2D1_TEXT_ANTIALIAS_MODE,
    pub tag1: u64,
    pub tag2: u64,
    pub transform: super::super::super::Foundation::Numerics::Matrix3x2,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for D2D1_DRAWING_STATE_DESCRIPTION {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for D2D1_DRAWING_STATE_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Foundation_Numerics\"`*"]
#[cfg(feature = "Foundation_Numerics")]
pub struct D2D1_DRAWING_STATE_DESCRIPTION1 {
    pub antialiasMode: D2D1_ANTIALIAS_MODE,
    pub textAntialiasMode: D2D1_TEXT_ANTIALIAS_MODE,
    pub tag1: u64,
    pub tag2: u64,
    pub transform: super::super::super::Foundation::Numerics::Matrix3x2,
    pub primitiveBlend: D2D1_PRIMITIVE_BLEND,
    pub unitMode: D2D1_UNIT_MODE,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for D2D1_DRAWING_STATE_DESCRIPTION1 {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for D2D1_DRAWING_STATE_DESCRIPTION1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_DRAW_TEXT_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DRAW_TEXT_OPTIONS_NO_SNAP: D2D1_DRAW_TEXT_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DRAW_TEXT_OPTIONS_CLIP: D2D1_DRAW_TEXT_OPTIONS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DRAW_TEXT_OPTIONS_ENABLE_COLOR_FONT: D2D1_DRAW_TEXT_OPTIONS = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DRAW_TEXT_OPTIONS_DISABLE_COLOR_BITMAP_SNAPPING: D2D1_DRAW_TEXT_OPTIONS = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DRAW_TEXT_OPTIONS_NONE: D2D1_DRAW_TEXT_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DRAW_TEXT_OPTIONS_FORCE_DWORD: D2D1_DRAW_TEXT_OPTIONS = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_EDGEDETECTION_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EDGEDETECTION_MODE_SOBEL: D2D1_EDGEDETECTION_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EDGEDETECTION_MODE_PREWITT: D2D1_EDGEDETECTION_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EDGEDETECTION_MODE_FORCE_DWORD: D2D1_EDGEDETECTION_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_EDGEDETECTION_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EDGEDETECTION_PROP_STRENGTH: D2D1_EDGEDETECTION_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EDGEDETECTION_PROP_BLUR_RADIUS: D2D1_EDGEDETECTION_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EDGEDETECTION_PROP_MODE: D2D1_EDGEDETECTION_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EDGEDETECTION_PROP_OVERLAY_EDGES: D2D1_EDGEDETECTION_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EDGEDETECTION_PROP_ALPHA_MODE: D2D1_EDGEDETECTION_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EDGEDETECTION_PROP_FORCE_DWORD: D2D1_EDGEDETECTION_PROP = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_EFFECT_INPUT_DESCRIPTION {
    pub effect: *mut *mut *mut *mut ID2D1Effect,
    pub inputIndex: u32,
    pub inputRectangle: Common::D2D_RECT_F,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_EFFECT_INPUT_DESCRIPTION {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_EFFECT_INPUT_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_ELLIPSE {
    pub point: Common::D2D_POINT_2F,
    pub radiusX: f32,
    pub radiusY: f32,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_ELLIPSE {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_ELLIPSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_EMBOSS_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EMBOSS_PROP_HEIGHT: D2D1_EMBOSS_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EMBOSS_PROP_DIRECTION: D2D1_EMBOSS_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EMBOSS_PROP_FORCE_DWORD: D2D1_EMBOSS_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_EXPOSURE_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EXPOSURE_PROP_EXPOSURE_VALUE: D2D1_EXPOSURE_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EXPOSURE_PROP_FORCE_DWORD: D2D1_EXPOSURE_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_EXTEND_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EXTEND_MODE_CLAMP: D2D1_EXTEND_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EXTEND_MODE_WRAP: D2D1_EXTEND_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EXTEND_MODE_MIRROR: D2D1_EXTEND_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EXTEND_MODE_FORCE_DWORD: D2D1_EXTEND_MODE = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_FACTORY_OPTIONS {
    pub debugLevel: D2D1_DEBUG_LEVEL,
}
impl ::core::marker::Copy for D2D1_FACTORY_OPTIONS {}
impl ::core::clone::Clone for D2D1_FACTORY_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_FACTORY_TYPE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FACTORY_TYPE_SINGLE_THREADED: D2D1_FACTORY_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FACTORY_TYPE_MULTI_THREADED: D2D1_FACTORY_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FACTORY_TYPE_FORCE_DWORD: D2D1_FACTORY_TYPE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_FEATURE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FEATURE_DOUBLES: D2D1_FEATURE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FEATURE_D3D10_X_HARDWARE_OPTIONS: D2D1_FEATURE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FEATURE_FORCE_DWORD: D2D1_FEATURE = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D2D1_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
    pub computeShaders_Plus_RawAndStructuredBuffers_Via_Shader_4_x: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D2D1_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D2D1_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D2D1_FEATURE_DATA_DOUBLES {
    pub doublePrecisionFloatShaderOps: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D2D1_FEATURE_DATA_DOUBLES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D2D1_FEATURE_DATA_DOUBLES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_FEATURE_LEVEL = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FEATURE_LEVEL_DEFAULT: D2D1_FEATURE_LEVEL = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FEATURE_LEVEL_9: D2D1_FEATURE_LEVEL = 37120u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FEATURE_LEVEL_10: D2D1_FEATURE_LEVEL = 40960u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FEATURE_LEVEL_FORCE_DWORD: D2D1_FEATURE_LEVEL = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_FILTER = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FILTER_MIN_MAG_MIP_POINT: D2D1_FILTER = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FILTER_MIN_MAG_POINT_MIP_LINEAR: D2D1_FILTER = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT: D2D1_FILTER = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FILTER_MIN_POINT_MAG_MIP_LINEAR: D2D1_FILTER = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FILTER_MIN_LINEAR_MAG_MIP_POINT: D2D1_FILTER = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D2D1_FILTER = 17u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FILTER_MIN_MAG_LINEAR_MIP_POINT: D2D1_FILTER = 20u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FILTER_MIN_MAG_MIP_LINEAR: D2D1_FILTER = 21u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FILTER_ANISOTROPIC: D2D1_FILTER = 85u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FILTER_FORCE_DWORD: D2D1_FILTER = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_FLOOD_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FLOOD_PROP_COLOR: D2D1_FLOOD_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FLOOD_PROP_FORCE_DWORD: D2D1_FLOOD_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_GAMMA = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMA_2_2: D2D1_GAMMA = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMA_1_0: D2D1_GAMMA = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMA_FORCE_DWORD: D2D1_GAMMA = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_GAMMA1 = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMA1_G22: D2D1_GAMMA1 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMA1_G10: D2D1_GAMMA1 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMA1_G2084: D2D1_GAMMA1 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMA1_FORCE_DWORD: D2D1_GAMMA1 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_GAMMATRANSFER_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_RED_AMPLITUDE: D2D1_GAMMATRANSFER_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_RED_EXPONENT: D2D1_GAMMATRANSFER_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_RED_OFFSET: D2D1_GAMMATRANSFER_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_RED_DISABLE: D2D1_GAMMATRANSFER_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_GREEN_AMPLITUDE: D2D1_GAMMATRANSFER_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_GREEN_EXPONENT: D2D1_GAMMATRANSFER_PROP = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_GREEN_OFFSET: D2D1_GAMMATRANSFER_PROP = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_GREEN_DISABLE: D2D1_GAMMATRANSFER_PROP = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_BLUE_AMPLITUDE: D2D1_GAMMATRANSFER_PROP = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_BLUE_EXPONENT: D2D1_GAMMATRANSFER_PROP = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_BLUE_OFFSET: D2D1_GAMMATRANSFER_PROP = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_BLUE_DISABLE: D2D1_GAMMATRANSFER_PROP = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_ALPHA_AMPLITUDE: D2D1_GAMMATRANSFER_PROP = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_ALPHA_EXPONENT: D2D1_GAMMATRANSFER_PROP = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_ALPHA_OFFSET: D2D1_GAMMATRANSFER_PROP = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_ALPHA_DISABLE: D2D1_GAMMATRANSFER_PROP = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_CLAMP_OUTPUT: D2D1_GAMMATRANSFER_PROP = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_FORCE_DWORD: D2D1_GAMMATRANSFER_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_GAUSSIANBLUR_OPTIMIZATION = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAUSSIANBLUR_OPTIMIZATION_SPEED: D2D1_GAUSSIANBLUR_OPTIMIZATION = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAUSSIANBLUR_OPTIMIZATION_BALANCED: D2D1_GAUSSIANBLUR_OPTIMIZATION = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAUSSIANBLUR_OPTIMIZATION_QUALITY: D2D1_GAUSSIANBLUR_OPTIMIZATION = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAUSSIANBLUR_OPTIMIZATION_FORCE_DWORD: D2D1_GAUSSIANBLUR_OPTIMIZATION = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_GAUSSIANBLUR_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAUSSIANBLUR_PROP_STANDARD_DEVIATION: D2D1_GAUSSIANBLUR_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAUSSIANBLUR_PROP_OPTIMIZATION: D2D1_GAUSSIANBLUR_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAUSSIANBLUR_PROP_BORDER_MODE: D2D1_GAUSSIANBLUR_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAUSSIANBLUR_PROP_FORCE_DWORD: D2D1_GAUSSIANBLUR_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_GEOMETRY_RELATION = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GEOMETRY_RELATION_UNKNOWN: D2D1_GEOMETRY_RELATION = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GEOMETRY_RELATION_DISJOINT: D2D1_GEOMETRY_RELATION = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GEOMETRY_RELATION_IS_CONTAINED: D2D1_GEOMETRY_RELATION = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GEOMETRY_RELATION_CONTAINS: D2D1_GEOMETRY_RELATION = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GEOMETRY_RELATION_OVERLAP: D2D1_GEOMETRY_RELATION = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GEOMETRY_RELATION_FORCE_DWORD: D2D1_GEOMETRY_RELATION = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_GEOMETRY_SIMPLIFICATION_OPTION = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GEOMETRY_SIMPLIFICATION_OPTION_CUBICS_AND_LINES: D2D1_GEOMETRY_SIMPLIFICATION_OPTION = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GEOMETRY_SIMPLIFICATION_OPTION_LINES: D2D1_GEOMETRY_SIMPLIFICATION_OPTION = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GEOMETRY_SIMPLIFICATION_OPTION_FORCE_DWORD: D2D1_GEOMETRY_SIMPLIFICATION_OPTION = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_GRADIENT_MESH_PATCH {
    pub point00: Common::D2D_POINT_2F,
    pub point01: Common::D2D_POINT_2F,
    pub point02: Common::D2D_POINT_2F,
    pub point03: Common::D2D_POINT_2F,
    pub point10: Common::D2D_POINT_2F,
    pub point11: Common::D2D_POINT_2F,
    pub point12: Common::D2D_POINT_2F,
    pub point13: Common::D2D_POINT_2F,
    pub point20: Common::D2D_POINT_2F,
    pub point21: Common::D2D_POINT_2F,
    pub point22: Common::D2D_POINT_2F,
    pub point23: Common::D2D_POINT_2F,
    pub point30: Common::D2D_POINT_2F,
    pub point31: Common::D2D_POINT_2F,
    pub point32: Common::D2D_POINT_2F,
    pub point33: Common::D2D_POINT_2F,
    pub color00: Common::D2D1_COLOR_F,
    pub color03: Common::D2D1_COLOR_F,
    pub color30: Common::D2D1_COLOR_F,
    pub color33: Common::D2D1_COLOR_F,
    pub topEdgeMode: D2D1_PATCH_EDGE_MODE,
    pub leftEdgeMode: D2D1_PATCH_EDGE_MODE,
    pub bottomEdgeMode: D2D1_PATCH_EDGE_MODE,
    pub rightEdgeMode: D2D1_PATCH_EDGE_MODE,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_GRADIENT_MESH_PATCH {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_GRADIENT_MESH_PATCH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_GRADIENT_STOP {
    pub position: f32,
    pub color: Common::D2D1_COLOR_F,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_GRADIENT_STOP {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_GRADIENT_STOP {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_HDRTONEMAP_DISPLAY_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HDRTONEMAP_DISPLAY_MODE_SDR: D2D1_HDRTONEMAP_DISPLAY_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HDRTONEMAP_DISPLAY_MODE_HDR: D2D1_HDRTONEMAP_DISPLAY_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HDRTONEMAP_DISPLAY_MODE_FORCE_DWORD: D2D1_HDRTONEMAP_DISPLAY_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_HDRTONEMAP_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HDRTONEMAP_PROP_INPUT_MAX_LUMINANCE: D2D1_HDRTONEMAP_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HDRTONEMAP_PROP_OUTPUT_MAX_LUMINANCE: D2D1_HDRTONEMAP_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HDRTONEMAP_PROP_DISPLAY_MODE: D2D1_HDRTONEMAP_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HDRTONEMAP_PROP_FORCE_DWORD: D2D1_HDRTONEMAP_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA_LINEAR: D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA_SRGB: D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA_FORCE_DWORD: D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_HIGHLIGHTSANDSHADOWS_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HIGHLIGHTSANDSHADOWS_PROP_HIGHLIGHTS: D2D1_HIGHLIGHTSANDSHADOWS_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HIGHLIGHTSANDSHADOWS_PROP_SHADOWS: D2D1_HIGHLIGHTSANDSHADOWS_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HIGHLIGHTSANDSHADOWS_PROP_CLARITY: D2D1_HIGHLIGHTSANDSHADOWS_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HIGHLIGHTSANDSHADOWS_PROP_INPUT_GAMMA: D2D1_HIGHLIGHTSANDSHADOWS_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HIGHLIGHTSANDSHADOWS_PROP_MASK_BLUR_RADIUS: D2D1_HIGHLIGHTSANDSHADOWS_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HIGHLIGHTSANDSHADOWS_PROP_FORCE_DWORD: D2D1_HIGHLIGHTSANDSHADOWS_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_HISTOGRAM_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HISTOGRAM_PROP_NUM_BINS: D2D1_HISTOGRAM_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HISTOGRAM_PROP_CHANNEL_SELECT: D2D1_HISTOGRAM_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HISTOGRAM_PROP_HISTOGRAM_OUTPUT: D2D1_HISTOGRAM_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HISTOGRAM_PROP_FORCE_DWORD: D2D1_HISTOGRAM_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_HUEROTATION_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HUEROTATION_PROP_ANGLE: D2D1_HUEROTATION_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HUEROTATION_PROP_FORCE_DWORD: D2D1_HUEROTATION_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_HUETORGB_INPUT_COLOR_SPACE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HUETORGB_INPUT_COLOR_SPACE_HUE_SATURATION_VALUE: D2D1_HUETORGB_INPUT_COLOR_SPACE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HUETORGB_INPUT_COLOR_SPACE_HUE_SATURATION_LIGHTNESS: D2D1_HUETORGB_INPUT_COLOR_SPACE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HUETORGB_INPUT_COLOR_SPACE_FORCE_DWORD: D2D1_HUETORGB_INPUT_COLOR_SPACE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_HUETORGB_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HUETORGB_PROP_INPUT_COLOR_SPACE: D2D1_HUETORGB_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HUETORGB_PROP_FORCE_DWORD: D2D1_HUETORGB_PROP = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub struct D2D1_HWND_RENDER_TARGET_PROPERTIES {
    pub hwnd: super::super::Foundation::HWND,
    pub pixelSize: Common::D2D_SIZE_U,
    pub presentOptions: D2D1_PRESENT_OPTIONS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::marker::Copy for D2D1_HWND_RENDER_TARGET_PROPERTIES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::clone::Clone for D2D1_HWND_RENDER_TARGET_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_IMAGE_BRUSH_PROPERTIES {
    pub sourceRectangle: Common::D2D_RECT_F,
    pub extendModeX: D2D1_EXTEND_MODE,
    pub extendModeY: D2D1_EXTEND_MODE,
    pub interpolationMode: D2D1_INTERPOLATION_MODE,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_IMAGE_BRUSH_PROPERTIES {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_IMAGE_BRUSH_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS_NONE: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS_LOW_QUALITY_PRIMARY_CONVERSION: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS_FORCE_DWORD: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_IMAGE_SOURCE_LOADING_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_IMAGE_SOURCE_LOADING_OPTIONS_NONE: D2D1_IMAGE_SOURCE_LOADING_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_IMAGE_SOURCE_LOADING_OPTIONS_RELEASE_SOURCE: D2D1_IMAGE_SOURCE_LOADING_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_IMAGE_SOURCE_LOADING_OPTIONS_CACHE_ON_DEMAND: D2D1_IMAGE_SOURCE_LOADING_OPTIONS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_IMAGE_SOURCE_LOADING_OPTIONS_FORCE_DWORD: D2D1_IMAGE_SOURCE_LOADING_OPTIONS = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_INK_BEZIER_SEGMENT {
    pub point1: D2D1_INK_POINT,
    pub point2: D2D1_INK_POINT,
    pub point3: D2D1_INK_POINT,
}
impl ::core::marker::Copy for D2D1_INK_BEZIER_SEGMENT {}
impl ::core::clone::Clone for D2D1_INK_BEZIER_SEGMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_INK_NIB_SHAPE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INK_NIB_SHAPE_ROUND: D2D1_INK_NIB_SHAPE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INK_NIB_SHAPE_SQUARE: D2D1_INK_NIB_SHAPE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INK_NIB_SHAPE_FORCE_DWORD: D2D1_INK_NIB_SHAPE = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_INK_POINT {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}
impl ::core::marker::Copy for D2D1_INK_POINT {}
impl ::core::clone::Clone for D2D1_INK_POINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Foundation_Numerics\"`*"]
#[cfg(feature = "Foundation_Numerics")]
pub struct D2D1_INK_STYLE_PROPERTIES {
    pub nibShape: D2D1_INK_NIB_SHAPE,
    pub nibTransform: super::super::super::Foundation::Numerics::Matrix3x2,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for D2D1_INK_STYLE_PROPERTIES {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for D2D1_INK_STYLE_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_INPUT_DESCRIPTION {
    pub filter: D2D1_FILTER,
    pub levelOfDetailCount: u32,
}
impl ::core::marker::Copy for D2D1_INPUT_DESCRIPTION {}
impl ::core::clone::Clone for D2D1_INPUT_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D2D1_INPUT_ELEMENT_DESC {
    pub semanticName: ::windows_sys::core::PCSTR,
    pub semanticIndex: u32,
    pub format: super::Dxgi::Common::DXGI_FORMAT,
    pub inputSlot: u32,
    pub alignedByteOffset: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D2D1_INPUT_ELEMENT_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D2D1_INPUT_ELEMENT_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_INTERPOLATION_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_INTERPOLATION_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_LINEAR: D2D1_INTERPOLATION_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_CUBIC: D2D1_INTERPOLATION_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR: D2D1_INTERPOLATION_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_ANISOTROPIC: D2D1_INTERPOLATION_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC: D2D1_INTERPOLATION_MODE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_FORCE_DWORD: D2D1_INTERPOLATION_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_INTERPOLATION_MODE_DEFINITION = i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_DEFINITION_NEAREST_NEIGHBOR: D2D1_INTERPOLATION_MODE_DEFINITION = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_DEFINITION_LINEAR: D2D1_INTERPOLATION_MODE_DEFINITION = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_DEFINITION_CUBIC: D2D1_INTERPOLATION_MODE_DEFINITION = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_DEFINITION_MULTI_SAMPLE_LINEAR: D2D1_INTERPOLATION_MODE_DEFINITION = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_DEFINITION_ANISOTROPIC: D2D1_INTERPOLATION_MODE_DEFINITION = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_DEFINITION_HIGH_QUALITY_CUBIC: D2D1_INTERPOLATION_MODE_DEFINITION = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_DEFINITION_FANT: D2D1_INTERPOLATION_MODE_DEFINITION = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_DEFINITION_MIPMAP_LINEAR: D2D1_INTERPOLATION_MODE_DEFINITION = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_LAYER_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LAYER_OPTIONS_NONE: D2D1_LAYER_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LAYER_OPTIONS_INITIALIZE_FOR_CLEARTYPE: D2D1_LAYER_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LAYER_OPTIONS_FORCE_DWORD: D2D1_LAYER_OPTIONS = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_LAYER_OPTIONS1 = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LAYER_OPTIONS1_NONE: D2D1_LAYER_OPTIONS1 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LAYER_OPTIONS1_INITIALIZE_FROM_BACKGROUND: D2D1_LAYER_OPTIONS1 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LAYER_OPTIONS1_IGNORE_ALPHA: D2D1_LAYER_OPTIONS1 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LAYER_OPTIONS1_FORCE_DWORD: D2D1_LAYER_OPTIONS1 = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub struct D2D1_LAYER_PARAMETERS {
    pub contentBounds: Common::D2D_RECT_F,
    pub geometricMask: *mut *mut *mut *mut ID2D1Geometry,
    pub maskAntialiasMode: D2D1_ANTIALIAS_MODE,
    pub maskTransform: super::super::super::Foundation::Numerics::Matrix3x2,
    pub opacity: f32,
    pub opacityBrush: *mut *mut *mut *mut ID2D1Brush,
    pub layerOptions: D2D1_LAYER_OPTIONS,
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::marker::Copy for D2D1_LAYER_PARAMETERS {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::clone::Clone for D2D1_LAYER_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub struct D2D1_LAYER_PARAMETERS1 {
    pub contentBounds: Common::D2D_RECT_F,
    pub geometricMask: *mut *mut *mut *mut ID2D1Geometry,
    pub maskAntialiasMode: D2D1_ANTIALIAS_MODE,
    pub maskTransform: super::super::super::Foundation::Numerics::Matrix3x2,
    pub opacity: f32,
    pub opacityBrush: *mut *mut *mut *mut ID2D1Brush,
    pub layerOptions: D2D1_LAYER_OPTIONS1,
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::marker::Copy for D2D1_LAYER_PARAMETERS1 {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::clone::Clone for D2D1_LAYER_PARAMETERS1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_LINEARTRANSFER_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_RED_Y_INTERCEPT: D2D1_LINEARTRANSFER_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_RED_SLOPE: D2D1_LINEARTRANSFER_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_RED_DISABLE: D2D1_LINEARTRANSFER_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_GREEN_Y_INTERCEPT: D2D1_LINEARTRANSFER_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_GREEN_SLOPE: D2D1_LINEARTRANSFER_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_GREEN_DISABLE: D2D1_LINEARTRANSFER_PROP = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_BLUE_Y_INTERCEPT: D2D1_LINEARTRANSFER_PROP = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_BLUE_SLOPE: D2D1_LINEARTRANSFER_PROP = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_BLUE_DISABLE: D2D1_LINEARTRANSFER_PROP = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_ALPHA_Y_INTERCEPT: D2D1_LINEARTRANSFER_PROP = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_ALPHA_SLOPE: D2D1_LINEARTRANSFER_PROP = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_ALPHA_DISABLE: D2D1_LINEARTRANSFER_PROP = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_CLAMP_OUTPUT: D2D1_LINEARTRANSFER_PROP = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_FORCE_DWORD: D2D1_LINEARTRANSFER_PROP = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
    pub startPoint: Common::D2D_POINT_2F,
    pub endPoint: Common::D2D_POINT_2F,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_LINE_JOIN = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINE_JOIN_MITER: D2D1_LINE_JOIN = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINE_JOIN_BEVEL: D2D1_LINE_JOIN = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINE_JOIN_ROUND: D2D1_LINE_JOIN = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINE_JOIN_MITER_OR_BEVEL: D2D1_LINE_JOIN = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINE_JOIN_FORCE_DWORD: D2D1_LINE_JOIN = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_LOOKUPTABLE3D_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LOOKUPTABLE3D_PROP_LUT: D2D1_LOOKUPTABLE3D_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LOOKUPTABLE3D_PROP_ALPHA_MODE: D2D1_LOOKUPTABLE3D_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LOOKUPTABLE3D_PROP_FORCE_DWORD: D2D1_LOOKUPTABLE3D_PROP = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_MAPPED_RECT {
    pub pitch: u32,
    pub bits: *mut u8,
}
impl ::core::marker::Copy for D2D1_MAPPED_RECT {}
impl ::core::clone::Clone for D2D1_MAPPED_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_MAP_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MAP_OPTIONS_NONE: D2D1_MAP_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MAP_OPTIONS_READ: D2D1_MAP_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MAP_OPTIONS_WRITE: D2D1_MAP_OPTIONS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MAP_OPTIONS_DISCARD: D2D1_MAP_OPTIONS = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MAP_OPTIONS_FORCE_DWORD: D2D1_MAP_OPTIONS = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_MORPHOLOGY_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MORPHOLOGY_MODE_ERODE: D2D1_MORPHOLOGY_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MORPHOLOGY_MODE_DILATE: D2D1_MORPHOLOGY_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MORPHOLOGY_MODE_FORCE_DWORD: D2D1_MORPHOLOGY_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_MORPHOLOGY_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MORPHOLOGY_PROP_MODE: D2D1_MORPHOLOGY_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MORPHOLOGY_PROP_WIDTH: D2D1_MORPHOLOGY_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MORPHOLOGY_PROP_HEIGHT: D2D1_MORPHOLOGY_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MORPHOLOGY_PROP_FORCE_DWORD: D2D1_MORPHOLOGY_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_OPACITYMETADATA_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_OPACITYMETADATA_PROP_INPUT_OPAQUE_RECT: D2D1_OPACITYMETADATA_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_OPACITYMETADATA_PROP_FORCE_DWORD: D2D1_OPACITYMETADATA_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_OPACITY_MASK_CONTENT = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_OPACITY_MASK_CONTENT_GRAPHICS: D2D1_OPACITY_MASK_CONTENT = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_OPACITY_MASK_CONTENT_TEXT_NATURAL: D2D1_OPACITY_MASK_CONTENT = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_OPACITY_MASK_CONTENT_TEXT_GDI_COMPATIBLE: D2D1_OPACITY_MASK_CONTENT = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_OPACITY_MASK_CONTENT_FORCE_DWORD: D2D1_OPACITY_MASK_CONTENT = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_OPACITY_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_OPACITY_PROP_OPACITY: D2D1_OPACITY_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_OPACITY_PROP_FORCE_DWORD: D2D1_OPACITY_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_ORIENTATION = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ORIENTATION_DEFAULT: D2D1_ORIENTATION = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ORIENTATION_FLIP_HORIZONTAL: D2D1_ORIENTATION = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE180: D2D1_ORIENTATION = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE180_FLIP_HORIZONTAL: D2D1_ORIENTATION = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE90_FLIP_HORIZONTAL: D2D1_ORIENTATION = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE270: D2D1_ORIENTATION = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE270_FLIP_HORIZONTAL: D2D1_ORIENTATION = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE90: D2D1_ORIENTATION = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ORIENTATION_FORCE_DWORD: D2D1_ORIENTATION = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_PATCH_EDGE_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PATCH_EDGE_MODE_ALIASED: D2D1_PATCH_EDGE_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PATCH_EDGE_MODE_ANTIALIASED: D2D1_PATCH_EDGE_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PATCH_EDGE_MODE_ALIASED_INFLATED: D2D1_PATCH_EDGE_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PATCH_EDGE_MODE_FORCE_DWORD: D2D1_PATCH_EDGE_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_PIXEL_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PIXEL_OPTIONS_NONE: D2D1_PIXEL_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PIXEL_OPTIONS_TRIVIAL_SAMPLING: D2D1_PIXEL_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PIXEL_OPTIONS_FORCE_DWORD: D2D1_PIXEL_OPTIONS = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_POINTDIFFUSE_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_PROP_LIGHT_POSITION: D2D1_POINTDIFFUSE_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_PROP_DIFFUSE_CONSTANT: D2D1_POINTDIFFUSE_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_PROP_SURFACE_SCALE: D2D1_POINTDIFFUSE_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_PROP_COLOR: D2D1_POINTDIFFUSE_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_PROP_KERNEL_UNIT_LENGTH: D2D1_POINTDIFFUSE_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_PROP_SCALE_MODE: D2D1_POINTDIFFUSE_PROP = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_PROP_FORCE_DWORD: D2D1_POINTDIFFUSE_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_POINTDIFFUSE_SCALE_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_POINTDIFFUSE_SCALE_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_SCALE_MODE_LINEAR: D2D1_POINTDIFFUSE_SCALE_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_SCALE_MODE_CUBIC: D2D1_POINTDIFFUSE_SCALE_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_POINTDIFFUSE_SCALE_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_SCALE_MODE_ANISOTROPIC: D2D1_POINTDIFFUSE_SCALE_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_SCALE_MODE_HIGH_QUALITY_CUBIC: D2D1_POINTDIFFUSE_SCALE_MODE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_SCALE_MODE_FORCE_DWORD: D2D1_POINTDIFFUSE_SCALE_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_POINTSPECULAR_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_PROP_LIGHT_POSITION: D2D1_POINTSPECULAR_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_PROP_SPECULAR_EXPONENT: D2D1_POINTSPECULAR_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_PROP_SPECULAR_CONSTANT: D2D1_POINTSPECULAR_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_PROP_SURFACE_SCALE: D2D1_POINTSPECULAR_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_PROP_COLOR: D2D1_POINTSPECULAR_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_PROP_KERNEL_UNIT_LENGTH: D2D1_POINTSPECULAR_PROP = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_PROP_SCALE_MODE: D2D1_POINTSPECULAR_PROP = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_PROP_FORCE_DWORD: D2D1_POINTSPECULAR_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_POINTSPECULAR_SCALE_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_POINTSPECULAR_SCALE_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_SCALE_MODE_LINEAR: D2D1_POINTSPECULAR_SCALE_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_SCALE_MODE_CUBIC: D2D1_POINTSPECULAR_SCALE_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_POINTSPECULAR_SCALE_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_SCALE_MODE_ANISOTROPIC: D2D1_POINTSPECULAR_SCALE_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_SCALE_MODE_HIGH_QUALITY_CUBIC: D2D1_POINTSPECULAR_SCALE_MODE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_SCALE_MODE_FORCE_DWORD: D2D1_POINTSPECULAR_SCALE_MODE = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_POINT_DESCRIPTION {
    pub point: Common::D2D_POINT_2F,
    pub unitTangentVector: Common::D2D_POINT_2F,
    pub endSegment: u32,
    pub endFigure: u32,
    pub lengthToEndSegment: f32,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_POINT_DESCRIPTION {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_POINT_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_POSTERIZE_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POSTERIZE_PROP_RED_VALUE_COUNT: D2D1_POSTERIZE_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POSTERIZE_PROP_GREEN_VALUE_COUNT: D2D1_POSTERIZE_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POSTERIZE_PROP_BLUE_VALUE_COUNT: D2D1_POSTERIZE_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POSTERIZE_PROP_FORCE_DWORD: D2D1_POSTERIZE_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_PRESENT_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRESENT_OPTIONS_NONE: D2D1_PRESENT_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRESENT_OPTIONS_RETAIN_CONTENTS: D2D1_PRESENT_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRESENT_OPTIONS_IMMEDIATELY: D2D1_PRESENT_OPTIONS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRESENT_OPTIONS_FORCE_DWORD: D2D1_PRESENT_OPTIONS = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_PRIMITIVE_BLEND = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRIMITIVE_BLEND_SOURCE_OVER: D2D1_PRIMITIVE_BLEND = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRIMITIVE_BLEND_COPY: D2D1_PRIMITIVE_BLEND = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRIMITIVE_BLEND_MIN: D2D1_PRIMITIVE_BLEND = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRIMITIVE_BLEND_ADD: D2D1_PRIMITIVE_BLEND = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRIMITIVE_BLEND_MAX: D2D1_PRIMITIVE_BLEND = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRIMITIVE_BLEND_FORCE_DWORD: D2D1_PRIMITIVE_BLEND = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_PRINT_CONTROL_PROPERTIES {
    pub fontSubset: D2D1_PRINT_FONT_SUBSET_MODE,
    pub rasterDPI: f32,
    pub colorSpace: D2D1_COLOR_SPACE,
}
impl ::core::marker::Copy for D2D1_PRINT_CONTROL_PROPERTIES {}
impl ::core::clone::Clone for D2D1_PRINT_CONTROL_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_PRINT_FONT_SUBSET_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRINT_FONT_SUBSET_MODE_DEFAULT: D2D1_PRINT_FONT_SUBSET_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRINT_FONT_SUBSET_MODE_EACHPAGE: D2D1_PRINT_FONT_SUBSET_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRINT_FONT_SUBSET_MODE_NONE: D2D1_PRINT_FONT_SUBSET_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRINT_FONT_SUBSET_MODE_FORCE_DWORD: D2D1_PRINT_FONT_SUBSET_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_PROPERTY = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_CLSID: D2D1_PROPERTY = 2147483648u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_DISPLAYNAME: D2D1_PROPERTY = 2147483649u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_AUTHOR: D2D1_PROPERTY = 2147483650u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_CATEGORY: D2D1_PROPERTY = 2147483651u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_DESCRIPTION: D2D1_PROPERTY = 2147483652u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_INPUTS: D2D1_PROPERTY = 2147483653u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_CACHED: D2D1_PROPERTY = 2147483654u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_PRECISION: D2D1_PROPERTY = 2147483655u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_MIN_INPUTS: D2D1_PROPERTY = 2147483656u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_MAX_INPUTS: D2D1_PROPERTY = 2147483657u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_FORCE_DWORD: D2D1_PROPERTY = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_PROPERTY_BINDING {
    pub propertyName: ::windows_sys::core::PCWSTR,
    pub setFunction: PD2D1_PROPERTY_SET_FUNCTION,
    pub getFunction: PD2D1_PROPERTY_GET_FUNCTION,
}
impl ::core::marker::Copy for D2D1_PROPERTY_BINDING {}
impl ::core::clone::Clone for D2D1_PROPERTY_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_PROPERTY_TYPE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_UNKNOWN: D2D1_PROPERTY_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_STRING: D2D1_PROPERTY_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_BOOL: D2D1_PROPERTY_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_UINT32: D2D1_PROPERTY_TYPE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_INT32: D2D1_PROPERTY_TYPE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_FLOAT: D2D1_PROPERTY_TYPE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_VECTOR2: D2D1_PROPERTY_TYPE = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_VECTOR3: D2D1_PROPERTY_TYPE = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_VECTOR4: D2D1_PROPERTY_TYPE = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_BLOB: D2D1_PROPERTY_TYPE = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_IUNKNOWN: D2D1_PROPERTY_TYPE = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_ENUM: D2D1_PROPERTY_TYPE = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_ARRAY: D2D1_PROPERTY_TYPE = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_CLSID: D2D1_PROPERTY_TYPE = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_MATRIX_3X2: D2D1_PROPERTY_TYPE = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_MATRIX_4X3: D2D1_PROPERTY_TYPE = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_MATRIX_4X4: D2D1_PROPERTY_TYPE = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_MATRIX_5X4: D2D1_PROPERTY_TYPE = 17u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_COLOR_CONTEXT: D2D1_PROPERTY_TYPE = 18u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_FORCE_DWORD: D2D1_PROPERTY_TYPE = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_QUADRATIC_BEZIER_SEGMENT {
    pub point1: Common::D2D_POINT_2F,
    pub point2: Common::D2D_POINT_2F,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_QUADRATIC_BEZIER_SEGMENT {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_QUADRATIC_BEZIER_SEGMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
    pub center: Common::D2D_POINT_2F,
    pub gradientOriginOffset: Common::D2D_POINT_2F,
    pub radiusX: f32,
    pub radiusY: f32,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_RENDERING_CONTROLS {
    pub bufferPrecision: D2D1_BUFFER_PRECISION,
    pub tileSize: Common::D2D_SIZE_U,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_RENDERING_CONTROLS {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_RENDERING_CONTROLS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_RENDERING_PRIORITY = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDERING_PRIORITY_NORMAL: D2D1_RENDERING_PRIORITY = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDERING_PRIORITY_LOW: D2D1_RENDERING_PRIORITY = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDERING_PRIORITY_FORCE_DWORD: D2D1_RENDERING_PRIORITY = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D2D1_RENDER_TARGET_PROPERTIES {
    pub r#type: D2D1_RENDER_TARGET_TYPE,
    pub pixelFormat: Common::D2D1_PIXEL_FORMAT,
    pub dpiX: f32,
    pub dpiY: f32,
    pub usage: D2D1_RENDER_TARGET_USAGE,
    pub minLevel: D2D1_FEATURE_LEVEL,
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D2D1_RENDER_TARGET_PROPERTIES {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D2D1_RENDER_TARGET_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_RENDER_TARGET_TYPE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDER_TARGET_TYPE_DEFAULT: D2D1_RENDER_TARGET_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDER_TARGET_TYPE_SOFTWARE: D2D1_RENDER_TARGET_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDER_TARGET_TYPE_HARDWARE: D2D1_RENDER_TARGET_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDER_TARGET_TYPE_FORCE_DWORD: D2D1_RENDER_TARGET_TYPE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_RENDER_TARGET_USAGE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDER_TARGET_USAGE_NONE: D2D1_RENDER_TARGET_USAGE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDER_TARGET_USAGE_FORCE_BITMAP_REMOTING: D2D1_RENDER_TARGET_USAGE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDER_TARGET_USAGE_GDI_COMPATIBLE: D2D1_RENDER_TARGET_USAGE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDER_TARGET_USAGE_FORCE_DWORD: D2D1_RENDER_TARGET_USAGE = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_RESOURCE_TEXTURE_PROPERTIES {
    pub extents: *const u32,
    pub dimensions: u32,
    pub bufferPrecision: D2D1_BUFFER_PRECISION,
    pub channelDepth: D2D1_CHANNEL_DEPTH,
    pub filter: D2D1_FILTER,
    pub extendModes: *const D2D1_EXTEND_MODE,
}
impl ::core::marker::Copy for D2D1_RESOURCE_TEXTURE_PROPERTIES {}
impl ::core::clone::Clone for D2D1_RESOURCE_TEXTURE_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE_HUE_SATURATION_VALUE: D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE_HUE_SATURATION_LIGHTNESS: D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE_FORCE_DWORD: D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_RGBTOHUE_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RGBTOHUE_PROP_OUTPUT_COLOR_SPACE: D2D1_RGBTOHUE_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RGBTOHUE_PROP_FORCE_DWORD: D2D1_RGBTOHUE_PROP = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_ROUNDED_RECT {
    pub rect: Common::D2D_RECT_F,
    pub radiusX: f32,
    pub radiusY: f32,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_ROUNDED_RECT {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_ROUNDED_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SATURATION_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SATURATION_PROP_SATURATION: D2D1_SATURATION_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SATURATION_PROP_FORCE_DWORD: D2D1_SATURATION_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SCALE_INTERPOLATION_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_SCALE_INTERPOLATION_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_INTERPOLATION_MODE_LINEAR: D2D1_SCALE_INTERPOLATION_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_INTERPOLATION_MODE_CUBIC: D2D1_SCALE_INTERPOLATION_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR: D2D1_SCALE_INTERPOLATION_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_INTERPOLATION_MODE_ANISOTROPIC: D2D1_SCALE_INTERPOLATION_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC: D2D1_SCALE_INTERPOLATION_MODE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_INTERPOLATION_MODE_FORCE_DWORD: D2D1_SCALE_INTERPOLATION_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SCALE_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_PROP_SCALE: D2D1_SCALE_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_PROP_CENTER_POINT: D2D1_SCALE_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_PROP_INTERPOLATION_MODE: D2D1_SCALE_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_PROP_BORDER_MODE: D2D1_SCALE_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_PROP_SHARPNESS: D2D1_SCALE_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_PROP_FORCE_DWORD: D2D1_SCALE_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCENE_REFERRED_SDR_WHITE_LEVEL: f32 = 80f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SEPIA_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SEPIA_PROP_INTENSITY: D2D1_SEPIA_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SEPIA_PROP_ALPHA_MODE: D2D1_SEPIA_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SEPIA_PROP_FORCE_DWORD: D2D1_SEPIA_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SHADOW_OPTIMIZATION = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHADOW_OPTIMIZATION_SPEED: D2D1_SHADOW_OPTIMIZATION = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHADOW_OPTIMIZATION_BALANCED: D2D1_SHADOW_OPTIMIZATION = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHADOW_OPTIMIZATION_QUALITY: D2D1_SHADOW_OPTIMIZATION = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHADOW_OPTIMIZATION_FORCE_DWORD: D2D1_SHADOW_OPTIMIZATION = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SHADOW_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHADOW_PROP_BLUR_STANDARD_DEVIATION: D2D1_SHADOW_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHADOW_PROP_COLOR: D2D1_SHADOW_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHADOW_PROP_OPTIMIZATION: D2D1_SHADOW_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHADOW_PROP_FORCE_DWORD: D2D1_SHADOW_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SHARPEN_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHARPEN_PROP_SHARPNESS: D2D1_SHARPEN_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHARPEN_PROP_THRESHOLD: D2D1_SHARPEN_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHARPEN_PROP_FORCE_DWORD: D2D1_SHARPEN_PROP = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_SIMPLE_COLOR_PROFILE {
    pub redPrimary: Common::D2D_POINT_2F,
    pub greenPrimary: Common::D2D_POINT_2F,
    pub bluePrimary: Common::D2D_POINT_2F,
    pub whitePointXZ: Common::D2D_POINT_2F,
    pub gamma: D2D1_GAMMA1,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_SIMPLE_COLOR_PROFILE {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_SIMPLE_COLOR_PROFILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SPOTDIFFUSE_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_PROP_LIGHT_POSITION: D2D1_SPOTDIFFUSE_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_PROP_POINTS_AT: D2D1_SPOTDIFFUSE_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_PROP_FOCUS: D2D1_SPOTDIFFUSE_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_PROP_LIMITING_CONE_ANGLE: D2D1_SPOTDIFFUSE_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_PROP_DIFFUSE_CONSTANT: D2D1_SPOTDIFFUSE_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_PROP_SURFACE_SCALE: D2D1_SPOTDIFFUSE_PROP = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_PROP_COLOR: D2D1_SPOTDIFFUSE_PROP = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_PROP_KERNEL_UNIT_LENGTH: D2D1_SPOTDIFFUSE_PROP = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_PROP_SCALE_MODE: D2D1_SPOTDIFFUSE_PROP = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_PROP_FORCE_DWORD: D2D1_SPOTDIFFUSE_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SPOTDIFFUSE_SCALE_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_SPOTDIFFUSE_SCALE_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_SCALE_MODE_LINEAR: D2D1_SPOTDIFFUSE_SCALE_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_SCALE_MODE_CUBIC: D2D1_SPOTDIFFUSE_SCALE_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_SPOTDIFFUSE_SCALE_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_SCALE_MODE_ANISOTROPIC: D2D1_SPOTDIFFUSE_SCALE_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_SCALE_MODE_HIGH_QUALITY_CUBIC: D2D1_SPOTDIFFUSE_SCALE_MODE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_SCALE_MODE_FORCE_DWORD: D2D1_SPOTDIFFUSE_SCALE_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SPOTSPECULAR_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_LIGHT_POSITION: D2D1_SPOTSPECULAR_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_POINTS_AT: D2D1_SPOTSPECULAR_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_FOCUS: D2D1_SPOTSPECULAR_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_LIMITING_CONE_ANGLE: D2D1_SPOTSPECULAR_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_SPECULAR_EXPONENT: D2D1_SPOTSPECULAR_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_SPECULAR_CONSTANT: D2D1_SPOTSPECULAR_PROP = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_SURFACE_SCALE: D2D1_SPOTSPECULAR_PROP = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_COLOR: D2D1_SPOTSPECULAR_PROP = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_KERNEL_UNIT_LENGTH: D2D1_SPOTSPECULAR_PROP = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_SCALE_MODE: D2D1_SPOTSPECULAR_PROP = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_FORCE_DWORD: D2D1_SPOTSPECULAR_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SPOTSPECULAR_SCALE_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_SPOTSPECULAR_SCALE_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_SCALE_MODE_LINEAR: D2D1_SPOTSPECULAR_SCALE_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_SCALE_MODE_CUBIC: D2D1_SPOTSPECULAR_SCALE_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_SPOTSPECULAR_SCALE_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_SCALE_MODE_ANISOTROPIC: D2D1_SPOTSPECULAR_SCALE_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_SCALE_MODE_HIGH_QUALITY_CUBIC: D2D1_SPOTSPECULAR_SCALE_MODE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_SCALE_MODE_FORCE_DWORD: D2D1_SPOTSPECULAR_SCALE_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SPRITE_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPRITE_OPTIONS_NONE: D2D1_SPRITE_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPRITE_OPTIONS_CLAMP_TO_SOURCE_RECTANGLE: D2D1_SPRITE_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPRITE_OPTIONS_FORCE_DWORD: D2D1_SPRITE_OPTIONS = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_STRAIGHTEN_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STRAIGHTEN_PROP_ANGLE: D2D1_STRAIGHTEN_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STRAIGHTEN_PROP_MAINTAIN_SIZE: D2D1_STRAIGHTEN_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STRAIGHTEN_PROP_SCALE_MODE: D2D1_STRAIGHTEN_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STRAIGHTEN_PROP_FORCE_DWORD: D2D1_STRAIGHTEN_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_STRAIGHTEN_SCALE_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STRAIGHTEN_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_STRAIGHTEN_SCALE_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STRAIGHTEN_SCALE_MODE_LINEAR: D2D1_STRAIGHTEN_SCALE_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STRAIGHTEN_SCALE_MODE_CUBIC: D2D1_STRAIGHTEN_SCALE_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STRAIGHTEN_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_STRAIGHTEN_SCALE_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STRAIGHTEN_SCALE_MODE_ANISOTROPIC: D2D1_STRAIGHTEN_SCALE_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STRAIGHTEN_SCALE_MODE_FORCE_DWORD: D2D1_STRAIGHTEN_SCALE_MODE = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_STROKE_STYLE_PROPERTIES {
    pub startCap: D2D1_CAP_STYLE,
    pub endCap: D2D1_CAP_STYLE,
    pub dashCap: D2D1_CAP_STYLE,
    pub lineJoin: D2D1_LINE_JOIN,
    pub miterLimit: f32,
    pub dashStyle: D2D1_DASH_STYLE,
    pub dashOffset: f32,
}
impl ::core::marker::Copy for D2D1_STROKE_STYLE_PROPERTIES {}
impl ::core::clone::Clone for D2D1_STROKE_STYLE_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_STROKE_STYLE_PROPERTIES1 {
    pub startCap: D2D1_CAP_STYLE,
    pub endCap: D2D1_CAP_STYLE,
    pub dashCap: D2D1_CAP_STYLE,
    pub lineJoin: D2D1_LINE_JOIN,
    pub miterLimit: f32,
    pub dashStyle: D2D1_DASH_STYLE,
    pub dashOffset: f32,
    pub transformType: D2D1_STROKE_TRANSFORM_TYPE,
}
impl ::core::marker::Copy for D2D1_STROKE_STYLE_PROPERTIES1 {}
impl ::core::clone::Clone for D2D1_STROKE_STYLE_PROPERTIES1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_STROKE_TRANSFORM_TYPE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STROKE_TRANSFORM_TYPE_NORMAL: D2D1_STROKE_TRANSFORM_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STROKE_TRANSFORM_TYPE_FIXED: D2D1_STROKE_TRANSFORM_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STROKE_TRANSFORM_TYPE_HAIRLINE: D2D1_STROKE_TRANSFORM_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STROKE_TRANSFORM_TYPE_FORCE_DWORD: D2D1_STROKE_TRANSFORM_TYPE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SUBPROPERTY = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SUBPROPERTY_DISPLAYNAME: D2D1_SUBPROPERTY = 2147483648u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SUBPROPERTY_ISREADONLY: D2D1_SUBPROPERTY = 2147483649u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SUBPROPERTY_MIN: D2D1_SUBPROPERTY = 2147483650u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SUBPROPERTY_MAX: D2D1_SUBPROPERTY = 2147483651u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SUBPROPERTY_DEFAULT: D2D1_SUBPROPERTY = 2147483652u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SUBPROPERTY_FIELDS: D2D1_SUBPROPERTY = 2147483653u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SUBPROPERTY_INDEX: D2D1_SUBPROPERTY = 2147483654u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SUBPROPERTY_FORCE_DWORD: D2D1_SUBPROPERTY = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SVG_ASPECT_ALIGN = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_NONE: D2D1_SVG_ASPECT_ALIGN = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_X_MIN_Y_MIN: D2D1_SVG_ASPECT_ALIGN = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_X_MID_Y_MIN: D2D1_SVG_ASPECT_ALIGN = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_X_MAX_Y_MIN: D2D1_SVG_ASPECT_ALIGN = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_X_MIN_Y_MID: D2D1_SVG_ASPECT_ALIGN = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_X_MID_Y_MID: D2D1_SVG_ASPECT_ALIGN = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_X_MAX_Y_MID: D2D1_SVG_ASPECT_ALIGN = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_X_MIN_Y_MAX: D2D1_SVG_ASPECT_ALIGN = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_X_MID_Y_MAX: D2D1_SVG_ASPECT_ALIGN = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_X_MAX_Y_MAX: D2D1_SVG_ASPECT_ALIGN = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_FORCE_DWORD: D2D1_SVG_ASPECT_ALIGN = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SVG_ASPECT_SCALING = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_SCALING_MEET: D2D1_SVG_ASPECT_SCALING = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_SCALING_SLICE: D2D1_SVG_ASPECT_SCALING = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_SCALING_FORCE_DWORD: D2D1_SVG_ASPECT_SCALING = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SVG_ATTRIBUTE_POD_TYPE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_FLOAT: D2D1_SVG_ATTRIBUTE_POD_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_COLOR: D2D1_SVG_ATTRIBUTE_POD_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_FILL_MODE: D2D1_SVG_ATTRIBUTE_POD_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_DISPLAY: D2D1_SVG_ATTRIBUTE_POD_TYPE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_OVERFLOW: D2D1_SVG_ATTRIBUTE_POD_TYPE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_LINE_CAP: D2D1_SVG_ATTRIBUTE_POD_TYPE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_LINE_JOIN: D2D1_SVG_ATTRIBUTE_POD_TYPE = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_VISIBILITY: D2D1_SVG_ATTRIBUTE_POD_TYPE = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_MATRIX: D2D1_SVG_ATTRIBUTE_POD_TYPE = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_UNIT_TYPE: D2D1_SVG_ATTRIBUTE_POD_TYPE = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_EXTEND_MODE: D2D1_SVG_ATTRIBUTE_POD_TYPE = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_PRESERVE_ASPECT_RATIO: D2D1_SVG_ATTRIBUTE_POD_TYPE = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_VIEWBOX: D2D1_SVG_ATTRIBUTE_POD_TYPE = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_LENGTH: D2D1_SVG_ATTRIBUTE_POD_TYPE = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_FORCE_DWORD: D2D1_SVG_ATTRIBUTE_POD_TYPE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SVG_ATTRIBUTE_STRING_TYPE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_STRING_TYPE_SVG: D2D1_SVG_ATTRIBUTE_STRING_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_STRING_TYPE_ID: D2D1_SVG_ATTRIBUTE_STRING_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_STRING_TYPE_FORCE_DWORD: D2D1_SVG_ATTRIBUTE_STRING_TYPE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SVG_DISPLAY = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_DISPLAY_INLINE: D2D1_SVG_DISPLAY = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_DISPLAY_NONE: D2D1_SVG_DISPLAY = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_DISPLAY_FORCE_DWORD: D2D1_SVG_DISPLAY = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_SVG_LENGTH {
    pub value: f32,
    pub units: D2D1_SVG_LENGTH_UNITS,
}
impl ::core::marker::Copy for D2D1_SVG_LENGTH {}
impl ::core::clone::Clone for D2D1_SVG_LENGTH {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SVG_LENGTH_UNITS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LENGTH_UNITS_NUMBER: D2D1_SVG_LENGTH_UNITS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LENGTH_UNITS_PERCENTAGE: D2D1_SVG_LENGTH_UNITS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LENGTH_UNITS_FORCE_DWORD: D2D1_SVG_LENGTH_UNITS = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SVG_LINE_CAP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LINE_CAP_BUTT: D2D1_SVG_LINE_CAP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LINE_CAP_SQUARE: D2D1_SVG_LINE_CAP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LINE_CAP_ROUND: D2D1_SVG_LINE_CAP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LINE_CAP_FORCE_DWORD: D2D1_SVG_LINE_CAP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SVG_LINE_JOIN = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LINE_JOIN_BEVEL: D2D1_SVG_LINE_JOIN = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LINE_JOIN_MITER: D2D1_SVG_LINE_JOIN = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LINE_JOIN_ROUND: D2D1_SVG_LINE_JOIN = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LINE_JOIN_FORCE_DWORD: D2D1_SVG_LINE_JOIN = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SVG_OVERFLOW = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_OVERFLOW_VISIBLE: D2D1_SVG_OVERFLOW = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_OVERFLOW_HIDDEN: D2D1_SVG_OVERFLOW = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_OVERFLOW_FORCE_DWORD: D2D1_SVG_OVERFLOW = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SVG_PAINT_TYPE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PAINT_TYPE_NONE: D2D1_SVG_PAINT_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PAINT_TYPE_COLOR: D2D1_SVG_PAINT_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PAINT_TYPE_CURRENT_COLOR: D2D1_SVG_PAINT_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PAINT_TYPE_URI: D2D1_SVG_PAINT_TYPE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PAINT_TYPE_URI_NONE: D2D1_SVG_PAINT_TYPE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PAINT_TYPE_URI_COLOR: D2D1_SVG_PAINT_TYPE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PAINT_TYPE_URI_CURRENT_COLOR: D2D1_SVG_PAINT_TYPE = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PAINT_TYPE_FORCE_DWORD: D2D1_SVG_PAINT_TYPE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SVG_PATH_COMMAND = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_CLOSE_PATH: D2D1_SVG_PATH_COMMAND = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_MOVE_ABSOLUTE: D2D1_SVG_PATH_COMMAND = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_MOVE_RELATIVE: D2D1_SVG_PATH_COMMAND = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_LINE_ABSOLUTE: D2D1_SVG_PATH_COMMAND = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_LINE_RELATIVE: D2D1_SVG_PATH_COMMAND = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_CUBIC_ABSOLUTE: D2D1_SVG_PATH_COMMAND = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_CUBIC_RELATIVE: D2D1_SVG_PATH_COMMAND = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_QUADRADIC_ABSOLUTE: D2D1_SVG_PATH_COMMAND = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_QUADRADIC_RELATIVE: D2D1_SVG_PATH_COMMAND = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_ARC_ABSOLUTE: D2D1_SVG_PATH_COMMAND = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_ARC_RELATIVE: D2D1_SVG_PATH_COMMAND = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_HORIZONTAL_ABSOLUTE: D2D1_SVG_PATH_COMMAND = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_HORIZONTAL_RELATIVE: D2D1_SVG_PATH_COMMAND = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_VERTICAL_ABSOLUTE: D2D1_SVG_PATH_COMMAND = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_VERTICAL_RELATIVE: D2D1_SVG_PATH_COMMAND = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_CUBIC_SMOOTH_ABSOLUTE: D2D1_SVG_PATH_COMMAND = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_CUBIC_SMOOTH_RELATIVE: D2D1_SVG_PATH_COMMAND = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_QUADRADIC_SMOOTH_ABSOLUTE: D2D1_SVG_PATH_COMMAND = 17u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_QUADRADIC_SMOOTH_RELATIVE: D2D1_SVG_PATH_COMMAND = 18u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_FORCE_DWORD: D2D1_SVG_PATH_COMMAND = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D2D1_SVG_PRESERVE_ASPECT_RATIO {
    pub defer: super::super::Foundation::BOOL,
    pub align: D2D1_SVG_ASPECT_ALIGN,
    pub meetOrSlice: D2D1_SVG_ASPECT_SCALING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D2D1_SVG_PRESERVE_ASPECT_RATIO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D2D1_SVG_PRESERVE_ASPECT_RATIO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SVG_UNIT_TYPE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_UNIT_TYPE_USER_SPACE_ON_USE: D2D1_SVG_UNIT_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_UNIT_TYPE_OBJECT_BOUNDING_BOX: D2D1_SVG_UNIT_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_UNIT_TYPE_FORCE_DWORD: D2D1_SVG_UNIT_TYPE = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_SVG_VIEWBOX {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
impl ::core::marker::Copy for D2D1_SVG_VIEWBOX {}
impl ::core::clone::Clone for D2D1_SVG_VIEWBOX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SVG_VISIBILITY = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_VISIBILITY_VISIBLE: D2D1_SVG_VISIBILITY = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_VISIBILITY_HIDDEN: D2D1_SVG_VISIBILITY = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_VISIBILITY_FORCE_DWORD: D2D1_SVG_VISIBILITY = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SWEEP_DIRECTION = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SWEEP_DIRECTION_COUNTER_CLOCKWISE: D2D1_SWEEP_DIRECTION = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SWEEP_DIRECTION_CLOCKWISE: D2D1_SWEEP_DIRECTION = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SWEEP_DIRECTION_FORCE_DWORD: D2D1_SWEEP_DIRECTION = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_TABLETRANSFER_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TABLETRANSFER_PROP_RED_TABLE: D2D1_TABLETRANSFER_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TABLETRANSFER_PROP_RED_DISABLE: D2D1_TABLETRANSFER_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TABLETRANSFER_PROP_GREEN_TABLE: D2D1_TABLETRANSFER_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TABLETRANSFER_PROP_GREEN_DISABLE: D2D1_TABLETRANSFER_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TABLETRANSFER_PROP_BLUE_TABLE: D2D1_TABLETRANSFER_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TABLETRANSFER_PROP_BLUE_DISABLE: D2D1_TABLETRANSFER_PROP = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TABLETRANSFER_PROP_ALPHA_TABLE: D2D1_TABLETRANSFER_PROP = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TABLETRANSFER_PROP_ALPHA_DISABLE: D2D1_TABLETRANSFER_PROP = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TABLETRANSFER_PROP_CLAMP_OUTPUT: D2D1_TABLETRANSFER_PROP = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TABLETRANSFER_PROP_FORCE_DWORD: D2D1_TABLETRANSFER_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_TEMPERATUREANDTINT_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TEMPERATUREANDTINT_PROP_TEMPERATURE: D2D1_TEMPERATUREANDTINT_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TEMPERATUREANDTINT_PROP_TINT: D2D1_TEMPERATUREANDTINT_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TEMPERATUREANDTINT_PROP_FORCE_DWORD: D2D1_TEMPERATUREANDTINT_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_TEXT_ANTIALIAS_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TEXT_ANTIALIAS_MODE_DEFAULT: D2D1_TEXT_ANTIALIAS_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TEXT_ANTIALIAS_MODE_CLEARTYPE: D2D1_TEXT_ANTIALIAS_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE: D2D1_TEXT_ANTIALIAS_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TEXT_ANTIALIAS_MODE_ALIASED: D2D1_TEXT_ANTIALIAS_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TEXT_ANTIALIAS_MODE_FORCE_DWORD: D2D1_TEXT_ANTIALIAS_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_THREADING_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_THREADING_MODE_SINGLE_THREADED: D2D1_THREADING_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_THREADING_MODE_MULTI_THREADED: D2D1_THREADING_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_THREADING_MODE_FORCE_DWORD: D2D1_THREADING_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_TILE_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TILE_PROP_RECT: D2D1_TILE_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TILE_PROP_FORCE_DWORD: D2D1_TILE_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_TINT_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TINT_PROP_COLOR: D2D1_TINT_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TINT_PROP_CLAMP_OUTPUT: D2D1_TINT_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TINT_PROP_FORCE_DWORD: D2D1_TINT_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS_NONE: D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS_DISABLE_DPI_SCALE: D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS_FORCE_DWORD: D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES {
    pub orientation: D2D1_ORIENTATION,
    pub scaleX: f32,
    pub scaleY: f32,
    pub interpolationMode: D2D1_INTERPOLATION_MODE,
    pub options: D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS,
}
impl ::core::marker::Copy for D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES {}
impl ::core::clone::Clone for D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_TRIANGLE {
    pub point1: Common::D2D_POINT_2F,
    pub point2: Common::D2D_POINT_2F,
    pub point3: Common::D2D_POINT_2F,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_TRIANGLE {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_TRIANGLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_TURBULENCE_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TURBULENCE_PROP_OFFSET: D2D1_TURBULENCE_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TURBULENCE_PROP_SIZE: D2D1_TURBULENCE_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TURBULENCE_PROP_BASE_FREQUENCY: D2D1_TURBULENCE_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TURBULENCE_PROP_NUM_OCTAVES: D2D1_TURBULENCE_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TURBULENCE_PROP_SEED: D2D1_TURBULENCE_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TURBULENCE_PROP_NOISE: D2D1_TURBULENCE_PROP = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TURBULENCE_PROP_STITCHABLE: D2D1_TURBULENCE_PROP = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TURBULENCE_PROP_FORCE_DWORD: D2D1_TURBULENCE_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_UNIT_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_UNIT_MODE_DIPS: D2D1_UNIT_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_UNIT_MODE_PIXELS: D2D1_UNIT_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_UNIT_MODE_FORCE_DWORD: D2D1_UNIT_MODE = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_VERTEX_BUFFER_PROPERTIES {
    pub inputCount: u32,
    pub usage: D2D1_VERTEX_USAGE,
    pub data: *const u8,
    pub byteWidth: u32,
}
impl ::core::marker::Copy for D2D1_VERTEX_BUFFER_PROPERTIES {}
impl ::core::clone::Clone for D2D1_VERTEX_BUFFER_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_VERTEX_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VERTEX_OPTIONS_NONE: D2D1_VERTEX_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VERTEX_OPTIONS_DO_NOT_CLEAR: D2D1_VERTEX_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VERTEX_OPTIONS_USE_DEPTH_BUFFER: D2D1_VERTEX_OPTIONS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VERTEX_OPTIONS_ASSUME_NO_OVERLAP: D2D1_VERTEX_OPTIONS = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VERTEX_OPTIONS_FORCE_DWORD: D2D1_VERTEX_OPTIONS = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_VERTEX_RANGE {
    pub startVertex: u32,
    pub vertexCount: u32,
}
impl ::core::marker::Copy for D2D1_VERTEX_RANGE {}
impl ::core::clone::Clone for D2D1_VERTEX_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_VERTEX_USAGE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VERTEX_USAGE_STATIC: D2D1_VERTEX_USAGE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VERTEX_USAGE_DYNAMIC: D2D1_VERTEX_USAGE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VERTEX_USAGE_FORCE_DWORD: D2D1_VERTEX_USAGE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_VIGNETTE_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VIGNETTE_PROP_COLOR: D2D1_VIGNETTE_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VIGNETTE_PROP_TRANSITION_SIZE: D2D1_VIGNETTE_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VIGNETTE_PROP_STRENGTH: D2D1_VIGNETTE_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VIGNETTE_PROP_FORCE_DWORD: D2D1_VIGNETTE_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_WHITELEVELADJUSTMENT_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_WHITELEVELADJUSTMENT_PROP_INPUT_WHITE_LEVEL: D2D1_WHITELEVELADJUSTMENT_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_WHITELEVELADJUSTMENT_PROP_OUTPUT_WHITE_LEVEL: D2D1_WHITELEVELADJUSTMENT_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_WHITELEVELADJUSTMENT_PROP_FORCE_DWORD: D2D1_WHITELEVELADJUSTMENT_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_WINDOW_STATE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_WINDOW_STATE_NONE: D2D1_WINDOW_STATE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_WINDOW_STATE_OCCLUDED: D2D1_WINDOW_STATE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_WINDOW_STATE_FORCE_DWORD: D2D1_WINDOW_STATE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_YCBCR_CHROMA_SUBSAMPLING = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_CHROMA_SUBSAMPLING_AUTO: D2D1_YCBCR_CHROMA_SUBSAMPLING = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_CHROMA_SUBSAMPLING_420: D2D1_YCBCR_CHROMA_SUBSAMPLING = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_CHROMA_SUBSAMPLING_422: D2D1_YCBCR_CHROMA_SUBSAMPLING = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_CHROMA_SUBSAMPLING_444: D2D1_YCBCR_CHROMA_SUBSAMPLING = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_CHROMA_SUBSAMPLING_440: D2D1_YCBCR_CHROMA_SUBSAMPLING = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_CHROMA_SUBSAMPLING_FORCE_DWORD: D2D1_YCBCR_CHROMA_SUBSAMPLING = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_YCBCR_INTERPOLATION_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_YCBCR_INTERPOLATION_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_INTERPOLATION_MODE_LINEAR: D2D1_YCBCR_INTERPOLATION_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_INTERPOLATION_MODE_CUBIC: D2D1_YCBCR_INTERPOLATION_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR: D2D1_YCBCR_INTERPOLATION_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_INTERPOLATION_MODE_ANISOTROPIC: D2D1_YCBCR_INTERPOLATION_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC: D2D1_YCBCR_INTERPOLATION_MODE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_INTERPOLATION_MODE_FORCE_DWORD: D2D1_YCBCR_INTERPOLATION_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_YCBCR_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_PROP_CHROMA_SUBSAMPLING: D2D1_YCBCR_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_PROP_TRANSFORM_MATRIX: D2D1_YCBCR_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_PROP_INTERPOLATION_MODE: D2D1_YCBCR_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_PROP_FORCE_DWORD: D2D1_YCBCR_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const FACILITY_D2D: u32 = 2201u32;
#[repr(C)]
pub struct ID2D1AnalysisTransform {
    pub base__: ::windows_sys::core::IUnknown,
    pub ProcessAnalysisResults: unsafe extern "system" fn(this: *mut *mut Self, analysisdata: *const u8, analysisdatacount: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1AnalysisTransform {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 56220720, data2: 38374, data3: 17768, data4: [144, 85, 39, 114, 13, 19, 14, 147] };
}
#[repr(C)]
pub struct ID2D1Bitmap {
    pub base__: ID2D1Image,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Common::D2D_SIZE_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetSize: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetPixelSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Common::D2D_SIZE_U),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetPixelSize: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetPixelFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Common::D2D1_PIXEL_FORMAT),
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    GetPixelFormat: usize,
    pub GetDpi: unsafe extern "system" fn(this: *mut *mut Self, dpix: *mut f32, dpiy: *mut f32),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CopyFromBitmap: unsafe extern "system" fn(this: *mut *mut Self, destpoint: *const Common::D2D_POINT_2U, bitmap: *mut ::core::ffi::c_void, srcrect: *const Common::D2D_RECT_U) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CopyFromBitmap: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CopyFromRenderTarget: unsafe extern "system" fn(this: *mut *mut Self, destpoint: *const Common::D2D_POINT_2U, rendertarget: *mut ::core::ffi::c_void, srcrect: *const Common::D2D_RECT_U) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CopyFromRenderTarget: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CopyFromMemory: unsafe extern "system" fn(this: *mut *mut Self, dstrect: *const Common::D2D_RECT_U, srcdata: *const ::core::ffi::c_void, pitch: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CopyFromMemory: usize,
}
impl ::windows_sys::core::Interface for ID2D1Bitmap {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2720620631, data2: 59970, data3: 16537, data4: [152, 59, 83, 159, 182, 80, 84, 38] };
}
#[repr(C)]
pub struct ID2D1Bitmap1 {
    pub base__: ID2D1Bitmap,
    pub GetColorContext: unsafe extern "system" fn(this: *mut *mut Self, colorcontext: *mut *mut ::core::ffi::c_void),
    pub GetOptions: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_BITMAP_OPTIONS,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub GetSurface: unsafe extern "system" fn(this: *mut *mut Self, dxgisurface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    GetSurface: usize,
    pub Map: unsafe extern "system" fn(this: *mut *mut Self, options: D2D1_MAP_OPTIONS, mappedrect: *mut D2D1_MAPPED_RECT) -> ::windows_sys::core::HRESULT,
    pub Unmap: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1Bitmap1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2828576844, data2: 14451, data3: 17800, data4: [176, 139, 235, 191, 151, 141, 240, 65] };
}
#[repr(C)]
pub struct ID2D1BitmapBrush {
    pub base__: ID2D1Brush,
    pub SetExtendModeX: unsafe extern "system" fn(this: *mut *mut Self, extendmodex: D2D1_EXTEND_MODE),
    pub SetExtendModeY: unsafe extern "system" fn(this: *mut *mut Self, extendmodey: D2D1_EXTEND_MODE),
    pub SetInterpolationMode: unsafe extern "system" fn(this: *mut *mut Self, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE),
    pub SetBitmap: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut ::core::ffi::c_void),
    pub GetExtendModeX: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_EXTEND_MODE,
    pub GetExtendModeY: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_EXTEND_MODE,
    pub GetInterpolationMode: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_BITMAP_INTERPOLATION_MODE,
    pub GetBitmap: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut *mut ::core::ffi::c_void),
}
impl ::windows_sys::core::Interface for ID2D1BitmapBrush {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 752420522, data2: 4834, data3: 4572, data4: [159, 237, 0, 17, 67, 160, 85, 249] };
}
#[repr(C)]
pub struct ID2D1BitmapBrush1 {
    pub base__: ID2D1BitmapBrush,
    pub SetInterpolationMode1: unsafe extern "system" fn(this: *mut *mut Self, interpolationmode: D2D1_INTERPOLATION_MODE),
    pub GetInterpolationMode1: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_INTERPOLATION_MODE,
}
impl ::windows_sys::core::Interface for ID2D1BitmapBrush1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1093941843, data2: 58394, data3: 18850, data4: [145, 205, 33, 121, 59, 187, 98, 229] };
}
#[repr(C)]
pub struct ID2D1BitmapRenderTarget {
    pub base__: ID2D1RenderTarget,
    pub GetBitmap: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1BitmapRenderTarget {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 752420501, data2: 4834, data3: 4572, data4: [159, 237, 0, 17, 67, 160, 85, 249] };
}
#[repr(C)]
pub struct ID2D1BlendTransform {
    pub base__: ID2D1ConcreteTransform,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, description: *const D2D1_BLEND_DESCRIPTION),
    pub GetDescription: unsafe extern "system" fn(this: *mut *mut Self, description: *mut D2D1_BLEND_DESCRIPTION),
}
impl ::windows_sys::core::Interface for ID2D1BlendTransform {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1672219442, data2: 47684, data3: 17679, data4: [136, 6, 127, 76, 161, 255, 47, 27] };
}
#[repr(C)]
pub struct ID2D1BorderTransform {
    pub base__: ID2D1ConcreteTransform,
    pub SetExtendModeX: unsafe extern "system" fn(this: *mut *mut Self, extendmode: D2D1_EXTEND_MODE),
    pub SetExtendModeY: unsafe extern "system" fn(this: *mut *mut Self, extendmode: D2D1_EXTEND_MODE),
    pub GetExtendModeX: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_EXTEND_MODE,
    pub GetExtendModeY: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_EXTEND_MODE,
}
impl ::windows_sys::core::Interface for ID2D1BorderTransform {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1234727772, data2: 14873, data3: 18236, data4: [151, 129, 101, 104, 71, 227, 163, 71] };
}
#[repr(C)]
pub struct ID2D1BoundsAdjustmentTransform {
    pub base__: ID2D1TransformNode,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOutputBounds: unsafe extern "system" fn(this: *mut *mut Self, outputbounds: *const super::super::Foundation::RECT),
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOutputBounds: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetOutputBounds: unsafe extern "system" fn(this: *mut *mut Self, outputbounds: *mut super::super::Foundation::RECT),
    #[cfg(not(feature = "Win32_Foundation"))]
    GetOutputBounds: usize,
}
impl ::windows_sys::core::Interface for ID2D1BoundsAdjustmentTransform {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2432119522, data2: 20626, data3: 17926, data4: [168, 25, 134, 81, 151, 11, 172, 205] };
}
#[repr(C)]
pub struct ID2D1Brush {
    pub base__: ID2D1Resource,
    pub SetOpacity: unsafe extern "system" fn(this: *mut *mut Self, opacity: f32),
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransform: unsafe extern "system" fn(this: *mut *mut Self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2),
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransform: usize,
    pub GetOpacity: unsafe extern "system" fn(this: *mut *mut Self) -> f32,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetTransform: unsafe extern "system" fn(this: *mut *mut Self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2),
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetTransform: usize,
}
impl ::windows_sys::core::Interface for ID2D1Brush {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 752420520, data2: 4834, data3: 4572, data4: [159, 237, 0, 17, 67, 160, 85, 249] };
}
#[repr(C)]
pub struct ID2D1ColorContext {
    pub base__: ID2D1Resource,
    pub GetColorSpace: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_COLOR_SPACE,
    pub GetProfileSize: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetProfile: unsafe extern "system" fn(this: *mut *mut Self, profile: *mut u8, profilesize: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1ColorContext {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 474489019, data2: 22385, data3: 17688, data4: [165, 129, 47, 228, 221, 14, 198, 87] };
}
#[repr(C)]
pub struct ID2D1ColorContext1 {
    pub base__: ID2D1ColorContext,
    pub GetColorContextType: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_COLOR_CONTEXT_TYPE,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDXGIColorSpace: unsafe extern "system" fn(this: *mut *mut Self) -> super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDXGIColorSpace: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetSimpleColorProfile: unsafe extern "system" fn(this: *mut *mut Self, simpleprofile: *mut D2D1_SIMPLE_COLOR_PROFILE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetSimpleColorProfile: usize,
}
impl ::windows_sys::core::Interface for ID2D1ColorContext1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 448014453, data2: 50559, data3: 19433, data4: [189, 133, 156, 215, 141, 111, 85, 238] };
}
#[repr(C)]
pub struct ID2D1CommandList {
    pub base__: ID2D1Image,
    pub Stream: unsafe extern "system" fn(this: *mut *mut Self, sink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1CommandList {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3035843097, data2: 9091, data3: 19830, data4: [148, 246, 236, 52, 54, 87, 195, 220] };
}
#[repr(C)]
pub struct ID2D1CommandSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub BeginDraw: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub EndDraw: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetAntialiasMode: unsafe extern "system" fn(this: *mut *mut Self, antialiasmode: D2D1_ANTIALIAS_MODE) -> ::windows_sys::core::HRESULT,
    pub SetTags: unsafe extern "system" fn(this: *mut *mut Self, tag1: u64, tag2: u64) -> ::windows_sys::core::HRESULT,
    pub SetTextAntialiasMode: unsafe extern "system" fn(this: *mut *mut Self, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub SetTextRenderingParams: unsafe extern "system" fn(this: *mut *mut Self, textrenderingparams: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_DirectWrite"))]
    SetTextRenderingParams: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransform: unsafe extern "system" fn(this: *mut *mut Self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransform: usize,
    pub SetPrimitiveBlend: unsafe extern "system" fn(this: *mut *mut Self, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows_sys::core::HRESULT,
    pub SetUnitMode: unsafe extern "system" fn(this: *mut *mut Self, unitmode: D2D1_UNIT_MODE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self, color: *const Common::D2D1_COLOR_F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    Clear: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub DrawGlyphRun: unsafe extern "system" fn(this: *mut *mut Self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION, foregroundbrush: *mut ::core::ffi::c_void, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    DrawGlyphRun: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawLine: unsafe extern "system" fn(this: *mut *mut Self, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawLine: usize,
    pub DrawGeometry: unsafe extern "system" fn(this: *mut *mut Self, geometry: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawRectangle: unsafe extern "system" fn(this: *mut *mut Self, rect: *const Common::D2D_RECT_F, brush: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawRectangle: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawBitmap: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut ::core::ffi::c_void, destinationrectangle: *const Common::D2D_RECT_F, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: *const Common::D2D_RECT_F, perspectivetransform: *const Common::D2D_MATRIX_4X4_F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawBitmap: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawImage: unsafe extern "system" fn(this: *mut *mut Self, image: *mut ::core::ffi::c_void, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawImage: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawGdiMetafile: unsafe extern "system" fn(this: *mut *mut Self, gdimetafile: *mut ::core::ffi::c_void, targetoffset: *const Common::D2D_POINT_2F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawGdiMetafile: usize,
    pub FillMesh: unsafe extern "system" fn(this: *mut *mut Self, mesh: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub FillOpacityMask: unsafe extern "system" fn(this: *mut *mut Self, opacitymask: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    FillOpacityMask: usize,
    pub FillGeometry: unsafe extern "system" fn(this: *mut *mut Self, geometry: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void, opacitybrush: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub FillRectangle: unsafe extern "system" fn(this: *mut *mut Self, rect: *const Common::D2D_RECT_F, brush: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    FillRectangle: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub PushAxisAlignedClip: unsafe extern "system" fn(this: *mut *mut Self, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    PushAxisAlignedClip: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub PushLayer: unsafe extern "system" fn(this: *mut *mut Self, layerparameters1: *const D2D1_LAYER_PARAMETERS1, layer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    PushLayer: usize,
    pub PopAxisAlignedClip: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub PopLayer: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1CommandSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1423411594, data2: 41057, data3: 16551, data4: [190, 199, 228, 101, 188, 186, 44, 79] };
}
#[repr(C)]
pub struct ID2D1CommandSink1 {
    pub base__: ID2D1CommandSink,
    pub SetPrimitiveBlend1: unsafe extern "system" fn(this: *mut *mut Self, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1CommandSink1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2662819837, data2: 17001, data3: 17511, data4: [184, 194, 235, 48, 203, 48, 87, 67] };
}
#[repr(C)]
pub struct ID2D1CommandSink2 {
    pub base__: ID2D1CommandSink1,
    pub DrawInk: unsafe extern "system" fn(this: *mut *mut Self, ink: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void, inkstyle: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DrawGradientMesh: unsafe extern "system" fn(this: *mut *mut Self, gradientmesh: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawGdiMetafile2: unsafe extern "system" fn(this: *mut *mut Self, gdimetafile: *mut ::core::ffi::c_void, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawGdiMetafile2: usize,
}
impl ::windows_sys::core::Interface for ID2D1CommandSink2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1001079822, data2: 16766, data3: 18399, data4: [162, 226, 188, 11, 230, 160, 9, 22] };
}
#[repr(C)]
pub struct ID2D1CommandSink3 {
    pub base__: ID2D1CommandSink2,
    pub DrawSpriteBatch: unsafe extern "system" fn(this: *mut *mut Self, spritebatch: *mut ::core::ffi::c_void, startindex: u32, spritecount: u32, bitmap: *mut ::core::ffi::c_void, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1CommandSink3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 403149109, data2: 19699, data3: 18536, data4: [188, 142, 6, 6, 126, 109, 36, 45] };
}
#[repr(C)]
pub struct ID2D1CommandSink4 {
    pub base__: ID2D1CommandSink3,
    pub SetPrimitiveBlend2: unsafe extern "system" fn(this: *mut *mut Self, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1CommandSink4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3347735833, data2: 16598, data3: 16920, data4: [178, 222, 190, 238, 183, 68, 187, 62] };
}
#[repr(C)]
pub struct ID2D1CommandSink5 {
    pub base__: ID2D1CommandSink4,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub BlendImage: unsafe extern "system" fn(this: *mut *mut Self, image: *mut ::core::ffi::c_void, blendmode: Common::D2D1_BLEND_MODE, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    BlendImage: usize,
}
impl ::windows_sys::core::Interface for ID2D1CommandSink5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1883757862, data2: 45543, data3: 17575, data4: [149, 154, 131, 73, 226, 20, 79, 168] };
}
#[repr(C)]
pub struct ID2D1ComputeInfo {
    pub base__: ID2D1RenderInfo,
    pub SetComputeShaderConstantBuffer: unsafe extern "system" fn(this: *mut *mut Self, buffer: *const u8, buffercount: u32) -> ::windows_sys::core::HRESULT,
    pub SetComputeShader: unsafe extern "system" fn(this: *mut *mut Self, shaderid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetResourceTexture: unsafe extern "system" fn(this: *mut *mut Self, textureindex: u32, resourcetexture: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1ComputeInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1436070219, data2: 40919, data3: 18615, data4: [155, 219, 143, 9, 100, 235, 56, 188] };
}
#[repr(C)]
pub struct ID2D1ComputeTransform {
    pub base__: ID2D1Transform,
    pub SetComputeInfo: unsafe extern "system" fn(this: *mut *mut Self, computeinfo: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CalculateThreadgroups: unsafe extern "system" fn(this: *mut *mut Self, outputrect: *const super::super::Foundation::RECT, dimensionx: *mut u32, dimensiony: *mut u32, dimensionz: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CalculateThreadgroups: usize,
}
impl ::windows_sys::core::Interface for ID2D1ComputeTransform {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 226842428, data2: 483, data3: 20349, data4: [191, 217, 13, 96, 96, 139, 243, 195] };
}
#[repr(C)]
pub struct ID2D1ConcreteTransform {
    pub base__: ID2D1TransformNode,
    pub SetOutputBuffer: unsafe extern "system" fn(this: *mut *mut Self, bufferprecision: D2D1_BUFFER_PRECISION, channeldepth: D2D1_CHANNEL_DEPTH) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCached: unsafe extern "system" fn(this: *mut *mut Self, iscached: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCached: usize,
}
impl ::windows_sys::core::Interface for ID2D1ConcreteTransform {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 444177802, data2: 27127, data3: 20044, data4: [159, 237, 67, 124, 204, 102, 132, 204] };
}
#[repr(C)]
pub struct ID2D1DCRenderTarget {
    pub base__: ID2D1RenderTarget,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub BindDC: unsafe extern "system" fn(this: *mut *mut Self, hdc: super::Gdi::HDC, psubrect: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    BindDC: usize,
}
impl ::windows_sys::core::Interface for ID2D1DCRenderTarget {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 475118692, data2: 56929, data3: 18173, data4: [152, 153, 99, 165, 216, 240, 57, 80] };
}
#[repr(C)]
pub struct ID2D1Device {
    pub base__: ID2D1Resource,
    pub CreateDeviceContext: unsafe extern "system" fn(this: *mut *mut Self, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
    pub CreatePrintControl: unsafe extern "system" fn(this: *mut *mut Self, wicfactory: *mut ::core::ffi::c_void, documenttarget: *mut ::core::ffi::c_void, printcontrolproperties: *const D2D1_PRINT_CONTROL_PROPERTIES, printcontrol: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing")))]
    CreatePrintControl: usize,
    pub SetMaximumTextureMemory: unsafe extern "system" fn(this: *mut *mut Self, maximuminbytes: u64),
    pub GetMaximumTextureMemory: unsafe extern "system" fn(this: *mut *mut Self) -> u64,
    pub ClearResources: unsafe extern "system" fn(this: *mut *mut Self, millisecondssinceuse: u32),
}
impl ::windows_sys::core::Interface for ID2D1Device {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1205688157, data2: 44037, data3: 19677, data4: [128, 73, 155, 2, 205, 22, 244, 76] };
}
#[repr(C)]
pub struct ID2D1Device1 {
    pub base__: ID2D1Device,
    pub GetRenderingPriority: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_RENDERING_PRIORITY,
    pub SetRenderingPriority: unsafe extern "system" fn(this: *mut *mut Self, renderingpriority: D2D1_RENDERING_PRIORITY),
    pub CreateDeviceContext2: unsafe extern "system" fn(this: *mut *mut Self, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext1: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1Device1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3524749537, data2: 9124, data3: 18467, data4: [161, 75, 124, 62, 186, 133, 214, 88] };
}
#[repr(C)]
pub struct ID2D1Device2 {
    pub base__: ID2D1Device1,
    pub CreateDeviceContext3: unsafe extern "system" fn(this: *mut *mut Self, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext2: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FlushDeviceContexts: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub GetDxgiDevice: unsafe extern "system" fn(this: *mut *mut Self, dxgidevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    GetDxgiDevice: usize,
}
impl ::windows_sys::core::Interface for ID2D1Device2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2755949281, data2: 36347, data3: 20064, data4: [132, 146, 110, 40, 97, 201, 202, 139] };
}
#[repr(C)]
pub struct ID2D1Device3 {
    pub base__: ID2D1Device2,
    pub CreateDeviceContext4: unsafe extern "system" fn(this: *mut *mut Self, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext3: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1Device3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2234458247, data2: 32812, data3: 16439, data4: [171, 96, 255, 46, 126, 230, 252, 1] };
}
#[repr(C)]
pub struct ID2D1Device4 {
    pub base__: ID2D1Device3,
    pub CreateDeviceContext5: unsafe extern "system" fn(this: *mut *mut Self, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext4: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetMaximumColorGlyphCacheMemory: unsafe extern "system" fn(this: *mut *mut Self, maximuminbytes: u64),
    pub GetMaximumColorGlyphCacheMemory: unsafe extern "system" fn(this: *mut *mut Self) -> u64,
}
impl ::windows_sys::core::Interface for ID2D1Device4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3619533145, data2: 22147, data3: 19014, data4: [188, 156, 114, 220, 114, 11, 133, 139] };
}
#[repr(C)]
pub struct ID2D1Device5 {
    pub base__: ID2D1Device4,
    pub CreateDeviceContext6: unsafe extern "system" fn(this: *mut *mut Self, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext5: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1Device5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3579551908, data2: 25605, data3: 18068, data4: [174, 245, 8, 238, 26, 67, 88, 180] };
}
#[repr(C)]
pub struct ID2D1Device6 {
    pub base__: ID2D1Device5,
    pub CreateDeviceContext7: unsafe extern "system" fn(this: *mut *mut Self, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext6: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1Device6 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2080307476, data2: 11637, data3: 19373, data4: [190, 135, 225, 141, 219, 7, 123, 109] };
}
#[repr(C)]
pub struct ID2D1DeviceContext {
    pub base__: ID2D1RenderTarget,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateBitmap2: unsafe extern "system" fn(this: *mut *mut Self, size: Common::D2D_SIZE_U, sourcedata: *const ::core::ffi::c_void, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1, bitmap: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateBitmap2: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub CreateBitmapFromWicBitmap2: unsafe extern "system" fn(this: *mut *mut Self, wicbitmapsource: *mut ::core::ffi::c_void, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1, bitmap: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging")))]
    CreateBitmapFromWicBitmap2: usize,
    pub CreateColorContext: unsafe extern "system" fn(this: *mut *mut Self, space: D2D1_COLOR_SPACE, profile: *const u8, profilesize: u32, colorcontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateColorContextFromFilename: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR, colorcontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub CreateColorContextFromWicColorContext: unsafe extern "system" fn(this: *mut *mut Self, wiccolorcontext: *mut ::core::ffi::c_void, colorcontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    CreateColorContextFromWicColorContext: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateBitmapFromDxgiSurface: unsafe extern "system" fn(this: *mut *mut Self, surface: *mut ::core::ffi::c_void, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1, bitmap: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateBitmapFromDxgiSurface: usize,
    pub CreateEffect: unsafe extern "system" fn(this: *mut *mut Self, effectid: *const ::windows_sys::core::GUID, effect: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreateGradientStopCollection2: unsafe extern "system" fn(this: *mut *mut Self, straightalphagradientstops: *const D2D1_GRADIENT_STOP, straightalphagradientstopscount: u32, preinterpolationspace: D2D1_COLOR_SPACE, postinterpolationspace: D2D1_COLOR_SPACE, bufferprecision: D2D1_BUFFER_PRECISION, extendmode: D2D1_EXTEND_MODE, colorinterpolationmode: D2D1_COLOR_INTERPOLATION_MODE, gradientstopcollection1: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreateGradientStopCollection2: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub CreateImageBrush: unsafe extern "system" fn(this: *mut *mut Self, image: *mut ::core::ffi::c_void, imagebrushproperties: *const D2D1_IMAGE_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, imagebrush: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    CreateImageBrush: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateBitmapBrush2: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut ::core::ffi::c_void, bitmapbrushproperties: *const D2D1_BITMAP_BRUSH_PROPERTIES1, brushproperties: *const D2D1_BRUSH_PROPERTIES, bitmapbrush: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateBitmapBrush2: usize,
    pub CreateCommandList: unsafe extern "system" fn(this: *mut *mut Self, commandlist: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub IsDxgiFormatSupported: unsafe extern "system" fn(this: *mut *mut Self, format: super::Dxgi::Common::DXGI_FORMAT) -> super::super::Foundation::BOOL,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    IsDxgiFormatSupported: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsBufferPrecisionSupported: unsafe extern "system" fn(this: *mut *mut Self, bufferprecision: D2D1_BUFFER_PRECISION) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsBufferPrecisionSupported: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetImageLocalBounds: unsafe extern "system" fn(this: *mut *mut Self, image: *mut ::core::ffi::c_void, localbounds: *mut Common::D2D_RECT_F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetImageLocalBounds: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetImageWorldBounds: unsafe extern "system" fn(this: *mut *mut Self, image: *mut ::core::ffi::c_void, worldbounds: *mut Common::D2D_RECT_F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetImageWorldBounds: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub GetGlyphRunWorldBounds: unsafe extern "system" fn(this: *mut *mut Self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE, bounds: *mut Common::D2D_RECT_F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    GetGlyphRunWorldBounds: usize,
    pub GetDevice: unsafe extern "system" fn(this: *mut *mut Self, device: *mut *mut ::core::ffi::c_void),
    pub SetTarget: unsafe extern "system" fn(this: *mut *mut Self, image: *mut ::core::ffi::c_void),
    pub GetTarget: unsafe extern "system" fn(this: *mut *mut Self, image: *mut *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetRenderingControls: unsafe extern "system" fn(this: *mut *mut Self, renderingcontrols: *const D2D1_RENDERING_CONTROLS),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetRenderingControls: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetRenderingControls: unsafe extern "system" fn(this: *mut *mut Self, renderingcontrols: *mut D2D1_RENDERING_CONTROLS),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetRenderingControls: usize,
    pub SetPrimitiveBlend: unsafe extern "system" fn(this: *mut *mut Self, primitiveblend: D2D1_PRIMITIVE_BLEND),
    pub GetPrimitiveBlend: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_PRIMITIVE_BLEND,
    pub SetUnitMode: unsafe extern "system" fn(this: *mut *mut Self, unitmode: D2D1_UNIT_MODE),
    pub GetUnitMode: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_UNIT_MODE,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub DrawGlyphRun2: unsafe extern "system" fn(this: *mut *mut Self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION, foregroundbrush: *mut ::core::ffi::c_void, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    DrawGlyphRun2: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawImage: unsafe extern "system" fn(this: *mut *mut Self, image: *mut ::core::ffi::c_void, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawImage: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawGdiMetafile: unsafe extern "system" fn(this: *mut *mut Self, gdimetafile: *mut ::core::ffi::c_void, targetoffset: *const Common::D2D_POINT_2F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawGdiMetafile: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawBitmap2: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut ::core::ffi::c_void, destinationrectangle: *const Common::D2D_RECT_F, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: *const Common::D2D_RECT_F, perspectivetransform: *const Common::D2D_MATRIX_4X4_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawBitmap2: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub PushLayer2: unsafe extern "system" fn(this: *mut *mut Self, layerparameters: *const D2D1_LAYER_PARAMETERS1, layer: *mut ::core::ffi::c_void),
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    PushLayer2: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub InvalidateEffectInputRectangle: unsafe extern "system" fn(this: *mut *mut Self, effect: *mut ::core::ffi::c_void, input: u32, inputrectangle: *const Common::D2D_RECT_F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    InvalidateEffectInputRectangle: usize,
    pub GetEffectInvalidRectangleCount: unsafe extern "system" fn(this: *mut *mut Self, effect: *mut ::core::ffi::c_void, rectanglecount: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetEffectInvalidRectangles: unsafe extern "system" fn(this: *mut *mut Self, effect: *mut ::core::ffi::c_void, rectangles: *mut Common::D2D_RECT_F, rectanglescount: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetEffectInvalidRectangles: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetEffectRequiredInputRectangles: unsafe extern "system" fn(this: *mut *mut Self, rendereffect: *mut ::core::ffi::c_void, renderimagerectangle: *const Common::D2D_RECT_F, inputdescriptions: *const D2D1_EFFECT_INPUT_DESCRIPTION, requiredinputrects: *mut Common::D2D_RECT_F, inputcount: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetEffectRequiredInputRectangles: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub FillOpacityMask2: unsafe extern "system" fn(this: *mut *mut Self, opacitymask: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    FillOpacityMask2: usize,
}
impl ::windows_sys::core::Interface for ID2D1DeviceContext {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3908566650, data2: 6428, data3: 18029, data4: [173, 149, 151, 86, 120, 189, 169, 152] };
}
#[repr(C)]
pub struct ID2D1DeviceContext1 {
    pub base__: ID2D1DeviceContext,
    pub CreateFilledGeometryRealization: unsafe extern "system" fn(this: *mut *mut Self, geometry: *mut ::core::ffi::c_void, flatteningtolerance: f32, geometryrealization: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateStrokedGeometryRealization: unsafe extern "system" fn(this: *mut *mut Self, geometry: *mut ::core::ffi::c_void, flatteningtolerance: f32, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void, geometryrealization: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DrawGeometryRealization: unsafe extern "system" fn(this: *mut *mut Self, geometryrealization: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void),
}
impl ::windows_sys::core::Interface for ID2D1DeviceContext1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3548338148, data2: 26888, data3: 17823, data4: [161, 153, 231, 47, 36, 247, 153, 135] };
}
#[repr(C)]
pub struct ID2D1DeviceContext2 {
    pub base__: ID2D1DeviceContext1,
    pub CreateInk: unsafe extern "system" fn(this: *mut *mut Self, startpoint: *const D2D1_INK_POINT, ink: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateInkStyle: unsafe extern "system" fn(this: *mut *mut Self, inkstyleproperties: *const D2D1_INK_STYLE_PROPERTIES, inkstyle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateInkStyle: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreateGradientMesh: unsafe extern "system" fn(this: *mut *mut Self, patches: *const D2D1_GRADIENT_MESH_PATCH, patchescount: u32, gradientmesh: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreateGradientMesh: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Imaging"))]
    pub CreateImageSourceFromWic: unsafe extern "system" fn(this: *mut *mut Self, wicbitmapsource: *mut ::core::ffi::c_void, loadingoptions: D2D1_IMAGE_SOURCE_LOADING_OPTIONS, alphamode: Common::D2D1_ALPHA_MODE, imagesource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Imaging")))]
    CreateImageSourceFromWic: usize,
    pub CreateLookupTable3D: unsafe extern "system" fn(this: *mut *mut Self, precision: D2D1_BUFFER_PRECISION, extents: *const u32, data: *const u8, datacount: u32, strides: *const u32, lookuptable: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateImageSourceFromDxgi: unsafe extern "system" fn(this: *mut *mut Self, surfaces: *const *mut ::core::ffi::c_void, surfacecount: u32, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, options: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS, imagesource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateImageSourceFromDxgi: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetGradientMeshWorldBounds: unsafe extern "system" fn(this: *mut *mut Self, gradientmesh: *mut ::core::ffi::c_void, pbounds: *mut Common::D2D_RECT_F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetGradientMeshWorldBounds: usize,
    pub DrawInk: unsafe extern "system" fn(this: *mut *mut Self, ink: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void, inkstyle: *mut ::core::ffi::c_void),
    pub DrawGradientMesh: unsafe extern "system" fn(this: *mut *mut Self, gradientmesh: *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawGdiMetafile2: unsafe extern "system" fn(this: *mut *mut Self, gdimetafile: *mut ::core::ffi::c_void, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawGdiMetafile2: usize,
    pub CreateTransformedImageSource: unsafe extern "system" fn(this: *mut *mut Self, imagesource: *mut ::core::ffi::c_void, properties: *const D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES, transformedimagesource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1DeviceContext2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 961455779, data2: 3124, data3: 17185, data4: [149, 11, 108, 162, 15, 11, 230, 199] };
}
#[repr(C)]
pub struct ID2D1DeviceContext3 {
    pub base__: ID2D1DeviceContext2,
    pub CreateSpriteBatch: unsafe extern "system" fn(this: *mut *mut Self, spritebatch: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DrawSpriteBatch: unsafe extern "system" fn(this: *mut *mut Self, spritebatch: *mut ::core::ffi::c_void, startindex: u32, spritecount: u32, bitmap: *mut ::core::ffi::c_void, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS),
}
impl ::windows_sys::core::Interface for ID2D1DeviceContext3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 593130646, data2: 33617, data3: 16716, data4: [188, 212, 102, 114, 171, 45, 142, 0] };
}
#[repr(C)]
pub struct ID2D1DeviceContext4 {
    pub base__: ID2D1DeviceContext3,
    pub CreateSvgGlyphStyle: unsafe extern "system" fn(this: *mut *mut Self, svgglyphstyle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub DrawText2: unsafe extern "system" fn(this: *mut *mut Self, string: ::windows_sys::core::PCWSTR, stringlength: u32, textformat: *mut ::core::ffi::c_void, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: *mut ::core::ffi::c_void, svgglyphstyle: *mut ::core::ffi::c_void, colorpaletteindex: u32, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE),
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    DrawText2: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub DrawTextLayout2: unsafe extern "system" fn(this: *mut *mut Self, origin: Common::D2D_POINT_2F, textlayout: *mut ::core::ffi::c_void, defaultfillbrush: *mut ::core::ffi::c_void, svgglyphstyle: *mut ::core::ffi::c_void, colorpaletteindex: u32, options: D2D1_DRAW_TEXT_OPTIONS),
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    DrawTextLayout2: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub DrawColorBitmapGlyphRun: unsafe extern "system" fn(this: *mut *mut Self, glyphimageformat: super::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE, bitmapsnapoption: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    DrawColorBitmapGlyphRun: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub DrawSvgGlyphRun: unsafe extern "system" fn(this: *mut *mut Self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, defaultfillbrush: *mut ::core::ffi::c_void, svgglyphstyle: *mut ::core::ffi::c_void, colorpaletteindex: u32, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    DrawSvgGlyphRun: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub GetColorBitmapGlyphImage: unsafe extern "system" fn(this: *mut *mut Self, glyphimageformat: super::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS, glyphorigin: Common::D2D_POINT_2F, fontface: *mut ::core::ffi::c_void, fontemsize: f32, glyphindex: u16, issideways: super::super::Foundation::BOOL, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, dpix: f32, dpiy: f32, glyphtransform: *mut super::super::super::Foundation::Numerics::Matrix3x2, glyphimage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    GetColorBitmapGlyphImage: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub GetSvgGlyphImage: unsafe extern "system" fn(this: *mut *mut Self, glyphorigin: Common::D2D_POINT_2F, fontface: *mut ::core::ffi::c_void, fontemsize: f32, glyphindex: u16, issideways: super::super::Foundation::BOOL, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, defaultfillbrush: *mut ::core::ffi::c_void, svgglyphstyle: *mut ::core::ffi::c_void, colorpaletteindex: u32, glyphtransform: *mut super::super::super::Foundation::Numerics::Matrix3x2, glyphimage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    GetSvgGlyphImage: usize,
}
impl ::windows_sys::core::Interface for ID2D1DeviceContext4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2353166385, data2: 15760, data3: 17526, data4: [182, 71, 196, 250, 227, 73, 228, 219] };
}
#[repr(C)]
pub struct ID2D1DeviceContext5 {
    pub base__: ID2D1DeviceContext4,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com"))]
    pub CreateSvgDocument: unsafe extern "system" fn(this: *mut *mut Self, inputxmlstream: *mut ::core::ffi::c_void, viewportsize: Common::D2D_SIZE_F, svgdocument: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com")))]
    CreateSvgDocument: usize,
    pub DrawSvgDocument: unsafe extern "system" fn(this: *mut *mut Self, svgdocument: *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateColorContextFromDxgiColorSpace: unsafe extern "system" fn(this: *mut *mut Self, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, colorcontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateColorContextFromDxgiColorSpace: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreateColorContextFromSimpleColorProfile: unsafe extern "system" fn(this: *mut *mut Self, simpleprofile: *const D2D1_SIMPLE_COLOR_PROFILE, colorcontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreateColorContextFromSimpleColorProfile: usize,
}
impl ::windows_sys::core::Interface for ID2D1DeviceContext5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2016858696, data2: 26828, data3: 19958, data4: [185, 232, 222, 153, 27, 246, 46, 183] };
}
#[repr(C)]
pub struct ID2D1DeviceContext6 {
    pub base__: ID2D1DeviceContext5,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub BlendImage: unsafe extern "system" fn(this: *mut *mut Self, image: *mut ::core::ffi::c_void, blendmode: Common::D2D1_BLEND_MODE, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    BlendImage: usize,
}
impl ::windows_sys::core::Interface for ID2D1DeviceContext6 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2556395063, data2: 20176, data3: 18969, data4: [152, 163, 21, 176, 237, 253, 227, 6] };
}
#[repr(C)]
pub struct ID2D1DrawInfo {
    pub base__: ID2D1RenderInfo,
    pub SetPixelShaderConstantBuffer: unsafe extern "system" fn(this: *mut *mut Self, buffer: *const u8, buffercount: u32) -> ::windows_sys::core::HRESULT,
    pub SetResourceTexture: unsafe extern "system" fn(this: *mut *mut Self, textureindex: u32, resourcetexture: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetVertexShaderConstantBuffer: unsafe extern "system" fn(this: *mut *mut Self, buffer: *const u8, buffercount: u32) -> ::windows_sys::core::HRESULT,
    pub SetPixelShader: unsafe extern "system" fn(this: *mut *mut Self, shaderid: *const ::windows_sys::core::GUID, pixeloptions: D2D1_PIXEL_OPTIONS) -> ::windows_sys::core::HRESULT,
    pub SetVertexProcessing: unsafe extern "system" fn(this: *mut *mut Self, vertexbuffer: *mut ::core::ffi::c_void, vertexoptions: D2D1_VERTEX_OPTIONS, blenddescription: *const D2D1_BLEND_DESCRIPTION, vertexrange: *const D2D1_VERTEX_RANGE, vertexshader: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1DrawInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1765598770, data2: 32559, data3: 17886, data4: [147, 254, 24, 216, 139, 55, 170, 33] };
}
#[repr(C)]
pub struct ID2D1DrawTransform {
    pub base__: ID2D1Transform,
    pub SetDrawInfo: unsafe extern "system" fn(this: *mut *mut Self, drawinfo: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1DrawTransform {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 918543542, data2: 38713, data3: 17245, data4: [163, 13, 166, 83, 190, 255, 106, 111] };
}
#[repr(C)]
pub struct ID2D1DrawingStateBlock {
    pub base__: ID2D1Resource,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetDescription: unsafe extern "system" fn(this: *mut *mut Self, statedescription: *mut D2D1_DRAWING_STATE_DESCRIPTION),
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetDescription: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, statedescription: *const D2D1_DRAWING_STATE_DESCRIPTION),
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetDescription: usize,
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub SetTextRenderingParams: unsafe extern "system" fn(this: *mut *mut Self, textrenderingparams: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_DirectWrite"))]
    SetTextRenderingParams: usize,
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub GetTextRenderingParams: unsafe extern "system" fn(this: *mut *mut Self, textrenderingparams: *mut *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_DirectWrite"))]
    GetTextRenderingParams: usize,
}
impl ::windows_sys::core::Interface for ID2D1DrawingStateBlock {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 676359737, data2: 60406, data3: 18081, data4: [187, 71, 253, 133, 86, 90, 185, 87] };
}
#[repr(C)]
pub struct ID2D1DrawingStateBlock1 {
    pub base__: ID2D1DrawingStateBlock,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetDescription2: unsafe extern "system" fn(this: *mut *mut Self, statedescription: *mut D2D1_DRAWING_STATE_DESCRIPTION1),
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetDescription2: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetDescription2: unsafe extern "system" fn(this: *mut *mut Self, statedescription: *const D2D1_DRAWING_STATE_DESCRIPTION1),
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetDescription2: usize,
}
impl ::windows_sys::core::Interface for ID2D1DrawingStateBlock1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1755258757, data2: 50990, data3: 20019, data4: [143, 25, 133, 117, 78, 253, 90, 206] };
}
#[repr(C)]
pub struct ID2D1Effect {
    pub base__: ID2D1Properties,
    #[cfg(feature = "Win32_Foundation")]
    pub SetInput: unsafe extern "system" fn(this: *mut *mut Self, index: u32, input: *mut ::core::ffi::c_void, invalidate: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    SetInput: usize,
    pub SetInputCount: unsafe extern "system" fn(this: *mut *mut Self, inputcount: u32) -> ::windows_sys::core::HRESULT,
    pub GetInput: unsafe extern "system" fn(this: *mut *mut Self, index: u32, input: *mut *mut ::core::ffi::c_void),
    pub GetInputCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetOutput: unsafe extern "system" fn(this: *mut *mut Self, outputimage: *mut *mut ::core::ffi::c_void),
}
impl ::windows_sys::core::Interface for ID2D1Effect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 673258051, data2: 32137, data3: 18287, data4: [129, 129, 45, 97, 89, 178, 32, 173] };
}
#[repr(C)]
pub struct ID2D1EffectContext {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetDpi: unsafe extern "system" fn(this: *mut *mut Self, dpix: *mut f32, dpiy: *mut f32),
    pub CreateEffect: unsafe extern "system" fn(this: *mut *mut Self, effectid: *const ::windows_sys::core::GUID, effect: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetMaximumSupportedFeatureLevel: unsafe extern "system" fn(this: *mut *mut Self, featurelevels: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevelscount: u32, maximumsupportedfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetMaximumSupportedFeatureLevel: usize,
    pub CreateTransformNodeFromEffect: unsafe extern "system" fn(this: *mut *mut Self, effect: *mut ::core::ffi::c_void, transformnode: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateBlendTransform: unsafe extern "system" fn(this: *mut *mut Self, numinputs: u32, blenddescription: *const D2D1_BLEND_DESCRIPTION, transform: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateBorderTransform: unsafe extern "system" fn(this: *mut *mut Self, extendmodex: D2D1_EXTEND_MODE, extendmodey: D2D1_EXTEND_MODE, transform: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateOffsetTransform: unsafe extern "system" fn(this: *mut *mut Self, offset: super::super::Foundation::POINT, transform: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateOffsetTransform: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateBoundsAdjustmentTransform: unsafe extern "system" fn(this: *mut *mut Self, outputrectangle: *const super::super::Foundation::RECT, transform: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateBoundsAdjustmentTransform: usize,
    pub LoadPixelShader: unsafe extern "system" fn(this: *mut *mut Self, shaderid: *const ::windows_sys::core::GUID, shaderbuffer: *const u8, shaderbuffercount: u32) -> ::windows_sys::core::HRESULT,
    pub LoadVertexShader: unsafe extern "system" fn(this: *mut *mut Self, resourceid: *const ::windows_sys::core::GUID, shaderbuffer: *const u8, shaderbuffercount: u32) -> ::windows_sys::core::HRESULT,
    pub LoadComputeShader: unsafe extern "system" fn(this: *mut *mut Self, resourceid: *const ::windows_sys::core::GUID, shaderbuffer: *const u8, shaderbuffercount: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsShaderLoaded: unsafe extern "system" fn(this: *mut *mut Self, shaderid: *const ::windows_sys::core::GUID) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsShaderLoaded: usize,
    pub CreateResourceTexture: unsafe extern "system" fn(this: *mut *mut Self, resourceid: *const ::windows_sys::core::GUID, resourcetextureproperties: *const D2D1_RESOURCE_TEXTURE_PROPERTIES, data: *const u8, strides: *const u32, datasize: u32, resourcetexture: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FindResourceTexture: unsafe extern "system" fn(this: *mut *mut Self, resourceid: *const ::windows_sys::core::GUID, resourcetexture: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateVertexBuffer: unsafe extern "system" fn(this: *mut *mut Self, vertexbufferproperties: *const D2D1_VERTEX_BUFFER_PROPERTIES, resourceid: *const ::windows_sys::core::GUID, customvertexbufferproperties: *const D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES, buffer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateVertexBuffer: usize,
    pub FindVertexBuffer: unsafe extern "system" fn(this: *mut *mut Self, resourceid: *const ::windows_sys::core::GUID, buffer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateColorContext: unsafe extern "system" fn(this: *mut *mut Self, space: D2D1_COLOR_SPACE, profile: *const u8, profilesize: u32, colorcontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateColorContextFromFilename: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR, colorcontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub CreateColorContextFromWicColorContext: unsafe extern "system" fn(this: *mut *mut Self, wiccolorcontext: *mut ::core::ffi::c_void, colorcontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    CreateColorContextFromWicColorContext: usize,
    pub CheckFeatureSupport: unsafe extern "system" fn(this: *mut *mut Self, feature: D2D1_FEATURE, featuresupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsBufferPrecisionSupported: unsafe extern "system" fn(this: *mut *mut Self, bufferprecision: D2D1_BUFFER_PRECISION) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsBufferPrecisionSupported: usize,
}
impl ::windows_sys::core::Interface for ID2D1EffectContext {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1033867627, data2: 10204, data3: 19159, data4: [180, 241, 100, 148, 83, 64, 245, 99] };
}
#[repr(C)]
pub struct ID2D1EffectContext1 {
    pub base__: ID2D1EffectContext,
    pub CreateLookupTable3D: unsafe extern "system" fn(this: *mut *mut Self, precision: D2D1_BUFFER_PRECISION, extents: *const u32, data: *const u8, datacount: u32, strides: *const u32, lookuptable: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1EffectContext1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2225822042, data2: 64641, data3: 17734, data4: [186, 205, 232, 239, 77, 138, 190, 122] };
}
#[repr(C)]
pub struct ID2D1EffectContext2 {
    pub base__: ID2D1EffectContext1,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateColorContextFromDxgiColorSpace: unsafe extern "system" fn(this: *mut *mut Self, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, colorcontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateColorContextFromDxgiColorSpace: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreateColorContextFromSimpleColorProfile: unsafe extern "system" fn(this: *mut *mut Self, simpleprofile: *const D2D1_SIMPLE_COLOR_PROFILE, colorcontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreateColorContextFromSimpleColorProfile: usize,
}
impl ::windows_sys::core::Interface for ID2D1EffectContext2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1467667104, data2: 40903, data3: 19930, data4: [139, 24, 218, 184, 16, 20, 0, 82] };
}
#[repr(C)]
pub struct ID2D1EffectImpl {
    pub base__: ::windows_sys::core::IUnknown,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, effectcontext: *mut ::core::ffi::c_void, transformgraph: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PrepareForRender: unsafe extern "system" fn(this: *mut *mut Self, changetype: D2D1_CHANGE_TYPE) -> ::windows_sys::core::HRESULT,
    pub SetGraph: unsafe extern "system" fn(this: *mut *mut Self, transformgraph: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1EffectImpl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2722692415, data2: 15980, data3: 20067, data4: [159, 3, 127, 104, 236, 201, 29, 185] };
}
#[repr(C)]
pub struct ID2D1EllipseGeometry {
    pub base__: ID2D1Geometry,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetEllipse: unsafe extern "system" fn(this: *mut *mut Self, ellipse: *mut D2D1_ELLIPSE),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetEllipse: usize,
}
impl ::windows_sys::core::Interface for ID2D1EllipseGeometry {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 752420516, data2: 4834, data3: 4572, data4: [159, 237, 0, 17, 67, 160, 85, 249] };
}
#[repr(C)]
pub struct ID2D1Factory {
    pub base__: ::windows_sys::core::IUnknown,
    pub ReloadSystemMetrics: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetDesktopDpi: unsafe extern "system" fn(this: *mut *mut Self, dpix: *mut f32, dpiy: *mut f32),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreateRectangleGeometry: unsafe extern "system" fn(this: *mut *mut Self, rectangle: *const Common::D2D_RECT_F, rectanglegeometry: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreateRectangleGeometry: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreateRoundedRectangleGeometry: unsafe extern "system" fn(this: *mut *mut Self, roundedrectangle: *const D2D1_ROUNDED_RECT, roundedrectanglegeometry: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreateRoundedRectangleGeometry: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreateEllipseGeometry: unsafe extern "system" fn(this: *mut *mut Self, ellipse: *const D2D1_ELLIPSE, ellipsegeometry: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreateEllipseGeometry: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreateGeometryGroup: unsafe extern "system" fn(this: *mut *mut Self, fillmode: Common::D2D1_FILL_MODE, geometries: *const *mut ::core::ffi::c_void, geometriescount: u32, geometrygroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreateGeometryGroup: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateTransformedGeometry: unsafe extern "system" fn(this: *mut *mut Self, sourcegeometry: *mut ::core::ffi::c_void, transform: *const super::super::super::Foundation::Numerics::Matrix3x2, transformedgeometry: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateTransformedGeometry: usize,
    pub CreatePathGeometry: unsafe extern "system" fn(this: *mut *mut Self, pathgeometry: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateStrokeStyle: unsafe extern "system" fn(this: *mut *mut Self, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES, dashes: *const f32, dashescount: u32, strokestyle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
    pub CreateDrawingStateBlock: unsafe extern "system" fn(this: *mut *mut Self, drawingstatedescription: *const D2D1_DRAWING_STATE_DESCRIPTION, textrenderingparams: *mut ::core::ffi::c_void, drawingstateblock: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite")))]
    CreateDrawingStateBlock: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub CreateWicBitmapRenderTarget: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, rendertarget: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging")))]
    CreateWicBitmapRenderTarget: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateHwndRenderTarget: unsafe extern "system" fn(this: *mut *mut Self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, hwndrendertargetproperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES, hwndrendertarget: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateHwndRenderTarget: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateDxgiSurfaceRenderTarget: unsafe extern "system" fn(this: *mut *mut Self, dxgisurface: *mut ::core::ffi::c_void, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, rendertarget: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateDxgiSurfaceRenderTarget: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateDCRenderTarget: unsafe extern "system" fn(this: *mut *mut Self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, dcrendertarget: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateDCRenderTarget: usize,
}
impl ::windows_sys::core::Interface for ID2D1Factory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 102048327, data2: 28496, data3: 18010, data4: [146, 69, 17, 139, 253, 59, 96, 7] };
}
#[repr(C)]
pub struct ID2D1Factory1 {
    pub base__: ID2D1Factory,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub CreateDevice: unsafe extern "system" fn(this: *mut *mut Self, dxgidevice: *mut ::core::ffi::c_void, d2ddevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    CreateDevice: usize,
    pub CreateStrokeStyle2: unsafe extern "system" fn(this: *mut *mut Self, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES1, dashes: *const f32, dashescount: u32, strokestyle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreatePathGeometry2: unsafe extern "system" fn(this: *mut *mut Self, pathgeometry: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
    pub CreateDrawingStateBlock2: unsafe extern "system" fn(this: *mut *mut Self, drawingstatedescription: *const D2D1_DRAWING_STATE_DESCRIPTION1, textrenderingparams: *mut ::core::ffi::c_void, drawingstateblock: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite")))]
    CreateDrawingStateBlock2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateGdiMetafile: unsafe extern "system" fn(this: *mut *mut Self, metafilestream: *mut ::core::ffi::c_void, metafile: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateGdiMetafile: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RegisterEffectFromStream: unsafe extern "system" fn(this: *mut *mut Self, classid: *const ::windows_sys::core::GUID, propertyxml: *mut ::core::ffi::c_void, bindings: *const D2D1_PROPERTY_BINDING, bindingscount: u32, effectfactory: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RegisterEffectFromStream: usize,
    pub RegisterEffectFromString: unsafe extern "system" fn(this: *mut *mut Self, classid: *const ::windows_sys::core::GUID, propertyxml: ::windows_sys::core::PCWSTR, bindings: *const D2D1_PROPERTY_BINDING, bindingscount: u32, effectfactory: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UnregisterEffect: unsafe extern "system" fn(this: *mut *mut Self, classid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetRegisteredEffects: unsafe extern "system" fn(this: *mut *mut Self, effects: *mut ::windows_sys::core::GUID, effectscount: u32, effectsreturned: *mut u32, effectsregistered: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetEffectProperties: unsafe extern "system" fn(this: *mut *mut Self, effectid: *const ::windows_sys::core::GUID, properties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1Factory1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3138573154, data2: 56046, data3: 19354, data4: [170, 29, 20, 186, 64, 28, 250, 31] };
}
#[repr(C)]
pub struct ID2D1Factory2 {
    pub base__: ID2D1Factory1,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub CreateDevice2: unsafe extern "system" fn(this: *mut *mut Self, dxgidevice: *mut ::core::ffi::c_void, d2ddevice1: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    CreateDevice2: usize,
}
impl ::windows_sys::core::Interface for ID2D1Factory2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2499287667, data2: 37394, data3: 17270, data4: [156, 88, 177, 106, 58, 13, 57, 146] };
}
#[repr(C)]
pub struct ID2D1Factory3 {
    pub base__: ID2D1Factory2,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub CreateDevice3: unsafe extern "system" fn(this: *mut *mut Self, dxgidevice: *mut ::core::ffi::c_void, d2ddevice2: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    CreateDevice3: usize,
}
impl ::windows_sys::core::Interface for ID2D1Factory3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 141129119, data2: 20224, data3: 16703, data4: [176, 62, 43, 218, 69, 64, 77, 15] };
}
#[repr(C)]
pub struct ID2D1Factory4 {
    pub base__: ID2D1Factory3,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub CreateDevice4: unsafe extern "system" fn(this: *mut *mut Self, dxgidevice: *mut ::core::ffi::c_void, d2ddevice3: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    CreateDevice4: usize,
}
impl ::windows_sys::core::Interface for ID2D1Factory4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3176055506, data2: 1634, data3: 19438, data4: [186, 142, 111, 41, 240, 50, 224, 150] };
}
#[repr(C)]
pub struct ID2D1Factory5 {
    pub base__: ID2D1Factory4,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub CreateDevice5: unsafe extern "system" fn(this: *mut *mut Self, dxgidevice: *mut ::core::ffi::c_void, d2ddevice4: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    CreateDevice5: usize,
}
impl ::windows_sys::core::Interface for ID2D1Factory5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3291781524, data2: 33678, data3: 19215, data4: [140, 171, 68, 153, 125, 158, 234, 204] };
}
#[repr(C)]
pub struct ID2D1Factory6 {
    pub base__: ID2D1Factory5,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub CreateDevice6: unsafe extern "system" fn(this: *mut *mut Self, dxgidevice: *mut ::core::ffi::c_void, d2ddevice5: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    CreateDevice6: usize,
}
impl ::windows_sys::core::Interface for ID2D1Factory6 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4187451206, data2: 63042, data3: 17601, data4: [151, 202, 218, 50, 234, 42, 38, 53] };
}
#[repr(C)]
pub struct ID2D1Factory7 {
    pub base__: ID2D1Factory6,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub CreateDevice7: unsafe extern "system" fn(this: *mut *mut Self, dxgidevice: *mut ::core::ffi::c_void, d2ddevice6: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    CreateDevice7: usize,
}
impl ::windows_sys::core::Interface for ID2D1Factory7 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3183656403, data2: 47468, data3: 19942, data4: [189, 247, 153, 212, 116, 84, 84, 222] };
}
#[repr(C)]
pub struct ID2D1GdiInteropRenderTarget {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetDC: unsafe extern "system" fn(this: *mut *mut Self, mode: D2D1_DC_INITIALIZE_MODE, hdc: *mut super::Gdi::HDC) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetDC: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReleaseDC: unsafe extern "system" fn(this: *mut *mut Self, update: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReleaseDC: usize,
}
impl ::windows_sys::core::Interface for ID2D1GdiInteropRenderTarget {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3772469699, data2: 28535, data3: 19374, data4: [179, 213, 228, 117, 9, 179, 88, 56] };
}
#[repr(C)]
pub struct ID2D1GdiMetafile {
    pub base__: ID2D1Resource,
    pub Stream: unsafe extern "system" fn(this: *mut *mut Self, sink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetBounds: unsafe extern "system" fn(this: *mut *mut Self, bounds: *mut Common::D2D_RECT_F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetBounds: usize,
}
impl ::windows_sys::core::Interface for ID2D1GdiMetafile {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 794049987, data2: 53185, data3: 16913, data4: [134, 79, 207, 217, 28, 111, 51, 149] };
}
#[repr(C)]
pub struct ID2D1GdiMetafile1 {
    pub base__: ID2D1GdiMetafile,
    pub GetDpi: unsafe extern "system" fn(this: *mut *mut Self, dpix: *mut f32, dpiy: *mut f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetSourceBounds: unsafe extern "system" fn(this: *mut *mut Self, bounds: *mut Common::D2D_RECT_F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetSourceBounds: usize,
}
impl ::windows_sys::core::Interface for ID2D1GdiMetafile1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 778697192, data2: 56639, data3: 19449, data4: [149, 186, 192, 79, 73, 215, 136, 223] };
}
#[repr(C)]
pub struct ID2D1GdiMetafileSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub ProcessRecord: unsafe extern "system" fn(this: *mut *mut Self, recordtype: u32, recorddata: *const ::core::ffi::c_void, recorddatasize: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1GdiMetafileSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2183361318, data2: 33041, data3: 20348, data4: [188, 244, 181, 193, 23, 85, 100, 254] };
}
#[repr(C)]
pub struct ID2D1GdiMetafileSink1 {
    pub base__: ID2D1GdiMetafileSink,
    pub ProcessRecord2: unsafe extern "system" fn(this: *mut *mut Self, recordtype: u32, recorddata: *const ::core::ffi::c_void, recorddatasize: u32, flags: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1GdiMetafileSink1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4245605227, data2: 37350, data3: 16670, data4: [134, 85, 57, 94, 118, 15, 145, 180] };
}
#[repr(C)]
pub struct ID2D1Geometry {
    pub base__: ID2D1Resource,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub GetBounds: unsafe extern "system" fn(this: *mut *mut Self, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, bounds: *mut Common::D2D_RECT_F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    GetBounds: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub GetWidenedBounds: unsafe extern "system" fn(this: *mut *mut Self, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, bounds: *mut Common::D2D_RECT_F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    GetWidenedBounds: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub StrokeContainsPoint: unsafe extern "system" fn(this: *mut *mut Self, point: Common::D2D_POINT_2F, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, contains: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common")))]
    StrokeContainsPoint: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub FillContainsPoint: unsafe extern "system" fn(this: *mut *mut Self, point: Common::D2D_POINT_2F, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, contains: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common")))]
    FillContainsPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CompareWithGeometry: unsafe extern "system" fn(this: *mut *mut Self, inputgeometry: *mut ::core::ffi::c_void, inputgeometrytransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, relation: *mut D2D1_GEOMETRY_RELATION) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CompareWithGeometry: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub Simplify: unsafe extern "system" fn(this: *mut *mut Self, simplificationoption: D2D1_GEOMETRY_SIMPLIFICATION_OPTION, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    Simplify: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Tessellate: unsafe extern "system" fn(this: *mut *mut Self, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, tessellationsink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Tessellate: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub CombineWithGeometry: unsafe extern "system" fn(this: *mut *mut Self, inputgeometry: *mut ::core::ffi::c_void, combinemode: D2D1_COMBINE_MODE, inputgeometrytransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    CombineWithGeometry: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub Outline: unsafe extern "system" fn(this: *mut *mut Self, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    Outline: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub ComputeArea: unsafe extern "system" fn(this: *mut *mut Self, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, area: *mut f32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ComputeArea: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub ComputeLength: unsafe extern "system" fn(this: *mut *mut Self, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, length: *mut f32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ComputeLength: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub ComputePointAtLength: unsafe extern "system" fn(this: *mut *mut Self, length: f32, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, point: *mut Common::D2D_POINT_2F, unittangentvector: *mut Common::D2D_POINT_2F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    ComputePointAtLength: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub Widen: unsafe extern "system" fn(this: *mut *mut Self, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    Widen: usize,
}
impl ::windows_sys::core::Interface for ID2D1Geometry {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 752420513, data2: 4834, data3: 4572, data4: [159, 237, 0, 17, 67, 160, 85, 249] };
}
#[repr(C)]
pub struct ID2D1GeometryGroup {
    pub base__: ID2D1Geometry,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetFillMode: unsafe extern "system" fn(this: *mut *mut Self) -> Common::D2D1_FILL_MODE,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetFillMode: usize,
    pub GetSourceGeometryCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetSourceGeometries: unsafe extern "system" fn(this: *mut *mut Self, geometries: *mut *mut ::core::ffi::c_void, geometriescount: u32),
}
impl ::windows_sys::core::Interface for ID2D1GeometryGroup {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 752420518, data2: 4834, data3: 4572, data4: [159, 237, 0, 17, 67, 160, 85, 249] };
}
#[repr(C)]
pub struct ID2D1GeometryRealization {
    pub base__: ID2D1Resource,
}
impl ::windows_sys::core::Interface for ID2D1GeometryRealization {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2708015063, data2: 48130, data3: 18433, data4: [153, 232, 140, 247, 244, 133, 247, 116] };
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[repr(C)]
pub struct ID2D1GeometrySink {
    pub base__: Common::ID2D1SimplifiedGeometrySink,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub AddLine: unsafe extern "system" fn(this: *mut *mut Self, point: Common::D2D_POINT_2F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    AddLine: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub AddBezier: unsafe extern "system" fn(this: *mut *mut Self, bezier: *const Common::D2D1_BEZIER_SEGMENT),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    AddBezier: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub AddQuadraticBezier: unsafe extern "system" fn(this: *mut *mut Self, bezier: *const D2D1_QUADRATIC_BEZIER_SEGMENT),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    AddQuadraticBezier: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub AddQuadraticBeziers: unsafe extern "system" fn(this: *mut *mut Self, beziers: *const D2D1_QUADRATIC_BEZIER_SEGMENT, bezierscount: u32),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    AddQuadraticBeziers: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub AddArc: unsafe extern "system" fn(this: *mut *mut Self, arc: *const D2D1_ARC_SEGMENT),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    AddArc: usize,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::windows_sys::core::Interface for ID2D1GeometrySink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 752420511, data2: 4834, data3: 4572, data4: [159, 237, 0, 17, 67, 160, 85, 249] };
}
#[repr(C)]
pub struct ID2D1GradientMesh {
    pub base__: ID2D1Resource,
    pub GetPatchCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetPatches: unsafe extern "system" fn(this: *mut *mut Self, startindex: u32, patches: *mut D2D1_GRADIENT_MESH_PATCH, patchescount: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetPatches: usize,
}
impl ::windows_sys::core::Interface for ID2D1GradientMesh {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4069712897, data2: 49232, data3: 19678, data4: [131, 215, 4, 150, 45, 59, 35, 194] };
}
#[repr(C)]
pub struct ID2D1GradientStopCollection {
    pub base__: ID2D1Resource,
    pub GetGradientStopCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetGradientStops: unsafe extern "system" fn(this: *mut *mut Self, gradientstops: *mut D2D1_GRADIENT_STOP, gradientstopscount: u32),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetGradientStops: usize,
    pub GetColorInterpolationGamma: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_GAMMA,
    pub GetExtendMode: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_EXTEND_MODE,
}
impl ::windows_sys::core::Interface for ID2D1GradientStopCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 752420519, data2: 4834, data3: 4572, data4: [159, 237, 0, 17, 67, 160, 85, 249] };
}
#[repr(C)]
pub struct ID2D1GradientStopCollection1 {
    pub base__: ID2D1GradientStopCollection,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetGradientStops1: unsafe extern "system" fn(this: *mut *mut Self, gradientstops: *mut D2D1_GRADIENT_STOP, gradientstopscount: u32),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetGradientStops1: usize,
    pub GetPreInterpolationSpace: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_COLOR_SPACE,
    pub GetPostInterpolationSpace: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_COLOR_SPACE,
    pub GetBufferPrecision: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_BUFFER_PRECISION,
    pub GetColorInterpolationMode: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_COLOR_INTERPOLATION_MODE,
}
impl ::windows_sys::core::Interface for ID2D1GradientStopCollection1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2920641268, data2: 24016, data3: 18295, data4: [153, 139, 146, 121, 71, 42, 230, 59] };
}
#[repr(C)]
pub struct ID2D1HwndRenderTarget {
    pub base__: ID2D1RenderTarget,
    pub CheckWindowState: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_WINDOW_STATE,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub Resize: unsafe extern "system" fn(this: *mut *mut Self, pixelsize: *const Common::D2D_SIZE_U) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    Resize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHwnd: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::HWND,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHwnd: usize,
}
impl ::windows_sys::core::Interface for ID2D1HwndRenderTarget {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 752420504, data2: 4834, data3: 4572, data4: [159, 237, 0, 17, 67, 160, 85, 249] };
}
#[repr(C)]
pub struct ID2D1Image {
    pub base__: ID2D1Resource,
}
impl ::windows_sys::core::Interface for ID2D1Image {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1694605173, data2: 36258, data3: 18812, data4: [179, 44, 223, 163, 78, 72, 237, 230] };
}
#[repr(C)]
pub struct ID2D1ImageBrush {
    pub base__: ID2D1Brush,
    pub SetImage: unsafe extern "system" fn(this: *mut *mut Self, image: *mut ::core::ffi::c_void),
    pub SetExtendModeX: unsafe extern "system" fn(this: *mut *mut Self, extendmodex: D2D1_EXTEND_MODE),
    pub SetExtendModeY: unsafe extern "system" fn(this: *mut *mut Self, extendmodey: D2D1_EXTEND_MODE),
    pub SetInterpolationMode: unsafe extern "system" fn(this: *mut *mut Self, interpolationmode: D2D1_INTERPOLATION_MODE),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetSourceRectangle: unsafe extern "system" fn(this: *mut *mut Self, sourcerectangle: *const Common::D2D_RECT_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetSourceRectangle: usize,
    pub GetImage: unsafe extern "system" fn(this: *mut *mut Self, image: *mut *mut ::core::ffi::c_void),
    pub GetExtendModeX: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_EXTEND_MODE,
    pub GetExtendModeY: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_EXTEND_MODE,
    pub GetInterpolationMode: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_INTERPOLATION_MODE,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetSourceRectangle: unsafe extern "system" fn(this: *mut *mut Self, sourcerectangle: *mut Common::D2D_RECT_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetSourceRectangle: usize,
}
impl ::windows_sys::core::Interface for ID2D1ImageBrush {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4271806541, data2: 16277, data3: 16508, data4: [181, 219, 203, 148, 212, 232, 248, 124] };
}
#[repr(C)]
pub struct ID2D1ImageSource {
    pub base__: ID2D1Image,
    pub OfferResources: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TryReclaimResources: unsafe extern "system" fn(this: *mut *mut Self, resourcesdiscarded: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TryReclaimResources: usize,
}
impl ::windows_sys::core::Interface for ID2D1ImageSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3384173797, data2: 29857, data3: 17272, data4: [154, 194, 238, 252, 55, 163, 244, 216] };
}
#[repr(C)]
pub struct ID2D1ImageSourceFromWic {
    pub base__: ID2D1ImageSource,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub EnsureCached: unsafe extern "system" fn(this: *mut *mut Self, rectangletofill: *const Common::D2D_RECT_U) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    EnsureCached: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub TrimCache: unsafe extern "system" fn(this: *mut *mut Self, rectangletopreserve: *const Common::D2D_RECT_U) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    TrimCache: usize,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetSource: unsafe extern "system" fn(this: *mut *mut Self, wicbitmapsource: *mut *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetSource: usize,
}
impl ::windows_sys::core::Interface for ID2D1ImageSourceFromWic {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2000245825, data2: 7311, data3: 17749, data4: [134, 131, 245, 13, 171, 15, 231, 146] };
}
#[repr(C)]
pub struct ID2D1Ink {
    pub base__: ID2D1Resource,
    pub SetStartPoint: unsafe extern "system" fn(this: *mut *mut Self, startpoint: *const D2D1_INK_POINT),
    pub GetStartPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut D2D1_INK_POINT),
    pub AddSegments: unsafe extern "system" fn(this: *mut *mut Self, segments: *const D2D1_INK_BEZIER_SEGMENT, segmentscount: u32) -> ::windows_sys::core::HRESULT,
    pub RemoveSegmentsAtEnd: unsafe extern "system" fn(this: *mut *mut Self, segmentscount: u32) -> ::windows_sys::core::HRESULT,
    pub SetSegments: unsafe extern "system" fn(this: *mut *mut Self, startsegment: u32, segments: *const D2D1_INK_BEZIER_SEGMENT, segmentscount: u32) -> ::windows_sys::core::HRESULT,
    pub SetSegmentAtEnd: unsafe extern "system" fn(this: *mut *mut Self, segment: *const D2D1_INK_BEZIER_SEGMENT) -> ::windows_sys::core::HRESULT,
    pub GetSegmentCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetSegments: unsafe extern "system" fn(this: *mut *mut Self, startsegment: u32, segments: *mut D2D1_INK_BEZIER_SEGMENT, segmentscount: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub StreamAsGeometry: unsafe extern "system" fn(this: *mut *mut Self, inkstyle: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    StreamAsGeometry: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub GetBounds: unsafe extern "system" fn(this: *mut *mut Self, inkstyle: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, bounds: *mut Common::D2D_RECT_F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    GetBounds: usize,
}
impl ::windows_sys::core::Interface for ID2D1Ink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3029963323, data2: 28713, data3: 18319, data4: [168, 179, 67, 44, 124, 95, 83, 18] };
}
#[repr(C)]
pub struct ID2D1InkStyle {
    pub base__: ID2D1Resource,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetNibTransform: unsafe extern "system" fn(this: *mut *mut Self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2),
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetNibTransform: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetNibTransform: unsafe extern "system" fn(this: *mut *mut Self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2),
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetNibTransform: usize,
    pub SetNibShape: unsafe extern "system" fn(this: *mut *mut Self, nibshape: D2D1_INK_NIB_SHAPE),
    pub GetNibShape: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_INK_NIB_SHAPE,
}
impl ::windows_sys::core::Interface for ID2D1InkStyle {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3135812420, data2: 9212, data3: 16497, data4: [140, 181, 208, 93, 111, 7, 56, 72] };
}
#[repr(C)]
pub struct ID2D1Layer {
    pub base__: ID2D1Resource,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Common::D2D_SIZE_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetSize: usize,
}
impl ::windows_sys::core::Interface for ID2D1Layer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 752420507, data2: 4834, data3: 4572, data4: [159, 237, 0, 17, 67, 160, 85, 249] };
}
#[repr(C)]
pub struct ID2D1LinearGradientBrush {
    pub base__: ID2D1Brush,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetStartPoint: unsafe extern "system" fn(this: *mut *mut Self, startpoint: Common::D2D_POINT_2F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetStartPoint: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetEndPoint: unsafe extern "system" fn(this: *mut *mut Self, endpoint: Common::D2D_POINT_2F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetEndPoint: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetStartPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Common::D2D_POINT_2F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetStartPoint: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetEndPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Common::D2D_POINT_2F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetEndPoint: usize,
    pub GetGradientStopCollection: unsafe extern "system" fn(this: *mut *mut Self, gradientstopcollection: *mut *mut ::core::ffi::c_void),
}
impl ::windows_sys::core::Interface for ID2D1LinearGradientBrush {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 752420523, data2: 4834, data3: 4572, data4: [159, 237, 0, 17, 67, 160, 85, 249] };
}
#[repr(C)]
pub struct ID2D1LookupTable3D {
    pub base__: ID2D1Resource,
}
impl ::windows_sys::core::Interface for ID2D1LookupTable3D {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1407031381, data2: 41904, data3: 19803, data4: [130, 225, 38, 226, 92, 94, 87, 151] };
}
#[repr(C)]
pub struct ID2D1Mesh {
    pub base__: ID2D1Resource,
    pub Open: unsafe extern "system" fn(this: *mut *mut Self, tessellationsink: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1Mesh {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 752420546, data2: 4834, data3: 4572, data4: [159, 237, 0, 17, 67, 160, 85, 249] };
}
#[repr(C)]
pub struct ID2D1Multithread {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMultithreadProtected: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMultithreadProtected: usize,
    pub Enter: unsafe extern "system" fn(this: *mut *mut Self),
    pub Leave: unsafe extern "system" fn(this: *mut *mut Self),
}
impl ::windows_sys::core::Interface for ID2D1Multithread {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 837216188, data2: 57599, data3: 19782, data4: [140, 100, 160, 168, 196, 28, 21, 211] };
}
#[repr(C)]
pub struct ID2D1OffsetTransform {
    pub base__: ID2D1TransformNode,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOffset: unsafe extern "system" fn(this: *mut *mut Self, offset: super::super::Foundation::POINT),
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOffset: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::POINT),
    #[cfg(not(feature = "Win32_Foundation"))]
    GetOffset: usize,
}
impl ::windows_sys::core::Interface for ID2D1OffsetTransform {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1072082410, data2: 30275, data3: 20307, data4: [189, 20, 160, 206, 99, 242, 64, 66] };
}
#[repr(C)]
pub struct ID2D1PathGeometry {
    pub base__: ID2D1Geometry,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub Open: unsafe extern "system" fn(this: *mut *mut Self, geometrysink: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    Open: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub Stream: unsafe extern "system" fn(this: *mut *mut Self, geometrysink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    Stream: usize,
    pub GetSegmentCount: unsafe extern "system" fn(this: *mut *mut Self, count: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetFigureCount: unsafe extern "system" fn(this: *mut *mut Self, count: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1PathGeometry {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 752420517, data2: 4834, data3: 4572, data4: [159, 237, 0, 17, 67, 160, 85, 249] };
}
#[repr(C)]
pub struct ID2D1PathGeometry1 {
    pub base__: ID2D1PathGeometry,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub ComputePointAndSegmentAtLength: unsafe extern "system" fn(this: *mut *mut Self, length: f32, startsegment: u32, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, pointdescription: *mut D2D1_POINT_DESCRIPTION) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    ComputePointAndSegmentAtLength: usize,
}
impl ::windows_sys::core::Interface for ID2D1PathGeometry1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1656398546, data2: 43860, data3: 16823, data4: [184, 114, 120, 126, 1, 6, 164, 33] };
}
#[repr(C)]
pub struct ID2D1PrintControl {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com"))]
    pub AddPage: unsafe extern "system" fn(this: *mut *mut Self, commandlist: *mut ::core::ffi::c_void, pagesize: Common::D2D_SIZE_F, pageprintticketstream: *mut ::core::ffi::c_void, tag1: *mut u64, tag2: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com")))]
    AddPage: usize,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1PrintControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 740132477, data2: 49808, data3: 16840, data4: [174, 126, 52, 169, 135, 2, 233, 165] };
}
#[repr(C)]
pub struct ID2D1Properties {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetPropertyCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetPropertyName: unsafe extern "system" fn(this: *mut *mut Self, index: u32, name: ::windows_sys::core::PWSTR, namecount: u32) -> ::windows_sys::core::HRESULT,
    pub GetPropertyNameLength: unsafe extern "system" fn(this: *mut *mut Self, index: u32) -> u32,
    pub GetType: unsafe extern "system" fn(this: *mut *mut Self, index: u32) -> D2D1_PROPERTY_TYPE,
    pub GetPropertyIndex: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR) -> u32,
    pub SetValueByName: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR, r#type: D2D1_PROPERTY_TYPE, data: *const u8, datasize: u32) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, index: u32, r#type: D2D1_PROPERTY_TYPE, data: *const u8, datasize: u32) -> ::windows_sys::core::HRESULT,
    pub GetValueByName: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR, r#type: D2D1_PROPERTY_TYPE, data: *mut u8, datasize: u32) -> ::windows_sys::core::HRESULT,
    pub GetValue: unsafe extern "system" fn(this: *mut *mut Self, index: u32, r#type: D2D1_PROPERTY_TYPE, data: *mut u8, datasize: u32) -> ::windows_sys::core::HRESULT,
    pub GetValueSize: unsafe extern "system" fn(this: *mut *mut Self, index: u32) -> u32,
    pub GetSubProperties: unsafe extern "system" fn(this: *mut *mut Self, index: u32, subproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1Properties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1211397079, data2: 52550, data3: 20381, data4: [157, 58, 49, 18, 170, 128, 21, 157] };
}
#[repr(C)]
pub struct ID2D1RadialGradientBrush {
    pub base__: ID2D1Brush,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetCenter: unsafe extern "system" fn(this: *mut *mut Self, center: Common::D2D_POINT_2F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetCenter: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetGradientOriginOffset: unsafe extern "system" fn(this: *mut *mut Self, gradientoriginoffset: Common::D2D_POINT_2F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetGradientOriginOffset: usize,
    pub SetRadiusX: unsafe extern "system" fn(this: *mut *mut Self, radiusx: f32),
    pub SetRadiusY: unsafe extern "system" fn(this: *mut *mut Self, radiusy: f32),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetCenter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Common::D2D_POINT_2F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetCenter: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetGradientOriginOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Common::D2D_POINT_2F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetGradientOriginOffset: usize,
    pub GetRadiusX: unsafe extern "system" fn(this: *mut *mut Self) -> f32,
    pub GetRadiusY: unsafe extern "system" fn(this: *mut *mut Self) -> f32,
    pub GetGradientStopCollection: unsafe extern "system" fn(this: *mut *mut Self, gradientstopcollection: *mut *mut ::core::ffi::c_void),
}
impl ::windows_sys::core::Interface for ID2D1RadialGradientBrush {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 752420524, data2: 4834, data3: 4572, data4: [159, 237, 0, 17, 67, 160, 85, 249] };
}
#[repr(C)]
pub struct ID2D1RectangleGeometry {
    pub base__: ID2D1Geometry,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetRect: unsafe extern "system" fn(this: *mut *mut Self, rect: *mut Common::D2D_RECT_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetRect: usize,
}
impl ::windows_sys::core::Interface for ID2D1RectangleGeometry {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 752420514, data2: 4834, data3: 4572, data4: [159, 237, 0, 17, 67, 160, 85, 249] };
}
#[repr(C)]
pub struct ID2D1RenderInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetInputDescription: unsafe extern "system" fn(this: *mut *mut Self, inputindex: u32, inputdescription: D2D1_INPUT_DESCRIPTION) -> ::windows_sys::core::HRESULT,
    pub SetOutputBuffer: unsafe extern "system" fn(this: *mut *mut Self, bufferprecision: D2D1_BUFFER_PRECISION, channeldepth: D2D1_CHANNEL_DEPTH) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCached: unsafe extern "system" fn(this: *mut *mut Self, iscached: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCached: usize,
    pub SetInstructionCountHint: unsafe extern "system" fn(this: *mut *mut Self, instructioncount: u32),
}
impl ::windows_sys::core::Interface for ID2D1RenderInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1369104829, data2: 53658, data3: 16909, data4: [184, 73, 54, 79, 89, 71, 118, 183] };
}
#[repr(C)]
pub struct ID2D1RenderTarget {
    pub base__: ID2D1Resource,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateBitmap: unsafe extern "system" fn(this: *mut *mut Self, size: Common::D2D_SIZE_U, srcdata: *const ::core::ffi::c_void, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES, bitmap: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateBitmap: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub CreateBitmapFromWicBitmap: unsafe extern "system" fn(this: *mut *mut Self, wicbitmapsource: *mut ::core::ffi::c_void, bitmapproperties: *const D2D1_BITMAP_PROPERTIES, bitmap: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging")))]
    CreateBitmapFromWicBitmap: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateSharedBitmap: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, data: *mut ::core::ffi::c_void, bitmapproperties: *const D2D1_BITMAP_PROPERTIES, bitmap: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateSharedBitmap: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateBitmapBrush: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut ::core::ffi::c_void, bitmapbrushproperties: *const D2D1_BITMAP_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, bitmapbrush: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateBitmapBrush: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub CreateSolidColorBrush: unsafe extern "system" fn(this: *mut *mut Self, color: *const Common::D2D1_COLOR_F, brushproperties: *const D2D1_BRUSH_PROPERTIES, solidcolorbrush: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    CreateSolidColorBrush: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreateGradientStopCollection: unsafe extern "system" fn(this: *mut *mut Self, gradientstops: *const D2D1_GRADIENT_STOP, gradientstopscount: u32, colorinterpolationgamma: D2D1_GAMMA, extendmode: D2D1_EXTEND_MODE, gradientstopcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreateGradientStopCollection: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub CreateLinearGradientBrush: unsafe extern "system" fn(this: *mut *mut Self, lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, gradientstopcollection: *mut ::core::ffi::c_void, lineargradientbrush: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    CreateLinearGradientBrush: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub CreateRadialGradientBrush: unsafe extern "system" fn(this: *mut *mut Self, radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, gradientstopcollection: *mut ::core::ffi::c_void, radialgradientbrush: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    CreateRadialGradientBrush: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateCompatibleRenderTarget: unsafe extern "system" fn(this: *mut *mut Self, desiredsize: *const Common::D2D_SIZE_F, desiredpixelsize: *const Common::D2D_SIZE_U, desiredformat: *const Common::D2D1_PIXEL_FORMAT, options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS, bitmaprendertarget: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateCompatibleRenderTarget: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreateLayer: unsafe extern "system" fn(this: *mut *mut Self, size: *const Common::D2D_SIZE_F, layer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreateLayer: usize,
    pub CreateMesh: unsafe extern "system" fn(this: *mut *mut Self, mesh: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawLine: unsafe extern "system" fn(this: *mut *mut Self, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawLine: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawRectangle: unsafe extern "system" fn(this: *mut *mut Self, rect: *const Common::D2D_RECT_F, brush: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawRectangle: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub FillRectangle: unsafe extern "system" fn(this: *mut *mut Self, rect: *const Common::D2D_RECT_F, brush: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    FillRectangle: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawRoundedRectangle: unsafe extern "system" fn(this: *mut *mut Self, roundedrect: *const D2D1_ROUNDED_RECT, brush: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawRoundedRectangle: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub FillRoundedRectangle: unsafe extern "system" fn(this: *mut *mut Self, roundedrect: *const D2D1_ROUNDED_RECT, brush: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    FillRoundedRectangle: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawEllipse: unsafe extern "system" fn(this: *mut *mut Self, ellipse: *const D2D1_ELLIPSE, brush: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawEllipse: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub FillEllipse: unsafe extern "system" fn(this: *mut *mut Self, ellipse: *const D2D1_ELLIPSE, brush: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    FillEllipse: usize,
    pub DrawGeometry: unsafe extern "system" fn(this: *mut *mut Self, geometry: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void),
    pub FillGeometry: unsafe extern "system" fn(this: *mut *mut Self, geometry: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void, opacitybrush: *mut ::core::ffi::c_void),
    pub FillMesh: unsafe extern "system" fn(this: *mut *mut Self, mesh: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub FillOpacityMask: unsafe extern "system" fn(this: *mut *mut Self, opacitymask: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void, content: D2D1_OPACITY_MASK_CONTENT, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    FillOpacityMask: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawBitmap: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut ::core::ffi::c_void, destinationrectangle: *const Common::D2D_RECT_F, opacity: f32, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, sourcerectangle: *const Common::D2D_RECT_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawBitmap: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub DrawText: unsafe extern "system" fn(this: *mut *mut Self, string: ::windows_sys::core::PCWSTR, stringlength: u32, textformat: *mut ::core::ffi::c_void, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: *mut ::core::ffi::c_void, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE),
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    DrawText: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub DrawTextLayout: unsafe extern "system" fn(this: *mut *mut Self, origin: Common::D2D_POINT_2F, textlayout: *mut ::core::ffi::c_void, defaultfillbrush: *mut ::core::ffi::c_void, options: D2D1_DRAW_TEXT_OPTIONS),
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    DrawTextLayout: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub DrawGlyphRun: unsafe extern "system" fn(this: *mut *mut Self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, foregroundbrush: *mut ::core::ffi::c_void, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    DrawGlyphRun: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransform: unsafe extern "system" fn(this: *mut *mut Self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2),
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransform: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetTransform: unsafe extern "system" fn(this: *mut *mut Self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2),
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetTransform: usize,
    pub SetAntialiasMode: unsafe extern "system" fn(this: *mut *mut Self, antialiasmode: D2D1_ANTIALIAS_MODE),
    pub GetAntialiasMode: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_ANTIALIAS_MODE,
    pub SetTextAntialiasMode: unsafe extern "system" fn(this: *mut *mut Self, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE),
    pub GetTextAntialiasMode: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_TEXT_ANTIALIAS_MODE,
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub SetTextRenderingParams: unsafe extern "system" fn(this: *mut *mut Self, textrenderingparams: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_DirectWrite"))]
    SetTextRenderingParams: usize,
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub GetTextRenderingParams: unsafe extern "system" fn(this: *mut *mut Self, textrenderingparams: *mut *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_DirectWrite"))]
    GetTextRenderingParams: usize,
    pub SetTags: unsafe extern "system" fn(this: *mut *mut Self, tag1: u64, tag2: u64),
    pub GetTags: unsafe extern "system" fn(this: *mut *mut Self, tag1: *mut u64, tag2: *mut u64),
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub PushLayer: unsafe extern "system" fn(this: *mut *mut Self, layerparameters: *const D2D1_LAYER_PARAMETERS, layer: *mut ::core::ffi::c_void),
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    PushLayer: usize,
    pub PopLayer: unsafe extern "system" fn(this: *mut *mut Self),
    pub Flush: unsafe extern "system" fn(this: *mut *mut Self, tag1: *mut u64, tag2: *mut u64) -> ::windows_sys::core::HRESULT,
    pub SaveDrawingState: unsafe extern "system" fn(this: *mut *mut Self, drawingstateblock: *mut ::core::ffi::c_void),
    pub RestoreDrawingState: unsafe extern "system" fn(this: *mut *mut Self, drawingstateblock: *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub PushAxisAlignedClip: unsafe extern "system" fn(this: *mut *mut Self, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    PushAxisAlignedClip: usize,
    pub PopAxisAlignedClip: unsafe extern "system" fn(this: *mut *mut Self),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self, clearcolor: *const Common::D2D1_COLOR_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    Clear: usize,
    pub BeginDraw: unsafe extern "system" fn(this: *mut *mut Self),
    pub EndDraw: unsafe extern "system" fn(this: *mut *mut Self, tag1: *mut u64, tag2: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetPixelFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Common::D2D1_PIXEL_FORMAT),
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    GetPixelFormat: usize,
    pub SetDpi: unsafe extern "system" fn(this: *mut *mut Self, dpix: f32, dpiy: f32),
    pub GetDpi: unsafe extern "system" fn(this: *mut *mut Self, dpix: *mut f32, dpiy: *mut f32),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Common::D2D_SIZE_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetSize: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetPixelSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Common::D2D_SIZE_U),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetPixelSize: usize,
    pub GetMaximumBitmapSize: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> super::super::Foundation::BOOL,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    IsSupported: usize,
}
impl ::windows_sys::core::Interface for ID2D1RenderTarget {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 752420500, data2: 4834, data3: 4572, data4: [159, 237, 0, 17, 67, 160, 85, 249] };
}
#[repr(C)]
pub struct ID2D1Resource {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetFactory: unsafe extern "system" fn(this: *mut *mut Self, factory: *mut *mut ::core::ffi::c_void),
}
impl ::windows_sys::core::Interface for ID2D1Resource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 752420497, data2: 4834, data3: 4572, data4: [159, 237, 0, 17, 67, 160, 85, 249] };
}
#[repr(C)]
pub struct ID2D1ResourceTexture {
    pub base__: ::windows_sys::core::IUnknown,
    pub Update: unsafe extern "system" fn(this: *mut *mut Self, minimumextents: *const u32, maximimumextents: *const u32, strides: *const u32, dimensions: u32, data: *const u8, datacount: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1ResourceTexture {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1754076611, data2: 688, data3: 17293, data4: [177, 58, 209, 180, 76, 50, 195, 154] };
}
#[repr(C)]
pub struct ID2D1RoundedRectangleGeometry {
    pub base__: ID2D1Geometry,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetRoundedRect: unsafe extern "system" fn(this: *mut *mut Self, roundedrect: *mut D2D1_ROUNDED_RECT),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetRoundedRect: usize,
}
impl ::windows_sys::core::Interface for ID2D1RoundedRectangleGeometry {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 752420515, data2: 4834, data3: 4572, data4: [159, 237, 0, 17, 67, 160, 85, 249] };
}
#[repr(C)]
pub struct ID2D1SolidColorBrush {
    pub base__: ID2D1Brush,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, color: *const Common::D2D1_COLOR_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetColor: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Common::D2D1_COLOR_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetColor: usize,
}
impl ::windows_sys::core::Interface for ID2D1SolidColorBrush {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 752420521, data2: 4834, data3: 4572, data4: [159, 237, 0, 17, 67, 160, 85, 249] };
}
#[repr(C)]
pub struct ID2D1SourceTransform {
    pub base__: ID2D1Transform,
    pub SetRenderInfo: unsafe extern "system" fn(this: *mut *mut Self, renderinfo: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub Draw: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, drawrect: *const super::super::Foundation::RECT, targetorigin: Common::D2D_POINT_2U) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common")))]
    Draw: usize,
}
impl ::windows_sys::core::Interface for ID2D1SourceTransform {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3675783389, data2: 3124, data3: 19705, data4: [190, 144, 49, 204, 10, 86, 83, 225] };
}
#[repr(C)]
pub struct ID2D1SpriteBatch {
    pub base__: ID2D1Resource,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub AddSprites: unsafe extern "system" fn(this: *mut *mut Self, spritecount: u32, destinationrectangles: *const Common::D2D_RECT_F, sourcerectangles: *const Common::D2D_RECT_U, colors: *const Common::D2D1_COLOR_F, transforms: *const super::super::super::Foundation::Numerics::Matrix3x2, destinationrectanglesstride: u32, sourcerectanglesstride: u32, colorsstride: u32, transformsstride: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    AddSprites: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub SetSprites: unsafe extern "system" fn(this: *mut *mut Self, startindex: u32, spritecount: u32, destinationrectangles: *const Common::D2D_RECT_F, sourcerectangles: *const Common::D2D_RECT_U, colors: *const Common::D2D1_COLOR_F, transforms: *const super::super::super::Foundation::Numerics::Matrix3x2, destinationrectanglesstride: u32, sourcerectanglesstride: u32, colorsstride: u32, transformsstride: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    SetSprites: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub GetSprites: unsafe extern "system" fn(this: *mut *mut Self, startindex: u32, spritecount: u32, destinationrectangles: *mut Common::D2D_RECT_F, sourcerectangles: *mut Common::D2D_RECT_U, colors: *mut Common::D2D1_COLOR_F, transforms: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    GetSprites: usize,
    pub GetSpriteCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self),
}
impl ::windows_sys::core::Interface for ID2D1SpriteBatch {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1304789951, data2: 14864, data3: 17290, data4: [135, 34, 233, 118, 82, 36, 241, 241] };
}
#[repr(C)]
pub struct ID2D1StrokeStyle {
    pub base__: ID2D1Resource,
    pub GetStartCap: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_CAP_STYLE,
    pub GetEndCap: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_CAP_STYLE,
    pub GetDashCap: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_CAP_STYLE,
    pub GetMiterLimit: unsafe extern "system" fn(this: *mut *mut Self) -> f32,
    pub GetLineJoin: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_LINE_JOIN,
    pub GetDashOffset: unsafe extern "system" fn(this: *mut *mut Self) -> f32,
    pub GetDashStyle: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_DASH_STYLE,
    pub GetDashesCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetDashes: unsafe extern "system" fn(this: *mut *mut Self, dashes: *mut f32, dashescount: u32),
}
impl ::windows_sys::core::Interface for ID2D1StrokeStyle {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 752420509, data2: 4834, data3: 4572, data4: [159, 237, 0, 17, 67, 160, 85, 249] };
}
#[repr(C)]
pub struct ID2D1StrokeStyle1 {
    pub base__: ID2D1StrokeStyle,
    pub GetStrokeTransformType: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_STROKE_TRANSFORM_TYPE,
}
impl ::windows_sys::core::Interface for ID2D1StrokeStyle1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 279390822, data2: 59676, data3: 17396, data4: [153, 63, 221, 244, 184, 43, 11, 74] };
}
#[repr(C)]
pub struct ID2D1SvgAttribute {
    pub base__: ID2D1Resource,
    pub GetElement: unsafe extern "system" fn(this: *mut *mut Self, element: *mut *mut ::core::ffi::c_void),
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, attribute: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1SvgAttribute {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3385700573, data2: 63689, data3: 20080, data4: [183, 194, 48, 28, 128, 41, 44, 94] };
}
#[repr(C)]
pub struct ID2D1SvgDocument {
    pub base__: ID2D1Resource,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetViewportSize: unsafe extern "system" fn(this: *mut *mut Self, viewportsize: Common::D2D_SIZE_F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetViewportSize: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetViewportSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Common::D2D_SIZE_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetViewportSize: usize,
    pub SetRoot: unsafe extern "system" fn(this: *mut *mut Self, root: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetRoot: unsafe extern "system" fn(this: *mut *mut Self, root: *mut *mut ::core::ffi::c_void),
    pub FindElementById: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::PCWSTR, svgelement: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Serialize: unsafe extern "system" fn(this: *mut *mut Self, outputxmlstream: *mut ::core::ffi::c_void, subtree: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Serialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Deserialize: unsafe extern "system" fn(this: *mut *mut Self, inputxmlstream: *mut ::core::ffi::c_void, subtree: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Deserialize: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreatePaint: unsafe extern "system" fn(this: *mut *mut Self, painttype: D2D1_SVG_PAINT_TYPE, color: *const Common::D2D1_COLOR_F, id: ::windows_sys::core::PCWSTR, paint: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreatePaint: usize,
    pub CreateStrokeDashArray: unsafe extern "system" fn(this: *mut *mut Self, dashes: *const D2D1_SVG_LENGTH, dashescount: u32, strokedasharray: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreatePointCollection: unsafe extern "system" fn(this: *mut *mut Self, points: *const Common::D2D_POINT_2F, pointscount: u32, pointcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreatePointCollection: usize,
    pub CreatePathData: unsafe extern "system" fn(this: *mut *mut Self, segmentdata: *const f32, segmentdatacount: u32, commands: *const D2D1_SVG_PATH_COMMAND, commandscount: u32, pathdata: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1SvgDocument {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2260241997, data2: 44964, data3: 19835, data4: [136, 228, 104, 165, 28, 74, 10, 236] };
}
#[repr(C)]
pub struct ID2D1SvgElement {
    pub base__: ID2D1Resource,
    pub GetDocument: unsafe extern "system" fn(this: *mut *mut Self, document: *mut *mut ::core::ffi::c_void),
    pub GetTagName: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PWSTR, namecount: u32) -> ::windows_sys::core::HRESULT,
    pub GetTagNameLength: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub IsTextContent: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsTextContent: usize,
    pub GetParent: unsafe extern "system" fn(this: *mut *mut Self, parent: *mut *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_Foundation")]
    pub HasChildren: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasChildren: usize,
    pub GetFirstChild: unsafe extern "system" fn(this: *mut *mut Self, child: *mut *mut ::core::ffi::c_void),
    pub GetLastChild: unsafe extern "system" fn(this: *mut *mut Self, child: *mut *mut ::core::ffi::c_void),
    pub GetPreviousChild: unsafe extern "system" fn(this: *mut *mut Self, referencechild: *mut ::core::ffi::c_void, previouschild: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetNextChild: unsafe extern "system" fn(this: *mut *mut Self, referencechild: *mut ::core::ffi::c_void, nextchild: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InsertChildBefore: unsafe extern "system" fn(this: *mut *mut Self, newchild: *mut ::core::ffi::c_void, referencechild: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AppendChild: unsafe extern "system" fn(this: *mut *mut Self, newchild: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ReplaceChild: unsafe extern "system" fn(this: *mut *mut Self, newchild: *mut ::core::ffi::c_void, oldchild: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveChild: unsafe extern "system" fn(this: *mut *mut Self, oldchild: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateChild: unsafe extern "system" fn(this: *mut *mut Self, tagname: ::windows_sys::core::PCWSTR, newchild: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsAttributeSpecified: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR, inherited: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsAttributeSpecified: usize,
    pub GetSpecifiedAttributeCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSpecifiedAttributeName: unsafe extern "system" fn(this: *mut *mut Self, index: u32, name: ::windows_sys::core::PWSTR, namecount: u32, inherited: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSpecifiedAttributeName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSpecifiedAttributeNameLength: unsafe extern "system" fn(this: *mut *mut Self, index: u32, namelength: *mut u32, inherited: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSpecifiedAttributeNameLength: usize,
    pub RemoveAttribute: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SetTextValue: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR, namecount: u32) -> ::windows_sys::core::HRESULT,
    pub GetTextValue: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PWSTR, namecount: u32) -> ::windows_sys::core::HRESULT,
    pub GetTextValueLength: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub SetAttributeValue: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAttributeValue2: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_POD_TYPE, value: *const ::core::ffi::c_void, valuesizeinbytes: u32) -> ::windows_sys::core::HRESULT,
    pub SetAttributeValue3: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, value: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetAttributeValue: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR, riid: *const ::windows_sys::core::GUID, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAttributeValue2: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_POD_TYPE, value: *mut ::core::ffi::c_void, valuesizeinbytes: u32) -> ::windows_sys::core::HRESULT,
    pub GetAttributeValue3: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, value: ::windows_sys::core::PWSTR, valuecount: u32) -> ::windows_sys::core::HRESULT,
    pub GetAttributeValueLength: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, valuelength: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1SvgElement {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2893768614, data2: 6206, data3: 18881, data4: [168, 35, 14, 190, 64, 176, 219, 41] };
}
#[repr(C)]
pub struct ID2D1SvgGlyphStyle {
    pub base__: ID2D1Resource,
    pub SetFill: unsafe extern "system" fn(this: *mut *mut Self, brush: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFill: unsafe extern "system" fn(this: *mut *mut Self, brush: *mut *mut ::core::ffi::c_void),
    pub SetStroke: unsafe extern "system" fn(this: *mut *mut Self, brush: *mut ::core::ffi::c_void, strokewidth: f32, dashes: *const f32, dashescount: u32, dashoffset: f32) -> ::windows_sys::core::HRESULT,
    pub GetStrokeDashesCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetStroke: unsafe extern "system" fn(this: *mut *mut Self, brush: *mut *mut ::core::ffi::c_void, strokewidth: *mut f32, dashes: *mut f32, dashescount: u32, dashoffset: *mut f32),
}
impl ::windows_sys::core::Interface for ID2D1SvgGlyphStyle {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2942768969, data2: 53825, data3: 19896, data4: [142, 65, 220, 194, 229, 193, 164, 56] };
}
#[repr(C)]
pub struct ID2D1SvgPaint {
    pub base__: ID2D1SvgAttribute,
    pub SetPaintType: unsafe extern "system" fn(this: *mut *mut Self, painttype: D2D1_SVG_PAINT_TYPE) -> ::windows_sys::core::HRESULT,
    pub GetPaintType: unsafe extern "system" fn(this: *mut *mut Self) -> D2D1_SVG_PAINT_TYPE,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, color: *const Common::D2D1_COLOR_F) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetColor: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetColor: unsafe extern "system" fn(this: *mut *mut Self, color: *mut Common::D2D1_COLOR_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetColor: usize,
    pub SetId: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetId: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::PWSTR, idcount: u32) -> ::windows_sys::core::HRESULT,
    pub GetIdLength: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
}
impl ::windows_sys::core::Interface for ID2D1SvgPaint {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3583748874, data2: 26786, data3: 17755, data4: [165, 220, 158, 178, 133, 78, 36, 144] };
}
#[repr(C)]
pub struct ID2D1SvgPathData {
    pub base__: ID2D1SvgAttribute,
    pub RemoveSegmentDataAtEnd: unsafe extern "system" fn(this: *mut *mut Self, datacount: u32) -> ::windows_sys::core::HRESULT,
    pub UpdateSegmentData: unsafe extern "system" fn(this: *mut *mut Self, data: *const f32, datacount: u32, startindex: u32) -> ::windows_sys::core::HRESULT,
    pub GetSegmentData: unsafe extern "system" fn(this: *mut *mut Self, data: *mut f32, datacount: u32, startindex: u32) -> ::windows_sys::core::HRESULT,
    pub GetSegmentDataCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub RemoveCommandsAtEnd: unsafe extern "system" fn(this: *mut *mut Self, commandscount: u32) -> ::windows_sys::core::HRESULT,
    pub UpdateCommands: unsafe extern "system" fn(this: *mut *mut Self, commands: *const D2D1_SVG_PATH_COMMAND, commandscount: u32, startindex: u32) -> ::windows_sys::core::HRESULT,
    pub GetCommands: unsafe extern "system" fn(this: *mut *mut Self, commands: *mut D2D1_SVG_PATH_COMMAND, commandscount: u32, startindex: u32) -> ::windows_sys::core::HRESULT,
    pub GetCommandsCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreatePathGeometry: unsafe extern "system" fn(this: *mut *mut Self, fillmode: Common::D2D1_FILL_MODE, pathgeometry: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreatePathGeometry: usize,
}
impl ::windows_sys::core::Interface for ID2D1SvgPathData {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3231048948, data2: 48024, data3: 17366, data4: [151, 69, 77, 27, 132, 236, 152, 136] };
}
#[repr(C)]
pub struct ID2D1SvgPointCollection {
    pub base__: ID2D1SvgAttribute,
    pub RemovePointsAtEnd: unsafe extern "system" fn(this: *mut *mut Self, pointscount: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub UpdatePoints: unsafe extern "system" fn(this: *mut *mut Self, points: *const Common::D2D_POINT_2F, pointscount: u32, startindex: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    UpdatePoints: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetPoints: unsafe extern "system" fn(this: *mut *mut Self, points: *mut Common::D2D_POINT_2F, pointscount: u32, startindex: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetPoints: usize,
    pub GetPointsCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
}
impl ::windows_sys::core::Interface for ID2D1SvgPointCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2646494221, data2: 13682, data3: 19929, data4: [152, 37, 85, 48, 129, 59, 183, 18] };
}
#[repr(C)]
pub struct ID2D1SvgStrokeDashArray {
    pub base__: ID2D1SvgAttribute,
    pub RemoveDashesAtEnd: unsafe extern "system" fn(this: *mut *mut Self, dashescount: u32) -> ::windows_sys::core::HRESULT,
    pub UpdateDashes: unsafe extern "system" fn(this: *mut *mut Self, dashes: *const D2D1_SVG_LENGTH, dashescount: u32, startindex: u32) -> ::windows_sys::core::HRESULT,
    pub UpdateDashes2: unsafe extern "system" fn(this: *mut *mut Self, dashes: *const f32, dashescount: u32, startindex: u32) -> ::windows_sys::core::HRESULT,
    pub GetDashes: unsafe extern "system" fn(this: *mut *mut Self, dashes: *mut D2D1_SVG_LENGTH, dashescount: u32, startindex: u32) -> ::windows_sys::core::HRESULT,
    pub GetDashes2: unsafe extern "system" fn(this: *mut *mut Self, dashes: *mut f32, dashescount: u32, startindex: u32) -> ::windows_sys::core::HRESULT,
    pub GetDashesCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
}
impl ::windows_sys::core::Interface for ID2D1SvgStrokeDashArray {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4055943762, data2: 37539, data3: 20224, data4: [180, 206, 243, 86, 145, 239, 217, 217] };
}
#[repr(C)]
pub struct ID2D1TessellationSink {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub AddTriangles: unsafe extern "system" fn(this: *mut *mut Self, triangles: *const D2D1_TRIANGLE, trianglescount: u32),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    AddTriangles: usize,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1TessellationSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 752420545, data2: 4834, data3: 4572, data4: [159, 237, 0, 17, 67, 160, 85, 249] };
}
#[repr(C)]
pub struct ID2D1Transform {
    pub base__: ID2D1TransformNode,
    #[cfg(feature = "Win32_Foundation")]
    pub MapOutputRectToInputRects: unsafe extern "system" fn(this: *mut *mut Self, outputrect: *const super::super::Foundation::RECT, inputrects: *mut super::super::Foundation::RECT, inputrectscount: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MapOutputRectToInputRects: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MapInputRectsToOutputRect: unsafe extern "system" fn(this: *mut *mut Self, inputrects: *const super::super::Foundation::RECT, inputopaquesubrects: *const super::super::Foundation::RECT, inputrectcount: u32, outputrect: *mut super::super::Foundation::RECT, outputopaquesubrect: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MapInputRectsToOutputRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MapInvalidRect: unsafe extern "system" fn(this: *mut *mut Self, inputindex: u32, invalidinputrect: super::super::Foundation::RECT, invalidoutputrect: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MapInvalidRect: usize,
}
impl ::windows_sys::core::Interface for ID2D1Transform {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4011468925, data2: 13354, data3: 20342, data4: [143, 219, 218, 13, 110, 169, 249, 43] };
}
#[repr(C)]
pub struct ID2D1TransformGraph {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetInputCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub SetSingleTransformNode: unsafe extern "system" fn(this: *mut *mut Self, node: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddNode: unsafe extern "system" fn(this: *mut *mut Self, node: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveNode: unsafe extern "system" fn(this: *mut *mut Self, node: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOutputNode: unsafe extern "system" fn(this: *mut *mut Self, node: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ConnectNode: unsafe extern "system" fn(this: *mut *mut Self, fromnode: *mut ::core::ffi::c_void, tonode: *mut ::core::ffi::c_void, tonodeinputindex: u32) -> ::windows_sys::core::HRESULT,
    pub ConnectToEffectInput: unsafe extern "system" fn(this: *mut *mut Self, toeffectinputindex: u32, node: *mut ::core::ffi::c_void, tonodeinputindex: u32) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self),
    pub SetPassthroughGraph: unsafe extern "system" fn(this: *mut *mut Self, effectinputindex: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1TransformGraph {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 332566584, data2: 50150, data3: 16436, data4: [144, 129, 19, 181, 58, 65, 121, 146] };
}
#[repr(C)]
pub struct ID2D1TransformNode {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetInputCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
}
impl ::windows_sys::core::Interface for ID2D1TransformNode {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3002065383, data2: 29343, data3: 16642, data4: [148, 159, 80, 95, 162, 27, 246, 102] };
}
#[repr(C)]
pub struct ID2D1TransformedGeometry {
    pub base__: ID2D1Geometry,
    pub GetSourceGeometry: unsafe extern "system" fn(this: *mut *mut Self, sourcegeometry: *mut *mut ::core::ffi::c_void),
    #[cfg(feature = "Foundation_Numerics")]
    pub GetTransform: unsafe extern "system" fn(this: *mut *mut Self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2),
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetTransform: usize,
}
impl ::windows_sys::core::Interface for ID2D1TransformedGeometry {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 752420539, data2: 4834, data3: 4572, data4: [159, 237, 0, 17, 67, 160, 85, 249] };
}
#[repr(C)]
pub struct ID2D1TransformedImageSource {
    pub base__: ID2D1Image,
    pub GetSource: unsafe extern "system" fn(this: *mut *mut Self, imagesource: *mut *mut ::core::ffi::c_void),
    pub GetProperties: unsafe extern "system" fn(this: *mut *mut Self, properties: *mut D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES),
}
impl ::windows_sys::core::Interface for ID2D1TransformedImageSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2132769253, data2: 10134, data3: 16748, data4: [143, 85, 112, 15, 145, 20, 69, 229] };
}
#[repr(C)]
pub struct ID2D1VertexBuffer {
    pub base__: ::windows_sys::core::IUnknown,
    pub Map: unsafe extern "system" fn(this: *mut *mut Self, data: *mut *mut u8, buffersize: u32) -> ::windows_sys::core::HRESULT,
    pub Unmap: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ID2D1VertexBuffer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2609582902, data2: 165, data3: 18024, data4: [146, 183, 206, 213, 216, 191, 155, 123] };
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct Matrix4x3F {
    pub __AnonymousBase_d2d1_1helper_L45_C31: Common::D2D_MATRIX_4X3_F,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for Matrix4x3F {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for Matrix4x3F {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct Matrix4x4F {
    pub __AnonymousBase_d2d1_1helper_L97_C31: Common::D2D_MATRIX_4X4_F,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for Matrix4x4F {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for Matrix4x4F {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct Matrix5x4F {
    pub __AnonymousBase_d2d1_1helper_L472_C31: Common::D2D_MATRIX_5X4_F,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for Matrix5x4F {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for Matrix5x4F {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type PD2D1_EFFECT_FACTORY = ::core::option::Option<unsafe extern "system" fn(effectimpl: *mut *mut *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type PD2D1_PROPERTY_GET_FUNCTION = ::core::option::Option<unsafe extern "system" fn(effect: *mut *mut ::windows_sys::core::IUnknown, data: *mut u8, datasize: u32, actualsize: *mut u32) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type PD2D1_PROPERTY_SET_FUNCTION = ::core::option::Option<unsafe extern "system" fn(effect: *mut *mut ::windows_sys::core::IUnknown, data: *const u8, datasize: u32) -> ::windows_sys::core::HRESULT>;
