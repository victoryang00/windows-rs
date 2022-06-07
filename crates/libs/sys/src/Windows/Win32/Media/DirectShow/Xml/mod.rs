pub const CLSID_XMLGraphBuilder: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 464542049, data2: 24511, data3: 4562, data4: [165, 33, 68, 223, 7, 193, 0, 0] };
#[repr(C)]
pub struct IXMLGraphBuilder {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub BuildFromXML: unsafe extern "system" fn(this: *mut *mut Self, pgraph: *mut ::core::ffi::c_void, pxml: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com")))]
    BuildFromXML: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SaveToXML: unsafe extern "system" fn(this: *mut *mut Self, pgraph: *mut ::core::ffi::c_void, pbstrxml: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SaveToXML: usize,
    pub BuildFromXMLFile: unsafe extern "system" fn(this: *mut *mut Self, pgraph: *mut ::core::ffi::c_void, wszfilename: ::windows_sys::core::PCWSTR, wszbaseurl: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IXMLGraphBuilder {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 464542048, data2: 24511, data3: 4562, data4: [165, 33, 68, 223, 7, 193, 0, 0] };
}
