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
#[repr(C)]
pub struct ISpatialGestureRecognizerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, settings: SpatialGestureSettings, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpatialHoldCanceledEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionSourceKind) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpatialHoldCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionSourceKind) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct ISpatialInteraction {
    pub base__: ::windows_sys::core::IInspectable,
    pub SourceState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct ISpatialInteractionController2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub TryGetRenderableModelAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    TryGetRenderableModelAsync: usize,
}
#[repr(C)]
pub struct ISpatialInteractionController3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Power")]
    pub TryGetBatteryReport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Power"))]
    TryGetBatteryReport: usize,
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
#[repr(C)]
pub struct ISpatialInteractionDetectedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub InteractionSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct ISpatialInteractionManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpatialInteractionManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSourceKindSupported: unsafe extern "system" fn(this: *mut *mut Self, kind: SpatialInteractionSourceKind, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpatialInteractionSource {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionSourceKind) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct ISpatialInteractionSource3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handedness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionSourceHandedness) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct ISpatialInteractionSourceEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpatialInteractionSourceEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PressKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionPressKind) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct ISpatialInteractionSourceLocation2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Orientation: usize,
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
#[repr(C)]
pub struct ISpatialInteractionSourceState2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSelectPressed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsMenuPressed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsGrasped: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SelectPressedValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ControllerProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpatialInteractionSourceState3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Perception_People")]
    pub TryGetHandPose: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Perception_People"))]
    TryGetHandPose: usize,
}
#[repr(C)]
pub struct ISpatialManipulationCanceledEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionSourceKind) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct ISpatialManipulationDelta {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub Translation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Translation: usize,
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
#[repr(C)]
pub struct ISpatialManipulationUpdatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionSourceKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetCumulativeDelta: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetCumulativeDelta: usize,
}
#[repr(C)]
pub struct ISpatialNavigationCanceledEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionSourceKind) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct ISpatialNavigationUpdatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionSourceKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub NormalizedOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    NormalizedOffset: usize,
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
#[repr(C)]
pub struct ISpatialPointerInteractionSourcePose2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Quaternion) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Orientation: usize,
    pub PositionAccuracy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionSourcePositionAccuracy) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct ISpatialPointerPose2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryGetInteractionSourcePose: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct ISpatialPointerPoseStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetAtTimestamp: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, timestamp: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetAtTimestamp: usize,
}
#[repr(C)]
pub struct ISpatialRecognitionEndedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialInteractionSourceKind) -> ::windows_sys::core::HRESULT,
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
