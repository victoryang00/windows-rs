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
impl ::windows_sys::core::Interface for ICompositionConditionalValue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1126499640, data2: 60275, data3: 17761, data4: [167, 29, 26, 67, 234, 235, 122, 155] };
}
#[repr(C)]
pub struct ICompositionConditionalValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionConditionalValueStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 151800690, data2: 33895, data3: 19722, data4: [144, 101, 172, 70, 184, 10, 85, 34] };
}
#[repr(C)]
pub struct ICompositionInteractionSource {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICompositionInteractionSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 70984753, data2: 1763, data3: 18778, data4: [186, 84, 64, 159, 0, 23, 250, 192] };
}
#[repr(C)]
pub struct ICompositionInteractionSourceCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionInteractionSourceCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 457608779, data2: 42431, data3: 18392, data4: [165, 71, 56, 148, 21, 90, 21, 140] };
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
impl ::windows_sys::core::Interface for IInteractionSourceConfiguration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2810398693, data2: 43473, data3: 19714, data4: [152, 94, 185, 48, 205, 11, 157, 164] };
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
impl ::windows_sys::core::Interface for IInteractionTracker {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 713985201, data2: 4096, data3: 17430, data4: [131, 99, 204, 39, 251, 135, 115, 8] };
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
impl ::windows_sys::core::Interface for IInteractionTracker2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 628529726, data2: 52845, data3: 17548, data4: [131, 134, 146, 98, 13, 36, 7, 86] };
}
#[repr(C)]
pub struct IInteractionTracker3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureVector2PositionInertiaModifiers: unsafe extern "system" fn(this: *mut *mut Self, modifiers: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureVector2PositionInertiaModifiers: usize,
}
impl ::windows_sys::core::Interface for IInteractionTracker3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3871725474, data2: 23627, data3: 17094, data4: [132, 183, 246, 148, 65, 177, 128, 145] };
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
impl ::windows_sys::core::Interface for IInteractionTracker4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3956417212, data2: 1199, data3: 19143, data4: [132, 125, 6, 234, 54, 232, 10, 22] };
}
#[repr(C)]
pub struct IInteractionTracker5 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdatePositionWithOption: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, posupdateoption: InteractionTrackerPositionUpdateOption, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdatePositionWithOption: usize,
}
impl ::windows_sys::core::Interface for IInteractionTracker5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3555679650, data2: 41556, data3: 16612, data4: [136, 213, 68, 228, 225, 107, 88, 9] };
}
#[repr(C)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequestId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInteractionTrackerCustomAnimationStateEnteredArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2367458545, data2: 55216, data3: 17228, data4: [165, 210, 45, 118, 17, 134, 72, 52] };
}
#[repr(C)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsFromBinding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInteractionTrackerCustomAnimationStateEnteredArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1205172663, data2: 2437, data3: 24217, data4: [176, 36, 47, 50, 195, 128, 193, 164] };
}
#[repr(C)]
pub struct IInteractionTrackerIdleStateEnteredArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequestId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInteractionTrackerIdleStateEnteredArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1342255018, data2: 5392, data3: 16706, data4: [161, 165, 1, 155, 9, 248, 133, 123] };
}
#[repr(C)]
pub struct IInteractionTrackerIdleStateEnteredArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsFromBinding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInteractionTrackerIdleStateEnteredArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075254253, data2: 47107, data3: 20791, data4: [148, 53, 28, 150, 228, 135, 33, 233] };
}
#[repr(C)]
pub struct IInteractionTrackerInertiaModifier {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IInteractionTrackerInertiaModifier {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2699217184, data2: 9908, data3: 19874, data4: [139, 97, 94, 104, 57, 121, 187, 226] };
}
#[repr(C)]
pub struct IInteractionTrackerInertiaModifierFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IInteractionTrackerInertiaModifierFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2570590462, data2: 51534, data3: 19334, data4: [135, 243, 146, 38, 101, 186, 70, 185] };
}
#[repr(C)]
pub struct IInteractionTrackerInertiaMotion {
    pub base__: ::windows_sys::core::IInspectable,
    pub Condition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Motion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetMotion: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInteractionTrackerInertiaMotion {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 76689372, data2: 61780, data3: 19640, data4: [191, 51, 204, 27, 166, 17, 230, 219] };
}
#[repr(C)]
pub struct IInteractionTrackerInertiaMotionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInteractionTrackerInertiaMotionStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2361933270, data2: 47739, data3: 17178, data4: [132, 75, 110, 172, 145, 48, 249, 154] };
}
#[repr(C)]
pub struct IInteractionTrackerInertiaNaturalMotion {
    pub base__: ::windows_sys::core::IInspectable,
    pub Condition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NaturalMotion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetNaturalMotion: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInteractionTrackerInertiaNaturalMotion {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1890376366, data2: 10204, data3: 18669, data4: [163, 195, 109, 97, 201, 160, 41, 210] };
}
#[repr(C)]
pub struct IInteractionTrackerInertiaNaturalMotionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInteractionTrackerInertiaNaturalMotionStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3487192496, data2: 24126, data3: 17033, data4: [147, 45, 238, 95, 80, 231, 66, 131] };
}
#[repr(C)]
pub struct IInteractionTrackerInertiaRestingValue {
    pub base__: ::windows_sys::core::IInspectable,
    pub Condition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RestingValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRestingValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInteractionTrackerInertiaRestingValue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2264394761, data2: 20630, data3: 16752, data4: [156, 200, 223, 47, 225, 1, 187, 147] };
}
#[repr(C)]
pub struct IInteractionTrackerInertiaRestingValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInteractionTrackerInertiaRestingValueStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 418203289, data2: 1861, data3: 16534, data4: [188, 171, 58, 78, 153, 86, 155, 207] };
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
impl ::windows_sys::core::Interface for IInteractionTrackerInertiaStateEnteredArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2266008818, data2: 59391, data3: 20349, data4: [159, 253, 215, 47, 30, 64, 155, 99] };
}
#[repr(C)]
pub struct IInteractionTrackerInertiaStateEnteredArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsInertiaFromImpulse: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInteractionTrackerInertiaStateEnteredArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2984981238, data2: 49772, data3: 16886, data4: [161, 137, 250, 188, 34, 179, 35, 204] };
}
#[repr(C)]
pub struct IInteractionTrackerInertiaStateEnteredArgs3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsFromBinding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInteractionTrackerInertiaStateEnteredArgs3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1219238959, data2: 18365, data3: 22959, data4: [165, 140, 121, 189, 46, 185, 239, 113] };
}
#[repr(C)]
pub struct IInteractionTrackerInteractingStateEnteredArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequestId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInteractionTrackerInteractingStateEnteredArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2804300089, data2: 41339, data3: 16401, data4: [153, 253, 181, 194, 79, 20, 55, 72] };
}
#[repr(C)]
pub struct IInteractionTrackerInteractingStateEnteredArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsFromBinding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInteractionTrackerInteractingStateEnteredArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1352028886, data2: 54408, data3: 22989, data4: [129, 159, 245, 35, 16, 41, 91, 17] };
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
impl ::windows_sys::core::Interface for IInteractionTrackerOwner {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3677260531, data2: 19947, data3: 20051, data4: [178, 156, 176, 108, 159, 150, 214, 81] };
}
#[repr(C)]
pub struct IInteractionTrackerRequestIgnoredArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequestId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInteractionTrackerRequestIgnoredArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2162000625, data2: 52773, data3: 18575, data4: [145, 221, 203, 100, 85, 204, 255, 46] };
}
#[repr(C)]
pub struct IInteractionTrackerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithOwner: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, owner: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInteractionTrackerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3148208055, data2: 26000, data3: 17560, data4: [141, 108, 235, 98, 181, 20, 201, 42] };
}
#[repr(C)]
pub struct IInteractionTrackerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetBindingMode: unsafe extern "system" fn(this: *mut *mut Self, boundtracker1: *mut ::core::ffi::c_void, boundtracker2: *mut ::core::ffi::c_void, axismode: InteractionBindingAxisModes) -> ::windows_sys::core::HRESULT,
    pub GetBindingMode: unsafe extern "system" fn(this: *mut *mut Self, boundtracker1: *mut ::core::ffi::c_void, boundtracker2: *mut ::core::ffi::c_void, result__: *mut InteractionBindingAxisModes) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInteractionTrackerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 904214304, data2: 18103, data3: 23728, data4: [181, 5, 243, 214, 136, 74, 97, 99] };
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
impl ::windows_sys::core::Interface for IInteractionTrackerValuesChangedArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3474290927, data2: 54239, data3: 17665, data4: [185, 230, 240, 47, 178, 47, 115, 208] };
}
#[repr(C)]
pub struct IInteractionTrackerVector2InertiaModifier {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IInteractionTrackerVector2InertiaModifier {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2279639728, data2: 12422, data3: 18515, data4: [164, 183, 119, 136, 42, 213, 215, 227] };
}
#[repr(C)]
pub struct IInteractionTrackerVector2InertiaModifierFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IInteractionTrackerVector2InertiaModifierFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1946277572, data2: 27757, data3: 18655, data4: [188, 62, 23, 30, 34, 126, 125, 127] };
}
#[repr(C)]
pub struct IInteractionTrackerVector2InertiaNaturalMotion {
    pub base__: ::windows_sys::core::IInspectable,
    pub Condition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NaturalMotion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetNaturalMotion: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInteractionTrackerVector2InertiaNaturalMotion {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1595369820, data2: 5677, data3: 19463, data4: [148, 0, 194, 130, 178, 130, 118, 202] };
}
#[repr(C)]
pub struct IInteractionTrackerVector2InertiaNaturalMotionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInteractionTrackerVector2InertiaNaturalMotionStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2181044808, data2: 2496, data3: 17231, data4: [129, 137, 20, 28, 102, 223, 54, 47] };
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
impl ::windows_sys::core::Interface for IVisualInteractionSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3389950598, data2: 55510, data3: 16657, data4: [176, 136, 112, 52, 123, 210, 176, 237] };
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
impl ::windows_sys::core::Interface for IVisualInteractionSource2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2861648019, data2: 42812, data3: 16717, data4: [128, 208, 36, 155, 173, 47, 189, 147] };
}
#[repr(C)]
pub struct IVisualInteractionSource3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PointerWheelConfig: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVisualInteractionSource3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3644976938, data2: 3420, data3: 16471, data4: [146, 215, 201, 113, 21, 51, 32, 79] };
}
#[repr(C)]
pub struct IVisualInteractionSourceObjectFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IVisualInteractionSourceObjectFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2999619964, data2: 59786, data3: 16882, data4: [179, 201, 137, 28, 146, 102, 200, 246] };
}
#[repr(C)]
pub struct IVisualInteractionSourceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVisualInteractionSourceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 916022753, data2: 34373, data3: 20341, data4: [186, 0, 100, 121, 205, 16, 200, 230] };
}
#[repr(C)]
pub struct IVisualInteractionSourceStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromIVisualElement: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVisualInteractionSourceStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2843328562, data2: 22372, data3: 21984, data4: [188, 31, 7, 120, 120, 109, 207, 222] };
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
