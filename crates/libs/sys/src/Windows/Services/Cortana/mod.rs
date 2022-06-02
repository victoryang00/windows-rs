pub type CortanaActionableInsights = *mut ::core::ffi::c_void;
pub type CortanaActionableInsightsOptions = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Services_Cortana\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct CortanaPermission(pub i32);
#[cfg(feature = "deprecated")]
impl CortanaPermission {
    pub const BrowsingHistory: Self = Self(0i32);
    pub const Calendar: Self = Self(1i32);
    pub const CallHistory: Self = Self(2i32);
    pub const Contacts: Self = Self(3i32);
    pub const Email: Self = Self(4i32);
    pub const InputPersonalization: Self = Self(5i32);
    pub const Location: Self = Self(6i32);
    pub const Messaging: Self = Self(7i32);
    pub const Microphone: Self = Self(8i32);
    pub const Personalization: Self = Self(9i32);
    pub const PhoneCall: Self = Self(10i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for CortanaPermission {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for CortanaPermission {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Services_Cortana\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct CortanaPermissionsChangeResult(pub i32);
#[cfg(feature = "deprecated")]
impl CortanaPermissionsChangeResult {
    pub const Success: Self = Self(0i32);
    pub const Unavailable: Self = Self(1i32);
    pub const DisabledByPolicy: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for CortanaPermissionsChangeResult {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for CortanaPermissionsChangeResult {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CortanaPermissionsManager = *mut ::core::ffi::c_void;
pub type CortanaSettings = *mut ::core::ffi::c_void;
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ICortanaActionableInsights {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "System", feature = "deprecated"))]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "System", feature = "deprecated")))]
    User: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub IsAvailableAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    IsAvailableAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub ShowInsightsForImageAsync: unsafe extern "system" fn(this: *mut *mut Self, imagestream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated")))]
    ShowInsightsForImageAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub ShowInsightsForImageWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, imagestream: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated")))]
    ShowInsightsForImageWithOptionsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ShowInsightsForTextAsync: unsafe extern "system" fn(this: *mut *mut Self, text: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ShowInsightsForTextAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ShowInsightsForTextWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, text: ::windows_sys::core::HSTRING, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ShowInsightsForTextWithOptionsAsync: usize,
    #[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "deprecated"))]
    pub ShowInsightsAsync: unsafe extern "system" fn(this: *mut *mut Self, datapackage: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "deprecated")))]
    ShowInsightsAsync: usize,
    #[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "deprecated"))]
    pub ShowInsightsWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, datapackage: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "deprecated")))]
    ShowInsightsWithOptionsAsync: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ICortanaActionableInsightsOptions {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ContentSourceWebLink: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ContentSourceWebLink: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetContentSourceWebLink: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetContentSourceWebLink: usize,
    #[cfg(feature = "deprecated")]
    pub SurroundingText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SurroundingText: usize,
    #[cfg(feature = "deprecated")]
    pub SetSurroundingText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetSurroundingText: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ICortanaActionableInsightsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetDefault: usize,
    #[cfg(all(feature = "System", feature = "deprecated"))]
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "System", feature = "deprecated")))]
    GetForUser: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ICortanaPermissionsManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsSupported: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub ArePermissionsGrantedAsync: unsafe extern "system" fn(this: *mut *mut Self, permissions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    ArePermissionsGrantedAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GrantPermissionsAsync: unsafe extern "system" fn(this: *mut *mut Self, permissions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GrantPermissionsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub RevokePermissionsAsync: unsafe extern "system" fn(this: *mut *mut Self, permissions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    RevokePermissionsAsync: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ICortanaPermissionsManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetDefault: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ICortanaSettings {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub HasUserConsentToVoiceActivation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    HasUserConsentToVoiceActivation: usize,
    #[cfg(feature = "deprecated")]
    pub IsVoiceActivationEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsVoiceActivationEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub SetIsVoiceActivationEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetIsVoiceActivationEnabled: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ICortanaSettingsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsSupported: usize,
    #[cfg(feature = "deprecated")]
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetDefault: usize,
}
