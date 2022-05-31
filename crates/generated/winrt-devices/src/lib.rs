
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#[cfg(feature = "Adc")]
pub mod Adc;
#[cfg(feature = "AllJoyn")]
pub mod AllJoyn;
#[cfg(feature = "Background")]
pub mod Background;
#[cfg(feature = "Bluetooth")]
pub mod Bluetooth;
#[cfg(feature = "Custom")]
pub mod Custom;
#[cfg(feature = "Display")]
pub mod Display;
#[cfg(feature = "Enumeration")]
pub mod Enumeration;
#[cfg(feature = "Geolocation")]
pub mod Geolocation;
#[cfg(feature = "Gpio")]
pub mod Gpio;
#[cfg(feature = "Haptics")]
pub mod Haptics;
#[cfg(feature = "HumanInterfaceDevice")]
pub mod HumanInterfaceDevice;
#[cfg(feature = "I2c")]
pub mod I2c;
#[cfg(feature = "Input")]
pub mod Input;
#[cfg(feature = "Lights")]
pub mod Lights;
#[cfg(feature = "Midi")]
pub mod Midi;
#[cfg(feature = "Perception")]
pub mod Perception;
#[cfg(feature = "PointOfService")]
pub mod PointOfService;
#[cfg(feature = "Portable")]
pub mod Portable;
#[cfg(feature = "Power")]
pub mod Power;
#[cfg(feature = "Printers")]
pub mod Printers;
#[cfg(feature = "Pwm")]
pub mod Pwm;
#[cfg(feature = "Radios")]
pub mod Radios;
#[cfg(feature = "Scanners")]
pub mod Scanners;
#[cfg(feature = "Sensors")]
pub mod Sensors;
#[cfg(feature = "SerialCommunication")]
pub mod SerialCommunication;
#[cfg(feature = "SmartCards")]
pub mod SmartCards;
#[cfg(feature = "Sms")]
pub mod Sms;
#[cfg(feature = "Spi")]
pub mod Spi;
#[cfg(feature = "Usb")]
pub mod Usb;
#[cfg(feature = "WiFi")]
pub mod WiFi;
#[cfg(feature = "WiFiDirect")]
pub mod WiFiDirect;
#[repr(transparent)]
pub struct ILowLevelDevicesAggregateProvider(::windows_core::IUnknown);
impl ILowLevelDevicesAggregateProvider {
    #[cfg(feature = "winrt-devices")]
    pub fn AdcControllerProvider(&self) -> ::windows_core::Result<Adc::Provider::IAdcControllerProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AdcControllerProvider)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Adc::Provider::IAdcControllerProvider>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn PwmControllerProvider(&self) -> ::windows_core::Result<Pwm::Provider::IPwmControllerProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PwmControllerProvider)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Pwm::Provider::IPwmControllerProvider>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn GpioControllerProvider(&self) -> ::windows_core::Result<Gpio::Provider::IGpioControllerProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GpioControllerProvider)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Gpio::Provider::IGpioControllerProvider>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn I2cControllerProvider(&self) -> ::windows_core::Result<I2c::Provider::II2cControllerProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).I2cControllerProvider)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<I2c::Provider::II2cControllerProvider>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn SpiControllerProvider(&self) -> ::windows_core::Result<Spi::Provider::ISpiControllerProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SpiControllerProvider)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Spi::Provider::ISpiControllerProvider>(result__)
        }
    }
}
impl ::core::convert::From<ILowLevelDevicesAggregateProvider> for ::windows_core::IUnknown {
    fn from(value: ILowLevelDevicesAggregateProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILowLevelDevicesAggregateProvider> for ::windows_core::IUnknown {
    fn from(value: &ILowLevelDevicesAggregateProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ILowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ILowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ILowLevelDevicesAggregateProvider> for ::windows_core::IInspectable {
    fn from(value: ILowLevelDevicesAggregateProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILowLevelDevicesAggregateProvider> for ::windows_core::IInspectable {
    fn from(value: &ILowLevelDevicesAggregateProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ILowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ILowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILowLevelDevicesAggregateProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILowLevelDevicesAggregateProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILowLevelDevicesAggregateProvider {}
impl ::core::fmt::Debug for ILowLevelDevicesAggregateProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILowLevelDevicesAggregateProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ILowLevelDevicesAggregateProvider {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{a73e561c-aac1-4ec7-a852-479f7060d01f}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ILowLevelDevicesAggregateProvider {
    type Vtable = ILowLevelDevicesAggregateProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa73e561c_aac1_4ec7_a852_479f7060d01f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLevelDevicesAggregateProvider_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-devices")]
    pub AdcControllerProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    AdcControllerProvider: usize,
    #[cfg(feature = "winrt-devices")]
    pub PwmControllerProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    PwmControllerProvider: usize,
    #[cfg(feature = "winrt-devices")]
    pub GpioControllerProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    GpioControllerProvider: usize,
    #[cfg(feature = "winrt-devices")]
    pub I2cControllerProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    I2cControllerProvider: usize,
    #[cfg(feature = "winrt-devices")]
    pub SpiControllerProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    SpiControllerProvider: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLevelDevicesAggregateProviderFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILowLevelDevicesAggregateProviderFactory {
    type Vtable = ILowLevelDevicesAggregateProviderFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9ac4aaf6_3473_465e_96d5_36281a2c57af);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLevelDevicesAggregateProviderFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "winrt-devices", feature = "winrt-devices", feature = "winrt-devices", feature = "winrt-devices", feature = "winrt-devices"))]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adc: ::windows_core::RawPtr, pwm: ::windows_core::RawPtr, gpio: ::windows_core::RawPtr, i2c: ::windows_core::RawPtr, spi: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-devices", feature = "winrt-devices", feature = "winrt-devices", feature = "winrt-devices", feature = "winrt-devices")))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLevelDevicesController(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILowLevelDevicesController {
    type Vtable = ILowLevelDevicesController_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2ec23dd4_179b_45de_9b39_3ae02527de52);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLevelDevicesController_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLevelDevicesControllerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILowLevelDevicesControllerStatics {
    type Vtable = ILowLevelDevicesControllerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x093e926a_fccb_4394_a697_19de637c2db3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLevelDevicesControllerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DefaultProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetDefaultProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct LowLevelDevicesAggregateProvider(::windows_core::IUnknown);
impl LowLevelDevicesAggregateProvider {
    #[cfg(feature = "winrt-devices")]
    pub fn AdcControllerProvider(&self) -> ::windows_core::Result<Adc::Provider::IAdcControllerProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AdcControllerProvider)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Adc::Provider::IAdcControllerProvider>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn PwmControllerProvider(&self) -> ::windows_core::Result<Pwm::Provider::IPwmControllerProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PwmControllerProvider)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Pwm::Provider::IPwmControllerProvider>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn GpioControllerProvider(&self) -> ::windows_core::Result<Gpio::Provider::IGpioControllerProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GpioControllerProvider)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Gpio::Provider::IGpioControllerProvider>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn I2cControllerProvider(&self) -> ::windows_core::Result<I2c::Provider::II2cControllerProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).I2cControllerProvider)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<I2c::Provider::II2cControllerProvider>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn SpiControllerProvider(&self) -> ::windows_core::Result<Spi::Provider::ISpiControllerProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SpiControllerProvider)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Spi::Provider::ISpiControllerProvider>(result__)
        }
    }
    #[cfg(all(feature = "winrt-devices", feature = "winrt-devices", feature = "winrt-devices", feature = "winrt-devices", feature = "winrt-devices"))]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, Adc::Provider::IAdcControllerProvider>, Param1: ::windows_core::IntoParam<'a, Pwm::Provider::IPwmControllerProvider>, Param2: ::windows_core::IntoParam<'a, Gpio::Provider::IGpioControllerProvider>, Param3: ::windows_core::IntoParam<'a, I2c::Provider::II2cControllerProvider>, Param4: ::windows_core::IntoParam<'a, Spi::Provider::ISpiControllerProvider>>(adc: Param0, pwm: Param1, gpio: Param2, i2c: Param3, spi: Param4) -> ::windows_core::Result<LowLevelDevicesAggregateProvider> {
        Self::ILowLevelDevicesAggregateProviderFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), adc.into_param().abi(), pwm.into_param().abi(), gpio.into_param().abi(), i2c.into_param().abi(), spi.into_param().abi(), result__.as_mut_ptr()).from_abi::<LowLevelDevicesAggregateProvider>(result__)
        })
    }
    pub fn ILowLevelDevicesAggregateProviderFactory<R, F: FnOnce(&ILowLevelDevicesAggregateProviderFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LowLevelDevicesAggregateProvider, ILowLevelDevicesAggregateProviderFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LowLevelDevicesAggregateProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LowLevelDevicesAggregateProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LowLevelDevicesAggregateProvider {}
impl ::core::fmt::Debug for LowLevelDevicesAggregateProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LowLevelDevicesAggregateProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LowLevelDevicesAggregateProvider {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.LowLevelDevicesAggregateProvider;{a73e561c-aac1-4ec7-a852-479f7060d01f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LowLevelDevicesAggregateProvider {
    type Vtable = ILowLevelDevicesAggregateProvider_Vtbl;
    const IID: ::windows_core::GUID = <ILowLevelDevicesAggregateProvider as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LowLevelDevicesAggregateProvider {
    const NAME: &'static str = "Windows.Devices.LowLevelDevicesAggregateProvider";
}
impl ::core::convert::From<LowLevelDevicesAggregateProvider> for ::windows_core::IUnknown {
    fn from(value: LowLevelDevicesAggregateProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LowLevelDevicesAggregateProvider> for ::windows_core::IUnknown {
    fn from(value: &LowLevelDevicesAggregateProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LowLevelDevicesAggregateProvider> for ::windows_core::IInspectable {
    fn from(value: LowLevelDevicesAggregateProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LowLevelDevicesAggregateProvider> for ::windows_core::IInspectable {
    fn from(value: &LowLevelDevicesAggregateProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LowLevelDevicesAggregateProvider> for ILowLevelDevicesAggregateProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: LowLevelDevicesAggregateProvider) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LowLevelDevicesAggregateProvider> for ILowLevelDevicesAggregateProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: &LowLevelDevicesAggregateProvider) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILowLevelDevicesAggregateProvider> for LowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ILowLevelDevicesAggregateProvider> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILowLevelDevicesAggregateProvider> for &LowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ILowLevelDevicesAggregateProvider> {
        ::core::convert::TryInto::<ILowLevelDevicesAggregateProvider>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LowLevelDevicesAggregateProvider {}
unsafe impl ::core::marker::Sync for LowLevelDevicesAggregateProvider {}
#[repr(transparent)]
pub struct LowLevelDevicesController(::windows_core::IUnknown);
impl LowLevelDevicesController {
    pub fn DefaultProvider() -> ::windows_core::Result<ILowLevelDevicesAggregateProvider> {
        Self::ILowLevelDevicesControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultProvider)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ILowLevelDevicesAggregateProvider>(result__)
        })
    }
    pub fn SetDefaultProvider<'a, Param0: ::windows_core::IntoParam<'a, ILowLevelDevicesAggregateProvider>>(value: Param0) -> ::windows_core::Result<()> {
        Self::ILowLevelDevicesControllerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetDefaultProvider)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() })
    }
    pub fn ILowLevelDevicesControllerStatics<R, F: FnOnce(&ILowLevelDevicesControllerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LowLevelDevicesController, ILowLevelDevicesControllerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LowLevelDevicesController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LowLevelDevicesController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LowLevelDevicesController {}
impl ::core::fmt::Debug for LowLevelDevicesController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LowLevelDevicesController").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LowLevelDevicesController {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.LowLevelDevicesController;{2ec23dd4-179b-45de-9b39-3ae02527de52})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LowLevelDevicesController {
    type Vtable = ILowLevelDevicesController_Vtbl;
    const IID: ::windows_core::GUID = <ILowLevelDevicesController as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LowLevelDevicesController {
    const NAME: &'static str = "Windows.Devices.LowLevelDevicesController";
}
impl ::core::convert::From<LowLevelDevicesController> for ::windows_core::IUnknown {
    fn from(value: LowLevelDevicesController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LowLevelDevicesController> for ::windows_core::IUnknown {
    fn from(value: &LowLevelDevicesController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LowLevelDevicesController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LowLevelDevicesController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LowLevelDevicesController> for ::windows_core::IInspectable {
    fn from(value: LowLevelDevicesController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LowLevelDevicesController> for ::windows_core::IInspectable {
    fn from(value: &LowLevelDevicesController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LowLevelDevicesController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LowLevelDevicesController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LowLevelDevicesController {}
unsafe impl ::core::marker::Sync for LowLevelDevicesController {}
