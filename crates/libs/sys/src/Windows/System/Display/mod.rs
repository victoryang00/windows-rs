pub type DisplayRequest = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IDisplayRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequestActive: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RequestRelease: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDisplayRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3849527364, data2: 62623, data3: 19296, data4: [141, 212, 94, 126, 58, 99, 42, 192] };
}
