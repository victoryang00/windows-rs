#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IKnownPerceptionFrameKindStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Color: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Color: usize,
    #[cfg(feature = "deprecated")]
    pub Depth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Depth: usize,
    #[cfg(feature = "deprecated")]
    pub Infrared: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Infrared: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionControlGroup {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub FrameProviderIds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    FrameProviderIds: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionControlGroupFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, ids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Create: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionCorrelation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub TargetId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TargetId: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "deprecated")))]
    Position: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Quaternion) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "deprecated")))]
    Orientation: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionCorrelationFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, targetid: ::windows_sys::core::HSTRING, position: super::super::super::Foundation::Numerics::Vector3, orientation: super::super::super::Foundation::Numerics::Quaternion, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "deprecated")))]
    Create: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionCorrelationGroup {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub RelativeLocations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    RelativeLocations: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionCorrelationGroupFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, relativelocations: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Create: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionFaceAuthenticationGroup {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub FrameProviderIds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    FrameProviderIds: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionFaceAuthenticationGroupFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, ids: *mut ::core::ffi::c_void, starthandler: *mut ::core::ffi::c_void, stophandler: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Create: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionFrame {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RelativeTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RelativeTime: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetRelativeTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetRelativeTime: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Properties: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub FrameData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    FrameData: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionFrameProvider {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub FrameProviderInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FrameProviderInfo: usize,
    #[cfg(feature = "deprecated")]
    pub Available: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Available: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Properties: usize,
    #[cfg(feature = "deprecated")]
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Start: usize,
    #[cfg(feature = "deprecated")]
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Stop: usize,
    #[cfg(feature = "deprecated")]
    pub SetProperty: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetProperty: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionFrameProviderInfo {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Id: usize,
    #[cfg(feature = "deprecated")]
    pub SetId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetId: usize,
    #[cfg(feature = "deprecated")]
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DisplayName: usize,
    #[cfg(feature = "deprecated")]
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetDisplayName: usize,
    #[cfg(feature = "deprecated")]
    pub DeviceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceKind: usize,
    #[cfg(feature = "deprecated")]
    pub SetDeviceKind: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetDeviceKind: usize,
    #[cfg(feature = "deprecated")]
    pub FrameKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FrameKind: usize,
    #[cfg(feature = "deprecated")]
    pub SetFrameKind: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetFrameKind: usize,
    #[cfg(feature = "deprecated")]
    pub Hidden: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Hidden: usize,
    #[cfg(feature = "deprecated")]
    pub SetHidden: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetHidden: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionFrameProviderManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub GetFrameProvider: unsafe extern "system" fn(this: *mut *mut Self, frameproviderinfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetFrameProvider: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionFrameProviderManagerServiceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub RegisterFrameProviderInfo: unsafe extern "system" fn(this: *mut *mut Self, manager: *mut ::core::ffi::c_void, frameproviderinfo: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RegisterFrameProviderInfo: usize,
    #[cfg(feature = "deprecated")]
    pub UnregisterFrameProviderInfo: unsafe extern "system" fn(this: *mut *mut Self, manager: *mut ::core::ffi::c_void, frameproviderinfo: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    UnregisterFrameProviderInfo: usize,
    #[cfg(feature = "deprecated")]
    pub RegisterFaceAuthenticationGroup: unsafe extern "system" fn(this: *mut *mut Self, manager: *mut ::core::ffi::c_void, faceauthenticationgroup: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RegisterFaceAuthenticationGroup: usize,
    #[cfg(feature = "deprecated")]
    pub UnregisterFaceAuthenticationGroup: unsafe extern "system" fn(this: *mut *mut Self, manager: *mut ::core::ffi::c_void, faceauthenticationgroup: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    UnregisterFaceAuthenticationGroup: usize,
    #[cfg(feature = "deprecated")]
    pub RegisterControlGroup: unsafe extern "system" fn(this: *mut *mut Self, manager: *mut ::core::ffi::c_void, controlgroup: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RegisterControlGroup: usize,
    #[cfg(feature = "deprecated")]
    pub UnregisterControlGroup: unsafe extern "system" fn(this: *mut *mut Self, manager: *mut ::core::ffi::c_void, controlgroup: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    UnregisterControlGroup: usize,
    #[cfg(feature = "deprecated")]
    pub RegisterCorrelationGroup: unsafe extern "system" fn(this: *mut *mut Self, manager: *mut ::core::ffi::c_void, correlationgroup: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RegisterCorrelationGroup: usize,
    #[cfg(feature = "deprecated")]
    pub UnregisterCorrelationGroup: unsafe extern "system" fn(this: *mut *mut Self, manager: *mut ::core::ffi::c_void, correlationgroup: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    UnregisterCorrelationGroup: usize,
    #[cfg(feature = "deprecated")]
    pub UpdateAvailabilityForProvider: unsafe extern "system" fn(this: *mut *mut Self, provider: *mut ::core::ffi::c_void, available: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    UpdateAvailabilityForProvider: usize,
    #[cfg(feature = "deprecated")]
    pub PublishFrameForProvider: unsafe extern "system" fn(this: *mut *mut Self, provider: *mut ::core::ffi::c_void, frame: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PublishFrameForProvider: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionPropertyChangeRequest {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Name: usize,
    #[cfg(feature = "deprecated")]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Value: usize,
    #[cfg(feature = "deprecated")]
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::PerceptionFrameSourcePropertyChangeStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Status: usize,
    #[cfg(feature = "deprecated")]
    pub SetStatus: unsafe extern "system" fn(this: *mut *mut Self, value: super::PerceptionFrameSourcePropertyChangeStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetStatus: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetDeferral: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionVideoFrameAllocator {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub AllocateFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AllocateFrame: usize,
    #[cfg(all(feature = "Media", feature = "deprecated"))]
    pub CopyFromVideoFrame: unsafe extern "system" fn(this: *mut *mut Self, frame: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Media", feature = "deprecated")))]
    CopyFromVideoFrame: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPerceptionVideoFrameAllocatorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "deprecated"))]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, maxoutstandingframecountforwrite: u32, format: super::super::super::Graphics::Imaging::BitmapPixelFormat, resolution: super::super::super::Foundation::Size, alpha: super::super::super::Graphics::Imaging::BitmapAlphaMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "deprecated")))]
    Create: usize,
}
pub type PerceptionControlGroup = *mut ::core::ffi::c_void;
pub type PerceptionCorrelation = *mut ::core::ffi::c_void;
pub type PerceptionCorrelationGroup = *mut ::core::ffi::c_void;
pub type PerceptionFaceAuthenticationGroup = *mut ::core::ffi::c_void;
pub type PerceptionFrame = *mut ::core::ffi::c_void;
pub type PerceptionFrameProviderInfo = *mut ::core::ffi::c_void;
pub type PerceptionPropertyChangeRequest = *mut ::core::ffi::c_void;
pub type PerceptionStartFaceAuthenticationHandler = *mut ::core::ffi::c_void;
pub type PerceptionStopFaceAuthenticationHandler = *mut ::core::ffi::c_void;
pub type PerceptionVideoFrameAllocator = *mut ::core::ffi::c_void;
