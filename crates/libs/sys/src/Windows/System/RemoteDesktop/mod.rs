#[cfg(feature = "System_RemoteDesktop_Input")]
pub mod Input;
#[repr(C)]
pub struct IInteractiveSessionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsRemote: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
