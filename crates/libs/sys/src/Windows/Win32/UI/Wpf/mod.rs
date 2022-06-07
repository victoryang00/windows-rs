pub const CLSID_MILBitmapEffectBevel: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4248182206, data2: 27803, data3: 19936, data4: [130, 144, 246, 64, 12, 39, 55, 237] };
pub const CLSID_MILBitmapEffectBlur: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2837766023, data2: 8797, data3: 17267, data4: [143, 91, 185, 14, 200, 90, 227, 222] };
pub const CLSID_MILBitmapEffectDropShadow: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1167736766, data2: 55468, data3: 18066, data4: [135, 75, 122, 38, 87, 21, 170, 22] };
pub const CLSID_MILBitmapEffectEmboss: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3442055238, data2: 33359, data3: 18412, data4: [160, 7, 18, 170, 118, 127, 40, 22] };
pub const CLSID_MILBitmapEffectGroup: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2895911578, data2: 32280, data3: 20324, data4: [172, 126, 71, 207, 127, 5, 30, 149] };
pub const CLSID_MILBitmapEffectOuterGlow: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3793099741, data2: 32438, data3: 18213, data4: [156, 11, 138, 42, 27, 79, 6, 103] };
#[repr(C)]
pub struct IMILBitmapEffect {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetOutput: unsafe extern "system" fn(this: *mut *mut Self, uiindex: u32, pcontext: *mut ::core::ffi::c_void, ppbitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetOutput: usize,
    pub GetParentEffect: unsafe extern "system" fn(this: *mut *mut Self, ppparenteffect: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub SetInputSource: unsafe extern "system" fn(this: *mut *mut Self, uiindex: u32, pbitmapsource: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    SetInputSource: usize,
}
impl ::windows_sys::core::Interface for IMILBitmapEffect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2322592545, data2: 51524, data3: 18971, data4: [153, 68, 153, 84, 175, 48, 18, 88] };
}
#[repr(C)]
pub struct IMILBitmapEffectConnections {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetInputConnector: unsafe extern "system" fn(this: *mut *mut Self, uiindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetOutputConnector: unsafe extern "system" fn(this: *mut *mut Self, uiindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMILBitmapEffectConnections {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3266697313, data2: 39706, data3: 17268, data4: [137, 176, 222, 196, 135, 77, 106, 129] };
}
#[repr(C)]
pub struct IMILBitmapEffectConnectionsInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetNumberInputs: unsafe extern "system" fn(this: *mut *mut Self, puinuminputs: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetNumberOutputs: unsafe extern "system" fn(this: *mut *mut Self, puinumoutputs: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetInputConnectorInfo: unsafe extern "system" fn(this: *mut *mut Self, uiindex: u32, ppconnectorinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetOutputConnectorInfo: unsafe extern "system" fn(this: *mut *mut Self, uiindex: u32, ppconnectorinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMILBitmapEffectConnectionsInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1198216074, data2: 51045, data3: 16951, data4: [186, 74, 214, 168, 128, 255, 12, 252] };
}
#[repr(C)]
pub struct IMILBitmapEffectConnector {
    pub base__: IMILBitmapEffectConnectorInfo,
    pub IsConnected: unsafe extern "system" fn(this: *mut *mut Self, pfconnected: *mut i16) -> ::windows_sys::core::HRESULT,
    pub GetBitmapEffect: unsafe extern "system" fn(this: *mut *mut Self, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMILBitmapEffectConnector {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4120209331, data2: 30401, data3: 19783, data4: [186, 30, 121, 249, 85, 227, 80, 239] };
}
#[repr(C)]
pub struct IMILBitmapEffectConnectorInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetIndex: unsafe extern "system" fn(this: *mut *mut Self, puiindex: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetOptimalFormat: unsafe extern "system" fn(this: *mut *mut Self, pformat: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetNumberFormats: unsafe extern "system" fn(this: *mut *mut Self, pulnumberformats: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetFormat: unsafe extern "system" fn(this: *mut *mut Self, ulindex: u32, pformat: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMILBitmapEffectConnectorInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4134350411, data2: 46187, data3: 17148, data4: [133, 158, 61, 160, 236, 219, 60, 67] };
}
#[repr(C)]
pub struct IMILBitmapEffectEvents {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub PropertyChange: unsafe extern "system" fn(this: *mut *mut Self, peffect: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PropertyChange: usize,
    pub DirtyRegion: unsafe extern "system" fn(this: *mut *mut Self, peffect: *mut ::core::ffi::c_void, prect: *const MilRectD) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMILBitmapEffectEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 780668376, data2: 63694, data3: 17787, data4: [129, 153, 214, 11, 179, 215, 239, 152] };
}
#[repr(C)]
pub struct IMILBitmapEffectFactory {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateEffect: unsafe extern "system" fn(this: *mut *mut Self, pguideffect: *const ::windows_sys::core::GUID, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateContext: unsafe extern "system" fn(this: *mut *mut Self, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateEffectOuter: unsafe extern "system" fn(this: *mut *mut Self, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMILBitmapEffectFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 866770740, data2: 41987, data3: 20167, data4: [176, 126, 188, 6, 130, 55, 8, 69] };
}
#[repr(C)]
pub struct IMILBitmapEffectGroup {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetInteriorInputConnector: unsafe extern "system" fn(this: *mut *mut Self, uiindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetInteriorOutputConnector: unsafe extern "system" fn(this: *mut *mut Self, uiindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, peffect: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMILBitmapEffectGroup {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 798303072, data2: 27018, data3: 19142, data4: [129, 161, 188, 253, 240, 142, 184, 232] };
}
#[repr(C)]
pub struct IMILBitmapEffectGroupImpl {
    pub base__: ::windows_sys::core::IUnknown,
    pub Preprocess: unsafe extern "system" fn(this: *mut *mut Self, pcontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetNumberChildren: unsafe extern "system" fn(this: *mut *mut Self, puinumberchildren: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetChildren: unsafe extern "system" fn(this: *mut *mut Self, pchildren: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMILBitmapEffectGroupImpl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2029966616, data2: 7420, data3: 18439, data4: [139, 133, 107, 110, 81, 57, 143, 98] };
}
#[repr(C)]
pub struct IMILBitmapEffectImpl {
    pub base__: ::windows_sys::core::IUnknown,
    pub IsInPlaceModificationAllowed: unsafe extern "system" fn(this: *mut *mut Self, poutputconnector: *mut ::core::ffi::c_void, pfmodifyinplace: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetParentEffect: unsafe extern "system" fn(this: *mut *mut Self, pparenteffect: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetInputSource: unsafe extern "system" fn(this: *mut *mut Self, uiindex: u32, ppbitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetInputSource: usize,
    pub GetInputSourceBounds: unsafe extern "system" fn(this: *mut *mut Self, uiindex: u32, prect: *mut MilRectD) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetInputBitmapSource: unsafe extern "system" fn(this: *mut *mut Self, uiindex: u32, prendercontext: *mut ::core::ffi::c_void, pfmodifyinplace: *mut i16, ppbitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetInputBitmapSource: usize,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetOutputBitmapSource: unsafe extern "system" fn(this: *mut *mut Self, uiindex: u32, prendercontext: *mut ::core::ffi::c_void, pfmodifyinplace: *mut i16, ppbitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetOutputBitmapSource: usize,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, pinner: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMILBitmapEffectImpl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3424938226, data2: 39222, data3: 18366, data4: [180, 175, 6, 181, 223, 93, 188, 187] };
}
#[repr(C)]
pub struct IMILBitmapEffectInputConnector {
    pub base__: IMILBitmapEffectConnector,
    pub ConnectTo: unsafe extern "system" fn(this: *mut *mut Self, pconnector: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetConnection: unsafe extern "system" fn(this: *mut *mut Self, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMILBitmapEffectInputConnector {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2847206570, data2: 31292, data3: 17895, data4: [133, 115, 244, 184, 27, 96, 221, 108] };
}
#[repr(C)]
pub struct IMILBitmapEffectInteriorInputConnector {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetInputConnector: unsafe extern "system" fn(this: *mut *mut Self, pinputconnector: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMILBitmapEffectInteriorInputConnector {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 539524766, data2: 34466, data3: 19989, data4: [149, 61, 235, 20, 56, 165, 184, 66] };
}
#[repr(C)]
pub struct IMILBitmapEffectInteriorOutputConnector {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetOutputConnector: unsafe extern "system" fn(this: *mut *mut Self, poutputconnector: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMILBitmapEffectInteriorOutputConnector {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 12302044, data2: 44233, data3: 19452, data4: [179, 68, 139, 238, 56, 61, 254, 250] };
}
#[repr(C)]
pub struct IMILBitmapEffectOutputConnector {
    pub base__: IMILBitmapEffectConnector,
    pub GetNumberConnections: unsafe extern "system" fn(this: *mut *mut Self, puinumberconnections: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetConnection: unsafe extern "system" fn(this: *mut *mut Self, uiindex: u32, ppconnection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMILBitmapEffectOutputConnector {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2459269805, data2: 33819, data3: 18534, data4: [130, 236, 135, 82, 70, 139, 7, 253] };
}
#[repr(C)]
pub struct IMILBitmapEffectOutputConnectorImpl {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddBackLink: unsafe extern "system" fn(this: *mut *mut Self, pconnection: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveBackLink: unsafe extern "system" fn(this: *mut *mut Self, pconnection: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMILBitmapEffectOutputConnectorImpl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 570091383, data2: 35641, data3: 19450, data4: [159, 45, 243, 148, 30, 211, 105, 19] };
}
#[repr(C)]
pub struct IMILBitmapEffectPrimitive {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetOutput: unsafe extern "system" fn(this: *mut *mut Self, uiindex: u32, pcontext: *mut ::core::ffi::c_void, pfmodifyinplace: *mut i16, ppbitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetOutput: usize,
    pub TransformPoint: unsafe extern "system" fn(this: *mut *mut Self, uiindex: u32, p: *mut MilPoint2D, fforwardtransform: i16, pcontext: *mut ::core::ffi::c_void, pfpointtransformed: *mut i16) -> ::windows_sys::core::HRESULT,
    pub TransformRect: unsafe extern "system" fn(this: *mut *mut Self, uiindex: u32, p: *mut MilRectD, fforwardtransform: i16, pcontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HasAffineTransform: unsafe extern "system" fn(this: *mut *mut Self, uiindex: u32, pfaffine: *mut i16) -> ::windows_sys::core::HRESULT,
    pub HasInverseTransform: unsafe extern "system" fn(this: *mut *mut Self, uiindex: u32, pfhasinverse: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dwm")]
    pub GetAffineMatrix: unsafe extern "system" fn(this: *mut *mut Self, uiindex: u32, pmatrix: *mut super::super::Graphics::Dwm::MilMatrix3x2D) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dwm"))]
    GetAffineMatrix: usize,
}
impl ::windows_sys::core::Interface for IMILBitmapEffectPrimitive {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1742934053, data2: 12433, data3: 19964, data4: [152, 214, 221, 73, 69, 81, 70, 29] };
}
#[repr(C)]
pub struct IMILBitmapEffectPrimitiveImpl {
    pub base__: ::windows_sys::core::IUnknown,
    pub IsDirty: unsafe extern "system" fn(this: *mut *mut Self, uioutputindex: u32, pfdirty: *mut i16) -> ::windows_sys::core::HRESULT,
    pub IsVolatile: unsafe extern "system" fn(this: *mut *mut Self, uioutputindex: u32, pfvolatile: *mut i16) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMILBitmapEffectPrimitiveImpl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3460423691, data2: 61350, data3: 17639, data4: [176, 7, 221, 4, 46, 58, 225, 38] };
}
#[repr(C)]
pub struct IMILBitmapEffectRenderContext {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetOutputPixelFormat: unsafe extern "system" fn(this: *mut *mut Self, format: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetOutputPixelFormat: unsafe extern "system" fn(this: *mut *mut Self, pformat: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetUseSoftwareRenderer: unsafe extern "system" fn(this: *mut *mut Self, fsoftware: i16) -> ::windows_sys::core::HRESULT,
    pub SetInitialTransform: unsafe extern "system" fn(this: *mut *mut Self, pmatrix: *const MILMatrixF) -> ::windows_sys::core::HRESULT,
    pub GetFinalTransform: unsafe extern "system" fn(this: *mut *mut Self, pmatrix: *mut MILMatrixF) -> ::windows_sys::core::HRESULT,
    pub SetOutputDPI: unsafe extern "system" fn(this: *mut *mut Self, dbldpix: f64, dbldpiy: f64) -> ::windows_sys::core::HRESULT,
    pub GetOutputDPI: unsafe extern "system" fn(this: *mut *mut Self, pdbldpix: *mut f64, pdbldpiy: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetRegionOfInterest: unsafe extern "system" fn(this: *mut *mut Self, prect: *const MilRectD) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMILBitmapEffectRenderContext {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 312667262, data2: 11571, data3: 17586, data4: [179, 52, 26, 187, 120, 70, 227, 144] };
}
#[repr(C)]
pub struct IMILBitmapEffectRenderContextImpl {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetUseSoftwareRenderer: unsafe extern "system" fn(this: *mut *mut Self, pfsoftware: *mut i16) -> ::windows_sys::core::HRESULT,
    pub GetTransform: unsafe extern "system" fn(this: *mut *mut Self, pmatrix: *mut MILMatrixF) -> ::windows_sys::core::HRESULT,
    pub UpdateTransform: unsafe extern "system" fn(this: *mut *mut Self, pmatrix: *const MILMatrixF) -> ::windows_sys::core::HRESULT,
    pub GetOutputBounds: unsafe extern "system" fn(this: *mut *mut Self, prect: *mut MilRectD) -> ::windows_sys::core::HRESULT,
    pub UpdateOutputBounds: unsafe extern "system" fn(this: *mut *mut Self, prect: *const MilRectD) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMILBitmapEffectRenderContextImpl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1294314699, data2: 31101, data3: 20434, data4: [177, 40, 223, 254, 255, 132, 252, 195] };
}
#[repr(C)]
pub struct IMILBitmapEffects {
    pub base__: ::windows_sys::core::IUnknown,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppiureturn: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Parent: unsafe extern "system" fn(this: *mut *mut Self, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, uindex: u32, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, puicount: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMILBitmapEffects {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1370242510, data2: 26565, data3: 17547, data4: [145, 128, 173, 62, 171, 221, 213, 221] };
}
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
pub const MILBITMAPEFFECT_SDK_VERSION: u32 = 16777216u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
pub struct MILMatrixF {
    pub _11: f64,
    pub _12: f64,
    pub _13: f64,
    pub _14: f64,
    pub _21: f64,
    pub _22: f64,
    pub _23: f64,
    pub _24: f64,
    pub _31: f64,
    pub _32: f64,
    pub _33: f64,
    pub _34: f64,
    pub _41: f64,
    pub _42: f64,
    pub _43: f64,
    pub _44: f64,
}
impl ::core::marker::Copy for MILMatrixF {}
impl ::core::clone::Clone for MILMatrixF {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
pub struct MilPoint2D {
    pub X: f64,
    pub Y: f64,
}
impl ::core::marker::Copy for MilPoint2D {}
impl ::core::clone::Clone for MilPoint2D {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
pub struct MilRectD {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}
impl ::core::marker::Copy for MilRectD {}
impl ::core::clone::Clone for MilRectD {
    fn clone(&self) -> Self {
        *self
    }
}
