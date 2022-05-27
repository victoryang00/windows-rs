#[doc(hidden)]
#[repr(transparent)]
pub struct IServiceDeviceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IServiceDeviceStatics {
    type Vtable = IServiceDeviceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa88214e1_59c7_4a20_aba6_9f6707937230);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceDeviceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicetype: ServiceDeviceType, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetDeviceSelectorFromServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceid: ::windows_core::GUID, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageDeviceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorageDeviceStatics {
    type Vtable = IStorageDeviceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5ece44ee_1b23_4dd2_8652_bc164f003128);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageDeviceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage")]
    pub FromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    FromId: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Portable\"`*"]
pub struct ServiceDevice;
impl ServiceDevice {
    #[doc = "*Required features: `\"Devices_Portable\"`*"]
    pub fn GetDeviceSelector(servicetype: ServiceDeviceType) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IServiceDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), servicetype, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Portable\"`*"]
    pub fn GetDeviceSelectorFromServiceId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(serviceid: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IServiceDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelectorFromServiceId)(::windows_core::Interface::as_raw(this), serviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IServiceDeviceStatics<R, F: FnOnce(&IServiceDeviceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ServiceDevice, IServiceDeviceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for ServiceDevice {
    const NAME: &'static str = "Windows.Devices.Portable.ServiceDevice";
}
#[doc = "*Required features: `\"Devices_Portable\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ServiceDeviceType(pub i32);
impl ServiceDeviceType {
    pub const CalendarService: Self = Self(0i32);
    pub const ContactsService: Self = Self(1i32);
    pub const DeviceStatusService: Self = Self(2i32);
    pub const NotesService: Self = Self(3i32);
    pub const RingtonesService: Self = Self(4i32);
    pub const SmsService: Self = Self(5i32);
    pub const TasksService: Self = Self(6i32);
}
impl ::core::marker::Copy for ServiceDeviceType {}
impl ::core::clone::Clone for ServiceDeviceType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ServiceDeviceType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ServiceDeviceType {
    type Abi = Self;
}
impl ::core::fmt::Debug for ServiceDeviceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ServiceDeviceType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ServiceDeviceType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Portable.ServiceDeviceType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Portable\"`*"]
pub struct StorageDevice;
impl StorageDevice {
    #[doc = "*Required features: `\"Devices_Portable\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn FromId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<super::super::Storage::StorageFolder> {
        Self::IStorageDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromId)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Storage::StorageFolder>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Portable\"`*"]
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IStorageDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorageDeviceStatics<R, F: FnOnce(&IStorageDeviceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<StorageDevice, IStorageDeviceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for StorageDevice {
    const NAME: &'static str = "Windows.Devices.Portable.StorageDevice";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
