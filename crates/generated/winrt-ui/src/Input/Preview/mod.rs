#[cfg(feature = "Injection")]
pub mod Injection;
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputActivationListenerPreviewStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputActivationListenerPreviewStatics {
    type Vtable = IInputActivationListenerPreviewStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf0551ce5_0de6_5be0_a589_f737201a4582);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputActivationListenerPreviewStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-ui")]
    pub CreateForApplicationWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    CreateForApplicationWindow: usize,
}
pub struct InputActivationListenerPreview;
impl InputActivationListenerPreview {
    #[cfg(feature = "winrt-ui")]
    pub fn CreateForApplicationWindow<'a, Param0: ::windows_core::IntoParam<'a, super::super::WindowManagement::AppWindow>>(window: Param0) -> ::windows_core::Result<super::InputActivationListener> {
        Self::IInputActivationListenerPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateForApplicationWindow)(::windows_core::Interface::as_raw(this), window.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::InputActivationListener>(result__)
        })
    }
    pub fn IInputActivationListenerPreviewStatics<R, F: FnOnce(&IInputActivationListenerPreviewStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<InputActivationListenerPreview, IInputActivationListenerPreviewStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for InputActivationListenerPreview {
    const NAME: &'static str = "Windows.UI.Input.Preview.InputActivationListenerPreview";
}
