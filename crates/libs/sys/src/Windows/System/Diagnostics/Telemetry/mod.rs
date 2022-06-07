#[repr(C)]
pub struct IPlatformTelemetryClientStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Register: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RegisterWithSettings: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlatformTelemetryClientStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2616455773, data2: 54723, data3: 20202, data4: [141, 190, 156, 141, 187, 13, 157, 143] };
}
#[repr(C)]
pub struct IPlatformTelemetryRegistrationResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PlatformTelemetryRegistrationStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlatformTelemetryRegistrationResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1300568235, data2: 8850, data3: 18877, data4: [161, 90, 61, 113, 210, 20, 81, 18] };
}
#[repr(C)]
pub struct IPlatformTelemetryRegistrationSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub StorageSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetStorageSize: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub UploadQuotaSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetUploadQuotaSize: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlatformTelemetryRegistrationSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2174387586, data2: 51737, data3: 16734, data4: [187, 121, 156, 34, 75, 250, 58, 115] };
}
pub type PlatformTelemetryRegistrationResult = *mut ::core::ffi::c_void;
pub type PlatformTelemetryRegistrationSettings = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System_Diagnostics_Telemetry\"`*"]
#[repr(transparent)]
pub struct PlatformTelemetryRegistrationStatus(pub i32);
impl PlatformTelemetryRegistrationStatus {
    pub const Success: Self = Self(0i32);
    pub const SettingsOutOfRange: Self = Self(1i32);
    pub const UnknownFailure: Self = Self(2i32);
}
impl ::core::marker::Copy for PlatformTelemetryRegistrationStatus {}
impl ::core::clone::Clone for PlatformTelemetryRegistrationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
