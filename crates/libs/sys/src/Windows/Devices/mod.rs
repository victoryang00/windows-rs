#[cfg(feature = "Devices_Adc")]
pub mod Adc;
#[cfg(feature = "Devices_AllJoyn")]
pub mod AllJoyn;
#[cfg(feature = "Devices_Background")]
pub mod Background;
#[cfg(feature = "Devices_Bluetooth")]
pub mod Bluetooth;
#[cfg(feature = "Devices_Custom")]
pub mod Custom;
#[cfg(feature = "Devices_Display")]
pub mod Display;
#[cfg(feature = "Devices_Enumeration")]
pub mod Enumeration;
#[cfg(feature = "Devices_Geolocation")]
pub mod Geolocation;
#[cfg(feature = "Devices_Gpio")]
pub mod Gpio;
#[cfg(feature = "Devices_Haptics")]
pub mod Haptics;
#[cfg(feature = "Devices_HumanInterfaceDevice")]
pub mod HumanInterfaceDevice;
#[cfg(feature = "Devices_I2c")]
pub mod I2c;
#[cfg(feature = "Devices_Input")]
pub mod Input;
#[cfg(feature = "Devices_Lights")]
pub mod Lights;
#[cfg(feature = "Devices_Midi")]
pub mod Midi;
#[cfg(feature = "Devices_Perception")]
pub mod Perception;
#[cfg(feature = "Devices_PointOfService")]
pub mod PointOfService;
#[cfg(feature = "Devices_Portable")]
pub mod Portable;
#[cfg(feature = "Devices_Power")]
pub mod Power;
#[cfg(feature = "Devices_Printers")]
pub mod Printers;
#[cfg(feature = "Devices_Pwm")]
pub mod Pwm;
#[cfg(feature = "Devices_Radios")]
pub mod Radios;
#[cfg(feature = "Devices_Scanners")]
pub mod Scanners;
#[cfg(feature = "Devices_Sensors")]
pub mod Sensors;
#[cfg(feature = "Devices_SerialCommunication")]
pub mod SerialCommunication;
#[cfg(feature = "Devices_SmartCards")]
pub mod SmartCards;
#[cfg(feature = "Devices_Sms")]
pub mod Sms;
#[cfg(feature = "Devices_Spi")]
pub mod Spi;
#[cfg(feature = "Devices_Usb")]
pub mod Usb;
#[cfg(feature = "Devices_WiFi")]
pub mod WiFi;
#[cfg(feature = "Devices_WiFiDirect")]
pub mod WiFiDirect;
#[repr(C)]
pub struct ILowLevelDevicesAggregateProvider {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Adc_Provider")]
    pub AdcControllerProvider: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Adc_Provider"))]
    AdcControllerProvider: usize,
    #[cfg(feature = "Devices_Pwm_Provider")]
    pub PwmControllerProvider: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Pwm_Provider"))]
    PwmControllerProvider: usize,
    #[cfg(feature = "Devices_Gpio_Provider")]
    pub GpioControllerProvider: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Gpio_Provider"))]
    GpioControllerProvider: usize,
    #[cfg(feature = "Devices_I2c_Provider")]
    pub I2cControllerProvider: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_I2c_Provider"))]
    I2cControllerProvider: usize,
    #[cfg(feature = "Devices_Spi_Provider")]
    pub SpiControllerProvider: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Spi_Provider"))]
    SpiControllerProvider: usize,
}
impl ::windows_sys::core::Interface for ILowLevelDevicesAggregateProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2805880348, data2: 43713, data3: 20167, data4: [168, 82, 71, 159, 112, 96, 208, 31] };
}
#[repr(C)]
pub struct ILowLevelDevicesAggregateProviderFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider"))]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, adc: *mut ::core::ffi::c_void, pwm: *mut ::core::ffi::c_void, gpio: *mut ::core::ffi::c_void, i2c: *mut ::core::ffi::c_void, spi: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider")))]
    Create: usize,
}
impl ::windows_sys::core::Interface for ILowLevelDevicesAggregateProviderFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2596580086, data2: 13427, data3: 18014, data4: [150, 213, 54, 40, 26, 44, 87, 175] };
}
#[repr(C)]
pub struct ILowLevelDevicesController {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ILowLevelDevicesController {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 784481748, data2: 6043, data3: 17886, data4: [155, 57, 58, 224, 37, 39, 222, 82] };
}
#[repr(C)]
pub struct ILowLevelDevicesControllerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub DefaultProvider: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetDefaultProvider: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILowLevelDevicesControllerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 155095658, data2: 64715, data3: 17300, data4: [166, 151, 25, 222, 99, 124, 45, 179] };
}
pub type LowLevelDevicesAggregateProvider = *mut ::core::ffi::c_void;
pub type LowLevelDevicesController = *mut ::core::ffi::c_void;
