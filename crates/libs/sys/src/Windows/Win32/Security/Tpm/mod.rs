#[repr(C)]
pub struct ITpmVirtualSmartCardManager {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateVirtualSmartCard: unsafe extern "system" fn(this: *mut *mut Self, pszfriendlyname: ::windows_sys::core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: *mut ::core::ffi::c_void, ppszinstanceid: *mut ::windows_sys::core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateVirtualSmartCard: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DestroyVirtualSmartCard: unsafe extern "system" fn(this: *mut *mut Self, pszinstanceid: ::windows_sys::core::PCWSTR, pstatuscallback: *mut ::core::ffi::c_void, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DestroyVirtualSmartCard: usize,
}
impl ::windows_sys::core::Interface for ITpmVirtualSmartCardManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 288038399, data2: 55772, data3: 16887, data4: [134, 159, 214, 127, 238, 124, 181, 145] };
}
#[repr(C)]
pub struct ITpmVirtualSmartCardManager2 {
    pub base__: ITpmVirtualSmartCardManager,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateVirtualSmartCardWithPinPolicy: unsafe extern "system" fn(this: *mut *mut Self, pszfriendlyname: ::windows_sys::core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: *mut ::core::ffi::c_void, ppszinstanceid: *mut ::windows_sys::core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateVirtualSmartCardWithPinPolicy: usize,
}
impl ::windows_sys::core::Interface for ITpmVirtualSmartCardManager2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4260930233, data2: 734, data3: 18420, data4: [188, 38, 170, 133, 171, 94, 82, 103] };
}
#[repr(C)]
pub struct ITpmVirtualSmartCardManager3 {
    pub base__: ITpmVirtualSmartCardManager2,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateVirtualSmartCardWithAttestation: unsafe extern "system" fn(this: *mut *mut Self, pszfriendlyname: ::windows_sys::core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, attestationtype: TPMVSC_ATTESTATION_TYPE, fgenerate: super::super::Foundation::BOOL, pstatuscallback: *mut ::core::ffi::c_void, ppszinstanceid: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateVirtualSmartCardWithAttestation: usize,
}
impl ::windows_sys::core::Interface for ITpmVirtualSmartCardManager3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1014258327, data2: 62325, data3: 16720, data4: [190, 23, 89, 80, 246, 148, 198, 153] };
}
#[repr(C)]
pub struct ITpmVirtualSmartCardManagerStatusCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub ReportProgress: unsafe extern "system" fn(this: *mut *mut Self, status: TPMVSCMGR_STATUS) -> ::windows_sys::core::HRESULT,
    pub ReportError: unsafe extern "system" fn(this: *mut *mut Self, error: TPMVSCMGR_ERROR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITpmVirtualSmartCardManagerStatusCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 438023007, data2: 43960, data3: 17692, data4: [161, 174, 51, 217, 143, 27, 239, 74] };
}
pub const RemoteTpmVirtualSmartCardManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 355377832, data2: 28892, data3: 19545, data4: [139, 42, 50, 170, 60, 160, 220, 172] };
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub type TPMVSCMGR_ERROR = i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_IMPERSONATION: TPMVSCMGR_ERROR = 0i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_PIN_COMPLEXITY: TPMVSCMGR_ERROR = 1i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_READER_COUNT_LIMIT: TPMVSCMGR_ERROR = 2i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_TERMINAL_SERVICES_SESSION: TPMVSCMGR_ERROR = 3i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VTPMSMARTCARD_INITIALIZE: TPMVSCMGR_ERROR = 4i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VTPMSMARTCARD_CREATE: TPMVSCMGR_ERROR = 5i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VTPMSMARTCARD_DESTROY: TPMVSCMGR_ERROR = 6i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_INITIALIZE: TPMVSCMGR_ERROR = 7i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_CREATE: TPMVSCMGR_ERROR = 8i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_DESTROY: TPMVSCMGR_ERROR = 9i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_WRITE_PROPERTY: TPMVSCMGR_ERROR = 10i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_READ_PROPERTY: TPMVSCMGR_ERROR = 11i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VREADER_INITIALIZE: TPMVSCMGR_ERROR = 12i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VREADER_CREATE: TPMVSCMGR_ERROR = 13i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VREADER_DESTROY: TPMVSCMGR_ERROR = 14i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_GENERATE_LOCATE_READER: TPMVSCMGR_ERROR = 15i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_GENERATE_FILESYSTEM: TPMVSCMGR_ERROR = 16i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_CARD_CREATE: TPMVSCMGR_ERROR = 17i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_CARD_DESTROY: TPMVSCMGR_ERROR = 18i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub type TPMVSCMGR_STATUS = i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_VTPMSMARTCARD_INITIALIZING: TPMVSCMGR_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_VTPMSMARTCARD_CREATING: TPMVSCMGR_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_VTPMSMARTCARD_DESTROYING: TPMVSCMGR_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_VGIDSSIMULATOR_INITIALIZING: TPMVSCMGR_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_VGIDSSIMULATOR_CREATING: TPMVSCMGR_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_VGIDSSIMULATOR_DESTROYING: TPMVSCMGR_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_VREADER_INITIALIZING: TPMVSCMGR_STATUS = 6i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_VREADER_CREATING: TPMVSCMGR_STATUS = 7i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_VREADER_DESTROYING: TPMVSCMGR_STATUS = 8i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_GENERATE_WAITING: TPMVSCMGR_STATUS = 9i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_GENERATE_AUTHENTICATING: TPMVSCMGR_STATUS = 10i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_GENERATE_RUNNING: TPMVSCMGR_STATUS = 11i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_CARD_CREATED: TPMVSCMGR_STATUS = 12i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_CARD_DESTROYED: TPMVSCMGR_STATUS = 13i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub type TPMVSC_ATTESTATION_TYPE = i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSC_ATTESTATION_NONE: TPMVSC_ATTESTATION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSC_ATTESTATION_AIK_ONLY: TPMVSC_ATTESTATION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSC_ATTESTATION_AIK_AND_CERTIFICATE: TPMVSC_ATTESTATION_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSC_DEFAULT_ADMIN_ALGORITHM_ID: u32 = 130u32;
pub const TpmVirtualSmartCardManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 379686534, data2: 32622, data3: 19488, data4: [173, 137, 79, 252, 13, 183, 169, 106] };
