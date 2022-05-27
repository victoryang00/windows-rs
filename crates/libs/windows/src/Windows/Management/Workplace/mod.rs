#[doc(hidden)]
#[repr(transparent)]
pub struct IMdmAllowPolicyStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMdmAllowPolicyStatics {
    type Vtable = IMdmAllowPolicyStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc39709e7_741c_41f2_a4b6_314c31502586);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMdmAllowPolicyStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsBrowserAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsCameraAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsMicrosoftAccountAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsStoreAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMdmPolicyStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMdmPolicyStatics2 {
    type Vtable = IMdmPolicyStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc99c7526_03d4_49f9_a993_43efccd265c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMdmPolicyStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetMessagingSyncPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MessagingSyncPolicy) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWorkplaceSettingsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWorkplaceSettingsStatics {
    type Vtable = IWorkplaceSettingsStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe4676ffd_2d92_4c08_bad4_f6590b54a6d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkplaceSettingsStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsMicrosoftAccountOptional: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Management_Workplace\"`*"]
pub struct MdmPolicy;
impl MdmPolicy {
    #[doc = "*Required features: `\"Management_Workplace\"`*"]
    pub fn IsBrowserAllowed() -> ::windows_core::Result<bool> {
        Self::IMdmAllowPolicyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBrowserAllowed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Management_Workplace\"`*"]
    pub fn IsCameraAllowed() -> ::windows_core::Result<bool> {
        Self::IMdmAllowPolicyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCameraAllowed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Management_Workplace\"`*"]
    pub fn IsMicrosoftAccountAllowed() -> ::windows_core::Result<bool> {
        Self::IMdmAllowPolicyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsMicrosoftAccountAllowed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Management_Workplace\"`*"]
    pub fn IsStoreAllowed() -> ::windows_core::Result<bool> {
        Self::IMdmAllowPolicyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsStoreAllowed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Management_Workplace\"`*"]
    pub fn GetMessagingSyncPolicy() -> ::windows_core::Result<MessagingSyncPolicy> {
        Self::IMdmPolicyStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MessagingSyncPolicy>::zeroed();
            (::windows_core::Interface::vtable(this).GetMessagingSyncPolicy)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MessagingSyncPolicy>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMdmAllowPolicyStatics<R, F: FnOnce(&IMdmAllowPolicyStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MdmPolicy, IMdmAllowPolicyStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMdmPolicyStatics2<R, F: FnOnce(&IMdmPolicyStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MdmPolicy, IMdmPolicyStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for MdmPolicy {
    const NAME: &'static str = "Windows.Management.Workplace.MdmPolicy";
}
#[doc = "*Required features: `\"Management_Workplace\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MessagingSyncPolicy(pub i32);
impl MessagingSyncPolicy {
    pub const Disallowed: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const Required: Self = Self(2i32);
}
impl ::core::marker::Copy for MessagingSyncPolicy {}
impl ::core::clone::Clone for MessagingSyncPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MessagingSyncPolicy {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MessagingSyncPolicy {
    type Abi = Self;
}
impl ::core::fmt::Debug for MessagingSyncPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MessagingSyncPolicy").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MessagingSyncPolicy {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Management.Workplace.MessagingSyncPolicy;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Management_Workplace\"`*"]
pub struct WorkplaceSettings;
impl WorkplaceSettings {
    #[doc = "*Required features: `\"Management_Workplace\"`*"]
    pub fn IsMicrosoftAccountOptional() -> ::windows_core::Result<bool> {
        Self::IWorkplaceSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsMicrosoftAccountOptional)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWorkplaceSettingsStatics<R, F: FnOnce(&IWorkplaceSettingsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<WorkplaceSettings, IWorkplaceSettingsStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for WorkplaceSettings {
    const NAME: &'static str = "Windows.Management.Workplace.WorkplaceSettings";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
