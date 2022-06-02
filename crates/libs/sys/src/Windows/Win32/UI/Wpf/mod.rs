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
#[repr(C)]
pub struct IMILBitmapEffectConnections {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetInputConnector: unsafe extern "system" fn(this: *mut *mut Self, uiindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetOutputConnector: unsafe extern "system" fn(this: *mut *mut Self, uiindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMILBitmapEffectConnectionsInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetNumberInputs: unsafe extern "system" fn(this: *mut *mut Self, puinuminputs: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetNumberOutputs: unsafe extern "system" fn(this: *mut *mut Self, puinumoutputs: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetInputConnectorInfo: unsafe extern "system" fn(this: *mut *mut Self, uiindex: u32, ppconnectorinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetOutputConnectorInfo: unsafe extern "system" fn(this: *mut *mut Self, uiindex: u32, ppconnectorinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMILBitmapEffectConnector {
    pub base__: IMILBitmapEffectConnectorInfo,
    pub IsConnected: unsafe extern "system" fn(this: *mut *mut Self, pfconnected: *mut i16) -> ::windows_sys::core::HRESULT,
    pub GetBitmapEffect: unsafe extern "system" fn(this: *mut *mut Self, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMILBitmapEffectConnectorInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetIndex: unsafe extern "system" fn(this: *mut *mut Self, puiindex: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetOptimalFormat: unsafe extern "system" fn(this: *mut *mut Self, pformat: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetNumberFormats: unsafe extern "system" fn(this: *mut *mut Self, pulnumberformats: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetFormat: unsafe extern "system" fn(this: *mut *mut Self, ulindex: u32, pformat: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IMILBitmapEffectFactory {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateEffect: unsafe extern "system" fn(this: *mut *mut Self, pguideffect: *const ::windows_sys::core::GUID, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateContext: unsafe extern "system" fn(this: *mut *mut Self, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateEffectOuter: unsafe extern "system" fn(this: *mut *mut Self, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMILBitmapEffectGroup {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetInteriorInputConnector: unsafe extern "system" fn(this: *mut *mut Self, uiindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetInteriorOutputConnector: unsafe extern "system" fn(this: *mut *mut Self, uiindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, peffect: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMILBitmapEffectGroupImpl {
    pub base__: ::windows_sys::core::IUnknown,
    pub Preprocess: unsafe extern "system" fn(this: *mut *mut Self, pcontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetNumberChildren: unsafe extern "system" fn(this: *mut *mut Self, puinumberchildren: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetChildren: unsafe extern "system" fn(this: *mut *mut Self, pchildren: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IMILBitmapEffectInputConnector {
    pub base__: IMILBitmapEffectConnector,
    pub ConnectTo: unsafe extern "system" fn(this: *mut *mut Self, pconnector: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetConnection: unsafe extern "system" fn(this: *mut *mut Self, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMILBitmapEffectInteriorInputConnector {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetInputConnector: unsafe extern "system" fn(this: *mut *mut Self, pinputconnector: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMILBitmapEffectInteriorOutputConnector {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetOutputConnector: unsafe extern "system" fn(this: *mut *mut Self, poutputconnector: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMILBitmapEffectOutputConnector {
    pub base__: IMILBitmapEffectConnector,
    pub GetNumberConnections: unsafe extern "system" fn(this: *mut *mut Self, puinumberconnections: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetConnection: unsafe extern "system" fn(this: *mut *mut Self, uiindex: u32, ppconnection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMILBitmapEffectOutputConnectorImpl {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddBackLink: unsafe extern "system" fn(this: *mut *mut Self, pconnection: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveBackLink: unsafe extern "system" fn(this: *mut *mut Self, pconnection: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IMILBitmapEffectPrimitiveImpl {
    pub base__: ::windows_sys::core::IUnknown,
    pub IsDirty: unsafe extern "system" fn(this: *mut *mut Self, uioutputindex: u32, pfdirty: *mut i16) -> ::windows_sys::core::HRESULT,
    pub IsVolatile: unsafe extern "system" fn(this: *mut *mut Self, uioutputindex: u32, pfvolatile: *mut i16) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IMILBitmapEffectRenderContextImpl {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetUseSoftwareRenderer: unsafe extern "system" fn(this: *mut *mut Self, pfsoftware: *mut i16) -> ::windows_sys::core::HRESULT,
    pub GetTransform: unsafe extern "system" fn(this: *mut *mut Self, pmatrix: *mut MILMatrixF) -> ::windows_sys::core::HRESULT,
    pub UpdateTransform: unsafe extern "system" fn(this: *mut *mut Self, pmatrix: *const MILMatrixF) -> ::windows_sys::core::HRESULT,
    pub GetOutputBounds: unsafe extern "system" fn(this: *mut *mut Self, prect: *mut MilRectD) -> ::windows_sys::core::HRESULT,
    pub UpdateOutputBounds: unsafe extern "system" fn(this: *mut *mut Self, prect: *const MilRectD) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMILBitmapEffects {
    pub base__: ::windows_sys::core::IUnknown,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppiureturn: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Parent: unsafe extern "system" fn(this: *mut *mut Self, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, uindex: u32, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, puicount: *mut u32) -> ::windows_sys::core::HRESULT,
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
