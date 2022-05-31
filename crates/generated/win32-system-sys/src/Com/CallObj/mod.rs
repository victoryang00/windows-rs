#[link(name = "windows")]
extern "system" {
    pub fn CoGetInterceptor(iidintercepted: *const ::windows_core_sys::GUID, punkouter: ::windows_core_sys::IUnknown, iid: *const ::windows_core_sys::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn CoGetInterceptorFromTypeInfo(iidintercepted: *const ::windows_core_sys::GUID, punkouter: ::windows_core_sys::IUnknown, typeinfo: super::ITypeInfo, iid: *const ::windows_core_sys::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
}
#[repr(C)]
pub struct CALLFRAMEINFO {
    pub iMethod: u32,
    pub fHasInValues: ::win32_foundation_sys::BOOL,
    pub fHasInOutValues: ::win32_foundation_sys::BOOL,
    pub fHasOutValues: ::win32_foundation_sys::BOOL,
    pub fDerivesFromIDispatch: ::win32_foundation_sys::BOOL,
    pub cInInterfacesMax: i32,
    pub cInOutInterfacesMax: i32,
    pub cOutInterfacesMax: i32,
    pub cTopLevelInInterfaces: i32,
    pub iid: ::windows_core_sys::GUID,
    pub cMethod: u32,
    pub cParams: u32,
}
impl ::core::marker::Copy for CALLFRAMEINFO {}
impl ::core::clone::Clone for CALLFRAMEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CALLFRAMEPARAMINFO {
    pub fIn: ::win32_foundation_sys::BOOLEAN,
    pub fOut: ::win32_foundation_sys::BOOLEAN,
    pub stackOffset: u32,
    pub cbParam: u32,
}
impl ::core::marker::Copy for CALLFRAMEPARAMINFO {}
impl ::core::clone::Clone for CALLFRAMEPARAMINFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CALLFRAME_COPY = i32;
pub const CALLFRAME_COPY_NESTED: CALLFRAME_COPY = 1i32;
pub const CALLFRAME_COPY_INDEPENDENT: CALLFRAME_COPY = 2i32;
pub type CALLFRAME_FREE = i32;
pub const CALLFRAME_FREE_NONE: CALLFRAME_FREE = 0i32;
pub const CALLFRAME_FREE_IN: CALLFRAME_FREE = 1i32;
pub const CALLFRAME_FREE_INOUT: CALLFRAME_FREE = 2i32;
pub const CALLFRAME_FREE_OUT: CALLFRAME_FREE = 4i32;
pub const CALLFRAME_FREE_TOP_INOUT: CALLFRAME_FREE = 8i32;
pub const CALLFRAME_FREE_TOP_OUT: CALLFRAME_FREE = 16i32;
pub const CALLFRAME_FREE_ALL: CALLFRAME_FREE = 31i32;
#[repr(C)]
pub struct CALLFRAME_MARSHALCONTEXT {
    pub fIn: ::win32_foundation_sys::BOOLEAN,
    pub dwDestContext: u32,
    pub pvDestContext: *mut ::core::ffi::c_void,
    pub punkReserved: ::windows_core_sys::IUnknown,
    pub guidTransferSyntax: ::windows_core_sys::GUID,
}
impl ::core::marker::Copy for CALLFRAME_MARSHALCONTEXT {}
impl ::core::clone::Clone for CALLFRAME_MARSHALCONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CALLFRAME_NULL = i32;
pub const CALLFRAME_NULL_NONE: CALLFRAME_NULL = 0i32;
pub const CALLFRAME_NULL_INOUT: CALLFRAME_NULL = 2i32;
pub const CALLFRAME_NULL_OUT: CALLFRAME_NULL = 4i32;
pub const CALLFRAME_NULL_ALL: CALLFRAME_NULL = 6i32;
pub type CALLFRAME_WALK = i32;
pub const CALLFRAME_WALK_IN: CALLFRAME_WALK = 1i32;
pub const CALLFRAME_WALK_INOUT: CALLFRAME_WALK = 2i32;
pub const CALLFRAME_WALK_OUT: CALLFRAME_WALK = 4i32;
pub type ICallFrame = *mut ::core::ffi::c_void;
pub type ICallFrameEvents = *mut ::core::ffi::c_void;
pub type ICallFrameWalker = *mut ::core::ffi::c_void;
pub type ICallIndirect = *mut ::core::ffi::c_void;
pub type ICallInterceptor = *mut ::core::ffi::c_void;
pub type ICallUnmarshal = *mut ::core::ffi::c_void;
pub type IInterfaceRelated = *mut ::core::ffi::c_void;
