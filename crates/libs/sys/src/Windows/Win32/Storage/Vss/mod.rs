#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
    pub fn CreateVssExpressWriterInternal(ppwriter: *mut *mut *mut IVssExpressWriter) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
pub struct IVssAdmin {
    pub base__: ::windows_sys::core::IUnknown,
    pub RegisterProvider: unsafe extern "system" fn(this: *mut *mut Self, pproviderid: ::windows_sys::core::GUID, classid: ::windows_sys::core::GUID, pwszprovidername: *const u16, eprovidertype: VSS_PROVIDER_TYPE, pwszproviderversion: *const u16, providerversionid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub UnregisterProvider: unsafe extern "system" fn(this: *mut *mut Self, providerid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub QueryProviders: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AbortAllSnapshotsInProgress: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVssAdminEx {
    pub base__: IVssAdmin,
    pub GetProviderCapability: unsafe extern "system" fn(this: *mut *mut Self, pproviderid: ::windows_sys::core::GUID, plloriginalcapabilitymask: *mut u64) -> ::windows_sys::core::HRESULT,
    pub GetProviderContext: unsafe extern "system" fn(this: *mut *mut Self, providerid: ::windows_sys::core::GUID, plcontext: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetProviderContext: unsafe extern "system" fn(this: *mut *mut Self, providerid: ::windows_sys::core::GUID, lcontext: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVssAsync {
    pub base__: ::windows_sys::core::IUnknown,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Wait: unsafe extern "system" fn(this: *mut *mut Self, dwmilliseconds: u32) -> ::windows_sys::core::HRESULT,
    pub QueryStatus: unsafe extern "system" fn(this: *mut *mut Self, phrresult: *mut ::windows_sys::core::HRESULT, preserved: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVssComponent {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLogicalPath: unsafe extern "system" fn(this: *mut *mut Self, pbstrpath: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLogicalPath: usize,
    pub GetComponentType: unsafe extern "system" fn(this: *mut *mut Self, pct: *mut VSS_COMPONENT_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetComponentName: unsafe extern "system" fn(this: *mut *mut Self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetComponentName: usize,
    pub GetBackupSucceeded: unsafe extern "system" fn(this: *mut *mut Self, pbsucceeded: *mut bool) -> ::windows_sys::core::HRESULT,
    pub GetAlternateLocationMappingCount: unsafe extern "system" fn(this: *mut *mut Self, pcmappings: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetAlternateLocationMapping: unsafe extern "system" fn(this: *mut *mut Self, imapping: u32, ppfiledesc: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBackupMetadata: unsafe extern "system" fn(this: *mut *mut Self, wszdata: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBackupMetadata: unsafe extern "system" fn(this: *mut *mut Self, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBackupMetadata: usize,
    pub AddPartialFile: unsafe extern "system" fn(this: *mut *mut Self, wszpath: ::windows_sys::core::PCWSTR, wszfilename: ::windows_sys::core::PCWSTR, wszranges: ::windows_sys::core::PCWSTR, wszmetadata: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetPartialFileCount: unsafe extern "system" fn(this: *mut *mut Self, pcpartialfiles: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPartialFile: unsafe extern "system" fn(this: *mut *mut Self, ipartialfile: u32, pbstrpath: *mut super::super::Foundation::BSTR, pbstrfilename: *mut super::super::Foundation::BSTR, pbstrrange: *mut super::super::Foundation::BSTR, pbstrmetadata: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPartialFile: usize,
    pub IsSelectedForRestore: unsafe extern "system" fn(this: *mut *mut Self, pbselectedforrestore: *mut bool) -> ::windows_sys::core::HRESULT,
    pub GetAdditionalRestores: unsafe extern "system" fn(this: *mut *mut Self, pbadditionalrestores: *mut bool) -> ::windows_sys::core::HRESULT,
    pub GetNewTargetCount: unsafe extern "system" fn(this: *mut *mut Self, pcnewtarget: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetNewTarget: unsafe extern "system" fn(this: *mut *mut Self, inewtarget: u32, ppfiledesc: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddDirectedTarget: unsafe extern "system" fn(this: *mut *mut Self, wszsourcepath: ::windows_sys::core::PCWSTR, wszsourcefilename: ::windows_sys::core::PCWSTR, wszsourcerangelist: ::windows_sys::core::PCWSTR, wszdestinationpath: ::windows_sys::core::PCWSTR, wszdestinationfilename: ::windows_sys::core::PCWSTR, wszdestinationrangelist: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetDirectedTargetCount: unsafe extern "system" fn(this: *mut *mut Self, pcdirectedtarget: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDirectedTarget: unsafe extern "system" fn(this: *mut *mut Self, idirectedtarget: u32, pbstrsourcepath: *mut super::super::Foundation::BSTR, pbstrsourcefilename: *mut super::super::Foundation::BSTR, pbstrsourcerangelist: *mut super::super::Foundation::BSTR, pbstrdestinationpath: *mut super::super::Foundation::BSTR, pbstrdestinationfilename: *mut super::super::Foundation::BSTR, pbstrdestinationrangelist: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDirectedTarget: usize,
    pub SetRestoreMetadata: unsafe extern "system" fn(this: *mut *mut Self, wszrestoremetadata: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRestoreMetadata: unsafe extern "system" fn(this: *mut *mut Self, pbstrrestoremetadata: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRestoreMetadata: usize,
    pub SetRestoreTarget: unsafe extern "system" fn(this: *mut *mut Self, target: VSS_RESTORE_TARGET) -> ::windows_sys::core::HRESULT,
    pub GetRestoreTarget: unsafe extern "system" fn(this: *mut *mut Self, ptarget: *mut VSS_RESTORE_TARGET) -> ::windows_sys::core::HRESULT,
    pub SetPreRestoreFailureMsg: unsafe extern "system" fn(this: *mut *mut Self, wszprerestorefailuremsg: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPreRestoreFailureMsg: unsafe extern "system" fn(this: *mut *mut Self, pbstrprerestorefailuremsg: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPreRestoreFailureMsg: usize,
    pub SetPostRestoreFailureMsg: unsafe extern "system" fn(this: *mut *mut Self, wszpostrestorefailuremsg: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPostRestoreFailureMsg: unsafe extern "system" fn(this: *mut *mut Self, pbstrpostrestorefailuremsg: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPostRestoreFailureMsg: usize,
    pub SetBackupStamp: unsafe extern "system" fn(this: *mut *mut Self, wszbackupstamp: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBackupStamp: unsafe extern "system" fn(this: *mut *mut Self, pbstrbackupstamp: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBackupStamp: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPreviousBackupStamp: unsafe extern "system" fn(this: *mut *mut Self, pbstrbackupstamp: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPreviousBackupStamp: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBackupOptions: unsafe extern "system" fn(this: *mut *mut Self, pbstrbackupoptions: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBackupOptions: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRestoreOptions: unsafe extern "system" fn(this: *mut *mut Self, pbstrrestoreoptions: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRestoreOptions: usize,
    pub GetRestoreSubcomponentCount: unsafe extern "system" fn(this: *mut *mut Self, pcrestoresubcomponent: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRestoreSubcomponent: unsafe extern "system" fn(this: *mut *mut Self, icomponent: u32, pbstrlogicalpath: *mut super::super::Foundation::BSTR, pbstrcomponentname: *mut super::super::Foundation::BSTR, pbrepair: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRestoreSubcomponent: usize,
    pub GetFileRestoreStatus: unsafe extern "system" fn(this: *mut *mut Self, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddDifferencedFilesByLastModifyTime: unsafe extern "system" fn(this: *mut *mut Self, wszpath: ::windows_sys::core::PCWSTR, wszfilespec: ::windows_sys::core::PCWSTR, brecursive: super::super::Foundation::BOOL, ftlastmodifytime: super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddDifferencedFilesByLastModifyTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AddDifferencedFilesByLastModifyLSN: unsafe extern "system" fn(this: *mut *mut Self, wszpath: ::windows_sys::core::PCWSTR, wszfilespec: ::windows_sys::core::PCWSTR, brecursive: super::super::Foundation::BOOL, bstrlsnstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddDifferencedFilesByLastModifyLSN: usize,
    pub GetDifferencedFilesCount: unsafe extern "system" fn(this: *mut *mut Self, pcdifferencedfiles: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDifferencedFile: unsafe extern "system" fn(this: *mut *mut Self, idifferencedfile: u32, pbstrpath: *mut super::super::Foundation::BSTR, pbstrfilespec: *mut super::super::Foundation::BSTR, pbrecursive: *mut super::super::Foundation::BOOL, pbstrlsnstring: *mut super::super::Foundation::BSTR, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDifferencedFile: usize,
}
#[repr(C)]
pub struct IVssComponentEx {
    pub base__: IVssComponent,
    pub SetPrepareForBackupFailureMsg: unsafe extern "system" fn(this: *mut *mut Self, wszfailuremsg: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SetPostSnapshotFailureMsg: unsafe extern "system" fn(this: *mut *mut Self, wszfailuremsg: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPrepareForBackupFailureMsg: unsafe extern "system" fn(this: *mut *mut Self, pbstrfailuremsg: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPrepareForBackupFailureMsg: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPostSnapshotFailureMsg: unsafe extern "system" fn(this: *mut *mut Self, pbstrfailuremsg: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPostSnapshotFailureMsg: usize,
    pub GetAuthoritativeRestore: unsafe extern "system" fn(this: *mut *mut Self, pbauth: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRollForward: unsafe extern "system" fn(this: *mut *mut Self, prolltype: *mut VSS_ROLLFORWARD_TYPE, pbstrpoint: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRollForward: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRestoreName: unsafe extern "system" fn(this: *mut *mut Self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRestoreName: usize,
}
#[repr(C)]
pub struct IVssComponentEx2 {
    pub base__: IVssComponentEx,
    pub SetFailure: unsafe extern "system" fn(this: *mut *mut Self, hr: ::windows_sys::core::HRESULT, hrapplication: ::windows_sys::core::HRESULT, wszapplicationmessage: ::windows_sys::core::PCWSTR, dwreserved: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFailure: unsafe extern "system" fn(this: *mut *mut Self, phr: *mut ::windows_sys::core::HRESULT, phrapplication: *mut ::windows_sys::core::HRESULT, pbstrapplicationmessage: *mut super::super::Foundation::BSTR, pdwreserved: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFailure: usize,
}
#[repr(C)]
pub struct IVssCreateExpressWriterMetadata {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddExcludeFiles: unsafe extern "system" fn(this: *mut *mut Self, wszpath: ::windows_sys::core::PCWSTR, wszfilespec: ::windows_sys::core::PCWSTR, brecursive: u8) -> ::windows_sys::core::HRESULT,
    pub AddComponent: unsafe extern "system" fn(this: *mut *mut Self, ct: VSS_COMPONENT_TYPE, wszlogicalpath: ::windows_sys::core::PCWSTR, wszcomponentname: ::windows_sys::core::PCWSTR, wszcaption: ::windows_sys::core::PCWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> ::windows_sys::core::HRESULT,
    pub AddFilesToFileGroup: unsafe extern "system" fn(this: *mut *mut Self, wszlogicalpath: ::windows_sys::core::PCWSTR, wszgroupname: ::windows_sys::core::PCWSTR, wszpath: ::windows_sys::core::PCWSTR, wszfilespec: ::windows_sys::core::PCWSTR, brecursive: u8, wszalternatelocation: ::windows_sys::core::PCWSTR, dwbackuptypemask: u32) -> ::windows_sys::core::HRESULT,
    pub SetRestoreMethod: unsafe extern "system" fn(this: *mut *mut Self, method: VSS_RESTOREMETHOD_ENUM, wszservice: ::windows_sys::core::PCWSTR, wszuserprocedure: ::windows_sys::core::PCWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> ::windows_sys::core::HRESULT,
    pub AddComponentDependency: unsafe extern "system" fn(this: *mut *mut Self, wszforlogicalpath: ::windows_sys::core::PCWSTR, wszforcomponentname: ::windows_sys::core::PCWSTR, onwriterid: ::windows_sys::core::GUID, wszonlogicalpath: ::windows_sys::core::PCWSTR, wszoncomponentname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SetBackupSchema: unsafe extern "system" fn(this: *mut *mut Self, dwschemamask: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SaveAsXML: unsafe extern "system" fn(this: *mut *mut Self, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SaveAsXML: usize,
}
#[repr(C)]
pub struct IVssCreateWriterMetadata {
    pub AddIncludeFiles: unsafe extern "system" fn(this: *mut *mut Self, wszpath: ::windows_sys::core::PCWSTR, wszfilespec: ::windows_sys::core::PCWSTR, brecursive: u8, wszalternatelocation: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub AddExcludeFiles: unsafe extern "system" fn(this: *mut *mut Self, wszpath: ::windows_sys::core::PCWSTR, wszfilespec: ::windows_sys::core::PCWSTR, brecursive: u8) -> ::windows_sys::core::HRESULT,
    pub AddComponent: unsafe extern "system" fn(this: *mut *mut Self, ct: VSS_COMPONENT_TYPE, wszlogicalpath: ::windows_sys::core::PCWSTR, wszcomponentname: ::windows_sys::core::PCWSTR, wszcaption: ::windows_sys::core::PCWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> ::windows_sys::core::HRESULT,
    pub AddDatabaseFiles: unsafe extern "system" fn(this: *mut *mut Self, wszlogicalpath: ::windows_sys::core::PCWSTR, wszdatabasename: ::windows_sys::core::PCWSTR, wszpath: ::windows_sys::core::PCWSTR, wszfilespec: ::windows_sys::core::PCWSTR, dwbackuptypemask: u32) -> ::windows_sys::core::HRESULT,
    pub AddDatabaseLogFiles: unsafe extern "system" fn(this: *mut *mut Self, wszlogicalpath: ::windows_sys::core::PCWSTR, wszdatabasename: ::windows_sys::core::PCWSTR, wszpath: ::windows_sys::core::PCWSTR, wszfilespec: ::windows_sys::core::PCWSTR, dwbackuptypemask: u32) -> ::windows_sys::core::HRESULT,
    pub AddFilesToFileGroup: unsafe extern "system" fn(this: *mut *mut Self, wszlogicalpath: ::windows_sys::core::PCWSTR, wszgroupname: ::windows_sys::core::PCWSTR, wszpath: ::windows_sys::core::PCWSTR, wszfilespec: ::windows_sys::core::PCWSTR, brecursive: u8, wszalternatelocation: ::windows_sys::core::PCWSTR, dwbackuptypemask: u32) -> ::windows_sys::core::HRESULT,
    pub SetRestoreMethod: unsafe extern "system" fn(this: *mut *mut Self, method: VSS_RESTOREMETHOD_ENUM, wszservice: ::windows_sys::core::PCWSTR, wszuserprocedure: ::windows_sys::core::PCWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> ::windows_sys::core::HRESULT,
    pub AddAlternateLocationMapping: unsafe extern "system" fn(this: *mut *mut Self, wszsourcepath: ::windows_sys::core::PCWSTR, wszsourcefilespec: ::windows_sys::core::PCWSTR, brecursive: u8, wszdestination: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub AddComponentDependency: unsafe extern "system" fn(this: *mut *mut Self, wszforlogicalpath: ::windows_sys::core::PCWSTR, wszforcomponentname: ::windows_sys::core::PCWSTR, onwriterid: ::windows_sys::core::GUID, wszonlogicalpath: ::windows_sys::core::PCWSTR, wszoncomponentname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SetBackupSchema: unsafe extern "system" fn(this: *mut *mut Self, dwschemamask: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub GetDocument: unsafe extern "system" fn(this: *mut *mut Self, pdoc: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com")))]
    GetDocument: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SaveAsXML: unsafe extern "system" fn(this: *mut *mut Self, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SaveAsXML: usize,
}
#[repr(C)]
pub struct IVssDifferentialSoftwareSnapshotMgmt {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddDiffArea: unsafe extern "system" fn(this: *mut *mut Self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows_sys::core::HRESULT,
    pub ChangeDiffAreaMaximumSize: unsafe extern "system" fn(this: *mut *mut Self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows_sys::core::HRESULT,
    pub QueryVolumesSupportedForDiffAreas: unsafe extern "system" fn(this: *mut *mut Self, pwszoriginalvolumename: *const u16, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub QueryDiffAreasForVolume: unsafe extern "system" fn(this: *mut *mut Self, pwszvolumename: *const u16, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub QueryDiffAreasOnVolume: unsafe extern "system" fn(this: *mut *mut Self, pwszvolumename: *const u16, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub QueryDiffAreasForSnapshot: unsafe extern "system" fn(this: *mut *mut Self, snapshotid: ::windows_sys::core::GUID, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVssDifferentialSoftwareSnapshotMgmt2 {
    pub base__: IVssDifferentialSoftwareSnapshotMgmt,
    #[cfg(feature = "Win32_Foundation")]
    pub ChangeDiffAreaMaximumSizeEx: unsafe extern "system" fn(this: *mut *mut Self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64, bvolatile: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ChangeDiffAreaMaximumSizeEx: usize,
    pub MigrateDiffAreas: unsafe extern "system" fn(this: *mut *mut Self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, pwsznewdiffareavolumename: *const u16) -> ::windows_sys::core::HRESULT,
    pub QueryMigrationStatus: unsafe extern "system" fn(this: *mut *mut Self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSnapshotPriority: unsafe extern "system" fn(this: *mut *mut Self, idsnapshot: ::windows_sys::core::GUID, priority: u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVssDifferentialSoftwareSnapshotMgmt3 {
    pub base__: IVssDifferentialSoftwareSnapshotMgmt2,
    pub SetVolumeProtectLevel: unsafe extern "system" fn(this: *mut *mut Self, pwszvolumename: *const u16, protectionlevel: VSS_PROTECTION_LEVEL) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetVolumeProtectLevel: unsafe extern "system" fn(this: *mut *mut Self, pwszvolumename: *const u16, protectionlevel: *mut VSS_VOLUME_PROTECTION_INFO) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetVolumeProtectLevel: usize,
    pub ClearVolumeProtectFault: unsafe extern "system" fn(this: *mut *mut Self, pwszvolumename: *const u16) -> ::windows_sys::core::HRESULT,
    pub DeleteUnusedDiffAreas: unsafe extern "system" fn(this: *mut *mut Self, pwszdiffareavolumename: *const u16) -> ::windows_sys::core::HRESULT,
    pub QuerySnapshotDeltaBitmap: unsafe extern "system" fn(this: *mut *mut Self, idsnapshotolder: ::windows_sys::core::GUID, idsnapshotyounger: ::windows_sys::core::GUID, pcblocksizeperbit: *mut u32, pcbitmaplength: *mut u32, ppbbitmap: *mut *mut u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVssEnumMgmtObject {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, rgelt: *mut VSS_MGMT_OBJECT_PROP, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVssEnumObject {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, rgelt: *mut VSS_OBJECT_PROP, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVssExamineWriterMetadata(pub u8);
#[repr(C)]
pub struct IVssExpressWriter {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateMetadata: unsafe extern "system" fn(this: *mut *mut Self, writerid: ::windows_sys::core::GUID, writername: ::windows_sys::core::PCWSTR, usagetype: VSS_USAGE_TYPE, versionmajor: u32, versionminor: u32, reserved: u32, ppmetadata: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LoadMetadata: unsafe extern "system" fn(this: *mut *mut Self, metadata: ::windows_sys::core::PCWSTR, reserved: u32) -> ::windows_sys::core::HRESULT,
    pub Register: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Unregister: unsafe extern "system" fn(this: *mut *mut Self, writerid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVssFileShareSnapshotProvider {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetContext: unsafe extern "system" fn(this: *mut *mut Self, lcontext: i32) -> ::windows_sys::core::HRESULT,
    pub GetSnapshotProperties: unsafe extern "system" fn(this: *mut *mut Self, snapshotid: ::windows_sys::core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> ::windows_sys::core::HRESULT,
    pub Query: unsafe extern "system" fn(this: *mut *mut Self, queriedobjectid: ::windows_sys::core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteSnapshots: unsafe extern "system" fn(this: *mut *mut Self, sourceobjectid: ::windows_sys::core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: super::super::Foundation::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteSnapshots: usize,
    pub BeginPrepareSnapshot: unsafe extern "system" fn(this: *mut *mut Self, snapshotsetid: ::windows_sys::core::GUID, snapshotid: ::windows_sys::core::GUID, pwszsharepath: *const u16, lnewcontext: i32, providerid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPathSupported: unsafe extern "system" fn(this: *mut *mut Self, pwszsharepath: *const u16, pbsupportedbythisprovider: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPathSupported: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPathSnapshotted: unsafe extern "system" fn(this: *mut *mut Self, pwszsharepath: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPathSnapshotted: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSnapshotProperty: unsafe extern "system" fn(this: *mut *mut Self, snapshotid: ::windows_sys::core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSnapshotProperty: usize,
}
#[repr(C)]
pub struct IVssHardwareSnapshotProvider {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub AreLunsSupported: unsafe extern "system" fn(this: *mut *mut Self, lluncount: i32, lcontext: i32, rgwszdevices: *const *const u16, pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))]
    AreLunsSupported: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub FillInLunInfo: unsafe extern "system" fn(this: *mut *mut Self, wszdevicename: *const u16, pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))]
    FillInLunInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub BeginPrepareSnapshot: unsafe extern "system" fn(this: *mut *mut Self, snapshotsetid: ::windows_sys::core::GUID, snapshotid: ::windows_sys::core::GUID, lcontext: i32, lluncount: i32, rgdevicenames: *const *const u16, rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))]
    BeginPrepareSnapshot: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub GetTargetLuns: unsafe extern "system" fn(this: *mut *mut Self, lluncount: i32, rgdevicenames: *const *const u16, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))]
    GetTargetLuns: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub LocateLuns: unsafe extern "system" fn(this: *mut *mut Self, lluncount: i32, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))]
    LocateLuns: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub OnLunEmpty: unsafe extern "system" fn(this: *mut *mut Self, wszdevicename: *const u16, pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))]
    OnLunEmpty: usize,
}
#[repr(C)]
pub struct IVssHardwareSnapshotProviderEx {
    pub base__: IVssHardwareSnapshotProvider,
    pub GetProviderCapabilities: unsafe extern "system" fn(this: *mut *mut Self, plloriginalcapabilitymask: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub OnLunStateChange: unsafe extern "system" fn(this: *mut *mut Self, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, dwflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))]
    OnLunStateChange: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub ResyncLuns: unsafe extern "system" fn(this: *mut *mut Self, psourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, ptargetluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))]
    ResyncLuns: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub OnReuseLuns: unsafe extern "system" fn(this: *mut *mut Self, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))]
    OnReuseLuns: usize,
}
#[repr(C)]
pub struct IVssProviderCreateSnapshotSet {
    pub base__: ::windows_sys::core::IUnknown,
    pub EndPrepareSnapshots: unsafe extern "system" fn(this: *mut *mut Self, snapshotsetid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub PreCommitSnapshots: unsafe extern "system" fn(this: *mut *mut Self, snapshotsetid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub CommitSnapshots: unsafe extern "system" fn(this: *mut *mut Self, snapshotsetid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub PostCommitSnapshots: unsafe extern "system" fn(this: *mut *mut Self, snapshotsetid: ::windows_sys::core::GUID, lsnapshotscount: i32) -> ::windows_sys::core::HRESULT,
    pub PreFinalCommitSnapshots: unsafe extern "system" fn(this: *mut *mut Self, snapshotsetid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub PostFinalCommitSnapshots: unsafe extern "system" fn(this: *mut *mut Self, snapshotsetid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub AbortSnapshots: unsafe extern "system" fn(this: *mut *mut Self, snapshotsetid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVssProviderNotifications {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnLoad: unsafe extern "system" fn(this: *mut *mut Self, pcallback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnUnload: unsafe extern "system" fn(this: *mut *mut Self, bforceunload: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnUnload: usize,
}
#[repr(C)]
pub struct IVssSnapshotMgmt {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetProviderMgmtInterface: unsafe extern "system" fn(this: *mut *mut Self, providerid: ::windows_sys::core::GUID, interfaceid: *const ::windows_sys::core::GUID, ppitf: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub QueryVolumesSupportedForSnapshots: unsafe extern "system" fn(this: *mut *mut Self, providerid: ::windows_sys::core::GUID, lcontext: i32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub QuerySnapshotsByVolume: unsafe extern "system" fn(this: *mut *mut Self, pwszvolumename: *const u16, providerid: ::windows_sys::core::GUID, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVssSnapshotMgmt2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetMinDiffAreaSize: unsafe extern "system" fn(this: *mut *mut Self, pllmindiffareasize: *mut i64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVssSoftwareSnapshotProvider {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetContext: unsafe extern "system" fn(this: *mut *mut Self, lcontext: i32) -> ::windows_sys::core::HRESULT,
    pub GetSnapshotProperties: unsafe extern "system" fn(this: *mut *mut Self, snapshotid: ::windows_sys::core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> ::windows_sys::core::HRESULT,
    pub Query: unsafe extern "system" fn(this: *mut *mut Self, queriedobjectid: ::windows_sys::core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteSnapshots: unsafe extern "system" fn(this: *mut *mut Self, sourceobjectid: ::windows_sys::core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: super::super::Foundation::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteSnapshots: usize,
    pub BeginPrepareSnapshot: unsafe extern "system" fn(this: *mut *mut Self, snapshotsetid: ::windows_sys::core::GUID, snapshotid: ::windows_sys::core::GUID, pwszvolumename: *const u16, lnewcontext: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsVolumeSupported: unsafe extern "system" fn(this: *mut *mut Self, pwszvolumename: *const u16, pbsupportedbythisprovider: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsVolumeSupported: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsVolumeSnapshotted: unsafe extern "system" fn(this: *mut *mut Self, pwszvolumename: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsVolumeSnapshotted: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSnapshotProperty: unsafe extern "system" fn(this: *mut *mut Self, snapshotid: ::windows_sys::core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSnapshotProperty: usize,
    pub RevertToSnapshot: unsafe extern "system" fn(this: *mut *mut Self, snapshotid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub QueryRevertStatus: unsafe extern "system" fn(this: *mut *mut Self, pwszvolume: *const u16, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVssWMDependency {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetWriterId: unsafe extern "system" fn(this: *mut *mut Self, pwriterid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLogicalPath: unsafe extern "system" fn(this: *mut *mut Self, pbstrlogicalpath: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLogicalPath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetComponentName: unsafe extern "system" fn(this: *mut *mut Self, pbstrcomponentname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetComponentName: usize,
}
#[repr(C)]
pub struct IVssWMFiledesc {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPath: unsafe extern "system" fn(this: *mut *mut Self, pbstrpath: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFilespec: unsafe extern "system" fn(this: *mut *mut Self, pbstrfilespec: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFilespec: usize,
    pub GetRecursive: unsafe extern "system" fn(this: *mut *mut Self, pbrecursive: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAlternateLocation: unsafe extern "system" fn(this: *mut *mut Self, pbstralternatelocation: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAlternateLocation: usize,
    pub GetBackupTypeMask: unsafe extern "system" fn(this: *mut *mut Self, pdwtypemask: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVssWriterComponents {
    pub GetComponentCount: unsafe extern "system" fn(this: *mut *mut Self, pccomponents: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetWriterInfo: unsafe extern "system" fn(this: *mut *mut Self, pidinstance: *mut ::windows_sys::core::GUID, pidwriter: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetComponent: unsafe extern "system" fn(this: *mut *mut Self, icomponent: u32, ppcomponent: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVssWriterImpl {
    pub base__: ::windows_sys::core::IUnknown,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, writerid: ::windows_sys::core::GUID, wszwritername: ::windows_sys::core::PCWSTR, wszwriterinstancename: ::windows_sys::core::PCWSTR, dwmajorversion: u32, dwminorversion: u32, ut: VSS_USAGE_TYPE, st: VSS_SOURCE_TYPE, nlevel: VSS_APPLICATION_LEVEL, dwtimeout: u32, aws: VSS_ALTERNATE_WRITER_STATE, biothrottlingonly: u8) -> ::windows_sys::core::HRESULT,
    pub Subscribe: unsafe extern "system" fn(this: *mut *mut Self, dwsubscribetimeout: u32, dweventflags: u32) -> ::windows_sys::core::HRESULT,
    pub Unsubscribe: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Uninitialize: unsafe extern "system" fn(this: *mut *mut Self),
    pub GetCurrentVolumeArray: unsafe extern "system" fn(this: *mut *mut Self) -> *mut ::windows_sys::core::PWSTR,
    pub GetCurrentVolumeCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetSnapshotDeviceName: unsafe extern "system" fn(this: *mut *mut Self, wszoriginalvolume: ::windows_sys::core::PCWSTR, ppwszsnapshotdevice: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetCurrentSnapshotSetId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID),
    pub GetContext: unsafe extern "system" fn(this: *mut *mut Self) -> i32,
    pub GetCurrentLevel: unsafe extern "system" fn(this: *mut *mut Self) -> VSS_APPLICATION_LEVEL,
    pub IsPathAffected: unsafe extern "system" fn(this: *mut *mut Self, wszpath: ::windows_sys::core::PCWSTR) -> bool,
    pub IsBootableSystemStateBackedUp: unsafe extern "system" fn(this: *mut *mut Self) -> bool,
    pub AreComponentsSelected: unsafe extern "system" fn(this: *mut *mut Self) -> bool,
    pub GetBackupType: unsafe extern "system" fn(this: *mut *mut Self) -> VSS_BACKUP_TYPE,
    pub GetRestoreType: unsafe extern "system" fn(this: *mut *mut Self) -> VSS_RESTORE_TYPE,
    pub SetWriterFailure: unsafe extern "system" fn(this: *mut *mut Self, hr: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub IsPartialFileSupportEnabled: unsafe extern "system" fn(this: *mut *mut Self) -> bool,
    pub InstallAlternateWriter: unsafe extern "system" fn(this: *mut *mut Self, idwriter: ::windows_sys::core::GUID, clsid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetIdentityInformation: unsafe extern "system" fn(this: *mut *mut Self) -> *mut IVssExamineWriterMetadata,
    pub SetWriterFailureEx: unsafe extern "system" fn(this: *mut *mut Self, hr: ::windows_sys::core::HRESULT, hrapplication: ::windows_sys::core::HRESULT, wszapplicationmessage: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetSessionId: unsafe extern "system" fn(this: *mut *mut Self, idsession: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub IsWriterShuttingDown: unsafe extern "system" fn(this: *mut *mut Self) -> bool,
}
pub const VSSCoordinator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3849956191, data2: 7364, data3: 17588, data4: [190, 217, 222, 9, 145, 255, 6, 35] };
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_ALTERNATE_WRITER_STATE = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_AWS_UNDEFINED: VSS_ALTERNATE_WRITER_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_AWS_NO_ALTERNATE_WRITER: VSS_ALTERNATE_WRITER_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_AWS_ALTERNATE_WRITER_EXISTS: VSS_ALTERNATE_WRITER_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_AWS_THIS_IS_ALTERNATE_WRITER: VSS_ALTERNATE_WRITER_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_APPLICATION_LEVEL = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_APP_UNKNOWN: VSS_APPLICATION_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_APP_SYSTEM: VSS_APPLICATION_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_APP_BACK_END: VSS_APPLICATION_LEVEL = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_APP_FRONT_END: VSS_APPLICATION_LEVEL = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_APP_SYSTEM_RM: VSS_APPLICATION_LEVEL = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_APP_AUTO: VSS_APPLICATION_LEVEL = -1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ASSOC_NO_MAX_SPACE: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ASSOC_REMOVE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_BACKUP_SCHEMA = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_UNDEFINED: VSS_BACKUP_SCHEMA = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_DIFFERENTIAL: VSS_BACKUP_SCHEMA = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_INCREMENTAL: VSS_BACKUP_SCHEMA = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_EXCLUSIVE_INCREMENTAL_DIFFERENTIAL: VSS_BACKUP_SCHEMA = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_LOG: VSS_BACKUP_SCHEMA = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_COPY: VSS_BACKUP_SCHEMA = 16i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_TIMESTAMPED: VSS_BACKUP_SCHEMA = 32i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_LAST_MODIFY: VSS_BACKUP_SCHEMA = 64i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_LSN: VSS_BACKUP_SCHEMA = 128i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_WRITER_SUPPORTS_NEW_TARGET: VSS_BACKUP_SCHEMA = 256i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_WRITER_SUPPORTS_RESTORE_WITH_MOVE: VSS_BACKUP_SCHEMA = 512i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_INDEPENDENT_SYSTEM_STATE: VSS_BACKUP_SCHEMA = 1024i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_ROLLFORWARD_RESTORE: VSS_BACKUP_SCHEMA = 4096i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_RESTORE_RENAME: VSS_BACKUP_SCHEMA = 8192i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_AUTHORITATIVE_RESTORE: VSS_BACKUP_SCHEMA = 16384i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_WRITER_SUPPORTS_PARALLEL_RESTORES: VSS_BACKUP_SCHEMA = 32768i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_BACKUP_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BT_UNDEFINED: VSS_BACKUP_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BT_FULL: VSS_BACKUP_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BT_INCREMENTAL: VSS_BACKUP_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BT_DIFFERENTIAL: VSS_BACKUP_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BT_LOG: VSS_BACKUP_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BT_COPY: VSS_BACKUP_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BT_OTHER: VSS_BACKUP_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_COMPONENT_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CF_BACKUP_RECOVERY: VSS_COMPONENT_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CF_APP_ROLLBACK_RECOVERY: VSS_COMPONENT_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CF_NOT_SYSTEM_STATE: VSS_COMPONENT_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_COMPONENT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CT_UNDEFINED: VSS_COMPONENT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CT_DATABASE: VSS_COMPONENT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CT_FILEGROUP: VSS_COMPONENT_TYPE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub struct VSS_DIFF_AREA_PROP {
    pub m_pwszVolumeName: *mut u16,
    pub m_pwszDiffAreaVolumeName: *mut u16,
    pub m_llMaximumDiffSpace: i64,
    pub m_llAllocatedDiffSpace: i64,
    pub m_llUsedDiffSpace: i64,
}
impl ::core::marker::Copy for VSS_DIFF_AREA_PROP {}
impl ::core::clone::Clone for VSS_DIFF_AREA_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub struct VSS_DIFF_VOLUME_PROP {
    pub m_pwszVolumeName: *mut u16,
    pub m_pwszVolumeDisplayName: *mut u16,
    pub m_llVolumeFreeSpace: i64,
    pub m_llVolumeTotalSpace: i64,
}
impl ::core::marker::Copy for VSS_DIFF_VOLUME_PROP {}
impl ::core::clone::Clone for VSS_DIFF_VOLUME_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_CRITICAL_DISKS_TOO_SMALL: ::windows_sys::core::HRESULT = -2147212280i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_CRITICAL_DISK_CANNOT_BE_EXCLUDED: ::windows_sys::core::HRESULT = -2147212267i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_DATADISK_RDISK0: ::windows_sys::core::HRESULT = -2147212282i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_DISK_ASSIGNMENT_FAILED: ::windows_sys::core::HRESULT = -2147212287i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_DISK_RECREATION_FAILED: ::windows_sys::core::HRESULT = -2147212286i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_DYNAMIC_VHD_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2147212278i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_FIXED_PHYSICAL_DISK_AVAILABLE_AFTER_DISK_EXCLUSION: ::windows_sys::core::HRESULT = -2147212268i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_MISSING_DYNDISK: ::windows_sys::core::HRESULT = -2147212284i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_NO_ARCPATH: ::windows_sys::core::HRESULT = -2147212285i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_NO_PHYSICAL_DISK_AVAILABLE: ::windows_sys::core::HRESULT = -2147212269i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_RDISK0_TOOSMALL: ::windows_sys::core::HRESULT = -2147212281i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_RDISK_FOR_SYSTEM_DISK_NOT_FOUND: ::windows_sys::core::HRESULT = -2147212270i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_SHARED_CRIDISK: ::windows_sys::core::HRESULT = -2147212283i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_SYSTEM_PARTITION_HIDDEN: ::windows_sys::core::HRESULT = -2147212266i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_AUTORECOVERY_FAILED: ::windows_sys::core::HRESULT = -2147212293i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_BAD_STATE: ::windows_sys::core::HRESULT = -2147212543i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_BREAK_REVERT_ID_FAILED: ::windows_sys::core::HRESULT = -2147212298i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_CANNOT_REVERT_DISKID: ::windows_sys::core::HRESULT = -2147212290i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_CLUSTER_ERROR: ::windows_sys::core::HRESULT = -2147212288i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_CLUSTER_TIMEOUT: ::windows_sys::core::HRESULT = -2147212498i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_CORRUPT_XML_DOCUMENT: ::windows_sys::core::HRESULT = -2147212528i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_CRITICAL_VOLUME_ON_INVALID_DISK: ::windows_sys::core::HRESULT = -2147212271i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_DYNAMIC_DISK_ERROR: ::windows_sys::core::HRESULT = -2147212292i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_FLUSH_WRITES_TIMEOUT: ::windows_sys::core::HRESULT = -2147212525i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_FSS_TIMEOUT: ::windows_sys::core::HRESULT = -2147212265i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_HOLD_WRITES_TIMEOUT: ::windows_sys::core::HRESULT = -2147212524i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_INSUFFICIENT_STORAGE: ::windows_sys::core::HRESULT = -2147212513i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_INVALID_XML_DOCUMENT: ::windows_sys::core::HRESULT = -2147212527i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_LEGACY_PROVIDER: ::windows_sys::core::HRESULT = -2147212297i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_MAXIMUM_DIFFAREA_ASSOCIATIONS_REACHED: ::windows_sys::core::HRESULT = -2147212514i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_MAXIMUM_NUMBER_OF_REMOTE_MACHINES_REACHED: ::windows_sys::core::HRESULT = -2147212510i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_MAXIMUM_NUMBER_OF_SNAPSHOTS_REACHED: ::windows_sys::core::HRESULT = -2147212521i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_MAXIMUM_NUMBER_OF_VOLUMES_REACHED: ::windows_sys::core::HRESULT = -2147212526i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_MISSING_DISK: ::windows_sys::core::HRESULT = -2147212296i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_MISSING_HIDDEN_VOLUME: ::windows_sys::core::HRESULT = -2147212295i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_MISSING_VOLUME: ::windows_sys::core::HRESULT = -2147212294i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_NESTED_VOLUME_LIMIT: ::windows_sys::core::HRESULT = -2147212500i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_NONTRANSPORTABLE_BCD: ::windows_sys::core::HRESULT = -2147212291i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2147212497i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_NO_SNAPSHOTS_IMPORTED: ::windows_sys::core::HRESULT = -2147212512i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_OBJECT_ALREADY_EXISTS: ::windows_sys::core::HRESULT = -2147212531i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_OBJECT_NOT_FOUND: ::windows_sys::core::HRESULT = -2147212536i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_PROVIDER_ALREADY_REGISTERED: ::windows_sys::core::HRESULT = -2147212541i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_PROVIDER_IN_USE: ::windows_sys::core::HRESULT = -2147212537i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_PROVIDER_NOT_REGISTERED: ::windows_sys::core::HRESULT = -2147212540i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_PROVIDER_VETO: ::windows_sys::core::HRESULT = -2147212538i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_REBOOT_REQUIRED: ::windows_sys::core::HRESULT = -2147212505i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_REMOTE_SERVER_UNAVAILABLE: ::windows_sys::core::HRESULT = -2147212509i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_REMOTE_SERVER_UNSUPPORTED: ::windows_sys::core::HRESULT = -2147212508i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_RESYNC_IN_PROGRESS: ::windows_sys::core::HRESULT = -2147212289i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_REVERT_IN_PROGRESS: ::windows_sys::core::HRESULT = -2147212507i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_REVERT_VOLUME_LOST: ::windows_sys::core::HRESULT = -2147212506i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_SNAPSHOT_NOT_IN_SET: ::windows_sys::core::HRESULT = -2147212501i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_SNAPSHOT_SET_IN_PROGRESS: ::windows_sys::core::HRESULT = -2147212522i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_SOME_SNAPSHOTS_NOT_IMPORTED: ::windows_sys::core::HRESULT = -2147212511i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_TRANSACTION_FREEZE_TIMEOUT: ::windows_sys::core::HRESULT = -2147212504i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_TRANSACTION_THAW_TIMEOUT: ::windows_sys::core::HRESULT = -2147212503i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_UNEXPECTED: ::windows_sys::core::HRESULT = -2147212542i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_UNEXPECTED_PROVIDER_ERROR: ::windows_sys::core::HRESULT = -2147212529i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_UNEXPECTED_WRITER_ERROR: ::windows_sys::core::HRESULT = -2147212523i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_UNSELECTED_VOLUME: ::windows_sys::core::HRESULT = -2147212502i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_UNSUPPORTED_CONTEXT: ::windows_sys::core::HRESULT = -2147212517i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_VOLUME_IN_USE: ::windows_sys::core::HRESULT = -2147212515i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_VOLUME_NOT_LOCAL: ::windows_sys::core::HRESULT = -2147212499i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_VOLUME_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2147212532i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_VOLUME_NOT_SUPPORTED_BY_PROVIDER: ::windows_sys::core::HRESULT = -2147212530i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITERERROR_INCONSISTENTSNAPSHOT: ::windows_sys::core::HRESULT = -2147212304i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITERERROR_NONRETRYABLE: ::windows_sys::core::HRESULT = -2147212300i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITERERROR_OUTOFRESOURCES: ::windows_sys::core::HRESULT = -2147212303i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITERERROR_PARTIAL_FAILURE: ::windows_sys::core::HRESULT = -2147212490i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITERERROR_RECOVERY_FAILED: ::windows_sys::core::HRESULT = -2147212299i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITERERROR_RETRYABLE: ::windows_sys::core::HRESULT = -2147212301i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITERERROR_TIMEOUT: ::windows_sys::core::HRESULT = -2147212302i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITER_ALREADY_SUBSCRIBED: ::windows_sys::core::HRESULT = -2147212518i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITER_INFRASTRUCTURE: ::windows_sys::core::HRESULT = -2147212520i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITER_NOT_RESPONDING: ::windows_sys::core::HRESULT = -2147212519i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITER_STATUS_NOT_AVAILABLE: ::windows_sys::core::HRESULT = -2147212279i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_FILE_RESTORE_STATUS = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RS_UNDEFINED: VSS_FILE_RESTORE_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RS_NONE: VSS_FILE_RESTORE_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RS_ALL: VSS_FILE_RESTORE_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RS_FAILED: VSS_FILE_RESTORE_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_FILE_SPEC_BACKUP_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_FULL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_DIFFERENTIAL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_INCREMENTAL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_LOG_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_FULL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 256i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_DIFFERENTIAL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 512i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_INCREMENTAL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 1024i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_LOG_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 2048i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_CREATED_DURING_BACKUP: VSS_FILE_SPEC_BACKUP_TYPE = 65536i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_ALL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 15i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_ALL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 3840i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_HARDWARE_OPTIONS = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BREAKEX_FLAG_MASK_LUNS: VSS_HARDWARE_OPTIONS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BREAKEX_FLAG_MAKE_READ_WRITE: VSS_HARDWARE_OPTIONS = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BREAKEX_FLAG_REVERT_IDENTITY_ALL: VSS_HARDWARE_OPTIONS = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BREAKEX_FLAG_REVERT_IDENTITY_NONE: VSS_HARDWARE_OPTIONS = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ONLUNSTATECHANGE_NOTIFY_READ_WRITE: VSS_HARDWARE_OPTIONS = 256i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ONLUNSTATECHANGE_NOTIFY_LUN_PRE_RECOVERY: VSS_HARDWARE_OPTIONS = 512i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ONLUNSTATECHANGE_NOTIFY_LUN_POST_RECOVERY: VSS_HARDWARE_OPTIONS = 1024i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ONLUNSTATECHANGE_DO_MASK_LUNS: VSS_HARDWARE_OPTIONS = 2048i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub struct VSS_MGMT_OBJECT_PROP {
    pub Type: VSS_MGMT_OBJECT_TYPE,
    pub Obj: VSS_MGMT_OBJECT_UNION,
}
impl ::core::marker::Copy for VSS_MGMT_OBJECT_PROP {}
impl ::core::clone::Clone for VSS_MGMT_OBJECT_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_MGMT_OBJECT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_MGMT_OBJECT_UNKNOWN: VSS_MGMT_OBJECT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_MGMT_OBJECT_VOLUME: VSS_MGMT_OBJECT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_MGMT_OBJECT_DIFF_VOLUME: VSS_MGMT_OBJECT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_MGMT_OBJECT_DIFF_AREA: VSS_MGMT_OBJECT_TYPE = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub union VSS_MGMT_OBJECT_UNION {
    pub Vol: VSS_VOLUME_PROP,
    pub DiffVol: VSS_DIFF_VOLUME_PROP,
    pub DiffArea: VSS_DIFF_AREA_PROP,
}
impl ::core::marker::Copy for VSS_MGMT_OBJECT_UNION {}
impl ::core::clone::Clone for VSS_MGMT_OBJECT_UNION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub struct VSS_OBJECT_PROP {
    pub Type: VSS_OBJECT_TYPE,
    pub Obj: VSS_OBJECT_UNION,
}
impl ::core::marker::Copy for VSS_OBJECT_PROP {}
impl ::core::clone::Clone for VSS_OBJECT_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_OBJECT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_OBJECT_UNKNOWN: VSS_OBJECT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_OBJECT_NONE: VSS_OBJECT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_OBJECT_SNAPSHOT_SET: VSS_OBJECT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_OBJECT_SNAPSHOT: VSS_OBJECT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_OBJECT_PROVIDER: VSS_OBJECT_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_OBJECT_TYPE_COUNT: VSS_OBJECT_TYPE = 5i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub union VSS_OBJECT_UNION {
    pub Snap: VSS_SNAPSHOT_PROP,
    pub Prov: VSS_PROVIDER_PROP,
}
impl ::core::marker::Copy for VSS_OBJECT_UNION {}
impl ::core::clone::Clone for VSS_OBJECT_UNION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_PROTECTION_FAULT = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_NONE: VSS_PROTECTION_FAULT = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_DIFF_AREA_MISSING: VSS_PROTECTION_FAULT = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_IO_FAILURE_DURING_ONLINE: VSS_PROTECTION_FAULT = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_META_DATA_CORRUPTION: VSS_PROTECTION_FAULT = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_MEMORY_ALLOCATION_FAILURE: VSS_PROTECTION_FAULT = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_MAPPED_MEMORY_FAILURE: VSS_PROTECTION_FAULT = 5i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_COW_READ_FAILURE: VSS_PROTECTION_FAULT = 6i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_COW_WRITE_FAILURE: VSS_PROTECTION_FAULT = 7i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_DIFF_AREA_FULL: VSS_PROTECTION_FAULT = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_GROW_TOO_SLOW: VSS_PROTECTION_FAULT = 9i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_GROW_FAILED: VSS_PROTECTION_FAULT = 10i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_DESTROY_ALL_SNAPSHOTS: VSS_PROTECTION_FAULT = 11i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_FILE_SYSTEM_FAILURE: VSS_PROTECTION_FAULT = 12i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_IO_FAILURE: VSS_PROTECTION_FAULT = 13i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_DIFF_AREA_REMOVED: VSS_PROTECTION_FAULT = 14i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_EXTERNAL_WRITER_TO_DIFF_AREA: VSS_PROTECTION_FAULT = 15i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_MOUNT_DURING_CLUSTER_OFFLINE: VSS_PROTECTION_FAULT = 16i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_PROTECTION_LEVEL = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_LEVEL_ORIGINAL_VOLUME: VSS_PROTECTION_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_LEVEL_SNAPSHOT: VSS_PROTECTION_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_PROVIDER_CAPABILITIES = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_LEGACY: VSS_PROVIDER_CAPABILITIES = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_COMPLIANT: VSS_PROVIDER_CAPABILITIES = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_LUN_REPOINT: VSS_PROVIDER_CAPABILITIES = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_LUN_RESYNC: VSS_PROVIDER_CAPABILITIES = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_OFFLINE_CREATION: VSS_PROVIDER_CAPABILITIES = 16i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_MULTIPLE_IMPORT: VSS_PROVIDER_CAPABILITIES = 32i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_RECYCLING: VSS_PROVIDER_CAPABILITIES = 64i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_PLEX: VSS_PROVIDER_CAPABILITIES = 128i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_DIFFERENTIAL: VSS_PROVIDER_CAPABILITIES = 256i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_CLUSTERED: VSS_PROVIDER_CAPABILITIES = 512i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub struct VSS_PROVIDER_PROP {
    pub m_ProviderId: ::windows_sys::core::GUID,
    pub m_pwszProviderName: *mut u16,
    pub m_eProviderType: VSS_PROVIDER_TYPE,
    pub m_pwszProviderVersion: *mut u16,
    pub m_ProviderVersionId: ::windows_sys::core::GUID,
    pub m_ClassId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VSS_PROVIDER_PROP {}
impl ::core::clone::Clone for VSS_PROVIDER_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_PROVIDER_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROV_UNKNOWN: VSS_PROVIDER_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROV_SYSTEM: VSS_PROVIDER_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROV_SOFTWARE: VSS_PROVIDER_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROV_HARDWARE: VSS_PROVIDER_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROV_FILESHARE: VSS_PROVIDER_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_RECOVERY_OPTIONS = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RECOVERY_REVERT_IDENTITY_ALL: VSS_RECOVERY_OPTIONS = 256i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RECOVERY_NO_VOLUME_CHECK: VSS_RECOVERY_OPTIONS = 512i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_RESTOREMETHOD_ENUM = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_UNDEFINED: VSS_RESTOREMETHOD_ENUM = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_RESTORE_IF_NOT_THERE: VSS_RESTOREMETHOD_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_RESTORE_IF_CAN_REPLACE: VSS_RESTOREMETHOD_ENUM = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_STOP_RESTORE_START: VSS_RESTOREMETHOD_ENUM = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_RESTORE_TO_ALTERNATE_LOCATION: VSS_RESTOREMETHOD_ENUM = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_RESTORE_AT_REBOOT: VSS_RESTOREMETHOD_ENUM = 5i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_RESTORE_AT_REBOOT_IF_CANNOT_REPLACE: VSS_RESTOREMETHOD_ENUM = 6i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_CUSTOM: VSS_RESTOREMETHOD_ENUM = 7i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_RESTORE_STOP_START: VSS_RESTOREMETHOD_ENUM = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_RESTORE_TARGET = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RT_UNDEFINED: VSS_RESTORE_TARGET = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RT_ORIGINAL: VSS_RESTORE_TARGET = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RT_ALTERNATE: VSS_RESTORE_TARGET = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RT_DIRECTED: VSS_RESTORE_TARGET = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RT_ORIGINAL_LOCATION: VSS_RESTORE_TARGET = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_RESTORE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RTYPE_UNDEFINED: VSS_RESTORE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RTYPE_BY_COPY: VSS_RESTORE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RTYPE_IMPORT: VSS_RESTORE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RTYPE_OTHER: VSS_RESTORE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_ROLLFORWARD_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RF_UNDEFINED: VSS_ROLLFORWARD_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RF_NONE: VSS_ROLLFORWARD_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RF_ALL: VSS_ROLLFORWARD_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RF_PARTIAL: VSS_ROLLFORWARD_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_SNAPSHOT_COMPATIBILITY = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SC_DISABLE_DEFRAG: VSS_SNAPSHOT_COMPATIBILITY = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SC_DISABLE_CONTENTINDEX: VSS_SNAPSHOT_COMPATIBILITY = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_SNAPSHOT_CONTEXT = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CTX_BACKUP: VSS_SNAPSHOT_CONTEXT = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CTX_FILE_SHARE_BACKUP: VSS_SNAPSHOT_CONTEXT = 16i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CTX_NAS_ROLLBACK: VSS_SNAPSHOT_CONTEXT = 25i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CTX_APP_ROLLBACK: VSS_SNAPSHOT_CONTEXT = 9i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CTX_CLIENT_ACCESSIBLE: VSS_SNAPSHOT_CONTEXT = 29i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CTX_CLIENT_ACCESSIBLE_WRITERS: VSS_SNAPSHOT_CONTEXT = 13i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CTX_ALL: VSS_SNAPSHOT_CONTEXT = -1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub struct VSS_SNAPSHOT_PROP {
    pub m_SnapshotId: ::windows_sys::core::GUID,
    pub m_SnapshotSetId: ::windows_sys::core::GUID,
    pub m_lSnapshotsCount: i32,
    pub m_pwszSnapshotDeviceObject: *mut u16,
    pub m_pwszOriginalVolumeName: *mut u16,
    pub m_pwszOriginatingMachine: *mut u16,
    pub m_pwszServiceMachine: *mut u16,
    pub m_pwszExposedName: *mut u16,
    pub m_pwszExposedPath: *mut u16,
    pub m_ProviderId: ::windows_sys::core::GUID,
    pub m_lSnapshotAttributes: i32,
    pub m_tsCreationTimestamp: i64,
    pub m_eStatus: VSS_SNAPSHOT_STATE,
}
impl ::core::marker::Copy for VSS_SNAPSHOT_PROP {}
impl ::core::clone::Clone for VSS_SNAPSHOT_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_SNAPSHOT_PROPERTY_ID = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_UNKNOWN: VSS_SNAPSHOT_PROPERTY_ID = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_SNAPSHOT_ID: VSS_SNAPSHOT_PROPERTY_ID = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_SNAPSHOT_SET_ID: VSS_SNAPSHOT_PROPERTY_ID = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_SNAPSHOTS_COUNT: VSS_SNAPSHOT_PROPERTY_ID = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_SNAPSHOT_DEVICE: VSS_SNAPSHOT_PROPERTY_ID = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_ORIGINAL_VOLUME: VSS_SNAPSHOT_PROPERTY_ID = 5i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_ORIGINATING_MACHINE: VSS_SNAPSHOT_PROPERTY_ID = 6i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_SERVICE_MACHINE: VSS_SNAPSHOT_PROPERTY_ID = 7i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_EXPOSED_NAME: VSS_SNAPSHOT_PROPERTY_ID = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_EXPOSED_PATH: VSS_SNAPSHOT_PROPERTY_ID = 9i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_PROVIDER_ID: VSS_SNAPSHOT_PROPERTY_ID = 10i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_SNAPSHOT_ATTRIBUTES: VSS_SNAPSHOT_PROPERTY_ID = 11i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_CREATION_TIMESTAMP: VSS_SNAPSHOT_PROPERTY_ID = 12i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_STATUS: VSS_SNAPSHOT_PROPERTY_ID = 13i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_SNAPSHOT_STATE = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_UNKNOWN: VSS_SNAPSHOT_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PREPARING: VSS_SNAPSHOT_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PROCESSING_PREPARE: VSS_SNAPSHOT_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PREPARED: VSS_SNAPSHOT_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PROCESSING_PRECOMMIT: VSS_SNAPSHOT_STATE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PRECOMMITTED: VSS_SNAPSHOT_STATE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PROCESSING_COMMIT: VSS_SNAPSHOT_STATE = 6i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_COMMITTED: VSS_SNAPSHOT_STATE = 7i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PROCESSING_POSTCOMMIT: VSS_SNAPSHOT_STATE = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PROCESSING_PREFINALCOMMIT: VSS_SNAPSHOT_STATE = 9i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PREFINALCOMMITTED: VSS_SNAPSHOT_STATE = 10i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PROCESSING_POSTFINALCOMMIT: VSS_SNAPSHOT_STATE = 11i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_CREATED: VSS_SNAPSHOT_STATE = 12i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_ABORTED: VSS_SNAPSHOT_STATE = 13i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_DELETED: VSS_SNAPSHOT_STATE = 14i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_POSTCOMMITTED: VSS_SNAPSHOT_STATE = 15i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_COUNT: VSS_SNAPSHOT_STATE = 16i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_SOURCE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ST_UNDEFINED: VSS_SOURCE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ST_TRANSACTEDDB: VSS_SOURCE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ST_NONTRANSACTEDDB: VSS_SOURCE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ST_OTHER: VSS_SOURCE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_SUBSCRIBE_MASK = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SM_POST_SNAPSHOT_FLAG: VSS_SUBSCRIBE_MASK = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SM_BACKUP_EVENTS_FLAG: VSS_SUBSCRIBE_MASK = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SM_RESTORE_EVENTS_FLAG: VSS_SUBSCRIBE_MASK = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SM_IO_THROTTLING_FLAG: VSS_SUBSCRIBE_MASK = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SM_ALL_FLAGS: VSS_SUBSCRIBE_MASK = -1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_S_ASYNC_CANCELLED: ::windows_sys::core::HRESULT = 271115i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_S_ASYNC_FINISHED: ::windows_sys::core::HRESULT = 271114i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_S_ASYNC_PENDING: ::windows_sys::core::HRESULT = 271113i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_S_SOME_SNAPSHOTS_NOT_IMPORTED: ::windows_sys::core::HRESULT = 271137i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_USAGE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_UT_UNDEFINED: VSS_USAGE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_UT_BOOTABLESYSTEMSTATE: VSS_USAGE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_UT_SYSTEMSERVICE: VSS_USAGE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_UT_USERDATA: VSS_USAGE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_UT_OTHER: VSS_USAGE_TYPE = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub struct VSS_VOLUME_PROP {
    pub m_pwszVolumeName: *mut u16,
    pub m_pwszVolumeDisplayName: *mut u16,
}
impl ::core::marker::Copy for VSS_VOLUME_PROP {}
impl ::core::clone::Clone for VSS_VOLUME_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VSS_VOLUME_PROTECTION_INFO {
    pub m_protectionLevel: VSS_PROTECTION_LEVEL,
    pub m_volumeIsOfflineForProtection: super::super::Foundation::BOOL,
    pub m_protectionFault: VSS_PROTECTION_FAULT,
    pub m_failureStatus: i32,
    pub m_volumeHasUnusedDiffArea: super::super::Foundation::BOOL,
    pub m_reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VSS_VOLUME_PROTECTION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VSS_VOLUME_PROTECTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_VOLUME_SNAPSHOT_ATTRIBUTES = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_PERSISTENT: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_NO_AUTORECOVERY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_CLIENT_ACCESSIBLE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_NO_AUTO_RELEASE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_NO_WRITERS: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 16i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_TRANSPORTABLE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 32i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_NOT_SURFACED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 64i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_NOT_TRANSACTED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 128i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_HARDWARE_ASSISTED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 65536i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_DIFFERENTIAL: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 131072i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_PLEX: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 262144i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_IMPORTED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 524288i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_EXPOSED_LOCALLY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 1048576i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_EXPOSED_REMOTELY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 2097152i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_AUTORECOVER: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 4194304i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_ROLLBACK_RECOVERY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 8388608i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_DELAYED_POSTSNAPSHOT: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 16777216i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_TXF_RECOVERY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 33554432i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_FILE_SHARE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 67108864i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_WRITERRESTORE_ENUM = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WRE_UNDEFINED: VSS_WRITERRESTORE_ENUM = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WRE_NEVER: VSS_WRITERRESTORE_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WRE_IF_REPLACE_FAILS: VSS_WRITERRESTORE_ENUM = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WRE_ALWAYS: VSS_WRITERRESTORE_ENUM = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_WRITER_STATE = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_UNKNOWN: VSS_WRITER_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_STABLE: VSS_WRITER_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_WAITING_FOR_FREEZE: VSS_WRITER_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_WAITING_FOR_THAW: VSS_WRITER_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_WAITING_FOR_POST_SNAPSHOT: VSS_WRITER_STATE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_WAITING_FOR_BACKUP_COMPLETE: VSS_WRITER_STATE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_IDENTIFY: VSS_WRITER_STATE = 6i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_PREPARE_BACKUP: VSS_WRITER_STATE = 7i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_PREPARE_SNAPSHOT: VSS_WRITER_STATE = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_FREEZE: VSS_WRITER_STATE = 9i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_THAW: VSS_WRITER_STATE = 10i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_POST_SNAPSHOT: VSS_WRITER_STATE = 11i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_BACKUP_COMPLETE: VSS_WRITER_STATE = 12i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_PRE_RESTORE: VSS_WRITER_STATE = 13i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_POST_RESTORE: VSS_WRITER_STATE = 14i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_BACKUPSHUTDOWN: VSS_WRITER_STATE = 15i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_COUNT: VSS_WRITER_STATE = 16i32;
pub const VssSnapshotMgmt: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 190458962, data2: 16057, data3: 18186, data4: [150, 226, 108, 109, 69, 112, 228, 15] };
