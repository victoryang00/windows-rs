pub trait ICustomXamlResourceLoaderOverrides_Impl: Sized {
    fn GetResource(&self, resourceid: &::windows_core::HSTRING, objecttype: &::windows_core::HSTRING, propertyname: &::windows_core::HSTRING, propertytype: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::IInspectable>;
}
impl ::windows_core::RuntimeName for ICustomXamlResourceLoaderOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Resources.ICustomXamlResourceLoaderOverrides";
}
impl ICustomXamlResourceLoaderOverrides_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICustomXamlResourceLoaderOverrides_Impl, const OFFSET: isize>() -> ICustomXamlResourceLoaderOverrides_Vtbl {
        unsafe extern "system" fn GetResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICustomXamlResourceLoaderOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, objecttype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, propertyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, propertytype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetResource(::core::mem::transmute(&resourceid), ::core::mem::transmute(&objecttype), ::core::mem::transmute(&propertyname), ::core::mem::transmute(&propertytype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, ICustomXamlResourceLoaderOverrides, OFFSET>(),
            GetResource: GetResource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICustomXamlResourceLoaderOverrides as ::windows_core::Interface>::IID
    }
}
