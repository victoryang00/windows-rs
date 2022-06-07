#[doc = "*Required features: `\"Security_Credentials_UI\"`*"]
#[repr(transparent)]
pub struct AuthenticationProtocol(pub i32);
impl AuthenticationProtocol {
    pub const Basic: Self = Self(0i32);
    pub const Digest: Self = Self(1i32);
    pub const Ntlm: Self = Self(2i32);
    pub const Kerberos: Self = Self(3i32);
    pub const Negotiate: Self = Self(4i32);
    pub const CredSsp: Self = Self(5i32);
    pub const Custom: Self = Self(6i32);
}
impl ::core::marker::Copy for AuthenticationProtocol {}
impl ::core::clone::Clone for AuthenticationProtocol {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CredentialPickerOptions = *mut ::core::ffi::c_void;
pub type CredentialPickerResults = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Credentials_UI\"`*"]
#[repr(transparent)]
pub struct CredentialSaveOption(pub i32);
impl CredentialSaveOption {
    pub const Unselected: Self = Self(0i32);
    pub const Selected: Self = Self(1i32);
    pub const Hidden: Self = Self(2i32);
}
impl ::core::marker::Copy for CredentialSaveOption {}
impl ::core::clone::Clone for CredentialSaveOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ICredentialPickerOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetCaption: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Caption: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetMessage: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetErrorCode: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TargetName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetAuthenticationProtocol: unsafe extern "system" fn(this: *mut *mut Self, value: AuthenticationProtocol) -> ::windows_sys::core::HRESULT,
    pub AuthenticationProtocol: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AuthenticationProtocol) -> ::windows_sys::core::HRESULT,
    pub SetCustomAuthenticationProtocol: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CustomAuthenticationProtocol: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SetPreviousCredential: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetPreviousCredential: usize,
    #[cfg(feature = "Storage_Streams")]
    pub PreviousCredential: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PreviousCredential: usize,
    pub SetAlwaysDisplayDialog: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AlwaysDisplayDialog: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCallerSavesCredential: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CallerSavesCredential: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCredentialSaveOption: unsafe extern "system" fn(this: *mut *mut Self, value: CredentialSaveOption) -> ::windows_sys::core::HRESULT,
    pub CredentialSaveOption: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CredentialSaveOption) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICredentialPickerOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2522483532, data2: 38394, data3: 18047, data4: [153, 43, 11, 34, 229, 133, 155, 246] };
}
#[repr(C)]
pub struct ICredentialPickerResults {
    pub base__: ::windows_sys::core::IInspectable,
    pub ErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub CredentialSaveOption: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CredentialSaveOption) -> ::windows_sys::core::HRESULT,
    pub CredentialSaved: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Credential: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Credential: usize,
    pub CredentialDomainName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CredentialUserName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CredentialPassword: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICredentialPickerResults {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 424212890, data2: 52272, data3: 16652, data4: [156, 56, 204, 8, 132, 197, 179, 215] };
}
#[repr(C)]
pub struct ICredentialPickerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PickWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickWithOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PickWithMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, targetname: ::windows_sys::core::HSTRING, message: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickWithMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PickWithCaptionAsync: unsafe extern "system" fn(this: *mut *mut Self, targetname: ::windows_sys::core::HSTRING, message: ::windows_sys::core::HSTRING, caption: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickWithCaptionAsync: usize,
}
impl ::windows_sys::core::Interface for ICredentialPickerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2855951475, data2: 51690, data3: 18306, data4: [153, 251, 230, 215, 233, 56, 225, 45] };
}
#[repr(C)]
pub struct IUserConsentVerifierStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CheckAvailabilityAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckAvailabilityAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestVerificationAsync: unsafe extern "system" fn(this: *mut *mut Self, message: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestVerificationAsync: usize,
}
impl ::windows_sys::core::Interface for IUserConsentVerifierStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2941206417, data2: 22092, data3: 19932, data4: [184, 181, 151, 52, 71, 98, 124, 101] };
}
#[doc = "*Required features: `\"Security_Credentials_UI\"`*"]
#[repr(transparent)]
pub struct UserConsentVerificationResult(pub i32);
impl UserConsentVerificationResult {
    pub const Verified: Self = Self(0i32);
    pub const DeviceNotPresent: Self = Self(1i32);
    pub const NotConfiguredForUser: Self = Self(2i32);
    pub const DisabledByPolicy: Self = Self(3i32);
    pub const DeviceBusy: Self = Self(4i32);
    pub const RetriesExhausted: Self = Self(5i32);
    pub const Canceled: Self = Self(6i32);
}
impl ::core::marker::Copy for UserConsentVerificationResult {}
impl ::core::clone::Clone for UserConsentVerificationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_Credentials_UI\"`*"]
#[repr(transparent)]
pub struct UserConsentVerifierAvailability(pub i32);
impl UserConsentVerifierAvailability {
    pub const Available: Self = Self(0i32);
    pub const DeviceNotPresent: Self = Self(1i32);
    pub const NotConfiguredForUser: Self = Self(2i32);
    pub const DisabledByPolicy: Self = Self(3i32);
    pub const DeviceBusy: Self = Self(4i32);
}
impl ::core::marker::Copy for UserConsentVerifierAvailability {}
impl ::core::clone::Clone for UserConsentVerifierAvailability {
    fn clone(&self) -> Self {
        *self
    }
}
