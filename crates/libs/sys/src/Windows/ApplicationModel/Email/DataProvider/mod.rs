pub type EmailDataProviderConnection = *mut ::core::ffi::c_void;
pub type EmailDataProviderTriggerDetails = *mut ::core::ffi::c_void;
pub type EmailMailboxCreateFolderRequest = *mut ::core::ffi::c_void;
pub type EmailMailboxCreateFolderRequestEventArgs = *mut ::core::ffi::c_void;
pub type EmailMailboxDeleteFolderRequest = *mut ::core::ffi::c_void;
pub type EmailMailboxDeleteFolderRequestEventArgs = *mut ::core::ffi::c_void;
pub type EmailMailboxDownloadAttachmentRequest = *mut ::core::ffi::c_void;
pub type EmailMailboxDownloadAttachmentRequestEventArgs = *mut ::core::ffi::c_void;
pub type EmailMailboxDownloadMessageRequest = *mut ::core::ffi::c_void;
pub type EmailMailboxDownloadMessageRequestEventArgs = *mut ::core::ffi::c_void;
pub type EmailMailboxEmptyFolderRequest = *mut ::core::ffi::c_void;
pub type EmailMailboxEmptyFolderRequestEventArgs = *mut ::core::ffi::c_void;
pub type EmailMailboxForwardMeetingRequest = *mut ::core::ffi::c_void;
pub type EmailMailboxForwardMeetingRequestEventArgs = *mut ::core::ffi::c_void;
pub type EmailMailboxGetAutoReplySettingsRequest = *mut ::core::ffi::c_void;
pub type EmailMailboxGetAutoReplySettingsRequestEventArgs = *mut ::core::ffi::c_void;
pub type EmailMailboxMoveFolderRequest = *mut ::core::ffi::c_void;
pub type EmailMailboxMoveFolderRequestEventArgs = *mut ::core::ffi::c_void;
pub type EmailMailboxProposeNewTimeForMeetingRequest = *mut ::core::ffi::c_void;
pub type EmailMailboxProposeNewTimeForMeetingRequestEventArgs = *mut ::core::ffi::c_void;
pub type EmailMailboxResolveRecipientsRequest = *mut ::core::ffi::c_void;
pub type EmailMailboxResolveRecipientsRequestEventArgs = *mut ::core::ffi::c_void;
pub type EmailMailboxServerSearchReadBatchRequest = *mut ::core::ffi::c_void;
pub type EmailMailboxServerSearchReadBatchRequestEventArgs = *mut ::core::ffi::c_void;
pub type EmailMailboxSetAutoReplySettingsRequest = *mut ::core::ffi::c_void;
pub type EmailMailboxSetAutoReplySettingsRequestEventArgs = *mut ::core::ffi::c_void;
pub type EmailMailboxSyncManagerSyncRequest = *mut ::core::ffi::c_void;
pub type EmailMailboxSyncManagerSyncRequestEventArgs = *mut ::core::ffi::c_void;
pub type EmailMailboxUpdateMeetingResponseRequest = *mut ::core::ffi::c_void;
pub type EmailMailboxUpdateMeetingResponseRequestEventArgs = *mut ::core::ffi::c_void;
pub type EmailMailboxValidateCertificatesRequest = *mut ::core::ffi::c_void;
pub type EmailMailboxValidateCertificatesRequestEventArgs = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IEmailDataProviderConnection {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub MailboxSyncRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MailboxSyncRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMailboxSyncRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMailboxSyncRequested: usize,
    #[cfg(feature = "Foundation")]
    pub DownloadMessageRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DownloadMessageRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDownloadMessageRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDownloadMessageRequested: usize,
    #[cfg(feature = "Foundation")]
    pub DownloadAttachmentRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DownloadAttachmentRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDownloadAttachmentRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDownloadAttachmentRequested: usize,
    #[cfg(feature = "Foundation")]
    pub CreateFolderRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFolderRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCreateFolderRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCreateFolderRequested: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteFolderRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteFolderRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDeleteFolderRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDeleteFolderRequested: usize,
    #[cfg(feature = "Foundation")]
    pub EmptyFolderRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EmptyFolderRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEmptyFolderRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEmptyFolderRequested: usize,
    #[cfg(feature = "Foundation")]
    pub MoveFolderRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MoveFolderRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMoveFolderRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMoveFolderRequested: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateMeetingResponseRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateMeetingResponseRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdateMeetingResponseRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdateMeetingResponseRequested: usize,
    #[cfg(feature = "Foundation")]
    pub ForwardMeetingRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ForwardMeetingRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveForwardMeetingRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveForwardMeetingRequested: usize,
    #[cfg(feature = "Foundation")]
    pub ProposeNewTimeForMeetingRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProposeNewTimeForMeetingRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProposeNewTimeForMeetingRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProposeNewTimeForMeetingRequested: usize,
    #[cfg(feature = "Foundation")]
    pub SetAutoReplySettingsRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAutoReplySettingsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSetAutoReplySettingsRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSetAutoReplySettingsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub GetAutoReplySettingsRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAutoReplySettingsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGetAutoReplySettingsRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGetAutoReplySettingsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub ResolveRecipientsRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResolveRecipientsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveResolveRecipientsRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveResolveRecipientsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub ValidateCertificatesRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ValidateCertificatesRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveValidateCertificatesRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveValidateCertificatesRequested: usize,
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
impl ::windows_sys::core::Interface for IEmailDataProviderConnection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1000119751, data2: 14258, data3: 19440, data4: [174, 48, 123, 100, 74, 28, 150, 225] };
}
#[repr(C)]
pub struct IEmailDataProviderTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub Connection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailDataProviderTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2403225168, data2: 13342, data3: 17907, data4: [187, 160, 132, 160, 5, 225, 49, 154] };
}
#[repr(C)]
pub struct IEmailMailboxCreateFolderRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub EmailMailboxId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ParentFolderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, folder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, status: super::EmailMailboxCreateFolderStatus, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxCreateFolderRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 407713653, data2: 51489, data3: 19513, data4: [163, 9, 225, 108, 159, 34, 176, 75] };
}
#[repr(C)]
pub struct IEmailMailboxCreateFolderRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxCreateFolderRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 65323052, data2: 9244, data3: 20137, data4: [166, 143, 255, 32, 188, 90, 252, 133] };
}
#[repr(C)]
pub struct IEmailMailboxDeleteFolderRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub EmailMailboxId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EmailFolderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, status: super::EmailMailboxDeleteFolderStatus, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxDeleteFolderRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2489968778, data2: 43313, data3: 18297, data4: [146, 61, 9, 163, 234, 41, 46, 41] };
}
#[repr(C)]
pub struct IEmailMailboxDeleteFolderRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxDeleteFolderRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3033738502, data2: 9010, data3: 18040, data4: [131, 120, 40, 181, 121, 51, 104, 70] };
}
#[repr(C)]
pub struct IEmailMailboxDownloadAttachmentRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub EmailMailboxId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EmailMessageId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EmailAttachmentId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxDownloadAttachmentRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 186497972, data2: 29964, data3: 18657, data4: [188, 228, 141, 88, 150, 132, 255, 188] };
}
#[repr(C)]
pub struct IEmailMailboxDownloadAttachmentRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxDownloadAttachmentRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3437085805, data2: 65448, data3: 18551, data4: [159, 157, 254, 215, 188, 175, 65, 4] };
}
#[repr(C)]
pub struct IEmailMailboxDownloadMessageRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub EmailMailboxId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EmailMessageId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxDownloadMessageRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1232814471, data2: 23373, data3: 19235, data4: [129, 108, 243, 132, 43, 235, 117, 62] };
}
#[repr(C)]
pub struct IEmailMailboxDownloadMessageRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxDownloadMessageRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1191446957, data2: 53408, data3: 19035, data4: [187, 42, 55, 98, 16, 57, 197, 62] };
}
#[repr(C)]
pub struct IEmailMailboxEmptyFolderRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub EmailMailboxId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EmailFolderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, status: super::EmailMailboxEmptyFolderStatus, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxEmptyFolderRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4266329003, data2: 63597, data3: 18137, data4: [180, 206, 188, 138, 109, 158, 146, 104] };
}
#[repr(C)]
pub struct IEmailMailboxEmptyFolderRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxEmptyFolderRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1904473220, data2: 39002, data3: 19136, data4: [179, 63, 238, 14, 38, 39, 163, 192] };
}
#[repr(C)]
pub struct IEmailMailboxForwardMeetingRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub EmailMailboxId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EmailMessageId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Recipients: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Recipients: usize,
    pub Subject: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ForwardHeaderType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::EmailMessageBodyKind) -> ::windows_sys::core::HRESULT,
    pub ForwardHeader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Comment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxForwardMeetingRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1634560753, data2: 28884, data3: 18482, data4: [184, 105, 184, 5, 66, 174, 155, 232] };
}
#[repr(C)]
pub struct IEmailMailboxForwardMeetingRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxForwardMeetingRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 735638330, data2: 10612, data3: 18265, data4: [165, 165, 88, 244, 77, 60, 2, 117] };
}
#[repr(C)]
pub struct IEmailMailboxGetAutoReplySettingsRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub EmailMailboxId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RequestedFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::EmailMailboxAutoReplyMessageResponseKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, autoreplysettings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxGetAutoReplySettingsRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2604140425, data2: 7816, data3: 19969, data4: [132, 204, 19, 134, 173, 154, 44, 47] };
}
#[repr(C)]
pub struct IEmailMailboxGetAutoReplySettingsRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxGetAutoReplySettingsRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3617543618, data2: 64837, data3: 16388, data4: [138, 145, 155, 172, 243, 139, 112, 34] };
}
#[repr(C)]
pub struct IEmailMailboxMoveFolderRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub EmailMailboxId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EmailFolderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NewParentFolderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NewFolderName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxMoveFolderRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 280635478, data2: 19093, data3: 16488, data4: [145, 204, 103, 204, 122, 207, 69, 79] };
}
#[repr(C)]
pub struct IEmailMailboxMoveFolderRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxMoveFolderRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 945958944, data2: 5306, data3: 19592, data4: [134, 152, 114, 57, 227, 200, 170, 167] };
}
#[repr(C)]
pub struct IEmailMailboxProposeNewTimeForMeetingRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub EmailMailboxId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EmailMessageId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NewStartTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NewStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub NewDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NewDuration: usize,
    pub Subject: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Comment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxProposeNewTimeForMeetingRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1525674322, data2: 38809, data3: 20383, data4: [163, 153, 255, 7, 243, 238, 240, 78] };
}
#[repr(C)]
pub struct IEmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4215802776, data2: 13229, data3: 19047, data4: [130, 81, 15, 156, 36, 155, 106, 32] };
}
#[repr(C)]
pub struct IEmailMailboxResolveRecipientsRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub EmailMailboxId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Recipients: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Recipients: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, resolutionresults: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxResolveRecipientsRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4020555632, data2: 31545, data3: 19611, data4: [129, 30, 65, 234, 244, 58, 51, 45] };
}
#[repr(C)]
pub struct IEmailMailboxResolveRecipientsRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxResolveRecipientsRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 638557698, data2: 45775, data3: 16632, data4: [140, 40, 227, 237, 67, 177, 232, 154] };
}
#[repr(C)]
pub struct IEmailMailboxServerSearchReadBatchRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub SessionId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EmailMailboxId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EmailFolderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Options: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SuggestedBatchSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaveMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, batchstatus: super::EmailBatchStatus, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxServerSearchReadBatchRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 151972831, data2: 23190, data3: 16851, data4: [138, 216, 52, 145, 47, 154, 166, 14] };
}
#[repr(C)]
pub struct IEmailMailboxServerSearchReadBatchRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxServerSearchReadBatchRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 336599886, data2: 60830, data3: 17873, data4: [173, 122, 204, 155, 127, 100, 58, 226] };
}
#[repr(C)]
pub struct IEmailMailboxSetAutoReplySettingsRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub EmailMailboxId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AutoReplySettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxSetAutoReplySettingsRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1973691088, data2: 43150, data3: 20052, data4: [141, 199, 194, 67, 24, 107, 119, 78] };
}
#[repr(C)]
pub struct IEmailMailboxSetAutoReplySettingsRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxSetAutoReplySettingsRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 165286317, data2: 55242, data3: 16519, data4: [172, 134, 83, 250, 103, 247, 98, 70] };
}
#[repr(C)]
pub struct IEmailMailboxSyncManagerSyncRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub EmailMailboxId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxSyncManagerSyncRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1309731044, data2: 32359, data3: 16474, data4: [182, 115, 220, 96, 201, 16, 144, 252] };
}
#[repr(C)]
pub struct IEmailMailboxSyncManagerSyncRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxSyncManagerSyncRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1134166810, data2: 36812, data3: 19173, data4: [185, 181, 212, 52, 224, 166, 90, 168] };
}
#[repr(C)]
pub struct IEmailMailboxUpdateMeetingResponseRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub EmailMailboxId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EmailMessageId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Response: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::EmailMeetingResponseType) -> ::windows_sys::core::HRESULT,
    pub Subject: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Comment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SendUpdate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxUpdateMeetingResponseRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1536797843, data2: 45775, data3: 18568, data4: [186, 79, 48, 107, 107, 102, 223, 48] };
}
#[repr(C)]
pub struct IEmailMailboxUpdateMeetingResponseRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxUpdateMeetingResponseRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1754847073, data2: 22217, data3: 20247, data4: [190, 49, 102, 253, 169, 75, 161, 89] };
}
#[repr(C)]
pub struct IEmailMailboxValidateCertificatesRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub EmailMailboxId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub Certificates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    Certificates: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, validationstatuses: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxValidateCertificatesRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2840410417, data2: 57626, data3: 20375, data4: [184, 26, 24, 122, 112, 168, 244, 26] };
}
#[repr(C)]
pub struct IEmailMailboxValidateCertificatesRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxValidateCertificatesRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 629391127, data2: 767, data3: 18942, data4: [167, 60, 3, 243, 117, 102, 198, 145] };
}
