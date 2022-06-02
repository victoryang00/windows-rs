#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
    pub fn MI_Application_InitializeV1(flags: u32, applicationid: *const u16, extendederror: *mut *mut MI_Instance, application: *mut MI_Application) -> MI_Result;
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type CIMTYPE_ENUMERATION = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_ILLEGAL: CIMTYPE_ENUMERATION = 4095i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_EMPTY: CIMTYPE_ENUMERATION = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_SINT8: CIMTYPE_ENUMERATION = 16i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_UINT8: CIMTYPE_ENUMERATION = 17i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_SINT16: CIMTYPE_ENUMERATION = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_UINT16: CIMTYPE_ENUMERATION = 18i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_SINT32: CIMTYPE_ENUMERATION = 3i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_UINT32: CIMTYPE_ENUMERATION = 19i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_SINT64: CIMTYPE_ENUMERATION = 20i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_UINT64: CIMTYPE_ENUMERATION = 21i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_REAL32: CIMTYPE_ENUMERATION = 4i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_REAL64: CIMTYPE_ENUMERATION = 5i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_BOOLEAN: CIMTYPE_ENUMERATION = 11i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_STRING: CIMTYPE_ENUMERATION = 8i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_DATETIME: CIMTYPE_ENUMERATION = 101i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_REFERENCE: CIMTYPE_ENUMERATION = 102i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_CHAR16: CIMTYPE_ENUMERATION = 103i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_OBJECT: CIMTYPE_ENUMERATION = 13i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_FLAG_ARRAY: CIMTYPE_ENUMERATION = 8192i32;
#[repr(C)]
pub struct IEnumWbemClassObject {
    pub base__: ::windows_sys::core::IUnknown,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, ltimeout: i32, ucount: u32, apobjects: *mut *mut ::core::ffi::c_void, pureturned: *mut u32) -> ::windows_sys::core::HRESULT,
    pub NextAsync: unsafe extern "system" fn(this: *mut *mut Self, ucount: u32, psink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, ltimeout: i32, ncount: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMofCompiler {
    pub base__: ::windows_sys::core::IUnknown,
    pub CompileFile: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR, serverandnamespace: ::windows_sys::core::PCWSTR, user: ::windows_sys::core::PCWSTR, authority: ::windows_sys::core::PCWSTR, password: ::windows_sys::core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows_sys::core::HRESULT,
    pub CompileBuffer: unsafe extern "system" fn(this: *mut *mut Self, buffsize: i32, pbuffer: *const u8, serverandnamespace: ::windows_sys::core::PCWSTR, user: ::windows_sys::core::PCWSTR, authority: ::windows_sys::core::PCWSTR, password: ::windows_sys::core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows_sys::core::HRESULT,
    pub CreateBMOF: unsafe extern "system" fn(this: *mut *mut Self, textfilename: ::windows_sys::core::PCWSTR, bmoffilename: ::windows_sys::core::PCWSTR, serverandnamespace: ::windows_sys::core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemDateTime {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, strvalue: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Value: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, strvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetValue: usize,
    pub Year: unsafe extern "system" fn(this: *mut *mut Self, iyear: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetYear: unsafe extern "system" fn(this: *mut *mut Self, iyear: i32) -> ::windows_sys::core::HRESULT,
    pub YearSpecified: unsafe extern "system" fn(this: *mut *mut Self, byearspecified: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetYearSpecified: unsafe extern "system" fn(this: *mut *mut Self, byearspecified: i16) -> ::windows_sys::core::HRESULT,
    pub Month: unsafe extern "system" fn(this: *mut *mut Self, imonth: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMonth: unsafe extern "system" fn(this: *mut *mut Self, imonth: i32) -> ::windows_sys::core::HRESULT,
    pub MonthSpecified: unsafe extern "system" fn(this: *mut *mut Self, bmonthspecified: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetMonthSpecified: unsafe extern "system" fn(this: *mut *mut Self, bmonthspecified: i16) -> ::windows_sys::core::HRESULT,
    pub Day: unsafe extern "system" fn(this: *mut *mut Self, iday: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetDay: unsafe extern "system" fn(this: *mut *mut Self, iday: i32) -> ::windows_sys::core::HRESULT,
    pub DaySpecified: unsafe extern "system" fn(this: *mut *mut Self, bdayspecified: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetDaySpecified: unsafe extern "system" fn(this: *mut *mut Self, bdayspecified: i16) -> ::windows_sys::core::HRESULT,
    pub Hours: unsafe extern "system" fn(this: *mut *mut Self, ihours: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetHours: unsafe extern "system" fn(this: *mut *mut Self, ihours: i32) -> ::windows_sys::core::HRESULT,
    pub HoursSpecified: unsafe extern "system" fn(this: *mut *mut Self, bhoursspecified: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetHoursSpecified: unsafe extern "system" fn(this: *mut *mut Self, bhoursspecified: i16) -> ::windows_sys::core::HRESULT,
    pub Minutes: unsafe extern "system" fn(this: *mut *mut Self, iminutes: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMinutes: unsafe extern "system" fn(this: *mut *mut Self, iminutes: i32) -> ::windows_sys::core::HRESULT,
    pub MinutesSpecified: unsafe extern "system" fn(this: *mut *mut Self, bminutesspecified: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetMinutesSpecified: unsafe extern "system" fn(this: *mut *mut Self, bminutesspecified: i16) -> ::windows_sys::core::HRESULT,
    pub Seconds: unsafe extern "system" fn(this: *mut *mut Self, iseconds: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetSeconds: unsafe extern "system" fn(this: *mut *mut Self, iseconds: i32) -> ::windows_sys::core::HRESULT,
    pub SecondsSpecified: unsafe extern "system" fn(this: *mut *mut Self, bsecondsspecified: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetSecondsSpecified: unsafe extern "system" fn(this: *mut *mut Self, bsecondsspecified: i16) -> ::windows_sys::core::HRESULT,
    pub Microseconds: unsafe extern "system" fn(this: *mut *mut Self, imicroseconds: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMicroseconds: unsafe extern "system" fn(this: *mut *mut Self, imicroseconds: i32) -> ::windows_sys::core::HRESULT,
    pub MicrosecondsSpecified: unsafe extern "system" fn(this: *mut *mut Self, bmicrosecondsspecified: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetMicrosecondsSpecified: unsafe extern "system" fn(this: *mut *mut Self, bmicrosecondsspecified: i16) -> ::windows_sys::core::HRESULT,
    pub UTC: unsafe extern "system" fn(this: *mut *mut Self, iutc: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetUTC: unsafe extern "system" fn(this: *mut *mut Self, iutc: i32) -> ::windows_sys::core::HRESULT,
    pub UTCSpecified: unsafe extern "system" fn(this: *mut *mut Self, butcspecified: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetUTCSpecified: unsafe extern "system" fn(this: *mut *mut Self, butcspecified: i16) -> ::windows_sys::core::HRESULT,
    pub IsInterval: unsafe extern "system" fn(this: *mut *mut Self, bisinterval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetIsInterval: unsafe extern "system" fn(this: *mut *mut Self, bisinterval: i16) -> ::windows_sys::core::HRESULT,
    pub GetVarDate: unsafe extern "system" fn(this: *mut *mut Self, bislocal: i16, dvardate: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetVarDate: unsafe extern "system" fn(this: *mut *mut Self, dvardate: f64, bislocal: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFileTime: unsafe extern "system" fn(this: *mut *mut Self, bislocal: i16, strfiletime: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFileTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFileTime: unsafe extern "system" fn(this: *mut *mut Self, strfiletime: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bislocal: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFileTime: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemEventSource {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub NextEvent: unsafe extern "system" fn(this: *mut *mut Self, itimeoutms: i32, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NextEvent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Security_: unsafe extern "system" fn(this: *mut *mut Self, objwbemsecurity: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Security_: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemLastError {
    pub base__: ISWbemObject,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemLocator {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ConnectServer: unsafe extern "system" fn(this: *mut *mut Self, strserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, struser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strauthority: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, isecurityflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemservices: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ConnectServer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Security_: unsafe extern "system" fn(this: *mut *mut Self, objwbemsecurity: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Security_: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemMethod {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, strname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Origin: unsafe extern "system" fn(this: *mut *mut Self, strorigin: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Origin: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InParameters: unsafe extern "system" fn(this: *mut *mut Self, objwbeminparameters: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InParameters: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OutParameters: unsafe extern "system" fn(this: *mut *mut Self, objwbemoutparameters: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OutParameters: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Qualifiers_: unsafe extern "system" fn(this: *mut *mut Self, objwbemqualifierset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Qualifiers_: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemMethodSet {
    pub base__: super::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, punk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemmethod: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, icount: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemNamedValue {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, varvalue: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, varvalue: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, strname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemNamedValueSet {
    pub base__: super::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, punk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, icount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varvalue: *const super::Com::VARIANT, iflags: i32, objwbemnamedvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Add: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Remove: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, objwbemnamedvalueset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone: usize,
    pub DeleteAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemObject {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Put_: unsafe extern "system" fn(this: *mut *mut Self, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectpath: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Put_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PutAsync_: unsafe extern "system" fn(this: *mut *mut Self, objwbemsink: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PutAsync_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Delete_: unsafe extern "system" fn(this: *mut *mut Self, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Delete_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DeleteAsync_: unsafe extern "system" fn(this: *mut *mut Self, objwbemsink: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeleteAsync_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Instances_: unsafe extern "system" fn(this: *mut *mut Self, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Instances_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InstancesAsync_: unsafe extern "system" fn(this: *mut *mut Self, objwbemsink: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InstancesAsync_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Subclasses_: unsafe extern "system" fn(this: *mut *mut Self, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Subclasses_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SubclassesAsync_: unsafe extern "system" fn(this: *mut *mut Self, objwbemsink: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SubclassesAsync_: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Associators_: unsafe extern "system" fn(this: *mut *mut Self, strassocclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Associators_: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AssociatorsAsync_: unsafe extern "system" fn(this: *mut *mut Self, objwbemsink: *mut ::core::ffi::c_void, strassocclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AssociatorsAsync_: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub References_: unsafe extern "system" fn(this: *mut *mut Self, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    References_: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ReferencesAsync_: unsafe extern "system" fn(this: *mut *mut Self, objwbemsink: *mut ::core::ffi::c_void, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ReferencesAsync_: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ExecMethod_: unsafe extern "system" fn(this: *mut *mut Self, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objwbeminparameters: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemoutparameters: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ExecMethod_: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ExecMethodAsync_: unsafe extern "system" fn(this: *mut *mut Self, objwbemsink: *mut ::core::ffi::c_void, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objwbeminparameters: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ExecMethodAsync_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone_: unsafe extern "system" fn(this: *mut *mut Self, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone_: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetObjectText_: unsafe extern "system" fn(this: *mut *mut Self, iflags: i32, strobjecttext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetObjectText_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SpawnDerivedClass_: unsafe extern "system" fn(this: *mut *mut Self, iflags: i32, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SpawnDerivedClass_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SpawnInstance_: unsafe extern "system" fn(this: *mut *mut Self, iflags: i32, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SpawnInstance_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CompareTo_: unsafe extern "system" fn(this: *mut *mut Self, objwbemobject: *mut ::core::ffi::c_void, iflags: i32, bresult: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CompareTo_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Qualifiers_: unsafe extern "system" fn(this: *mut *mut Self, objwbemqualifierset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Qualifiers_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties_: unsafe extern "system" fn(this: *mut *mut Self, objwbempropertyset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Methods_: unsafe extern "system" fn(this: *mut *mut Self, objwbemmethodset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Methods_: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Derivation_: unsafe extern "system" fn(this: *mut *mut Self, strclassnamearray: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Derivation_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Path_: unsafe extern "system" fn(this: *mut *mut Self, objwbemobjectpath: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Path_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Security_: unsafe extern "system" fn(this: *mut *mut Self, objwbemsecurity: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Security_: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemObjectEx {
    pub base__: ISWbemObject,
    #[cfg(feature = "Win32_System_Com")]
    pub Refresh_: unsafe extern "system" fn(this: *mut *mut Self, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Refresh_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SystemProperties_: unsafe extern "system" fn(this: *mut *mut Self, objwbempropertyset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SystemProperties_: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetText_: unsafe extern "system" fn(this: *mut *mut Self, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, bstext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetText_: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SetFromText_: unsafe extern "system" fn(this: *mut *mut Self, bstext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SetFromText_: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemObjectPath {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, strpath: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Path: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPath: unsafe extern "system" fn(this: *mut *mut Self, strpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RelPath: unsafe extern "system" fn(this: *mut *mut Self, strrelpath: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RelPath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRelPath: unsafe extern "system" fn(this: *mut *mut Self, strrelpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRelPath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Server: unsafe extern "system" fn(this: *mut *mut Self, strserver: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Server: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetServer: unsafe extern "system" fn(this: *mut *mut Self, strserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetServer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Namespace: unsafe extern "system" fn(this: *mut *mut Self, strnamespace: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Namespace: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNamespace: unsafe extern "system" fn(this: *mut *mut Self, strnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNamespace: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ParentNamespace: unsafe extern "system" fn(this: *mut *mut Self, strparentnamespace: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ParentNamespace: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, strdisplayname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, strdisplayname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Class: unsafe extern "system" fn(this: *mut *mut Self, strclass: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Class: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClass: unsafe extern "system" fn(this: *mut *mut Self, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClass: usize,
    pub IsClass: unsafe extern "system" fn(this: *mut *mut Self, bisclass: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAsClass: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub IsSingleton: unsafe extern "system" fn(this: *mut *mut Self, bissingleton: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAsSingleton: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Keys: unsafe extern "system" fn(this: *mut *mut Self, objwbemnamedvalueset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Keys: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Security_: unsafe extern "system" fn(this: *mut *mut Self, objwbemsecurity: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Security_: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Locale: unsafe extern "system" fn(this: *mut *mut Self, strlocale: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Locale: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLocale: unsafe extern "system" fn(this: *mut *mut Self, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLocale: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Authority: unsafe extern "system" fn(this: *mut *mut Self, strauthority: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Authority: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAuthority: unsafe extern "system" fn(this: *mut *mut Self, strauthority: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAuthority: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemObjectSet {
    pub base__: super::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, punk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, icount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Security_: unsafe extern "system" fn(this: *mut *mut Self, objwbemsecurity: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Security_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ItemIndex: unsafe extern "system" fn(this: *mut *mut Self, lindex: i32, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ItemIndex: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemPrivilege {
    pub base__: super::Com::IDispatch,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, bisenabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut *mut Self, bisenabled: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, strdisplayname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, strdisplayname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayName: usize,
    pub Identifier: unsafe extern "system" fn(this: *mut *mut Self, iprivilege: *mut WbemPrivilegeEnum) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemPrivilegeSet {
    pub base__: super::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, punk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, iprivilege: WbemPrivilegeEnum, objwbemprivilege: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, icount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, iprivilege: WbemPrivilegeEnum, bisenabled: i16, objwbemprivilege: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, iprivilege: WbemPrivilegeEnum) -> ::windows_sys::core::HRESULT,
    pub DeleteAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddAsString: unsafe extern "system" fn(this: *mut *mut Self, strprivilege: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bisenabled: i16, objwbemprivilege: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddAsString: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemProperty {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, varvalue: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, varvalue: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, strname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    pub IsLocal: unsafe extern "system" fn(this: *mut *mut Self, bislocal: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Origin: unsafe extern "system" fn(this: *mut *mut Self, strorigin: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Origin: usize,
    pub CIMType: unsafe extern "system" fn(this: *mut *mut Self, icimtype: *mut WbemCimtypeEnum) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Qualifiers_: unsafe extern "system" fn(this: *mut *mut Self, objwbemqualifierset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Qualifiers_: usize,
    pub IsArray: unsafe extern "system" fn(this: *mut *mut Self, bisarray: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemPropertySet {
    pub base__: super::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, punk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemproperty: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, icount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, icimtype: WbemCimtypeEnum, bisarray: i16, iflags: i32, objwbemproperty: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Add: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Remove: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemQualifier {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, varvalue: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, varvalue: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, strname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    pub IsLocal: unsafe extern "system" fn(this: *mut *mut Self, bislocal: *mut i16) -> ::windows_sys::core::HRESULT,
    pub PropagatesToSubclass: unsafe extern "system" fn(this: *mut *mut Self, bpropagatestosubclass: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetPropagatesToSubclass: unsafe extern "system" fn(this: *mut *mut Self, bpropagatestosubclass: i16) -> ::windows_sys::core::HRESULT,
    pub PropagatesToInstance: unsafe extern "system" fn(this: *mut *mut Self, bpropagatestoinstance: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetPropagatesToInstance: unsafe extern "system" fn(this: *mut *mut Self, bpropagatestoinstance: i16) -> ::windows_sys::core::HRESULT,
    pub IsOverridable: unsafe extern "system" fn(this: *mut *mut Self, bisoverridable: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetIsOverridable: unsafe extern "system" fn(this: *mut *mut Self, bisoverridable: i16) -> ::windows_sys::core::HRESULT,
    pub IsAmended: unsafe extern "system" fn(this: *mut *mut Self, bisamended: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemQualifierSet {
    pub base__: super::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, punk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemqualifier: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, icount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varval: *const super::Com::VARIANT, bpropagatestosubclass: i16, bpropagatestoinstance: i16, bisoverridable: i16, iflags: i32, objwbemqualifier: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Add: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Remove: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemRefreshableItem {
    pub base__: super::Com::IDispatch,
    pub Index: unsafe extern "system" fn(this: *mut *mut Self, iindex: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Refresher: unsafe extern "system" fn(this: *mut *mut Self, objwbemrefresher: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Refresher: usize,
    pub IsSet: unsafe extern "system" fn(this: *mut *mut Self, bisset: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Object: unsafe extern "system" fn(this: *mut *mut Self, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Object: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ObjectSet: unsafe extern "system" fn(this: *mut *mut Self, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ObjectSet: usize,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, iflags: i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemRefresher {
    pub base__: super::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, punk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, iindex: i32, objwbemrefreshableitem: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, icount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, objwbemservices: *mut ::core::ffi::c_void, bsinstancepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemrefreshableitem: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Add: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddEnum: unsafe extern "system" fn(this: *mut *mut Self, objwbemservices: *mut ::core::ffi::c_void, bsclassname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemrefreshableitem: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddEnum: usize,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, iindex: i32, iflags: i32) -> ::windows_sys::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self, iflags: i32) -> ::windows_sys::core::HRESULT,
    pub AutoReconnect: unsafe extern "system" fn(this: *mut *mut Self, bcount: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAutoReconnect: unsafe extern "system" fn(this: *mut *mut Self, bcount: i16) -> ::windows_sys::core::HRESULT,
    pub DeleteAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemSecurity {
    pub base__: super::Com::IDispatch,
    pub ImpersonationLevel: unsafe extern "system" fn(this: *mut *mut Self, iimpersonationlevel: *mut WbemImpersonationLevelEnum) -> ::windows_sys::core::HRESULT,
    pub SetImpersonationLevel: unsafe extern "system" fn(this: *mut *mut Self, iimpersonationlevel: WbemImpersonationLevelEnum) -> ::windows_sys::core::HRESULT,
    pub AuthenticationLevel: unsafe extern "system" fn(this: *mut *mut Self, iauthenticationlevel: *mut WbemAuthenticationLevelEnum) -> ::windows_sys::core::HRESULT,
    pub SetAuthenticationLevel: unsafe extern "system" fn(this: *mut *mut Self, iauthenticationlevel: WbemAuthenticationLevelEnum) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Privileges: unsafe extern "system" fn(this: *mut *mut Self, objwbemprivilegeset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Privileges: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemServices {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Get: unsafe extern "system" fn(this: *mut *mut Self, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Get: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetAsync: unsafe extern "system" fn(this: *mut *mut Self, objwbemsink: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetAsync: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Delete: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut *mut Self, objwbemsink: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    DeleteAsync: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub InstancesOf: unsafe extern "system" fn(this: *mut *mut Self, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    InstancesOf: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub InstancesOfAsync: unsafe extern "system" fn(this: *mut *mut Self, objwbemsink: *mut ::core::ffi::c_void, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    InstancesOfAsync: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SubclassesOf: unsafe extern "system" fn(this: *mut *mut Self, strsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SubclassesOf: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SubclassesOfAsync: unsafe extern "system" fn(this: *mut *mut Self, objwbemsink: *mut ::core::ffi::c_void, strsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SubclassesOfAsync: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ExecQuery: unsafe extern "system" fn(this: *mut *mut Self, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ExecQuery: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ExecQueryAsync: unsafe extern "system" fn(this: *mut *mut Self, objwbemsink: *mut ::core::ffi::c_void, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ExecQueryAsync: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AssociatorsOf: unsafe extern "system" fn(this: *mut *mut Self, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strassocclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AssociatorsOf: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AssociatorsOfAsync: unsafe extern "system" fn(
        this: *mut *mut Self,
        objwbemsink: *mut ::core::ffi::c_void,
        strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        strassocclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        strresultrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bclassesonly: i16,
        bschemaonly: i16,
        strrequiredassocqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        iflags: i32,
        objwbemnamedvalueset: *mut ::core::ffi::c_void,
        objwbemasynccontext: *mut ::core::ffi::c_void,
    ) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AssociatorsOfAsync: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ReferencesTo: unsafe extern "system" fn(this: *mut *mut Self, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ReferencesTo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ReferencesToAsync: unsafe extern "system" fn(this: *mut *mut Self, objwbemsink: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ReferencesToAsync: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ExecNotificationQuery: unsafe extern "system" fn(this: *mut *mut Self, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemeventsource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ExecNotificationQuery: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ExecNotificationQueryAsync: unsafe extern "system" fn(this: *mut *mut Self, objwbemsink: *mut ::core::ffi::c_void, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ExecNotificationQueryAsync: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ExecMethod: unsafe extern "system" fn(this: *mut *mut Self, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objwbeminparameters: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemoutparameters: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ExecMethod: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ExecMethodAsync: unsafe extern "system" fn(this: *mut *mut Self, objwbemsink: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objwbeminparameters: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ExecMethodAsync: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Security_: unsafe extern "system" fn(this: *mut *mut Self, objwbemsecurity: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Security_: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemServicesEx {
    pub base__: ISWbemServices,
    #[cfg(feature = "Win32_System_Com")]
    pub Put: unsafe extern "system" fn(this: *mut *mut Self, objwbemobject: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectpath: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Put: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PutAsync: unsafe extern "system" fn(this: *mut *mut Self, objwbemsink: *mut ::core::ffi::c_void, objwbemobject: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PutAsync: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemSink {
    pub base__: super::Com::IDispatch,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemSinkEvents {
    pub base__: super::Com::IDispatch,
}
#[repr(C)]
pub struct IUnsecuredApartment {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateObjectStub: unsafe extern "system" fn(this: *mut *mut Self, pobject: *mut ::core::ffi::c_void, ppstub: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWMIExtension {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub WMIObjectPath: unsafe extern "system" fn(this: *mut *mut Self, strwmiobjectpath: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WMIObjectPath: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWMIObject: unsafe extern "system" fn(this: *mut *mut Self, objwmiobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWMIObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWMIServices: unsafe extern "system" fn(this: *mut *mut Self, objwmiservices: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWMIServices: usize,
}
#[repr(C)]
pub struct IWbemAddressResolution {
    pub base__: ::windows_sys::core::IUnknown,
    pub Resolve: unsafe extern "system" fn(this: *mut *mut Self, wsznamespacepath: ::windows_sys::core::PCWSTR, wszaddresstype: ::windows_sys::core::PWSTR, pdwaddresslength: *mut u32, pabbinaryaddress: *mut *mut u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWbemBackupRestore {
    pub base__: ::windows_sys::core::IUnknown,
    pub Backup: unsafe extern "system" fn(this: *mut *mut Self, strbackuptofile: ::windows_sys::core::PCWSTR, lflags: i32) -> ::windows_sys::core::HRESULT,
    pub Restore: unsafe extern "system" fn(this: *mut *mut Self, strrestorefromfile: ::windows_sys::core::PCWSTR, lflags: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWbemBackupRestoreEx {
    pub base__: IWbemBackupRestore,
    pub Pause: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWbemCallResult {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetResultObject: unsafe extern "system" fn(this: *mut *mut Self, ltimeout: i32, ppresultobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetResultString: unsafe extern "system" fn(this: *mut *mut Self, ltimeout: i32, pstrresultstring: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetResultString: usize,
    pub GetResultServices: unsafe extern "system" fn(this: *mut *mut Self, ltimeout: i32, ppservices: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetCallStatus: unsafe extern "system" fn(this: *mut *mut Self, ltimeout: i32, plstatus: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWbemClassObject {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetQualifierSet: unsafe extern "system" fn(this: *mut *mut Self, ppqualset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Get: unsafe extern "system" fn(this: *mut *mut Self, wszname: ::windows_sys::core::PCWSTR, lflags: i32, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Get: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Put: unsafe extern "system" fn(this: *mut *mut Self, wszname: ::windows_sys::core::PCWSTR, lflags: i32, pval: *const super::Com::VARIANT, r#type: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Put: usize,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self, wszname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetNames: unsafe extern "system" fn(this: *mut *mut Self, wszqualifiername: ::windows_sys::core::PCWSTR, lflags: i32, pqualifierval: *const super::Com::VARIANT, pnames: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetNames: usize,
    pub BeginEnumeration: unsafe extern "system" fn(this: *mut *mut Self, lenumflags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, strname: *mut super::super::Foundation::BSTR, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Next: usize,
    pub EndEnumeration: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetPropertyQualifierSet: unsafe extern "system" fn(this: *mut *mut Self, wszproperty: ::windows_sys::core::PCWSTR, ppqualset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppcopy: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetObjectText: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, pstrobjecttext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetObjectText: usize,
    pub SpawnDerivedClass: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, ppnewclass: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SpawnInstance: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, ppnewinstance: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CompareTo: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, pcompareto: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPropertyOrigin: unsafe extern "system" fn(this: *mut *mut Self, wszname: ::windows_sys::core::PCWSTR, pstrclassname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPropertyOrigin: usize,
    pub InheritsFrom: unsafe extern "system" fn(this: *mut *mut Self, strancestor: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetMethod: unsafe extern "system" fn(this: *mut *mut Self, wszname: ::windows_sys::core::PCWSTR, lflags: i32, ppinsignature: *mut *mut ::core::ffi::c_void, ppoutsignature: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PutMethod: unsafe extern "system" fn(this: *mut *mut Self, wszname: ::windows_sys::core::PCWSTR, lflags: i32, pinsignature: *mut ::core::ffi::c_void, poutsignature: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DeleteMethod: unsafe extern "system" fn(this: *mut *mut Self, wszname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub BeginMethodEnumeration: unsafe extern "system" fn(this: *mut *mut Self, lenumflags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub NextMethod: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, pstrname: *mut super::super::Foundation::BSTR, ppinsignature: *mut *mut ::core::ffi::c_void, ppoutsignature: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NextMethod: usize,
    pub EndMethodEnumeration: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetMethodQualifierSet: unsafe extern "system" fn(this: *mut *mut Self, wszmethod: ::windows_sys::core::PCWSTR, ppqualset: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMethodOrigin: unsafe extern "system" fn(this: *mut *mut Self, wszmethodname: ::windows_sys::core::PCWSTR, pstrclassname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMethodOrigin: usize,
}
#[repr(C)]
pub struct IWbemClientConnectionTransport {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Open: unsafe extern "system" fn(this: *mut *mut Self, straddresstype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, struser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, riid: *const ::windows_sys::core::GUID, pinterface: *mut *mut ::core::ffi::c_void, pcallres: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Open: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OpenAsync: unsafe extern "system" fn(this: *mut *mut Self, straddresstype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, struser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, riid: *const ::windows_sys::core::GUID, presponsehandler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OpenAsync: usize,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, phandler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWbemClientTransport {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub ConnectServer: unsafe extern "system" fn(this: *mut *mut Self, straddresstype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strnetworkresource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, struser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lsecurityflags: i32, strauthority: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pctx: *mut ::core::ffi::c_void, ppnamespace: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConnectServer: usize,
}
#[repr(C)]
pub struct IWbemConfigureRefresher {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddObjectByPath: unsafe extern "system" fn(this: *mut *mut Self, pnamespace: *mut ::core::ffi::c_void, wszpath: ::windows_sys::core::PCWSTR, lflags: i32, pcontext: *mut ::core::ffi::c_void, pprefreshable: *mut *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows_sys::core::HRESULT,
    pub AddObjectByTemplate: unsafe extern "system" fn(this: *mut *mut Self, pnamespace: *mut ::core::ffi::c_void, ptemplate: *mut ::core::ffi::c_void, lflags: i32, pcontext: *mut ::core::ffi::c_void, pprefreshable: *mut *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows_sys::core::HRESULT,
    pub AddRefresher: unsafe extern "system" fn(this: *mut *mut Self, prefresher: *mut ::core::ffi::c_void, lflags: i32, plid: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, lid: i32, lflags: i32) -> ::windows_sys::core::HRESULT,
    pub AddEnum: unsafe extern "system" fn(this: *mut *mut Self, pnamespace: *mut ::core::ffi::c_void, wszclassname: ::windows_sys::core::PCWSTR, lflags: i32, pcontext: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWbemConnectorLogin {
    pub base__: ::windows_sys::core::IUnknown,
    pub ConnectorLogin: unsafe extern "system" fn(this: *mut *mut Self, wsznetworkresource: ::windows_sys::core::PCWSTR, wszpreferredlocale: ::windows_sys::core::PCWSTR, lflags: i32, pctx: *mut ::core::ffi::c_void, riid: *const ::windows_sys::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWbemConstructClassObject {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetInheritanceChain: unsafe extern "system" fn(this: *mut *mut Self, lnumantecedents: i32, awszantecedents: *const ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetPropertyOrigin: unsafe extern "system" fn(this: *mut *mut Self, wszpropertyname: ::windows_sys::core::PCWSTR, loriginindex: i32) -> ::windows_sys::core::HRESULT,
    pub SetMethodOrigin: unsafe extern "system" fn(this: *mut *mut Self, wszmethodname: ::windows_sys::core::PCWSTR, loriginindex: i32) -> ::windows_sys::core::HRESULT,
    pub SetServerNamespace: unsafe extern "system" fn(this: *mut *mut Self, wszserver: ::windows_sys::core::PCWSTR, wsznamespace: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWbemContext {
    pub base__: ::windows_sys::core::IUnknown,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppnewcopy: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNames: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, pnames: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNames: usize,
    pub BeginEnumeration: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, pstrname: *mut super::super::Foundation::BSTR, pvalue: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Next: usize,
    pub EndEnumeration: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, wszname: ::windows_sys::core::PCWSTR, lflags: i32, pvalue: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetValue: unsafe extern "system" fn(this: *mut *mut Self, wszname: ::windows_sys::core::PCWSTR, lflags: i32, pvalue: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetValue: usize,
    pub DeleteValue: unsafe extern "system" fn(this: *mut *mut Self, wszname: ::windows_sys::core::PCWSTR, lflags: i32) -> ::windows_sys::core::HRESULT,
    pub DeleteAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWbemDecoupledBasicEventProvider {
    pub base__: IWbemDecoupledRegistrar,
    pub GetSink: unsafe extern "system" fn(this: *mut *mut Self, a_flags: i32, a_context: *mut ::core::ffi::c_void, a_sink: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetService: unsafe extern "system" fn(this: *mut *mut Self, a_flags: i32, a_context: *mut ::core::ffi::c_void, a_service: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWbemDecoupledRegistrar {
    pub base__: ::windows_sys::core::IUnknown,
    pub Register: unsafe extern "system" fn(this: *mut *mut Self, a_flags: i32, a_context: *mut ::core::ffi::c_void, a_user: ::windows_sys::core::PCWSTR, a_locale: ::windows_sys::core::PCWSTR, a_scope: ::windows_sys::core::PCWSTR, a_registration: ::windows_sys::core::PCWSTR, piunknown: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UnRegister: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWbemEventConsumerProvider {
    pub base__: ::windows_sys::core::IUnknown,
    pub FindConsumer: unsafe extern "system" fn(this: *mut *mut Self, plogicalconsumer: *mut ::core::ffi::c_void, ppconsumer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWbemEventProvider {
    pub base__: ::windows_sys::core::IUnknown,
    pub ProvideEvents: unsafe extern "system" fn(this: *mut *mut Self, psink: *mut ::core::ffi::c_void, lflags: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWbemEventProviderQuerySink {
    pub base__: ::windows_sys::core::IUnknown,
    pub NewQuery: unsafe extern "system" fn(this: *mut *mut Self, dwid: u32, wszquerylanguage: *const u16, wszquery: *const u16) -> ::windows_sys::core::HRESULT,
    pub CancelQuery: unsafe extern "system" fn(this: *mut *mut Self, dwid: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWbemEventProviderSecurity {
    pub base__: ::windows_sys::core::IUnknown,
    pub AccessCheck: unsafe extern "system" fn(this: *mut *mut Self, wszquerylanguage: *const u16, wszquery: *const u16, lsidlength: i32, psid: *const u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWbemEventSink {
    pub base__: IWbemObjectSink,
    pub SetSinkSecurity: unsafe extern "system" fn(this: *mut *mut Self, lsdlength: i32, psd: *const u8) -> ::windows_sys::core::HRESULT,
    pub IsActive: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetRestrictedSink: unsafe extern "system" fn(this: *mut *mut Self, lnumqueries: i32, awszqueries: *const ::windows_sys::core::PWSTR, pcallback: *mut ::core::ffi::c_void, ppsink: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBatchingParameters: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, dwmaxbuffersize: u32, dwmaxsendlatency: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWbemHiPerfEnum {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddObjects: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, unumobjects: u32, apids: *const i32, apobj: *const *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveObjects: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, unumobjects: u32, apids: *const i32) -> ::windows_sys::core::HRESULT,
    pub GetObjects: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, unumobjects: u32, apobj: *mut *mut ::core::ffi::c_void, pureturned: *mut u32) -> ::windows_sys::core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWbemHiPerfProvider {
    pub base__: ::windows_sys::core::IUnknown,
    pub QueryInstances: unsafe extern "system" fn(this: *mut *mut Self, pnamespace: *mut ::core::ffi::c_void, wszclass: ::windows_sys::core::PCWSTR, lflags: i32, pctx: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateRefresher: unsafe extern "system" fn(this: *mut *mut Self, pnamespace: *mut ::core::ffi::c_void, lflags: i32, pprefresher: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateRefreshableObject: unsafe extern "system" fn(this: *mut *mut Self, pnamespace: *mut ::core::ffi::c_void, ptemplate: *mut ::core::ffi::c_void, prefresher: *mut ::core::ffi::c_void, lflags: i32, pcontext: *mut ::core::ffi::c_void, pprefreshable: *mut *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows_sys::core::HRESULT,
    pub StopRefreshing: unsafe extern "system" fn(this: *mut *mut Self, prefresher: *mut ::core::ffi::c_void, lid: i32, lflags: i32) -> ::windows_sys::core::HRESULT,
    pub CreateRefreshableEnum: unsafe extern "system" fn(this: *mut *mut Self, pnamespace: *mut ::core::ffi::c_void, wszclass: ::windows_sys::core::PCWSTR, prefresher: *mut ::core::ffi::c_void, lflags: i32, pcontext: *mut ::core::ffi::c_void, phiperfenum: *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetObjects: unsafe extern "system" fn(this: *mut *mut Self, pnamespace: *mut ::core::ffi::c_void, lnumobjects: i32, apobj: *mut *mut ::core::ffi::c_void, lflags: i32, pcontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWbemLevel1Login {
    pub base__: ::windows_sys::core::IUnknown,
    pub EstablishPosition: unsafe extern "system" fn(this: *mut *mut Self, wszlocalelist: ::windows_sys::core::PCWSTR, dwnumlocales: u32, reserved: *mut u32) -> ::windows_sys::core::HRESULT,
    pub RequestChallenge: unsafe extern "system" fn(this: *mut *mut Self, wsznetworkresource: ::windows_sys::core::PCWSTR, wszuser: ::windows_sys::core::PCWSTR, nonce: *mut u8) -> ::windows_sys::core::HRESULT,
    pub WBEMLogin: unsafe extern "system" fn(this: *mut *mut Self, wszpreferredlocale: ::windows_sys::core::PCWSTR, accesstoken: *const u8, lflags: i32, pctx: *mut ::core::ffi::c_void, ppnamespace: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NTLMLogin: unsafe extern "system" fn(this: *mut *mut Self, wsznetworkresource: ::windows_sys::core::PCWSTR, wszpreferredlocale: ::windows_sys::core::PCWSTR, lflags: i32, pctx: *mut ::core::ffi::c_void, ppnamespace: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWbemLocator {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub ConnectServer: unsafe extern "system" fn(this: *mut *mut Self, strnetworkresource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, struser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lsecurityflags: i32, strauthority: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pctx: *mut ::core::ffi::c_void, ppnamespace: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConnectServer: usize,
}
#[repr(C)]
pub struct IWbemObjectAccess {
    pub base__: IWbemClassObject,
    pub GetPropertyHandle: unsafe extern "system" fn(this: *mut *mut Self, wszpropertyname: ::windows_sys::core::PCWSTR, ptype: *mut i32, plhandle: *mut i32) -> ::windows_sys::core::HRESULT,
    pub WritePropertyValue: unsafe extern "system" fn(this: *mut *mut Self, lhandle: i32, lnumbytes: i32, adata: *const u8) -> ::windows_sys::core::HRESULT,
    pub ReadPropertyValue: unsafe extern "system" fn(this: *mut *mut Self, lhandle: i32, lbuffersize: i32, plnumbytes: *mut i32, adata: *mut u8) -> ::windows_sys::core::HRESULT,
    pub ReadDWORD: unsafe extern "system" fn(this: *mut *mut Self, lhandle: i32, pdw: *mut u32) -> ::windows_sys::core::HRESULT,
    pub WriteDWORD: unsafe extern "system" fn(this: *mut *mut Self, lhandle: i32, dw: u32) -> ::windows_sys::core::HRESULT,
    pub ReadQWORD: unsafe extern "system" fn(this: *mut *mut Self, lhandle: i32, pqw: *mut u64) -> ::windows_sys::core::HRESULT,
    pub WriteQWORD: unsafe extern "system" fn(this: *mut *mut Self, lhandle: i32, pw: u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPropertyInfoByHandle: unsafe extern "system" fn(this: *mut *mut Self, lhandle: i32, pstrname: *mut super::super::Foundation::BSTR, ptype: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPropertyInfoByHandle: usize,
    pub Lock: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32) -> ::windows_sys::core::HRESULT,
    pub Unlock: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWbemObjectSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub Indicate: unsafe extern "system" fn(this: *mut *mut Self, lobjectcount: i32, apobjarray: *const *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetStatus: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, hresult: ::windows_sys::core::HRESULT, strparam: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pobjparam: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetStatus: usize,
}
#[repr(C)]
pub struct IWbemObjectSinkEx {
    pub base__: IWbemObjectSink,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteMessage: unsafe extern "system" fn(this: *mut *mut Self, uchannel: u32, strmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteMessage: usize,
    pub WriteError: unsafe extern "system" fn(this: *mut *mut Self, pobjerror: *mut ::core::ffi::c_void, pureturned: *mut u8) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PromptUser: unsafe extern "system" fn(this: *mut *mut Self, strmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, uprompttype: u8, pureturned: *mut u8) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PromptUser: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteProgress: unsafe extern "system" fn(this: *mut *mut Self, stractivity: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strcurrentoperation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strstatusdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, upercentcomplete: u32, usecondsremaining: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteProgress: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub WriteStreamParameter: unsafe extern "system" fn(this: *mut *mut Self, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vtvalue: *const super::Com::VARIANT, ultype: u32, ulflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    WriteStreamParameter: usize,
}
#[repr(C)]
pub struct IWbemObjectTextSrc {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetText: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, pobj: *mut ::core::ffi::c_void, uobjtextformat: u32, pctx: *mut ::core::ffi::c_void, strtext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateFromText: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, strtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, uobjtextformat: u32, pctx: *mut ::core::ffi::c_void, pnewobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateFromText: usize,
}
#[repr(C)]
pub struct IWbemPath {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, umode: u32, pszpath: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetText: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, pubufflength: *mut u32, psztext: ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetInfo: unsafe extern "system" fn(this: *mut *mut Self, urequestedinfo: u32, puresponse: *mut u64) -> ::windows_sys::core::HRESULT,
    pub SetServer: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetServer: unsafe extern "system" fn(this: *mut *mut Self, punamebuflength: *mut u32, pname: ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetNamespaceCount: unsafe extern "system" fn(this: *mut *mut Self, pucount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetNamespaceAt: unsafe extern "system" fn(this: *mut *mut Self, uindex: u32, pszname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetNamespaceAt: unsafe extern "system" fn(this: *mut *mut Self, uindex: u32, punamebuflength: *mut u32, pname: ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub RemoveNamespaceAt: unsafe extern "system" fn(this: *mut *mut Self, uindex: u32) -> ::windows_sys::core::HRESULT,
    pub RemoveAllNamespaces: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetScopeCount: unsafe extern "system" fn(this: *mut *mut Self, pucount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetScope: unsafe extern "system" fn(this: *mut *mut Self, uindex: u32, pszclass: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SetScopeFromText: unsafe extern "system" fn(this: *mut *mut Self, uindex: u32, psztext: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetScope: unsafe extern "system" fn(this: *mut *mut Self, uindex: u32, puclassnamebufsize: *mut u32, pszclass: ::windows_sys::core::PWSTR, pkeylist: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetScopeAsText: unsafe extern "system" fn(this: *mut *mut Self, uindex: u32, putextbufsize: *mut u32, psztext: ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub RemoveScope: unsafe extern "system" fn(this: *mut *mut Self, uindex: u32) -> ::windows_sys::core::HRESULT,
    pub RemoveAllScopes: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetClassName: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetClassName: unsafe extern "system" fn(this: *mut *mut Self, pubufflength: *mut u32, pszname: ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetKeyList: unsafe extern "system" fn(this: *mut *mut Self, pout: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateClassPart: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, name: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub DeleteClassPart: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRelative: unsafe extern "system" fn(this: *mut *mut Self, wszmachine: ::windows_sys::core::PCWSTR, wsznamespace: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRelative: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRelativeOrChild: unsafe extern "system" fn(this: *mut *mut Self, wszmachine: ::windows_sys::core::PCWSTR, wsznamespace: ::windows_sys::core::PCWSTR, lflags: i32) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRelativeOrChild: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsLocal: unsafe extern "system" fn(this: *mut *mut Self, wszmachine: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsLocal: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSameClassName: unsafe extern "system" fn(this: *mut *mut Self, wszclass: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSameClassName: usize,
}
#[repr(C)]
pub struct IWbemPathKeyList {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, pukeycount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetKey: unsafe extern "system" fn(this: *mut *mut Self, wszname: ::windows_sys::core::PCWSTR, uflags: u32, ucimtype: u32, pkeyval: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetKey2: unsafe extern "system" fn(this: *mut *mut Self, wszname: ::windows_sys::core::PCWSTR, uflags: u32, ucimtype: u32, pkeyval: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetKey2: usize,
    pub GetKey: unsafe extern "system" fn(this: *mut *mut Self, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: ::windows_sys::core::PWSTR, pukeyvalbufsize: *mut u32, pkeyval: *mut ::core::ffi::c_void, puapparentcimtype: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetKey2: unsafe extern "system" fn(this: *mut *mut Self, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: ::windows_sys::core::PWSTR, pkeyvalue: *mut super::Com::VARIANT, puapparentcimtype: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetKey2: usize,
    pub RemoveKey: unsafe extern "system" fn(this: *mut *mut Self, wszname: ::windows_sys::core::PCWSTR, uflags: u32) -> ::windows_sys::core::HRESULT,
    pub RemoveAllKeys: unsafe extern "system" fn(this: *mut *mut Self, uflags: u32) -> ::windows_sys::core::HRESULT,
    pub MakeSingleton: unsafe extern "system" fn(this: *mut *mut Self, bset: u8) -> ::windows_sys::core::HRESULT,
    pub GetInfo: unsafe extern "system" fn(this: *mut *mut Self, urequestedinfo: u32, puresponse: *mut u64) -> ::windows_sys::core::HRESULT,
    pub GetText: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, pubufflength: *mut u32, psztext: ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWbemPropertyProvider {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strclassmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strinstmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpropmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvvalue: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PutProperty: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strclassmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strinstmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpropmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvvalue: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PutProperty: usize,
}
#[repr(C)]
pub struct IWbemProviderIdentity {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetRegistrationObject: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, pprovreg: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWbemProviderInit {
    pub base__: ::windows_sys::core::IUnknown,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, wszuser: ::windows_sys::core::PCWSTR, lflags: i32, wsznamespace: ::windows_sys::core::PCWSTR, wszlocale: ::windows_sys::core::PCWSTR, pnamespace: *mut ::core::ffi::c_void, pctx: *mut ::core::ffi::c_void, pinitsink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWbemProviderInitSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetStatus: unsafe extern "system" fn(this: *mut *mut Self, lstatus: i32, lflags: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWbemQualifierSet {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Get: unsafe extern "system" fn(this: *mut *mut Self, wszname: ::windows_sys::core::PCWSTR, lflags: i32, pval: *mut super::Com::VARIANT, plflavor: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Get: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Put: unsafe extern "system" fn(this: *mut *mut Self, wszname: ::windows_sys::core::PCWSTR, pval: *const super::Com::VARIANT, lflavor: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Put: usize,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self, wszname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNames: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, pnames: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNames: usize,
    pub BeginEnumeration: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, pstrname: *mut super::super::Foundation::BSTR, pval: *mut super::Com::VARIANT, plflavor: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Next: usize,
    pub EndEnumeration: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWbemQuery {
    pub base__: ::windows_sys::core::IUnknown,
    pub Empty: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetLanguageFeatures: unsafe extern "system" fn(this: *mut *mut Self, uflags: u32, uarraysize: u32, pufeatures: *const u32) -> ::windows_sys::core::HRESULT,
    pub TestLanguageFeatures: unsafe extern "system" fn(this: *mut *mut Self, uflags: u32, uarraysize: *mut u32, pufeatures: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, pszlang: ::windows_sys::core::PCWSTR, pszquery: ::windows_sys::core::PCWSTR, uflags: u32) -> ::windows_sys::core::HRESULT,
    pub GetAnalysis: unsafe extern "system" fn(this: *mut *mut Self, uanalysistype: u32, uflags: u32, panalysis: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FreeMemory: unsafe extern "system" fn(this: *mut *mut Self, pmem: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetQueryInfo: unsafe extern "system" fn(this: *mut *mut Self, uanalysistype: u32, uinfoid: u32, ubufsize: u32, pdestbuf: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWbemRefresher {
    pub base__: ::windows_sys::core::IUnknown,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWbemServices {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub OpenNamespace: unsafe extern "system" fn(this: *mut *mut Self, strnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, ppworkingnamespace: *mut *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OpenNamespace: usize,
    pub CancelAsyncCall: unsafe extern "system" fn(this: *mut *mut Self, psink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub QueryObjectSink: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32, ppresponsehandler: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetObject: unsafe extern "system" fn(this: *mut *mut Self, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, ppobject: *mut *mut ::core::ffi::c_void, ppcallresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetObject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetObjectAsync: unsafe extern "system" fn(this: *mut *mut Self, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetObjectAsync: usize,
    pub PutClass: unsafe extern "system" fn(this: *mut *mut Self, pobject: *mut ::core::ffi::c_void, lflags: i32, pctx: *mut ::core::ffi::c_void, ppcallresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PutClassAsync: unsafe extern "system" fn(this: *mut *mut Self, pobject: *mut ::core::ffi::c_void, lflags: i32, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteClass: unsafe extern "system" fn(this: *mut *mut Self, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, ppcallresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteClass: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteClassAsync: unsafe extern "system" fn(this: *mut *mut Self, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteClassAsync: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateClassEnum: unsafe extern "system" fn(this: *mut *mut Self, strsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateClassEnum: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateClassEnumAsync: unsafe extern "system" fn(this: *mut *mut Self, strsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateClassEnumAsync: usize,
    pub PutInstance: unsafe extern "system" fn(this: *mut *mut Self, pinst: *mut ::core::ffi::c_void, lflags: i32, pctx: *mut ::core::ffi::c_void, ppcallresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PutInstanceAsync: unsafe extern "system" fn(this: *mut *mut Self, pinst: *mut ::core::ffi::c_void, lflags: i32, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteInstance: unsafe extern "system" fn(this: *mut *mut Self, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, ppcallresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteInstance: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteInstanceAsync: unsafe extern "system" fn(this: *mut *mut Self, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteInstanceAsync: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateInstanceEnum: unsafe extern "system" fn(this: *mut *mut Self, strfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateInstanceEnum: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateInstanceEnumAsync: unsafe extern "system" fn(this: *mut *mut Self, strfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateInstanceEnumAsync: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExecQuery: unsafe extern "system" fn(this: *mut *mut Self, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExecQuery: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExecQueryAsync: unsafe extern "system" fn(this: *mut *mut Self, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExecQueryAsync: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExecNotificationQuery: unsafe extern "system" fn(this: *mut *mut Self, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExecNotificationQuery: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExecNotificationQueryAsync: unsafe extern "system" fn(this: *mut *mut Self, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExecNotificationQueryAsync: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExecMethod: unsafe extern "system" fn(this: *mut *mut Self, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, pinparams: *mut ::core::ffi::c_void, ppoutparams: *mut *mut ::core::ffi::c_void, ppcallresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExecMethod: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExecMethodAsync: unsafe extern "system" fn(this: *mut *mut Self, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, pinparams: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExecMethodAsync: usize,
}
#[repr(C)]
pub struct IWbemShutdown {
    pub base__: ::windows_sys::core::IUnknown,
    pub Shutdown: unsafe extern "system" fn(this: *mut *mut Self, ureason: i32, umaxmilliseconds: u32, pctx: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWbemStatusCodeText {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetErrorCodeText: unsafe extern "system" fn(this: *mut *mut Self, hres: ::windows_sys::core::HRESULT, localeid: u32, lflags: i32, messagetext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetErrorCodeText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFacilityCodeText: unsafe extern "system" fn(this: *mut *mut Self, hres: ::windows_sys::core::HRESULT, localeid: u32, lflags: i32, messagetext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFacilityCodeText: usize,
}
#[repr(C)]
pub struct IWbemTransport {
    pub base__: ::windows_sys::core::IUnknown,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWbemUnboundObjectSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub IndicateToConsumer: unsafe extern "system" fn(this: *mut *mut Self, plogicalconsumer: *mut ::core::ffi::c_void, lnumobjects: i32, apobjects: *const *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWbemUnsecuredApartment {
    pub base__: IUnsecuredApartment,
    pub CreateSinkStub: unsafe extern "system" fn(this: *mut *mut Self, psink: *mut ::core::ffi::c_void, dwflags: u32, wszreserved: ::windows_sys::core::PCWSTR, ppstub: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Application {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_ApplicationFT,
}
impl ::core::marker::Copy for MI_Application {}
impl ::core::clone::Clone for MI_Application {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ApplicationFT {
    pub Close: isize,
    pub NewSession: isize,
    pub NewHostedProvider: isize,
    pub NewInstance: isize,
    pub NewDestinationOptions: isize,
    pub NewOperationOptions: isize,
    pub NewSubscriptionDeliveryOptions: isize,
    pub NewSerializer: isize,
    pub NewDeserializer: isize,
    pub NewInstanceFromClass: isize,
    pub NewClass: isize,
}
impl ::core::marker::Copy for MI_ApplicationFT {}
impl ::core::clone::Clone for MI_ApplicationFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Array {
    pub data: *mut ::core::ffi::c_void,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Array {}
impl ::core::clone::Clone for MI_Array {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ArrayField {
    pub value: MI_Array,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ArrayField {}
impl ::core::clone::Clone for MI_ArrayField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_BooleanA {
    pub data: *mut u8,
    pub size: u32,
}
impl ::core::marker::Copy for MI_BooleanA {}
impl ::core::clone::Clone for MI_BooleanA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_BooleanAField {
    pub value: MI_BooleanA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_BooleanAField {}
impl ::core::clone::Clone for MI_BooleanAField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_BooleanField {
    pub value: u8,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_BooleanField {}
impl ::core::clone::Clone for MI_BooleanField {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_CALL_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_CHAR_TYPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_CallbackMode = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_CALLBACKMODE_REPORT: MI_CallbackMode = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_CALLBACKMODE_INQUIRE: MI_CallbackMode = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_CALLBACKMODE_IGNORE: MI_CallbackMode = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_CancelCallback = ::core::option::Option<unsafe extern "system" fn(reason: MI_CancellationReason, callbackdata: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_CancellationReason = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_REASON_NONE: MI_CancellationReason = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_REASON_TIMEOUT: MI_CancellationReason = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_REASON_SHUTDOWN: MI_CancellationReason = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_REASON_SERVICESTOP: MI_CancellationReason = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Char16A {
    pub data: *mut u16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Char16A {}
impl ::core::clone::Clone for MI_Char16A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Char16AField {
    pub value: MI_Char16A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Char16AField {}
impl ::core::clone::Clone for MI_Char16AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Char16Field {
    pub value: u16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Char16Field {}
impl ::core::clone::Clone for MI_Char16Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Class {
    pub ft: *const MI_ClassFT,
    pub classDecl: *const MI_ClassDecl,
    pub namespaceName: *const u16,
    pub serverName: *const u16,
    pub reserved: [isize; 4],
}
impl ::core::marker::Copy for MI_Class {}
impl ::core::clone::Clone for MI_Class {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ClassDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *const u16,
    pub qualifiers: *const *const MI_Qualifier,
    pub numQualifiers: u32,
    pub properties: *const *const MI_PropertyDecl,
    pub numProperties: u32,
    pub size: u32,
    pub superClass: *const u16,
    pub superClassDecl: *const MI_ClassDecl,
    pub methods: *const *const MI_MethodDecl,
    pub numMethods: u32,
    pub schema: *const MI_SchemaDecl,
    pub providerFT: *const MI_ProviderFT,
    pub owningClass: *mut MI_Class,
}
impl ::core::marker::Copy for MI_ClassDecl {}
impl ::core::clone::Clone for MI_ClassDecl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ClassFT {
    pub GetClassNameA: isize,
    pub GetNameSpace: isize,
    pub GetServerName: isize,
    pub GetElementCount: isize,
    pub GetElement: isize,
    pub GetElementAt: isize,
    pub GetClassQualifierSet: isize,
    pub GetMethodCount: isize,
    pub GetMethodAt: isize,
    pub GetMethod: isize,
    pub GetParentClassName: isize,
    pub GetParentClass: isize,
    pub Delete: isize,
    pub Clone: isize,
}
impl ::core::marker::Copy for MI_ClassFT {}
impl ::core::clone::Clone for MI_ClassFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ClientFT_V1 {
    pub applicationFT: *const MI_ApplicationFT,
    pub sessionFT: *const MI_SessionFT,
    pub operationFT: *const MI_OperationFT,
    pub hostedProviderFT: *const MI_HostedProviderFT,
    pub serializerFT: *const MI_SerializerFT,
    pub deserializerFT: *const MI_DeserializerFT,
    pub subscribeDeliveryOptionsFT: *const MI_SubscriptionDeliveryOptionsFT,
    pub destinationOptionsFT: *const MI_DestinationOptionsFT,
    pub operationOptionsFT: *const MI_OperationOptionsFT,
    pub utilitiesFT: *const MI_UtilitiesFT,
}
impl ::core::marker::Copy for MI_ClientFT_V1 {}
impl ::core::clone::Clone for MI_ClientFT_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstBooleanA {
    pub data: *const u8,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstBooleanA {}
impl ::core::clone::Clone for MI_ConstBooleanA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstBooleanAField {
    pub value: MI_ConstBooleanA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstBooleanAField {}
impl ::core::clone::Clone for MI_ConstBooleanAField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstBooleanField {
    pub value: u8,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstBooleanField {}
impl ::core::clone::Clone for MI_ConstBooleanField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstChar16A {
    pub data: *const u16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstChar16A {}
impl ::core::clone::Clone for MI_ConstChar16A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstChar16AField {
    pub value: MI_ConstChar16A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstChar16AField {}
impl ::core::clone::Clone for MI_ConstChar16AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstChar16Field {
    pub value: u16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstChar16Field {}
impl ::core::clone::Clone for MI_ConstChar16Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstDatetimeA {
    pub data: *const MI_Datetime,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstDatetimeA {}
impl ::core::clone::Clone for MI_ConstDatetimeA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstDatetimeAField {
    pub value: MI_ConstDatetimeA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstDatetimeAField {}
impl ::core::clone::Clone for MI_ConstDatetimeAField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstDatetimeField {
    pub value: MI_Datetime,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstDatetimeField {}
impl ::core::clone::Clone for MI_ConstDatetimeField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstInstanceA {
    pub data: *const *const MI_Instance,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstInstanceA {}
impl ::core::clone::Clone for MI_ConstInstanceA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstInstanceAField {
    pub value: MI_ConstInstanceA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstInstanceAField {}
impl ::core::clone::Clone for MI_ConstInstanceAField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstInstanceField {
    pub value: *const MI_Instance,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstInstanceField {}
impl ::core::clone::Clone for MI_ConstInstanceField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstReal32A {
    pub data: *const f32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstReal32A {}
impl ::core::clone::Clone for MI_ConstReal32A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstReal32AField {
    pub value: MI_ConstReal32A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstReal32AField {}
impl ::core::clone::Clone for MI_ConstReal32AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstReal32Field {
    pub value: f32,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstReal32Field {}
impl ::core::clone::Clone for MI_ConstReal32Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstReal64A {
    pub data: *const f64,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstReal64A {}
impl ::core::clone::Clone for MI_ConstReal64A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstReal64AField {
    pub value: MI_ConstReal64A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstReal64AField {}
impl ::core::clone::Clone for MI_ConstReal64AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstReal64Field {
    pub value: f64,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstReal64Field {}
impl ::core::clone::Clone for MI_ConstReal64Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstReferenceA {
    pub data: *const *const MI_Instance,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstReferenceA {}
impl ::core::clone::Clone for MI_ConstReferenceA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstReferenceAField {
    pub value: MI_ConstReferenceA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstReferenceAField {}
impl ::core::clone::Clone for MI_ConstReferenceAField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstReferenceField {
    pub value: *const MI_Instance,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstReferenceField {}
impl ::core::clone::Clone for MI_ConstReferenceField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstSint16A {
    pub data: *const i16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstSint16A {}
impl ::core::clone::Clone for MI_ConstSint16A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstSint16AField {
    pub value: MI_ConstSint16A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint16AField {}
impl ::core::clone::Clone for MI_ConstSint16AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstSint16Field {
    pub value: i16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint16Field {}
impl ::core::clone::Clone for MI_ConstSint16Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstSint32A {
    pub data: *const i32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstSint32A {}
impl ::core::clone::Clone for MI_ConstSint32A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstSint32AField {
    pub value: MI_ConstSint32A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint32AField {}
impl ::core::clone::Clone for MI_ConstSint32AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstSint32Field {
    pub value: i32,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint32Field {}
impl ::core::clone::Clone for MI_ConstSint32Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstSint64A {
    pub data: *const i64,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstSint64A {}
impl ::core::clone::Clone for MI_ConstSint64A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstSint64AField {
    pub value: MI_ConstSint64A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint64AField {}
impl ::core::clone::Clone for MI_ConstSint64AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstSint64Field {
    pub value: i64,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint64Field {}
impl ::core::clone::Clone for MI_ConstSint64Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstSint8A {
    pub data: *const i8,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstSint8A {}
impl ::core::clone::Clone for MI_ConstSint8A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstSint8AField {
    pub value: MI_ConstSint8A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint8AField {}
impl ::core::clone::Clone for MI_ConstSint8AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstSint8Field {
    pub value: i8,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint8Field {}
impl ::core::clone::Clone for MI_ConstSint8Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstStringA {
    pub data: *const *const u16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstStringA {}
impl ::core::clone::Clone for MI_ConstStringA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstStringAField {
    pub value: MI_ConstStringA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstStringAField {}
impl ::core::clone::Clone for MI_ConstStringAField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstStringField {
    pub value: *const u16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstStringField {}
impl ::core::clone::Clone for MI_ConstStringField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstUint16A {
    pub data: *const u16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstUint16A {}
impl ::core::clone::Clone for MI_ConstUint16A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstUint16AField {
    pub value: MI_ConstUint16A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint16AField {}
impl ::core::clone::Clone for MI_ConstUint16AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstUint16Field {
    pub value: u16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint16Field {}
impl ::core::clone::Clone for MI_ConstUint16Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstUint32A {
    pub data: *const u32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstUint32A {}
impl ::core::clone::Clone for MI_ConstUint32A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstUint32AField {
    pub value: MI_ConstUint32A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint32AField {}
impl ::core::clone::Clone for MI_ConstUint32AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstUint32Field {
    pub value: u32,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint32Field {}
impl ::core::clone::Clone for MI_ConstUint32Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstUint64A {
    pub data: *const u64,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstUint64A {}
impl ::core::clone::Clone for MI_ConstUint64A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstUint64AField {
    pub value: MI_ConstUint64A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint64AField {}
impl ::core::clone::Clone for MI_ConstUint64AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstUint64Field {
    pub value: u64,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint64Field {}
impl ::core::clone::Clone for MI_ConstUint64Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstUint8A {
    pub data: *const u8,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstUint8A {}
impl ::core::clone::Clone for MI_ConstUint8A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstUint8AField {
    pub value: MI_ConstUint8A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint8AField {}
impl ::core::clone::Clone for MI_ConstUint8AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstUint8Field {
    pub value: u8,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint8Field {}
impl ::core::clone::Clone for MI_ConstUint8Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Context {
    pub ft: *const MI_ContextFT,
    pub reserved: [isize; 3],
}
impl ::core::marker::Copy for MI_Context {}
impl ::core::clone::Clone for MI_Context {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ContextFT {
    pub PostResult: isize,
    pub PostInstance: isize,
    pub PostIndication: isize,
    pub ConstructInstance: isize,
    pub ConstructParameters: isize,
    pub NewInstance: isize,
    pub NewDynamicInstance: isize,
    pub NewParameters: isize,
    pub Canceled: isize,
    pub GetLocale: isize,
    pub RegisterCancel: isize,
    pub RequestUnload: isize,
    pub RefuseUnload: isize,
    pub GetLocalSession: isize,
    pub SetStringOption: isize,
    pub GetStringOption: isize,
    pub GetNumberOption: isize,
    pub GetCustomOption: isize,
    pub GetCustomOptionCount: isize,
    pub GetCustomOptionAt: isize,
    pub WriteMessage: isize,
    pub WriteProgress: isize,
    pub WriteStreamParameter: isize,
    pub WriteCimError: isize,
    pub PromptUser: isize,
    pub ShouldProcess: isize,
    pub ShouldContinue: isize,
    pub PostError: isize,
    pub PostCimError: isize,
    pub WriteError: isize,
}
impl ::core::marker::Copy for MI_ContextFT {}
impl ::core::clone::Clone for MI_ContextFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Datetime {
    pub isTimestamp: u32,
    pub u: MI_Datetime_0,
}
impl ::core::marker::Copy for MI_Datetime {}
impl ::core::clone::Clone for MI_Datetime {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub union MI_Datetime_0 {
    pub timestamp: MI_Timestamp,
    pub interval: MI_Interval,
}
impl ::core::marker::Copy for MI_Datetime_0 {}
impl ::core::clone::Clone for MI_Datetime_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_DatetimeA {
    pub data: *mut MI_Datetime,
    pub size: u32,
}
impl ::core::marker::Copy for MI_DatetimeA {}
impl ::core::clone::Clone for MI_DatetimeA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_DatetimeAField {
    pub value: MI_DatetimeA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_DatetimeAField {}
impl ::core::clone::Clone for MI_DatetimeAField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_DatetimeField {
    pub value: MI_Datetime,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_DatetimeField {}
impl ::core::clone::Clone for MI_DatetimeField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Deserializer {
    pub reserved1: u64,
    pub reserved2: isize,
}
impl ::core::marker::Copy for MI_Deserializer {}
impl ::core::clone::Clone for MI_Deserializer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_DeserializerFT {
    pub Close: isize,
    pub DeserializeClass: isize,
    pub Class_GetClassName: isize,
    pub Class_GetParentClassName: isize,
    pub DeserializeInstance: isize,
    pub Instance_GetClassName: isize,
}
impl ::core::marker::Copy for MI_DeserializerFT {}
impl ::core::clone::Clone for MI_DeserializerFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_Deserializer_ClassObjectNeeded = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, servername: *const u16, namespacename: *const u16, classname: *const u16, requestedclassobject: *mut *mut MI_Class) -> MI_Result>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_DestinationOptions {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_DestinationOptionsFT,
}
impl ::core::marker::Copy for MI_DestinationOptions {}
impl ::core::clone::Clone for MI_DestinationOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_DestinationOptionsFT {
    pub Delete: isize,
    pub SetString: isize,
    pub SetNumber: isize,
    pub AddCredentials: isize,
    pub GetString: isize,
    pub GetNumber: isize,
    pub GetOptionCount: isize,
    pub GetOptionAt: isize,
    pub GetOption: isize,
    pub GetCredentialsCount: isize,
    pub GetCredentialsAt: isize,
    pub GetCredentialsPasswordAt: isize,
    pub Clone: isize,
    pub SetInterval: isize,
    pub GetInterval: isize,
}
impl ::core::marker::Copy for MI_DestinationOptionsFT {}
impl ::core::clone::Clone for MI_DestinationOptionsFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_DestinationOptions_ImpersonationType = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_DestinationOptions_ImpersonationType_Default: MI_DestinationOptions_ImpersonationType = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_DestinationOptions_ImpersonationType_None: MI_DestinationOptions_ImpersonationType = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_DestinationOptions_ImpersonationType_Identify: MI_DestinationOptions_ImpersonationType = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_DestinationOptions_ImpersonationType_Impersonate: MI_DestinationOptions_ImpersonationType = 3i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_DestinationOptions_ImpersonationType_Delegate: MI_DestinationOptions_ImpersonationType = 4i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_ErrorCategory = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_NOT_SPECIFIED: MI_ErrorCategory = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_OPEN_ERROR: MI_ErrorCategory = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_CLOS_EERROR: MI_ErrorCategory = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_DEVICE_ERROR: MI_ErrorCategory = 3i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_DEADLOCK_DETECTED: MI_ErrorCategory = 4i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_INVALID_ARGUMENT: MI_ErrorCategory = 5i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_INVALID_DATA: MI_ErrorCategory = 6i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_INVALID_OPERATION: MI_ErrorCategory = 7i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_INVALID_RESULT: MI_ErrorCategory = 8i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_INVALID_TYPE: MI_ErrorCategory = 9i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_METADATA_ERROR: MI_ErrorCategory = 10i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_NOT_IMPLEMENTED: MI_ErrorCategory = 11i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_NOT_INSTALLED: MI_ErrorCategory = 12i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_OBJECT_NOT_FOUND: MI_ErrorCategory = 13i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_OPERATION_STOPPED: MI_ErrorCategory = 14i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_OPERATION_TIMEOUT: MI_ErrorCategory = 15i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_SYNTAX_ERROR: MI_ErrorCategory = 16i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_PARSER_ERROR: MI_ErrorCategory = 17i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_ACCESS_DENIED: MI_ErrorCategory = 18i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_RESOURCE_BUSY: MI_ErrorCategory = 19i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_RESOURCE_EXISTS: MI_ErrorCategory = 20i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_RESOURCE_UNAVAILABLE: MI_ErrorCategory = 21i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_READ_ERROR: MI_ErrorCategory = 22i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_WRITE_ERROR: MI_ErrorCategory = 23i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_FROM_STDERR: MI_ErrorCategory = 24i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_SECURITY_ERROR: MI_ErrorCategory = 25i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_PROTOCOL_ERROR: MI_ErrorCategory = 26i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_CONNECTION_ERROR: MI_ErrorCategory = 27i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_AUTHENTICATION_ERROR: MI_ErrorCategory = 28i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_LIMITS_EXCEEDED: MI_ErrorCategory = 29i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_QUOTA_EXCEEDED: MI_ErrorCategory = 30i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_NOT_ENABLED: MI_ErrorCategory = 31i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_ABSTRACT: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_ADOPT: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_ANY: u32 = 127u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_ASSOCIATION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_BORROW: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_CLASS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_DISABLEOVERRIDE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_ENABLEOVERRIDE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_EXPENSIVE: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_EXTENDED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_IN: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_INDICATION: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_KEY: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_METHOD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_NOT_MODIFIED: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_NULL: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_OUT: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_PARAMETER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_PROPERTY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_READONLY: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_REFERENCE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_REQUIRED: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_RESTRICTED: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_STATIC: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_STREAM: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_TERMINAL: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_TOSUBCLASS: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_TRANSLATABLE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_VERSION: u32 = 469762048u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_FeatureDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *const u16,
    pub qualifiers: *const *const MI_Qualifier,
    pub numQualifiers: u32,
}
impl ::core::marker::Copy for MI_FeatureDecl {}
impl ::core::clone::Clone for MI_FeatureDecl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Filter {
    pub ft: *const MI_FilterFT,
    pub reserved: [isize; 3],
}
impl ::core::marker::Copy for MI_Filter {}
impl ::core::clone::Clone for MI_Filter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_FilterFT {
    pub Evaluate: isize,
    pub GetExpression: isize,
}
impl ::core::marker::Copy for MI_FilterFT {}
impl ::core::clone::Clone for MI_FilterFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_HostedProvider {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_HostedProviderFT,
}
impl ::core::marker::Copy for MI_HostedProvider {}
impl ::core::clone::Clone for MI_HostedProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_HostedProviderFT {
    pub Close: isize,
    pub GetApplication: isize,
}
impl ::core::marker::Copy for MI_HostedProviderFT {}
impl ::core::clone::Clone for MI_HostedProviderFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Instance {
    pub ft: *const MI_InstanceFT,
    pub classDecl: *const MI_ClassDecl,
    pub serverName: *const u16,
    pub nameSpace: *const u16,
    pub reserved: [isize; 4],
}
impl ::core::marker::Copy for MI_Instance {}
impl ::core::clone::Clone for MI_Instance {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_InstanceA {
    pub data: *mut *mut MI_Instance,
    pub size: u32,
}
impl ::core::marker::Copy for MI_InstanceA {}
impl ::core::clone::Clone for MI_InstanceA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_InstanceAField {
    pub value: MI_InstanceA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_InstanceAField {}
impl ::core::clone::Clone for MI_InstanceAField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_InstanceExFT {
    pub parent: MI_InstanceFT,
    pub Normalize: isize,
}
impl ::core::marker::Copy for MI_InstanceExFT {}
impl ::core::clone::Clone for MI_InstanceExFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_InstanceFT {
    pub Clone: isize,
    pub Destruct: isize,
    pub Delete: isize,
    pub IsA: isize,
    pub GetClassNameA: isize,
    pub SetNameSpace: isize,
    pub GetNameSpace: isize,
    pub GetElementCount: isize,
    pub AddElement: isize,
    pub SetElement: isize,
    pub SetElementAt: isize,
    pub GetElement: isize,
    pub GetElementAt: isize,
    pub ClearElement: isize,
    pub ClearElementAt: isize,
    pub GetServerName: isize,
    pub SetServerName: isize,
    pub GetClass: isize,
}
impl ::core::marker::Copy for MI_InstanceFT {}
impl ::core::clone::Clone for MI_InstanceFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_InstanceField {
    pub value: *mut MI_Instance,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_InstanceField {}
impl ::core::clone::Clone for MI_InstanceField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Interval {
    pub days: u32,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub microseconds: u32,
    pub __padding1: u32,
    pub __padding2: u32,
    pub __padding3: u32,
}
impl ::core::marker::Copy for MI_Interval {}
impl ::core::clone::Clone for MI_Interval {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_LocaleType = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_LOCALE_TYPE_REQUESTED_UI: MI_LocaleType = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_LOCALE_TYPE_REQUESTED_DATA: MI_LocaleType = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_LOCALE_TYPE_CLOSEST_UI: MI_LocaleType = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_LOCALE_TYPE_CLOSEST_DATA: MI_LocaleType = 3i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_MAX_LOCALE_SIZE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_MODULE_FLAG_BOOLEANS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_MODULE_FLAG_CPLUSPLUS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_MODULE_FLAG_DESCRIPTIONS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_MODULE_FLAG_FILTER_SUPPORT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_MODULE_FLAG_LOCALIZED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_MODULE_FLAG_MAPPING_STRINGS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_MODULE_FLAG_STANDARD_QUALIFIERS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_MODULE_FLAG_VALUES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_MainFunction = ::core::option::Option<unsafe extern "system" fn(server: *mut MI_Server) -> *mut MI_Module>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_MethodDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *const u16,
    pub qualifiers: *const *const MI_Qualifier,
    pub numQualifiers: u32,
    pub parameters: *const *const MI_ParameterDecl,
    pub numParameters: u32,
    pub size: u32,
    pub returnType: u32,
    pub origin: *const u16,
    pub propagator: *const u16,
    pub schema: *const MI_SchemaDecl,
    pub function: MI_MethodDecl_Invoke,
}
impl ::core::marker::Copy for MI_MethodDecl {}
impl ::core::clone::Clone for MI_MethodDecl {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_MethodDecl_Invoke = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, methodname: *const u16, instancename: *const MI_Instance, parameters: *const MI_Instance)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Module {
    pub version: u32,
    pub generatorVersion: u32,
    pub flags: u32,
    pub charSize: u32,
    pub schemaDecl: *mut MI_SchemaDecl,
    pub Load: MI_Module_Load,
    pub Unload: MI_Module_Unload,
    pub dynamicProviderFT: *const MI_ProviderFT,
}
impl ::core::marker::Copy for MI_Module {}
impl ::core::clone::Clone for MI_Module {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_Module_Load = ::core::option::Option<unsafe extern "system" fn(self_: *mut *mut MI_Module_Self, context: *const MI_Context)>;
#[repr(C)]
pub struct MI_Module_Self(pub u8);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_Module_Unload = ::core::option::Option<unsafe extern "system" fn(self_: *const MI_Module_Self, context: *const MI_Context)>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_OPERATIONFLAGS_BASIC_RTTI: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_OPERATIONFLAGS_DEFAULT_RTTI: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_OPERATIONFLAGS_EXPENSIVE_PROPERTIES: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_OPERATIONFLAGS_FULL_RTTI: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_OPERATIONFLAGS_LOCALIZED_QUALIFIERS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_OPERATIONFLAGS_MANUAL_ACK_RESULTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_OPERATIONFLAGS_NO_RTTI: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_OPERATIONFLAGS_POLYMORPHISM_DEEP_BASE_PROPS_ONLY: u32 = 384u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_OPERATIONFLAGS_POLYMORPHISM_SHALLOW: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_OPERATIONFLAGS_REPORT_OPERATION_STARTED: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_OPERATIONFLAGS_STANDARD_RTTI: u32 = 2048u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ObjectDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *const u16,
    pub qualifiers: *const *const MI_Qualifier,
    pub numQualifiers: u32,
    pub properties: *const *const MI_PropertyDecl,
    pub numProperties: u32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ObjectDecl {}
impl ::core::clone::Clone for MI_ObjectDecl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Operation {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_OperationFT,
}
impl ::core::marker::Copy for MI_Operation {}
impl ::core::clone::Clone for MI_Operation {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_OperationCallback_Class = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, classresult: *const MI_Class, moreresults: u8, resultcode: MI_Result, errorstring: *const u16, errordetails: *const MI_Instance, resultacknowledgement: isize)>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_OperationCallback_Indication = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, instance: *const MI_Instance, bookmark: *const u16, machineid: *const u16, moreresults: u8, resultcode: MI_Result, errorstring: *const u16, errordetails: *const MI_Instance, resultacknowledgement: isize)>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_OperationCallback_Instance = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, instance: *const MI_Instance, moreresults: u8, resultcode: MI_Result, errorstring: *const u16, errordetails: *const MI_Instance, resultacknowledgement: isize)>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_OperationCallback_PromptUser = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, message: *const u16, prompttype: MI_PromptType, promptuserresult: isize)>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_OperationCallback_ResponseType = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_OperationCallback_ResponseType_No: MI_OperationCallback_ResponseType = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_OperationCallback_ResponseType_Yes: MI_OperationCallback_ResponseType = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_OperationCallback_ResponseType_NoToAll: MI_OperationCallback_ResponseType = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_OperationCallback_ResponseType_YesToAll: MI_OperationCallback_ResponseType = 3i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_OperationCallback_StreamedParameter = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, parametername: *const u16, resulttype: MI_Type, result: *const MI_Value, resultacknowledgement: isize)>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_OperationCallback_WriteError = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, instance: *const MI_Instance, writeerrorresult: isize)>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_OperationCallback_WriteMessage = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, channel: u32, message: *const u16)>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_OperationCallback_WriteProgress = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, activity: *const u16, currentoperation: *const u16, statusdescription: *const u16, percentagecomplete: u32, secondsremaining: u32)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_OperationCallbacks {
    pub callbackContext: *mut ::core::ffi::c_void,
    pub promptUser: MI_OperationCallback_PromptUser,
    pub writeError: MI_OperationCallback_WriteError,
    pub writeMessage: MI_OperationCallback_WriteMessage,
    pub writeProgress: MI_OperationCallback_WriteProgress,
    pub instanceResult: MI_OperationCallback_Instance,
    pub indicationResult: MI_OperationCallback_Indication,
    pub classResult: MI_OperationCallback_Class,
    pub streamedParameterResult: MI_OperationCallback_StreamedParameter,
}
impl ::core::marker::Copy for MI_OperationCallbacks {}
impl ::core::clone::Clone for MI_OperationCallbacks {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_OperationFT {
    pub Close: isize,
    pub Cancel: isize,
    pub GetSession: isize,
    pub GetInstance: isize,
    pub GetIndication: isize,
    pub GetClass: isize,
}
impl ::core::marker::Copy for MI_OperationFT {}
impl ::core::clone::Clone for MI_OperationFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_OperationOptions {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_OperationOptionsFT,
}
impl ::core::marker::Copy for MI_OperationOptions {}
impl ::core::clone::Clone for MI_OperationOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_OperationOptionsFT {
    pub Delete: isize,
    pub SetString: isize,
    pub SetNumber: isize,
    pub SetCustomOption: isize,
    pub GetString: isize,
    pub GetNumber: isize,
    pub GetOptionCount: isize,
    pub GetOptionAt: isize,
    pub GetOption: isize,
    pub GetEnabledChannels: isize,
    pub Clone: isize,
    pub SetInterval: isize,
    pub GetInterval: isize,
}
impl ::core::marker::Copy for MI_OperationOptionsFT {}
impl ::core::clone::Clone for MI_OperationOptionsFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ParameterDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *const u16,
    pub qualifiers: *const *const MI_Qualifier,
    pub numQualifiers: u32,
    pub r#type: u32,
    pub className: *const u16,
    pub subscript: u32,
    pub offset: u32,
}
impl ::core::marker::Copy for MI_ParameterDecl {}
impl ::core::clone::Clone for MI_ParameterDecl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ParameterSet {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_ParameterSetFT,
}
impl ::core::marker::Copy for MI_ParameterSet {}
impl ::core::clone::Clone for MI_ParameterSet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ParameterSetFT {
    pub GetMethodReturnType: isize,
    pub GetParameterCount: isize,
    pub GetParameterAt: isize,
    pub GetParameter: isize,
}
impl ::core::marker::Copy for MI_ParameterSetFT {}
impl ::core::clone::Clone for MI_ParameterSetFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_PromptType = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_PROMPTTYPE_NORMAL: MI_PromptType = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_PROMPTTYPE_CRITICAL: MI_PromptType = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_PropertyDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *const u16,
    pub qualifiers: *const *const MI_Qualifier,
    pub numQualifiers: u32,
    pub r#type: u32,
    pub className: *const u16,
    pub subscript: u32,
    pub offset: u32,
    pub origin: *const u16,
    pub propagator: *const u16,
    pub value: *const ::core::ffi::c_void,
}
impl ::core::marker::Copy for MI_PropertyDecl {}
impl ::core::clone::Clone for MI_PropertyDecl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_PropertySet {
    pub ft: *const MI_PropertySetFT,
    pub reserved: [isize; 3],
}
impl ::core::marker::Copy for MI_PropertySet {}
impl ::core::clone::Clone for MI_PropertySet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_PropertySetFT {
    pub GetElementCount: isize,
    pub ContainsElement: isize,
    pub AddElement: isize,
    pub GetElementAt: isize,
    pub Clear: isize,
    pub Destruct: isize,
    pub Delete: isize,
    pub Clone: isize,
}
impl ::core::marker::Copy for MI_PropertySetFT {}
impl ::core::clone::Clone for MI_PropertySetFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_ProviderArchitecture = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_PROVIDER_ARCHITECTURE_32BIT: MI_ProviderArchitecture = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_PROVIDER_ARCHITECTURE_64BIT: MI_ProviderArchitecture = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ProviderFT {
    pub Load: MI_ProviderFT_Load,
    pub Unload: MI_ProviderFT_Unload,
    pub GetInstance: MI_ProviderFT_GetInstance,
    pub EnumerateInstances: MI_ProviderFT_EnumerateInstances,
    pub CreateInstance: MI_ProviderFT_CreateInstance,
    pub ModifyInstance: MI_ProviderFT_ModifyInstance,
    pub DeleteInstance: MI_ProviderFT_DeleteInstance,
    pub AssociatorInstances: MI_ProviderFT_AssociatorInstances,
    pub ReferenceInstances: MI_ProviderFT_ReferenceInstances,
    pub EnableIndications: MI_ProviderFT_EnableIndications,
    pub DisableIndications: MI_ProviderFT_DisableIndications,
    pub Subscribe: MI_ProviderFT_Subscribe,
    pub Unsubscribe: MI_ProviderFT_Unsubscribe,
    pub Invoke: MI_ProviderFT_Invoke,
}
impl ::core::marker::Copy for MI_ProviderFT {}
impl ::core::clone::Clone for MI_ProviderFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_ProviderFT_AssociatorInstances = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, instancename: *const MI_Instance, resultclass: *const u16, role: *const u16, resultrole: *const u16, propertyset: *const MI_PropertySet, keysonly: u8, filter: *const MI_Filter)>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_ProviderFT_CreateInstance = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, newinstance: *const MI_Instance)>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_ProviderFT_DeleteInstance = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, instancename: *const MI_Instance)>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_ProviderFT_DisableIndications = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, indicationscontext: *const MI_Context, namespace: *const u16, classname: *const u16)>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_ProviderFT_EnableIndications = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, indicationscontext: *const MI_Context, namespace: *const u16, classname: *const u16)>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_ProviderFT_EnumerateInstances = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, propertyset: *const MI_PropertySet, keysonly: u8, filter: *const MI_Filter)>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_ProviderFT_GetInstance = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, instancename: *const MI_Instance, propertyset: *const MI_PropertySet)>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_ProviderFT_Invoke = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, methodname: *const u16, instancename: *const MI_Instance, inputparameters: *const MI_Instance)>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_ProviderFT_Load = ::core::option::Option<unsafe extern "system" fn(self_: *mut *mut ::core::ffi::c_void, selfmodule: *const MI_Module_Self, context: *const MI_Context)>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_ProviderFT_ModifyInstance = ::core::option::Option<unsafe extern "system" fn(self_: *mut ::core::ffi::c_void, context: *mut MI_Context, namespace: *const u16, classname: *const u16, modifiedinstance: *const MI_Instance, propertyset: *const MI_PropertySet)>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_ProviderFT_ReferenceInstances = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, instancename: *const MI_Instance, role: *const u16, propertyset: *const MI_PropertySet, keysonly: u8, filter: *const MI_Filter)>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_ProviderFT_Subscribe = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, filter: *const MI_Filter, bookmark: *const u16, subscriptionid: u64, subscriptionself: *mut *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_ProviderFT_Unload = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context)>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_ProviderFT_Unsubscribe = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, subscriptionid: u64, subscriptionself: *const ::core::ffi::c_void)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Qualifier {
    pub name: *const u16,
    pub r#type: u32,
    pub flavor: u32,
    pub value: *const ::core::ffi::c_void,
}
impl ::core::marker::Copy for MI_Qualifier {}
impl ::core::clone::Clone for MI_Qualifier {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_QualifierDecl {
    pub name: *const u16,
    pub r#type: u32,
    pub scope: u32,
    pub flavor: u32,
    pub subscript: u32,
    pub value: *const ::core::ffi::c_void,
}
impl ::core::marker::Copy for MI_QualifierDecl {}
impl ::core::clone::Clone for MI_QualifierDecl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_QualifierSet {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_QualifierSetFT,
}
impl ::core::marker::Copy for MI_QualifierSet {}
impl ::core::clone::Clone for MI_QualifierSet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_QualifierSetFT {
    pub GetQualifierCount: isize,
    pub GetQualifierAt: isize,
    pub GetQualifier: isize,
}
impl ::core::marker::Copy for MI_QualifierSetFT {}
impl ::core::clone::Clone for MI_QualifierSetFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Real32A {
    pub data: *mut f32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Real32A {}
impl ::core::clone::Clone for MI_Real32A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Real32AField {
    pub value: MI_Real32A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Real32AField {}
impl ::core::clone::Clone for MI_Real32AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Real32Field {
    pub value: f32,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Real32Field {}
impl ::core::clone::Clone for MI_Real32Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Real64A {
    pub data: *mut f64,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Real64A {}
impl ::core::clone::Clone for MI_Real64A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Real64AField {
    pub value: MI_Real64A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Real64AField {}
impl ::core::clone::Clone for MI_Real64AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Real64Field {
    pub value: f64,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Real64Field {}
impl ::core::clone::Clone for MI_Real64Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ReferenceA {
    pub data: *mut *mut MI_Instance,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ReferenceA {}
impl ::core::clone::Clone for MI_ReferenceA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ReferenceAField {
    pub value: MI_ReferenceA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ReferenceAField {}
impl ::core::clone::Clone for MI_ReferenceAField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ReferenceField {
    pub value: *mut MI_Instance,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ReferenceField {}
impl ::core::clone::Clone for MI_ReferenceField {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_Result = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_OK: MI_Result = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_FAILED: MI_Result = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_ACCESS_DENIED: MI_Result = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_INVALID_NAMESPACE: MI_Result = 3i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_INVALID_PARAMETER: MI_Result = 4i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_INVALID_CLASS: MI_Result = 5i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_NOT_FOUND: MI_Result = 6i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_NOT_SUPPORTED: MI_Result = 7i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_CLASS_HAS_CHILDREN: MI_Result = 8i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_CLASS_HAS_INSTANCES: MI_Result = 9i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_INVALID_SUPERCLASS: MI_Result = 10i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_ALREADY_EXISTS: MI_Result = 11i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_NO_SUCH_PROPERTY: MI_Result = 12i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_TYPE_MISMATCH: MI_Result = 13i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_QUERY_LANGUAGE_NOT_SUPPORTED: MI_Result = 14i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_INVALID_QUERY: MI_Result = 15i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_METHOD_NOT_AVAILABLE: MI_Result = 16i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_METHOD_NOT_FOUND: MI_Result = 17i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_NAMESPACE_NOT_EMPTY: MI_Result = 20i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_INVALID_ENUMERATION_CONTEXT: MI_Result = 21i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_INVALID_OPERATION_TIMEOUT: MI_Result = 22i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_PULL_HAS_BEEN_ABANDONED: MI_Result = 23i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_PULL_CANNOT_BE_ABANDONED: MI_Result = 24i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_FILTERED_ENUMERATION_NOT_SUPPORTED: MI_Result = 25i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_CONTINUATION_ON_ERROR_NOT_SUPPORTED: MI_Result = 26i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_SERVER_LIMITS_EXCEEDED: MI_Result = 27i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_SERVER_IS_SHUTTING_DOWN: MI_Result = 28i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_SERIALIZER_FLAGS_CLASS_DEEP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_SERIALIZER_FLAGS_INSTANCE_WITH_CLASS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_SUBSCRIBE_BOOKMARK_NEWEST: &str = "MI_SUBSCRIBE_BOOKMARK_NEWEST";
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_SUBSCRIBE_BOOKMARK_OLDEST: &str = "MI_SUBSCRIBE_BOOKMARK_OLDEST";
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_SchemaDecl {
    pub qualifierDecls: *const *const MI_QualifierDecl,
    pub numQualifierDecls: u32,
    pub classDecls: *const *const MI_ClassDecl,
    pub numClassDecls: u32,
}
impl ::core::marker::Copy for MI_SchemaDecl {}
impl ::core::clone::Clone for MI_SchemaDecl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Serializer {
    pub reserved1: u64,
    pub reserved2: isize,
}
impl ::core::marker::Copy for MI_Serializer {}
impl ::core::clone::Clone for MI_Serializer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_SerializerFT {
    pub Close: isize,
    pub SerializeClass: isize,
    pub SerializeInstance: isize,
}
impl ::core::marker::Copy for MI_SerializerFT {}
impl ::core::clone::Clone for MI_SerializerFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Server {
    pub serverFT: *const MI_ServerFT,
    pub contextFT: *const MI_ContextFT,
    pub instanceFT: *const MI_InstanceFT,
    pub propertySetFT: *const MI_PropertySetFT,
    pub filterFT: *const MI_FilterFT,
}
impl ::core::marker::Copy for MI_Server {}
impl ::core::clone::Clone for MI_Server {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ServerFT {
    pub GetVersion: isize,
    pub GetSystemName: isize,
}
impl ::core::marker::Copy for MI_ServerFT {}
impl ::core::clone::Clone for MI_ServerFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Session {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_SessionFT,
}
impl ::core::marker::Copy for MI_Session {}
impl ::core::clone::Clone for MI_Session {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_SessionCallbacks {
    pub callbackContext: *mut ::core::ffi::c_void,
    pub writeMessage: isize,
    pub writeError: isize,
}
impl ::core::marker::Copy for MI_SessionCallbacks {}
impl ::core::clone::Clone for MI_SessionCallbacks {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_SessionFT {
    pub Close: isize,
    pub GetApplication: isize,
    pub GetInstance: isize,
    pub ModifyInstance: isize,
    pub CreateInstance: isize,
    pub DeleteInstance: isize,
    pub Invoke: isize,
    pub EnumerateInstances: isize,
    pub QueryInstances: isize,
    pub AssociatorInstances: isize,
    pub ReferenceInstances: isize,
    pub Subscribe: isize,
    pub GetClass: isize,
    pub EnumerateClasses: isize,
    pub TestConnection: isize,
}
impl ::core::marker::Copy for MI_SessionFT {}
impl ::core::clone::Clone for MI_SessionFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Sint16A {
    pub data: *mut i16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Sint16A {}
impl ::core::clone::Clone for MI_Sint16A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Sint16AField {
    pub value: MI_Sint16A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint16AField {}
impl ::core::clone::Clone for MI_Sint16AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Sint16Field {
    pub value: i16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint16Field {}
impl ::core::clone::Clone for MI_Sint16Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Sint32A {
    pub data: *mut i32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Sint32A {}
impl ::core::clone::Clone for MI_Sint32A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Sint32AField {
    pub value: MI_Sint32A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint32AField {}
impl ::core::clone::Clone for MI_Sint32AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Sint32Field {
    pub value: i32,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint32Field {}
impl ::core::clone::Clone for MI_Sint32Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Sint64A {
    pub data: *mut i64,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Sint64A {}
impl ::core::clone::Clone for MI_Sint64A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Sint64AField {
    pub value: MI_Sint64A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint64AField {}
impl ::core::clone::Clone for MI_Sint64AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Sint64Field {
    pub value: i64,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint64Field {}
impl ::core::clone::Clone for MI_Sint64Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Sint8A {
    pub data: *mut i8,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Sint8A {}
impl ::core::clone::Clone for MI_Sint8A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Sint8AField {
    pub value: MI_Sint8A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint8AField {}
impl ::core::clone::Clone for MI_Sint8AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Sint8Field {
    pub value: i8,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint8Field {}
impl ::core::clone::Clone for MI_Sint8Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_StringA {
    pub data: *mut *mut u16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_StringA {}
impl ::core::clone::Clone for MI_StringA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_StringAField {
    pub value: MI_StringA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_StringAField {}
impl ::core::clone::Clone for MI_StringAField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_StringField {
    pub value: *mut u16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_StringField {}
impl ::core::clone::Clone for MI_StringField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_SubscriptionDeliveryOptions {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_SubscriptionDeliveryOptionsFT,
}
impl ::core::marker::Copy for MI_SubscriptionDeliveryOptions {}
impl ::core::clone::Clone for MI_SubscriptionDeliveryOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_SubscriptionDeliveryOptionsFT {
    pub SetString: isize,
    pub SetNumber: isize,
    pub SetDateTime: isize,
    pub SetInterval: isize,
    pub AddCredentials: isize,
    pub Delete: isize,
    pub GetString: isize,
    pub GetNumber: isize,
    pub GetDateTime: isize,
    pub GetInterval: isize,
    pub GetOptionCount: isize,
    pub GetOptionAt: isize,
    pub GetOption: isize,
    pub GetCredentialsCount: isize,
    pub GetCredentialsAt: isize,
    pub GetCredentialsPasswordAt: isize,
    pub Clone: isize,
}
impl ::core::marker::Copy for MI_SubscriptionDeliveryOptionsFT {}
impl ::core::clone::Clone for MI_SubscriptionDeliveryOptionsFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_SubscriptionDeliveryType = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_SubscriptionDeliveryType_Pull: MI_SubscriptionDeliveryType = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_SubscriptionDeliveryType_Push: MI_SubscriptionDeliveryType = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Timestamp {
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub microseconds: u32,
    pub utc: i32,
}
impl ::core::marker::Copy for MI_Timestamp {}
impl ::core::clone::Clone for MI_Timestamp {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_Type = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_BOOLEAN: MI_Type = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_UINT8: MI_Type = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_SINT8: MI_Type = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_UINT16: MI_Type = 3i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_SINT16: MI_Type = 4i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_UINT32: MI_Type = 5i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_SINT32: MI_Type = 6i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_UINT64: MI_Type = 7i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_SINT64: MI_Type = 8i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_REAL32: MI_Type = 9i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_REAL64: MI_Type = 10i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_CHAR16: MI_Type = 11i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_DATETIME: MI_Type = 12i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_STRING: MI_Type = 13i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_REFERENCE: MI_Type = 14i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_INSTANCE: MI_Type = 15i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_BOOLEANA: MI_Type = 16i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_UINT8A: MI_Type = 17i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_SINT8A: MI_Type = 18i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_UINT16A: MI_Type = 19i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_SINT16A: MI_Type = 20i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_UINT32A: MI_Type = 21i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_SINT32A: MI_Type = 22i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_UINT64A: MI_Type = 23i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_SINT64A: MI_Type = 24i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_REAL32A: MI_Type = 25i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_REAL64A: MI_Type = 26i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_CHAR16A: MI_Type = 27i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_DATETIMEA: MI_Type = 28i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_STRINGA: MI_Type = 29i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_REFERENCEA: MI_Type = 30i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_INSTANCEA: MI_Type = 31i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ARRAY: MI_Type = 16i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Uint16A {
    pub data: *mut u16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Uint16A {}
impl ::core::clone::Clone for MI_Uint16A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Uint16AField {
    pub value: MI_Uint16A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint16AField {}
impl ::core::clone::Clone for MI_Uint16AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Uint16Field {
    pub value: u16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint16Field {}
impl ::core::clone::Clone for MI_Uint16Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Uint32A {
    pub data: *mut u32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Uint32A {}
impl ::core::clone::Clone for MI_Uint32A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Uint32AField {
    pub value: MI_Uint32A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint32AField {}
impl ::core::clone::Clone for MI_Uint32AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Uint32Field {
    pub value: u32,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint32Field {}
impl ::core::clone::Clone for MI_Uint32Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Uint64A {
    pub data: *mut u64,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Uint64A {}
impl ::core::clone::Clone for MI_Uint64A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Uint64AField {
    pub value: MI_Uint64A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint64AField {}
impl ::core::clone::Clone for MI_Uint64AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Uint64Field {
    pub value: u64,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint64Field {}
impl ::core::clone::Clone for MI_Uint64Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Uint8A {
    pub data: *mut u8,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Uint8A {}
impl ::core::clone::Clone for MI_Uint8A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Uint8AField {
    pub value: MI_Uint8A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint8AField {}
impl ::core::clone::Clone for MI_Uint8AField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Uint8Field {
    pub value: u8,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint8Field {}
impl ::core::clone::Clone for MI_Uint8Field {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_UserCredentials {
    pub authenticationType: *const u16,
    pub credentials: MI_UserCredentials_0,
}
impl ::core::marker::Copy for MI_UserCredentials {}
impl ::core::clone::Clone for MI_UserCredentials {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub union MI_UserCredentials_0 {
    pub usernamePassword: MI_UsernamePasswordCreds,
    pub certificateThumbprint: *const u16,
}
impl ::core::marker::Copy for MI_UserCredentials_0 {}
impl ::core::clone::Clone for MI_UserCredentials_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_UsernamePasswordCreds {
    pub domain: *const u16,
    pub username: *const u16,
    pub password: *const u16,
}
impl ::core::marker::Copy for MI_UsernamePasswordCreds {}
impl ::core::clone::Clone for MI_UsernamePasswordCreds {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_UtilitiesFT {
    pub MapErrorToMiErrorCategory: isize,
    pub CimErrorFromErrorCode: isize,
}
impl ::core::marker::Copy for MI_UtilitiesFT {}
impl ::core::clone::Clone for MI_UtilitiesFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub union MI_Value {
    pub boolean: u8,
    pub uint8: u8,
    pub sint8: i8,
    pub uint16: u16,
    pub sint16: i16,
    pub uint32: u32,
    pub sint32: i32,
    pub uint64: u64,
    pub sint64: i64,
    pub real32: f32,
    pub real64: f64,
    pub char16: u16,
    pub datetime: MI_Datetime,
    pub string: *mut u16,
    pub instance: *mut MI_Instance,
    pub reference: *mut MI_Instance,
    pub booleana: MI_BooleanA,
    pub uint8a: MI_Uint8A,
    pub sint8a: MI_Sint8A,
    pub uint16a: MI_Uint16A,
    pub sint16a: MI_Sint16A,
    pub uint32a: MI_Uint32A,
    pub sint32a: MI_Sint32A,
    pub uint64a: MI_Uint64A,
    pub sint64a: MI_Sint64A,
    pub real32a: MI_Real32A,
    pub real64a: MI_Real64A,
    pub char16a: MI_Char16A,
    pub datetimea: MI_DatetimeA,
    pub stringa: MI_StringA,
    pub referencea: MI_ReferenceA,
    pub instancea: MI_InstanceA,
    pub array: MI_Array,
}
impl ::core::marker::Copy for MI_Value {}
impl ::core::clone::Clone for MI_Value {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_WRITEMESSAGE_CHANNEL_DEBUG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_WRITEMESSAGE_CHANNEL_VERBOSE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_WRITEMESSAGE_CHANNEL_WARNING: u32 = 0u32;
pub const MofCompiler: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1840224087, data2: 11831, data3: 4562, data4: [174, 201, 0, 192, 79, 182, 136, 32] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SWbemAnalysisMatrix {
    pub m_uVersion: u32,
    pub m_uMatrixType: u32,
    pub m_pszProperty: ::windows_sys::core::PCWSTR,
    pub m_uPropertyType: u32,
    pub m_uEntries: u32,
    pub m_pValues: *mut *mut ::core::ffi::c_void,
    pub m_pbTruthTable: *mut super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SWbemAnalysisMatrix {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SWbemAnalysisMatrix {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SWbemAnalysisMatrixList {
    pub m_uVersion: u32,
    pub m_uMatrixType: u32,
    pub m_uNumMatrices: u32,
    pub m_pMatrices: *mut SWbemAnalysisMatrix,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SWbemAnalysisMatrixList {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SWbemAnalysisMatrixList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct SWbemAssocQueryInf {
    pub m_uVersion: u32,
    pub m_uAnalysisType: u32,
    pub m_uFeatureMask: u32,
    pub m_pPath: *mut *mut *mut *mut IWbemPath,
    pub m_pszPath: ::windows_sys::core::PWSTR,
    pub m_pszQueryText: ::windows_sys::core::PWSTR,
    pub m_pszResultClass: ::windows_sys::core::PWSTR,
    pub m_pszAssocClass: ::windows_sys::core::PWSTR,
    pub m_pszRole: ::windows_sys::core::PWSTR,
    pub m_pszResultRole: ::windows_sys::core::PWSTR,
    pub m_pszRequiredQualifier: ::windows_sys::core::PWSTR,
    pub m_pszRequiredAssocQualifier: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for SWbemAssocQueryInf {}
impl ::core::clone::Clone for SWbemAssocQueryInf {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SWbemDateTime: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1205845588, data2: 53110, data3: 4563, data4: [179, 143, 0, 16, 90, 31, 71, 58] };
pub const SWbemEventSource: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 79183192, data2: 8622, data3: 4562, data4: [139, 51, 0, 96, 8, 6, 217, 182] };
pub const SWbemLastError: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3271487148, data2: 53197, data3: 4561, data4: [139, 5, 0, 96, 8, 6, 217, 182] };
pub const SWbemLocator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1990607192, data2: 52033, data3: 4561, data4: [139, 2, 0, 96, 8, 6, 217, 182] };
pub const SWbemMethod: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 79183195, data2: 8622, data3: 4562, data4: [139, 51, 0, 96, 8, 6, 217, 182] };
pub const SWbemMethodSet: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 79183194, data2: 8622, data3: 4562, data4: [139, 51, 0, 96, 8, 6, 217, 182] };
pub const SWbemNamedValue: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 79183200, data2: 8622, data3: 4562, data4: [139, 51, 0, 96, 8, 6, 217, 182] };
pub const SWbemNamedValueSet: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2599237710, data2: 52875, data3: 4561, data4: [139, 5, 0, 96, 8, 6, 217, 182] };
pub const SWbemObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 79183202, data2: 8622, data3: 4562, data4: [139, 51, 0, 96, 8, 6, 217, 182] };
pub const SWbemObjectEx: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3602755506, data2: 37941, data3: 18719, data4: [187, 135, 106, 160, 240, 188, 49, 162] };
pub const SWbemObjectPath: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1469168678, data2: 52892, data3: 4561, data4: [151, 191, 0, 0, 248, 30, 132, 156] };
pub const SWbemObjectSet: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 79183201, data2: 8622, data3: 4562, data4: [139, 51, 0, 96, 8, 6, 217, 182] };
pub const SWbemPrivilege: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 653158332, data2: 22532, data3: 4562, data4: [139, 74, 0, 96, 8, 6, 217, 182] };
pub const SWbemPrivilegeSet: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 653158334, data2: 22532, data3: 4562, data4: [139, 74, 0, 96, 8, 6, 217, 182] };
pub const SWbemProperty: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 79183197, data2: 8622, data3: 4562, data4: [139, 51, 0, 96, 8, 6, 217, 182] };
pub const SWbemPropertySet: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 79183196, data2: 8622, data3: 4562, data4: [139, 51, 0, 96, 8, 6, 217, 182] };
pub const SWbemQualifier: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 79183199, data2: 8622, data3: 4562, data4: [139, 51, 0, 96, 8, 6, 217, 182] };
pub const SWbemQualifierSet: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 79183198, data2: 8622, data3: 4562, data4: [139, 51, 0, 96, 8, 6, 217, 182] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SWbemQueryQualifiedName {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uNameListSize: u32,
    pub m_ppszNameList: *mut ::windows_sys::core::PWSTR,
    pub m_bArraysUsed: super::super::Foundation::BOOL,
    pub m_pbArrayElUsed: *mut super::super::Foundation::BOOL,
    pub m_puArrayIndex: *mut u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SWbemQueryQualifiedName {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SWbemQueryQualifiedName {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SWbemRefreshableItem: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2355647676, data2: 56907, data3: 4563, data4: [179, 144, 0, 16, 90, 31, 71, 58] };
pub const SWbemRefresher: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3530145628, data2: 55745, data3: 4563, data4: [179, 143, 0, 16, 90, 31, 71, 58] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union SWbemRpnConst {
    pub m_pszStrVal: ::windows_sys::core::PCWSTR,
    pub m_bBoolVal: super::super::Foundation::BOOL,
    pub m_lLongVal: i32,
    pub m_uLongVal: u32,
    pub m_dblVal: f64,
    pub m_lVal64: i64,
    pub m_uVal64: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SWbemRpnConst {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SWbemRpnConst {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SWbemRpnEncodedQuery {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uParsedFeatureMask: u64,
    pub m_uDetectedArraySize: u32,
    pub m_puDetectedFeatures: *mut u32,
    pub m_uSelectListSize: u32,
    pub m_ppSelectList: *mut *mut SWbemQueryQualifiedName,
    pub m_uFromTargetType: u32,
    pub m_pszOptionalFromPath: ::windows_sys::core::PCWSTR,
    pub m_uFromListSize: u32,
    pub m_ppszFromList: *mut ::windows_sys::core::PWSTR,
    pub m_uWhereClauseSize: u32,
    pub m_ppRpnWhereClause: *mut *mut SWbemRpnQueryToken,
    pub m_dblWithinPolling: f64,
    pub m_dblWithinWindow: f64,
    pub m_uOrderByListSize: u32,
    pub m_ppszOrderByList: *mut ::windows_sys::core::PWSTR,
    pub m_uOrderDirectionEl: *mut u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SWbemRpnEncodedQuery {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SWbemRpnEncodedQuery {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SWbemRpnQueryToken {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uSubexpressionShape: u32,
    pub m_uOperator: u32,
    pub m_pRightIdent: *mut SWbemQueryQualifiedName,
    pub m_pLeftIdent: *mut SWbemQueryQualifiedName,
    pub m_uConstApparentType: u32,
    pub m_Const: SWbemRpnConst,
    pub m_uConst2ApparentType: u32,
    pub m_Const2: SWbemRpnConst,
    pub m_pszRightFunc: ::windows_sys::core::PCWSTR,
    pub m_pszLeftFunc: ::windows_sys::core::PCWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SWbemRpnQueryToken {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SWbemRpnQueryToken {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct SWbemRpnTokenList {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uNumTokens: u32,
}
impl ::core::marker::Copy for SWbemRpnTokenList {}
impl ::core::clone::Clone for SWbemRpnTokenList {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SWbemSecurity: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3041748713, data2: 8839, data3: 4562, data4: [139, 51, 0, 96, 8, 6, 217, 182] };
pub const SWbemServices: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 79183203, data2: 8622, data3: 4562, data4: [139, 51, 0, 96, 8, 6, 217, 182] };
pub const SWbemServicesEx: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1659183836, data2: 36083, data3: 16552, data4: [139, 46, 55, 213, 149, 101, 30, 64] };
pub const SWbemSink: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1970375834, data2: 61481, data3: 4561, data4: [161, 172, 0, 192, 79, 182, 194, 35] };
pub const UnsecuredApartment: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1237131304, data2: 5411, data3: 4561, data4: [173, 121, 0, 192, 79, 216, 253, 255] };
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEMSTATUS = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_NO_ERROR: WBEMSTATUS = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_NO_ERROR: WBEMSTATUS = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_SAME: WBEMSTATUS = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_FALSE: WBEMSTATUS = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_ALREADY_EXISTS: WBEMSTATUS = 262145i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_RESET_TO_DEFAULT: WBEMSTATUS = 262146i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_DIFFERENT: WBEMSTATUS = 262147i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_TIMEDOUT: WBEMSTATUS = 262148i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_NO_MORE_DATA: WBEMSTATUS = 262149i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_OPERATION_CANCELLED: WBEMSTATUS = 262150i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_PENDING: WBEMSTATUS = 262151i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_DUPLICATE_OBJECTS: WBEMSTATUS = 262152i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_ACCESS_DENIED: WBEMSTATUS = 262153i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_PARTIAL_RESULTS: WBEMSTATUS = 262160i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_SOURCE_NOT_AVAILABLE: WBEMSTATUS = 262167i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_FAILED: WBEMSTATUS = -2147217407i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_NOT_FOUND: WBEMSTATUS = -2147217406i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_ACCESS_DENIED: WBEMSTATUS = -2147217405i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PROVIDER_FAILURE: WBEMSTATUS = -2147217404i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_TYPE_MISMATCH: WBEMSTATUS = -2147217403i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_OUT_OF_MEMORY: WBEMSTATUS = -2147217402i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_CONTEXT: WBEMSTATUS = -2147217401i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_PARAMETER: WBEMSTATUS = -2147217400i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_NOT_AVAILABLE: WBEMSTATUS = -2147217399i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_CRITICAL_ERROR: WBEMSTATUS = -2147217398i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_STREAM: WBEMSTATUS = -2147217397i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_NOT_SUPPORTED: WBEMSTATUS = -2147217396i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_SUPERCLASS: WBEMSTATUS = -2147217395i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_NAMESPACE: WBEMSTATUS = -2147217394i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_OBJECT: WBEMSTATUS = -2147217393i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_CLASS: WBEMSTATUS = -2147217392i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PROVIDER_NOT_FOUND: WBEMSTATUS = -2147217391i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_PROVIDER_REGISTRATION: WBEMSTATUS = -2147217390i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PROVIDER_LOAD_FAILURE: WBEMSTATUS = -2147217389i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INITIALIZATION_FAILURE: WBEMSTATUS = -2147217388i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_TRANSPORT_FAILURE: WBEMSTATUS = -2147217387i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_OPERATION: WBEMSTATUS = -2147217386i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_QUERY: WBEMSTATUS = -2147217385i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_QUERY_TYPE: WBEMSTATUS = -2147217384i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_ALREADY_EXISTS: WBEMSTATUS = -2147217383i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_OVERRIDE_NOT_ALLOWED: WBEMSTATUS = -2147217382i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PROPAGATED_QUALIFIER: WBEMSTATUS = -2147217381i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PROPAGATED_PROPERTY: WBEMSTATUS = -2147217380i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_UNEXPECTED: WBEMSTATUS = -2147217379i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_ILLEGAL_OPERATION: WBEMSTATUS = -2147217378i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_CANNOT_BE_KEY: WBEMSTATUS = -2147217377i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INCOMPLETE_CLASS: WBEMSTATUS = -2147217376i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_SYNTAX: WBEMSTATUS = -2147217375i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_NONDECORATED_OBJECT: WBEMSTATUS = -2147217374i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_READ_ONLY: WBEMSTATUS = -2147217373i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PROVIDER_NOT_CAPABLE: WBEMSTATUS = -2147217372i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_CLASS_HAS_CHILDREN: WBEMSTATUS = -2147217371i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_CLASS_HAS_INSTANCES: WBEMSTATUS = -2147217370i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_QUERY_NOT_IMPLEMENTED: WBEMSTATUS = -2147217369i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_ILLEGAL_NULL: WBEMSTATUS = -2147217368i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_QUALIFIER_TYPE: WBEMSTATUS = -2147217367i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_PROPERTY_TYPE: WBEMSTATUS = -2147217366i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_VALUE_OUT_OF_RANGE: WBEMSTATUS = -2147217365i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_CANNOT_BE_SINGLETON: WBEMSTATUS = -2147217364i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_CIM_TYPE: WBEMSTATUS = -2147217363i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_METHOD: WBEMSTATUS = -2147217362i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_METHOD_PARAMETERS: WBEMSTATUS = -2147217361i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_SYSTEM_PROPERTY: WBEMSTATUS = -2147217360i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_PROPERTY: WBEMSTATUS = -2147217359i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_CALL_CANCELLED: WBEMSTATUS = -2147217358i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_SHUTTING_DOWN: WBEMSTATUS = -2147217357i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PROPAGATED_METHOD: WBEMSTATUS = -2147217356i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_UNSUPPORTED_PARAMETER: WBEMSTATUS = -2147217355i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_MISSING_PARAMETER_ID: WBEMSTATUS = -2147217354i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_PARAMETER_ID: WBEMSTATUS = -2147217353i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_NONCONSECUTIVE_PARAMETER_IDS: WBEMSTATUS = -2147217352i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PARAMETER_ID_ON_RETVAL: WBEMSTATUS = -2147217351i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_OBJECT_PATH: WBEMSTATUS = -2147217350i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_OUT_OF_DISK_SPACE: WBEMSTATUS = -2147217349i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_BUFFER_TOO_SMALL: WBEMSTATUS = -2147217348i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_UNSUPPORTED_PUT_EXTENSION: WBEMSTATUS = -2147217347i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_UNKNOWN_OBJECT_TYPE: WBEMSTATUS = -2147217346i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_UNKNOWN_PACKET_TYPE: WBEMSTATUS = -2147217345i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_MARSHAL_VERSION_MISMATCH: WBEMSTATUS = -2147217344i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_MARSHAL_INVALID_SIGNATURE: WBEMSTATUS = -2147217343i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_QUALIFIER: WBEMSTATUS = -2147217342i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_DUPLICATE_PARAMETER: WBEMSTATUS = -2147217341i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_TOO_MUCH_DATA: WBEMSTATUS = -2147217340i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_SERVER_TOO_BUSY: WBEMSTATUS = -2147217339i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_FLAVOR: WBEMSTATUS = -2147217338i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_CIRCULAR_REFERENCE: WBEMSTATUS = -2147217337i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_UNSUPPORTED_CLASS_UPDATE: WBEMSTATUS = -2147217336i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_CANNOT_CHANGE_KEY_INHERITANCE: WBEMSTATUS = -2147217335i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_CANNOT_CHANGE_INDEX_INHERITANCE: WBEMSTATUS = -2147217328i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_TOO_MANY_PROPERTIES: WBEMSTATUS = -2147217327i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_UPDATE_TYPE_MISMATCH: WBEMSTATUS = -2147217326i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_UPDATE_OVERRIDE_NOT_ALLOWED: WBEMSTATUS = -2147217325i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_UPDATE_PROPAGATED_METHOD: WBEMSTATUS = -2147217324i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_METHOD_NOT_IMPLEMENTED: WBEMSTATUS = -2147217323i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_METHOD_DISABLED: WBEMSTATUS = -2147217322i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_REFRESHER_BUSY: WBEMSTATUS = -2147217321i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_UNPARSABLE_QUERY: WBEMSTATUS = -2147217320i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_NOT_EVENT_CLASS: WBEMSTATUS = -2147217319i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_MISSING_GROUP_WITHIN: WBEMSTATUS = -2147217318i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_MISSING_AGGREGATION_LIST: WBEMSTATUS = -2147217317i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PROPERTY_NOT_AN_OBJECT: WBEMSTATUS = -2147217316i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_AGGREGATING_BY_OBJECT: WBEMSTATUS = -2147217315i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_UNINTERPRETABLE_PROVIDER_QUERY: WBEMSTATUS = -2147217313i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_BACKUP_RESTORE_WINMGMT_RUNNING: WBEMSTATUS = -2147217312i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_QUEUE_OVERFLOW: WBEMSTATUS = -2147217311i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PRIVILEGE_NOT_HELD: WBEMSTATUS = -2147217310i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_OPERATOR: WBEMSTATUS = -2147217309i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_LOCAL_CREDENTIALS: WBEMSTATUS = -2147217308i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_CANNOT_BE_ABSTRACT: WBEMSTATUS = -2147217307i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_AMENDED_OBJECT: WBEMSTATUS = -2147217306i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_CLIENT_TOO_SLOW: WBEMSTATUS = -2147217305i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_NULL_SECURITY_DESCRIPTOR: WBEMSTATUS = -2147217304i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_TIMED_OUT: WBEMSTATUS = -2147217303i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_ASSOCIATION: WBEMSTATUS = -2147217302i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_AMBIGUOUS_OPERATION: WBEMSTATUS = -2147217301i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_QUOTA_VIOLATION: WBEMSTATUS = -2147217300i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_RESERVED_001: WBEMSTATUS = -2147217299i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_RESERVED_002: WBEMSTATUS = -2147217298i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_UNSUPPORTED_LOCALE: WBEMSTATUS = -2147217297i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_HANDLE_OUT_OF_DATE: WBEMSTATUS = -2147217296i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_CONNECTION_FAILED: WBEMSTATUS = -2147217295i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_HANDLE_REQUEST: WBEMSTATUS = -2147217294i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PROPERTY_NAME_TOO_WIDE: WBEMSTATUS = -2147217293i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_CLASS_NAME_TOO_WIDE: WBEMSTATUS = -2147217292i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_METHOD_NAME_TOO_WIDE: WBEMSTATUS = -2147217291i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_QUALIFIER_NAME_TOO_WIDE: WBEMSTATUS = -2147217290i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_RERUN_COMMAND: WBEMSTATUS = -2147217289i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_DATABASE_VER_MISMATCH: WBEMSTATUS = -2147217288i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_VETO_DELETE: WBEMSTATUS = -2147217287i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_VETO_PUT: WBEMSTATUS = -2147217286i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_LOCALE: WBEMSTATUS = -2147217280i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PROVIDER_SUSPENDED: WBEMSTATUS = -2147217279i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_SYNCHRONIZATION_REQUIRED: WBEMSTATUS = -2147217278i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_NO_SCHEMA: WBEMSTATUS = -2147217277i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PROVIDER_ALREADY_REGISTERED: WBEMSTATUS = -2147217276i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PROVIDER_NOT_REGISTERED: WBEMSTATUS = -2147217275i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_FATAL_TRANSPORT_ERROR: WBEMSTATUS = -2147217274i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_ENCRYPTED_CONNECTION_REQUIRED: WBEMSTATUS = -2147217273i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PROVIDER_TIMED_OUT: WBEMSTATUS = -2147217272i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_NO_KEY: WBEMSTATUS = -2147217271i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PROVIDER_DISABLED: WBEMSTATUS = -2147217270i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMESS_E_REGISTRATION_TOO_BROAD: WBEMSTATUS = -2147213311i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMESS_E_REGISTRATION_TOO_PRECISE: WBEMSTATUS = -2147213310i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMESS_E_AUTHZ_NOT_PRIVILEGED: WBEMSTATUS = -2147213309i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_EXPECTED_QUALIFIER_NAME: WBEMSTATUS = -2147205119i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_EXPECTED_SEMI: WBEMSTATUS = -2147205118i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_EXPECTED_OPEN_BRACE: WBEMSTATUS = -2147205117i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_EXPECTED_CLOSE_BRACE: WBEMSTATUS = -2147205116i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_EXPECTED_CLOSE_BRACKET: WBEMSTATUS = -2147205115i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_EXPECTED_CLOSE_PAREN: WBEMSTATUS = -2147205114i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_ILLEGAL_CONSTANT_VALUE: WBEMSTATUS = -2147205113i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_EXPECTED_TYPE_IDENTIFIER: WBEMSTATUS = -2147205112i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_EXPECTED_OPEN_PAREN: WBEMSTATUS = -2147205111i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_UNRECOGNIZED_TOKEN: WBEMSTATUS = -2147205110i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_UNRECOGNIZED_TYPE: WBEMSTATUS = -2147205109i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_EXPECTED_PROPERTY_NAME: WBEMSTATUS = -2147205108i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_TYPEDEF_NOT_SUPPORTED: WBEMSTATUS = -2147205107i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_UNEXPECTED_ALIAS: WBEMSTATUS = -2147205106i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_UNEXPECTED_ARRAY_INIT: WBEMSTATUS = -2147205105i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_INVALID_AMENDMENT_SYNTAX: WBEMSTATUS = -2147205104i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_INVALID_DUPLICATE_AMENDMENT: WBEMSTATUS = -2147205103i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_INVALID_PRAGMA: WBEMSTATUS = -2147205102i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_INVALID_NAMESPACE_SYNTAX: WBEMSTATUS = -2147205101i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_EXPECTED_CLASS_NAME: WBEMSTATUS = -2147205100i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_TYPE_MISMATCH: WBEMSTATUS = -2147205099i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_EXPECTED_ALIAS_NAME: WBEMSTATUS = -2147205098i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_INVALID_CLASS_DECLARATION: WBEMSTATUS = -2147205097i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_INVALID_INSTANCE_DECLARATION: WBEMSTATUS = -2147205096i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_EXPECTED_DOLLAR: WBEMSTATUS = -2147205095i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_CIMTYPE_QUALIFIER: WBEMSTATUS = -2147205094i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_DUPLICATE_PROPERTY: WBEMSTATUS = -2147205093i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_INVALID_NAMESPACE_SPECIFICATION: WBEMSTATUS = -2147205092i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_OUT_OF_RANGE: WBEMSTATUS = -2147205091i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_INVALID_FILE: WBEMSTATUS = -2147205090i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_ALIASES_IN_EMBEDDED: WBEMSTATUS = -2147205089i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_NULL_ARRAY_ELEM: WBEMSTATUS = -2147205088i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_DUPLICATE_QUALIFIER: WBEMSTATUS = -2147205087i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_EXPECTED_FLAVOR_TYPE: WBEMSTATUS = -2147205086i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_INCOMPATIBLE_FLAVOR_TYPES: WBEMSTATUS = -2147205085i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_MULTIPLE_ALIASES: WBEMSTATUS = -2147205084i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_INCOMPATIBLE_FLAVOR_TYPES2: WBEMSTATUS = -2147205083i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_NO_ARRAYS_RETURNED: WBEMSTATUS = -2147205082i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_MUST_BE_IN_OR_OUT: WBEMSTATUS = -2147205081i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_INVALID_FLAGS_SYNTAX: WBEMSTATUS = -2147205080i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_EXPECTED_BRACE_OR_BAD_TYPE: WBEMSTATUS = -2147205079i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_UNSUPPORTED_CIMV22_QUAL_VALUE: WBEMSTATUS = -2147205078i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_UNSUPPORTED_CIMV22_DATA_TYPE: WBEMSTATUS = -2147205077i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_INVALID_DELETEINSTANCE_SYNTAX: WBEMSTATUS = -2147205076i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_INVALID_QUALIFIER_SYNTAX: WBEMSTATUS = -2147205075i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_QUALIFIER_USED_OUTSIDE_SCOPE: WBEMSTATUS = -2147205074i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_ERROR_CREATING_TEMP_FILE: WBEMSTATUS = -2147205073i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_ERROR_INVALID_INCLUDE_FILE: WBEMSTATUS = -2147205072i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_INVALID_DELETECLASS_SYNTAX: WBEMSTATUS = -2147205071i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEMSTATUS_FORMAT = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMSTATUS_FORMAT_NEWLINE: WBEMSTATUS_FORMAT = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMSTATUS_FORMAT_NO_NEWLINE: WBEMSTATUS_FORMAT = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMS_DISPID_COMPLETED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMS_DISPID_CONNECTION_READY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMS_DISPID_DERIVATION: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMS_DISPID_OBJECT_PUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMS_DISPID_OBJECT_READY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMS_DISPID_PROGRESS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEM_BACKUP_RESTORE_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_BACKUP_RESTORE_DEFAULT: WBEM_BACKUP_RESTORE_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_BACKUP_RESTORE_FORCE_SHUTDOWN: WBEM_BACKUP_RESTORE_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEM_BATCH_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_BATCH_IF_NEEDED: WBEM_BATCH_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_MUST_BATCH: WBEM_BATCH_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_MUST_NOT_BATCH: WBEM_BATCH_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEM_CHANGE_FLAG_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_CREATE_OR_UPDATE: WBEM_CHANGE_FLAG_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_UPDATE_ONLY: WBEM_CHANGE_FLAG_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_CREATE_ONLY: WBEM_CHANGE_FLAG_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_UPDATE_COMPATIBLE: WBEM_CHANGE_FLAG_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_UPDATE_SAFE_MODE: WBEM_CHANGE_FLAG_TYPE = 32i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_UPDATE_FORCE_MODE: WBEM_CHANGE_FLAG_TYPE = 64i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_MASK_UPDATE_MODE: WBEM_CHANGE_FLAG_TYPE = 96i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_ADVISORY: WBEM_CHANGE_FLAG_TYPE = 65536i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEM_COMPARISON_FLAG = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_COMPARISON_INCLUDE_ALL: WBEM_COMPARISON_FLAG = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_IGNORE_QUALIFIERS: WBEM_COMPARISON_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_IGNORE_OBJECT_SOURCE: WBEM_COMPARISON_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_IGNORE_DEFAULT_VALUES: WBEM_COMPARISON_FLAG = 4i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_IGNORE_CLASS: WBEM_COMPARISON_FLAG = 8i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_IGNORE_CASE: WBEM_COMPARISON_FLAG = 16i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_IGNORE_FLAVOR: WBEM_COMPARISON_FLAG = 32i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEM_COMPILER_OPTIONS = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_CHECK_ONLY: WBEM_COMPILER_OPTIONS = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_AUTORECOVER: WBEM_COMPILER_OPTIONS = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_WMI_CHECK: WBEM_COMPILER_OPTIONS = 4i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_CONSOLE_PRINT: WBEM_COMPILER_OPTIONS = 8i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_DONT_ADD_TO_LIST: WBEM_COMPILER_OPTIONS = 16i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_SPLIT_FILES: WBEM_COMPILER_OPTIONS = 32i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_STORE_FILE: WBEM_COMPILER_OPTIONS = 256i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct WBEM_COMPILE_STATUS_INFO {
    pub lPhaseError: i32,
    pub hRes: ::windows_sys::core::HRESULT,
    pub ObjectNum: i32,
    pub FirstLine: i32,
    pub LastLine: i32,
    pub dwOutFlags: u32,
}
impl ::core::marker::Copy for WBEM_COMPILE_STATUS_INFO {}
impl ::core::clone::Clone for WBEM_COMPILE_STATUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEM_CONDITION_FLAG_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_ALWAYS: WBEM_CONDITION_FLAG_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_ONLY_IF_TRUE: WBEM_CONDITION_FLAG_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_ONLY_IF_FALSE: WBEM_CONDITION_FLAG_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_ONLY_IF_IDENTICAL: WBEM_CONDITION_FLAG_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_MASK_PRIMARY_CONDITION: WBEM_CONDITION_FLAG_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_KEYS_ONLY: WBEM_CONDITION_FLAG_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_REFS_ONLY: WBEM_CONDITION_FLAG_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_LOCAL_ONLY: WBEM_CONDITION_FLAG_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_PROPAGATED_ONLY: WBEM_CONDITION_FLAG_TYPE = 32i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_SYSTEM_ONLY: WBEM_CONDITION_FLAG_TYPE = 48i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_NONSYSTEM_ONLY: WBEM_CONDITION_FLAG_TYPE = 64i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_MASK_CONDITION_ORIGIN: WBEM_CONDITION_FLAG_TYPE = 112i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_CLASS_OVERRIDES_ONLY: WBEM_CONDITION_FLAG_TYPE = 256i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_CLASS_LOCAL_AND_OVERRIDES: WBEM_CONDITION_FLAG_TYPE = 512i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_MASK_CLASS_CONDITION: WBEM_CONDITION_FLAG_TYPE = 768i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEM_CONNECT_OPTIONS = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_CONNECT_REPOSITORY_ONLY: WBEM_CONNECT_OPTIONS = 64i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_CONNECT_USE_MAX_WAIT: WBEM_CONNECT_OPTIONS = 128i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_CONNECT_PROVIDERS: WBEM_CONNECT_OPTIONS = 256i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEM_EXTRA_RETURN_CODES = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_INITIALIZED: WBEM_EXTRA_RETURN_CODES = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_LIMITED_SERVICE: WBEM_EXTRA_RETURN_CODES = 274433i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_INDIRECTLY_UPDATED: WBEM_EXTRA_RETURN_CODES = 274434i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_SUBJECT_TO_SDS: WBEM_EXTRA_RETURN_CODES = 274435i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_RETRY_LATER: WBEM_EXTRA_RETURN_CODES = -2147209215i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_RESOURCE_CONTENTION: WBEM_EXTRA_RETURN_CODES = -2147209214i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEM_FLAVOR_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAVOR_DONT_PROPAGATE: WBEM_FLAVOR_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAVOR_FLAG_PROPAGATE_TO_INSTANCE: WBEM_FLAVOR_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAVOR_FLAG_PROPAGATE_TO_DERIVED_CLASS: WBEM_FLAVOR_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAVOR_MASK_PROPAGATION: WBEM_FLAVOR_TYPE = 15i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAVOR_OVERRIDABLE: WBEM_FLAVOR_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAVOR_NOT_OVERRIDABLE: WBEM_FLAVOR_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAVOR_MASK_PERMISSIONS: WBEM_FLAVOR_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAVOR_ORIGIN_LOCAL: WBEM_FLAVOR_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAVOR_ORIGIN_PROPAGATED: WBEM_FLAVOR_TYPE = 32i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAVOR_ORIGIN_SYSTEM: WBEM_FLAVOR_TYPE = 64i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAVOR_MASK_ORIGIN: WBEM_FLAVOR_TYPE = 96i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAVOR_NOT_AMENDED: WBEM_FLAVOR_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAVOR_AMENDED: WBEM_FLAVOR_TYPE = 128i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAVOR_MASK_AMENDED: WBEM_FLAVOR_TYPE = 128i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEM_GENERIC_FLAG_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_RETURN_IMMEDIATELY: WBEM_GENERIC_FLAG_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_RETURN_WBEM_COMPLETE: WBEM_GENERIC_FLAG_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_BIDIRECTIONAL: WBEM_GENERIC_FLAG_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_FORWARD_ONLY: WBEM_GENERIC_FLAG_TYPE = 32i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_NO_ERROR_OBJECT: WBEM_GENERIC_FLAG_TYPE = 64i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_RETURN_ERROR_OBJECT: WBEM_GENERIC_FLAG_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_SEND_STATUS: WBEM_GENERIC_FLAG_TYPE = 128i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_DONT_SEND_STATUS: WBEM_GENERIC_FLAG_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_ENSURE_LOCATABLE: WBEM_GENERIC_FLAG_TYPE = 256i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_DIRECT_READ: WBEM_GENERIC_FLAG_TYPE = 512i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_SEND_ONLY_SELECTED: WBEM_GENERIC_FLAG_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_RETURN_WHEN_COMPLETE: WBEM_GENERIC_FLAG_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_RETURN_IMMEDIATELY: WBEM_GENERIC_FLAG_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_MASK_RESERVED_FLAGS: WBEM_GENERIC_FLAG_TYPE = 126976i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_USE_AMENDED_QUALIFIERS: WBEM_GENERIC_FLAG_TYPE = 131072i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_STRONG_VALIDATION: WBEM_GENERIC_FLAG_TYPE = 1048576i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEM_GENUS_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_GENUS_CLASS: WBEM_GENUS_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_GENUS_INSTANCE: WBEM_GENUS_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEM_GET_KEY_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_TEXT: WBEM_GET_KEY_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_QUOTEDTEXT: WBEM_GET_KEY_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEM_GET_TEXT_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_COMPRESSED: WBEM_GET_TEXT_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_GET_RELATIVE_ONLY: WBEM_GET_TEXT_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_GET_SERVER_TOO: WBEM_GET_TEXT_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_GET_SERVER_AND_NAMESPACE_ONLY: WBEM_GET_TEXT_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_GET_NAMESPACE_ONLY: WBEM_GET_TEXT_FLAGS = 16i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_GET_ORIGINAL: WBEM_GET_TEXT_FLAGS = 32i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEM_INFORMATION_FLAG_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_SHORT_NAME: WBEM_INFORMATION_FLAG_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_LONG_NAME: WBEM_INFORMATION_FLAG_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEM_LIMITATION_FLAG_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_EXCLUDE_OBJECT_QUALIFIERS: WBEM_LIMITATION_FLAG_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_EXCLUDE_PROPERTY_QUALIFIERS: WBEM_LIMITATION_FLAG_TYPE = 32i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEM_LIMITS = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_MAX_IDENTIFIER: WBEM_LIMITS = 4096i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_MAX_QUERY: WBEM_LIMITS = 16384i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_MAX_PATH: WBEM_LIMITS = 8192i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_MAX_OBJECT_NESTING: WBEM_LIMITS = 64i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_MAX_USER_PROPERTIES: WBEM_LIMITS = 1024i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEM_LOCKING = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_ALLOW_READ: WBEM_LOCKING = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEM_PATH_CREATE_FLAG = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_CREATE_ACCEPT_RELATIVE: WBEM_PATH_CREATE_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_CREATE_ACCEPT_ABSOLUTE: WBEM_PATH_CREATE_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_CREATE_ACCEPT_ALL: WBEM_PATH_CREATE_FLAG = 4i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_TREAT_SINGLE_IDENT_AS_NS: WBEM_PATH_CREATE_FLAG = 8i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEM_PATH_STATUS_FLAG = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_ANON_LOCAL_MACHINE: WBEM_PATH_STATUS_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_HAS_MACHINE_NAME: WBEM_PATH_STATUS_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_IS_CLASS_REF: WBEM_PATH_STATUS_FLAG = 4i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_IS_INST_REF: WBEM_PATH_STATUS_FLAG = 8i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_HAS_SUBSCOPES: WBEM_PATH_STATUS_FLAG = 16i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_IS_COMPOUND: WBEM_PATH_STATUS_FLAG = 32i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_HAS_V2_REF_PATHS: WBEM_PATH_STATUS_FLAG = 64i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_HAS_IMPLIED_KEY: WBEM_PATH_STATUS_FLAG = 128i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_CONTAINS_SINGLETON: WBEM_PATH_STATUS_FLAG = 256i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_V1_COMPLIANT: WBEM_PATH_STATUS_FLAG = 512i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_V2_COMPLIANT: WBEM_PATH_STATUS_FLAG = 1024i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_CIM_COMPLIANT: WBEM_PATH_STATUS_FLAG = 2048i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_IS_SINGLETON: WBEM_PATH_STATUS_FLAG = 4096i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_IS_PARENT: WBEM_PATH_STATUS_FLAG = 8192i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_SERVER_NAMESPACE_ONLY: WBEM_PATH_STATUS_FLAG = 16384i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_NATIVE_PATH: WBEM_PATH_STATUS_FLAG = 32768i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_WMI_PATH: WBEM_PATH_STATUS_FLAG = 65536i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_PATH_HAD_SERVER: WBEM_PATH_STATUS_FLAG = 131072i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEM_PROVIDER_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_OWNER_UPDATE: WBEM_PROVIDER_FLAGS = 65536i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEM_PROVIDER_REQUIREMENTS_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_REQUIREMENTS_START_POSTFILTER: WBEM_PROVIDER_REQUIREMENTS_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_REQUIREMENTS_STOP_POSTFILTER: WBEM_PROVIDER_REQUIREMENTS_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_REQUIREMENTS_RECHECK_SUBSCRIPTIONS: WBEM_PROVIDER_REQUIREMENTS_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEM_QUERY_FLAG_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_DEEP: WBEM_QUERY_FLAG_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_SHALLOW: WBEM_QUERY_FLAG_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_PROTOTYPE: WBEM_QUERY_FLAG_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEM_REFRESHER_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_REFRESH_AUTO_RECONNECT: WBEM_REFRESHER_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_REFRESH_NO_AUTO_RECONNECT: WBEM_REFRESHER_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEM_SECURITY_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_ENABLE: WBEM_SECURITY_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_METHOD_EXECUTE: WBEM_SECURITY_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FULL_WRITE_REP: WBEM_SECURITY_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_PARTIAL_WRITE_REP: WBEM_SECURITY_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_WRITE_PROVIDER: WBEM_SECURITY_FLAGS = 16i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_REMOTE_ACCESS: WBEM_SECURITY_FLAGS = 32i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_RIGHT_SUBSCRIBE: WBEM_SECURITY_FLAGS = 64i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_RIGHT_PUBLISH: WBEM_SECURITY_FLAGS = 128i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEM_SHUTDOWN_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_SHUTDOWN_UNLOAD_COMPONENT: WBEM_SHUTDOWN_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_SHUTDOWN_WMI: WBEM_SHUTDOWN_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_SHUTDOWN_OS: WBEM_SHUTDOWN_FLAGS = 3i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEM_STATUS_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_STATUS_COMPLETE: WBEM_STATUS_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_STATUS_REQUIREMENTS: WBEM_STATUS_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_STATUS_PROGRESS: WBEM_STATUS_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_STATUS_LOGGING_INFORMATION: WBEM_STATUS_TYPE = 256i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_STATUS_LOGGING_INFORMATION_PROVIDER: WBEM_STATUS_TYPE = 512i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_STATUS_LOGGING_INFORMATION_HOST: WBEM_STATUS_TYPE = 1024i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_STATUS_LOGGING_INFORMATION_REPOSITORY: WBEM_STATUS_TYPE = 2048i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_STATUS_LOGGING_INFORMATION_ESS: WBEM_STATUS_TYPE = 4096i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEM_TEXT_FLAG_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_NO_FLAVORS: WBEM_TEXT_FLAG_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEM_TIMEOUT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_NO_WAIT: WBEM_TIMEOUT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_INFINITE: WBEM_TIMEOUT_TYPE = -1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WBEM_UNSECAPP_FLAG_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_UNSECAPP_DEFAULT_CHECK_ACCESS: WBEM_UNSECAPP_FLAG_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_UNSECAPP_CHECK_ACCESS: WBEM_UNSECAPP_FLAG_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_UNSECAPP_DONT_CHECK_ACCESS: WBEM_UNSECAPP_FLAG_TYPE = 2i32;
pub const WMIExtension: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4036451070, data2: 23679, data3: 4562, data4: [139, 116, 0, 16, 75, 42, 251, 65] };
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WMIQ_ANALYSIS_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ANALYSIS_RPN_SEQUENCE: WMIQ_ANALYSIS_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ANALYSIS_ASSOC_QUERY: WMIQ_ANALYSIS_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ANALYSIS_PROP_ANALYSIS_MATRIX: WMIQ_ANALYSIS_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ANALYSIS_QUERY_TEXT: WMIQ_ANALYSIS_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ANALYSIS_RESERVED: WMIQ_ANALYSIS_TYPE = 134217728i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WMIQ_ASSOCQ_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ASSOCQ_ASSOCIATORS: WMIQ_ASSOCQ_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ASSOCQ_REFERENCES: WMIQ_ASSOCQ_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ASSOCQ_RESULTCLASS: WMIQ_ASSOCQ_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ASSOCQ_ASSOCCLASS: WMIQ_ASSOCQ_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ASSOCQ_ROLE: WMIQ_ASSOCQ_FLAGS = 16i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ASSOCQ_RESULTROLE: WMIQ_ASSOCQ_FLAGS = 32i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ASSOCQ_REQUIREDQUALIFIER: WMIQ_ASSOCQ_FLAGS = 64i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ASSOCQ_REQUIREDASSOCQUALIFIER: WMIQ_ASSOCQ_FLAGS = 128i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ASSOCQ_CLASSDEFSONLY: WMIQ_ASSOCQ_FLAGS = 256i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ASSOCQ_KEYSONLY: WMIQ_ASSOCQ_FLAGS = 512i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ASSOCQ_SCHEMAONLY: WMIQ_ASSOCQ_FLAGS = 1024i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ASSOCQ_CLASSREFSONLY: WMIQ_ASSOCQ_FLAGS = 2048i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WMIQ_LANGUAGE_FEATURES = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF1_BASIC_SELECT: WMIQ_LANGUAGE_FEATURES = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF2_CLASS_NAME_IN_QUERY: WMIQ_LANGUAGE_FEATURES = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF3_STRING_CASE_FUNCTIONS: WMIQ_LANGUAGE_FEATURES = 3i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF4_PROP_TO_PROP_TESTS: WMIQ_LANGUAGE_FEATURES = 4i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF5_COUNT_STAR: WMIQ_LANGUAGE_FEATURES = 5i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF6_ORDER_BY: WMIQ_LANGUAGE_FEATURES = 6i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF7_DISTINCT: WMIQ_LANGUAGE_FEATURES = 7i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF8_ISA: WMIQ_LANGUAGE_FEATURES = 8i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF9_THIS: WMIQ_LANGUAGE_FEATURES = 9i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF10_COMPEX_SUBEXPRESSIONS: WMIQ_LANGUAGE_FEATURES = 10i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF11_ALIASING: WMIQ_LANGUAGE_FEATURES = 11i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF12_GROUP_BY_HAVING: WMIQ_LANGUAGE_FEATURES = 12i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF13_WMI_WITHIN: WMIQ_LANGUAGE_FEATURES = 13i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF14_SQL_WRITE_OPERATIONS: WMIQ_LANGUAGE_FEATURES = 14i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF15_GO: WMIQ_LANGUAGE_FEATURES = 15i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF16_SINGLE_LEVEL_TRANSACTIONS: WMIQ_LANGUAGE_FEATURES = 16i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF17_QUALIFIED_NAMES: WMIQ_LANGUAGE_FEATURES = 17i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF18_ASSOCIATONS: WMIQ_LANGUAGE_FEATURES = 18i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF19_SYSTEM_PROPERTIES: WMIQ_LANGUAGE_FEATURES = 19i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF20_EXTENDED_SYSTEM_PROPERTIES: WMIQ_LANGUAGE_FEATURES = 20i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF21_SQL89_JOINS: WMIQ_LANGUAGE_FEATURES = 21i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF22_SQL92_JOINS: WMIQ_LANGUAGE_FEATURES = 22i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF23_SUBSELECTS: WMIQ_LANGUAGE_FEATURES = 23i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF24_UMI_EXTENSIONS: WMIQ_LANGUAGE_FEATURES = 24i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF25_DATEPART: WMIQ_LANGUAGE_FEATURES = 25i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF26_LIKE: WMIQ_LANGUAGE_FEATURES = 26i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF27_CIM_TEMPORAL_CONSTRUCTS: WMIQ_LANGUAGE_FEATURES = 27i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF28_STANDARD_AGGREGATES: WMIQ_LANGUAGE_FEATURES = 28i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF29_MULTI_LEVEL_ORDER_BY: WMIQ_LANGUAGE_FEATURES = 29i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF30_WMI_PRAGMAS: WMIQ_LANGUAGE_FEATURES = 30i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF31_QUALIFIER_TESTS: WMIQ_LANGUAGE_FEATURES = 31i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF32_SP_EXECUTE: WMIQ_LANGUAGE_FEATURES = 32i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF33_ARRAY_ACCESS: WMIQ_LANGUAGE_FEATURES = 33i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF34_UNION: WMIQ_LANGUAGE_FEATURES = 34i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF35_COMPLEX_SELECT_TARGET: WMIQ_LANGUAGE_FEATURES = 35i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF36_REFERENCE_TESTS: WMIQ_LANGUAGE_FEATURES = 36i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF37_SELECT_INTO: WMIQ_LANGUAGE_FEATURES = 37i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF38_BASIC_DATETIME_TESTS: WMIQ_LANGUAGE_FEATURES = 38i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF39_COUNT_COLUMN: WMIQ_LANGUAGE_FEATURES = 39i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF40_BETWEEN: WMIQ_LANGUAGE_FEATURES = 40i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF_LAST: WMIQ_LANGUAGE_FEATURES = 40i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WMIQ_RPNQ_FEATURE = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPNF_WHERE_CLAUSE_PRESENT: WMIQ_RPNQ_FEATURE = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPNF_QUERY_IS_CONJUNCTIVE: WMIQ_RPNQ_FEATURE = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPNF_QUERY_IS_DISJUNCTIVE: WMIQ_RPNQ_FEATURE = 4i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPNF_PROJECTION: WMIQ_RPNQ_FEATURE = 8i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPNF_FEATURE_SELECT_STAR: WMIQ_RPNQ_FEATURE = 16i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPNF_EQUALITY_TESTS_ONLY: WMIQ_RPNQ_FEATURE = 32i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPNF_COUNT_STAR: WMIQ_RPNQ_FEATURE = 64i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPNF_QUALIFIED_NAMES_USED: WMIQ_RPNQ_FEATURE = 128i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPNF_SYSPROP_CLASS_USED: WMIQ_RPNQ_FEATURE = 256i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPNF_PROP_TO_PROP_TESTS: WMIQ_RPNQ_FEATURE = 512i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPNF_ORDER_BY: WMIQ_RPNQ_FEATURE = 1024i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPNF_ISA_USED: WMIQ_RPNQ_FEATURE = 2048i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPNF_GROUP_BY_HAVING: WMIQ_RPNQ_FEATURE = 4096i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPNF_ARRAY_ACCESS_USED: WMIQ_RPNQ_FEATURE = 8192i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WMIQ_RPN_TOKEN_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_TOKEN_EXPRESSION: WMIQ_RPN_TOKEN_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_TOKEN_AND: WMIQ_RPN_TOKEN_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_TOKEN_OR: WMIQ_RPN_TOKEN_FLAGS = 3i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_TOKEN_NOT: WMIQ_RPN_TOKEN_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_OP_UNDEFINED: WMIQ_RPN_TOKEN_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_OP_EQ: WMIQ_RPN_TOKEN_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_OP_NE: WMIQ_RPN_TOKEN_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_OP_GE: WMIQ_RPN_TOKEN_FLAGS = 3i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_OP_LE: WMIQ_RPN_TOKEN_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_OP_LT: WMIQ_RPN_TOKEN_FLAGS = 5i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_OP_GT: WMIQ_RPN_TOKEN_FLAGS = 6i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_OP_LIKE: WMIQ_RPN_TOKEN_FLAGS = 7i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_OP_ISA: WMIQ_RPN_TOKEN_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_OP_ISNOTA: WMIQ_RPN_TOKEN_FLAGS = 9i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_OP_ISNULL: WMIQ_RPN_TOKEN_FLAGS = 10i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_OP_ISNOTNULL: WMIQ_RPN_TOKEN_FLAGS = 11i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_LEFT_PROPERTY_NAME: WMIQ_RPN_TOKEN_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_RIGHT_PROPERTY_NAME: WMIQ_RPN_TOKEN_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_CONST2: WMIQ_RPN_TOKEN_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_CONST: WMIQ_RPN_TOKEN_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_RELOP: WMIQ_RPN_TOKEN_FLAGS = 16i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_LEFT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = 32i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_RIGHT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = 64i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_GET_TOKEN_TYPE: WMIQ_RPN_TOKEN_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_GET_EXPR_SHAPE: WMIQ_RPN_TOKEN_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_GET_LEFT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = 3i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_GET_RIGHT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_GET_RELOP: WMIQ_RPN_TOKEN_FLAGS = 5i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_NEXT_TOKEN: WMIQ_RPN_TOKEN_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_FROM_UNARY: WMIQ_RPN_TOKEN_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_FROM_PATH: WMIQ_RPN_TOKEN_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_FROM_CLASS_LIST: WMIQ_RPN_TOKEN_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_FROM_MULTIPLE: WMIQ_RPN_TOKEN_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WMI_OBJ_TEXT = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMI_OBJ_TEXT_CIM_DTD_2_0: WMI_OBJ_TEXT = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMI_OBJ_TEXT_WMI_DTD_2_0: WMI_OBJ_TEXT = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMI_OBJ_TEXT_WMI_EXT1: WMI_OBJ_TEXT = 3i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMI_OBJ_TEXT_WMI_EXT2: WMI_OBJ_TEXT = 4i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMI_OBJ_TEXT_WMI_EXT3: WMI_OBJ_TEXT = 5i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMI_OBJ_TEXT_WMI_EXT4: WMI_OBJ_TEXT = 6i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMI_OBJ_TEXT_WMI_EXT5: WMI_OBJ_TEXT = 7i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMI_OBJ_TEXT_WMI_EXT6: WMI_OBJ_TEXT = 8i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMI_OBJ_TEXT_WMI_EXT7: WMI_OBJ_TEXT = 9i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMI_OBJ_TEXT_WMI_EXT8: WMI_OBJ_TEXT = 10i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMI_OBJ_TEXT_WMI_EXT9: WMI_OBJ_TEXT = 11i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMI_OBJ_TEXT_WMI_EXT10: WMI_OBJ_TEXT = 12i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMI_OBJ_TEXT_LAST: WMI_OBJ_TEXT = 13i32;
pub const WbemAdministrativeLocator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3414513100, data2: 37160, data3: 4561, data4: [173, 155, 0, 192, 79, 216, 253, 255] };
pub const WbemAuthenticatedLocator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3440919350, data2: 37160, data3: 4561, data4: [173, 155, 0, 192, 79, 216, 253, 255] };
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WbemAuthenticationLevelEnum = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemAuthenticationLevelDefault: WbemAuthenticationLevelEnum = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemAuthenticationLevelNone: WbemAuthenticationLevelEnum = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemAuthenticationLevelConnect: WbemAuthenticationLevelEnum = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemAuthenticationLevelCall: WbemAuthenticationLevelEnum = 3i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemAuthenticationLevelPkt: WbemAuthenticationLevelEnum = 4i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemAuthenticationLevelPktIntegrity: WbemAuthenticationLevelEnum = 5i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemAuthenticationLevelPktPrivacy: WbemAuthenticationLevelEnum = 6i32;
pub const WbemBackupRestore: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3298702022, data2: 48267, data3: 4562, data4: [133, 212, 0, 16, 90, 31, 131, 4] };
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WbemChangeFlagEnum = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemChangeFlagCreateOrUpdate: WbemChangeFlagEnum = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemChangeFlagUpdateOnly: WbemChangeFlagEnum = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemChangeFlagCreateOnly: WbemChangeFlagEnum = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemChangeFlagUpdateCompatible: WbemChangeFlagEnum = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemChangeFlagUpdateSafeMode: WbemChangeFlagEnum = 32i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemChangeFlagUpdateForceMode: WbemChangeFlagEnum = 64i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemChangeFlagStrongValidation: WbemChangeFlagEnum = 128i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemChangeFlagAdvisory: WbemChangeFlagEnum = 65536i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WbemCimtypeEnum = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeSint8: WbemCimtypeEnum = 16i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeUint8: WbemCimtypeEnum = 17i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeSint16: WbemCimtypeEnum = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeUint16: WbemCimtypeEnum = 18i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeSint32: WbemCimtypeEnum = 3i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeUint32: WbemCimtypeEnum = 19i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeSint64: WbemCimtypeEnum = 20i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeUint64: WbemCimtypeEnum = 21i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeReal32: WbemCimtypeEnum = 4i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeReal64: WbemCimtypeEnum = 5i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeBoolean: WbemCimtypeEnum = 11i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeString: WbemCimtypeEnum = 8i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeDatetime: WbemCimtypeEnum = 101i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeReference: WbemCimtypeEnum = 102i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeChar16: WbemCimtypeEnum = 103i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeObject: WbemCimtypeEnum = 13i32;
pub const WbemClassObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2590322822, data2: 5967, data3: 4562, data4: [181, 249, 0, 16, 75, 112, 62, 253] };
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WbemComparisonFlagEnum = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemComparisonFlagIncludeAll: WbemComparisonFlagEnum = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemComparisonFlagIgnoreQualifiers: WbemComparisonFlagEnum = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemComparisonFlagIgnoreObjectSource: WbemComparisonFlagEnum = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemComparisonFlagIgnoreDefaultValues: WbemComparisonFlagEnum = 4i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemComparisonFlagIgnoreClass: WbemComparisonFlagEnum = 8i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemComparisonFlagIgnoreCase: WbemComparisonFlagEnum = 16i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemComparisonFlagIgnoreFlavor: WbemComparisonFlagEnum = 32i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WbemConnectOptionsEnum = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemConnectFlagUseMaxWait: WbemConnectOptionsEnum = 128i32;
pub const WbemContext: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1732994712, data2: 61074, data3: 4560, data4: [173, 113, 0, 192, 79, 216, 253, 255] };
pub const WbemDCOMTransport: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4157484563, data2: 35984, data3: 4561, data4: [158, 123, 0, 192, 79, 195, 36, 168] };
pub const WbemDecoupledBasicEventProvider: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4126627639, data2: 10307, data3: 20258, data4: [147, 61, 199, 106, 151, 205, 166, 47] };
pub const WbemDecoupledRegistrar: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1291614514, data2: 3997, data3: 19439, data4: [156, 50, 142, 162, 166, 181, 111, 203] };
pub const WbemDefPath: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3477914629, data2: 58053, data3: 19933, data4: [179, 206, 94, 117, 130, 216, 201, 250] };
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WbemErrorEnum = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemNoErr: WbemErrorEnum = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrFailed: WbemErrorEnum = -2147217407i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrNotFound: WbemErrorEnum = -2147217406i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrAccessDenied: WbemErrorEnum = -2147217405i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrProviderFailure: WbemErrorEnum = -2147217404i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrTypeMismatch: WbemErrorEnum = -2147217403i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrOutOfMemory: WbemErrorEnum = -2147217402i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidContext: WbemErrorEnum = -2147217401i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidParameter: WbemErrorEnum = -2147217400i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrNotAvailable: WbemErrorEnum = -2147217399i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrCriticalError: WbemErrorEnum = -2147217398i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidStream: WbemErrorEnum = -2147217397i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrNotSupported: WbemErrorEnum = -2147217396i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidSuperclass: WbemErrorEnum = -2147217395i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidNamespace: WbemErrorEnum = -2147217394i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidObject: WbemErrorEnum = -2147217393i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidClass: WbemErrorEnum = -2147217392i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrProviderNotFound: WbemErrorEnum = -2147217391i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidProviderRegistration: WbemErrorEnum = -2147217390i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrProviderLoadFailure: WbemErrorEnum = -2147217389i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInitializationFailure: WbemErrorEnum = -2147217388i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrTransportFailure: WbemErrorEnum = -2147217387i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidOperation: WbemErrorEnum = -2147217386i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidQuery: WbemErrorEnum = -2147217385i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidQueryType: WbemErrorEnum = -2147217384i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrAlreadyExists: WbemErrorEnum = -2147217383i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrOverrideNotAllowed: WbemErrorEnum = -2147217382i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrPropagatedQualifier: WbemErrorEnum = -2147217381i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrPropagatedProperty: WbemErrorEnum = -2147217380i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrUnexpected: WbemErrorEnum = -2147217379i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrIllegalOperation: WbemErrorEnum = -2147217378i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrCannotBeKey: WbemErrorEnum = -2147217377i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrIncompleteClass: WbemErrorEnum = -2147217376i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidSyntax: WbemErrorEnum = -2147217375i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrNondecoratedObject: WbemErrorEnum = -2147217374i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrReadOnly: WbemErrorEnum = -2147217373i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrProviderNotCapable: WbemErrorEnum = -2147217372i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrClassHasChildren: WbemErrorEnum = -2147217371i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrClassHasInstances: WbemErrorEnum = -2147217370i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrQueryNotImplemented: WbemErrorEnum = -2147217369i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrIllegalNull: WbemErrorEnum = -2147217368i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidQualifierType: WbemErrorEnum = -2147217367i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidPropertyType: WbemErrorEnum = -2147217366i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrValueOutOfRange: WbemErrorEnum = -2147217365i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrCannotBeSingleton: WbemErrorEnum = -2147217364i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidCimType: WbemErrorEnum = -2147217363i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidMethod: WbemErrorEnum = -2147217362i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidMethodParameters: WbemErrorEnum = -2147217361i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrSystemProperty: WbemErrorEnum = -2147217360i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidProperty: WbemErrorEnum = -2147217359i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrCallCancelled: WbemErrorEnum = -2147217358i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrShuttingDown: WbemErrorEnum = -2147217357i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrPropagatedMethod: WbemErrorEnum = -2147217356i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrUnsupportedParameter: WbemErrorEnum = -2147217355i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrMissingParameter: WbemErrorEnum = -2147217354i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidParameterId: WbemErrorEnum = -2147217353i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrNonConsecutiveParameterIds: WbemErrorEnum = -2147217352i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrParameterIdOnRetval: WbemErrorEnum = -2147217351i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidObjectPath: WbemErrorEnum = -2147217350i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrOutOfDiskSpace: WbemErrorEnum = -2147217349i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrBufferTooSmall: WbemErrorEnum = -2147217348i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrUnsupportedPutExtension: WbemErrorEnum = -2147217347i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrUnknownObjectType: WbemErrorEnum = -2147217346i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrUnknownPacketType: WbemErrorEnum = -2147217345i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrMarshalVersionMismatch: WbemErrorEnum = -2147217344i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrMarshalInvalidSignature: WbemErrorEnum = -2147217343i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidQualifier: WbemErrorEnum = -2147217342i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidDuplicateParameter: WbemErrorEnum = -2147217341i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrTooMuchData: WbemErrorEnum = -2147217340i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrServerTooBusy: WbemErrorEnum = -2147217339i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidFlavor: WbemErrorEnum = -2147217338i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrCircularReference: WbemErrorEnum = -2147217337i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrUnsupportedClassUpdate: WbemErrorEnum = -2147217336i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrCannotChangeKeyInheritance: WbemErrorEnum = -2147217335i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrCannotChangeIndexInheritance: WbemErrorEnum = -2147217328i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrTooManyProperties: WbemErrorEnum = -2147217327i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrUpdateTypeMismatch: WbemErrorEnum = -2147217326i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrUpdateOverrideNotAllowed: WbemErrorEnum = -2147217325i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrUpdatePropagatedMethod: WbemErrorEnum = -2147217324i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrMethodNotImplemented: WbemErrorEnum = -2147217323i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrMethodDisabled: WbemErrorEnum = -2147217322i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrRefresherBusy: WbemErrorEnum = -2147217321i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrUnparsableQuery: WbemErrorEnum = -2147217320i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrNotEventClass: WbemErrorEnum = -2147217319i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrMissingGroupWithin: WbemErrorEnum = -2147217318i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrMissingAggregationList: WbemErrorEnum = -2147217317i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrPropertyNotAnObject: WbemErrorEnum = -2147217316i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrAggregatingByObject: WbemErrorEnum = -2147217315i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrUninterpretableProviderQuery: WbemErrorEnum = -2147217313i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrBackupRestoreWinmgmtRunning: WbemErrorEnum = -2147217312i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrQueueOverflow: WbemErrorEnum = -2147217311i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrPrivilegeNotHeld: WbemErrorEnum = -2147217310i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidOperator: WbemErrorEnum = -2147217309i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrLocalCredentials: WbemErrorEnum = -2147217308i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrCannotBeAbstract: WbemErrorEnum = -2147217307i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrAmendedObject: WbemErrorEnum = -2147217306i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrClientTooSlow: WbemErrorEnum = -2147217305i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrNullSecurityDescriptor: WbemErrorEnum = -2147217304i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrTimeout: WbemErrorEnum = -2147217303i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidAssociation: WbemErrorEnum = -2147217302i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrAmbiguousOperation: WbemErrorEnum = -2147217301i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrQuotaViolation: WbemErrorEnum = -2147217300i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrTransactionConflict: WbemErrorEnum = -2147217299i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrForcedRollback: WbemErrorEnum = -2147217298i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrUnsupportedLocale: WbemErrorEnum = -2147217297i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrHandleOutOfDate: WbemErrorEnum = -2147217296i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrConnectionFailed: WbemErrorEnum = -2147217295i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidHandleRequest: WbemErrorEnum = -2147217294i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrPropertyNameTooWide: WbemErrorEnum = -2147217293i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrClassNameTooWide: WbemErrorEnum = -2147217292i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrMethodNameTooWide: WbemErrorEnum = -2147217291i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrQualifierNameTooWide: WbemErrorEnum = -2147217290i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrRerunCommand: WbemErrorEnum = -2147217289i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrDatabaseVerMismatch: WbemErrorEnum = -2147217288i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrVetoPut: WbemErrorEnum = -2147217287i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrVetoDelete: WbemErrorEnum = -2147217286i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidLocale: WbemErrorEnum = -2147217280i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrProviderSuspended: WbemErrorEnum = -2147217279i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrSynchronizationRequired: WbemErrorEnum = -2147217278i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrNoSchema: WbemErrorEnum = -2147217277i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrProviderAlreadyRegistered: WbemErrorEnum = -2147217276i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrProviderNotRegistered: WbemErrorEnum = -2147217275i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrFatalTransportError: WbemErrorEnum = -2147217274i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrEncryptedConnectionRequired: WbemErrorEnum = -2147217273i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrRegistrationTooBroad: WbemErrorEnum = -2147213311i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrRegistrationTooPrecise: WbemErrorEnum = -2147213310i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrTimedout: WbemErrorEnum = -2147209215i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrResetToDefault: WbemErrorEnum = -2147209214i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WbemFlagEnum = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemFlagReturnImmediately: WbemFlagEnum = 16i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemFlagReturnWhenComplete: WbemFlagEnum = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemFlagBidirectional: WbemFlagEnum = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemFlagForwardOnly: WbemFlagEnum = 32i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemFlagNoErrorObject: WbemFlagEnum = 64i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemFlagReturnErrorObject: WbemFlagEnum = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemFlagSendStatus: WbemFlagEnum = 128i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemFlagDontSendStatus: WbemFlagEnum = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemFlagEnsureLocatable: WbemFlagEnum = 256i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemFlagDirectRead: WbemFlagEnum = 512i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemFlagSendOnlySelected: WbemFlagEnum = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemFlagUseAmendedQualifiers: WbemFlagEnum = 131072i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemFlagGetDefault: WbemFlagEnum = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemFlagSpawnInstance: WbemFlagEnum = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemFlagUseCurrentTime: WbemFlagEnum = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WbemImpersonationLevelEnum = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemImpersonationLevelAnonymous: WbemImpersonationLevelEnum = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemImpersonationLevelIdentify: WbemImpersonationLevelEnum = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemImpersonationLevelImpersonate: WbemImpersonationLevelEnum = 3i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemImpersonationLevelDelegate: WbemImpersonationLevelEnum = 4i32;
pub const WbemLevel1Login: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2344874078, data2: 55403, data3: 4560, data4: [160, 117, 0, 192, 79, 182, 136, 32] };
pub const WbemLocalAddrRes: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2701412353, data2: 36734, data3: 4561, data4: [158, 124, 0, 192, 79, 195, 36, 168] };
pub const WbemLocator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1167128593, data2: 7482, data3: 4560, data4: [137, 31, 0, 170, 0, 75, 46, 36] };
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WbemObjectTextFormatEnum = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemObjectTextFormatCIMDTD20: WbemObjectTextFormatEnum = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemObjectTextFormatWMIDTD20: WbemObjectTextFormatEnum = 2i32;
pub const WbemObjectTextSrc: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2367444381, data2: 34032, data3: 19379, data4: [167, 213, 86, 167, 67, 90, 155, 166] };
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WbemPrivilegeEnum = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeCreateToken: WbemPrivilegeEnum = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegePrimaryToken: WbemPrivilegeEnum = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeLockMemory: WbemPrivilegeEnum = 3i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeIncreaseQuota: WbemPrivilegeEnum = 4i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeMachineAccount: WbemPrivilegeEnum = 5i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeTcb: WbemPrivilegeEnum = 6i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeSecurity: WbemPrivilegeEnum = 7i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeTakeOwnership: WbemPrivilegeEnum = 8i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeLoadDriver: WbemPrivilegeEnum = 9i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeSystemProfile: WbemPrivilegeEnum = 10i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeSystemtime: WbemPrivilegeEnum = 11i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeProfileSingleProcess: WbemPrivilegeEnum = 12i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeIncreaseBasePriority: WbemPrivilegeEnum = 13i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeCreatePagefile: WbemPrivilegeEnum = 14i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeCreatePermanent: WbemPrivilegeEnum = 15i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeBackup: WbemPrivilegeEnum = 16i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeRestore: WbemPrivilegeEnum = 17i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeShutdown: WbemPrivilegeEnum = 18i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeDebug: WbemPrivilegeEnum = 19i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeAudit: WbemPrivilegeEnum = 20i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeSystemEnvironment: WbemPrivilegeEnum = 21i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeChangeNotify: WbemPrivilegeEnum = 22i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeRemoteShutdown: WbemPrivilegeEnum = 23i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeUndock: WbemPrivilegeEnum = 24i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeSyncAgent: WbemPrivilegeEnum = 25i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeEnableDelegation: WbemPrivilegeEnum = 26i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeManageVolume: WbemPrivilegeEnum = 27i32;
pub const WbemQuery: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3939016740, data2: 8674, data3: 17699, data4: [173, 115, 167, 26, 10, 162, 245, 106] };
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WbemQueryFlagEnum = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemQueryFlagDeep: WbemQueryFlagEnum = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemQueryFlagShallow: WbemQueryFlagEnum = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemQueryFlagPrototype: WbemQueryFlagEnum = 2i32;
pub const WbemRefresher: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3340068594, data2: 22046, data3: 4561, data4: [173, 135, 0, 192, 79, 216, 253, 255] };
pub const WbemStatusCodeText: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3951550909, data2: 12851, data3: 4562, data4: [174, 201, 0, 192, 79, 182, 136, 32] };
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WbemTextFlagEnum = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemTextFlagNoFlavors: WbemTextFlagEnum = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type WbemTimeout = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemTimeoutInfinite: WbemTimeout = -1i32;
pub const WbemUnauthenticatedLocator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1144945529, data2: 56881, data3: 4562, data4: [179, 64, 0, 16, 75, 204, 75, 74] };
pub const WbemUninitializedClassObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2046961654, data2: 28936, data3: 4561, data4: [173, 144, 0, 192, 79, 216, 253, 255] };
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type tag_WBEM_LOGIN_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_INPROC_LOGIN: tag_WBEM_LOGIN_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_LOCAL_LOGIN: tag_WBEM_LOGIN_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_REMOTE_LOGIN: tag_WBEM_LOGIN_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_AUTHENTICATION_METHOD_MASK: tag_WBEM_LOGIN_TYPE = 15i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_USE_MULTIPLE_CHALLENGES: tag_WBEM_LOGIN_TYPE = 16i32;
