pub const Catalog: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1857169537, data2: 35353, data3: 4560, data4: [129, 182, 0, 160, 201, 35, 28, 41] };
pub const CatalogCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1857169539, data2: 35353, data3: 4560, data4: [129, 182, 0, 160, 201, 35, 28, 41] };
pub const CatalogObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1857169538, data2: 35353, data3: 4560, data4: [129, 182, 0, 160, 201, 35, 28, 41] };
pub const ComponentUtil: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1857169540, data2: 35353, data3: 4560, data4: [129, 182, 0, 160, 201, 35, 28, 41] };
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICatalog {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetCollection: unsafe extern "system" fn(this: *mut *mut Self, bstrcollname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetCollection: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Connect: unsafe extern "system" fn(this: *mut *mut Self, bstrconnectstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Connect: usize,
    pub MajorVersion: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IComponentUtil {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub InstallComponent: unsafe extern "system" fn(this: *mut *mut Self, bstrdllfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtypelibfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrproxystubdllfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InstallComponent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ImportComponent: unsafe extern "system" fn(this: *mut *mut Self, bstrclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ImportComponent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ImportComponentByName: unsafe extern "system" fn(this: *mut *mut Self, bstrprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ImportComponentByName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetCLSIDs: unsafe extern "system" fn(this: *mut *mut Self, bstrdllfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtypelibfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, aclsids: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetCLSIDs: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IPackageUtil {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub InstallPackage: unsafe extern "system" fn(this: *mut *mut Self, bstrpackagefile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrinstallpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InstallPackage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExportPackage: unsafe extern "system" fn(this: *mut *mut Self, bstrpackageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpackagefile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExportPackage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShutdownPackage: unsafe extern "system" fn(this: *mut *mut Self, bstrpackageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShutdownPackage: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRemoteComponentUtil {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub InstallRemoteComponent: unsafe extern "system" fn(this: *mut *mut Self, bstrserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpackageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InstallRemoteComponent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InstallRemoteComponentByName: unsafe extern "system" fn(this: *mut *mut Self, bstrserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpackagename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InstallRemoteComponentByName: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRoleAssociationUtil {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub AssociateRole: unsafe extern "system" fn(this: *mut *mut Self, bstrroleid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AssociateRole: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AssociateRoleByName: unsafe extern "system" fn(this: *mut *mut Self, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AssociateRoleByName: usize,
}
pub const PackageUtil: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1857169541, data2: 35353, data3: 4560, data4: [129, 182, 0, 160, 201, 35, 28, 41] };
pub const RemoteComponentUtil: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1857169542, data2: 35353, data3: 4560, data4: [129, 182, 0, 160, 201, 35, 28, 41] };
pub const RoleAssociationUtil: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1857169543, data2: 35353, data3: 4560, data4: [129, 182, 0, 160, 201, 35, 28, 41] };
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub type __MIDL___MIDL_itf_mtxadmin_0107_0001 = i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsInstallUsers: __MIDL___MIDL_itf_mtxadmin_0107_0001 = 1i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub type __MIDL___MIDL_itf_mtxadmin_0107_0002 = i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsExportUsers: __MIDL___MIDL_itf_mtxadmin_0107_0002 = 1i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub type __MIDL___MIDL_itf_mtxadmin_0107_0003 = i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrObjectErrors: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368511i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrObjectInvalid: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368510i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrKeyMissing: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368509i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrAlreadyInstalled: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368508i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrDownloadFailed: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368507i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrPDFWriteFail: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368505i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrPDFReadFail: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368504i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrPDFVersion: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368503i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCoReqCompInstalled: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368496i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrBadPath: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368502i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrPackageExists: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368501i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrRoleExists: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368500i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCantCopyFile: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368499i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoTypeLib: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368498i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoUser: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368497i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrInvalidUserids: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368496i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoRegistryCLSID: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368495i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrBadRegistryProgID: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368494i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrAuthenticationLevel: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368493i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrUserPasswdNotValid: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368492i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoRegistryRead: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368491i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoRegistryWrite: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368490i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoRegistryRepair: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368489i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCLSIDOrIIDMismatch: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368488i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrRemoteInterface: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368487i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrDllRegisterServer: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368486i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoServerShare: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368485i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoAccessToUNC: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368484i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrDllLoadFailed: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368483i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrBadRegistryLibID: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368482i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrPackDirNotFound: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368481i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrTreatAs: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368480i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrBadForward: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368479i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrBadIID: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368478i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrRegistrarFailed: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368477i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCompFileDoesNotExist: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368476i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCompFileLoadDLLFail: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368475i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCompFileGetClassObj: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368474i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCompFileClassNotAvail: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368473i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCompFileBadTLB: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368472i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCompFileNotInstallable: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368471i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNotChangeable: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368470i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNotDeletable: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368469i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrSession: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368468i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCompFileNoRegistrar: __MIDL___MIDL_itf_mtxadmin_0107_0003 = -2146368460i32;
