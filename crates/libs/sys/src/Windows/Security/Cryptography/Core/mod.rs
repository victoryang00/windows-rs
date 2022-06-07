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
impl ::windows_sys::core::Interface for IAsymmetricAlgorithmNamesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3405184228, data2: 26560, data3: 18090, data4: [132, 249, 117, 46, 119, 68, 159, 155] };
}
#[repr(C)]
pub struct IAsymmetricAlgorithmNamesStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub EcdsaSha256: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EcdsaSha384: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EcdsaSha512: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAsymmetricAlgorithmNamesStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4047618262, data2: 19455, data3: 20259, data4: [186, 102, 96, 69, 177, 55, 213, 223] };
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
impl ::windows_sys::core::Interface for IAsymmetricKeyAlgorithmProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3906142007, data2: 25177, data3: 20104, data4: [183, 224, 148, 25, 31, 222, 105, 158] };
}
#[repr(C)]
pub struct IAsymmetricKeyAlgorithmProvider2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateKeyPairWithCurveName: unsafe extern "system" fn(this: *mut *mut Self, curvename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateKeyPairWithCurveParameters: unsafe extern "system" fn(this: *mut *mut Self, parameters_array_size: u32, parameters: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAsymmetricKeyAlgorithmProvider2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1311910526, data2: 31821, data3: 18839, data4: [172, 79, 27, 132, 139, 54, 48, 110] };
}
#[repr(C)]
pub struct IAsymmetricKeyAlgorithmProviderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub OpenAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, algorithm: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAsymmetricKeyAlgorithmProviderStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1113316888, data2: 42995, data3: 18342, data4: [168, 210, 196, 141, 96, 51, 166, 92] };
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
impl ::windows_sys::core::Interface for ICryptographicEngineStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2682914361, data2: 28663, data3: 19589, data4: [160, 149, 149, 235, 49, 113, 94, 185] };
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
impl ::windows_sys::core::Interface for ICryptographicEngineStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1733904638, data2: 57247, data3: 16785, data4: [146, 199, 108, 230, 245, 132, 32, 224] };
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
impl ::windows_sys::core::Interface for ICryptographicKey {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3978967920, data2: 36475, data3: 16393, data4: [132, 1, 255, 209, 166, 46, 235, 39] };
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
impl ::windows_sys::core::Interface for IEccCurveNamesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3019870988, data2: 44779, data3: 16542, data4: [183, 212, 155, 149, 41, 90, 174, 207] };
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
impl ::windows_sys::core::Interface for IEncryptedAndAuthenticatedData {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1873031143, data2: 7883, data3: 19200, data4: [190, 165, 96, 184, 63, 134, 47, 23] };
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
impl ::windows_sys::core::Interface for IHashAlgorithmNamesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1801323798, data2: 56982, data3: 20234, data4: [141, 87, 220, 201, 218, 227, 108, 118] };
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
impl ::windows_sys::core::Interface for IHashAlgorithmProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3197841536, data2: 45763, data3: 16939, data4: [188, 225, 236, 144, 239, 181, 215, 181] };
}
#[repr(C)]
pub struct IHashAlgorithmProviderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub OpenAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, algorithm: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHashAlgorithmProviderStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2678888257, data2: 23748, data3: 17206, data4: [174, 56, 98, 18, 183, 90, 145, 90] };
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
impl ::windows_sys::core::Interface for IHashComputation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1493488054, data2: 44337, data3: 17923, data4: [163, 164, 177, 189, 169, 142, 37, 98] };
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
impl ::windows_sys::core::Interface for IKeyDerivationAlgorithmNamesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2070820414, data2: 38098, data3: 18233, data4: [165, 123, 2, 46, 12, 58, 64, 42] };
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
impl ::windows_sys::core::Interface for IKeyDerivationAlgorithmNamesStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1469398955, data2: 24644, data3: 18031, data4: [151, 244, 51, 123, 120, 8, 56, 77] };
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
impl ::windows_sys::core::Interface for IKeyDerivationAlgorithmProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3791366203, data2: 18033, data3: 17335, data4: [145, 88, 118, 58, 170, 152, 182, 191] };
}
#[repr(C)]
pub struct IKeyDerivationAlgorithmProviderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub OpenAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, algorithm: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IKeyDerivationAlgorithmProviderStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 170002810, data2: 2588, data3: 17467, data4: [148, 24, 185, 73, 138, 235, 22, 3] };
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
impl ::windows_sys::core::Interface for IKeyDerivationParameters {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2079349095, data2: 1147, data3: 19084, data4: [150, 74, 70, 159, 253, 85, 34, 226] };
}
#[repr(C)]
pub struct IKeyDerivationParameters2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Capi1KdfTargetAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Capi1KdfTargetAlgorithm) -> ::windows_sys::core::HRESULT,
    pub SetCapi1KdfTargetAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, value: Capi1KdfTargetAlgorithm) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IKeyDerivationParameters2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3443615441, data2: 16766, data3: 20300, data4: [182, 102, 192, 216, 121, 243, 248, 224] };
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
impl ::windows_sys::core::Interface for IKeyDerivationParametersStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3935707070, data2: 62335, data3: 16710, data4: [157, 254, 164, 86, 241, 115, 95, 75] };
}
#[repr(C)]
pub struct IKeyDerivationParametersStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BuildForCapi1Kdf: unsafe extern "system" fn(this: *mut *mut Self, capi1kdftargetalgorithm: Capi1KdfTargetAlgorithm, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IKeyDerivationParametersStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2776120789, data2: 22755, data3: 20219, data4: [178, 131, 161, 101, 49, 38, 225, 190] };
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
impl ::windows_sys::core::Interface for IMacAlgorithmNamesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1094788728, data2: 64286, data3: 17316, data4: [137, 94, 169, 2, 110, 67, 144, 163] };
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
impl ::windows_sys::core::Interface for IMacAlgorithmProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1245693379, data2: 7357, data3: 16846, data4: [160, 146, 170, 11, 197, 210, 210, 245] };
}
#[repr(C)]
pub struct IMacAlgorithmProvider2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub CreateHash: unsafe extern "system" fn(this: *mut *mut Self, keymaterial: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateHash: usize,
}
impl ::windows_sys::core::Interface for IMacAlgorithmProvider2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1839409685, data2: 55601, data3: 17133, data4: [142, 126, 195, 1, 202, 238, 17, 156] };
}
#[repr(C)]
pub struct IMacAlgorithmProviderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub OpenAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, algorithm: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMacAlgorithmProviderStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3384656199, data2: 52343, data3: 19952, data4: [158, 78, 185, 33, 224, 128, 100, 76] };
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
impl ::windows_sys::core::Interface for IPersistedKeyProviderStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1999063060, data2: 55764, data3: 19701, data4: [182, 104, 224, 69, 125, 243, 8, 148] };
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
impl ::windows_sys::core::Interface for ISymmetricAlgorithmNamesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1752199803, data2: 51606, data3: 20142, data4: [132, 215, 121, 178, 174, 183, 59, 156] };
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
impl ::windows_sys::core::Interface for ISymmetricKeyAlgorithmProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1031686707, data2: 15312, data3: 18690, data4: [138, 200, 71, 13, 80, 210, 19, 118] };
}
#[repr(C)]
pub struct ISymmetricKeyAlgorithmProviderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub OpenAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, algorithm: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISymmetricKeyAlgorithmProviderStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2369463078, data2: 7991, data3: 18719, data4: [182, 14, 245, 67, 27, 38, 180, 131] };
}
pub type KeyDerivationAlgorithmProvider = *mut ::core::ffi::c_void;
pub type KeyDerivationParameters = *mut ::core::ffi::c_void;
pub type MacAlgorithmProvider = *mut ::core::ffi::c_void;
pub type SymmetricKeyAlgorithmProvider = *mut ::core::ffi::c_void;
