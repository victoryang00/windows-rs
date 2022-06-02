pub type Direct3D11CaptureFrame = *mut ::core::ffi::c_void;
pub type Direct3D11CaptureFramePool = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Capture\"`*"]
#[repr(transparent)]
pub struct GraphicsCaptureAccessKind(pub i32);
impl GraphicsCaptureAccessKind {
    pub const Borderless: Self = Self(0i32);
    pub const Programmatic: Self = Self(1i32);
}
impl ::core::marker::Copy for GraphicsCaptureAccessKind {}
impl ::core::clone::Clone for GraphicsCaptureAccessKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GraphicsCaptureItem = *mut ::core::ffi::c_void;
pub type GraphicsCapturePicker = *mut ::core::ffi::c_void;
pub type GraphicsCaptureSession = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IDirect3D11CaptureFrame {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Surface: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Surface: usize,
    #[cfg(feature = "Foundation")]
    pub SystemRelativeTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemRelativeTime: usize,
    pub ContentSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::SizeInt32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDirect3D11CaptureFramePool {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Recreate: unsafe extern "system" fn(this: *mut *mut Self, device: *mut ::core::ffi::c_void, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Recreate: usize,
    pub TryGetNextFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FrameArrived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FrameArrived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFrameArrived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFrameArrived: usize,
    pub CreateCaptureSession: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub DispatcherQueue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    DispatcherQueue: usize,
}
#[repr(C)]
pub struct IDirect3D11CaptureFramePoolStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, device: *mut ::core::ffi::c_void, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Create: usize,
}
#[repr(C)]
pub struct IDirect3D11CaptureFramePoolStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateFreeThreaded: unsafe extern "system" fn(this: *mut *mut Self, device: *mut ::core::ffi::c_void, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateFreeThreaded: usize,
}
#[repr(C)]
pub struct IGraphicsCaptureAccessStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Security_Authorization_AppCapabilityAccess"))]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut *mut Self, request: GraphicsCaptureAccessKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Authorization_AppCapabilityAccess")))]
    RequestAccessAsync: usize,
}
#[repr(C)]
pub struct IGraphicsCaptureItem {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::SizeInt32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
}
#[repr(C)]
pub struct IGraphicsCaptureItemStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Composition")]
    pub CreateFromVisual: unsafe extern "system" fn(this: *mut *mut Self, visual: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateFromVisual: usize,
}
#[repr(C)]
pub struct IGraphicsCaptureItemStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI")]
    pub TryCreateFromWindowId: unsafe extern "system" fn(this: *mut *mut Self, windowid: super::super::UI::WindowId, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    TryCreateFromWindowId: usize,
    pub TryCreateFromDisplayId: unsafe extern "system" fn(this: *mut *mut Self, displayid: super::DisplayId, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGraphicsCapturePicker {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PickSingleItemAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickSingleItemAsync: usize,
}
#[repr(C)]
pub struct IGraphicsCaptureSession {
    pub base__: ::windows_sys::core::IInspectable,
    pub StartCapture: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGraphicsCaptureSession2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsCursorCaptureEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsCursorCaptureEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGraphicsCaptureSession3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsBorderRequired: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsBorderRequired: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGraphicsCaptureSessionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
