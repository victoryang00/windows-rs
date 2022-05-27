#[cfg(feature = "Foundation")]
pub trait ICoreDropOperationTarget_Impl: Sized {
    fn EnterAsync(&self, draginfo: &::core::option::Option<CoreDragInfo>, draguioverride: &::core::option::Option<CoreDragUIOverride>) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>;
    fn OverAsync(&self, draginfo: &::core::option::Option<CoreDragInfo>, draguioverride: &::core::option::Option<CoreDragUIOverride>) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>;
    fn LeaveAsync(&self, draginfo: &::core::option::Option<CoreDragInfo>) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn DropAsync(&self, draginfo: &::core::option::Option<CoreDragInfo>) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::RuntimeName for ICoreDropOperationTarget {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDropOperationTarget";
}
#[cfg(feature = "Foundation")]
impl ICoreDropOperationTarget_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreDropOperationTarget_Impl, const OFFSET: isize>() -> ICoreDropOperationTarget_Vtbl {
        unsafe extern "system" fn EnterAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreDropOperationTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, draginfo: ::windows_core::RawPtr, draguioverride: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnterAsync(::core::mem::transmute(&draginfo), ::core::mem::transmute(&draguioverride)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreDropOperationTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, draginfo: ::windows_core::RawPtr, draguioverride: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OverAsync(::core::mem::transmute(&draginfo), ::core::mem::transmute(&draguioverride)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LeaveAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreDropOperationTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, draginfo: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LeaveAsync(::core::mem::transmute(&draginfo)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreDropOperationTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, draginfo: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DropAsync(::core::mem::transmute(&draginfo)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectableVtbl::new::<Identity, ICoreDropOperationTarget, OFFSET>(),
            EnterAsync: EnterAsync::<Identity, Impl, OFFSET>,
            OverAsync: OverAsync::<Identity, Impl, OFFSET>,
            LeaveAsync: LeaveAsync::<Identity, Impl, OFFSET>,
            DropAsync: DropAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreDropOperationTarget as ::windows_core::Interface>::IID
    }
}
