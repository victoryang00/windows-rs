#[repr(transparent)]
pub struct ApplicationDataManager(::windows_core::IUnknown);
impl ApplicationDataManager {
    #[cfg(feature = "Storage")]
    pub fn CreateForPackageFamily<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(packagefamilyname: Param0) -> ::windows_core::Result<super::super::Storage::ApplicationData> {
        Self::IApplicationDataManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateForPackageFamily)(::windows_core::Interface::as_raw(this), packagefamilyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Storage::ApplicationData>(result__)
        })
    }
    pub fn IApplicationDataManagerStatics<R, F: FnOnce(&IApplicationDataManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ApplicationDataManager, IApplicationDataManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ApplicationDataManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ApplicationDataManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ApplicationDataManager {}
impl ::core::fmt::Debug for ApplicationDataManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationDataManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ApplicationDataManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Management.Core.ApplicationDataManager;{74d10432-2e99-4000-9a3a-64307e858129})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ApplicationDataManager {
    type Vtable = IApplicationDataManager_Vtbl;
    const IID: ::windows_core::GUID = <IApplicationDataManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ApplicationDataManager {
    const NAME: &'static str = "Windows.Management.Core.ApplicationDataManager";
}
impl ::core::convert::From<ApplicationDataManager> for ::windows_core::IUnknown {
    fn from(value: ApplicationDataManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ApplicationDataManager> for ::windows_core::IUnknown {
    fn from(value: &ApplicationDataManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ApplicationDataManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ApplicationDataManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ApplicationDataManager> for ::windows_core::IInspectable {
    fn from(value: ApplicationDataManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ApplicationDataManager> for ::windows_core::IInspectable {
    fn from(value: &ApplicationDataManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ApplicationDataManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ApplicationDataManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ApplicationDataManager {}
unsafe impl ::core::marker::Sync for ApplicationDataManager {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationDataManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IApplicationDataManager {
    type Vtable = IApplicationDataManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x74d10432_2e99_4000_9a3a_64307e858129);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationDataManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationDataManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IApplicationDataManagerStatics {
    type Vtable = IApplicationDataManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1e1862e3_698e_49a1_9752_dee94925b9b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationDataManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage")]
    pub CreateForPackageFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    CreateForPackageFamily: usize,
}
