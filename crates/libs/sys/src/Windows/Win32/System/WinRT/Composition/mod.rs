#[repr(C)]
pub struct ICompositionCapabilitiesInteropFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))]
    pub GetForWindow: unsafe extern "system" fn(this: *mut *mut Self, hwnd: super::super::super::Foundation::HWND, result: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "Win32_Foundation")))]
    GetForWindow: usize,
}
impl ::windows_sys::core::Interface for ICompositionCapabilitiesInteropFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 748532566, data2: 59149, data3: 17986, data4: [130, 152, 188, 74, 165, 180, 134, 92] };
}
#[repr(C)]
pub struct ICompositionDrawingSurfaceInterop {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginDraw: unsafe extern "system" fn(this: *mut *mut Self, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows_sys::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::super::Foundation::POINT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginDraw: usize,
    pub EndDraw: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Resize: unsafe extern "system" fn(this: *mut *mut Self, sizepixels: super::super::super::Foundation::SIZE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Resize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Scroll: unsafe extern "system" fn(this: *mut *mut Self, scrollrect: *const super::super::super::Foundation::RECT, cliprect: *const super::super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Scroll: usize,
    pub ResumeDraw: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SuspendDraw: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionDrawingSurfaceInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4244956899, data2: 65036, data3: 19516, data4: [171, 25, 160, 118, 1, 165, 118, 238] };
}
#[repr(C)]
pub struct ICompositionDrawingSurfaceInterop2 {
    pub base__: ICompositionDrawingSurfaceInterop,
    #[cfg(feature = "Win32_Foundation")]
    pub CopySurface: unsafe extern "system" fn(this: *mut *mut Self, destinationresource: *mut ::core::ffi::c_void, destinationoffsetx: i32, destinationoffsety: i32, sourcerectangle: *const super::super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CopySurface: usize,
}
impl ::windows_sys::core::Interface for ICompositionDrawingSurfaceInterop2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1105611438, data2: 39104, data3: 16953, data4: [142, 149, 163, 48, 221, 106, 161, 139] };
}
#[repr(C)]
pub struct ICompositionGraphicsDeviceInterop {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetRenderingDevice: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRenderingDevice: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionGraphicsDeviceInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2702638961, data2: 63679, data3: 19594, data4: [156, 152, 112, 119, 154, 50, 169, 200] };
}
#[repr(C)]
pub struct ICompositorDesktopInterop {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "UI_Composition_Desktop", feature = "Win32_Foundation"))]
    pub CreateDesktopWindowTarget: unsafe extern "system" fn(this: *mut *mut Self, hwndtarget: super::super::super::Foundation::HWND, istopmost: super::super::super::Foundation::BOOL, result: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition_Desktop", feature = "Win32_Foundation")))]
    CreateDesktopWindowTarget: usize,
    pub EnsureOnThread: unsafe extern "system" fn(this: *mut *mut Self, threadid: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositorDesktopInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 702976506, data2: 17767, data3: 19914, data4: [179, 25, 208, 242, 7, 235, 104, 7] };
}
#[repr(C)]
pub struct ICompositorInterop {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))]
    pub CreateCompositionSurfaceForHandle: unsafe extern "system" fn(this: *mut *mut Self, swapchain: super::super::super::Foundation::HANDLE, result: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "Win32_Foundation")))]
    CreateCompositionSurfaceForHandle: usize,
    #[cfg(feature = "UI_Composition")]
    pub CreateCompositionSurfaceForSwapChain: unsafe extern "system" fn(this: *mut *mut Self, swapchain: *mut ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateCompositionSurfaceForSwapChain: usize,
    #[cfg(feature = "UI_Composition")]
    pub CreateGraphicsDevice: unsafe extern "system" fn(this: *mut *mut Self, renderingdevice: *mut ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateGraphicsDevice: usize,
}
impl ::windows_sys::core::Interface for ICompositorInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 623476060, data2: 15060, data3: 19612, data4: [181, 207, 227, 106, 56, 81, 35, 48] };
}
#[repr(C)]
pub struct IDesktopWindowTargetInterop {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Hwnd: unsafe extern "system" fn(this: *mut *mut Self, value: *mut super::super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Hwnd: usize,
}
impl ::windows_sys::core::Interface for IDesktopWindowTargetInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 903607710, data2: 58361, data3: 17840, data4: [129, 231, 254, 117, 244, 20, 93, 201] };
}
#[repr(C)]
pub struct ISwapChainInterop {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetSwapChain: unsafe extern "system" fn(this: *mut *mut Self, swapchain: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISwapChainInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 653563552, data2: 32568, data3: 17915, data4: [136, 247, 250, 170, 190, 103, 221, 89] };
}
#[repr(C)]
pub struct IVisualInteractionSourceInterop {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
    pub TryRedirectForManipulation: unsafe extern "system" fn(this: *mut *mut Self, pointerinfo: *const super::super::super::UI::Input::Pointer::POINTER_INFO) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging")))]
    TryRedirectForManipulation: usize,
}
impl ::windows_sys::core::Interface for IVisualInteractionSourceInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 301346001, data2: 12189, data3: 17107, data4: [176, 95, 214, 121, 13, 158, 159, 142] };
}
