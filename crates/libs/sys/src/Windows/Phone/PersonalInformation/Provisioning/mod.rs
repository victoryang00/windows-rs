#[repr(C)]
pub struct IContactPartnerProvisioningManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AssociateNetworkAccountAsync: unsafe extern "system" fn(this: *mut *mut Self, store: *mut ::core::ffi::c_void, networkname: ::windows_sys::core::HSTRING, networkaccountid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AssociateNetworkAccountAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ImportVcardToSystemAsync: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ImportVcardToSystemAsync: usize,
}
#[repr(C)]
pub struct IContactPartnerProvisioningManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AssociateSocialNetworkAccountAsync: unsafe extern "system" fn(this: *mut *mut Self, store: *mut ::core::ffi::c_void, networkname: ::windows_sys::core::HSTRING, networkaccountid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AssociateSocialNetworkAccountAsync: usize,
}
#[repr(C)]
pub struct IMessagePartnerProvisioningManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ImportSmsToSystemAsync: unsafe extern "system" fn(this: *mut *mut Self, incoming: bool, read: bool, body: ::windows_sys::core::HSTRING, sender: ::windows_sys::core::HSTRING, recipients: *mut ::core::ffi::c_void, deliverytime: super::super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ImportSmsToSystemAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ImportMmsToSystemAsync: unsafe extern "system" fn(this: *mut *mut Self, incoming: bool, read: bool, subject: ::windows_sys::core::HSTRING, sender: ::windows_sys::core::HSTRING, recipients: *mut ::core::ffi::c_void, deliverytime: super::super::super::Foundation::DateTime, attachments: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ImportMmsToSystemAsync: usize,
}
