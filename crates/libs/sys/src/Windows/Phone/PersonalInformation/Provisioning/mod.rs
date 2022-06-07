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
impl ::windows_sys::core::Interface for IContactPartnerProvisioningManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3235355169, data2: 431, data3: 20435, data4: [152, 205, 179, 214, 86, 222, 21, 244] };
}
#[repr(C)]
pub struct IContactPartnerProvisioningManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AssociateSocialNetworkAccountAsync: unsafe extern "system" fn(this: *mut *mut Self, store: *mut ::core::ffi::c_void, networkname: ::windows_sys::core::HSTRING, networkaccountid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AssociateSocialNetworkAccountAsync: usize,
}
impl ::windows_sys::core::Interface for IContactPartnerProvisioningManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3261158903, data2: 21997, data3: 18269, data4: [147, 52, 197, 212, 132, 195, 15, 26] };
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
impl ::windows_sys::core::Interface for IMessagePartnerProvisioningManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2317027408, data2: 29637, data3: 17788, data4: [188, 89, 237, 125, 97, 92, 5, 164] };
}
