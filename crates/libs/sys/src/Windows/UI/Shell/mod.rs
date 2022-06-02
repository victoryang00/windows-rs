#[repr(C)]
pub struct IAdaptiveCard {
    pub base__: ::windows_sys::core::IInspectable,
    pub ToJson: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAdaptiveCardBuilderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateAdaptiveCardFromJson: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISecurityAppManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Register: unsafe extern "system" fn(this: *mut *mut Self, kind: SecurityAppKind, displayname: ::windows_sys::core::HSTRING, detailsuri: *mut ::core::ffi::c_void, registerperuser: bool, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Register: usize,
    pub Unregister: unsafe extern "system" fn(this: *mut *mut Self, kind: SecurityAppKind, guidregistration: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UpdateState: unsafe extern "system" fn(this: *mut *mut Self, kind: SecurityAppKind, guidregistration: ::windows_sys::core::GUID, state: SecurityAppState, substatus: SecurityAppSubstatus, detailsuri: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateState: usize,
}
#[repr(C)]
pub struct IShareWindowCommandEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub WindowId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::WindowId) -> ::windows_sys::core::HRESULT,
    pub Command: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ShareWindowCommand) -> ::windows_sys::core::HRESULT,
    pub SetCommand: unsafe extern "system" fn(this: *mut *mut Self, value: ShareWindowCommand) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IShareWindowCommandSource {
    pub base__: ::windows_sys::core::IInspectable,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ReportCommandChanged: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CommandRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CommandRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCommandRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCommandRequested: usize,
    #[cfg(feature = "Foundation")]
    pub CommandInvoked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CommandInvoked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCommandInvoked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCommandInvoked: usize,
}
#[repr(C)]
pub struct IShareWindowCommandSourceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITaskbarManager {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsPinningAllowed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsCurrentAppPinnedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsCurrentAppPinnedAsync: usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub IsAppListEntryPinnedAsync: unsafe extern "system" fn(this: *mut *mut Self, applistentry: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation")))]
    IsAppListEntryPinnedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestPinCurrentAppAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPinCurrentAppAsync: usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub RequestPinAppListEntryAsync: unsafe extern "system" fn(this: *mut *mut Self, applistentry: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation")))]
    RequestPinAppListEntryAsync: usize,
}
#[repr(C)]
pub struct ITaskbarManager2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub IsSecondaryTilePinnedAsync: unsafe extern "system" fn(this: *mut *mut Self, tileid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsSecondaryTilePinnedAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_StartScreen"))]
    pub RequestPinSecondaryTileAsync: unsafe extern "system" fn(this: *mut *mut Self, secondarytile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_StartScreen")))]
    RequestPinSecondaryTileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryUnpinSecondaryTileAsync: unsafe extern "system" fn(this: *mut *mut Self, tileid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryUnpinSecondaryTileAsync: usize,
}
#[repr(C)]
pub struct ITaskbarManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Shell\"`*"]
#[repr(transparent)]
pub struct SecurityAppKind(pub i32);
impl SecurityAppKind {
    pub const WebProtection: Self = Self(0i32);
}
impl ::core::marker::Copy for SecurityAppKind {}
impl ::core::clone::Clone for SecurityAppKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SecurityAppManager = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Shell\"`*"]
#[repr(transparent)]
pub struct SecurityAppState(pub i32);
impl SecurityAppState {
    pub const Disabled: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
}
impl ::core::marker::Copy for SecurityAppState {}
impl ::core::clone::Clone for SecurityAppState {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Shell\"`*"]
#[repr(transparent)]
pub struct SecurityAppSubstatus(pub i32);
impl SecurityAppSubstatus {
    pub const Undetermined: Self = Self(0i32);
    pub const NoActionNeeded: Self = Self(1i32);
    pub const ActionRecommended: Self = Self(2i32);
    pub const ActionNeeded: Self = Self(3i32);
}
impl ::core::marker::Copy for SecurityAppSubstatus {}
impl ::core::clone::Clone for SecurityAppSubstatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Shell\"`*"]
#[repr(transparent)]
pub struct ShareWindowCommand(pub i32);
impl ShareWindowCommand {
    pub const None: Self = Self(0i32);
    pub const StartSharing: Self = Self(1i32);
    pub const StopSharing: Self = Self(2i32);
}
impl ::core::marker::Copy for ShareWindowCommand {}
impl ::core::clone::Clone for ShareWindowCommand {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ShareWindowCommandEventArgs = *mut ::core::ffi::c_void;
pub type ShareWindowCommandSource = *mut ::core::ffi::c_void;
pub type TaskbarManager = *mut ::core::ffi::c_void;
