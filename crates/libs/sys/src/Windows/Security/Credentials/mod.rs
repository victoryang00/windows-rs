#[cfg(feature = "Security_Credentials_UI")]
pub mod UI;
#[repr(C)]
pub struct ICredentialFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreatePasswordCredential: unsafe extern "system" fn(this: *mut *mut Self, resource: ::windows_sys::core::HSTRING, username: ::windows_sys::core::HSTRING, password: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKeyCredential {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub RetrievePublicKeyWithDefaultBlobType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RetrievePublicKeyWithDefaultBlobType: usize,
    #[cfg(all(feature = "Security_Cryptography_Core", feature = "Storage_Streams"))]
    pub RetrievePublicKeyWithBlobType: unsafe extern "system" fn(this: *mut *mut Self, blobtype: super::Cryptography::Core::CryptographicPublicKeyBlobType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Security_Cryptography_Core", feature = "Storage_Streams")))]
    RetrievePublicKeyWithBlobType: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub RequestSignAsync: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    RequestSignAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAttestationAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAttestationAsync: usize,
}
#[repr(C)]
pub struct IKeyCredentialAttestationResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub CertificateChainBuffer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CertificateChainBuffer: usize,
    #[cfg(feature = "Storage_Streams")]
    pub AttestationBuffer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AttestationBuffer: usize,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut KeyCredentialAttestationStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKeyCredentialManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub IsSupportedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsSupportedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RenewAttestationAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RenewAttestationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestCreateAsync: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, option: KeyCredentialCreationOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestCreateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub OpenAsync: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
}
#[repr(C)]
pub struct IKeyCredentialOperationResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Result: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Result: usize,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut KeyCredentialStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKeyCredentialRetrievalResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Credential: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut KeyCredentialStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPasswordCredential {
    pub base__: ::windows_sys::core::IInspectable,
    pub Resource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetResource: unsafe extern "system" fn(this: *mut *mut Self, resource: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub UserName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetUserName: unsafe extern "system" fn(this: *mut *mut Self, username: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Password: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(this: *mut *mut Self, password: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RetrievePassword: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[repr(C)]
pub struct IPasswordVault {
    pub base__: ::windows_sys::core::IInspectable,
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, credential: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, credential: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Retrieve: unsafe extern "system" fn(this: *mut *mut Self, resource: ::windows_sys::core::HSTRING, username: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllByResource: unsafe extern "system" fn(this: *mut *mut Self, resource: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllByResource: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllByUserName: unsafe extern "system" fn(this: *mut *mut Self, username: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllByUserName: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RetrieveAll: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RetrieveAll: usize,
}
#[repr(C)]
pub struct IWebAccount {
    pub base__: ::windows_sys::core::IInspectable,
    pub WebAccountProvider: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UserName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WebAccountState) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebAccount2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetPictureAsync: unsafe extern "system" fn(this: *mut *mut Self, desizedsize: WebAccountPictureSize, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetPictureAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SignOutAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SignOutAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SignOutWithClientIdAsync: unsafe extern "system" fn(this: *mut *mut Self, clientid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SignOutWithClientIdAsync: usize,
}
#[repr(C)]
pub struct IWebAccountFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWebAccount: unsafe extern "system" fn(this: *mut *mut Self, webaccountprovider: *mut ::core::ffi::c_void, username: ::windows_sys::core::HSTRING, state: WebAccountState, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebAccountProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub IconUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    IconUri: usize,
}
#[repr(C)]
pub struct IWebAccountProvider2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayPurpose: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Authority: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebAccountProvider3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[repr(C)]
pub struct IWebAccountProvider4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSystemProvider: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebAccountProviderFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateWebAccountProvider: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, displayname: ::windows_sys::core::HSTRING, iconuri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWebAccountProvider: usize,
}
pub type KeyCredential = *mut ::core::ffi::c_void;
pub type KeyCredentialAttestationResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Credentials\"`*"]
#[repr(transparent)]
pub struct KeyCredentialAttestationStatus(pub i32);
impl KeyCredentialAttestationStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const NotSupported: Self = Self(2i32);
    pub const TemporaryFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for KeyCredentialAttestationStatus {}
impl ::core::clone::Clone for KeyCredentialAttestationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_Credentials\"`*"]
#[repr(transparent)]
pub struct KeyCredentialCreationOption(pub i32);
impl KeyCredentialCreationOption {
    pub const ReplaceExisting: Self = Self(0i32);
    pub const FailIfExists: Self = Self(1i32);
}
impl ::core::marker::Copy for KeyCredentialCreationOption {}
impl ::core::clone::Clone for KeyCredentialCreationOption {
    fn clone(&self) -> Self {
        *self
    }
}
pub type KeyCredentialOperationResult = *mut ::core::ffi::c_void;
pub type KeyCredentialRetrievalResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Credentials\"`*"]
#[repr(transparent)]
pub struct KeyCredentialStatus(pub i32);
impl KeyCredentialStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const NotFound: Self = Self(2i32);
    pub const UserCanceled: Self = Self(3i32);
    pub const UserPrefersPassword: Self = Self(4i32);
    pub const CredentialAlreadyExists: Self = Self(5i32);
    pub const SecurityDeviceLocked: Self = Self(6i32);
}
impl ::core::marker::Copy for KeyCredentialStatus {}
impl ::core::clone::Clone for KeyCredentialStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PasswordCredential = *mut ::core::ffi::c_void;
pub type PasswordCredentialPropertyStore = *mut ::core::ffi::c_void;
pub type PasswordVault = *mut ::core::ffi::c_void;
pub type WebAccount = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Credentials\"`*"]
#[repr(transparent)]
pub struct WebAccountPictureSize(pub i32);
impl WebAccountPictureSize {
    pub const Size64x64: Self = Self(64i32);
    pub const Size208x208: Self = Self(208i32);
    pub const Size424x424: Self = Self(424i32);
    pub const Size1080x1080: Self = Self(1080i32);
}
impl ::core::marker::Copy for WebAccountPictureSize {}
impl ::core::clone::Clone for WebAccountPictureSize {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WebAccountProvider = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Credentials\"`*"]
#[repr(transparent)]
pub struct WebAccountState(pub i32);
impl WebAccountState {
    pub const None: Self = Self(0i32);
    pub const Connected: Self = Self(1i32);
    pub const Error: Self = Self(2i32);
}
impl ::core::marker::Copy for WebAccountState {}
impl ::core::clone::Clone for WebAccountState {
    fn clone(&self) -> Self {
        *self
    }
}
