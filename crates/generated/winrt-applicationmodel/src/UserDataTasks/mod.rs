#[cfg(feature = "DataProvider")]
pub mod DataProvider;
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTask(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataTask {
    type Vtable = IUserDataTask_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7c6585d1_e0d4_4f99_aee2_bc2d5ddadf4c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTask_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ListId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CompletedDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCompletedDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Details: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DetailsKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskDetailsKind) -> ::windows_core::HRESULT,
    pub SetDetailsKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UserDataTaskDetailsKind) -> ::windows_core::HRESULT,
    pub DueDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetDueDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskKind) -> ::windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskPriority) -> ::windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UserDataTaskPriority) -> ::windows_core::HRESULT,
    pub RecurrenceProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetRecurrenceProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RegenerationProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetRegenerationProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Reminder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetReminder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Sensitivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskSensitivity) -> ::windows_core::HRESULT,
    pub SetSensitivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UserDataTaskSensitivity) -> ::windows_core::HRESULT,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub StartDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetStartDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskBatch(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataTaskBatch {
    type Vtable = IUserDataTaskBatch_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x382da5fe_20b5_431c_8f42_a5d292ec930c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskBatch_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Tasks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Tasks: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskList(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataTaskList {
    type Vtable = IUserDataTaskList_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x49412e39_7c1d_4df1_bed3_314b7cbf5e4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskList_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub UserDataAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SourceDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub OtherAppReadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskListOtherAppReadAccess) -> ::windows_core::HRESULT,
    pub SetOtherAppReadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UserDataTaskListOtherAppReadAccess) -> ::windows_core::HRESULT,
    pub OtherAppWriteAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskListOtherAppWriteAccess) -> ::windows_core::HRESULT,
    pub SetOtherAppWriteAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UserDataTaskListOtherAppWriteAccess) -> ::windows_core::HRESULT,
    pub LimitedWriteOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SyncManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RegisterSyncManagerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetTaskReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetTaskReaderWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetTaskAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userdatatask: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SaveTaskAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userdatatask: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeleteTaskAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userdatataskid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskListLimitedWriteOperations(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataTaskListLimitedWriteOperations {
    type Vtable = IUserDataTaskListLimitedWriteOperations_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7aa267f2_6078_4183_919e_4f29f19cfae9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskListLimitedWriteOperations_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TryCompleteTaskAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userdatataskid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryCreateOrUpdateTaskAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userdatatask: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryDeleteTaskAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userdatataskid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TrySkipOccurrenceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userdatataskid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskListSyncManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataTaskListSyncManager {
    type Vtable = IUserDataTaskListSyncManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8e591a95_1dcf_469f_93ec_ba48bb553c6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskListSyncManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LastAttemptedSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub SetLastAttemptedSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub LastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub SetLastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskListSyncStatus) -> ::windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UserDataTaskListSyncStatus) -> ::windows_core::HRESULT,
    pub SyncAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SyncStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSyncStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataTaskManager {
    type Vtable = IUserDataTaskManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8451c914_e60b_48a9_9211_7fb8a56cb84c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accesstype: UserDataTaskStoreAccessType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-system")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataTaskManagerStatics {
    type Vtable = IUserDataTaskManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb35539f8_c502_47fc_a81e_100883719d55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-system")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    GetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskQueryOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataTaskQueryOptions {
    type Vtable = IUserDataTaskQueryOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x959f27ed_909a_4d30_8c1b_331d8fe667e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskQueryOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SortProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskQuerySortProperty) -> ::windows_core::HRESULT,
    pub SetSortProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UserDataTaskQuerySortProperty) -> ::windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskQueryKind) -> ::windows_core::HRESULT,
    pub SetKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UserDataTaskQueryKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskReader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataTaskReader {
    type Vtable = IUserDataTaskReader_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x03e688b1_4ccf_4500_883b_e76290cfed63);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskReader_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskRecurrenceProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataTaskRecurrenceProperties {
    type Vtable = IUserDataTaskRecurrenceProperties_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x73df80b0_27c6_40ce_b149_9cd41485a69e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskRecurrenceProperties_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Unit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskRecurrenceUnit) -> ::windows_core::HRESULT,
    pub SetUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UserDataTaskRecurrenceUnit) -> ::windows_core::HRESULT,
    pub Occurrences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetOccurrences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Until: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetUntil: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Interval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub DaysOfWeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetDaysOfWeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub WeekOfMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetWeekOfMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Month: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Day: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskRegenerationProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataTaskRegenerationProperties {
    type Vtable = IUserDataTaskRegenerationProperties_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x92ab0007_090e_4704_bb5c_84fc0b0d9c31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskRegenerationProperties_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Unit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskRegenerationUnit) -> ::windows_core::HRESULT,
    pub SetUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UserDataTaskRegenerationUnit) -> ::windows_core::HRESULT,
    pub Occurrences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetOccurrences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Until: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetUntil: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Interval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskStore(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataTaskStore {
    type Vtable = IUserDataTaskStore_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf06a9cb0_f1db_45ba_8a62_086004c0213d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskStore_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateListAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateListInAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, userdataaccountid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub FindListsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    FindListsAsync: usize,
    pub GetListAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tasklistid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct UserDataTask(::windows_core::IUnknown);
impl UserDataTask {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UserDataTask, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ListId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ListId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn RemoteId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RemoteId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetRemoteId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRemoteId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CompletedDate(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CompletedDate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn SetCompletedDate<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::DateTime>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCompletedDate)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Details(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Details)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetDetails<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDetails)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DetailsKind(&self) -> ::windows_core::Result<UserDataTaskDetailsKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UserDataTaskDetailsKind>::zeroed();
            (::windows_core::Interface::vtable(this).DetailsKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserDataTaskDetailsKind>(result__)
        }
    }
    pub fn SetDetailsKind(&self, value: UserDataTaskDetailsKind) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDetailsKind)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DueDate(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DueDate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn SetDueDate<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::DateTime>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDueDate)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<UserDataTaskKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UserDataTaskKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserDataTaskKind>(result__)
        }
    }
    pub fn Priority(&self) -> ::windows_core::Result<UserDataTaskPriority> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UserDataTaskPriority>::zeroed();
            (::windows_core::Interface::vtable(this).Priority)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserDataTaskPriority>(result__)
        }
    }
    pub fn SetPriority(&self, value: UserDataTaskPriority) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPriority)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RecurrenceProperties(&self) -> ::windows_core::Result<UserDataTaskRecurrenceProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RecurrenceProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserDataTaskRecurrenceProperties>(result__)
        }
    }
    pub fn SetRecurrenceProperties<'a, Param0: ::windows_core::IntoParam<'a, UserDataTaskRecurrenceProperties>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRecurrenceProperties)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RegenerationProperties(&self) -> ::windows_core::Result<UserDataTaskRegenerationProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RegenerationProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserDataTaskRegenerationProperties>(result__)
        }
    }
    pub fn SetRegenerationProperties<'a, Param0: ::windows_core::IntoParam<'a, UserDataTaskRegenerationProperties>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRegenerationProperties)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Reminder(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Reminder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn SetReminder<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::DateTime>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReminder)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Sensitivity(&self) -> ::windows_core::Result<UserDataTaskSensitivity> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UserDataTaskSensitivity>::zeroed();
            (::windows_core::Interface::vtable(this).Sensitivity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserDataTaskSensitivity>(result__)
        }
    }
    pub fn SetSensitivity(&self, value: UserDataTaskSensitivity) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSensitivity)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Subject(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Subject)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetSubject<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSubject)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartDate(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartDate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn SetStartDate<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::DateTime>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartDate)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for UserDataTask {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTask {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTask {}
impl ::core::fmt::Debug for UserDataTask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTask").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTask {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTask;{7c6585d1-e0d4-4f99-aee2-bc2d5ddadf4c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataTask {
    type Vtable = IUserDataTask_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTask as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTask {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTask";
}
impl ::core::convert::From<UserDataTask> for ::windows_core::IUnknown {
    fn from(value: UserDataTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTask> for ::windows_core::IUnknown {
    fn from(value: &UserDataTask) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataTask {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataTask {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTask> for ::windows_core::IInspectable {
    fn from(value: UserDataTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTask> for ::windows_core::IInspectable {
    fn from(value: &UserDataTask) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataTask {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataTask {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTask {}
unsafe impl ::core::marker::Sync for UserDataTask {}
#[repr(transparent)]
pub struct UserDataTaskBatch(::windows_core::IUnknown);
impl UserDataTaskBatch {
    #[cfg(feature = "winrt-foundation")]
    pub fn Tasks(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<UserDataTask>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Tasks)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<UserDataTask>>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskBatch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskBatch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskBatch {}
impl ::core::fmt::Debug for UserDataTaskBatch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskBatch").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskBatch {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskBatch;{382da5fe-20b5-431c-8f42-a5d292ec930c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataTaskBatch {
    type Vtable = IUserDataTaskBatch_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskBatch as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskBatch {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskBatch";
}
impl ::core::convert::From<UserDataTaskBatch> for ::windows_core::IUnknown {
    fn from(value: UserDataTaskBatch) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskBatch> for ::windows_core::IUnknown {
    fn from(value: &UserDataTaskBatch) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataTaskBatch {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataTaskBatch {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskBatch> for ::windows_core::IInspectable {
    fn from(value: UserDataTaskBatch) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskBatch> for ::windows_core::IInspectable {
    fn from(value: &UserDataTaskBatch) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataTaskBatch {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataTaskBatch {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskBatch {}
unsafe impl ::core::marker::Sync for UserDataTaskBatch {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for UserDataTaskDaysOfWeek {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserDataTaskDaysOfWeek {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataTaskDaysOfWeek {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskDaysOfWeek").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for UserDataTaskDaysOfWeek {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for UserDataTaskDaysOfWeek {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for UserDataTaskDaysOfWeek {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for UserDataTaskDaysOfWeek {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for UserDataTaskDaysOfWeek {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskDaysOfWeek {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskDaysOfWeek;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for UserDataTaskDetailsKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserDataTaskDetailsKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataTaskDetailsKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskDetailsKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskDetailsKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskDetailsKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for UserDataTaskKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserDataTaskKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataTaskKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct UserDataTaskList(::windows_core::IUnknown);
impl UserDataTaskList {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn UserDataAccountId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).UserDataAccountId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetDisplayName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SourceDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SourceDisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn OtherAppReadAccess(&self) -> ::windows_core::Result<UserDataTaskListOtherAppReadAccess> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UserDataTaskListOtherAppReadAccess>::zeroed();
            (::windows_core::Interface::vtable(this).OtherAppReadAccess)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserDataTaskListOtherAppReadAccess>(result__)
        }
    }
    pub fn SetOtherAppReadAccess(&self, value: UserDataTaskListOtherAppReadAccess) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOtherAppReadAccess)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OtherAppWriteAccess(&self) -> ::windows_core::Result<UserDataTaskListOtherAppWriteAccess> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UserDataTaskListOtherAppWriteAccess>::zeroed();
            (::windows_core::Interface::vtable(this).OtherAppWriteAccess)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserDataTaskListOtherAppWriteAccess>(result__)
        }
    }
    pub fn SetOtherAppWriteAccess(&self, value: UserDataTaskListOtherAppWriteAccess) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOtherAppWriteAccess)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LimitedWriteOperations(&self) -> ::windows_core::Result<UserDataTaskListLimitedWriteOperations> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LimitedWriteOperations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserDataTaskListLimitedWriteOperations>(result__)
        }
    }
    pub fn SyncManager(&self) -> ::windows_core::Result<UserDataTaskListSyncManager> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SyncManager)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserDataTaskListSyncManager>(result__)
        }
    }
    pub fn RegisterSyncManagerAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RegisterSyncManagerAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn GetTaskReader(&self) -> ::windows_core::Result<UserDataTaskReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetTaskReader)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserDataTaskReader>(result__)
        }
    }
    pub fn GetTaskReaderWithOptions<'a, Param0: ::windows_core::IntoParam<'a, UserDataTaskQueryOptions>>(&self, options: Param0) -> ::windows_core::Result<UserDataTaskReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetTaskReaderWithOptions)(::windows_core::Interface::as_raw(this), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<UserDataTaskReader>(result__)
        }
    }
    pub fn GetTaskAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, userdatatask: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<UserDataTask>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetTaskAsync)(::windows_core::Interface::as_raw(this), userdatatask.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<UserDataTask>>(result__)
        }
    }
    pub fn SaveTaskAsync<'a, Param0: ::windows_core::IntoParam<'a, UserDataTask>>(&self, userdatatask: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SaveTaskAsync)(::windows_core::Interface::as_raw(this), userdatatask.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn DeleteTaskAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, userdatataskid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteTaskAsync)(::windows_core::Interface::as_raw(this), userdatataskid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn DeleteAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn SaveAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SaveAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskList {}
impl ::core::fmt::Debug for UserDataTaskList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskList").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskList {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskList;{49412e39-7c1d-4df1-bed3-314b7cbf5e4e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataTaskList {
    type Vtable = IUserDataTaskList_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskList as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskList {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskList";
}
impl ::core::convert::From<UserDataTaskList> for ::windows_core::IUnknown {
    fn from(value: UserDataTaskList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskList> for ::windows_core::IUnknown {
    fn from(value: &UserDataTaskList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataTaskList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataTaskList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskList> for ::windows_core::IInspectable {
    fn from(value: UserDataTaskList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskList> for ::windows_core::IInspectable {
    fn from(value: &UserDataTaskList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataTaskList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataTaskList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskList {}
unsafe impl ::core::marker::Sync for UserDataTaskList {}
#[repr(transparent)]
pub struct UserDataTaskListLimitedWriteOperations(::windows_core::IUnknown);
impl UserDataTaskListLimitedWriteOperations {
    pub fn TryCompleteTaskAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, userdatataskid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryCompleteTaskAsync)(::windows_core::Interface::as_raw(this), userdatataskid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn TryCreateOrUpdateTaskAsync<'a, Param0: ::windows_core::IntoParam<'a, UserDataTask>>(&self, userdatatask: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateOrUpdateTaskAsync)(::windows_core::Interface::as_raw(this), userdatatask.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TryDeleteTaskAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, userdatataskid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryDeleteTaskAsync)(::windows_core::Interface::as_raw(this), userdatataskid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TrySkipOccurrenceAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, userdatataskid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TrySkipOccurrenceAsync)(::windows_core::Interface::as_raw(this), userdatataskid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskListLimitedWriteOperations {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskListLimitedWriteOperations {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskListLimitedWriteOperations {}
impl ::core::fmt::Debug for UserDataTaskListLimitedWriteOperations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskListLimitedWriteOperations").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskListLimitedWriteOperations {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskListLimitedWriteOperations;{7aa267f2-6078-4183-919e-4f29f19cfae9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataTaskListLimitedWriteOperations {
    type Vtable = IUserDataTaskListLimitedWriteOperations_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskListLimitedWriteOperations as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskListLimitedWriteOperations {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskListLimitedWriteOperations";
}
impl ::core::convert::From<UserDataTaskListLimitedWriteOperations> for ::windows_core::IUnknown {
    fn from(value: UserDataTaskListLimitedWriteOperations) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskListLimitedWriteOperations> for ::windows_core::IUnknown {
    fn from(value: &UserDataTaskListLimitedWriteOperations) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataTaskListLimitedWriteOperations {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataTaskListLimitedWriteOperations {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskListLimitedWriteOperations> for ::windows_core::IInspectable {
    fn from(value: UserDataTaskListLimitedWriteOperations) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskListLimitedWriteOperations> for ::windows_core::IInspectable {
    fn from(value: &UserDataTaskListLimitedWriteOperations) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataTaskListLimitedWriteOperations {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataTaskListLimitedWriteOperations {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskListLimitedWriteOperations {}
unsafe impl ::core::marker::Sync for UserDataTaskListLimitedWriteOperations {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for UserDataTaskListOtherAppReadAccess {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserDataTaskListOtherAppReadAccess {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataTaskListOtherAppReadAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskListOtherAppReadAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskListOtherAppReadAccess {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskListOtherAppReadAccess;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for UserDataTaskListOtherAppWriteAccess {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserDataTaskListOtherAppWriteAccess {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataTaskListOtherAppWriteAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskListOtherAppWriteAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskListOtherAppWriteAccess {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskListOtherAppWriteAccess;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct UserDataTaskListSyncManager(::windows_core::IUnknown);
impl UserDataTaskListSyncManager {
    pub fn LastAttemptedSyncTime(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).LastAttemptedSyncTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn SetLastAttemptedSyncTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLastAttemptedSyncTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn LastSuccessfulSyncTime(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).LastSuccessfulSyncTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn SetLastSuccessfulSyncTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLastSuccessfulSyncTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Status(&self) -> ::windows_core::Result<UserDataTaskListSyncStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UserDataTaskListSyncStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserDataTaskListSyncStatus>(result__)
        }
    }
    pub fn SetStatus(&self, value: UserDataTaskListSyncStatus) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStatus)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SyncAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SyncAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn SyncStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<UserDataTaskListSyncManager, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SyncStatusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSyncStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSyncStatusChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for UserDataTaskListSyncManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskListSyncManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskListSyncManager {}
impl ::core::fmt::Debug for UserDataTaskListSyncManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskListSyncManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskListSyncManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskListSyncManager;{8e591a95-1dcf-469f-93ec-ba48bb553c6b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataTaskListSyncManager {
    type Vtable = IUserDataTaskListSyncManager_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskListSyncManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskListSyncManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskListSyncManager";
}
impl ::core::convert::From<UserDataTaskListSyncManager> for ::windows_core::IUnknown {
    fn from(value: UserDataTaskListSyncManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskListSyncManager> for ::windows_core::IUnknown {
    fn from(value: &UserDataTaskListSyncManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataTaskListSyncManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataTaskListSyncManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskListSyncManager> for ::windows_core::IInspectable {
    fn from(value: UserDataTaskListSyncManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskListSyncManager> for ::windows_core::IInspectable {
    fn from(value: &UserDataTaskListSyncManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataTaskListSyncManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataTaskListSyncManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskListSyncManager {}
unsafe impl ::core::marker::Sync for UserDataTaskListSyncManager {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for UserDataTaskListSyncStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserDataTaskListSyncStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataTaskListSyncStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskListSyncStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskListSyncStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskListSyncStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct UserDataTaskManager(::windows_core::IUnknown);
impl UserDataTaskManager {
    pub fn RequestStoreAsync(&self, accesstype: UserDataTaskStoreAccessType) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<UserDataTaskStore>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestStoreAsync)(::windows_core::Interface::as_raw(this), accesstype, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<UserDataTaskStore>>(result__)
        }
    }
    #[cfg(feature = "winrt-system")]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<UserDataTaskManager> {
        Self::IUserDataTaskManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserDataTaskManager>(result__)
        })
    }
    #[cfg(feature = "winrt-system")]
    pub fn GetForUser<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>>(user: Param0) -> ::windows_core::Result<UserDataTaskManager> {
        Self::IUserDataTaskManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), result__.as_mut_ptr()).from_abi::<UserDataTaskManager>(result__)
        })
    }
    pub fn IUserDataTaskManagerStatics<R, F: FnOnce(&IUserDataTaskManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UserDataTaskManager, IUserDataTaskManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UserDataTaskManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskManager {}
impl ::core::fmt::Debug for UserDataTaskManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskManager;{8451c914-e60b-48a9-9211-7fb8a56cb84c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataTaskManager {
    type Vtable = IUserDataTaskManager_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskManager";
}
impl ::core::convert::From<UserDataTaskManager> for ::windows_core::IUnknown {
    fn from(value: UserDataTaskManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskManager> for ::windows_core::IUnknown {
    fn from(value: &UserDataTaskManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataTaskManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataTaskManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskManager> for ::windows_core::IInspectable {
    fn from(value: UserDataTaskManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskManager> for ::windows_core::IInspectable {
    fn from(value: &UserDataTaskManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataTaskManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataTaskManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskManager {}
unsafe impl ::core::marker::Sync for UserDataTaskManager {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for UserDataTaskPriority {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserDataTaskPriority {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataTaskPriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskPriority").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskPriority {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskPriority;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for UserDataTaskQueryKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserDataTaskQueryKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataTaskQueryKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskQueryKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskQueryKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskQueryKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct UserDataTaskQueryOptions(::windows_core::IUnknown);
impl UserDataTaskQueryOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UserDataTaskQueryOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn SortProperty(&self) -> ::windows_core::Result<UserDataTaskQuerySortProperty> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UserDataTaskQuerySortProperty>::zeroed();
            (::windows_core::Interface::vtable(this).SortProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserDataTaskQuerySortProperty>(result__)
        }
    }
    pub fn SetSortProperty(&self, value: UserDataTaskQuerySortProperty) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSortProperty)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<UserDataTaskQueryKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UserDataTaskQueryKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserDataTaskQueryKind>(result__)
        }
    }
    pub fn SetKind(&self, value: UserDataTaskQueryKind) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetKind)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for UserDataTaskQueryOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskQueryOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskQueryOptions {}
impl ::core::fmt::Debug for UserDataTaskQueryOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskQueryOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskQueryOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskQueryOptions;{959f27ed-909a-4d30-8c1b-331d8fe667e2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataTaskQueryOptions {
    type Vtable = IUserDataTaskQueryOptions_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskQueryOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskQueryOptions {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskQueryOptions";
}
impl ::core::convert::From<UserDataTaskQueryOptions> for ::windows_core::IUnknown {
    fn from(value: UserDataTaskQueryOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskQueryOptions> for ::windows_core::IUnknown {
    fn from(value: &UserDataTaskQueryOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataTaskQueryOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataTaskQueryOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskQueryOptions> for ::windows_core::IInspectable {
    fn from(value: UserDataTaskQueryOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskQueryOptions> for ::windows_core::IInspectable {
    fn from(value: &UserDataTaskQueryOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataTaskQueryOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataTaskQueryOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskQueryOptions {}
unsafe impl ::core::marker::Sync for UserDataTaskQueryOptions {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for UserDataTaskQuerySortProperty {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserDataTaskQuerySortProperty {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataTaskQuerySortProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskQuerySortProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskQuerySortProperty {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskQuerySortProperty;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct UserDataTaskReader(::windows_core::IUnknown);
impl UserDataTaskReader {
    pub fn ReadBatchAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<UserDataTaskBatch>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadBatchAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<UserDataTaskBatch>>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskReader {}
impl ::core::fmt::Debug for UserDataTaskReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskReader").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskReader {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskReader;{03e688b1-4ccf-4500-883b-e76290cfed63})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataTaskReader {
    type Vtable = IUserDataTaskReader_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskReader as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskReader {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskReader";
}
impl ::core::convert::From<UserDataTaskReader> for ::windows_core::IUnknown {
    fn from(value: UserDataTaskReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskReader> for ::windows_core::IUnknown {
    fn from(value: &UserDataTaskReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataTaskReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataTaskReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskReader> for ::windows_core::IInspectable {
    fn from(value: UserDataTaskReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskReader> for ::windows_core::IInspectable {
    fn from(value: &UserDataTaskReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataTaskReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataTaskReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskReader {}
unsafe impl ::core::marker::Sync for UserDataTaskReader {}
#[repr(transparent)]
pub struct UserDataTaskRecurrenceProperties(::windows_core::IUnknown);
impl UserDataTaskRecurrenceProperties {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UserDataTaskRecurrenceProperties, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Unit(&self) -> ::windows_core::Result<UserDataTaskRecurrenceUnit> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UserDataTaskRecurrenceUnit>::zeroed();
            (::windows_core::Interface::vtable(this).Unit)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserDataTaskRecurrenceUnit>(result__)
        }
    }
    pub fn SetUnit(&self, value: UserDataTaskRecurrenceUnit) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUnit)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Occurrences(&self) -> ::windows_core::Result<::winrt_foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Occurrences)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<i32>>(result__)
        }
    }
    pub fn SetOccurrences<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<i32>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOccurrences)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Until(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Until)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn SetUntil<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::DateTime>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUntil)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Interval(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Interval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetInterval(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInterval)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DaysOfWeek(&self) -> ::windows_core::Result<::winrt_foundation::IReference<UserDataTaskDaysOfWeek>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DaysOfWeek)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<UserDataTaskDaysOfWeek>>(result__)
        }
    }
    pub fn SetDaysOfWeek<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<UserDataTaskDaysOfWeek>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDaysOfWeek)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn WeekOfMonth(&self) -> ::windows_core::Result<::winrt_foundation::IReference<UserDataTaskWeekOfMonth>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WeekOfMonth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<UserDataTaskWeekOfMonth>>(result__)
        }
    }
    pub fn SetWeekOfMonth<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<UserDataTaskWeekOfMonth>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetWeekOfMonth)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Month(&self) -> ::windows_core::Result<::winrt_foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Month)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<i32>>(result__)
        }
    }
    pub fn SetMonth<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<i32>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMonth)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Day(&self) -> ::windows_core::Result<::winrt_foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Day)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<i32>>(result__)
        }
    }
    pub fn SetDay<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<i32>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDay)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for UserDataTaskRecurrenceProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskRecurrenceProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskRecurrenceProperties {}
impl ::core::fmt::Debug for UserDataTaskRecurrenceProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskRecurrenceProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskRecurrenceProperties {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskRecurrenceProperties;{73df80b0-27c6-40ce-b149-9cd41485a69e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataTaskRecurrenceProperties {
    type Vtable = IUserDataTaskRecurrenceProperties_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskRecurrenceProperties as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskRecurrenceProperties {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskRecurrenceProperties";
}
impl ::core::convert::From<UserDataTaskRecurrenceProperties> for ::windows_core::IUnknown {
    fn from(value: UserDataTaskRecurrenceProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskRecurrenceProperties> for ::windows_core::IUnknown {
    fn from(value: &UserDataTaskRecurrenceProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataTaskRecurrenceProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataTaskRecurrenceProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskRecurrenceProperties> for ::windows_core::IInspectable {
    fn from(value: UserDataTaskRecurrenceProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskRecurrenceProperties> for ::windows_core::IInspectable {
    fn from(value: &UserDataTaskRecurrenceProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataTaskRecurrenceProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataTaskRecurrenceProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskRecurrenceProperties {}
unsafe impl ::core::marker::Sync for UserDataTaskRecurrenceProperties {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for UserDataTaskRecurrenceUnit {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserDataTaskRecurrenceUnit {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataTaskRecurrenceUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskRecurrenceUnit").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskRecurrenceUnit {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskRecurrenceUnit;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct UserDataTaskRegenerationProperties(::windows_core::IUnknown);
impl UserDataTaskRegenerationProperties {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UserDataTaskRegenerationProperties, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Unit(&self) -> ::windows_core::Result<UserDataTaskRegenerationUnit> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UserDataTaskRegenerationUnit>::zeroed();
            (::windows_core::Interface::vtable(this).Unit)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserDataTaskRegenerationUnit>(result__)
        }
    }
    pub fn SetUnit(&self, value: UserDataTaskRegenerationUnit) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUnit)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Occurrences(&self) -> ::windows_core::Result<::winrt_foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Occurrences)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<i32>>(result__)
        }
    }
    pub fn SetOccurrences<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<i32>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOccurrences)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Until(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Until)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn SetUntil<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::DateTime>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUntil)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Interval(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Interval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetInterval(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInterval)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for UserDataTaskRegenerationProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskRegenerationProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskRegenerationProperties {}
impl ::core::fmt::Debug for UserDataTaskRegenerationProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskRegenerationProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskRegenerationProperties {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskRegenerationProperties;{92ab0007-090e-4704-bb5c-84fc0b0d9c31})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataTaskRegenerationProperties {
    type Vtable = IUserDataTaskRegenerationProperties_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskRegenerationProperties as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskRegenerationProperties {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskRegenerationProperties";
}
impl ::core::convert::From<UserDataTaskRegenerationProperties> for ::windows_core::IUnknown {
    fn from(value: UserDataTaskRegenerationProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskRegenerationProperties> for ::windows_core::IUnknown {
    fn from(value: &UserDataTaskRegenerationProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataTaskRegenerationProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataTaskRegenerationProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskRegenerationProperties> for ::windows_core::IInspectable {
    fn from(value: UserDataTaskRegenerationProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskRegenerationProperties> for ::windows_core::IInspectable {
    fn from(value: &UserDataTaskRegenerationProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataTaskRegenerationProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataTaskRegenerationProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskRegenerationProperties {}
unsafe impl ::core::marker::Sync for UserDataTaskRegenerationProperties {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for UserDataTaskRegenerationUnit {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserDataTaskRegenerationUnit {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataTaskRegenerationUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskRegenerationUnit").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskRegenerationUnit {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskRegenerationUnit;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for UserDataTaskSensitivity {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserDataTaskSensitivity {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataTaskSensitivity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskSensitivity").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskSensitivity {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskSensitivity;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct UserDataTaskStore(::windows_core::IUnknown);
impl UserDataTaskStore {
    pub fn CreateListAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<UserDataTaskList>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateListAsync)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<UserDataTaskList>>(result__)
        }
    }
    pub fn CreateListInAccountAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0, userdataaccountid: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<UserDataTaskList>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateListInAccountAsync)(::windows_core::Interface::as_raw(this), name.into_param().abi(), userdataaccountid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<UserDataTaskList>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn FindListsAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<UserDataTaskList>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindListsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<UserDataTaskList>>>(result__)
        }
    }
    pub fn GetListAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, tasklistid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<UserDataTaskList>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetListAsync)(::windows_core::Interface::as_raw(this), tasklistid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<UserDataTaskList>>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskStore {}
impl ::core::fmt::Debug for UserDataTaskStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskStore").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskStore {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskStore;{f06a9cb0-f1db-45ba-8a62-086004c0213d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataTaskStore {
    type Vtable = IUserDataTaskStore_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskStore as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskStore {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskStore";
}
impl ::core::convert::From<UserDataTaskStore> for ::windows_core::IUnknown {
    fn from(value: UserDataTaskStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskStore> for ::windows_core::IUnknown {
    fn from(value: &UserDataTaskStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataTaskStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataTaskStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskStore> for ::windows_core::IInspectable {
    fn from(value: UserDataTaskStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskStore> for ::windows_core::IInspectable {
    fn from(value: &UserDataTaskStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataTaskStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataTaskStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskStore {}
unsafe impl ::core::marker::Sync for UserDataTaskStore {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for UserDataTaskStoreAccessType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserDataTaskStoreAccessType {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataTaskStoreAccessType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskStoreAccessType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskStoreAccessType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskStoreAccessType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for UserDataTaskWeekOfMonth {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserDataTaskWeekOfMonth {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataTaskWeekOfMonth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskWeekOfMonth").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskWeekOfMonth {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskWeekOfMonth;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
