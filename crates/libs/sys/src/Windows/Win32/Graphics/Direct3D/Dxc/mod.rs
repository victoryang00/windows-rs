#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub fn DxcCreateInstance(rclsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn DxcCreateInstance2(pmalloc: *mut *mut super::super::super::System::Com::IMalloc, rclsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
pub const CLSID_DxcAssembler: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3609779048, data2: 63747, data3: 20352, data4: [148, 205, 220, 207, 118, 236, 113, 81] };
pub const CLSID_DxcCompiler: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1944202643, data2: 59086, data3: 18419, data4: [181, 191, 240, 102, 79, 57, 193, 176] };
pub const CLSID_DxcCompilerArgs: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1045868162, data2: 8781, data3: 18191, data4: [161, 161, 254, 48, 22, 238, 159, 157] };
pub const CLSID_DxcContainerBuilder: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2484290196, data2: 16671, data3: 17780, data4: [180, 208, 135, 65, 226, 82, 64, 210] };
pub const CLSID_DxcContainerReflection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3119858825, data2: 21944, data3: 16396, data4: [186, 58, 22, 117, 228, 114, 139, 145] };
pub const CLSID_DxcDiaDataSource: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3441388403, data2: 10928, data3: 18509, data4: [142, 220, 235, 231, 164, 60, 160, 159] };
pub const CLSID_DxcLibrary: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1648744111, data2: 26336, data3: 18685, data4: [128, 180, 77, 39, 23, 150, 116, 140] };
pub const CLSID_DxcLinker: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4016734343, data2: 45290, data3: 19798, data4: [158, 69, 208, 126, 26, 139, 120, 6] };
pub const CLSID_DxcOptimizer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2922174367, data2: 52258, data3: 17727, data4: [155, 107, 177, 36, 231, 165, 32, 76] };
pub const CLSID_DxcPdbUtils: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1415716347, data2: 62158, data3: 17790, data4: [174, 140, 236, 53, 95, 174, 236, 124] };
pub const CLSID_DxcValidator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2359550485, data2: 63272, data3: 19699, data4: [140, 221, 136, 175, 145, 117, 135, 161] };
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_ALL_RESOURCES_BOUND: &str = "-all_resources_bound";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_AVOID_FLOW_CONTROL: &str = "-Gfa";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_DEBUG: &str = "-Zi";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_DEBUG_NAME_FOR_BINARY: &str = "-Zsb";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_DEBUG_NAME_FOR_SOURCE: &str = "-Zss";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_ENABLE_BACKWARDS_COMPATIBILITY: &str = "-Gec";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_ENABLE_STRICTNESS: &str = "-Ges";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_IEEE_STRICTNESS: &str = "-Gis";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_OPTIMIZATION_LEVEL0: &str = "-O0";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_OPTIMIZATION_LEVEL1: &str = "-O1";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_OPTIMIZATION_LEVEL2: &str = "-O2";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_OPTIMIZATION_LEVEL3: &str = "-O3";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_PACK_MATRIX_COLUMN_MAJOR: &str = "-Zpc";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_PACK_MATRIX_ROW_MAJOR: &str = "-Zpr";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_PREFER_FLOW_CONTROL: &str = "-Gfp";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_RESOURCES_MAY_ALIAS: &str = "-res_may_alias";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_SKIP_OPTIMIZATIONS: &str = "-Od";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_SKIP_VALIDATION: &str = "-Vd";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_WARNINGS_ARE_ERRORS: &str = "-WX";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub type DXC_CP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_CP_ACP: DXC_CP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_CP_UTF16: DXC_CP = 1200u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_CP_UTF8: DXC_CP = 65001u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_EXTRA_OUTPUT_NAME_STDERR: &str = "*stderr*";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_EXTRA_OUTPUT_NAME_STDOUT: &str = "*stdout*";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_HASHFLAG_INCLUDES_SOURCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub type DXC_OUT_KIND = i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_NONE: DXC_OUT_KIND = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_OBJECT: DXC_OUT_KIND = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_ERRORS: DXC_OUT_KIND = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_PDB: DXC_OUT_KIND = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_SHADER_HASH: DXC_OUT_KIND = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_DISASSEMBLY: DXC_OUT_KIND = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_HLSL: DXC_OUT_KIND = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_TEXT: DXC_OUT_KIND = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_REFLECTION: DXC_OUT_KIND = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_ROOT_SIGNATURE: DXC_OUT_KIND = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_EXTRA_OUTPUTS: DXC_OUT_KIND = 10i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_FORCE_DWORD: DXC_OUT_KIND = -1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub struct DxcArgPair {
    pub pName: ::windows_sys::core::PCWSTR,
    pub pValue: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for DxcArgPair {}
impl ::core::clone::Clone for DxcArgPair {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
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
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub type DxcCreateInstance2Proc = ::core::option::Option<unsafe extern "system" fn(pmalloc: *mut *mut super::super::super::System::Com::IMalloc, rclsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub type DxcCreateInstanceProc = ::core::option::Option<unsafe extern "system" fn(rclsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub struct DxcDefine {
    pub Name: ::windows_sys::core::PCWSTR,
    pub Value: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for DxcDefine {}
impl ::core::clone::Clone for DxcDefine {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
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
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DxcValidatorFlags_Default: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DxcValidatorFlags_InPlaceEdit: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DxcValidatorFlags_ModuleOnly: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DxcValidatorFlags_RootSignatureOnly: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DxcValidatorFlags_ValidMask: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DxcVersionInfoFlags_Debug: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DxcVersionInfoFlags_Internal: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DxcVersionInfoFlags_None: u32 = 0u32;
#[repr(C)]
pub struct IDxcAssembler {
    pub base__: ::windows_sys::core::IUnknown,
    pub AssembleToContainer: unsafe extern "system" fn(this: *mut *mut Self, pshader: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDxcAssembler {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 153057830, data2: 7199, data3: 18760, data4: [144, 75, 230, 227, 168, 167, 113, 213] };
}
#[repr(C)]
pub struct IDxcBlob {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetBufferPointer: unsafe extern "system" fn(this: *mut *mut Self) -> *mut ::core::ffi::c_void,
    pub GetBufferSize: unsafe extern "system" fn(this: *mut *mut Self) -> usize,
}
impl ::windows_sys::core::Interface for IDxcBlob {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2342910728, data2: 20885, data3: 16610, data4: [172, 88, 13, 152, 156, 58, 1, 2] };
}
#[repr(C)]
pub struct IDxcBlobEncoding {
    pub base__: IDxcBlob,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEncoding: unsafe extern "system" fn(this: *mut *mut Self, pknown: *mut super::super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEncoding: usize,
}
impl ::windows_sys::core::Interface for IDxcBlobEncoding {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1916916772, data2: 9798, data3: 16785, data4: [151, 192, 152, 233, 110, 66, 252, 104] };
}
#[repr(C)]
pub struct IDxcBlobUtf16 {
    pub base__: IDxcBlobEncoding,
    pub GetStringPointer: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::PWSTR,
    pub GetStringLength: unsafe extern "system" fn(this: *mut *mut Self) -> usize,
}
impl ::windows_sys::core::Interface for IDxcBlobUtf16 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2750959275, data2: 4010, data3: 18814, data4: [163, 156, 238, 110, 214, 11, 45, 132] };
}
#[repr(C)]
pub struct IDxcBlobUtf8 {
    pub base__: IDxcBlobEncoding,
    pub GetStringPointer: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::PSTR,
    pub GetStringLength: unsafe extern "system" fn(this: *mut *mut Self) -> usize,
}
impl ::windows_sys::core::Interface for IDxcBlobUtf8 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1034303177, data2: 47729, data3: 16420, data4: [163, 1, 48, 203, 241, 37, 48, 91] };
}
#[repr(C)]
pub struct IDxcCompiler {
    pub base__: ::windows_sys::core::IUnknown,
    pub Compile: unsafe extern "system" fn(this: *mut *mut Self, psource: *mut ::core::ffi::c_void, psourcename: ::windows_sys::core::PCWSTR, pentrypoint: ::windows_sys::core::PCWSTR, ptargetprofile: ::windows_sys::core::PCWSTR, parguments: *const ::windows_sys::core::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Preprocess: unsafe extern "system" fn(this: *mut *mut Self, psource: *mut ::core::ffi::c_void, psourcename: ::windows_sys::core::PCWSTR, parguments: *const ::windows_sys::core::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Disassemble: unsafe extern "system" fn(this: *mut *mut Self, psource: *mut ::core::ffi::c_void, ppdisassembly: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDxcCompiler {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2350975987, data2: 287, data3: 17442, data4: [141, 112, 111, 154, 203, 141, 182, 23] };
}
#[repr(C)]
pub struct IDxcCompiler2 {
    pub base__: IDxcCompiler,
    pub CompileWithDebug: unsafe extern "system" fn(this: *mut *mut Self, psource: *mut ::core::ffi::c_void, psourcename: ::windows_sys::core::PCWSTR, pentrypoint: ::windows_sys::core::PCWSTR, ptargetprofile: ::windows_sys::core::PCWSTR, parguments: *const ::windows_sys::core::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void, ppdebugblobname: *mut ::windows_sys::core::PWSTR, ppdebugblob: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDxcCompiler2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2684725721, data2: 47291, data3: 17812, data4: [181, 201, 14, 99, 59, 236, 77, 55] };
}
#[repr(C)]
pub struct IDxcCompiler3 {
    pub base__: ::windows_sys::core::IUnknown,
    pub Compile: unsafe extern "system" fn(this: *mut *mut Self, psource: *const DxcBuffer, parguments: *const ::windows_sys::core::PWSTR, argcount: u32, pincludehandler: *mut ::core::ffi::c_void, riid: *const ::windows_sys::core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Disassemble: unsafe extern "system" fn(this: *mut *mut Self, pobject: *const DxcBuffer, riid: *const ::windows_sys::core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDxcCompiler3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 579552903, data2: 23146, data3: 18224, data4: [144, 12, 151, 2, 178, 32, 63, 84] };
}
#[repr(C)]
pub struct IDxcCompilerArgs {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetArguments: unsafe extern "system" fn(this: *mut *mut Self) -> *mut ::windows_sys::core::PWSTR,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub AddArguments: unsafe extern "system" fn(this: *mut *mut Self, parguments: *const ::windows_sys::core::PWSTR, argcount: u32) -> ::windows_sys::core::HRESULT,
    pub AddArgumentsUTF8: unsafe extern "system" fn(this: *mut *mut Self, parguments: *const ::windows_sys::core::PSTR, argcount: u32) -> ::windows_sys::core::HRESULT,
    pub AddDefines: unsafe extern "system" fn(this: *mut *mut Self, pdefines: *const DxcDefine, definecount: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDxcCompilerArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1945108010, data2: 28892, data3: 17912, data4: [150, 144, 239, 246, 76, 2, 66, 157] };
}
#[repr(C)]
pub struct IDxcContainerBuilder {
    pub base__: ::windows_sys::core::IUnknown,
    pub Load: unsafe extern "system" fn(this: *mut *mut Self, pdxilcontainerheader: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddPart: unsafe extern "system" fn(this: *mut *mut Self, fourcc: u32, psource: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemovePart: unsafe extern "system" fn(this: *mut *mut Self, fourcc: u32) -> ::windows_sys::core::HRESULT,
    pub SerializeContainer: unsafe extern "system" fn(this: *mut *mut Self, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDxcContainerBuilder {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 860561232, data2: 8850, data3: 19253, data4: [153, 161, 37, 88, 141, 140, 23, 254] };
}
#[repr(C)]
pub struct IDxcContainerReflection {
    pub base__: ::windows_sys::core::IUnknown,
    pub Load: unsafe extern "system" fn(this: *mut *mut Self, pcontainer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPartCount: unsafe extern "system" fn(this: *mut *mut Self, presult: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetPartKind: unsafe extern "system" fn(this: *mut *mut Self, idx: u32, presult: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetPartContent: unsafe extern "system" fn(this: *mut *mut Self, idx: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FindFirstPartKind: unsafe extern "system" fn(this: *mut *mut Self, kind: u32, presult: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetPartReflection: unsafe extern "system" fn(this: *mut *mut Self, idx: u32, iid: *const ::windows_sys::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDxcContainerReflection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3535936294, data2: 33616, data3: 19420, data4: [151, 106, 51, 28, 230, 244, 197, 76] };
}
#[repr(C)]
pub struct IDxcExtraOutputs {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetOutputCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetOutput: unsafe extern "system" fn(this: *mut *mut Self, uindex: u32, iid: *const ::windows_sys::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputtype: *mut *mut ::core::ffi::c_void, ppoutputname: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDxcExtraOutputs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 832255906, data2: 42434, data3: 18762, data4: [165, 222, 72, 1, 178, 250, 249, 137] };
}
#[repr(C)]
pub struct IDxcIncludeHandler {
    pub base__: ::windows_sys::core::IUnknown,
    pub LoadSource: unsafe extern "system" fn(this: *mut *mut Self, pfilename: ::windows_sys::core::PCWSTR, ppincludesource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDxcIncludeHandler {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2137128061, data2: 38157, data3: 18047, data4: [179, 227, 60, 2, 251, 73, 24, 124] };
}
#[repr(C)]
pub struct IDxcLibrary {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub SetMalloc: unsafe extern "system" fn(this: *mut *mut Self, pmalloc: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetMalloc: usize,
    pub CreateBlobFromBlob: unsafe extern "system" fn(this: *mut *mut Self, pblob: *mut ::core::ffi::c_void, offset: u32, length: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateBlobFromFile: unsafe extern "system" fn(this: *mut *mut Self, pfilename: ::windows_sys::core::PCWSTR, codepage: *const DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateBlobWithEncodingFromPinned: unsafe extern "system" fn(this: *mut *mut Self, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateBlobWithEncodingOnHeapCopy: unsafe extern "system" fn(this: *mut *mut Self, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateBlobWithEncodingOnMalloc: unsafe extern "system" fn(this: *mut *mut Self, ptext: *const ::core::ffi::c_void, pimalloc: *mut ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateBlobWithEncodingOnMalloc: usize,
    pub CreateIncludeHandler: unsafe extern "system" fn(this: *mut *mut Self, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateStreamFromBlobReadOnly: unsafe extern "system" fn(this: *mut *mut Self, pblob: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateStreamFromBlobReadOnly: usize,
    pub GetBlobAsUtf8: unsafe extern "system" fn(this: *mut *mut Self, pblob: *mut ::core::ffi::c_void, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetBlobAsUtf16: unsafe extern "system" fn(this: *mut *mut Self, pblob: *mut ::core::ffi::c_void, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDxcLibrary {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3844099527, data2: 53644, data3: 19516, data4: [189, 251, 133, 22, 115, 152, 15, 231] };
}
#[repr(C)]
pub struct IDxcLinker {
    pub base__: ::windows_sys::core::IUnknown,
    pub RegisterLibrary: unsafe extern "system" fn(this: *mut *mut Self, plibname: ::windows_sys::core::PCWSTR, plib: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Link: unsafe extern "system" fn(this: *mut *mut Self, pentryname: ::windows_sys::core::PCWSTR, ptargetprofile: ::windows_sys::core::PCWSTR, plibnames: *const ::windows_sys::core::PWSTR, libcount: u32, parguments: *const ::windows_sys::core::PWSTR, argcount: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDxcLinker {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4055219754, data2: 25309, data3: 17191, data4: [161, 194, 66, 172, 30, 30, 120, 230] };
}
#[repr(C)]
pub struct IDxcOperationResult {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, pstatus: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub GetResult: unsafe extern "system" fn(this: *mut *mut Self, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetErrorBuffer: unsafe extern "system" fn(this: *mut *mut Self, pperrors: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDxcOperationResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3470477386, data2: 54505, data3: 17498, data4: [185, 145, 202, 33, 202, 21, 125, 194] };
}
#[repr(C)]
pub struct IDxcOptimizer {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetAvailablePassCount: unsafe extern "system" fn(this: *mut *mut Self, pcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetAvailablePass: unsafe extern "system" fn(this: *mut *mut Self, index: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RunOptimizer: unsafe extern "system" fn(this: *mut *mut Self, pblob: *mut ::core::ffi::c_void, ppoptions: *const ::windows_sys::core::PWSTR, optioncount: u32, poutputmodule: *mut *mut ::core::ffi::c_void, ppoutputtext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDxcOptimizer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 628362798, data2: 40122, data3: 16411, data4: [145, 25, 79, 180, 47, 57, 242, 112] };
}
#[repr(C)]
pub struct IDxcOptimizerPass {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetOptionName: unsafe extern "system" fn(this: *mut *mut Self, ppresult: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(this: *mut *mut Self, ppresult: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetOptionArgCount: unsafe extern "system" fn(this: *mut *mut Self, pcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetOptionArgName: unsafe extern "system" fn(this: *mut *mut Self, argindex: u32, ppresult: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetOptionArgDescription: unsafe extern "system" fn(this: *mut *mut Self, argindex: u32, ppresult: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDxcOptimizerPass {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2922174367, data2: 52258, data3: 17727, data4: [155, 107, 177, 36, 231, 165, 32, 76] };
}
#[repr(C)]
pub struct IDxcPdbUtils {
    pub base__: ::windows_sys::core::IUnknown,
    pub Load: unsafe extern "system" fn(this: *mut *mut Self, ppdbordxil: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSourceCount: unsafe extern "system" fn(this: *mut *mut Self, pcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetSource: unsafe extern "system" fn(this: *mut *mut Self, uindex: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSourceName: unsafe extern "system" fn(this: *mut *mut Self, uindex: u32, presult: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSourceName: usize,
    pub GetFlagCount: unsafe extern "system" fn(this: *mut *mut Self, pcount: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFlag: unsafe extern "system" fn(this: *mut *mut Self, uindex: u32, presult: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFlag: usize,
    pub GetArgCount: unsafe extern "system" fn(this: *mut *mut Self, pcount: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetArg: unsafe extern "system" fn(this: *mut *mut Self, uindex: u32, presult: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetArg: usize,
    pub GetArgPairCount: unsafe extern "system" fn(this: *mut *mut Self, pcount: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetArgPair: unsafe extern "system" fn(this: *mut *mut Self, uindex: u32, pname: *mut super::super::super::Foundation::BSTR, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetArgPair: usize,
    pub GetDefineCount: unsafe extern "system" fn(this: *mut *mut Self, pcount: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDefine: unsafe extern "system" fn(this: *mut *mut Self, uindex: u32, presult: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDefine: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTargetProfile: unsafe extern "system" fn(this: *mut *mut Self, presult: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTargetProfile: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEntryPoint: unsafe extern "system" fn(this: *mut *mut Self, presult: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEntryPoint: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMainFileName: unsafe extern "system" fn(this: *mut *mut Self, presult: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMainFileName: usize,
    pub GetHash: unsafe extern "system" fn(this: *mut *mut Self, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, presult: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsFullPDB: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsFullPDB: usize,
    pub GetFullPDB: unsafe extern "system" fn(this: *mut *mut Self, ppfullpdb: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetVersionInfo: unsafe extern "system" fn(this: *mut *mut Self, ppversioninfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCompiler: unsafe extern "system" fn(this: *mut *mut Self, pcompiler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CompileForFullPDB: unsafe extern "system" fn(this: *mut *mut Self, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OverrideArgs: unsafe extern "system" fn(this: *mut *mut Self, pargpairs: *const DxcArgPair, unumargpairs: u32) -> ::windows_sys::core::HRESULT,
    pub OverrideRootSignature: unsafe extern "system" fn(this: *mut *mut Self, prootsignature: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDxcPdbUtils {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3871958142, data2: 40298, data3: 19515, data4: [185, 76, 82, 75, 90, 108, 52, 61] };
}
#[repr(C)]
pub struct IDxcResult {
    pub base__: IDxcOperationResult,
    #[cfg(feature = "Win32_Foundation")]
    pub HasOutput: unsafe extern "system" fn(this: *mut *mut Self, dxcoutkind: DXC_OUT_KIND) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasOutput: usize,
    pub GetOutput: unsafe extern "system" fn(this: *mut *mut Self, dxcoutkind: DXC_OUT_KIND, iid: *const ::windows_sys::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputname: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetNumOutputs: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetOutputByIndex: unsafe extern "system" fn(this: *mut *mut Self, index: u32) -> DXC_OUT_KIND,
    pub PrimaryOutput: unsafe extern "system" fn(this: *mut *mut Self) -> DXC_OUT_KIND,
}
impl ::windows_sys::core::Interface for IDxcResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1479830746, data2: 56807, data3: 17559, data4: [148, 97, 111, 135, 175, 94, 6, 89] };
}
#[repr(C)]
pub struct IDxcUtils {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateBlobFromBlob: unsafe extern "system" fn(this: *mut *mut Self, pblob: *mut ::core::ffi::c_void, offset: u32, length: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateBlobFromPinned: unsafe extern "system" fn(this: *mut *mut Self, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub MoveToBlob: unsafe extern "system" fn(this: *mut *mut Self, pdata: *const ::core::ffi::c_void, pimalloc: *mut ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    MoveToBlob: usize,
    pub CreateBlob: unsafe extern "system" fn(this: *mut *mut Self, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LoadFile: unsafe extern "system" fn(this: *mut *mut Self, pfilename: ::windows_sys::core::PCWSTR, pcodepage: *const DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateReadOnlyStreamFromBlob: unsafe extern "system" fn(this: *mut *mut Self, pblob: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateReadOnlyStreamFromBlob: usize,
    pub CreateDefaultIncludeHandler: unsafe extern "system" fn(this: *mut *mut Self, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetBlobAsUtf8: unsafe extern "system" fn(this: *mut *mut Self, pblob: *mut ::core::ffi::c_void, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetBlobAsUtf16: unsafe extern "system" fn(this: *mut *mut Self, pblob: *mut ::core::ffi::c_void, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDxilContainerPart: unsafe extern "system" fn(this: *mut *mut Self, pshader: *const DxcBuffer, dxcpart: u32, pppartdata: *mut *mut ::core::ffi::c_void, ppartsizeinbytes: *mut u32) -> ::windows_sys::core::HRESULT,
    pub CreateReflection: unsafe extern "system" fn(this: *mut *mut Self, pdata: *const DxcBuffer, iid: *const ::windows_sys::core::GUID, ppvreflection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BuildArguments: unsafe extern "system" fn(this: *mut *mut Self, psourcename: ::windows_sys::core::PCWSTR, pentrypoint: ::windows_sys::core::PCWSTR, ptargetprofile: ::windows_sys::core::PCWSTR, parguments: *const ::windows_sys::core::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, ppargs: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPDBContents: unsafe extern "system" fn(this: *mut *mut Self, ppdbblob: *mut ::core::ffi::c_void, pphash: *mut *mut ::core::ffi::c_void, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDxcUtils {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1174783179, data2: 8217, data3: 18730, data4: [173, 164, 101, 242, 11, 183, 214, 127] };
}
#[repr(C)]
pub struct IDxcValidator {
    pub base__: ::windows_sys::core::IUnknown,
    pub Validate: unsafe extern "system" fn(this: *mut *mut Self, pshader: *mut ::core::ffi::c_void, flags: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDxcValidator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2800233426, data2: 8151, data3: 18470, data4: [152, 17, 40, 87, 231, 151, 244, 154] };
}
#[repr(C)]
pub struct IDxcValidator2 {
    pub base__: IDxcValidator,
    pub ValidateWithDebug: unsafe extern "system" fn(this: *mut *mut Self, pshader: *mut ::core::ffi::c_void, flags: u32, poptdebugbitcode: *const DxcBuffer, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDxcValidator2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1166942161, data2: 45490, data3: 18256, data4: [166, 225, 156, 16, 240, 59, 237, 146] };
}
#[repr(C)]
pub struct IDxcVersionInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetVersion: unsafe extern "system" fn(this: *mut *mut Self, pmajor: *mut u32, pminor: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut *mut Self, pflags: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDxcVersionInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2957990736, data2: 8281, data3: 20242, data4: [168, 255, 161, 224, 205, 225, 204, 126] };
}
#[repr(C)]
pub struct IDxcVersionInfo2 {
    pub base__: IDxcVersionInfo,
    pub GetCommitInfo: unsafe extern "system" fn(this: *mut *mut Self, pcommitcount: *mut u32, pcommithash: *mut *mut i8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDxcVersionInfo2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4217963716, data2: 17136, data3: 19298, data4: [156, 70, 152, 58, 247, 218, 124, 131] };
}
#[repr(C)]
pub struct IDxcVersionInfo3 {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCustomVersionString: unsafe extern "system" fn(this: *mut *mut Self, pversionstring: *mut *mut i8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDxcVersionInfo3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1578362947, data2: 40229, data3: 18236, data4: [154, 210, 3, 178, 208, 180, 75, 30] };
}
