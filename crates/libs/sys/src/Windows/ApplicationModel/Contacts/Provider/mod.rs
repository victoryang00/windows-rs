#[doc = "*Required features: `\"ApplicationModel_Contacts_Provider\"`*"]
#[repr(transparent)]
pub struct AddContactResult(pub i32);
impl AddContactResult {
    pub const Added: Self = Self(0i32);
    pub const AlreadyAdded: Self = Self(1i32);
    pub const Unavailable: Self = Self(2i32);
}
impl ::core::marker::Copy for AddContactResult {}
impl ::core::clone::Clone for AddContactResult {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ContactPickerUI = *mut ::core::ffi::c_void;
pub type ContactRemovedEventArgs = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IContactPickerUI {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub AddContact: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, contact: *mut ::core::ffi::c_void, result__: *mut AddContactResult) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AddContact: usize,
    pub RemoveContact: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ContainsContact: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub DesiredFields: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    DesiredFields: usize,
    pub SelectionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::ContactSelectionMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ContactRemoved: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContactRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContactRemoved: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContactRemoved: usize,
}
#[repr(C)]
pub struct IContactPickerUI2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AddContact: unsafe extern "system" fn(this: *mut *mut Self, contact: *mut ::core::ffi::c_void, result__: *mut AddContactResult) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DesiredFieldsWithContactFieldType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DesiredFieldsWithContactFieldType: usize,
}
#[repr(C)]
pub struct IContactRemovedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
