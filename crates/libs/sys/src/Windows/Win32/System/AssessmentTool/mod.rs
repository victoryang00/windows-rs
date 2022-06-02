pub const CAccessiblityWinSAT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1847130566, data2: 41963, data3: 18778, data4: [137, 183, 149, 100, 130, 225, 159, 122] };
pub const CInitiateWinSAT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1217606108, data2: 62944, data3: 17704, data4: [159, 218, 69, 51, 27, 244, 165, 113] };
pub const CProvideWinSATVisuals: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2671213950, data2: 58705, data3: 17656, data4: [159, 148, 157, 179, 146, 176, 59, 123] };
pub const CQueryAllWinSAT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 98536723, data2: 50005, data3: 18420, data4: [161, 30, 133, 27, 51, 140, 239, 184] };
pub const CQueryOEMWinSATCustomization: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3296346551, data2: 46889, data3: 16975, data4: [154, 249, 92, 179, 147, 79, 45, 250] };
pub const CQueryWinSAT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4089314003, data2: 62070, data3: 18921, data4: [155, 23, 196, 116, 244, 143, 7, 100] };
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
#[repr(C)]
pub struct IAccessibleWinSAT {
    pub base__: super::super::UI::Accessibility::IAccessible,
    pub SetAccessiblityData: unsafe extern "system" fn(this: *mut *mut Self, wsname: ::windows_sys::core::PCWSTR, wsvalue: ::windows_sys::core::PCWSTR, wsdesc: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInitiateWinSATAssessment {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub InitiateAssessment: unsafe extern "system" fn(this: *mut *mut Self, cmdline: ::windows_sys::core::PCWSTR, pcallbacks: *mut ::core::ffi::c_void, callerhwnd: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitiateAssessment: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitiateFormalAssessment: unsafe extern "system" fn(this: *mut *mut Self, pcallbacks: *mut ::core::ffi::c_void, callerhwnd: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitiateFormalAssessment: usize,
    pub CancelAssessment: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IProvideWinSATAssessmentInfo {
    pub base__: super::Com::IDispatch,
    pub Score: unsafe extern "system" fn(this: *mut *mut Self, score: *mut f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, title: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Title: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, description: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IProvideWinSATResultsInfo {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAssessmentInfo: unsafe extern "system" fn(this: *mut *mut Self, assessment: WINSAT_ASSESSMENT_TYPE, ppinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAssessmentInfo: usize,
    pub AssessmentState: unsafe extern "system" fn(this: *mut *mut Self, state: *mut WINSAT_ASSESSMENT_STATE) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AssessmentDateTime: unsafe extern "system" fn(this: *mut *mut Self, filetime: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AssessmentDateTime: usize,
    pub SystemRating: unsafe extern "system" fn(this: *mut *mut Self, level: *mut f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RatingStateDesc: unsafe extern "system" fn(this: *mut *mut Self, description: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RatingStateDesc: usize,
}
#[repr(C)]
pub struct IProvideWinSATVisuals {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub get_Bitmap: unsafe extern "system" fn(this: *mut *mut Self, bitmapsize: WINSAT_BITMAP_SIZE, state: WINSAT_ASSESSMENT_STATE, rating: f32, pbitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    get_Bitmap: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IQueryAllWinSATAssessments {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub get_AllXML: unsafe extern "system" fn(this: *mut *mut Self, xpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, namespaces: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdomnodelist: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    get_AllXML: usize,
}
#[repr(C)]
pub struct IQueryOEMWinSATCustomization {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetOEMPrePopulationInfo: unsafe extern "system" fn(this: *mut *mut Self, state: *mut WINSAT_OEM_DATA_TYPE) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IQueryRecentWinSATAssessment {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub get_XML: unsafe extern "system" fn(this: *mut *mut Self, xpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, namespaces: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdomnodelist: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    get_XML: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Info: unsafe extern "system" fn(this: *mut *mut Self, ppwinsatassessmentinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Info: usize,
}
#[repr(C)]
pub struct IWinSATInitiateEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub WinSATComplete: unsafe extern "system" fn(this: *mut *mut Self, hresult: ::windows_sys::core::HRESULT, strdescription: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub WinSATUpdate: unsafe extern "system" fn(this: *mut *mut Self, ucurrenttick: u32, uticktotal: u32, strcurrentstate: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub type WINSAT_ASSESSMENT_STATE = i32;
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_STATE_MIN: WINSAT_ASSESSMENT_STATE = 0i32;
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_STATE_UNKNOWN: WINSAT_ASSESSMENT_STATE = 0i32;
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_STATE_VALID: WINSAT_ASSESSMENT_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_STATE_INCOHERENT_WITH_HARDWARE: WINSAT_ASSESSMENT_STATE = 2i32;
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_STATE_NOT_AVAILABLE: WINSAT_ASSESSMENT_STATE = 3i32;
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_STATE_INVALID: WINSAT_ASSESSMENT_STATE = 4i32;
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_STATE_MAX: WINSAT_ASSESSMENT_STATE = 4i32;
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub type WINSAT_ASSESSMENT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_MEMORY: WINSAT_ASSESSMENT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_CPU: WINSAT_ASSESSMENT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_DISK: WINSAT_ASSESSMENT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_D3D: WINSAT_ASSESSMENT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_ASSESSMENT_GRAPHICS: WINSAT_ASSESSMENT_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub type WINSAT_BITMAP_SIZE = i32;
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_BITMAP_SIZE_SMALL: WINSAT_BITMAP_SIZE = 0i32;
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_BITMAP_SIZE_NORMAL: WINSAT_BITMAP_SIZE = 1i32;
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub type WINSAT_OEM_DATA_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_OEM_DATA_VALID: WINSAT_OEM_DATA_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_OEM_DATA_NON_SYS_CONFIG_MATCH: WINSAT_OEM_DATA_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_OEM_DATA_INVALID: WINSAT_OEM_DATA_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_AssessmentTool\"`*"]
pub const WINSAT_OEM_NO_DATA_SUPPLIED: WINSAT_OEM_DATA_TYPE = 3i32;
