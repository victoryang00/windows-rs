#[cfg(feature = "Provider")]
pub mod Provider;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct I2cBusSpeed(pub i32);
impl I2cBusSpeed {
    pub const StandardMode: Self = Self(0i32);
    pub const FastMode: Self = Self(1i32);
}
impl ::core::marker::Copy for I2cBusSpeed {}
impl ::core::clone::Clone for I2cBusSpeed {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for I2cBusSpeed {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for I2cBusSpeed {
    type Abi = Self;
}
impl ::core::fmt::Debug for I2cBusSpeed {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("I2cBusSpeed").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for I2cBusSpeed {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.I2c.I2cBusSpeed;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct I2cConnectionSettings(::windows_core::IUnknown);
impl I2cConnectionSettings {
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
    pub fn BusSpeed(&self) -> ::windows_core::Result<I2cBusSpeed> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<I2cBusSpeed>::zeroed();
            (::windows_core::Interface::vtable(this).BusSpeed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<I2cBusSpeed>(result__)
        }
    }
    pub fn SetBusSpeed(&self, value: I2cBusSpeed) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBusSpeed)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SharingMode(&self) -> ::windows_core::Result<I2cSharingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<I2cSharingMode>::zeroed();
            (::windows_core::Interface::vtable(this).SharingMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<I2cSharingMode>(result__)
        }
    }
    pub fn SetSharingMode(&self, value: I2cSharingMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSharingMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create(slaveaddress: i32) -> ::windows_core::Result<I2cConnectionSettings> {
        Self::II2cConnectionSettingsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), slaveaddress, result__.as_mut_ptr()).from_abi::<I2cConnectionSettings>(result__)
        })
    }
    pub fn II2cConnectionSettingsFactory<R, F: FnOnce(&II2cConnectionSettingsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<I2cConnectionSettings, II2cConnectionSettingsFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for I2cConnectionSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for I2cConnectionSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for I2cConnectionSettings {}
impl ::core::fmt::Debug for I2cConnectionSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("I2cConnectionSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for I2cConnectionSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.I2c.I2cConnectionSettings;{f2db1307-ab6f-4639-a767-54536dc3460f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for I2cConnectionSettings {
    type Vtable = II2cConnectionSettings_Vtbl;
    const IID: ::windows_core::GUID = <II2cConnectionSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for I2cConnectionSettings {
    const NAME: &'static str = "Windows.Devices.I2c.I2cConnectionSettings";
}
impl ::core::convert::From<I2cConnectionSettings> for ::windows_core::IUnknown {
    fn from(value: I2cConnectionSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&I2cConnectionSettings> for ::windows_core::IUnknown {
    fn from(value: &I2cConnectionSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for I2cConnectionSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a I2cConnectionSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<I2cConnectionSettings> for ::windows_core::IInspectable {
    fn from(value: I2cConnectionSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&I2cConnectionSettings> for ::windows_core::IInspectable {
    fn from(value: &I2cConnectionSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for I2cConnectionSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a I2cConnectionSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for I2cConnectionSettings {}
unsafe impl ::core::marker::Sync for I2cConnectionSettings {}
#[repr(transparent)]
pub struct I2cController(::windows_core::IUnknown);
impl I2cController {
    pub fn GetDevice<'a, Param0: ::windows_core::IntoParam<'a, I2cConnectionSettings>>(&self, settings: Param0) -> ::windows_core::Result<I2cDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDevice)(::windows_core::Interface::as_raw(this), settings.into_param().abi(), result__.as_mut_ptr()).from_abi::<I2cDevice>(result__)
        }
    }
    #[cfg(all(feature = "Devices_I2c_Provider", feature = "Foundation_Collections"))]
    pub fn GetControllersAsync<'a, Param0: ::windows_core::IntoParam<'a, Provider::II2cProvider>>(provider: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<I2cController>>> {
        Self::II2cControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetControllersAsync)(::windows_core::Interface::as_raw(this), provider.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<I2cController>>>(result__)
        })
    }
    pub fn GetDefaultAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<I2cController>> {
        Self::II2cControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<I2cController>>(result__)
        })
    }
    pub fn II2cControllerStatics<R, F: FnOnce(&II2cControllerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<I2cController, II2cControllerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for I2cController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for I2cController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for I2cController {}
impl ::core::fmt::Debug for I2cController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("I2cController").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for I2cController {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.I2c.I2cController;{c48ab1b2-87a0-4166-8e3e-b4b8f97cd729})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for I2cController {
    type Vtable = II2cController_Vtbl;
    const IID: ::windows_core::GUID = <II2cController as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for I2cController {
    const NAME: &'static str = "Windows.Devices.I2c.I2cController";
}
impl ::core::convert::From<I2cController> for ::windows_core::IUnknown {
    fn from(value: I2cController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&I2cController> for ::windows_core::IUnknown {
    fn from(value: &I2cController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for I2cController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a I2cController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<I2cController> for ::windows_core::IInspectable {
    fn from(value: I2cController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&I2cController> for ::windows_core::IInspectable {
    fn from(value: &I2cController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for I2cController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a I2cController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for I2cController {}
unsafe impl ::core::marker::Sync for I2cController {}
#[repr(transparent)]
pub struct I2cDevice(::windows_core::IUnknown);
impl I2cDevice {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ConnectionSettings(&self) -> ::windows_core::Result<I2cConnectionSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionSettings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<I2cConnectionSettings>(result__)
        }
    }
    pub fn Write(&self, buffer: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Write)(::windows_core::Interface::as_raw(this), buffer.len() as u32, ::core::mem::transmute(buffer.as_ptr())).ok() }
    }
    pub fn WritePartial(&self, buffer: &[u8]) -> ::windows_core::Result<I2cTransferResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<I2cTransferResult>::zeroed();
            (::windows_core::Interface::vtable(this).WritePartial)(::windows_core::Interface::as_raw(this), buffer.len() as u32, ::core::mem::transmute(buffer.as_ptr()), result__.as_mut_ptr()).from_abi::<I2cTransferResult>(result__)
        }
    }
    pub fn Read(&self, buffer: &mut [u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Read)(::windows_core::Interface::as_raw(this), buffer.len() as u32, ::core::mem::transmute_copy(&buffer)).ok() }
    }
    pub fn ReadPartial(&self, buffer: &mut [u8]) -> ::windows_core::Result<I2cTransferResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<I2cTransferResult>::zeroed();
            (::windows_core::Interface::vtable(this).ReadPartial)(::windows_core::Interface::as_raw(this), buffer.len() as u32, ::core::mem::transmute_copy(&buffer), result__.as_mut_ptr()).from_abi::<I2cTransferResult>(result__)
        }
    }
    pub fn WriteRead(&self, writebuffer: &[u8], readbuffer: &mut [u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteRead)(::windows_core::Interface::as_raw(this), writebuffer.len() as u32, ::core::mem::transmute(writebuffer.as_ptr()), readbuffer.len() as u32, ::core::mem::transmute_copy(&readbuffer)).ok() }
    }
    pub fn WriteReadPartial(&self, writebuffer: &[u8], readbuffer: &mut [u8]) -> ::windows_core::Result<I2cTransferResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<I2cTransferResult>::zeroed();
            (::windows_core::Interface::vtable(this).WriteReadPartial)(::windows_core::Interface::as_raw(this), writebuffer.len() as u32, ::core::mem::transmute(writebuffer.as_ptr()), readbuffer.len() as u32, ::core::mem::transmute_copy(&readbuffer), result__.as_mut_ptr()).from_abi::<I2cTransferResult>(result__)
        }
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::II2cDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorFromFriendlyName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(friendlyname: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::II2cDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelectorFromFriendlyName)(::windows_core::Interface::as_raw(this), friendlyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, I2cConnectionSettings>>(deviceid: Param0, settings: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<I2cDevice>> {
        Self::II2cDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), settings.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<I2cDevice>>(result__)
        })
    }
    pub fn II2cDeviceStatics<R, F: FnOnce(&II2cDeviceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<I2cDevice, II2cDeviceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for I2cDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for I2cDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for I2cDevice {}
impl ::core::fmt::Debug for I2cDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("I2cDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for I2cDevice {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.I2c.I2cDevice;{8636c136-b9c5-4f70-9449-cc46dc6f57eb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for I2cDevice {
    type Vtable = II2cDevice_Vtbl;
    const IID: ::windows_core::GUID = <II2cDevice as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for I2cDevice {
    const NAME: &'static str = "Windows.Devices.I2c.I2cDevice";
}
impl ::core::convert::From<I2cDevice> for ::windows_core::IUnknown {
    fn from(value: I2cDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&I2cDevice> for ::windows_core::IUnknown {
    fn from(value: &I2cDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for I2cDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a I2cDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<I2cDevice> for ::windows_core::IInspectable {
    fn from(value: I2cDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&I2cDevice> for ::windows_core::IInspectable {
    fn from(value: &I2cDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for I2cDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a I2cDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<I2cDevice> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: I2cDevice) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&I2cDevice> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &I2cDevice) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for I2cDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &I2cDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for I2cDevice {}
unsafe impl ::core::marker::Sync for I2cDevice {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct I2cSharingMode(pub i32);
impl I2cSharingMode {
    pub const Exclusive: Self = Self(0i32);
    pub const Shared: Self = Self(1i32);
}
impl ::core::marker::Copy for I2cSharingMode {}
impl ::core::clone::Clone for I2cSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for I2cSharingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for I2cSharingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for I2cSharingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("I2cSharingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for I2cSharingMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.I2c.I2cSharingMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
pub struct I2cTransferResult {
    pub Status: I2cTransferStatus,
    pub BytesTransferred: u32,
}
impl ::core::marker::Copy for I2cTransferResult {}
impl ::core::clone::Clone for I2cTransferResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for I2cTransferResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("I2cTransferResult").field("Status", &self.Status).field("BytesTransferred", &self.BytesTransferred).finish()
    }
}
unsafe impl ::windows_core::Abi for I2cTransferResult {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for I2cTransferResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Devices.I2c.I2cTransferResult;enum(Windows.Devices.I2c.I2cTransferStatus;i4);u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for I2cTransferResult {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<I2cTransferResult>()) == 0 }
    }
}
impl ::core::cmp::Eq for I2cTransferResult {}
impl ::core::default::Default for I2cTransferResult {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct I2cTransferStatus(pub i32);
impl I2cTransferStatus {
    pub const FullTransfer: Self = Self(0i32);
    pub const PartialTransfer: Self = Self(1i32);
    pub const SlaveAddressNotAcknowledged: Self = Self(2i32);
    pub const ClockStretchTimeout: Self = Self(3i32);
    pub const UnknownError: Self = Self(4i32);
}
impl ::core::marker::Copy for I2cTransferStatus {}
impl ::core::clone::Clone for I2cTransferStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for I2cTransferStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for I2cTransferStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for I2cTransferStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("I2cTransferStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for I2cTransferStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.I2c.I2cTransferStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct II2cConnectionSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for II2cConnectionSettings {
    type Vtable = II2cConnectionSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf2db1307_ab6f_4639_a767_54536dc3460f);
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cConnectionSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SlaveAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetSlaveAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub BusSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut I2cBusSpeed) -> ::windows_core::HRESULT,
    pub SetBusSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: I2cBusSpeed) -> ::windows_core::HRESULT,
    pub SharingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut I2cSharingMode) -> ::windows_core::HRESULT,
    pub SetSharingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: I2cSharingMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct II2cConnectionSettingsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for II2cConnectionSettingsFactory {
    type Vtable = II2cConnectionSettingsFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x81b586b3_9693_41b1_a243_ded4f6e66926);
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cConnectionSettingsFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, slaveaddress: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct II2cController(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for II2cController {
    type Vtable = II2cController_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc48ab1b2_87a0_4166_8e3e_b4b8f97cd729);
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cController_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct II2cControllerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for II2cControllerStatics {
    type Vtable = II2cControllerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x40fc0365_5f05_4e7e_84bd_100db8e0aec5);
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cControllerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Devices_I2c_Provider", feature = "Foundation_Collections"))]
    pub GetControllersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_I2c_Provider", feature = "Foundation_Collections")))]
    GetControllersAsync: usize,
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct II2cDevice(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for II2cDevice {
    type Vtable = II2cDevice_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8636c136_b9c5_4f70_9449_cc46dc6f57eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cDevice_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ConnectionSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *const u8) -> ::windows_core::HRESULT,
    pub WritePartial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *const u8, result__: *mut I2cTransferResult) -> ::windows_core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8) -> ::windows_core::HRESULT,
    pub ReadPartial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8, result__: *mut I2cTransferResult) -> ::windows_core::HRESULT,
    pub WriteRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> ::windows_core::HRESULT,
    pub WriteReadPartial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8, result__: *mut I2cTransferResult) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct II2cDeviceStatics(::windows_core::IUnknown);
impl II2cDeviceStatics {
    pub fn GetDeviceSelector(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetDeviceSelectorFromFriendlyName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, friendlyname: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelectorFromFriendlyName)(::windows_core::Interface::as_raw(this), friendlyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, I2cConnectionSettings>>(&self, deviceid: Param0, settings: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<I2cDevice>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), settings.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<I2cDevice>>(result__)
        }
    }
}
impl ::core::convert::From<II2cDeviceStatics> for ::windows_core::IUnknown {
    fn from(value: II2cDeviceStatics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&II2cDeviceStatics> for ::windows_core::IUnknown {
    fn from(value: &II2cDeviceStatics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for II2cDeviceStatics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a II2cDeviceStatics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<II2cDeviceStatics> for ::windows_core::IInspectable {
    fn from(value: II2cDeviceStatics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&II2cDeviceStatics> for ::windows_core::IInspectable {
    fn from(value: &II2cDeviceStatics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for II2cDeviceStatics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a II2cDeviceStatics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for II2cDeviceStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for II2cDeviceStatics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for II2cDeviceStatics {}
impl ::core::fmt::Debug for II2cDeviceStatics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("II2cDeviceStatics").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for II2cDeviceStatics {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{91a33be3-7334-4512-96bc-fbae9459f5f6}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for II2cDeviceStatics {
    type Vtable = II2cDeviceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x91a33be3_7334_4512_96bc_fbae9459f5f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cDeviceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetDeviceSelectorFromFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, friendlyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, settings: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
