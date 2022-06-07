#[cfg(feature = "Security_Credentials_UI")]
pub mod UI;
#[repr(C)]
pub struct ICredentialFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreatePasswordCredential: unsafe extern "system" fn(this: *mut *mut Self, resource: ::windows_sys::core::HSTRING, username: ::windows_sys::core::HSTRING, password: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICredentialFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1424954273, data2: 48934, data3: 18357, data4: [151, 221, 222, 119, 155, 124, 173, 88] };
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
impl ::windows_sys::core::Interface for IKeyCredential {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2508582797, data2: 17787, data3: 18503, data4: [177, 26, 250, 150, 11, 189, 177, 56] };
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
impl ::windows_sys::core::Interface for IKeyCredentialAttestationResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2024453025, data2: 41921, data3: 16643, data4: [182, 204, 71, 44, 68, 23, 28, 187] };
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
impl ::windows_sys::core::Interface for IKeyCredentialManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1789675147, data2: 3825, data3: 19680, data4: [130, 144, 65, 6, 218, 106, 99, 181] };
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
impl ::windows_sys::core::Interface for IKeyCredentialOperationResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4114056897, data2: 21089, data3: 19677, data4: [151, 109, 204, 144, 154, 199, 22, 32] };
}
#[repr(C)]
pub struct IKeyCredentialRetrievalResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Credential: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut KeyCredentialStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IKeyCredentialRetrievalResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1489860355, data2: 36231, data3: 16969, data4: [155, 88, 246, 89, 140, 201, 100, 78] };
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
impl ::windows_sys::core::Interface for IPasswordCredential {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1790019977, data2: 50976, data3: 16807, data4: [166, 193, 254, 173, 179, 99, 41, 160] };
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
impl ::windows_sys::core::Interface for IPasswordVault {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1643981835, data2: 51412, data3: 18625, data4: [165, 79, 188, 90, 100, 32, 90, 242] };
}
#[repr(C)]
pub struct IWebAccount {
    pub base__: ::windows_sys::core::IInspectable,
    pub WebAccountProvider: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UserName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WebAccountState) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebAccount {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1766276786, data2: 32817, data3: 18878, data4: [128, 187, 150, 203, 70, 217, 154, 186] };
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
impl ::windows_sys::core::Interface for IWebAccount2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2069288696, data2: 39179, data3: 20149, data4: [148, 167, 86, 33, 243, 168, 184, 36] };
}
#[repr(C)]
pub struct IWebAccountFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWebAccount: unsafe extern "system" fn(this: *mut *mut Self, webaccountprovider: *mut ::core::ffi::c_void, username: ::windows_sys::core::HSTRING, state: WebAccountState, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebAccountFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2895838009, data2: 7657, data3: 20114, data4: [183, 143, 5, 129, 168, 127, 110, 92] };
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
impl ::windows_sys::core::Interface for IWebAccountProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 702335171, data2: 31417, data3: 19068, data4: [163, 54, 185, 66, 249, 219, 247, 199] };
}
#[repr(C)]
pub struct IWebAccountProvider2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayPurpose: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Authority: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebAccountProvider2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1241639685, data2: 20034, data3: 16852, data4: [181, 24, 224, 8, 165, 22, 54, 20] };
}
#[repr(C)]
pub struct IWebAccountProvider3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
impl ::windows_sys::core::Interface for IWebAccountProvider3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3659288971, data2: 38669, data3: 19785, data4: [130, 92, 242, 112, 111, 140, 167, 254] };
}
#[repr(C)]
pub struct IWebAccountProvider4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSystemProvider: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebAccountProvider4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1905252571, data2: 59286, data3: 16912, data4: [183, 78, 132, 210, 152, 148, 176, 128] };
}
#[repr(C)]
pub struct IWebAccountProviderFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateWebAccountProvider: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, displayname: ::windows_sys::core::HSTRING, iconuri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWebAccountProvider: usize,
}
impl ::windows_sys::core::Interface for IWebAccountProviderFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 494304753, data2: 57825, data3: 19354, data4: [167, 116, 92, 124, 126, 59, 243, 113] };
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
