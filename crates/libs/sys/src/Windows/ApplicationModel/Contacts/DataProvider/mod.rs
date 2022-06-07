pub type ContactDataProviderConnection = *mut ::core::ffi::c_void;
pub type ContactDataProviderTriggerDetails = *mut ::core::ffi::c_void;
pub type ContactListCreateOrUpdateContactRequest = *mut ::core::ffi::c_void;
pub type ContactListCreateOrUpdateContactRequestEventArgs = *mut ::core::ffi::c_void;
pub type ContactListDeleteContactRequest = *mut ::core::ffi::c_void;
pub type ContactListDeleteContactRequestEventArgs = *mut ::core::ffi::c_void;
pub type ContactListServerSearchReadBatchRequest = *mut ::core::ffi::c_void;
pub type ContactListServerSearchReadBatchRequestEventArgs = *mut ::core::ffi::c_void;
pub type ContactListSyncManagerSyncRequest = *mut ::core::ffi::c_void;
pub type ContactListSyncManagerSyncRequestEventArgs = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IContactDataProviderConnection {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SyncRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SyncRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSyncRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSyncRequested: usize,
    #[cfg(feature = "Foundation")]
    pub ServerSearchReadBatchRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServerSearchReadBatchRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveServerSearchReadBatchRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveServerSearchReadBatchRequested: usize,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactDataProviderConnection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 439978578, data2: 35997, data3: 19823, data4: [164, 224, 17, 30, 154, 18, 90, 48] };
}
#[repr(C)]
pub struct IContactDataProviderConnection2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateOrUpdateContactRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateOrUpdateContactRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCreateOrUpdateContactRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCreateOrUpdateContactRequested: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteContactRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteContactRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDeleteContactRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDeleteContactRequested: usize,
}
impl ::windows_sys::core::Interface for IContactDataProviderConnection2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2714970032, data2: 6508, data3: 19453, data4: [143, 15, 198, 141, 103, 242, 73, 211] };
}
#[repr(C)]
pub struct IContactDataProviderTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub Connection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactDataProviderTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1383138494, data2: 15458, data3: 17352, data4: [154, 231, 219, 83, 22, 133, 205, 153] };
}
#[repr(C)]
pub struct IContactListCreateOrUpdateContactRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContactListId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Contact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, createdorupdatedcontact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IContactListCreateOrUpdateContactRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3031384351, data2: 51273, data3: 18384, data4: [177, 25, 145, 207, 96, 91, 47, 42] };
}
#[repr(C)]
pub struct IContactListCreateOrUpdateContactRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IContactListCreateOrUpdateContactRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2233210512, data2: 6737, data3: 19212, data4: [174, 239, 18, 64, 172, 91, 237, 117] };
}
#[repr(C)]
pub struct IContactListDeleteContactRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContactListId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ContactId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IContactListDeleteContactRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1578190471, data2: 52739, data3: 19941, data4: [133, 87, 156, 207, 85, 45, 71, 42] };
}
#[repr(C)]
pub struct IContactListDeleteContactRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IContactListDeleteContactRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2988463265, data2: 59642, data3: 19893, data4: [147, 137, 45, 18, 238, 125, 21, 238] };
}
#[repr(C)]
pub struct IContactListServerSearchReadBatchRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub SessionId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ContactListId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Options: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SuggestedBatchSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaveContactAsync: unsafe extern "system" fn(this: *mut *mut Self, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveContactAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, batchstatus: super::ContactBatchStatus, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IContactListServerSearchReadBatchRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3128388247, data2: 16432, data3: 18725, data4: [159, 180, 20, 59, 41, 94, 101, 59] };
}
#[repr(C)]
pub struct IContactListServerSearchReadBatchRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IContactListServerSearchReadBatchRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 438823035, data2: 27095, data3: 20046, data4: [128, 66, 134, 28, 186, 97, 71, 30] };
}
#[repr(C)]
pub struct IContactListSyncManagerSyncRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContactListId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IContactListSyncManagerSyncRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1007572900, data2: 50407, data3: 18800, data4: [154, 143, 154, 102, 162, 187, 108, 26] };
}
#[repr(C)]
pub struct IContactListSyncManagerSyncRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IContactListSyncManagerSyncRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 361647532, data2: 17517, data3: 20240, data4: [175, 194, 2, 104, 62, 197, 51, 166] };
}
