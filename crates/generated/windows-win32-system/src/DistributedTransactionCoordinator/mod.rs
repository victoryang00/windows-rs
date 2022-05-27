#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct APPLICATIONTYPE(pub i32);
pub const LOCAL_APPLICATIONTYPE: APPLICATIONTYPE = APPLICATIONTYPE(0i32);
pub const CLUSTERRESOURCE_APPLICATIONTYPE: APPLICATIONTYPE = APPLICATIONTYPE(1i32);
impl ::core::marker::Copy for APPLICATIONTYPE {}
impl ::core::clone::Clone for APPLICATIONTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APPLICATIONTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for APPLICATIONTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for APPLICATIONTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APPLICATIONTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUTHENTICATION_LEVEL(pub i32);
pub const NO_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = AUTHENTICATION_LEVEL(0i32);
pub const INCOMING_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = AUTHENTICATION_LEVEL(1i32);
pub const MUTUAL_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = AUTHENTICATION_LEVEL(2i32);
impl ::core::marker::Copy for AUTHENTICATION_LEVEL {}
impl ::core::clone::Clone for AUTHENTICATION_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUTHENTICATION_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AUTHENTICATION_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for AUTHENTICATION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHENTICATION_LEVEL").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct BOID {
    pub rgb: [u8; 16],
}
impl ::core::marker::Copy for BOID {}
impl ::core::clone::Clone for BOID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BOID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BOID").field("rgb", &self.rgb).finish()
    }
}
unsafe impl ::windows_core::Abi for BOID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BOID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BOID>()) == 0 }
    }
}
impl ::core::cmp::Eq for BOID {}
impl ::core::default::Default for BOID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const CLSID_MSDtcTransaction: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x39f8d76b_0928_11d1_97df_00c04fb9618a);
pub const CLSID_MSDtcTransactionManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5b18ab61_091d_11d1_97df_00c04fb9618a);
pub const DTCINSTALL_E_CLIENT_ALREADY_INSTALLED: i32 = 384i32;
pub const DTCINSTALL_E_SERVER_ALREADY_INSTALLED: i32 = 385i32;
pub type DTC_GET_TRANSACTION_MANAGER = ::core::option::Option<unsafe extern "system" fn(pszhost: ::windows_core::PCSTR, psztmname: ::windows_core::PCSTR, rid: *const ::windows_core::GUID, dwreserved1: u32, wcbreserved2: u16, pvreserved2: *mut ::core::ffi::c_void, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT>;
pub type DTC_GET_TRANSACTION_MANAGER_EX_A = ::core::option::Option<unsafe extern "system" fn(i_pszhost: ::windows_core::PCSTR, i_psztmname: ::windows_core::PCSTR, i_riid: *const ::windows_core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT>;
pub type DTC_GET_TRANSACTION_MANAGER_EX_W = ::core::option::Option<unsafe extern "system" fn(i_pwszhost: ::windows_core::PCWSTR, i_pwsztmname: ::windows_core::PCWSTR, i_riid: *const ::windows_core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT>;
pub type DTC_INSTALL_CLIENT = ::core::option::Option<unsafe extern "system" fn(i_pszremotetmhostname: *mut i8, i_dwprotocol: u32, i_dwoverwrite: u32) -> ::windows_core::HRESULT>;
pub const DTC_INSTALL_OVERWRITE_CLIENT: u32 = 1u32;
pub const DTC_INSTALL_OVERWRITE_SERVER: u32 = 2u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DTC_STATUS_(pub i32);
pub const DTC_STATUS_UNKNOWN: DTC_STATUS_ = DTC_STATUS_(0i32);
pub const DTC_STATUS_STARTING: DTC_STATUS_ = DTC_STATUS_(1i32);
pub const DTC_STATUS_STARTED: DTC_STATUS_ = DTC_STATUS_(2i32);
pub const DTC_STATUS_PAUSING: DTC_STATUS_ = DTC_STATUS_(3i32);
pub const DTC_STATUS_PAUSED: DTC_STATUS_ = DTC_STATUS_(4i32);
pub const DTC_STATUS_CONTINUING: DTC_STATUS_ = DTC_STATUS_(5i32);
pub const DTC_STATUS_STOPPING: DTC_STATUS_ = DTC_STATUS_(6i32);
pub const DTC_STATUS_STOPPED: DTC_STATUS_ = DTC_STATUS_(7i32);
pub const DTC_STATUS_E_CANTCONTROL: DTC_STATUS_ = DTC_STATUS_(8i32);
pub const DTC_STATUS_FAILED: DTC_STATUS_ = DTC_STATUS_(9i32);
impl ::core::marker::Copy for DTC_STATUS_ {}
impl ::core::clone::Clone for DTC_STATUS_ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DTC_STATUS_ {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DTC_STATUS_ {
    type Abi = Self;
}
impl ::core::fmt::Debug for DTC_STATUS_ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DTC_STATUS_").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn DtcGetTransactionManager<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(i_pszhost: Param0, i_psztmname: Param1, i_riid: *const ::windows_core::GUID, i_dwreserved1: u32, i_wcbreserved2: u16, i_pvreserved2: *const ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DtcGetTransactionManager(i_pszhost: ::windows_core::PCSTR, i_psztmname: ::windows_core::PCSTR, i_riid: *const ::windows_core::GUID, i_dwreserved1: u32, i_wcbreserved2: u16, i_pvreserved2: *const ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        DtcGetTransactionManager(i_pszhost.into_param().abi(), i_psztmname.into_param().abi(), ::core::mem::transmute(i_riid), ::core::mem::transmute(i_dwreserved1), ::core::mem::transmute(i_wcbreserved2), ::core::mem::transmute(i_pvreserved2), ::core::mem::transmute(o_ppvobject)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DtcGetTransactionManagerC<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(i_pszhost: Param0, i_psztmname: Param1, i_riid: *const ::windows_core::GUID, i_dwreserved1: u32, i_wcbreserved2: u16, i_pvreserved2: *const ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DtcGetTransactionManagerC(i_pszhost: ::windows_core::PCSTR, i_psztmname: ::windows_core::PCSTR, i_riid: *const ::windows_core::GUID, i_dwreserved1: u32, i_wcbreserved2: u16, i_pvreserved2: *const ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        DtcGetTransactionManagerC(i_pszhost.into_param().abi(), i_psztmname.into_param().abi(), ::core::mem::transmute(i_riid), ::core::mem::transmute(i_dwreserved1), ::core::mem::transmute(i_wcbreserved2), ::core::mem::transmute(i_pvreserved2), ::core::mem::transmute(o_ppvobject)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DtcGetTransactionManagerExA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(i_pszhost: Param0, i_psztmname: Param1, i_riid: *const ::windows_core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DtcGetTransactionManagerExA(i_pszhost: ::windows_core::PCSTR, i_psztmname: ::windows_core::PCSTR, i_riid: *const ::windows_core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        DtcGetTransactionManagerExA(i_pszhost.into_param().abi(), i_psztmname.into_param().abi(), ::core::mem::transmute(i_riid), ::core::mem::transmute(i_grfoptions), ::core::mem::transmute(i_pvconfigparams), ::core::mem::transmute(o_ppvobject)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DtcGetTransactionManagerExW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(i_pwszhost: Param0, i_pwsztmname: Param1, i_riid: *const ::windows_core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DtcGetTransactionManagerExW(i_pwszhost: ::windows_core::PCWSTR, i_pwsztmname: ::windows_core::PCWSTR, i_riid: *const ::windows_core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        DtcGetTransactionManagerExW(i_pwszhost.into_param().abi(), i_pwsztmname.into_param().abi(), ::core::mem::transmute(i_riid), ::core::mem::transmute(i_grfoptions), ::core::mem::transmute(i_pvconfigparams), ::core::mem::transmute(o_ppvobject)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
pub struct IDtcLuConfigure(::windows_core::IUnknown);
impl IDtcLuConfigure {
    pub unsafe fn Add(&self, puclupair: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(puclupair)), puclupair.len() as _).ok()
    }
    pub unsafe fn Delete(&self, puclupair: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(puclupair)), puclupair.len() as _).ok()
    }
}
impl ::core::convert::From<IDtcLuConfigure> for ::windows_core::IUnknown {
    fn from(value: IDtcLuConfigure) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcLuConfigure> for ::windows_core::IUnknown {
    fn from(value: &IDtcLuConfigure) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDtcLuConfigure {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDtcLuConfigure {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDtcLuConfigure {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcLuConfigure {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuConfigure {}
impl ::core::fmt::Debug for IDtcLuConfigure {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuConfigure").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDtcLuConfigure {
    type Vtable = IDtcLuConfigure_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4131e760_1aea_11d0_944b_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuConfigure_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puclupair: *const u8, cblupair: u32) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puclupair: *const u8, cblupair: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDtcLuRecovery(::windows_core::IUnknown);
impl IDtcLuRecovery {}
impl ::core::convert::From<IDtcLuRecovery> for ::windows_core::IUnknown {
    fn from(value: IDtcLuRecovery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcLuRecovery> for ::windows_core::IUnknown {
    fn from(value: &IDtcLuRecovery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDtcLuRecovery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDtcLuRecovery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDtcLuRecovery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcLuRecovery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuRecovery {}
impl ::core::fmt::Debug for IDtcLuRecovery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuRecovery").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDtcLuRecovery {
    type Vtable = IDtcLuRecovery_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xac2b8ad2_d6f0_11d0_b386_00a0c9083365);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecovery_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
}
#[repr(transparent)]
pub struct IDtcLuRecoveryFactory(::windows_core::IUnknown);
impl IDtcLuRecoveryFactory {
    pub unsafe fn Create(&self, puclupair: &[u8]) -> ::windows_core::Result<IDtcLuRecovery> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(puclupair)), puclupair.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDtcLuRecovery>(result__)
    }
}
impl ::core::convert::From<IDtcLuRecoveryFactory> for ::windows_core::IUnknown {
    fn from(value: IDtcLuRecoveryFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcLuRecoveryFactory> for ::windows_core::IUnknown {
    fn from(value: &IDtcLuRecoveryFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDtcLuRecoveryFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDtcLuRecoveryFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDtcLuRecoveryFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcLuRecoveryFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuRecoveryFactory {}
impl ::core::fmt::Debug for IDtcLuRecoveryFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuRecoveryFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDtcLuRecoveryFactory {
    type Vtable = IDtcLuRecoveryFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4131e762_1aea_11d0_944b_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryFactory_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puclupair: *const u8, cblupair: u32, pprecovery: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDtcLuRecoveryInitiatedByDtc(::windows_core::IUnknown);
impl IDtcLuRecoveryInitiatedByDtc {
    pub unsafe fn GetWork(&self, pwork: *mut _DtcLu_LocalRecovery_Work, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetWork)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwork), ::core::mem::transmute(ppv)).ok()
    }
}
impl ::core::convert::From<IDtcLuRecoveryInitiatedByDtc> for ::windows_core::IUnknown {
    fn from(value: IDtcLuRecoveryInitiatedByDtc) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcLuRecoveryInitiatedByDtc> for ::windows_core::IUnknown {
    fn from(value: &IDtcLuRecoveryInitiatedByDtc) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDtcLuRecoveryInitiatedByDtc {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDtcLuRecoveryInitiatedByDtc {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDtcLuRecoveryInitiatedByDtc {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcLuRecoveryInitiatedByDtc {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuRecoveryInitiatedByDtc {}
impl ::core::fmt::Debug for IDtcLuRecoveryInitiatedByDtc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuRecoveryInitiatedByDtc").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDtcLuRecoveryInitiatedByDtc {
    type Vtable = IDtcLuRecoveryInitiatedByDtc_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4131e764_1aea_11d0_944b_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryInitiatedByDtc_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetWork: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwork: *mut _DtcLu_LocalRecovery_Work, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDtcLuRecoveryInitiatedByDtcStatusWork(::windows_core::IUnknown);
impl IDtcLuRecoveryInitiatedByDtcStatusWork {
    pub unsafe fn HandleCheckLuStatus(&self, lrecoveryseqnum: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).HandleCheckLuStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lrecoveryseqnum)).ok()
    }
}
impl ::core::convert::From<IDtcLuRecoveryInitiatedByDtcStatusWork> for ::windows_core::IUnknown {
    fn from(value: IDtcLuRecoveryInitiatedByDtcStatusWork) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcLuRecoveryInitiatedByDtcStatusWork> for ::windows_core::IUnknown {
    fn from(value: &IDtcLuRecoveryInitiatedByDtcStatusWork) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDtcLuRecoveryInitiatedByDtcStatusWork {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDtcLuRecoveryInitiatedByDtcStatusWork {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDtcLuRecoveryInitiatedByDtcStatusWork {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcLuRecoveryInitiatedByDtcStatusWork {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuRecoveryInitiatedByDtcStatusWork {}
impl ::core::fmt::Debug for IDtcLuRecoveryInitiatedByDtcStatusWork {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuRecoveryInitiatedByDtcStatusWork").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDtcLuRecoveryInitiatedByDtcStatusWork {
    type Vtable = IDtcLuRecoveryInitiatedByDtcStatusWork_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4131e766_1aea_11d0_944b_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryInitiatedByDtcStatusWork_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub HandleCheckLuStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lrecoveryseqnum: i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDtcLuRecoveryInitiatedByDtcTransWork(::windows_core::IUnknown);
impl IDtcLuRecoveryInitiatedByDtcTransWork {
    pub unsafe fn GetLogNameSizes(&self, pcbourlogname: *mut u32, pcbremotelogname: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetLogNameSizes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcbourlogname), ::core::mem::transmute(pcbremotelogname)).ok()
    }
    pub unsafe fn GetOurXln(&self, pxln: *mut _DtcLu_Xln, pourlogname: *mut u8, premotelogname: *mut u8, pdwprotocol: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetOurXln)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pxln), ::core::mem::transmute(pourlogname), ::core::mem::transmute(premotelogname), ::core::mem::transmute(pdwprotocol)).ok()
    }
    pub unsafe fn HandleConfirmationFromOurXln(&self, confirmation: _DtcLu_Xln_Confirmation) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).HandleConfirmationFromOurXln)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(confirmation)).ok()
    }
    pub unsafe fn HandleTheirXlnResponse(&self, xln: _DtcLu_Xln, premotelogname: *mut u8, cbremotelogname: u32, dwprotocol: u32, pconfirmation: *mut _DtcLu_Xln_Confirmation) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).HandleTheirXlnResponse)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(xln), ::core::mem::transmute(premotelogname), ::core::mem::transmute(cbremotelogname), ::core::mem::transmute(dwprotocol), ::core::mem::transmute(pconfirmation)).ok()
    }
    pub unsafe fn HandleErrorFromOurXln(&self, error: _DtcLu_Xln_Error) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).HandleErrorFromOurXln)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(error)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CheckForCompareStates(&self, fcomparestates: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CheckForCompareStates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(fcomparestates)).ok()
    }
    pub unsafe fn GetOurTransIdSize(&self, pcbourtransid: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetOurTransIdSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcbourtransid)).ok()
    }
    pub unsafe fn GetOurCompareStates(&self, pourtransid: *mut u8, pcomparestate: *mut _DtcLu_CompareState) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetOurCompareStates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pourtransid), ::core::mem::transmute(pcomparestate)).ok()
    }
    pub unsafe fn HandleTheirCompareStatesResponse(&self, comparestate: _DtcLu_CompareState, pconfirmation: *mut _DtcLu_CompareStates_Confirmation) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).HandleTheirCompareStatesResponse)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(comparestate), ::core::mem::transmute(pconfirmation)).ok()
    }
    pub unsafe fn HandleErrorFromOurCompareStates(&self, error: _DtcLu_CompareStates_Error) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).HandleErrorFromOurCompareStates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(error)).ok()
    }
    pub unsafe fn ConversationLost(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ConversationLost)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetRecoverySeqNum(&self, plrecoveryseqnum: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRecoverySeqNum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(plrecoveryseqnum)).ok()
    }
    pub unsafe fn ObsoleteRecoverySeqNum(&self, lnewrecoveryseqnum: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ObsoleteRecoverySeqNum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lnewrecoveryseqnum)).ok()
    }
}
impl ::core::convert::From<IDtcLuRecoveryInitiatedByDtcTransWork> for ::windows_core::IUnknown {
    fn from(value: IDtcLuRecoveryInitiatedByDtcTransWork) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcLuRecoveryInitiatedByDtcTransWork> for ::windows_core::IUnknown {
    fn from(value: &IDtcLuRecoveryInitiatedByDtcTransWork) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDtcLuRecoveryInitiatedByDtcTransWork {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDtcLuRecoveryInitiatedByDtcTransWork {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDtcLuRecoveryInitiatedByDtcTransWork {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcLuRecoveryInitiatedByDtcTransWork {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuRecoveryInitiatedByDtcTransWork {}
impl ::core::fmt::Debug for IDtcLuRecoveryInitiatedByDtcTransWork {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuRecoveryInitiatedByDtcTransWork").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDtcLuRecoveryInitiatedByDtcTransWork {
    type Vtable = IDtcLuRecoveryInitiatedByDtcTransWork_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4131e765_1aea_11d0_944b_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryInitiatedByDtcTransWork_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetLogNameSizes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbourlogname: *mut u32, pcbremotelogname: *mut u32) -> ::windows_core::HRESULT,
    pub GetOurXln: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pxln: *mut _DtcLu_Xln, pourlogname: *mut u8, premotelogname: *mut u8, pdwprotocol: *mut u32) -> ::windows_core::HRESULT,
    pub HandleConfirmationFromOurXln: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, confirmation: _DtcLu_Xln_Confirmation) -> ::windows_core::HRESULT,
    pub HandleTheirXlnResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xln: _DtcLu_Xln, premotelogname: *mut u8, cbremotelogname: u32, dwprotocol: u32, pconfirmation: *mut _DtcLu_Xln_Confirmation) -> ::windows_core::HRESULT,
    pub HandleErrorFromOurXln: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, error: _DtcLu_Xln_Error) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CheckForCompareStates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fcomparestates: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CheckForCompareStates: usize,
    pub GetOurTransIdSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbourtransid: *mut u32) -> ::windows_core::HRESULT,
    pub GetOurCompareStates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pourtransid: *mut u8, pcomparestate: *mut _DtcLu_CompareState) -> ::windows_core::HRESULT,
    pub HandleTheirCompareStatesResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, comparestate: _DtcLu_CompareState, pconfirmation: *mut _DtcLu_CompareStates_Confirmation) -> ::windows_core::HRESULT,
    pub HandleErrorFromOurCompareStates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, error: _DtcLu_CompareStates_Error) -> ::windows_core::HRESULT,
    pub ConversationLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetRecoverySeqNum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plrecoveryseqnum: *mut i32) -> ::windows_core::HRESULT,
    pub ObsoleteRecoverySeqNum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnewrecoveryseqnum: i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDtcLuRecoveryInitiatedByLu(::windows_core::IUnknown);
impl IDtcLuRecoveryInitiatedByLu {
    pub unsafe fn GetObjectToHandleWorkFromLu(&self) -> ::windows_core::Result<IDtcLuRecoveryInitiatedByLuWork> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetObjectToHandleWorkFromLu)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDtcLuRecoveryInitiatedByLuWork>(result__)
    }
}
impl ::core::convert::From<IDtcLuRecoveryInitiatedByLu> for ::windows_core::IUnknown {
    fn from(value: IDtcLuRecoveryInitiatedByLu) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcLuRecoveryInitiatedByLu> for ::windows_core::IUnknown {
    fn from(value: &IDtcLuRecoveryInitiatedByLu) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDtcLuRecoveryInitiatedByLu {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDtcLuRecoveryInitiatedByLu {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDtcLuRecoveryInitiatedByLu {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcLuRecoveryInitiatedByLu {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuRecoveryInitiatedByLu {}
impl ::core::fmt::Debug for IDtcLuRecoveryInitiatedByLu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuRecoveryInitiatedByLu").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDtcLuRecoveryInitiatedByLu {
    type Vtable = IDtcLuRecoveryInitiatedByLu_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4131e768_1aea_11d0_944b_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryInitiatedByLu_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetObjectToHandleWorkFromLu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwork: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDtcLuRecoveryInitiatedByLuWork(::windows_core::IUnknown);
impl IDtcLuRecoveryInitiatedByLuWork {
    pub unsafe fn HandleTheirXln(&self, lrecoveryseqnum: i32, xln: _DtcLu_Xln, premotelogname: *mut u8, cbremotelogname: u32, pourlogname: *mut u8, cbourlogname: u32, dwprotocol: u32, presponse: *mut _DtcLu_Xln_Response) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).HandleTheirXln)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lrecoveryseqnum), ::core::mem::transmute(xln), ::core::mem::transmute(premotelogname), ::core::mem::transmute(cbremotelogname), ::core::mem::transmute(pourlogname), ::core::mem::transmute(cbourlogname), ::core::mem::transmute(dwprotocol), ::core::mem::transmute(presponse)).ok()
    }
    pub unsafe fn GetOurLogNameSize(&self, pcbourlogname: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetOurLogNameSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcbourlogname)).ok()
    }
    pub unsafe fn GetOurXln(&self, pxln: *mut _DtcLu_Xln, pourlogname: *mut u8, pdwprotocol: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetOurXln)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pxln), ::core::mem::transmute(pourlogname), ::core::mem::transmute(pdwprotocol)).ok()
    }
    pub unsafe fn HandleConfirmationOfOurXln(&self, confirmation: _DtcLu_Xln_Confirmation) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).HandleConfirmationOfOurXln)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(confirmation)).ok()
    }
    pub unsafe fn HandleTheirCompareStates(&self, premotetransid: *mut u8, cbremotetransid: u32, comparestate: _DtcLu_CompareState, presponse: *mut _DtcLu_CompareStates_Response, pcomparestate: *mut _DtcLu_CompareState) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).HandleTheirCompareStates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(premotetransid), ::core::mem::transmute(cbremotetransid), ::core::mem::transmute(comparestate), ::core::mem::transmute(presponse), ::core::mem::transmute(pcomparestate)).ok()
    }
    pub unsafe fn HandleConfirmationOfOurCompareStates(&self, confirmation: _DtcLu_CompareStates_Confirmation) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).HandleConfirmationOfOurCompareStates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(confirmation)).ok()
    }
    pub unsafe fn HandleErrorFromOurCompareStates(&self, error: _DtcLu_CompareStates_Error) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).HandleErrorFromOurCompareStates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(error)).ok()
    }
    pub unsafe fn ConversationLost(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ConversationLost)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IDtcLuRecoveryInitiatedByLuWork> for ::windows_core::IUnknown {
    fn from(value: IDtcLuRecoveryInitiatedByLuWork) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcLuRecoveryInitiatedByLuWork> for ::windows_core::IUnknown {
    fn from(value: &IDtcLuRecoveryInitiatedByLuWork) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDtcLuRecoveryInitiatedByLuWork {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDtcLuRecoveryInitiatedByLuWork {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDtcLuRecoveryInitiatedByLuWork {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcLuRecoveryInitiatedByLuWork {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuRecoveryInitiatedByLuWork {}
impl ::core::fmt::Debug for IDtcLuRecoveryInitiatedByLuWork {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuRecoveryInitiatedByLuWork").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDtcLuRecoveryInitiatedByLuWork {
    type Vtable = IDtcLuRecoveryInitiatedByLuWork_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xac2b8ad1_d6f0_11d0_b386_00a0c9083365);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryInitiatedByLuWork_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub HandleTheirXln: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lrecoveryseqnum: i32, xln: _DtcLu_Xln, premotelogname: *mut u8, cbremotelogname: u32, pourlogname: *mut u8, cbourlogname: u32, dwprotocol: u32, presponse: *mut _DtcLu_Xln_Response) -> ::windows_core::HRESULT,
    pub GetOurLogNameSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbourlogname: *mut u32) -> ::windows_core::HRESULT,
    pub GetOurXln: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pxln: *mut _DtcLu_Xln, pourlogname: *mut u8, pdwprotocol: *mut u32) -> ::windows_core::HRESULT,
    pub HandleConfirmationOfOurXln: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, confirmation: _DtcLu_Xln_Confirmation) -> ::windows_core::HRESULT,
    pub HandleTheirCompareStates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, premotetransid: *mut u8, cbremotetransid: u32, comparestate: _DtcLu_CompareState, presponse: *mut _DtcLu_CompareStates_Response, pcomparestate: *mut _DtcLu_CompareState) -> ::windows_core::HRESULT,
    pub HandleConfirmationOfOurCompareStates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, confirmation: _DtcLu_CompareStates_Confirmation) -> ::windows_core::HRESULT,
    pub HandleErrorFromOurCompareStates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, error: _DtcLu_CompareStates_Error) -> ::windows_core::HRESULT,
    pub ConversationLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDtcLuRmEnlistment(::windows_core::IUnknown);
impl IDtcLuRmEnlistment {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unplug<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fconversationlost: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Unplug)(::windows_core::Interface::as_raw(self), fconversationlost.into_param().abi()).ok()
    }
    pub unsafe fn BackedOut(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BackedOut)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BackOut(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BackOut)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Committed(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Committed)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Forget(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Forget)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RequestCommit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RequestCommit)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IDtcLuRmEnlistment> for ::windows_core::IUnknown {
    fn from(value: IDtcLuRmEnlistment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcLuRmEnlistment> for ::windows_core::IUnknown {
    fn from(value: &IDtcLuRmEnlistment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDtcLuRmEnlistment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDtcLuRmEnlistment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDtcLuRmEnlistment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcLuRmEnlistment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuRmEnlistment {}
impl ::core::fmt::Debug for IDtcLuRmEnlistment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuRmEnlistment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDtcLuRmEnlistment {
    type Vtable = IDtcLuRmEnlistment_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4131e769_1aea_11d0_944b_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRmEnlistment_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Unplug: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fconversationlost: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Unplug: usize,
    pub BackedOut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub BackOut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Committed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Forget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RequestCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDtcLuRmEnlistmentFactory(::windows_core::IUnknown);
impl IDtcLuRmEnlistmentFactory {
    pub unsafe fn Create<'a, Param2: ::windows_core::IntoParam<'a, ITransaction>, Param5: ::windows_core::IntoParam<'a, IDtcLuRmEnlistmentSink>>(&self, puclupair: *mut u8, cblupair: u32, pitransaction: Param2, ptransid: *mut u8, cbtransid: u32, prmenlistmentsink: Param5, pprmenlistment: *mut ::core::option::Option<IDtcLuRmEnlistment>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(puclupair), ::core::mem::transmute(cblupair), pitransaction.into_param().abi(), ::core::mem::transmute(ptransid), ::core::mem::transmute(cbtransid), prmenlistmentsink.into_param().abi(), ::core::mem::transmute(pprmenlistment)).ok()
    }
}
impl ::core::convert::From<IDtcLuRmEnlistmentFactory> for ::windows_core::IUnknown {
    fn from(value: IDtcLuRmEnlistmentFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcLuRmEnlistmentFactory> for ::windows_core::IUnknown {
    fn from(value: &IDtcLuRmEnlistmentFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDtcLuRmEnlistmentFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDtcLuRmEnlistmentFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDtcLuRmEnlistmentFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcLuRmEnlistmentFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuRmEnlistmentFactory {}
impl ::core::fmt::Debug for IDtcLuRmEnlistmentFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuRmEnlistmentFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDtcLuRmEnlistmentFactory {
    type Vtable = IDtcLuRmEnlistmentFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4131e771_1aea_11d0_944b_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRmEnlistmentFactory_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puclupair: *mut u8, cblupair: u32, pitransaction: ::windows_core::RawPtr, ptransid: *mut u8, cbtransid: u32, prmenlistmentsink: ::windows_core::RawPtr, pprmenlistment: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDtcLuRmEnlistmentSink(::windows_core::IUnknown);
impl IDtcLuRmEnlistmentSink {
    pub unsafe fn AckUnplug(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AckUnplug)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn TmDown(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TmDown)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SessionLost(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SessionLost)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BackedOut(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BackedOut)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BackOut(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BackOut)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Committed(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Committed)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Forget(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Forget)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Prepare(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Prepare)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RequestCommit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RequestCommit)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IDtcLuRmEnlistmentSink> for ::windows_core::IUnknown {
    fn from(value: IDtcLuRmEnlistmentSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcLuRmEnlistmentSink> for ::windows_core::IUnknown {
    fn from(value: &IDtcLuRmEnlistmentSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDtcLuRmEnlistmentSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDtcLuRmEnlistmentSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDtcLuRmEnlistmentSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcLuRmEnlistmentSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuRmEnlistmentSink {}
impl ::core::fmt::Debug for IDtcLuRmEnlistmentSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuRmEnlistmentSink").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDtcLuRmEnlistmentSink {
    type Vtable = IDtcLuRmEnlistmentSink_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4131e770_1aea_11d0_944b_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRmEnlistmentSink_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AckUnplug: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TmDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SessionLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub BackedOut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub BackOut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Committed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Forget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Prepare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RequestCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDtcLuSubordinateDtc(::windows_core::IUnknown);
impl IDtcLuSubordinateDtc {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unplug<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fconversationlost: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Unplug)(::windows_core::Interface::as_raw(self), fconversationlost.into_param().abi()).ok()
    }
    pub unsafe fn BackedOut(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BackedOut)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BackOut(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BackOut)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Committed(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Committed)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Forget(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Forget)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Prepare(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Prepare)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RequestCommit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RequestCommit)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IDtcLuSubordinateDtc> for ::windows_core::IUnknown {
    fn from(value: IDtcLuSubordinateDtc) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcLuSubordinateDtc> for ::windows_core::IUnknown {
    fn from(value: &IDtcLuSubordinateDtc) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDtcLuSubordinateDtc {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDtcLuSubordinateDtc {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDtcLuSubordinateDtc {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcLuSubordinateDtc {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuSubordinateDtc {}
impl ::core::fmt::Debug for IDtcLuSubordinateDtc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuSubordinateDtc").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDtcLuSubordinateDtc {
    type Vtable = IDtcLuSubordinateDtc_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4131e773_1aea_11d0_944b_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuSubordinateDtc_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Unplug: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fconversationlost: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Unplug: usize,
    pub BackedOut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub BackOut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Committed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Forget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Prepare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RequestCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDtcLuSubordinateDtcFactory(::windows_core::IUnknown);
impl IDtcLuSubordinateDtcFactory {
    pub unsafe fn Create<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param5: ::windows_core::IntoParam<'a, ITransactionOptions>, Param9: ::windows_core::IntoParam<'a, IDtcLuSubordinateDtcSink>>(&self, puclupair: *mut u8, cblupair: u32, punktransactionouter: Param2, isolevel: i32, isoflags: u32, poptions: Param5, pptransaction: *mut ::core::option::Option<ITransaction>, ptransid: *mut u8, cbtransid: u32, psubordinatedtcsink: Param9, ppsubordinatedtc: *mut ::core::option::Option<IDtcLuSubordinateDtc>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(puclupair), ::core::mem::transmute(cblupair), punktransactionouter.into_param().abi(), ::core::mem::transmute(isolevel), ::core::mem::transmute(isoflags), poptions.into_param().abi(), ::core::mem::transmute(pptransaction), ::core::mem::transmute(ptransid), ::core::mem::transmute(cbtransid), psubordinatedtcsink.into_param().abi(), ::core::mem::transmute(ppsubordinatedtc)).ok()
    }
}
impl ::core::convert::From<IDtcLuSubordinateDtcFactory> for ::windows_core::IUnknown {
    fn from(value: IDtcLuSubordinateDtcFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcLuSubordinateDtcFactory> for ::windows_core::IUnknown {
    fn from(value: &IDtcLuSubordinateDtcFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDtcLuSubordinateDtcFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDtcLuSubordinateDtcFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDtcLuSubordinateDtcFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcLuSubordinateDtcFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuSubordinateDtcFactory {}
impl ::core::fmt::Debug for IDtcLuSubordinateDtcFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuSubordinateDtcFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDtcLuSubordinateDtcFactory {
    type Vtable = IDtcLuSubordinateDtcFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4131e775_1aea_11d0_944b_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuSubordinateDtcFactory_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puclupair: *mut u8, cblupair: u32, punktransactionouter: *mut ::core::ffi::c_void, isolevel: i32, isoflags: u32, poptions: ::windows_core::RawPtr, pptransaction: *mut ::windows_core::RawPtr, ptransid: *mut u8, cbtransid: u32, psubordinatedtcsink: ::windows_core::RawPtr, ppsubordinatedtc: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDtcLuSubordinateDtcSink(::windows_core::IUnknown);
impl IDtcLuSubordinateDtcSink {
    pub unsafe fn AckUnplug(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AckUnplug)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn TmDown(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TmDown)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SessionLost(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SessionLost)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BackedOut(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BackedOut)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BackOut(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BackOut)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Committed(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Committed)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Forget(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Forget)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RequestCommit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RequestCommit)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IDtcLuSubordinateDtcSink> for ::windows_core::IUnknown {
    fn from(value: IDtcLuSubordinateDtcSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcLuSubordinateDtcSink> for ::windows_core::IUnknown {
    fn from(value: &IDtcLuSubordinateDtcSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDtcLuSubordinateDtcSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDtcLuSubordinateDtcSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDtcLuSubordinateDtcSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcLuSubordinateDtcSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuSubordinateDtcSink {}
impl ::core::fmt::Debug for IDtcLuSubordinateDtcSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuSubordinateDtcSink").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDtcLuSubordinateDtcSink {
    type Vtable = IDtcLuSubordinateDtcSink_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4131e774_1aea_11d0_944b_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuSubordinateDtcSink_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AckUnplug: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TmDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SessionLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub BackedOut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub BackOut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Committed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Forget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RequestCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDtcNetworkAccessConfig(::windows_core::IUnknown);
impl IDtcNetworkAccessConfig {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnyNetworkAccess(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetAnyNetworkAccess)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAnyNetworkAccess<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, banynetworkaccess: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAnyNetworkAccess)(::windows_core::Interface::as_raw(self), banynetworkaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkAdministrationAccess(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetNetworkAdministrationAccess)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkAdministrationAccess<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworkadministrationaccess: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNetworkAdministrationAccess)(::windows_core::Interface::as_raw(self), bnetworkadministrationaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkTransactionAccess(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetNetworkTransactionAccess)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkTransactionAccess<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworktransactionaccess: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNetworkTransactionAccess)(::windows_core::Interface::as_raw(self), bnetworktransactionaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkClientAccess(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetNetworkClientAccess)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkClientAccess<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworkclientaccess: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNetworkClientAccess)(::windows_core::Interface::as_raw(self), bnetworkclientaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkTIPAccess(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetNetworkTIPAccess)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkTIPAccess<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworktipaccess: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNetworkTIPAccess)(::windows_core::Interface::as_raw(self), bnetworktipaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetXAAccess(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetXAAccess)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXAAccess<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bxaaccess: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetXAAccess)(::windows_core::Interface::as_raw(self), bxaaccess.into_param().abi()).ok()
    }
    pub unsafe fn RestartDtcService(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RestartDtcService)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IDtcNetworkAccessConfig> for ::windows_core::IUnknown {
    fn from(value: IDtcNetworkAccessConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcNetworkAccessConfig> for ::windows_core::IUnknown {
    fn from(value: &IDtcNetworkAccessConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDtcNetworkAccessConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDtcNetworkAccessConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDtcNetworkAccessConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcNetworkAccessConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcNetworkAccessConfig {}
impl ::core::fmt::Debug for IDtcNetworkAccessConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcNetworkAccessConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDtcNetworkAccessConfig {
    type Vtable = IDtcNetworkAccessConfig_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9797c15d_a428_4291_87b6_0995031a678d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcNetworkAccessConfig_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAnyNetworkAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbanynetworkaccess: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAnyNetworkAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAnyNetworkAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, banynetworkaccess: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAnyNetworkAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetNetworkAdministrationAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbnetworkadministrationaccess: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetNetworkAdministrationAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNetworkAdministrationAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bnetworkadministrationaccess: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNetworkAdministrationAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetNetworkTransactionAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbnetworktransactionaccess: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetNetworkTransactionAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNetworkTransactionAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bnetworktransactionaccess: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNetworkTransactionAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetNetworkClientAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbnetworkclientaccess: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetNetworkClientAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNetworkClientAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bnetworkclientaccess: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNetworkClientAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetNetworkTIPAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbnetworktipaccess: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetNetworkTIPAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNetworkTIPAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bnetworktipaccess: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNetworkTIPAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetXAAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbxaaccess: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetXAAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetXAAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bxaaccess: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetXAAccess: usize,
    pub RestartDtcService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDtcNetworkAccessConfig2(::windows_core::IUnknown);
impl IDtcNetworkAccessConfig2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnyNetworkAccess(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetAnyNetworkAccess)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAnyNetworkAccess<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, banynetworkaccess: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetAnyNetworkAccess)(::windows_core::Interface::as_raw(self), banynetworkaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkAdministrationAccess(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetNetworkAdministrationAccess)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkAdministrationAccess<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworkadministrationaccess: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetNetworkAdministrationAccess)(::windows_core::Interface::as_raw(self), bnetworkadministrationaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkTransactionAccess(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetNetworkTransactionAccess)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkTransactionAccess<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworktransactionaccess: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetNetworkTransactionAccess)(::windows_core::Interface::as_raw(self), bnetworktransactionaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkClientAccess(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetNetworkClientAccess)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkClientAccess<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworkclientaccess: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetNetworkClientAccess)(::windows_core::Interface::as_raw(self), bnetworkclientaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkTIPAccess(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetNetworkTIPAccess)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkTIPAccess<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworktipaccess: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetNetworkTIPAccess)(::windows_core::Interface::as_raw(self), bnetworktipaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetXAAccess(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetXAAccess)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXAAccess<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bxaaccess: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetXAAccess)(::windows_core::Interface::as_raw(self), bxaaccess.into_param().abi()).ok()
    }
    pub unsafe fn RestartDtcService(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RestartDtcService)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkInboundAccess(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetNetworkInboundAccess)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkOutboundAccess(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetNetworkOutboundAccess)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkInboundAccess<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, binbound: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNetworkInboundAccess)(::windows_core::Interface::as_raw(self), binbound.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkOutboundAccess<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, boutbound: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNetworkOutboundAccess)(::windows_core::Interface::as_raw(self), boutbound.into_param().abi()).ok()
    }
    pub unsafe fn GetAuthenticationLevel(&self) -> ::windows_core::Result<AUTHENTICATION_LEVEL> {
        let mut result__ = ::core::mem::MaybeUninit::<AUTHENTICATION_LEVEL>::zeroed();
        (::windows_core::Interface::vtable(self).GetAuthenticationLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<AUTHENTICATION_LEVEL>(result__)
    }
    pub unsafe fn SetAuthenticationLevel(&self, authlevel: AUTHENTICATION_LEVEL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAuthenticationLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(authlevel)).ok()
    }
}
impl ::core::convert::From<IDtcNetworkAccessConfig2> for ::windows_core::IUnknown {
    fn from(value: IDtcNetworkAccessConfig2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcNetworkAccessConfig2> for ::windows_core::IUnknown {
    fn from(value: &IDtcNetworkAccessConfig2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDtcNetworkAccessConfig2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDtcNetworkAccessConfig2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDtcNetworkAccessConfig2> for IDtcNetworkAccessConfig {
    fn from(value: IDtcNetworkAccessConfig2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcNetworkAccessConfig2> for IDtcNetworkAccessConfig {
    fn from(value: &IDtcNetworkAccessConfig2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDtcNetworkAccessConfig> for IDtcNetworkAccessConfig2 {
    fn into_param(self) -> ::windows_core::Param<'a, IDtcNetworkAccessConfig> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDtcNetworkAccessConfig> for &'a IDtcNetworkAccessConfig2 {
    fn into_param(self) -> ::windows_core::Param<'a, IDtcNetworkAccessConfig> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDtcNetworkAccessConfig2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcNetworkAccessConfig2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcNetworkAccessConfig2 {}
impl ::core::fmt::Debug for IDtcNetworkAccessConfig2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcNetworkAccessConfig2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDtcNetworkAccessConfig2 {
    type Vtable = IDtcNetworkAccessConfig2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa7aa013b_eb7d_4f42_b41c_b2dec09ae034);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcNetworkAccessConfig2_Vtbl {
    pub base__: IDtcNetworkAccessConfig_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetNetworkInboundAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbinbound: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetNetworkInboundAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetNetworkOutboundAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pboutbound: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetNetworkOutboundAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNetworkInboundAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, binbound: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNetworkInboundAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNetworkOutboundAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, boutbound: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNetworkOutboundAccess: usize,
    pub GetAuthenticationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pauthlevel: *mut AUTHENTICATION_LEVEL) -> ::windows_core::HRESULT,
    pub SetAuthenticationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, authlevel: AUTHENTICATION_LEVEL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDtcNetworkAccessConfig3(::windows_core::IUnknown);
impl IDtcNetworkAccessConfig3 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnyNetworkAccess(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetAnyNetworkAccess)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAnyNetworkAccess<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, banynetworkaccess: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetAnyNetworkAccess)(::windows_core::Interface::as_raw(self), banynetworkaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkAdministrationAccess(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetNetworkAdministrationAccess)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkAdministrationAccess<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworkadministrationaccess: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetNetworkAdministrationAccess)(::windows_core::Interface::as_raw(self), bnetworkadministrationaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkTransactionAccess(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetNetworkTransactionAccess)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkTransactionAccess<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworktransactionaccess: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetNetworkTransactionAccess)(::windows_core::Interface::as_raw(self), bnetworktransactionaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkClientAccess(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetNetworkClientAccess)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkClientAccess<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworkclientaccess: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetNetworkClientAccess)(::windows_core::Interface::as_raw(self), bnetworkclientaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkTIPAccess(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetNetworkTIPAccess)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkTIPAccess<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bnetworktipaccess: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetNetworkTIPAccess)(::windows_core::Interface::as_raw(self), bnetworktipaccess.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetXAAccess(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetXAAccess)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXAAccess<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bxaaccess: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetXAAccess)(::windows_core::Interface::as_raw(self), bxaaccess.into_param().abi()).ok()
    }
    pub unsafe fn RestartDtcService(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.RestartDtcService)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkInboundAccess(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetNetworkInboundAccess)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkOutboundAccess(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetNetworkOutboundAccess)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkInboundAccess<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, binbound: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetNetworkInboundAccess)(::windows_core::Interface::as_raw(self), binbound.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkOutboundAccess<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, boutbound: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetNetworkOutboundAccess)(::windows_core::Interface::as_raw(self), boutbound.into_param().abi()).ok()
    }
    pub unsafe fn GetAuthenticationLevel(&self) -> ::windows_core::Result<AUTHENTICATION_LEVEL> {
        let mut result__ = ::core::mem::MaybeUninit::<AUTHENTICATION_LEVEL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetAuthenticationLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<AUTHENTICATION_LEVEL>(result__)
    }
    pub unsafe fn SetAuthenticationLevel(&self, authlevel: AUTHENTICATION_LEVEL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetAuthenticationLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(authlevel)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLUAccess(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetLUAccess)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLUAccess<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bluaccess: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLUAccess)(::windows_core::Interface::as_raw(self), bluaccess.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDtcNetworkAccessConfig3> for ::windows_core::IUnknown {
    fn from(value: IDtcNetworkAccessConfig3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcNetworkAccessConfig3> for ::windows_core::IUnknown {
    fn from(value: &IDtcNetworkAccessConfig3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDtcNetworkAccessConfig3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDtcNetworkAccessConfig3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDtcNetworkAccessConfig3> for IDtcNetworkAccessConfig {
    fn from(value: IDtcNetworkAccessConfig3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcNetworkAccessConfig3> for IDtcNetworkAccessConfig {
    fn from(value: &IDtcNetworkAccessConfig3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDtcNetworkAccessConfig> for IDtcNetworkAccessConfig3 {
    fn into_param(self) -> ::windows_core::Param<'a, IDtcNetworkAccessConfig> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDtcNetworkAccessConfig> for &'a IDtcNetworkAccessConfig3 {
    fn into_param(self) -> ::windows_core::Param<'a, IDtcNetworkAccessConfig> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDtcNetworkAccessConfig3> for IDtcNetworkAccessConfig2 {
    fn from(value: IDtcNetworkAccessConfig3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcNetworkAccessConfig3> for IDtcNetworkAccessConfig2 {
    fn from(value: &IDtcNetworkAccessConfig3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDtcNetworkAccessConfig2> for IDtcNetworkAccessConfig3 {
    fn into_param(self) -> ::windows_core::Param<'a, IDtcNetworkAccessConfig2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDtcNetworkAccessConfig2> for &'a IDtcNetworkAccessConfig3 {
    fn into_param(self) -> ::windows_core::Param<'a, IDtcNetworkAccessConfig2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDtcNetworkAccessConfig3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcNetworkAccessConfig3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcNetworkAccessConfig3 {}
impl ::core::fmt::Debug for IDtcNetworkAccessConfig3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcNetworkAccessConfig3").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDtcNetworkAccessConfig3 {
    type Vtable = IDtcNetworkAccessConfig3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x76e4b4f3_2ca5_466b_89d5_fd218ee75b49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcNetworkAccessConfig3_Vtbl {
    pub base__: IDtcNetworkAccessConfig2_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLUAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbluaccess: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLUAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLUAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluaccess: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLUAccess: usize,
}
#[repr(transparent)]
pub struct IDtcToXaHelper(::windows_core::IUnknown);
impl IDtcToXaHelper {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Close<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, i_fdorecovery: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self), i_fdorecovery.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TranslateTridToXid<'a, Param0: ::windows_core::IntoParam<'a, ITransaction>>(&self, pitransaction: Param0, pguidbqual: *const ::windows_core::GUID) -> ::windows_core::Result<xid_t> {
        let mut result__ = ::core::mem::MaybeUninit::<xid_t>::zeroed();
        (::windows_core::Interface::vtable(self).TranslateTridToXid)(::windows_core::Interface::as_raw(self), pitransaction.into_param().abi(), ::core::mem::transmute(pguidbqual), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<xid_t>(result__)
    }
}
impl ::core::convert::From<IDtcToXaHelper> for ::windows_core::IUnknown {
    fn from(value: IDtcToXaHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcToXaHelper> for ::windows_core::IUnknown {
    fn from(value: &IDtcToXaHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDtcToXaHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDtcToXaHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDtcToXaHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcToXaHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcToXaHelper {}
impl ::core::fmt::Debug for IDtcToXaHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcToXaHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDtcToXaHelper {
    type Vtable = IDtcToXaHelper_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa9861611_304a_11d1_9813_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcToXaHelper_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i_fdorecovery: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Close: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TranslateTridToXid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitransaction: ::windows_core::RawPtr, pguidbqual: *const ::windows_core::GUID, pxid: *mut xid_t) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TranslateTridToXid: usize,
}
#[repr(transparent)]
pub struct IDtcToXaHelperFactory(::windows_core::IUnknown);
impl IDtcToXaHelperFactory {
    pub unsafe fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(&self, pszdsn: Param0, pszclientdllname: Param1, pguidrm: *mut ::windows_core::GUID, ppxahelper: *mut ::core::option::Option<IDtcToXaHelper>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), pszdsn.into_param().abi(), pszclientdllname.into_param().abi(), ::core::mem::transmute(pguidrm), ::core::mem::transmute(ppxahelper)).ok()
    }
}
impl ::core::convert::From<IDtcToXaHelperFactory> for ::windows_core::IUnknown {
    fn from(value: IDtcToXaHelperFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcToXaHelperFactory> for ::windows_core::IUnknown {
    fn from(value: &IDtcToXaHelperFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDtcToXaHelperFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDtcToXaHelperFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDtcToXaHelperFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcToXaHelperFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcToXaHelperFactory {}
impl ::core::fmt::Debug for IDtcToXaHelperFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcToXaHelperFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDtcToXaHelperFactory {
    type Vtable = IDtcToXaHelperFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa9861610_304a_11d1_9813_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcToXaHelperFactory_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdsn: ::windows_core::PCSTR, pszclientdllname: ::windows_core::PCSTR, pguidrm: *mut ::windows_core::GUID, ppxahelper: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDtcToXaHelperSinglePipe(::windows_core::IUnknown);
impl IDtcToXaHelperSinglePipe {
    pub unsafe fn XARMCreate<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(&self, pszdsn: Param0, pszclientdll: Param1, pdwrmcookie: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).XARMCreate)(::windows_core::Interface::as_raw(self), pszdsn.into_param().abi(), pszclientdll.into_param().abi(), ::core::mem::transmute(pdwrmcookie)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConvertTridToXID(&self, pdwitrans: *mut u32, dwrmcookie: u32, pxid: *mut xid_t) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ConvertTridToXID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwitrans), ::core::mem::transmute(dwrmcookie), ::core::mem::transmute(pxid)).ok()
    }
    pub unsafe fn EnlistWithRM<'a, Param1: ::windows_core::IntoParam<'a, ITransaction>, Param2: ::windows_core::IntoParam<'a, ITransactionResourceAsync>>(&self, dwrmcookie: u32, i_pitransaction: Param1, i_pitransres: Param2) -> ::windows_core::Result<ITransactionEnlistmentAsync> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnlistWithRM)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwrmcookie), i_pitransaction.into_param().abi(), i_pitransres.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransactionEnlistmentAsync>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseRMCookie<'a, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, i_dwrmcookie: u32, i_fnormal: Param1) {
        (::windows_core::Interface::vtable(self).ReleaseRMCookie)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(i_dwrmcookie), i_fnormal.into_param().abi())
    }
}
impl ::core::convert::From<IDtcToXaHelperSinglePipe> for ::windows_core::IUnknown {
    fn from(value: IDtcToXaHelperSinglePipe) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcToXaHelperSinglePipe> for ::windows_core::IUnknown {
    fn from(value: &IDtcToXaHelperSinglePipe) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDtcToXaHelperSinglePipe {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDtcToXaHelperSinglePipe {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDtcToXaHelperSinglePipe {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcToXaHelperSinglePipe {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcToXaHelperSinglePipe {}
impl ::core::fmt::Debug for IDtcToXaHelperSinglePipe {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcToXaHelperSinglePipe").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDtcToXaHelperSinglePipe {
    type Vtable = IDtcToXaHelperSinglePipe_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x47ed4971_53b3_11d1_bbb9_00c04fd658f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcToXaHelperSinglePipe_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub XARMCreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdsn: ::windows_core::PCSTR, pszclientdll: ::windows_core::PCSTR, pdwrmcookie: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ConvertTridToXID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwitrans: *mut u32, dwrmcookie: u32, pxid: *mut xid_t) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConvertTridToXID: usize,
    pub EnlistWithRM: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwrmcookie: u32, i_pitransaction: ::windows_core::RawPtr, i_pitransres: ::windows_core::RawPtr, o_ppitransenslitment: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ReleaseRMCookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i_dwrmcookie: u32, i_fnormal: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    ReleaseRMCookie: usize,
}
#[repr(transparent)]
pub struct IDtcToXaMapper(::windows_core::IUnknown);
impl IDtcToXaMapper {
    pub unsafe fn RequestNewResourceManager<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(&self, pszdsn: Param0, pszclientdllname: Param1, pdwrmcookie: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RequestNewResourceManager)(::windows_core::Interface::as_raw(self), pszdsn.into_param().abi(), pszclientdllname.into_param().abi(), ::core::mem::transmute(pdwrmcookie)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TranslateTridToXid(&self, pdwitransaction: *const u32, dwrmcookie: u32, pxid: *mut xid_t) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TranslateTridToXid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwitransaction), ::core::mem::transmute(dwrmcookie), ::core::mem::transmute(pxid)).ok()
    }
    pub unsafe fn EnlistResourceManager(&self, dwrmcookie: u32, pdwitransaction: *const u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnlistResourceManager)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwrmcookie), ::core::mem::transmute(pdwitransaction)).ok()
    }
    pub unsafe fn ReleaseResourceManager(&self, dwrmcookie: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReleaseResourceManager)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwrmcookie)).ok()
    }
}
impl ::core::convert::From<IDtcToXaMapper> for ::windows_core::IUnknown {
    fn from(value: IDtcToXaMapper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDtcToXaMapper> for ::windows_core::IUnknown {
    fn from(value: &IDtcToXaMapper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDtcToXaMapper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDtcToXaMapper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDtcToXaMapper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDtcToXaMapper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcToXaMapper {}
impl ::core::fmt::Debug for IDtcToXaMapper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcToXaMapper").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDtcToXaMapper {
    type Vtable = IDtcToXaMapper_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64ffabe0_7ce9_11d0_8ce6_00c04fdc877e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcToXaMapper_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub RequestNewResourceManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdsn: ::windows_core::PCSTR, pszclientdllname: ::windows_core::PCSTR, pdwrmcookie: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TranslateTridToXid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwitransaction: *const u32, dwrmcookie: u32, pxid: *mut xid_t) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TranslateTridToXid: usize,
    pub EnlistResourceManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwrmcookie: u32, pdwitransaction: *const u32) -> ::windows_core::HRESULT,
    pub ReleaseResourceManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwrmcookie: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IGetDispenser(::windows_core::IUnknown);
impl IGetDispenser {
    pub unsafe fn GetDispenser(&self, iid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDispenser)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iid), ::core::mem::transmute(ppvobject)).ok()
    }
}
impl ::core::convert::From<IGetDispenser> for ::windows_core::IUnknown {
    fn from(value: IGetDispenser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGetDispenser> for ::windows_core::IUnknown {
    fn from(value: &IGetDispenser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGetDispenser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGetDispenser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGetDispenser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGetDispenser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGetDispenser {}
impl ::core::fmt::Debug for IGetDispenser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetDispenser").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IGetDispenser {
    type Vtable = IGetDispenser_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc23cc370_87ef_11ce_8081_0080c758527e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetDispenser_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetDispenser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IKernelTransaction(::windows_core::IUnknown);
impl IKernelTransaction {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHandle(&self) -> ::windows_core::Result<super::super::Foundation::HANDLE> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::HANDLE>::zeroed();
        (::windows_core::Interface::vtable(self).GetHandle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
}
impl ::core::convert::From<IKernelTransaction> for ::windows_core::IUnknown {
    fn from(value: IKernelTransaction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IKernelTransaction> for ::windows_core::IUnknown {
    fn from(value: &IKernelTransaction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IKernelTransaction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IKernelTransaction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IKernelTransaction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IKernelTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IKernelTransaction {}
impl ::core::fmt::Debug for IKernelTransaction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKernelTransaction").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IKernelTransaction {
    type Vtable = IKernelTransaction_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79427a2b_f895_40e0_be79_b57dc82ed231);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKernelTransaction_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHandle: usize,
}
#[repr(transparent)]
pub struct ILastResourceManager(::windows_core::IUnknown);
impl ILastResourceManager {
    pub unsafe fn TransactionCommitted(&self, pprepinfo: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TransactionCommitted)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(pprepinfo)), pprepinfo.len() as _).ok()
    }
    pub unsafe fn RecoveryDone(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RecoveryDone)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ILastResourceManager> for ::windows_core::IUnknown {
    fn from(value: ILastResourceManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILastResourceManager> for ::windows_core::IUnknown {
    fn from(value: &ILastResourceManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ILastResourceManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ILastResourceManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILastResourceManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILastResourceManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILastResourceManager {}
impl ::core::fmt::Debug for ILastResourceManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILastResourceManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ILastResourceManager {
    type Vtable = ILastResourceManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d964ad4_5b33_11d3_8a91_00c04f79eb6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILastResourceManager_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub TransactionCommitted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprepinfo: *const u8, cbprepinfo: u32) -> ::windows_core::HRESULT,
    pub RecoveryDone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPrepareInfo(::windows_core::IUnknown);
impl IPrepareInfo {
    pub unsafe fn GetPrepareInfoSize(&self, pcbprepinfo: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPrepareInfoSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcbprepinfo)).ok()
    }
    pub unsafe fn GetPrepareInfo(&self, pprepinfo: *mut u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPrepareInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pprepinfo)).ok()
    }
}
impl ::core::convert::From<IPrepareInfo> for ::windows_core::IUnknown {
    fn from(value: IPrepareInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrepareInfo> for ::windows_core::IUnknown {
    fn from(value: &IPrepareInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPrepareInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPrepareInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPrepareInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrepareInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrepareInfo {}
impl ::core::fmt::Debug for IPrepareInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrepareInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPrepareInfo {
    type Vtable = IPrepareInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x80c7bfd0_87ee_11ce_8081_0080c758527e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrepareInfo_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetPrepareInfoSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbprepinfo: *mut u32) -> ::windows_core::HRESULT,
    pub GetPrepareInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprepinfo: *mut u8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPrepareInfo2(::windows_core::IUnknown);
impl IPrepareInfo2 {
    pub unsafe fn GetPrepareInfoSize(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetPrepareInfoSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetPrepareInfo(&self, pprepinfo: &mut [u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPrepareInfo)(::windows_core::Interface::as_raw(self), pprepinfo.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pprepinfo))).ok()
    }
}
impl ::core::convert::From<IPrepareInfo2> for ::windows_core::IUnknown {
    fn from(value: IPrepareInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrepareInfo2> for ::windows_core::IUnknown {
    fn from(value: &IPrepareInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPrepareInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPrepareInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPrepareInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrepareInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrepareInfo2 {}
impl ::core::fmt::Debug for IPrepareInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrepareInfo2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPrepareInfo2 {
    type Vtable = IPrepareInfo2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5fab2547_9779_11d1_b886_00c04fb9618a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrepareInfo2_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetPrepareInfoSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbprepinfo: *mut u32) -> ::windows_core::HRESULT,
    pub GetPrepareInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbprepareinfo: u32, pprepinfo: *mut u8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRMHelper(::windows_core::IUnknown);
impl IRMHelper {
    pub unsafe fn RMCount(&self, dwctotalnumberofrms: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RMCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwctotalnumberofrms)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RMInfo<'a, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param4: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, pxa_switch: *mut xa_switch_t, fcdeclcallingconv: Param1, pszopenstring: Param2, pszclosestring: Param3, guidrmrecovery: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RMInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pxa_switch), fcdeclcallingconv.into_param().abi(), pszopenstring.into_param().abi(), pszclosestring.into_param().abi(), guidrmrecovery.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IRMHelper> for ::windows_core::IUnknown {
    fn from(value: IRMHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRMHelper> for ::windows_core::IUnknown {
    fn from(value: &IRMHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRMHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRMHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRMHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRMHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRMHelper {}
impl ::core::fmt::Debug for IRMHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRMHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRMHelper {
    type Vtable = IRMHelper_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe793f6d1_f53d_11cf_a60d_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRMHelper_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub RMCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwctotalnumberofrms: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RMInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pxa_switch: *mut xa_switch_t, fcdeclcallingconv: super::super::Foundation::BOOL, pszopenstring: ::windows_core::PCSTR, pszclosestring: ::windows_core::PCSTR, guidrmrecovery: ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RMInfo: usize,
}
#[repr(transparent)]
pub struct IResourceManager(::windows_core::IUnknown);
impl IResourceManager {
    pub unsafe fn Enlist<'a, Param0: ::windows_core::IntoParam<'a, ITransaction>, Param1: ::windows_core::IntoParam<'a, ITransactionResourceAsync>>(&self, ptransaction: Param0, pres: Param1, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut ::core::option::Option<ITransactionEnlistmentAsync>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Enlist)(::windows_core::Interface::as_raw(self), ptransaction.into_param().abi(), pres.into_param().abi(), ::core::mem::transmute(puow), ::core::mem::transmute(pisolevel), ::core::mem::transmute(ppenlist)).ok()
    }
    pub unsafe fn Reenlist(&self, pprepinfo: &[u8], ltimeout: u32) -> ::windows_core::Result<XACTSTAT> {
        let mut result__ = ::core::mem::MaybeUninit::<XACTSTAT>::zeroed();
        (::windows_core::Interface::vtable(self).Reenlist)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(pprepinfo)), pprepinfo.len() as _, ::core::mem::transmute(ltimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<XACTSTAT>(result__)
    }
    pub unsafe fn ReenlistmentComplete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReenlistmentComplete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetDistributedTransactionManager(&self, iid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDistributedTransactionManager)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iid), ::core::mem::transmute(ppvobject)).ok()
    }
}
impl ::core::convert::From<IResourceManager> for ::windows_core::IUnknown {
    fn from(value: IResourceManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResourceManager> for ::windows_core::IUnknown {
    fn from(value: &IResourceManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IResourceManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IResourceManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IResourceManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IResourceManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResourceManager {}
impl ::core::fmt::Debug for IResourceManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResourceManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IResourceManager {
    type Vtable = IResourceManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13741d21_87eb_11ce_8081_0080c758527e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManager_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Enlist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptransaction: ::windows_core::RawPtr, pres: ::windows_core::RawPtr, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Reenlist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows_core::HRESULT,
    pub ReenlistmentComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDistributedTransactionManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IResourceManager2(::windows_core::IUnknown);
impl IResourceManager2 {
    pub unsafe fn Enlist<'a, Param0: ::windows_core::IntoParam<'a, ITransaction>, Param1: ::windows_core::IntoParam<'a, ITransactionResourceAsync>>(&self, ptransaction: Param0, pres: Param1, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut ::core::option::Option<ITransactionEnlistmentAsync>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Enlist)(::windows_core::Interface::as_raw(self), ptransaction.into_param().abi(), pres.into_param().abi(), ::core::mem::transmute(puow), ::core::mem::transmute(pisolevel), ::core::mem::transmute(ppenlist)).ok()
    }
    pub unsafe fn Reenlist(&self, pprepinfo: &[u8], ltimeout: u32) -> ::windows_core::Result<XACTSTAT> {
        let mut result__ = ::core::mem::MaybeUninit::<XACTSTAT>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Reenlist)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(pprepinfo)), pprepinfo.len() as _, ::core::mem::transmute(ltimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<XACTSTAT>(result__)
    }
    pub unsafe fn ReenlistmentComplete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ReenlistmentComplete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetDistributedTransactionManager(&self, iid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDistributedTransactionManager)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iid), ::core::mem::transmute(ppvobject)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enlist2<'a, Param0: ::windows_core::IntoParam<'a, ITransaction>, Param1: ::windows_core::IntoParam<'a, ITransactionResourceAsync>>(&self, ptransaction: Param0, presasync: Param1, puow: *mut BOID, pisolevel: *mut i32, pxid: *mut xid_t, ppenlist: *mut ::core::option::Option<ITransactionEnlistmentAsync>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Enlist2)(::windows_core::Interface::as_raw(self), ptransaction.into_param().abi(), presasync.into_param().abi(), ::core::mem::transmute(puow), ::core::mem::transmute(pisolevel), ::core::mem::transmute(pxid), ::core::mem::transmute(ppenlist)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Reenlist2(&self, pxid: *const xid_t, dwtimeout: u32) -> ::windows_core::Result<XACTSTAT> {
        let mut result__ = ::core::mem::MaybeUninit::<XACTSTAT>::zeroed();
        (::windows_core::Interface::vtable(self).Reenlist2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pxid), ::core::mem::transmute(dwtimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<XACTSTAT>(result__)
    }
}
impl ::core::convert::From<IResourceManager2> for ::windows_core::IUnknown {
    fn from(value: IResourceManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResourceManager2> for ::windows_core::IUnknown {
    fn from(value: &IResourceManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IResourceManager2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IResourceManager2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IResourceManager2> for IResourceManager {
    fn from(value: IResourceManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResourceManager2> for IResourceManager {
    fn from(value: &IResourceManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IResourceManager> for IResourceManager2 {
    fn into_param(self) -> ::windows_core::Param<'a, IResourceManager> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IResourceManager> for &'a IResourceManager2 {
    fn into_param(self) -> ::windows_core::Param<'a, IResourceManager> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IResourceManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IResourceManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResourceManager2 {}
impl ::core::fmt::Debug for IResourceManager2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResourceManager2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IResourceManager2 {
    type Vtable = IResourceManager2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd136c69a_f749_11d1_8f47_00c04f8ee57d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManager2_Vtbl {
    pub base__: IResourceManager_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Enlist2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptransaction: ::windows_core::RawPtr, presasync: ::windows_core::RawPtr, puow: *mut BOID, pisolevel: *mut i32, pxid: *mut xid_t, ppenlist: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enlist2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Reenlist2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pxid: *const xid_t, dwtimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Reenlist2: usize,
}
#[repr(transparent)]
pub struct IResourceManagerFactory(::windows_core::IUnknown);
impl IResourceManagerFactory {
    pub unsafe fn Create<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param2: ::windows_core::IntoParam<'a, IResourceManagerSink>>(&self, pguidrm: *const ::windows_core::GUID, pszrmname: Param1, piresmgrsink: Param2) -> ::windows_core::Result<IResourceManager> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguidrm), pszrmname.into_param().abi(), piresmgrsink.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IResourceManager>(result__)
    }
}
impl ::core::convert::From<IResourceManagerFactory> for ::windows_core::IUnknown {
    fn from(value: IResourceManagerFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResourceManagerFactory> for ::windows_core::IUnknown {
    fn from(value: &IResourceManagerFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IResourceManagerFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IResourceManagerFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IResourceManagerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IResourceManagerFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResourceManagerFactory {}
impl ::core::fmt::Debug for IResourceManagerFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResourceManagerFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IResourceManagerFactory {
    type Vtable = IResourceManagerFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13741d20_87eb_11ce_8081_0080c758527e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerFactory_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidrm: *const ::windows_core::GUID, pszrmname: ::windows_core::PCSTR, piresmgrsink: ::windows_core::RawPtr, ppresmgr: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IResourceManagerFactory2(::windows_core::IUnknown);
impl IResourceManagerFactory2 {
    pub unsafe fn Create<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param2: ::windows_core::IntoParam<'a, IResourceManagerSink>>(&self, pguidrm: *const ::windows_core::GUID, pszrmname: Param1, piresmgrsink: Param2) -> ::windows_core::Result<IResourceManager> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Create)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguidrm), pszrmname.into_param().abi(), piresmgrsink.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IResourceManager>(result__)
    }
    pub unsafe fn CreateEx<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param2: ::windows_core::IntoParam<'a, IResourceManagerSink>>(&self, pguidrm: *const ::windows_core::GUID, pszrmname: Param1, piresmgrsink: Param2, riidrequested: *const ::windows_core::GUID, ppvresmgr: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguidrm), pszrmname.into_param().abi(), piresmgrsink.into_param().abi(), ::core::mem::transmute(riidrequested), ::core::mem::transmute(ppvresmgr)).ok()
    }
}
impl ::core::convert::From<IResourceManagerFactory2> for ::windows_core::IUnknown {
    fn from(value: IResourceManagerFactory2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResourceManagerFactory2> for ::windows_core::IUnknown {
    fn from(value: &IResourceManagerFactory2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IResourceManagerFactory2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IResourceManagerFactory2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IResourceManagerFactory2> for IResourceManagerFactory {
    fn from(value: IResourceManagerFactory2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResourceManagerFactory2> for IResourceManagerFactory {
    fn from(value: &IResourceManagerFactory2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IResourceManagerFactory> for IResourceManagerFactory2 {
    fn into_param(self) -> ::windows_core::Param<'a, IResourceManagerFactory> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IResourceManagerFactory> for &'a IResourceManagerFactory2 {
    fn into_param(self) -> ::windows_core::Param<'a, IResourceManagerFactory> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IResourceManagerFactory2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IResourceManagerFactory2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResourceManagerFactory2 {}
impl ::core::fmt::Debug for IResourceManagerFactory2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResourceManagerFactory2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IResourceManagerFactory2 {
    type Vtable = IResourceManagerFactory2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6b369c21_fbd2_11d1_8f47_00c04f8ee57d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerFactory2_Vtbl {
    pub base__: IResourceManagerFactory_Vtbl,
    pub CreateEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidrm: *const ::windows_core::GUID, pszrmname: ::windows_core::PCSTR, piresmgrsink: ::windows_core::RawPtr, riidrequested: *const ::windows_core::GUID, ppvresmgr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IResourceManagerRejoinable(::windows_core::IUnknown);
impl IResourceManagerRejoinable {
    pub unsafe fn Enlist<'a, Param0: ::windows_core::IntoParam<'a, ITransaction>, Param1: ::windows_core::IntoParam<'a, ITransactionResourceAsync>>(&self, ptransaction: Param0, pres: Param1, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut ::core::option::Option<ITransactionEnlistmentAsync>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Enlist)(::windows_core::Interface::as_raw(self), ptransaction.into_param().abi(), pres.into_param().abi(), ::core::mem::transmute(puow), ::core::mem::transmute(pisolevel), ::core::mem::transmute(ppenlist)).ok()
    }
    pub unsafe fn Reenlist(&self, pprepinfo: &[u8], ltimeout: u32) -> ::windows_core::Result<XACTSTAT> {
        let mut result__ = ::core::mem::MaybeUninit::<XACTSTAT>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Reenlist)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(pprepinfo)), pprepinfo.len() as _, ::core::mem::transmute(ltimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<XACTSTAT>(result__)
    }
    pub unsafe fn ReenlistmentComplete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.ReenlistmentComplete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetDistributedTransactionManager(&self, iid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetDistributedTransactionManager)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iid), ::core::mem::transmute(ppvobject)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enlist2<'a, Param0: ::windows_core::IntoParam<'a, ITransaction>, Param1: ::windows_core::IntoParam<'a, ITransactionResourceAsync>>(&self, ptransaction: Param0, presasync: Param1, puow: *mut BOID, pisolevel: *mut i32, pxid: *mut xid_t, ppenlist: *mut ::core::option::Option<ITransactionEnlistmentAsync>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Enlist2)(::windows_core::Interface::as_raw(self), ptransaction.into_param().abi(), presasync.into_param().abi(), ::core::mem::transmute(puow), ::core::mem::transmute(pisolevel), ::core::mem::transmute(pxid), ::core::mem::transmute(ppenlist)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Reenlist2(&self, pxid: *const xid_t, dwtimeout: u32) -> ::windows_core::Result<XACTSTAT> {
        let mut result__ = ::core::mem::MaybeUninit::<XACTSTAT>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Reenlist2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pxid), ::core::mem::transmute(dwtimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<XACTSTAT>(result__)
    }
    pub unsafe fn Rejoin(&self, pprepinfo: &[u8], ltimeout: u32) -> ::windows_core::Result<XACTSTAT> {
        let mut result__ = ::core::mem::MaybeUninit::<XACTSTAT>::zeroed();
        (::windows_core::Interface::vtable(self).Rejoin)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(pprepinfo)), pprepinfo.len() as _, ::core::mem::transmute(ltimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<XACTSTAT>(result__)
    }
}
impl ::core::convert::From<IResourceManagerRejoinable> for ::windows_core::IUnknown {
    fn from(value: IResourceManagerRejoinable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResourceManagerRejoinable> for ::windows_core::IUnknown {
    fn from(value: &IResourceManagerRejoinable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IResourceManagerRejoinable {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IResourceManagerRejoinable {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IResourceManagerRejoinable> for IResourceManager {
    fn from(value: IResourceManagerRejoinable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResourceManagerRejoinable> for IResourceManager {
    fn from(value: &IResourceManagerRejoinable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IResourceManager> for IResourceManagerRejoinable {
    fn into_param(self) -> ::windows_core::Param<'a, IResourceManager> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IResourceManager> for &'a IResourceManagerRejoinable {
    fn into_param(self) -> ::windows_core::Param<'a, IResourceManager> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IResourceManagerRejoinable> for IResourceManager2 {
    fn from(value: IResourceManagerRejoinable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResourceManagerRejoinable> for IResourceManager2 {
    fn from(value: &IResourceManagerRejoinable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IResourceManager2> for IResourceManagerRejoinable {
    fn into_param(self) -> ::windows_core::Param<'a, IResourceManager2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IResourceManager2> for &'a IResourceManagerRejoinable {
    fn into_param(self) -> ::windows_core::Param<'a, IResourceManager2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IResourceManagerRejoinable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IResourceManagerRejoinable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResourceManagerRejoinable {}
impl ::core::fmt::Debug for IResourceManagerRejoinable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResourceManagerRejoinable").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IResourceManagerRejoinable {
    type Vtable = IResourceManagerRejoinable_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f6de620_b5df_4f3e_9cfa_c8aebd05172b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerRejoinable_Vtbl {
    pub base__: IResourceManager2_Vtbl,
    pub Rejoin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IResourceManagerSink(::windows_core::IUnknown);
impl IResourceManagerSink {
    pub unsafe fn TMDown(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TMDown)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IResourceManagerSink> for ::windows_core::IUnknown {
    fn from(value: IResourceManagerSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IResourceManagerSink> for ::windows_core::IUnknown {
    fn from(value: &IResourceManagerSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IResourceManagerSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IResourceManagerSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IResourceManagerSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IResourceManagerSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResourceManagerSink {}
impl ::core::fmt::Debug for IResourceManagerSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResourceManagerSink").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IResourceManagerSink {
    type Vtable = IResourceManagerSink_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0d563181_defb_11ce_aed1_00aa0051e2c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerSink_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub TMDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ISOFLAG(pub i32);
pub const ISOFLAG_RETAIN_COMMIT_DC: ISOFLAG = ISOFLAG(1i32);
pub const ISOFLAG_RETAIN_COMMIT: ISOFLAG = ISOFLAG(2i32);
pub const ISOFLAG_RETAIN_COMMIT_NO: ISOFLAG = ISOFLAG(3i32);
pub const ISOFLAG_RETAIN_ABORT_DC: ISOFLAG = ISOFLAG(4i32);
pub const ISOFLAG_RETAIN_ABORT: ISOFLAG = ISOFLAG(8i32);
pub const ISOFLAG_RETAIN_ABORT_NO: ISOFLAG = ISOFLAG(12i32);
pub const ISOFLAG_RETAIN_DONTCARE: ISOFLAG = ISOFLAG(5i32);
pub const ISOFLAG_RETAIN_BOTH: ISOFLAG = ISOFLAG(10i32);
pub const ISOFLAG_RETAIN_NONE: ISOFLAG = ISOFLAG(15i32);
pub const ISOFLAG_OPTIMISTIC: ISOFLAG = ISOFLAG(16i32);
pub const ISOFLAG_READONLY: ISOFLAG = ISOFLAG(32i32);
impl ::core::marker::Copy for ISOFLAG {}
impl ::core::clone::Clone for ISOFLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ISOFLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ISOFLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for ISOFLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISOFLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ISOLATIONLEVEL(pub i32);
pub const ISOLATIONLEVEL_UNSPECIFIED: ISOLATIONLEVEL = ISOLATIONLEVEL(-1i32);
pub const ISOLATIONLEVEL_CHAOS: ISOLATIONLEVEL = ISOLATIONLEVEL(16i32);
pub const ISOLATIONLEVEL_READUNCOMMITTED: ISOLATIONLEVEL = ISOLATIONLEVEL(256i32);
pub const ISOLATIONLEVEL_BROWSE: ISOLATIONLEVEL = ISOLATIONLEVEL(256i32);
pub const ISOLATIONLEVEL_CURSORSTABILITY: ISOLATIONLEVEL = ISOLATIONLEVEL(4096i32);
pub const ISOLATIONLEVEL_READCOMMITTED: ISOLATIONLEVEL = ISOLATIONLEVEL(4096i32);
pub const ISOLATIONLEVEL_REPEATABLEREAD: ISOLATIONLEVEL = ISOLATIONLEVEL(65536i32);
pub const ISOLATIONLEVEL_SERIALIZABLE: ISOLATIONLEVEL = ISOLATIONLEVEL(1048576i32);
pub const ISOLATIONLEVEL_ISOLATED: ISOLATIONLEVEL = ISOLATIONLEVEL(1048576i32);
impl ::core::marker::Copy for ISOLATIONLEVEL {}
impl ::core::clone::Clone for ISOLATIONLEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ISOLATIONLEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ISOLATIONLEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for ISOLATIONLEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISOLATIONLEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
pub struct ITipHelper(::windows_core::IUnknown);
impl ITipHelper {
    pub unsafe fn Pull(&self, i_psztxurl: *const u8) -> ::windows_core::Result<ITransaction> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Pull)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(i_psztxurl), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransaction>(result__)
    }
    pub unsafe fn PullAsync<'a, Param1: ::windows_core::IntoParam<'a, ITipPullSink>>(&self, i_psztxurl: *const u8, i_ptippullsink: Param1) -> ::windows_core::Result<ITransaction> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).PullAsync)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(i_psztxurl), i_ptippullsink.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransaction>(result__)
    }
    pub unsafe fn GetLocalTmUrl(&self) -> ::windows_core::Result<*mut u8> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut u8>::zeroed();
        (::windows_core::Interface::vtable(self).GetLocalTmUrl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut u8>(result__)
    }
}
impl ::core::convert::From<ITipHelper> for ::windows_core::IUnknown {
    fn from(value: ITipHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITipHelper> for ::windows_core::IUnknown {
    fn from(value: &ITipHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITipHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITipHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITipHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITipHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITipHelper {}
impl ::core::fmt::Debug for ITipHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITipHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITipHelper {
    type Vtable = ITipHelper_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x17cf72d1_bac5_11d1_b1bf_00c04fc2f3ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITipHelper_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Pull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i_psztxurl: *const u8, o_ppitransaction: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PullAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i_psztxurl: *const u8, i_ptippullsink: ::windows_core::RawPtr, o_ppitransaction: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetLocalTmUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, o_ppszlocaltmurl: *mut *mut u8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITipPullSink(::windows_core::IUnknown);
impl ITipPullSink {
    pub unsafe fn PullComplete(&self, i_hrpull: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PullComplete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(i_hrpull)).ok()
    }
}
impl ::core::convert::From<ITipPullSink> for ::windows_core::IUnknown {
    fn from(value: ITipPullSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITipPullSink> for ::windows_core::IUnknown {
    fn from(value: &ITipPullSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITipPullSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITipPullSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITipPullSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITipPullSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITipPullSink {}
impl ::core::fmt::Debug for ITipPullSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITipPullSink").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITipPullSink {
    type Vtable = ITipPullSink_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x17cf72d2_bac5_11d1_b1bf_00c04fc2f3ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITipPullSink_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub PullComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i_hrpull: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITipTransaction(::windows_core::IUnknown);
impl ITipTransaction {
    pub unsafe fn Push(&self, i_pszremotetmurl: *const u8) -> ::windows_core::Result<::windows_core::PSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PSTR>::zeroed();
        (::windows_core::Interface::vtable(self).Push)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(i_pszremotetmurl), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PSTR>(result__)
    }
    pub unsafe fn GetTransactionUrl(&self) -> ::windows_core::Result<::windows_core::PSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetTransactionUrl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PSTR>(result__)
    }
}
impl ::core::convert::From<ITipTransaction> for ::windows_core::IUnknown {
    fn from(value: ITipTransaction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITipTransaction> for ::windows_core::IUnknown {
    fn from(value: &ITipTransaction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITipTransaction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITipTransaction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITipTransaction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITipTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITipTransaction {}
impl ::core::fmt::Debug for ITipTransaction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITipTransaction").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITipTransaction {
    type Vtable = ITipTransaction_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x17cf72d0_bac5_11d1_b1bf_00c04fc2f3ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITipTransaction_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Push: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i_pszremotetmurl: *const u8, o_ppszremotetxurl: *mut ::windows_core::PSTR) -> ::windows_core::HRESULT,
    pub GetTransactionUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, o_ppszlocaltxurl: *mut ::windows_core::PSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITmNodeName(::windows_core::IUnknown);
impl ITmNodeName {
    pub unsafe fn GetNodeNameSize(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetNodeNameSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetNodeName(&self, cbnodenamebuffersize: u32, pnodenamebuffer: ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetNodeName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cbnodenamebuffersize), ::core::mem::transmute(pnodenamebuffer)).ok()
    }
}
impl ::core::convert::From<ITmNodeName> for ::windows_core::IUnknown {
    fn from(value: ITmNodeName) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITmNodeName> for ::windows_core::IUnknown {
    fn from(value: &ITmNodeName) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITmNodeName {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITmNodeName {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITmNodeName {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITmNodeName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITmNodeName {}
impl ::core::fmt::Debug for ITmNodeName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITmNodeName").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITmNodeName {
    type Vtable = ITmNodeName_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30274f88_6ee4_474e_9b95_7807bc9ef8cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITmNodeName_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetNodeNameSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbnodenamesize: *mut u32) -> ::windows_core::HRESULT,
    pub GetNodeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbnodenamebuffersize: u32, pnodenamebuffer: ::windows_core::PWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITransaction(::windows_core::IUnknown);
impl ITransaction {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Commit<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fretaining: Param0, grftc: u32, grfrm: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Commit)(::windows_core::Interface::as_raw(self), fretaining.into_param().abi(), ::core::mem::transmute(grftc), ::core::mem::transmute(grfrm)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Abort<'a, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pboidreason: *const BOID, fretaining: Param1, fasync: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Abort)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pboidreason), fretaining.into_param().abi(), fasync.into_param().abi()).ok()
    }
    pub unsafe fn GetTransactionInfo(&self) -> ::windows_core::Result<XACTTRANSINFO> {
        let mut result__ = ::core::mem::MaybeUninit::<XACTTRANSINFO>::zeroed();
        (::windows_core::Interface::vtable(self).GetTransactionInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<XACTTRANSINFO>(result__)
    }
}
impl ::core::convert::From<ITransaction> for ::windows_core::IUnknown {
    fn from(value: ITransaction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransaction> for ::windows_core::IUnknown {
    fn from(value: &ITransaction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransaction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransaction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransaction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransaction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransaction {}
impl ::core::fmt::Debug for ITransaction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransaction").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransaction {
    type Vtable = ITransaction_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0fb15084_af41_11ce_bd2b_204c4f4f5020);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransaction_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fretaining: super::super::Foundation::BOOL, grftc: u32, grfrm: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Commit: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, fasync: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Abort: usize,
    pub GetTransactionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *mut XACTTRANSINFO) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITransaction2(::windows_core::IUnknown);
impl ITransaction2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Commit<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fretaining: Param0, grftc: u32, grfrm: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Commit)(::windows_core::Interface::as_raw(self), fretaining.into_param().abi(), ::core::mem::transmute(grftc), ::core::mem::transmute(grfrm)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Abort<'a, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pboidreason: *const BOID, fretaining: Param1, fasync: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Abort)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pboidreason), fretaining.into_param().abi(), fasync.into_param().abi()).ok()
    }
    pub unsafe fn GetTransactionInfo(&self) -> ::windows_core::Result<XACTTRANSINFO> {
        let mut result__ = ::core::mem::MaybeUninit::<XACTTRANSINFO>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetTransactionInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<XACTTRANSINFO>(result__)
    }
    pub unsafe fn CloneWithCommitDisabled(&self) -> ::windows_core::Result<ITransaction> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CloneWithCommitDisabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransaction>(result__)
    }
    pub unsafe fn GetTransactionInfo2(&self) -> ::windows_core::Result<XACTTRANSINFO> {
        let mut result__ = ::core::mem::MaybeUninit::<XACTTRANSINFO>::zeroed();
        (::windows_core::Interface::vtable(self).GetTransactionInfo2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<XACTTRANSINFO>(result__)
    }
}
impl ::core::convert::From<ITransaction2> for ::windows_core::IUnknown {
    fn from(value: ITransaction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransaction2> for ::windows_core::IUnknown {
    fn from(value: &ITransaction2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransaction2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransaction2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITransaction2> for ITransaction {
    fn from(value: ITransaction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransaction2> for ITransaction {
    fn from(value: &ITransaction2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ITransaction> for ITransaction2 {
    fn into_param(self) -> ::windows_core::Param<'a, ITransaction> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ITransaction> for &'a ITransaction2 {
    fn into_param(self) -> ::windows_core::Param<'a, ITransaction> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITransaction2> for ITransactionCloner {
    fn from(value: ITransaction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransaction2> for ITransactionCloner {
    fn from(value: &ITransaction2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ITransactionCloner> for ITransaction2 {
    fn into_param(self) -> ::windows_core::Param<'a, ITransactionCloner> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ITransactionCloner> for &'a ITransaction2 {
    fn into_param(self) -> ::windows_core::Param<'a, ITransactionCloner> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransaction2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransaction2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransaction2 {}
impl ::core::fmt::Debug for ITransaction2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransaction2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransaction2 {
    type Vtable = ITransaction2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x34021548_0065_11d3_bac1_00c04f797be2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransaction2_Vtbl {
    pub base__: ITransactionCloner_Vtbl,
    pub GetTransactionInfo2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *mut XACTTRANSINFO) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITransactionCloner(::windows_core::IUnknown);
impl ITransactionCloner {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Commit<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fretaining: Param0, grftc: u32, grfrm: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Commit)(::windows_core::Interface::as_raw(self), fretaining.into_param().abi(), ::core::mem::transmute(grftc), ::core::mem::transmute(grfrm)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Abort<'a, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pboidreason: *const BOID, fretaining: Param1, fasync: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Abort)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pboidreason), fretaining.into_param().abi(), fasync.into_param().abi()).ok()
    }
    pub unsafe fn GetTransactionInfo(&self) -> ::windows_core::Result<XACTTRANSINFO> {
        let mut result__ = ::core::mem::MaybeUninit::<XACTTRANSINFO>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransactionInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<XACTTRANSINFO>(result__)
    }
    pub unsafe fn CloneWithCommitDisabled(&self) -> ::windows_core::Result<ITransaction> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CloneWithCommitDisabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransaction>(result__)
    }
}
impl ::core::convert::From<ITransactionCloner> for ::windows_core::IUnknown {
    fn from(value: ITransactionCloner) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionCloner> for ::windows_core::IUnknown {
    fn from(value: &ITransactionCloner) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransactionCloner {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransactionCloner {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITransactionCloner> for ITransaction {
    fn from(value: ITransactionCloner) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionCloner> for ITransaction {
    fn from(value: &ITransactionCloner) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ITransaction> for ITransactionCloner {
    fn into_param(self) -> ::windows_core::Param<'a, ITransaction> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ITransaction> for &'a ITransactionCloner {
    fn into_param(self) -> ::windows_core::Param<'a, ITransaction> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransactionCloner {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionCloner {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionCloner {}
impl ::core::fmt::Debug for ITransactionCloner {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionCloner").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransactionCloner {
    type Vtable = ITransactionCloner_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x02656950_2152_11d0_944c_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionCloner_Vtbl {
    pub base__: ITransaction_Vtbl,
    pub CloneWithCommitDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppitransaction: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITransactionDispenser(::windows_core::IUnknown);
impl ITransactionDispenser {
    pub unsafe fn GetOptionsObject(&self) -> ::windows_core::Result<ITransactionOptions> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetOptionsObject)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransactionOptions>(result__)
    }
    pub unsafe fn BeginTransaction<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param3: ::windows_core::IntoParam<'a, ITransactionOptions>>(&self, punkouter: Param0, isolevel: i32, isoflags: u32, poptions: Param3) -> ::windows_core::Result<ITransaction> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).BeginTransaction)(::windows_core::Interface::as_raw(self), punkouter.into_param().abi(), ::core::mem::transmute(isolevel), ::core::mem::transmute(isoflags), poptions.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransaction>(result__)
    }
}
impl ::core::convert::From<ITransactionDispenser> for ::windows_core::IUnknown {
    fn from(value: ITransactionDispenser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionDispenser> for ::windows_core::IUnknown {
    fn from(value: &ITransactionDispenser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransactionDispenser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransactionDispenser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransactionDispenser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionDispenser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionDispenser {}
impl ::core::fmt::Debug for ITransactionDispenser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionDispenser").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransactionDispenser {
    type Vtable = ITransactionDispenser_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3a6ad9e1_23b9_11cf_ad60_00aa00a74ccd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionDispenser_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetOptionsObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppoptions: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub BeginTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, isolevel: i32, isoflags: u32, poptions: ::windows_core::RawPtr, pptransaction: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITransactionEnlistmentAsync(::windows_core::IUnknown);
impl ITransactionEnlistmentAsync {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrepareRequestDone<'a, Param1: ::windows_core::IntoParam<'a, super::Com::IMoniker>>(&self, hr: ::windows_core::HRESULT, pmk: Param1, pboidreason: *const BOID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PrepareRequestDone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hr), pmk.into_param().abi(), ::core::mem::transmute(pboidreason)).ok()
    }
    pub unsafe fn CommitRequestDone(&self, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CommitRequestDone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hr)).ok()
    }
    pub unsafe fn AbortRequestDone(&self, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AbortRequestDone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hr)).ok()
    }
}
impl ::core::convert::From<ITransactionEnlistmentAsync> for ::windows_core::IUnknown {
    fn from(value: ITransactionEnlistmentAsync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionEnlistmentAsync> for ::windows_core::IUnknown {
    fn from(value: &ITransactionEnlistmentAsync) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransactionEnlistmentAsync {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransactionEnlistmentAsync {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransactionEnlistmentAsync {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionEnlistmentAsync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionEnlistmentAsync {}
impl ::core::fmt::Debug for ITransactionEnlistmentAsync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionEnlistmentAsync").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransactionEnlistmentAsync {
    type Vtable = ITransactionEnlistmentAsync_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0fb15081_af41_11ce_bd2b_204c4f4f5020);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionEnlistmentAsync_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub PrepareRequestDone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT, pmk: ::windows_core::RawPtr, pboidreason: *const BOID) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrepareRequestDone: usize,
    pub CommitRequestDone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub AbortRequestDone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITransactionExport(::windows_core::IUnknown);
impl ITransactionExport {
    pub unsafe fn Export<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, punktransaction: Param0) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).Export)(::windows_core::Interface::as_raw(self), punktransaction.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetTransactionCookie<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, punktransaction: Param0, rgbtransactioncookie: &mut [u8], pcbused: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetTransactionCookie)(::windows_core::Interface::as_raw(self), punktransaction.into_param().abi(), rgbtransactioncookie.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(rgbtransactioncookie)), ::core::mem::transmute(pcbused)).ok()
    }
}
impl ::core::convert::From<ITransactionExport> for ::windows_core::IUnknown {
    fn from(value: ITransactionExport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionExport> for ::windows_core::IUnknown {
    fn from(value: &ITransactionExport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransactionExport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransactionExport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransactionExport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionExport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionExport {}
impl ::core::fmt::Debug for ITransactionExport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionExport").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransactionExport {
    type Vtable = ITransactionExport_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0141fda5_8fc0_11ce_bd18_204c4f4f5020);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionExport_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Export: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punktransaction: *mut ::core::ffi::c_void, pcbtransactioncookie: *mut u32) -> ::windows_core::HRESULT,
    pub GetTransactionCookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punktransaction: *mut ::core::ffi::c_void, cbtransactioncookie: u32, rgbtransactioncookie: *mut u8, pcbused: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITransactionExportFactory(::windows_core::IUnknown);
impl ITransactionExportFactory {
    pub unsafe fn GetRemoteClassId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).GetRemoteClassId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn Create(&self, rgbwhereabouts: &[u8]) -> ::windows_core::Result<ITransactionExport> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), rgbwhereabouts.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(rgbwhereabouts)), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransactionExport>(result__)
    }
}
impl ::core::convert::From<ITransactionExportFactory> for ::windows_core::IUnknown {
    fn from(value: ITransactionExportFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionExportFactory> for ::windows_core::IUnknown {
    fn from(value: &ITransactionExportFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransactionExportFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransactionExportFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransactionExportFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionExportFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionExportFactory {}
impl ::core::fmt::Debug for ITransactionExportFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionExportFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransactionExportFactory {
    type Vtable = ITransactionExportFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe1cf9b53_8745_11ce_a9ba_00aa006c3706);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionExportFactory_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetRemoteClassId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbwhereabouts: u32, rgbwhereabouts: *const u8, ppexport: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITransactionImport(::windows_core::IUnknown);
impl ITransactionImport {
    pub unsafe fn Import<T: ::windows_core::Interface>(&self, rgbtransactioncookie: &[u8]) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).Import)(::windows_core::Interface::as_raw(self), rgbtransactioncookie.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(rgbtransactioncookie)), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<ITransactionImport> for ::windows_core::IUnknown {
    fn from(value: ITransactionImport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionImport> for ::windows_core::IUnknown {
    fn from(value: &ITransactionImport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransactionImport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransactionImport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransactionImport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionImport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionImport {}
impl ::core::fmt::Debug for ITransactionImport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionImport").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransactionImport {
    type Vtable = ITransactionImport_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe1cf9b5a_8745_11ce_a9ba_00aa006c3706);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionImport_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Import: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbtransactioncookie: u32, rgbtransactioncookie: *const u8, piid: *const ::windows_core::GUID, ppvtransaction: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITransactionImportWhereabouts(::windows_core::IUnknown);
impl ITransactionImportWhereabouts {
    pub unsafe fn GetWhereaboutsSize(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetWhereaboutsSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetWhereabouts(&self, rgbwhereabouts: &mut [u8], pcbused: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetWhereabouts)(::windows_core::Interface::as_raw(self), rgbwhereabouts.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(rgbwhereabouts)), ::core::mem::transmute(pcbused)).ok()
    }
}
impl ::core::convert::From<ITransactionImportWhereabouts> for ::windows_core::IUnknown {
    fn from(value: ITransactionImportWhereabouts) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionImportWhereabouts> for ::windows_core::IUnknown {
    fn from(value: &ITransactionImportWhereabouts) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransactionImportWhereabouts {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransactionImportWhereabouts {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransactionImportWhereabouts {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionImportWhereabouts {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionImportWhereabouts {}
impl ::core::fmt::Debug for ITransactionImportWhereabouts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionImportWhereabouts").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransactionImportWhereabouts {
    type Vtable = ITransactionImportWhereabouts_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0141fda4_8fc0_11ce_bd18_204c4f4f5020);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionImportWhereabouts_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetWhereaboutsSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbwhereabouts: *mut u32) -> ::windows_core::HRESULT,
    pub GetWhereabouts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbwhereabouts: u32, rgbwhereabouts: *mut u8, pcbused: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITransactionLastEnlistmentAsync(::windows_core::IUnknown);
impl ITransactionLastEnlistmentAsync {
    pub unsafe fn TransactionOutcome(&self, xactstat: XACTSTAT, pboidreason: *const BOID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TransactionOutcome)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(xactstat), ::core::mem::transmute(pboidreason)).ok()
    }
}
impl ::core::convert::From<ITransactionLastEnlistmentAsync> for ::windows_core::IUnknown {
    fn from(value: ITransactionLastEnlistmentAsync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionLastEnlistmentAsync> for ::windows_core::IUnknown {
    fn from(value: &ITransactionLastEnlistmentAsync) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransactionLastEnlistmentAsync {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransactionLastEnlistmentAsync {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransactionLastEnlistmentAsync {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionLastEnlistmentAsync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionLastEnlistmentAsync {}
impl ::core::fmt::Debug for ITransactionLastEnlistmentAsync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionLastEnlistmentAsync").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransactionLastEnlistmentAsync {
    type Vtable = ITransactionLastEnlistmentAsync_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc82bd533_5b30_11d3_8a91_00c04f79eb6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionLastEnlistmentAsync_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub TransactionOutcome: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xactstat: XACTSTAT, pboidreason: *const BOID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITransactionLastResourceAsync(::windows_core::IUnknown);
impl ITransactionLastResourceAsync {
    pub unsafe fn DelegateCommit(&self, grfrm: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DelegateCommit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(grfrm)).ok()
    }
    pub unsafe fn ForgetRequest(&self, pnewuow: *const BOID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ForgetRequest)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pnewuow)).ok()
    }
}
impl ::core::convert::From<ITransactionLastResourceAsync> for ::windows_core::IUnknown {
    fn from(value: ITransactionLastResourceAsync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionLastResourceAsync> for ::windows_core::IUnknown {
    fn from(value: &ITransactionLastResourceAsync) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransactionLastResourceAsync {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransactionLastResourceAsync {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransactionLastResourceAsync {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionLastResourceAsync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionLastResourceAsync {}
impl ::core::fmt::Debug for ITransactionLastResourceAsync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionLastResourceAsync").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransactionLastResourceAsync {
    type Vtable = ITransactionLastResourceAsync_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc82bd532_5b30_11d3_8a91_00c04f79eb6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionLastResourceAsync_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub DelegateCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfrm: u32) -> ::windows_core::HRESULT,
    pub ForgetRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnewuow: *const BOID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITransactionOptions(::windows_core::IUnknown);
impl ITransactionOptions {
    pub unsafe fn SetOptions(&self, poptions: *const XACTOPT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(poptions)).ok()
    }
    pub unsafe fn GetOptions(&self, poptions: *mut XACTOPT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetOptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(poptions)).ok()
    }
}
impl ::core::convert::From<ITransactionOptions> for ::windows_core::IUnknown {
    fn from(value: ITransactionOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionOptions> for ::windows_core::IUnknown {
    fn from(value: &ITransactionOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransactionOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransactionOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransactionOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionOptions {}
impl ::core::fmt::Debug for ITransactionOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransactionOptions {
    type Vtable = ITransactionOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3a6ad9e0_23b9_11cf_ad60_00aa00a74ccd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionOptions_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poptions: *const XACTOPT) -> ::windows_core::HRESULT,
    pub GetOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poptions: *mut XACTOPT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITransactionOutcomeEvents(::windows_core::IUnknown);
impl ITransactionOutcomeEvents {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Committed<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fretaining: Param0, pnewuow: *const BOID, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Committed)(::windows_core::Interface::as_raw(self), fretaining.into_param().abi(), ::core::mem::transmute(pnewuow), ::core::mem::transmute(hr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Aborted<'a, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pboidreason: *const BOID, fretaining: Param1, pnewuow: *const BOID, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Aborted)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pboidreason), fretaining.into_param().abi(), ::core::mem::transmute(pnewuow), ::core::mem::transmute(hr)).ok()
    }
    pub unsafe fn HeuristicDecision(&self, dwdecision: u32, pboidreason: *const BOID, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).HeuristicDecision)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwdecision), ::core::mem::transmute(pboidreason), ::core::mem::transmute(hr)).ok()
    }
    pub unsafe fn Indoubt(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Indoubt)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ITransactionOutcomeEvents> for ::windows_core::IUnknown {
    fn from(value: ITransactionOutcomeEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionOutcomeEvents> for ::windows_core::IUnknown {
    fn from(value: &ITransactionOutcomeEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransactionOutcomeEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransactionOutcomeEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransactionOutcomeEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionOutcomeEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionOutcomeEvents {}
impl ::core::fmt::Debug for ITransactionOutcomeEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionOutcomeEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransactionOutcomeEvents {
    type Vtable = ITransactionOutcomeEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3a6ad9e2_23b9_11cf_ad60_00aa00a74ccd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionOutcomeEvents_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Committed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Committed: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Aborted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Aborted: usize,
    pub HeuristicDecision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdecision: u32, pboidreason: *const BOID, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub Indoubt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITransactionPhase0EnlistmentAsync(::windows_core::IUnknown);
impl ITransactionPhase0EnlistmentAsync {
    pub unsafe fn Enable(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Enable)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WaitForEnlistment(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WaitForEnlistment)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Phase0Done(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Phase0Done)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Unenlist(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Unenlist)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetTransaction(&self) -> ::windows_core::Result<ITransaction> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetTransaction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransaction>(result__)
    }
}
impl ::core::convert::From<ITransactionPhase0EnlistmentAsync> for ::windows_core::IUnknown {
    fn from(value: ITransactionPhase0EnlistmentAsync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionPhase0EnlistmentAsync> for ::windows_core::IUnknown {
    fn from(value: &ITransactionPhase0EnlistmentAsync) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransactionPhase0EnlistmentAsync {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransactionPhase0EnlistmentAsync {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransactionPhase0EnlistmentAsync {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionPhase0EnlistmentAsync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionPhase0EnlistmentAsync {}
impl ::core::fmt::Debug for ITransactionPhase0EnlistmentAsync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionPhase0EnlistmentAsync").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransactionPhase0EnlistmentAsync {
    type Vtable = ITransactionPhase0EnlistmentAsync_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x82dc88e1_a954_11d1_8f88_00600895e7d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionPhase0EnlistmentAsync_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub WaitForEnlistment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Phase0Done: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Unenlist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppitransaction: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITransactionPhase0Factory(::windows_core::IUnknown);
impl ITransactionPhase0Factory {
    pub unsafe fn Create<'a, Param0: ::windows_core::IntoParam<'a, ITransactionPhase0NotifyAsync>>(&self, pphase0notify: Param0) -> ::windows_core::Result<ITransactionPhase0EnlistmentAsync> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), pphase0notify.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransactionPhase0EnlistmentAsync>(result__)
    }
}
impl ::core::convert::From<ITransactionPhase0Factory> for ::windows_core::IUnknown {
    fn from(value: ITransactionPhase0Factory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionPhase0Factory> for ::windows_core::IUnknown {
    fn from(value: &ITransactionPhase0Factory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransactionPhase0Factory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransactionPhase0Factory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransactionPhase0Factory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionPhase0Factory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionPhase0Factory {}
impl ::core::fmt::Debug for ITransactionPhase0Factory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionPhase0Factory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransactionPhase0Factory {
    type Vtable = ITransactionPhase0Factory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x82dc88e0_a954_11d1_8f88_00600895e7d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionPhase0Factory_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pphase0notify: ::windows_core::RawPtr, ppphase0enlistment: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITransactionPhase0NotifyAsync(::windows_core::IUnknown);
impl ITransactionPhase0NotifyAsync {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Phase0Request<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fabortinghint: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Phase0Request)(::windows_core::Interface::as_raw(self), fabortinghint.into_param().abi()).ok()
    }
    pub unsafe fn EnlistCompleted(&self, status: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnlistCompleted)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(status)).ok()
    }
}
impl ::core::convert::From<ITransactionPhase0NotifyAsync> for ::windows_core::IUnknown {
    fn from(value: ITransactionPhase0NotifyAsync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionPhase0NotifyAsync> for ::windows_core::IUnknown {
    fn from(value: &ITransactionPhase0NotifyAsync) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransactionPhase0NotifyAsync {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransactionPhase0NotifyAsync {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransactionPhase0NotifyAsync {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionPhase0NotifyAsync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionPhase0NotifyAsync {}
impl ::core::fmt::Debug for ITransactionPhase0NotifyAsync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionPhase0NotifyAsync").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransactionPhase0NotifyAsync {
    type Vtable = ITransactionPhase0NotifyAsync_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef081809_0c76_11d2_87a6_00c04f990f34);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionPhase0NotifyAsync_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Phase0Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fabortinghint: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Phase0Request: usize,
    pub EnlistCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITransactionReceiver(::windows_core::IUnknown);
impl ITransactionReceiver {
    pub unsafe fn UnmarshalPropagationToken(&self, rgbtoken: &[u8]) -> ::windows_core::Result<ITransaction> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).UnmarshalPropagationToken)(::windows_core::Interface::as_raw(self), rgbtoken.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(rgbtoken)), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransaction>(result__)
    }
    pub unsafe fn GetReturnTokenSize(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetReturnTokenSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn MarshalReturnToken(&self, rgbreturntoken: &mut [u8], pcbused: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MarshalReturnToken)(::windows_core::Interface::as_raw(self), rgbreturntoken.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(rgbreturntoken)), ::core::mem::transmute(pcbused)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ITransactionReceiver> for ::windows_core::IUnknown {
    fn from(value: ITransactionReceiver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionReceiver> for ::windows_core::IUnknown {
    fn from(value: &ITransactionReceiver) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransactionReceiver {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransactionReceiver {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransactionReceiver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionReceiver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionReceiver {}
impl ::core::fmt::Debug for ITransactionReceiver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionReceiver").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransactionReceiver {
    type Vtable = ITransactionReceiver_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x59313e03_b36c_11cf_a539_00aa006887c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionReceiver_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub UnmarshalPropagationToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbtoken: u32, rgbtoken: *const u8, pptransaction: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetReturnTokenSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbreturntoken: *mut u32) -> ::windows_core::HRESULT,
    pub MarshalReturnToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbreturntoken: u32, rgbreturntoken: *mut u8, pcbused: *mut u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITransactionReceiverFactory(::windows_core::IUnknown);
impl ITransactionReceiverFactory {
    pub unsafe fn Create(&self) -> ::windows_core::Result<ITransactionReceiver> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransactionReceiver>(result__)
    }
}
impl ::core::convert::From<ITransactionReceiverFactory> for ::windows_core::IUnknown {
    fn from(value: ITransactionReceiverFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionReceiverFactory> for ::windows_core::IUnknown {
    fn from(value: &ITransactionReceiverFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransactionReceiverFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransactionReceiverFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransactionReceiverFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionReceiverFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionReceiverFactory {}
impl ::core::fmt::Debug for ITransactionReceiverFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionReceiverFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransactionReceiverFactory {
    type Vtable = ITransactionReceiverFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x59313e02_b36c_11cf_a539_00aa006887c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionReceiverFactory_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppreceiver: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITransactionResource(::windows_core::IUnknown);
impl ITransactionResource {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PrepareRequest<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fretaining: Param0, grfrm: u32, fwantmoniker: Param2, fsinglephase: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PrepareRequest)(::windows_core::Interface::as_raw(self), fretaining.into_param().abi(), ::core::mem::transmute(grfrm), fwantmoniker.into_param().abi(), fsinglephase.into_param().abi()).ok()
    }
    pub unsafe fn CommitRequest(&self, grfrm: u32, pnewuow: *const BOID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CommitRequest)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(grfrm), ::core::mem::transmute(pnewuow)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AbortRequest<'a, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pboidreason: *const BOID, fretaining: Param1, pnewuow: *const BOID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AbortRequest)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pboidreason), fretaining.into_param().abi(), ::core::mem::transmute(pnewuow)).ok()
    }
    pub unsafe fn TMDown(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TMDown)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ITransactionResource> for ::windows_core::IUnknown {
    fn from(value: ITransactionResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionResource> for ::windows_core::IUnknown {
    fn from(value: &ITransactionResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransactionResource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransactionResource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransactionResource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionResource {}
impl ::core::fmt::Debug for ITransactionResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionResource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransactionResource {
    type Vtable = ITransactionResource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xee5ff7b3_4572_11d0_9452_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionResource_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub PrepareRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fretaining: super::super::Foundation::BOOL, grfrm: u32, fwantmoniker: super::super::Foundation::BOOL, fsinglephase: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PrepareRequest: usize,
    pub CommitRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfrm: u32, pnewuow: *const BOID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AbortRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AbortRequest: usize,
    pub TMDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITransactionResourceAsync(::windows_core::IUnknown);
impl ITransactionResourceAsync {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PrepareRequest<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fretaining: Param0, grfrm: u32, fwantmoniker: Param2, fsinglephase: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PrepareRequest)(::windows_core::Interface::as_raw(self), fretaining.into_param().abi(), ::core::mem::transmute(grfrm), fwantmoniker.into_param().abi(), fsinglephase.into_param().abi()).ok()
    }
    pub unsafe fn CommitRequest(&self, grfrm: u32, pnewuow: *const BOID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CommitRequest)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(grfrm), ::core::mem::transmute(pnewuow)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AbortRequest<'a, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pboidreason: *const BOID, fretaining: Param1, pnewuow: *const BOID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AbortRequest)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pboidreason), fretaining.into_param().abi(), ::core::mem::transmute(pnewuow)).ok()
    }
    pub unsafe fn TMDown(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TMDown)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ITransactionResourceAsync> for ::windows_core::IUnknown {
    fn from(value: ITransactionResourceAsync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionResourceAsync> for ::windows_core::IUnknown {
    fn from(value: &ITransactionResourceAsync) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransactionResourceAsync {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransactionResourceAsync {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransactionResourceAsync {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionResourceAsync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionResourceAsync {}
impl ::core::fmt::Debug for ITransactionResourceAsync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionResourceAsync").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransactionResourceAsync {
    type Vtable = ITransactionResourceAsync_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x69e971f0_23ce_11cf_ad60_00aa00a74ccd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionResourceAsync_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub PrepareRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fretaining: super::super::Foundation::BOOL, grfrm: u32, fwantmoniker: super::super::Foundation::BOOL, fsinglephase: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PrepareRequest: usize,
    pub CommitRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfrm: u32, pnewuow: *const BOID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AbortRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AbortRequest: usize,
    pub TMDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITransactionTransmitter(::windows_core::IUnknown);
impl ITransactionTransmitter {
    pub unsafe fn Set<'a, Param0: ::windows_core::IntoParam<'a, ITransaction>>(&self, ptransaction: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Set)(::windows_core::Interface::as_raw(self), ptransaction.into_param().abi()).ok()
    }
    pub unsafe fn GetPropagationTokenSize(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetPropagationTokenSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn MarshalPropagationToken(&self, rgbtoken: &mut [u8], pcbused: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MarshalPropagationToken)(::windows_core::Interface::as_raw(self), rgbtoken.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(rgbtoken)), ::core::mem::transmute(pcbused)).ok()
    }
    pub unsafe fn UnmarshalReturnToken(&self, rgbreturntoken: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnmarshalReturnToken)(::windows_core::Interface::as_raw(self), rgbreturntoken.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(rgbreturntoken))).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ITransactionTransmitter> for ::windows_core::IUnknown {
    fn from(value: ITransactionTransmitter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionTransmitter> for ::windows_core::IUnknown {
    fn from(value: &ITransactionTransmitter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransactionTransmitter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransactionTransmitter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransactionTransmitter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionTransmitter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionTransmitter {}
impl ::core::fmt::Debug for ITransactionTransmitter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionTransmitter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransactionTransmitter {
    type Vtable = ITransactionTransmitter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x59313e01_b36c_11cf_a539_00aa006887c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionTransmitter_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Set: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptransaction: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetPropagationTokenSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbtoken: *mut u32) -> ::windows_core::HRESULT,
    pub MarshalPropagationToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbtoken: u32, rgbtoken: *mut u8, pcbused: *mut u32) -> ::windows_core::HRESULT,
    pub UnmarshalReturnToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbreturntoken: u32, rgbreturntoken: *const u8) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITransactionTransmitterFactory(::windows_core::IUnknown);
impl ITransactionTransmitterFactory {
    pub unsafe fn Create(&self) -> ::windows_core::Result<ITransactionTransmitter> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransactionTransmitter>(result__)
    }
}
impl ::core::convert::From<ITransactionTransmitterFactory> for ::windows_core::IUnknown {
    fn from(value: ITransactionTransmitterFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionTransmitterFactory> for ::windows_core::IUnknown {
    fn from(value: &ITransactionTransmitterFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransactionTransmitterFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransactionTransmitterFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransactionTransmitterFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionTransmitterFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionTransmitterFactory {}
impl ::core::fmt::Debug for ITransactionTransmitterFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionTransmitterFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransactionTransmitterFactory {
    type Vtable = ITransactionTransmitterFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x59313e00_b36c_11cf_a539_00aa006887c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionTransmitterFactory_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptransmitter: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITransactionVoterBallotAsync2(::windows_core::IUnknown);
impl ITransactionVoterBallotAsync2 {
    pub unsafe fn VoteRequestDone(&self, hr: ::windows_core::HRESULT, pboidreason: *const BOID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).VoteRequestDone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hr), ::core::mem::transmute(pboidreason)).ok()
    }
}
impl ::core::convert::From<ITransactionVoterBallotAsync2> for ::windows_core::IUnknown {
    fn from(value: ITransactionVoterBallotAsync2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionVoterBallotAsync2> for ::windows_core::IUnknown {
    fn from(value: &ITransactionVoterBallotAsync2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransactionVoterBallotAsync2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransactionVoterBallotAsync2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransactionVoterBallotAsync2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionVoterBallotAsync2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionVoterBallotAsync2 {}
impl ::core::fmt::Debug for ITransactionVoterBallotAsync2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionVoterBallotAsync2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransactionVoterBallotAsync2 {
    type Vtable = ITransactionVoterBallotAsync2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5433376c_414d_11d3_b206_00c04fc2f3ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionVoterBallotAsync2_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub VoteRequestDone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT, pboidreason: *const BOID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITransactionVoterFactory2(::windows_core::IUnknown);
impl ITransactionVoterFactory2 {
    pub unsafe fn Create<'a, Param0: ::windows_core::IntoParam<'a, ITransaction>, Param1: ::windows_core::IntoParam<'a, ITransactionVoterNotifyAsync2>>(&self, ptransaction: Param0, pvoternotify: Param1) -> ::windows_core::Result<ITransactionVoterBallotAsync2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), ptransaction.into_param().abi(), pvoternotify.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransactionVoterBallotAsync2>(result__)
    }
}
impl ::core::convert::From<ITransactionVoterFactory2> for ::windows_core::IUnknown {
    fn from(value: ITransactionVoterFactory2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionVoterFactory2> for ::windows_core::IUnknown {
    fn from(value: &ITransactionVoterFactory2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransactionVoterFactory2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransactionVoterFactory2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransactionVoterFactory2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionVoterFactory2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionVoterFactory2 {}
impl ::core::fmt::Debug for ITransactionVoterFactory2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionVoterFactory2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransactionVoterFactory2 {
    type Vtable = ITransactionVoterFactory2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5433376a_414d_11d3_b206_00c04fc2f3ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionVoterFactory2_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptransaction: ::windows_core::RawPtr, pvoternotify: ::windows_core::RawPtr, ppvoterballot: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITransactionVoterNotifyAsync2(::windows_core::IUnknown);
impl ITransactionVoterNotifyAsync2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Committed<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fretaining: Param0, pnewuow: *const BOID, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Committed)(::windows_core::Interface::as_raw(self), fretaining.into_param().abi(), ::core::mem::transmute(pnewuow), ::core::mem::transmute(hr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Aborted<'a, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pboidreason: *const BOID, fretaining: Param1, pnewuow: *const BOID, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Aborted)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pboidreason), fretaining.into_param().abi(), ::core::mem::transmute(pnewuow), ::core::mem::transmute(hr)).ok()
    }
    pub unsafe fn HeuristicDecision(&self, dwdecision: u32, pboidreason: *const BOID, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.HeuristicDecision)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwdecision), ::core::mem::transmute(pboidreason), ::core::mem::transmute(hr)).ok()
    }
    pub unsafe fn Indoubt(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Indoubt)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn VoteRequest(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).VoteRequest)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ITransactionVoterNotifyAsync2> for ::windows_core::IUnknown {
    fn from(value: ITransactionVoterNotifyAsync2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionVoterNotifyAsync2> for ::windows_core::IUnknown {
    fn from(value: &ITransactionVoterNotifyAsync2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransactionVoterNotifyAsync2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransactionVoterNotifyAsync2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITransactionVoterNotifyAsync2> for ITransactionOutcomeEvents {
    fn from(value: ITransactionVoterNotifyAsync2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionVoterNotifyAsync2> for ITransactionOutcomeEvents {
    fn from(value: &ITransactionVoterNotifyAsync2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ITransactionOutcomeEvents> for ITransactionVoterNotifyAsync2 {
    fn into_param(self) -> ::windows_core::Param<'a, ITransactionOutcomeEvents> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ITransactionOutcomeEvents> for &'a ITransactionVoterNotifyAsync2 {
    fn into_param(self) -> ::windows_core::Param<'a, ITransactionOutcomeEvents> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransactionVoterNotifyAsync2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionVoterNotifyAsync2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionVoterNotifyAsync2 {}
impl ::core::fmt::Debug for ITransactionVoterNotifyAsync2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionVoterNotifyAsync2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransactionVoterNotifyAsync2 {
    type Vtable = ITransactionVoterNotifyAsync2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5433376b_414d_11d3_b206_00c04fc2f3ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionVoterNotifyAsync2_Vtbl {
    pub base__: ITransactionOutcomeEvents_Vtbl,
    pub VoteRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IXAConfig(::windows_core::IUnknown);
impl IXAConfig {
    pub unsafe fn Initialize<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, clsidhelperdll: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), clsidhelperdll.into_param().abi()).ok()
    }
    pub unsafe fn Terminate(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Terminate)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IXAConfig> for ::windows_core::IUnknown {
    fn from(value: IXAConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXAConfig> for ::windows_core::IUnknown {
    fn from(value: &IXAConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IXAConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IXAConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXAConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXAConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXAConfig {}
impl ::core::fmt::Debug for IXAConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXAConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IXAConfig {
    type Vtable = IXAConfig_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc8a6e3a1_9a8c_11cf_a308_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAConfig_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsidhelperdll: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Terminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IXAObtainRMInfo(::windows_core::IUnknown);
impl IXAObtainRMInfo {
    pub unsafe fn ObtainRMInfo<'a, Param0: ::windows_core::IntoParam<'a, IRMHelper>>(&self, pirmhelper: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ObtainRMInfo)(::windows_core::Interface::as_raw(self), pirmhelper.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IXAObtainRMInfo> for ::windows_core::IUnknown {
    fn from(value: IXAObtainRMInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXAObtainRMInfo> for ::windows_core::IUnknown {
    fn from(value: &IXAObtainRMInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IXAObtainRMInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IXAObtainRMInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXAObtainRMInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXAObtainRMInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXAObtainRMInfo {}
impl ::core::fmt::Debug for IXAObtainRMInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXAObtainRMInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IXAObtainRMInfo {
    type Vtable = IXAObtainRMInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe793f6d2_f53d_11cf_a60d_00a0c905416e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAObtainRMInfo_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ObtainRMInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pirmhelper: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IXATransLookup(::windows_core::IUnknown);
impl IXATransLookup {
    pub unsafe fn Lookup(&self) -> ::windows_core::Result<ITransaction> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Lookup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransaction>(result__)
    }
}
impl ::core::convert::From<IXATransLookup> for ::windows_core::IUnknown {
    fn from(value: IXATransLookup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXATransLookup> for ::windows_core::IUnknown {
    fn from(value: &IXATransLookup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IXATransLookup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IXATransLookup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXATransLookup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXATransLookup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXATransLookup {}
impl ::core::fmt::Debug for IXATransLookup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXATransLookup").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IXATransLookup {
    type Vtable = IXATransLookup_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf3b1f131_eeda_11ce_aed4_00aa0051e2c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXATransLookup_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Lookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptransaction: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IXATransLookup2(::windows_core::IUnknown);
impl IXATransLookup2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Lookup(&self, pxid: *const xid_t) -> ::windows_core::Result<ITransaction> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Lookup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pxid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransaction>(result__)
    }
}
impl ::core::convert::From<IXATransLookup2> for ::windows_core::IUnknown {
    fn from(value: IXATransLookup2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXATransLookup2> for ::windows_core::IUnknown {
    fn from(value: &IXATransLookup2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IXATransLookup2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IXATransLookup2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXATransLookup2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXATransLookup2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXATransLookup2 {}
impl ::core::fmt::Debug for IXATransLookup2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXATransLookup2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IXATransLookup2 {
    type Vtable = IXATransLookup2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbf193c85_0d1a_4290_b88f_d2cb8873d1e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXATransLookup2_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Lookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pxid: *const xid_t, pptransaction: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Lookup: usize,
}
pub const MAXBQUALSIZE: u32 = 64u32;
pub const MAXGTRIDSIZE: u32 = 64u32;
pub const MAXINFOSIZE: u32 = 256u32;
#[repr(C)]
pub struct OLE_TM_CONFIG_PARAMS_V1 {
    pub dwVersion: u32,
    pub dwcConcurrencyHint: u32,
}
impl ::core::marker::Copy for OLE_TM_CONFIG_PARAMS_V1 {}
impl ::core::clone::Clone for OLE_TM_CONFIG_PARAMS_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OLE_TM_CONFIG_PARAMS_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLE_TM_CONFIG_PARAMS_V1").field("dwVersion", &self.dwVersion).field("dwcConcurrencyHint", &self.dwcConcurrencyHint).finish()
    }
}
unsafe impl ::windows_core::Abi for OLE_TM_CONFIG_PARAMS_V1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for OLE_TM_CONFIG_PARAMS_V1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OLE_TM_CONFIG_PARAMS_V1>()) == 0 }
    }
}
impl ::core::cmp::Eq for OLE_TM_CONFIG_PARAMS_V1 {}
impl ::core::default::Default for OLE_TM_CONFIG_PARAMS_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct OLE_TM_CONFIG_PARAMS_V2 {
    pub dwVersion: u32,
    pub dwcConcurrencyHint: u32,
    pub applicationType: APPLICATIONTYPE,
    pub clusterResourceId: ::windows_core::GUID,
}
impl ::core::marker::Copy for OLE_TM_CONFIG_PARAMS_V2 {}
impl ::core::clone::Clone for OLE_TM_CONFIG_PARAMS_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OLE_TM_CONFIG_PARAMS_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLE_TM_CONFIG_PARAMS_V2").field("dwVersion", &self.dwVersion).field("dwcConcurrencyHint", &self.dwcConcurrencyHint).field("applicationType", &self.applicationType).field("clusterResourceId", &self.clusterResourceId).finish()
    }
}
unsafe impl ::windows_core::Abi for OLE_TM_CONFIG_PARAMS_V2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for OLE_TM_CONFIG_PARAMS_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OLE_TM_CONFIG_PARAMS_V2>()) == 0 }
    }
}
impl ::core::cmp::Eq for OLE_TM_CONFIG_PARAMS_V2 {}
impl ::core::default::Default for OLE_TM_CONFIG_PARAMS_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const OLE_TM_CONFIG_VERSION_1: u32 = 1u32;
pub const OLE_TM_CONFIG_VERSION_2: u32 = 2u32;
pub const OLE_TM_FLAG_INTERNAL_TO_TM: u32 = 1073741824u32;
pub const OLE_TM_FLAG_NOAGILERECOVERY: u32 = 2u32;
pub const OLE_TM_FLAG_NODEMANDSTART: u32 = 1u32;
pub const OLE_TM_FLAG_NONE: u32 = 0u32;
pub const OLE_TM_FLAG_QUERY_SERVICE_LOCKSTATUS: u32 = 2147483648u32;
pub const RMNAMESZ: u32 = 32u32;
pub const TMASYNC: i32 = -2147483648i32;
pub const TMENDRSCAN: i32 = 8388608i32;
pub const TMER_INVAL: i32 = -2i32;
pub const TMER_PROTO: i32 = -3i32;
pub const TMER_TMERR: i32 = -1i32;
pub const TMFAIL: i32 = 536870912i32;
pub const TMJOIN: i32 = 2097152i32;
pub const TMMIGRATE: i32 = 1048576i32;
pub const TMMULTIPLE: i32 = 4194304i32;
pub const TMNOFLAGS: i32 = 0i32;
pub const TMNOMIGRATE: i32 = 2i32;
pub const TMNOWAIT: i32 = 268435456i32;
pub const TMONEPHASE: i32 = 1073741824i32;
pub const TMREGISTER: i32 = 1i32;
pub const TMRESUME: i32 = 134217728i32;
pub const TMSTARTRSCAN: i32 = 16777216i32;
pub const TMSUCCESS: i32 = 67108864i32;
pub const TMSUSPEND: i32 = 33554432i32;
pub const TMUSEASYNC: i32 = 4i32;
pub const TM_JOIN: u32 = 2u32;
pub const TM_OK: u32 = 0u32;
pub const TM_RESUME: u32 = 1u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TX_MISC_CONSTANTS(pub i32);
pub const MAX_TRAN_DESC: TX_MISC_CONSTANTS = TX_MISC_CONSTANTS(40i32);
impl ::core::marker::Copy for TX_MISC_CONSTANTS {}
impl ::core::clone::Clone for TX_MISC_CONSTANTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TX_MISC_CONSTANTS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TX_MISC_CONSTANTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TX_MISC_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TX_MISC_CONSTANTS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XACTCONST(pub i32);
pub const XACTCONST_TIMEOUTINFINITE: XACTCONST = XACTCONST(0i32);
impl ::core::marker::Copy for XACTCONST {}
impl ::core::clone::Clone for XACTCONST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XACTCONST {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for XACTCONST {
    type Abi = Self;
}
impl ::core::fmt::Debug for XACTCONST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XACTCONST").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XACTHEURISTIC(pub i32);
pub const XACTHEURISTIC_ABORT: XACTHEURISTIC = XACTHEURISTIC(1i32);
pub const XACTHEURISTIC_COMMIT: XACTHEURISTIC = XACTHEURISTIC(2i32);
pub const XACTHEURISTIC_DAMAGE: XACTHEURISTIC = XACTHEURISTIC(3i32);
pub const XACTHEURISTIC_DANGER: XACTHEURISTIC = XACTHEURISTIC(4i32);
impl ::core::marker::Copy for XACTHEURISTIC {}
impl ::core::clone::Clone for XACTHEURISTIC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XACTHEURISTIC {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for XACTHEURISTIC {
    type Abi = Self;
}
impl ::core::fmt::Debug for XACTHEURISTIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XACTHEURISTIC").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct XACTOPT {
    pub ulTimeout: u32,
    pub szDescription: [u8; 40],
}
impl ::core::marker::Copy for XACTOPT {}
impl ::core::clone::Clone for XACTOPT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XACTOPT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XACTOPT").field("ulTimeout", &self.ulTimeout).field("szDescription", &self.szDescription).finish()
    }
}
unsafe impl ::windows_core::Abi for XACTOPT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for XACTOPT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<XACTOPT>()) == 0 }
    }
}
impl ::core::cmp::Eq for XACTOPT {}
impl ::core::default::Default for XACTOPT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XACTRM(pub i32);
pub const XACTRM_OPTIMISTICLASTWINS: XACTRM = XACTRM(1i32);
pub const XACTRM_NOREADONLYPREPARES: XACTRM = XACTRM(2i32);
impl ::core::marker::Copy for XACTRM {}
impl ::core::clone::Clone for XACTRM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XACTRM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for XACTRM {
    type Abi = Self;
}
impl ::core::fmt::Debug for XACTRM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XACTRM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XACTSTAT(pub i32);
pub const XACTSTAT_NONE: XACTSTAT = XACTSTAT(0i32);
pub const XACTSTAT_OPENNORMAL: XACTSTAT = XACTSTAT(1i32);
pub const XACTSTAT_OPENREFUSED: XACTSTAT = XACTSTAT(2i32);
pub const XACTSTAT_PREPARING: XACTSTAT = XACTSTAT(4i32);
pub const XACTSTAT_PREPARED: XACTSTAT = XACTSTAT(8i32);
pub const XACTSTAT_PREPARERETAINING: XACTSTAT = XACTSTAT(16i32);
pub const XACTSTAT_PREPARERETAINED: XACTSTAT = XACTSTAT(32i32);
pub const XACTSTAT_COMMITTING: XACTSTAT = XACTSTAT(64i32);
pub const XACTSTAT_COMMITRETAINING: XACTSTAT = XACTSTAT(128i32);
pub const XACTSTAT_ABORTING: XACTSTAT = XACTSTAT(256i32);
pub const XACTSTAT_ABORTED: XACTSTAT = XACTSTAT(512i32);
pub const XACTSTAT_COMMITTED: XACTSTAT = XACTSTAT(1024i32);
pub const XACTSTAT_HEURISTIC_ABORT: XACTSTAT = XACTSTAT(2048i32);
pub const XACTSTAT_HEURISTIC_COMMIT: XACTSTAT = XACTSTAT(4096i32);
pub const XACTSTAT_HEURISTIC_DAMAGE: XACTSTAT = XACTSTAT(8192i32);
pub const XACTSTAT_HEURISTIC_DANGER: XACTSTAT = XACTSTAT(16384i32);
pub const XACTSTAT_FORCED_ABORT: XACTSTAT = XACTSTAT(32768i32);
pub const XACTSTAT_FORCED_COMMIT: XACTSTAT = XACTSTAT(65536i32);
pub const XACTSTAT_INDOUBT: XACTSTAT = XACTSTAT(131072i32);
pub const XACTSTAT_CLOSED: XACTSTAT = XACTSTAT(262144i32);
pub const XACTSTAT_OPEN: XACTSTAT = XACTSTAT(3i32);
pub const XACTSTAT_NOTPREPARED: XACTSTAT = XACTSTAT(524227i32);
pub const XACTSTAT_ALL: XACTSTAT = XACTSTAT(524287i32);
impl ::core::marker::Copy for XACTSTAT {}
impl ::core::clone::Clone for XACTSTAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XACTSTAT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for XACTSTAT {
    type Abi = Self;
}
impl ::core::fmt::Debug for XACTSTAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XACTSTAT").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct XACTSTATS {
    pub cOpen: u32,
    pub cCommitting: u32,
    pub cCommitted: u32,
    pub cAborting: u32,
    pub cAborted: u32,
    pub cInDoubt: u32,
    pub cHeuristicDecision: u32,
    pub timeTransactionsUp: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for XACTSTATS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for XACTSTATS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for XACTSTATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XACTSTATS").field("cOpen", &self.cOpen).field("cCommitting", &self.cCommitting).field("cCommitted", &self.cCommitted).field("cAborting", &self.cAborting).field("cAborted", &self.cAborted).field("cInDoubt", &self.cInDoubt).field("cHeuristicDecision", &self.cHeuristicDecision).field("timeTransactionsUp", &self.timeTransactionsUp).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows_core::Abi for XACTSTATS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for XACTSTATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<XACTSTATS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for XACTSTATS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for XACTSTATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XACTTC(pub i32);
pub const XACTTC_NONE: XACTTC = XACTTC(0i32);
pub const XACTTC_SYNC_PHASEONE: XACTTC = XACTTC(1i32);
pub const XACTTC_SYNC_PHASETWO: XACTTC = XACTTC(2i32);
pub const XACTTC_SYNC: XACTTC = XACTTC(2i32);
pub const XACTTC_ASYNC_PHASEONE: XACTTC = XACTTC(4i32);
pub const XACTTC_ASYNC: XACTTC = XACTTC(4i32);
impl ::core::marker::Copy for XACTTC {}
impl ::core::clone::Clone for XACTTC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XACTTC {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for XACTTC {
    type Abi = Self;
}
impl ::core::fmt::Debug for XACTTC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XACTTC").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct XACTTRANSINFO {
    pub uow: BOID,
    pub isoLevel: i32,
    pub isoFlags: u32,
    pub grfTCSupported: u32,
    pub grfRMSupported: u32,
    pub grfTCSupportedRetaining: u32,
    pub grfRMSupportedRetaining: u32,
}
impl ::core::marker::Copy for XACTTRANSINFO {}
impl ::core::clone::Clone for XACTTRANSINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XACTTRANSINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XACTTRANSINFO").field("uow", &self.uow).field("isoLevel", &self.isoLevel).field("isoFlags", &self.isoFlags).field("grfTCSupported", &self.grfTCSupported).field("grfRMSupported", &self.grfRMSupported).field("grfTCSupportedRetaining", &self.grfTCSupportedRetaining).field("grfRMSupportedRetaining", &self.grfRMSupportedRetaining).finish()
    }
}
unsafe impl ::windows_core::Abi for XACTTRANSINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for XACTTRANSINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<XACTTRANSINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for XACTTRANSINFO {}
impl ::core::default::Default for XACTTRANSINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XACT_DTC_CONSTANTS(pub i32);
pub const XACT_E_CONNECTION_REQUEST_DENIED: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147168000i32);
pub const XACT_E_TOOMANY_ENLISTMENTS: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167999i32);
pub const XACT_E_DUPLICATE_GUID: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167998i32);
pub const XACT_E_NOTSINGLEPHASE: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167997i32);
pub const XACT_E_RECOVERYALREADYDONE: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167996i32);
pub const XACT_E_PROTOCOL: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167995i32);
pub const XACT_E_RM_FAILURE: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167994i32);
pub const XACT_E_RECOVERY_FAILED: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167993i32);
pub const XACT_E_LU_NOT_FOUND: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167992i32);
pub const XACT_E_DUPLICATE_LU: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167991i32);
pub const XACT_E_LU_NOT_CONNECTED: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167990i32);
pub const XACT_E_DUPLICATE_TRANSID: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167989i32);
pub const XACT_E_LU_BUSY: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167988i32);
pub const XACT_E_LU_NO_RECOVERY_PROCESS: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167987i32);
pub const XACT_E_LU_DOWN: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167986i32);
pub const XACT_E_LU_RECOVERING: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167985i32);
pub const XACT_E_LU_RECOVERY_MISMATCH: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167984i32);
pub const XACT_E_RM_UNAVAILABLE: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167983i32);
pub const XACT_E_LRMRECOVERYALREADYDONE: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167982i32);
pub const XACT_E_NOLASTRESOURCEINTERFACE: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167981i32);
pub const XACT_S_NONOTIFY: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(315648i32);
pub const XACT_OK_NONOTIFY: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(315649i32);
pub const dwUSER_MS_SQLSERVER: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(65535i32);
impl ::core::marker::Copy for XACT_DTC_CONSTANTS {}
impl ::core::clone::Clone for XACT_DTC_CONSTANTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XACT_DTC_CONSTANTS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for XACT_DTC_CONSTANTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for XACT_DTC_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XACT_DTC_CONSTANTS").field(&self.0).finish()
    }
}
pub const XAER_ASYNC: i32 = -2i32;
pub const XAER_DUPID: i32 = -8i32;
pub const XAER_INVAL: i32 = -5i32;
pub const XAER_NOTA: i32 = -4i32;
pub const XAER_OUTSIDE: i32 = -9i32;
pub const XAER_PROTO: i32 = -6i32;
pub const XAER_RMERR: i32 = -3i32;
pub const XAER_RMFAIL: i32 = -7i32;
pub type XA_CLOSE_EPT = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_core::PCSTR, param1: i32, param2: i32) -> i32>;
#[cfg(feature = "Win32_Foundation")]
pub type XA_COMMIT_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32>;
pub type XA_COMPLETE_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut i32, param1: *mut i32, param2: i32, param3: i32) -> i32>;
#[cfg(feature = "Win32_Foundation")]
pub type XA_END_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32>;
pub const XA_FMTID_DTC: u32 = 4478019u32;
pub const XA_FMTID_DTC_VER1: u32 = 21255235u32;
#[cfg(feature = "Win32_Foundation")]
pub type XA_FORGET_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32>;
pub const XA_HEURCOM: u32 = 7u32;
pub const XA_HEURHAZ: u32 = 8u32;
pub const XA_HEURMIX: u32 = 5u32;
pub const XA_HEURRB: u32 = 6u32;
pub const XA_NOMIGRATE: u32 = 9u32;
pub const XA_OK: u32 = 0u32;
pub type XA_OPEN_EPT = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_core::PCSTR, param1: i32, param2: i32) -> i32>;
#[cfg(feature = "Win32_Foundation")]
pub type XA_PREPARE_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32>;
pub const XA_RBBASE: u32 = 100u32;
pub const XA_RBCOMMFAIL: u32 = 101u32;
pub const XA_RBDEADLOCK: u32 = 102u32;
pub const XA_RBEND: u32 = 107u32;
pub const XA_RBINTEGRITY: u32 = 103u32;
pub const XA_RBOTHER: u32 = 104u32;
pub const XA_RBPROTO: u32 = 105u32;
pub const XA_RBROLLBACK: u32 = 100u32;
pub const XA_RBTIMEOUT: u32 = 106u32;
pub const XA_RBTRANSIENT: u32 = 107u32;
pub const XA_RDONLY: u32 = 3u32;
#[cfg(feature = "Win32_Foundation")]
pub type XA_RECOVER_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32, param3: i32) -> i32>;
pub const XA_RETRY: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
pub type XA_ROLLBACK_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32>;
#[cfg(feature = "Win32_Foundation")]
pub type XA_START_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32>;
pub const XA_SWITCH_F_DTC: u32 = 1u32;
pub const XIDDATASIZE: u32 = 128u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct _DtcLu_CompareState(pub i32);
pub const DTCLUCOMPARESTATE_COMMITTED: _DtcLu_CompareState = _DtcLu_CompareState(1i32);
pub const DTCLUCOMPARESTATE_HEURISTICCOMMITTED: _DtcLu_CompareState = _DtcLu_CompareState(2i32);
pub const DTCLUCOMPARESTATE_HEURISTICMIXED: _DtcLu_CompareState = _DtcLu_CompareState(3i32);
pub const DTCLUCOMPARESTATE_HEURISTICRESET: _DtcLu_CompareState = _DtcLu_CompareState(4i32);
pub const DTCLUCOMPARESTATE_INDOUBT: _DtcLu_CompareState = _DtcLu_CompareState(5i32);
pub const DTCLUCOMPARESTATE_RESET: _DtcLu_CompareState = _DtcLu_CompareState(6i32);
impl ::core::marker::Copy for _DtcLu_CompareState {}
impl ::core::clone::Clone for _DtcLu_CompareState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DtcLu_CompareState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for _DtcLu_CompareState {
    type Abi = Self;
}
impl ::core::fmt::Debug for _DtcLu_CompareState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DtcLu_CompareState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct _DtcLu_CompareStates_Confirmation(pub i32);
pub const DTCLUCOMPARESTATESCONFIRMATION_CONFIRM: _DtcLu_CompareStates_Confirmation = _DtcLu_CompareStates_Confirmation(1i32);
pub const DTCLUCOMPARESTATESCONFIRMATION_PROTOCOL: _DtcLu_CompareStates_Confirmation = _DtcLu_CompareStates_Confirmation(2i32);
impl ::core::marker::Copy for _DtcLu_CompareStates_Confirmation {}
impl ::core::clone::Clone for _DtcLu_CompareStates_Confirmation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DtcLu_CompareStates_Confirmation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for _DtcLu_CompareStates_Confirmation {
    type Abi = Self;
}
impl ::core::fmt::Debug for _DtcLu_CompareStates_Confirmation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DtcLu_CompareStates_Confirmation").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct _DtcLu_CompareStates_Error(pub i32);
pub const DTCLUCOMPARESTATESERROR_PROTOCOL: _DtcLu_CompareStates_Error = _DtcLu_CompareStates_Error(1i32);
impl ::core::marker::Copy for _DtcLu_CompareStates_Error {}
impl ::core::clone::Clone for _DtcLu_CompareStates_Error {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DtcLu_CompareStates_Error {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for _DtcLu_CompareStates_Error {
    type Abi = Self;
}
impl ::core::fmt::Debug for _DtcLu_CompareStates_Error {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DtcLu_CompareStates_Error").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct _DtcLu_CompareStates_Response(pub i32);
pub const DTCLUCOMPARESTATESRESPONSE_OK: _DtcLu_CompareStates_Response = _DtcLu_CompareStates_Response(1i32);
pub const DTCLUCOMPARESTATESRESPONSE_PROTOCOL: _DtcLu_CompareStates_Response = _DtcLu_CompareStates_Response(2i32);
impl ::core::marker::Copy for _DtcLu_CompareStates_Response {}
impl ::core::clone::Clone for _DtcLu_CompareStates_Response {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DtcLu_CompareStates_Response {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for _DtcLu_CompareStates_Response {
    type Abi = Self;
}
impl ::core::fmt::Debug for _DtcLu_CompareStates_Response {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DtcLu_CompareStates_Response").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct _DtcLu_LocalRecovery_Work(pub i32);
pub const DTCINITIATEDRECOVERYWORK_CHECKLUSTATUS: _DtcLu_LocalRecovery_Work = _DtcLu_LocalRecovery_Work(1i32);
pub const DTCINITIATEDRECOVERYWORK_TRANS: _DtcLu_LocalRecovery_Work = _DtcLu_LocalRecovery_Work(2i32);
pub const DTCINITIATEDRECOVERYWORK_TMDOWN: _DtcLu_LocalRecovery_Work = _DtcLu_LocalRecovery_Work(3i32);
impl ::core::marker::Copy for _DtcLu_LocalRecovery_Work {}
impl ::core::clone::Clone for _DtcLu_LocalRecovery_Work {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DtcLu_LocalRecovery_Work {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for _DtcLu_LocalRecovery_Work {
    type Abi = Self;
}
impl ::core::fmt::Debug for _DtcLu_LocalRecovery_Work {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DtcLu_LocalRecovery_Work").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct _DtcLu_Xln(pub i32);
pub const DTCLUXLN_COLD: _DtcLu_Xln = _DtcLu_Xln(1i32);
pub const DTCLUXLN_WARM: _DtcLu_Xln = _DtcLu_Xln(2i32);
impl ::core::marker::Copy for _DtcLu_Xln {}
impl ::core::clone::Clone for _DtcLu_Xln {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DtcLu_Xln {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for _DtcLu_Xln {
    type Abi = Self;
}
impl ::core::fmt::Debug for _DtcLu_Xln {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DtcLu_Xln").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct _DtcLu_Xln_Confirmation(pub i32);
pub const DTCLUXLNCONFIRMATION_CONFIRM: _DtcLu_Xln_Confirmation = _DtcLu_Xln_Confirmation(1i32);
pub const DTCLUXLNCONFIRMATION_LOGNAMEMISMATCH: _DtcLu_Xln_Confirmation = _DtcLu_Xln_Confirmation(2i32);
pub const DTCLUXLNCONFIRMATION_COLDWARMMISMATCH: _DtcLu_Xln_Confirmation = _DtcLu_Xln_Confirmation(3i32);
pub const DTCLUXLNCONFIRMATION_OBSOLETE: _DtcLu_Xln_Confirmation = _DtcLu_Xln_Confirmation(4i32);
impl ::core::marker::Copy for _DtcLu_Xln_Confirmation {}
impl ::core::clone::Clone for _DtcLu_Xln_Confirmation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DtcLu_Xln_Confirmation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for _DtcLu_Xln_Confirmation {
    type Abi = Self;
}
impl ::core::fmt::Debug for _DtcLu_Xln_Confirmation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DtcLu_Xln_Confirmation").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct _DtcLu_Xln_Error(pub i32);
pub const DTCLUXLNERROR_PROTOCOL: _DtcLu_Xln_Error = _DtcLu_Xln_Error(1i32);
pub const DTCLUXLNERROR_LOGNAMEMISMATCH: _DtcLu_Xln_Error = _DtcLu_Xln_Error(2i32);
pub const DTCLUXLNERROR_COLDWARMMISMATCH: _DtcLu_Xln_Error = _DtcLu_Xln_Error(3i32);
impl ::core::marker::Copy for _DtcLu_Xln_Error {}
impl ::core::clone::Clone for _DtcLu_Xln_Error {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DtcLu_Xln_Error {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for _DtcLu_Xln_Error {
    type Abi = Self;
}
impl ::core::fmt::Debug for _DtcLu_Xln_Error {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DtcLu_Xln_Error").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct _DtcLu_Xln_Response(pub i32);
pub const DTCLUXLNRESPONSE_OK_SENDOURXLNBACK: _DtcLu_Xln_Response = _DtcLu_Xln_Response(1i32);
pub const DTCLUXLNRESPONSE_OK_SENDCONFIRMATION: _DtcLu_Xln_Response = _DtcLu_Xln_Response(2i32);
pub const DTCLUXLNRESPONSE_LOGNAMEMISMATCH: _DtcLu_Xln_Response = _DtcLu_Xln_Response(3i32);
pub const DTCLUXLNRESPONSE_COLDWARMMISMATCH: _DtcLu_Xln_Response = _DtcLu_Xln_Response(4i32);
impl ::core::marker::Copy for _DtcLu_Xln_Response {}
impl ::core::clone::Clone for _DtcLu_Xln_Response {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DtcLu_Xln_Response {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for _DtcLu_Xln_Response {
    type Abi = Self;
}
impl ::core::fmt::Debug for _DtcLu_Xln_Response {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DtcLu_Xln_Response").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct _ProxyConfigParams {
    pub wcThreadsMax: u16,
}
impl ::core::marker::Copy for _ProxyConfigParams {}
impl ::core::clone::Clone for _ProxyConfigParams {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for _ProxyConfigParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_ProxyConfigParams").field("wcThreadsMax", &self.wcThreadsMax).finish()
    }
}
unsafe impl ::windows_core::Abi for _ProxyConfigParams {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for _ProxyConfigParams {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<_ProxyConfigParams>()) == 0 }
    }
}
impl ::core::cmp::Eq for _ProxyConfigParams {}
impl ::core::default::Default for _ProxyConfigParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct xa_switch_t {
    pub name: [super::super::Foundation::CHAR; 32],
    pub flags: i32,
    pub version: i32,
    pub xa_open_entry: isize,
    pub xa_close_entry: isize,
    pub xa_start_entry: isize,
    pub xa_end_entry: isize,
    pub xa_rollback_entry: isize,
    pub xa_prepare_entry: isize,
    pub xa_commit_entry: isize,
    pub xa_recover_entry: isize,
    pub xa_forget_entry: isize,
    pub xa_complete_entry: isize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for xa_switch_t {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for xa_switch_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for xa_switch_t {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("xa_switch_t")
            .field("name", &self.name)
            .field("flags", &self.flags)
            .field("version", &self.version)
            .field("xa_open_entry", &self.xa_open_entry)
            .field("xa_close_entry", &self.xa_close_entry)
            .field("xa_start_entry", &self.xa_start_entry)
            .field("xa_end_entry", &self.xa_end_entry)
            .field("xa_rollback_entry", &self.xa_rollback_entry)
            .field("xa_prepare_entry", &self.xa_prepare_entry)
            .field("xa_commit_entry", &self.xa_commit_entry)
            .field("xa_recover_entry", &self.xa_recover_entry)
            .field("xa_forget_entry", &self.xa_forget_entry)
            .field("xa_complete_entry", &self.xa_complete_entry)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows_core::Abi for xa_switch_t {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for xa_switch_t {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<xa_switch_t>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for xa_switch_t {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for xa_switch_t {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct xid_t {
    pub formatID: i32,
    pub gtrid_length: i32,
    pub bqual_length: i32,
    pub data: [super::super::Foundation::CHAR; 128],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for xid_t {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for xid_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for xid_t {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("xid_t").field("formatID", &self.formatID).field("gtrid_length", &self.gtrid_length).field("bqual_length", &self.bqual_length).field("data", &self.data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows_core::Abi for xid_t {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for xid_t {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<xid_t>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for xid_t {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for xid_t {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
