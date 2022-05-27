pub trait ITextElementOverrides_Impl: Sized {
    fn OnDisconnectVisualChildren(&self) -> ::windows_core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
impl ::windows_core::RuntimeName for ITextElementOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.ITextElementOverrides";
}
impl ITextElementOverrides_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextElementOverrides_Impl, const OFFSET: isize>() -> ITextElementOverrides_Vtbl {
        unsafe extern "system" fn OnDisconnectVisualChildren<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextElementOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDisconnectVisualChildren().into()
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, ITextElementOverrides, OFFSET>(),
            OnDisconnectVisualChildren: OnDisconnectVisualChildren::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextElementOverrides as ::windows_core::Interface>::IID
    }
}
