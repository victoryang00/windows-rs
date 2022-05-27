pub trait IXamlUIPresenterHost_Impl: Sized {
    fn ResolveFileResource(&self, path: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::RuntimeName for IXamlUIPresenterHost {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.IXamlUIPresenterHost";
}
impl IXamlUIPresenterHost_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXamlUIPresenterHost_Impl, const OFFSET: isize>() -> IXamlUIPresenterHost_Vtbl {
        unsafe extern "system" fn ResolveFileResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXamlUIPresenterHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResolveFileResource(::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, IXamlUIPresenterHost, OFFSET>(),
            ResolveFileResource: ResolveFileResource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXamlUIPresenterHost as ::windows_core::Interface>::IID
    }
}
pub trait IXamlUIPresenterHost2_Impl: Sized {
    fn GetGenericXamlFilePath(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::RuntimeName for IXamlUIPresenterHost2 {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.IXamlUIPresenterHost2";
}
impl IXamlUIPresenterHost2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXamlUIPresenterHost2_Impl, const OFFSET: isize>() -> IXamlUIPresenterHost2_Vtbl {
        unsafe extern "system" fn GetGenericXamlFilePath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXamlUIPresenterHost2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGenericXamlFilePath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, IXamlUIPresenterHost2, OFFSET>(),
            GetGenericXamlFilePath: GetGenericXamlFilePath::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXamlUIPresenterHost2 as ::windows_core::Interface>::IID
    }
}
pub trait IXamlUIPresenterHost3_Impl: Sized {
    fn ResolveDictionaryResource(&self, dictionary: &::core::option::Option<super::ResourceDictionary>, dictionarykey: &::core::option::Option<::windows_core::IInspectable>, suggestedvalue: &::core::option::Option<::windows_core::IInspectable>) -> ::windows_core::Result<::windows_core::IInspectable>;
}
impl ::windows_core::RuntimeName for IXamlUIPresenterHost3 {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.IXamlUIPresenterHost3";
}
impl IXamlUIPresenterHost3_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXamlUIPresenterHost3_Impl, const OFFSET: isize>() -> IXamlUIPresenterHost3_Vtbl {
        unsafe extern "system" fn ResolveDictionaryResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXamlUIPresenterHost3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionary: ::windows_core::RawPtr, dictionarykey: *mut ::core::ffi::c_void, suggestedvalue: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResolveDictionaryResource(::core::mem::transmute(&dictionary), ::core::mem::transmute(&dictionarykey), ::core::mem::transmute(&suggestedvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, IXamlUIPresenterHost3, OFFSET>(),
            ResolveDictionaryResource: ResolveDictionaryResource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXamlUIPresenterHost3 as ::windows_core::Interface>::IID
    }
}
