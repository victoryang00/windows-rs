pub struct CoreUserActivityManager;
impl CoreUserActivityManager {
    pub fn CreateUserActivitySessionInBackground<'a, Param0: ::windows_core::IntoParam<'a, super::UserActivity>>(activity: Param0) -> ::windows_core::Result<super::UserActivitySession> {
        Self::ICoreUserActivityManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateUserActivitySessionInBackground)(::windows_core::Interface::as_raw(this), activity.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::UserActivitySession>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn DeleteUserActivitySessionsInTimeRangeAsync<'a, Param0: ::windows_core::IntoParam<'a, super::UserActivityChannel>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::DateTime>, Param2: ::windows_core::IntoParam<'a, super::super::super::Foundation::DateTime>>(channel: Param0, starttime: Param1, endtime: Param2) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::ICoreUserActivityManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteUserActivitySessionsInTimeRangeAsync)(::windows_core::Interface::as_raw(this), channel.into_param().abi(), starttime.into_param().abi(), endtime.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    pub fn ICoreUserActivityManagerStatics<R, F: FnOnce(&ICoreUserActivityManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CoreUserActivityManager, ICoreUserActivityManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for CoreUserActivityManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.Core.CoreUserActivityManager";
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreUserActivityManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreUserActivityManagerStatics {
    type Vtable = ICoreUserActivityManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xca3adb02_a4be_4d4d_bfa8_6795f4264efb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreUserActivityManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateUserActivitySessionInBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activity: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeleteUserActivitySessionsInTimeRangeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: ::windows_core::RawPtr, starttime: super::super::super::Foundation::DateTime, endtime: super::super::super::Foundation::DateTime, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteUserActivitySessionsInTimeRangeAsync: usize,
}
