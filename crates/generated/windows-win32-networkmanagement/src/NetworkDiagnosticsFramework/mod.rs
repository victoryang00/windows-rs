#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ATTRIBUTE_TYPE(pub i32);
pub const AT_INVALID: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(0i32);
pub const AT_BOOLEAN: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(1i32);
pub const AT_INT8: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(2i32);
pub const AT_UINT8: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(3i32);
pub const AT_INT16: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(4i32);
pub const AT_UINT16: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(5i32);
pub const AT_INT32: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(6i32);
pub const AT_UINT32: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(7i32);
pub const AT_INT64: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(8i32);
pub const AT_UINT64: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(9i32);
pub const AT_STRING: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(10i32);
pub const AT_GUID: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(11i32);
pub const AT_LIFE_TIME: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(12i32);
pub const AT_SOCKADDR: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(13i32);
pub const AT_OCTET_STRING: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(14i32);
impl ::core::marker::Copy for ATTRIBUTE_TYPE {}
impl ::core::clone::Clone for ATTRIBUTE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ATTRIBUTE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ATTRIBUTE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ATTRIBUTE_TYPE").field(&self.0).finish()
    }
}
pub const DF_IMPERSONATION: u32 = 2147483648u32;
pub const DF_TRACELESS: u32 = 1073741824u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DIAGNOSIS_STATUS(pub i32);
pub const DS_NOT_IMPLEMENTED: DIAGNOSIS_STATUS = DIAGNOSIS_STATUS(0i32);
pub const DS_CONFIRMED: DIAGNOSIS_STATUS = DIAGNOSIS_STATUS(1i32);
pub const DS_REJECTED: DIAGNOSIS_STATUS = DIAGNOSIS_STATUS(2i32);
pub const DS_INDETERMINATE: DIAGNOSIS_STATUS = DIAGNOSIS_STATUS(3i32);
pub const DS_DEFERRED: DIAGNOSIS_STATUS = DIAGNOSIS_STATUS(4i32);
pub const DS_PASSTHROUGH: DIAGNOSIS_STATUS = DIAGNOSIS_STATUS(5i32);
impl ::core::marker::Copy for DIAGNOSIS_STATUS {}
impl ::core::clone::Clone for DIAGNOSIS_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIAGNOSIS_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DIAGNOSIS_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIAGNOSIS_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIAGNOSIS_STATUS").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DIAG_SOCKADDR {
    pub family: u16,
    pub data: [super::super::Foundation::CHAR; 126],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DIAG_SOCKADDR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DIAG_SOCKADDR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DIAG_SOCKADDR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIAG_SOCKADDR").field("family", &self.family).field("data", &self.data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows_core::Abi for DIAG_SOCKADDR {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIAG_SOCKADDR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIAG_SOCKADDR>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIAG_SOCKADDR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIAG_SOCKADDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DiagnosticsInfo {
    pub cost: i32,
    pub flags: u32,
}
impl ::core::marker::Copy for DiagnosticsInfo {}
impl ::core::clone::Clone for DiagnosticsInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DiagnosticsInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DiagnosticsInfo").field("cost", &self.cost).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for DiagnosticsInfo {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DiagnosticsInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DiagnosticsInfo>()) == 0 }
    }
}
impl ::core::cmp::Eq for DiagnosticsInfo {}
impl ::core::default::Default for DiagnosticsInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HELPER_ATTRIBUTE {
    pub pwszName: ::windows_core::PWSTR,
    pub r#type: ATTRIBUTE_TYPE,
    pub Anonymous: HELPER_ATTRIBUTE_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HELPER_ATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HELPER_ATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows_core::Abi for HELPER_ATTRIBUTE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HELPER_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HELPER_ATTRIBUTE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HELPER_ATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HELPER_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union HELPER_ATTRIBUTE_0 {
    pub Boolean: super::super::Foundation::BOOL,
    pub Char: u8,
    pub Byte: u8,
    pub Short: i16,
    pub Word: u16,
    pub Int: i32,
    pub DWord: u32,
    pub Int64: i64,
    pub UInt64: u64,
    pub PWStr: ::windows_core::PWSTR,
    pub Guid: ::windows_core::GUID,
    pub LifeTime: LIFE_TIME,
    pub Address: DIAG_SOCKADDR,
    pub OctetString: OCTET_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HELPER_ATTRIBUTE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HELPER_ATTRIBUTE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows_core::Abi for HELPER_ATTRIBUTE_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HELPER_ATTRIBUTE_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HELPER_ATTRIBUTE_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HELPER_ATTRIBUTE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HELPER_ATTRIBUTE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HYPOTHESIS {
    pub pwszClassName: ::windows_core::PWSTR,
    pub pwszDescription: ::windows_core::PWSTR,
    pub celt: u32,
    pub rgAttributes: *mut HELPER_ATTRIBUTE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HYPOTHESIS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HYPOTHESIS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HYPOTHESIS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HYPOTHESIS").field("pwszClassName", &self.pwszClassName).field("pwszDescription", &self.pwszDescription).field("celt", &self.celt).field("rgAttributes", &self.rgAttributes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows_core::Abi for HYPOTHESIS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HYPOTHESIS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HYPOTHESIS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HYPOTHESIS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HYPOTHESIS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct HelperAttributeInfo {
    pub pwszName: ::windows_core::PWSTR,
    pub r#type: ATTRIBUTE_TYPE,
}
impl ::core::marker::Copy for HelperAttributeInfo {}
impl ::core::clone::Clone for HelperAttributeInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HelperAttributeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HelperAttributeInfo").field("pwszName", &self.pwszName).field("type", &self.r#type).finish()
    }
}
unsafe impl ::windows_core::Abi for HelperAttributeInfo {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HelperAttributeInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HelperAttributeInfo>()) == 0 }
    }
}
impl ::core::cmp::Eq for HelperAttributeInfo {}
impl ::core::default::Default for HelperAttributeInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HypothesisResult {
    pub hypothesis: HYPOTHESIS,
    pub pathStatus: DIAGNOSIS_STATUS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HypothesisResult {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HypothesisResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HypothesisResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HypothesisResult").field("hypothesis", &self.hypothesis).field("pathStatus", &self.pathStatus).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows_core::Abi for HypothesisResult {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HypothesisResult {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HypothesisResult>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HypothesisResult {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HypothesisResult {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct INetDiagExtensibleHelper(::windows_core::IUnknown);
impl INetDiagExtensibleHelper {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ResolveAttributes(&self, rgkeyattributes: &[HELPER_ATTRIBUTE], pcelt: *mut u32, prgmatchvalues: *mut *mut HELPER_ATTRIBUTE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ResolveAttributes)(::windows_core::Interface::as_raw(self), rgkeyattributes.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(rgkeyattributes)), ::core::mem::transmute(pcelt), ::core::mem::transmute(prgmatchvalues)).ok()
    }
}
impl ::core::convert::From<INetDiagExtensibleHelper> for ::windows_core::IUnknown {
    fn from(value: INetDiagExtensibleHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetDiagExtensibleHelper> for ::windows_core::IUnknown {
    fn from(value: &INetDiagExtensibleHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetDiagExtensibleHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetDiagExtensibleHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetDiagExtensibleHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetDiagExtensibleHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetDiagExtensibleHelper {}
impl ::core::fmt::Debug for INetDiagExtensibleHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetDiagExtensibleHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for INetDiagExtensibleHelper {
    type Vtable = INetDiagExtensibleHelper_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc0b35748_ebf5_11d8_bbe9_505054503030);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetDiagExtensibleHelper_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ResolveAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgkeyattributes: *const HELPER_ATTRIBUTE, pcelt: *mut u32, prgmatchvalues: *mut *mut HELPER_ATTRIBUTE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ResolveAttributes: usize,
}
#[repr(transparent)]
pub struct INetDiagHelper(::windows_core::IUnknown);
impl INetDiagHelper {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize(&self, rgattributes: &[HELPER_ATTRIBUTE]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), rgattributes.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(rgattributes))).ok()
    }
    pub unsafe fn GetDiagnosticsInfo(&self) -> ::windows_core::Result<*mut DiagnosticsInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut DiagnosticsInfo>::zeroed();
        (::windows_core::Interface::vtable(self).GetDiagnosticsInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut DiagnosticsInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetKeyAttributes(&self, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetKeyAttributes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcelt), ::core::mem::transmute(pprgattributes)).ok()
    }
    pub unsafe fn LowHealth<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszinstancedescription: Param0, ppwszdescription: *mut ::windows_core::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LowHealth)(::windows_core::Interface::as_raw(self), pwszinstancedescription.into_param().abi(), ::core::mem::transmute(ppwszdescription), ::core::mem::transmute(pdeferredtime), ::core::mem::transmute(pstatus)).ok()
    }
    pub unsafe fn HighUtilization<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszinstancedescription: Param0, ppwszdescription: *mut ::windows_core::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).HighUtilization)(::windows_core::Interface::as_raw(self), pwszinstancedescription.into_param().abi(), ::core::mem::transmute(ppwszdescription), ::core::mem::transmute(pdeferredtime), ::core::mem::transmute(pstatus)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLowerHypotheses(&self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetLowerHypotheses)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcelt), ::core::mem::transmute(pprghypotheses)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDownStreamHypotheses(&self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDownStreamHypotheses)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcelt), ::core::mem::transmute(pprghypotheses)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHigherHypotheses(&self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetHigherHypotheses)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcelt), ::core::mem::transmute(pprghypotheses)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUpStreamHypotheses(&self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetUpStreamHypotheses)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcelt), ::core::mem::transmute(pprghypotheses)).ok()
    }
    pub unsafe fn Repair(&self, pinfo: *const RepairInfo, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Repair)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(pdeferredtime), ::core::mem::transmute(pstatus)).ok()
    }
    pub unsafe fn Validate(&self, problem: PROBLEM_TYPE, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Validate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(problem), ::core::mem::transmute(pdeferredtime), ::core::mem::transmute(pstatus)).ok()
    }
    pub unsafe fn GetRepairInfo(&self, problem: PROBLEM_TYPE, pcelt: *mut u32, ppinfo: *mut *mut RepairInfo) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRepairInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(problem), ::core::mem::transmute(pcelt), ::core::mem::transmute(ppinfo)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLifeTime(&self) -> ::windows_core::Result<LIFE_TIME> {
        let mut result__ = ::core::mem::MaybeUninit::<LIFE_TIME>::zeroed();
        (::windows_core::Interface::vtable(self).GetLifeTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<LIFE_TIME>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLifeTime<'a, Param0: ::windows_core::IntoParam<'a, LIFE_TIME>>(&self, lifetime: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLifeTime)(::windows_core::Interface::as_raw(self), lifetime.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCacheTime(&self) -> ::windows_core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::FILETIME>::zeroed();
        (::windows_core::Interface::vtable(self).GetCacheTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributes(&self, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAttributes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcelt), ::core::mem::transmute(pprgattributes)).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Cleanup(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cleanup)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<INetDiagHelper> for ::windows_core::IUnknown {
    fn from(value: INetDiagHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetDiagHelper> for ::windows_core::IUnknown {
    fn from(value: &INetDiagHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetDiagHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetDiagHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetDiagHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetDiagHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetDiagHelper {}
impl ::core::fmt::Debug for INetDiagHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetDiagHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for INetDiagHelper {
    type Vtable = INetDiagHelper_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc0b35746_ebf5_11d8_bbe9_505054503030);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetDiagHelper_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgattributes: *const HELPER_ATTRIBUTE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub GetDiagnosticsInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppinfo: *mut *mut DiagnosticsInfo) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetKeyAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetKeyAttributes: usize,
    pub LowHealth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszinstancedescription: ::windows_core::PCWSTR, ppwszdescription: *mut ::windows_core::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> ::windows_core::HRESULT,
    pub HighUtilization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszinstancedescription: ::windows_core::PCWSTR, ppwszdescription: *mut ::windows_core::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLowerHypotheses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLowerHypotheses: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDownStreamHypotheses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDownStreamHypotheses: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHigherHypotheses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHigherHypotheses: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetUpStreamHypotheses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetUpStreamHypotheses: usize,
    pub Repair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const RepairInfo, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> ::windows_core::HRESULT,
    pub Validate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, problem: PROBLEM_TYPE, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> ::windows_core::HRESULT,
    pub GetRepairInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, problem: PROBLEM_TYPE, pcelt: *mut u32, ppinfo: *mut *mut RepairInfo) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLifeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plifetime: *mut LIFE_TIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLifeTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLifeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lifetime: LIFE_TIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLifeTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCacheTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcachetime: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCacheTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAttributes: usize,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Cleanup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct INetDiagHelperEx(::windows_core::IUnknown);
impl INetDiagHelperEx {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReconfirmLowHealth(&self, presults: &[HypothesisResult], ppwszupdateddescription: *mut ::windows_core::PWSTR, pupdatedstatus: *mut DIAGNOSIS_STATUS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReconfirmLowHealth)(::windows_core::Interface::as_raw(self), presults.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(presults)), ::core::mem::transmute(ppwszupdateddescription), ::core::mem::transmute(pupdatedstatus)).ok()
    }
    pub unsafe fn SetUtilities<'a, Param0: ::windows_core::IntoParam<'a, INetDiagHelperUtilFactory>>(&self, putilities: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUtilities)(::windows_core::Interface::as_raw(self), putilities.into_param().abi()).ok()
    }
    pub unsafe fn ReproduceFailure(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReproduceFailure)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<INetDiagHelperEx> for ::windows_core::IUnknown {
    fn from(value: INetDiagHelperEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetDiagHelperEx> for ::windows_core::IUnknown {
    fn from(value: &INetDiagHelperEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetDiagHelperEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetDiagHelperEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetDiagHelperEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetDiagHelperEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetDiagHelperEx {}
impl ::core::fmt::Debug for INetDiagHelperEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetDiagHelperEx").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for INetDiagHelperEx {
    type Vtable = INetDiagHelperEx_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x972dab4d_e4e3_4fc6_ae54_5f65ccde4a15);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetDiagHelperEx_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ReconfirmLowHealth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, presults: *const HypothesisResult, ppwszupdateddescription: *mut ::windows_core::PWSTR, pupdatedstatus: *mut DIAGNOSIS_STATUS) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReconfirmLowHealth: usize,
    pub SetUtilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, putilities: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReproduceFailure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct INetDiagHelperInfo(::windows_core::IUnknown);
impl INetDiagHelperInfo {
    pub unsafe fn GetAttributeInfo(&self, pcelt: *mut u32, pprgattributeinfos: *mut *mut HelperAttributeInfo) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAttributeInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcelt), ::core::mem::transmute(pprgattributeinfos)).ok()
    }
}
impl ::core::convert::From<INetDiagHelperInfo> for ::windows_core::IUnknown {
    fn from(value: INetDiagHelperInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetDiagHelperInfo> for ::windows_core::IUnknown {
    fn from(value: &INetDiagHelperInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetDiagHelperInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetDiagHelperInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetDiagHelperInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetDiagHelperInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetDiagHelperInfo {}
impl ::core::fmt::Debug for INetDiagHelperInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetDiagHelperInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for INetDiagHelperInfo {
    type Vtable = INetDiagHelperInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc0b35747_ebf5_11d8_bbe9_505054503030);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetDiagHelperInfo_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetAttributeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprgattributeinfos: *mut *mut HelperAttributeInfo) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct INetDiagHelperUtilFactory(::windows_core::IUnknown);
impl INetDiagHelperUtilFactory {
    pub unsafe fn CreateUtilityInstance<T: ::windows_core::Interface>(&self) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).CreateUtilityInstance)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<INetDiagHelperUtilFactory> for ::windows_core::IUnknown {
    fn from(value: INetDiagHelperUtilFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetDiagHelperUtilFactory> for ::windows_core::IUnknown {
    fn from(value: &INetDiagHelperUtilFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INetDiagHelperUtilFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INetDiagHelperUtilFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetDiagHelperUtilFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetDiagHelperUtilFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetDiagHelperUtilFactory {}
impl ::core::fmt::Debug for INetDiagHelperUtilFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetDiagHelperUtilFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for INetDiagHelperUtilFactory {
    type Vtable = INetDiagHelperUtilFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x104613fb_bc57_4178_95ba_88809698354a);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetDiagHelperUtilFactory_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CreateUtilityInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct LIFE_TIME {
    pub startTime: super::super::Foundation::FILETIME,
    pub endTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LIFE_TIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LIFE_TIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LIFE_TIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LIFE_TIME").field("startTime", &self.startTime).field("endTime", &self.endTime).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows_core::Abi for LIFE_TIME {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LIFE_TIME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LIFE_TIME>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LIFE_TIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LIFE_TIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const NDF_ADD_CAPTURE_TRACE: u32 = 1u32;
pub const NDF_APPLY_INCLUSION_LIST_FILTER: u32 = 2u32;
pub const NDF_ERROR_START: u32 = 63744u32;
pub const NDF_E_BAD_PARAM: ::windows_core::HRESULT = ::windows_core::HRESULT(-2146895611i32);
pub const NDF_E_CANCELLED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2146895614i32);
pub const NDF_E_DISABLED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2146895612i32);
pub const NDF_E_LENGTH_EXCEEDED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2146895616i32);
pub const NDF_E_NOHELPERCLASS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2146895615i32);
pub const NDF_E_PROBLEM_PRESENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2146895608i32);
pub const NDF_E_UNKNOWN: ::windows_core::HRESULT = ::windows_core::HRESULT(-2146895609i32);
pub const NDF_E_VALIDATION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2146895610i32);
pub const NDF_INBOUND_FLAG_EDGETRAVERSAL: u32 = 1u32;
pub const NDF_INBOUND_FLAG_HEALTHCHECK: u32 = 2u32;
#[inline]
pub unsafe fn NdfCancelIncident(handle: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfCancelIncident(handle: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        NdfCancelIncident(::core::mem::transmute(handle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn NdfCloseIncident(handle: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfCloseIncident(handle: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        NdfCloseIncident(::core::mem::transmute(handle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn NdfCreateConnectivityIncident(handle: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfCreateConnectivityIncident(handle: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        NdfCreateConnectivityIncident(::core::mem::transmute(handle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn NdfCreateDNSIncident<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hostname: Param0, querytype: u16, handle: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfCreateDNSIncident(hostname: ::windows_core::PCWSTR, querytype: u16, handle: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        NdfCreateDNSIncident(hostname.into_param().abi(), ::core::mem::transmute(querytype), ::core::mem::transmute(handle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn NdfCreateGroupingIncident<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param5: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(cloudname: Param0, groupname: Param1, identity: Param2, invitation: Param3, addresses: *const super::super::Networking::WinSock::SOCKET_ADDRESS_LIST, appid: Param5, handle: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfCreateGroupingIncident(cloudname: ::windows_core::PCWSTR, groupname: ::windows_core::PCWSTR, identity: ::windows_core::PCWSTR, invitation: ::windows_core::PCWSTR, addresses: *const super::super::Networking::WinSock::SOCKET_ADDRESS_LIST, appid: ::windows_core::PCWSTR, handle: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        NdfCreateGroupingIncident(cloudname.into_param().abi(), groupname.into_param().abi(), identity.into_param().abi(), invitation.into_param().abi(), ::core::mem::transmute(addresses), appid.into_param().abi(), ::core::mem::transmute(handle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NdfCreateIncident<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(helperclassname: Param0, attributes: &[HELPER_ATTRIBUTE], handle: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfCreateIncident(helperclassname: ::windows_core::PCWSTR, celt: u32, attributes: *const HELPER_ATTRIBUTE, handle: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        NdfCreateIncident(helperclassname.into_param().abi(), attributes.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(attributes)), ::core::mem::transmute(handle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn NdfCreateNetConnectionIncident<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(handle: *mut *mut ::core::ffi::c_void, id: Param1) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfCreateNetConnectionIncident(handle: *mut *mut ::core::ffi::c_void, id: ::windows_core::GUID) -> ::windows_core::HRESULT;
        }
        NdfCreateNetConnectionIncident(::core::mem::transmute(handle), id.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NdfCreatePnrpIncident<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(cloudname: Param0, peername: Param1, diagnosepublish: Param2, appid: Param3, handle: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfCreatePnrpIncident(cloudname: ::windows_core::PCWSTR, peername: ::windows_core::PCWSTR, diagnosepublish: super::super::Foundation::BOOL, appid: ::windows_core::PCWSTR, handle: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        NdfCreatePnrpIncident(cloudname.into_param().abi(), peername.into_param().abi(), diagnosepublish.into_param().abi(), appid.into_param().abi(), ::core::mem::transmute(handle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn NdfCreateSharingIncident<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(uncpath: Param0, handle: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfCreateSharingIncident(uncpath: ::windows_core::PCWSTR, handle: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        NdfCreateSharingIncident(uncpath.into_param().abi(), ::core::mem::transmute(handle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn NdfCreateWebIncident<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(url: Param0, handle: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfCreateWebIncident(url: ::windows_core::PCWSTR, handle: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        NdfCreateWebIncident(url.into_param().abi(), ::core::mem::transmute(handle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NdfCreateWebIncidentEx<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(url: Param0, usewinhttp: Param1, modulename: Param2, handle: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfCreateWebIncidentEx(url: ::windows_core::PCWSTR, usewinhttp: super::super::Foundation::BOOL, modulename: ::windows_core::PCWSTR, handle: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        NdfCreateWebIncidentEx(url.into_param().abi(), usewinhttp.into_param().abi(), modulename.into_param().abi(), ::core::mem::transmute(handle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NdfCreateWinSockIncident<'a, Param0: ::windows_core::IntoParam<'a, super::super::Networking::WinSock::SOCKET>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(sock: Param0, host: Param1, port: u16, appid: Param3, userid: *const super::super::Security::SID, handle: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfCreateWinSockIncident(sock: super::super::Networking::WinSock::SOCKET, host: ::windows_core::PCWSTR, port: u16, appid: ::windows_core::PCWSTR, userid: *const super::super::Security::SID, handle: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        NdfCreateWinSockIncident(sock.into_param().abi(), host.into_param().abi(), ::core::mem::transmute(port), appid.into_param().abi(), ::core::mem::transmute(userid), ::core::mem::transmute(handle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn NdfDiagnoseIncident(handle: *const ::core::ffi::c_void, rootcausecount: *mut u32, rootcauses: *mut *mut RootCauseInfo, dwwait: u32, dwflags: u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfDiagnoseIncident(handle: *const ::core::ffi::c_void, rootcausecount: *mut u32, rootcauses: *mut *mut RootCauseInfo, dwwait: u32, dwflags: u32) -> ::windows_core::HRESULT;
        }
        NdfDiagnoseIncident(::core::mem::transmute(handle), ::core::mem::transmute(rootcausecount), ::core::mem::transmute(rootcauses), ::core::mem::transmute(dwwait), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NdfExecuteDiagnosis<'a, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::HWND>>(handle: *const ::core::ffi::c_void, hwnd: Param1) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfExecuteDiagnosis(handle: *const ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows_core::HRESULT;
        }
        NdfExecuteDiagnosis(::core::mem::transmute(handle), hwnd.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn NdfGetTraceFile(handle: *const ::core::ffi::c_void) -> ::windows_core::Result<::windows_core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfGetTraceFile(handle: *const ::core::ffi::c_void, tracefilelocation: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        NdfGetTraceFile(::core::mem::transmute(handle), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn NdfRepairIncident(handle: *const ::core::ffi::c_void, repairex: *const RepairInfoEx, dwwait: u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfRepairIncident(handle: *const ::core::ffi::c_void, repairex: *const RepairInfoEx, dwwait: u32) -> ::windows_core::HRESULT;
        }
        NdfRepairIncident(::core::mem::transmute(handle), ::core::mem::transmute(repairex), ::core::mem::transmute(dwwait)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct OCTET_STRING {
    pub dwLength: u32,
    pub lpValue: *mut u8,
}
impl ::core::marker::Copy for OCTET_STRING {}
impl ::core::clone::Clone for OCTET_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OCTET_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OCTET_STRING").field("dwLength", &self.dwLength).field("lpValue", &self.lpValue).finish()
    }
}
unsafe impl ::windows_core::Abi for OCTET_STRING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for OCTET_STRING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OCTET_STRING>()) == 0 }
    }
}
impl ::core::cmp::Eq for OCTET_STRING {}
impl ::core::default::Default for OCTET_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROBLEM_TYPE(pub i32);
pub const PT_INVALID: PROBLEM_TYPE = PROBLEM_TYPE(0i32);
pub const PT_LOW_HEALTH: PROBLEM_TYPE = PROBLEM_TYPE(1i32);
pub const PT_LOWER_HEALTH: PROBLEM_TYPE = PROBLEM_TYPE(2i32);
pub const PT_DOWN_STREAM_HEALTH: PROBLEM_TYPE = PROBLEM_TYPE(4i32);
pub const PT_HIGH_UTILIZATION: PROBLEM_TYPE = PROBLEM_TYPE(8i32);
pub const PT_HIGHER_UTILIZATION: PROBLEM_TYPE = PROBLEM_TYPE(16i32);
pub const PT_UP_STREAM_UTILIZATION: PROBLEM_TYPE = PROBLEM_TYPE(32i32);
impl ::core::marker::Copy for PROBLEM_TYPE {}
impl ::core::clone::Clone for PROBLEM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROBLEM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PROBLEM_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROBLEM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROBLEM_TYPE").field(&self.0).finish()
    }
}
pub const RCF_ISCONFIRMED: u32 = 2u32;
pub const RCF_ISLEAF: u32 = 1u32;
pub const RCF_ISTHIRDPARTY: u32 = 4u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct REPAIR_RISK(pub i32);
pub const RR_NOROLLBACK: REPAIR_RISK = REPAIR_RISK(0i32);
pub const RR_ROLLBACK: REPAIR_RISK = REPAIR_RISK(1i32);
pub const RR_NORISK: REPAIR_RISK = REPAIR_RISK(2i32);
impl ::core::marker::Copy for REPAIR_RISK {}
impl ::core::clone::Clone for REPAIR_RISK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REPAIR_RISK {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for REPAIR_RISK {
    type Abi = Self;
}
impl ::core::fmt::Debug for REPAIR_RISK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REPAIR_RISK").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct REPAIR_SCOPE(pub i32);
pub const RS_SYSTEM: REPAIR_SCOPE = REPAIR_SCOPE(0i32);
pub const RS_USER: REPAIR_SCOPE = REPAIR_SCOPE(1i32);
pub const RS_APPLICATION: REPAIR_SCOPE = REPAIR_SCOPE(2i32);
pub const RS_PROCESS: REPAIR_SCOPE = REPAIR_SCOPE(3i32);
impl ::core::marker::Copy for REPAIR_SCOPE {}
impl ::core::clone::Clone for REPAIR_SCOPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REPAIR_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for REPAIR_SCOPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for REPAIR_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REPAIR_SCOPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct REPAIR_STATUS(pub i32);
pub const RS_NOT_IMPLEMENTED: REPAIR_STATUS = REPAIR_STATUS(0i32);
pub const RS_REPAIRED: REPAIR_STATUS = REPAIR_STATUS(1i32);
pub const RS_UNREPAIRED: REPAIR_STATUS = REPAIR_STATUS(2i32);
pub const RS_DEFERRED: REPAIR_STATUS = REPAIR_STATUS(3i32);
pub const RS_USER_ACTION: REPAIR_STATUS = REPAIR_STATUS(4i32);
impl ::core::marker::Copy for REPAIR_STATUS {}
impl ::core::clone::Clone for REPAIR_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REPAIR_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for REPAIR_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for REPAIR_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REPAIR_STATUS").field(&self.0).finish()
    }
}
pub const RF_CONTACT_ADMIN: u32 = 131072u32;
pub const RF_INFORMATION_ONLY: u32 = 33554432u32;
pub const RF_REPRO: u32 = 2097152u32;
pub const RF_RESERVED: u32 = 1073741824u32;
pub const RF_RESERVED_CA: u32 = 2147483648u32;
pub const RF_RESERVED_LNI: u32 = 65536u32;
pub const RF_SHOW_EVENTS: u32 = 8388608u32;
pub const RF_UI_ONLY: u32 = 16777216u32;
pub const RF_USER_ACTION: u32 = 268435456u32;
pub const RF_USER_CONFIRMATION: u32 = 134217728u32;
pub const RF_VALIDATE_HELPTOPIC: u32 = 4194304u32;
pub const RF_WORKAROUND: u32 = 536870912u32;
#[repr(C)]
pub struct RepairInfo {
    pub guid: ::windows_core::GUID,
    pub pwszClassName: ::windows_core::PWSTR,
    pub pwszDescription: ::windows_core::PWSTR,
    pub sidType: u32,
    pub cost: i32,
    pub flags: u32,
    pub scope: REPAIR_SCOPE,
    pub risk: REPAIR_RISK,
    pub UiInfo: UiInfo,
    pub rootCauseIndex: i32,
}
impl ::core::marker::Copy for RepairInfo {}
impl ::core::clone::Clone for RepairInfo {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for RepairInfo {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RepairInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RepairInfo>()) == 0 }
    }
}
impl ::core::cmp::Eq for RepairInfo {}
impl ::core::default::Default for RepairInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct RepairInfoEx {
    pub repair: RepairInfo,
    pub repairRank: u16,
}
impl ::core::marker::Copy for RepairInfoEx {}
impl ::core::clone::Clone for RepairInfoEx {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for RepairInfoEx {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RepairInfoEx {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RepairInfoEx>()) == 0 }
    }
}
impl ::core::cmp::Eq for RepairInfoEx {}
impl ::core::default::Default for RepairInfoEx {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct RootCauseInfo {
    pub pwszDescription: ::windows_core::PWSTR,
    pub rootCauseID: ::windows_core::GUID,
    pub rootCauseFlags: u32,
    pub networkInterfaceID: ::windows_core::GUID,
    pub pRepairs: *mut RepairInfoEx,
    pub repairCount: u16,
}
impl ::core::marker::Copy for RootCauseInfo {}
impl ::core::clone::Clone for RootCauseInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RootCauseInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RootCauseInfo").field("pwszDescription", &self.pwszDescription).field("rootCauseID", &self.rootCauseID).field("rootCauseFlags", &self.rootCauseFlags).field("networkInterfaceID", &self.networkInterfaceID).field("pRepairs", &self.pRepairs).field("repairCount", &self.repairCount).finish()
    }
}
unsafe impl ::windows_core::Abi for RootCauseInfo {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RootCauseInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RootCauseInfo>()) == 0 }
    }
}
impl ::core::cmp::Eq for RootCauseInfo {}
impl ::core::default::Default for RootCauseInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ShellCommandInfo {
    pub pwszOperation: ::windows_core::PWSTR,
    pub pwszFile: ::windows_core::PWSTR,
    pub pwszParameters: ::windows_core::PWSTR,
    pub pwszDirectory: ::windows_core::PWSTR,
    pub nShowCmd: u32,
}
impl ::core::marker::Copy for ShellCommandInfo {}
impl ::core::clone::Clone for ShellCommandInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ShellCommandInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ShellCommandInfo").field("pwszOperation", &self.pwszOperation).field("pwszFile", &self.pwszFile).field("pwszParameters", &self.pwszParameters).field("pwszDirectory", &self.pwszDirectory).field("nShowCmd", &self.nShowCmd).finish()
    }
}
unsafe impl ::windows_core::Abi for ShellCommandInfo {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ShellCommandInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ShellCommandInfo>()) == 0 }
    }
}
impl ::core::cmp::Eq for ShellCommandInfo {}
impl ::core::default::Default for ShellCommandInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UI_INFO_TYPE(pub i32);
pub const UIT_INVALID: UI_INFO_TYPE = UI_INFO_TYPE(0i32);
pub const UIT_NONE: UI_INFO_TYPE = UI_INFO_TYPE(1i32);
pub const UIT_SHELL_COMMAND: UI_INFO_TYPE = UI_INFO_TYPE(2i32);
pub const UIT_HELP_PANE: UI_INFO_TYPE = UI_INFO_TYPE(3i32);
pub const UIT_DUI: UI_INFO_TYPE = UI_INFO_TYPE(4i32);
impl ::core::marker::Copy for UI_INFO_TYPE {}
impl ::core::clone::Clone for UI_INFO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UI_INFO_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_INFO_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct UiInfo {
    pub r#type: UI_INFO_TYPE,
    pub Anonymous: UiInfo_0,
}
impl ::core::marker::Copy for UiInfo {}
impl ::core::clone::Clone for UiInfo {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for UiInfo {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for UiInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<UiInfo>()) == 0 }
    }
}
impl ::core::cmp::Eq for UiInfo {}
impl ::core::default::Default for UiInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union UiInfo_0 {
    pub pwzNull: ::windows_core::PWSTR,
    pub ShellInfo: ShellCommandInfo,
    pub pwzHelpUrl: ::windows_core::PWSTR,
    pub pwzDui: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for UiInfo_0 {}
impl ::core::clone::Clone for UiInfo_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for UiInfo_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for UiInfo_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<UiInfo_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for UiInfo_0 {}
impl ::core::default::Default for UiInfo_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
