#[cfg(feature = "Extensions")]
pub mod Extensions;
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppAttributeError(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIppAttributeError {
    type Vtable = IIppAttributeError_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x750feda1_9eef_5c39_93e4_46149bbcef27);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppAttributeError_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IppAttributeErrorReason) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUnsupportedValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUnsupportedValues: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppAttributeValue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIppAttributeValue {
    type Vtable = IIppAttributeValue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x99407fed_e2bb_59a3_988b_28a974052a26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppAttributeValue_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IppAttributeValueKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetIntegerArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetIntegerArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetBooleanArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetBooleanArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetEnumArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetEnumArray: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub GetOctetStringArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    GetOctetStringArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetDateTimeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetDateTimeArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetResolutionArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetResolutionArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRangeOfIntegerArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRangeOfIntegerArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCollectionArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCollectionArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTextWithLanguageArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTextWithLanguageArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNameWithLanguageArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNameWithLanguageArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTextWithoutLanguageArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTextWithoutLanguageArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNameWithoutLanguageArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNameWithoutLanguageArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetKeywordArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetKeywordArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUriArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUriArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUriSchemaArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUriSchemaArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCharsetArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCharsetArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNaturalLanguageArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNaturalLanguageArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMimeMediaTypeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMimeMediaTypeArray: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppAttributeValueStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIppAttributeValueStatics {
    type Vtable = IIppAttributeValueStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x10d43942_dd94_5998_b235_afafb6fa7935);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppAttributeValueStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateUnsupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateUnknown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateNoValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateInteger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateIntegerArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateIntegerArray: usize,
    pub CreateBoolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateBooleanArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateBooleanArray: usize,
    pub CreateEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateEnumArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateEnumArray: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateOctetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateOctetString: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub CreateOctetStringArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    CreateOctetStringArray: usize,
    #[cfg(feature = "Foundation")]
    pub CreateDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateDateTime: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateDateTimeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateDateTimeArray: usize,
    pub CreateResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateResolutionArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateResolutionArray: usize,
    pub CreateRangeOfInteger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateRangeOfIntegerArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateRangeOfIntegerArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memberattributes: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateCollection: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateCollectionArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memberattributesarray: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateCollectionArray: usize,
    pub CreateTextWithLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateTextWithLanguageArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateTextWithLanguageArray: usize,
    pub CreateNameWithLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateNameWithLanguageArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateNameWithLanguageArray: usize,
    pub CreateTextWithoutLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateTextWithoutLanguageArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateTextWithoutLanguageArray: usize,
    pub CreateNameWithoutLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateNameWithoutLanguageArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateNameWithoutLanguageArray: usize,
    pub CreateKeyword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateKeywordArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateKeywordArray: usize,
    #[cfg(feature = "Foundation")]
    pub CreateUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateUri: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateUriArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateUriArray: usize,
    pub CreateUriSchema: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateUriSchemaArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateUriSchemaArray: usize,
    pub CreateCharset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateCharsetArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateCharsetArray: usize,
    pub CreateNaturalLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateNaturalLanguageArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateNaturalLanguageArray: usize,
    pub CreateMimeMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateMimeMediaArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateMimeMediaArray: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppIntegerRange(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIppIntegerRange {
    type Vtable = IIppIntegerRange_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x92907346_c3ea_5ed6_bdb1_3752c62c6f7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppIntegerRange_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub End: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppIntegerRangeFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIppIntegerRangeFactory {
    type Vtable = IIppIntegerRangeFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75d4ecae_f87e_54ad_b5d0_465204db7553);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppIntegerRangeFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, start: i32, end: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppPrintDevice(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIppPrintDevice {
    type Vtable = IIppPrintDevice_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd748ac56_76f3_5dc6_afd4_c2a8686b9359);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppPrintDevice_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PrinterName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PrinterUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrinterUri: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub GetPrinterAttributesAsBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributenames: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    GetPrinterAttributesAsBuffer: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPrinterAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributenames: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPrinterAttributes: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetPrinterAttributesFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printerattributesbuffer: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetPrinterAttributesFromBuffer: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetPrinterAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printerattributes: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetPrinterAttributes: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppResolution(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIppResolution {
    type Vtable = IIppResolution_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcb493f86_6bf3_56f5_86ce_263d08aead63);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppResolution_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Unit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IppResolutionUnit) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppResolutionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIppResolutionFactory {
    type Vtable = IIppResolutionFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe481c2ae_251a_5326_b173_95543ed99a35);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppResolutionFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: i32, height: i32, unit: IppResolutionUnit, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppSetAttributesResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIppSetAttributesResult {
    type Vtable = IIppSetAttributesResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d1c7f55_aa9d_58a3_90e9_17bdc5281f07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppSetAttributesResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AttributeErrors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AttributeErrors: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppTextWithLanguage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIppTextWithLanguage {
    type Vtable = IIppTextWithLanguage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x326447a6_5149_5936_90e8_0c736036bf77);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppTextWithLanguage_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppTextWithLanguageFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIppTextWithLanguageFactory {
    type Vtable = IIppTextWithLanguageFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xca4a1e8d_2968_5775_997c_8a46f1a574ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppTextWithLanguageFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, text: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrint3DDevice(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrint3DDevice {
    type Vtable = IPrint3DDevice_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x041c3d19_9713_42a2_9813_7dc3337428d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DDevice_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PrintSchema: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrint3DDeviceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrint3DDeviceStatics {
    type Vtable = IPrint3DDeviceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfde3620a_67cd_41b7_a344_5150a1fd75b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DDeviceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintSchema(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintSchema {
    type Vtable = IPrintSchema_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc2b98316_26b8_4bfb_8138_9f962c22a35b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchema_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetDefaultPrintTicketAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetDefaultPrintTicketAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetCapabilitiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, constrainticket: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetCapabilitiesAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub MergeAndValidateWithDefaultPrintTicketAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deltaticket: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    MergeAndValidateWithDefaultPrintTicketAsync: usize,
}
#[repr(transparent)]
pub struct IppAttributeError(::windows_core::IUnknown);
impl IppAttributeError {
    pub fn Reason(&self) -> ::windows_core::Result<IppAttributeErrorReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<IppAttributeErrorReason>::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IppAttributeErrorReason>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetUnsupportedValues(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<IppAttributeValue>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetUnsupportedValues)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<IppAttributeValue>>(result__)
        }
    }
}
impl ::core::clone::Clone for IppAttributeError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IppAttributeError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IppAttributeError {}
impl ::core::fmt::Debug for IppAttributeError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IppAttributeError").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IppAttributeError {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.IppAttributeError;{750feda1-9eef-5c39-93e4-46149bbcef27})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IppAttributeError {
    type Vtable = IIppAttributeError_Vtbl;
    const IID: ::windows_core::GUID = <IIppAttributeError as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for IppAttributeError {
    const NAME: &'static str = "Windows.Devices.Printers.IppAttributeError";
}
impl ::core::convert::From<IppAttributeError> for ::windows_core::IUnknown {
    fn from(value: IppAttributeError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IppAttributeError> for ::windows_core::IUnknown {
    fn from(value: &IppAttributeError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IppAttributeError {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IppAttributeError {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IppAttributeError> for ::windows_core::IInspectable {
    fn from(value: IppAttributeError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IppAttributeError> for ::windows_core::IInspectable {
    fn from(value: &IppAttributeError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IppAttributeError {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IppAttributeError {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IppAttributeError {}
unsafe impl ::core::marker::Sync for IppAttributeError {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IppAttributeErrorReason(pub i32);
impl IppAttributeErrorReason {
    pub const RequestEntityTooLarge: Self = Self(0i32);
    pub const AttributeNotSupported: Self = Self(1i32);
    pub const AttributeValuesNotSupported: Self = Self(2i32);
    pub const AttributeNotSettable: Self = Self(3i32);
    pub const ConflictingAttributes: Self = Self(4i32);
}
impl ::core::marker::Copy for IppAttributeErrorReason {}
impl ::core::clone::Clone for IppAttributeErrorReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IppAttributeErrorReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for IppAttributeErrorReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for IppAttributeErrorReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IppAttributeErrorReason").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IppAttributeErrorReason {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Printers.IppAttributeErrorReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct IppAttributeValue(::windows_core::IUnknown);
impl IppAttributeValue {
    pub fn Kind(&self) -> ::windows_core::Result<IppAttributeValueKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<IppAttributeValueKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IppAttributeValueKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetIntegerArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetIntegerArray)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetBooleanArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetBooleanArray)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetEnumArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetEnumArray)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<i32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn GetOctetStringArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetOctetStringArray)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetDateTimeArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDateTimeArray)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetResolutionArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<IppResolution>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetResolutionArray)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<IppResolution>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRangeOfIntegerArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<IppIntegerRange>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetRangeOfIntegerArray)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<IppIntegerRange>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCollectionArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, IppAttributeValue>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCollectionArray)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, IppAttributeValue>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTextWithLanguageArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<IppTextWithLanguage>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetTextWithLanguageArray)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<IppTextWithLanguage>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetNameWithLanguageArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<IppTextWithLanguage>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetNameWithLanguageArray)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<IppTextWithLanguage>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTextWithoutLanguageArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetTextWithoutLanguageArray)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetNameWithoutLanguageArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetNameWithoutLanguageArray)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetKeywordArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetKeywordArray)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetUriArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetUriArray)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetUriSchemaArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetUriSchemaArray)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCharsetArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCharsetArray)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetNaturalLanguageArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetNaturalLanguageArray)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMimeMediaTypeArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetMimeMediaTypeArray)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn CreateUnsupported() -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateUnsupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateUnknown() -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateUnknown)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateNoValue() -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateNoValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateInteger(value: i32) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInteger)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateIntegerArray<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i32>>>(values: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateIntegerArray)(::windows_core::Interface::as_raw(this), values.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateBoolean(value: bool) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateBoolean)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateBooleanArray<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<bool>>>(values: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateBooleanArray)(::windows_core::Interface::as_raw(this), values.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateEnum(value: i32) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateEnum)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateEnumArray<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i32>>>(values: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateEnumArray)(::windows_core::Interface::as_raw(this), values.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateOctetString<'a, Param0: ::windows_core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(value: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateOctetString)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn CreateOctetStringArray<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Storage::Streams::IBuffer>>>(values: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateOctetStringArray)(::windows_core::Interface::as_raw(this), values.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateDateTime<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::DateTime>>(value: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateDateTime)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateDateTimeArray<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::DateTime>>>(values: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateDateTimeArray)(::windows_core::Interface::as_raw(this), values.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateResolution<'a, Param0: ::windows_core::IntoParam<'a, IppResolution>>(value: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateResolution)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateResolutionArray<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<IppResolution>>>(values: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateResolutionArray)(::windows_core::Interface::as_raw(this), values.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateRangeOfInteger<'a, Param0: ::windows_core::IntoParam<'a, IppIntegerRange>>(value: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateRangeOfInteger)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateRangeOfIntegerArray<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<IppIntegerRange>>>(values: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateRangeOfIntegerArray)(::windows_core::Interface::as_raw(this), values.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateCollection<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, IppAttributeValue>>>>(memberattributes: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateCollection)(::windows_core::Interface::as_raw(this), memberattributes.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateCollectionArray<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, IppAttributeValue>>>>>(memberattributesarray: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateCollectionArray)(::windows_core::Interface::as_raw(this), memberattributesarray.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateTextWithLanguage<'a, Param0: ::windows_core::IntoParam<'a, IppTextWithLanguage>>(value: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateTextWithLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateTextWithLanguageArray<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<IppTextWithLanguage>>>(values: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateTextWithLanguageArray)(::windows_core::Interface::as_raw(this), values.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateNameWithLanguage<'a, Param0: ::windows_core::IntoParam<'a, IppTextWithLanguage>>(value: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateNameWithLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateNameWithLanguageArray<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<IppTextWithLanguage>>>(values: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateNameWithLanguageArray)(::windows_core::Interface::as_raw(this), values.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateTextWithoutLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(value: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateTextWithoutLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateTextWithoutLanguageArray<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>>(values: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateTextWithoutLanguageArray)(::windows_core::Interface::as_raw(this), values.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateNameWithoutLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(value: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateNameWithoutLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateNameWithoutLanguageArray<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>>(values: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateNameWithoutLanguageArray)(::windows_core::Interface::as_raw(this), values.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateKeyword<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(value: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateKeyword)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateKeywordArray<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>>(values: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateKeywordArray)(::windows_core::Interface::as_raw(this), values.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateUri<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Uri>>(value: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateUri)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateUriArray<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>>(values: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateUriArray)(::windows_core::Interface::as_raw(this), values.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateUriSchema<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(value: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateUriSchema)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateUriSchemaArray<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>>(values: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateUriSchemaArray)(::windows_core::Interface::as_raw(this), values.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateCharset<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(value: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateCharset)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateCharsetArray<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>>(values: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateCharsetArray)(::windows_core::Interface::as_raw(this), values.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateNaturalLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(value: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateNaturalLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateNaturalLanguageArray<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>>(values: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateNaturalLanguageArray)(::windows_core::Interface::as_raw(this), values.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateMimeMedia<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(value: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateMimeMedia)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateMimeMediaArray<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>>(values: Param0) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateMimeMediaArray)(::windows_core::Interface::as_raw(this), values.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn IIppAttributeValueStatics<R, F: FnOnce(&IIppAttributeValueStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<IppAttributeValue, IIppAttributeValueStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for IppAttributeValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IppAttributeValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IppAttributeValue {}
impl ::core::fmt::Debug for IppAttributeValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IppAttributeValue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IppAttributeValue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.IppAttributeValue;{99407fed-e2bb-59a3-988b-28a974052a26})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IppAttributeValue {
    type Vtable = IIppAttributeValue_Vtbl;
    const IID: ::windows_core::GUID = <IIppAttributeValue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for IppAttributeValue {
    const NAME: &'static str = "Windows.Devices.Printers.IppAttributeValue";
}
impl ::core::convert::From<IppAttributeValue> for ::windows_core::IUnknown {
    fn from(value: IppAttributeValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IppAttributeValue> for ::windows_core::IUnknown {
    fn from(value: &IppAttributeValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IppAttributeValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IppAttributeValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IppAttributeValue> for ::windows_core::IInspectable {
    fn from(value: IppAttributeValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IppAttributeValue> for ::windows_core::IInspectable {
    fn from(value: &IppAttributeValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IppAttributeValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IppAttributeValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IppAttributeValue {}
unsafe impl ::core::marker::Sync for IppAttributeValue {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IppAttributeValueKind(pub i32);
impl IppAttributeValueKind {
    pub const Unsupported: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const NoValue: Self = Self(2i32);
    pub const Integer: Self = Self(3i32);
    pub const Boolean: Self = Self(4i32);
    pub const Enum: Self = Self(5i32);
    pub const OctetString: Self = Self(6i32);
    pub const DateTime: Self = Self(7i32);
    pub const Resolution: Self = Self(8i32);
    pub const RangeOfInteger: Self = Self(9i32);
    pub const Collection: Self = Self(10i32);
    pub const TextWithLanguage: Self = Self(11i32);
    pub const NameWithLanguage: Self = Self(12i32);
    pub const TextWithoutLanguage: Self = Self(13i32);
    pub const NameWithoutLanguage: Self = Self(14i32);
    pub const Keyword: Self = Self(15i32);
    pub const Uri: Self = Self(16i32);
    pub const UriSchema: Self = Self(17i32);
    pub const Charset: Self = Self(18i32);
    pub const NaturalLanguage: Self = Self(19i32);
    pub const MimeMediaType: Self = Self(20i32);
}
impl ::core::marker::Copy for IppAttributeValueKind {}
impl ::core::clone::Clone for IppAttributeValueKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IppAttributeValueKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for IppAttributeValueKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for IppAttributeValueKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IppAttributeValueKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IppAttributeValueKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Printers.IppAttributeValueKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct IppIntegerRange(::windows_core::IUnknown);
impl IppIntegerRange {
    pub fn Start(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn End(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).End)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn CreateInstance(start: i32, end: i32) -> ::windows_core::Result<IppIntegerRange> {
        Self::IIppIntegerRangeFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), start, end, result__.as_mut_ptr()).from_abi::<IppIntegerRange>(result__)
        })
    }
    pub fn IIppIntegerRangeFactory<R, F: FnOnce(&IIppIntegerRangeFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<IppIntegerRange, IIppIntegerRangeFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for IppIntegerRange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IppIntegerRange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IppIntegerRange {}
impl ::core::fmt::Debug for IppIntegerRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IppIntegerRange").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IppIntegerRange {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.IppIntegerRange;{92907346-c3ea-5ed6-bdb1-3752c62c6f7f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IppIntegerRange {
    type Vtable = IIppIntegerRange_Vtbl;
    const IID: ::windows_core::GUID = <IIppIntegerRange as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for IppIntegerRange {
    const NAME: &'static str = "Windows.Devices.Printers.IppIntegerRange";
}
impl ::core::convert::From<IppIntegerRange> for ::windows_core::IUnknown {
    fn from(value: IppIntegerRange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IppIntegerRange> for ::windows_core::IUnknown {
    fn from(value: &IppIntegerRange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IppIntegerRange {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IppIntegerRange {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IppIntegerRange> for ::windows_core::IInspectable {
    fn from(value: IppIntegerRange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IppIntegerRange> for ::windows_core::IInspectable {
    fn from(value: &IppIntegerRange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IppIntegerRange {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IppIntegerRange {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IppIntegerRange {}
unsafe impl ::core::marker::Sync for IppIntegerRange {}
#[repr(transparent)]
pub struct IppPrintDevice(::windows_core::IUnknown);
impl IppPrintDevice {
    pub fn PrinterName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PrinterName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PrinterUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PrinterUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn GetPrinterAttributesAsBuffer<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, attributenames: Param0) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPrinterAttributesAsBuffer)(::windows_core::Interface::as_raw(this), attributenames.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPrinterAttributes<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, attributenames: Param0) -> ::windows_core::Result<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, IppAttributeValue>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPrinterAttributes)(::windows_core::Interface::as_raw(this), attributenames.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, IppAttributeValue>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetPrinterAttributesFromBuffer<'a, Param0: ::windows_core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, printerattributesbuffer: Param0) -> ::windows_core::Result<IppSetAttributesResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetPrinterAttributesFromBuffer)(::windows_core::Interface::as_raw(this), printerattributesbuffer.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppSetAttributesResult>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetPrinterAttributes<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, IppAttributeValue>>>>(&self, printerattributes: Param0) -> ::windows_core::Result<IppSetAttributesResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetPrinterAttributes)(::windows_core::Interface::as_raw(this), printerattributes.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppSetAttributesResult>(result__)
        }
    }
}
impl ::core::clone::Clone for IppPrintDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IppPrintDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IppPrintDevice {}
impl ::core::fmt::Debug for IppPrintDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IppPrintDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IppPrintDevice {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.IppPrintDevice;{d748ac56-76f3-5dc6-afd4-c2a8686b9359})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IppPrintDevice {
    type Vtable = IIppPrintDevice_Vtbl;
    const IID: ::windows_core::GUID = <IIppPrintDevice as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for IppPrintDevice {
    const NAME: &'static str = "Windows.Devices.Printers.IppPrintDevice";
}
impl ::core::convert::From<IppPrintDevice> for ::windows_core::IUnknown {
    fn from(value: IppPrintDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IppPrintDevice> for ::windows_core::IUnknown {
    fn from(value: &IppPrintDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IppPrintDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IppPrintDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IppPrintDevice> for ::windows_core::IInspectable {
    fn from(value: IppPrintDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IppPrintDevice> for ::windows_core::IInspectable {
    fn from(value: &IppPrintDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IppPrintDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IppPrintDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IppPrintDevice {}
unsafe impl ::core::marker::Sync for IppPrintDevice {}
#[repr(transparent)]
pub struct IppResolution(::windows_core::IUnknown);
impl IppResolution {
    pub fn Width(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Width)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Height(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Height)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Unit(&self) -> ::windows_core::Result<IppResolutionUnit> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<IppResolutionUnit>::zeroed();
            (::windows_core::Interface::vtable(this).Unit)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IppResolutionUnit>(result__)
        }
    }
    pub fn CreateInstance(width: i32, height: i32, unit: IppResolutionUnit) -> ::windows_core::Result<IppResolution> {
        Self::IIppResolutionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), width, height, unit, result__.as_mut_ptr()).from_abi::<IppResolution>(result__)
        })
    }
    pub fn IIppResolutionFactory<R, F: FnOnce(&IIppResolutionFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<IppResolution, IIppResolutionFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for IppResolution {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IppResolution {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IppResolution {}
impl ::core::fmt::Debug for IppResolution {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IppResolution").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IppResolution {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.IppResolution;{cb493f86-6bf3-56f5-86ce-263d08aead63})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IppResolution {
    type Vtable = IIppResolution_Vtbl;
    const IID: ::windows_core::GUID = <IIppResolution as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for IppResolution {
    const NAME: &'static str = "Windows.Devices.Printers.IppResolution";
}
impl ::core::convert::From<IppResolution> for ::windows_core::IUnknown {
    fn from(value: IppResolution) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IppResolution> for ::windows_core::IUnknown {
    fn from(value: &IppResolution) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IppResolution {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IppResolution {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IppResolution> for ::windows_core::IInspectable {
    fn from(value: IppResolution) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IppResolution> for ::windows_core::IInspectable {
    fn from(value: &IppResolution) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IppResolution {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IppResolution {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IppResolution {}
unsafe impl ::core::marker::Sync for IppResolution {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IppResolutionUnit(pub i32);
impl IppResolutionUnit {
    pub const DotsPerInch: Self = Self(0i32);
    pub const DotsPerCentimeter: Self = Self(1i32);
}
impl ::core::marker::Copy for IppResolutionUnit {}
impl ::core::clone::Clone for IppResolutionUnit {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IppResolutionUnit {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for IppResolutionUnit {
    type Abi = Self;
}
impl ::core::fmt::Debug for IppResolutionUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IppResolutionUnit").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IppResolutionUnit {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Printers.IppResolutionUnit;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct IppSetAttributesResult(::windows_core::IUnknown);
impl IppSetAttributesResult {
    pub fn Succeeded(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Succeeded)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AttributeErrors(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, IppAttributeError>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AttributeErrors)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, IppAttributeError>>(result__)
        }
    }
}
impl ::core::clone::Clone for IppSetAttributesResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IppSetAttributesResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IppSetAttributesResult {}
impl ::core::fmt::Debug for IppSetAttributesResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IppSetAttributesResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IppSetAttributesResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.IppSetAttributesResult;{7d1c7f55-aa9d-58a3-90e9-17bdc5281f07})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IppSetAttributesResult {
    type Vtable = IIppSetAttributesResult_Vtbl;
    const IID: ::windows_core::GUID = <IIppSetAttributesResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for IppSetAttributesResult {
    const NAME: &'static str = "Windows.Devices.Printers.IppSetAttributesResult";
}
impl ::core::convert::From<IppSetAttributesResult> for ::windows_core::IUnknown {
    fn from(value: IppSetAttributesResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IppSetAttributesResult> for ::windows_core::IUnknown {
    fn from(value: &IppSetAttributesResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IppSetAttributesResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IppSetAttributesResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IppSetAttributesResult> for ::windows_core::IInspectable {
    fn from(value: IppSetAttributesResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IppSetAttributesResult> for ::windows_core::IInspectable {
    fn from(value: &IppSetAttributesResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IppSetAttributesResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IppSetAttributesResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IppSetAttributesResult {}
unsafe impl ::core::marker::Sync for IppSetAttributesResult {}
#[repr(transparent)]
pub struct IppTextWithLanguage(::windows_core::IUnknown);
impl IppTextWithLanguage {
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CreateInstance<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(language: Param0, text: Param1) -> ::windows_core::Result<IppTextWithLanguage> {
        Self::IIppTextWithLanguageFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), language.into_param().abi(), text.into_param().abi(), result__.as_mut_ptr()).from_abi::<IppTextWithLanguage>(result__)
        })
    }
    pub fn IIppTextWithLanguageFactory<R, F: FnOnce(&IIppTextWithLanguageFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<IppTextWithLanguage, IIppTextWithLanguageFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for IppTextWithLanguage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IppTextWithLanguage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IppTextWithLanguage {}
impl ::core::fmt::Debug for IppTextWithLanguage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IppTextWithLanguage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IppTextWithLanguage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.IppTextWithLanguage;{326447a6-5149-5936-90e8-0c736036bf77})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IppTextWithLanguage {
    type Vtable = IIppTextWithLanguage_Vtbl;
    const IID: ::windows_core::GUID = <IIppTextWithLanguage as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for IppTextWithLanguage {
    const NAME: &'static str = "Windows.Devices.Printers.IppTextWithLanguage";
}
impl ::core::convert::From<IppTextWithLanguage> for ::windows_core::IUnknown {
    fn from(value: IppTextWithLanguage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IppTextWithLanguage> for ::windows_core::IUnknown {
    fn from(value: &IppTextWithLanguage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IppTextWithLanguage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IppTextWithLanguage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IppTextWithLanguage> for ::windows_core::IInspectable {
    fn from(value: IppTextWithLanguage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IppTextWithLanguage> for ::windows_core::IInspectable {
    fn from(value: &IppTextWithLanguage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IppTextWithLanguage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IppTextWithLanguage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IppTextWithLanguage {}
unsafe impl ::core::marker::Sync for IppTextWithLanguage {}
#[repr(transparent)]
pub struct Print3DDevice(::windows_core::IUnknown);
impl Print3DDevice {
    pub fn PrintSchema(&self) -> ::windows_core::Result<PrintSchema> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PrintSchema)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintSchema>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Print3DDevice>> {
        Self::IPrint3DDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<Print3DDevice>>(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IPrint3DDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IPrint3DDeviceStatics<R, F: FnOnce(&IPrint3DDeviceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Print3DDevice, IPrint3DDeviceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Print3DDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Print3DDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DDevice {}
impl ::core::fmt::Debug for Print3DDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Print3DDevice {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.Print3DDevice;{041c3d19-9713-42a2-9813-7dc3337428d3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Print3DDevice {
    type Vtable = IPrint3DDevice_Vtbl;
    const IID: ::windows_core::GUID = <IPrint3DDevice as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Print3DDevice {
    const NAME: &'static str = "Windows.Devices.Printers.Print3DDevice";
}
impl ::core::convert::From<Print3DDevice> for ::windows_core::IUnknown {
    fn from(value: Print3DDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Print3DDevice> for ::windows_core::IUnknown {
    fn from(value: &Print3DDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Print3DDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Print3DDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Print3DDevice> for ::windows_core::IInspectable {
    fn from(value: Print3DDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Print3DDevice> for ::windows_core::IInspectable {
    fn from(value: &Print3DDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Print3DDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Print3DDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Print3DDevice {}
unsafe impl ::core::marker::Sync for Print3DDevice {}
#[repr(transparent)]
pub struct PrintSchema(::windows_core::IUnknown);
impl PrintSchema {
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetDefaultPrintTicketAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultPrintTicketAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetCapabilitiesAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType>>(&self, constrainticket: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCapabilitiesAsync)(::windows_core::Interface::as_raw(this), constrainticket.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn MergeAndValidateWithDefaultPrintTicketAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType>>(&self, deltaticket: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MergeAndValidateWithDefaultPrintTicketAsync)(::windows_core::Interface::as_raw(this), deltaticket.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintSchema {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintSchema {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintSchema {}
impl ::core::fmt::Debug for PrintSchema {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintSchema").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintSchema {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.PrintSchema;{c2b98316-26b8-4bfb-8138-9f962c22a35b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintSchema {
    type Vtable = IPrintSchema_Vtbl;
    const IID: ::windows_core::GUID = <IPrintSchema as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintSchema {
    const NAME: &'static str = "Windows.Devices.Printers.PrintSchema";
}
impl ::core::convert::From<PrintSchema> for ::windows_core::IUnknown {
    fn from(value: PrintSchema) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintSchema> for ::windows_core::IUnknown {
    fn from(value: &PrintSchema) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintSchema {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintSchema {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintSchema> for ::windows_core::IInspectable {
    fn from(value: PrintSchema) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintSchema> for ::windows_core::IInspectable {
    fn from(value: &PrintSchema) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintSchema {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintSchema {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintSchema {}
unsafe impl ::core::marker::Sync for PrintSchema {}
