#[repr(C)]
pub struct ISpatialGestureRecognizer {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RecognitionStarted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecognitionStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRecognitionStarted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRecognitionStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RecognitionEnded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecognitionEnded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRecognitionEnded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRecognitionEnded: usize,
    #[cfg(feature = "Foundation")]
    pub Tapped: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Tapped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTapped: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTapped: usize,
    #[cfg(feature = "Foundation")]
    pub HoldStarted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HoldStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHoldStarted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHoldStarted: usize,
    #[cfg(feature = "Foundation")]
    pub HoldCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HoldCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHoldCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHoldCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub HoldCanceled: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HoldCanceled: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHoldCanceled: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHoldCanceled: usize,
    #[cfg(feature = "Foundation")]
    pub ManipulationStarted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ManipulationStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveManipulationStarted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveManipulationStarted: usize,
    #[cfg(feature = "Foundation")]
    pub ManipulationUpdated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ManipulationUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveManipulationUpdated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveManipulationUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub ManipulationCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ManipulationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveManipulationCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveManipulationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub ManipulationCanceled: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ManipulationCanceled: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveManipulationCanceled: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveManipulationCanceled: usize,
    #[cfg(feature = "Foundation")]
    pub NavigationStarted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NavigationStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNavigationStarted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNavigationStarted: usize,
    #[cfg(feature = "Foundation")]
    pub NavigationUpdated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NavigationUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNavigationUpdated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNavigationUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub NavigationCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NavigationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNavigationCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNavigationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub NavigationCanceled: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NavigationCanceled: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNavigationCanceled: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNavigationCanceled: usize,
    pub CaptureInteraction: unsafe extern "system" fn(this: *mut *mut Self, interaction: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CancelPendingGestures: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub TrySetGestureSettings: unsafe extern "system" fn(this: *mut *mut Self, settings: SpatialGestureSettings, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub GestureSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialGestureSettings) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialGestureRecognizer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1902140364, data2: 3125, data3: 18035, data4: [173, 189, 204, 4, 202, 166, 239, 69] };
}
#[repr(C)]
pub struct ISpatialGestureRecognizerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, settings: SpatialGestureSettings, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialGestureRecognizerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1998668166, data2: 22457, data3: 12624, data4: [131, 130, 105, 139, 36, 226, 100, 208] };
}
#[repr(C)]
pub struct ISpatialHoldCanceledEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionSourceKind) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialHoldCanceledEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1576842855, data2: 19626, data3: 16531, data4: [140, 53, 182, 1, 168, 57, 243, 27] };
}
#[repr(C)]
pub struct ISpatialHoldCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionSourceKind) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialHoldCompletedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1063536395, data2: 19709, data3: 17370, data4: [141, 196, 230, 69, 82, 23, 57, 113] };
}
#[repr(C)]
pub struct ISpatialHoldStartedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionSourceKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetPointerPose: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetPointerPose: usize,
}
impl ::windows_sys::core::Interface for ISpatialHoldStartedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2385788281, data2: 44214, data3: 16708, data4: [134, 21, 44, 251, 168, 163, 203, 63] };
}
#[repr(C)]
pub struct ISpatialInteraction {
    pub base__: ::windows_sys::core::IInspectable,
    pub SourceState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialInteraction {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4237719097, data2: 35046, data3: 17990, data4: [145, 18, 67, 68, 170, 236, 157, 250] };
}
#[repr(C)]
pub struct ISpatialInteractionController {
    pub base__: ::windows_sys::core::IInspectable,
    pub HasTouchpad: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub HasThumbstick: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
    pub VendorId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub ProductId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialInteractionController {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1594776483, data2: 2388, data3: 20119, data4: [134, 197, 231, 243, 11, 17, 77, 253] };
}
#[repr(C)]
pub struct ISpatialInteractionController2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub TryGetRenderableModelAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    TryGetRenderableModelAsync: usize,
}
impl ::windows_sys::core::Interface for ISpatialInteractionController2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 901175588, data2: 51106, data3: 18871, data4: [183, 46, 84, 54, 178, 251, 143, 156] };
}
#[repr(C)]
pub struct ISpatialInteractionController3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Power")]
    pub TryGetBatteryReport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Power"))]
    TryGetBatteryReport: usize,
}
impl ::windows_sys::core::Interface for ISpatialInteractionController3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1652844192, data2: 40337, data3: 18955, data4: [136, 141, 22, 94, 103, 10, 140, 213] };
}
#[repr(C)]
pub struct ISpatialInteractionControllerProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTouchpadTouched: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsTouchpadPressed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsThumbstickPressed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ThumbstickX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ThumbstickY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub TouchpadX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub TouchpadY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialInteractionControllerProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1627746225, data2: 31657, data3: 20021, data4: [185, 63, 146, 114, 203, 169, 178, 139] };
}
#[repr(C)]
pub struct ISpatialInteractionDetectedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionSourceKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetPointerPose: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetPointerPose: usize,
    pub Interaction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialInteractionDetectedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 123238628, data2: 22881, data3: 15169, data4: [157, 251, 206, 165, 216, 156, 195, 138] };
}
#[repr(C)]
pub struct ISpatialInteractionDetectedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub InteractionSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialInteractionDetectedEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2066103955, data2: 24339, data3: 16796, data4: [151, 213, 131, 70, 120, 38, 106, 166] };
}
#[repr(C)]
pub struct ISpatialInteractionManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SourceDetected: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceDetected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSourceDetected: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSourceDetected: usize,
    #[cfg(feature = "Foundation")]
    pub SourceLost: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceLost: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSourceLost: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSourceLost: usize,
    #[cfg(feature = "Foundation")]
    pub SourceUpdated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSourceUpdated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSourceUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub SourcePressed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourcePressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSourcePressed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSourcePressed: usize,
    #[cfg(feature = "Foundation")]
    pub SourceReleased: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceReleased: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSourceReleased: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSourceReleased: usize,
    #[cfg(feature = "Foundation")]
    pub InteractionDetected: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InteractionDetected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInteractionDetected: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInteractionDetected: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Perception"))]
    pub GetDetectedSourcesAtTimestamp: unsafe extern "system" fn(this: *mut *mut Self, timestamp: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Perception")))]
    GetDetectedSourcesAtTimestamp: usize,
}
impl ::windows_sys::core::Interface for ISpatialInteractionManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 849759912, data2: 41306, data3: 14741, data4: [184, 189, 128, 81, 60, 181, 173, 239] };
}
#[repr(C)]
pub struct ISpatialInteractionManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialInteractionManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 14884774, data2: 36002, data3: 12479, data4: [145, 254, 217, 203, 74, 0, 137, 144] };
}
#[repr(C)]
pub struct ISpatialInteractionManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSourceKindSupported: unsafe extern "system" fn(this: *mut *mut Self, kind: SpatialInteractionSourceKind, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialInteractionManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2482072658, data2: 47242, data3: 22825, data4: [141, 124, 72, 203, 148, 139, 8, 28] };
}
#[repr(C)]
pub struct ISpatialInteractionSource {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionSourceKind) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialInteractionSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4216599482, data2: 45235, data3: 12616, data4: [159, 59, 233, 245, 222, 86, 143, 93] };
}
#[repr(C)]
pub struct ISpatialInteractionSource2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsPointingSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsMenuSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsGraspSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Controller: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Perception")]
    pub TryGetStateAtTimestamp: unsafe extern "system" fn(this: *mut *mut Self, timestamp: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Perception"))]
    TryGetStateAtTimestamp: usize,
}
impl ::windows_sys::core::Interface for ISpatialInteractionSource2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3838162700, data2: 1136, data3: 16424, data4: [136, 192, 160, 235, 68, 211, 78, 254] };
}
#[repr(C)]
pub struct ISpatialInteractionSource3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handedness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionSourceHandedness) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialInteractionSource3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 67557881, data2: 39677, data3: 17657, data4: [133, 220, 112, 0, 35, 169, 98, 227] };
}
#[repr(C)]
pub struct ISpatialInteractionSource4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Perception_People")]
    pub TryCreateHandMeshObserver: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Perception_People"))]
    TryCreateHandMeshObserver: usize,
    #[cfg(all(feature = "Foundation", feature = "Perception_People"))]
    pub TryCreateHandMeshObserverAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Perception_People")))]
    TryCreateHandMeshObserverAsync: usize,
}
impl ::windows_sys::core::Interface for ISpatialInteractionSource4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 7584845, data2: 57190, data3: 23185, data4: [162, 186, 206, 163, 229, 197, 138, 25] };
}
#[repr(C)]
pub struct ISpatialInteractionSourceEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialInteractionSourceEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 599230159, data2: 60451, data3: 14713, data4: [178, 124, 235, 14, 18, 254, 183, 199] };
}
#[repr(C)]
pub struct ISpatialInteractionSourceEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PressKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionPressKind) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialInteractionSourceEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3635721319, data2: 58952, data3: 19794, data4: [171, 73, 224, 210, 39, 25, 159, 99] };
}
#[repr(C)]
pub struct ISpatialInteractionSourceLocation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Velocity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Velocity: usize,
}
impl ::windows_sys::core::Interface for ISpatialInteractionSourceLocation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3930494660, data2: 32395, data3: 12490, data4: [188, 197, 199, 113, 137, 206, 163, 10] };
}
#[repr(C)]
pub struct ISpatialInteractionSourceLocation2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Orientation: usize,
}
impl ::windows_sys::core::Interface for ISpatialInteractionSourceLocation2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1281822789, data2: 14615, data3: 16636, data4: [169, 172, 49, 201, 207, 95, 249, 27] };
}
#[repr(C)]
pub struct ISpatialInteractionSourceLocation3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PositionAccuracy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionSourcePositionAccuracy) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub AngularVelocity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AngularVelocity: usize,
    pub SourcePointerPose: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialInteractionSourceLocation3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1728243294, data2: 59669, data3: 19707, data4: [156, 27, 5, 56, 239, 200, 102, 135] };
}
#[repr(C)]
pub struct ISpatialInteractionSourceProperties {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub TryGetSourceLossMitigationDirection: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    TryGetSourceLossMitigationDirection: usize,
    pub SourceLossRisk: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetLocation: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetLocation: usize,
}
impl ::windows_sys::core::Interface for ISpatialInteractionSourceProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 90195266, data2: 16119, data3: 12834, data4: [159, 83, 99, 201, 203, 126, 59, 199] };
}
#[repr(C)]
pub struct ISpatialInteractionSourceState {
    pub base__: ::windows_sys::core::IInspectable,
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsPressed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Perception")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Perception"))]
    Timestamp: usize,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetPointerPose: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetPointerPose: usize,
}
impl ::windows_sys::core::Interface for ISpatialInteractionSourceState {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3586422255, data2: 19299, data3: 14316, data4: [152, 185, 159, 198, 82, 185, 210, 242] };
}
#[repr(C)]
pub struct ISpatialInteractionSourceState2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSelectPressed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsMenuPressed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsGrasped: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SelectPressedValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ControllerProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialInteractionSourceState2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1173803197, data2: 6003, data3: 18734, data4: [155, 163, 138, 193, 203, 231, 124, 8] };
}
#[repr(C)]
pub struct ISpatialInteractionSourceState3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Perception_People")]
    pub TryGetHandPose: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Perception_People"))]
    TryGetHandPose: usize,
}
impl ::windows_sys::core::Interface for ISpatialInteractionSourceState3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075817922, data2: 48427, data3: 18945, data4: [168, 251, 50, 62, 1, 88, 82, 124] };
}
#[repr(C)]
pub struct ISpatialManipulationCanceledEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionSourceKind) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialManipulationCanceledEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 759222731, data2: 59354, data3: 16928, data4: [176, 191, 129, 147, 1, 103, 71, 128] };
}
#[repr(C)]
pub struct ISpatialManipulationCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionSourceKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetCumulativeDelta: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetCumulativeDelta: usize,
}
impl ::windows_sys::core::Interface for ISpatialManipulationCompletedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 84436994, data2: 62209, data3: 17219, data4: [146, 80, 47, 186, 165, 248, 122, 55] };
}
#[repr(C)]
pub struct ISpatialManipulationDelta {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub Translation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Translation: usize,
}
impl ::windows_sys::core::Interface for ISpatialManipulationDelta {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2817300090, data2: 53539, data3: 14977, data4: [161, 91, 153, 41, 35, 220, 190, 145] };
}
#[repr(C)]
pub struct ISpatialManipulationStartedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionSourceKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetPointerPose: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetPointerPose: usize,
}
impl ::windows_sys::core::Interface for ISpatialManipulationStartedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2715204558, data2: 17061, data3: 14203, data4: [173, 166, 210, 142, 61, 56, 71, 55] };
}
#[repr(C)]
pub struct ISpatialManipulationUpdatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionSourceKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetCumulativeDelta: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetCumulativeDelta: usize,
}
impl ::windows_sys::core::Interface for ISpatialManipulationUpdatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1596132251, data2: 24774, data3: 19910, data4: [189, 201, 159, 74, 111, 21, 254, 73] };
}
#[repr(C)]
pub struct ISpatialNavigationCanceledEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionSourceKind) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialNavigationCanceledEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3461365468, data2: 59557, data3: 18160, data4: [146, 212, 60, 18, 43, 53, 17, 42] };
}
#[repr(C)]
pub struct ISpatialNavigationCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionSourceKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub NormalizedOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    NormalizedOffset: usize,
}
impl ::windows_sys::core::Interface for ISpatialNavigationCompletedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 19824823, data2: 44859, data3: 17090, data4: [158, 65, 186, 170, 14, 114, 31, 58] };
}
#[repr(C)]
pub struct ISpatialNavigationStartedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionSourceKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetPointerPose: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetPointerPose: usize,
    pub IsNavigatingX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsNavigatingY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsNavigatingZ: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialNavigationStartedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1967797386, data2: 64356, data3: 18006, data4: [142, 189, 157, 238, 202, 175, 228, 117] };
}
#[repr(C)]
pub struct ISpatialNavigationUpdatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionSourceKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub NormalizedOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    NormalizedOffset: usize,
}
impl ::windows_sys::core::Interface for ISpatialNavigationUpdatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2607890391, data2: 33693, data3: 19060, data4: [135, 50, 69, 70, 111, 192, 68, 181] };
}
#[repr(C)]
pub struct ISpatialPointerInteractionSourcePose {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub ForwardDirection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ForwardDirection: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub UpDirection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    UpDirection: usize,
}
impl ::windows_sys::core::Interface for ISpatialPointerInteractionSourcePose {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2802860807, data2: 11307, data3: 19770, data4: [146, 167, 128, 206, 215, 196, 160, 208] };
}
#[repr(C)]
pub struct ISpatialPointerInteractionSourcePose2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Quaternion) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Orientation: usize,
    pub PositionAccuracy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionSourcePositionAccuracy) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialPointerInteractionSourcePose2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3972892344, data2: 21211, data3: 18079, data4: [158, 63, 128, 196, 127, 116, 188, 233] };
}
#[repr(C)]
pub struct ISpatialPointerPose {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Perception")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Perception"))]
    Timestamp: usize,
    #[cfg(feature = "Perception_People")]
    pub Head: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Perception_People"))]
    Head: usize,
}
impl ::windows_sys::core::Interface for ISpatialPointerPose {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1767089198, data2: 49534, data3: 13693, data4: [151, 161, 114, 105, 208, 237, 45, 16] };
}
#[repr(C)]
pub struct ISpatialPointerPose2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryGetInteractionSourcePose: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialPointerPose2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2636131095, data2: 38222, data3: 19980, data4: [150, 209, 182, 121, 11, 111, 194, 253] };
}
#[repr(C)]
pub struct ISpatialPointerPose3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Perception_People")]
    pub Eyes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Perception_People"))]
    Eyes: usize,
    pub IsHeadCapturedBySystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialPointerPose3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1665332208, data2: 60489, data3: 23371, data4: [184, 209, 209, 108, 187, 22, 190, 132] };
}
#[repr(C)]
pub struct ISpatialPointerPoseStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetAtTimestamp: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, timestamp: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetAtTimestamp: usize,
}
impl ::windows_sys::core::Interface for ISpatialPointerPoseStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2723516841, data2: 44193, data3: 16096, data4: [152, 22, 120, 92, 251, 46, 63, 184] };
}
#[repr(C)]
pub struct ISpatialRecognitionEndedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionSourceKind) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialRecognitionEndedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 238417355, data2: 16245, data3: 17395, data4: [172, 129, 209, 220, 45, 249, 177, 251] };
}
#[repr(C)]
pub struct ISpatialRecognitionStartedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionSourceKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetPointerPose: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetPointerPose: usize,
    pub IsGesturePossible: unsafe extern "system" fn(this: *mut *mut Self, gesture: SpatialGestureSettings, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialRecognitionStartedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 618271375, data2: 8, data3: 19053, data4: [170, 80, 42, 118, 249, 207, 178, 100] };
}
#[repr(C)]
pub struct ISpatialTappedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionSourceKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetPointerPose: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetPointerPose: usize,
    pub TapCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialTappedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 695043038, data2: 62532, data3: 19105, data4: [178, 191, 157, 200, 141, 86, 125, 166] };
}
pub type SpatialGestureRecognizer = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Input_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialGestureSettings(pub u32);
impl SpatialGestureSettings {
    pub const None: Self = Self(0u32);
    pub const Tap: Self = Self(1u32);
    pub const DoubleTap: Self = Self(2u32);
    pub const Hold: Self = Self(4u32);
    pub const ManipulationTranslate: Self = Self(8u32);
    pub const NavigationX: Self = Self(16u32);
    pub const NavigationY: Self = Self(32u32);
    pub const NavigationZ: Self = Self(64u32);
    pub const NavigationRailsX: Self = Self(128u32);
    pub const NavigationRailsY: Self = Self(256u32);
    pub const NavigationRailsZ: Self = Self(512u32);
}
impl ::core::marker::Copy for SpatialGestureSettings {}
impl ::core::clone::Clone for SpatialGestureSettings {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SpatialHoldCanceledEventArgs = *mut ::core::ffi::c_void;
pub type SpatialHoldCompletedEventArgs = *mut ::core::ffi::c_void;
pub type SpatialHoldStartedEventArgs = *mut ::core::ffi::c_void;
pub type SpatialInteraction = *mut ::core::ffi::c_void;
pub type SpatialInteractionController = *mut ::core::ffi::c_void;
pub type SpatialInteractionControllerProperties = *mut ::core::ffi::c_void;
pub type SpatialInteractionDetectedEventArgs = *mut ::core::ffi::c_void;
pub type SpatialInteractionManager = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Input_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialInteractionPressKind(pub i32);
impl SpatialInteractionPressKind {
    pub const None: Self = Self(0i32);
    pub const Select: Self = Self(1i32);
    pub const Menu: Self = Self(2i32);
    pub const Grasp: Self = Self(3i32);
    pub const Touchpad: Self = Self(4i32);
    pub const Thumbstick: Self = Self(5i32);
}
impl ::core::marker::Copy for SpatialInteractionPressKind {}
impl ::core::clone::Clone for SpatialInteractionPressKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SpatialInteractionSource = *mut ::core::ffi::c_void;
pub type SpatialInteractionSourceEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Input_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialInteractionSourceHandedness(pub i32);
impl SpatialInteractionSourceHandedness {
    pub const Unspecified: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
}
impl ::core::marker::Copy for SpatialInteractionSourceHandedness {}
impl ::core::clone::Clone for SpatialInteractionSourceHandedness {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Input_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialInteractionSourceKind(pub i32);
impl SpatialInteractionSourceKind {
    pub const Other: Self = Self(0i32);
    pub const Hand: Self = Self(1i32);
    pub const Voice: Self = Self(2i32);
    pub const Controller: Self = Self(3i32);
}
impl ::core::marker::Copy for SpatialInteractionSourceKind {}
impl ::core::clone::Clone for SpatialInteractionSourceKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SpatialInteractionSourceLocation = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Input_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialInteractionSourcePositionAccuracy(pub i32);
impl SpatialInteractionSourcePositionAccuracy {
    pub const High: Self = Self(0i32);
    pub const Approximate: Self = Self(1i32);
}
impl ::core::marker::Copy for SpatialInteractionSourcePositionAccuracy {}
impl ::core::clone::Clone for SpatialInteractionSourcePositionAccuracy {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SpatialInteractionSourceProperties = *mut ::core::ffi::c_void;
pub type SpatialInteractionSourceState = *mut ::core::ffi::c_void;
pub type SpatialManipulationCanceledEventArgs = *mut ::core::ffi::c_void;
pub type SpatialManipulationCompletedEventArgs = *mut ::core::ffi::c_void;
pub type SpatialManipulationDelta = *mut ::core::ffi::c_void;
pub type SpatialManipulationStartedEventArgs = *mut ::core::ffi::c_void;
pub type SpatialManipulationUpdatedEventArgs = *mut ::core::ffi::c_void;
pub type SpatialNavigationCanceledEventArgs = *mut ::core::ffi::c_void;
pub type SpatialNavigationCompletedEventArgs = *mut ::core::ffi::c_void;
pub type SpatialNavigationStartedEventArgs = *mut ::core::ffi::c_void;
pub type SpatialNavigationUpdatedEventArgs = *mut ::core::ffi::c_void;
pub type SpatialPointerInteractionSourcePose = *mut ::core::ffi::c_void;
pub type SpatialPointerPose = *mut ::core::ffi::c_void;
pub type SpatialRecognitionEndedEventArgs = *mut ::core::ffi::c_void;
pub type SpatialRecognitionStartedEventArgs = *mut ::core::ffi::c_void;
pub type SpatialTappedEventArgs = *mut ::core::ffi::c_void;
