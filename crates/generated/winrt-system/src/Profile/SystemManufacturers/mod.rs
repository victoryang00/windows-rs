#[doc(hidden)]
#[repr(transparent)]
pub struct IOemSupportInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOemSupportInfo {
    type Vtable = IOemSupportInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8d2eae55_87ef_4266_86d0_c4afbeb29bb9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOemSupportInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SupportLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SupportAppLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SupportProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmbiosInformationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmbiosInformationStatics {
    type Vtable = ISmbiosInformationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x080cca7c_637c_48c4_b728_f9273812db8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmbiosInformationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemSupportDeviceInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemSupportDeviceInfo {
    type Vtable = ISystemSupportDeviceInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x05880b99_8247_441b_a996_a1784bab79a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemSupportDeviceInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OperatingSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SystemManufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SystemProductName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SystemSku: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SystemHardwareVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SystemFirmwareVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemSupportInfoStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemSupportInfoStatics {
    type Vtable = ISystemSupportInfoStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef750974_c422_45d7_a44d_5c1c0043a2b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemSupportInfoStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LocalSystemEdition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub OemSupportInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemSupportInfoStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemSupportInfoStatics2 {
    type Vtable = ISystemSupportInfoStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33f349a4_3fa1_4986_aa4b_057420455e6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemSupportInfoStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LocalDeviceInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct OemSupportInfo(::windows_core::IUnknown);
impl OemSupportInfo {
    pub fn SupportLink(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportLink)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SupportAppLink(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportAppLink)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SupportProvider(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SupportProvider)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for OemSupportInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OemSupportInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OemSupportInfo {}
impl ::core::fmt::Debug for OemSupportInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OemSupportInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for OemSupportInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Profile.SystemManufacturers.OemSupportInfo;{8d2eae55-87ef-4266-86d0-c4afbeb29bb9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for OemSupportInfo {
    type Vtable = IOemSupportInfo_Vtbl;
    const IID: ::windows_core::GUID = <IOemSupportInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for OemSupportInfo {
    const NAME: &'static str = "Windows.System.Profile.SystemManufacturers.OemSupportInfo";
}
impl ::core::convert::From<OemSupportInfo> for ::windows_core::IUnknown {
    fn from(value: OemSupportInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OemSupportInfo> for ::windows_core::IUnknown {
    fn from(value: &OemSupportInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for OemSupportInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a OemSupportInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OemSupportInfo> for ::windows_core::IInspectable {
    fn from(value: OemSupportInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OemSupportInfo> for ::windows_core::IInspectable {
    fn from(value: &OemSupportInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for OemSupportInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a OemSupportInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OemSupportInfo {}
unsafe impl ::core::marker::Sync for OemSupportInfo {}
pub struct SmbiosInformation;
impl SmbiosInformation {
    pub fn SerialNumber() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISmbiosInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SerialNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn ISmbiosInformationStatics<R, F: FnOnce(&ISmbiosInformationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmbiosInformation, ISmbiosInformationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for SmbiosInformation {
    const NAME: &'static str = "Windows.System.Profile.SystemManufacturers.SmbiosInformation";
}
#[repr(transparent)]
pub struct SystemSupportDeviceInfo(::windows_core::IUnknown);
impl SystemSupportDeviceInfo {
    pub fn OperatingSystem(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).OperatingSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FriendlyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FriendlyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SystemManufacturer(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SystemManufacturer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SystemProductName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SystemProductName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SystemSku(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SystemSku)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SystemHardwareVersion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SystemHardwareVersion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SystemFirmwareVersion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SystemFirmwareVersion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for SystemSupportDeviceInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemSupportDeviceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemSupportDeviceInfo {}
impl ::core::fmt::Debug for SystemSupportDeviceInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemSupportDeviceInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SystemSupportDeviceInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Profile.SystemManufacturers.SystemSupportDeviceInfo;{05880b99-8247-441b-a996-a1784bab79a8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SystemSupportDeviceInfo {
    type Vtable = ISystemSupportDeviceInfo_Vtbl;
    const IID: ::windows_core::GUID = <ISystemSupportDeviceInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SystemSupportDeviceInfo {
    const NAME: &'static str = "Windows.System.Profile.SystemManufacturers.SystemSupportDeviceInfo";
}
impl ::core::convert::From<SystemSupportDeviceInfo> for ::windows_core::IUnknown {
    fn from(value: SystemSupportDeviceInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemSupportDeviceInfo> for ::windows_core::IUnknown {
    fn from(value: &SystemSupportDeviceInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SystemSupportDeviceInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SystemSupportDeviceInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemSupportDeviceInfo> for ::windows_core::IInspectable {
    fn from(value: SystemSupportDeviceInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemSupportDeviceInfo> for ::windows_core::IInspectable {
    fn from(value: &SystemSupportDeviceInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SystemSupportDeviceInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SystemSupportDeviceInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemSupportDeviceInfo {}
unsafe impl ::core::marker::Sync for SystemSupportDeviceInfo {}
pub struct SystemSupportInfo;
impl SystemSupportInfo {
    pub fn LocalSystemEdition() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISystemSupportInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).LocalSystemEdition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn OemSupportInfo() -> ::windows_core::Result<OemSupportInfo> {
        Self::ISystemSupportInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OemSupportInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<OemSupportInfo>(result__)
        })
    }
    pub fn LocalDeviceInfo() -> ::windows_core::Result<SystemSupportDeviceInfo> {
        Self::ISystemSupportInfoStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LocalDeviceInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SystemSupportDeviceInfo>(result__)
        })
    }
    pub fn ISystemSupportInfoStatics<R, F: FnOnce(&ISystemSupportInfoStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SystemSupportInfo, ISystemSupportInfoStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISystemSupportInfoStatics2<R, F: FnOnce(&ISystemSupportInfoStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SystemSupportInfo, ISystemSupportInfoStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for SystemSupportInfo {
    const NAME: &'static str = "Windows.System.Profile.SystemManufacturers.SystemSupportInfo";
}
