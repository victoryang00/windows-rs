#[repr(C)]
pub struct INotificationActivationCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub Activate: unsafe extern "system" fn(this: *mut *mut Self, appusermodelid: ::windows_sys::core::PCWSTR, invokedargs: ::windows_sys::core::PCWSTR, data: *const NOTIFICATION_USER_INPUT_DATA, count: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Notifications\"`*"]
pub struct NOTIFICATION_USER_INPUT_DATA {
    pub Key: ::windows_sys::core::PCWSTR,
    pub Value: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for NOTIFICATION_USER_INPUT_DATA {}
impl ::core::clone::Clone for NOTIFICATION_USER_INPUT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
