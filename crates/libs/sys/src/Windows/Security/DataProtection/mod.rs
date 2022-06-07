#[repr(C)]
pub struct IUserDataAvailabilityStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IUserDataAvailabilityStateChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2808447689, data2: 1698, data3: 17011, data4: [168, 3, 131, 76, 159, 135, 251, 235] };
}
#[repr(C)]
pub struct IUserDataBufferUnprotectResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UserDataBufferUnprotectStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub UnprotectedBuffer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    UnprotectedBuffer: usize,
}
impl ::windows_sys::core::Interface for IUserDataBufferUnprotectResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2398949008, data2: 64154, data3: 18084, data4: [163, 119, 1, 206, 191, 30, 116, 216] };
}
#[repr(C)]
pub struct IUserDataProtectionManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub ProtectStorageItemAsync: unsafe extern "system" fn(this: *mut *mut Self, storageitem: *mut ::core::ffi::c_void, availability: UserDataAvailability, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    ProtectStorageItemAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub GetStorageItemProtectionInfoAsync: unsafe extern "system" fn(this: *mut *mut Self, storageitem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    GetStorageItemProtectionInfoAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ProtectBufferAsync: unsafe extern "system" fn(this: *mut *mut Self, unprotectedbuffer: *mut ::core::ffi::c_void, availability: UserDataAvailability, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ProtectBufferAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub UnprotectBufferAsync: unsafe extern "system" fn(this: *mut *mut Self, protectedbuffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    UnprotectBufferAsync: usize,
    pub IsContinuedDataAvailabilityExpected: unsafe extern "system" fn(this: *mut *mut Self, availability: UserDataAvailability, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DataAvailabilityStateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DataAvailabilityStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDataAvailabilityStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDataAvailabilityStateChanged: usize,
}
impl ::windows_sys::core::Interface for IUserDataProtectionManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 521347965, data2: 46126, data3: 19080, data4: [148, 128, 15, 36, 9, 36, 200, 118] };
}
#[repr(C)]
pub struct IUserDataProtectionManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryGetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub TryGetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    TryGetForUser: usize,
}
impl ::windows_sys::core::Interface for IUserDataProtectionManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2541191400, data2: 28110, data3: 20398, data4: [175, 133, 120, 42, 194, 207, 69, 114] };
}
#[repr(C)]
pub struct IUserDataStorageItemProtectionInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub Availability: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UserDataAvailability) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserDataStorageItemProtectionInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1533444342, data2: 59519, data3: 16545, data4: [177, 157, 166, 24, 122, 12, 102, 47] };
}
#[doc = "*Required features: `\"Security_DataProtection\"`*"]
#[repr(transparent)]
pub struct UserDataAvailability(pub i32);
impl UserDataAvailability {
    pub const Always: Self = Self(0i32);
    pub const AfterFirstUnlock: Self = Self(1i32);
    pub const WhileUnlocked: Self = Self(2i32);
}
impl ::core::marker::Copy for UserDataAvailability {}
impl ::core::clone::Clone for UserDataAvailability {
    fn clone(&self) -> Self {
        *self
    }
}
pub type UserDataAvailabilityStateChangedEventArgs = *mut ::core::ffi::c_void;
pub type UserDataBufferUnprotectResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_DataProtection\"`*"]
#[repr(transparent)]
pub struct UserDataBufferUnprotectStatus(pub i32);
impl UserDataBufferUnprotectStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const Unavailable: Self = Self(1i32);
}
impl ::core::marker::Copy for UserDataBufferUnprotectStatus {}
impl ::core::clone::Clone for UserDataBufferUnprotectStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type UserDataProtectionManager = *mut ::core::ffi::c_void;
pub type UserDataStorageItemProtectionInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_DataProtection\"`*"]
#[repr(transparent)]
pub struct UserDataStorageItemProtectionStatus(pub i32);
impl UserDataStorageItemProtectionStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const NotProtectable: Self = Self(1i32);
    pub const DataUnavailable: Self = Self(2i32);
}
impl ::core::marker::Copy for UserDataStorageItemProtectionStatus {}
impl ::core::clone::Clone for UserDataStorageItemProtectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
