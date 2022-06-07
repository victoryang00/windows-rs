pub type GpioPinProviderValueChangedEventArgs = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IGpioControllerProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub PinCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub OpenPinProvider: unsafe extern "system" fn(this: *mut *mut Self, pin: i32, sharingmode: ProviderGpioSharingMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGpioControllerProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2903625415, data2: 6634, data3: 19233, data4: [135, 79, 185, 26, 237, 74, 37, 219] };
}
#[repr(C)]
pub struct IGpioPinProvider {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ValueChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ValueChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveValueChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveValueChanged: usize,
    #[cfg(feature = "Foundation")]
    pub DebounceTimeout: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DebounceTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub SetDebounceTimeout: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDebounceTimeout: usize,
    pub PinNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SharingMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ProviderGpioSharingMode) -> ::windows_sys::core::HRESULT,
    pub IsDriveModeSupported: unsafe extern "system" fn(this: *mut *mut Self, drivemode: ProviderGpioPinDriveMode, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub GetDriveMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ProviderGpioPinDriveMode) -> ::windows_sys::core::HRESULT,
    pub SetDriveMode: unsafe extern "system" fn(this: *mut *mut Self, value: ProviderGpioPinDriveMode) -> ::windows_sys::core::HRESULT,
    pub Write: unsafe extern "system" fn(this: *mut *mut Self, value: ProviderGpioPinValue) -> ::windows_sys::core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ProviderGpioPinValue) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGpioPinProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1110723767, data2: 27324, data3: 16639, data4: [156, 231, 115, 184, 83, 1, 185, 0] };
}
#[repr(C)]
pub struct IGpioPinProviderValueChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Edge: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ProviderGpioPinEdge) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGpioPinProviderValueChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 849794802, data2: 15707, data3: 17613, data4: [143, 190, 19, 166, 159, 46, 219, 36] };
}
#[repr(C)]
pub struct IGpioPinProviderValueChangedEventArgsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, edge: ProviderGpioPinEdge, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGpioPinProviderValueChangedEventArgsFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1053494105, data2: 22156, data3: 17298, data4: [178, 74, 138, 89, 169, 2, 177, 241] };
}
#[repr(C)]
pub struct IGpioProvider {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetControllers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetControllers: usize,
}
impl ::windows_sys::core::Interface for IGpioProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1156065031, data2: 2250, data3: 17226, data4: [175, 224, 214, 21, 128, 68, 111, 126] };
}
#[doc = "*Required features: `\"Devices_Gpio_Provider\"`*"]
#[repr(transparent)]
pub struct ProviderGpioPinDriveMode(pub i32);
impl ProviderGpioPinDriveMode {
    pub const Input: Self = Self(0i32);
    pub const Output: Self = Self(1i32);
    pub const InputPullUp: Self = Self(2i32);
    pub const InputPullDown: Self = Self(3i32);
    pub const OutputOpenDrain: Self = Self(4i32);
    pub const OutputOpenDrainPullUp: Self = Self(5i32);
    pub const OutputOpenSource: Self = Self(6i32);
    pub const OutputOpenSourcePullDown: Self = Self(7i32);
}
impl ::core::marker::Copy for ProviderGpioPinDriveMode {}
impl ::core::clone::Clone for ProviderGpioPinDriveMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Gpio_Provider\"`*"]
#[repr(transparent)]
pub struct ProviderGpioPinEdge(pub i32);
impl ProviderGpioPinEdge {
    pub const FallingEdge: Self = Self(0i32);
    pub const RisingEdge: Self = Self(1i32);
}
impl ::core::marker::Copy for ProviderGpioPinEdge {}
impl ::core::clone::Clone for ProviderGpioPinEdge {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Gpio_Provider\"`*"]
#[repr(transparent)]
pub struct ProviderGpioPinValue(pub i32);
impl ProviderGpioPinValue {
    pub const Low: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl ::core::marker::Copy for ProviderGpioPinValue {}
impl ::core::clone::Clone for ProviderGpioPinValue {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Gpio_Provider\"`*"]
#[repr(transparent)]
pub struct ProviderGpioSharingMode(pub i32);
impl ProviderGpioSharingMode {
    pub const Exclusive: Self = Self(0i32);
    pub const SharedReadOnly: Self = Self(1i32);
}
impl ::core::marker::Copy for ProviderGpioSharingMode {}
impl ::core::clone::Clone for ProviderGpioSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
