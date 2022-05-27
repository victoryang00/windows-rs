#[cfg(feature = "Win32_Foundation")]
pub trait IDesktopWindowXamlSourceNative_Impl: Sized {
    fn AttachToWindow(&self, parentwnd: super::super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn WindowHandle(&self) -> ::windows_core::Result<super::super::super::Foundation::HWND>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IDesktopWindowXamlSourceNative {}
#[cfg(feature = "Win32_Foundation")]
impl IDesktopWindowXamlSourceNative_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDesktopWindowXamlSourceNative_Impl, const OFFSET: isize>() -> IDesktopWindowXamlSourceNative_Vtbl {
        unsafe extern "system" fn AttachToWindow<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDesktopWindowXamlSourceNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parentwnd: super::super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AttachToWindow(::core::mem::transmute_copy(&parentwnd)).into()
        }
        unsafe extern "system" fn WindowHandle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDesktopWindowXamlSourceNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: *mut super::super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WindowHandle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hwnd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AttachToWindow: AttachToWindow::<Identity, Impl, OFFSET>,
            WindowHandle: WindowHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDesktopWindowXamlSourceNative as ::windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDesktopWindowXamlSourceNative2_Impl: Sized + IDesktopWindowXamlSourceNative_Impl {
    fn PreTranslateMessage(&self, message: *const super::super::super::UI::WindowsAndMessaging::MSG, result: *mut super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::RuntimeName for IDesktopWindowXamlSourceNative2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IDesktopWindowXamlSourceNative2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDesktopWindowXamlSourceNative2_Impl, const OFFSET: isize>() -> IDesktopWindowXamlSourceNative2_Vtbl {
        unsafe extern "system" fn PreTranslateMessage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDesktopWindowXamlSourceNative2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: *const super::super::super::UI::WindowsAndMessaging::MSG, result: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PreTranslateMessage(::core::mem::transmute_copy(&message), ::core::mem::transmute_copy(&result)).into()
        }
        Self { base__: IDesktopWindowXamlSourceNative_Vtbl::new::<Identity, Impl, OFFSET>(), PreTranslateMessage: PreTranslateMessage::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDesktopWindowXamlSourceNative2 as ::windows_core::Interface>::IID || iid == &<IDesktopWindowXamlSourceNative as ::windows_core::Interface>::IID
    }
}
pub trait IFindReferenceTargetsCallback_Impl: Sized {
    fn FoundTrackerTarget(&self, target: &::core::option::Option<IReferenceTrackerTarget>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IFindReferenceTargetsCallback {}
impl IFindReferenceTargetsCallback_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFindReferenceTargetsCallback_Impl, const OFFSET: isize>() -> IFindReferenceTargetsCallback_Vtbl {
        unsafe extern "system" fn FoundTrackerTarget<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFindReferenceTargetsCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FoundTrackerTarget(::core::mem::transmute(&target)).into()
        }
        Self { base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(), FoundTrackerTarget: FoundTrackerTarget::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFindReferenceTargetsCallback as ::windows_core::Interface>::IID
    }
}
pub trait IReferenceTracker_Impl: Sized {
    fn ConnectFromTrackerSource(&self) -> ::windows_core::Result<()>;
    fn DisconnectFromTrackerSource(&self) -> ::windows_core::Result<()>;
    fn FindTrackerTargets(&self, callback: &::core::option::Option<IFindReferenceTargetsCallback>) -> ::windows_core::Result<()>;
    fn GetReferenceTrackerManager(&self) -> ::windows_core::Result<IReferenceTrackerManager>;
    fn AddRefFromTrackerSource(&self) -> ::windows_core::Result<()>;
    fn ReleaseFromTrackerSource(&self) -> ::windows_core::Result<()>;
    fn PegFromTrackerSource(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IReferenceTracker {}
impl IReferenceTracker_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceTracker_Impl, const OFFSET: isize>() -> IReferenceTracker_Vtbl {
        unsafe extern "system" fn ConnectFromTrackerSource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConnectFromTrackerSource().into()
        }
        unsafe extern "system" fn DisconnectFromTrackerSource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisconnectFromTrackerSource().into()
        }
        unsafe extern "system" fn FindTrackerTargets<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callback: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindTrackerTargets(::core::mem::transmute(&callback)).into()
        }
        unsafe extern "system" fn GetReferenceTrackerManager<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReferenceTrackerManager() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRefFromTrackerSource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddRefFromTrackerSource().into()
        }
        unsafe extern "system" fn ReleaseFromTrackerSource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseFromTrackerSource().into()
        }
        unsafe extern "system" fn PegFromTrackerSource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PegFromTrackerSource().into()
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ConnectFromTrackerSource: ConnectFromTrackerSource::<Identity, Impl, OFFSET>,
            DisconnectFromTrackerSource: DisconnectFromTrackerSource::<Identity, Impl, OFFSET>,
            FindTrackerTargets: FindTrackerTargets::<Identity, Impl, OFFSET>,
            GetReferenceTrackerManager: GetReferenceTrackerManager::<Identity, Impl, OFFSET>,
            AddRefFromTrackerSource: AddRefFromTrackerSource::<Identity, Impl, OFFSET>,
            ReleaseFromTrackerSource: ReleaseFromTrackerSource::<Identity, Impl, OFFSET>,
            PegFromTrackerSource: PegFromTrackerSource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReferenceTracker as ::windows_core::Interface>::IID
    }
}
pub trait IReferenceTrackerExtension_Impl: Sized {}
impl ::windows_core::RuntimeName for IReferenceTrackerExtension {}
impl IReferenceTrackerExtension_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceTrackerExtension_Impl, const OFFSET: isize>() -> IReferenceTrackerExtension_Vtbl {
        Self { base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReferenceTrackerExtension as ::windows_core::Interface>::IID
    }
}
pub trait IReferenceTrackerHost_Impl: Sized {
    fn DisconnectUnusedReferenceSources(&self, options: XAML_REFERENCETRACKER_DISCONNECT) -> ::windows_core::Result<()>;
    fn ReleaseDisconnectedReferenceSources(&self) -> ::windows_core::Result<()>;
    fn NotifyEndOfReferenceTrackingOnThread(&self) -> ::windows_core::Result<()>;
    fn GetTrackerTarget(&self, unknown: &::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<IReferenceTrackerTarget>;
    fn AddMemoryPressure(&self, bytesallocated: u64) -> ::windows_core::Result<()>;
    fn RemoveMemoryPressure(&self, bytesallocated: u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IReferenceTrackerHost {}
impl IReferenceTrackerHost_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceTrackerHost_Impl, const OFFSET: isize>() -> IReferenceTrackerHost_Vtbl {
        unsafe extern "system" fn DisconnectUnusedReferenceSources<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceTrackerHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: XAML_REFERENCETRACKER_DISCONNECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisconnectUnusedReferenceSources(::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn ReleaseDisconnectedReferenceSources<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceTrackerHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseDisconnectedReferenceSources().into()
        }
        unsafe extern "system" fn NotifyEndOfReferenceTrackingOnThread<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceTrackerHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotifyEndOfReferenceTrackingOnThread().into()
        }
        unsafe extern "system" fn GetTrackerTarget<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceTrackerHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unknown: *mut ::core::ffi::c_void, newreference: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTrackerTarget(::core::mem::transmute(&unknown)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newreference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddMemoryPressure<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceTrackerHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bytesallocated: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddMemoryPressure(::core::mem::transmute_copy(&bytesallocated)).into()
        }
        unsafe extern "system" fn RemoveMemoryPressure<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceTrackerHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bytesallocated: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveMemoryPressure(::core::mem::transmute_copy(&bytesallocated)).into()
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            DisconnectUnusedReferenceSources: DisconnectUnusedReferenceSources::<Identity, Impl, OFFSET>,
            ReleaseDisconnectedReferenceSources: ReleaseDisconnectedReferenceSources::<Identity, Impl, OFFSET>,
            NotifyEndOfReferenceTrackingOnThread: NotifyEndOfReferenceTrackingOnThread::<Identity, Impl, OFFSET>,
            GetTrackerTarget: GetTrackerTarget::<Identity, Impl, OFFSET>,
            AddMemoryPressure: AddMemoryPressure::<Identity, Impl, OFFSET>,
            RemoveMemoryPressure: RemoveMemoryPressure::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReferenceTrackerHost as ::windows_core::Interface>::IID
    }
}
pub trait IReferenceTrackerManager_Impl: Sized {
    fn ReferenceTrackingStarted(&self) -> ::windows_core::Result<()>;
    fn FindTrackerTargetsCompleted(&self, findfailed: u8) -> ::windows_core::Result<()>;
    fn ReferenceTrackingCompleted(&self) -> ::windows_core::Result<()>;
    fn SetReferenceTrackerHost(&self, value: &::core::option::Option<IReferenceTrackerHost>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IReferenceTrackerManager {}
impl IReferenceTrackerManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceTrackerManager_Impl, const OFFSET: isize>() -> IReferenceTrackerManager_Vtbl {
        unsafe extern "system" fn ReferenceTrackingStarted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceTrackerManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReferenceTrackingStarted().into()
        }
        unsafe extern "system" fn FindTrackerTargetsCompleted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceTrackerManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, findfailed: u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindTrackerTargetsCompleted(::core::mem::transmute_copy(&findfailed)).into()
        }
        unsafe extern "system" fn ReferenceTrackingCompleted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceTrackerManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReferenceTrackingCompleted().into()
        }
        unsafe extern "system" fn SetReferenceTrackerHost<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceTrackerManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetReferenceTrackerHost(::core::mem::transmute(&value)).into()
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ReferenceTrackingStarted: ReferenceTrackingStarted::<Identity, Impl, OFFSET>,
            FindTrackerTargetsCompleted: FindTrackerTargetsCompleted::<Identity, Impl, OFFSET>,
            ReferenceTrackingCompleted: ReferenceTrackingCompleted::<Identity, Impl, OFFSET>,
            SetReferenceTrackerHost: SetReferenceTrackerHost::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReferenceTrackerManager as ::windows_core::Interface>::IID
    }
}
pub trait IReferenceTrackerTarget_Impl: Sized {
    fn AddRefFromReferenceTracker(&self) -> u32;
    fn ReleaseFromReferenceTracker(&self) -> u32;
    fn Peg(&self) -> ::windows_core::Result<()>;
    fn Unpeg(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IReferenceTrackerTarget {}
impl IReferenceTrackerTarget_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceTrackerTarget_Impl, const OFFSET: isize>() -> IReferenceTrackerTarget_Vtbl {
        unsafe extern "system" fn AddRefFromReferenceTracker<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceTrackerTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddRefFromReferenceTracker()
        }
        unsafe extern "system" fn ReleaseFromReferenceTracker<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceTrackerTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseFromReferenceTracker()
        }
        unsafe extern "system" fn Peg<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceTrackerTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Peg().into()
        }
        unsafe extern "system" fn Unpeg<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReferenceTrackerTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unpeg().into()
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddRefFromReferenceTracker: AddRefFromReferenceTracker::<Identity, Impl, OFFSET>,
            ReleaseFromReferenceTracker: ReleaseFromReferenceTracker::<Identity, Impl, OFFSET>,
            Peg: Peg::<Identity, Impl, OFFSET>,
            Unpeg: Unpeg::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReferenceTrackerTarget as ::windows_core::Interface>::IID
    }
}
pub trait ISurfaceImageSourceManagerNative_Impl: Sized {
    fn FlushAllSurfacesWithDevice(&self, device: &::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISurfaceImageSourceManagerNative {}
impl ISurfaceImageSourceManagerNative_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISurfaceImageSourceManagerNative_Impl, const OFFSET: isize>() -> ISurfaceImageSourceManagerNative_Vtbl {
        unsafe extern "system" fn FlushAllSurfacesWithDevice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISurfaceImageSourceManagerNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FlushAllSurfacesWithDevice(::core::mem::transmute(&device)).into()
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            FlushAllSurfacesWithDevice: FlushAllSurfacesWithDevice::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISurfaceImageSourceManagerNative as ::windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
pub trait ISurfaceImageSourceNative_Impl: Sized {
    fn SetDevice(&self, device: &::core::option::Option<super::super::super::Graphics::Dxgi::IDXGIDevice>) -> ::windows_core::Result<()>;
    fn BeginDraw(&self, updaterect: &super::super::super::Foundation::RECT, surface: *mut ::core::option::Option<super::super::super::Graphics::Dxgi::IDXGISurface>, offset: *mut super::super::super::Foundation::POINT) -> ::windows_core::Result<()>;
    fn EndDraw(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
impl ::windows_core::RuntimeName for ISurfaceImageSourceNative {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
impl ISurfaceImageSourceNative_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISurfaceImageSourceNative_Impl, const OFFSET: isize>() -> ISurfaceImageSourceNative_Vtbl {
        unsafe extern "system" fn SetDevice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISurfaceImageSourceNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, device: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDevice(::core::mem::transmute(&device)).into()
        }
        unsafe extern "system" fn BeginDraw<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISurfaceImageSourceNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updaterect: super::super::super::Foundation::RECT, surface: *mut ::windows_core::RawPtr, offset: *mut super::super::super::Foundation::POINT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginDraw(::core::mem::transmute(&updaterect), ::core::mem::transmute_copy(&surface), ::core::mem::transmute_copy(&offset)).into()
        }
        unsafe extern "system" fn EndDraw<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISurfaceImageSourceNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDraw().into()
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetDevice: SetDevice::<Identity, Impl, OFFSET>,
            BeginDraw: BeginDraw::<Identity, Impl, OFFSET>,
            EndDraw: EndDraw::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISurfaceImageSourceNative as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISurfaceImageSourceNativeWithD2D_Impl: Sized {
    fn SetDevice(&self, device: &::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn BeginDraw(&self, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows_core::GUID, updateobject: *mut *mut ::core::ffi::c_void, offset: *mut super::super::super::Foundation::POINT) -> ::windows_core::Result<()>;
    fn EndDraw(&self) -> ::windows_core::Result<()>;
    fn SuspendDraw(&self) -> ::windows_core::Result<()>;
    fn ResumeDraw(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ISurfaceImageSourceNativeWithD2D {}
#[cfg(feature = "Win32_Foundation")]
impl ISurfaceImageSourceNativeWithD2D_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISurfaceImageSourceNativeWithD2D_Impl, const OFFSET: isize>() -> ISurfaceImageSourceNativeWithD2D_Vtbl {
        unsafe extern "system" fn SetDevice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISurfaceImageSourceNativeWithD2D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDevice(::core::mem::transmute(&device)).into()
        }
        unsafe extern "system" fn BeginDraw<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISurfaceImageSourceNativeWithD2D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows_core::GUID, updateobject: *mut *mut ::core::ffi::c_void, offset: *mut super::super::super::Foundation::POINT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginDraw(::core::mem::transmute_copy(&updaterect), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&updateobject), ::core::mem::transmute_copy(&offset)).into()
        }
        unsafe extern "system" fn EndDraw<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISurfaceImageSourceNativeWithD2D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDraw().into()
        }
        unsafe extern "system" fn SuspendDraw<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISurfaceImageSourceNativeWithD2D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SuspendDraw().into()
        }
        unsafe extern "system" fn ResumeDraw<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISurfaceImageSourceNativeWithD2D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResumeDraw().into()
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetDevice: SetDevice::<Identity, Impl, OFFSET>,
            BeginDraw: BeginDraw::<Identity, Impl, OFFSET>,
            EndDraw: EndDraw::<Identity, Impl, OFFSET>,
            SuspendDraw: SuspendDraw::<Identity, Impl, OFFSET>,
            ResumeDraw: ResumeDraw::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISurfaceImageSourceNativeWithD2D as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub trait ISwapChainBackgroundPanelNative_Impl: Sized {
    fn SetSwapChain(&self, swapchain: &::core::option::Option<super::super::super::Graphics::Dxgi::IDXGISwapChain>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl ::windows_core::RuntimeName for ISwapChainBackgroundPanelNative {}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl ISwapChainBackgroundPanelNative_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISwapChainBackgroundPanelNative_Impl, const OFFSET: isize>() -> ISwapChainBackgroundPanelNative_Vtbl {
        unsafe extern "system" fn SetSwapChain<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISwapChainBackgroundPanelNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, swapchain: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSwapChain(::core::mem::transmute(&swapchain)).into()
        }
        Self { base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(), SetSwapChain: SetSwapChain::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISwapChainBackgroundPanelNative as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub trait ISwapChainPanelNative_Impl: Sized {
    fn SetSwapChain(&self, swapchain: &::core::option::Option<super::super::super::Graphics::Dxgi::IDXGISwapChain>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl ::windows_core::RuntimeName for ISwapChainPanelNative {}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl ISwapChainPanelNative_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISwapChainPanelNative_Impl, const OFFSET: isize>() -> ISwapChainPanelNative_Vtbl {
        unsafe extern "system" fn SetSwapChain<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISwapChainPanelNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, swapchain: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSwapChain(::core::mem::transmute(&swapchain)).into()
        }
        Self { base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(), SetSwapChain: SetSwapChain::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISwapChainPanelNative as ::windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
pub trait ISwapChainPanelNative2_Impl: Sized + ISwapChainPanelNative_Impl {
    fn SetSwapChainHandle(&self, swapchainhandle: super::super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
impl ::windows_core::RuntimeName for ISwapChainPanelNative2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
impl ISwapChainPanelNative2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISwapChainPanelNative2_Impl, const OFFSET: isize>() -> ISwapChainPanelNative2_Vtbl {
        unsafe extern "system" fn SetSwapChainHandle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISwapChainPanelNative2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, swapchainhandle: super::super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSwapChainHandle(::core::mem::transmute_copy(&swapchainhandle)).into()
        }
        Self { base__: ISwapChainPanelNative_Vtbl::new::<Identity, Impl, OFFSET>(), SetSwapChainHandle: SetSwapChainHandle::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISwapChainPanelNative2 as ::windows_core::Interface>::IID || iid == &<ISwapChainPanelNative as ::windows_core::Interface>::IID
    }
}
pub trait ITrackerOwner_Impl: Sized {
    fn CreateTrackerHandle(&self) -> ::windows_core::Result<*mut TrackerHandle__>;
    fn DeleteTrackerHandle(&self, handle: *const TrackerHandle__) -> ::windows_core::Result<()>;
    fn SetTrackerValue(&self, handle: *const TrackerHandle__, value: &::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn TryGetSafeTrackerValue(&self, handle: *const TrackerHandle__, returnvalue: *mut ::core::option::Option<::windows_core::IUnknown>) -> u8;
}
impl ::windows_core::RuntimeName for ITrackerOwner {}
impl ITrackerOwner_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITrackerOwner_Impl, const OFFSET: isize>() -> ITrackerOwner_Vtbl {
        unsafe extern "system" fn CreateTrackerHandle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITrackerOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, returnvalue: *mut *mut TrackerHandle__) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTrackerHandle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(returnvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteTrackerHandle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITrackerOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: *const TrackerHandle__) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteTrackerHandle(::core::mem::transmute_copy(&handle)).into()
        }
        unsafe extern "system" fn SetTrackerValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITrackerOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: *const TrackerHandle__, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTrackerValue(::core::mem::transmute_copy(&handle), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn TryGetSafeTrackerValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITrackerOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: *const TrackerHandle__, returnvalue: *mut *mut ::core::ffi::c_void) -> u8 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TryGetSafeTrackerValue(::core::mem::transmute_copy(&handle), ::core::mem::transmute_copy(&returnvalue))
        }
        Self {
            base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateTrackerHandle: CreateTrackerHandle::<Identity, Impl, OFFSET>,
            DeleteTrackerHandle: DeleteTrackerHandle::<Identity, Impl, OFFSET>,
            SetTrackerValue: SetTrackerValue::<Identity, Impl, OFFSET>,
            TryGetSafeTrackerValue: TryGetSafeTrackerValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITrackerOwner as ::windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
pub trait IVirtualSurfaceImageSourceNative_Impl: Sized + ISurfaceImageSourceNative_Impl {
    fn Invalidate(&self, updaterect: &super::super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn GetUpdateRectCount(&self) -> ::windows_core::Result<u32>;
    fn GetUpdateRects(&self, updates: *mut super::super::super::Foundation::RECT, count: u32) -> ::windows_core::Result<()>;
    fn GetVisibleBounds(&self) -> ::windows_core::Result<super::super::super::Foundation::RECT>;
    fn RegisterForUpdatesNeeded(&self, callback: &::core::option::Option<IVirtualSurfaceUpdatesCallbackNative>) -> ::windows_core::Result<()>;
    fn Resize(&self, newwidth: i32, newheight: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
impl ::windows_core::RuntimeName for IVirtualSurfaceImageSourceNative {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
impl IVirtualSurfaceImageSourceNative_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVirtualSurfaceImageSourceNative_Impl, const OFFSET: isize>() -> IVirtualSurfaceImageSourceNative_Vtbl {
        unsafe extern "system" fn Invalidate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVirtualSurfaceImageSourceNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updaterect: super::super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Invalidate(::core::mem::transmute(&updaterect)).into()
        }
        unsafe extern "system" fn GetUpdateRectCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVirtualSurfaceImageSourceNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUpdateRectCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateRects<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVirtualSurfaceImageSourceNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updates: *mut super::super::super::Foundation::RECT, count: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetUpdateRects(::core::mem::transmute_copy(&updates), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetVisibleBounds<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVirtualSurfaceImageSourceNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bounds: *mut super::super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVisibleBounds() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bounds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterForUpdatesNeeded<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVirtualSurfaceImageSourceNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callback: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterForUpdatesNeeded(::core::mem::transmute(&callback)).into()
        }
        unsafe extern "system" fn Resize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVirtualSurfaceImageSourceNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newwidth: i32, newheight: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Resize(::core::mem::transmute_copy(&newwidth), ::core::mem::transmute_copy(&newheight)).into()
        }
        Self {
            base__: ISurfaceImageSourceNative_Vtbl::new::<Identity, Impl, OFFSET>(),
            Invalidate: Invalidate::<Identity, Impl, OFFSET>,
            GetUpdateRectCount: GetUpdateRectCount::<Identity, Impl, OFFSET>,
            GetUpdateRects: GetUpdateRects::<Identity, Impl, OFFSET>,
            GetVisibleBounds: GetVisibleBounds::<Identity, Impl, OFFSET>,
            RegisterForUpdatesNeeded: RegisterForUpdatesNeeded::<Identity, Impl, OFFSET>,
            Resize: Resize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVirtualSurfaceImageSourceNative as ::windows_core::Interface>::IID || iid == &<ISurfaceImageSourceNative as ::windows_core::Interface>::IID
    }
}
pub trait IVirtualSurfaceUpdatesCallbackNative_Impl: Sized {
    fn UpdatesNeeded(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IVirtualSurfaceUpdatesCallbackNative {}
impl IVirtualSurfaceUpdatesCallbackNative_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVirtualSurfaceUpdatesCallbackNative_Impl, const OFFSET: isize>() -> IVirtualSurfaceUpdatesCallbackNative_Vtbl {
        unsafe extern "system" fn UpdatesNeeded<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVirtualSurfaceUpdatesCallbackNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdatesNeeded().into()
        }
        Self { base__: ::windows_core::IUnknownVtbl::new::<Identity, OFFSET>(), UpdatesNeeded: UpdatesNeeded::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVirtualSurfaceUpdatesCallbackNative as ::windows_core::Interface>::IID
    }
}
