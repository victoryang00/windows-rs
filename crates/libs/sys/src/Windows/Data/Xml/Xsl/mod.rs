#[repr(C)]
pub struct IXsltProcessor {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Data_Xml_Dom")]
    pub TransformToString: unsafe extern "system" fn(this: *mut *mut Self, inputnode: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    TransformToString: usize,
}
#[repr(C)]
pub struct IXsltProcessor2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Data_Xml_Dom")]
    pub TransformToDocument: unsafe extern "system" fn(this: *mut *mut Self, inputnode: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    TransformToDocument: usize,
}
#[repr(C)]
pub struct IXsltProcessorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Data_Xml_Dom")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, document: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    CreateInstance: usize,
}
pub type XsltProcessor = *mut ::core::ffi::c_void;
