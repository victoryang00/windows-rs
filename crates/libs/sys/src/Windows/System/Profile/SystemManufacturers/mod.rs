#[repr(C)]
pub struct IOemSupportInfo {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SupportLink: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SupportLink: usize,
    #[cfg(feature = "Foundation")]
    pub SupportAppLink: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SupportAppLink: usize,
    pub SupportProvider: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOemSupportInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2368646741, data2: 34799, data3: 16998, data4: [134, 208, 196, 175, 190, 178, 155, 185] };
}
#[repr(C)]
pub struct ISmbiosInformationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub SerialNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmbiosInformationStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 135055996, data2: 25468, data3: 18628, data4: [183, 40, 249, 39, 56, 18, 219, 142] };
}
#[repr(C)]
pub struct ISystemSupportDeviceInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub OperatingSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SystemManufacturer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SystemProductName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SystemSku: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SystemHardwareVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SystemFirmwareVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemSupportDeviceInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 92801945, data2: 33351, data3: 17435, data4: [169, 150, 161, 120, 75, 171, 121, 168] };
}
#[repr(C)]
pub struct ISystemSupportInfoStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub LocalSystemEdition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub OemSupportInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemSupportInfoStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4017424756, data2: 50210, data3: 17879, data4: [164, 77, 92, 28, 0, 67, 162, 179] };
}
#[repr(C)]
pub struct ISystemSupportInfoStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LocalDeviceInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemSupportInfoStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 871582116, data2: 16289, data3: 18822, data4: [170, 75, 5, 116, 32, 69, 94, 109] };
}
pub type OemSupportInfo = *mut ::core::ffi::c_void;
pub type SystemSupportDeviceInfo = *mut ::core::ffi::c_void;
