pub type AsymmetricKeyAlgorithmProvider = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Cryptography_Core\"`*"]
#[repr(transparent)]
pub struct Capi1KdfTargetAlgorithm(pub i32);
impl Capi1KdfTargetAlgorithm {
    pub const NotAes: Self = Self(0i32);
    pub const Aes: Self = Self(1i32);
}
impl ::core::marker::Copy for Capi1KdfTargetAlgorithm {}
impl ::core::clone::Clone for Capi1KdfTargetAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CryptographicHash = *mut ::core::ffi::c_void;
pub type CryptographicKey = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Cryptography_Core\"`*"]
#[repr(transparent)]
pub struct CryptographicPadding(pub i32);
impl CryptographicPadding {
    pub const None: Self = Self(0i32);
    pub const RsaOaep: Self = Self(1i32);
    pub const RsaPkcs1V15: Self = Self(2i32);
    pub const RsaPss: Self = Self(3i32);
}
impl ::core::marker::Copy for CryptographicPadding {}
impl ::core::clone::Clone for CryptographicPadding {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_Cryptography_Core\"`*"]
#[repr(transparent)]
pub struct CryptographicPrivateKeyBlobType(pub i32);
impl CryptographicPrivateKeyBlobType {
    pub const Pkcs8RawPrivateKeyInfo: Self = Self(0i32);
    pub const Pkcs1RsaPrivateKey: Self = Self(1i32);
    pub const BCryptPrivateKey: Self = Self(2i32);
    pub const Capi1PrivateKey: Self = Self(3i32);
    pub const BCryptEccFullPrivateKey: Self = Self(4i32);
}
impl ::core::marker::Copy for CryptographicPrivateKeyBlobType {}
impl ::core::clone::Clone for CryptographicPrivateKeyBlobType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_Cryptography_Core\"`*"]
#[repr(transparent)]
pub struct CryptographicPublicKeyBlobType(pub i32);
impl CryptographicPublicKeyBlobType {
    pub const X509SubjectPublicKeyInfo: Self = Self(0i32);
    pub const Pkcs1RsaPublicKey: Self = Self(1i32);
    pub const BCryptPublicKey: Self = Self(2i32);
    pub const Capi1PublicKey: Self = Self(3i32);
    pub const BCryptEccFullPublicKey: Self = Self(4i32);
}
impl ::core::marker::Copy for CryptographicPublicKeyBlobType {}
impl ::core::clone::Clone for CryptographicPublicKeyBlobType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EncryptedAndAuthenticatedData = *mut ::core::ffi::c_void;
pub type HashAlgorithmProvider = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAsymmetricAlgorithmNamesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub RsaPkcs1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RsaOaepSha1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RsaOaepSha256: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RsaOaepSha384: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RsaOaepSha512: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EcdsaP256Sha256: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EcdsaP384Sha384: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EcdsaP521Sha512: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DsaSha1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DsaSha256: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RsaSignPkcs1Sha1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RsaSignPkcs1Sha256: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RsaSignPkcs1Sha384: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RsaSignPkcs1Sha512: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RsaSignPssSha1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RsaSignPssSha256: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RsaSignPssSha384: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RsaSignPssSha512: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAsymmetricAlgorithmNamesStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub EcdsaSha256: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EcdsaSha384: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EcdsaSha512: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAsymmetricKeyAlgorithmProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub AlgorithmName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CreateKeyPair: unsafe extern "system" fn(this: *mut *mut Self, keysize: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ImportDefaultPrivateKeyBlob: unsafe extern "system" fn(this: *mut *mut Self, keyblob: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ImportDefaultPrivateKeyBlob: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ImportKeyPairWithBlobType: unsafe extern "system" fn(this: *mut *mut Self, keyblob: *mut ::core::ffi::c_void, blobtype: CryptographicPrivateKeyBlobType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ImportKeyPairWithBlobType: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ImportDefaultPublicKeyBlob: unsafe extern "system" fn(this: *mut *mut Self, keyblob: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ImportDefaultPublicKeyBlob: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ImportPublicKeyWithBlobType: unsafe extern "system" fn(this: *mut *mut Self, keyblob: *mut ::core::ffi::c_void, blobtype: CryptographicPublicKeyBlobType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ImportPublicKeyWithBlobType: usize,
}
#[repr(C)]
pub struct IAsymmetricKeyAlgorithmProvider2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateKeyPairWithCurveName: unsafe extern "system" fn(this: *mut *mut Self, curvename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateKeyPairWithCurveParameters: unsafe extern "system" fn(this: *mut *mut Self, parameters_array_size: u32, parameters: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAsymmetricKeyAlgorithmProviderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub OpenAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, algorithm: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICryptographicEngineStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Encrypt: unsafe extern "system" fn(this: *mut *mut Self, key: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, iv: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Encrypt: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Decrypt: unsafe extern "system" fn(this: *mut *mut Self, key: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, iv: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Decrypt: usize,
    #[cfg(feature = "Storage_Streams")]
    pub EncryptAndAuthenticate: unsafe extern "system" fn(this: *mut *mut Self, key: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, nonce: *mut ::core::ffi::c_void, authenticateddata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    EncryptAndAuthenticate: usize,
    #[cfg(feature = "Storage_Streams")]
    pub DecryptAndAuthenticate: unsafe extern "system" fn(this: *mut *mut Self, key: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, nonce: *mut ::core::ffi::c_void, authenticationtag: *mut ::core::ffi::c_void, authenticateddata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DecryptAndAuthenticate: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Sign: unsafe extern "system" fn(this: *mut *mut Self, key: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Sign: usize,
    #[cfg(feature = "Storage_Streams")]
    pub VerifySignature: unsafe extern "system" fn(this: *mut *mut Self, key: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, signature: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    VerifySignature: usize,
    #[cfg(feature = "Storage_Streams")]
    pub DeriveKeyMaterial: unsafe extern "system" fn(this: *mut *mut Self, key: *mut ::core::ffi::c_void, parameters: *mut ::core::ffi::c_void, desiredkeysize: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DeriveKeyMaterial: usize,
}
#[repr(C)]
pub struct ICryptographicEngineStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub SignHashedData: unsafe extern "system" fn(this: *mut *mut Self, key: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SignHashedData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub VerifySignatureWithHashInput: unsafe extern "system" fn(this: *mut *mut Self, key: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, signature: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    VerifySignatureWithHashInput: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub DecryptAsync: unsafe extern "system" fn(this: *mut *mut Self, key: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, iv: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    DecryptAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SignAsync: unsafe extern "system" fn(this: *mut *mut Self, key: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SignAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SignHashedDataAsync: unsafe extern "system" fn(this: *mut *mut Self, key: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SignHashedDataAsync: usize,
}
#[repr(C)]
pub struct ICryptographicKey {
    pub base__: ::windows_sys::core::IInspectable,
    pub KeySize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ExportDefaultPrivateKeyBlobType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ExportDefaultPrivateKeyBlobType: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ExportPrivateKeyWithBlobType: unsafe extern "system" fn(this: *mut *mut Self, blobtype: CryptographicPrivateKeyBlobType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ExportPrivateKeyWithBlobType: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ExportDefaultPublicKeyBlobType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ExportDefaultPublicKeyBlobType: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ExportPublicKeyWithBlobType: unsafe extern "system" fn(this: *mut *mut Self, blobtype: CryptographicPublicKeyBlobType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ExportPublicKeyWithBlobType: usize,
}
#[repr(C)]
pub struct IEccCurveNamesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub BrainpoolP160r1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub BrainpoolP160t1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub BrainpoolP192r1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub BrainpoolP192t1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub BrainpoolP224r1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub BrainpoolP224t1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub BrainpoolP256r1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub BrainpoolP256t1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub BrainpoolP320r1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub BrainpoolP320t1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub BrainpoolP384r1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub BrainpoolP384t1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub BrainpoolP512r1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub BrainpoolP512t1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Curve25519: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Ec192wapi: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NistP192: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NistP224: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NistP256: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NistP384: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NistP521: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NumsP256t1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NumsP384t1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NumsP512t1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SecP160k1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SecP160r1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SecP160r2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SecP192k1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SecP192r1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SecP224k1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SecP224r1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SecP256k1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SecP256r1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SecP384r1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SecP521r1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Wtls7: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Wtls9: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Wtls12: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub X962P192v1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub X962P192v2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub X962P192v3: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub X962P239v1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub X962P239v2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub X962P239v3: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub X962P256v1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AllEccCurveNames: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllEccCurveNames: usize,
}
#[repr(C)]
pub struct IEncryptedAndAuthenticatedData {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub EncryptedData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    EncryptedData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub AuthenticationTag: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AuthenticationTag: usize,
}
#[repr(C)]
pub struct IHashAlgorithmNamesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Md5: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Sha1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Sha256: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Sha384: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Sha512: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHashAlgorithmProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub AlgorithmName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HashLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub HashData: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    HashData: usize,
    pub CreateHash: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHashAlgorithmProviderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub OpenAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, algorithm: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHashComputation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Append: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Append: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GetValueAndReset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetValueAndReset: usize,
}
#[repr(C)]
pub struct IKeyDerivationAlgorithmNamesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Pbkdf2Md5: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Pbkdf2Sha1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Pbkdf2Sha256: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Pbkdf2Sha384: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Pbkdf2Sha512: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Sp800108CtrHmacMd5: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Sp800108CtrHmacSha1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Sp800108CtrHmacSha256: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Sp800108CtrHmacSha384: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Sp800108CtrHmacSha512: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Sp80056aConcatMd5: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Sp80056aConcatSha1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Sp80056aConcatSha256: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Sp80056aConcatSha384: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Sp80056aConcatSha512: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKeyDerivationAlgorithmNamesStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CapiKdfMd5: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CapiKdfSha1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CapiKdfSha256: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CapiKdfSha384: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CapiKdfSha512: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKeyDerivationAlgorithmProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub AlgorithmName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateKey: unsafe extern "system" fn(this: *mut *mut Self, keymaterial: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateKey: usize,
}
#[repr(C)]
pub struct IKeyDerivationAlgorithmProviderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub OpenAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, algorithm: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKeyDerivationParameters {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub KdfGenericBinary: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    KdfGenericBinary: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetKdfGenericBinary: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetKdfGenericBinary: usize,
    pub IterationCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKeyDerivationParameters2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Capi1KdfTargetAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Capi1KdfTargetAlgorithm) -> ::windows_sys::core::HRESULT,
    pub SetCapi1KdfTargetAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, value: Capi1KdfTargetAlgorithm) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IKeyDerivationParametersStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub BuildForPbkdf2: unsafe extern "system" fn(this: *mut *mut Self, pbkdf2salt: *mut ::core::ffi::c_void, iterationcount: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    BuildForPbkdf2: usize,
    #[cfg(feature = "Storage_Streams")]
    pub BuildForSP800108: unsafe extern "system" fn(this: *mut *mut Self, label: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    BuildForSP800108: usize,
    #[cfg(feature = "Storage_Streams")]
    pub BuildForSP80056a: unsafe extern "system" fn(this: *mut *mut Self, algorithmid: *mut ::core::ffi::c_void, partyuinfo: *mut ::core::ffi::c_void, partyvinfo: *mut ::core::ffi::c_void, supppubinfo: *mut ::core::ffi::c_void, suppprivinfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    BuildForSP80056a: usize,
}
#[repr(C)]
pub struct IKeyDerivationParametersStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BuildForCapi1Kdf: unsafe extern "system" fn(this: *mut *mut Self, capi1kdftargetalgorithm: Capi1KdfTargetAlgorithm, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMacAlgorithmNamesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub HmacMd5: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HmacSha1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HmacSha256: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HmacSha384: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HmacSha512: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AesCmac: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMacAlgorithmProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub AlgorithmName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MacLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateKey: unsafe extern "system" fn(this: *mut *mut Self, keymaterial: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateKey: usize,
}
#[repr(C)]
pub struct IMacAlgorithmProvider2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub CreateHash: unsafe extern "system" fn(this: *mut *mut Self, keymaterial: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateHash: usize,
}
#[repr(C)]
pub struct IMacAlgorithmProviderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub OpenAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, algorithm: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPersistedKeyProviderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Certificates"))]
    pub OpenKeyPairFromCertificateAsync: unsafe extern "system" fn(this: *mut *mut Self, certificate: *mut ::core::ffi::c_void, hashalgorithmname: ::windows_sys::core::HSTRING, padding: CryptographicPadding, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Cryptography_Certificates")))]
    OpenKeyPairFromCertificateAsync: usize,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub OpenPublicKeyFromCertificate: unsafe extern "system" fn(this: *mut *mut Self, certificate: *mut ::core::ffi::c_void, hashalgorithmname: ::windows_sys::core::HSTRING, padding: CryptographicPadding, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    OpenPublicKeyFromCertificate: usize,
}
#[repr(C)]
pub struct ISymmetricAlgorithmNamesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub DesCbc: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DesEcb: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TripleDesCbc: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TripleDesEcb: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Rc2Cbc: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Rc2Ecb: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AesCbc: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AesEcb: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AesGcm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AesCcm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AesCbcPkcs7: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AesEcbPkcs7: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DesCbcPkcs7: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DesEcbPkcs7: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TripleDesCbcPkcs7: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TripleDesEcbPkcs7: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Rc2CbcPkcs7: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Rc2EcbPkcs7: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Rc4: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISymmetricKeyAlgorithmProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub AlgorithmName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub BlockLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateSymmetricKey: unsafe extern "system" fn(this: *mut *mut Self, keymaterial: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateSymmetricKey: usize,
}
#[repr(C)]
pub struct ISymmetricKeyAlgorithmProviderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub OpenAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, algorithm: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
pub type KeyDerivationAlgorithmProvider = *mut ::core::ffi::c_void;
pub type KeyDerivationParameters = *mut ::core::ffi::c_void;
pub type MacAlgorithmProvider = *mut ::core::ffi::c_void;
pub type SymmetricKeyAlgorithmProvider = *mut ::core::ffi::c_void;
