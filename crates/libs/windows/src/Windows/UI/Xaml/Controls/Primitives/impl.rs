pub trait IFlyoutBaseOverrides_Impl: Sized {
    fn CreatePresenter(&self) -> ::windows_core::Result<super::Control>;
}
impl ::windows_core::RuntimeName for IFlyoutBaseOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBaseOverrides";
}
impl IFlyoutBaseOverrides_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFlyoutBaseOverrides_Impl, const OFFSET: isize>() -> IFlyoutBaseOverrides_Vtbl {
        unsafe extern "system" fn CreatePresenter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFlyoutBaseOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePresenter() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, IFlyoutBaseOverrides, OFFSET>(),
            CreatePresenter: CreatePresenter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFlyoutBaseOverrides as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Input")]
pub trait IFlyoutBaseOverrides4_Impl: Sized {
    fn OnProcessKeyboardAccelerators(&self, args: &::core::option::Option<super::super::Input::ProcessKeyboardAcceleratorEventArgs>) -> ::windows_core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
#[cfg(feature = "UI_Xaml_Input")]
impl ::windows_core::RuntimeName for IFlyoutBaseOverrides4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBaseOverrides4";
}
#[cfg(feature = "UI_Xaml_Input")]
impl IFlyoutBaseOverrides4_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFlyoutBaseOverrides4_Impl, const OFFSET: isize>() -> IFlyoutBaseOverrides4_Vtbl {
        unsafe extern "system" fn OnProcessKeyboardAccelerators<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFlyoutBaseOverrides4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnProcessKeyboardAccelerators(::core::mem::transmute(&args)).into()
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, IFlyoutBaseOverrides4, OFFSET>(),
            OnProcessKeyboardAccelerators: OnProcessKeyboardAccelerators::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFlyoutBaseOverrides4 as ::windows_core::Interface>::IID
    }
}
pub trait IPickerFlyoutBaseOverrides_Impl: Sized {
    fn OnConfirmed(&self) -> ::windows_core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn ShouldShowConfirmationButtons(&self) -> ::windows_core::Result<bool>;
}
impl ::windows_core::RuntimeName for IPickerFlyoutBaseOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPickerFlyoutBaseOverrides";
}
impl IPickerFlyoutBaseOverrides_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPickerFlyoutBaseOverrides_Impl, const OFFSET: isize>() -> IPickerFlyoutBaseOverrides_Vtbl {
        unsafe extern "system" fn OnConfirmed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPickerFlyoutBaseOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnConfirmed().into()
        }
        unsafe extern "system" fn ShouldShowConfirmationButtons<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPickerFlyoutBaseOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ShouldShowConfirmationButtons() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, IPickerFlyoutBaseOverrides, OFFSET>(),
            OnConfirmed: OnConfirmed::<Identity, Impl, OFFSET>,
            ShouldShowConfirmationButtons: ShouldShowConfirmationButtons::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPickerFlyoutBaseOverrides as ::windows_core::Interface>::IID
    }
}
pub trait IRangeBaseOverrides_Impl: Sized {
    fn OnMinimumChanged(&self, oldminimum: f64, newminimum: f64) -> ::windows_core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnMaximumChanged(&self, oldmaximum: f64, newmaximum: f64) -> ::windows_core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnValueChanged(&self, oldvalue: f64, newvalue: f64) -> ::windows_core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
impl ::windows_core::RuntimeName for IRangeBaseOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IRangeBaseOverrides";
}
impl IRangeBaseOverrides_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRangeBaseOverrides_Impl, const OFFSET: isize>() -> IRangeBaseOverrides_Vtbl {
        unsafe extern "system" fn OnMinimumChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRangeBaseOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldminimum: f64, newminimum: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnMinimumChanged(oldminimum, newminimum).into()
        }
        unsafe extern "system" fn OnMaximumChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRangeBaseOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldmaximum: f64, newmaximum: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnMaximumChanged(oldmaximum, newmaximum).into()
        }
        unsafe extern "system" fn OnValueChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRangeBaseOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldvalue: f64, newvalue: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnValueChanged(oldvalue, newvalue).into()
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, IRangeBaseOverrides, OFFSET>(),
            OnMinimumChanged: OnMinimumChanged::<Identity, Impl, OFFSET>,
            OnMaximumChanged: OnMaximumChanged::<Identity, Impl, OFFSET>,
            OnValueChanged: OnValueChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRangeBaseOverrides as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IScrollSnapPointsInfo_Impl: Sized {
    fn AreHorizontalSnapPointsRegular(&self) -> ::windows_core::Result<bool>;
    fn AreVerticalSnapPointsRegular(&self) -> ::windows_core::Result<bool>;
    fn HorizontalSnapPointsChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows_core::IInspectable>>) -> ::windows_core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveHorizontalSnapPointsChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn VerticalSnapPointsChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows_core::IInspectable>>) -> ::windows_core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveVerticalSnapPointsChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn GetIrregularSnapPoints(&self, orientation: super::Orientation, alignment: SnapPointsAlignment) -> ::windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<f32>>;
    fn GetRegularSnapPoints(&self, orientation: super::Orientation, alignment: SnapPointsAlignment, offset: &mut f32) -> ::windows_core::Result<f32>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for IScrollSnapPointsInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IScrollSnapPointsInfo";
}
#[cfg(feature = "Foundation_Collections")]
impl IScrollSnapPointsInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>() -> IScrollSnapPointsInfo_Vtbl {
        unsafe extern "system" fn AreHorizontalSnapPointsRegular<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AreHorizontalSnapPointsRegular() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AreVerticalSnapPointsRegular<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AreVerticalSnapPointsRegular() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalSnapPointsChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HorizontalSnapPointsChanged(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHorizontalSnapPointsChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveHorizontalSnapPointsChanged(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn VerticalSnapPointsChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VerticalSnapPointsChanged(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVerticalSnapPointsChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveVerticalSnapPointsChanged(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn GetIrregularSnapPoints<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, orientation: super::Orientation, alignment: SnapPointsAlignment, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIrregularSnapPoints(orientation, alignment) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegularSnapPoints<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, orientation: super::Orientation, alignment: SnapPointsAlignment, offset: *mut f32, result__: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRegularSnapPoints(orientation, alignment, ::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, IScrollSnapPointsInfo, OFFSET>(),
            AreHorizontalSnapPointsRegular: AreHorizontalSnapPointsRegular::<Identity, Impl, OFFSET>,
            AreVerticalSnapPointsRegular: AreVerticalSnapPointsRegular::<Identity, Impl, OFFSET>,
            HorizontalSnapPointsChanged: HorizontalSnapPointsChanged::<Identity, Impl, OFFSET>,
            RemoveHorizontalSnapPointsChanged: RemoveHorizontalSnapPointsChanged::<Identity, Impl, OFFSET>,
            VerticalSnapPointsChanged: VerticalSnapPointsChanged::<Identity, Impl, OFFSET>,
            RemoveVerticalSnapPointsChanged: RemoveVerticalSnapPointsChanged::<Identity, Impl, OFFSET>,
            GetIrregularSnapPoints: GetIrregularSnapPoints::<Identity, Impl, OFFSET>,
            GetRegularSnapPoints: GetRegularSnapPoints::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IScrollSnapPointsInfo as ::windows_core::Interface>::IID
    }
}
pub trait IToggleButtonOverrides_Impl: Sized {
    fn OnToggle(&self) -> ::windows_core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
impl ::windows_core::RuntimeName for IToggleButtonOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IToggleButtonOverrides";
}
impl IToggleButtonOverrides_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IToggleButtonOverrides_Impl, const OFFSET: isize>() -> IToggleButtonOverrides_Vtbl {
        unsafe extern "system" fn OnToggle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IToggleButtonOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnToggle().into()
        }
        Self { base__: ::windows_core::IInspectableVtbl::new::<Identity, IToggleButtonOverrides, OFFSET>(), OnToggle: OnToggle::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IToggleButtonOverrides as ::windows_core::Interface>::IID
    }
}
