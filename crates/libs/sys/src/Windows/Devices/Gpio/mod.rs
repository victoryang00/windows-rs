#[cfg(feature = "Devices_Gpio_Provider")]
pub mod Provider;
#[repr(C)]
#[doc = "*Required features: `\"Devices_Gpio\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
pub struct GpioChangeCount {
    pub Count: u64,
    pub RelativeTime: super::super::Foundation::TimeSpan,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for GpioChangeCount {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for GpioChangeCount {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GpioChangeCounter = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Gpio\"`*"]
#[repr(transparent)]
pub struct GpioChangePolarity(pub i32);
impl GpioChangePolarity {
    pub const Falling: Self = Self(0i32);
    pub const Rising: Self = Self(1i32);
    pub const Both: Self = Self(2i32);
}
impl ::core::marker::Copy for GpioChangePolarity {}
impl ::core::clone::Clone for GpioChangePolarity {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GpioChangeReader = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"Devices_Gpio\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
pub struct GpioChangeRecord {
    pub RelativeTime: super::super::Foundation::TimeSpan,
    pub Edge: GpioPinEdge,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for GpioChangeRecord {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for GpioChangeRecord {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GpioController = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Gpio\"`*"]
#[repr(transparent)]
pub struct GpioOpenStatus(pub i32);
impl GpioOpenStatus {
    pub const PinOpened: Self = Self(0i32);
    pub const PinUnavailable: Self = Self(1i32);
    pub const SharingViolation: Self = Self(2i32);
    pub const MuxingConflict: Self = Self(3i32);
    pub const UnknownError: Self = Self(4i32);
}
impl ::core::marker::Copy for GpioOpenStatus {}
impl ::core::clone::Clone for GpioOpenStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GpioPin = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Gpio\"`*"]
#[repr(transparent)]
pub struct GpioPinDriveMode(pub i32);
impl GpioPinDriveMode {
    pub const Input: Self = Self(0i32);
    pub const Output: Self = Self(1i32);
    pub const InputPullUp: Self = Self(2i32);
    pub const InputPullDown: Self = Self(3i32);
    pub const OutputOpenDrain: Self = Self(4i32);
    pub const OutputOpenDrainPullUp: Self = Self(5i32);
    pub const OutputOpenSource: Self = Self(6i32);
    pub const OutputOpenSourcePullDown: Self = Self(7i32);
}
impl ::core::marker::Copy for GpioPinDriveMode {}
impl ::core::clone::Clone for GpioPinDriveMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Gpio\"`*"]
#[repr(transparent)]
pub struct GpioPinEdge(pub i32);
impl GpioPinEdge {
    pub const FallingEdge: Self = Self(0i32);
    pub const RisingEdge: Self = Self(1i32);
}
impl ::core::marker::Copy for GpioPinEdge {}
impl ::core::clone::Clone for GpioPinEdge {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Gpio\"`*"]
#[repr(transparent)]
pub struct GpioPinValue(pub i32);
impl GpioPinValue {
    pub const Low: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl ::core::marker::Copy for GpioPinValue {}
impl ::core::clone::Clone for GpioPinValue {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GpioPinValueChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Gpio\"`*"]
#[repr(transparent)]
pub struct GpioSharingMode(pub i32);
impl GpioSharingMode {
    pub const Exclusive: Self = Self(0i32);
    pub const SharedReadOnly: Self = Self(1i32);
}
impl ::core::marker::Copy for GpioSharingMode {}
impl ::core::clone::Clone for GpioSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IGpioChangeCounter {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetPolarity: unsafe extern "system" fn(this: *mut *mut Self, value: GpioChangePolarity) -> ::windows_sys::core::HRESULT,
    pub Polarity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GpioChangePolarity) -> ::windows_sys::core::HRESULT,
    pub IsStarted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Read: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GpioChangeCount) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Read: usize,
    #[cfg(feature = "Foundation")]
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GpioChangeCount) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Reset: usize,
}
impl ::windows_sys::core::Interface for IGpioChangeCounter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3411984606, data2: 26625, data3: 17407, data4: [128, 61, 69, 118, 98, 138, 139, 38] };
}
#[repr(C)]
pub struct IGpioChangeCounterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, pin: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGpioChangeCounterFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 343774390, data2: 2718, data3: 16652, data4: [180, 250, 248, 159, 64, 82, 8, 77] };
}
#[repr(C)]
pub struct IGpioChangeReader {
    pub base__: ::windows_sys::core::IInspectable,
    pub Capacity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub IsEmpty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsOverflowed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetPolarity: unsafe extern "system" fn(this: *mut *mut Self, value: GpioChangePolarity) -> ::windows_sys::core::HRESULT,
    pub Polarity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GpioChangePolarity) -> ::windows_sys::core::HRESULT,
    pub IsStarted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetNextItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GpioChangeRecord) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetNextItem: usize,
    #[cfg(feature = "Foundation")]
    pub PeekNextItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GpioChangeRecord) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PeekNextItem: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllItems: usize,
    #[cfg(feature = "Foundation")]
    pub WaitForItemsAsync: unsafe extern "system" fn(this: *mut *mut Self, count: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WaitForItemsAsync: usize,
}
impl ::windows_sys::core::Interface for IGpioChangeReader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 180127839, data2: 57393, data3: 18664, data4: [133, 144, 112, 222, 120, 54, 60, 109] };
}
#[repr(C)]
pub struct IGpioChangeReaderFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, pin: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithCapacity: unsafe extern "system" fn(this: *mut *mut Self, pin: *mut ::core::ffi::c_void, mincapacity: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGpioChangeReaderFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2841218803, data2: 14606, data3: 17434, data4: [157, 28, 232, 222, 11, 45, 240, 223] };
}
#[repr(C)]
pub struct IGpioController {
    pub base__: ::windows_sys::core::IInspectable,
    pub PinCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub OpenPin: unsafe extern "system" fn(this: *mut *mut Self, pinnumber: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OpenPinWithSharingMode: unsafe extern "system" fn(this: *mut *mut Self, pinnumber: i32, sharingmode: GpioSharingMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryOpenPin: unsafe extern "system" fn(this: *mut *mut Self, pinnumber: i32, sharingmode: GpioSharingMode, pin: *mut *mut ::core::ffi::c_void, openstatus: *mut GpioOpenStatus, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGpioController {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 675287779, data2: 29793, data3: 18076, data4: [168, 188, 97, 214, 157, 8, 165, 60] };
}
#[repr(C)]
pub struct IGpioControllerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGpioControllerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 785839150, data2: 31479, data3: 16662, data4: [149, 51, 196, 61, 153, 161, 251, 100] };
}
#[repr(C)]
pub struct IGpioControllerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Devices_Gpio_Provider", feature = "Foundation_Collections"))]
    pub GetControllersAsync: unsafe extern "system" fn(this: *mut *mut Self, provider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Gpio_Provider", feature = "Foundation_Collections")))]
    GetControllersAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
}
impl ::windows_sys::core::Interface for IGpioControllerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2435546400, data2: 27812, data3: 16646, data4: [163, 115, 255, 253, 52, 107, 14, 91] };
}
#[repr(C)]
pub struct IGpioPin {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ValueChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ValueChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveValueChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveValueChanged: usize,
    #[cfg(feature = "Foundation")]
    pub DebounceTimeout: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DebounceTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub SetDebounceTimeout: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDebounceTimeout: usize,
    pub PinNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SharingMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GpioSharingMode) -> ::windows_sys::core::HRESULT,
    pub IsDriveModeSupported: unsafe extern "system" fn(this: *mut *mut Self, drivemode: GpioPinDriveMode, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub GetDriveMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GpioPinDriveMode) -> ::windows_sys::core::HRESULT,
    pub SetDriveMode: unsafe extern "system" fn(this: *mut *mut Self, value: GpioPinDriveMode) -> ::windows_sys::core::HRESULT,
    pub Write: unsafe extern "system" fn(this: *mut *mut Self, value: GpioPinValue) -> ::windows_sys::core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GpioPinValue) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGpioPin {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 299479175, data2: 44974, data3: 18320, data4: [158, 233, 224, 234, 201, 66, 210, 1] };
}
#[repr(C)]
pub struct IGpioPinValueChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Edge: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GpioPinEdge) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGpioPinValueChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 825731809, data2: 28733, data3: 16473, data4: [189, 36, 181, 178, 93, 255, 184, 78] };
}
