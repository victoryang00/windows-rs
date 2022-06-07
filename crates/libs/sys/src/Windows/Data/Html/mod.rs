#[repr(C)]
pub struct IHtmlUtilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub ConvertToText: unsafe extern "system" fn(this: *mut *mut Self, html: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHtmlUtilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4273998557, data2: 9113, data3: 20396, data4: [181, 167, 5, 233, 172, 215, 24, 29] };
}
