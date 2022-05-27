#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintDocumentPackageStatusEvent_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn PackageStatusUpdated(&self, packagestatus: *const PrintDocumentPackageStatus) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows_core::RuntimeName for IPrintDocumentPackageStatusEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintDocumentPackageStatusEvent_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintDocumentPackageStatusEvent_Impl, const OFFSET: isize>() -> IPrintDocumentPackageStatusEvent_Vtbl {
        unsafe extern "system" fn PackageStatusUpdated<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintDocumentPackageStatusEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagestatus: *const PrintDocumentPackageStatus) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PackageStatusUpdated(::core::mem::transmute_copy(&packagestatus)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            PackageStatusUpdated: PackageStatusUpdated::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintDocumentPackageStatusEvent as ::windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
pub trait IPrintDocumentPackageTarget_Impl: Sized {
    fn GetPackageTargetTypes(&self, targetcount: *mut u32, targettypes: *mut *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetPackageTarget(&self, guidtargettype: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppvtarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Cancel(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPrintDocumentPackageTarget {}
impl IPrintDocumentPackageTarget_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintDocumentPackageTarget_Impl, const OFFSET: isize>() -> IPrintDocumentPackageTarget_Vtbl {
        unsafe extern "system" fn GetPackageTargetTypes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintDocumentPackageTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetcount: *mut u32, targettypes: *mut *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPackageTargetTypes(::core::mem::transmute_copy(&targetcount), ::core::mem::transmute_copy(&targettypes)).into()
        }
        unsafe extern "system" fn GetPackageTarget<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintDocumentPackageTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtargettype: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppvtarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPackageTarget(::core::mem::transmute_copy(&guidtargettype), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvtarget)).into()
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintDocumentPackageTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel().into()
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetPackageTargetTypes: GetPackageTargetTypes::<Identity, Impl, OFFSET>,
            GetPackageTarget: GetPackageTarget::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintDocumentPackageTarget as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintDocumentPackageTargetFactory_Impl: Sized {
    fn CreateDocumentPackageTargetForPrintJob(&self, printername: &::windows_core::PCWSTR, jobname: &::windows_core::PCWSTR, joboutputstream: &::core::option::Option<super::super::super::System::Com::IStream>, jobprintticketstream: &::core::option::Option<super::super::super::System::Com::IStream>) -> ::windows_core::Result<IPrintDocumentPackageTarget>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IPrintDocumentPackageTargetFactory {}
#[cfg(feature = "Win32_System_Com")]
impl IPrintDocumentPackageTargetFactory_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintDocumentPackageTargetFactory_Impl, const OFFSET: isize>() -> IPrintDocumentPackageTargetFactory_Vtbl {
        unsafe extern "system" fn CreateDocumentPackageTargetForPrintJob<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintDocumentPackageTargetFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printername: ::windows_core::PCWSTR, jobname: ::windows_core::PCWSTR, joboutputstream: ::windows_core::RawPtr, jobprintticketstream: ::windows_core::RawPtr, docpackagetarget: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDocumentPackageTargetForPrintJob(::core::mem::transmute(&printername), ::core::mem::transmute(&jobname), ::core::mem::transmute(&joboutputstream), ::core::mem::transmute(&jobprintticketstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(docpackagetarget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateDocumentPackageTargetForPrintJob: CreateDocumentPackageTargetForPrintJob::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintDocumentPackageTargetFactory as ::windows_core::Interface>::IID
    }
}
pub trait IXpsPrintJob_Impl: Sized {
    fn Cancel(&self) -> ::windows_core::Result<()>;
    fn GetJobStatus(&self) -> ::windows_core::Result<XPS_JOB_STATUS>;
}
impl ::windows_core::RuntimeName for IXpsPrintJob {}
impl IXpsPrintJob_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsPrintJob_Impl, const OFFSET: isize>() -> IXpsPrintJob_Vtbl {
        unsafe extern "system" fn Cancel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel().into()
        }
        unsafe extern "system" fn GetJobStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobstatus: *mut XPS_JOB_STATUS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetJobStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(jobstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            GetJobStatus: GetJobStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsPrintJob as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXpsPrintJobStream_Impl: Sized + super::super::super::System::Com::ISequentialStream_Impl {
    fn Close(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IXpsPrintJobStream {}
#[cfg(feature = "Win32_System_Com")]
impl IXpsPrintJobStream_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsPrintJobStream_Impl, const OFFSET: isize>() -> IXpsPrintJobStream_Vtbl {
        unsafe extern "system" fn Close<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXpsPrintJobStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        Self { base__: super::super::super::System::Com::ISequentialStream_Vtbl::new::<Identity, Impl, OFFSET>(), Close: Close::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsPrintJobStream as ::windows_core::Interface>::IID || iid == &<super::super::super::System::Com::ISequentialStream as ::windows_core::Interface>::IID
    }
}
