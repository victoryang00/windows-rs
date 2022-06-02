#[repr(C)]
pub struct ILampArrayBitmapEffect {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    #[cfg(feature = "Foundation")]
    pub StartDelay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartDelay: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartDelay: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartDelay: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateInterval: usize,
    #[cfg(feature = "Foundation")]
    pub SetUpdateInterval: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUpdateInterval: usize,
    #[cfg(feature = "Foundation")]
    pub SuggestedBitmapSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SuggestedBitmapSize: usize,
    #[cfg(feature = "Foundation")]
    pub BitmapRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BitmapRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBitmapRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBitmapRequested: usize,
}
#[repr(C)]
pub struct ILampArrayBitmapEffectFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, lamparray: *mut ::core::ffi::c_void, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILampArrayBitmapRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SinceStarted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SinceStarted: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub UpdateBitmap: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    UpdateBitmap: usize,
}
#[repr(C)]
pub struct ILampArrayBlinkEffect {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI")]
    pub Color: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Color: usize,
    #[cfg(feature = "UI")]
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColor: usize,
    #[cfg(feature = "Foundation")]
    pub AttackDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AttackDuration: usize,
    #[cfg(feature = "Foundation")]
    pub SetAttackDuration: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAttackDuration: usize,
    #[cfg(feature = "Foundation")]
    pub SustainDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SustainDuration: usize,
    #[cfg(feature = "Foundation")]
    pub SetSustainDuration: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSustainDuration: usize,
    #[cfg(feature = "Foundation")]
    pub DecayDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DecayDuration: usize,
    #[cfg(feature = "Foundation")]
    pub SetDecayDuration: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDecayDuration: usize,
    #[cfg(feature = "Foundation")]
    pub RepetitionDelay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RepetitionDelay: usize,
    #[cfg(feature = "Foundation")]
    pub SetRepetitionDelay: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRepetitionDelay: usize,
    #[cfg(feature = "Foundation")]
    pub StartDelay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartDelay: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartDelay: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartDelay: usize,
    pub Occurrences: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetOccurrences: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub RepetitionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LampArrayRepetitionMode) -> ::windows_sys::core::HRESULT,
    pub SetRepetitionMode: unsafe extern "system" fn(this: *mut *mut Self, value: LampArrayRepetitionMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILampArrayBlinkEffectFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, lamparray: *mut ::core::ffi::c_void, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILampArrayColorRampEffect {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI")]
    pub Color: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Color: usize,
    #[cfg(feature = "UI")]
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColor: usize,
    #[cfg(feature = "Foundation")]
    pub RampDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RampDuration: usize,
    #[cfg(feature = "Foundation")]
    pub SetRampDuration: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRampDuration: usize,
    #[cfg(feature = "Foundation")]
    pub StartDelay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartDelay: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartDelay: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartDelay: usize,
    pub CompletionBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LampArrayEffectCompletionBehavior) -> ::windows_sys::core::HRESULT,
    pub SetCompletionBehavior: unsafe extern "system" fn(this: *mut *mut Self, value: LampArrayEffectCompletionBehavior) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILampArrayColorRampEffectFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, lamparray: *mut ::core::ffi::c_void, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILampArrayCustomEffect {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateInterval: usize,
    #[cfg(feature = "Foundation")]
    pub SetUpdateInterval: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUpdateInterval: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdateRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdateRequested: usize,
}
#[repr(C)]
pub struct ILampArrayCustomEffectFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, lamparray: *mut ::core::ffi::c_void, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILampArrayEffect {
    pub base__: ::windows_sys::core::IInspectable,
    pub ZIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetZIndex: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILampArrayEffectPlaylist {
    pub base__: ::windows_sys::core::IInspectable,
    pub Append: unsafe extern "system" fn(this: *mut *mut Self, effect: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OverrideZIndex: unsafe extern "system" fn(this: *mut *mut Self, zindex: i32) -> ::windows_sys::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub EffectStartMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LampArrayEffectStartMode) -> ::windows_sys::core::HRESULT,
    pub SetEffectStartMode: unsafe extern "system" fn(this: *mut *mut Self, value: LampArrayEffectStartMode) -> ::windows_sys::core::HRESULT,
    pub Occurrences: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetOccurrences: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub RepetitionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LampArrayRepetitionMode) -> ::windows_sys::core::HRESULT,
    pub SetRepetitionMode: unsafe extern "system" fn(this: *mut *mut Self, value: LampArrayRepetitionMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILampArrayEffectPlaylistStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub StartAll: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StartAll: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StopAll: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StopAll: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PauseAll: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PauseAll: usize,
}
#[repr(C)]
pub struct ILampArraySolidEffect {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI")]
    pub Color: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Color: usize,
    #[cfg(feature = "UI")]
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColor: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    #[cfg(feature = "Foundation")]
    pub StartDelay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartDelay: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartDelay: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartDelay: usize,
    pub CompletionBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LampArrayEffectCompletionBehavior) -> ::windows_sys::core::HRESULT,
    pub SetCompletionBehavior: unsafe extern "system" fn(this: *mut *mut Self, value: LampArrayEffectCompletionBehavior) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILampArraySolidEffectFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, lamparray: *mut ::core::ffi::c_void, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILampArrayUpdateRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SinceStarted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SinceStarted: usize,
    #[cfg(feature = "UI")]
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, desiredcolor: super::super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColor: usize,
    #[cfg(feature = "UI")]
    pub SetColorForIndex: unsafe extern "system" fn(this: *mut *mut Self, lampindex: i32, desiredcolor: super::super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColorForIndex: usize,
    #[cfg(feature = "UI")]
    pub SetSingleColorForIndices: unsafe extern "system" fn(this: *mut *mut Self, desiredcolor: super::super::super::UI::Color, lampIndexes_array_size: u32, lampindexes: *const i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetSingleColorForIndices: usize,
    #[cfg(feature = "UI")]
    pub SetColorsForIndices: unsafe extern "system" fn(this: *mut *mut Self, desiredColors_array_size: u32, desiredcolors: *const super::super::super::UI::Color, lampIndexes_array_size: u32, lampindexes: *const i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColorsForIndices: usize,
}
pub type LampArrayBitmapEffect = *mut ::core::ffi::c_void;
pub type LampArrayBitmapRequestedEventArgs = *mut ::core::ffi::c_void;
pub type LampArrayBlinkEffect = *mut ::core::ffi::c_void;
pub type LampArrayColorRampEffect = *mut ::core::ffi::c_void;
pub type LampArrayCustomEffect = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Lights_Effects\"`*"]
#[repr(transparent)]
pub struct LampArrayEffectCompletionBehavior(pub i32);
impl LampArrayEffectCompletionBehavior {
    pub const ClearState: Self = Self(0i32);
    pub const KeepState: Self = Self(1i32);
}
impl ::core::marker::Copy for LampArrayEffectCompletionBehavior {}
impl ::core::clone::Clone for LampArrayEffectCompletionBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LampArrayEffectPlaylist = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Lights_Effects\"`*"]
#[repr(transparent)]
pub struct LampArrayEffectStartMode(pub i32);
impl LampArrayEffectStartMode {
    pub const Sequential: Self = Self(0i32);
    pub const Simultaneous: Self = Self(1i32);
}
impl ::core::marker::Copy for LampArrayEffectStartMode {}
impl ::core::clone::Clone for LampArrayEffectStartMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Lights_Effects\"`*"]
#[repr(transparent)]
pub struct LampArrayRepetitionMode(pub i32);
impl LampArrayRepetitionMode {
    pub const Occurrences: Self = Self(0i32);
    pub const Forever: Self = Self(1i32);
}
impl ::core::marker::Copy for LampArrayRepetitionMode {}
impl ::core::clone::Clone for LampArrayRepetitionMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LampArraySolidEffect = *mut ::core::ffi::c_void;
pub type LampArrayUpdateRequestedEventArgs = *mut ::core::ffi::c_void;
