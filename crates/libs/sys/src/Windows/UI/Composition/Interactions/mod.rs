pub type CompositionConditionalValue = *mut ::core::ffi::c_void;
pub type CompositionInteractionSourceCollection = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct ICompositionConditionalValue {
    pub base__: ::windows_sys::core::IInspectable,
    pub Condition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionConditionalValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositionInteractionSource {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICompositionInteractionSourceCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInteractionSourceConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    pub PositionXSourceMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InteractionSourceRedirectionMode) -> ::windows_sys::core::HRESULT,
    pub SetPositionXSourceMode: unsafe extern "system" fn(this: *mut *mut Self, value: InteractionSourceRedirectionMode) -> ::windows_sys::core::HRESULT,
    pub PositionYSourceMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InteractionSourceRedirectionMode) -> ::windows_sys::core::HRESULT,
    pub SetPositionYSourceMode: unsafe extern "system" fn(this: *mut *mut Self, value: InteractionSourceRedirectionMode) -> ::windows_sys::core::HRESULT,
    pub ScaleSourceMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InteractionSourceRedirectionMode) -> ::windows_sys::core::HRESULT,
    pub SetScaleSourceMode: unsafe extern "system" fn(this: *mut *mut Self, value: InteractionSourceRedirectionMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInteractionTracker {
    pub base__: ::windows_sys::core::IInspectable,
    pub InteractionSources: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsPositionRoundingSuggested: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub MaxPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    MaxPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetMaxPosition: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetMaxPosition: usize,
    pub MaxScale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetMaxScale: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub MinPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    MinPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetMinPosition: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetMinPosition: usize,
    pub MinScale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetMinScale: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub NaturalRestingPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    NaturalRestingPosition: usize,
    pub NaturalRestingScale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub Owner: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub PositionInertiaDecayRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PositionInertiaDecayRate: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetPositionInertiaDecayRate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetPositionInertiaDecayRate: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub PositionVelocityInPixelsPerSecond: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PositionVelocityInPixelsPerSecond: usize,
    pub Scale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ScaleInertiaDecayRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScaleInertiaDecayRate: usize,
    #[cfg(feature = "Foundation")]
    pub SetScaleInertiaDecayRate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetScaleInertiaDecayRate: usize,
    pub ScaleVelocityInPercentPerSecond: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub AdjustPositionXIfGreaterThanThreshold: unsafe extern "system" fn(this: *mut *mut Self, adjustment: f32, positionthreshold: f32) -> ::windows_sys::core::HRESULT,
    pub AdjustPositionYIfGreaterThanThreshold: unsafe extern "system" fn(this: *mut *mut Self, adjustment: f32, positionthreshold: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigurePositionXInertiaModifiers: unsafe extern "system" fn(this: *mut *mut Self, modifiers: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigurePositionXInertiaModifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigurePositionYInertiaModifiers: unsafe extern "system" fn(this: *mut *mut Self, modifiers: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigurePositionYInertiaModifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureScaleInertiaModifiers: unsafe extern "system" fn(this: *mut *mut Self, modifiers: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureScaleInertiaModifiers: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdatePosition: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdatePosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdatePositionBy: unsafe extern "system" fn(this: *mut *mut Self, amount: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdatePositionBy: usize,
    pub TryUpdatePositionWithAnimation: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdatePositionWithAdditionalVelocity: unsafe extern "system" fn(this: *mut *mut Self, velocityinpixelspersecond: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdatePositionWithAdditionalVelocity: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdateScale: unsafe extern "system" fn(this: *mut *mut Self, value: f32, centerpoint: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdateScale: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdateScaleWithAnimation: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void, centerpoint: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdateScaleWithAnimation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdateScaleWithAdditionalVelocity: unsafe extern "system" fn(this: *mut *mut Self, velocityinpercentpersecond: f32, centerpoint: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdateScaleWithAdditionalVelocity: usize,
}
#[repr(C)]
pub struct IInteractionTracker2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureCenterPointXInertiaModifiers: unsafe extern "system" fn(this: *mut *mut Self, conditionalvalues: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureCenterPointXInertiaModifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureCenterPointYInertiaModifiers: unsafe extern "system" fn(this: *mut *mut Self, conditionalvalues: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureCenterPointYInertiaModifiers: usize,
}
#[repr(C)]
pub struct IInteractionTracker3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureVector2PositionInertiaModifiers: unsafe extern "system" fn(this: *mut *mut Self, modifiers: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureVector2PositionInertiaModifiers: usize,
}
#[repr(C)]
pub struct IInteractionTracker4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdatePositionWithOption: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdatePositionWithOption: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdatePositionByWithOption: unsafe extern "system" fn(this: *mut *mut Self, amount: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdatePositionByWithOption: usize,
    pub IsInertiaFromImpulse: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInteractionTracker5 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdatePositionWithOption: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, posupdateoption: InteractionTrackerPositionUpdateOption, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdatePositionWithOption: usize,
}
#[repr(C)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequestId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsFromBinding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInteractionTrackerIdleStateEnteredArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequestId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInteractionTrackerIdleStateEnteredArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsFromBinding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInteractionTrackerInertiaModifier {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IInteractionTrackerInertiaModifierFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IInteractionTrackerInertiaMotion {
    pub base__: ::windows_sys::core::IInspectable,
    pub Condition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Motion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetMotion: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInteractionTrackerInertiaMotionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInteractionTrackerInertiaNaturalMotion {
    pub base__: ::windows_sys::core::IInspectable,
    pub Condition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NaturalMotion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetNaturalMotion: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInteractionTrackerInertiaNaturalMotionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInteractionTrackerInertiaRestingValue {
    pub base__: ::windows_sys::core::IInspectable,
    pub Condition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RestingValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRestingValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInteractionTrackerInertiaRestingValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInteractionTrackerInertiaStateEnteredArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub ModifiedRestingPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ModifiedRestingPosition: usize,
    #[cfg(feature = "Foundation")]
    pub ModifiedRestingScale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ModifiedRestingScale: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub NaturalRestingPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    NaturalRestingPosition: usize,
    pub NaturalRestingScale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub PositionVelocityInPixelsPerSecond: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PositionVelocityInPixelsPerSecond: usize,
    pub RequestId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ScaleVelocityInPercentPerSecond: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInteractionTrackerInertiaStateEnteredArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsInertiaFromImpulse: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInteractionTrackerInertiaStateEnteredArgs3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsFromBinding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInteractionTrackerInteractingStateEnteredArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequestId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInteractionTrackerInteractingStateEnteredArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsFromBinding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInteractionTrackerOwner {
    pub base__: ::windows_sys::core::IInspectable,
    pub CustomAnimationStateEntered: unsafe extern "system" fn(this: *mut *mut Self, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IdleStateEntered: unsafe extern "system" fn(this: *mut *mut Self, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InertiaStateEntered: unsafe extern "system" fn(this: *mut *mut Self, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InteractingStateEntered: unsafe extern "system" fn(this: *mut *mut Self, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RequestIgnored: unsafe extern "system" fn(this: *mut *mut Self, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ValuesChanged: unsafe extern "system" fn(this: *mut *mut Self, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInteractionTrackerRequestIgnoredArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequestId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInteractionTrackerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithOwner: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, owner: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInteractionTrackerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetBindingMode: unsafe extern "system" fn(this: *mut *mut Self, boundtracker1: *mut ::core::ffi::c_void, boundtracker2: *mut ::core::ffi::c_void, axismode: InteractionBindingAxisModes) -> ::windows_sys::core::HRESULT,
    pub GetBindingMode: unsafe extern "system" fn(this: *mut *mut Self, boundtracker1: *mut ::core::ffi::c_void, boundtracker2: *mut ::core::ffi::c_void, result__: *mut InteractionBindingAxisModes) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInteractionTrackerValuesChangedArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    pub RequestId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Scale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInteractionTrackerVector2InertiaModifier {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IInteractionTrackerVector2InertiaModifierFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IInteractionTrackerVector2InertiaNaturalMotion {
    pub base__: ::windows_sys::core::IInspectable,
    pub Condition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NaturalMotion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetNaturalMotion: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInteractionTrackerVector2InertiaNaturalMotionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVisualInteractionSource {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsPositionXRailsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsPositionXRailsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsPositionYRailsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsPositionYRailsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ManipulationRedirectionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VisualInteractionSourceRedirectionMode) -> ::windows_sys::core::HRESULT,
    pub SetManipulationRedirectionMode: unsafe extern "system" fn(this: *mut *mut Self, value: VisualInteractionSourceRedirectionMode) -> ::windows_sys::core::HRESULT,
    pub PositionXChainingMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InteractionChainingMode) -> ::windows_sys::core::HRESULT,
    pub SetPositionXChainingMode: unsafe extern "system" fn(this: *mut *mut Self, value: InteractionChainingMode) -> ::windows_sys::core::HRESULT,
    pub PositionXSourceMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InteractionSourceMode) -> ::windows_sys::core::HRESULT,
    pub SetPositionXSourceMode: unsafe extern "system" fn(this: *mut *mut Self, value: InteractionSourceMode) -> ::windows_sys::core::HRESULT,
    pub PositionYChainingMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InteractionChainingMode) -> ::windows_sys::core::HRESULT,
    pub SetPositionYChainingMode: unsafe extern "system" fn(this: *mut *mut Self, value: InteractionChainingMode) -> ::windows_sys::core::HRESULT,
    pub PositionYSourceMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InteractionSourceMode) -> ::windows_sys::core::HRESULT,
    pub SetPositionYSourceMode: unsafe extern "system" fn(this: *mut *mut Self, value: InteractionSourceMode) -> ::windows_sys::core::HRESULT,
    pub ScaleChainingMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InteractionChainingMode) -> ::windows_sys::core::HRESULT,
    pub SetScaleChainingMode: unsafe extern "system" fn(this: *mut *mut Self, value: InteractionChainingMode) -> ::windows_sys::core::HRESULT,
    pub ScaleSourceMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InteractionSourceMode) -> ::windows_sys::core::HRESULT,
    pub SetScaleSourceMode: unsafe extern "system" fn(this: *mut *mut Self, value: InteractionSourceMode) -> ::windows_sys::core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Input")]
    pub TryRedirectForManipulation: unsafe extern "system" fn(this: *mut *mut Self, pointerpoint: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    TryRedirectForManipulation: usize,
}
#[repr(C)]
pub struct IVisualInteractionSource2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub DeltaPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DeltaPosition: usize,
    pub DeltaScale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub PositionVelocity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PositionVelocity: usize,
    pub Scale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub ScaleVelocity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureCenterPointXModifiers: unsafe extern "system" fn(this: *mut *mut Self, conditionalvalues: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureCenterPointXModifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureCenterPointYModifiers: unsafe extern "system" fn(this: *mut *mut Self, conditionalvalues: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureCenterPointYModifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureDeltaPositionXModifiers: unsafe extern "system" fn(this: *mut *mut Self, conditionalvalues: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureDeltaPositionXModifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureDeltaPositionYModifiers: unsafe extern "system" fn(this: *mut *mut Self, conditionalvalues: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureDeltaPositionYModifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureDeltaScaleModifiers: unsafe extern "system" fn(this: *mut *mut Self, conditionalvalues: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureDeltaScaleModifiers: usize,
}
#[repr(C)]
pub struct IVisualInteractionSource3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PointerWheelConfig: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVisualInteractionSourceObjectFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IVisualInteractionSourceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVisualInteractionSourceStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromIVisualElement: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionBindingAxisModes(pub u32);
impl InteractionBindingAxisModes {
    pub const None: Self = Self(0u32);
    pub const PositionX: Self = Self(1u32);
    pub const PositionY: Self = Self(2u32);
    pub const Scale: Self = Self(4u32);
}
impl ::core::marker::Copy for InteractionBindingAxisModes {}
impl ::core::clone::Clone for InteractionBindingAxisModes {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionChainingMode(pub i32);
impl InteractionChainingMode {
    pub const Auto: Self = Self(0i32);
    pub const Always: Self = Self(1i32);
    pub const Never: Self = Self(2i32);
}
impl ::core::marker::Copy for InteractionChainingMode {}
impl ::core::clone::Clone for InteractionChainingMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type InteractionSourceConfiguration = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionSourceMode(pub i32);
impl InteractionSourceMode {
    pub const Disabled: Self = Self(0i32);
    pub const EnabledWithInertia: Self = Self(1i32);
    pub const EnabledWithoutInertia: Self = Self(2i32);
}
impl ::core::marker::Copy for InteractionSourceMode {}
impl ::core::clone::Clone for InteractionSourceMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionSourceRedirectionMode(pub i32);
impl InteractionSourceRedirectionMode {
    pub const Disabled: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
}
impl ::core::marker::Copy for InteractionSourceRedirectionMode {}
impl ::core::clone::Clone for InteractionSourceRedirectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type InteractionTracker = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerClampingOption(pub i32);
impl InteractionTrackerClampingOption {
    pub const Auto: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
}
impl ::core::marker::Copy for InteractionTrackerClampingOption {}
impl ::core::clone::Clone for InteractionTrackerClampingOption {
    fn clone(&self) -> Self {
        *self
    }
}
pub type InteractionTrackerCustomAnimationStateEnteredArgs = *mut ::core::ffi::c_void;
pub type InteractionTrackerIdleStateEnteredArgs = *mut ::core::ffi::c_void;
pub type InteractionTrackerInertiaModifier = *mut ::core::ffi::c_void;
pub type InteractionTrackerInertiaMotion = *mut ::core::ffi::c_void;
pub type InteractionTrackerInertiaNaturalMotion = *mut ::core::ffi::c_void;
pub type InteractionTrackerInertiaRestingValue = *mut ::core::ffi::c_void;
pub type InteractionTrackerInertiaStateEnteredArgs = *mut ::core::ffi::c_void;
pub type InteractionTrackerInteractingStateEnteredArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerPositionUpdateOption(pub i32);
impl InteractionTrackerPositionUpdateOption {
    pub const Default: Self = Self(0i32);
    pub const AllowActiveCustomScaleAnimation: Self = Self(1i32);
}
impl ::core::marker::Copy for InteractionTrackerPositionUpdateOption {}
impl ::core::clone::Clone for InteractionTrackerPositionUpdateOption {
    fn clone(&self) -> Self {
        *self
    }
}
pub type InteractionTrackerRequestIgnoredArgs = *mut ::core::ffi::c_void;
pub type InteractionTrackerValuesChangedArgs = *mut ::core::ffi::c_void;
pub type InteractionTrackerVector2InertiaModifier = *mut ::core::ffi::c_void;
pub type InteractionTrackerVector2InertiaNaturalMotion = *mut ::core::ffi::c_void;
pub type VisualInteractionSource = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct VisualInteractionSourceRedirectionMode(pub i32);
impl VisualInteractionSourceRedirectionMode {
    pub const Off: Self = Self(0i32);
    pub const CapableTouchpadOnly: Self = Self(1i32);
    pub const PointerWheelOnly: Self = Self(2i32);
    pub const CapableTouchpadAndPointerWheel: Self = Self(3i32);
}
impl ::core::marker::Copy for VisualInteractionSourceRedirectionMode {}
impl ::core::clone::Clone for VisualInteractionSourceRedirectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
