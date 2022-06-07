#[repr(C)]
pub struct IXsltProcessor {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Data_Xml_Dom")]
    pub TransformToString: unsafe extern "system" fn(this: *mut *mut Self, inputnode: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    TransformToString: usize,
}
impl ::windows_sys::core::Interface for IXsltProcessor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2070179903, data2: 21772, data3: 18630, data4: [169, 15, 147, 165, 185, 100, 81, 143] };
}
#[repr(C)]
pub struct IXsltProcessor2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Data_Xml_Dom")]
    pub TransformToDocument: unsafe extern "system" fn(this: *mut *mut Self, inputnode: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    TransformToDocument: usize,
}
impl ::windows_sys::core::Interface for IXsltProcessor2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2376358998, data2: 38821, data3: 17611, data4: [168, 190, 39, 216, 98, 128, 199, 10] };
}
#[repr(C)]
pub struct IXsltProcessorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Data_Xml_Dom")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, document: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    CreateInstance: usize,
}
impl ::windows_sys::core::Interface for IXsltProcessorFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 658589376, data2: 39505, data3: 18019, data4: [191, 48, 14, 247, 66, 20, 111, 32] };
}
pub type XsltProcessor = *mut ::core::ffi::c_void;
