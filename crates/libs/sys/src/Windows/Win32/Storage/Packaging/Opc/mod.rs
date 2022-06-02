#[repr(C)]
pub struct IOpcCertificateEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MovePrevious: unsafe extern "system" fn(this: *mut *mut Self, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MovePrevious: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub GetCurrent: unsafe extern "system" fn(this: *mut *mut Self, certificate: *const *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))]
    GetCurrent: usize,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, copy: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOpcCertificateSet {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))]
    Add: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))]
    Remove: usize,
    pub GetEnumerator: unsafe extern "system" fn(this: *mut *mut Self, certificateenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOpcDigitalSignature {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetNamespaces: unsafe extern "system" fn(this: *mut *mut Self, prefixes: *mut *mut ::windows_sys::core::PWSTR, namespaces: *mut *mut ::windows_sys::core::PWSTR, count: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetSignatureId: unsafe extern "system" fn(this: *mut *mut Self, signatureid: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSignaturePartName: unsafe extern "system" fn(this: *mut *mut Self, signaturepartname: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSignaturePartName: usize,
    pub GetSignatureMethod: unsafe extern "system" fn(this: *mut *mut Self, signaturemethod: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetCanonicalizationMethod: unsafe extern "system" fn(this: *mut *mut Self, canonicalizationmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows_sys::core::HRESULT,
    pub GetSignatureValue: unsafe extern "system" fn(this: *mut *mut Self, signaturevalue: *mut *mut u8, count: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetSignaturePartReferenceEnumerator: unsafe extern "system" fn(this: *mut *mut Self, partreferenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSignatureRelationshipReferenceEnumerator: unsafe extern "system" fn(this: *mut *mut Self, relationshipreferenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSigningTime: unsafe extern "system" fn(this: *mut *mut Self, signingtime: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetTimeFormat: unsafe extern "system" fn(this: *mut *mut Self, timeformat: *mut OPC_SIGNATURE_TIME_FORMAT) -> ::windows_sys::core::HRESULT,
    pub GetPackageObjectReference: unsafe extern "system" fn(this: *mut *mut Self, packageobjectreference: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetCertificateEnumerator: unsafe extern "system" fn(this: *mut *mut Self, certificateenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetCustomReferenceEnumerator: unsafe extern "system" fn(this: *mut *mut Self, customreferenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetCustomObjectEnumerator: unsafe extern "system" fn(this: *mut *mut Self, customobjectenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSignatureXml: unsafe extern "system" fn(this: *mut *mut Self, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOpcDigitalSignatureEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MovePrevious: unsafe extern "system" fn(this: *mut *mut Self, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MovePrevious: usize,
    pub GetCurrent: unsafe extern "system" fn(this: *mut *mut Self, digitalsignature: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, copy: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOpcDigitalSignatureManager {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSignatureOriginPartName: unsafe extern "system" fn(this: *mut *mut Self, signatureoriginpartname: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSignatureOriginPartName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSignatureOriginPartName: unsafe extern "system" fn(this: *mut *mut Self, signatureoriginpartname: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSignatureOriginPartName: usize,
    pub GetSignatureEnumerator: unsafe extern "system" fn(this: *mut *mut Self, signatureenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RemoveSignature: unsafe extern "system" fn(this: *mut *mut Self, signaturepartname: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RemoveSignature: usize,
    pub CreateSigningOptions: unsafe extern "system" fn(this: *mut *mut Self, signingoptions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub Validate: unsafe extern "system" fn(this: *mut *mut Self, signature: *mut ::core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, validationresult: *mut OPC_SIGNATURE_VALIDATION_RESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))]
    Validate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub Sign: unsafe extern "system" fn(this: *mut *mut Self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, signingoptions: *mut ::core::ffi::c_void, digitalsignature: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))]
    Sign: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ReplaceSignatureXml: unsafe extern "system" fn(this: *mut *mut Self, signaturepartname: *mut ::core::ffi::c_void, newsignaturexml: *const u8, count: u32, digitalsignature: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReplaceSignatureXml: usize,
}
#[repr(C)]
pub struct IOpcFactory {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePackageRootUri: unsafe extern "system" fn(this: *mut *mut Self, rooturi: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePackageRootUri: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePartUri: unsafe extern "system" fn(this: *mut *mut Self, pwzuri: ::windows_sys::core::PCWSTR, parturi: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePartUri: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com"))]
    pub CreateStreamOnFile: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR, iomode: OPC_STREAM_IO_MODE, securityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, dwflagsandattributes: u32, stream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com")))]
    CreateStreamOnFile: usize,
    pub CreatePackage: unsafe extern "system" fn(this: *mut *mut Self, package: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ReadPackageFromStream: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, flags: OPC_READ_FLAGS, package: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReadPackageFromStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub WritePackageToStream: unsafe extern "system" fn(this: *mut *mut Self, package: *mut ::core::ffi::c_void, flags: OPC_WRITE_FLAGS, stream: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WritePackageToStream: usize,
    pub CreateDigitalSignatureManager: unsafe extern "system" fn(this: *mut *mut Self, package: *mut ::core::ffi::c_void, signaturemanager: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOpcPackage {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetPartSet: unsafe extern "system" fn(this: *mut *mut Self, partset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetRelationshipSet: unsafe extern "system" fn(this: *mut *mut Self, relationshipset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOpcPart {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetRelationshipSet: unsafe extern "system" fn(this: *mut *mut Self, relationshipset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetContentStream: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetContentStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, name: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetName: usize,
    pub GetContentType: unsafe extern "system" fn(this: *mut *mut Self, contenttype: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetCompressionOptions: unsafe extern "system" fn(this: *mut *mut Self, compressionoptions: *mut OPC_COMPRESSION_OPTIONS) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOpcPartEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MovePrevious: unsafe extern "system" fn(this: *mut *mut Self, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MovePrevious: usize,
    pub GetCurrent: unsafe extern "system" fn(this: *mut *mut Self, part: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, copy: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOpcPartSet {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPart: unsafe extern "system" fn(this: *mut *mut Self, name: *mut ::core::ffi::c_void, part: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPart: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePart: unsafe extern "system" fn(this: *mut *mut Self, name: *mut ::core::ffi::c_void, contenttype: ::windows_sys::core::PCWSTR, compressionoptions: OPC_COMPRESSION_OPTIONS, part: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePart: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DeletePart: unsafe extern "system" fn(this: *mut *mut Self, name: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeletePart: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub PartExists: unsafe extern "system" fn(this: *mut *mut Self, name: *mut ::core::ffi::c_void, partexists: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    PartExists: usize,
    pub GetEnumerator: unsafe extern "system" fn(this: *mut *mut Self, partenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IOpcPartUri {
    pub base__: IOpcUri,
    #[cfg(feature = "Win32_System_Com")]
    pub ComparePartUri: unsafe extern "system" fn(this: *mut *mut Self, parturi: *mut ::core::ffi::c_void, comparisonresult: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ComparePartUri: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSourceUri: unsafe extern "system" fn(this: *mut *mut Self, sourceuri: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSourceUri: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRelationshipsPartUri: unsafe extern "system" fn(this: *mut *mut Self, isrelationshipuri: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRelationshipsPartUri: usize,
}
#[repr(C)]
pub struct IOpcRelationship {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetId: unsafe extern "system" fn(this: *mut *mut Self, relationshipidentifier: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetRelationshipType: unsafe extern "system" fn(this: *mut *mut Self, relationshiptype: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSourceUri: unsafe extern "system" fn(this: *mut *mut Self, sourceuri: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSourceUri: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetTargetUri: unsafe extern "system" fn(this: *mut *mut Self, targeturi: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetTargetUri: usize,
    pub GetTargetMode: unsafe extern "system" fn(this: *mut *mut Self, targetmode: *mut OPC_URI_TARGET_MODE) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOpcRelationshipEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MovePrevious: unsafe extern "system" fn(this: *mut *mut Self, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MovePrevious: usize,
    pub GetCurrent: unsafe extern "system" fn(this: *mut *mut Self, relationship: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, copy: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOpcRelationshipSelector {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetSelectorType: unsafe extern "system" fn(this: *mut *mut Self, selector: *mut OPC_RELATIONSHIP_SELECTOR) -> ::windows_sys::core::HRESULT,
    pub GetSelectionCriterion: unsafe extern "system" fn(this: *mut *mut Self, selectioncriterion: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOpcRelationshipSelectorEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MovePrevious: unsafe extern "system" fn(this: *mut *mut Self, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MovePrevious: usize,
    pub GetCurrent: unsafe extern "system" fn(this: *mut *mut Self, relationshipselector: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, copy: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOpcRelationshipSelectorSet {
    pub base__: ::windows_sys::core::IUnknown,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, selector: OPC_RELATIONSHIP_SELECTOR, selectioncriterion: ::windows_sys::core::PCWSTR, relationshipselector: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self, relationshipselector: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetEnumerator: unsafe extern "system" fn(this: *mut *mut Self, relationshipselectorenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOpcRelationshipSet {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetRelationship: unsafe extern "system" fn(this: *mut *mut Self, relationshipidentifier: ::windows_sys::core::PCWSTR, relationship: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateRelationship: unsafe extern "system" fn(this: *mut *mut Self, relationshipidentifier: ::windows_sys::core::PCWSTR, relationshiptype: ::windows_sys::core::PCWSTR, targeturi: *mut ::core::ffi::c_void, targetmode: OPC_URI_TARGET_MODE, relationship: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateRelationship: usize,
    pub DeleteRelationship: unsafe extern "system" fn(this: *mut *mut Self, relationshipidentifier: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RelationshipExists: unsafe extern "system" fn(this: *mut *mut Self, relationshipidentifier: ::windows_sys::core::PCWSTR, relationshipexists: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RelationshipExists: usize,
    pub GetEnumerator: unsafe extern "system" fn(this: *mut *mut Self, relationshipenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetEnumeratorForType: unsafe extern "system" fn(this: *mut *mut Self, relationshiptype: ::windows_sys::core::PCWSTR, relationshipenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRelationshipsContentStream: unsafe extern "system" fn(this: *mut *mut Self, contents: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRelationshipsContentStream: usize,
}
#[repr(C)]
pub struct IOpcSignatureCustomObject {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetXml: unsafe extern "system" fn(this: *mut *mut Self, xmlmarkup: *mut *mut u8, count: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOpcSignatureCustomObjectEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MovePrevious: unsafe extern "system" fn(this: *mut *mut Self, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MovePrevious: usize,
    pub GetCurrent: unsafe extern "system" fn(this: *mut *mut Self, customobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, copy: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOpcSignatureCustomObjectSet {
    pub base__: ::windows_sys::core::IUnknown,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, xmlmarkup: *const u8, count: u32, customobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self, customobject: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetEnumerator: unsafe extern "system" fn(this: *mut *mut Self, customobjectenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOpcSignaturePartReference {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPartName: unsafe extern "system" fn(this: *mut *mut Self, partname: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPartName: usize,
    pub GetContentType: unsafe extern "system" fn(this: *mut *mut Self, contenttype: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetDigestMethod: unsafe extern "system" fn(this: *mut *mut Self, digestmethod: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetDigestValue: unsafe extern "system" fn(this: *mut *mut Self, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetTransformMethod: unsafe extern "system" fn(this: *mut *mut Self, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOpcSignaturePartReferenceEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MovePrevious: unsafe extern "system" fn(this: *mut *mut Self, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MovePrevious: usize,
    pub GetCurrent: unsafe extern "system" fn(this: *mut *mut Self, partreference: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, copy: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOpcSignaturePartReferenceSet {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, parturi: *mut ::core::ffi::c_void, digestmethod: ::windows_sys::core::PCWSTR, transformmethod: OPC_CANONICALIZATION_METHOD, partreference: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Create: usize,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self, partreference: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetEnumerator: unsafe extern "system" fn(this: *mut *mut Self, partreferenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOpcSignatureReference {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetId: unsafe extern "system" fn(this: *mut *mut Self, referenceid: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetUri: unsafe extern "system" fn(this: *mut *mut Self, referenceuri: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetUri: usize,
    pub GetType: unsafe extern "system" fn(this: *mut *mut Self, r#type: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetTransformMethod: unsafe extern "system" fn(this: *mut *mut Self, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows_sys::core::HRESULT,
    pub GetDigestMethod: unsafe extern "system" fn(this: *mut *mut Self, digestmethod: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetDigestValue: unsafe extern "system" fn(this: *mut *mut Self, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOpcSignatureReferenceEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MovePrevious: unsafe extern "system" fn(this: *mut *mut Self, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MovePrevious: usize,
    pub GetCurrent: unsafe extern "system" fn(this: *mut *mut Self, reference: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, copy: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOpcSignatureReferenceSet {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, referenceuri: *mut ::core::ffi::c_void, referenceid: ::windows_sys::core::PCWSTR, r#type: ::windows_sys::core::PCWSTR, digestmethod: ::windows_sys::core::PCWSTR, transformmethod: OPC_CANONICALIZATION_METHOD, reference: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Create: usize,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self, reference: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetEnumerator: unsafe extern "system" fn(this: *mut *mut Self, referenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOpcSignatureRelationshipReference {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSourceUri: unsafe extern "system" fn(this: *mut *mut Self, sourceuri: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSourceUri: usize,
    pub GetDigestMethod: unsafe extern "system" fn(this: *mut *mut Self, digestmethod: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetDigestValue: unsafe extern "system" fn(this: *mut *mut Self, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetTransformMethod: unsafe extern "system" fn(this: *mut *mut Self, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows_sys::core::HRESULT,
    pub GetRelationshipSigningOption: unsafe extern "system" fn(this: *mut *mut Self, relationshipsigningoption: *mut OPC_RELATIONSHIPS_SIGNING_OPTION) -> ::windows_sys::core::HRESULT,
    pub GetRelationshipSelectorEnumerator: unsafe extern "system" fn(this: *mut *mut Self, selectorenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOpcSignatureRelationshipReferenceEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MovePrevious: unsafe extern "system" fn(this: *mut *mut Self, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MovePrevious: usize,
    pub GetCurrent: unsafe extern "system" fn(this: *mut *mut Self, relationshipreference: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, copy: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOpcSignatureRelationshipReferenceSet {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, sourceuri: *mut ::core::ffi::c_void, digestmethod: ::windows_sys::core::PCWSTR, relationshipsigningoption: OPC_RELATIONSHIPS_SIGNING_OPTION, selectorset: *mut ::core::ffi::c_void, transformmethod: OPC_CANONICALIZATION_METHOD, relationshipreference: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Create: usize,
    pub CreateRelationshipSelectorSet: unsafe extern "system" fn(this: *mut *mut Self, selectorset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self, relationshipreference: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetEnumerator: unsafe extern "system" fn(this: *mut *mut Self, relationshipreferenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOpcSigningOptions {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetSignatureId: unsafe extern "system" fn(this: *mut *mut Self, signatureid: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetSignatureId: unsafe extern "system" fn(this: *mut *mut Self, signatureid: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetSignatureMethod: unsafe extern "system" fn(this: *mut *mut Self, signaturemethod: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetSignatureMethod: unsafe extern "system" fn(this: *mut *mut Self, signaturemethod: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetDefaultDigestMethod: unsafe extern "system" fn(this: *mut *mut Self, digestmethod: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetDefaultDigestMethod: unsafe extern "system" fn(this: *mut *mut Self, digestmethod: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetCertificateEmbeddingOption: unsafe extern "system" fn(this: *mut *mut Self, embeddingoption: *mut OPC_CERTIFICATE_EMBEDDING_OPTION) -> ::windows_sys::core::HRESULT,
    pub SetCertificateEmbeddingOption: unsafe extern "system" fn(this: *mut *mut Self, embeddingoption: OPC_CERTIFICATE_EMBEDDING_OPTION) -> ::windows_sys::core::HRESULT,
    pub GetTimeFormat: unsafe extern "system" fn(this: *mut *mut Self, timeformat: *mut OPC_SIGNATURE_TIME_FORMAT) -> ::windows_sys::core::HRESULT,
    pub SetTimeFormat: unsafe extern "system" fn(this: *mut *mut Self, timeformat: OPC_SIGNATURE_TIME_FORMAT) -> ::windows_sys::core::HRESULT,
    pub GetSignaturePartReferenceSet: unsafe extern "system" fn(this: *mut *mut Self, partreferenceset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSignatureRelationshipReferenceSet: unsafe extern "system" fn(this: *mut *mut Self, relationshipreferenceset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetCustomObjectSet: unsafe extern "system" fn(this: *mut *mut Self, customobjectset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetCustomReferenceSet: unsafe extern "system" fn(this: *mut *mut Self, customreferenceset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetCertificateSet: unsafe extern "system" fn(this: *mut *mut Self, certificateset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSignaturePartName: unsafe extern "system" fn(this: *mut *mut Self, signaturepartname: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSignaturePartName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSignaturePartName: unsafe extern "system" fn(this: *mut *mut Self, signaturepartname: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSignaturePartName: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IOpcUri {
    pub base__: super::super::super::System::Com::IUri,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRelationshipsPartUri: unsafe extern "system" fn(this: *mut *mut Self, relationshipparturi: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRelationshipsPartUri: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRelativeUri: unsafe extern "system" fn(this: *mut *mut Self, targetparturi: *mut ::core::ffi::c_void, relativeuri: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRelativeUri: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CombinePartUri: unsafe extern "system" fn(this: *mut *mut Self, relativeuri: *mut ::core::ffi::c_void, combineduri: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CombinePartUri: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub type OPC_CANONICALIZATION_METHOD = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_CANONICALIZATION_NONE: OPC_CANONICALIZATION_METHOD = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_CANONICALIZATION_C14N: OPC_CANONICALIZATION_METHOD = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_CANONICALIZATION_C14N_WITH_COMMENTS: OPC_CANONICALIZATION_METHOD = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub type OPC_CERTIFICATE_EMBEDDING_OPTION = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_CERTIFICATE_IN_CERTIFICATE_PART: OPC_CERTIFICATE_EMBEDDING_OPTION = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_CERTIFICATE_IN_SIGNATURE_PART: OPC_CERTIFICATE_EMBEDDING_OPTION = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_CERTIFICATE_NOT_EMBEDDED: OPC_CERTIFICATE_EMBEDDING_OPTION = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub type OPC_COMPRESSION_OPTIONS = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_COMPRESSION_NONE: OPC_COMPRESSION_OPTIONS = -1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_COMPRESSION_NORMAL: OPC_COMPRESSION_OPTIONS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_COMPRESSION_MAXIMUM: OPC_COMPRESSION_OPTIONS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_COMPRESSION_FAST: OPC_COMPRESSION_OPTIONS = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_COMPRESSION_SUPERFAST: OPC_COMPRESSION_OPTIONS = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_CONFLICTING_SETTINGS: ::windows_sys::core::HRESULT = -2142175212i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_COULD_NOT_RECOVER: ::windows_sys::core::HRESULT = -2142175154i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_DEFAULT_DIGEST_METHOD_NOT_SET: ::windows_sys::core::HRESULT = -2142175161i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_DIGEST_VALUE_ERROR: ::windows_sys::core::HRESULT = -2142175206i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_DUPLICATE_PACKAGE_OBJECT_REFERENCES: ::windows_sys::core::HRESULT = -2142175187i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_DUPLICATE_SIGNATURE_ORIGIN_RELATIONSHIP: ::windows_sys::core::HRESULT = -2142175205i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_DUPLICATE_SIGNATURE_PROPERTY_ELEMENT: ::windows_sys::core::HRESULT = -2142175192i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_EXTERNAL_SIGNATURE: ::windows_sys::core::HRESULT = -2142175202i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_EXTERNAL_SIGNATURE_REFERENCE: ::windows_sys::core::HRESULT = -2142175185i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_INVALID_CANONICALIZATION_METHOD: ::windows_sys::core::HRESULT = -2142175198i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_INVALID_CERTIFICATE_RELATIONSHIP: ::windows_sys::core::HRESULT = -2142175203i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_INVALID_OPC_SIGNATURE_TIME_FORMAT: ::windows_sys::core::HRESULT = -2142175196i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_INVALID_RELATIONSHIPS_SIGNING_OPTION: ::windows_sys::core::HRESULT = -2142175197i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_INVALID_RELATIONSHIP_TRANSFORM_XML: ::windows_sys::core::HRESULT = -2142175199i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_INVALID_SIGNATURE_COUNT: ::windows_sys::core::HRESULT = -2142175189i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_INVALID_SIGNATURE_ORIGIN_RELATIONSHIP: ::windows_sys::core::HRESULT = -2142175204i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_INVALID_SIGNATURE_XML: ::windows_sys::core::HRESULT = -2142175190i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_MISSING_CANONICALIZATION_TRANSFORM: ::windows_sys::core::HRESULT = -2142175182i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_MISSING_CERTIFICATE_PART: ::windows_sys::core::HRESULT = -2142175146i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_MISSING_PACKAGE_OBJECT_REFERENCE: ::windows_sys::core::HRESULT = -2142175186i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_MISSING_SIGNATURE_ALGORITHM: ::windows_sys::core::HRESULT = -2142175188i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_MISSING_SIGNATURE_ORIGIN_PART: ::windows_sys::core::HRESULT = -2142175201i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_MISSING_SIGNATURE_PART: ::windows_sys::core::HRESULT = -2142175200i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_MISSING_SIGNATURE_PROPERTIES_ELEMENT: ::windows_sys::core::HRESULT = -2142175194i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_MISSING_SIGNATURE_PROPERTY_ELEMENT: ::windows_sys::core::HRESULT = -2142175193i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_MISSING_SIGNATURE_TIME_PROPERTY: ::windows_sys::core::HRESULT = -2142175191i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_MULTIPLE_RELATIONSHIP_TRANSFORMS: ::windows_sys::core::HRESULT = -2142175183i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_PACKAGE_REFERENCE_URI_RESERVED: ::windows_sys::core::HRESULT = -2142175195i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_REFERENCE_MISSING_CONTENT_TYPE: ::windows_sys::core::HRESULT = -2142175184i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_SIGNATURE_CORRUPT: ::windows_sys::core::HRESULT = -2142175207i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_SIGNATURE_METHOD_NOT_SET: ::windows_sys::core::HRESULT = -2142175162i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_SIGNATURE_ORIGIN_EXISTS: ::windows_sys::core::HRESULT = -2142175148i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_SIGNATURE_PROPERTY_MISSING_TARGET: ::windows_sys::core::HRESULT = -2142175163i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_SIGNATURE_REFERENCE_MISSING_URI: ::windows_sys::core::HRESULT = -2142175165i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_UNSIGNED_PACKAGE: ::windows_sys::core::HRESULT = -2142175147i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DUPLICATE_DEFAULT_EXTENSION: ::windows_sys::core::HRESULT = -2142175217i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DUPLICATE_OVERRIDE_PART: ::windows_sys::core::HRESULT = -2142175219i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DUPLICATE_PART: ::windows_sys::core::HRESULT = -2142175221i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DUPLICATE_PIECE: ::windows_sys::core::HRESULT = -2142175211i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DUPLICATE_RELATIONSHIP: ::windows_sys::core::HRESULT = -2142175213i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ENUM_CANNOT_MOVE_NEXT: ::windows_sys::core::HRESULT = -2142175151i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ENUM_CANNOT_MOVE_PREVIOUS: ::windows_sys::core::HRESULT = -2142175150i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ENUM_COLLECTION_CHANGED: ::windows_sys::core::HRESULT = -2142175152i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ENUM_INVALID_POSITION: ::windows_sys::core::HRESULT = -2142175149i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_INVALID_CONTENT_TYPE: ::windows_sys::core::HRESULT = -2142175164i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_INVALID_CONTENT_TYPE_XML: ::windows_sys::core::HRESULT = -2142175226i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_INVALID_DEFAULT_EXTENSION: ::windows_sys::core::HRESULT = -2142175218i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_INVALID_OVERRIDE_PART_NAME: ::windows_sys::core::HRESULT = -2142175220i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_INVALID_PIECE: ::windows_sys::core::HRESULT = -2142175210i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_INVALID_RELATIONSHIP_ID: ::windows_sys::core::HRESULT = -2142175216i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_INVALID_RELATIONSHIP_TARGET: ::windows_sys::core::HRESULT = -2142175214i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_INVALID_RELATIONSHIP_TARGET_MODE: ::windows_sys::core::HRESULT = -2142175155i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_INVALID_RELATIONSHIP_TYPE: ::windows_sys::core::HRESULT = -2142175215i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_INVALID_RELS_XML: ::windows_sys::core::HRESULT = -2142175222i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_INVALID_XML_ENCODING: ::windows_sys::core::HRESULT = -2142175166i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_INCONSISTENT_PRESERVE_ATTRIBUTES: ::windows_sys::core::HRESULT = -2142175157i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_INCONSISTENT_PRESERVE_ELEMENTS: ::windows_sys::core::HRESULT = -2142175156i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_INCONSISTENT_PROCESS_CONTENT: ::windows_sys::core::HRESULT = -2142175158i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_INVALID_ATTRIBUTES_ON_IGNORABLE_ELEMENT: ::windows_sys::core::HRESULT = -2142175168i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_INVALID_ENUM_TYPE: ::windows_sys::core::HRESULT = -2142175172i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_INVALID_PREFIX_LIST: ::windows_sys::core::HRESULT = -2142175177i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_INVALID_QNAME_LIST: ::windows_sys::core::HRESULT = -2142175176i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_INVALID_XMLNS_ATTRIBUTE: ::windows_sys::core::HRESULT = -2142175167i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_MISSING_CHOICE: ::windows_sys::core::HRESULT = -2142175173i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_MISSING_REQUIRES_ATTR: ::windows_sys::core::HRESULT = -2142175179i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_MULTIPLE_FALLBACK_ELEMENTS: ::windows_sys::core::HRESULT = -2142175159i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_NESTED_ALTERNATE_CONTENT: ::windows_sys::core::HRESULT = -2142175175i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_UNEXPECTED_ATTR: ::windows_sys::core::HRESULT = -2142175178i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_UNEXPECTED_CHOICE: ::windows_sys::core::HRESULT = -2142175174i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_UNEXPECTED_ELEMENT: ::windows_sys::core::HRESULT = -2142175181i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_UNEXPECTED_REQUIRES_ATTR: ::windows_sys::core::HRESULT = -2142175180i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_UNKNOWN_NAMESPACE: ::windows_sys::core::HRESULT = -2142175170i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_UNKNOWN_PREFIX: ::windows_sys::core::HRESULT = -2142175169i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MISSING_CONTENT_TYPES: ::windows_sys::core::HRESULT = -2142175225i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MISSING_PIECE: ::windows_sys::core::HRESULT = -2142175209i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_NONCONFORMING_CONTENT_TYPES_XML: ::windows_sys::core::HRESULT = -2142175224i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_NONCONFORMING_RELS_XML: ::windows_sys::core::HRESULT = -2142175223i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_NONCONFORMING_URI: ::windows_sys::core::HRESULT = -2142175231i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_NO_SUCH_PART: ::windows_sys::core::HRESULT = -2142175208i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_NO_SUCH_RELATIONSHIP: ::windows_sys::core::HRESULT = -2142175160i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_NO_SUCH_SETTINGS: ::windows_sys::core::HRESULT = -2142175145i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_PART_CANNOT_BE_DIRECTORY: ::windows_sys::core::HRESULT = -2142175228i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_RELATIONSHIP_URI_REQUIRED: ::windows_sys::core::HRESULT = -2142175229i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_RELATIVE_URI_REQUIRED: ::windows_sys::core::HRESULT = -2142175230i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_UNEXPECTED_CONTENT_TYPE: ::windows_sys::core::HRESULT = -2142175227i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_UNSUPPORTED_PACKAGE: ::windows_sys::core::HRESULT = -2142175153i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_CENTRAL_DIRECTORY_TOO_LARGE: ::windows_sys::core::HRESULT = -2142171127i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_COMMENT_TOO_LARGE: ::windows_sys::core::HRESULT = -2142171124i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_COMPRESSION_FAILED: ::windows_sys::core::HRESULT = -2142171133i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_CORRUPTED_ARCHIVE: ::windows_sys::core::HRESULT = -2142171134i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_DECOMPRESSION_FAILED: ::windows_sys::core::HRESULT = -2142171132i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_DUPLICATE_NAME: ::windows_sys::core::HRESULT = -2142171125i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_EXTRA_FIELDS_TOO_LARGE: ::windows_sys::core::HRESULT = -2142171123i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_FILE_HEADER_TOO_LARGE: ::windows_sys::core::HRESULT = -2142171122i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_INCONSISTENT_DIRECTORY: ::windows_sys::core::HRESULT = -2142171130i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_INCONSISTENT_FILEITEM: ::windows_sys::core::HRESULT = -2142171131i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_INCORRECT_DATA_SIZE: ::windows_sys::core::HRESULT = -2142171135i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_MISSING_DATA_DESCRIPTOR: ::windows_sys::core::HRESULT = -2142171129i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_MISSING_END_OF_CENTRAL_DIRECTORY: ::windows_sys::core::HRESULT = -2142171121i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_NAME_TOO_LARGE: ::windows_sys::core::HRESULT = -2142171126i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_REQUIRES_64_BIT: ::windows_sys::core::HRESULT = -2142171120i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_UNSUPPORTEDARCHIVE: ::windows_sys::core::HRESULT = -2142171128i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub type OPC_READ_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_READ_DEFAULT: OPC_READ_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_VALIDATE_ON_LOAD: OPC_READ_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_CACHE_ON_ACCESS: OPC_READ_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub type OPC_RELATIONSHIPS_SIGNING_OPTION = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_RELATIONSHIP_SIGN_USING_SELECTORS: OPC_RELATIONSHIPS_SIGNING_OPTION = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_RELATIONSHIP_SIGN_PART: OPC_RELATIONSHIPS_SIGNING_OPTION = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub type OPC_RELATIONSHIP_SELECTOR = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_RELATIONSHIP_SELECT_BY_ID: OPC_RELATIONSHIP_SELECTOR = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_RELATIONSHIP_SELECT_BY_TYPE: OPC_RELATIONSHIP_SELECTOR = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub type OPC_SIGNATURE_TIME_FORMAT = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_SIGNATURE_TIME_FORMAT_MILLISECONDS: OPC_SIGNATURE_TIME_FORMAT = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_SIGNATURE_TIME_FORMAT_SECONDS: OPC_SIGNATURE_TIME_FORMAT = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_SIGNATURE_TIME_FORMAT_MINUTES: OPC_SIGNATURE_TIME_FORMAT = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_SIGNATURE_TIME_FORMAT_DAYS: OPC_SIGNATURE_TIME_FORMAT = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_SIGNATURE_TIME_FORMAT_MONTHS: OPC_SIGNATURE_TIME_FORMAT = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_SIGNATURE_TIME_FORMAT_YEARS: OPC_SIGNATURE_TIME_FORMAT = 5i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub type OPC_SIGNATURE_VALIDATION_RESULT = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_SIGNATURE_VALID: OPC_SIGNATURE_VALIDATION_RESULT = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_SIGNATURE_INVALID: OPC_SIGNATURE_VALIDATION_RESULT = -1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub type OPC_STREAM_IO_MODE = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_STREAM_IO_READ: OPC_STREAM_IO_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_STREAM_IO_WRITE: OPC_STREAM_IO_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub type OPC_URI_TARGET_MODE = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_URI_TARGET_MODE_INTERNAL: OPC_URI_TARGET_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_URI_TARGET_MODE_EXTERNAL: OPC_URI_TARGET_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub type OPC_WRITE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_WRITE_DEFAULT: OPC_WRITE_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_WRITE_FORCE_ZIP32: OPC_WRITE_FLAGS = 1u32;
pub const OpcFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1798138784, data2: 40766, data3: 20263, data4: [146, 11, 49, 60, 196, 38, 163, 158] };
