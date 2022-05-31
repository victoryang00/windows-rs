
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#[cfg(feature = "Automation")]
pub mod Automation;
#[cfg(feature = "People")]
pub mod People;
#[cfg(feature = "Spatial")]
pub mod Spatial;
#[doc(hidden)]
#[repr(transparent)]
pub struct IPerceptionTimestamp(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPerceptionTimestamp {
    type Vtable = IPerceptionTimestamp_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x87c24804_a22e_4adb_ba26_d78ef639bcf4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionTimestamp_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TargetTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub PredictionAmount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPerceptionTimestamp2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPerceptionTimestamp2 {
    type Vtable = IPerceptionTimestamp2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe354b7ed_2bd1_41b7_9ed0_74a15c354537);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionTimestamp2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SystemRelativeTargetTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPerceptionTimestampHelperStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPerceptionTimestampHelperStatics {
    type Vtable = IPerceptionTimestampHelperStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x47a611d4_a9df_4edc_855d_f4d339d967ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionTimestampHelperStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FromHistoricalTargetTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targettime: ::winrt_foundation::DateTime, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPerceptionTimestampHelperStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPerceptionTimestampHelperStatics2 {
    type Vtable = IPerceptionTimestampHelperStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x73d1a7fe_3fb9_4571_87d4_3c920a5e86eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionTimestampHelperStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FromSystemRelativeTargetTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targettime: ::winrt_foundation::TimeSpan, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct PerceptionTimestamp(::windows_core::IUnknown);
impl PerceptionTimestamp {
    pub fn TargetTime(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).TargetTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn PredictionAmount(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).PredictionAmount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SystemRelativeTargetTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = &::windows_core::Interface::cast::<IPerceptionTimestamp2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).SystemRelativeTargetTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for PerceptionTimestamp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PerceptionTimestamp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PerceptionTimestamp {}
impl ::core::fmt::Debug for PerceptionTimestamp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionTimestamp").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PerceptionTimestamp {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Perception.PerceptionTimestamp;{87c24804-a22e-4adb-ba26-d78ef639bcf4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PerceptionTimestamp {
    type Vtable = IPerceptionTimestamp_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionTimestamp as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PerceptionTimestamp {
    const NAME: &'static str = "Windows.Perception.PerceptionTimestamp";
}
impl ::core::convert::From<PerceptionTimestamp> for ::windows_core::IUnknown {
    fn from(value: PerceptionTimestamp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PerceptionTimestamp> for ::windows_core::IUnknown {
    fn from(value: &PerceptionTimestamp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionTimestamp {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionTimestamp {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PerceptionTimestamp> for ::windows_core::IInspectable {
    fn from(value: PerceptionTimestamp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PerceptionTimestamp> for ::windows_core::IInspectable {
    fn from(value: &PerceptionTimestamp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionTimestamp {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionTimestamp {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PerceptionTimestamp {}
unsafe impl ::core::marker::Sync for PerceptionTimestamp {}
pub struct PerceptionTimestampHelper;
impl PerceptionTimestampHelper {
    pub fn FromHistoricalTargetTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(targettime: Param0) -> ::windows_core::Result<PerceptionTimestamp> {
        Self::IPerceptionTimestampHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromHistoricalTargetTime)(::windows_core::Interface::as_raw(this), targettime.into_param().abi(), result__.as_mut_ptr()).from_abi::<PerceptionTimestamp>(result__)
        })
    }
    pub fn FromSystemRelativeTargetTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(targettime: Param0) -> ::windows_core::Result<PerceptionTimestamp> {
        Self::IPerceptionTimestampHelperStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromSystemRelativeTargetTime)(::windows_core::Interface::as_raw(this), targettime.into_param().abi(), result__.as_mut_ptr()).from_abi::<PerceptionTimestamp>(result__)
        })
    }
    pub fn IPerceptionTimestampHelperStatics<R, F: FnOnce(&IPerceptionTimestampHelperStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PerceptionTimestampHelper, IPerceptionTimestampHelperStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPerceptionTimestampHelperStatics2<R, F: FnOnce(&IPerceptionTimestampHelperStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PerceptionTimestampHelper, IPerceptionTimestampHelperStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for PerceptionTimestampHelper {
    const NAME: &'static str = "Windows.Perception.PerceptionTimestampHelper";
}
