#[cfg(feature = "System_RemoteDesktop_Input")]
pub mod Input;
#[repr(C)]
pub struct IInteractiveSessionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsRemote: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInteractiveSessionStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1619543601, data2: 56634, data3: 17782, data4: [156, 141, 232, 2, 118, 24, 189, 206] };
}
