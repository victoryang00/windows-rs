#[doc = "*Required features: `\"Win32_System_WinRT_Xaml\"`*"]
pub const E_SURFACE_CONTENTS_LOST: u32 = 2150301728u32;
#[repr(C)]
pub struct IDesktopWindowXamlSourceNative {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub AttachToWindow: unsafe extern "system" fn(this: *mut *mut Self, parentwnd: super::super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AttachToWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub WindowHandle: unsafe extern "system" fn(this: *mut *mut Self, hwnd: *mut super::super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WindowHandle: usize,
}
#[repr(C)]
pub struct IDesktopWindowXamlSourceNative2 {
    pub base__: IDesktopWindowXamlSourceNative,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub PreTranslateMessage: unsafe extern "system" fn(this: *mut *mut Self, message: *const super::super::super::UI::WindowsAndMessaging::MSG, result: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    PreTranslateMessage: usize,
}
#[repr(C)]
pub struct IFindReferenceTargetsCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub FoundTrackerTarget: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IReferenceTracker {
    pub base__: ::windows_sys::core::IUnknown,
    pub ConnectFromTrackerSource: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub DisconnectFromTrackerSource: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub FindTrackerTargets: unsafe extern "system" fn(this: *mut *mut Self, callback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetReferenceTrackerManager: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddRefFromTrackerSource: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ReleaseFromTrackerSource: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub PegFromTrackerSource: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IReferenceTrackerExtension {
    pub base__: ::windows_sys::core::IUnknown,
}
#[repr(C)]
pub struct IReferenceTrackerHost {
    pub base__: ::windows_sys::core::IUnknown,
    pub DisconnectUnusedReferenceSources: unsafe extern "system" fn(this: *mut *mut Self, options: XAML_REFERENCETRACKER_DISCONNECT) -> ::windows_sys::core::HRESULT,
    pub ReleaseDisconnectedReferenceSources: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub NotifyEndOfReferenceTrackingOnThread: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetTrackerTarget: unsafe extern "system" fn(this: *mut *mut Self, unknown: *mut ::core::ffi::c_void, newreference: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddMemoryPressure: unsafe extern "system" fn(this: *mut *mut Self, bytesallocated: u64) -> ::windows_sys::core::HRESULT,
    pub RemoveMemoryPressure: unsafe extern "system" fn(this: *mut *mut Self, bytesallocated: u64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IReferenceTrackerManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub ReferenceTrackingStarted: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub FindTrackerTargetsCompleted: unsafe extern "system" fn(this: *mut *mut Self, findfailed: u8) -> ::windows_sys::core::HRESULT,
    pub ReferenceTrackingCompleted: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetReferenceTrackerHost: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IReferenceTrackerTarget {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddRefFromReferenceTracker: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub ReleaseFromReferenceTracker: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub Peg: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Unpeg: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISurfaceImageSourceManagerNative {
    pub base__: ::windows_sys::core::IUnknown,
    pub FlushAllSurfacesWithDevice: unsafe extern "system" fn(this: *mut *mut Self, device: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISurfaceImageSourceNative {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub SetDevice: unsafe extern "system" fn(this: *mut *mut Self, device: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    SetDevice: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
    pub BeginDraw: unsafe extern "system" fn(this: *mut *mut Self, updaterect: super::super::super::Foundation::RECT, surface: *mut *mut ::core::ffi::c_void, offset: *mut super::super::super::Foundation::POINT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi")))]
    BeginDraw: usize,
    pub EndDraw: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISurfaceImageSourceNativeWithD2D {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetDevice: unsafe extern "system" fn(this: *mut *mut Self, device: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginDraw: unsafe extern "system" fn(this: *mut *mut Self, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows_sys::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, offset: *mut super::super::super::Foundation::POINT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginDraw: usize,
    pub EndDraw: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SuspendDraw: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ResumeDraw: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISwapChainBackgroundPanelNative {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub SetSwapChain: unsafe extern "system" fn(this: *mut *mut Self, swapchain: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    SetSwapChain: usize,
}
#[repr(C)]
pub struct ISwapChainPanelNative {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub SetSwapChain: unsafe extern "system" fn(this: *mut *mut Self, swapchain: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    SetSwapChain: usize,
}
#[repr(C)]
pub struct ISwapChainPanelNative2 {
    pub base__: ISwapChainPanelNative,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSwapChainHandle: unsafe extern "system" fn(this: *mut *mut Self, swapchainhandle: super::super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSwapChainHandle: usize,
}
#[repr(C)]
pub struct ITrackerOwner {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateTrackerHandle: unsafe extern "system" fn(this: *mut *mut Self, returnvalue: *mut *mut TrackerHandle__) -> ::windows_sys::core::HRESULT,
    pub DeleteTrackerHandle: unsafe extern "system" fn(this: *mut *mut Self, handle: *const TrackerHandle__) -> ::windows_sys::core::HRESULT,
    pub SetTrackerValue: unsafe extern "system" fn(this: *mut *mut Self, handle: *const TrackerHandle__, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryGetSafeTrackerValue: unsafe extern "system" fn(this: *mut *mut Self, handle: *const TrackerHandle__, returnvalue: *mut *mut ::core::ffi::c_void) -> u8,
}
#[repr(C)]
pub struct IVirtualSurfaceImageSourceNative {
    pub base__: ISurfaceImageSourceNative,
    #[cfg(feature = "Win32_Foundation")]
    pub Invalidate: unsafe extern "system" fn(this: *mut *mut Self, updaterect: super::super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Invalidate: usize,
    pub GetUpdateRectCount: unsafe extern "system" fn(this: *mut *mut Self, count: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetUpdateRects: unsafe extern "system" fn(this: *mut *mut Self, updates: *mut super::super::super::Foundation::RECT, count: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetUpdateRects: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetVisibleBounds: unsafe extern "system" fn(this: *mut *mut Self, bounds: *mut super::super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetVisibleBounds: usize,
    pub RegisterForUpdatesNeeded: unsafe extern "system" fn(this: *mut *mut Self, callback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Resize: unsafe extern "system" fn(this: *mut *mut Self, newwidth: i32, newheight: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVirtualSurfaceUpdatesCallbackNative {
    pub base__: ::windows_sys::core::IUnknown,
    pub UpdatesNeeded: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT_Xaml\"`*"]
pub struct TrackerHandle__ {
    pub unused: i32,
}
impl ::core::marker::Copy for TrackerHandle__ {}
impl ::core::clone::Clone for TrackerHandle__ {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Xaml\"`*"]
pub type XAML_REFERENCETRACKER_DISCONNECT = i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Xaml\"`*"]
pub const XAML_REFERENCETRACKER_DISCONNECT_DEFAULT: XAML_REFERENCETRACKER_DISCONNECT = 0i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Xaml\"`*"]
pub const XAML_REFERENCETRACKER_DISCONNECT_SUSPEND: XAML_REFERENCETRACKER_DISCONNECT = 1i32;
