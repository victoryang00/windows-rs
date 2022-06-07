#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const EVCCBF_LASTNOTIFICATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const EVCF_DONTSHOWIFZERO: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const EVCF_ENABLEBYDEFAULT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const EVCF_ENABLEBYDEFAULT_AUTO: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const EVCF_HASSETTINGS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const EVCF_OUTOFDISKSPACE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const EVCF_REMOVEFROMLIST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const EVCF_SETTINGSMODE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const EVCF_SYSTEMAUTORUN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const EVCF_USERCONSENTOBTAINED: u32 = 128u32;
#[repr(C)]
pub struct IADesktopP2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub ReReadWallpaper: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetADObjectFlags: unsafe extern "system" fn(this: *mut *mut Self, pdwflags: *mut u32, dwmask: u32) -> ::windows_sys::core::HRESULT,
    pub UpdateAllDesktopSubscriptions: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")]
    pub MakeDynamicChanges: unsafe extern "system" fn(this: *mut *mut Self, poleobj: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    MakeDynamicChanges: usize,
}
impl ::windows_sys::core::Interface for IADesktopP2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2988922082, data2: 17780, data3: 4561, data4: [152, 136, 0, 96, 151, 222, 172, 249] };
}
#[repr(C)]
pub struct IActiveDesktopP {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetSafeMode: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub EnsureUpdateHTML: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetScheme: unsafe extern "system" fn(this: *mut *mut Self, pwszschemename: ::windows_sys::core::PCWSTR, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub GetScheme: unsafe extern "system" fn(this: *mut *mut Self, pwszschemename: ::windows_sys::core::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IActiveDesktopP {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1380986592, data2: 60544, data3: 4560, data4: [137, 171, 0, 192, 79, 194, 151, 45] };
}
#[repr(C)]
pub struct IBriefcaseInitiator {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub IsMonikerInBriefcase: unsafe extern "system" fn(this: *mut *mut Self, pmk: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IsMonikerInBriefcase: usize,
}
impl ::windows_sys::core::Interface for IBriefcaseInitiator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2568487268, data2: 55830, data3: 4122, data4: [147, 92, 68, 69, 83, 84, 0, 0] };
}
#[repr(C)]
pub struct IEmptyVolumeCache {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Registry")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: ::windows_sys::core::PCWSTR, ppwszdisplayname: *mut ::windows_sys::core::PWSTR, ppwszdescription: *mut ::windows_sys::core::PWSTR, pdwflags: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    Initialize: usize,
    pub GetSpaceUsed: unsafe extern "system" fn(this: *mut *mut Self, pdwlspaceused: *mut u64, picb: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Purge: unsafe extern "system" fn(this: *mut *mut Self, dwlspacetofree: u64, picb: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowProperties: unsafe extern "system" fn(this: *mut *mut Self, hwnd: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowProperties: usize,
    pub Deactivate: unsafe extern "system" fn(this: *mut *mut Self, pdwflags: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmptyVolumeCache {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2412663335, data2: 1242, data3: 4561, data4: [160, 4, 0, 128, 95, 138, 190, 6] };
}
#[repr(C)]
pub struct IEmptyVolumeCache2 {
    pub base__: IEmptyVolumeCache,
    #[cfg(feature = "Win32_System_Registry")]
    pub InitializeEx: unsafe extern "system" fn(this: *mut *mut Self, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: ::windows_sys::core::PCWSTR, pcwszkeyname: ::windows_sys::core::PCWSTR, ppwszdisplayname: *mut ::windows_sys::core::PWSTR, ppwszdescription: *mut ::windows_sys::core::PWSTR, ppwszbtntext: *mut ::windows_sys::core::PWSTR, pdwflags: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    InitializeEx: usize,
}
impl ::windows_sys::core::Interface for IEmptyVolumeCache2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 45605818, data2: 19891, data3: 4562, data4: [178, 217, 0, 192, 79, 142, 236, 140] };
}
#[repr(C)]
pub struct IEmptyVolumeCacheCallBack {
    pub base__: ::windows_sys::core::IUnknown,
    pub ScanProgress: unsafe extern "system" fn(this: *mut *mut Self, dwlspaceused: u64, dwflags: u32, pcwszstatus: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub PurgeProgress: unsafe extern "system" fn(this: *mut *mut Self, dwlspacefreed: u64, dwlspacetofree: u64, dwflags: u32, pcwszstatus: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmptyVolumeCacheCallBack {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1853436769, data2: 29638, data3: 4560, data4: [132, 105, 0, 170, 0, 68, 41, 1] };
}
#[repr(C)]
pub struct IReconcilableObject {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub Reconcile: unsafe extern "system" fn(this: *mut *mut Self, pinitiator: *mut ::core::ffi::c_void, dwflags: u32, hwndowner: super::super::Foundation::HWND, hwndprogressfeedback: super::super::Foundation::HWND, ulcinput: u32, rgpmkotherinput: *mut *mut ::core::ffi::c_void, ploutindex: *mut i32, pstgnewresidues: *mut ::core::ffi::c_void, pvreserved: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    Reconcile: usize,
    pub GetProgressFeedbackMaxEstimate: unsafe extern "system" fn(this: *mut *mut Self, pulprogressmax: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IReconcilableObject {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2568487266, data2: 55830, data3: 4122, data4: [147, 92, 68, 69, 83, 84, 0, 0] };
}
#[repr(C)]
pub struct IReconcileInitiator {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetAbortCallback: unsafe extern "system" fn(this: *mut *mut Self, punkforabort: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetProgressFeedback: unsafe extern "system" fn(this: *mut *mut Self, ulprogress: u32, ulprogressmax: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IReconcileInitiator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2568487265, data2: 55830, data3: 4122, data4: [147, 92, 68, 69, 83, 84, 0, 0] };
}
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const REC_E_ABORTED: ::windows_sys::core::HRESULT = -2147217408i32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const REC_E_INEEDTODOTHEUPDATES: ::windows_sys::core::HRESULT = -2147217404i32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const REC_E_NOCALLBACK: ::windows_sys::core::HRESULT = -2147217407i32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const REC_E_NORESIDUES: ::windows_sys::core::HRESULT = -2147217406i32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const REC_E_TOODIFFERENT: ::windows_sys::core::HRESULT = -2147217405i32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const REC_S_IDIDTHEUPDATES: ::windows_sys::core::HRESULT = 266240i32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const REC_S_NOTCOMPLETE: ::windows_sys::core::HRESULT = 266241i32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const REC_S_NOTCOMPLETEBUTPROPAGATE: ::windows_sys::core::HRESULT = 266242i32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const STATEBITS_FLAT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub type _reconcilef = i32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const RECONCILEF_MAYBOTHERUSER: _reconcilef = 1i32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const RECONCILEF_FEEDBACKWINDOWVALID: _reconcilef = 2i32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const RECONCILEF_NORESIDUESOK: _reconcilef = 4i32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const RECONCILEF_OMITSELFRESIDUE: _reconcilef = 8i32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const RECONCILEF_RESUMERECONCILIATION: _reconcilef = 16i32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const RECONCILEF_YOUMAYDOTHEUPDATES: _reconcilef = 32i32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const RECONCILEF_ONLYYOUWERECHANGED: _reconcilef = 64i32;
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`*"]
pub const ALL_RECONCILE_FLAGS: _reconcilef = 127i32;
