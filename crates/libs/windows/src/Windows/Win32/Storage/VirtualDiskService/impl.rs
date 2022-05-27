pub trait IEnumVdsObject_Impl: Sized {
    fn Next(&self, celt: u32, ppobjectarray: *mut ::core::option::Option<::windows_core::IUnknown>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IEnumVdsObject>;
}
impl ::windows_core::RuntimeName for IEnumVdsObject {}
impl IEnumVdsObject_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumVdsObject_Impl, const OFFSET: isize>() -> IEnumVdsObject_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumVdsObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppobjectarray: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppobjectarray), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumVdsObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumVdsObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumVdsObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumVdsObject as ::windows_core::Interface>::IID
    }
}
pub trait IVdsAdmin_Impl: Sized {
    fn RegisterProvider(&self, providerid: &::windows_core::GUID, providerclsid: &::windows_core::GUID, pwszname: &::windows_core::PCWSTR, r#type: VDS_PROVIDER_TYPE, pwszmachinename: &::windows_core::PCWSTR, pwszversion: &::windows_core::PCWSTR, guidversionid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn UnregisterProvider(&self, providerid: &::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IVdsAdmin {}
impl IVdsAdmin_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsAdmin_Impl, const OFFSET: isize>() -> IVdsAdmin_Vtbl {
        unsafe extern "system" fn RegisterProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: ::windows_core::GUID, providerclsid: ::windows_core::GUID, pwszname: ::windows_core::PCWSTR, r#type: VDS_PROVIDER_TYPE, pwszmachinename: ::windows_core::PCWSTR, pwszversion: ::windows_core::PCWSTR, guidversionid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterProvider(::core::mem::transmute(&providerid), ::core::mem::transmute(&providerclsid), ::core::mem::transmute(&pwszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute(&pwszmachinename), ::core::mem::transmute(&pwszversion), ::core::mem::transmute(&guidversionid)).into()
        }
        unsafe extern "system" fn UnregisterProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterProvider(::core::mem::transmute(&providerid)).into()
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RegisterProvider: RegisterProvider::<Identity, Impl, OFFSET>,
            UnregisterProvider: UnregisterProvider::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsAdmin as ::windows_core::Interface>::IID
    }
}
pub trait IVdsAdviseSink_Impl: Sized {
    fn OnNotify(&self, lnumberofnotifications: i32, pnotificationarray: *const VDS_NOTIFICATION) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IVdsAdviseSink {}
impl IVdsAdviseSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsAdviseSink_Impl, const OFFSET: isize>() -> IVdsAdviseSink_Vtbl {
        unsafe extern "system" fn OnNotify<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsAdviseSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnumberofnotifications: i32, pnotificationarray: *const VDS_NOTIFICATION) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnNotify(::core::mem::transmute_copy(&lnumberofnotifications), ::core::mem::transmute_copy(&pnotificationarray)).into()
        }
        Self { base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(), OnNotify: OnNotify::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsAdviseSink as ::windows_core::Interface>::IID
    }
}
pub trait IVdsAsync_Impl: Sized {
    fn Cancel(&self) -> ::windows_core::Result<()>;
    fn Wait(&self, phrresult: *mut ::windows_core::HRESULT, pasyncout: *mut VDS_ASYNC_OUTPUT) -> ::windows_core::Result<()>;
    fn QueryStatus(&self, phrresult: *mut ::windows_core::HRESULT, pulpercentcompleted: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IVdsAsync {}
impl IVdsAsync_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsAsync_Impl, const OFFSET: isize>() -> IVdsAsync_Vtbl {
        unsafe extern "system" fn Cancel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel().into()
        }
        unsafe extern "system" fn Wait<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows_core::HRESULT, pasyncout: *mut VDS_ASYNC_OUTPUT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Wait(::core::mem::transmute_copy(&phrresult), ::core::mem::transmute_copy(&pasyncout)).into()
        }
        unsafe extern "system" fn QueryStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows_core::HRESULT, pulpercentcompleted: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryStatus(::core::mem::transmute_copy(&phrresult), ::core::mem::transmute_copy(&pulpercentcompleted)).into()
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Wait: Wait::<Identity, Impl, OFFSET>,
            QueryStatus: QueryStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsAsync as ::windows_core::Interface>::IID
    }
}
pub trait IVdsController_Impl: Sized {
    fn GetProperties(&self) -> ::windows_core::Result<VDS_CONTROLLER_PROP>;
    fn GetSubSystem(&self) -> ::windows_core::Result<IVdsSubSystem>;
    fn GetPortProperties(&self, sportnumber: i16) -> ::windows_core::Result<VDS_PORT_PROP>;
    fn FlushCache(&self) -> ::windows_core::Result<()>;
    fn InvalidateCache(&self) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn QueryAssociatedLuns(&self) -> ::windows_core::Result<IEnumVdsObject>;
    fn SetStatus(&self, status: VDS_CONTROLLER_STATUS) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IVdsController {}
impl IVdsController_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsController_Impl, const OFFSET: isize>() -> IVdsController_Vtbl {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontrollerprop: *mut VDS_CONTROLLER_PROP) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcontrollerprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubSystem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubsystem: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSubSystem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsubsystem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPortProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sportnumber: i16, pportprop: *mut VDS_PORT_PROP) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPortProperties(::core::mem::transmute_copy(&sportnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pportprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FlushCache().into()
        }
        unsafe extern "system" fn InvalidateCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InvalidateCache().into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn QueryAssociatedLuns<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryAssociatedLuns() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: VDS_CONTROLLER_STATUS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStatus(::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetSubSystem: GetSubSystem::<Identity, Impl, OFFSET>,
            GetPortProperties: GetPortProperties::<Identity, Impl, OFFSET>,
            FlushCache: FlushCache::<Identity, Impl, OFFSET>,
            InvalidateCache: InvalidateCache::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            QueryAssociatedLuns: QueryAssociatedLuns::<Identity, Impl, OFFSET>,
            SetStatus: SetStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsController as ::windows_core::Interface>::IID
    }
}
pub trait IVdsControllerControllerPort_Impl: Sized {
    fn QueryControllerPorts(&self) -> ::windows_core::Result<IEnumVdsObject>;
}
impl ::windows_core::RuntimeName for IVdsControllerControllerPort {}
impl IVdsControllerControllerPort_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsControllerControllerPort_Impl, const OFFSET: isize>() -> IVdsControllerControllerPort_Vtbl {
        unsafe extern "system" fn QueryControllerPorts<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsControllerControllerPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryControllerPorts() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(), QueryControllerPorts: QueryControllerPorts::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsControllerControllerPort as ::windows_core::Interface>::IID
    }
}
pub trait IVdsControllerPort_Impl: Sized {
    fn GetProperties(&self) -> ::windows_core::Result<VDS_PORT_PROP>;
    fn GetController(&self) -> ::windows_core::Result<IVdsController>;
    fn QueryAssociatedLuns(&self) -> ::windows_core::Result<IEnumVdsObject>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn SetStatus(&self, status: VDS_PORT_STATUS) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IVdsControllerPort {}
impl IVdsControllerPort_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsControllerPort_Impl, const OFFSET: isize>() -> IVdsControllerPort_Vtbl {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsControllerPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pportprop: *mut VDS_PORT_PROP) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pportprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetController<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsControllerPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontroller: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetController() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontroller, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAssociatedLuns<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsControllerPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryAssociatedLuns() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsControllerPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn SetStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsControllerPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: VDS_PORT_STATUS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStatus(::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetController: GetController::<Identity, Impl, OFFSET>,
            QueryAssociatedLuns: QueryAssociatedLuns::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            SetStatus: SetStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsControllerPort as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsDrive_Impl: Sized {
    fn GetProperties(&self) -> ::windows_core::Result<VDS_DRIVE_PROP>;
    fn GetSubSystem(&self) -> ::windows_core::Result<IVdsSubSystem>;
    fn QueryExtents(&self, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows_core::Result<()>;
    fn SetFlags(&self, ulflags: u32) -> ::windows_core::Result<()>;
    fn ClearFlags(&self, ulflags: u32) -> ::windows_core::Result<()>;
    fn SetStatus(&self, status: VDS_DRIVE_STATUS) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IVdsDrive {}
#[cfg(feature = "Win32_Foundation")]
impl IVdsDrive_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsDrive_Impl, const OFFSET: isize>() -> IVdsDrive_Vtbl {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsDrive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdriveprop: *mut VDS_DRIVE_PROP) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdriveprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubSystem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsDrive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubsystem: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSubSystem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsubsystem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryExtents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsDrive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryExtents(::core::mem::transmute_copy(&ppextentarray), ::core::mem::transmute_copy(&plnumberofextents)).into()
        }
        unsafe extern "system" fn SetFlags<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsDrive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFlags(::core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn ClearFlags<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsDrive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearFlags(::core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn SetStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsDrive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: VDS_DRIVE_STATUS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStatus(::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetSubSystem: GetSubSystem::<Identity, Impl, OFFSET>,
            QueryExtents: QueryExtents::<Identity, Impl, OFFSET>,
            SetFlags: SetFlags::<Identity, Impl, OFFSET>,
            ClearFlags: ClearFlags::<Identity, Impl, OFFSET>,
            SetStatus: SetStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsDrive as ::windows_core::Interface>::IID
    }
}
pub trait IVdsDrive2_Impl: Sized {
    fn GetProperties2(&self) -> ::windows_core::Result<VDS_DRIVE_PROP2>;
}
impl ::windows_core::RuntimeName for IVdsDrive2 {}
impl IVdsDrive2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsDrive2_Impl, const OFFSET: isize>() -> IVdsDrive2_Vtbl {
        unsafe extern "system" fn GetProperties2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsDrive2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdriveprop2: *mut VDS_DRIVE_PROP2) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperties2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdriveprop2, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(), GetProperties2: GetProperties2::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsDrive2 as ::windows_core::Interface>::IID
    }
}
pub trait IVdsHwProvider_Impl: Sized {
    fn QuerySubSystems(&self) -> ::windows_core::Result<IEnumVdsObject>;
    fn Reenumerate(&self) -> ::windows_core::Result<()>;
    fn Refresh(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IVdsHwProvider {}
impl IVdsHwProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsHwProvider_Impl, const OFFSET: isize>() -> IVdsHwProvider_Vtbl {
        unsafe extern "system" fn QuerySubSystems<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsHwProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QuerySubSystems() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reenumerate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsHwProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reenumerate().into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsHwProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            QuerySubSystems: QuerySubSystems::<Identity, Impl, OFFSET>,
            Reenumerate: Reenumerate::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsHwProvider as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsHwProviderPrivate_Impl: Sized {
    fn QueryIfCreatedLun(&self, pwszdevicepath: &::windows_core::PCWSTR, pvdsluninformation: *const VDS_LUN_INFORMATION) -> ::windows_core::Result<::windows_core::GUID>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IVdsHwProviderPrivate {}
#[cfg(feature = "Win32_Foundation")]
impl IVdsHwProviderPrivate_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsHwProviderPrivate_Impl, const OFFSET: isize>() -> IVdsHwProviderPrivate_Vtbl {
        unsafe extern "system" fn QueryIfCreatedLun<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsHwProviderPrivate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicepath: ::windows_core::PCWSTR, pvdsluninformation: *const VDS_LUN_INFORMATION, plunid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryIfCreatedLun(::core::mem::transmute(&pwszdevicepath), ::core::mem::transmute_copy(&pvdsluninformation)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plunid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(), QueryIfCreatedLun: QueryIfCreatedLun::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsHwProviderPrivate as ::windows_core::Interface>::IID
    }
}
pub trait IVdsHwProviderPrivateMpio_Impl: Sized {
    fn SetAllPathStatusesFromHbaPort(&self, hbaportprop: &VDS_HBAPORT_PROP, status: VDS_PATH_STATUS) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IVdsHwProviderPrivateMpio {}
impl IVdsHwProviderPrivateMpio_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsHwProviderPrivateMpio_Impl, const OFFSET: isize>() -> IVdsHwProviderPrivateMpio_Vtbl {
        unsafe extern "system" fn SetAllPathStatusesFromHbaPort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsHwProviderPrivateMpio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hbaportprop: VDS_HBAPORT_PROP, status: VDS_PATH_STATUS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllPathStatusesFromHbaPort(::core::mem::transmute(&hbaportprop), ::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetAllPathStatusesFromHbaPort: SetAllPathStatusesFromHbaPort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsHwProviderPrivateMpio as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsHwProviderStoragePools_Impl: Sized {
    fn QueryStoragePools(&self, ulflags: u32, ullremainingfreespace: u64, ppoolattributes: *const VDS_POOL_ATTRIBUTES) -> ::windows_core::Result<IEnumVdsObject>;
    fn CreateLunInStoragePool(&self, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, storagepoolid: &::windows_core::GUID, pwszunmaskinglist: &::windows_core::PCWSTR, phints2: *const VDS_HINTS2) -> ::windows_core::Result<IVdsAsync>;
    fn QueryMaxLunCreateSizeInStoragePool(&self, r#type: VDS_LUN_TYPE, storagepoolid: &::windows_core::GUID, phints2: *const VDS_HINTS2) -> ::windows_core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IVdsHwProviderStoragePools {}
#[cfg(feature = "Win32_Foundation")]
impl IVdsHwProviderStoragePools_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsHwProviderStoragePools_Impl, const OFFSET: isize>() -> IVdsHwProviderStoragePools_Vtbl {
        unsafe extern "system" fn QueryStoragePools<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsHwProviderStoragePools_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, ullremainingfreespace: u64, ppoolattributes: *const VDS_POOL_ATTRIBUTES, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryStoragePools(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&ullremainingfreespace), ::core::mem::transmute_copy(&ppoolattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLunInStoragePool<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsHwProviderStoragePools_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, storagepoolid: ::windows_core::GUID, pwszunmaskinglist: ::windows_core::PCWSTR, phints2: *const VDS_HINTS2, ppasync: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateLunInStoragePool(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&ullsizeinbytes), ::core::mem::transmute(&storagepoolid), ::core::mem::transmute(&pwszunmaskinglist), ::core::mem::transmute_copy(&phints2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryMaxLunCreateSizeInStoragePool<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsHwProviderStoragePools_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, storagepoolid: ::windows_core::GUID, phints2: *const VDS_HINTS2, pullmaxlunsize: *mut u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryMaxLunCreateSizeInStoragePool(::core::mem::transmute_copy(&r#type), ::core::mem::transmute(&storagepoolid), ::core::mem::transmute_copy(&phints2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullmaxlunsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            QueryStoragePools: QueryStoragePools::<Identity, Impl, OFFSET>,
            CreateLunInStoragePool: CreateLunInStoragePool::<Identity, Impl, OFFSET>,
            QueryMaxLunCreateSizeInStoragePool: QueryMaxLunCreateSizeInStoragePool::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsHwProviderStoragePools as ::windows_core::Interface>::IID
    }
}
pub trait IVdsHwProviderType_Impl: Sized {
    fn GetProviderType(&self) -> ::windows_core::Result<VDS_HWPROVIDER_TYPE>;
}
impl ::windows_core::RuntimeName for IVdsHwProviderType {}
impl IVdsHwProviderType_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsHwProviderType_Impl, const OFFSET: isize>() -> IVdsHwProviderType_Vtbl {
        unsafe extern "system" fn GetProviderType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsHwProviderType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut VDS_HWPROVIDER_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProviderType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(), GetProviderType: GetProviderType::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsHwProviderType as ::windows_core::Interface>::IID
    }
}
pub trait IVdsHwProviderType2_Impl: Sized {
    fn GetProviderType2(&self) -> ::windows_core::Result<VDS_HWPROVIDER_TYPE>;
}
impl ::windows_core::RuntimeName for IVdsHwProviderType2 {}
impl IVdsHwProviderType2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsHwProviderType2_Impl, const OFFSET: isize>() -> IVdsHwProviderType2_Vtbl {
        unsafe extern "system" fn GetProviderType2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsHwProviderType2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut VDS_HWPROVIDER_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProviderType2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(), GetProviderType2: GetProviderType2::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsHwProviderType2 as ::windows_core::Interface>::IID
    }
}
pub trait IVdsIscsiPortal_Impl: Sized {
    fn GetProperties(&self) -> ::windows_core::Result<VDS_ISCSI_PORTAL_PROP>;
    fn GetSubSystem(&self) -> ::windows_core::Result<IVdsSubSystem>;
    fn QueryAssociatedPortalGroups(&self) -> ::windows_core::Result<IEnumVdsObject>;
    fn SetStatus(&self, status: VDS_ISCSI_PORTAL_STATUS) -> ::windows_core::Result<()>;
    fn SetIpsecTunnelAddress(&self, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> ::windows_core::Result<()>;
    fn GetIpsecSecurity(&self, pinitiatorportaladdress: *const VDS_IPADDRESS) -> ::windows_core::Result<u64>;
    fn SetIpsecSecurity(&self, pinitiatorportaladdress: *const VDS_IPADDRESS, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IVdsIscsiPortal {}
impl IVdsIscsiPortal_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsIscsiPortal_Impl, const OFFSET: isize>() -> IVdsIscsiPortal_Vtbl {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsIscsiPortal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pportalprop: *mut VDS_ISCSI_PORTAL_PROP) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pportalprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubSystem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsIscsiPortal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubsystem: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSubSystem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsubsystem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAssociatedPortalGroups<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsIscsiPortal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryAssociatedPortalGroups() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsIscsiPortal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: VDS_ISCSI_PORTAL_STATUS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStatus(::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn SetIpsecTunnelAddress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsIscsiPortal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIpsecTunnelAddress(::core::mem::transmute_copy(&ptunneladdress), ::core::mem::transmute_copy(&pdestinationaddress)).into()
        }
        unsafe extern "system" fn GetIpsecSecurity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsIscsiPortal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinitiatorportaladdress: *const VDS_IPADDRESS, pullsecurityflags: *mut u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIpsecSecurity(::core::mem::transmute_copy(&pinitiatorportaladdress)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullsecurityflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpsecSecurity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsIscsiPortal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinitiatorportaladdress: *const VDS_IPADDRESS, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIpsecSecurity(::core::mem::transmute_copy(&pinitiatorportaladdress), ::core::mem::transmute_copy(&ullsecurityflags), ::core::mem::transmute_copy(&pipseckey)).into()
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetSubSystem: GetSubSystem::<Identity, Impl, OFFSET>,
            QueryAssociatedPortalGroups: QueryAssociatedPortalGroups::<Identity, Impl, OFFSET>,
            SetStatus: SetStatus::<Identity, Impl, OFFSET>,
            SetIpsecTunnelAddress: SetIpsecTunnelAddress::<Identity, Impl, OFFSET>,
            GetIpsecSecurity: GetIpsecSecurity::<Identity, Impl, OFFSET>,
            SetIpsecSecurity: SetIpsecSecurity::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsIscsiPortal as ::windows_core::Interface>::IID
    }
}
pub trait IVdsIscsiPortalGroup_Impl: Sized {
    fn GetProperties(&self) -> ::windows_core::Result<VDS_ISCSI_PORTALGROUP_PROP>;
    fn GetTarget(&self) -> ::windows_core::Result<IVdsIscsiTarget>;
    fn QueryAssociatedPortals(&self) -> ::windows_core::Result<IEnumVdsObject>;
    fn AddPortal(&self, portalid: &::windows_core::GUID) -> ::windows_core::Result<IVdsAsync>;
    fn RemovePortal(&self, portalid: &::windows_core::GUID) -> ::windows_core::Result<IVdsAsync>;
    fn Delete(&self) -> ::windows_core::Result<IVdsAsync>;
}
impl ::windows_core::RuntimeName for IVdsIscsiPortalGroup {}
impl IVdsIscsiPortalGroup_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsIscsiPortalGroup_Impl, const OFFSET: isize>() -> IVdsIscsiPortalGroup_Vtbl {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsIscsiPortalGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pportalgroupprop: *mut VDS_ISCSI_PORTALGROUP_PROP) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pportalgroupprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTarget<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsIscsiPortalGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptarget: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTarget() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptarget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAssociatedPortals<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsIscsiPortalGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryAssociatedPortals() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPortal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsIscsiPortalGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portalid: ::windows_core::GUID, ppasync: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddPortal(::core::mem::transmute(&portalid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePortal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsIscsiPortalGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portalid: ::windows_core::GUID, ppasync: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RemovePortal(::core::mem::transmute(&portalid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsIscsiPortalGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppasync: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Delete() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetTarget: GetTarget::<Identity, Impl, OFFSET>,
            QueryAssociatedPortals: QueryAssociatedPortals::<Identity, Impl, OFFSET>,
            AddPortal: AddPortal::<Identity, Impl, OFFSET>,
            RemovePortal: RemovePortal::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsIscsiPortalGroup as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsIscsiTarget_Impl: Sized {
    fn GetProperties(&self) -> ::windows_core::Result<VDS_ISCSI_TARGET_PROP>;
    fn GetSubSystem(&self) -> ::windows_core::Result<IVdsSubSystem>;
    fn QueryPortalGroups(&self) -> ::windows_core::Result<IEnumVdsObject>;
    fn QueryAssociatedLuns(&self) -> ::windows_core::Result<IEnumVdsObject>;
    fn CreatePortalGroup(&self) -> ::windows_core::Result<IVdsAsync>;
    fn Delete(&self) -> ::windows_core::Result<IVdsAsync>;
    fn SetFriendlyName(&self, pwszfriendlyname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetSharedSecret(&self, ptargetsharedsecret: *const VDS_ISCSI_SHARED_SECRET, pwszinitiatorname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn RememberInitiatorSharedSecret(&self, pwszinitiatorname: &::windows_core::PCWSTR, pinitiatorsharedsecret: *const VDS_ISCSI_SHARED_SECRET) -> ::windows_core::Result<()>;
    fn GetConnectedInitiators(&self, pppwszinitiatorlist: *mut *mut ::windows_core::PWSTR, plnumberofinitiators: *mut i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IVdsIscsiTarget {}
#[cfg(feature = "Win32_Foundation")]
impl IVdsIscsiTarget_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>() -> IVdsIscsiTarget_Vtbl {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetprop: *mut VDS_ISCSI_TARGET_PROP) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptargetprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubSystem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubsystem: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSubSystem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsubsystem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryPortalGroups<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryPortalGroups() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAssociatedLuns<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryAssociatedLuns() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePortalGroup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppasync: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePortalGroup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppasync: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Delete() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFriendlyName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfriendlyname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFriendlyName(::core::mem::transmute(&pwszfriendlyname)).into()
        }
        unsafe extern "system" fn SetSharedSecret<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetsharedsecret: *const VDS_ISCSI_SHARED_SECRET, pwszinitiatorname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSharedSecret(::core::mem::transmute_copy(&ptargetsharedsecret), ::core::mem::transmute(&pwszinitiatorname)).into()
        }
        unsafe extern "system" fn RememberInitiatorSharedSecret<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszinitiatorname: ::windows_core::PCWSTR, pinitiatorsharedsecret: *const VDS_ISCSI_SHARED_SECRET) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RememberInitiatorSharedSecret(::core::mem::transmute(&pwszinitiatorname), ::core::mem::transmute_copy(&pinitiatorsharedsecret)).into()
        }
        unsafe extern "system" fn GetConnectedInitiators<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsIscsiTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppwszinitiatorlist: *mut *mut ::windows_core::PWSTR, plnumberofinitiators: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetConnectedInitiators(::core::mem::transmute_copy(&pppwszinitiatorlist), ::core::mem::transmute_copy(&plnumberofinitiators)).into()
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetSubSystem: GetSubSystem::<Identity, Impl, OFFSET>,
            QueryPortalGroups: QueryPortalGroups::<Identity, Impl, OFFSET>,
            QueryAssociatedLuns: QueryAssociatedLuns::<Identity, Impl, OFFSET>,
            CreatePortalGroup: CreatePortalGroup::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            SetFriendlyName: SetFriendlyName::<Identity, Impl, OFFSET>,
            SetSharedSecret: SetSharedSecret::<Identity, Impl, OFFSET>,
            RememberInitiatorSharedSecret: RememberInitiatorSharedSecret::<Identity, Impl, OFFSET>,
            GetConnectedInitiators: GetConnectedInitiators::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsIscsiTarget as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsLun_Impl: Sized {
    fn GetProperties(&self) -> ::windows_core::Result<VDS_LUN_PROP>;
    fn GetSubSystem(&self) -> ::windows_core::Result<IVdsSubSystem>;
    fn GetIdentificationData(&self) -> ::windows_core::Result<VDS_LUN_INFORMATION>;
    fn QueryActiveControllers(&self) -> ::windows_core::Result<IEnumVdsObject>;
    fn Extend(&self, ullnumberofbytestoadd: u64, pdriveidarray: *const ::windows_core::GUID, lnumberofdrives: i32) -> ::windows_core::Result<IVdsAsync>;
    fn Shrink(&self, ullnumberofbytestoremove: u64) -> ::windows_core::Result<IVdsAsync>;
    fn QueryPlexes(&self) -> ::windows_core::Result<IEnumVdsObject>;
    fn AddPlex(&self, lunid: &::windows_core::GUID) -> ::windows_core::Result<IVdsAsync>;
    fn RemovePlex(&self, plexid: &::windows_core::GUID) -> ::windows_core::Result<IVdsAsync>;
    fn Recover(&self) -> ::windows_core::Result<IVdsAsync>;
    fn SetMask(&self, pwszunmaskinglist: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Delete(&self) -> ::windows_core::Result<()>;
    fn AssociateControllers(&self, pactivecontrolleridarray: *const ::windows_core::GUID, lnumberofactivecontrollers: i32, pinactivecontrolleridarray: *const ::windows_core::GUID, lnumberofinactivecontrollers: i32) -> ::windows_core::Result<()>;
    fn QueryHints(&self) -> ::windows_core::Result<VDS_HINTS>;
    fn ApplyHints(&self, phints: *const VDS_HINTS) -> ::windows_core::Result<()>;
    fn SetStatus(&self, status: VDS_LUN_STATUS) -> ::windows_core::Result<()>;
    fn QueryMaxLunExtendSize(&self, pdriveidarray: *const ::windows_core::GUID, lnumberofdrives: i32) -> ::windows_core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IVdsLun {}
#[cfg(feature = "Win32_Foundation")]
impl IVdsLun_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: isize>() -> IVdsLun_Vtbl {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plunprop: *mut VDS_LUN_PROP) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plunprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubSystem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubsystem: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSubSystem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsubsystem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIdentificationData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pluninfo: *mut VDS_LUN_INFORMATION) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIdentificationData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pluninfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryActiveControllers<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryActiveControllers() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extend<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullnumberofbytestoadd: u64, pdriveidarray: *const ::windows_core::GUID, lnumberofdrives: i32, ppasync: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Extend(::core::mem::transmute_copy(&ullnumberofbytestoadd), ::core::mem::transmute_copy(&pdriveidarray), ::core::mem::transmute_copy(&lnumberofdrives)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shrink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullnumberofbytestoremove: u64, ppasync: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Shrink(::core::mem::transmute_copy(&ullnumberofbytestoremove)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryPlexes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryPlexes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPlex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lunid: ::windows_core::GUID, ppasync: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddPlex(::core::mem::transmute(&lunid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePlex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plexid: ::windows_core::GUID, ppasync: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RemovePlex(::core::mem::transmute(&plexid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recover<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppasync: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Recover() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMask<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszunmaskinglist: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMask(::core::mem::transmute(&pwszunmaskinglist)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        unsafe extern "system" fn AssociateControllers<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactivecontrolleridarray: *const ::windows_core::GUID, lnumberofactivecontrollers: i32, pinactivecontrolleridarray: *const ::windows_core::GUID, lnumberofinactivecontrollers: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AssociateControllers(::core::mem::transmute_copy(&pactivecontrolleridarray), ::core::mem::transmute_copy(&lnumberofactivecontrollers), ::core::mem::transmute_copy(&pinactivecontrolleridarray), ::core::mem::transmute_copy(&lnumberofinactivecontrollers)).into()
        }
        unsafe extern "system" fn QueryHints<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phints: *mut VDS_HINTS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryHints() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phints, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyHints<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phints: *const VDS_HINTS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ApplyHints(::core::mem::transmute_copy(&phints)).into()
        }
        unsafe extern "system" fn SetStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: VDS_LUN_STATUS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStatus(::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn QueryMaxLunExtendSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLun_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdriveidarray: *const ::windows_core::GUID, lnumberofdrives: i32, pullmaxbytestobeadded: *mut u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryMaxLunExtendSize(::core::mem::transmute_copy(&pdriveidarray), ::core::mem::transmute_copy(&lnumberofdrives)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullmaxbytestobeadded, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetSubSystem: GetSubSystem::<Identity, Impl, OFFSET>,
            GetIdentificationData: GetIdentificationData::<Identity, Impl, OFFSET>,
            QueryActiveControllers: QueryActiveControllers::<Identity, Impl, OFFSET>,
            Extend: Extend::<Identity, Impl, OFFSET>,
            Shrink: Shrink::<Identity, Impl, OFFSET>,
            QueryPlexes: QueryPlexes::<Identity, Impl, OFFSET>,
            AddPlex: AddPlex::<Identity, Impl, OFFSET>,
            RemovePlex: RemovePlex::<Identity, Impl, OFFSET>,
            Recover: Recover::<Identity, Impl, OFFSET>,
            SetMask: SetMask::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            AssociateControllers: AssociateControllers::<Identity, Impl, OFFSET>,
            QueryHints: QueryHints::<Identity, Impl, OFFSET>,
            ApplyHints: ApplyHints::<Identity, Impl, OFFSET>,
            SetStatus: SetStatus::<Identity, Impl, OFFSET>,
            QueryMaxLunExtendSize: QueryMaxLunExtendSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsLun as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsLun2_Impl: Sized {
    fn QueryHints2(&self) -> ::windows_core::Result<VDS_HINTS2>;
    fn ApplyHints2(&self, phints2: *const VDS_HINTS2) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IVdsLun2 {}
#[cfg(feature = "Win32_Foundation")]
impl IVdsLun2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLun2_Impl, const OFFSET: isize>() -> IVdsLun2_Vtbl {
        unsafe extern "system" fn QueryHints2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLun2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phints2: *mut VDS_HINTS2) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryHints2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phints2, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyHints2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLun2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phints2: *const VDS_HINTS2) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ApplyHints2(::core::mem::transmute_copy(&phints2)).into()
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            QueryHints2: QueryHints2::<Identity, Impl, OFFSET>,
            ApplyHints2: ApplyHints2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsLun2 as ::windows_core::Interface>::IID
    }
}
pub trait IVdsLunControllerPorts_Impl: Sized {
    fn AssociateControllerPorts(&self, pactivecontrollerportidarray: *const ::windows_core::GUID, lnumberofactivecontrollerports: i32, pinactivecontrollerportidarray: *const ::windows_core::GUID, lnumberofinactivecontrollerports: i32) -> ::windows_core::Result<()>;
    fn QueryActiveControllerPorts(&self) -> ::windows_core::Result<IEnumVdsObject>;
}
impl ::windows_core::RuntimeName for IVdsLunControllerPorts {}
impl IVdsLunControllerPorts_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLunControllerPorts_Impl, const OFFSET: isize>() -> IVdsLunControllerPorts_Vtbl {
        unsafe extern "system" fn AssociateControllerPorts<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLunControllerPorts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactivecontrollerportidarray: *const ::windows_core::GUID, lnumberofactivecontrollerports: i32, pinactivecontrollerportidarray: *const ::windows_core::GUID, lnumberofinactivecontrollerports: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AssociateControllerPorts(::core::mem::transmute_copy(&pactivecontrollerportidarray), ::core::mem::transmute_copy(&lnumberofactivecontrollerports), ::core::mem::transmute_copy(&pinactivecontrollerportidarray), ::core::mem::transmute_copy(&lnumberofinactivecontrollerports)).into()
        }
        unsafe extern "system" fn QueryActiveControllerPorts<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLunControllerPorts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryActiveControllerPorts() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AssociateControllerPorts: AssociateControllerPorts::<Identity, Impl, OFFSET>,
            QueryActiveControllerPorts: QueryActiveControllerPorts::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsLunControllerPorts as ::windows_core::Interface>::IID
    }
}
pub trait IVdsLunIscsi_Impl: Sized {
    fn AssociateTargets(&self, ptargetidarray: *const ::windows_core::GUID, lnumberoftargets: i32) -> ::windows_core::Result<()>;
    fn QueryAssociatedTargets(&self) -> ::windows_core::Result<IEnumVdsObject>;
}
impl ::windows_core::RuntimeName for IVdsLunIscsi {}
impl IVdsLunIscsi_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLunIscsi_Impl, const OFFSET: isize>() -> IVdsLunIscsi_Vtbl {
        unsafe extern "system" fn AssociateTargets<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLunIscsi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetidarray: *const ::windows_core::GUID, lnumberoftargets: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AssociateTargets(::core::mem::transmute_copy(&ptargetidarray), ::core::mem::transmute_copy(&lnumberoftargets)).into()
        }
        unsafe extern "system" fn QueryAssociatedTargets<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLunIscsi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryAssociatedTargets() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AssociateTargets: AssociateTargets::<Identity, Impl, OFFSET>,
            QueryAssociatedTargets: QueryAssociatedTargets::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsLunIscsi as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsLunMpio_Impl: Sized {
    fn GetPathInfo(&self, pppaths: *mut *mut VDS_PATH_INFO, plnumberofpaths: *mut i32) -> ::windows_core::Result<()>;
    fn GetLoadBalancePolicy(&self, ppolicy: *mut VDS_LOADBALANCE_POLICY_ENUM, pppaths: *mut *mut VDS_PATH_POLICY, plnumberofpaths: *mut i32) -> ::windows_core::Result<()>;
    fn SetLoadBalancePolicy(&self, policy: VDS_LOADBALANCE_POLICY_ENUM, ppaths: *const VDS_PATH_POLICY, lnumberofpaths: i32) -> ::windows_core::Result<()>;
    fn GetSupportedLbPolicies(&self) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IVdsLunMpio {}
#[cfg(feature = "Win32_Foundation")]
impl IVdsLunMpio_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLunMpio_Impl, const OFFSET: isize>() -> IVdsLunMpio_Vtbl {
        unsafe extern "system" fn GetPathInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLunMpio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppaths: *mut *mut VDS_PATH_INFO, plnumberofpaths: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPathInfo(::core::mem::transmute_copy(&pppaths), ::core::mem::transmute_copy(&plnumberofpaths)).into()
        }
        unsafe extern "system" fn GetLoadBalancePolicy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLunMpio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppolicy: *mut VDS_LOADBALANCE_POLICY_ENUM, pppaths: *mut *mut VDS_PATH_POLICY, plnumberofpaths: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLoadBalancePolicy(::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&pppaths), ::core::mem::transmute_copy(&plnumberofpaths)).into()
        }
        unsafe extern "system" fn SetLoadBalancePolicy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLunMpio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, policy: VDS_LOADBALANCE_POLICY_ENUM, ppaths: *const VDS_PATH_POLICY, lnumberofpaths: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLoadBalancePolicy(::core::mem::transmute_copy(&policy), ::core::mem::transmute_copy(&ppaths), ::core::mem::transmute_copy(&lnumberofpaths)).into()
        }
        unsafe extern "system" fn GetSupportedLbPolicies<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLunMpio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullbflags: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSupportedLbPolicies() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullbflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetPathInfo: GetPathInfo::<Identity, Impl, OFFSET>,
            GetLoadBalancePolicy: GetLoadBalancePolicy::<Identity, Impl, OFFSET>,
            SetLoadBalancePolicy: SetLoadBalancePolicy::<Identity, Impl, OFFSET>,
            GetSupportedLbPolicies: GetSupportedLbPolicies::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsLunMpio as ::windows_core::Interface>::IID
    }
}
pub trait IVdsLunNaming_Impl: Sized {
    fn SetFriendlyName(&self, pwszfriendlyname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IVdsLunNaming {}
impl IVdsLunNaming_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLunNaming_Impl, const OFFSET: isize>() -> IVdsLunNaming_Vtbl {
        unsafe extern "system" fn SetFriendlyName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLunNaming_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfriendlyname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFriendlyName(::core::mem::transmute(&pwszfriendlyname)).into()
        }
        Self { base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(), SetFriendlyName: SetFriendlyName::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsLunNaming as ::windows_core::Interface>::IID
    }
}
pub trait IVdsLunNumber_Impl: Sized {
    fn GetLunNumber(&self) -> ::windows_core::Result<u32>;
}
impl ::windows_core::RuntimeName for IVdsLunNumber {}
impl IVdsLunNumber_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLunNumber_Impl, const OFFSET: isize>() -> IVdsLunNumber_Vtbl {
        unsafe extern "system" fn GetLunNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLunNumber_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullunnumber: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLunNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullunnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(), GetLunNumber: GetLunNumber::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsLunNumber as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsLunPlex_Impl: Sized {
    fn GetProperties(&self) -> ::windows_core::Result<VDS_LUN_PLEX_PROP>;
    fn GetLun(&self) -> ::windows_core::Result<IVdsLun>;
    fn QueryExtents(&self, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows_core::Result<()>;
    fn QueryHints(&self) -> ::windows_core::Result<VDS_HINTS>;
    fn ApplyHints(&self, phints: *const VDS_HINTS) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IVdsLunPlex {}
#[cfg(feature = "Win32_Foundation")]
impl IVdsLunPlex_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLunPlex_Impl, const OFFSET: isize>() -> IVdsLunPlex_Vtbl {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLunPlex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplexprop: *mut VDS_LUN_PLEX_PROP) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplexprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLun<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLunPlex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplun: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLun() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplun, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryExtents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLunPlex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryExtents(::core::mem::transmute_copy(&ppextentarray), ::core::mem::transmute_copy(&plnumberofextents)).into()
        }
        unsafe extern "system" fn QueryHints<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLunPlex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phints: *mut VDS_HINTS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryHints() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phints, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyHints<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsLunPlex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phints: *const VDS_HINTS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ApplyHints(::core::mem::transmute_copy(&phints)).into()
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetLun: GetLun::<Identity, Impl, OFFSET>,
            QueryExtents: QueryExtents::<Identity, Impl, OFFSET>,
            QueryHints: QueryHints::<Identity, Impl, OFFSET>,
            ApplyHints: ApplyHints::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsLunPlex as ::windows_core::Interface>::IID
    }
}
pub trait IVdsMaintenance_Impl: Sized {
    fn StartMaintenance(&self, operation: VDS_MAINTENANCE_OPERATION) -> ::windows_core::Result<()>;
    fn StopMaintenance(&self, operation: VDS_MAINTENANCE_OPERATION) -> ::windows_core::Result<()>;
    fn PulseMaintenance(&self, operation: VDS_MAINTENANCE_OPERATION, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IVdsMaintenance {}
impl IVdsMaintenance_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsMaintenance_Impl, const OFFSET: isize>() -> IVdsMaintenance_Vtbl {
        unsafe extern "system" fn StartMaintenance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsMaintenance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operation: VDS_MAINTENANCE_OPERATION) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartMaintenance(::core::mem::transmute_copy(&operation)).into()
        }
        unsafe extern "system" fn StopMaintenance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsMaintenance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operation: VDS_MAINTENANCE_OPERATION) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StopMaintenance(::core::mem::transmute_copy(&operation)).into()
        }
        unsafe extern "system" fn PulseMaintenance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsMaintenance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operation: VDS_MAINTENANCE_OPERATION, ulcount: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PulseMaintenance(::core::mem::transmute_copy(&operation), ::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            StartMaintenance: StartMaintenance::<Identity, Impl, OFFSET>,
            StopMaintenance: StopMaintenance::<Identity, Impl, OFFSET>,
            PulseMaintenance: PulseMaintenance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsMaintenance as ::windows_core::Interface>::IID
    }
}
pub trait IVdsProvider_Impl: Sized {
    fn GetProperties(&self) -> ::windows_core::Result<VDS_PROVIDER_PROP>;
}
impl ::windows_core::RuntimeName for IVdsProvider {}
impl IVdsProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsProvider_Impl, const OFFSET: isize>() -> IVdsProvider_Vtbl {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproviderprop: *mut VDS_PROVIDER_PROP) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproviderprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(), GetProperties: GetProperties::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsProvider as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsProviderPrivate_Impl: Sized {
    fn GetObject(&self, objectid: &::windows_core::GUID, r#type: VDS_OBJECT_TYPE) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn OnLoad(&self, pwszmachinename: &::windows_core::PCWSTR, pcallbackobject: &::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn OnUnload(&self, bforceunload: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IVdsProviderPrivate {}
#[cfg(feature = "Win32_Foundation")]
impl IVdsProviderPrivate_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsProviderPrivate_Impl, const OFFSET: isize>() -> IVdsProviderPrivate_Vtbl {
        unsafe extern "system" fn GetObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsProviderPrivate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectid: ::windows_core::GUID, r#type: VDS_OBJECT_TYPE, ppobjectunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetObject(::core::mem::transmute(&objectid), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobjectunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnLoad<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsProviderPrivate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszmachinename: ::windows_core::PCWSTR, pcallbackobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnLoad(::core::mem::transmute(&pwszmachinename), ::core::mem::transmute(&pcallbackobject)).into()
        }
        unsafe extern "system" fn OnUnload<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsProviderPrivate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bforceunload: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnUnload(::core::mem::transmute_copy(&bforceunload)).into()
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetObject: GetObject::<Identity, Impl, OFFSET>,
            OnLoad: OnLoad::<Identity, Impl, OFFSET>,
            OnUnload: OnUnload::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsProviderPrivate as ::windows_core::Interface>::IID
    }
}
pub trait IVdsProviderSupport_Impl: Sized {
    fn GetVersionSupport(&self) -> ::windows_core::Result<u32>;
}
impl ::windows_core::RuntimeName for IVdsProviderSupport {}
impl IVdsProviderSupport_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsProviderSupport_Impl, const OFFSET: isize>() -> IVdsProviderSupport_Vtbl {
        unsafe extern "system" fn GetVersionSupport<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsProviderSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulversionsupport: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVersionSupport() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ulversionsupport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(), GetVersionSupport: GetVersionSupport::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsProviderSupport as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsStoragePool_Impl: Sized {
    fn GetProvider(&self) -> ::windows_core::Result<IVdsProvider>;
    fn GetProperties(&self) -> ::windows_core::Result<VDS_STORAGE_POOL_PROP>;
    fn GetAttributes(&self) -> ::windows_core::Result<VDS_POOL_ATTRIBUTES>;
    fn QueryDriveExtents(&self, ppextentarray: *mut *mut VDS_STORAGE_POOL_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows_core::Result<()>;
    fn QueryAllocatedLuns(&self) -> ::windows_core::Result<IEnumVdsObject>;
    fn QueryAllocatedStoragePools(&self) -> ::windows_core::Result<IEnumVdsObject>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IVdsStoragePool {}
#[cfg(feature = "Win32_Foundation")]
impl IVdsStoragePool_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsStoragePool_Impl, const OFFSET: isize>() -> IVdsStoragePool_Vtbl {
        unsafe extern "system" fn GetProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsStoragePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprovider: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProvider() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprovider, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsStoragePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstoragepoolprop: *mut VDS_STORAGE_POOL_PROP) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstoragepoolprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsStoragePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstoragepoolattributes: *mut VDS_POOL_ATTRIBUTES) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstoragepoolattributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryDriveExtents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsStoragePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppextentarray: *mut *mut VDS_STORAGE_POOL_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryDriveExtents(::core::mem::transmute_copy(&ppextentarray), ::core::mem::transmute_copy(&plnumberofextents)).into()
        }
        unsafe extern "system" fn QueryAllocatedLuns<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsStoragePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryAllocatedLuns() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAllocatedStoragePools<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsStoragePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryAllocatedStoragePools() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProvider: GetProvider::<Identity, Impl, OFFSET>,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetAttributes: GetAttributes::<Identity, Impl, OFFSET>,
            QueryDriveExtents: QueryDriveExtents::<Identity, Impl, OFFSET>,
            QueryAllocatedLuns: QueryAllocatedLuns::<Identity, Impl, OFFSET>,
            QueryAllocatedStoragePools: QueryAllocatedStoragePools::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsStoragePool as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsSubSystem_Impl: Sized {
    fn GetProperties(&self) -> ::windows_core::Result<VDS_SUB_SYSTEM_PROP>;
    fn GetProvider(&self) -> ::windows_core::Result<IVdsProvider>;
    fn QueryControllers(&self) -> ::windows_core::Result<IEnumVdsObject>;
    fn QueryLuns(&self) -> ::windows_core::Result<IEnumVdsObject>;
    fn QueryDrives(&self) -> ::windows_core::Result<IEnumVdsObject>;
    fn GetDrive(&self, sbusnumber: i16, sslotnumber: i16) -> ::windows_core::Result<IVdsDrive>;
    fn Reenumerate(&self) -> ::windows_core::Result<()>;
    fn SetControllerStatus(&self, ponlinecontrolleridarray: *const ::windows_core::GUID, lnumberofonlinecontrollers: i32, pofflinecontrolleridarray: *const ::windows_core::GUID, lnumberofofflinecontrollers: i32) -> ::windows_core::Result<()>;
    fn CreateLun(&self, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const ::windows_core::GUID, lnumberofdrives: i32, pwszunmaskinglist: &::windows_core::PCWSTR, phints: *const VDS_HINTS) -> ::windows_core::Result<IVdsAsync>;
    fn ReplaceDrive(&self, drivetobereplaced: &::windows_core::GUID, replacementdrive: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SetStatus(&self, status: VDS_SUB_SYSTEM_STATUS) -> ::windows_core::Result<()>;
    fn QueryMaxLunCreateSize(&self, r#type: VDS_LUN_TYPE, pdriveidarray: *const ::windows_core::GUID, lnumberofdrives: i32, phints: *const VDS_HINTS) -> ::windows_core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IVdsSubSystem {}
#[cfg(feature = "Win32_Foundation")]
impl IVdsSubSystem_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsSubSystem_Impl, const OFFSET: isize>() -> IVdsSubSystem_Vtbl {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubsystemprop: *mut VDS_SUB_SYSTEM_PROP) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psubsystemprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprovider: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProvider() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprovider, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryControllers<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryControllers() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryLuns<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryLuns() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryDrives<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryDrives() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDrive<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sbusnumber: i16, sslotnumber: i16, ppdrive: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDrive(::core::mem::transmute_copy(&sbusnumber), ::core::mem::transmute_copy(&sslotnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdrive, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reenumerate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reenumerate().into()
        }
        unsafe extern "system" fn SetControllerStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ponlinecontrolleridarray: *const ::windows_core::GUID, lnumberofonlinecontrollers: i32, pofflinecontrolleridarray: *const ::windows_core::GUID, lnumberofofflinecontrollers: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetControllerStatus(::core::mem::transmute_copy(&ponlinecontrolleridarray), ::core::mem::transmute_copy(&lnumberofonlinecontrollers), ::core::mem::transmute_copy(&pofflinecontrolleridarray), ::core::mem::transmute_copy(&lnumberofofflinecontrollers)).into()
        }
        unsafe extern "system" fn CreateLun<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const ::windows_core::GUID, lnumberofdrives: i32, pwszunmaskinglist: ::windows_core::PCWSTR, phints: *const VDS_HINTS, ppasync: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateLun(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&ullsizeinbytes), ::core::mem::transmute_copy(&pdriveidarray), ::core::mem::transmute_copy(&lnumberofdrives), ::core::mem::transmute(&pwszunmaskinglist), ::core::mem::transmute_copy(&phints)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReplaceDrive<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drivetobereplaced: ::windows_core::GUID, replacementdrive: ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReplaceDrive(::core::mem::transmute(&drivetobereplaced), ::core::mem::transmute(&replacementdrive)).into()
        }
        unsafe extern "system" fn SetStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: VDS_SUB_SYSTEM_STATUS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStatus(::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn QueryMaxLunCreateSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsSubSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, pdriveidarray: *const ::windows_core::GUID, lnumberofdrives: i32, phints: *const VDS_HINTS, pullmaxlunsize: *mut u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryMaxLunCreateSize(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdriveidarray), ::core::mem::transmute_copy(&lnumberofdrives), ::core::mem::transmute_copy(&phints)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullmaxlunsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetProvider: GetProvider::<Identity, Impl, OFFSET>,
            QueryControllers: QueryControllers::<Identity, Impl, OFFSET>,
            QueryLuns: QueryLuns::<Identity, Impl, OFFSET>,
            QueryDrives: QueryDrives::<Identity, Impl, OFFSET>,
            GetDrive: GetDrive::<Identity, Impl, OFFSET>,
            Reenumerate: Reenumerate::<Identity, Impl, OFFSET>,
            SetControllerStatus: SetControllerStatus::<Identity, Impl, OFFSET>,
            CreateLun: CreateLun::<Identity, Impl, OFFSET>,
            ReplaceDrive: ReplaceDrive::<Identity, Impl, OFFSET>,
            SetStatus: SetStatus::<Identity, Impl, OFFSET>,
            QueryMaxLunCreateSize: QueryMaxLunCreateSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsSubSystem as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsSubSystem2_Impl: Sized {
    fn GetProperties2(&self) -> ::windows_core::Result<VDS_SUB_SYSTEM_PROP2>;
    fn GetDrive2(&self, sbusnumber: i16, sslotnumber: i16, ulenclosurenumber: u32) -> ::windows_core::Result<IVdsDrive>;
    fn CreateLun2(&self, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const ::windows_core::GUID, lnumberofdrives: i32, pwszunmaskinglist: &::windows_core::PCWSTR, phints2: *const VDS_HINTS2) -> ::windows_core::Result<IVdsAsync>;
    fn QueryMaxLunCreateSize2(&self, r#type: VDS_LUN_TYPE, pdriveidarray: *const ::windows_core::GUID, lnumberofdrives: i32, phints2: *const VDS_HINTS2) -> ::windows_core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IVdsSubSystem2 {}
#[cfg(feature = "Win32_Foundation")]
impl IVdsSubSystem2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsSubSystem2_Impl, const OFFSET: isize>() -> IVdsSubSystem2_Vtbl {
        unsafe extern "system" fn GetProperties2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsSubSystem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubsystemprop2: *mut VDS_SUB_SYSTEM_PROP2) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperties2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psubsystemprop2, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDrive2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsSubSystem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sbusnumber: i16, sslotnumber: i16, ulenclosurenumber: u32, ppdrive: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDrive2(::core::mem::transmute_copy(&sbusnumber), ::core::mem::transmute_copy(&sslotnumber), ::core::mem::transmute_copy(&ulenclosurenumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdrive, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLun2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsSubSystem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const ::windows_core::GUID, lnumberofdrives: i32, pwszunmaskinglist: ::windows_core::PCWSTR, phints2: *const VDS_HINTS2, ppasync: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateLun2(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&ullsizeinbytes), ::core::mem::transmute_copy(&pdriveidarray), ::core::mem::transmute_copy(&lnumberofdrives), ::core::mem::transmute(&pwszunmaskinglist), ::core::mem::transmute_copy(&phints2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryMaxLunCreateSize2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsSubSystem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, pdriveidarray: *const ::windows_core::GUID, lnumberofdrives: i32, phints2: *const VDS_HINTS2, pullmaxlunsize: *mut u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryMaxLunCreateSize2(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdriveidarray), ::core::mem::transmute_copy(&lnumberofdrives), ::core::mem::transmute_copy(&phints2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullmaxlunsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProperties2: GetProperties2::<Identity, Impl, OFFSET>,
            GetDrive2: GetDrive2::<Identity, Impl, OFFSET>,
            CreateLun2: CreateLun2::<Identity, Impl, OFFSET>,
            QueryMaxLunCreateSize2: QueryMaxLunCreateSize2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsSubSystem2 as ::windows_core::Interface>::IID
    }
}
pub trait IVdsSubSystemInterconnect_Impl: Sized {
    fn GetSupportedInterconnects(&self) -> ::windows_core::Result<u32>;
}
impl ::windows_core::RuntimeName for IVdsSubSystemInterconnect {}
impl IVdsSubSystemInterconnect_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsSubSystemInterconnect_Impl, const OFFSET: isize>() -> IVdsSubSystemInterconnect_Vtbl {
        unsafe extern "system" fn GetSupportedInterconnects<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsSubSystemInterconnect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulsupportedinterconnectsflag: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSupportedInterconnects() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulsupportedinterconnectsflag, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(), GetSupportedInterconnects: GetSupportedInterconnects::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsSubSystemInterconnect as ::windows_core::Interface>::IID
    }
}
pub trait IVdsSubSystemIscsi_Impl: Sized {
    fn QueryTargets(&self) -> ::windows_core::Result<IEnumVdsObject>;
    fn QueryPortals(&self) -> ::windows_core::Result<IEnumVdsObject>;
    fn CreateTarget(&self, pwsziscsiname: &::windows_core::PCWSTR, pwszfriendlyname: &::windows_core::PCWSTR) -> ::windows_core::Result<IVdsAsync>;
    fn SetIpsecGroupPresharedKey(&self, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IVdsSubSystemIscsi {}
impl IVdsSubSystemIscsi_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsSubSystemIscsi_Impl, const OFFSET: isize>() -> IVdsSubSystemIscsi_Vtbl {
        unsafe extern "system" fn QueryTargets<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsSubSystemIscsi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryTargets() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryPortals<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsSubSystemIscsi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryPortals() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTarget<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsSubSystemIscsi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsziscsiname: ::windows_core::PCWSTR, pwszfriendlyname: ::windows_core::PCWSTR, ppasync: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTarget(::core::mem::transmute(&pwsziscsiname), ::core::mem::transmute(&pwszfriendlyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpsecGroupPresharedKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsSubSystemIscsi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIpsecGroupPresharedKey(::core::mem::transmute_copy(&pipseckey)).into()
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            QueryTargets: QueryTargets::<Identity, Impl, OFFSET>,
            QueryPortals: QueryPortals::<Identity, Impl, OFFSET>,
            CreateTarget: CreateTarget::<Identity, Impl, OFFSET>,
            SetIpsecGroupPresharedKey: SetIpsecGroupPresharedKey::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsSubSystemIscsi as ::windows_core::Interface>::IID
    }
}
pub trait IVdsSubSystemNaming_Impl: Sized {
    fn SetFriendlyName(&self, pwszfriendlyname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IVdsSubSystemNaming {}
impl IVdsSubSystemNaming_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsSubSystemNaming_Impl, const OFFSET: isize>() -> IVdsSubSystemNaming_Vtbl {
        unsafe extern "system" fn SetFriendlyName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVdsSubSystemNaming_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfriendlyname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFriendlyName(::core::mem::transmute(&pwszfriendlyname)).into()
        }
        Self { base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(), SetFriendlyName: SetFriendlyName::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsSubSystemNaming as ::windows_core::Interface>::IID
    }
}
