#[doc(hidden)]
#[repr(transparent)]
pub struct IRfcommDeviceService(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRfcommDeviceService {
    type Vtable = IRfcommDeviceService_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae81ff1f_c5a1_4c40_8c28_f3efd69062f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommDeviceService_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Networking")]
    pub ConnectionHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    ConnectionHostName: usize,
    pub ConnectionServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Networking_Sockets")]
    pub ProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Networking::Sockets::SocketProtectionLevel) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    ProtectionLevel: usize,
    #[cfg(feature = "Networking_Sockets")]
    pub MaxProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Networking::Sockets::SocketProtectionLevel) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    MaxProtectionLevel: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub GetSdpRawAttributesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    GetSdpRawAttributesAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub GetSdpRawAttributesWithCacheModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cachemode: super::BluetoothCacheMode, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    GetSdpRawAttributesWithCacheModeAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRfcommDeviceService2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRfcommDeviceService2 {
    type Vtable = IRfcommDeviceService2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x536ced14_ebcd_49fe_bf9f_40efc689b20d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommDeviceService2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRfcommDeviceService3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRfcommDeviceService3 {
    type Vtable = IRfcommDeviceService3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1c22ace6_dd44_4d23_866d_8f3486ee6490);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommDeviceService3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceAccessInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceAccessInformation: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))]
    RequestAccessAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRfcommDeviceServiceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRfcommDeviceServiceStatics {
    type Vtable = IRfcommDeviceServiceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa4a149ef_626d_41ac_b253_87ac5c27e28a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommDeviceServiceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceid: ::windows_core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRfcommDeviceServiceStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRfcommDeviceServiceStatics2 {
    type Vtable = IRfcommDeviceServiceStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaa8cb1c9_e78d_4be4_8076_0a3d87a0a05f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommDeviceServiceStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeviceSelectorForBluetoothDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluetoothdevice: ::windows_core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetDeviceSelectorForBluetoothDeviceWithCacheMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluetoothdevice: ::windows_core::RawPtr, cachemode: super::BluetoothCacheMode, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetDeviceSelectorForBluetoothDeviceAndServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluetoothdevice: ::windows_core::RawPtr, serviceid: ::windows_core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetDeviceSelectorForBluetoothDeviceAndServiceIdWithCacheMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluetoothdevice: ::windows_core::RawPtr, serviceid: ::windows_core::RawPtr, cachemode: super::BluetoothCacheMode, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRfcommDeviceServicesResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRfcommDeviceServicesResult {
    type Vtable = IRfcommDeviceServicesResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3b48388c_7ccf_488e_9625_d259a5732d55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommDeviceServicesResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Services: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Services: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRfcommServiceId(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRfcommServiceId {
    type Vtable = IRfcommServiceId_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x22629204_7e02_4017_8136_da1b6a1b9bbf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommServiceId_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Uuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub AsShortId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub AsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRfcommServiceIdStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRfcommServiceIdStatics {
    type Vtable = IRfcommServiceIdStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a179eba_a975_46e3_b56b_08ffd783a5fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommServiceIdStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FromUuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uuid: ::windows_core::GUID, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FromShortId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shortid: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SerialPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ObexObjectPush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ObexFileTransfer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PhoneBookAccessPce: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PhoneBookAccessPse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GenericFileTransfer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRfcommServiceProvider(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRfcommServiceProvider {
    type Vtable = IRfcommServiceProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeadbfdc4_b1f6_44ff_9f7c_e7a82ab86821);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommServiceProvider_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub SdpRawAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    SdpRawAttributes: usize,
    #[cfg(feature = "Networking_Sockets")]
    pub StartAdvertising: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, listener: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    StartAdvertising: usize,
    pub StopAdvertising: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRfcommServiceProvider2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRfcommServiceProvider2 {
    type Vtable = IRfcommServiceProvider2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x736bdfc6_3c81_4d1e_baf2_ddbb81284512);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommServiceProvider2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Networking_Sockets")]
    pub StartAdvertisingWithRadioDiscoverability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, listener: ::windows_core::RawPtr, radiodiscoverable: bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    StartAdvertisingWithRadioDiscoverability: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRfcommServiceProviderStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRfcommServiceProviderStatics {
    type Vtable = IRfcommServiceProviderStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x98888303_69ca_413a_84f7_4344c7292997);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommServiceProviderStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceid: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
}
#[repr(transparent)]
pub struct RfcommDeviceService(::windows_core::IUnknown);
impl RfcommDeviceService {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Networking")]
    pub fn ConnectionHostName(&self) -> ::windows_core::Result<super::super::super::Networking::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionHostName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Networking::HostName>(result__)
        }
    }
    pub fn ConnectionServiceName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionServiceName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ServiceId(&self) -> ::windows_core::Result<RfcommServiceId> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RfcommServiceId>(result__)
        }
    }
    #[cfg(feature = "Networking_Sockets")]
    pub fn ProtectionLevel(&self) -> ::windows_core::Result<super::super::super::Networking::Sockets::SocketProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Networking::Sockets::SocketProtectionLevel>::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionLevel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Networking::Sockets::SocketProtectionLevel>(result__)
        }
    }
    #[cfg(feature = "Networking_Sockets")]
    pub fn MaxProtectionLevel(&self) -> ::windows_core::Result<super::super::super::Networking::Sockets::SocketProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Networking::Sockets::SocketProtectionLevel>::zeroed();
            (::windows_core::Interface::vtable(this).MaxProtectionLevel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Networking::Sockets::SocketProtectionLevel>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn GetSdpRawAttributesAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<u32, super::super::super::Storage::Streams::IBuffer>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSdpRawAttributesAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<u32, super::super::super::Storage::Streams::IBuffer>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn GetSdpRawAttributesWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<u32, super::super::super::Storage::Streams::IBuffer>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSdpRawAttributesWithCacheModeAsync)(::windows_core::Interface::as_raw(this), cachemode, result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<u32, super::super::super::Storage::Streams::IBuffer>>>(result__)
        }
    }
    pub fn Device(&self) -> ::windows_core::Result<super::BluetoothDevice> {
        let this = &::windows_core::Interface::cast::<IRfcommDeviceService2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Device)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::BluetoothDevice>(result__)
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceAccessInformation(&self) -> ::windows_core::Result<super::super::Enumeration::DeviceAccessInformation> {
        let this = &::windows_core::Interface::cast::<IRfcommDeviceService3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceAccessInformation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Enumeration::DeviceAccessInformation>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub fn RequestAccessAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::Enumeration::DeviceAccessStatus>> {
        let this = &::windows_core::Interface::cast::<IRfcommDeviceService3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::Enumeration::DeviceAccessStatus>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<RfcommDeviceService>> {
        Self::IRfcommDeviceServiceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<RfcommDeviceService>>(result__)
        })
    }
    pub fn GetDeviceSelector<'a, Param0: ::windows_core::IntoParam<'a, RfcommServiceId>>(serviceid: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IRfcommDeviceServiceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), serviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorForBluetoothDevice<'a, Param0: ::windows_core::IntoParam<'a, super::BluetoothDevice>>(bluetoothdevice: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IRfcommDeviceServiceStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelectorForBluetoothDevice)(::windows_core::Interface::as_raw(this), bluetoothdevice.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorForBluetoothDeviceWithCacheMode<'a, Param0: ::windows_core::IntoParam<'a, super::BluetoothDevice>>(bluetoothdevice: Param0, cachemode: super::BluetoothCacheMode) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IRfcommDeviceServiceStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelectorForBluetoothDeviceWithCacheMode)(::windows_core::Interface::as_raw(this), bluetoothdevice.into_param().abi(), cachemode, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorForBluetoothDeviceAndServiceId<'a, Param0: ::windows_core::IntoParam<'a, super::BluetoothDevice>, Param1: ::windows_core::IntoParam<'a, RfcommServiceId>>(bluetoothdevice: Param0, serviceid: Param1) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IRfcommDeviceServiceStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelectorForBluetoothDeviceAndServiceId)(::windows_core::Interface::as_raw(this), bluetoothdevice.into_param().abi(), serviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorForBluetoothDeviceAndServiceIdWithCacheMode<'a, Param0: ::windows_core::IntoParam<'a, super::BluetoothDevice>, Param1: ::windows_core::IntoParam<'a, RfcommServiceId>>(bluetoothdevice: Param0, serviceid: Param1, cachemode: super::BluetoothCacheMode) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IRfcommDeviceServiceStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelectorForBluetoothDeviceAndServiceIdWithCacheMode)(::windows_core::Interface::as_raw(this), bluetoothdevice.into_param().abi(), serviceid.into_param().abi(), cachemode, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IRfcommDeviceServiceStatics<R, F: FnOnce(&IRfcommDeviceServiceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RfcommDeviceService, IRfcommDeviceServiceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IRfcommDeviceServiceStatics2<R, F: FnOnce(&IRfcommDeviceServiceStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RfcommDeviceService, IRfcommDeviceServiceStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RfcommDeviceService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RfcommDeviceService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RfcommDeviceService {}
impl ::core::fmt::Debug for RfcommDeviceService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RfcommDeviceService").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RfcommDeviceService {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceService;{ae81ff1f-c5a1-4c40-8c28-f3efd69062f3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RfcommDeviceService {
    type Vtable = IRfcommDeviceService_Vtbl;
    const IID: ::windows_core::GUID = <IRfcommDeviceService as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RfcommDeviceService {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceService";
}
impl ::core::convert::From<RfcommDeviceService> for ::windows_core::IUnknown {
    fn from(value: RfcommDeviceService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RfcommDeviceService> for ::windows_core::IUnknown {
    fn from(value: &RfcommDeviceService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RfcommDeviceService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RfcommDeviceService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RfcommDeviceService> for ::windows_core::IInspectable {
    fn from(value: RfcommDeviceService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RfcommDeviceService> for ::windows_core::IInspectable {
    fn from(value: &RfcommDeviceService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RfcommDeviceService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RfcommDeviceService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<RfcommDeviceService> for super::super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: RfcommDeviceService) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&RfcommDeviceService> for super::super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &RfcommDeviceService) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Foundation::IClosable> for RfcommDeviceService {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Foundation::IClosable> for &RfcommDeviceService {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for RfcommDeviceService {}
unsafe impl ::core::marker::Sync for RfcommDeviceService {}
#[repr(transparent)]
pub struct RfcommDeviceServicesResult(::windows_core::IUnknown);
impl RfcommDeviceServicesResult {
    pub fn Error(&self) -> ::windows_core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::BluetoothError>::zeroed();
            (::windows_core::Interface::vtable(this).Error)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::BluetoothError>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Services(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<RfcommDeviceService>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Services)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<RfcommDeviceService>>(result__)
        }
    }
}
impl ::core::clone::Clone for RfcommDeviceServicesResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RfcommDeviceServicesResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RfcommDeviceServicesResult {}
impl ::core::fmt::Debug for RfcommDeviceServicesResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RfcommDeviceServicesResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RfcommDeviceServicesResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceServicesResult;{3b48388c-7ccf-488e-9625-d259a5732d55})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RfcommDeviceServicesResult {
    type Vtable = IRfcommDeviceServicesResult_Vtbl;
    const IID: ::windows_core::GUID = <IRfcommDeviceServicesResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RfcommDeviceServicesResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceServicesResult";
}
impl ::core::convert::From<RfcommDeviceServicesResult> for ::windows_core::IUnknown {
    fn from(value: RfcommDeviceServicesResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RfcommDeviceServicesResult> for ::windows_core::IUnknown {
    fn from(value: &RfcommDeviceServicesResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RfcommDeviceServicesResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RfcommDeviceServicesResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RfcommDeviceServicesResult> for ::windows_core::IInspectable {
    fn from(value: RfcommDeviceServicesResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RfcommDeviceServicesResult> for ::windows_core::IInspectable {
    fn from(value: &RfcommDeviceServicesResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RfcommDeviceServicesResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RfcommDeviceServicesResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RfcommDeviceServicesResult {}
unsafe impl ::core::marker::Sync for RfcommDeviceServicesResult {}
#[repr(transparent)]
pub struct RfcommServiceId(::windows_core::IUnknown);
impl RfcommServiceId {
    pub fn Uuid(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).Uuid)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn AsShortId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).AsShortId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn AsString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AsString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FromUuid<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(uuid: Param0) -> ::windows_core::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromUuid)(::windows_core::Interface::as_raw(this), uuid.into_param().abi(), result__.as_mut_ptr()).from_abi::<RfcommServiceId>(result__)
        })
    }
    pub fn FromShortId(shortid: u32) -> ::windows_core::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromShortId)(::windows_core::Interface::as_raw(this), shortid, result__.as_mut_ptr()).from_abi::<RfcommServiceId>(result__)
        })
    }
    pub fn SerialPort() -> ::windows_core::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SerialPort)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RfcommServiceId>(result__)
        })
    }
    pub fn ObexObjectPush() -> ::windows_core::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ObexObjectPush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RfcommServiceId>(result__)
        })
    }
    pub fn ObexFileTransfer() -> ::windows_core::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ObexFileTransfer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RfcommServiceId>(result__)
        })
    }
    pub fn PhoneBookAccessPce() -> ::windows_core::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PhoneBookAccessPce)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RfcommServiceId>(result__)
        })
    }
    pub fn PhoneBookAccessPse() -> ::windows_core::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PhoneBookAccessPse)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RfcommServiceId>(result__)
        })
    }
    pub fn GenericFileTransfer() -> ::windows_core::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GenericFileTransfer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RfcommServiceId>(result__)
        })
    }
    pub fn IRfcommServiceIdStatics<R, F: FnOnce(&IRfcommServiceIdStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RfcommServiceId, IRfcommServiceIdStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RfcommServiceId {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RfcommServiceId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RfcommServiceId {}
impl ::core::fmt::Debug for RfcommServiceId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RfcommServiceId").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RfcommServiceId {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Rfcomm.RfcommServiceId;{22629204-7e02-4017-8136-da1b6a1b9bbf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RfcommServiceId {
    type Vtable = IRfcommServiceId_Vtbl;
    const IID: ::windows_core::GUID = <IRfcommServiceId as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RfcommServiceId {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.RfcommServiceId";
}
impl ::core::convert::From<RfcommServiceId> for ::windows_core::IUnknown {
    fn from(value: RfcommServiceId) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RfcommServiceId> for ::windows_core::IUnknown {
    fn from(value: &RfcommServiceId) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RfcommServiceId {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RfcommServiceId {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RfcommServiceId> for ::windows_core::IInspectable {
    fn from(value: RfcommServiceId) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RfcommServiceId> for ::windows_core::IInspectable {
    fn from(value: &RfcommServiceId) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RfcommServiceId {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RfcommServiceId {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RfcommServiceId {}
unsafe impl ::core::marker::Sync for RfcommServiceId {}
#[repr(transparent)]
pub struct RfcommServiceProvider(::windows_core::IUnknown);
impl RfcommServiceProvider {
    pub fn ServiceId(&self) -> ::windows_core::Result<RfcommServiceId> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RfcommServiceId>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn SdpRawAttributes(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IMap<u32, super::super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SdpRawAttributes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IMap<u32, super::super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[cfg(feature = "Networking_Sockets")]
    pub fn StartAdvertising<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Networking::Sockets::StreamSocketListener>>(&self, listener: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StartAdvertising)(::windows_core::Interface::as_raw(this), listener.into_param().abi()).ok() }
    }
    pub fn StopAdvertising(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StopAdvertising)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Networking_Sockets")]
    pub fn StartAdvertisingWithRadioDiscoverability<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Networking::Sockets::StreamSocketListener>>(&self, listener: Param0, radiodiscoverable: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IRfcommServiceProvider2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAdvertisingWithRadioDiscoverability)(::windows_core::Interface::as_raw(this), listener.into_param().abi(), radiodiscoverable).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateAsync<'a, Param0: ::windows_core::IntoParam<'a, RfcommServiceId>>(serviceid: Param0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<RfcommServiceProvider>> {
        Self::IRfcommServiceProviderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAsync)(::windows_core::Interface::as_raw(this), serviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<RfcommServiceProvider>>(result__)
        })
    }
    pub fn IRfcommServiceProviderStatics<R, F: FnOnce(&IRfcommServiceProviderStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RfcommServiceProvider, IRfcommServiceProviderStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RfcommServiceProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RfcommServiceProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RfcommServiceProvider {}
impl ::core::fmt::Debug for RfcommServiceProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RfcommServiceProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RfcommServiceProvider {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Rfcomm.RfcommServiceProvider;{eadbfdc4-b1f6-44ff-9f7c-e7a82ab86821})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RfcommServiceProvider {
    type Vtable = IRfcommServiceProvider_Vtbl;
    const IID: ::windows_core::GUID = <IRfcommServiceProvider as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RfcommServiceProvider {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.RfcommServiceProvider";
}
impl ::core::convert::From<RfcommServiceProvider> for ::windows_core::IUnknown {
    fn from(value: RfcommServiceProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RfcommServiceProvider> for ::windows_core::IUnknown {
    fn from(value: &RfcommServiceProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RfcommServiceProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RfcommServiceProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RfcommServiceProvider> for ::windows_core::IInspectable {
    fn from(value: RfcommServiceProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RfcommServiceProvider> for ::windows_core::IInspectable {
    fn from(value: &RfcommServiceProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RfcommServiceProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RfcommServiceProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RfcommServiceProvider {}
unsafe impl ::core::marker::Sync for RfcommServiceProvider {}
