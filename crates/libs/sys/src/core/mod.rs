// TODO: make windows::core's HRESULT and GUID type aliases for windows_sys::core so that they can share basic COM support?

#[repr(C)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}
impl ::core::marker::Copy for GUID {}
impl ::core::clone::Clone for GUID {
    fn clone(&self) -> Self {
        *self
    }
}

pub type HRESULT = i32;
pub type HSTRING = *mut ::core::ffi::c_void;
pub type PSTR = *mut u8;
pub type PWSTR = *mut u16;
pub type PCSTR = *const u8;
pub type PCWSTR = *const u16;

#[repr(C)]
pub struct IUnknown {
    pub QueryInterface: unsafe extern "system" fn(this: *mut *mut Self, iid: &GUID, interface: *mut *const core::ffi::c_void) -> HRESULT,
    pub AddRef: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub Release: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
}

#[repr(C)]
pub struct IInspectable {
    pub base__: IUnknown,
    pub GetIids: unsafe extern "system" fn(this: *mut *mut Self, count: *mut u32, values: *mut *mut GUID) -> HRESULT,
    pub GetRuntimeClassName: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut core::ffi::c_void) -> HRESULT,
    pub GetTrustLevel: unsafe extern "system" fn(this: *mut *mut Self, value: *mut i32) -> HRESULT,
}
