#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
    pub fn CoGetInterceptor(iidintercepted: *const ::windows_sys::core::GUID, punkouter: *mut *mut ::windows_sys::core::IUnknown, iid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
    pub fn CoGetInterceptorFromTypeInfo(iidintercepted: *const ::windows_sys::core::GUID, punkouter: *mut *mut ::windows_sys::core::IUnknown, typeinfo: *mut *mut super::ITypeInfo, iid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CALLFRAMEINFO {
    pub iMethod: u32,
    pub fHasInValues: super::super::super::Foundation::BOOL,
    pub fHasInOutValues: super::super::super::Foundation::BOOL,
    pub fHasOutValues: super::super::super::Foundation::BOOL,
    pub fDerivesFromIDispatch: super::super::super::Foundation::BOOL,
    pub cInInterfacesMax: i32,
    pub cInOutInterfacesMax: i32,
    pub cOutInterfacesMax: i32,
    pub cTopLevelInInterfaces: i32,
    pub iid: ::windows_sys::core::GUID,
    pub cMethod: u32,
    pub cParams: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CALLFRAMEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CALLFRAMEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CALLFRAMEPARAMINFO {
    pub fIn: super::super::super::Foundation::BOOLEAN,
    pub fOut: super::super::super::Foundation::BOOLEAN,
    pub stackOffset: u32,
    pub cbParam: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CALLFRAMEPARAMINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CALLFRAMEPARAMINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub type CALLFRAME_COPY = i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_COPY_NESTED: CALLFRAME_COPY = 1i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_COPY_INDEPENDENT: CALLFRAME_COPY = 2i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub type CALLFRAME_FREE = i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_FREE_NONE: CALLFRAME_FREE = 0i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_FREE_IN: CALLFRAME_FREE = 1i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_FREE_INOUT: CALLFRAME_FREE = 2i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_FREE_OUT: CALLFRAME_FREE = 4i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_FREE_TOP_INOUT: CALLFRAME_FREE = 8i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_FREE_TOP_OUT: CALLFRAME_FREE = 16i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_FREE_ALL: CALLFRAME_FREE = 31i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CALLFRAME_MARSHALCONTEXT {
    pub fIn: super::super::super::Foundation::BOOLEAN,
    pub dwDestContext: u32,
    pub pvDestContext: *mut ::core::ffi::c_void,
    pub punkReserved: *mut *mut *mut *mut ::windows_sys::core::IUnknown,
    pub guidTransferSyntax: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CALLFRAME_MARSHALCONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CALLFRAME_MARSHALCONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub type CALLFRAME_NULL = i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_NULL_NONE: CALLFRAME_NULL = 0i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_NULL_INOUT: CALLFRAME_NULL = 2i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_NULL_OUT: CALLFRAME_NULL = 4i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_NULL_ALL: CALLFRAME_NULL = 6i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub type CALLFRAME_WALK = i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_WALK_IN: CALLFRAME_WALK = 1i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_WALK_INOUT: CALLFRAME_WALK = 2i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_WALK_OUT: CALLFRAME_WALK = 4i32;
#[repr(C)]
pub struct ICallFrame {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetInfo: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *mut CALLFRAMEINFO) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetInfo: usize,
    pub GetIIDAndMethod: unsafe extern "system" fn(this: *mut *mut Self, piid: *mut ::windows_sys::core::GUID, pimethod: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetNames: unsafe extern "system" fn(this: *mut *mut Self, pwszinterface: *mut ::windows_sys::core::PWSTR, pwszmethod: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetStackLocation: unsafe extern "system" fn(this: *mut *mut Self) -> *mut ::core::ffi::c_void,
    pub SetStackLocation: unsafe extern "system" fn(this: *mut *mut Self, pvstack: *const ::core::ffi::c_void),
    pub SetReturnValue: unsafe extern "system" fn(this: *mut *mut Self, hr: ::windows_sys::core::HRESULT),
    pub GetReturnValue: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetParamInfo: unsafe extern "system" fn(this: *mut *mut Self, iparam: u32, pinfo: *mut CALLFRAMEPARAMINFO) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetParamInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub SetParam: unsafe extern "system" fn(this: *mut *mut Self, iparam: u32, pvar: *const super::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    SetParam: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetParam: unsafe extern "system" fn(this: *mut *mut Self, iparam: u32, pvar: *mut super::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetParam: usize,
    pub Copy: unsafe extern "system" fn(this: *mut *mut Self, copycontrol: CALLFRAME_COPY, pwalker: *mut ::core::ffi::c_void, ppframe: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Free: unsafe extern "system" fn(this: *mut *mut Self, pframeargsdest: *mut ::core::ffi::c_void, pwalkerdestfree: *mut ::core::ffi::c_void, pwalkercopy: *mut ::core::ffi::c_void, freeflags: u32, pwalkerfree: *mut ::core::ffi::c_void, nullflags: u32) -> ::windows_sys::core::HRESULT,
    pub FreeParam: unsafe extern "system" fn(this: *mut *mut Self, iparam: u32, freeflags: u32, pwalkerfree: *mut ::core::ffi::c_void, nullflags: u32) -> ::windows_sys::core::HRESULT,
    pub WalkFrame: unsafe extern "system" fn(this: *mut *mut Self, walkwhat: u32, pwalker: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMarshalSizeMax: unsafe extern "system" fn(this: *mut *mut Self, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pcbbufferneeded: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMarshalSizeMax: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Marshal: unsafe extern "system" fn(this: *mut *mut Self, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, pcbbufferused: *mut u32, pdatarep: *mut u32, prpcflags: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Marshal: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Unmarshal: unsafe extern "system" fn(this: *mut *mut Self, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Unmarshal: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReleaseMarshalData: unsafe extern "system" fn(this: *mut *mut Self, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReleaseMarshalData: usize,
    pub Invoke: unsafe extern "system" fn(this: *mut *mut Self, pvreceiver: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICallFrame {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3581129904, data2: 35150, data3: 4562, data4: [184, 182, 0, 192, 79, 185, 97, 138] };
}
#[repr(C)]
pub struct ICallFrameEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnCall: unsafe extern "system" fn(this: *mut *mut Self, pframe: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICallFrameEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4250798147, data2: 64657, data3: 4560, data4: [151, 215, 0, 192, 79, 185, 97, 138] };
}
#[repr(C)]
pub struct ICallFrameWalker {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub OnWalkInterface: unsafe extern "system" fn(this: *mut *mut Self, iid: *const ::windows_sys::core::GUID, ppvinterface: *const *const ::core::ffi::c_void, fin: super::super::super::Foundation::BOOL, fout: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnWalkInterface: usize,
}
impl ::windows_sys::core::Interface for ICallFrameWalker {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 145897753, data2: 14637, data3: 4562, data4: [184, 164, 0, 192, 79, 185, 97, 138] };
}
#[repr(C)]
pub struct ICallIndirect {
    pub base__: ::windows_sys::core::IUnknown,
    pub CallIndirect: unsafe extern "system" fn(this: *mut *mut Self, phrreturn: *mut ::windows_sys::core::HRESULT, imethod: u32, pvargs: *const ::core::ffi::c_void, cbargs: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMethodInfo: unsafe extern "system" fn(this: *mut *mut Self, imethod: u32, pinfo: *mut CALLFRAMEINFO, pwszmethod: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMethodInfo: usize,
    pub GetStackSize: unsafe extern "system" fn(this: *mut *mut Self, imethod: u32, cbargs: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIID: unsafe extern "system" fn(this: *mut *mut Self, piid: *mut ::windows_sys::core::GUID, pfderivesfromidispatch: *mut super::super::super::Foundation::BOOL, pcmethod: *mut u32, pwszinterface: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIID: usize,
}
impl ::windows_sys::core::Interface for ICallIndirect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3581129905, data2: 35150, data3: 4562, data4: [184, 182, 0, 192, 79, 185, 97, 138] };
}
#[repr(C)]
pub struct ICallInterceptor {
    pub base__: ICallIndirect,
    pub RegisterSink: unsafe extern "system" fn(this: *mut *mut Self, psink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetRegisteredSink: unsafe extern "system" fn(this: *mut *mut Self, ppsink: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICallInterceptor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1623706229, data2: 35181, data3: 4562, data4: [184, 182, 0, 192, 79, 185, 97, 138] };
}
#[repr(C)]
pub struct ICallUnmarshal {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Unmarshal: unsafe extern "system" fn(this: *mut *mut Self, imethod: u32, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, fforcebuffercopy: super::super::super::Foundation::BOOL, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32, ppframe: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Unmarshal: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReleaseMarshalData: unsafe extern "system" fn(this: *mut *mut Self, imethod: u32, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReleaseMarshalData: usize,
}
impl ::windows_sys::core::Interface for ICallUnmarshal {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1395896323, data2: 11842, data3: 4562, data4: [184, 157, 0, 192, 79, 185, 97, 138] };
}
#[repr(C)]
pub struct IInterfaceRelated {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetIID: unsafe extern "system" fn(this: *mut *mut Self, iid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetIID: unsafe extern "system" fn(this: *mut *mut Self, piid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInterfaceRelated {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3522910841, data2: 30470, data3: 4561, data4: [173, 186, 0, 192, 79, 194, 173, 192] };
}
