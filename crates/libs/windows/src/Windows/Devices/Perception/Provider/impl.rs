#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
pub trait IPerceptionFrameProvider_Impl: Sized + super::super::super::Foundation::IClosable_Impl {
    fn FrameProviderInfo(&self) -> ::windows_core::Result<PerceptionFrameProviderInfo>;
    fn Available(&self) -> ::windows_core::Result<bool>;
    fn Properties(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IPropertySet>;
    fn Start(&self) -> ::windows_core::Result<()>;
    fn Stop(&self) -> ::windows_core::Result<()>;
    fn SetProperty(&self, value: &::core::option::Option<PerceptionPropertyChangeRequest>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl ::windows_core::RuntimeName for IPerceptionFrameProvider {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionFrameProvider";
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl IPerceptionFrameProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPerceptionFrameProvider_Impl, const OFFSET: isize>() -> IPerceptionFrameProvider_Vtbl {
        unsafe extern "system" fn FrameProviderInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPerceptionFrameProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FrameProviderInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Available<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPerceptionFrameProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Available() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPerceptionFrameProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Properties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPerceptionFrameProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Start().into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPerceptionFrameProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Stop().into()
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPerceptionFrameProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute(&value)).into()
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, IPerceptionFrameProvider, OFFSET>(),
            FrameProviderInfo: FrameProviderInfo::<Identity, Impl, OFFSET>,
            Available: Available::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPerceptionFrameProvider as ::windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
pub trait IPerceptionFrameProviderManager_Impl: Sized + super::super::super::Foundation::IClosable_Impl {
    fn GetFrameProvider(&self, frameproviderinfo: &::core::option::Option<PerceptionFrameProviderInfo>) -> ::windows_core::Result<IPerceptionFrameProvider>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows_core::RuntimeName for IPerceptionFrameProviderManager {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionFrameProviderManager";
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl IPerceptionFrameProviderManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPerceptionFrameProviderManager_Impl, const OFFSET: isize>() -> IPerceptionFrameProviderManager_Vtbl {
        unsafe extern "system" fn GetFrameProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPerceptionFrameProviderManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frameproviderinfo: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFrameProvider(::core::mem::transmute(&frameproviderinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, IPerceptionFrameProviderManager, OFFSET>(),
            GetFrameProvider: GetFrameProvider::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPerceptionFrameProviderManager as ::windows_core::Interface>::IID
    }
}
