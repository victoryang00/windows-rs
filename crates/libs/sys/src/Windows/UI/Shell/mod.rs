#[repr(C)]
pub struct IAdaptiveCard {
    pub base__: ::windows_sys::core::IInspectable,
    pub ToJson: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAdaptiveCard {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1926256268, data2: 41588, data3: 16845, data4: [130, 168, 152, 157, 64, 185, 176, 94] };
}
#[repr(C)]
pub struct IAdaptiveCardBuilderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateAdaptiveCardFromJson: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAdaptiveCardBuilderStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1986891528, data2: 54270, data3: 17223, data4: [160, 188, 185, 234, 154, 109, 194, 142] };
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
impl ::windows_sys::core::Interface for ISecurityAppManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2527875084, data2: 44756, data3: 22045, data4: [189, 232, 149, 53, 32, 52, 58, 45] };
}
#[repr(C)]
pub struct IShareWindowCommandEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub WindowId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::WindowId) -> ::windows_sys::core::HRESULT,
    pub Command: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ShareWindowCommand) -> ::windows_sys::core::HRESULT,
    pub SetCommand: unsafe extern "system" fn(this: *mut *mut Self, value: ShareWindowCommand) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IShareWindowCommandEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1165548553, data2: 42275, data3: 22358, data4: [169, 149, 228, 254, 185, 145, 255, 240] };
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
impl ::windows_sys::core::Interface for IShareWindowCommandSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3409672931, data2: 27548, data3: 22046, data4: [188, 204, 97, 230, 142, 10, 191, 239] };
}
#[repr(C)]
pub struct IShareWindowCommandSourceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IShareWindowCommandSourceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2968217174, data2: 40108, data3: 20860, data4: [182, 199, 142, 247, 21, 8, 66, 149] };
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
impl ::windows_sys::core::Interface for ITaskbarManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2269710873, data2: 6873, data3: 18932, data4: [178, 232, 134, 115, 141, 197, 172, 64] };
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
impl ::windows_sys::core::Interface for ITaskbarManager2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2045812846, data2: 31490, data3: 18705, data4: [145, 140, 222, 224, 187, 210, 11, 164] };
}
#[repr(C)]
pub struct ITaskbarManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITaskbarManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3677530996, data2: 56914, data3: 20454, data4: [183, 182, 149, 255, 159, 131, 149, 223] };
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
