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
impl ::windows_sys::core::Interface for IKnownPerceptionFrameKindStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 988172758, data2: 38505, data3: 16646, data4: [159, 174, 72, 53, 193, 185, 97, 4] };
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
impl ::windows_sys::core::Interface for IPerceptionControlGroup {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 388778114, data2: 12249, data3: 19534, data4: [186, 52, 253, 242, 10, 115, 221, 229] };
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
impl ::windows_sys::core::Interface for IPerceptionControlGroupFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 790295264, data2: 47857, data3: 17723, data4: [190, 212, 205, 157, 70, 25, 21, 76] };
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
impl ::windows_sys::core::Interface for IPerceptionCorrelation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3021150850, data2: 57333, data3: 16455, data4: [138, 25, 59, 77, 128, 95, 113, 118] };
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
impl ::windows_sys::core::Interface for IPerceptionCorrelationFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3567698981, data2: 10372, data3: 19087, data4: [129, 52, 40, 53, 215, 40, 108, 191] };
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
impl ::windows_sys::core::Interface for IPerceptionCorrelationGroup {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1965689094, data2: 13991, data3: 18363, data4: [155, 121, 86, 204, 107, 116, 103, 112] };
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
impl ::windows_sys::core::Interface for IPerceptionCorrelationGroupFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2113806472, data2: 25567, data3: 18669, data4: [131, 177, 74, 184, 41, 19, 41, 149] };
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
impl ::windows_sys::core::Interface for IPerceptionFaceAuthenticationGroup {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3892418580, data2: 19089, data3: 16816, data4: [131, 166, 136, 26, 23, 117, 53, 62] };
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
impl ::windows_sys::core::Interface for IPerceptionFaceAuthenticationGroupFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3867805140, data2: 46604, data3: 16628, data4: [188, 185, 242, 77, 70, 70, 115, 32] };
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
impl ::windows_sys::core::Interface for IPerceptionFrame {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2097051685, data2: 21691, data3: 19869, data4: [190, 197, 142, 246, 97, 81, 210, 172] };
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
impl ::windows_sys::core::Interface for IPerceptionFrameProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2035251897, data2: 45949, data3: 15155, data4: [161, 13, 48, 98, 100, 25, 206, 101] };
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
impl ::windows_sys::core::Interface for IPerceptionFrameProviderInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3433650664, data2: 31102, data3: 20099, data4: [155, 135, 3, 106, 116, 20, 47, 196] };
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
impl ::windows_sys::core::Interface for IPerceptionFrameProviderManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2841234951, data2: 60115, data3: 13279, data4: [142, 193, 185, 36, 171, 224, 25, 196] };
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
impl ::windows_sys::core::Interface for IPerceptionFrameProviderManagerServiceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2927855334, data2: 51929, data3: 17241, data4: [143, 150, 142, 174, 81, 129, 5, 38] };
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
impl ::windows_sys::core::Interface for IPerceptionPropertyChangeRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1012591441, data2: 13579, data3: 19960, data4: [148, 20, 89, 224, 152, 21, 81, 11] };
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
impl ::windows_sys::core::Interface for IPerceptionVideoFrameAllocator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1278781402, data2: 64984, data3: 20180, data4: [160, 57, 42, 111, 155, 35, 80, 56] };
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
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IPerceptionVideoFrameAllocatorFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 442020065, data2: 59674, data3: 18462, data4: [184, 118, 168, 158, 43, 188, 107, 51] };
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
