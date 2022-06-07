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
impl ::windows_sys::core::Interface for ILampArrayBitmapEffect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 842588261, data2: 55415, data3: 17959, data4: [137, 229, 42, 136, 247, 5, 47, 166] };
}
#[repr(C)]
pub struct ILampArrayBitmapEffectFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, lamparray: *mut ::core::ffi::c_void, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILampArrayBitmapEffectFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 325091472, data2: 58166, data3: 19599, data4: [144, 83, 169, 36, 7, 202, 123, 29] };
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
impl ::windows_sys::core::Interface for ILampArrayBitmapRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3367284638, data2: 65123, data3: 19793, data4: [186, 189, 97, 157, 239, 180, 84, 186] };
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
impl ::windows_sys::core::Interface for ILampArrayBlinkEffect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3955176950, data2: 12229, data3: 19379, data4: [179, 195, 98, 33, 167, 104, 13, 19] };
}
#[repr(C)]
pub struct ILampArrayBlinkEffectFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, lamparray: *mut ::core::ffi::c_void, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILampArrayBlinkEffectFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2275351959, data2: 40784, data3: 18866, data4: [165, 111, 1, 58, 160, 141, 85, 224] };
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
impl ::windows_sys::core::Interface for ILampArrayColorRampEffect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 721437751, data2: 16551, data3: 17198, data4: [160, 185, 13, 87, 12, 33, 83, 255] };
}
#[repr(C)]
pub struct ILampArrayColorRampEffectFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, lamparray: *mut ::core::ffi::c_void, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILampArrayColorRampEffectFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1376506163, data2: 3188, data3: 19957, data4: [190, 167, 72, 153, 224, 38, 107, 15] };
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
impl ::windows_sys::core::Interface for ILampArrayCustomEffect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3965161840, data2: 15412, data3: 18550, data4: [129, 139, 87, 101, 247, 139, 14, 228] };
}
#[repr(C)]
pub struct ILampArrayCustomEffectFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, lamparray: *mut ::core::ffi::c_void, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILampArrayCustomEffectFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1756657485, data2: 25573, data3: 19184, data4: [165, 139, 62, 83, 91, 148, 232, 201] };
}
#[repr(C)]
pub struct ILampArrayEffect {
    pub base__: ::windows_sys::core::IInspectable,
    pub ZIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetZIndex: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILampArrayEffect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 299128208, data2: 22523, data3: 17734, data4: [177, 206, 134, 49, 7, 247, 64, 223] };
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
impl ::windows_sys::core::Interface for ILampArrayEffectPlaylist {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2112195582, data2: 28513, data3: 16643, data4: [152, 199, 214, 99, 47, 123, 145, 105] };
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
impl ::windows_sys::core::Interface for ILampArrayEffectPlaylistStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4212466524, data2: 59957, data3: 19583, data4: [160, 22, 243, 191, 198, 166, 196, 125] };
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
impl ::windows_sys::core::Interface for ILampArraySolidEffect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1142915603, data2: 17356, data3: 19251, data4: [128, 235, 198, 221, 222, 125, 200, 237] };
}
#[repr(C)]
pub struct ILampArraySolidEffectFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, lamparray: *mut ::core::ffi::c_void, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILampArraySolidEffectFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4167213868, data2: 21878, data3: 17217, data4: [150, 27, 174, 225, 241, 60, 249, 221] };
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
impl ::windows_sys::core::Interface for ILampArrayUpdateRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1935019370, data2: 22378, data3: 18607, data4: [133, 57, 103, 255, 160, 171, 53, 22] };
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
