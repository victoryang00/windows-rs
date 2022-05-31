pub const CLSID_DxcAssembler: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd728db68_f903_4f80_94cd_dccf76ec7151);
pub const CLSID_DxcCompiler: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x73e22d93_e6ce_47f3_b5bf_f0664f39c1b0);
pub const CLSID_DxcCompilerArgs: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3e56ae82_224d_470f_a1a1_fe3016ee9f9d);
pub const CLSID_DxcContainerBuilder: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x94134294_411f_4574_b4d0_8741e25240d2);
pub const CLSID_DxcContainerReflection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9f54489_55b8_400c_ba3a_1675e4728b91);
pub const CLSID_DxcDiaDataSource: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd1f6b73_2ab0_484d_8edc_ebe7a43ca09f);
pub const CLSID_DxcLibrary: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6245d6af_66e0_48fd_80b4_4d271796748c);
pub const CLSID_DxcLinker: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef6a8087_b0ea_4d56_9e45_d07e1a8b7806);
pub const CLSID_DxcOptimizer: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae2cd79f_cc22_453f_9b6b_b124e7a5204c);
pub const CLSID_DxcPdbUtils: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x54621dfb_f2ce_457e_ae8c_ec355faeec7c);
pub const CLSID_DxcValidator: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8ca3e215_f728_4cf3_8cdd_88af917587a1);
pub const DXC_ARG_ALL_RESOURCES_BOUND: &str = "-all_resources_bound";
pub const DXC_ARG_AVOID_FLOW_CONTROL: &str = "-Gfa";
pub const DXC_ARG_DEBUG: &str = "-Zi";
pub const DXC_ARG_DEBUG_NAME_FOR_BINARY: &str = "-Zsb";
pub const DXC_ARG_DEBUG_NAME_FOR_SOURCE: &str = "-Zss";
pub const DXC_ARG_ENABLE_BACKWARDS_COMPATIBILITY: &str = "-Gec";
pub const DXC_ARG_ENABLE_STRICTNESS: &str = "-Ges";
pub const DXC_ARG_IEEE_STRICTNESS: &str = "-Gis";
pub const DXC_ARG_OPTIMIZATION_LEVEL0: &str = "-O0";
pub const DXC_ARG_OPTIMIZATION_LEVEL1: &str = "-O1";
pub const DXC_ARG_OPTIMIZATION_LEVEL2: &str = "-O2";
pub const DXC_ARG_OPTIMIZATION_LEVEL3: &str = "-O3";
pub const DXC_ARG_PACK_MATRIX_COLUMN_MAJOR: &str = "-Zpc";
pub const DXC_ARG_PACK_MATRIX_ROW_MAJOR: &str = "-Zpr";
pub const DXC_ARG_PREFER_FLOW_CONTROL: &str = "-Gfp";
pub const DXC_ARG_RESOURCES_MAY_ALIAS: &str = "-res_may_alias";
pub const DXC_ARG_SKIP_OPTIMIZATIONS: &str = "-Od";
pub const DXC_ARG_SKIP_VALIDATION: &str = "-Vd";
pub const DXC_ARG_WARNINGS_ARE_ERRORS: &str = "-WX";
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXC_CP(pub u32);
pub const DXC_CP_ACP: DXC_CP = DXC_CP(0u32);
pub const DXC_CP_UTF16: DXC_CP = DXC_CP(1200u32);
pub const DXC_CP_UTF8: DXC_CP = DXC_CP(65001u32);
impl ::core::marker::Copy for DXC_CP {}
impl ::core::clone::Clone for DXC_CP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXC_CP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DXC_CP {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXC_CP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXC_CP").field(&self.0).finish()
    }
}
pub const DXC_EXTRA_OUTPUT_NAME_STDERR: &str = "*stderr*";
pub const DXC_EXTRA_OUTPUT_NAME_STDOUT: &str = "*stdout*";
pub const DXC_HASHFLAG_INCLUDES_SOURCE: u32 = 1u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXC_OUT_KIND(pub i32);
pub const DXC_OUT_NONE: DXC_OUT_KIND = DXC_OUT_KIND(0i32);
pub const DXC_OUT_OBJECT: DXC_OUT_KIND = DXC_OUT_KIND(1i32);
pub const DXC_OUT_ERRORS: DXC_OUT_KIND = DXC_OUT_KIND(2i32);
pub const DXC_OUT_PDB: DXC_OUT_KIND = DXC_OUT_KIND(3i32);
pub const DXC_OUT_SHADER_HASH: DXC_OUT_KIND = DXC_OUT_KIND(4i32);
pub const DXC_OUT_DISASSEMBLY: DXC_OUT_KIND = DXC_OUT_KIND(5i32);
pub const DXC_OUT_HLSL: DXC_OUT_KIND = DXC_OUT_KIND(6i32);
pub const DXC_OUT_TEXT: DXC_OUT_KIND = DXC_OUT_KIND(7i32);
pub const DXC_OUT_REFLECTION: DXC_OUT_KIND = DXC_OUT_KIND(8i32);
pub const DXC_OUT_ROOT_SIGNATURE: DXC_OUT_KIND = DXC_OUT_KIND(9i32);
pub const DXC_OUT_EXTRA_OUTPUTS: DXC_OUT_KIND = DXC_OUT_KIND(10i32);
pub const DXC_OUT_FORCE_DWORD: DXC_OUT_KIND = DXC_OUT_KIND(-1i32);
impl ::core::marker::Copy for DXC_OUT_KIND {}
impl ::core::clone::Clone for DXC_OUT_KIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXC_OUT_KIND {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DXC_OUT_KIND {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXC_OUT_KIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXC_OUT_KIND").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct DxcArgPair {
    pub pName: ::windows_core::PCWSTR,
    pub pValue: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for DxcArgPair {}
impl ::core::clone::Clone for DxcArgPair {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DxcArgPair {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DxcArgPair").field("pName", &self.pName).field("pValue", &self.pValue).finish()
    }
}
unsafe impl ::windows_core::Abi for DxcArgPair {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DxcArgPair {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DxcArgPair>()) == 0 }
    }
}
impl ::core::cmp::Eq for DxcArgPair {}
impl ::core::default::Default for DxcArgPair {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DxcBuffer {
    pub Ptr: *const ::core::ffi::c_void,
    pub Size: usize,
    pub Encoding: u32,
}
impl ::core::marker::Copy for DxcBuffer {}
impl ::core::clone::Clone for DxcBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DxcBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DxcBuffer").field("Ptr", &self.Ptr).field("Size", &self.Size).field("Encoding", &self.Encoding).finish()
    }
}
unsafe impl ::windows_core::Abi for DxcBuffer {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DxcBuffer {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DxcBuffer>()) == 0 }
    }
}
impl ::core::cmp::Eq for DxcBuffer {}
impl ::core::default::Default for DxcBuffer {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[inline]
pub unsafe fn DxcCreateInstance<T: ::windows_core::Interface>(rclsid: *const ::windows_core::GUID) -> ::windows_core::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DxcCreateInstance(rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::option::Option::None;
        DxcCreateInstance(::core::mem::transmute(rclsid), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn DxcCreateInstance2<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::IMalloc>, T: ::windows_core::Interface>(pmalloc: Param0, rclsid: *const ::windows_core::GUID) -> ::windows_core::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DxcCreateInstance2(pmalloc: ::windows_core::RawPtr, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::option::Option::None;
        DxcCreateInstance2(pmalloc.into_param().abi(), ::core::mem::transmute(rclsid), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com")]
pub type DxcCreateInstance2Proc = ::core::option::Option<unsafe extern "system" fn(pmalloc: ::core::option::Option<::win32_system::Com::IMalloc>, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT>;
pub type DxcCreateInstanceProc = ::core::option::Option<unsafe extern "system" fn(rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT>;
#[repr(C)]
pub struct DxcDefine {
    pub Name: ::windows_core::PCWSTR,
    pub Value: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for DxcDefine {}
impl ::core::clone::Clone for DxcDefine {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DxcDefine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DxcDefine").field("Name", &self.Name).field("Value", &self.Value).finish()
    }
}
unsafe impl ::windows_core::Abi for DxcDefine {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DxcDefine {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DxcDefine>()) == 0 }
    }
}
impl ::core::cmp::Eq for DxcDefine {}
impl ::core::default::Default for DxcDefine {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DxcShaderHash {
    pub Flags: u32,
    pub HashDigest: [u8; 16],
}
impl ::core::marker::Copy for DxcShaderHash {}
impl ::core::clone::Clone for DxcShaderHash {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DxcShaderHash {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DxcShaderHash").field("Flags", &self.Flags).field("HashDigest", &self.HashDigest).finish()
    }
}
unsafe impl ::windows_core::Abi for DxcShaderHash {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DxcShaderHash {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DxcShaderHash>()) == 0 }
    }
}
impl ::core::cmp::Eq for DxcShaderHash {}
impl ::core::default::Default for DxcShaderHash {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DxcValidatorFlags_Default: u32 = 0u32;
pub const DxcValidatorFlags_InPlaceEdit: u32 = 1u32;
pub const DxcValidatorFlags_ModuleOnly: u32 = 4u32;
pub const DxcValidatorFlags_RootSignatureOnly: u32 = 2u32;
pub const DxcValidatorFlags_ValidMask: u32 = 7u32;
pub const DxcVersionInfoFlags_Debug: u32 = 1u32;
pub const DxcVersionInfoFlags_Internal: u32 = 2u32;
pub const DxcVersionInfoFlags_None: u32 = 0u32;
#[repr(transparent)]
pub struct IDxcAssembler(::windows_core::IUnknown);
impl IDxcAssembler {
    pub unsafe fn AssembleToContainer<'a, Param0: ::windows_core::IntoParam<'a, IDxcBlob>>(&self, pshader: Param0) -> ::windows_core::Result<IDxcOperationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AssembleToContainer)(::windows_core::Interface::as_raw(self), pshader.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcOperationResult>(result__)
    }
}
impl ::core::convert::From<IDxcAssembler> for ::windows_core::IUnknown {
    fn from(value: IDxcAssembler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcAssembler> for ::windows_core::IUnknown {
    fn from(value: &IDxcAssembler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDxcAssembler {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDxcAssembler {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcAssembler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcAssembler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcAssembler {}
impl ::core::fmt::Debug for IDxcAssembler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcAssembler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDxcAssembler {
    type Vtable = IDxcAssembler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x091f7a26_1c1f_4948_904b_e6e3a8a771d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcAssembler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AssembleToContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshader: ::windows_core::RawPtr, ppresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDxcBlob(::windows_core::IUnknown);
impl IDxcBlob {
    pub unsafe fn GetBufferPointer(&self) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).GetBufferPointer)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn GetBufferSize(&self) -> usize {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).GetBufferSize)(::windows_core::Interface::as_raw(self)))
    }
}
impl ::core::convert::From<IDxcBlob> for ::windows_core::IUnknown {
    fn from(value: IDxcBlob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcBlob> for ::windows_core::IUnknown {
    fn from(value: &IDxcBlob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDxcBlob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDxcBlob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcBlob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcBlob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcBlob {}
impl ::core::fmt::Debug for IDxcBlob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcBlob").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDxcBlob {
    type Vtable = IDxcBlob_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8ba5fb08_5195_40e2_ac58_0d989c3a0102);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlob_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetBufferPointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
    pub GetBufferSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> usize,
}
#[repr(transparent)]
pub struct IDxcBlobEncoding(::windows_core::IUnknown);
impl IDxcBlobEncoding {
    pub unsafe fn GetBufferPointer(&self) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).base__.GetBufferPointer)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn GetBufferSize(&self) -> usize {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).base__.GetBufferSize)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn GetEncoding(&self, pknown: *mut ::win32_foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetEncoding)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pknown), ::core::mem::transmute(pcodepage)).ok()
    }
}
impl ::core::convert::From<IDxcBlobEncoding> for ::windows_core::IUnknown {
    fn from(value: IDxcBlobEncoding) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcBlobEncoding> for ::windows_core::IUnknown {
    fn from(value: &IDxcBlobEncoding) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDxcBlobEncoding {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDxcBlobEncoding {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDxcBlobEncoding> for IDxcBlob {
    fn from(value: IDxcBlobEncoding) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcBlobEncoding> for IDxcBlob {
    fn from(value: &IDxcBlobEncoding) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDxcBlob> for IDxcBlobEncoding {
    fn into_param(self) -> ::windows_core::Param<'a, IDxcBlob> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDxcBlob> for &'a IDxcBlobEncoding {
    fn into_param(self) -> ::windows_core::Param<'a, IDxcBlob> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcBlobEncoding {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcBlobEncoding {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcBlobEncoding {}
impl ::core::fmt::Debug for IDxcBlobEncoding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcBlobEncoding").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDxcBlobEncoding {
    type Vtable = IDxcBlobEncoding_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7241d424_2646_4191_97c0_98e96e42fc68);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlobEncoding_Vtbl {
    pub base__: IDxcBlob_Vtbl,
    pub GetEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pknown: *mut ::win32_foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDxcBlobUtf16(::windows_core::IUnknown);
impl IDxcBlobUtf16 {
    pub unsafe fn GetBufferPointer(&self) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).base__.base__.GetBufferPointer)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn GetBufferSize(&self) -> usize {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).base__.base__.GetBufferSize)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn GetEncoding(&self, pknown: *mut ::win32_foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetEncoding)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pknown), ::core::mem::transmute(pcodepage)).ok()
    }
    pub unsafe fn GetStringPointer(&self) -> ::windows_core::PWSTR {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).GetStringPointer)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn GetStringLength(&self) -> usize {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).GetStringLength)(::windows_core::Interface::as_raw(self)))
    }
}
impl ::core::convert::From<IDxcBlobUtf16> for ::windows_core::IUnknown {
    fn from(value: IDxcBlobUtf16) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcBlobUtf16> for ::windows_core::IUnknown {
    fn from(value: &IDxcBlobUtf16) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDxcBlobUtf16 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDxcBlobUtf16 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDxcBlobUtf16> for IDxcBlob {
    fn from(value: IDxcBlobUtf16) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcBlobUtf16> for IDxcBlob {
    fn from(value: &IDxcBlobUtf16) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDxcBlob> for IDxcBlobUtf16 {
    fn into_param(self) -> ::windows_core::Param<'a, IDxcBlob> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDxcBlob> for &'a IDxcBlobUtf16 {
    fn into_param(self) -> ::windows_core::Param<'a, IDxcBlob> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDxcBlobUtf16> for IDxcBlobEncoding {
    fn from(value: IDxcBlobUtf16) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcBlobUtf16> for IDxcBlobEncoding {
    fn from(value: &IDxcBlobUtf16) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDxcBlobEncoding> for IDxcBlobUtf16 {
    fn into_param(self) -> ::windows_core::Param<'a, IDxcBlobEncoding> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDxcBlobEncoding> for &'a IDxcBlobUtf16 {
    fn into_param(self) -> ::windows_core::Param<'a, IDxcBlobEncoding> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcBlobUtf16 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcBlobUtf16 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcBlobUtf16 {}
impl ::core::fmt::Debug for IDxcBlobUtf16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcBlobUtf16").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDxcBlobUtf16 {
    type Vtable = IDxcBlobUtf16_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa3f84eab_0faa_497e_a39c_ee6ed60b2d84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlobUtf16_Vtbl {
    pub base__: IDxcBlobEncoding_Vtbl,
    pub GetStringPointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::PWSTR,
    pub GetStringLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> usize,
}
#[repr(transparent)]
pub struct IDxcBlobUtf8(::windows_core::IUnknown);
impl IDxcBlobUtf8 {
    pub unsafe fn GetBufferPointer(&self) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).base__.base__.GetBufferPointer)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn GetBufferSize(&self) -> usize {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).base__.base__.GetBufferSize)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn GetEncoding(&self, pknown: *mut ::win32_foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetEncoding)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pknown), ::core::mem::transmute(pcodepage)).ok()
    }
    pub unsafe fn GetStringPointer(&self) -> ::windows_core::PSTR {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).GetStringPointer)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn GetStringLength(&self) -> usize {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).GetStringLength)(::windows_core::Interface::as_raw(self)))
    }
}
impl ::core::convert::From<IDxcBlobUtf8> for ::windows_core::IUnknown {
    fn from(value: IDxcBlobUtf8) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcBlobUtf8> for ::windows_core::IUnknown {
    fn from(value: &IDxcBlobUtf8) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDxcBlobUtf8 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDxcBlobUtf8 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDxcBlobUtf8> for IDxcBlob {
    fn from(value: IDxcBlobUtf8) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcBlobUtf8> for IDxcBlob {
    fn from(value: &IDxcBlobUtf8) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDxcBlob> for IDxcBlobUtf8 {
    fn into_param(self) -> ::windows_core::Param<'a, IDxcBlob> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDxcBlob> for &'a IDxcBlobUtf8 {
    fn into_param(self) -> ::windows_core::Param<'a, IDxcBlob> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDxcBlobUtf8> for IDxcBlobEncoding {
    fn from(value: IDxcBlobUtf8) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcBlobUtf8> for IDxcBlobEncoding {
    fn from(value: &IDxcBlobUtf8) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDxcBlobEncoding> for IDxcBlobUtf8 {
    fn into_param(self) -> ::windows_core::Param<'a, IDxcBlobEncoding> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDxcBlobEncoding> for &'a IDxcBlobUtf8 {
    fn into_param(self) -> ::windows_core::Param<'a, IDxcBlobEncoding> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcBlobUtf8 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcBlobUtf8 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcBlobUtf8 {}
impl ::core::fmt::Debug for IDxcBlobUtf8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcBlobUtf8").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDxcBlobUtf8 {
    type Vtable = IDxcBlobUtf8_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3da636c9_ba71_4024_a301_30cbf125305b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlobUtf8_Vtbl {
    pub base__: IDxcBlobEncoding_Vtbl,
    pub GetStringPointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::PSTR,
    pub GetStringLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> usize,
}
#[repr(transparent)]
pub struct IDxcCompiler(::windows_core::IUnknown);
impl IDxcCompiler {
    pub unsafe fn Compile<'a, Param0: ::windows_core::IntoParam<'a, IDxcBlob>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param8: ::windows_core::IntoParam<'a, IDxcIncludeHandler>>(&self, psource: Param0, psourcename: Param1, pentrypoint: Param2, ptargetprofile: Param3, parguments: &[::windows_core::PWSTR], pdefines: &[DxcDefine], pincludehandler: Param8) -> ::windows_core::Result<IDxcOperationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Compile)(::windows_core::Interface::as_raw(self), psource.into_param().abi(), psourcename.into_param().abi(), pentrypoint.into_param().abi(), ptargetprofile.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(parguments)), parguments.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pdefines)), pdefines.len() as _, pincludehandler.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcOperationResult>(result__)
    }
    pub unsafe fn Preprocess<'a, Param0: ::windows_core::IntoParam<'a, IDxcBlob>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param6: ::windows_core::IntoParam<'a, IDxcIncludeHandler>>(&self, psource: Param0, psourcename: Param1, parguments: &[::windows_core::PWSTR], pdefines: &[DxcDefine], pincludehandler: Param6) -> ::windows_core::Result<IDxcOperationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Preprocess)(::windows_core::Interface::as_raw(self), psource.into_param().abi(), psourcename.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(parguments)), parguments.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pdefines)), pdefines.len() as _, pincludehandler.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcOperationResult>(result__)
    }
    pub unsafe fn Disassemble<'a, Param0: ::windows_core::IntoParam<'a, IDxcBlob>>(&self, psource: Param0) -> ::windows_core::Result<IDxcBlobEncoding> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Disassemble)(::windows_core::Interface::as_raw(self), psource.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobEncoding>(result__)
    }
}
impl ::core::convert::From<IDxcCompiler> for ::windows_core::IUnknown {
    fn from(value: IDxcCompiler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcCompiler> for ::windows_core::IUnknown {
    fn from(value: &IDxcCompiler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDxcCompiler {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDxcCompiler {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcCompiler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcCompiler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcCompiler {}
impl ::core::fmt::Debug for IDxcCompiler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcCompiler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDxcCompiler {
    type Vtable = IDxcCompiler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8c210bf3_011f_4422_8d70_6f9acb8db617);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompiler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Compile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psource: ::windows_core::RawPtr, psourcename: ::windows_core::PCWSTR, pentrypoint: ::windows_core::PCWSTR, ptargetprofile: ::windows_core::PCWSTR, parguments: *const ::windows_core::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::windows_core::RawPtr, ppresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Preprocess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psource: ::windows_core::RawPtr, psourcename: ::windows_core::PCWSTR, parguments: *const ::windows_core::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::windows_core::RawPtr, ppresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Disassemble: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psource: ::windows_core::RawPtr, ppdisassembly: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDxcCompiler2(::windows_core::IUnknown);
impl IDxcCompiler2 {
    pub unsafe fn Compile<'a, Param0: ::windows_core::IntoParam<'a, IDxcBlob>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param8: ::windows_core::IntoParam<'a, IDxcIncludeHandler>>(&self, psource: Param0, psourcename: Param1, pentrypoint: Param2, ptargetprofile: Param3, parguments: &[::windows_core::PWSTR], pdefines: &[DxcDefine], pincludehandler: Param8) -> ::windows_core::Result<IDxcOperationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Compile)(::windows_core::Interface::as_raw(self), psource.into_param().abi(), psourcename.into_param().abi(), pentrypoint.into_param().abi(), ptargetprofile.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(parguments)), parguments.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pdefines)), pdefines.len() as _, pincludehandler.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcOperationResult>(result__)
    }
    pub unsafe fn Preprocess<'a, Param0: ::windows_core::IntoParam<'a, IDxcBlob>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param6: ::windows_core::IntoParam<'a, IDxcIncludeHandler>>(&self, psource: Param0, psourcename: Param1, parguments: &[::windows_core::PWSTR], pdefines: &[DxcDefine], pincludehandler: Param6) -> ::windows_core::Result<IDxcOperationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Preprocess)(::windows_core::Interface::as_raw(self), psource.into_param().abi(), psourcename.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(parguments)), parguments.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pdefines)), pdefines.len() as _, pincludehandler.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcOperationResult>(result__)
    }
    pub unsafe fn Disassemble<'a, Param0: ::windows_core::IntoParam<'a, IDxcBlob>>(&self, psource: Param0) -> ::windows_core::Result<IDxcBlobEncoding> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Disassemble)(::windows_core::Interface::as_raw(self), psource.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobEncoding>(result__)
    }
    pub unsafe fn CompileWithDebug<'a, Param0: ::windows_core::IntoParam<'a, IDxcBlob>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param8: ::windows_core::IntoParam<'a, IDxcIncludeHandler>>(&self, psource: Param0, psourcename: Param1, pentrypoint: Param2, ptargetprofile: Param3, parguments: &[::windows_core::PWSTR], pdefines: &[DxcDefine], pincludehandler: Param8, ppresult: *mut ::core::option::Option<IDxcOperationResult>, ppdebugblobname: *mut ::windows_core::PWSTR, ppdebugblob: *mut ::core::option::Option<IDxcBlob>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CompileWithDebug)(
            ::windows_core::Interface::as_raw(self),
            psource.into_param().abi(),
            psourcename.into_param().abi(),
            pentrypoint.into_param().abi(),
            ptargetprofile.into_param().abi(),
            ::core::mem::transmute(::windows_core::as_ptr_or_null(parguments)),
            parguments.len() as _,
            ::core::mem::transmute(::windows_core::as_ptr_or_null(pdefines)),
            pdefines.len() as _,
            pincludehandler.into_param().abi(),
            ::core::mem::transmute(ppresult),
            ::core::mem::transmute(ppdebugblobname),
            ::core::mem::transmute(ppdebugblob),
        )
        .ok()
    }
}
impl ::core::convert::From<IDxcCompiler2> for ::windows_core::IUnknown {
    fn from(value: IDxcCompiler2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcCompiler2> for ::windows_core::IUnknown {
    fn from(value: &IDxcCompiler2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDxcCompiler2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDxcCompiler2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDxcCompiler2> for IDxcCompiler {
    fn from(value: IDxcCompiler2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcCompiler2> for IDxcCompiler {
    fn from(value: &IDxcCompiler2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDxcCompiler> for IDxcCompiler2 {
    fn into_param(self) -> ::windows_core::Param<'a, IDxcCompiler> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDxcCompiler> for &'a IDxcCompiler2 {
    fn into_param(self) -> ::windows_core::Param<'a, IDxcCompiler> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcCompiler2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcCompiler2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcCompiler2 {}
impl ::core::fmt::Debug for IDxcCompiler2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcCompiler2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDxcCompiler2 {
    type Vtable = IDxcCompiler2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa005a9d9_b8bb_4594_b5c9_0e633bec4d37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompiler2_Vtbl {
    pub base__: IDxcCompiler_Vtbl,
    pub CompileWithDebug: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psource: ::windows_core::RawPtr, psourcename: ::windows_core::PCWSTR, pentrypoint: ::windows_core::PCWSTR, ptargetprofile: ::windows_core::PCWSTR, parguments: *const ::windows_core::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::windows_core::RawPtr, ppresult: *mut ::windows_core::RawPtr, ppdebugblobname: *mut ::windows_core::PWSTR, ppdebugblob: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDxcCompiler3(::windows_core::IUnknown);
impl IDxcCompiler3 {
    pub unsafe fn Compile<'a, Param3: ::windows_core::IntoParam<'a, IDxcIncludeHandler>>(&self, psource: *const DxcBuffer, parguments: &[::windows_core::PWSTR], pincludehandler: Param3, riid: *const ::windows_core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Compile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(psource), ::core::mem::transmute(::windows_core::as_ptr_or_null(parguments)), parguments.len() as _, pincludehandler.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppresult)).ok()
    }
    pub unsafe fn Disassemble(&self, pobject: *const DxcBuffer, riid: *const ::windows_core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Disassemble)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pobject), ::core::mem::transmute(riid), ::core::mem::transmute(ppresult)).ok()
    }
}
impl ::core::convert::From<IDxcCompiler3> for ::windows_core::IUnknown {
    fn from(value: IDxcCompiler3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcCompiler3> for ::windows_core::IUnknown {
    fn from(value: &IDxcCompiler3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDxcCompiler3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDxcCompiler3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcCompiler3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcCompiler3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcCompiler3 {}
impl ::core::fmt::Debug for IDxcCompiler3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcCompiler3").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDxcCompiler3 {
    type Vtable = IDxcCompiler3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x228b4687_5a6a_4730_900c_9702b2203f54);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompiler3_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Compile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psource: *const DxcBuffer, parguments: *const ::windows_core::PWSTR, argcount: u32, pincludehandler: ::windows_core::RawPtr, riid: *const ::windows_core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Disassemble: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobject: *const DxcBuffer, riid: *const ::windows_core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDxcCompilerArgs(::windows_core::IUnknown);
impl IDxcCompilerArgs {
    pub unsafe fn GetArguments(&self) -> *mut ::windows_core::PWSTR {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).GetArguments)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn GetCount(&self) -> u32 {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn AddArguments(&self, parguments: &[::windows_core::PWSTR]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddArguments)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(parguments)), parguments.len() as _).ok()
    }
    pub unsafe fn AddArgumentsUTF8(&self, parguments: &[::windows_core::PSTR]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddArgumentsUTF8)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(parguments)), parguments.len() as _).ok()
    }
    pub unsafe fn AddDefines(&self, pdefines: &[DxcDefine]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddDefines)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(pdefines)), pdefines.len() as _).ok()
    }
}
impl ::core::convert::From<IDxcCompilerArgs> for ::windows_core::IUnknown {
    fn from(value: IDxcCompilerArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcCompilerArgs> for ::windows_core::IUnknown {
    fn from(value: &IDxcCompilerArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDxcCompilerArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDxcCompilerArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcCompilerArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcCompilerArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcCompilerArgs {}
impl ::core::fmt::Debug for IDxcCompilerArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcCompilerArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDxcCompilerArgs {
    type Vtable = IDxcCompilerArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x73effe2a_70dc_45f8_9690_eff64c02429d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompilerArgs_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> *mut ::windows_core::PWSTR,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub AddArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parguments: *const ::windows_core::PWSTR, argcount: u32) -> ::windows_core::HRESULT,
    pub AddArgumentsUTF8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parguments: *const ::windows_core::PSTR, argcount: u32) -> ::windows_core::HRESULT,
    pub AddDefines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdefines: *const DxcDefine, definecount: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDxcContainerBuilder(::windows_core::IUnknown);
impl IDxcContainerBuilder {
    pub unsafe fn Load<'a, Param0: ::windows_core::IntoParam<'a, IDxcBlob>>(&self, pdxilcontainerheader: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Load)(::windows_core::Interface::as_raw(self), pdxilcontainerheader.into_param().abi()).ok()
    }
    pub unsafe fn AddPart<'a, Param1: ::windows_core::IntoParam<'a, IDxcBlob>>(&self, fourcc: u32, psource: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddPart)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(fourcc), psource.into_param().abi()).ok()
    }
    pub unsafe fn RemovePart(&self, fourcc: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemovePart)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(fourcc)).ok()
    }
    pub unsafe fn SerializeContainer(&self) -> ::windows_core::Result<IDxcOperationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).SerializeContainer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcOperationResult>(result__)
    }
}
impl ::core::convert::From<IDxcContainerBuilder> for ::windows_core::IUnknown {
    fn from(value: IDxcContainerBuilder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcContainerBuilder> for ::windows_core::IUnknown {
    fn from(value: &IDxcContainerBuilder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDxcContainerBuilder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDxcContainerBuilder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcContainerBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcContainerBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcContainerBuilder {}
impl ::core::fmt::Debug for IDxcContainerBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcContainerBuilder").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDxcContainerBuilder {
    type Vtable = IDxcContainerBuilder_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x334b1f50_2292_4b35_99a1_25588d8c17fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcContainerBuilder_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdxilcontainerheader: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AddPart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fourcc: u32, psource: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemovePart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fourcc: u32) -> ::windows_core::HRESULT,
    pub SerializeContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDxcContainerReflection(::windows_core::IUnknown);
impl IDxcContainerReflection {
    pub unsafe fn Load<'a, Param0: ::windows_core::IntoParam<'a, IDxcBlob>>(&self, pcontainer: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Load)(::windows_core::Interface::as_raw(self), pcontainer.into_param().abi()).ok()
    }
    pub unsafe fn GetPartCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetPartCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetPartKind(&self, idx: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetPartKind)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(idx), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetPartContent(&self, idx: u32) -> ::windows_core::Result<IDxcBlob> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetPartContent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(idx), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlob>(result__)
    }
    pub unsafe fn FindFirstPartKind(&self, kind: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).FindFirstPartKind)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(kind), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetPartReflection(&self, idx: u32, iid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPartReflection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(idx), ::core::mem::transmute(iid), ::core::mem::transmute(ppvobject)).ok()
    }
}
impl ::core::convert::From<IDxcContainerReflection> for ::windows_core::IUnknown {
    fn from(value: IDxcContainerReflection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcContainerReflection> for ::windows_core::IUnknown {
    fn from(value: &IDxcContainerReflection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDxcContainerReflection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDxcContainerReflection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcContainerReflection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcContainerReflection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcContainerReflection {}
impl ::core::fmt::Debug for IDxcContainerReflection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcContainerReflection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDxcContainerReflection {
    type Vtable = IDxcContainerReflection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd2c21b26_8350_4bdc_976a_331ce6f4c54c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcContainerReflection_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontainer: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetPartCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presult: *mut u32) -> ::windows_core::HRESULT,
    pub GetPartKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idx: u32, presult: *mut u32) -> ::windows_core::HRESULT,
    pub GetPartContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idx: u32, ppresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FindFirstPartKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: u32, presult: *mut u32) -> ::windows_core::HRESULT,
    pub GetPartReflection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idx: u32, iid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDxcExtraOutputs(::windows_core::IUnknown);
impl IDxcExtraOutputs {
    pub unsafe fn GetOutputCount(&self) -> u32 {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).GetOutputCount)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn GetOutput(&self, uindex: u32, iid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputtype: *mut ::core::option::Option<IDxcBlobUtf16>, ppoutputname: *mut ::core::option::Option<IDxcBlobUtf16>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetOutput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uindex), ::core::mem::transmute(iid), ::core::mem::transmute(ppvobject), ::core::mem::transmute(ppoutputtype), ::core::mem::transmute(ppoutputname)).ok()
    }
}
impl ::core::convert::From<IDxcExtraOutputs> for ::windows_core::IUnknown {
    fn from(value: IDxcExtraOutputs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcExtraOutputs> for ::windows_core::IUnknown {
    fn from(value: &IDxcExtraOutputs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDxcExtraOutputs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDxcExtraOutputs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcExtraOutputs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcExtraOutputs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcExtraOutputs {}
impl ::core::fmt::Debug for IDxcExtraOutputs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcExtraOutputs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDxcExtraOutputs {
    type Vtable = IDxcExtraOutputs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x319b37a2_a5c2_494a_a5de_4801b2faf989);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcExtraOutputs_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetOutputCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, iid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputtype: *mut ::windows_core::RawPtr, ppoutputname: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDxcIncludeHandler(::windows_core::IUnknown);
impl IDxcIncludeHandler {
    pub unsafe fn LoadSource<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pfilename: Param0) -> ::windows_core::Result<IDxcBlob> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).LoadSource)(::windows_core::Interface::as_raw(self), pfilename.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlob>(result__)
    }
}
impl ::core::convert::From<IDxcIncludeHandler> for ::windows_core::IUnknown {
    fn from(value: IDxcIncludeHandler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcIncludeHandler> for ::windows_core::IUnknown {
    fn from(value: &IDxcIncludeHandler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDxcIncludeHandler {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDxcIncludeHandler {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcIncludeHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcIncludeHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcIncludeHandler {}
impl ::core::fmt::Debug for IDxcIncludeHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcIncludeHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDxcIncludeHandler {
    type Vtable = IDxcIncludeHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7f61fc7d_950d_467f_b3e3_3c02fb49187c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcIncludeHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub LoadSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilename: ::windows_core::PCWSTR, ppincludesource: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDxcLibrary(::windows_core::IUnknown);
impl IDxcLibrary {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetMalloc<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::IMalloc>>(&self, pmalloc: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMalloc)(::windows_core::Interface::as_raw(self), pmalloc.into_param().abi()).ok()
    }
    pub unsafe fn CreateBlobFromBlob<'a, Param0: ::windows_core::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0, offset: u32, length: u32) -> ::windows_core::Result<IDxcBlob> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateBlobFromBlob)(::windows_core::Interface::as_raw(self), pblob.into_param().abi(), ::core::mem::transmute(offset), ::core::mem::transmute(length), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlob>(result__)
    }
    pub unsafe fn CreateBlobFromFile<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pfilename: Param0, codepage: *const DXC_CP) -> ::windows_core::Result<IDxcBlobEncoding> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateBlobFromFile)(::windows_core::Interface::as_raw(self), pfilename.into_param().abi(), ::core::mem::transmute(codepage), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobEncoding>(result__)
    }
    pub unsafe fn CreateBlobWithEncodingFromPinned(&self, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows_core::Result<IDxcBlobEncoding> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateBlobWithEncodingFromPinned)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptext), ::core::mem::transmute(size), ::core::mem::transmute(codepage), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobEncoding>(result__)
    }
    pub unsafe fn CreateBlobWithEncodingOnHeapCopy(&self, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows_core::Result<IDxcBlobEncoding> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateBlobWithEncodingOnHeapCopy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptext), ::core::mem::transmute(size), ::core::mem::transmute(codepage), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateBlobWithEncodingOnMalloc<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::IMalloc>>(&self, ptext: *const ::core::ffi::c_void, pimalloc: Param1, size: u32, codepage: DXC_CP) -> ::windows_core::Result<IDxcBlobEncoding> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateBlobWithEncodingOnMalloc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptext), pimalloc.into_param().abi(), ::core::mem::transmute(size), ::core::mem::transmute(codepage), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobEncoding>(result__)
    }
    pub unsafe fn CreateIncludeHandler(&self) -> ::windows_core::Result<IDxcIncludeHandler> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateIncludeHandler)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcIncludeHandler>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateStreamFromBlobReadOnly<'a, Param0: ::windows_core::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0) -> ::windows_core::Result<::win32_system::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateStreamFromBlobReadOnly)(::windows_core::Interface::as_raw(self), pblob.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::IStream>(result__)
    }
    pub unsafe fn GetBlobAsUtf8<'a, Param0: ::windows_core::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0) -> ::windows_core::Result<IDxcBlobEncoding> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetBlobAsUtf8)(::windows_core::Interface::as_raw(self), pblob.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobEncoding>(result__)
    }
    pub unsafe fn GetBlobAsUtf16<'a, Param0: ::windows_core::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0) -> ::windows_core::Result<IDxcBlobEncoding> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetBlobAsUtf16)(::windows_core::Interface::as_raw(self), pblob.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobEncoding>(result__)
    }
}
impl ::core::convert::From<IDxcLibrary> for ::windows_core::IUnknown {
    fn from(value: IDxcLibrary) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcLibrary> for ::windows_core::IUnknown {
    fn from(value: &IDxcLibrary) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDxcLibrary {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDxcLibrary {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcLibrary {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcLibrary {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcLibrary {}
impl ::core::fmt::Debug for IDxcLibrary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcLibrary").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDxcLibrary {
    type Vtable = IDxcLibrary_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe5204dc7_d18c_4c3c_bdfb_851673980fe7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcLibrary_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SetMalloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmalloc: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetMalloc: usize,
    pub CreateBlobFromBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: ::windows_core::RawPtr, offset: u32, length: u32, ppresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateBlobFromFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilename: ::windows_core::PCWSTR, codepage: *const DXC_CP, pblobencoding: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateBlobWithEncodingFromPinned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateBlobWithEncodingOnHeapCopy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateBlobWithEncodingOnMalloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptext: *const ::core::ffi::c_void, pimalloc: ::windows_core::RawPtr, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateBlobWithEncodingOnMalloc: usize,
    pub CreateIncludeHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateStreamFromBlobReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: ::windows_core::RawPtr, ppstream: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateStreamFromBlobReadOnly: usize,
    pub GetBlobAsUtf8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: ::windows_core::RawPtr, pblobencoding: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetBlobAsUtf16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: ::windows_core::RawPtr, pblobencoding: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDxcLinker(::windows_core::IUnknown);
impl IDxcLinker {
    pub unsafe fn RegisterLibrary<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, IDxcBlob>>(&self, plibname: Param0, plib: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterLibrary)(::windows_core::Interface::as_raw(self), plibname.into_param().abi(), plib.into_param().abi()).ok()
    }
    pub unsafe fn Link<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pentryname: Param0, ptargetprofile: Param1, plibnames: &[::windows_core::PWSTR], parguments: &[::windows_core::PWSTR]) -> ::windows_core::Result<IDxcOperationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Link)(::windows_core::Interface::as_raw(self), pentryname.into_param().abi(), ptargetprofile.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(plibnames)), plibnames.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(parguments)), parguments.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcOperationResult>(result__)
    }
}
impl ::core::convert::From<IDxcLinker> for ::windows_core::IUnknown {
    fn from(value: IDxcLinker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcLinker> for ::windows_core::IUnknown {
    fn from(value: &IDxcLinker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDxcLinker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDxcLinker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcLinker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcLinker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcLinker {}
impl ::core::fmt::Debug for IDxcLinker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcLinker").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDxcLinker {
    type Vtable = IDxcLinker_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf1b5be2a_62dd_4327_a1c2_42ac1e1e78e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcLinker_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub RegisterLibrary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plibname: ::windows_core::PCWSTR, plib: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Link: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pentryname: ::windows_core::PCWSTR, ptargetprofile: ::windows_core::PCWSTR, plibnames: *const ::windows_core::PWSTR, libcount: u32, parguments: *const ::windows_core::PWSTR, argcount: u32, ppresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDxcOperationResult(::windows_core::IUnknown);
impl IDxcOperationResult {
    pub unsafe fn GetStatus(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
        (::windows_core::Interface::vtable(self).GetStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::HRESULT>(result__)
    }
    pub unsafe fn GetResult(&self) -> ::windows_core::Result<IDxcBlob> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetResult)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlob>(result__)
    }
    pub unsafe fn GetErrorBuffer(&self) -> ::windows_core::Result<IDxcBlobEncoding> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetErrorBuffer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobEncoding>(result__)
    }
}
impl ::core::convert::From<IDxcOperationResult> for ::windows_core::IUnknown {
    fn from(value: IDxcOperationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcOperationResult> for ::windows_core::IUnknown {
    fn from(value: &IDxcOperationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDxcOperationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDxcOperationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcOperationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcOperationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcOperationResult {}
impl ::core::fmt::Debug for IDxcOperationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcOperationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDxcOperationResult {
    type Vtable = IDxcOperationResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcedb484a_d4e9_445a_b991_ca21ca157dc2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcOperationResult_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub GetResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetErrorBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pperrors: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDxcOptimizer(::windows_core::IUnknown);
impl IDxcOptimizer {
    pub unsafe fn GetAvailablePassCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetAvailablePassCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAvailablePass(&self, index: u32) -> ::windows_core::Result<IDxcOptimizerPass> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetAvailablePass)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcOptimizerPass>(result__)
    }
    pub unsafe fn RunOptimizer<'a, Param0: ::windows_core::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0, ppoptions: &[::windows_core::PWSTR], poutputmodule: *mut ::core::option::Option<IDxcBlob>, ppoutputtext: *mut ::core::option::Option<IDxcBlobEncoding>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RunOptimizer)(::windows_core::Interface::as_raw(self), pblob.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(ppoptions)), ppoptions.len() as _, ::core::mem::transmute(poutputmodule), ::core::mem::transmute(ppoutputtext)).ok()
    }
}
impl ::core::convert::From<IDxcOptimizer> for ::windows_core::IUnknown {
    fn from(value: IDxcOptimizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcOptimizer> for ::windows_core::IUnknown {
    fn from(value: &IDxcOptimizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDxcOptimizer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDxcOptimizer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcOptimizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcOptimizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcOptimizer {}
impl ::core::fmt::Debug for IDxcOptimizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcOptimizer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDxcOptimizer {
    type Vtable = IDxcOptimizer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x25740e2e_9cba_401b_9119_4fb42f39f270);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcOptimizer_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetAvailablePassCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT,
    pub GetAvailablePass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RunOptimizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: ::windows_core::RawPtr, ppoptions: *const ::windows_core::PWSTR, optioncount: u32, poutputmodule: *mut ::windows_core::RawPtr, ppoutputtext: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDxcOptimizerPass(::windows_core::IUnknown);
impl IDxcOptimizerPass {
    pub unsafe fn GetOptionName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetOptionName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetDescription(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetOptionArgCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetOptionArgCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOptionArgName(&self, argindex: u32) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetOptionArgName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(argindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetOptionArgDescription(&self, argindex: u32) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetOptionArgDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(argindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IDxcOptimizerPass> for ::windows_core::IUnknown {
    fn from(value: IDxcOptimizerPass) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcOptimizerPass> for ::windows_core::IUnknown {
    fn from(value: &IDxcOptimizerPass) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDxcOptimizerPass {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDxcOptimizerPass {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcOptimizerPass {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcOptimizerPass {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcOptimizerPass {}
impl ::core::fmt::Debug for IDxcOptimizerPass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcOptimizerPass").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDxcOptimizerPass {
    type Vtable = IDxcOptimizerPass_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae2cd79f_cc22_453f_9b6b_b124e7a5204c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcOptimizerPass_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetOptionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetOptionArgCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT,
    pub GetOptionArgName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, argindex: u32, ppresult: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetOptionArgDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, argindex: u32, ppresult: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDxcPdbUtils(::windows_core::IUnknown);
impl IDxcPdbUtils {
    pub unsafe fn Load<'a, Param0: ::windows_core::IntoParam<'a, IDxcBlob>>(&self, ppdbordxil: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Load)(::windows_core::Interface::as_raw(self), ppdbordxil.into_param().abi()).ok()
    }
    pub unsafe fn GetSourceCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetSourceCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetSource(&self, uindex: u32) -> ::windows_core::Result<IDxcBlobEncoding> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSource)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobEncoding>(result__)
    }
    pub unsafe fn GetSourceName(&self, uindex: u32) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetSourceName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetFlagCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetFlagCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetFlag(&self, uindex: u32) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetFlag)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetArgCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetArgCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetArg(&self, uindex: u32) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetArg)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetArgPairCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetArgPairCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetArgPair(&self, uindex: u32, pname: *mut ::win32_foundation::BSTR, pvalue: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetArgPair)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uindex), ::core::mem::transmute(pname), ::core::mem::transmute(pvalue)).ok()
    }
    pub unsafe fn GetDefineCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetDefineCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetDefine(&self, uindex: u32) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetDefine)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetTargetProfile(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetTargetProfile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetEntryPoint(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetEntryPoint)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetMainFileName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetMainFileName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetHash(&self) -> ::windows_core::Result<IDxcBlob> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetHash)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlob>(result__)
    }
    pub unsafe fn GetName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn IsFullPDB(&self) -> ::win32_foundation::BOOL {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).IsFullPDB)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn GetFullPDB(&self) -> ::windows_core::Result<IDxcBlob> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetFullPDB)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlob>(result__)
    }
    pub unsafe fn GetVersionInfo(&self) -> ::windows_core::Result<IDxcVersionInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetVersionInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcVersionInfo>(result__)
    }
    pub unsafe fn SetCompiler<'a, Param0: ::windows_core::IntoParam<'a, IDxcCompiler3>>(&self, pcompiler: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCompiler)(::windows_core::Interface::as_raw(self), pcompiler.into_param().abi()).ok()
    }
    pub unsafe fn CompileForFullPDB(&self) -> ::windows_core::Result<IDxcResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CompileForFullPDB)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcResult>(result__)
    }
    pub unsafe fn OverrideArgs(&self, pargpairs: *const DxcArgPair, unumargpairs: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OverrideArgs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pargpairs), ::core::mem::transmute(unumargpairs)).ok()
    }
    pub unsafe fn OverrideRootSignature<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, prootsignature: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OverrideRootSignature)(::windows_core::Interface::as_raw(self), prootsignature.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDxcPdbUtils> for ::windows_core::IUnknown {
    fn from(value: IDxcPdbUtils) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcPdbUtils> for ::windows_core::IUnknown {
    fn from(value: &IDxcPdbUtils) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDxcPdbUtils {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDxcPdbUtils {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcPdbUtils {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcPdbUtils {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcPdbUtils {}
impl ::core::fmt::Debug for IDxcPdbUtils {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcPdbUtils").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDxcPdbUtils {
    type Vtable = IDxcPdbUtils_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe6c9647e_9d6a_4c3b_b94c_524b5a6c343d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcPdbUtils_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdbordxil: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetSourceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT,
    pub GetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, ppresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetSourceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetFlagCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT,
    pub GetFlag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetArgCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT,
    pub GetArg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetArgPairCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT,
    pub GetArgPair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, pname: *mut ::win32_foundation::BSTR, pvalue: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetDefineCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT,
    pub GetDefine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetTargetProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presult: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetEntryPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presult: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetMainFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presult: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presult: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub IsFullPDB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL,
    pub GetFullPDB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfullpdb: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetVersionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppversioninfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCompiler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcompiler: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CompileForFullPDB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OverrideArgs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pargpairs: *const DxcArgPair, unumargpairs: u32) -> ::windows_core::HRESULT,
    pub OverrideRootSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prootsignature: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDxcResult(::windows_core::IUnknown);
impl IDxcResult {
    pub unsafe fn GetStatus(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::HRESULT>(result__)
    }
    pub unsafe fn GetResult(&self) -> ::windows_core::Result<IDxcBlob> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetResult)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlob>(result__)
    }
    pub unsafe fn GetErrorBuffer(&self) -> ::windows_core::Result<IDxcBlobEncoding> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetErrorBuffer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobEncoding>(result__)
    }
    pub unsafe fn HasOutput(&self, dxcoutkind: DXC_OUT_KIND) -> ::win32_foundation::BOOL {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).HasOutput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dxcoutkind)))
    }
    pub unsafe fn GetOutput(&self, dxcoutkind: DXC_OUT_KIND, iid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputname: *mut ::core::option::Option<IDxcBlobUtf16>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetOutput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dxcoutkind), ::core::mem::transmute(iid), ::core::mem::transmute(ppvobject), ::core::mem::transmute(ppoutputname)).ok()
    }
    pub unsafe fn GetNumOutputs(&self) -> u32 {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).GetNumOutputs)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn GetOutputByIndex(&self, index: u32) -> DXC_OUT_KIND {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).GetOutputByIndex)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index)))
    }
    pub unsafe fn PrimaryOutput(&self) -> DXC_OUT_KIND {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).PrimaryOutput)(::windows_core::Interface::as_raw(self)))
    }
}
impl ::core::convert::From<IDxcResult> for ::windows_core::IUnknown {
    fn from(value: IDxcResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcResult> for ::windows_core::IUnknown {
    fn from(value: &IDxcResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDxcResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDxcResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDxcResult> for IDxcOperationResult {
    fn from(value: IDxcResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcResult> for IDxcOperationResult {
    fn from(value: &IDxcResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDxcOperationResult> for IDxcResult {
    fn into_param(self) -> ::windows_core::Param<'a, IDxcOperationResult> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDxcOperationResult> for &'a IDxcResult {
    fn into_param(self) -> ::windows_core::Param<'a, IDxcOperationResult> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcResult {}
impl ::core::fmt::Debug for IDxcResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDxcResult {
    type Vtable = IDxcResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x58346cda_dde7_4497_9461_6f87af5e0659);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcResult_Vtbl {
    pub base__: IDxcOperationResult_Vtbl,
    pub HasOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dxcoutkind: DXC_OUT_KIND) -> ::win32_foundation::BOOL,
    pub GetOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dxcoutkind: DXC_OUT_KIND, iid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputname: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetNumOutputs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetOutputByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> DXC_OUT_KIND,
    pub PrimaryOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DXC_OUT_KIND,
}
#[repr(transparent)]
pub struct IDxcUtils(::windows_core::IUnknown);
impl IDxcUtils {
    pub unsafe fn CreateBlobFromBlob<'a, Param0: ::windows_core::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0, offset: u32, length: u32) -> ::windows_core::Result<IDxcBlob> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateBlobFromBlob)(::windows_core::Interface::as_raw(self), pblob.into_param().abi(), ::core::mem::transmute(offset), ::core::mem::transmute(length), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlob>(result__)
    }
    pub unsafe fn CreateBlobFromPinned(&self, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows_core::Result<IDxcBlobEncoding> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateBlobFromPinned)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdata), ::core::mem::transmute(size), ::core::mem::transmute(codepage), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MoveToBlob<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::IMalloc>>(&self, pdata: *const ::core::ffi::c_void, pimalloc: Param1, size: u32, codepage: DXC_CP) -> ::windows_core::Result<IDxcBlobEncoding> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).MoveToBlob)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdata), pimalloc.into_param().abi(), ::core::mem::transmute(size), ::core::mem::transmute(codepage), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobEncoding>(result__)
    }
    pub unsafe fn CreateBlob(&self, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows_core::Result<IDxcBlobEncoding> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateBlob)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdata), ::core::mem::transmute(size), ::core::mem::transmute(codepage), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobEncoding>(result__)
    }
    pub unsafe fn LoadFile<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pfilename: Param0, pcodepage: *const DXC_CP) -> ::windows_core::Result<IDxcBlobEncoding> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).LoadFile)(::windows_core::Interface::as_raw(self), pfilename.into_param().abi(), ::core::mem::transmute(pcodepage), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateReadOnlyStreamFromBlob<'a, Param0: ::windows_core::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0) -> ::windows_core::Result<::win32_system::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateReadOnlyStreamFromBlob)(::windows_core::Interface::as_raw(self), pblob.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::IStream>(result__)
    }
    pub unsafe fn CreateDefaultIncludeHandler(&self) -> ::windows_core::Result<IDxcIncludeHandler> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateDefaultIncludeHandler)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcIncludeHandler>(result__)
    }
    pub unsafe fn GetBlobAsUtf8<'a, Param0: ::windows_core::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0) -> ::windows_core::Result<IDxcBlobUtf8> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetBlobAsUtf8)(::windows_core::Interface::as_raw(self), pblob.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobUtf8>(result__)
    }
    pub unsafe fn GetBlobAsUtf16<'a, Param0: ::windows_core::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0) -> ::windows_core::Result<IDxcBlobUtf16> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetBlobAsUtf16)(::windows_core::Interface::as_raw(self), pblob.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcBlobUtf16>(result__)
    }
    pub unsafe fn GetDxilContainerPart(&self, pshader: *const DxcBuffer, dxcpart: u32, pppartdata: *mut *mut ::core::ffi::c_void, ppartsizeinbytes: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDxilContainerPart)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pshader), ::core::mem::transmute(dxcpart), ::core::mem::transmute(pppartdata), ::core::mem::transmute(ppartsizeinbytes)).ok()
    }
    pub unsafe fn CreateReflection(&self, pdata: *const DxcBuffer, iid: *const ::windows_core::GUID, ppvreflection: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateReflection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdata), ::core::mem::transmute(iid), ::core::mem::transmute(ppvreflection)).ok()
    }
    pub unsafe fn BuildArguments<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, psourcename: Param0, pentrypoint: Param1, ptargetprofile: Param2, parguments: &[::windows_core::PWSTR], pdefines: &[DxcDefine]) -> ::windows_core::Result<IDxcCompilerArgs> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).BuildArguments)(::windows_core::Interface::as_raw(self), psourcename.into_param().abi(), pentrypoint.into_param().abi(), ptargetprofile.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(parguments)), parguments.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pdefines)), pdefines.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcCompilerArgs>(result__)
    }
    pub unsafe fn GetPDBContents<'a, Param0: ::windows_core::IntoParam<'a, IDxcBlob>>(&self, ppdbblob: Param0, pphash: *mut ::core::option::Option<IDxcBlob>, ppcontainer: *mut ::core::option::Option<IDxcBlob>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPDBContents)(::windows_core::Interface::as_raw(self), ppdbblob.into_param().abi(), ::core::mem::transmute(pphash), ::core::mem::transmute(ppcontainer)).ok()
    }
}
impl ::core::convert::From<IDxcUtils> for ::windows_core::IUnknown {
    fn from(value: IDxcUtils) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcUtils> for ::windows_core::IUnknown {
    fn from(value: &IDxcUtils) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDxcUtils {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDxcUtils {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcUtils {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcUtils {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcUtils {}
impl ::core::fmt::Debug for IDxcUtils {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcUtils").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDxcUtils {
    type Vtable = IDxcUtils_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4605c4cb_2019_492a_ada4_65f20bb7d67f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcUtils_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CreateBlobFromBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: ::windows_core::RawPtr, offset: u32, length: u32, ppresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateBlobFromPinned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub MoveToBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, pimalloc: ::windows_core::RawPtr, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    MoveToBlob: usize,
    pub CreateBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LoadFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilename: ::windows_core::PCWSTR, pcodepage: *const DXC_CP, pblobencoding: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateReadOnlyStreamFromBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: ::windows_core::RawPtr, ppstream: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateReadOnlyStreamFromBlob: usize,
    pub CreateDefaultIncludeHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetBlobAsUtf8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: ::windows_core::RawPtr, pblobencoding: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetBlobAsUtf16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: ::windows_core::RawPtr, pblobencoding: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDxilContainerPart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshader: *const DxcBuffer, dxcpart: u32, pppartdata: *mut *mut ::core::ffi::c_void, ppartsizeinbytes: *mut u32) -> ::windows_core::HRESULT,
    pub CreateReflection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const DxcBuffer, iid: *const ::windows_core::GUID, ppvreflection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub BuildArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psourcename: ::windows_core::PCWSTR, pentrypoint: ::windows_core::PCWSTR, ptargetprofile: ::windows_core::PCWSTR, parguments: *const ::windows_core::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, ppargs: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetPDBContents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdbblob: ::windows_core::RawPtr, pphash: *mut ::windows_core::RawPtr, ppcontainer: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDxcValidator(::windows_core::IUnknown);
impl IDxcValidator {
    pub unsafe fn Validate<'a, Param0: ::windows_core::IntoParam<'a, IDxcBlob>>(&self, pshader: Param0, flags: u32) -> ::windows_core::Result<IDxcOperationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Validate)(::windows_core::Interface::as_raw(self), pshader.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcOperationResult>(result__)
    }
}
impl ::core::convert::From<IDxcValidator> for ::windows_core::IUnknown {
    fn from(value: IDxcValidator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcValidator> for ::windows_core::IUnknown {
    fn from(value: &IDxcValidator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDxcValidator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDxcValidator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcValidator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcValidator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcValidator {}
impl ::core::fmt::Debug for IDxcValidator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcValidator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDxcValidator {
    type Vtable = IDxcValidator_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa6e82bd2_1fd7_4826_9811_2857e797f49a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcValidator_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Validate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshader: ::windows_core::RawPtr, flags: u32, ppresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDxcValidator2(::windows_core::IUnknown);
impl IDxcValidator2 {
    pub unsafe fn Validate<'a, Param0: ::windows_core::IntoParam<'a, IDxcBlob>>(&self, pshader: Param0, flags: u32) -> ::windows_core::Result<IDxcOperationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Validate)(::windows_core::Interface::as_raw(self), pshader.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcOperationResult>(result__)
    }
    pub unsafe fn ValidateWithDebug<'a, Param0: ::windows_core::IntoParam<'a, IDxcBlob>>(&self, pshader: Param0, flags: u32, poptdebugbitcode: *const DxcBuffer) -> ::windows_core::Result<IDxcOperationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ValidateWithDebug)(::windows_core::Interface::as_raw(self), pshader.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(poptdebugbitcode), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDxcOperationResult>(result__)
    }
}
impl ::core::convert::From<IDxcValidator2> for ::windows_core::IUnknown {
    fn from(value: IDxcValidator2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcValidator2> for ::windows_core::IUnknown {
    fn from(value: &IDxcValidator2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDxcValidator2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDxcValidator2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDxcValidator2> for IDxcValidator {
    fn from(value: IDxcValidator2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcValidator2> for IDxcValidator {
    fn from(value: &IDxcValidator2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDxcValidator> for IDxcValidator2 {
    fn into_param(self) -> ::windows_core::Param<'a, IDxcValidator> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDxcValidator> for &'a IDxcValidator2 {
    fn into_param(self) -> ::windows_core::Param<'a, IDxcValidator> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcValidator2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcValidator2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcValidator2 {}
impl ::core::fmt::Debug for IDxcValidator2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcValidator2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDxcValidator2 {
    type Vtable = IDxcValidator2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x458e1fd1_b1b2_4750_a6e1_9c10f03bed92);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcValidator2_Vtbl {
    pub base__: IDxcValidator_Vtbl,
    pub ValidateWithDebug: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshader: ::windows_core::RawPtr, flags: u32, poptdebugbitcode: *const DxcBuffer, ppresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDxcVersionInfo(::windows_core::IUnknown);
impl IDxcVersionInfo {
    pub unsafe fn GetVersion(&self, pmajor: *mut u32, pminor: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmajor), ::core::mem::transmute(pminor)).ok()
    }
    pub unsafe fn GetFlags(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IDxcVersionInfo> for ::windows_core::IUnknown {
    fn from(value: IDxcVersionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcVersionInfo> for ::windows_core::IUnknown {
    fn from(value: &IDxcVersionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDxcVersionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDxcVersionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcVersionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcVersionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcVersionInfo {}
impl ::core::fmt::Debug for IDxcVersionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcVersionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDxcVersionInfo {
    type Vtable = IDxcVersionInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb04f5b50_2059_4f12_a8ff_a1e0cde1cc7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcVersionInfo_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmajor: *mut u32, pminor: *mut u32) -> ::windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDxcVersionInfo2(::windows_core::IUnknown);
impl IDxcVersionInfo2 {
    pub unsafe fn GetVersion(&self, pmajor: *mut u32, pminor: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmajor), ::core::mem::transmute(pminor)).ok()
    }
    pub unsafe fn GetFlags(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetCommitInfo(&self, pcommitcount: *mut u32, pcommithash: *mut *mut i8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCommitInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcommitcount), ::core::mem::transmute(pcommithash)).ok()
    }
}
impl ::core::convert::From<IDxcVersionInfo2> for ::windows_core::IUnknown {
    fn from(value: IDxcVersionInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcVersionInfo2> for ::windows_core::IUnknown {
    fn from(value: &IDxcVersionInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDxcVersionInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDxcVersionInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDxcVersionInfo2> for IDxcVersionInfo {
    fn from(value: IDxcVersionInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcVersionInfo2> for IDxcVersionInfo {
    fn from(value: &IDxcVersionInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDxcVersionInfo> for IDxcVersionInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, IDxcVersionInfo> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDxcVersionInfo> for &'a IDxcVersionInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, IDxcVersionInfo> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcVersionInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcVersionInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcVersionInfo2 {}
impl ::core::fmt::Debug for IDxcVersionInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcVersionInfo2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDxcVersionInfo2 {
    type Vtable = IDxcVersionInfo2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb6904c4_42f0_4b62_9c46_983af7da7c83);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcVersionInfo2_Vtbl {
    pub base__: IDxcVersionInfo_Vtbl,
    pub GetCommitInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcommitcount: *mut u32, pcommithash: *mut *mut i8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDxcVersionInfo3(::windows_core::IUnknown);
impl IDxcVersionInfo3 {
    pub unsafe fn GetCustomVersionString(&self) -> ::windows_core::Result<*mut i8> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut i8>::zeroed();
        (::windows_core::Interface::vtable(self).GetCustomVersionString)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut i8>(result__)
    }
}
impl ::core::convert::From<IDxcVersionInfo3> for ::windows_core::IUnknown {
    fn from(value: IDxcVersionInfo3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcVersionInfo3> for ::windows_core::IUnknown {
    fn from(value: &IDxcVersionInfo3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDxcVersionInfo3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDxcVersionInfo3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcVersionInfo3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcVersionInfo3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcVersionInfo3 {}
impl ::core::fmt::Debug for IDxcVersionInfo3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcVersionInfo3").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDxcVersionInfo3 {
    type Vtable = IDxcVersionInfo3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e13e843_9d25_473c_9ad2_03b2d0b44b1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcVersionInfo3_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetCustomVersionString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pversionstring: *mut *mut i8) -> ::windows_core::HRESULT,
}
