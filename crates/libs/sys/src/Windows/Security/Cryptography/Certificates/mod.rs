pub type Certificate = *mut ::core::ffi::c_void;
pub type CertificateChain = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
pub struct CertificateChainPolicy(pub i32);
impl CertificateChainPolicy {
    pub const Base: Self = Self(0i32);
    pub const Ssl: Self = Self(1i32);
    pub const NTAuthentication: Self = Self(2i32);
    pub const MicrosoftRoot: Self = Self(3i32);
}
impl ::core::marker::Copy for CertificateChainPolicy {}
impl ::core::clone::Clone for CertificateChainPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CertificateExtension = *mut ::core::ffi::c_void;
pub type CertificateKeyUsages = *mut ::core::ffi::c_void;
pub type CertificateQuery = *mut ::core::ffi::c_void;
pub type CertificateRequestProperties = *mut ::core::ffi::c_void;
pub type CertificateStore = *mut ::core::ffi::c_void;
pub type ChainBuildingParameters = *mut ::core::ffi::c_void;
pub type ChainValidationParameters = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
pub struct ChainValidationResult(pub i32);
impl ChainValidationResult {
    pub const Success: Self = Self(0i32);
    pub const Untrusted: Self = Self(1i32);
    pub const Revoked: Self = Self(2i32);
    pub const Expired: Self = Self(3i32);
    pub const IncompleteChain: Self = Self(4i32);
    pub const InvalidSignature: Self = Self(5i32);
    pub const WrongUsage: Self = Self(6i32);
    pub const InvalidName: Self = Self(7i32);
    pub const InvalidCertificateAuthorityPolicy: Self = Self(8i32);
    pub const BasicConstraintsError: Self = Self(9i32);
    pub const UnknownCriticalExtension: Self = Self(10i32);
    pub const RevocationInformationMissing: Self = Self(11i32);
    pub const RevocationFailure: Self = Self(12i32);
    pub const OtherErrors: Self = Self(13i32);
}
impl ::core::marker::Copy for ChainValidationResult {}
impl ::core::clone::Clone for ChainValidationResult {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CmsAttachedSignature = *mut ::core::ffi::c_void;
pub type CmsDetachedSignature = *mut ::core::ffi::c_void;
pub type CmsSignerInfo = *mut ::core::ffi::c_void;
pub type CmsTimestampInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
pub struct EnrollKeyUsages(pub u32);
impl EnrollKeyUsages {
    pub const None: Self = Self(0u32);
    pub const Decryption: Self = Self(1u32);
    pub const Signing: Self = Self(2u32);
    pub const KeyAgreement: Self = Self(4u32);
    pub const All: Self = Self(16777215u32);
}
impl ::core::marker::Copy for EnrollKeyUsages {}
impl ::core::clone::Clone for EnrollKeyUsages {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
pub struct ExportOption(pub i32);
impl ExportOption {
    pub const NotExportable: Self = Self(0i32);
    pub const Exportable: Self = Self(1i32);
}
impl ::core::marker::Copy for ExportOption {}
impl ::core::clone::Clone for ExportOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ICertificate {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub BuildChainAsync: unsafe extern "system" fn(this: *mut *mut Self, certificates: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BuildChainAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub BuildChainWithParametersAsync: unsafe extern "system" fn(this: *mut *mut Self, certificates: *mut ::core::ffi::c_void, parameters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BuildChainWithParametersAsync: usize,
    pub SerialNumber: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    pub GetHashValue: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    pub GetHashValueWithAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, hashalgorithmname: ::windows_sys::core::HSTRING, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetCertificateBlob: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetCertificateBlob: usize,
    pub Subject: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Issuer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HasPrivateKey: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsStronglyProtected: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ValidFrom: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ValidFrom: usize,
    #[cfg(feature = "Foundation")]
    pub ValidTo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ValidTo: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub EnhancedKeyUsages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EnhancedKeyUsages: usize,
    pub SetFriendlyName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICertificate2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSecurityDeviceBound: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub KeyUsages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub KeyAlgorithmName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SignatureAlgorithmName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SignatureHashAlgorithmName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SubjectAlternativeName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICertificate3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsPerUser: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub StoreName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub KeyStorageProviderName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICertificateChain {
    pub base__: ::windows_sys::core::IInspectable,
    pub Validate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ChainValidationResult) -> ::windows_sys::core::HRESULT,
    pub ValidateWithParameters: unsafe extern "system" fn(this: *mut *mut Self, parameter: *mut ::core::ffi::c_void, result__: *mut ChainValidationResult) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCertificates: unsafe extern "system" fn(this: *mut *mut Self, includeroot: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCertificates: usize,
}
#[repr(C)]
pub struct ICertificateEnrollmentManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateRequestAsync: unsafe extern "system" fn(this: *mut *mut Self, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateRequestAsync: usize,
    #[cfg(feature = "Foundation")]
    pub InstallCertificateAsync: unsafe extern "system" fn(this: *mut *mut Self, certificate: ::windows_sys::core::HSTRING, installoption: InstallOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InstallCertificateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ImportPfxDataAsync: unsafe extern "system" fn(this: *mut *mut Self, pfxdata: ::windows_sys::core::HSTRING, password: ::windows_sys::core::HSTRING, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImportPfxDataAsync: usize,
}
#[repr(C)]
pub struct ICertificateEnrollmentManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub UserCertificateEnrollmentManager: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ImportPfxDataToKspAsync: unsafe extern "system" fn(this: *mut *mut Self, pfxdata: ::windows_sys::core::HSTRING, password: ::windows_sys::core::HSTRING, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: ::windows_sys::core::HSTRING, keystorageprovider: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImportPfxDataToKspAsync: usize,
}
#[repr(C)]
pub struct ICertificateEnrollmentManagerStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ImportPfxDataToKspWithParametersAsync: unsafe extern "system" fn(this: *mut *mut Self, pfxdata: ::windows_sys::core::HSTRING, password: ::windows_sys::core::HSTRING, pfximportparameters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImportPfxDataToKspWithParametersAsync: usize,
}
#[repr(C)]
pub struct ICertificateExtension {
    pub base__: ::windows_sys::core::IInspectable,
    pub ObjectId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetObjectId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsCritical: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsCritical: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub EncodeValue: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICertificateFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub CreateCertificate: unsafe extern "system" fn(this: *mut *mut Self, certblob: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateCertificate: usize,
}
#[repr(C)]
pub struct ICertificateKeyUsages {
    pub base__: ::windows_sys::core::IInspectable,
    pub EncipherOnly: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetEncipherOnly: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CrlSign: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCrlSign: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub KeyCertificateSign: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetKeyCertificateSign: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub KeyAgreement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetKeyAgreement: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub DataEncipherment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetDataEncipherment: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub KeyEncipherment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetKeyEncipherment: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub NonRepudiation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetNonRepudiation: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub DigitalSignature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetDigitalSignature: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICertificateQuery {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub EnhancedKeyUsages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EnhancedKeyUsages: usize,
    pub IssuerName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetIssuerName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetFriendlyName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Thumbprint: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetThumbprint: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const u8) -> ::windows_sys::core::HRESULT,
    pub HardwareOnly: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHardwareOnly: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICertificateQuery2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IncludeDuplicates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIncludeDuplicates: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IncludeExpiredCertificates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIncludeExpiredCertificates: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub StoreName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetStoreName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICertificateRequestProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub Subject: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub KeyAlgorithmName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetKeyAlgorithmName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub KeySize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetKeySize: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetFriendlyName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HashAlgorithmName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetHashAlgorithmName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Exportable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ExportOption) -> ::windows_sys::core::HRESULT,
    pub SetExportable: unsafe extern "system" fn(this: *mut *mut Self, value: ExportOption) -> ::windows_sys::core::HRESULT,
    pub KeyUsages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EnrollKeyUsages) -> ::windows_sys::core::HRESULT,
    pub SetKeyUsages: unsafe extern "system" fn(this: *mut *mut Self, value: EnrollKeyUsages) -> ::windows_sys::core::HRESULT,
    pub KeyProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut KeyProtectionLevel) -> ::windows_sys::core::HRESULT,
    pub SetKeyProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, value: KeyProtectionLevel) -> ::windows_sys::core::HRESULT,
    pub KeyStorageProviderName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetKeyStorageProviderName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICertificateRequestProperties2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SmartcardReaderName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSmartcardReaderName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SigningCertificate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSigningCertificate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AttestationCredentialCertificate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAttestationCredentialCertificate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICertificateRequestProperties3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CurveName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCurveName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CurveParameters: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetCurveParameters: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const u8) -> ::windows_sys::core::HRESULT,
    pub ContainerNamePrefix: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetContainerNamePrefix: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ContainerName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetContainerName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub UseExistingKey: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetUseExistingKey: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICertificateRequestProperties4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub SuppressedDefaults: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SuppressedDefaults: usize,
    pub SubjectAlternativeName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Extensions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Extensions: usize,
}
#[repr(C)]
pub struct ICertificateStore {
    pub base__: ::windows_sys::core::IInspectable,
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, certificate: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self, certificate: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICertificateStore2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICertificateStoresStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllWithQueryAsync: unsafe extern "system" fn(this: *mut *mut Self, query: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllWithQueryAsync: usize,
    pub TrustedRootCertificationAuthorities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IntermediateCertificationAuthorities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStoreByName: unsafe extern "system" fn(this: *mut *mut Self, storename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICertificateStoresStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetUserStoreByName: unsafe extern "system" fn(this: *mut *mut Self, storename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IChainBuildingParameters {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub EnhancedKeyUsages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EnhancedKeyUsages: usize,
    #[cfg(feature = "Foundation")]
    pub ValidationTimestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ValidationTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub SetValidationTimestamp: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValidationTimestamp: usize,
    pub RevocationCheckEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetRevocationCheckEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub NetworkRetrievalEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetNetworkRetrievalEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AuthorityInformationAccessEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAuthorityInformationAccessEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CurrentTimeValidationEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCurrentTimeValidationEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ExclusiveTrustRoots: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExclusiveTrustRoots: usize,
}
#[repr(C)]
pub struct IChainValidationParameters {
    pub base__: ::windows_sys::core::IInspectable,
    pub CertificateChainPolicy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CertificateChainPolicy) -> ::windows_sys::core::HRESULT,
    pub SetCertificateChainPolicy: unsafe extern "system" fn(this: *mut *mut Self, value: CertificateChainPolicy) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Networking")]
    pub ServerDnsName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    ServerDnsName: usize,
    #[cfg(feature = "Networking")]
    pub SetServerDnsName: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    SetServerDnsName: usize,
}
#[repr(C)]
pub struct ICmsAttachedSignature {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Certificates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Certificates: usize,
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Signers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Signers: usize,
    pub VerifySignature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SignatureValidationResult) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICmsAttachedSignatureFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub CreateCmsAttachedSignature: unsafe extern "system" fn(this: *mut *mut Self, inputblob: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateCmsAttachedSignature: usize,
}
#[repr(C)]
pub struct ICmsAttachedSignatureStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub GenerateSignatureAsync: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void, signers: *mut ::core::ffi::c_void, certificates: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    GenerateSignatureAsync: usize,
}
#[repr(C)]
pub struct ICmsDetachedSignature {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Certificates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Certificates: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Signers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Signers: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub VerifySignatureAsync: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    VerifySignatureAsync: usize,
}
#[repr(C)]
pub struct ICmsDetachedSignatureFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub CreateCmsDetachedSignature: unsafe extern "system" fn(this: *mut *mut Self, inputblob: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateCmsDetachedSignature: usize,
}
#[repr(C)]
pub struct ICmsDetachedSignatureStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub GenerateSignatureAsync: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void, signers: *mut ::core::ffi::c_void, certificates: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    GenerateSignatureAsync: usize,
}
#[repr(C)]
pub struct ICmsSignerInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub Certificate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCertificate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HashAlgorithmName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetHashAlgorithmName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TimestampInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICmsTimestampInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub SigningCertificate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Certificates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Certificates: usize,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
}
#[repr(C)]
pub struct IKeyAlgorithmNamesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Rsa: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Dsa: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Ecdh256: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Ecdh384: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Ecdh521: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Ecdsa256: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Ecdsa384: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Ecdsa521: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKeyAlgorithmNamesStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Ecdsa: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Ecdh: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKeyAttestationHelperStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub DecryptTpmAttestationCredentialAsync: unsafe extern "system" fn(this: *mut *mut Self, credential: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DecryptTpmAttestationCredentialAsync: usize,
    pub GetTpmAttestationCredentialId: unsafe extern "system" fn(this: *mut *mut Self, credential: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKeyAttestationHelperStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub DecryptTpmAttestationCredentialWithContainerNameAsync: unsafe extern "system" fn(this: *mut *mut Self, credential: ::windows_sys::core::HSTRING, containername: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DecryptTpmAttestationCredentialWithContainerNameAsync: usize,
}
#[repr(C)]
pub struct IKeyStorageProviderNamesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub SoftwareKeyStorageProvider: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SmartcardKeyStorageProvider: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PlatformKeyStorageProvider: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKeyStorageProviderNamesStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PassportKeyStorageProvider: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPfxImportParameters {
    pub base__: ::windows_sys::core::IInspectable,
    pub Exportable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ExportOption) -> ::windows_sys::core::HRESULT,
    pub SetExportable: unsafe extern "system" fn(this: *mut *mut Self, value: ExportOption) -> ::windows_sys::core::HRESULT,
    pub KeyProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut KeyProtectionLevel) -> ::windows_sys::core::HRESULT,
    pub SetKeyProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, value: KeyProtectionLevel) -> ::windows_sys::core::HRESULT,
    pub InstallOptions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InstallOptions) -> ::windows_sys::core::HRESULT,
    pub SetInstallOptions: unsafe extern "system" fn(this: *mut *mut Self, value: InstallOptions) -> ::windows_sys::core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetFriendlyName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub KeyStorageProviderName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetKeyStorageProviderName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ContainerNamePrefix: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetContainerNamePrefix: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ReaderName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetReaderName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStandardCertificateStoreNamesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Personal: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TrustedRootCertificationAuthorities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IntermediateCertificationAuthorities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISubjectAlternativeNameInfo {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub EmailName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EmailName: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub IPAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    IPAddress: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Url: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Url: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DnsName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DnsName: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DistinguishedName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DistinguishedName: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PrincipalName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PrincipalName: usize,
}
#[repr(C)]
pub struct ISubjectAlternativeNameInfo2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub EmailNames: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EmailNames: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub IPAddresses: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    IPAddresses: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Urls: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Urls: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DnsNames: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DnsNames: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DistinguishedNames: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DistinguishedNames: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PrincipalNames: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PrincipalNames: usize,
    pub Extension: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUserCertificateEnrollmentManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateRequestAsync: unsafe extern "system" fn(this: *mut *mut Self, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateRequestAsync: usize,
    #[cfg(feature = "Foundation")]
    pub InstallCertificateAsync: unsafe extern "system" fn(this: *mut *mut Self, certificate: ::windows_sys::core::HSTRING, installoption: InstallOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InstallCertificateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ImportPfxDataAsync: unsafe extern "system" fn(this: *mut *mut Self, pfxdata: ::windows_sys::core::HSTRING, password: ::windows_sys::core::HSTRING, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImportPfxDataAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ImportPfxDataToKspAsync: unsafe extern "system" fn(this: *mut *mut Self, pfxdata: ::windows_sys::core::HSTRING, password: ::windows_sys::core::HSTRING, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: ::windows_sys::core::HSTRING, keystorageprovider: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImportPfxDataToKspAsync: usize,
}
#[repr(C)]
pub struct IUserCertificateEnrollmentManager2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ImportPfxDataToKspWithParametersAsync: unsafe extern "system" fn(this: *mut *mut Self, pfxdata: ::windows_sys::core::HSTRING, password: ::windows_sys::core::HSTRING, pfximportparameters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImportPfxDataToKspWithParametersAsync: usize,
}
#[repr(C)]
pub struct IUserCertificateStore {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestAddAsync: unsafe extern "system" fn(this: *mut *mut Self, certificate: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAddAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestDeleteAsync: unsafe extern "system" fn(this: *mut *mut Self, certificate: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestDeleteAsync: usize,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
pub struct InstallOptions(pub u32);
impl InstallOptions {
    pub const None: Self = Self(0u32);
    pub const DeleteExpired: Self = Self(1u32);
}
impl ::core::marker::Copy for InstallOptions {}
impl ::core::clone::Clone for InstallOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
pub struct KeyProtectionLevel(pub i32);
impl KeyProtectionLevel {
    pub const NoConsent: Self = Self(0i32);
    pub const ConsentOnly: Self = Self(1i32);
    pub const ConsentWithPassword: Self = Self(2i32);
    pub const ConsentWithFingerprint: Self = Self(3i32);
}
impl ::core::marker::Copy for KeyProtectionLevel {}
impl ::core::clone::Clone for KeyProtectionLevel {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
pub struct KeySize(pub i32);
impl KeySize {
    pub const Invalid: Self = Self(0i32);
    pub const Rsa2048: Self = Self(2048i32);
    pub const Rsa4096: Self = Self(4096i32);
}
impl ::core::marker::Copy for KeySize {}
impl ::core::clone::Clone for KeySize {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PfxImportParameters = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
pub struct SignatureValidationResult(pub i32);
impl SignatureValidationResult {
    pub const Success: Self = Self(0i32);
    pub const InvalidParameter: Self = Self(1i32);
    pub const BadMessage: Self = Self(2i32);
    pub const InvalidSignature: Self = Self(3i32);
    pub const OtherErrors: Self = Self(4i32);
}
impl ::core::marker::Copy for SignatureValidationResult {}
impl ::core::clone::Clone for SignatureValidationResult {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SubjectAlternativeNameInfo = *mut ::core::ffi::c_void;
pub type UserCertificateEnrollmentManager = *mut ::core::ffi::c_void;
pub type UserCertificateStore = *mut ::core::ffi::c_void;
