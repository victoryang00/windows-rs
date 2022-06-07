pub type CompositorController = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct ICompositorController {
    pub base__: ::windows_sys::core::IInspectable,
    pub Compositor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Commit: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnsurePreviousCommitCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnsurePreviousCommitCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CommitNeeded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CommitNeeded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCommitNeeded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCommitNeeded: usize,
}
impl ::windows_sys::core::Interface for ICompositorController {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 762704730, data2: 28839, data3: 17301, data4: [186, 45, 206, 240, 177, 131, 153, 249] };
}
