#[repr(C)]
pub struct IUserDataTaskDataProviderConnection {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateOrUpdateTaskRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateOrUpdateTaskRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCreateOrUpdateTaskRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCreateOrUpdateTaskRequested: usize,
    #[cfg(feature = "Foundation")]
    pub SyncRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SyncRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSyncRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSyncRequested: usize,
    #[cfg(feature = "Foundation")]
    pub SkipOccurrenceRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SkipOccurrenceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSkipOccurrenceRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSkipOccurrenceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub CompleteTaskRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CompleteTaskRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompleteTaskRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompleteTaskRequested: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteTaskRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteTaskRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDeleteTaskRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDeleteTaskRequested: usize,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserDataTaskDataProviderConnection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2683542813, data2: 42055, data3: 17035, data4: [175, 233, 229, 64, 43, 222, 176, 65] };
}
#[repr(C)]
pub struct IUserDataTaskDataProviderTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub Connection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserDataTaskDataProviderTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2921804290, data2: 45513, data3: 17726, data4: [175, 197, 179, 10, 243, 189, 33, 125] };
}
#[repr(C)]
pub struct IUserDataTaskListCompleteTaskRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub TaskListId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TaskId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, completedtaskid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IUserDataTaskListCompleteTaskRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4133360803, data2: 6722, data3: 18906, data4: [133, 82, 40, 115, 229, 44, 85, 235] };
}
#[repr(C)]
pub struct IUserDataTaskListCompleteTaskRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IUserDataTaskListCompleteTaskRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3615242557, data2: 19698, data3: 18605, data4: [135, 253, 150, 63, 14, 170, 122, 149] };
}
#[repr(C)]
pub struct IUserDataTaskListCreateOrUpdateTaskRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub TaskListId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Task: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, createdorupdateduserdatatask: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IUserDataTaskListCreateOrUpdateTaskRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 557020972, data2: 21954, data3: 17152, data4: [130, 121, 4, 50, 110, 7, 204, 228] };
}
#[repr(C)]
pub struct IUserDataTaskListCreateOrUpdateTaskRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IUserDataTaskListCreateOrUpdateTaskRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 314923602, data2: 58232, data3: 16795, data4: [174, 56, 165, 233, 230, 4, 71, 110] };
}
#[repr(C)]
pub struct IUserDataTaskListDeleteTaskRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub TaskListId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TaskId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IUserDataTaskListDeleteTaskRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1267088488, data2: 30295, data3: 20285, data4: [176, 116, 212, 126, 200, 223, 7, 231] };
}
#[repr(C)]
pub struct IUserDataTaskListDeleteTaskRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IUserDataTaskListDeleteTaskRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1617156825, data2: 62818, data3: 16709, data4: [142, 254, 213, 0, 120, 201, 43, 127] };
}
#[repr(C)]
pub struct IUserDataTaskListSkipOccurrenceRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub TaskListId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TaskId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IUserDataTaskListSkipOccurrenceRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2877809485, data2: 7379, data3: 17180, data4: [159, 88, 8, 154, 164, 51, 141, 133] };
}
#[repr(C)]
pub struct IUserDataTaskListSkipOccurrenceRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IUserDataTaskListSkipOccurrenceRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2050724426, data2: 52271, data3: 20091, data4: [170, 205, 165, 185, 210, 156, 250, 78] };
}
#[repr(C)]
pub struct IUserDataTaskListSyncManagerSyncRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub TaskListId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IUserDataTaskListSyncManagerSyncRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1084700679, data2: 30096, data3: 16713, data4: [174, 25, 178, 17, 67, 26, 159, 72] };
}
#[repr(C)]
pub struct IUserDataTaskListSyncManagerSyncRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IUserDataTaskListSyncManagerSyncRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2393709586, data2: 30350, data3: 17341, data4: [131, 133, 92, 220, 53, 31, 253, 234] };
}
pub type UserDataTaskDataProviderConnection = *mut ::core::ffi::c_void;
pub type UserDataTaskDataProviderTriggerDetails = *mut ::core::ffi::c_void;
pub type UserDataTaskListCompleteTaskRequest = *mut ::core::ffi::c_void;
pub type UserDataTaskListCompleteTaskRequestEventArgs = *mut ::core::ffi::c_void;
pub type UserDataTaskListCreateOrUpdateTaskRequest = *mut ::core::ffi::c_void;
pub type UserDataTaskListCreateOrUpdateTaskRequestEventArgs = *mut ::core::ffi::c_void;
pub type UserDataTaskListDeleteTaskRequest = *mut ::core::ffi::c_void;
pub type UserDataTaskListDeleteTaskRequestEventArgs = *mut ::core::ffi::c_void;
pub type UserDataTaskListSkipOccurrenceRequest = *mut ::core::ffi::c_void;
pub type UserDataTaskListSkipOccurrenceRequestEventArgs = *mut ::core::ffi::c_void;
pub type UserDataTaskListSyncManagerSyncRequest = *mut ::core::ffi::c_void;
pub type UserDataTaskListSyncManagerSyncRequestEventArgs = *mut ::core::ffi::c_void;
