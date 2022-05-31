#[repr(transparent)]
pub struct II2cControllerProvider(::windows_core::IUnknown);
impl II2cControllerProvider {
    pub fn GetDeviceProvider<'a, Param0: ::windows_core::IntoParam<'a, ProviderI2cConnectionSettings>>(&self, settings: Param0) -> ::windows_core::Result<II2cDeviceProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceProvider)(::windows_core::Interface::as_raw(this), settings.into_param().abi(), result__.as_mut_ptr()).from_abi::<II2cDeviceProvider>(result__)
        }
    }
}
impl ::core::convert::From<II2cControllerProvider> for ::windows_core::IUnknown {
    fn from(value: II2cControllerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&II2cControllerProvider> for ::windows_core::IUnknown {
    fn from(value: &II2cControllerProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for II2cControllerProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a II2cControllerProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<II2cControllerProvider> for ::windows_core::IInspectable {
    fn from(value: II2cControllerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&II2cControllerProvider> for ::windows_core::IInspectable {
    fn from(value: &II2cControllerProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for II2cControllerProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a II2cControllerProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for II2cControllerProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for II2cControllerProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for II2cControllerProvider {}
impl ::core::fmt::Debug for II2cControllerProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("II2cControllerProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for II2cControllerProvider {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{61c2bb82-4510-4163-a87c-4e15a9558980}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for II2cControllerProvider {
    type Vtable = II2cControllerProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x61c2bb82_4510_4163_a87c_4e15a9558980);
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cControllerProvider_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeviceProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct II2cDeviceProvider(::windows_core::IUnknown);
impl II2cDeviceProvider {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Write(&self, buffer: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Write)(::windows_core::Interface::as_raw(this), buffer.len() as u32, ::core::mem::transmute(buffer.as_ptr())).ok() }
    }
    pub fn WritePartial(&self, buffer: &[u8]) -> ::windows_core::Result<ProviderI2cTransferResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ProviderI2cTransferResult>::zeroed();
            (::windows_core::Interface::vtable(this).WritePartial)(::windows_core::Interface::as_raw(this), buffer.len() as u32, ::core::mem::transmute(buffer.as_ptr()), result__.as_mut_ptr()).from_abi::<ProviderI2cTransferResult>(result__)
        }
    }
    pub fn Read(&self, buffer: &mut [u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Read)(::windows_core::Interface::as_raw(this), buffer.len() as u32, ::core::mem::transmute_copy(&buffer)).ok() }
    }
    pub fn ReadPartial(&self, buffer: &mut [u8]) -> ::windows_core::Result<ProviderI2cTransferResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ProviderI2cTransferResult>::zeroed();
            (::windows_core::Interface::vtable(this).ReadPartial)(::windows_core::Interface::as_raw(this), buffer.len() as u32, ::core::mem::transmute_copy(&buffer), result__.as_mut_ptr()).from_abi::<ProviderI2cTransferResult>(result__)
        }
    }
    pub fn WriteRead(&self, writebuffer: &[u8], readbuffer: &mut [u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteRead)(::windows_core::Interface::as_raw(this), writebuffer.len() as u32, ::core::mem::transmute(writebuffer.as_ptr()), readbuffer.len() as u32, ::core::mem::transmute_copy(&readbuffer)).ok() }
    }
    pub fn WriteReadPartial(&self, writebuffer: &[u8], readbuffer: &mut [u8]) -> ::windows_core::Result<ProviderI2cTransferResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ProviderI2cTransferResult>::zeroed();
            (::windows_core::Interface::vtable(this).WriteReadPartial)(::windows_core::Interface::as_raw(this), writebuffer.len() as u32, ::core::mem::transmute(writebuffer.as_ptr()), readbuffer.len() as u32, ::core::mem::transmute_copy(&readbuffer), result__.as_mut_ptr()).from_abi::<ProviderI2cTransferResult>(result__)
        }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::convert::From<II2cDeviceProvider> for ::windows_core::IUnknown {
    fn from(value: II2cDeviceProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&II2cDeviceProvider> for ::windows_core::IUnknown {
    fn from(value: &II2cDeviceProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for II2cDeviceProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a II2cDeviceProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<II2cDeviceProvider> for ::windows_core::IInspectable {
    fn from(value: II2cDeviceProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&II2cDeviceProvider> for ::windows_core::IInspectable {
    fn from(value: &II2cDeviceProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for II2cDeviceProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a II2cDeviceProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<II2cDeviceProvider> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: II2cDeviceProvider) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&II2cDeviceProvider> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &II2cDeviceProvider) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for II2cDeviceProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &II2cDeviceProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for II2cDeviceProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for II2cDeviceProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for II2cDeviceProvider {}
impl ::core::fmt::Debug for II2cDeviceProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("II2cDeviceProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for II2cDeviceProvider {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{ad342654-57e8-453e-8329-d1e447d103a9}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for II2cDeviceProvider {
    type Vtable = II2cDeviceProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xad342654_57e8_453e_8329_d1e447d103a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cDeviceProvider_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *const u8) -> ::windows_core::HRESULT,
    pub WritePartial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *const u8, result__: *mut ProviderI2cTransferResult) -> ::windows_core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8) -> ::windows_core::HRESULT,
    pub ReadPartial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8, result__: *mut ProviderI2cTransferResult) -> ::windows_core::HRESULT,
    pub WriteRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> ::windows_core::HRESULT,
    pub WriteReadPartial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8, result__: *mut ProviderI2cTransferResult) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct II2cProvider(::windows_core::IUnknown);
impl II2cProvider {
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetControllersAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<II2cControllerProvider>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetControllersAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<II2cControllerProvider>>>(result__)
        }
    }
}
impl ::core::convert::From<II2cProvider> for ::windows_core::IUnknown {
    fn from(value: II2cProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&II2cProvider> for ::windows_core::IUnknown {
    fn from(value: &II2cProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for II2cProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a II2cProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<II2cProvider> for ::windows_core::IInspectable {
    fn from(value: II2cProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&II2cProvider> for ::windows_core::IInspectable {
    fn from(value: &II2cProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for II2cProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a II2cProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for II2cProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for II2cProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for II2cProvider {}
impl ::core::fmt::Debug for II2cProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("II2cProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for II2cProvider {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{6f13083e-bf62-4fe2-a95a-f08999669818}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for II2cProvider {
    type Vtable = II2cProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f13083e_bf62_4fe2_a95a_f08999669818);
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cProvider_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetControllersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetControllersAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProviderI2cConnectionSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProviderI2cConnectionSettings {
    type Vtable = IProviderI2cConnectionSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe9db4e34_e510_44b7_809d_f2f85b555339);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderI2cConnectionSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SlaveAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetSlaveAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub BusSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ProviderI2cBusSpeed) -> ::windows_core::HRESULT,
    pub SetBusSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ProviderI2cBusSpeed) -> ::windows_core::HRESULT,
    pub SharingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ProviderI2cSharingMode) -> ::windows_core::HRESULT,
    pub SetSharingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ProviderI2cSharingMode) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ProviderI2cBusSpeed(pub i32);
impl ProviderI2cBusSpeed {
    pub const StandardMode: Self = Self(0i32);
    pub const FastMode: Self = Self(1i32);
}
impl ::core::marker::Copy for ProviderI2cBusSpeed {}
impl ::core::clone::Clone for ProviderI2cBusSpeed {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProviderI2cBusSpeed {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ProviderI2cBusSpeed {
    type Abi = Self;
}
impl ::core::fmt::Debug for ProviderI2cBusSpeed {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProviderI2cBusSpeed").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProviderI2cBusSpeed {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.I2c.Provider.ProviderI2cBusSpeed;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ProviderI2cConnectionSettings(::windows_core::IUnknown);
impl ProviderI2cConnectionSettings {
    pub fn SlaveAddress(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).SlaveAddress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetSlaveAddress(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSlaveAddress)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BusSpeed(&self) -> ::windows_core::Result<ProviderI2cBusSpeed> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ProviderI2cBusSpeed>::zeroed();
            (::windows_core::Interface::vtable(this).BusSpeed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ProviderI2cBusSpeed>(result__)
        }
    }
    pub fn SetBusSpeed(&self, value: ProviderI2cBusSpeed) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBusSpeed)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SharingMode(&self) -> ::windows_core::Result<ProviderI2cSharingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ProviderI2cSharingMode>::zeroed();
            (::windows_core::Interface::vtable(this).SharingMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ProviderI2cSharingMode>(result__)
        }
    }
    pub fn SetSharingMode(&self, value: ProviderI2cSharingMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSharingMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for ProviderI2cConnectionSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProviderI2cConnectionSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProviderI2cConnectionSettings {}
impl ::core::fmt::Debug for ProviderI2cConnectionSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProviderI2cConnectionSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProviderI2cConnectionSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.I2c.Provider.ProviderI2cConnectionSettings;{e9db4e34-e510-44b7-809d-f2f85b555339})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProviderI2cConnectionSettings {
    type Vtable = IProviderI2cConnectionSettings_Vtbl;
    const IID: ::windows_core::GUID = <IProviderI2cConnectionSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProviderI2cConnectionSettings {
    const NAME: &'static str = "Windows.Devices.I2c.Provider.ProviderI2cConnectionSettings";
}
impl ::core::convert::From<ProviderI2cConnectionSettings> for ::windows_core::IUnknown {
    fn from(value: ProviderI2cConnectionSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProviderI2cConnectionSettings> for ::windows_core::IUnknown {
    fn from(value: &ProviderI2cConnectionSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProviderI2cConnectionSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProviderI2cConnectionSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProviderI2cConnectionSettings> for ::windows_core::IInspectable {
    fn from(value: ProviderI2cConnectionSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProviderI2cConnectionSettings> for ::windows_core::IInspectable {
    fn from(value: &ProviderI2cConnectionSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProviderI2cConnectionSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProviderI2cConnectionSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProviderI2cConnectionSettings {}
unsafe impl ::core::marker::Sync for ProviderI2cConnectionSettings {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ProviderI2cSharingMode(pub i32);
impl ProviderI2cSharingMode {
    pub const Exclusive: Self = Self(0i32);
    pub const Shared: Self = Self(1i32);
}
impl ::core::marker::Copy for ProviderI2cSharingMode {}
impl ::core::clone::Clone for ProviderI2cSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProviderI2cSharingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ProviderI2cSharingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for ProviderI2cSharingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProviderI2cSharingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProviderI2cSharingMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.I2c.Provider.ProviderI2cSharingMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
pub struct ProviderI2cTransferResult {
    pub Status: ProviderI2cTransferStatus,
    pub BytesTransferred: u32,
}
impl ::core::marker::Copy for ProviderI2cTransferResult {}
impl ::core::clone::Clone for ProviderI2cTransferResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ProviderI2cTransferResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ProviderI2cTransferResult").field("Status", &self.Status).field("BytesTransferred", &self.BytesTransferred).finish()
    }
}
unsafe impl ::windows_core::Abi for ProviderI2cTransferResult {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for ProviderI2cTransferResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Devices.I2c.Provider.ProviderI2cTransferResult;enum(Windows.Devices.I2c.Provider.ProviderI2cTransferStatus;i4);u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for ProviderI2cTransferResult {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ProviderI2cTransferResult>()) == 0 }
    }
}
impl ::core::cmp::Eq for ProviderI2cTransferResult {}
impl ::core::default::Default for ProviderI2cTransferResult {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ProviderI2cTransferStatus(pub i32);
impl ProviderI2cTransferStatus {
    pub const FullTransfer: Self = Self(0i32);
    pub const PartialTransfer: Self = Self(1i32);
    pub const SlaveAddressNotAcknowledged: Self = Self(2i32);
}
impl ::core::marker::Copy for ProviderI2cTransferStatus {}
impl ::core::clone::Clone for ProviderI2cTransferStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProviderI2cTransferStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ProviderI2cTransferStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for ProviderI2cTransferStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProviderI2cTransferStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProviderI2cTransferStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.I2c.Provider.ProviderI2cTransferStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
