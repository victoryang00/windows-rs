#[cfg(feature = "Foundation")]
pub trait ISysStorageProviderEventSource_Impl: Sized {
    fn EventReceived(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ISysStorageProviderEventSource, SysStorageProviderEventReceivedEventArgs>>) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveEventReceived(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::RuntimeName for ISysStorageProviderEventSource {
    const NAME: &'static str = "Windows.System.Implementation.FileExplorer.ISysStorageProviderEventSource";
}
#[cfg(feature = "Foundation")]
impl ISysStorageProviderEventSource_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISysStorageProviderEventSource_Impl, const OFFSET: isize>() -> ISysStorageProviderEventSource_Vtbl {
        unsafe extern "system" fn EventReceived<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISysStorageProviderEventSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EventReceived(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEventReceived<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISysStorageProviderEventSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveEventReceived(::core::mem::transmute(&token)).into()
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, ISysStorageProviderEventSource, OFFSET>(),
            EventReceived: EventReceived::<Identity, Impl, OFFSET>,
            RemoveEventReceived: RemoveEventReceived::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISysStorageProviderEventSource as ::windows_core::Interface>::IID
    }
}
pub trait ISysStorageProviderHandlerFactory_Impl: Sized {
    fn GetHttpRequestProvider(&self, syncrootid: &::windows_core::HSTRING) -> ::windows_core::Result<ISysStorageProviderHttpRequestProvider>;
    fn GetEventSource(&self, syncrootid: &::windows_core::HSTRING, eventname: &::windows_core::HSTRING) -> ::windows_core::Result<ISysStorageProviderEventSource>;
}
impl ::windows_core::RuntimeName for ISysStorageProviderHandlerFactory {
    const NAME: &'static str = "Windows.System.Implementation.FileExplorer.ISysStorageProviderHandlerFactory";
}
impl ISysStorageProviderHandlerFactory_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISysStorageProviderHandlerFactory_Impl, const OFFSET: isize>() -> ISysStorageProviderHandlerFactory_Vtbl {
        unsafe extern "system" fn GetHttpRequestProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISysStorageProviderHandlerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncrootid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHttpRequestProvider(::core::mem::transmute(&syncrootid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventSource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISysStorageProviderHandlerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncrootid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, eventname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEventSource(::core::mem::transmute(&syncrootid), ::core::mem::transmute(&eventname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, ISysStorageProviderHandlerFactory, OFFSET>(),
            GetHttpRequestProvider: GetHttpRequestProvider::<Identity, Impl, OFFSET>,
            GetEventSource: GetEventSource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISysStorageProviderHandlerFactory as ::windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Web_Http"))]
pub trait ISysStorageProviderHttpRequestProvider_Impl: Sized {
    fn SendRequestAsync(&self, request: &::core::option::Option<super::super::super::Web::Http::HttpRequestMessage>) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Web::Http::HttpResponseMessage>>;
}
#[cfg(all(feature = "Foundation", feature = "Web_Http"))]
impl ::windows_core::RuntimeName for ISysStorageProviderHttpRequestProvider {
    const NAME: &'static str = "Windows.System.Implementation.FileExplorer.ISysStorageProviderHttpRequestProvider";
}
#[cfg(all(feature = "Foundation", feature = "Web_Http"))]
impl ISysStorageProviderHttpRequestProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISysStorageProviderHttpRequestProvider_Impl, const OFFSET: isize>() -> ISysStorageProviderHttpRequestProvider_Vtbl {
        unsafe extern "system" fn SendRequestAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISysStorageProviderHttpRequestProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, request: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SendRequestAsync(::core::mem::transmute(&request)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, ISysStorageProviderHttpRequestProvider, OFFSET>(),
            SendRequestAsync: SendRequestAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISysStorageProviderHttpRequestProvider as ::windows_core::Interface>::IID
    }
}
