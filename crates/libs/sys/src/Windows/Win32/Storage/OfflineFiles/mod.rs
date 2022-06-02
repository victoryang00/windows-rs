#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OfflineFilesEnable(benable: super::super::Foundation::BOOL, pbrebootrequired: *mut super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OfflineFilesQueryStatus(pbactive: *mut super::super::Foundation::BOOL, pbenabled: *mut super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OfflineFilesQueryStatusEx(pbactive: *mut super::super::Foundation::BOOL, pbenabled: *mut super::super::Foundation::BOOL, pbavailable: *mut super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
    pub fn OfflineFilesStart() -> u32;
}
#[repr(C)]
pub struct IEnumOfflineFilesItems {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnumOfflineFilesSettings {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOfflineFilesCache {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Synchronize: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const ::windows_sys::core::PWSTR, cpaths: u32, basync: super::super::Foundation::BOOL, dwsynccontrol: u32, pisyncconflicthandler: *mut ::core::ffi::c_void, piprogress: *mut ::core::ffi::c_void, psyncid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Synchronize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteItems: unsafe extern "system" fn(this: *mut *mut Self, rgpszpaths: *const ::windows_sys::core::PWSTR, cpaths: u32, dwflags: u32, basync: super::super::Foundation::BOOL, piprogress: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteItems: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteItemsForUser: unsafe extern "system" fn(this: *mut *mut Self, pszuser: ::windows_sys::core::PCWSTR, rgpszpaths: *const ::windows_sys::core::PWSTR, cpaths: u32, dwflags: u32, basync: super::super::Foundation::BOOL, piprogress: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteItemsForUser: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Pin: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const ::windows_sys::core::PWSTR, cpaths: u32, bdeep: super::super::Foundation::BOOL, basync: super::super::Foundation::BOOL, dwpincontrolflags: u32, piprogress: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Pin: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Unpin: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const ::windows_sys::core::PWSTR, cpaths: u32, bdeep: super::super::Foundation::BOOL, basync: super::super::Foundation::BOOL, dwpincontrolflags: u32, piprogress: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Unpin: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEncryptionStatus: unsafe extern "system" fn(this: *mut *mut Self, pbencrypted: *mut super::super::Foundation::BOOL, pbpartial: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEncryptionStatus: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Encrypt: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::Foundation::HWND, bencrypt: super::super::Foundation::BOOL, dwencryptioncontrolflags: u32, basync: super::super::Foundation::BOOL, piprogress: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Encrypt: usize,
    pub FindItem: unsafe extern "system" fn(this: *mut *mut Self, pszpath: ::windows_sys::core::PCWSTR, dwqueryflags: u32, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FindItemEx: unsafe extern "system" fn(this: *mut *mut Self, pszpath: ::windows_sys::core::PCWSTR, pincludefilefilter: *mut ::core::ffi::c_void, pincludedirfilter: *mut ::core::ffi::c_void, pexcludefilefilter: *mut ::core::ffi::c_void, pexcludedirfilter: *mut ::core::ffi::c_void, dwqueryflags: u32, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RenameItem: unsafe extern "system" fn(this: *mut *mut Self, pszpathoriginal: ::windows_sys::core::PCWSTR, pszpathnew: ::windows_sys::core::PCWSTR, breplaceifexists: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RenameItem: usize,
    pub GetLocation: unsafe extern "system" fn(this: *mut *mut Self, ppszpath: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetDiskSpaceInformation: unsafe extern "system" fn(this: *mut *mut Self, pcbvolumetotal: *mut u64, pcblimit: *mut u64, pcbused: *mut u64, pcbunpinnedlimit: *mut u64, pcbunpinnedused: *mut u64) -> ::windows_sys::core::HRESULT,
    pub SetDiskSpaceLimits: unsafe extern "system" fn(this: *mut *mut Self, cblimit: u64, cbunpinnedlimit: u64) -> ::windows_sys::core::HRESULT,
    pub ProcessAdminPinPolicy: unsafe extern "system" fn(this: *mut *mut Self, ppinprogress: *mut ::core::ffi::c_void, punpinprogress: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSettingObject: unsafe extern "system" fn(this: *mut *mut Self, pszsettingname: ::windows_sys::core::PCWSTR, ppsetting: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EnumSettingObjects: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPathCacheable: unsafe extern "system" fn(this: *mut *mut Self, pszpath: ::windows_sys::core::PCWSTR, pbcacheable: *mut super::super::Foundation::BOOL, psharecachingmode: *mut OFFLINEFILES_CACHING_MODE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPathCacheable: usize,
}
#[repr(C)]
pub struct IOfflineFilesCache2 {
    pub base__: IOfflineFilesCache,
    #[cfg(feature = "Win32_Foundation")]
    pub RenameItemEx: unsafe extern "system" fn(this: *mut *mut Self, pszpathoriginal: ::windows_sys::core::PCWSTR, pszpathnew: ::windows_sys::core::PCWSTR, breplaceifexists: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RenameItemEx: usize,
}
#[repr(C)]
pub struct IOfflineFilesChangeInfo {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub IsDirty: unsafe extern "system" fn(this: *mut *mut Self, pbdirty: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsDirty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsDeletedOffline: unsafe extern "system" fn(this: *mut *mut Self, pbdeletedoffline: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsDeletedOffline: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsCreatedOffline: unsafe extern "system" fn(this: *mut *mut Self, pbcreatedoffline: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsCreatedOffline: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsLocallyModifiedData: unsafe extern "system" fn(this: *mut *mut Self, pblocallymodifieddata: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsLocallyModifiedData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsLocallyModifiedAttributes: unsafe extern "system" fn(this: *mut *mut Self, pblocallymodifiedattributes: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsLocallyModifiedAttributes: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsLocallyModifiedTime: unsafe extern "system" fn(this: *mut *mut Self, pblocallymodifiedtime: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsLocallyModifiedTime: usize,
}
#[repr(C)]
pub struct IOfflineFilesConnectionInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetConnectState: unsafe extern "system" fn(this: *mut *mut Self, pconnectstate: *mut OFFLINEFILES_CONNECT_STATE, pofflinereason: *mut OFFLINEFILES_OFFLINE_REASON) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetConnectState: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::Foundation::HWND, dwflags: u32, connectstate: OFFLINEFILES_CONNECT_STATE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetConnectState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TransitionOnline: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TransitionOnline: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TransitionOffline: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::Foundation::HWND, dwflags: u32, bforceopenfilesclosed: super::super::Foundation::BOOL, pbopenfilespreventedtransition: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TransitionOffline: usize,
}
#[repr(C)]
pub struct IOfflineFilesDirectoryItem {
    pub base__: IOfflineFilesItem,
}
#[repr(C)]
pub struct IOfflineFilesDirtyInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub LocalDirtyByteCount: unsafe extern "system" fn(this: *mut *mut Self, pdirtybytecount: *mut i64) -> ::windows_sys::core::HRESULT,
    pub RemoteDirtyByteCount: unsafe extern "system" fn(this: *mut *mut Self, pdirtybytecount: *mut i64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOfflineFilesErrorInfo {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRawData: unsafe extern "system" fn(this: *mut *mut Self, ppblob: *mut *mut super::super::System::Com::BYTE_BLOB) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRawData: usize,
    pub GetDescription: unsafe extern "system" fn(this: *mut *mut Self, ppszdescription: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOfflineFilesEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub CacheMoved: unsafe extern "system" fn(this: *mut *mut Self, pszoldpath: ::windows_sys::core::PCWSTR, psznewpath: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub CacheIsFull: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub CacheIsCorrupted: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, benabled: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EncryptionChanged: unsafe extern "system" fn(this: *mut *mut Self, bwasencrypted: super::super::Foundation::BOOL, bwaspartial: super::super::Foundation::BOOL, bisencrypted: super::super::Foundation::BOOL, bispartial: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EncryptionChanged: usize,
    pub SyncBegin: unsafe extern "system" fn(this: *mut *mut Self, rsyncid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SyncFileResult: unsafe extern "system" fn(this: *mut *mut Self, rsyncid: *const ::windows_sys::core::GUID, pszfile: ::windows_sys::core::PCWSTR, hrresult: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SyncConflictRecAdded: unsafe extern "system" fn(this: *mut *mut Self, pszconflictpath: ::windows_sys::core::PCWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SyncConflictRecAdded: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SyncConflictRecUpdated: unsafe extern "system" fn(this: *mut *mut Self, pszconflictpath: ::windows_sys::core::PCWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SyncConflictRecUpdated: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SyncConflictRecRemoved: unsafe extern "system" fn(this: *mut *mut Self, pszconflictpath: ::windows_sys::core::PCWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SyncConflictRecRemoved: usize,
    pub SyncEnd: unsafe extern "system" fn(this: *mut *mut Self, rsyncid: *const ::windows_sys::core::GUID, hrresult: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub NetTransportArrived: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub NoNetTransports: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ItemDisconnected: unsafe extern "system" fn(this: *mut *mut Self, pszpath: ::windows_sys::core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_sys::core::HRESULT,
    pub ItemReconnected: unsafe extern "system" fn(this: *mut *mut Self, pszpath: ::windows_sys::core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_sys::core::HRESULT,
    pub ItemAvailableOffline: unsafe extern "system" fn(this: *mut *mut Self, pszpath: ::windows_sys::core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_sys::core::HRESULT,
    pub ItemNotAvailableOffline: unsafe extern "system" fn(this: *mut *mut Self, pszpath: ::windows_sys::core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_sys::core::HRESULT,
    pub ItemPinned: unsafe extern "system" fn(this: *mut *mut Self, pszpath: ::windows_sys::core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_sys::core::HRESULT,
    pub ItemNotPinned: unsafe extern "system" fn(this: *mut *mut Self, pszpath: ::windows_sys::core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ItemModified: unsafe extern "system" fn(this: *mut *mut Self, pszpath: ::windows_sys::core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: super::super::Foundation::BOOL, bmodifiedattributes: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ItemModified: usize,
    pub ItemAddedToCache: unsafe extern "system" fn(this: *mut *mut Self, pszpath: ::windows_sys::core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_sys::core::HRESULT,
    pub ItemDeletedFromCache: unsafe extern "system" fn(this: *mut *mut Self, pszpath: ::windows_sys::core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_sys::core::HRESULT,
    pub ItemRenamed: unsafe extern "system" fn(this: *mut *mut Self, pszoldpath: ::windows_sys::core::PCWSTR, psznewpath: ::windows_sys::core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_sys::core::HRESULT,
    pub DataLost: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Ping: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOfflineFilesEvents2 {
    pub base__: IOfflineFilesEvents,
    pub ItemReconnectBegin: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ItemReconnectEnd: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub CacheEvictBegin: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub CacheEvictEnd: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub BackgroundSyncBegin: unsafe extern "system" fn(this: *mut *mut Self, dwsynccontrolflags: u32) -> ::windows_sys::core::HRESULT,
    pub BackgroundSyncEnd: unsafe extern "system" fn(this: *mut *mut Self, dwsynccontrolflags: u32) -> ::windows_sys::core::HRESULT,
    pub PolicyChangeDetected: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub PreferenceChangeDetected: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SettingsChangesApplied: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOfflineFilesEvents3 {
    pub base__: IOfflineFilesEvents2,
    #[cfg(feature = "Win32_Foundation")]
    pub TransparentCacheItemNotify: unsafe extern "system" fn(this: *mut *mut Self, pszpath: ::windows_sys::core::PCWSTR, eventtype: OFFLINEFILES_EVENTS, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: super::super::Foundation::BOOL, bmodifiedattributes: super::super::Foundation::BOOL, pzsoldpath: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TransparentCacheItemNotify: usize,
    pub PrefetchFileBegin: unsafe extern "system" fn(this: *mut *mut Self, pszpath: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub PrefetchFileEnd: unsafe extern "system" fn(this: *mut *mut Self, pszpath: ::windows_sys::core::PCWSTR, hrresult: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOfflineFilesEvents4 {
    pub base__: IOfflineFilesEvents3,
    pub PrefetchCloseHandleBegin: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub PrefetchCloseHandleEnd: unsafe extern "system" fn(this: *mut *mut Self, dwclosedhandlecount: u32, dwopenhandlecount: u32, hrresult: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOfflineFilesEventsFilter {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetPathFilter: unsafe extern "system" fn(this: *mut *mut Self, ppszfilter: *mut ::windows_sys::core::PWSTR, pmatch: *mut OFFLINEFILES_PATHFILTER_MATCH) -> ::windows_sys::core::HRESULT,
    pub GetIncludedEvents: unsafe extern "system" fn(this: *mut *mut Self, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetExcludedEvents: unsafe extern "system" fn(this: *mut *mut Self, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOfflineFilesFileItem {
    pub base__: IOfflineFilesItem,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSparse: unsafe extern "system" fn(this: *mut *mut Self, pbissparse: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSparse: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEncrypted: unsafe extern "system" fn(this: *mut *mut Self, pbisencrypted: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEncrypted: usize,
}
#[repr(C)]
pub struct IOfflineFilesFileSysInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetAttributes: unsafe extern "system" fn(this: *mut *mut Self, copy: OFFLINEFILES_ITEM_COPY, pdwattributes: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTimes: unsafe extern "system" fn(this: *mut *mut Self, copy: OFFLINEFILES_ITEM_COPY, pftcreationtime: *mut super::super::Foundation::FILETIME, pftlastwritetime: *mut super::super::Foundation::FILETIME, pftchangetime: *mut super::super::Foundation::FILETIME, pftlastaccesstime: *mut super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTimes: usize,
    pub GetFileSize: unsafe extern "system" fn(this: *mut *mut Self, copy: OFFLINEFILES_ITEM_COPY, psize: *mut i64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOfflineFilesGhostInfo {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub IsGhosted: unsafe extern "system" fn(this: *mut *mut Self, pbghosted: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsGhosted: usize,
}
#[repr(C)]
pub struct IOfflineFilesItem {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetItemType: unsafe extern "system" fn(this: *mut *mut Self, pitemtype: *mut OFFLINEFILES_ITEM_TYPE) -> ::windows_sys::core::HRESULT,
    pub GetPath: unsafe extern "system" fn(this: *mut *mut Self, ppszpath: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetParentItem: unsafe extern "system" fn(this: *mut *mut Self, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self, dwqueryflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsMarkedForDeletion: unsafe extern "system" fn(this: *mut *mut Self, pbmarkedfordeletion: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsMarkedForDeletion: usize,
}
#[repr(C)]
pub struct IOfflineFilesItemContainer {
    pub base__: ::windows_sys::core::IUnknown,
    pub EnumItems: unsafe extern "system" fn(this: *mut *mut Self, dwqueryflags: u32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EnumItemsEx: unsafe extern "system" fn(this: *mut *mut Self, pincludefilefilter: *mut ::core::ffi::c_void, pincludedirfilter: *mut ::core::ffi::c_void, pexcludefilefilter: *mut ::core::ffi::c_void, pexcludedirfilter: *mut ::core::ffi::c_void, dwenumflags: u32, dwqueryflags: u32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOfflineFilesItemFilter {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetFilterFlags: unsafe extern "system" fn(this: *mut *mut Self, pullflags: *mut u64, pullmask: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTimeFilter: unsafe extern "system" fn(this: *mut *mut Self, pfttime: *mut super::super::Foundation::FILETIME, pbevaltimeofday: *mut super::super::Foundation::BOOL, ptimetype: *mut OFFLINEFILES_ITEM_TIME, pcompare: *mut OFFLINEFILES_COMPARE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTimeFilter: usize,
    pub GetPatternFilter: unsafe extern "system" fn(this: *mut *mut Self, pszpattern: ::windows_sys::core::PWSTR, cchpattern: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOfflineFilesPinInfo {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPinned: unsafe extern "system" fn(this: *mut *mut Self, pbpinned: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPinned: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPinnedForUser: unsafe extern "system" fn(this: *mut *mut Self, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPinnedForUser: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPinnedForUserByPolicy: unsafe extern "system" fn(this: *mut *mut Self, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPinnedForUserByPolicy: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPinnedForComputer: unsafe extern "system" fn(this: *mut *mut Self, pbpinnedforcomputer: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPinnedForComputer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPinnedForFolderRedirection: unsafe extern "system" fn(this: *mut *mut Self, pbpinnedforfolderredirection: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPinnedForFolderRedirection: usize,
}
#[repr(C)]
pub struct IOfflineFilesPinInfo2 {
    pub base__: IOfflineFilesPinInfo,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPartlyPinned: unsafe extern "system" fn(this: *mut *mut Self, pbpartlypinned: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPartlyPinned: usize,
}
#[repr(C)]
pub struct IOfflineFilesProgress {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Begin: unsafe extern "system" fn(this: *mut *mut Self, pbabort: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Begin: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryAbort: unsafe extern "system" fn(this: *mut *mut Self, pbabort: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryAbort: usize,
    pub End: unsafe extern "system" fn(this: *mut *mut Self, hrresult: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOfflineFilesServerItem {
    pub base__: IOfflineFilesItem,
}
#[repr(C)]
pub struct IOfflineFilesSetting {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, ppszname: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetValueType: unsafe extern "system" fn(this: *mut *mut Self, ptype: *mut OFFLINEFILES_SETTING_VALUE_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetPreference: unsafe extern "system" fn(this: *mut *mut Self, pvarvalue: *mut super::super::System::Com::VARIANT, dwscope: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetPreference: usize,
    pub GetPreferenceScope: unsafe extern "system" fn(this: *mut *mut Self, pdwscope: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetPreference: unsafe extern "system" fn(this: *mut *mut Self, pvarvalue: *const super::super::System::Com::VARIANT, dwscope: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetPreference: usize,
    pub DeletePreference: unsafe extern "system" fn(this: *mut *mut Self, dwscope: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetPolicy: unsafe extern "system" fn(this: *mut *mut Self, pvarvalue: *mut super::super::System::Com::VARIANT, dwscope: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetPolicy: usize,
    pub GetPolicyScope: unsafe extern "system" fn(this: *mut *mut Self, pdwscope: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetValue: unsafe extern "system" fn(this: *mut *mut Self, pvarvalue: *mut super::super::System::Com::VARIANT, pbsetbypolicy: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetValue: usize,
}
#[repr(C)]
pub struct IOfflineFilesShareInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetShareItem: unsafe extern "system" fn(this: *mut *mut Self, ppshareitem: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetShareCachingMode: unsafe extern "system" fn(this: *mut *mut Self, pcachingmode: *mut OFFLINEFILES_CACHING_MODE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsShareDfsJunction: unsafe extern "system" fn(this: *mut *mut Self, pbisdfsjunction: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsShareDfsJunction: usize,
}
#[repr(C)]
pub struct IOfflineFilesShareItem {
    pub base__: IOfflineFilesItem,
}
#[repr(C)]
pub struct IOfflineFilesSimpleProgress {
    pub base__: IOfflineFilesProgress,
    pub ItemBegin: unsafe extern "system" fn(this: *mut *mut Self, pszfile: ::windows_sys::core::PCWSTR, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows_sys::core::HRESULT,
    pub ItemResult: unsafe extern "system" fn(this: *mut *mut Self, pszfile: ::windows_sys::core::PCWSTR, hrresult: ::windows_sys::core::HRESULT, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOfflineFilesSuspend {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub SuspendRoot: unsafe extern "system" fn(this: *mut *mut Self, bsuspend: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SuspendRoot: usize,
}
#[repr(C)]
pub struct IOfflineFilesSuspendInfo {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSuspended: unsafe extern "system" fn(this: *mut *mut Self, pbsuspended: *mut super::super::Foundation::BOOL, pbsuspendedroot: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSuspended: usize,
}
#[repr(C)]
pub struct IOfflineFilesSyncConflictHandler {
    pub base__: ::windows_sys::core::IUnknown,
    pub ResolveConflict: unsafe extern "system" fn(this: *mut *mut Self, pszpath: ::windows_sys::core::PCWSTR, fstateknown: u32, state: OFFLINEFILES_SYNC_STATE, fchangedetails: u32, pconflictresolution: *mut OFFLINEFILES_SYNC_CONFLICT_RESOLVE, ppsznewname: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOfflineFilesSyncErrorInfo {
    pub base__: IOfflineFilesErrorInfo,
    pub GetSyncOperation: unsafe extern "system" fn(this: *mut *mut Self, psyncop: *mut OFFLINEFILES_SYNC_OPERATION) -> ::windows_sys::core::HRESULT,
    pub GetItemChangeFlags: unsafe extern "system" fn(this: *mut *mut Self, pdwitemchangeflags: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub InfoEnumerated: unsafe extern "system" fn(this: *mut *mut Self, pblocalenumerated: *mut super::super::Foundation::BOOL, pbremoteenumerated: *mut super::super::Foundation::BOOL, pboriginalenumerated: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InfoEnumerated: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InfoAvailable: unsafe extern "system" fn(this: *mut *mut Self, pblocalinfo: *mut super::super::Foundation::BOOL, pbremoteinfo: *mut super::super::Foundation::BOOL, pboriginalinfo: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InfoAvailable: usize,
    pub GetLocalInfo: unsafe extern "system" fn(this: *mut *mut Self, ppinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetRemoteInfo: unsafe extern "system" fn(this: *mut *mut Self, ppinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetOriginalInfo: unsafe extern "system" fn(this: *mut *mut Self, ppinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOfflineFilesSyncErrorItemInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetFileAttributes: unsafe extern "system" fn(this: *mut *mut Self, pdwattributes: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFileTimes: unsafe extern "system" fn(this: *mut *mut Self, pftlastwrite: *mut super::super::Foundation::FILETIME, pftchange: *mut super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFileTimes: usize,
    pub GetFileSize: unsafe extern "system" fn(this: *mut *mut Self, psize: *mut i64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOfflineFilesSyncProgress {
    pub base__: IOfflineFilesProgress,
    pub SyncItemBegin: unsafe extern "system" fn(this: *mut *mut Self, pszfile: ::windows_sys::core::PCWSTR, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows_sys::core::HRESULT,
    pub SyncItemResult: unsafe extern "system" fn(this: *mut *mut Self, pszfile: ::windows_sys::core::PCWSTR, hrresult: ::windows_sys::core::HRESULT, perrorinfo: *mut ::core::ffi::c_void, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOfflineFilesTransparentCacheInfo {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub IsTransparentlyCached: unsafe extern "system" fn(this: *mut *mut Self, pbtransparentlycached: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsTransparentlyCached: usize,
}
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub type OFFLINEFILES_CACHING_MODE = i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_CACHING_MODE_NONE: OFFLINEFILES_CACHING_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_CACHING_MODE_NOCACHING: OFFLINEFILES_CACHING_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_CACHING_MODE_MANUAL: OFFLINEFILES_CACHING_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_CACHING_MODE_AUTO_DOC: OFFLINEFILES_CACHING_MODE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_CACHING_MODE_AUTO_PROGANDDOC: OFFLINEFILES_CACHING_MODE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_CHANGES_LOCAL_ATTRIBUTES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_CHANGES_LOCAL_SIZE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_CHANGES_LOCAL_TIME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_CHANGES_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_CHANGES_REMOTE_ATTRIBUTES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_CHANGES_REMOTE_SIZE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_CHANGES_REMOTE_TIME: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub type OFFLINEFILES_COMPARE = i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_COMPARE_EQ: OFFLINEFILES_COMPARE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_COMPARE_NEQ: OFFLINEFILES_COMPARE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_COMPARE_LT: OFFLINEFILES_COMPARE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_COMPARE_GT: OFFLINEFILES_COMPARE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_COMPARE_LTE: OFFLINEFILES_COMPARE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_COMPARE_GTE: OFFLINEFILES_COMPARE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub type OFFLINEFILES_CONNECT_STATE = i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_CONNECT_STATE_UNKNOWN: OFFLINEFILES_CONNECT_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_CONNECT_STATE_OFFLINE: OFFLINEFILES_CONNECT_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_CONNECT_STATE_ONLINE: OFFLINEFILES_CONNECT_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_CONNECT_STATE_TRANSPARENTLY_CACHED: OFFLINEFILES_CONNECT_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_CONNECT_STATE_PARTLY_TRANSPARENTLY_CACHED: OFFLINEFILES_CONNECT_STATE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_DELETE_FLAG_ADMIN: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_DELETE_FLAG_DELMODIFIED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_DELETE_FLAG_NOAUTOCACHED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_DELETE_FLAG_NOPINNED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ENCRYPTION_CONTROL_FLAG_ASYNCPROGRESS: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ENCRYPTION_CONTROL_FLAG_BACKGROUND: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ENCRYPTION_CONTROL_FLAG_CONSOLE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ENCRYPTION_CONTROL_FLAG_INTERACTIVE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ENCRYPTION_CONTROL_FLAG_LOWPRIORITY: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ENUM_FLAT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ENUM_FLAT_FILESONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub type OFFLINEFILES_EVENTS = i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_CACHEMOVED: OFFLINEFILES_EVENTS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_CACHEISFULL: OFFLINEFILES_EVENTS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_CACHEISCORRUPTED: OFFLINEFILES_EVENTS = 2i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_ENABLED: OFFLINEFILES_EVENTS = 3i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_ENCRYPTIONCHANGED: OFFLINEFILES_EVENTS = 4i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_SYNCBEGIN: OFFLINEFILES_EVENTS = 5i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_SYNCFILERESULT: OFFLINEFILES_EVENTS = 6i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_SYNCCONFLICTRECADDED: OFFLINEFILES_EVENTS = 7i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_SYNCCONFLICTRECUPDATED: OFFLINEFILES_EVENTS = 8i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_SYNCCONFLICTRECREMOVED: OFFLINEFILES_EVENTS = 9i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_SYNCEND: OFFLINEFILES_EVENTS = 10i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_BACKGROUNDSYNCBEGIN: OFFLINEFILES_EVENTS = 11i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_BACKGROUNDSYNCEND: OFFLINEFILES_EVENTS = 12i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_NETTRANSPORTARRIVED: OFFLINEFILES_EVENTS = 13i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_NONETTRANSPORTS: OFFLINEFILES_EVENTS = 14i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_ITEMDISCONNECTED: OFFLINEFILES_EVENTS = 15i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_ITEMRECONNECTED: OFFLINEFILES_EVENTS = 16i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_ITEMAVAILABLEOFFLINE: OFFLINEFILES_EVENTS = 17i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_ITEMNOTAVAILABLEOFFLINE: OFFLINEFILES_EVENTS = 18i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_ITEMPINNED: OFFLINEFILES_EVENTS = 19i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_ITEMNOTPINNED: OFFLINEFILES_EVENTS = 20i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_ITEMMODIFIED: OFFLINEFILES_EVENTS = 21i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_ITEMADDEDTOCACHE: OFFLINEFILES_EVENTS = 22i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_ITEMDELETEDFROMCACHE: OFFLINEFILES_EVENTS = 23i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_ITEMRENAMED: OFFLINEFILES_EVENTS = 24i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_DATALOST: OFFLINEFILES_EVENTS = 25i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_PING: OFFLINEFILES_EVENTS = 26i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_ITEMRECONNECTBEGIN: OFFLINEFILES_EVENTS = 27i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_ITEMRECONNECTEND: OFFLINEFILES_EVENTS = 28i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_CACHEEVICTBEGIN: OFFLINEFILES_EVENTS = 29i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_CACHEEVICTEND: OFFLINEFILES_EVENTS = 30i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_POLICYCHANGEDETECTED: OFFLINEFILES_EVENTS = 31i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_PREFERENCECHANGEDETECTED: OFFLINEFILES_EVENTS = 32i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_SETTINGSCHANGESAPPLIED: OFFLINEFILES_EVENTS = 33i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_TRANSPARENTCACHEITEMNOTIFY: OFFLINEFILES_EVENTS = 34i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_PREFETCHFILEBEGIN: OFFLINEFILES_EVENTS = 35i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_PREFETCHFILEEND: OFFLINEFILES_EVENTS = 36i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_PREFETCHCLOSEHANDLEBEGIN: OFFLINEFILES_EVENTS = 37i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_EVENT_PREFETCHCLOSEHANDLEEND: OFFLINEFILES_EVENTS = 38i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_NUM_EVENTS: OFFLINEFILES_EVENTS = 39i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub type OFFLINEFILES_ITEM_COPY = i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_COPY_LOCAL: OFFLINEFILES_ITEM_COPY = 0i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_COPY_REMOTE: OFFLINEFILES_ITEM_COPY = 1i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_COPY_ORIGINAL: OFFLINEFILES_ITEM_COPY = 2i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_CREATED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_DELETED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_DIRECTORY: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_DIRTY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_FILE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_GHOST: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_GUEST_ANYACCESS: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_GUEST_READ: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_GUEST_WRITE: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_MODIFIED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_MODIFIED_ATTRIBUTES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_MODIFIED_DATA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_OFFLINE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_ONLINE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_OTHER_ANYACCESS: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_OTHER_READ: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_OTHER_WRITE: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_PINNED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_PINNED_COMPUTER: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_PINNED_OTHERS: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_PINNED_USER: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_SPARSE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_SUSPENDED: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_USER_ANYACCESS: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_USER_READ: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_FILTER_FLAG_USER_WRITE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_QUERY_ADMIN: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_QUERY_ATTEMPT_TRANSITIONONLINE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_QUERY_CONNECTIONSTATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_QUERY_INCLUDETRANSPARENTCACHE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_QUERY_LOCALDIRTYBYTECOUNT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_QUERY_REMOTEDIRTYBYTECOUNT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_QUERY_REMOTEINFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub type OFFLINEFILES_ITEM_TIME = i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_TIME_CREATION: OFFLINEFILES_ITEM_TIME = 0i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_TIME_LASTACCESS: OFFLINEFILES_ITEM_TIME = 1i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_TIME_LASTWRITE: OFFLINEFILES_ITEM_TIME = 2i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub type OFFLINEFILES_ITEM_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_TYPE_FILE: OFFLINEFILES_ITEM_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_TYPE_DIRECTORY: OFFLINEFILES_ITEM_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_TYPE_SHARE: OFFLINEFILES_ITEM_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_ITEM_TYPE_SERVER: OFFLINEFILES_ITEM_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub type OFFLINEFILES_OFFLINE_REASON = i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_OFFLINE_REASON_UNKNOWN: OFFLINEFILES_OFFLINE_REASON = 0i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_OFFLINE_REASON_NOT_APPLICABLE: OFFLINEFILES_OFFLINE_REASON = 1i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_OFFLINE_REASON_CONNECTION_FORCED: OFFLINEFILES_OFFLINE_REASON = 2i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_OFFLINE_REASON_CONNECTION_SLOW: OFFLINEFILES_OFFLINE_REASON = 3i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_OFFLINE_REASON_CONNECTION_ERROR: OFFLINEFILES_OFFLINE_REASON = 4i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_OFFLINE_REASON_ITEM_VERSION_CONFLICT: OFFLINEFILES_OFFLINE_REASON = 5i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_OFFLINE_REASON_ITEM_SUSPENDED: OFFLINEFILES_OFFLINE_REASON = 6i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub type OFFLINEFILES_OP_RESPONSE = i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_OP_CONTINUE: OFFLINEFILES_OP_RESPONSE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_OP_RETRY: OFFLINEFILES_OP_RESPONSE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_OP_ABORT: OFFLINEFILES_OP_RESPONSE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub type OFFLINEFILES_PATHFILTER_MATCH = i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_PATHFILTER_SELF: OFFLINEFILES_PATHFILTER_MATCH = 0i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_PATHFILTER_CHILD: OFFLINEFILES_PATHFILTER_MATCH = 1i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_PATHFILTER_DESCENDENT: OFFLINEFILES_PATHFILTER_MATCH = 2i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_PATHFILTER_SELFORCHILD: OFFLINEFILES_PATHFILTER_MATCH = 3i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_PATHFILTER_SELFORDESCENDENT: OFFLINEFILES_PATHFILTER_MATCH = 4i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_PINLINKTARGETS_ALWAYS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_PINLINKTARGETS_EXPLICIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_PINLINKTARGETS_NEVER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_PIN_CONTROL_FLAG_ASYNCPROGRESS: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_PIN_CONTROL_FLAG_BACKGROUND: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_PIN_CONTROL_FLAG_CONSOLE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_PIN_CONTROL_FLAG_FILL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_PIN_CONTROL_FLAG_FORALL: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_PIN_CONTROL_FLAG_FORREDIR: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_PIN_CONTROL_FLAG_FORUSER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_PIN_CONTROL_FLAG_FORUSER_POLICY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_PIN_CONTROL_FLAG_INTERACTIVE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_PIN_CONTROL_FLAG_LOWPRIORITY: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_PIN_CONTROL_FLAG_PINLINKTARGETS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SETTING_PinLinkTargets: &str = "LinkTargetCaching";
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SETTING_SCOPE_COMPUTER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SETTING_SCOPE_USER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub type OFFLINEFILES_SETTING_VALUE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SETTING_VALUE_UI4: OFFLINEFILES_SETTING_VALUE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SETTING_VALUE_BSTR: OFFLINEFILES_SETTING_VALUE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SETTING_VALUE_BSTR_DBLNULTERM: OFFLINEFILES_SETTING_VALUE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SETTING_VALUE_2DIM_ARRAY_BSTR_UI4: OFFLINEFILES_SETTING_VALUE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SETTING_VALUE_2DIM_ARRAY_BSTR_BSTR: OFFLINEFILES_SETTING_VALUE_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub type OFFLINEFILES_SYNC_CONFLICT_RESOLVE = i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_NONE: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_KEEPLOCAL: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_KEEPREMOTE: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_KEEPALLCHANGES: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_KEEPLATEST: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_LOG: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_SKIP: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = 6i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_CONFLICT_ABORT: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = 7i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_NUMCODES: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = 8i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_CONTROL_CR_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_CONTROL_CR_KEEPLATEST: u32 = 805306368u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_CONTROL_CR_KEEPLOCAL: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_CONTROL_CR_KEEPREMOTE: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_CONTROL_CR_MASK: u32 = 4026531840u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_ASYNCPROGRESS: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_BACKGROUND: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_CONSOLE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_FILLSPARSE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_INTERACTIVE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_LOWPRIORITY: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_NONEWFILESOUT: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_PINFORALL: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_PINFORREDIR: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_PINFORUSER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_PINFORUSER_POLICY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_PINLINKTARGETS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_PINNEWFILES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_SKIPSUSPENDEDDIRS: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_SYNCIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_SYNCOUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_ITEM_CHANGE_ATTRIBUTES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_ITEM_CHANGE_CHANGETIME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_ITEM_CHANGE_FILESIZE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_ITEM_CHANGE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_ITEM_CHANGE_WRITETIME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub type OFFLINEFILES_SYNC_OPERATION = i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_OPERATION_CREATE_COPY_ON_SERVER: OFFLINEFILES_SYNC_OPERATION = 0i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_OPERATION_CREATE_COPY_ON_CLIENT: OFFLINEFILES_SYNC_OPERATION = 1i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_OPERATION_SYNC_TO_SERVER: OFFLINEFILES_SYNC_OPERATION = 2i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_OPERATION_SYNC_TO_CLIENT: OFFLINEFILES_SYNC_OPERATION = 3i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_OPERATION_DELETE_SERVER_COPY: OFFLINEFILES_SYNC_OPERATION = 4i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_OPERATION_DELETE_CLIENT_COPY: OFFLINEFILES_SYNC_OPERATION = 5i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_OPERATION_PIN: OFFLINEFILES_SYNC_OPERATION = 6i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_OPERATION_PREPARE: OFFLINEFILES_SYNC_OPERATION = 7i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub type OFFLINEFILES_SYNC_STATE = i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_Stable: OFFLINEFILES_SYNC_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_FileOnClient_DirOnServer: OFFLINEFILES_SYNC_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_FileOnClient_NoServerCopy: OFFLINEFILES_SYNC_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_DirOnClient_FileOnServer: OFFLINEFILES_SYNC_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_DirOnClient_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_DirOnClient_NoServerCopy: OFFLINEFILES_SYNC_STATE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_FileCreatedOnClient_NoServerCopy: OFFLINEFILES_SYNC_STATE = 6i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_FileCreatedOnClient_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = 7i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_FileCreatedOnClient_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = 8i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_FileCreatedOnClient_FileOnServer: OFFLINEFILES_SYNC_STATE = 9i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_FileCreatedOnClient_DirOnServer: OFFLINEFILES_SYNC_STATE = 10i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_FileCreatedOnClient_DeletedOnServer: OFFLINEFILES_SYNC_STATE = 11i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_FileChangedOnClient_ChangedOnServer: OFFLINEFILES_SYNC_STATE = 12i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_FileChangedOnClient_DirOnServer: OFFLINEFILES_SYNC_STATE = 13i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_FileChangedOnClient_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = 14i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_FileChangedOnClient_DeletedOnServer: OFFLINEFILES_SYNC_STATE = 15i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_FileSparseOnClient_ChangedOnServer: OFFLINEFILES_SYNC_STATE = 16i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_FileSparseOnClient_DeletedOnServer: OFFLINEFILES_SYNC_STATE = 17i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_FileSparseOnClient_DirOnServer: OFFLINEFILES_SYNC_STATE = 18i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_FileSparseOnClient_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = 19i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_DirCreatedOnClient_NoServerCopy: OFFLINEFILES_SYNC_STATE = 20i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_DirCreatedOnClient_DirOnServer: OFFLINEFILES_SYNC_STATE = 21i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_DirCreatedOnClient_FileOnServer: OFFLINEFILES_SYNC_STATE = 22i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_DirCreatedOnClient_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = 23i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_DirCreatedOnClient_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = 24i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_DirCreatedOnClient_DeletedOnServer: OFFLINEFILES_SYNC_STATE = 25i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_DirChangedOnClient_FileOnServer: OFFLINEFILES_SYNC_STATE = 26i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_DirChangedOnClient_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = 27i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_DirChangedOnClient_ChangedOnServer: OFFLINEFILES_SYNC_STATE = 28i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_DirChangedOnClient_DeletedOnServer: OFFLINEFILES_SYNC_STATE = 29i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_NoClientCopy_FileOnServer: OFFLINEFILES_SYNC_STATE = 30i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_NoClientCopy_DirOnServer: OFFLINEFILES_SYNC_STATE = 31i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_NoClientCopy_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = 32i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_NoClientCopy_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = 33i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_DeletedOnClient_FileOnServer: OFFLINEFILES_SYNC_STATE = 34i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_DeletedOnClient_DirOnServer: OFFLINEFILES_SYNC_STATE = 35i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_DeletedOnClient_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = 36i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_DeletedOnClient_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = 37i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_FileSparseOnClient: OFFLINEFILES_SYNC_STATE = 38i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_FileChangedOnClient: OFFLINEFILES_SYNC_STATE = 39i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_FileRenamedOnClient: OFFLINEFILES_SYNC_STATE = 40i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_DirSparseOnClient: OFFLINEFILES_SYNC_STATE = 41i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_DirChangedOnClient: OFFLINEFILES_SYNC_STATE = 42i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_DirRenamedOnClient: OFFLINEFILES_SYNC_STATE = 43i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = 44i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_FileRenamedOnServer: OFFLINEFILES_SYNC_STATE = 45i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_FileDeletedOnServer: OFFLINEFILES_SYNC_STATE = 46i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = 47i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_DirRenamedOnServer: OFFLINEFILES_SYNC_STATE = 48i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_DirDeletedOnServer: OFFLINEFILES_SYNC_STATE = 49i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_FileReplacedAndDeletedOnClient_FileOnServer: OFFLINEFILES_SYNC_STATE = 50i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_FileReplacedAndDeletedOnClient_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = 51i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_FileReplacedAndDeletedOnClient_DirOnServer: OFFLINEFILES_SYNC_STATE = 52i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_FileReplacedAndDeletedOnClient_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = 53i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_NUMSTATES: OFFLINEFILES_SYNC_STATE = 54i32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_LOCAL_KNOWN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_SYNC_STATE_REMOTE_KNOWN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_TRANSITION_FLAG_CONSOLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_OfflineFiles\"`*"]
pub const OFFLINEFILES_TRANSITION_FLAG_INTERACTIVE: u32 = 1u32;
pub const OfflineFilesCache: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1220984444, data2: 14449, data3: 17356, data4: [180, 111, 20, 73, 161, 187, 47, 243] };
pub const OfflineFilesSetting: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4248197609, data2: 43296, data3: 16675, data4: [173, 100, 127, 199, 108, 122, 172, 223] };
