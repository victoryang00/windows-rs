pub trait IAnimationObject_Impl: Sized {
    fn PopulatePropertyInfo(&self, propertyname: &::windows_core::HSTRING, propertyinfo: &::core::option::Option<AnimationPropertyInfo>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IAnimationObject {
    const NAME: &'static str = "Windows.UI.Composition.IAnimationObject";
}
impl IAnimationObject_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAnimationObject_Impl, const OFFSET: isize>() -> IAnimationObject_Vtbl {
        unsafe extern "system" fn PopulatePropertyInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAnimationObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, propertyinfo: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PopulatePropertyInfo(::core::mem::transmute(&propertyname), ::core::mem::transmute(&propertyinfo)).into()
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, IAnimationObject, OFFSET>(),
            PopulatePropertyInfo: PopulatePropertyInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAnimationObject as ::windows_core::Interface>::IID
    }
}
pub trait ICompositionAnimationBase_Impl: Sized {}
impl ::windows_core::RuntimeName for ICompositionAnimationBase {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionAnimationBase";
}
impl ICompositionAnimationBase_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionAnimationBase_Impl, const OFFSET: isize>() -> ICompositionAnimationBase_Vtbl {
        Self { base__: ::windows_core::IInspectableVtbl::new::<Identity, ICompositionAnimationBase, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionAnimationBase as ::windows_core::Interface>::IID
    }
}
pub trait ICompositionSupportsSystemBackdrop_Impl: Sized {
    fn SystemBackdrop(&self) -> ::windows_core::Result<CompositionBrush>;
    fn SetSystemBackdrop(&self, value: &::core::option::Option<CompositionBrush>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ICompositionSupportsSystemBackdrop {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionSupportsSystemBackdrop";
}
impl ICompositionSupportsSystemBackdrop_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionSupportsSystemBackdrop_Impl, const OFFSET: isize>() -> ICompositionSupportsSystemBackdrop_Vtbl {
        unsafe extern "system" fn SystemBackdrop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionSupportsSystemBackdrop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SystemBackdrop() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSystemBackdrop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionSupportsSystemBackdrop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSystemBackdrop(::core::mem::transmute(&value)).into()
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, ICompositionSupportsSystemBackdrop, OFFSET>(),
            SystemBackdrop: SystemBackdrop::<Identity, Impl, OFFSET>,
            SetSystemBackdrop: SetSystemBackdrop::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionSupportsSystemBackdrop as ::windows_core::Interface>::IID
    }
}
pub trait ICompositionSurface_Impl: Sized {}
impl ::windows_core::RuntimeName for ICompositionSurface {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionSurface";
}
impl ICompositionSurface_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionSurface_Impl, const OFFSET: isize>() -> ICompositionSurface_Vtbl {
        Self { base__: ::windows_core::IInspectableVtbl::new::<Identity, ICompositionSurface, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionSurface as ::windows_core::Interface>::IID
    }
}
pub trait ICompositionSurfaceFacade_Impl: Sized {
    fn GetRealSurface(&self) -> ::windows_core::Result<ICompositionSurface>;
}
impl ::windows_core::RuntimeName for ICompositionSurfaceFacade {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionSurfaceFacade";
}
impl ICompositionSurfaceFacade_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionSurfaceFacade_Impl, const OFFSET: isize>() -> ICompositionSurfaceFacade_Vtbl {
        unsafe extern "system" fn GetRealSurface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionSurfaceFacade_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRealSurface() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, ICompositionSurfaceFacade, OFFSET>(),
            GetRealSurface: GetRealSurface::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionSurfaceFacade as ::windows_core::Interface>::IID
    }
}
pub trait IVisualElement_Impl: Sized {}
impl ::windows_core::RuntimeName for IVisualElement {
    const NAME: &'static str = "Windows.UI.Composition.IVisualElement";
}
impl IVisualElement_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVisualElement_Impl, const OFFSET: isize>() -> IVisualElement_Vtbl {
        Self { base__: ::windows_core::IInspectableVtbl::new::<Identity, IVisualElement, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVisualElement as ::windows_core::Interface>::IID
    }
}
pub trait IVisualElement2_Impl: Sized {
    fn GetVisualInternal(&self) -> ::windows_core::Result<Visual>;
}
impl ::windows_core::RuntimeName for IVisualElement2 {
    const NAME: &'static str = "Windows.UI.Composition.IVisualElement2";
}
impl IVisualElement2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVisualElement2_Impl, const OFFSET: isize>() -> IVisualElement2_Vtbl {
        unsafe extern "system" fn GetVisualInternal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVisualElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVisualInternal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, IVisualElement2, OFFSET>(),
            GetVisualInternal: GetVisualInternal::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVisualElement2 as ::windows_core::Interface>::IID
    }
}
