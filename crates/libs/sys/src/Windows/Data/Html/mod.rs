#[repr(C)]
pub struct IHtmlUtilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub ConvertToText: unsafe extern "system" fn(this: *mut *mut Self, html: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
