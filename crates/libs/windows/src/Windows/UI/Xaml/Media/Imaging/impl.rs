#[cfg(feature = "ApplicationModel_Background")]
pub trait IXamlRenderingBackgroundTaskOverrides_Impl: Sized {
    fn OnRun(&self, taskinstance: &::core::option::Option<super::super::super::super::ApplicationModel::Background::IBackgroundTaskInstance>) -> ::windows_core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl ::windows_core::RuntimeName for IXamlRenderingBackgroundTaskOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IXamlRenderingBackgroundTaskOverrides";
}
#[cfg(feature = "ApplicationModel_Background")]
impl IXamlRenderingBackgroundTaskOverrides_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXamlRenderingBackgroundTaskOverrides_Impl, const OFFSET: isize>() -> IXamlRenderingBackgroundTaskOverrides_Vtbl {
        unsafe extern "system" fn OnRun<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXamlRenderingBackgroundTaskOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskinstance: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnRun(::core::mem::transmute(&taskinstance)).into()
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, IXamlRenderingBackgroundTaskOverrides, OFFSET>(),
            OnRun: OnRun::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXamlRenderingBackgroundTaskOverrides as ::windows_core::Interface>::IID
    }
}
