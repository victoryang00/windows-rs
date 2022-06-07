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
impl ::windows_sys::core::Interface for IDesktopWindowXamlSourceNative {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1019015615, data2: 12150, data3: 20124, data4: [150, 171, 232, 75, 55, 151, 37, 84] };
}
#[repr(C)]
pub struct IDesktopWindowXamlSourceNative2 {
    pub base__: IDesktopWindowXamlSourceNative,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub PreTranslateMessage: unsafe extern "system" fn(this: *mut *mut Self, message: *const super::super::super::UI::WindowsAndMessaging::MSG, result: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    PreTranslateMessage: usize,
}
impl ::windows_sys::core::Interface for IDesktopWindowXamlSourceNative2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3822901447, data2: 12375, data3: 18066, data4: [153, 195, 123, 119, 32, 175, 218, 49] };
}
#[repr(C)]
pub struct IFindReferenceTargetsCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub FoundTrackerTarget: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFindReferenceTargetsCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 78858348, data2: 18055, data3: 16937, data4: [141, 20, 80, 90, 181, 132, 221, 136] };
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
impl ::windows_sys::core::Interface for IReferenceTracker {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 299086138, data2: 6158, data3: 18313, data4: [168, 190, 119, 18, 136, 40, 147, 230] };
}
#[repr(C)]
pub struct IReferenceTrackerExtension {
    pub base__: ::windows_sys::core::IUnknown,
}
impl ::windows_sys::core::Interface for IReferenceTrackerExtension {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1317633194, data2: 22997, data3: 17939, data4: [143, 140, 247, 235, 209, 243, 153, 176] };
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
impl ::windows_sys::core::Interface for IReferenceTrackerHost {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 698817642, data2: 15426, data3: 17430, data4: [163, 157, 226, 130, 90, 7, 167, 115] };
}
#[repr(C)]
pub struct IReferenceTrackerManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub ReferenceTrackingStarted: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub FindTrackerTargetsCompleted: unsafe extern "system" fn(this: *mut *mut Self, findfailed: u8) -> ::windows_sys::core::HRESULT,
    pub ReferenceTrackingCompleted: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetReferenceTrackerHost: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IReferenceTrackerManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1022461108, data2: 31947, data3: 19930, data4: [132, 85, 126, 108, 233, 154, 50, 152] };
}
#[repr(C)]
pub struct IReferenceTrackerTarget {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddRefFromReferenceTracker: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub ReleaseFromReferenceTracker: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub Peg: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Unpeg: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IReferenceTrackerTarget {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1690125304, data2: 49134, data3: 20164, data4: [183, 235, 41, 53, 21, 141, 174, 33] };
}
#[repr(C)]
pub struct ISurfaceImageSourceManagerNative {
    pub base__: ::windows_sys::core::IUnknown,
    pub FlushAllSurfacesWithDevice: unsafe extern "system" fn(this: *mut *mut Self, device: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISurfaceImageSourceManagerNative {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1283954871, data2: 7560, data3: 18959, data4: [181, 155, 185, 63, 96, 13, 232, 200] };
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
impl ::windows_sys::core::Interface for ISurfaceImageSourceNative {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075417025, data2: 54023, data3: 17701, data4: [152, 134, 15, 175, 170, 68, 22, 60] };
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
impl ::windows_sys::core::Interface for ISurfaceImageSourceNativeWithD2D {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1412006435, data2: 16865, data3: 19009, data4: [156, 8, 2, 232, 37, 104, 100, 161] };
}
#[repr(C)]
pub struct ISwapChainBackgroundPanelNative {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub SetSwapChain: unsafe extern "system" fn(this: *mut *mut Self, swapchain: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    SetSwapChain: usize,
}
impl ::windows_sys::core::Interface for ISwapChainBackgroundPanelNative {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1136573774, data2: 44501, data3: 16437, data4: [143, 133, 86, 8, 208, 142, 157, 201] };
}
#[repr(C)]
pub struct ISwapChainPanelNative {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub SetSwapChain: unsafe extern "system" fn(this: *mut *mut Self, swapchain: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    SetSwapChain: usize,
}
impl ::windows_sys::core::Interface for ISwapChainPanelNative {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4180613586, data2: 15070, data3: 17830, data4: [162, 12, 246, 241, 234, 144, 85, 75] };
}
#[repr(C)]
pub struct ISwapChainPanelNative2 {
    pub base__: ISwapChainPanelNative,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSwapChainHandle: unsafe extern "system" fn(this: *mut *mut Self, swapchainhandle: super::super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSwapChainHandle: usize,
}
impl ::windows_sys::core::Interface for ISwapChainPanelNative2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3584226828, data2: 14258, data3: 17570, data4: [147, 123, 141, 142, 185, 114, 104, 33] };
}
#[repr(C)]
pub struct ITrackerOwner {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateTrackerHandle: unsafe extern "system" fn(this: *mut *mut Self, returnvalue: *mut *mut TrackerHandle__) -> ::windows_sys::core::HRESULT,
    pub DeleteTrackerHandle: unsafe extern "system" fn(this: *mut *mut Self, handle: *const TrackerHandle__) -> ::windows_sys::core::HRESULT,
    pub SetTrackerValue: unsafe extern "system" fn(this: *mut *mut Self, handle: *const TrackerHandle__, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryGetSafeTrackerValue: unsafe extern "system" fn(this: *mut *mut Self, handle: *const TrackerHandle__, returnvalue: *mut *mut ::core::ffi::c_void) -> u8,
}
impl ::windows_sys::core::Interface for ITrackerOwner {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3945054731, data2: 38934, data3: 19143, data4: [140, 255, 54, 246, 122, 17, 143, 78] };
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
impl ::windows_sys::core::Interface for IVirtualSurfaceImageSourceNative {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3914664323, data2: 13835, data3: 20307, data4: [179, 145, 175, 214, 149, 7, 134, 145] };
}
#[repr(C)]
pub struct IVirtualSurfaceUpdatesCallbackNative {
    pub base__: ::windows_sys::core::IUnknown,
    pub UpdatesNeeded: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVirtualSurfaceUpdatesCallbackNative {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3690129735, data2: 36460, data3: 16980, data4: [158, 238, 119, 56, 247, 19, 134, 201] };
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
