#[doc = "*Required features: `\"Embedded_DeviceLockdown\"`*"]
pub struct DeviceLockdownProfile;
impl DeviceLockdownProfile {
    #[doc = "*Required features: `\"Embedded_DeviceLockdown\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedLockdownProfiles() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::GUID>> {
        Self::IDeviceLockdownProfileStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSupportedLockdownProfiles)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<::windows_core::GUID>>(result__)
        })
    }
    #[doc = "*Required features: `\"Embedded_DeviceLockdown\"`*"]
    pub fn GetCurrentLockdownProfile() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IDeviceLockdownProfileStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentLockdownProfile)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `\"Embedded_DeviceLockdown\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ApplyLockdownProfileAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(profileid: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        Self::IDeviceLockdownProfileStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ApplyLockdownProfileAsync)(::windows_core::Interface::as_raw(this), profileid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Embedded_DeviceLockdown\"`*"]
    pub fn GetLockdownProfileInformation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(profileid: Param0) -> ::windows_core::Result<DeviceLockdownProfileInformation> {
        Self::IDeviceLockdownProfileStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetLockdownProfileInformation)(::windows_core::Interface::as_raw(this), profileid.into_param().abi(), result__.as_mut_ptr()).from_abi::<DeviceLockdownProfileInformation>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDeviceLockdownProfileStatics<R, F: FnOnce(&IDeviceLockdownProfileStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DeviceLockdownProfile, IDeviceLockdownProfileStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for DeviceLockdownProfile {
    const NAME: &'static str = "Windows.Embedded.DeviceLockdown.DeviceLockdownProfile";
}
#[doc = "*Required features: `\"Embedded_DeviceLockdown\"`*"]
#[repr(transparent)]
pub struct DeviceLockdownProfileInformation(::windows_core::IUnknown);
impl DeviceLockdownProfileInformation {
    #[doc = "*Required features: `\"Embedded_DeviceLockdown\"`*"]
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for DeviceLockdownProfileInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeviceLockdownProfileInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceLockdownProfileInformation {}
impl ::core::fmt::Debug for DeviceLockdownProfileInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceLockdownProfileInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeviceLockdownProfileInformation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Embedded.DeviceLockdown.DeviceLockdownProfileInformation;{7980e14e-45b1-4a96-92fc-62756b739678})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DeviceLockdownProfileInformation {
    type Vtable = IDeviceLockdownProfileInformation_Vtbl;
    const IID: ::windows_core::GUID = <IDeviceLockdownProfileInformation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DeviceLockdownProfileInformation {
    const NAME: &'static str = "Windows.Embedded.DeviceLockdown.DeviceLockdownProfileInformation";
}
impl ::core::convert::From<DeviceLockdownProfileInformation> for ::windows_core::IUnknown {
    fn from(value: DeviceLockdownProfileInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceLockdownProfileInformation> for ::windows_core::IUnknown {
    fn from(value: &DeviceLockdownProfileInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DeviceLockdownProfileInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DeviceLockdownProfileInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DeviceLockdownProfileInformation> for ::windows_core::IInspectable {
    fn from(value: DeviceLockdownProfileInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceLockdownProfileInformation> for ::windows_core::IInspectable {
    fn from(value: &DeviceLockdownProfileInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DeviceLockdownProfileInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DeviceLockdownProfileInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DeviceLockdownProfileInformation {}
unsafe impl ::core::marker::Sync for DeviceLockdownProfileInformation {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceLockdownProfileInformation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceLockdownProfileInformation {
    type Vtable = IDeviceLockdownProfileInformation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7980e14e_45b1_4a96_92fc_62756b739678);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceLockdownProfileInformation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceLockdownProfileStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceLockdownProfileStatics {
    type Vtable = IDeviceLockdownProfileStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x622f6965_f9a8_41a1_a691_88cd80c7a069);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceLockdownProfileStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedLockdownProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedLockdownProfiles: usize,
    pub GetCurrentLockdownProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ApplyLockdownProfileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profileid: ::windows_core::GUID, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApplyLockdownProfileAsync: usize,
    pub GetLockdownProfileInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profileid: ::windows_core::GUID, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
