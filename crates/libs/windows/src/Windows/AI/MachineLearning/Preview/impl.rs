#[cfg(feature = "deprecated")]
pub trait ILearningModelVariableDescriptorPreview_Impl: Sized {
    fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn ModelFeatureKind(&self) -> ::windows_core::Result<LearningModelFeatureKindPreview>;
    fn IsRequired(&self) -> ::windows_core::Result<bool>;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for ILearningModelVariableDescriptorPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.ILearningModelVariableDescriptorPreview";
}
#[cfg(feature = "deprecated")]
impl ILearningModelVariableDescriptorPreview_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILearningModelVariableDescriptorPreview_Impl, const OFFSET: isize>() -> ILearningModelVariableDescriptorPreview_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILearningModelVariableDescriptorPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILearningModelVariableDescriptorPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelFeatureKind<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILearningModelVariableDescriptorPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LearningModelFeatureKindPreview) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ModelFeatureKind() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRequired<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILearningModelVariableDescriptorPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsRequired() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, ILearningModelVariableDescriptorPreview, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            ModelFeatureKind: ModelFeatureKind::<Identity, Impl, OFFSET>,
            IsRequired: IsRequired::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILearningModelVariableDescriptorPreview as ::windows_core::Interface>::IID
    }
}
