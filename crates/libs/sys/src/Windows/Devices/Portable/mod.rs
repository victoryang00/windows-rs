#[repr(C)]
pub struct IServiceDeviceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, servicetype: ServiceDeviceType, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorFromServiceId: unsafe extern "system" fn(this: *mut *mut Self, serviceid: ::windows_sys::core::GUID, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IServiceDeviceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2827097313, data2: 22983, data3: 18976, data4: [171, 166, 159, 103, 7, 147, 114, 48] };
}
#[repr(C)]
pub struct IStorageDeviceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage")]
    pub FromId: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    FromId: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorageDeviceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1590576366, data2: 6947, data3: 19922, data4: [134, 82, 188, 22, 79, 0, 49, 40] };
}
#[doc = "*Required features: `\"Devices_Portable\"`*"]
#[repr(transparent)]
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
