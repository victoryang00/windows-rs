#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub fn CreatePresentationFactory(d3ddevice: *mut *mut ::windows_sys::core::IUnknown, riid: *const ::windows_sys::core::GUID, presentationfactory: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct CompositionFrameDisplayInstance {
    pub displayAdapterLUID: super::super::Foundation::LUID,
    pub displayVidPnSourceId: u32,
    pub displayUniqueId: u32,
    pub renderAdapterLUID: super::super::Foundation::LUID,
    pub instanceKind: CompositionFrameInstanceKind,
    pub finalTransform: PresentationTransform,
    pub requiredCrossAdapterCopy: u8,
    pub colorSpace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for CompositionFrameDisplayInstance {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for CompositionFrameDisplayInstance {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub type CompositionFrameInstanceKind = i32;
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub const CompositionFrameInstanceKind_ComposedOnScreen: CompositionFrameInstanceKind = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub const CompositionFrameInstanceKind_ScanoutOnScreen: CompositionFrameInstanceKind = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub const CompositionFrameInstanceKind_ComposedToIntermediate: CompositionFrameInstanceKind = 2i32;
#[repr(C)]
pub struct ICompositionFramePresentStatistics {
    pub base__: IPresentStatistics,
    pub GetContentTag: unsafe extern "system" fn(this: *mut *mut Self) -> usize,
    pub GetCompositionFrameId: unsafe extern "system" fn(this: *mut *mut Self) -> u64,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetDisplayInstanceArray: unsafe extern "system" fn(this: *mut *mut Self, displayinstancearraycount: *mut u32, displayinstancearray: *mut *mut CompositionFrameDisplayInstance),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    GetDisplayInstanceArray: usize,
}
impl ::windows_sys::core::Interface for ICompositionFramePresentStatistics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2873217319, data2: 49409, data3: 19466, data4: [145, 29, 249, 242, 233, 208, 142, 100] };
}
#[repr(C)]
pub struct IIndependentFlipFramePresentStatistics {
    pub base__: IPresentStatistics,
    #[cfg(feature = "Win32_Foundation")]
    pub GetOutputAdapterLUID: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::LUID),
    #[cfg(not(feature = "Win32_Foundation"))]
    GetOutputAdapterLUID: usize,
    pub GetOutputVidPnSourceId: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetContentTag: unsafe extern "system" fn(this: *mut *mut Self) -> usize,
    pub GetDisplayedTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SystemInterruptTime),
    pub GetPresentDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SystemInterruptTime),
}
impl ::windows_sys::core::Interface for IIndependentFlipFramePresentStatistics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2358492711, data2: 44436, data3: 19872, data4: [143, 212, 36, 19, 19, 45, 18, 78] };
}
#[repr(C)]
pub struct IPresentStatistics {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetPresentId: unsafe extern "system" fn(this: *mut *mut Self) -> u64,
    pub GetKind: unsafe extern "system" fn(this: *mut *mut Self) -> PresentStatisticsKind,
}
impl ::windows_sys::core::Interface for IPresentStatistics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3024849882, data2: 29314, data3: 18781, data4: [157, 215, 206, 173, 216, 180, 187, 134] };
}
#[repr(C)]
pub struct IPresentStatusPresentStatistics {
    pub base__: IPresentStatistics,
    pub GetCompositionFrameId: unsafe extern "system" fn(this: *mut *mut Self) -> u64,
    pub GetPresentStatus: unsafe extern "system" fn(this: *mut *mut Self) -> PresentStatus,
}
impl ::windows_sys::core::Interface for IPresentStatusPresentStatistics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3387763265, data2: 31179, data3: 17246, data4: [150, 78, 200, 85, 48, 85, 66, 12] };
}
#[repr(C)]
pub struct IPresentationBuffer {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAvailableEvent: unsafe extern "system" fn(this: *mut *mut Self, availableeventhandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAvailableEvent: usize,
    pub IsAvailable: unsafe extern "system" fn(this: *mut *mut Self, isavailable: *mut u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPresentationBuffer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 773946682, data2: 23227, data3: 16696, data4: [154, 19, 167, 117, 89, 60, 137, 202] };
}
#[repr(C)]
pub struct IPresentationContent {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetTag: unsafe extern "system" fn(this: *mut *mut Self, tag: usize),
}
impl ::windows_sys::core::Interface for IPresentationContent {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1449704313, data2: 15758, data3: 16732, data4: [178, 21, 243, 128, 32, 242, 210, 82] };
}
#[repr(C)]
pub struct IPresentationFactory {
    pub base__: ::windows_sys::core::IUnknown,
    pub IsPresentationSupported: unsafe extern "system" fn(this: *mut *mut Self) -> u8,
    pub IsPresentationSupportedWithIndependentFlip: unsafe extern "system" fn(this: *mut *mut Self) -> u8,
    pub CreatePresentationManager: unsafe extern "system" fn(this: *mut *mut Self, pppresentationmanager: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPresentationFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2410904408, data2: 7540, data3: 20324, data4: [164, 156, 31, 151, 168, 10, 46, 192] };
}
#[repr(C)]
pub struct IPresentationManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddBufferFromResource: unsafe extern "system" fn(this: *mut *mut Self, resource: *mut ::core::ffi::c_void, presentationbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreatePresentationSurface: unsafe extern "system" fn(this: *mut *mut Self, compositionsurfacehandle: super::super::Foundation::HANDLE, presentationsurface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreatePresentationSurface: usize,
    pub GetNextPresentId: unsafe extern "system" fn(this: *mut *mut Self) -> u64,
    pub SetTargetTime: unsafe extern "system" fn(this: *mut *mut Self, targettime: SystemInterruptTime) -> ::windows_sys::core::HRESULT,
    pub SetPreferredPresentDuration: unsafe extern "system" fn(this: *mut *mut Self, preferredduration: SystemInterruptTime, deviationtolerance: SystemInterruptTime) -> ::windows_sys::core::HRESULT,
    pub ForceVSyncInterrupt: unsafe extern "system" fn(this: *mut *mut Self, forcevsyncinterrupt: u8) -> ::windows_sys::core::HRESULT,
    pub Present: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetPresentRetiringFence: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, fence: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CancelPresentsFrom: unsafe extern "system" fn(this: *mut *mut Self, presentidtocancelfrom: u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLostEvent: unsafe extern "system" fn(this: *mut *mut Self, losteventhandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLostEvent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPresentStatisticsAvailableEvent: unsafe extern "system" fn(this: *mut *mut Self, presentstatisticsavailableeventhandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPresentStatisticsAvailableEvent: usize,
    pub EnablePresentStatisticsKind: unsafe extern "system" fn(this: *mut *mut Self, presentstatisticskind: PresentStatisticsKind, enabled: u8) -> ::windows_sys::core::HRESULT,
    pub GetNextPresentStatistics: unsafe extern "system" fn(this: *mut *mut Self, nextpresentstatistics: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPresentationManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4216729474, data2: 25234, data3: 18186, data4: [136, 177, 132, 54, 97, 231, 242, 12] };
}
#[repr(C)]
pub struct IPresentationSurface {
    pub base__: IPresentationContent,
    pub SetBuffer: unsafe extern "system" fn(this: *mut *mut Self, presentationbuffer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub SetColorSpace: unsafe extern "system" fn(this: *mut *mut Self, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    SetColorSpace: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub SetAlphaMode: unsafe extern "system" fn(this: *mut *mut Self, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    SetAlphaMode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSourceRect: unsafe extern "system" fn(this: *mut *mut Self, sourcerect: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSourceRect: usize,
    pub SetTransform: unsafe extern "system" fn(this: *mut *mut Self, transform: *const PresentationTransform) -> ::windows_sys::core::HRESULT,
    pub RestrictToOutput: unsafe extern "system" fn(this: *mut *mut Self, output: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetDisableReadback: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
    pub SetLetterboxingMargins: unsafe extern "system" fn(this: *mut *mut Self, leftletterboxsize: f32, topletterboxsize: f32, rightletterboxsize: f32, bottomletterboxsize: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPresentationSurface {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2506559739, data2: 59968, data3: 20154, data4: [163, 235, 67, 117, 160, 235, 78, 220] };
}
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub type PresentStatisticsKind = i32;
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub const PresentStatisticsKind_PresentStatus: PresentStatisticsKind = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub const PresentStatisticsKind_CompositionFrame: PresentStatisticsKind = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub const PresentStatisticsKind_IndependentFlipFrame: PresentStatisticsKind = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub type PresentStatus = i32;
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub const PresentStatus_Queued: PresentStatus = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub const PresentStatus_Skipped: PresentStatus = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub const PresentStatus_Canceled: PresentStatus = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub struct PresentationTransform {
    pub M11: f32,
    pub M12: f32,
    pub M21: f32,
    pub M22: f32,
    pub M31: f32,
    pub M32: f32,
}
impl ::core::marker::Copy for PresentationTransform {}
impl ::core::clone::Clone for PresentationTransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub struct SystemInterruptTime {
    pub value: u64,
}
impl ::core::marker::Copy for SystemInterruptTime {}
impl ::core::clone::Clone for SystemInterruptTime {
    fn clone(&self) -> Self {
        *self
    }
}
