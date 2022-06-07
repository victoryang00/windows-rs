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
impl ::windows_sys::core::Interface for IDirect3D11CaptureFrame {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4199597603, data2: 14554, data3: 19250, data4: [172, 243, 250, 151, 52, 173, 128, 14] };
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
impl ::windows_sys::core::Interface for IDirect3D11CaptureFramePool {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 619408674, data2: 6517, data3: 16942, data4: [130, 231, 120, 13, 189, 141, 223, 36] };
}
#[repr(C)]
pub struct IDirect3D11CaptureFramePoolStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, device: *mut ::core::ffi::c_void, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for IDirect3D11CaptureFramePoolStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2005140842, data2: 26538, data3: 19795, data4: [174, 84, 16, 136, 213, 168, 202, 33] };
}
#[repr(C)]
pub struct IDirect3D11CaptureFramePoolStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateFreeThreaded: unsafe extern "system" fn(this: *mut *mut Self, device: *mut ::core::ffi::c_void, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateFreeThreaded: usize,
}
impl ::windows_sys::core::Interface for IDirect3D11CaptureFramePoolStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1486557247, data2: 27580, data3: 24053, data4: [169, 145, 2, 226, 139, 59, 102, 213] };
}
#[repr(C)]
pub struct IGraphicsCaptureAccessStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Security_Authorization_AppCapabilityAccess"))]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut *mut Self, request: GraphicsCaptureAccessKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Authorization_AppCapabilityAccess")))]
    RequestAccessAsync: usize,
}
impl ::windows_sys::core::Interface for IGraphicsCaptureAccessStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1950274416, data2: 1772, data3: 20544, data4: [165, 138, 144, 31, 15, 117, 112, 149] };
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
impl ::windows_sys::core::Interface for IGraphicsCaptureItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2042886491, data2: 12791, data3: 20162, data4: [164, 100, 99, 46, 245, 211, 7, 96] };
}
#[repr(C)]
pub struct IGraphicsCaptureItemStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Composition")]
    pub CreateFromVisual: unsafe extern "system" fn(this: *mut *mut Self, visual: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateFromVisual: usize,
}
impl ::windows_sys::core::Interface for IGraphicsCaptureItemStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2826878629, data2: 17788, data3: 22408, data4: [171, 71, 12, 241, 211, 99, 126, 116] };
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
impl ::windows_sys::core::Interface for IGraphicsCaptureItemStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 999468233, data2: 58756, data3: 22626, data4: [191, 92, 156, 49, 108, 109, 45, 187] };
}
#[repr(C)]
pub struct IGraphicsCapturePicker {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PickSingleItemAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickSingleItemAsync: usize,
}
impl ::windows_sys::core::Interface for IGraphicsCapturePicker {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1511461299, data2: 44409, data3: 19274, data4: [147, 54, 19, 24, 253, 222, 53, 57] };
}
#[repr(C)]
pub struct IGraphicsCaptureSession {
    pub base__: ::windows_sys::core::IInspectable,
    pub StartCapture: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGraphicsCaptureSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2169389737, data2: 63247, data3: 19159, data4: [147, 155, 253, 220, 198, 235, 136, 13] };
}
#[repr(C)]
pub struct IGraphicsCaptureSession2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsCursorCaptureEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsCursorCaptureEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGraphicsCaptureSession2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 741977664, data2: 32046, data3: 20548, data4: [128, 78, 139, 103, 153, 212, 207, 158] };
}
#[repr(C)]
pub struct IGraphicsCaptureSession3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsBorderRequired: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsBorderRequired: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGraphicsCaptureSession3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4073576806, data2: 8878, data3: 24225, data4: [149, 150, 58, 40, 147, 68, 195, 190] };
}
#[repr(C)]
pub struct IGraphicsCaptureSessionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGraphicsCaptureSessionStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 572826944, data2: 22900, data3: 18858, data4: [178, 50, 8, 130, 83, 111, 76, 181] };
}
