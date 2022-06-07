#[cfg(feature = "ApplicationModel_UserDataTasks_DataProvider")]
pub mod DataProvider;
#[repr(C)]
pub struct IUserDataTask {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ListId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CompletedDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CompletedDate: usize,
    #[cfg(feature = "Foundation")]
    pub SetCompletedDate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCompletedDate: usize,
    pub Details: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDetails: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DetailsKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UserDataTaskDetailsKind) -> ::windows_sys::core::HRESULT,
    pub SetDetailsKind: unsafe extern "system" fn(this: *mut *mut Self, value: UserDataTaskDetailsKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DueDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DueDate: usize,
    #[cfg(feature = "Foundation")]
    pub SetDueDate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDueDate: usize,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UserDataTaskKind) -> ::windows_sys::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UserDataTaskPriority) -> ::windows_sys::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut *mut Self, value: UserDataTaskPriority) -> ::windows_sys::core::HRESULT,
    pub RecurrenceProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRecurrenceProperties: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RegenerationProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRegenerationProperties: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Reminder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Reminder: usize,
    #[cfg(feature = "Foundation")]
    pub SetReminder: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetReminder: usize,
    pub Sensitivity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UserDataTaskSensitivity) -> ::windows_sys::core::HRESULT,
    pub SetSensitivity: unsafe extern "system" fn(this: *mut *mut Self, value: UserDataTaskSensitivity) -> ::windows_sys::core::HRESULT,
    pub Subject: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartDate: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartDate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartDate: usize,
}
impl ::windows_sys::core::Interface for IUserDataTask {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2087028177, data2: 57556, data3: 20377, data4: [174, 226, 188, 45, 93, 218, 223, 76] };
}
#[repr(C)]
pub struct IUserDataTaskBatch {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Tasks: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Tasks: usize,
}
impl ::windows_sys::core::Interface for IUserDataTaskBatch {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 942515710, data2: 8373, data3: 17180, data4: [143, 66, 165, 210, 146, 236, 147, 12] };
}
#[repr(C)]
pub struct IUserDataTaskList {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub UserDataAccountId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SourceDisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub OtherAppReadAccess: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UserDataTaskListOtherAppReadAccess) -> ::windows_sys::core::HRESULT,
    pub SetOtherAppReadAccess: unsafe extern "system" fn(this: *mut *mut Self, value: UserDataTaskListOtherAppReadAccess) -> ::windows_sys::core::HRESULT,
    pub OtherAppWriteAccess: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UserDataTaskListOtherAppWriteAccess) -> ::windows_sys::core::HRESULT,
    pub SetOtherAppWriteAccess: unsafe extern "system" fn(this: *mut *mut Self, value: UserDataTaskListOtherAppWriteAccess) -> ::windows_sys::core::HRESULT,
    pub LimitedWriteOperations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SyncManager: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RegisterSyncManagerAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RegisterSyncManagerAsync: usize,
    pub GetTaskReader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetTaskReaderWithOptions: unsafe extern "system" fn(this: *mut *mut Self, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetTaskAsync: unsafe extern "system" fn(this: *mut *mut Self, userdatatask: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetTaskAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SaveTaskAsync: unsafe extern "system" fn(this: *mut *mut Self, userdatatask: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveTaskAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteTaskAsync: unsafe extern "system" fn(this: *mut *mut Self, userdatataskid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteTaskAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
}
impl ::windows_sys::core::Interface for IUserDataTaskList {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1229008441, data2: 31773, data3: 19953, data4: [190, 211, 49, 75, 124, 191, 94, 78] };
}
#[repr(C)]
pub struct IUserDataTaskListLimitedWriteOperations {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub TryCompleteTaskAsync: unsafe extern "system" fn(this: *mut *mut Self, userdatataskid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryCompleteTaskAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryCreateOrUpdateTaskAsync: unsafe extern "system" fn(this: *mut *mut Self, userdatatask: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryCreateOrUpdateTaskAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryDeleteTaskAsync: unsafe extern "system" fn(this: *mut *mut Self, userdatataskid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDeleteTaskAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySkipOccurrenceAsync: unsafe extern "system" fn(this: *mut *mut Self, userdatataskid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySkipOccurrenceAsync: usize,
}
impl ::windows_sys::core::Interface for IUserDataTaskListLimitedWriteOperations {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2057463794, data2: 24696, data3: 16771, data4: [145, 158, 79, 41, 241, 156, 250, 233] };
}
#[repr(C)]
pub struct IUserDataTaskListSyncManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub LastAttemptedSyncTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastAttemptedSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetLastAttemptedSyncTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastAttemptedSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub LastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastSuccessfulSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetLastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastSuccessfulSyncTime: usize,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UserDataTaskListSyncStatus) -> ::windows_sys::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut *mut Self, value: UserDataTaskListSyncStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SyncAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SyncAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SyncStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SyncStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSyncStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSyncStatusChanged: usize,
}
impl ::windows_sys::core::Interface for IUserDataTaskListSyncManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2388204181, data2: 7631, data3: 18079, data4: [147, 236, 186, 72, 187, 85, 60, 107] };
}
#[repr(C)]
pub struct IUserDataTaskManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut *mut Self, accesstype: UserDataTaskStoreAccessType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
impl ::windows_sys::core::Interface for IUserDataTaskManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2219952404, data2: 58891, data3: 18601, data4: [146, 17, 127, 184, 165, 108, 184, 76] };
}
#[repr(C)]
pub struct IUserDataTaskManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
impl ::windows_sys::core::Interface for IUserDataTaskManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3008707064, data2: 50434, data3: 18428, data4: [168, 30, 16, 8, 131, 113, 157, 85] };
}
#[repr(C)]
pub struct IUserDataTaskQueryOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub SortProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UserDataTaskQuerySortProperty) -> ::windows_sys::core::HRESULT,
    pub SetSortProperty: unsafe extern "system" fn(this: *mut *mut Self, value: UserDataTaskQuerySortProperty) -> ::windows_sys::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UserDataTaskQueryKind) -> ::windows_sys::core::HRESULT,
    pub SetKind: unsafe extern "system" fn(this: *mut *mut Self, value: UserDataTaskQueryKind) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserDataTaskQueryOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2510235629, data2: 37018, data3: 19760, data4: [140, 27, 51, 29, 143, 230, 103, 226] };
}
#[repr(C)]
pub struct IUserDataTaskReader {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadBatchAsync: usize,
}
impl ::windows_sys::core::Interface for IUserDataTaskReader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 65439921, data2: 19663, data3: 17664, data4: [136, 59, 231, 98, 144, 207, 237, 99] };
}
#[repr(C)]
pub struct IUserDataTaskRecurrenceProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub Unit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UserDataTaskRecurrenceUnit) -> ::windows_sys::core::HRESULT,
    pub SetUnit: unsafe extern "system" fn(this: *mut *mut Self, value: UserDataTaskRecurrenceUnit) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Occurrences: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Occurrences: usize,
    #[cfg(feature = "Foundation")]
    pub SetOccurrences: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetOccurrences: usize,
    #[cfg(feature = "Foundation")]
    pub Until: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Until: usize,
    #[cfg(feature = "Foundation")]
    pub SetUntil: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUntil: usize,
    pub Interval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetInterval: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DaysOfWeek: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DaysOfWeek: usize,
    #[cfg(feature = "Foundation")]
    pub SetDaysOfWeek: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDaysOfWeek: usize,
    #[cfg(feature = "Foundation")]
    pub WeekOfMonth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WeekOfMonth: usize,
    #[cfg(feature = "Foundation")]
    pub SetWeekOfMonth: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetWeekOfMonth: usize,
    #[cfg(feature = "Foundation")]
    pub Month: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Month: usize,
    #[cfg(feature = "Foundation")]
    pub SetMonth: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMonth: usize,
    #[cfg(feature = "Foundation")]
    pub Day: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Day: usize,
    #[cfg(feature = "Foundation")]
    pub SetDay: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDay: usize,
}
impl ::windows_sys::core::Interface for IUserDataTaskRecurrenceProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1944027312, data2: 10182, data3: 16590, data4: [177, 73, 156, 212, 20, 133, 166, 158] };
}
#[repr(C)]
pub struct IUserDataTaskRegenerationProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub Unit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UserDataTaskRegenerationUnit) -> ::windows_sys::core::HRESULT,
    pub SetUnit: unsafe extern "system" fn(this: *mut *mut Self, value: UserDataTaskRegenerationUnit) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Occurrences: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Occurrences: usize,
    #[cfg(feature = "Foundation")]
    pub SetOccurrences: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetOccurrences: usize,
    #[cfg(feature = "Foundation")]
    pub Until: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Until: usize,
    #[cfg(feature = "Foundation")]
    pub SetUntil: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUntil: usize,
    pub Interval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetInterval: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserDataTaskRegenerationProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2460680199, data2: 2318, data3: 18180, data4: [187, 92, 132, 252, 11, 13, 156, 49] };
}
#[repr(C)]
pub struct IUserDataTaskStore {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateListAsync: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateListAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateListInAccountAsync: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, userdataaccountid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateListInAccountAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindListsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindListsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetListAsync: unsafe extern "system" fn(this: *mut *mut Self, tasklistid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetListAsync: usize,
}
impl ::windows_sys::core::Interface for IUserDataTaskStore {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4033518768, data2: 61915, data3: 17850, data4: [138, 98, 8, 96, 4, 192, 33, 61] };
}
pub type UserDataTask = *mut ::core::ffi::c_void;
pub type UserDataTaskBatch = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
pub struct UserDataTaskDaysOfWeek(pub u32);
impl UserDataTaskDaysOfWeek {
    pub const None: Self = Self(0u32);
    pub const Sunday: Self = Self(1u32);
    pub const Monday: Self = Self(2u32);
    pub const Tuesday: Self = Self(4u32);
    pub const Wednesday: Self = Self(8u32);
    pub const Thursday: Self = Self(16u32);
    pub const Friday: Self = Self(32u32);
    pub const Saturday: Self = Self(64u32);
}
impl ::core::marker::Copy for UserDataTaskDaysOfWeek {}
impl ::core::clone::Clone for UserDataTaskDaysOfWeek {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
pub struct UserDataTaskDetailsKind(pub i32);
impl UserDataTaskDetailsKind {
    pub const PlainText: Self = Self(0i32);
    pub const Html: Self = Self(1i32);
}
impl ::core::marker::Copy for UserDataTaskDetailsKind {}
impl ::core::clone::Clone for UserDataTaskDetailsKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
pub struct UserDataTaskKind(pub i32);
impl UserDataTaskKind {
    pub const Single: Self = Self(0i32);
    pub const Recurring: Self = Self(1i32);
    pub const Regenerating: Self = Self(2i32);
}
impl ::core::marker::Copy for UserDataTaskKind {}
impl ::core::clone::Clone for UserDataTaskKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type UserDataTaskList = *mut ::core::ffi::c_void;
pub type UserDataTaskListLimitedWriteOperations = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
pub struct UserDataTaskListOtherAppReadAccess(pub i32);
impl UserDataTaskListOtherAppReadAccess {
    pub const Full: Self = Self(0i32);
    pub const SystemOnly: Self = Self(1i32);
    pub const None: Self = Self(2i32);
}
impl ::core::marker::Copy for UserDataTaskListOtherAppReadAccess {}
impl ::core::clone::Clone for UserDataTaskListOtherAppReadAccess {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
pub struct UserDataTaskListOtherAppWriteAccess(pub i32);
impl UserDataTaskListOtherAppWriteAccess {
    pub const Limited: Self = Self(0i32);
    pub const None: Self = Self(1i32);
}
impl ::core::marker::Copy for UserDataTaskListOtherAppWriteAccess {}
impl ::core::clone::Clone for UserDataTaskListOtherAppWriteAccess {
    fn clone(&self) -> Self {
        *self
    }
}
pub type UserDataTaskListSyncManager = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
pub struct UserDataTaskListSyncStatus(pub i32);
impl UserDataTaskListSyncStatus {
    pub const Idle: Self = Self(0i32);
    pub const Syncing: Self = Self(1i32);
    pub const UpToDate: Self = Self(2i32);
    pub const AuthenticationError: Self = Self(3i32);
    pub const PolicyError: Self = Self(4i32);
    pub const UnknownError: Self = Self(5i32);
}
impl ::core::marker::Copy for UserDataTaskListSyncStatus {}
impl ::core::clone::Clone for UserDataTaskListSyncStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type UserDataTaskManager = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
pub struct UserDataTaskPriority(pub i32);
impl UserDataTaskPriority {
    pub const Normal: Self = Self(0i32);
    pub const Low: Self = Self(-1i32);
    pub const High: Self = Self(1i32);
}
impl ::core::marker::Copy for UserDataTaskPriority {}
impl ::core::clone::Clone for UserDataTaskPriority {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
pub struct UserDataTaskQueryKind(pub i32);
impl UserDataTaskQueryKind {
    pub const All: Self = Self(0i32);
    pub const Incomplete: Self = Self(1i32);
    pub const Complete: Self = Self(2i32);
}
impl ::core::marker::Copy for UserDataTaskQueryKind {}
impl ::core::clone::Clone for UserDataTaskQueryKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type UserDataTaskQueryOptions = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
pub struct UserDataTaskQuerySortProperty(pub i32);
impl UserDataTaskQuerySortProperty {
    pub const DueDate: Self = Self(0i32);
}
impl ::core::marker::Copy for UserDataTaskQuerySortProperty {}
impl ::core::clone::Clone for UserDataTaskQuerySortProperty {
    fn clone(&self) -> Self {
        *self
    }
}
pub type UserDataTaskReader = *mut ::core::ffi::c_void;
pub type UserDataTaskRecurrenceProperties = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
pub struct UserDataTaskRecurrenceUnit(pub i32);
impl UserDataTaskRecurrenceUnit {
    pub const Daily: Self = Self(0i32);
    pub const Weekly: Self = Self(1i32);
    pub const Monthly: Self = Self(2i32);
    pub const MonthlyOnDay: Self = Self(3i32);
    pub const Yearly: Self = Self(4i32);
    pub const YearlyOnDay: Self = Self(5i32);
}
impl ::core::marker::Copy for UserDataTaskRecurrenceUnit {}
impl ::core::clone::Clone for UserDataTaskRecurrenceUnit {
    fn clone(&self) -> Self {
        *self
    }
}
pub type UserDataTaskRegenerationProperties = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
pub struct UserDataTaskRegenerationUnit(pub i32);
impl UserDataTaskRegenerationUnit {
    pub const Daily: Self = Self(0i32);
    pub const Weekly: Self = Self(1i32);
    pub const Monthly: Self = Self(2i32);
    pub const Yearly: Self = Self(4i32);
}
impl ::core::marker::Copy for UserDataTaskRegenerationUnit {}
impl ::core::clone::Clone for UserDataTaskRegenerationUnit {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
pub struct UserDataTaskSensitivity(pub i32);
impl UserDataTaskSensitivity {
    pub const Public: Self = Self(0i32);
    pub const Private: Self = Self(1i32);
}
impl ::core::marker::Copy for UserDataTaskSensitivity {}
impl ::core::clone::Clone for UserDataTaskSensitivity {
    fn clone(&self) -> Self {
        *self
    }
}
pub type UserDataTaskStore = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
pub struct UserDataTaskStoreAccessType(pub i32);
impl UserDataTaskStoreAccessType {
    pub const AppTasksReadWrite: Self = Self(0i32);
    pub const AllTasksLimitedReadWrite: Self = Self(1i32);
}
impl ::core::marker::Copy for UserDataTaskStoreAccessType {}
impl ::core::clone::Clone for UserDataTaskStoreAccessType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
pub struct UserDataTaskWeekOfMonth(pub i32);
impl UserDataTaskWeekOfMonth {
    pub const First: Self = Self(0i32);
    pub const Second: Self = Self(1i32);
    pub const Third: Self = Self(2i32);
    pub const Fourth: Self = Self(3i32);
    pub const Last: Self = Self(4i32);
}
impl ::core::marker::Copy for UserDataTaskWeekOfMonth {}
impl ::core::clone::Clone for UserDataTaskWeekOfMonth {
    fn clone(&self) -> Self {
        *self
    }
}
