#[repr(C)]
pub struct APPLETIDLIST {
    pub count: i32,
    pub pIIDList: *mut ::windows_core::GUID,
}
impl ::core::marker::Copy for APPLETIDLIST {}
impl ::core::clone::Clone for APPLETIDLIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for APPLETIDLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APPLETIDLIST").field("count", &self.count).field("pIIDList", &self.pIIDList).finish()
    }
}
unsafe impl ::windows_core::Abi for APPLETIDLIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for APPLETIDLIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<APPLETIDLIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for APPLETIDLIST {}
impl ::core::default::Default for APPLETIDLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct APPLYCANDEXPARAM {
    pub dwSize: u32,
    pub lpwstrDisplay: ::windows_core::PWSTR,
    pub lpwstrReading: ::windows_core::PWSTR,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for APPLYCANDEXPARAM {}
impl ::core::clone::Clone for APPLYCANDEXPARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for APPLYCANDEXPARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APPLYCANDEXPARAM").field("dwSize", &self.dwSize).field("lpwstrDisplay", &self.lpwstrDisplay).field("lpwstrReading", &self.lpwstrReading).field("dwReserved", &self.dwReserved).finish()
    }
}
unsafe impl ::windows_core::Abi for APPLYCANDEXPARAM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for APPLYCANDEXPARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<APPLYCANDEXPARAM>()) == 0 }
    }
}
impl ::core::cmp::Eq for APPLYCANDEXPARAM {}
impl ::core::default::Default for APPLYCANDEXPARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const ATTR_CONVERTED: u32 = 2u32;
pub const ATTR_FIXEDCONVERTED: u32 = 5u32;
pub const ATTR_INPUT: u32 = 0u32;
pub const ATTR_INPUT_ERROR: u32 = 4u32;
pub const ATTR_TARGET_CONVERTED: u32 = 1u32;
pub const ATTR_TARGET_NOTCONVERTED: u32 = 3u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CANDIDATEFORM {
    pub dwIndex: u32,
    pub dwStyle: u32,
    pub ptCurrentPos: super::super::super::Foundation::POINT,
    pub rcArea: super::super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CANDIDATEFORM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CANDIDATEFORM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CANDIDATEFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CANDIDATEFORM").field("dwIndex", &self.dwIndex).field("dwStyle", &self.dwStyle).field("ptCurrentPos", &self.ptCurrentPos).field("rcArea", &self.rcArea).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows_core::Abi for CANDIDATEFORM {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CANDIDATEFORM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CANDIDATEFORM>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CANDIDATEFORM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CANDIDATEFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CANDIDATEINFO {
    pub dwSize: u32,
    pub dwCount: u32,
    pub dwOffset: [u32; 32],
    pub dwPrivateSize: u32,
    pub dwPrivateOffset: u32,
}
impl ::core::marker::Copy for CANDIDATEINFO {}
impl ::core::clone::Clone for CANDIDATEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CANDIDATEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CANDIDATEINFO").field("dwSize", &self.dwSize).field("dwCount", &self.dwCount).field("dwOffset", &self.dwOffset).field("dwPrivateSize", &self.dwPrivateSize).field("dwPrivateOffset", &self.dwPrivateOffset).finish()
    }
}
unsafe impl ::windows_core::Abi for CANDIDATEINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CANDIDATEINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CANDIDATEINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for CANDIDATEINFO {}
impl ::core::default::Default for CANDIDATEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CANDIDATELIST {
    pub dwSize: u32,
    pub dwStyle: u32,
    pub dwCount: u32,
    pub dwSelection: u32,
    pub dwPageStart: u32,
    pub dwPageSize: u32,
    pub dwOffset: [u32; 1],
}
impl ::core::marker::Copy for CANDIDATELIST {}
impl ::core::clone::Clone for CANDIDATELIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CANDIDATELIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CANDIDATELIST").field("dwSize", &self.dwSize).field("dwStyle", &self.dwStyle).field("dwCount", &self.dwCount).field("dwSelection", &self.dwSelection).field("dwPageStart", &self.dwPageStart).field("dwPageSize", &self.dwPageSize).field("dwOffset", &self.dwOffset).finish()
    }
}
unsafe impl ::windows_core::Abi for CANDIDATELIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CANDIDATELIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CANDIDATELIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for CANDIDATELIST {}
impl ::core::default::Default for CANDIDATELIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const CATID_MSIME_IImePadApplet: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7566cad1_4ec9_4478_9fe9_8ed766619edf);
pub const CATID_MSIME_IImePadApplet1000: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe081e1d6_2389_43cb_b66f_609f823d9f9c);
pub const CATID_MSIME_IImePadApplet1200: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa47fb5fc_7d15_4223_a789_b781bf9ae667);
pub const CATID_MSIME_IImePadApplet900: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfaae51bf_5e5b_4a1d_8de1_17c1d9e1728d);
pub const CATID_MSIME_IImePadApplet_VER7: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4a0f8e31_c3ee_11d1_afef_00805f0c8b6d);
pub const CATID_MSIME_IImePadApplet_VER80: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x56f7a792_fef1_11d3_8463_00c04f7a06e5);
pub const CATID_MSIME_IImePadApplet_VER81: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x656520b0_bb88_11d4_84c0_00c04f7a06e5);
pub const CActiveIMM: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4955dd33_b159_11d0_8fcf_00aa006bcc59);
pub const CFS_CANDIDATEPOS: u32 = 64u32;
pub const CFS_DEFAULT: u32 = 0u32;
pub const CFS_EXCLUDE: u32 = 128u32;
pub const CFS_FORCE_POSITION: u32 = 32u32;
pub const CFS_POINT: u32 = 2u32;
pub const CFS_RECT: u32 = 1u32;
pub const CHARINFO_APPLETID_MASK: u32 = 4278190080u32;
pub const CHARINFO_CHARID_MASK: u32 = 65535u32;
pub const CHARINFO_FEID_MASK: u32 = 15728640u32;
pub const CLSID_ImePlugInDictDictionaryList_CHS: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7bf0129b_5bef_4de4_9b0b_5edb66ac2fa6);
pub const CLSID_ImePlugInDictDictionaryList_JPN: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4fe2776b_b0f9_4396_b5fc_e9d4cf1ec195);
pub const CLSID_VERSION_DEPENDENT_MSIME_JAPANESE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6a91029e_aa49_471b_aee7_7d332785660d);
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct COMPOSITIONFORM {
    pub dwStyle: u32,
    pub ptCurrentPos: super::super::super::Foundation::POINT,
    pub rcArea: super::super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COMPOSITIONFORM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COMPOSITIONFORM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COMPOSITIONFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPOSITIONFORM").field("dwStyle", &self.dwStyle).field("ptCurrentPos", &self.ptCurrentPos).field("rcArea", &self.rcArea).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows_core::Abi for COMPOSITIONFORM {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COMPOSITIONFORM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COMPOSITIONFORM>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COMPOSITIONFORM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COMPOSITIONFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct COMPOSITIONSTRING {
    pub dwSize: u32,
    pub dwCompReadAttrLen: u32,
    pub dwCompReadAttrOffset: u32,
    pub dwCompReadClauseLen: u32,
    pub dwCompReadClauseOffset: u32,
    pub dwCompReadStrLen: u32,
    pub dwCompReadStrOffset: u32,
    pub dwCompAttrLen: u32,
    pub dwCompAttrOffset: u32,
    pub dwCompClauseLen: u32,
    pub dwCompClauseOffset: u32,
    pub dwCompStrLen: u32,
    pub dwCompStrOffset: u32,
    pub dwCursorPos: u32,
    pub dwDeltaStart: u32,
    pub dwResultReadClauseLen: u32,
    pub dwResultReadClauseOffset: u32,
    pub dwResultReadStrLen: u32,
    pub dwResultReadStrOffset: u32,
    pub dwResultClauseLen: u32,
    pub dwResultClauseOffset: u32,
    pub dwResultStrLen: u32,
    pub dwResultStrOffset: u32,
    pub dwPrivateSize: u32,
    pub dwPrivateOffset: u32,
}
impl ::core::marker::Copy for COMPOSITIONSTRING {}
impl ::core::clone::Clone for COMPOSITIONSTRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMPOSITIONSTRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPOSITIONSTRING")
            .field("dwSize", &self.dwSize)
            .field("dwCompReadAttrLen", &self.dwCompReadAttrLen)
            .field("dwCompReadAttrOffset", &self.dwCompReadAttrOffset)
            .field("dwCompReadClauseLen", &self.dwCompReadClauseLen)
            .field("dwCompReadClauseOffset", &self.dwCompReadClauseOffset)
            .field("dwCompReadStrLen", &self.dwCompReadStrLen)
            .field("dwCompReadStrOffset", &self.dwCompReadStrOffset)
            .field("dwCompAttrLen", &self.dwCompAttrLen)
            .field("dwCompAttrOffset", &self.dwCompAttrOffset)
            .field("dwCompClauseLen", &self.dwCompClauseLen)
            .field("dwCompClauseOffset", &self.dwCompClauseOffset)
            .field("dwCompStrLen", &self.dwCompStrLen)
            .field("dwCompStrOffset", &self.dwCompStrOffset)
            .field("dwCursorPos", &self.dwCursorPos)
            .field("dwDeltaStart", &self.dwDeltaStart)
            .field("dwResultReadClauseLen", &self.dwResultReadClauseLen)
            .field("dwResultReadClauseOffset", &self.dwResultReadClauseOffset)
            .field("dwResultReadStrLen", &self.dwResultReadStrLen)
            .field("dwResultReadStrOffset", &self.dwResultReadStrOffset)
            .field("dwResultClauseLen", &self.dwResultClauseLen)
            .field("dwResultClauseOffset", &self.dwResultClauseOffset)
            .field("dwResultStrLen", &self.dwResultStrLen)
            .field("dwResultStrOffset", &self.dwResultStrOffset)
            .field("dwPrivateSize", &self.dwPrivateSize)
            .field("dwPrivateOffset", &self.dwPrivateOffset)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for COMPOSITIONSTRING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COMPOSITIONSTRING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COMPOSITIONSTRING>()) == 0 }
    }
}
impl ::core::cmp::Eq for COMPOSITIONSTRING {}
impl ::core::default::Default for COMPOSITIONSTRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const CS_INSERTCHAR: u32 = 8192u32;
pub const CS_NOMOVECARET: u32 = 16384u32;
pub const E_LARGEINPUT: u32 = 51u32;
pub const E_NOCAND: u32 = 48u32;
pub const E_NOTENOUGH_BUFFER: u32 = 49u32;
pub const E_NOTENOUGH_WDD: u32 = 50u32;
pub const FEID_CHINESE_HONGKONG: u32 = 3u32;
pub const FEID_CHINESE_SIMPLIFIED: u32 = 2u32;
pub const FEID_CHINESE_SINGAPORE: u32 = 4u32;
pub const FEID_CHINESE_TRADITIONAL: u32 = 1u32;
pub const FEID_JAPANESE: u32 = 5u32;
pub const FEID_KOREAN: u32 = 6u32;
pub const FEID_KOREAN_JOHAB: u32 = 7u32;
pub const FEID_NONE: u32 = 0u32;
pub const FELANG_CLMN_FIXD: u32 = 32u32;
pub const FELANG_CLMN_FIXR: u32 = 16u32;
pub const FELANG_CLMN_NOPBREAK: u32 = 8u32;
pub const FELANG_CLMN_NOWBREAK: u32 = 2u32;
pub const FELANG_CLMN_PBREAK: u32 = 4u32;
pub const FELANG_CLMN_WBREAK: u32 = 1u32;
pub const FELANG_CMODE_AUTOMATIC: u32 = 134217728u32;
pub const FELANG_CMODE_BESTFIRST: u32 = 16384u32;
pub const FELANG_CMODE_BOPOMOFO: u32 = 64u32;
pub const FELANG_CMODE_CONVERSATION: u32 = 536870912u32;
pub const FELANG_CMODE_FULLWIDTHOUT: u32 = 32u32;
pub const FELANG_CMODE_HALFWIDTHOUT: u32 = 16u32;
pub const FELANG_CMODE_HANGUL: u32 = 128u32;
pub const FELANG_CMODE_HIRAGANAOUT: u32 = 0u32;
pub const FELANG_CMODE_KATAKANAOUT: u32 = 8u32;
pub const FELANG_CMODE_MERGECAND: u32 = 4096u32;
pub const FELANG_CMODE_MONORUBY: u32 = 2u32;
pub const FELANG_CMODE_NAME: u32 = 268435456u32;
pub const FELANG_CMODE_NOINVISIBLECHAR: u32 = 1073741824u32;
pub const FELANG_CMODE_NONE: u32 = 16777216u32;
pub const FELANG_CMODE_NOPRUNING: u32 = 4u32;
pub const FELANG_CMODE_PHRASEPREDICT: u32 = 268435456u32;
pub const FELANG_CMODE_PINYIN: u32 = 256u32;
pub const FELANG_CMODE_PLAURALCLAUSE: u32 = 33554432u32;
pub const FELANG_CMODE_PRECONV: u32 = 512u32;
pub const FELANG_CMODE_RADICAL: u32 = 1024u32;
pub const FELANG_CMODE_ROMAN: u32 = 8192u32;
pub const FELANG_CMODE_SINGLECONVERT: u32 = 67108864u32;
pub const FELANG_CMODE_UNKNOWNREADING: u32 = 2048u32;
pub const FELANG_CMODE_USENOREVWORDS: u32 = 32768u32;
pub const FELANG_INVALD_PO: u32 = 65535u32;
pub const FELANG_REQ_CONV: u32 = 65536u32;
pub const FELANG_REQ_RECONV: u32 = 131072u32;
pub const FELANG_REQ_REV: u32 = 196608u32;
pub const FID_MSIME_KMS_DEL_KEYLIST: u32 = 4u32;
pub const FID_MSIME_KMS_FUNCDESC: u32 = 9u32;
pub const FID_MSIME_KMS_GETMAP: u32 = 6u32;
pub const FID_MSIME_KMS_GETMAPFAST: u32 = 11u32;
pub const FID_MSIME_KMS_GETMAPSEAMLESS: u32 = 10u32;
pub const FID_MSIME_KMS_INIT: u32 = 2u32;
pub const FID_MSIME_KMS_INVOKE: u32 = 7u32;
pub const FID_MSIME_KMS_NOTIFY: u32 = 5u32;
pub const FID_MSIME_KMS_SETMAP: u32 = 8u32;
pub const FID_MSIME_KMS_TERM: u32 = 3u32;
pub const FID_MSIME_KMS_VERSION: u32 = 1u32;
pub const FID_MSIME_VERSION: u32 = 0u32;
pub const FID_RECONVERT_VERSION: u32 = 268435456u32;
pub const GCSEX_CANCELRECONVERT: u32 = 268435456u32;
pub const GCS_COMPATTR: u32 = 16u32;
pub const GCS_COMPCLAUSE: u32 = 32u32;
pub const GCS_COMPREADATTR: u32 = 2u32;
pub const GCS_COMPREADCLAUSE: u32 = 4u32;
pub const GCS_COMPREADSTR: u32 = 1u32;
pub const GCS_COMPSTR: u32 = 8u32;
pub const GCS_CURSORPOS: u32 = 128u32;
pub const GCS_DELTASTART: u32 = 256u32;
pub const GCS_RESULTCLAUSE: u32 = 4096u32;
pub const GCS_RESULTREADCLAUSE: u32 = 1024u32;
pub const GCS_RESULTREADSTR: u32 = 512u32;
pub const GCS_RESULTSTR: u32 = 2048u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GET_CONVERSION_LIST_FLAG(pub u32);
pub const GCL_CONVERSION: GET_CONVERSION_LIST_FLAG = GET_CONVERSION_LIST_FLAG(1u32);
pub const GCL_REVERSECONVERSION: GET_CONVERSION_LIST_FLAG = GET_CONVERSION_LIST_FLAG(2u32);
pub const GCL_REVERSE_LENGTH: GET_CONVERSION_LIST_FLAG = GET_CONVERSION_LIST_FLAG(3u32);
impl ::core::marker::Copy for GET_CONVERSION_LIST_FLAG {}
impl ::core::clone::Clone for GET_CONVERSION_LIST_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_CONVERSION_LIST_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GET_CONVERSION_LIST_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for GET_CONVERSION_LIST_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_CONVERSION_LIST_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GET_GUIDE_LINE_TYPE(pub u32);
pub const GGL_LEVEL: GET_GUIDE_LINE_TYPE = GET_GUIDE_LINE_TYPE(1u32);
pub const GGL_INDEX: GET_GUIDE_LINE_TYPE = GET_GUIDE_LINE_TYPE(2u32);
pub const GGL_STRING: GET_GUIDE_LINE_TYPE = GET_GUIDE_LINE_TYPE(3u32);
pub const GGL_PRIVATE: GET_GUIDE_LINE_TYPE = GET_GUIDE_LINE_TYPE(4u32);
impl ::core::marker::Copy for GET_GUIDE_LINE_TYPE {}
impl ::core::clone::Clone for GET_GUIDE_LINE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_GUIDE_LINE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GET_GUIDE_LINE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for GET_GUIDE_LINE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_GUIDE_LINE_TYPE").field(&self.0).finish()
    }
}
pub const GL_ID_CANNOTSAVE: u32 = 17u32;
pub const GL_ID_CHOOSECANDIDATE: u32 = 40u32;
pub const GL_ID_INPUTCODE: u32 = 38u32;
pub const GL_ID_INPUTRADICAL: u32 = 37u32;
pub const GL_ID_INPUTREADING: u32 = 36u32;
pub const GL_ID_INPUTSYMBOL: u32 = 39u32;
pub const GL_ID_NOCONVERT: u32 = 32u32;
pub const GL_ID_NODICTIONARY: u32 = 16u32;
pub const GL_ID_NOMODULE: u32 = 1u32;
pub const GL_ID_PRIVATE_FIRST: u32 = 32768u32;
pub const GL_ID_PRIVATE_LAST: u32 = 65535u32;
pub const GL_ID_READINGCONFLICT: u32 = 35u32;
pub const GL_ID_REVERSECONVERSION: u32 = 41u32;
pub const GL_ID_TOOMANYSTROKE: u32 = 34u32;
pub const GL_ID_TYPINGERROR: u32 = 33u32;
pub const GL_ID_UNKNOWN: u32 = 0u32;
pub const GL_LEVEL_ERROR: u32 = 2u32;
pub const GL_LEVEL_FATAL: u32 = 1u32;
pub const GL_LEVEL_INFORMATION: u32 = 4u32;
pub const GL_LEVEL_NOGUIDELINE: u32 = 0u32;
pub const GL_LEVEL_WARNING: u32 = 3u32;
#[repr(C)]
pub struct GUIDELINE {
    pub dwSize: u32,
    pub dwLevel: u32,
    pub dwIndex: u32,
    pub dwStrLen: u32,
    pub dwStrOffset: u32,
    pub dwPrivateSize: u32,
    pub dwPrivateOffset: u32,
}
impl ::core::marker::Copy for GUIDELINE {}
impl ::core::clone::Clone for GUIDELINE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GUIDELINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GUIDELINE").field("dwSize", &self.dwSize).field("dwLevel", &self.dwLevel).field("dwIndex", &self.dwIndex).field("dwStrLen", &self.dwStrLen).field("dwStrOffset", &self.dwStrOffset).field("dwPrivateSize", &self.dwPrivateSize).field("dwPrivateOffset", &self.dwPrivateOffset).finish()
    }
}
unsafe impl ::windows_core::Abi for GUIDELINE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GUIDELINE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GUIDELINE>()) == 0 }
    }
}
impl ::core::cmp::Eq for GUIDELINE {}
impl ::core::default::Default for GUIDELINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const IACE_CHILDREN: u32 = 1u32;
pub const IACE_DEFAULT: u32 = 16u32;
pub const IACE_IGNORENOCONTEXT: u32 = 32u32;
#[repr(transparent)]
pub struct IActiveIME(::windows_core::IUnknown);
impl IActiveIME {
    pub unsafe fn Inquire(&self, dwsysteminfoflags: u32, pimeinfo: *mut IMEINFO, szwndclass: ::windows_core::PWSTR, pdwprivate: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Inquire)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwsysteminfoflags), ::core::mem::transmute(pimeinfo), ::core::mem::transmute(szwndclass), ::core::mem::transmute(pdwprivate)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn ConversionList<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, himc: Param0, szsource: Param1, uflag: u32, ubuflen: u32, pdest: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ConversionList)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), szsource.into_param().abi(), ::core::mem::transmute(uflag), ::core::mem::transmute(ubuflen), ::core::mem::transmute(pdest), ::core::mem::transmute(pucopied)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn Configure<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hkl: Param0, hwnd: Param1, dwmode: u32, pregisterword: *const REGISTERWORDW) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Configure)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), hwnd.into_param().abi(), ::core::mem::transmute(dwmode), ::core::mem::transmute(pregisterword)).ok()
    }
    pub unsafe fn Destroy(&self, ureserved: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Destroy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ureserved)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn Escape<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Escape)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(uescape), ::core::mem::transmute(pdata), ::core::mem::transmute(plresult)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn SetActiveContext<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, himc: Param0, fflag: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetActiveContext)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), fflag.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn ProcessKey<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, uvirkey: u32, lparam: u32, pbkeystate: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ProcessKey)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(uvirkey), ::core::mem::transmute(lparam), ::core::mem::transmute(pbkeystate)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn Notify<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Notify)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(dwaction), ::core::mem::transmute(dwindex), ::core::mem::transmute(dwvalue)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn Select<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, himc: Param0, fselect: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Select)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), fselect.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn SetCompositionString<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCompositionString)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(pcomp), ::core::mem::transmute(dwcomplen), ::core::mem::transmute(pread), ::core::mem::transmute(dwreadlen)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn ToAsciiEx<'a, Param4: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, uvirkey: u32, uscancode: u32, pbkeystate: *const u8, fustate: u32, himc: Param4, pdwtransbuf: *mut u32, pusize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ToAsciiEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uvirkey), ::core::mem::transmute(uscancode), ::core::mem::transmute(pbkeystate), ::core::mem::transmute(fustate), himc.into_param().abi(), ::core::mem::transmute(pdwtransbuf), ::core::mem::transmute(pusize)).ok()
    }
    pub unsafe fn RegisterWord<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, szreading: Param0, dwstyle: u32, szstring: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterWord)(::windows_core::Interface::as_raw(self), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szstring.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterWord<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, szreading: Param0, dwstyle: u32, szstring: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnregisterWord)(::windows_core::Interface::as_raw(self), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szstring.into_param().abi()).ok()
    }
    pub unsafe fn GetRegisterWordStyle(&self, nitem: u32, pstylebuf: *mut STYLEBUFW, pubufsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRegisterWordStyle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(nitem), ::core::mem::transmute(pstylebuf), ::core::mem::transmute(pubufsize)).ok()
    }
    pub unsafe fn EnumRegisterWord<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, szreading: Param0, dwstyle: u32, szregister: Param2, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<IEnumRegisterWordW> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumRegisterWord)(::windows_core::Interface::as_raw(self), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szregister.into_param().abi(), ::core::mem::transmute(pdata), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumRegisterWordW>(result__)
    }
    pub unsafe fn GetCodePageA(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetCodePageA)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetLangId(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).GetLangId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
}
impl ::core::convert::From<IActiveIME> for ::windows_core::IUnknown {
    fn from(value: IActiveIME) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActiveIME> for ::windows_core::IUnknown {
    fn from(value: &IActiveIME) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IActiveIME {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IActiveIME {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IActiveIME {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActiveIME {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveIME {}
impl ::core::fmt::Debug for IActiveIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveIME").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IActiveIME {
    type Vtable = IActiveIME_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6fe20962_d077_11d0_8fe7_00aa006bcc59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveIME_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Inquire: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsysteminfoflags: u32, pimeinfo: *mut IMEINFO, szwndclass: ::windows_core::PWSTR, pdwprivate: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Globalization")]
    pub ConversionList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, szsource: ::windows_core::PCWSTR, uflag: u32, ubuflen: u32, pdest: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    ConversionList: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub Configure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pregisterword: *const REGISTERWORDW) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))]
    Configure: usize,
    pub Destroy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ureserved: u32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub Escape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    Escape: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub SetActiveContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fflag: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    SetActiveContext: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub ProcessKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, uvirkey: u32, lparam: u32, pbkeystate: *const u8) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    ProcessKey: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub Notify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    Notify: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub Select: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fselect: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    Select: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub SetCompositionString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    SetCompositionString: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub ToAsciiEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uvirkey: u32, uscancode: u32, pbkeystate: *const u8, fustate: u32, himc: super::super::super::Globalization::HIMC, pdwtransbuf: *mut u32, pusize: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    ToAsciiEx: usize,
    pub RegisterWord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szreading: ::windows_core::PCWSTR, dwstyle: u32, szstring: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub UnregisterWord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szreading: ::windows_core::PCWSTR, dwstyle: u32, szstring: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetRegisterWordStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nitem: u32, pstylebuf: *mut STYLEBUFW, pubufsize: *mut u32) -> ::windows_core::HRESULT,
    pub EnumRegisterWord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szreading: ::windows_core::PCWSTR, dwstyle: u32, szregister: ::windows_core::PCWSTR, pdata: *const ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetCodePageA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ucodepage: *mut u32) -> ::windows_core::HRESULT,
    pub GetLangId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plid: *mut u16) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IActiveIME2(::windows_core::IUnknown);
impl IActiveIME2 {
    pub unsafe fn Inquire(&self, dwsysteminfoflags: u32, pimeinfo: *mut IMEINFO, szwndclass: ::windows_core::PWSTR, pdwprivate: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Inquire)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwsysteminfoflags), ::core::mem::transmute(pimeinfo), ::core::mem::transmute(szwndclass), ::core::mem::transmute(pdwprivate)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn ConversionList<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, himc: Param0, szsource: Param1, uflag: u32, ubuflen: u32, pdest: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ConversionList)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), szsource.into_param().abi(), ::core::mem::transmute(uflag), ::core::mem::transmute(ubuflen), ::core::mem::transmute(pdest), ::core::mem::transmute(pucopied)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn Configure<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hkl: Param0, hwnd: Param1, dwmode: u32, pregisterword: *const REGISTERWORDW) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Configure)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), hwnd.into_param().abi(), ::core::mem::transmute(dwmode), ::core::mem::transmute(pregisterword)).ok()
    }
    pub unsafe fn Destroy(&self, ureserved: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Destroy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ureserved)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn Escape<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Escape)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(uescape), ::core::mem::transmute(pdata), ::core::mem::transmute(plresult)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn SetActiveContext<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, himc: Param0, fflag: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetActiveContext)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), fflag.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn ProcessKey<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, uvirkey: u32, lparam: u32, pbkeystate: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ProcessKey)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(uvirkey), ::core::mem::transmute(lparam), ::core::mem::transmute(pbkeystate)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn Notify<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Notify)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(dwaction), ::core::mem::transmute(dwindex), ::core::mem::transmute(dwvalue)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn Select<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, himc: Param0, fselect: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Select)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), fselect.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn SetCompositionString<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetCompositionString)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(pcomp), ::core::mem::transmute(dwcomplen), ::core::mem::transmute(pread), ::core::mem::transmute(dwreadlen)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn ToAsciiEx<'a, Param4: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, uvirkey: u32, uscancode: u32, pbkeystate: *const u8, fustate: u32, himc: Param4, pdwtransbuf: *mut u32, pusize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ToAsciiEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uvirkey), ::core::mem::transmute(uscancode), ::core::mem::transmute(pbkeystate), ::core::mem::transmute(fustate), himc.into_param().abi(), ::core::mem::transmute(pdwtransbuf), ::core::mem::transmute(pusize)).ok()
    }
    pub unsafe fn RegisterWord<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, szreading: Param0, dwstyle: u32, szstring: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RegisterWord)(::windows_core::Interface::as_raw(self), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szstring.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterWord<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, szreading: Param0, dwstyle: u32, szstring: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.UnregisterWord)(::windows_core::Interface::as_raw(self), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szstring.into_param().abi()).ok()
    }
    pub unsafe fn GetRegisterWordStyle(&self, nitem: u32, pstylebuf: *mut STYLEBUFW, pubufsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRegisterWordStyle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(nitem), ::core::mem::transmute(pstylebuf), ::core::mem::transmute(pubufsize)).ok()
    }
    pub unsafe fn EnumRegisterWord<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, szreading: Param0, dwstyle: u32, szregister: Param2, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<IEnumRegisterWordW> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumRegisterWord)(::windows_core::Interface::as_raw(self), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szregister.into_param().abi(), ::core::mem::transmute(pdata), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumRegisterWordW>(result__)
    }
    pub unsafe fn GetCodePageA(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCodePageA)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetLangId(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLangId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn Sleep(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Sleep)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unsleep<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fdead: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Unsleep)(::windows_core::Interface::as_raw(self), fdead.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IActiveIME2> for ::windows_core::IUnknown {
    fn from(value: IActiveIME2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActiveIME2> for ::windows_core::IUnknown {
    fn from(value: &IActiveIME2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IActiveIME2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IActiveIME2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IActiveIME2> for IActiveIME {
    fn from(value: IActiveIME2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActiveIME2> for IActiveIME {
    fn from(value: &IActiveIME2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActiveIME> for IActiveIME2 {
    fn into_param(self) -> ::windows_core::Param<'a, IActiveIME> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActiveIME> for &'a IActiveIME2 {
    fn into_param(self) -> ::windows_core::Param<'a, IActiveIME> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IActiveIME2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActiveIME2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveIME2 {}
impl ::core::fmt::Debug for IActiveIME2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveIME2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IActiveIME2 {
    type Vtable = IActiveIME2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe1c4bf0e_2d53_11d2_93e1_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveIME2_Vtbl {
    pub base__: IActiveIME_Vtbl,
    pub Sleep: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Unsleep: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fdead: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Unsleep: usize,
}
#[repr(transparent)]
pub struct IActiveIMMApp(::windows_core::IUnknown);
impl IActiveIMMApp {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn AssociateContext<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>, Param1: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, hwnd: Param0, hime: Param1) -> ::windows_core::Result<super::super::super::Globalization::HIMC> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Globalization::HIMC>::zeroed();
        (::windows_core::Interface::vtable(self).AssociateContext)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), hime.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Globalization::HIMC>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn ConfigureIMEA<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hkl: Param0, hwnd: Param1, dwmode: u32, pdata: *const REGISTERWORDA) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ConfigureIMEA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), hwnd.into_param().abi(), ::core::mem::transmute(dwmode), ::core::mem::transmute(pdata)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn ConfigureIMEW<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hkl: Param0, hwnd: Param1, dwmode: u32, pdata: *const REGISTERWORDW) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ConfigureIMEW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), hwnd.into_param().abi(), ::core::mem::transmute(dwmode), ::core::mem::transmute(pdata)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn CreateContext(&self) -> ::windows_core::Result<super::super::super::Globalization::HIMC> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Globalization::HIMC>::zeroed();
        (::windows_core::Interface::vtable(self).CreateContext)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Globalization::HIMC>(result__)
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn DestroyContext<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, hime: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DestroyContext)(::windows_core::Interface::as_raw(self), hime.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn EnumRegisterWordA<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(&self, hkl: Param0, szreading: Param1, dwstyle: u32, szregister: Param3, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<IEnumRegisterWordA> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumRegisterWordA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szregister.into_param().abi(), ::core::mem::transmute(pdata), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumRegisterWordA>(result__)
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn EnumRegisterWordW<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, hkl: Param0, szreading: Param1, dwstyle: u32, szregister: Param3, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<IEnumRegisterWordW> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumRegisterWordW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szregister.into_param().abi(), ::core::mem::transmute(pdata), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumRegisterWordW>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub unsafe fn EscapeA<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, hkl: Param0, himc: Param1, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EscapeA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), himc.into_param().abi(), ::core::mem::transmute(uescape), ::core::mem::transmute(pdata), ::core::mem::transmute(plresult)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub unsafe fn EscapeW<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, hkl: Param0, himc: Param1, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EscapeW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), himc.into_param().abi(), ::core::mem::transmute(uescape), ::core::mem::transmute(pdata), ::core::mem::transmute(plresult)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCandidateListA<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCandidateListA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(ubuflen), ::core::mem::transmute(pcandlist), ::core::mem::transmute(pucopied)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCandidateListW<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCandidateListW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(ubuflen), ::core::mem::transmute(pcandlist), ::core::mem::transmute(pucopied)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCandidateListCountA<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCandidateListCountA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(pdwlistsize), ::core::mem::transmute(pdwbuflen)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCandidateListCountW<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCandidateListCountW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(pdwlistsize), ::core::mem::transmute(pdwbuflen)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn GetCandidateWindow<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32) -> ::windows_core::Result<CANDIDATEFORM> {
        let mut result__ = ::core::mem::MaybeUninit::<CANDIDATEFORM>::zeroed();
        (::windows_core::Interface::vtable(self).GetCandidateWindow)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<CANDIDATEFORM>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetCompositionFontA<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) -> ::windows_core::Result<super::super::super::Graphics::Gdi::LOGFONTA> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Graphics::Gdi::LOGFONTA>::zeroed();
        (::windows_core::Interface::vtable(self).GetCompositionFontA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Graphics::Gdi::LOGFONTA>(result__)
    }
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetCompositionFontW<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) -> ::windows_core::Result<super::super::super::Graphics::Gdi::LOGFONTW> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Graphics::Gdi::LOGFONTW>::zeroed();
        (::windows_core::Interface::vtable(self).GetCompositionFontW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Graphics::Gdi::LOGFONTW>(result__)
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCompositionStringA<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCompositionStringA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(dwbuflen), ::core::mem::transmute(plcopied), ::core::mem::transmute(pbuf)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCompositionStringW<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCompositionStringW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(dwbuflen), ::core::mem::transmute(plcopied), ::core::mem::transmute(pbuf)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn GetCompositionWindow<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) -> ::windows_core::Result<COMPOSITIONFORM> {
        let mut result__ = ::core::mem::MaybeUninit::<COMPOSITIONFORM>::zeroed();
        (::windows_core::Interface::vtable(self).GetCompositionWindow)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<COMPOSITIONFORM>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn GetContext<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwnd: Param0) -> ::windows_core::Result<super::super::super::Globalization::HIMC> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Globalization::HIMC>::zeroed();
        (::windows_core::Interface::vtable(self).GetContext)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Globalization::HIMC>(result__)
    }
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub unsafe fn GetConversionListA<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(&self, hkl: Param0, himc: Param1, psrc: Param2, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetConversionListA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), himc.into_param().abi(), psrc.into_param().abi(), ::core::mem::transmute(ubuflen), ::core::mem::transmute(uflag), ::core::mem::transmute(pdst), ::core::mem::transmute(pucopied)).ok()
    }
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub unsafe fn GetConversionListW<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, hkl: Param0, himc: Param1, psrc: Param2, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetConversionListW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), himc.into_param().abi(), psrc.into_param().abi(), ::core::mem::transmute(ubuflen), ::core::mem::transmute(uflag), ::core::mem::transmute(pdst), ::core::mem::transmute(pucopied)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetConversionStatus<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetConversionStatus)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(pfdwconversion), ::core::mem::transmute(pfdwsentence)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDefaultIMEWnd<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwnd: Param0) -> ::windows_core::Result<super::super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::HWND>::zeroed();
        (::windows_core::Interface::vtable(self).GetDefaultIMEWnd)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::HWND>(result__)
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetDescriptionA<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0, ubuflen: u32, szdescription: ::windows_core::PSTR, pucopied: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDescriptionA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), ::core::mem::transmute(ubuflen), ::core::mem::transmute(szdescription), ::core::mem::transmute(pucopied)).ok()
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetDescriptionW<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0, ubuflen: u32, szdescription: ::windows_core::PWSTR, pucopied: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDescriptionW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), ::core::mem::transmute(ubuflen), ::core::mem::transmute(szdescription), ::core::mem::transmute(pucopied)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetGuideLineA<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, dwbuflen: u32, pbuf: ::windows_core::PSTR, pdwresult: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetGuideLineA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(dwbuflen), ::core::mem::transmute(pbuf), ::core::mem::transmute(pdwresult)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetGuideLineW<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, dwbuflen: u32, pbuf: ::windows_core::PWSTR, pdwresult: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetGuideLineW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(dwbuflen), ::core::mem::transmute(pbuf), ::core::mem::transmute(pdwresult)).ok()
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetIMEFileNameA<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0, ubuflen: u32, szfilename: ::windows_core::PSTR, pucopied: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetIMEFileNameA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), ::core::mem::transmute(ubuflen), ::core::mem::transmute(szfilename), ::core::mem::transmute(pucopied)).ok()
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetIMEFileNameW<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0, ubuflen: u32, szfilename: ::windows_core::PWSTR, pucopied: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetIMEFileNameW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), ::core::mem::transmute(ubuflen), ::core::mem::transmute(szfilename), ::core::mem::transmute(pucopied)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetOpenStatus<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetOpenStatus)(::windows_core::Interface::as_raw(self), himc.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetProperty<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0, fdwindex: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), ::core::mem::transmute(fdwindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn GetRegisterWordStyleA<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRegisterWordStyleA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), ::core::mem::transmute(nitem), ::core::mem::transmute(pstylebuf), ::core::mem::transmute(pucopied)).ok()
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetRegisterWordStyleW<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRegisterWordStyleW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), ::core::mem::transmute(nitem), ::core::mem::transmute(pstylebuf), ::core::mem::transmute(pucopied)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn GetStatusWindowPos<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) -> ::windows_core::Result<super::super::super::Foundation::POINT> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::POINT>::zeroed();
        (::windows_core::Interface::vtable(self).GetStatusWindowPos)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::POINT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVirtualKey<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwnd: Param0) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetVirtualKey)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn InstallIMEA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(&self, szimefilename: Param0, szlayouttext: Param1) -> ::windows_core::Result<super::super::TextServices::HKL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::TextServices::HKL>::zeroed();
        (::windows_core::Interface::vtable(self).InstallIMEA)(::windows_core::Interface::as_raw(self), szimefilename.into_param().abi(), szlayouttext.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::TextServices::HKL>(result__)
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn InstallIMEW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, szimefilename: Param0, szlayouttext: Param1) -> ::windows_core::Result<super::super::TextServices::HKL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::TextServices::HKL>::zeroed();
        (::windows_core::Interface::vtable(self).InstallIMEW)(::windows_core::Interface::as_raw(self), szimefilename.into_param().abi(), szlayouttext.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::TextServices::HKL>(result__)
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn IsIME<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsIME)(::windows_core::Interface::as_raw(self), hkl.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUIMessageA<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>, Param2: ::windows_core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param3: ::windows_core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(&self, hwndime: Param0, msg: u32, wparam: Param2, lparam: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsUIMessageA)(::windows_core::Interface::as_raw(self), hwndime.into_param().abi(), ::core::mem::transmute(msg), wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUIMessageW<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>, Param2: ::windows_core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param3: ::windows_core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(&self, hwndime: Param0, msg: u32, wparam: Param2, lparam: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsUIMessageW)(::windows_core::Interface::as_raw(self), hwndime.into_param().abi(), ::core::mem::transmute(msg), wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn NotifyIME<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NotifyIME)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(dwaction), ::core::mem::transmute(dwindex), ::core::mem::transmute(dwvalue)).ok()
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn RegisterWordA<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(&self, hkl: Param0, szreading: Param1, dwstyle: u32, szregister: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterWordA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szregister.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn RegisterWordW<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, hkl: Param0, szreading: Param1, dwstyle: u32, szregister: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterWordW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szregister.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn ReleaseContext<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>, Param1: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, hwnd: Param0, himc: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReleaseContext)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), himc.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn SetCandidateWindow<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, pcandidate: *const CANDIDATEFORM) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCandidateWindow)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(pcandidate)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn SetCompositionFontA<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCompositionFontA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(plf)).ok()
    }
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn SetCompositionFontW<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCompositionFontW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(plf)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn SetCompositionStringA<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCompositionStringA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(pcomp), ::core::mem::transmute(dwcomplen), ::core::mem::transmute(pread), ::core::mem::transmute(dwreadlen)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn SetCompositionStringW<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCompositionStringW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(pcomp), ::core::mem::transmute(dwcomplen), ::core::mem::transmute(pread), ::core::mem::transmute(dwreadlen)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn SetCompositionWindow<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, pcompform: *const COMPOSITIONFORM) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCompositionWindow)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(pcompform)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn SetConversionStatus<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, fdwconversion: u32, fdwsentence: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetConversionStatus)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(fdwconversion), ::core::mem::transmute(fdwsentence)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn SetOpenStatus<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, himc: Param0, fopen: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOpenStatus)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), fopen.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn SetStatusWindowPos<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, pptpos: *const super::super::super::Foundation::POINT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStatusWindowPos)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(pptpos)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SimulateHotKey<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwnd: Param0, dwhotkeyid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SimulateHotKey)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), ::core::mem::transmute(dwhotkeyid)).ok()
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn UnregisterWordA<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(&self, hkl: Param0, szreading: Param1, dwstyle: u32, szunregister: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnregisterWordA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szunregister.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn UnregisterWordW<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, hkl: Param0, szreading: Param1, dwstyle: u32, szunregister: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnregisterWordW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szunregister.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, frestorelayout: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Activate)(::windows_core::Interface::as_raw(self), frestorelayout.into_param().abi()).ok()
    }
    pub unsafe fn Deactivate(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Deactivate)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnDefWindowProc<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>, Param2: ::windows_core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param3: ::windows_core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(&self, hwnd: Param0, msg: u32, wparam: Param2, lparam: Param3) -> ::windows_core::Result<super::super::super::Foundation::LRESULT> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::LRESULT>::zeroed();
        (::windows_core::Interface::vtable(self).OnDefWindowProc)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), ::core::mem::transmute(msg), wparam.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::LRESULT>(result__)
    }
    pub unsafe fn FilterClientWindows(&self, aaclasslist: *const u16, usize: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FilterClientWindows)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(aaclasslist), ::core::mem::transmute(usize)).ok()
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetCodePageA<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetCodePageA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetLangId<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).GetLangId)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn AssociateContextEx<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>, Param1: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, hwnd: Param0, himc: Param1, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AssociateContextEx)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), himc.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn DisableIME(&self, idthread: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisableIME)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(idthread)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetImeMenuItemsA<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetImeMenuItemsA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwtype), ::core::mem::transmute(pimeparentmenu), ::core::mem::transmute(pimemenu), ::core::mem::transmute(dwsize), ::core::mem::transmute(pdwresult)).ok()
    }
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetImeMenuItemsW<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetImeMenuItemsW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwtype), ::core::mem::transmute(pimeparentmenu), ::core::mem::transmute(pimemenu), ::core::mem::transmute(dwsize), ::core::mem::transmute(pdwresult)).ok()
    }
    pub unsafe fn EnumInputContext(&self, idthread: u32) -> ::windows_core::Result<IEnumInputContext> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumInputContext)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(idthread), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumInputContext>(result__)
    }
}
impl ::core::convert::From<IActiveIMMApp> for ::windows_core::IUnknown {
    fn from(value: IActiveIMMApp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActiveIMMApp> for ::windows_core::IUnknown {
    fn from(value: &IActiveIMMApp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IActiveIMMApp {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IActiveIMMApp {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IActiveIMMApp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActiveIMMApp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveIMMApp {}
impl ::core::fmt::Debug for IActiveIMMApp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveIMMApp").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IActiveIMMApp {
    type Vtable = IActiveIMMApp_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08c0e040_62d1_11d1_9326_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveIMMApp_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub AssociateContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, hime: super::super::super::Globalization::HIMC, phprev: *mut super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    AssociateContext: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub ConfigureIMEA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDA) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))]
    ConfigureIMEA: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub ConfigureIMEW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDW) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))]
    ConfigureIMEW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub CreateContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phimc: *mut super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    CreateContext: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub DestroyContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hime: super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    DestroyContext: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub EnumRegisterWordA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCSTR, dwstyle: u32, szregister: ::windows_core::PCSTR, pdata: *const ::core::ffi::c_void, penum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    EnumRegisterWordA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub EnumRegisterWordW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCWSTR, dwstyle: u32, szregister: ::windows_core::PCWSTR, pdata: *const ::core::ffi::c_void, penum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    EnumRegisterWordW: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub EscapeA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices")))]
    EscapeA: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub EscapeW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices")))]
    EscapeW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetCandidateListA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetCandidateListA: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetCandidateListW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetCandidateListW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetCandidateListCountA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetCandidateListCountA: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetCandidateListCountW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetCandidateListCountW: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub GetCandidateWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcandidate: *mut CANDIDATEFORM) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    GetCandidateWindow: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub GetCompositionFontA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))]
    GetCompositionFontA: usize,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub GetCompositionFontW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))]
    GetCompositionFontW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetCompositionStringA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetCompositionStringA: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetCompositionStringW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetCompositionStringW: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub GetCompositionWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcompform: *mut COMPOSITIONFORM) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    GetCompositionWindow: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub GetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phimc: *mut super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    GetContext: usize,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub GetConversionListA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: ::windows_core::PCSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_UI_TextServices")))]
    GetConversionListA: usize,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub GetConversionListW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: ::windows_core::PCWSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_UI_TextServices")))]
    GetConversionListW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetConversionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetConversionStatus: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDefaultIMEWnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phdefwnd: *mut super::super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDefaultIMEWnd: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetDescriptionA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: ::windows_core::PSTR, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetDescriptionA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetDescriptionW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: ::windows_core::PWSTR, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetDescriptionW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetGuideLineA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: ::windows_core::PSTR, pdwresult: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetGuideLineA: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetGuideLineW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: ::windows_core::PWSTR, pdwresult: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetGuideLineW: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetIMEFileNameA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: ::windows_core::PSTR, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetIMEFileNameA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetIMEFileNameW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: ::windows_core::PWSTR, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetIMEFileNameW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetOpenStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetOpenStatus: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, fdwindex: u32, pdwproperty: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub GetRegisterWordStyleA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))]
    GetRegisterWordStyleA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetRegisterWordStyleW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetRegisterWordStyleW: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub GetStatusWindowPos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pptpos: *mut super::super::super::Foundation::POINT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    GetStatusWindowPos: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetVirtualKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, puvirtualkey: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetVirtualKey: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub InstallIMEA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szimefilename: ::windows_core::PCSTR, szlayouttext: ::windows_core::PCSTR, phkl: *mut super::super::TextServices::HKL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    InstallIMEA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub InstallIMEW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szimefilename: ::windows_core::PCWSTR, szlayouttext: ::windows_core::PCWSTR, phkl: *mut super::super::TextServices::HKL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    InstallIMEW: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub IsIME: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    IsIME: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUIMessageA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUIMessageA: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUIMessageW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUIMessageW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub NotifyIME: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    NotifyIME: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub RegisterWordA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCSTR, dwstyle: u32, szregister: ::windows_core::PCSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    RegisterWordA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub RegisterWordW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCWSTR, dwstyle: u32, szregister: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    RegisterWordW: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub ReleaseContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    ReleaseContext: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub SetCandidateWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcandidate: *const CANDIDATEFORM) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    SetCandidateWindow: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub SetCompositionFontA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))]
    SetCompositionFontA: usize,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub SetCompositionFontW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))]
    SetCompositionFontW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub SetCompositionStringA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    SetCompositionStringA: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub SetCompositionStringW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    SetCompositionStringW: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub SetCompositionWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcompform: *const COMPOSITIONFORM) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    SetCompositionWindow: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub SetConversionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fdwconversion: u32, fdwsentence: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    SetConversionStatus: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub SetOpenStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fopen: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    SetOpenStatus: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub SetStatusWindowPos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pptpos: *const super::super::super::Foundation::POINT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    SetStatusWindowPos: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SimulateHotKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, dwhotkeyid: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SimulateHotKey: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub UnregisterWordA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCSTR, dwstyle: u32, szunregister: ::windows_core::PCSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    UnregisterWordA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub UnregisterWordW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCWSTR, dwstyle: u32, szunregister: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    UnregisterWordW: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frestorelayout: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Activate: usize,
    pub Deactivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnDefWindowProc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnDefWindowProc: usize,
    pub FilterClientWindows: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aaclasslist: *const u16, usize: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetCodePageA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ucodepage: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetCodePageA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetLangId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, plid: *mut u16) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetLangId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub AssociateContextEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC, dwflags: u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    AssociateContextEx: usize,
    pub DisableIME: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idthread: u32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub GetImeMenuItemsA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))]
    GetImeMenuItemsA: usize,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub GetImeMenuItemsW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))]
    GetImeMenuItemsW: usize,
    pub EnumInputContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idthread: u32, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IActiveIMMIME(::windows_core::IUnknown);
impl IActiveIMMIME {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn AssociateContext<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>, Param1: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, hwnd: Param0, hime: Param1) -> ::windows_core::Result<super::super::super::Globalization::HIMC> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Globalization::HIMC>::zeroed();
        (::windows_core::Interface::vtable(self).AssociateContext)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), hime.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Globalization::HIMC>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn ConfigureIMEA<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hkl: Param0, hwnd: Param1, dwmode: u32, pdata: *const REGISTERWORDA) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ConfigureIMEA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), hwnd.into_param().abi(), ::core::mem::transmute(dwmode), ::core::mem::transmute(pdata)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn ConfigureIMEW<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hkl: Param0, hwnd: Param1, dwmode: u32, pdata: *const REGISTERWORDW) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ConfigureIMEW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), hwnd.into_param().abi(), ::core::mem::transmute(dwmode), ::core::mem::transmute(pdata)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn CreateContext(&self) -> ::windows_core::Result<super::super::super::Globalization::HIMC> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Globalization::HIMC>::zeroed();
        (::windows_core::Interface::vtable(self).CreateContext)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Globalization::HIMC>(result__)
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn DestroyContext<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, hime: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DestroyContext)(::windows_core::Interface::as_raw(self), hime.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn EnumRegisterWordA<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(&self, hkl: Param0, szreading: Param1, dwstyle: u32, szregister: Param3, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<IEnumRegisterWordA> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumRegisterWordA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szregister.into_param().abi(), ::core::mem::transmute(pdata), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumRegisterWordA>(result__)
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn EnumRegisterWordW<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, hkl: Param0, szreading: Param1, dwstyle: u32, szregister: Param3, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<IEnumRegisterWordW> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumRegisterWordW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szregister.into_param().abi(), ::core::mem::transmute(pdata), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumRegisterWordW>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub unsafe fn EscapeA<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, hkl: Param0, himc: Param1, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EscapeA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), himc.into_param().abi(), ::core::mem::transmute(uescape), ::core::mem::transmute(pdata), ::core::mem::transmute(plresult)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub unsafe fn EscapeW<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, hkl: Param0, himc: Param1, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EscapeW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), himc.into_param().abi(), ::core::mem::transmute(uescape), ::core::mem::transmute(pdata), ::core::mem::transmute(plresult)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCandidateListA<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCandidateListA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(ubuflen), ::core::mem::transmute(pcandlist), ::core::mem::transmute(pucopied)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCandidateListW<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCandidateListW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(ubuflen), ::core::mem::transmute(pcandlist), ::core::mem::transmute(pucopied)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCandidateListCountA<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCandidateListCountA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(pdwlistsize), ::core::mem::transmute(pdwbuflen)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCandidateListCountW<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCandidateListCountW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(pdwlistsize), ::core::mem::transmute(pdwbuflen)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn GetCandidateWindow<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32) -> ::windows_core::Result<CANDIDATEFORM> {
        let mut result__ = ::core::mem::MaybeUninit::<CANDIDATEFORM>::zeroed();
        (::windows_core::Interface::vtable(self).GetCandidateWindow)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<CANDIDATEFORM>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetCompositionFontA<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) -> ::windows_core::Result<super::super::super::Graphics::Gdi::LOGFONTA> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Graphics::Gdi::LOGFONTA>::zeroed();
        (::windows_core::Interface::vtable(self).GetCompositionFontA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Graphics::Gdi::LOGFONTA>(result__)
    }
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetCompositionFontW<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) -> ::windows_core::Result<super::super::super::Graphics::Gdi::LOGFONTW> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Graphics::Gdi::LOGFONTW>::zeroed();
        (::windows_core::Interface::vtable(self).GetCompositionFontW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Graphics::Gdi::LOGFONTW>(result__)
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCompositionStringA<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCompositionStringA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(dwbuflen), ::core::mem::transmute(plcopied), ::core::mem::transmute(pbuf)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCompositionStringW<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCompositionStringW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(dwbuflen), ::core::mem::transmute(plcopied), ::core::mem::transmute(pbuf)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn GetCompositionWindow<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) -> ::windows_core::Result<COMPOSITIONFORM> {
        let mut result__ = ::core::mem::MaybeUninit::<COMPOSITIONFORM>::zeroed();
        (::windows_core::Interface::vtable(self).GetCompositionWindow)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<COMPOSITIONFORM>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn GetContext<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwnd: Param0) -> ::windows_core::Result<super::super::super::Globalization::HIMC> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Globalization::HIMC>::zeroed();
        (::windows_core::Interface::vtable(self).GetContext)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Globalization::HIMC>(result__)
    }
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub unsafe fn GetConversionListA<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(&self, hkl: Param0, himc: Param1, psrc: Param2, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetConversionListA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), himc.into_param().abi(), psrc.into_param().abi(), ::core::mem::transmute(ubuflen), ::core::mem::transmute(uflag), ::core::mem::transmute(pdst), ::core::mem::transmute(pucopied)).ok()
    }
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub unsafe fn GetConversionListW<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, hkl: Param0, himc: Param1, psrc: Param2, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetConversionListW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), himc.into_param().abi(), psrc.into_param().abi(), ::core::mem::transmute(ubuflen), ::core::mem::transmute(uflag), ::core::mem::transmute(pdst), ::core::mem::transmute(pucopied)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetConversionStatus<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetConversionStatus)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(pfdwconversion), ::core::mem::transmute(pfdwsentence)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDefaultIMEWnd<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwnd: Param0) -> ::windows_core::Result<super::super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::HWND>::zeroed();
        (::windows_core::Interface::vtable(self).GetDefaultIMEWnd)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::HWND>(result__)
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetDescriptionA<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0, ubuflen: u32, szdescription: ::windows_core::PSTR, pucopied: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDescriptionA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), ::core::mem::transmute(ubuflen), ::core::mem::transmute(szdescription), ::core::mem::transmute(pucopied)).ok()
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetDescriptionW<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0, ubuflen: u32, szdescription: ::windows_core::PWSTR, pucopied: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDescriptionW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), ::core::mem::transmute(ubuflen), ::core::mem::transmute(szdescription), ::core::mem::transmute(pucopied)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetGuideLineA<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, dwbuflen: u32, pbuf: ::windows_core::PSTR, pdwresult: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetGuideLineA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(dwbuflen), ::core::mem::transmute(pbuf), ::core::mem::transmute(pdwresult)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetGuideLineW<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, dwbuflen: u32, pbuf: ::windows_core::PWSTR, pdwresult: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetGuideLineW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(dwbuflen), ::core::mem::transmute(pbuf), ::core::mem::transmute(pdwresult)).ok()
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetIMEFileNameA<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0, ubuflen: u32, szfilename: ::windows_core::PSTR, pucopied: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetIMEFileNameA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), ::core::mem::transmute(ubuflen), ::core::mem::transmute(szfilename), ::core::mem::transmute(pucopied)).ok()
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetIMEFileNameW<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0, ubuflen: u32, szfilename: ::windows_core::PWSTR, pucopied: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetIMEFileNameW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), ::core::mem::transmute(ubuflen), ::core::mem::transmute(szfilename), ::core::mem::transmute(pucopied)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetOpenStatus<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetOpenStatus)(::windows_core::Interface::as_raw(self), himc.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetProperty<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0, fdwindex: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), ::core::mem::transmute(fdwindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn GetRegisterWordStyleA<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRegisterWordStyleA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), ::core::mem::transmute(nitem), ::core::mem::transmute(pstylebuf), ::core::mem::transmute(pucopied)).ok()
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetRegisterWordStyleW<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRegisterWordStyleW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), ::core::mem::transmute(nitem), ::core::mem::transmute(pstylebuf), ::core::mem::transmute(pucopied)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn GetStatusWindowPos<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) -> ::windows_core::Result<super::super::super::Foundation::POINT> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::POINT>::zeroed();
        (::windows_core::Interface::vtable(self).GetStatusWindowPos)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::POINT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVirtualKey<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwnd: Param0) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetVirtualKey)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn InstallIMEA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(&self, szimefilename: Param0, szlayouttext: Param1) -> ::windows_core::Result<super::super::TextServices::HKL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::TextServices::HKL>::zeroed();
        (::windows_core::Interface::vtable(self).InstallIMEA)(::windows_core::Interface::as_raw(self), szimefilename.into_param().abi(), szlayouttext.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::TextServices::HKL>(result__)
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn InstallIMEW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, szimefilename: Param0, szlayouttext: Param1) -> ::windows_core::Result<super::super::TextServices::HKL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::TextServices::HKL>::zeroed();
        (::windows_core::Interface::vtable(self).InstallIMEW)(::windows_core::Interface::as_raw(self), szimefilename.into_param().abi(), szlayouttext.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::TextServices::HKL>(result__)
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn IsIME<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsIME)(::windows_core::Interface::as_raw(self), hkl.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUIMessageA<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>, Param2: ::windows_core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param3: ::windows_core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(&self, hwndime: Param0, msg: u32, wparam: Param2, lparam: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsUIMessageA)(::windows_core::Interface::as_raw(self), hwndime.into_param().abi(), ::core::mem::transmute(msg), wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUIMessageW<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>, Param2: ::windows_core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param3: ::windows_core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(&self, hwndime: Param0, msg: u32, wparam: Param2, lparam: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsUIMessageW)(::windows_core::Interface::as_raw(self), hwndime.into_param().abi(), ::core::mem::transmute(msg), wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn NotifyIME<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NotifyIME)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(dwaction), ::core::mem::transmute(dwindex), ::core::mem::transmute(dwvalue)).ok()
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn RegisterWordA<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(&self, hkl: Param0, szreading: Param1, dwstyle: u32, szregister: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterWordA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szregister.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn RegisterWordW<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, hkl: Param0, szreading: Param1, dwstyle: u32, szregister: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterWordW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szregister.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn ReleaseContext<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>, Param1: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, hwnd: Param0, himc: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReleaseContext)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), himc.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn SetCandidateWindow<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, pcandidate: *const CANDIDATEFORM) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCandidateWindow)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(pcandidate)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn SetCompositionFontA<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCompositionFontA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(plf)).ok()
    }
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn SetCompositionFontW<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCompositionFontW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(plf)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn SetCompositionStringA<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCompositionStringA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(pcomp), ::core::mem::transmute(dwcomplen), ::core::mem::transmute(pread), ::core::mem::transmute(dwreadlen)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn SetCompositionStringW<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCompositionStringW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(pcomp), ::core::mem::transmute(dwcomplen), ::core::mem::transmute(pread), ::core::mem::transmute(dwreadlen)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn SetCompositionWindow<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, pcompform: *const COMPOSITIONFORM) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCompositionWindow)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(pcompform)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn SetConversionStatus<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, fdwconversion: u32, fdwsentence: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetConversionStatus)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(fdwconversion), ::core::mem::transmute(fdwsentence)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn SetOpenStatus<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, himc: Param0, fopen: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOpenStatus)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), fopen.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn SetStatusWindowPos<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, pptpos: *const super::super::super::Foundation::POINT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStatusWindowPos)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(pptpos)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SimulateHotKey<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwnd: Param0, dwhotkeyid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SimulateHotKey)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), ::core::mem::transmute(dwhotkeyid)).ok()
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn UnregisterWordA<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(&self, hkl: Param0, szreading: Param1, dwstyle: u32, szunregister: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnregisterWordA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szunregister.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn UnregisterWordW<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, hkl: Param0, szreading: Param1, dwstyle: u32, szunregister: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnregisterWordW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szunregister.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GenerateMessage<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GenerateMessage)(::windows_core::Interface::as_raw(self), himc.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn LockIMC<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) -> ::windows_core::Result<*mut INPUTCONTEXT> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut INPUTCONTEXT>::zeroed();
        (::windows_core::Interface::vtable(self).LockIMC)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut INPUTCONTEXT>(result__)
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn UnlockIMC<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnlockIMC)(::windows_core::Interface::as_raw(self), himc.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetIMCLockCount<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetIMCLockCount)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn CreateIMCC(&self, dwsize: u32) -> ::windows_core::Result<super::super::super::Globalization::HIMCC> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Globalization::HIMCC>::zeroed();
        (::windows_core::Interface::vtable(self).CreateIMCC)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwsize), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Globalization::HIMCC>(result__)
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn DestroyIMCC<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMCC>>(&self, himcc: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DestroyIMCC)(::windows_core::Interface::as_raw(self), himcc.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn LockIMCC<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMCC>>(&self, himcc: Param0, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LockIMCC)(::windows_core::Interface::as_raw(self), himcc.into_param().abi(), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn UnlockIMCC<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMCC>>(&self, himcc: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnlockIMCC)(::windows_core::Interface::as_raw(self), himcc.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn ReSizeIMCC<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMCC>>(&self, himcc: Param0, dwsize: u32) -> ::windows_core::Result<super::super::super::Globalization::HIMCC> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Globalization::HIMCC>::zeroed();
        (::windows_core::Interface::vtable(self).ReSizeIMCC)(::windows_core::Interface::as_raw(self), himcc.into_param().abi(), ::core::mem::transmute(dwsize), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Globalization::HIMCC>(result__)
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetIMCCSize<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMCC>>(&self, himcc: Param0) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetIMCCSize)(::windows_core::Interface::as_raw(self), himcc.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetIMCCLockCount<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMCC>>(&self, himcc: Param0) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetIMCCLockCount)(::windows_core::Interface::as_raw(self), himcc.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetHotKey(&self, dwhotkeyid: u32, pumodifiers: *mut u32, puvkey: *mut u32, phkl: *mut super::super::TextServices::HKL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetHotKey)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwhotkeyid), ::core::mem::transmute(pumodifiers), ::core::mem::transmute(puvkey), ::core::mem::transmute(phkl)).ok()
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn SetHotKey<'a, Param3: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>>(&self, dwhotkeyid: u32, umodifiers: u32, uvkey: u32, hkl: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetHotKey)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwhotkeyid), ::core::mem::transmute(umodifiers), ::core::mem::transmute(uvkey), hkl.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftKeyboard<'a, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, utype: u32, howner: Param1, x: i32, y: i32) -> ::windows_core::Result<super::super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::HWND>::zeroed();
        (::windows_core::Interface::vtable(self).CreateSoftKeyboard)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(utype), howner.into_param().abi(), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::HWND>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DestroySoftKeyboard<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hsoftkbdwnd: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DestroySoftKeyboard)(::windows_core::Interface::as_raw(self), hsoftkbdwnd.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowSoftKeyboard<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hsoftkbdwnd: Param0, ncmdshow: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ShowSoftKeyboard)(::windows_core::Interface::as_raw(self), hsoftkbdwnd.into_param().abi(), ::core::mem::transmute(ncmdshow)).ok()
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetCodePageA<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetCodePageA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetLangId<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).GetLangId)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn KeybdEvent(&self, lgidime: u16, bvk: u8, bscan: u8, dwflags: u32, dwextrainfo: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).KeybdEvent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lgidime), ::core::mem::transmute(bvk), ::core::mem::transmute(bscan), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwextrainfo)).ok()
    }
    pub unsafe fn LockModal(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LockModal)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn UnlockModal(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnlockModal)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn AssociateContextEx<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>, Param1: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, hwnd: Param0, himc: Param1, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AssociateContextEx)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), himc.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn DisableIME(&self, idthread: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisableIME)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(idthread)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetImeMenuItemsA<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetImeMenuItemsA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwtype), ::core::mem::transmute(pimeparentmenu), ::core::mem::transmute(pimemenu), ::core::mem::transmute(dwsize), ::core::mem::transmute(pdwresult)).ok()
    }
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetImeMenuItemsW<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetImeMenuItemsW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwtype), ::core::mem::transmute(pimeparentmenu), ::core::mem::transmute(pimemenu), ::core::mem::transmute(dwsize), ::core::mem::transmute(pdwresult)).ok()
    }
    pub unsafe fn EnumInputContext(&self, idthread: u32) -> ::windows_core::Result<IEnumInputContext> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumInputContext)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(idthread), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumInputContext>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn RequestMessageA<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param2: ::windows_core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(&self, himc: Param0, wparam: Param1, lparam: Param2) -> ::windows_core::Result<super::super::super::Foundation::LRESULT> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::LRESULT>::zeroed();
        (::windows_core::Interface::vtable(self).RequestMessageA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), wparam.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::LRESULT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn RequestMessageW<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param2: ::windows_core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(&self, himc: Param0, wparam: Param1, lparam: Param2) -> ::windows_core::Result<super::super::super::Foundation::LRESULT> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::LRESULT>::zeroed();
        (::windows_core::Interface::vtable(self).RequestMessageW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), wparam.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::LRESULT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SendIMCA<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>, Param2: ::windows_core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param3: ::windows_core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(&self, hwnd: Param0, umsg: u32, wparam: Param2, lparam: Param3) -> ::windows_core::Result<super::super::super::Foundation::LRESULT> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::LRESULT>::zeroed();
        (::windows_core::Interface::vtable(self).SendIMCA)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), ::core::mem::transmute(umsg), wparam.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::LRESULT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SendIMCW<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>, Param2: ::windows_core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param3: ::windows_core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(&self, hwnd: Param0, umsg: u32, wparam: Param2, lparam: Param3) -> ::windows_core::Result<super::super::super::Foundation::LRESULT> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::LRESULT>::zeroed();
        (::windows_core::Interface::vtable(self).SendIMCW)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), ::core::mem::transmute(umsg), wparam.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::LRESULT>(result__)
    }
    pub unsafe fn IsSleeping(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsSleeping)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IActiveIMMIME> for ::windows_core::IUnknown {
    fn from(value: IActiveIMMIME) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActiveIMMIME> for ::windows_core::IUnknown {
    fn from(value: &IActiveIMMIME) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IActiveIMMIME {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IActiveIMMIME {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IActiveIMMIME {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActiveIMMIME {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveIMMIME {}
impl ::core::fmt::Debug for IActiveIMMIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveIMMIME").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IActiveIMMIME {
    type Vtable = IActiveIMMIME_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08c03411_f96b_11d0_a475_00aa006bcc59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveIMMIME_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub AssociateContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, hime: super::super::super::Globalization::HIMC, phprev: *mut super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    AssociateContext: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub ConfigureIMEA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDA) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))]
    ConfigureIMEA: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub ConfigureIMEW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDW) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))]
    ConfigureIMEW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub CreateContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phimc: *mut super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    CreateContext: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub DestroyContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hime: super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    DestroyContext: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub EnumRegisterWordA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCSTR, dwstyle: u32, szregister: ::windows_core::PCSTR, pdata: *const ::core::ffi::c_void, penum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    EnumRegisterWordA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub EnumRegisterWordW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCWSTR, dwstyle: u32, szregister: ::windows_core::PCWSTR, pdata: *const ::core::ffi::c_void, penum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    EnumRegisterWordW: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub EscapeA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices")))]
    EscapeA: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub EscapeW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices")))]
    EscapeW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetCandidateListA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetCandidateListA: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetCandidateListW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetCandidateListW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetCandidateListCountA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetCandidateListCountA: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetCandidateListCountW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetCandidateListCountW: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub GetCandidateWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcandidate: *mut CANDIDATEFORM) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    GetCandidateWindow: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub GetCompositionFontA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))]
    GetCompositionFontA: usize,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub GetCompositionFontW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))]
    GetCompositionFontW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetCompositionStringA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetCompositionStringA: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetCompositionStringW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetCompositionStringW: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub GetCompositionWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcompform: *mut COMPOSITIONFORM) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    GetCompositionWindow: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub GetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phimc: *mut super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    GetContext: usize,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub GetConversionListA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: ::windows_core::PCSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_UI_TextServices")))]
    GetConversionListA: usize,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub GetConversionListW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: ::windows_core::PCWSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_UI_TextServices")))]
    GetConversionListW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetConversionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetConversionStatus: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDefaultIMEWnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phdefwnd: *mut super::super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDefaultIMEWnd: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetDescriptionA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: ::windows_core::PSTR, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetDescriptionA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetDescriptionW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: ::windows_core::PWSTR, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetDescriptionW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetGuideLineA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: ::windows_core::PSTR, pdwresult: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetGuideLineA: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetGuideLineW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: ::windows_core::PWSTR, pdwresult: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetGuideLineW: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetIMEFileNameA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: ::windows_core::PSTR, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetIMEFileNameA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetIMEFileNameW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: ::windows_core::PWSTR, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetIMEFileNameW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetOpenStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetOpenStatus: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, fdwindex: u32, pdwproperty: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub GetRegisterWordStyleA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))]
    GetRegisterWordStyleA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetRegisterWordStyleW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetRegisterWordStyleW: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub GetStatusWindowPos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pptpos: *mut super::super::super::Foundation::POINT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    GetStatusWindowPos: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetVirtualKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, puvirtualkey: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetVirtualKey: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub InstallIMEA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szimefilename: ::windows_core::PCSTR, szlayouttext: ::windows_core::PCSTR, phkl: *mut super::super::TextServices::HKL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    InstallIMEA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub InstallIMEW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szimefilename: ::windows_core::PCWSTR, szlayouttext: ::windows_core::PCWSTR, phkl: *mut super::super::TextServices::HKL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    InstallIMEW: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub IsIME: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    IsIME: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUIMessageA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUIMessageA: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUIMessageW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUIMessageW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub NotifyIME: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    NotifyIME: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub RegisterWordA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCSTR, dwstyle: u32, szregister: ::windows_core::PCSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    RegisterWordA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub RegisterWordW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCWSTR, dwstyle: u32, szregister: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    RegisterWordW: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub ReleaseContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    ReleaseContext: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub SetCandidateWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcandidate: *const CANDIDATEFORM) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    SetCandidateWindow: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub SetCompositionFontA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))]
    SetCompositionFontA: usize,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub SetCompositionFontW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))]
    SetCompositionFontW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub SetCompositionStringA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    SetCompositionStringA: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub SetCompositionStringW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    SetCompositionStringW: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub SetCompositionWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcompform: *const COMPOSITIONFORM) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    SetCompositionWindow: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub SetConversionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fdwconversion: u32, fdwsentence: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    SetConversionStatus: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub SetOpenStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fopen: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    SetOpenStatus: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub SetStatusWindowPos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pptpos: *const super::super::super::Foundation::POINT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    SetStatusWindowPos: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SimulateHotKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, dwhotkeyid: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SimulateHotKey: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub UnregisterWordA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCSTR, dwstyle: u32, szunregister: ::windows_core::PCSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    UnregisterWordA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub UnregisterWordW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCWSTR, dwstyle: u32, szunregister: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    UnregisterWordW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GenerateMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GenerateMessage: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub LockIMC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, ppimc: *mut *mut INPUTCONTEXT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))]
    LockIMC: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub UnlockIMC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    UnlockIMC: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetIMCLockCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlockcount: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetIMCLockCount: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub CreateIMCC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsize: u32, phimcc: *mut super::super::super::Globalization::HIMCC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    CreateIMCC: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub DestroyIMCC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    DestroyIMCC: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub LockIMCC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    LockIMCC: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub UnlockIMCC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    UnlockIMCC: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub ReSizeIMCC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC, dwsize: u32, phimcc: *mut super::super::super::Globalization::HIMCC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    ReSizeIMCC: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetIMCCSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC, pdwsize: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetIMCCSize: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetIMCCLockCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC, pdwlockcount: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetIMCCLockCount: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetHotKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwhotkeyid: u32, pumodifiers: *mut u32, puvkey: *mut u32, phkl: *mut super::super::TextServices::HKL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetHotKey: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub SetHotKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwhotkeyid: u32, umodifiers: u32, uvkey: u32, hkl: super::super::TextServices::HKL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    SetHotKey: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSoftKeyboard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, utype: u32, howner: super::super::super::Foundation::HWND, x: i32, y: i32, phsoftkbdwnd: *mut super::super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSoftKeyboard: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DestroySoftKeyboard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hsoftkbdwnd: super::super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DestroySoftKeyboard: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowSoftKeyboard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hsoftkbdwnd: super::super::super::Foundation::HWND, ncmdshow: i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowSoftKeyboard: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetCodePageA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ucodepage: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetCodePageA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetLangId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, plid: *mut u16) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetLangId: usize,
    pub KeybdEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lgidime: u16, bvk: u8, bscan: u8, dwflags: u32, dwextrainfo: u32) -> ::windows_core::HRESULT,
    pub LockModal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub UnlockModal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub AssociateContextEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC, dwflags: u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    AssociateContextEx: usize,
    pub DisableIME: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idthread: u32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub GetImeMenuItemsA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))]
    GetImeMenuItemsA: usize,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub GetImeMenuItemsW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))]
    GetImeMenuItemsW: usize,
    pub EnumInputContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idthread: u32, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub RequestMessageA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    RequestMessageA: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub RequestMessageW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    RequestMessageW: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SendIMCA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SendIMCA: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SendIMCW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SendIMCW: usize,
    pub IsSleeping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IActiveIMMMessagePumpOwner(::windows_core::IUnknown);
impl IActiveIMMMessagePumpOwner {
    pub unsafe fn Start(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Start)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn End(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).End)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn OnTranslateMessage(&self, pmsg: *const super::super::WindowsAndMessaging::MSG) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnTranslateMessage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmsg)).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).Pause)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn Resume(&self, dwcookie: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Resume)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwcookie)).ok()
    }
}
impl ::core::convert::From<IActiveIMMMessagePumpOwner> for ::windows_core::IUnknown {
    fn from(value: IActiveIMMMessagePumpOwner) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActiveIMMMessagePumpOwner> for ::windows_core::IUnknown {
    fn from(value: &IActiveIMMMessagePumpOwner) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IActiveIMMMessagePumpOwner {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IActiveIMMMessagePumpOwner {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IActiveIMMMessagePumpOwner {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActiveIMMMessagePumpOwner {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveIMMMessagePumpOwner {}
impl ::core::fmt::Debug for IActiveIMMMessagePumpOwner {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveIMMMessagePumpOwner").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IActiveIMMMessagePumpOwner {
    type Vtable = IActiveIMMMessagePumpOwner_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb5cf2cfa_8aeb_11d1_9364_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveIMMMessagePumpOwner_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub End: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub OnTranslateMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *const super::super::WindowsAndMessaging::MSG) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    OnTranslateMessage: usize,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IActiveIMMRegistrar(::windows_core::IUnknown);
impl IActiveIMMRegistrar {
    pub unsafe fn RegisterIME<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, rclsid: *const ::windows_core::GUID, lgid: u16, psziconfile: Param2, pszdesc: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterIME)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(lgid), psziconfile.into_param().abi(), pszdesc.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterIME(&self, rclsid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnregisterIME)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rclsid)).ok()
    }
}
impl ::core::convert::From<IActiveIMMRegistrar> for ::windows_core::IUnknown {
    fn from(value: IActiveIMMRegistrar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActiveIMMRegistrar> for ::windows_core::IUnknown {
    fn from(value: &IActiveIMMRegistrar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IActiveIMMRegistrar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IActiveIMMRegistrar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IActiveIMMRegistrar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActiveIMMRegistrar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveIMMRegistrar {}
impl ::core::fmt::Debug for IActiveIMMRegistrar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveIMMRegistrar").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IActiveIMMRegistrar {
    type Vtable = IActiveIMMRegistrar_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb3458082_bd00_11d1_939b_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveIMMRegistrar_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub RegisterIME: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, lgid: u16, psziconfile: ::windows_core::PCWSTR, pszdesc: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub UnregisterIME: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumInputContext(::windows_core::IUnknown);
impl IEnumInputContext {
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumInputContext> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumInputContext>(result__)
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn Next(&self, ulcount: u32, rginputcontext: *mut super::super::super::Globalization::HIMC, pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(rginputcontext), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ulcount)).ok()
    }
}
impl ::core::convert::From<IEnumInputContext> for ::windows_core::IUnknown {
    fn from(value: IEnumInputContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumInputContext> for ::windows_core::IUnknown {
    fn from(value: &IEnumInputContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumInputContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumInputContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumInputContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumInputContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumInputContext {}
impl ::core::fmt::Debug for IEnumInputContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumInputContext").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumInputContext {
    type Vtable = IEnumInputContext_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x09b5eab0_f997_11d1_93d4_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumInputContext_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Globalization")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, rginputcontext: *mut super::super::super::Globalization::HIMC, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumRegisterWordA(::windows_core::IUnknown);
impl IEnumRegisterWordA {
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumRegisterWordA> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumRegisterWordA>(result__)
    }
    pub unsafe fn Next(&self, ulcount: u32, rgregisterword: *mut REGISTERWORDA, pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(rgregisterword), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ulcount)).ok()
    }
}
impl ::core::convert::From<IEnumRegisterWordA> for ::windows_core::IUnknown {
    fn from(value: IEnumRegisterWordA) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumRegisterWordA> for ::windows_core::IUnknown {
    fn from(value: &IEnumRegisterWordA) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumRegisterWordA {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumRegisterWordA {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumRegisterWordA {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumRegisterWordA {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumRegisterWordA {}
impl ::core::fmt::Debug for IEnumRegisterWordA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumRegisterWordA").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumRegisterWordA {
    type Vtable = IEnumRegisterWordA_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08c03412_f96b_11d0_a475_00aa006bcc59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumRegisterWordA_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, rgregisterword: *mut REGISTERWORDA, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumRegisterWordW(::windows_core::IUnknown);
impl IEnumRegisterWordW {
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumRegisterWordW> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumRegisterWordW>(result__)
    }
    pub unsafe fn Next(&self, ulcount: u32, rgregisterword: *mut REGISTERWORDW, pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(rgregisterword), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ulcount)).ok()
    }
}
impl ::core::convert::From<IEnumRegisterWordW> for ::windows_core::IUnknown {
    fn from(value: IEnumRegisterWordW) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumRegisterWordW> for ::windows_core::IUnknown {
    fn from(value: &IEnumRegisterWordW) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumRegisterWordW {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumRegisterWordW {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumRegisterWordW {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumRegisterWordW {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumRegisterWordW {}
impl ::core::fmt::Debug for IEnumRegisterWordW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumRegisterWordW").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumRegisterWordW {
    type Vtable = IEnumRegisterWordW_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4955dd31_b159_11d0_8fcf_00aa006bcc59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumRegisterWordW_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, rgregisterword: *mut REGISTERWORDW, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT,
}
pub const IFEC_S_ALREADY_DEFAULT: ::windows_core::HRESULT = ::windows_core::HRESULT(291840i32);
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFEClassFactory(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFEClassFactory {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateInstance<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, T: ::windows_core::Interface>(&self, punkouter: Param0) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).base__.CreateInstance)(::windows_core::Interface::as_raw(self), punkouter.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn LockServer<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, flock: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.LockServer)(::windows_core::Interface::as_raw(self), flock.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFEClassFactory> for ::windows_core::IUnknown {
    fn from(value: IFEClassFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFEClassFactory> for ::windows_core::IUnknown {
    fn from(value: &IFEClassFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFEClassFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFEClassFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFEClassFactory> for super::super::super::System::Com::IClassFactory {
    fn from(value: IFEClassFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFEClassFactory> for super::super::super::System::Com::IClassFactory {
    fn from(value: &IFEClassFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::System::Com::IClassFactory> for IFEClassFactory {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::System::Com::IClassFactory> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::System::Com::IClassFactory> for &'a IFEClassFactory {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::System::Com::IClassFactory> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFEClassFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFEClassFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFEClassFactory {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFEClassFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFEClassFactory").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFEClassFactory {
    type Vtable = IFEClassFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::zeroed();
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFEClassFactory_Vtbl {
    pub base__: super::super::super::System::Com::IClassFactory_Vtbl,
}
#[repr(transparent)]
pub struct IFECommon(::windows_core::IUnknown);
impl IFECommon {
    pub unsafe fn IsDefaultIME(&self, szname: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsDefaultIME)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(szname)), szname.len() as _).ok()
    }
    pub unsafe fn SetDefaultIME(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDefaultIME)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InvokeWordRegDialog(&self, pimedlg: *mut IMEDLG) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InvokeWordRegDialog)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pimedlg)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InvokeDictToolDialog(&self, pimedlg: *mut IMEDLG) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InvokeDictToolDialog)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pimedlg)).ok()
    }
}
impl ::core::convert::From<IFECommon> for ::windows_core::IUnknown {
    fn from(value: IFECommon) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFECommon> for ::windows_core::IUnknown {
    fn from(value: &IFECommon) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFECommon {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFECommon {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFECommon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFECommon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFECommon {}
impl ::core::fmt::Debug for IFECommon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFECommon").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IFECommon {
    type Vtable = IFECommon_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x019f7151_e6db_11d0_83c3_00c04fddb82e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFECommon_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub IsDefaultIME: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szname: ::windows_core::PCSTR, cszname: i32) -> ::windows_core::HRESULT,
    pub SetDefaultIME: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub InvokeWordRegDialog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimedlg: *mut IMEDLG) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InvokeWordRegDialog: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InvokeDictToolDialog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimedlg: *mut IMEDLG) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InvokeDictToolDialog: usize,
}
pub const IFED_E_INVALID_FORMAT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147192063i32);
pub const IFED_E_NOT_FOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147192064i32);
pub const IFED_E_NOT_SUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147192057i32);
pub const IFED_E_NOT_USER_DIC: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147192058i32);
pub const IFED_E_NO_ENTRY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147192060i32);
pub const IFED_E_OPEN_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147192062i32);
pub const IFED_E_REGISTER_DISCONNECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147192053i32);
pub const IFED_E_REGISTER_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147192059i32);
pub const IFED_E_REGISTER_ILLEGAL_POS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147192055i32);
pub const IFED_E_REGISTER_IMPROPER_WORD: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147192054i32);
pub const IFED_E_USER_COMMENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147192056i32);
pub const IFED_E_WRITE_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147192061i32);
pub const IFED_POS_ADJECTIVE: u32 = 4u32;
pub const IFED_POS_ADJECTIVE_VERB: u32 = 8u32;
pub const IFED_POS_ADNOUN: u32 = 32u32;
pub const IFED_POS_ADVERB: u32 = 16u32;
pub const IFED_POS_AFFIX: u32 = 1536u32;
pub const IFED_POS_ALL: u32 = 131071u32;
pub const IFED_POS_AUXILIARY_VERB: u32 = 32768u32;
pub const IFED_POS_CONJUNCTION: u32 = 64u32;
pub const IFED_POS_DEPENDENT: u32 = 114688u32;
pub const IFED_POS_IDIOMS: u32 = 4096u32;
pub const IFED_POS_INDEPENDENT: u32 = 255u32;
pub const IFED_POS_INFLECTIONALSUFFIX: u32 = 256u32;
pub const IFED_POS_INTERJECTION: u32 = 128u32;
pub const IFED_POS_NONE: u32 = 0u32;
pub const IFED_POS_NOUN: u32 = 1u32;
pub const IFED_POS_PARTICLE: u32 = 16384u32;
pub const IFED_POS_PREFIX: u32 = 512u32;
pub const IFED_POS_SUB_VERB: u32 = 65536u32;
pub const IFED_POS_SUFFIX: u32 = 1024u32;
pub const IFED_POS_SYMBOLS: u32 = 8192u32;
pub const IFED_POS_TANKANJI: u32 = 2048u32;
pub const IFED_POS_VERB: u32 = 2u32;
pub const IFED_REG_ALL: u32 = 7u32;
pub const IFED_REG_AUTO: u32 = 2u32;
pub const IFED_REG_GRAMMAR: u32 = 4u32;
pub const IFED_REG_NONE: u32 = 0u32;
pub const IFED_REG_USER: u32 = 1u32;
pub const IFED_SELECT_ALL: u32 = 15u32;
pub const IFED_SELECT_COMMENT: u32 = 8u32;
pub const IFED_SELECT_DISPLAY: u32 = 2u32;
pub const IFED_SELECT_NONE: u32 = 0u32;
pub const IFED_SELECT_POS: u32 = 4u32;
pub const IFED_SELECT_READING: u32 = 1u32;
pub const IFED_S_COMMENT_CHANGED: ::windows_core::HRESULT = ::windows_core::HRESULT(291331i32);
pub const IFED_S_EMPTY_DICTIONARY: ::windows_core::HRESULT = ::windows_core::HRESULT(291329i32);
pub const IFED_S_MORE_ENTRIES: ::windows_core::HRESULT = ::windows_core::HRESULT(291328i32);
pub const IFED_S_WORD_EXISTS: ::windows_core::HRESULT = ::windows_core::HRESULT(291330i32);
pub const IFED_TYPE_ALL: u32 = 31u32;
pub const IFED_TYPE_ENGLISH: u32 = 16u32;
pub const IFED_TYPE_GENERAL: u32 = 1u32;
pub const IFED_TYPE_NAMEPLACE: u32 = 2u32;
pub const IFED_TYPE_NONE: u32 = 0u32;
pub const IFED_TYPE_REVERSE: u32 = 8u32;
pub const IFED_TYPE_SPEECH: u32 = 4u32;
#[repr(transparent)]
pub struct IFEDictionary(::windows_core::IUnknown);
impl IFEDictionary {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Open(&self, pchdictpath: &mut [u8; 260], pshf: *mut IMESHF) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Open)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pchdictpath)), ::core::mem::transmute(pshf)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHeader(&self, pchdictpath: &mut [u8; 260], pshf: *mut IMESHF, pjfmt: *mut IMEFMT, pultype: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetHeader)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pchdictpath)), ::core::mem::transmute(pshf), ::core::mem::transmute(pjfmt), ::core::mem::transmute(pultype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayProperty<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwnd: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisplayProperty)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi()).ok()
    }
    pub unsafe fn GetPosTable(&self, prgpostbl: *mut *mut POSTBL, pcpostbl: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPosTable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prgpostbl), ::core::mem::transmute(pcpostbl)).ok()
    }
    pub unsafe fn GetWords<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwchfirst: Param0, pwchlast: Param1, pwchdisplay: Param2, ulpos: u32, ulselect: u32, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetWords)(::windows_core::Interface::as_raw(self), pwchfirst.into_param().abi(), pwchlast.into_param().abi(), pwchdisplay.into_param().abi(), ::core::mem::transmute(ulpos), ::core::mem::transmute(ulselect), ::core::mem::transmute(ulwordsrc), ::core::mem::transmute(pchbuffer), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(pcwrd)).ok()
    }
    pub unsafe fn NextWords(&self, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NextWords)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pchbuffer), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(pcwrd)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(&self, pchdictpath: Param0, pshf: *mut IMESHF) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), pchdictpath.into_param().abi(), ::core::mem::transmute(pshf)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHeader(&self, pshf: *mut IMESHF) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetHeader)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pshf)).ok()
    }
    pub unsafe fn ExistWord(&self, pwrd: *mut IMEWRD) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ExistWord)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwrd)).ok()
    }
    pub unsafe fn ExistDependency(&self, pdp: *mut IMEDP) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ExistDependency)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdp)).ok()
    }
    pub unsafe fn RegisterWord(&self, reg: IMEREG, pwrd: *mut IMEWRD) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterWord)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(reg), ::core::mem::transmute(pwrd)).ok()
    }
    pub unsafe fn RegisterDependency(&self, reg: IMEREG, pdp: *mut IMEDP) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterDependency)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(reg), ::core::mem::transmute(pdp)).ok()
    }
    pub unsafe fn GetDependencies<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param4: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwchkakarireading: Param0, pwchkakaridisplay: Param1, ulkakaripos: u32, pwchukereading: Param3, pwchukedisplay: Param4, ulukepos: u32, jrel: IMEREL, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDependencies)(::windows_core::Interface::as_raw(self), pwchkakarireading.into_param().abi(), pwchkakaridisplay.into_param().abi(), ::core::mem::transmute(ulkakaripos), pwchukereading.into_param().abi(), pwchukedisplay.into_param().abi(), ::core::mem::transmute(ulukepos), ::core::mem::transmute(jrel), ::core::mem::transmute(ulwordsrc), ::core::mem::transmute(pchbuffer), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(pcdp)).ok()
    }
    pub unsafe fn NextDependencies(&self, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NextDependencies)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pchbuffer), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(pcdp)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConvertFromOldMSIME<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(&self, pchdic: Param0, pfnlog: PFNLOG, reg: IMEREG) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ConvertFromOldMSIME)(::windows_core::Interface::as_raw(self), pchdic.into_param().abi(), ::core::mem::transmute(pfnlog), ::core::mem::transmute(reg)).ok()
    }
    pub unsafe fn ConvertFromUserToSys(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ConvertFromUserToSys)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IFEDictionary> for ::windows_core::IUnknown {
    fn from(value: IFEDictionary) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFEDictionary> for ::windows_core::IUnknown {
    fn from(value: &IFEDictionary) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFEDictionary {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFEDictionary {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFEDictionary {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFEDictionary {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFEDictionary {}
impl ::core::fmt::Debug for IFEDictionary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFEDictionary").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IFEDictionary {
    type Vtable = IFEDictionary_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x019f7153_e6db_11d0_83c3_00c04fddb82e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFEDictionary_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchdictpath: ::windows_core::PSTR, pshf: *mut IMESHF) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Open: usize,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchdictpath: ::windows_core::PSTR, pshf: *mut IMESHF, pjfmt: *mut IMEFMT, pultype: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHeader: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayProperty: usize,
    pub GetPosTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prgpostbl: *mut *mut POSTBL, pcpostbl: *mut i32) -> ::windows_core::HRESULT,
    pub GetWords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchfirst: ::windows_core::PCWSTR, pwchlast: ::windows_core::PCWSTR, pwchdisplay: ::windows_core::PCWSTR, ulpos: u32, ulselect: u32, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> ::windows_core::HRESULT,
    pub NextWords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchdictpath: ::windows_core::PCSTR, pshf: *mut IMESHF) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Create: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshf: *mut IMESHF) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetHeader: usize,
    pub ExistWord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwrd: *mut IMEWRD) -> ::windows_core::HRESULT,
    pub ExistDependency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdp: *mut IMEDP) -> ::windows_core::HRESULT,
    pub RegisterWord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reg: IMEREG, pwrd: *mut IMEWRD) -> ::windows_core::HRESULT,
    pub RegisterDependency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reg: IMEREG, pdp: *mut IMEDP) -> ::windows_core::HRESULT,
    pub GetDependencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchkakarireading: ::windows_core::PCWSTR, pwchkakaridisplay: ::windows_core::PCWSTR, ulkakaripos: u32, pwchukereading: ::windows_core::PCWSTR, pwchukedisplay: ::windows_core::PCWSTR, ulukepos: u32, jrel: IMEREL, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> ::windows_core::HRESULT,
    pub NextDependencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ConvertFromOldMSIME: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchdic: ::windows_core::PCSTR, pfnlog: ::windows_core::RawPtr, reg: IMEREG) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConvertFromOldMSIME: usize,
    pub ConvertFromUserToSys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IFELanguage(::windows_core::IUnknown);
impl IFELanguage {
    pub unsafe fn Open(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Open)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetJMorphResult<'a, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, dwrequest: u32, dwcmode: u32, cwchinput: i32, pwchinput: Param3, pfcinfo: *mut u32, ppresult: *mut *mut MORRSLT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetJMorphResult)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwrequest), ::core::mem::transmute(dwcmode), ::core::mem::transmute(cwchinput), pwchinput.into_param().abi(), ::core::mem::transmute(pfcinfo), ::core::mem::transmute(ppresult)).ok()
    }
    pub unsafe fn GetConversionModeCaps(&self, pdwcaps: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetConversionModeCaps)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwcaps)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPhonetic<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, string: Param0, start: i32, length: i32, phonetic: *mut super::super::super::Foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPhonetic)(::windows_core::Interface::as_raw(self), string.into_param().abi(), ::core::mem::transmute(start), ::core::mem::transmute(length), ::core::mem::transmute(phonetic)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetConversion<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, string: Param0, start: i32, length: i32, result: *mut super::super::super::Foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetConversion)(::windows_core::Interface::as_raw(self), string.into_param().abi(), ::core::mem::transmute(start), ::core::mem::transmute(length), ::core::mem::transmute(result)).ok()
    }
}
impl ::core::convert::From<IFELanguage> for ::windows_core::IUnknown {
    fn from(value: IFELanguage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFELanguage> for ::windows_core::IUnknown {
    fn from(value: &IFELanguage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFELanguage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFELanguage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFELanguage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFELanguage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFELanguage {}
impl ::core::fmt::Debug for IFELanguage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFELanguage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IFELanguage {
    type Vtable = IFELanguage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x019f7152_e6db_11d0_83c3_00c04fddb82e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFELanguage_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetJMorphResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwrequest: u32, dwcmode: u32, cwchinput: i32, pwchinput: ::windows_core::PCWSTR, pfcinfo: *mut u32, ppresult: *mut *mut MORRSLT) -> ::windows_core::HRESULT,
    pub GetConversionModeCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcaps: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPhonetic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, string: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, start: i32, length: i32, phonetic: *mut super::super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPhonetic: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetConversion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, string: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, start: i32, length: i32, result: *mut super::super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetConversion: usize,
}
pub const IGIMIF_RIGHTMENU: u32 = 1u32;
pub const IGIMII_CMODE: u32 = 1u32;
pub const IGIMII_CONFIGURE: u32 = 4u32;
pub const IGIMII_HELP: u32 = 16u32;
pub const IGIMII_INPUTTOOLS: u32 = 64u32;
pub const IGIMII_OTHER: u32 = 32u32;
pub const IGIMII_SMODE: u32 = 2u32;
pub const IGIMII_TOOLS: u32 = 8u32;
#[repr(transparent)]
pub struct IImePad(::windows_core::IUnknown);
impl IImePad {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Request<'a, Param0: ::windows_core::IntoParam<'a, IImePadApplet>, Param2: ::windows_core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param3: ::windows_core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(&self, piimepadapplet: Param0, reqid: IME_PAD_REQUEST_FLAGS, wparam: Param2, lparam: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Request)(::windows_core::Interface::as_raw(self), piimepadapplet.into_param().abi(), ::core::mem::transmute(reqid), wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IImePad> for ::windows_core::IUnknown {
    fn from(value: IImePad) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IImePad> for ::windows_core::IUnknown {
    fn from(value: &IImePad) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IImePad {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IImePad {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IImePad {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IImePad {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IImePad {}
impl ::core::fmt::Debug for IImePad {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IImePad").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IImePad {
    type Vtable = IImePad_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d8e643a_c3a9_11d1_afef_00805f0c8b6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImePad_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piimepadapplet: ::windows_core::RawPtr, reqid: IME_PAD_REQUEST_FLAGS, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Request: usize,
}
#[repr(transparent)]
pub struct IImePadApplet(::windows_core::IUnknown);
impl IImePadApplet {
    pub unsafe fn Initialize<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, lpiimepad: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), lpiimepad.into_param().abi()).ok()
    }
    pub unsafe fn Terminate(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Terminate)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn GetAppletConfig(&self, lpappletcfg: *mut IMEAPPLETCFG) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAppletConfig)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpappletcfg)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateUI<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwndparent: Param0, lpimeappletui: *mut IMEAPPLETUI) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateUI)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), ::core::mem::transmute(lpimeappletui)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Notify<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param2: ::windows_core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param3: ::windows_core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(&self, lpimepad: Param0, notify: i32, wparam: Param2, lparam: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Notify)(::windows_core::Interface::as_raw(self), lpimepad.into_param().abi(), ::core::mem::transmute(notify), wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IImePadApplet> for ::windows_core::IUnknown {
    fn from(value: IImePadApplet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IImePadApplet> for ::windows_core::IUnknown {
    fn from(value: &IImePadApplet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IImePadApplet {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IImePadApplet {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IImePadApplet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IImePadApplet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IImePadApplet {}
impl ::core::fmt::Debug for IImePadApplet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IImePadApplet").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IImePadApplet {
    type Vtable = IImePadApplet_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d8e643b_c3a9_11d1_afef_00805f0c8b6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImePadApplet_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpiimepad: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Terminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub GetAppletConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpappletcfg: *mut IMEAPPLETCFG) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    GetAppletConfig: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::Foundation::HWND, lpimeappletui: *mut IMEAPPLETUI) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateUI: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Notify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpimepad: *mut ::core::ffi::c_void, notify: i32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Notify: usize,
}
#[repr(transparent)]
pub struct IImePlugInDictDictionaryList(::windows_core::IUnknown);
impl IImePlugInDictDictionaryList {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDictionariesInUse(&self, prgdictionaryguid: *mut *mut super::super::super::System::Com::SAFEARRAY, prgdatecreated: *mut *mut super::super::super::System::Com::SAFEARRAY, prgfencrypted: *mut *mut super::super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDictionariesInUse)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prgdictionaryguid), ::core::mem::transmute(prgdatecreated), ::core::mem::transmute(prgfencrypted)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteDictionary<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrdictionaryguid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteDictionary)(::windows_core::Interface::as_raw(self), bstrdictionaryguid.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IImePlugInDictDictionaryList> for ::windows_core::IUnknown {
    fn from(value: IImePlugInDictDictionaryList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IImePlugInDictDictionaryList> for ::windows_core::IUnknown {
    fn from(value: &IImePlugInDictDictionaryList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IImePlugInDictDictionaryList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IImePlugInDictDictionaryList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IImePlugInDictDictionaryList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IImePlugInDictDictionaryList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IImePlugInDictDictionaryList {}
impl ::core::fmt::Debug for IImePlugInDictDictionaryList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IImePlugInDictDictionaryList").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IImePlugInDictDictionaryList {
    type Vtable = IImePlugInDictDictionaryList_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x98752974_b0a6_489b_8f6f_bff3769c8eeb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImePlugInDictDictionaryList_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDictionariesInUse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prgdictionaryguid: *mut *mut super::super::super::System::Com::SAFEARRAY, prgdatecreated: *mut *mut super::super::super::System::Com::SAFEARRAY, prgfencrypted: *mut *mut super::super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDictionariesInUse: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteDictionary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdictionaryguid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteDictionary: usize,
}
#[repr(transparent)]
pub struct IImeSpecifyApplets(::windows_core::IUnknown);
impl IImeSpecifyApplets {
    pub unsafe fn GetAppletIIDList(&self, refiid: *const ::windows_core::GUID, lpiidlist: *mut APPLETIDLIST) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAppletIIDList)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(refiid), ::core::mem::transmute(lpiidlist)).ok()
    }
}
impl ::core::convert::From<IImeSpecifyApplets> for ::windows_core::IUnknown {
    fn from(value: IImeSpecifyApplets) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IImeSpecifyApplets> for ::windows_core::IUnknown {
    fn from(value: &IImeSpecifyApplets) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IImeSpecifyApplets {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IImeSpecifyApplets {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IImeSpecifyApplets {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IImeSpecifyApplets {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IImeSpecifyApplets {}
impl ::core::fmt::Debug for IImeSpecifyApplets {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IImeSpecifyApplets").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IImeSpecifyApplets {
    type Vtable = IImeSpecifyApplets_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d8e643c_c3a9_11d1_afef_00805f0c8b6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImeSpecifyApplets_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetAppletIIDList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, refiid: *const ::windows_core::GUID, lpiidlist: *mut APPLETIDLIST) -> ::windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
pub type IMCENUMPROC = ::core::option::Option<unsafe extern "system" fn(param0: super::super::super::Globalization::HIMC, param1: super::super::super::Foundation::LPARAM) -> super::super::super::Foundation::BOOL>;
pub const IMC_CLOSESTATUSWINDOW: u32 = 33u32;
pub const IMC_GETCANDIDATEPOS: u32 = 7u32;
pub const IMC_GETCOMPOSITIONFONT: u32 = 9u32;
pub const IMC_GETCOMPOSITIONWINDOW: u32 = 11u32;
pub const IMC_GETSOFTKBDFONT: u32 = 17u32;
pub const IMC_GETSOFTKBDPOS: u32 = 19u32;
pub const IMC_GETSOFTKBDSUBTYPE: u32 = 21u32;
pub const IMC_GETSTATUSWINDOWPOS: u32 = 15u32;
pub const IMC_OPENSTATUSWINDOW: u32 = 34u32;
pub const IMC_SETCANDIDATEPOS: u32 = 8u32;
pub const IMC_SETCOMPOSITIONFONT: u32 = 10u32;
pub const IMC_SETCOMPOSITIONWINDOW: u32 = 12u32;
pub const IMC_SETCONVERSIONMODE: u32 = 2u32;
pub const IMC_SETOPENSTATUS: u32 = 6u32;
pub const IMC_SETSENTENCEMODE: u32 = 4u32;
pub const IMC_SETSOFTKBDDATA: u32 = 24u32;
pub const IMC_SETSOFTKBDFONT: u32 = 18u32;
pub const IMC_SETSOFTKBDPOS: u32 = 20u32;
pub const IMC_SETSOFTKBDSUBTYPE: u32 = 22u32;
pub const IMC_SETSTATUSWINDOWPOS: u32 = 16u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct IMEAPPLETCFG {
    pub dwConfig: u32,
    pub wchTitle: [u16; 64],
    pub wchTitleFontFace: [u16; 32],
    pub dwCharSet: u32,
    pub iCategory: i32,
    pub hIcon: super::super::WindowsAndMessaging::HICON,
    pub langID: u16,
    pub dummy: u16,
    pub lReserved1: super::super::super::Foundation::LPARAM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for IMEAPPLETCFG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for IMEAPPLETCFG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for IMEAPPLETCFG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMEAPPLETCFG").field("dwConfig", &self.dwConfig).field("wchTitle", &self.wchTitle).field("wchTitleFontFace", &self.wchTitleFontFace).field("dwCharSet", &self.dwCharSet).field("iCategory", &self.iCategory).field("hIcon", &self.hIcon).field("langID", &self.langID).field("dummy", &self.dummy).field("lReserved1", &self.lReserved1).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for IMEAPPLETCFG {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for IMEAPPLETCFG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEAPPLETCFG>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for IMEAPPLETCFG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for IMEAPPLETCFG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IMEAPPLETUI {
    pub hwnd: super::super::super::Foundation::HWND,
    pub dwStyle: u32,
    pub width: i32,
    pub height: i32,
    pub minWidth: i32,
    pub minHeight: i32,
    pub maxWidth: i32,
    pub maxHeight: i32,
    pub lReserved1: super::super::super::Foundation::LPARAM,
    pub lReserved2: super::super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMEAPPLETUI {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMEAPPLETUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMEAPPLETUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMEAPPLETUI").field("hwnd", &self.hwnd).field("dwStyle", &self.dwStyle).field("width", &self.width).field("height", &self.height).field("minWidth", &self.minWidth).field("minHeight", &self.minHeight).field("maxWidth", &self.maxWidth).field("maxHeight", &self.maxHeight).field("lReserved1", &self.lReserved1).field("lReserved2", &self.lReserved2).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows_core::Abi for IMEAPPLETUI {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMEAPPLETUI {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEAPPLETUI>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMEAPPLETUI {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMEAPPLETUI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct IMECHARINFO {
    pub wch: u16,
    pub dwCharInfo: u32,
}
impl ::core::marker::Copy for IMECHARINFO {}
impl ::core::clone::Clone for IMECHARINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMECHARINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMECHARINFO").field("wch", &self.wch).field("dwCharInfo", &self.dwCharInfo).finish()
    }
}
unsafe impl ::windows_core::Abi for IMECHARINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMECHARINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMECHARINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMECHARINFO {}
impl ::core::default::Default for IMECHARINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IMECHARPOSITION {
    pub dwSize: u32,
    pub dwCharPos: u32,
    pub pt: super::super::super::Foundation::POINT,
    pub cLineHeight: u32,
    pub rcDocument: super::super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMECHARPOSITION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMECHARPOSITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMECHARPOSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMECHARPOSITION").field("dwSize", &self.dwSize).field("dwCharPos", &self.dwCharPos).field("pt", &self.pt).field("cLineHeight", &self.cLineHeight).field("rcDocument", &self.rcDocument).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows_core::Abi for IMECHARPOSITION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMECHARPOSITION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMECHARPOSITION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMECHARPOSITION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMECHARPOSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct IMECOMPOSITIONSTRINGINFO {
    pub iCompStrLen: i32,
    pub iCaretPos: i32,
    pub iEditStart: i32,
    pub iEditLen: i32,
    pub iTargetStart: i32,
    pub iTargetLen: i32,
}
impl ::core::marker::Copy for IMECOMPOSITIONSTRINGINFO {}
impl ::core::clone::Clone for IMECOMPOSITIONSTRINGINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMECOMPOSITIONSTRINGINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMECOMPOSITIONSTRINGINFO").field("iCompStrLen", &self.iCompStrLen).field("iCaretPos", &self.iCaretPos).field("iEditStart", &self.iEditStart).field("iEditLen", &self.iEditLen).field("iTargetStart", &self.iTargetStart).field("iTargetLen", &self.iTargetLen).finish()
    }
}
unsafe impl ::windows_core::Abi for IMECOMPOSITIONSTRINGINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMECOMPOSITIONSTRINGINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMECOMPOSITIONSTRINGINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMECOMPOSITIONSTRINGINFO {}
impl ::core::default::Default for IMECOMPOSITIONSTRINGINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct IMEDLG {
    pub cbIMEDLG: i32,
    pub hwnd: super::super::super::Foundation::HWND,
    pub lpwstrWord: ::windows_core::PWSTR,
    pub nTabId: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMEDLG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMEDLG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows_core::Abi for IMEDLG {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMEDLG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEDLG>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMEDLG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMEDLG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct IMEDP {
    pub wrdModifier: IMEWRD,
    pub wrdModifiee: IMEWRD,
    pub relID: IMEREL,
}
impl ::core::marker::Copy for IMEDP {}
impl ::core::clone::Clone for IMEDP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for IMEDP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMEDP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEDP>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMEDP {}
impl ::core::default::Default for IMEDP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const IMEFAREASTINFO_TYPE_COMMENT: u32 = 2u32;
pub const IMEFAREASTINFO_TYPE_COSTTIME: u32 = 3u32;
pub const IMEFAREASTINFO_TYPE_DEFAULT: u32 = 0u32;
pub const IMEFAREASTINFO_TYPE_READING: u32 = 1u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IMEFMT(pub i32);
pub const IFED_UNKNOWN: IMEFMT = IMEFMT(0i32);
pub const IFED_MSIME2_BIN_SYSTEM: IMEFMT = IMEFMT(1i32);
pub const IFED_MSIME2_BIN_USER: IMEFMT = IMEFMT(2i32);
pub const IFED_MSIME2_TEXT_USER: IMEFMT = IMEFMT(3i32);
pub const IFED_MSIME95_BIN_SYSTEM: IMEFMT = IMEFMT(4i32);
pub const IFED_MSIME95_BIN_USER: IMEFMT = IMEFMT(5i32);
pub const IFED_MSIME95_TEXT_USER: IMEFMT = IMEFMT(6i32);
pub const IFED_MSIME97_BIN_SYSTEM: IMEFMT = IMEFMT(7i32);
pub const IFED_MSIME97_BIN_USER: IMEFMT = IMEFMT(8i32);
pub const IFED_MSIME97_TEXT_USER: IMEFMT = IMEFMT(9i32);
pub const IFED_MSIME98_BIN_SYSTEM: IMEFMT = IMEFMT(10i32);
pub const IFED_MSIME98_BIN_USER: IMEFMT = IMEFMT(11i32);
pub const IFED_MSIME98_TEXT_USER: IMEFMT = IMEFMT(12i32);
pub const IFED_ACTIVE_DICT: IMEFMT = IMEFMT(13i32);
pub const IFED_ATOK9: IMEFMT = IMEFMT(14i32);
pub const IFED_ATOK10: IMEFMT = IMEFMT(15i32);
pub const IFED_NEC_AI_: IMEFMT = IMEFMT(16i32);
pub const IFED_WX_II: IMEFMT = IMEFMT(17i32);
pub const IFED_WX_III: IMEFMT = IMEFMT(18i32);
pub const IFED_VJE_20: IMEFMT = IMEFMT(19i32);
pub const IFED_MSIME98_SYSTEM_CE: IMEFMT = IMEFMT(20i32);
pub const IFED_MSIME_BIN_SYSTEM: IMEFMT = IMEFMT(21i32);
pub const IFED_MSIME_BIN_USER: IMEFMT = IMEFMT(22i32);
pub const IFED_MSIME_TEXT_USER: IMEFMT = IMEFMT(23i32);
pub const IFED_PIME2_BIN_USER: IMEFMT = IMEFMT(24i32);
pub const IFED_PIME2_BIN_SYSTEM: IMEFMT = IMEFMT(25i32);
pub const IFED_PIME2_BIN_STANDARD_SYSTEM: IMEFMT = IMEFMT(26i32);
impl ::core::marker::Copy for IMEFMT {}
impl ::core::clone::Clone for IMEFMT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMEFMT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for IMEFMT {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMEFMT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMEFMT").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct IMEINFO {
    pub dwPrivateDataSize: u32,
    pub fdwProperty: u32,
    pub fdwConversionCaps: u32,
    pub fdwSentenceCaps: u32,
    pub fdwUICaps: u32,
    pub fdwSCSCaps: u32,
    pub fdwSelectCaps: u32,
}
impl ::core::marker::Copy for IMEINFO {}
impl ::core::clone::Clone for IMEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMEINFO").field("dwPrivateDataSize", &self.dwPrivateDataSize).field("fdwProperty", &self.fdwProperty).field("fdwConversionCaps", &self.fdwConversionCaps).field("fdwSentenceCaps", &self.fdwSentenceCaps).field("fdwUICaps", &self.fdwUICaps).field("fdwSCSCaps", &self.fdwSCSCaps).field("fdwSelectCaps", &self.fdwSelectCaps).finish()
    }
}
unsafe impl ::windows_core::Abi for IMEINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMEINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMEINFO {}
impl ::core::default::Default for IMEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct IMEITEM {
    pub cbSize: i32,
    pub iType: i32,
    pub lpItemData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for IMEITEM {}
impl ::core::clone::Clone for IMEITEM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMEITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMEITEM").field("cbSize", &self.cbSize).field("iType", &self.iType).field("lpItemData", &self.lpItemData).finish()
    }
}
unsafe impl ::windows_core::Abi for IMEITEM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMEITEM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEITEM>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMEITEM {}
impl ::core::default::Default for IMEITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct IMEITEMCANDIDATE {
    pub uCount: u32,
    pub imeItem: [IMEITEM; 1],
}
impl ::core::marker::Copy for IMEITEMCANDIDATE {}
impl ::core::clone::Clone for IMEITEMCANDIDATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMEITEMCANDIDATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMEITEMCANDIDATE").field("uCount", &self.uCount).field("imeItem", &self.imeItem).finish()
    }
}
unsafe impl ::windows_core::Abi for IMEITEMCANDIDATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMEITEMCANDIDATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEITEMCANDIDATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMEITEMCANDIDATE {}
impl ::core::default::Default for IMEITEMCANDIDATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const IMEKEYCTRLMASK_ALT: u32 = 1u32;
pub const IMEKEYCTRLMASK_CTRL: u32 = 2u32;
pub const IMEKEYCTRLMASK_SHIFT: u32 = 4u32;
pub const IMEKEYCTRL_DOWN: u32 = 0u32;
pub const IMEKEYCTRL_UP: u32 = 1u32;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Globalization")]
pub struct IMEKMS {
    pub cbSize: i32,
    pub hIMC: super::super::super::Globalization::HIMC,
    pub cKeyList: u32,
    pub pKeyList: *mut IMEKMSKEY,
}
#[cfg(feature = "Win32_Globalization")]
impl ::core::marker::Copy for IMEKMS {}
#[cfg(feature = "Win32_Globalization")]
impl ::core::clone::Clone for IMEKMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Globalization")]
unsafe impl ::windows_core::Abi for IMEKMS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Globalization")]
impl ::core::cmp::PartialEq for IMEKMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEKMS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Globalization")]
impl ::core::cmp::Eq for IMEKMS {}
#[cfg(feature = "Win32_Globalization")]
impl ::core::default::Default for IMEKMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct IMEKMSFUNCDESC {
    pub cbSize: i32,
    pub idLang: u16,
    pub dwControl: u32,
    pub pwszDescription: [u16; 128],
}
impl ::core::marker::Copy for IMEKMSFUNCDESC {}
impl ::core::clone::Clone for IMEKMSFUNCDESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for IMEKMSFUNCDESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMEKMSFUNCDESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEKMSFUNCDESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMEKMSFUNCDESC {}
impl ::core::default::Default for IMEKMSFUNCDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct IMEKMSINIT {
    pub cbSize: i32,
    pub hWnd: super::super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMEKMSINIT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMEKMSINIT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows_core::Abi for IMEKMSINIT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMEKMSINIT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEKMSINIT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMEKMSINIT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMEKMSINIT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Globalization")]
pub struct IMEKMSINVK {
    pub cbSize: i32,
    pub hIMC: super::super::super::Globalization::HIMC,
    pub dwControl: u32,
}
#[cfg(feature = "Win32_Globalization")]
impl ::core::marker::Copy for IMEKMSINVK {}
#[cfg(feature = "Win32_Globalization")]
impl ::core::clone::Clone for IMEKMSINVK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Globalization")]
unsafe impl ::windows_core::Abi for IMEKMSINVK {
    type Abi = Self;
}
#[cfg(feature = "Win32_Globalization")]
impl ::core::cmp::PartialEq for IMEKMSINVK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEKMSINVK>()) == 0 }
    }
}
#[cfg(feature = "Win32_Globalization")]
impl ::core::cmp::Eq for IMEKMSINVK {}
#[cfg(feature = "Win32_Globalization")]
impl ::core::default::Default for IMEKMSINVK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct IMEKMSKEY {
    pub dwStatus: u32,
    pub dwCompStatus: u32,
    pub dwVKEY: u32,
    pub Anonymous1: IMEKMSKEY_0,
    pub Anonymous2: IMEKMSKEY_1,
}
impl ::core::marker::Copy for IMEKMSKEY {}
impl ::core::clone::Clone for IMEKMSKEY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for IMEKMSKEY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMEKMSKEY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEKMSKEY>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMEKMSKEY {}
impl ::core::default::Default for IMEKMSKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub union IMEKMSKEY_0 {
    pub dwControl: u32,
    pub dwNotUsed: u32,
}
impl ::core::marker::Copy for IMEKMSKEY_0 {}
impl ::core::clone::Clone for IMEKMSKEY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for IMEKMSKEY_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMEKMSKEY_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEKMSKEY_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMEKMSKEY_0 {}
impl ::core::default::Default for IMEKMSKEY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub union IMEKMSKEY_1 {
    pub pwszDscr: [u16; 31],
    pub pwszNoUse: [u16; 31],
}
impl ::core::marker::Copy for IMEKMSKEY_1 {}
impl ::core::clone::Clone for IMEKMSKEY_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for IMEKMSKEY_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMEKMSKEY_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEKMSKEY_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMEKMSKEY_1 {}
impl ::core::default::Default for IMEKMSKEY_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Globalization")]
pub struct IMEKMSKMP {
    pub cbSize: i32,
    pub hIMC: super::super::super::Globalization::HIMC,
    pub idLang: u16,
    pub wVKStart: u16,
    pub wVKEnd: u16,
    pub cKeyList: i32,
    pub pKeyList: *mut IMEKMSKEY,
}
#[cfg(feature = "Win32_Globalization")]
impl ::core::marker::Copy for IMEKMSKMP {}
#[cfg(feature = "Win32_Globalization")]
impl ::core::clone::Clone for IMEKMSKMP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Globalization")]
unsafe impl ::windows_core::Abi for IMEKMSKMP {
    type Abi = Self;
}
#[cfg(feature = "Win32_Globalization")]
impl ::core::cmp::PartialEq for IMEKMSKMP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEKMSKMP>()) == 0 }
    }
}
#[cfg(feature = "Win32_Globalization")]
impl ::core::cmp::Eq for IMEKMSKMP {}
#[cfg(feature = "Win32_Globalization")]
impl ::core::default::Default for IMEKMSKMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
pub struct IMEKMSNTFY {
    pub cbSize: i32,
    pub hIMC: super::super::super::Globalization::HIMC,
    pub fSelect: super::super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
impl ::core::marker::Copy for IMEKMSNTFY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
impl ::core::clone::Clone for IMEKMSNTFY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
unsafe impl ::windows_core::Abi for IMEKMSNTFY {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
impl ::core::cmp::PartialEq for IMEKMSNTFY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEKMSNTFY>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
impl ::core::cmp::Eq for IMEKMSNTFY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
impl ::core::default::Default for IMEKMSNTFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const IMEKMS_2NDLEVEL: u32 = 4u32;
pub const IMEKMS_CANDIDATE: u32 = 6u32;
pub const IMEKMS_COMPOSITION: u32 = 1u32;
pub const IMEKMS_IMEOFF: u32 = 3u32;
pub const IMEKMS_INPTGL: u32 = 5u32;
pub const IMEKMS_NOCOMPOSITION: u32 = 0u32;
pub const IMEKMS_SELECTION: u32 = 2u32;
pub const IMEKMS_TYPECAND: u32 = 7u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct IMEMENUITEMINFOA {
    pub cbSize: u32,
    pub fType: u32,
    pub fState: u32,
    pub wID: u32,
    pub hbmpChecked: super::super::super::Graphics::Gdi::HBITMAP,
    pub hbmpUnchecked: super::super::super::Graphics::Gdi::HBITMAP,
    pub dwItemData: u32,
    pub szString: [super::super::super::Foundation::CHAR; 80],
    pub hbmpItem: super::super::super::Graphics::Gdi::HBITMAP,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for IMEMENUITEMINFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for IMEMENUITEMINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for IMEMENUITEMINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMEMENUITEMINFOA").field("cbSize", &self.cbSize).field("fType", &self.fType).field("fState", &self.fState).field("wID", &self.wID).field("hbmpChecked", &self.hbmpChecked).field("hbmpUnchecked", &self.hbmpUnchecked).field("dwItemData", &self.dwItemData).field("szString", &self.szString).field("hbmpItem", &self.hbmpItem).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows_core::Abi for IMEMENUITEMINFOA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for IMEMENUITEMINFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEMENUITEMINFOA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for IMEMENUITEMINFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for IMEMENUITEMINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct IMEMENUITEMINFOW {
    pub cbSize: u32,
    pub fType: u32,
    pub fState: u32,
    pub wID: u32,
    pub hbmpChecked: super::super::super::Graphics::Gdi::HBITMAP,
    pub hbmpUnchecked: super::super::super::Graphics::Gdi::HBITMAP,
    pub dwItemData: u32,
    pub szString: [u16; 80],
    pub hbmpItem: super::super::super::Graphics::Gdi::HBITMAP,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for IMEMENUITEMINFOW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for IMEMENUITEMINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for IMEMENUITEMINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMEMENUITEMINFOW").field("cbSize", &self.cbSize).field("fType", &self.fType).field("fState", &self.fState).field("wID", &self.wID).field("hbmpChecked", &self.hbmpChecked).field("hbmpUnchecked", &self.hbmpUnchecked).field("dwItemData", &self.dwItemData).field("szString", &self.szString).field("hbmpItem", &self.hbmpItem).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for IMEMENUITEMINFOW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for IMEMENUITEMINFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEMENUITEMINFOW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for IMEMENUITEMINFOW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for IMEMENUITEMINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const IMEMENUITEM_STRING_SIZE: u32 = 80u32;
pub const IMEMOUSERET_NOTHANDLED: i32 = -1i32;
pub const IMEMOUSE_LDOWN: u32 = 1u32;
pub const IMEMOUSE_MDOWN: u32 = 4u32;
pub const IMEMOUSE_NONE: u32 = 0u32;
pub const IMEMOUSE_RDOWN: u32 = 2u32;
pub const IMEMOUSE_VERSION: u32 = 255u32;
pub const IMEMOUSE_WDOWN: u32 = 32u32;
pub const IMEMOUSE_WUP: u32 = 16u32;
pub const IMEPADCTRL_CARETBACKSPACE: u32 = 10u32;
pub const IMEPADCTRL_CARETBOTTOM: u32 = 9u32;
pub const IMEPADCTRL_CARETDELETE: u32 = 11u32;
pub const IMEPADCTRL_CARETLEFT: u32 = 6u32;
pub const IMEPADCTRL_CARETRIGHT: u32 = 7u32;
pub const IMEPADCTRL_CARETSET: u32 = 5u32;
pub const IMEPADCTRL_CARETTOP: u32 = 8u32;
pub const IMEPADCTRL_CLEARALL: u32 = 4u32;
pub const IMEPADCTRL_CONVERTALL: u32 = 1u32;
pub const IMEPADCTRL_DETERMINALL: u32 = 2u32;
pub const IMEPADCTRL_DETERMINCHAR: u32 = 3u32;
pub const IMEPADCTRL_INSERTFULLSPACE: u32 = 14u32;
pub const IMEPADCTRL_INSERTHALFSPACE: u32 = 15u32;
pub const IMEPADCTRL_INSERTSPACE: u32 = 13u32;
pub const IMEPADCTRL_OFFIME: u32 = 17u32;
pub const IMEPADCTRL_OFFPRECONVERSION: u32 = 19u32;
pub const IMEPADCTRL_ONIME: u32 = 16u32;
pub const IMEPADCTRL_ONPRECONVERSION: u32 = 18u32;
pub const IMEPADCTRL_PHONETICCANDIDATE: u32 = 20u32;
pub const IMEPADCTRL_PHRASEDELETE: u32 = 12u32;
pub const IMEPADREQ_CHANGESTRINGCANDIDATEINFO: u32 = 4111u32;
pub const IMEPADREQ_CHANGESTRINGINFO: u32 = 4115u32;
pub const IMEPADREQ_FIRST: u32 = 4096u32;
pub const IMEPADREQ_GETAPPLETDATA: u32 = 4106u32;
pub const IMEPADREQ_GETCOMPOSITIONSTRINGID: u32 = 4109u32;
pub const IMEPADREQ_GETCURRENTUILANGID: u32 = 4120u32;
pub const IMEPADREQ_GETSELECTEDSTRING: u32 = 4103u32;
pub const IMEPADREQ_INSERTITEMCANDIDATE: u32 = 4099u32;
pub const IMEPADREQ_INSERTSTRINGCANDIDATE: u32 = 4098u32;
pub const IMEPADREQ_INSERTSTRINGCANDIDATEINFO: u32 = 4110u32;
pub const IMEPADREQ_INSERTSTRINGINFO: u32 = 4114u32;
pub const IMEPADREQ_SENDKEYCONTROL: u32 = 4101u32;
pub const IMEPADREQ_SETAPPLETDATA: u32 = 4105u32;
pub const IMEPADREQ_SETTITLEFONT: u32 = 4107u32;
pub const IMEPN_ACTIVATE: u32 = 257u32;
pub const IMEPN_APPLYCAND: u32 = 267u32;
pub const IMEPN_APPLYCANDEX: u32 = 268u32;
pub const IMEPN_CONFIG: u32 = 264u32;
pub const IMEPN_FIRST: u32 = 256u32;
pub const IMEPN_HELP: u32 = 265u32;
pub const IMEPN_HIDE: u32 = 261u32;
pub const IMEPN_INACTIVATE: u32 = 258u32;
pub const IMEPN_QUERYCAND: u32 = 266u32;
pub const IMEPN_SETTINGCHANGED: u32 = 269u32;
pub const IMEPN_SHOW: u32 = 260u32;
pub const IMEPN_SIZECHANGED: u32 = 263u32;
pub const IMEPN_SIZECHANGING: u32 = 262u32;
pub const IMEPN_USER: u32 = 356u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IMEREG(pub i32);
pub const IFED_REG_HEAD: IMEREG = IMEREG(0i32);
pub const IFED_REG_TAIL: IMEREG = IMEREG(1i32);
pub const IFED_REG_DEL: IMEREG = IMEREG(2i32);
impl ::core::marker::Copy for IMEREG {}
impl ::core::clone::Clone for IMEREG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMEREG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for IMEREG {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMEREG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMEREG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IMEREL(pub i32);
pub const IFED_REL_NONE: IMEREL = IMEREL(0i32);
pub const IFED_REL_NO: IMEREL = IMEREL(1i32);
pub const IFED_REL_GA: IMEREL = IMEREL(2i32);
pub const IFED_REL_WO: IMEREL = IMEREL(3i32);
pub const IFED_REL_NI: IMEREL = IMEREL(4i32);
pub const IFED_REL_DE: IMEREL = IMEREL(5i32);
pub const IFED_REL_YORI: IMEREL = IMEREL(6i32);
pub const IFED_REL_KARA: IMEREL = IMEREL(7i32);
pub const IFED_REL_MADE: IMEREL = IMEREL(8i32);
pub const IFED_REL_HE: IMEREL = IMEREL(9i32);
pub const IFED_REL_TO: IMEREL = IMEREL(10i32);
pub const IFED_REL_IDEOM: IMEREL = IMEREL(11i32);
pub const IFED_REL_FUKU_YOUGEN: IMEREL = IMEREL(12i32);
pub const IFED_REL_KEIYOU_YOUGEN: IMEREL = IMEREL(13i32);
pub const IFED_REL_KEIDOU1_YOUGEN: IMEREL = IMEREL(14i32);
pub const IFED_REL_KEIDOU2_YOUGEN: IMEREL = IMEREL(15i32);
pub const IFED_REL_TAIGEN: IMEREL = IMEREL(16i32);
pub const IFED_REL_YOUGEN: IMEREL = IMEREL(17i32);
pub const IFED_REL_RENTAI_MEI: IMEREL = IMEREL(18i32);
pub const IFED_REL_RENSOU: IMEREL = IMEREL(19i32);
pub const IFED_REL_KEIYOU_TO_YOUGEN: IMEREL = IMEREL(20i32);
pub const IFED_REL_KEIYOU_TARU_YOUGEN: IMEREL = IMEREL(21i32);
pub const IFED_REL_UNKNOWN1: IMEREL = IMEREL(22i32);
pub const IFED_REL_UNKNOWN2: IMEREL = IMEREL(23i32);
pub const IFED_REL_ALL: IMEREL = IMEREL(24i32);
impl ::core::marker::Copy for IMEREL {}
impl ::core::clone::Clone for IMEREL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMEREL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for IMEREL {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMEREL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMEREL").field(&self.0).finish()
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct IMESHF {
    pub cbShf: u16,
    pub verDic: u16,
    pub szTitle: [super::super::super::Foundation::CHAR; 48],
    pub szDescription: [super::super::super::Foundation::CHAR; 256],
    pub szCopyright: [super::super::super::Foundation::CHAR; 128],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMESHF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMESHF {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows_core::Abi for IMESHF {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMESHF {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMESHF>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMESHF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMESHF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct IMESTRINGCANDIDATE {
    pub uCount: u32,
    pub lpwstr: [::windows_core::PWSTR; 1],
}
impl ::core::marker::Copy for IMESTRINGCANDIDATE {}
impl ::core::clone::Clone for IMESTRINGCANDIDATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMESTRINGCANDIDATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMESTRINGCANDIDATE").field("uCount", &self.uCount).field("lpwstr", &self.lpwstr).finish()
    }
}
unsafe impl ::windows_core::Abi for IMESTRINGCANDIDATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMESTRINGCANDIDATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMESTRINGCANDIDATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMESTRINGCANDIDATE {}
impl ::core::default::Default for IMESTRINGCANDIDATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct IMESTRINGCANDIDATEINFO {
    pub dwFarEastId: u32,
    pub lpFarEastInfo: *mut tabIMEFAREASTINFO,
    pub fInfoMask: u32,
    pub iSelIndex: i32,
    pub uCount: u32,
    pub lpwstr: [::windows_core::PWSTR; 1],
}
impl ::core::marker::Copy for IMESTRINGCANDIDATEINFO {}
impl ::core::clone::Clone for IMESTRINGCANDIDATEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMESTRINGCANDIDATEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMESTRINGCANDIDATEINFO").field("dwFarEastId", &self.dwFarEastId).field("lpFarEastInfo", &self.lpFarEastInfo).field("fInfoMask", &self.fInfoMask).field("iSelIndex", &self.iSelIndex).field("uCount", &self.uCount).field("lpwstr", &self.lpwstr).finish()
    }
}
unsafe impl ::windows_core::Abi for IMESTRINGCANDIDATEINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMESTRINGCANDIDATEINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMESTRINGCANDIDATEINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMESTRINGCANDIDATEINFO {}
impl ::core::default::Default for IMESTRINGCANDIDATEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IMEUCT(pub i32);
pub const IFED_UCT_NONE: IMEUCT = IMEUCT(0i32);
pub const IFED_UCT_STRING_SJIS: IMEUCT = IMEUCT(1i32);
pub const IFED_UCT_STRING_UNICODE: IMEUCT = IMEUCT(2i32);
pub const IFED_UCT_USER_DEFINED: IMEUCT = IMEUCT(3i32);
pub const IFED_UCT_MAX: IMEUCT = IMEUCT(4i32);
impl ::core::marker::Copy for IMEUCT {}
impl ::core::clone::Clone for IMEUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMEUCT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for IMEUCT {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMEUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMEUCT").field(&self.0).finish()
    }
}
pub const IMEVER_0310: u32 = 196618u32;
pub const IMEVER_0400: u32 = 262144u32;
#[repr(C, packed(1))]
pub struct IMEWRD {
    pub pwchReading: ::windows_core::PWSTR,
    pub pwchDisplay: ::windows_core::PWSTR,
    pub Anonymous: IMEWRD_0,
    pub rgulAttrs: [u32; 2],
    pub cbComment: i32,
    pub uct: IMEUCT,
    pub pvComment: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for IMEWRD {}
impl ::core::clone::Clone for IMEWRD {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for IMEWRD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMEWRD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEWRD>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMEWRD {}
impl ::core::default::Default for IMEWRD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub union IMEWRD_0 {
    pub ulPos: u32,
    pub Anonymous: IMEWRD_0_0,
}
impl ::core::marker::Copy for IMEWRD_0 {}
impl ::core::clone::Clone for IMEWRD_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for IMEWRD_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMEWRD_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEWRD_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMEWRD_0 {}
impl ::core::default::Default for IMEWRD_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct IMEWRD_0_0 {
    pub nPos1: u16,
    pub nPos2: u16,
}
impl ::core::marker::Copy for IMEWRD_0_0 {}
impl ::core::clone::Clone for IMEWRD_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for IMEWRD_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMEWRD_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEWRD_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMEWRD_0_0 {}
impl ::core::default::Default for IMEWRD_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const IME_CAND_CODE: u32 = 2u32;
pub const IME_CAND_MEANING: u32 = 3u32;
pub const IME_CAND_RADICAL: u32 = 4u32;
pub const IME_CAND_READ: u32 = 1u32;
pub const IME_CAND_STROKE: u32 = 5u32;
pub const IME_CAND_UNKNOWN: u32 = 0u32;
pub const IME_CHOTKEY_IME_NONIME_TOGGLE: u32 = 16u32;
pub const IME_CHOTKEY_SHAPE_TOGGLE: u32 = 17u32;
pub const IME_CHOTKEY_SYMBOL_TOGGLE: u32 = 18u32;
pub const IME_CMODE_EUDC: u32 = 512u32;
pub const IME_CMODE_FIXED: u32 = 2048u32;
pub const IME_CMODE_NOCONVERSION: u32 = 256u32;
pub const IME_CMODE_RESERVED: u32 = 4026531840u32;
pub const IME_CMODE_SOFTKBD: u32 = 128u32;
pub const IME_CMODE_SYMBOL: u32 = 1024u32;
pub const IME_CONFIG_GENERAL: u32 = 1u32;
pub const IME_CONFIG_REGISTERWORD: u32 = 2u32;
pub const IME_CONFIG_SELECTDICTIONARY: u32 = 3u32;
pub const IME_ESC_AUTOMATA: u32 = 4105u32;
pub const IME_ESC_GETHELPFILENAME: u32 = 4107u32;
pub const IME_ESC_GET_EUDC_DICTIONARY: u32 = 4099u32;
pub const IME_ESC_HANJA_MODE: u32 = 4104u32;
pub const IME_ESC_IME_NAME: u32 = 4102u32;
pub const IME_ESC_MAX_KEY: u32 = 4101u32;
pub const IME_ESC_PRIVATE_FIRST: u32 = 2048u32;
pub const IME_ESC_PRIVATE_HOTKEY: u32 = 4106u32;
pub const IME_ESC_PRIVATE_LAST: u32 = 4095u32;
pub const IME_ESC_QUERY_SUPPORT: u32 = 3u32;
pub const IME_ESC_RESERVED_FIRST: u32 = 4u32;
pub const IME_ESC_RESERVED_LAST: u32 = 2047u32;
pub const IME_ESC_SEQUENCE_TO_INTERNAL: u32 = 4097u32;
pub const IME_ESC_SET_EUDC_DICTIONARY: u32 = 4100u32;
pub const IME_ESC_STRING_BUFFER_SIZE: u32 = 80u32;
pub const IME_ESC_SYNC_HOTKEY: u32 = 4103u32;
pub const IME_HOTKEY_DSWITCH_FIRST: u32 = 256u32;
pub const IME_HOTKEY_DSWITCH_LAST: u32 = 287u32;
pub const IME_HOTKEY_PRIVATE_FIRST: u32 = 512u32;
pub const IME_HOTKEY_PRIVATE_LAST: u32 = 543u32;
pub const IME_ITHOTKEY_PREVIOUS_COMPOSITION: u32 = 513u32;
pub const IME_ITHOTKEY_RECONVERTSTRING: u32 = 515u32;
pub const IME_ITHOTKEY_RESEND_RESULTSTR: u32 = 512u32;
pub const IME_ITHOTKEY_UISTYLE_TOGGLE: u32 = 514u32;
pub const IME_JHOTKEY_CLOSE_OPEN: u32 = 48u32;
pub const IME_KHOTKEY_ENGLISH: u32 = 82u32;
pub const IME_KHOTKEY_HANJACONVERT: u32 = 81u32;
pub const IME_KHOTKEY_SHAPE_TOGGLE: u32 = 80u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IME_PAD_REQUEST_FLAGS(pub u32);
pub const IMEPADREQ_INSERTSTRING: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4097u32);
pub const IMEPADREQ_SENDCONTROL: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4100u32);
pub const IMEPADREQ_SETAPPLETSIZE: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4104u32);
pub const IMEPADREQ_GETCOMPOSITIONSTRING: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4102u32);
pub const IMEPADREQ_GETCOMPOSITIONSTRINGINFO: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4108u32);
pub const IMEPADREQ_DELETESTRING: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4112u32);
pub const IMEPADREQ_CHANGESTRING: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4113u32);
pub const IMEPADREQ_GETAPPLHWND: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4116u32);
pub const IMEPADREQ_FORCEIMEPADWINDOWSHOW: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4117u32);
pub const IMEPADREQ_POSTMODALNOTIFY: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4118u32);
pub const IMEPADREQ_GETDEFAULTUILANGID: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4119u32);
pub const IMEPADREQ_GETAPPLETUISTYLE: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4121u32);
pub const IMEPADREQ_SETAPPLETUISTYLE: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4122u32);
pub const IMEPADREQ_ISAPPLETACTIVE: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4123u32);
pub const IMEPADREQ_ISIMEPADWINDOWVISIBLE: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4124u32);
pub const IMEPADREQ_SETAPPLETMINMAXSIZE: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4125u32);
pub const IMEPADREQ_GETCONVERSIONSTATUS: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4126u32);
pub const IMEPADREQ_GETVERSION: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4127u32);
pub const IMEPADREQ_GETCURRENTIMEINFO: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4128u32);
impl ::core::marker::Copy for IME_PAD_REQUEST_FLAGS {}
impl ::core::clone::Clone for IME_PAD_REQUEST_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IME_PAD_REQUEST_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for IME_PAD_REQUEST_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for IME_PAD_REQUEST_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IME_PAD_REQUEST_FLAGS").field(&self.0).finish()
    }
}
pub const IME_PROP_ACCEPT_WIDE_VKEY: u32 = 32u32;
pub const IME_PROP_AT_CARET: u32 = 65536u32;
pub const IME_PROP_CANDLIST_START_FROM_1: u32 = 262144u32;
pub const IME_PROP_COMPLETE_ON_UNSELECT: u32 = 1048576u32;
pub const IME_PROP_END_UNLOAD: u32 = 1u32;
pub const IME_PROP_IGNORE_UPKEYS: u32 = 4u32;
pub const IME_PROP_KBD_CHAR_FIRST: u32 = 2u32;
pub const IME_PROP_NEED_ALTKEY: u32 = 8u32;
pub const IME_PROP_NO_KEYS_ON_CLOSE: u32 = 16u32;
pub const IME_PROP_SPECIAL_UI: u32 = 131072u32;
pub const IME_PROP_UNICODE: u32 = 524288u32;
pub const IME_REGWORD_STYLE_EUDC: u32 = 1u32;
pub const IME_REGWORD_STYLE_USER_FIRST: u32 = 2147483648u32;
pub const IME_REGWORD_STYLE_USER_LAST: u32 = 4294967295u32;
pub const IME_SMODE_AUTOMATIC: u32 = 4u32;
pub const IME_SMODE_CONVERSATION: u32 = 16u32;
pub const IME_SMODE_NONE: u32 = 0u32;
pub const IME_SMODE_PHRASEPREDICT: u32 = 8u32;
pub const IME_SMODE_PLAURALCLAUSE: u32 = 1u32;
pub const IME_SMODE_RESERVED: u32 = 61440u32;
pub const IME_SMODE_SINGLECONVERT: u32 = 2u32;
pub const IME_SYSINFO_WINLOGON: u32 = 1u32;
pub const IME_SYSINFO_WOW16: u32 = 2u32;
pub const IME_THOTKEY_IME_NONIME_TOGGLE: u32 = 112u32;
pub const IME_THOTKEY_SHAPE_TOGGLE: u32 = 113u32;
pub const IME_THOTKEY_SYMBOL_TOGGLE: u32 = 114u32;
pub const IME_UI_CLASS_NAME_SIZE: u32 = 16u32;
pub const IMFT_RADIOCHECK: u32 = 1u32;
pub const IMFT_SEPARATOR: u32 = 2u32;
pub const IMFT_SUBMENU: u32 = 4u32;
pub const IMMGWLP_IMC: u32 = 0u32;
pub const IMMGWL_IMC: u32 = 0u32;
pub const IMM_ERROR_GENERAL: i32 = -2i32;
pub const IMM_ERROR_NODATA: i32 = -1i32;
pub const IMN_CHANGECANDIDATE: u32 = 3u32;
pub const IMN_CLOSECANDIDATE: u32 = 4u32;
pub const IMN_CLOSESTATUSWINDOW: u32 = 1u32;
pub const IMN_GUIDELINE: u32 = 13u32;
pub const IMN_OPENCANDIDATE: u32 = 5u32;
pub const IMN_OPENSTATUSWINDOW: u32 = 2u32;
pub const IMN_PRIVATE: u32 = 14u32;
pub const IMN_SETCANDIDATEPOS: u32 = 9u32;
pub const IMN_SETCOMPOSITIONFONT: u32 = 10u32;
pub const IMN_SETCOMPOSITIONWINDOW: u32 = 11u32;
pub const IMN_SETCONVERSIONMODE: u32 = 6u32;
pub const IMN_SETOPENSTATUS: u32 = 8u32;
pub const IMN_SETSENTENCEMODE: u32 = 7u32;
pub const IMN_SETSTATUSWINDOWPOS: u32 = 12u32;
pub const IMN_SOFTKBDDESTROYED: u32 = 17u32;
pub const IMR_CANDIDATEWINDOW: u32 = 2u32;
pub const IMR_COMPOSITIONFONT: u32 = 3u32;
pub const IMR_COMPOSITIONWINDOW: u32 = 1u32;
pub const IMR_CONFIRMRECONVERTSTRING: u32 = 5u32;
pub const IMR_DOCUMENTFEED: u32 = 7u32;
pub const IMR_QUERYCHARPOSITION: u32 = 6u32;
pub const IMR_RECONVERTSTRING: u32 = 4u32;
pub const INFOMASK_APPLY_CAND: u32 = 2u32;
pub const INFOMASK_APPLY_CAND_EX: u32 = 4u32;
pub const INFOMASK_BLOCK_CAND: u32 = 262144u32;
pub const INFOMASK_HIDE_CAND: u32 = 131072u32;
pub const INFOMASK_NONE: u32 = 0u32;
pub const INFOMASK_QUERY_CAND: u32 = 1u32;
pub const INFOMASK_STRING_FIX: u32 = 65536u32;
pub const INIT_COMPFORM: u32 = 16u32;
pub const INIT_CONVERSION: u32 = 2u32;
pub const INIT_LOGFONT: u32 = 8u32;
pub const INIT_SENTENCE: u32 = 4u32;
pub const INIT_SOFTKBDPOS: u32 = 32u32;
pub const INIT_STATUSWNDPOS: u32 = 1u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
pub struct INPUTCONTEXT {
    pub hWnd: super::super::super::Foundation::HWND,
    pub fOpen: super::super::super::Foundation::BOOL,
    pub ptStatusWndPos: super::super::super::Foundation::POINT,
    pub ptSoftKbdPos: super::super::super::Foundation::POINT,
    pub fdwConversion: u32,
    pub fdwSentence: u32,
    pub lfFont: INPUTCONTEXT_0,
    pub cfCompForm: COMPOSITIONFORM,
    pub cfCandForm: [CANDIDATEFORM; 4],
    pub hCompStr: super::super::super::Globalization::HIMCC,
    pub hCandInfo: super::super::super::Globalization::HIMCC,
    pub hGuideLine: super::super::super::Globalization::HIMCC,
    pub hPrivate: super::super::super::Globalization::HIMCC,
    pub dwNumMsgBuf: u32,
    pub hMsgBuf: super::super::super::Globalization::HIMCC,
    pub fdwInit: u32,
    pub dwReserve: [u32; 3],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for INPUTCONTEXT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for INPUTCONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows_core::Abi for INPUTCONTEXT {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for INPUTCONTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INPUTCONTEXT>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for INPUTCONTEXT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for INPUTCONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
pub union INPUTCONTEXT_0 {
    pub A: super::super::super::Graphics::Gdi::LOGFONTA,
    pub W: super::super::super::Graphics::Gdi::LOGFONTW,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for INPUTCONTEXT_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for INPUTCONTEXT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows_core::Abi for INPUTCONTEXT_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for INPUTCONTEXT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INPUTCONTEXT_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for INPUTCONTEXT_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for INPUTCONTEXT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const IPACFG_CATEGORY: i32 = 262144i32;
pub const IPACFG_HELP: i32 = 2i32;
pub const IPACFG_LANG: i32 = 16i32;
pub const IPACFG_NONE: i32 = 0i32;
pub const IPACFG_PROPERTY: i32 = 1i32;
pub const IPACFG_TITLE: i32 = 65536i32;
pub const IPACFG_TITLEFONTFACE: i32 = 131072i32;
pub const IPACID_CHARLIST: u32 = 9u32;
pub const IPACID_EPWING: u32 = 7u32;
pub const IPACID_HANDWRITING: u32 = 2u32;
pub const IPACID_NONE: u32 = 0u32;
pub const IPACID_OCR: u32 = 8u32;
pub const IPACID_RADICALSEARCH: u32 = 4u32;
pub const IPACID_SOFTKEY: u32 = 1u32;
pub const IPACID_STROKESEARCH: u32 = 3u32;
pub const IPACID_SYMBOLSEARCH: u32 = 5u32;
pub const IPACID_USER: u32 = 256u32;
pub const IPACID_VOICE: u32 = 6u32;
pub const IPAWS_ENABLED: i32 = 1i32;
pub const IPAWS_HORIZONTALFIXED: i32 = 512i32;
pub const IPAWS_MAXHEIGHTFIXED: i32 = 8192i32;
pub const IPAWS_MAXSIZEFIXED: i32 = 12288i32;
pub const IPAWS_MAXWIDTHFIXED: i32 = 4096i32;
pub const IPAWS_MINHEIGHTFIXED: i32 = 131072i32;
pub const IPAWS_MINSIZEFIXED: i32 = 196608i32;
pub const IPAWS_MINWIDTHFIXED: i32 = 65536i32;
pub const IPAWS_SIZEFIXED: i32 = 768i32;
pub const IPAWS_SIZINGNOTIFY: i32 = 4i32;
pub const IPAWS_VERTICALFIXED: i32 = 256i32;
pub const ISC_SHOWUIALL: u32 = 3221225487u32;
pub const ISC_SHOWUIALLCANDIDATEWINDOW: u32 = 15u32;
pub const ISC_SHOWUICANDIDATEWINDOW: u32 = 1u32;
pub const ISC_SHOWUICOMPOSITIONWINDOW: u32 = 2147483648u32;
pub const ISC_SHOWUIGUIDELINE: u32 = 1073741824u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmAssociateContext<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>, Param1: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, param1: Param1) -> super::super::super::Globalization::HIMC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmAssociateContext(param0: super::super::super::Foundation::HWND, param1: super::super::super::Globalization::HIMC) -> super::super::super::Globalization::HIMC;
        }
        ::core::mem::transmute(ImmAssociateContext(param0.into_param().abi(), param1.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmAssociateContextEx<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>, Param1: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, param1: Param1, param2: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmAssociateContextEx(param0: super::super::super::Foundation::HWND, param1: super::super::super::Globalization::HIMC, param2: u32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmAssociateContextEx(param0.into_param().abi(), param1.into_param().abi(), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmConfigureIMEA<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>>(param0: Param0, param1: Param1, param2: u32, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmConfigureIMEA(param0: super::super::TextServices::HKL, param1: super::super::super::Foundation::HWND, param2: u32, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmConfigureIMEA(param0.into_param().abi(), param1.into_param().abi(), ::core::mem::transmute(param2), ::core::mem::transmute(param3)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmConfigureIMEW<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>>(param0: Param0, param1: Param1, param2: u32, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmConfigureIMEW(param0: super::super::TextServices::HKL, param1: super::super::super::Foundation::HWND, param2: u32, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmConfigureIMEW(param0.into_param().abi(), param1.into_param().abi(), ::core::mem::transmute(param2), ::core::mem::transmute(param3)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmCreateContext() -> super::super::super::Globalization::HIMC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmCreateContext() -> super::super::super::Globalization::HIMC;
        }
        ::core::mem::transmute(ImmCreateContext())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmCreateIMCC(param0: u32) -> super::super::super::Globalization::HIMCC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmCreateIMCC(param0: u32) -> super::super::super::Globalization::HIMCC;
        }
        ::core::mem::transmute(ImmCreateIMCC(::core::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmCreateSoftKeyboard<'a, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>>(param0: u32, param1: Param1, param2: i32, param3: i32) -> super::super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmCreateSoftKeyboard(param0: u32, param1: super::super::super::Foundation::HWND, param2: i32, param3: i32) -> super::super::super::Foundation::HWND;
        }
        ::core::mem::transmute(ImmCreateSoftKeyboard(::core::mem::transmute(param0), param1.into_param().abi(), ::core::mem::transmute(param2), ::core::mem::transmute(param3)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmDestroyContext<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmDestroyContext(param0: super::super::super::Globalization::HIMC) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmDestroyContext(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmDestroyIMCC<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMCC>>(param0: Param0) -> super::super::super::Globalization::HIMCC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmDestroyIMCC(param0: super::super::super::Globalization::HIMCC) -> super::super::super::Globalization::HIMCC;
        }
        ::core::mem::transmute(ImmDestroyIMCC(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmDestroySoftKeyboard<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>>(param0: Param0) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmDestroySoftKeyboard(param0: super::super::super::Foundation::HWND) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmDestroySoftKeyboard(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmDisableIME(param0: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmDisableIME(param0: u32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmDisableIME(::core::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmDisableLegacyIME() -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmDisableLegacyIME() -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmDisableLegacyIME())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmDisableTextFrameService(idthread: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmDisableTextFrameService(idthread: u32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmDisableTextFrameService(::core::mem::transmute(idthread)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmEnumInputContext<'a, Param2: ::windows_core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(idthread: u32, lpfn: IMCENUMPROC, lparam: Param2) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmEnumInputContext(idthread: u32, lpfn: ::windows_core::RawPtr, lparam: super::super::super::Foundation::LPARAM) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmEnumInputContext(::core::mem::transmute(idthread), ::core::mem::transmute(lpfn), lparam.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_TextServices")]
#[inline]
pub unsafe fn ImmEnumRegisterWordA<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param4: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(param0: Param0, param1: REGISTERWORDENUMPROCA, lpszreading: Param2, param3: u32, lpszregister: Param4, param5: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmEnumRegisterWordA(param0: super::super::TextServices::HKL, param1: ::windows_core::RawPtr, lpszreading: ::windows_core::PCSTR, param3: u32, lpszregister: ::windows_core::PCSTR, param5: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(ImmEnumRegisterWordA(param0.into_param().abi(), ::core::mem::transmute(param1), lpszreading.into_param().abi(), ::core::mem::transmute(param3), lpszregister.into_param().abi(), ::core::mem::transmute(param5)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_TextServices")]
#[inline]
pub unsafe fn ImmEnumRegisterWordW<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param4: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(param0: Param0, param1: REGISTERWORDENUMPROCW, lpszreading: Param2, param3: u32, lpszregister: Param4, param5: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmEnumRegisterWordW(param0: super::super::TextServices::HKL, param1: ::windows_core::RawPtr, lpszreading: ::windows_core::PCWSTR, param3: u32, lpszregister: ::windows_core::PCWSTR, param5: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(ImmEnumRegisterWordW(param0.into_param().abi(), ::core::mem::transmute(param1), lpszreading.into_param().abi(), ::core::mem::transmute(param3), lpszregister.into_param().abi(), ::core::mem::transmute(param5)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmEscapeA<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, param1: Param1, param2: u32, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmEscapeA(param0: super::super::TextServices::HKL, param1: super::super::super::Globalization::HIMC, param2: u32, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::LRESULT;
        }
        ::core::mem::transmute(ImmEscapeA(param0.into_param().abi(), param1.into_param().abi(), ::core::mem::transmute(param2), ::core::mem::transmute(param3)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmEscapeW<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, param1: Param1, param2: u32, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmEscapeW(param0: super::super::TextServices::HKL, param1: super::super::super::Globalization::HIMC, param2: u32, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::LRESULT;
        }
        ::core::mem::transmute(ImmEscapeW(param0.into_param().abi(), param1.into_param().abi(), ::core::mem::transmute(param2), ::core::mem::transmute(param3)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmGenerateMessage<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGenerateMessage(param0: super::super::super::Globalization::HIMC) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmGenerateMessage(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetCandidateListA<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, deindex: u32, lpcandlist: *mut CANDIDATELIST, dwbuflen: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetCandidateListA(param0: super::super::super::Globalization::HIMC, deindex: u32, lpcandlist: *mut CANDIDATELIST, dwbuflen: u32) -> u32;
        }
        ::core::mem::transmute(ImmGetCandidateListA(param0.into_param().abi(), ::core::mem::transmute(deindex), ::core::mem::transmute(lpcandlist), ::core::mem::transmute(dwbuflen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetCandidateListCountA<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, lpdwlistcount: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetCandidateListCountA(param0: super::super::super::Globalization::HIMC, lpdwlistcount: *mut u32) -> u32;
        }
        ::core::mem::transmute(ImmGetCandidateListCountA(param0.into_param().abi(), ::core::mem::transmute(lpdwlistcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetCandidateListCountW<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, lpdwlistcount: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetCandidateListCountW(param0: super::super::super::Globalization::HIMC, lpdwlistcount: *mut u32) -> u32;
        }
        ::core::mem::transmute(ImmGetCandidateListCountW(param0.into_param().abi(), ::core::mem::transmute(lpdwlistcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetCandidateListW<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, deindex: u32, lpcandlist: *mut CANDIDATELIST, dwbuflen: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetCandidateListW(param0: super::super::super::Globalization::HIMC, deindex: u32, lpcandlist: *mut CANDIDATELIST, dwbuflen: u32) -> u32;
        }
        ::core::mem::transmute(ImmGetCandidateListW(param0.into_param().abi(), ::core::mem::transmute(deindex), ::core::mem::transmute(lpcandlist), ::core::mem::transmute(dwbuflen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmGetCandidateWindow<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, param1: u32, lpcandidate: *mut CANDIDATEFORM) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetCandidateWindow(param0: super::super::super::Globalization::HIMC, param1: u32, lpcandidate: *mut CANDIDATEFORM) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmGetCandidateWindow(param0.into_param().abi(), ::core::mem::transmute(param1), ::core::mem::transmute(lpcandidate)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImmGetCompositionFontA<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, lplf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetCompositionFontA(param0: super::super::super::Globalization::HIMC, lplf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmGetCompositionFontA(param0.into_param().abi(), ::core::mem::transmute(lplf)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImmGetCompositionFontW<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, lplf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetCompositionFontW(param0: super::super::super::Globalization::HIMC, lplf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmGetCompositionFontW(param0.into_param().abi(), ::core::mem::transmute(lplf)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetCompositionStringA<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, param1: u32, lpbuf: *mut ::core::ffi::c_void, dwbuflen: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetCompositionStringA(param0: super::super::super::Globalization::HIMC, param1: u32, lpbuf: *mut ::core::ffi::c_void, dwbuflen: u32) -> i32;
        }
        ::core::mem::transmute(ImmGetCompositionStringA(param0.into_param().abi(), ::core::mem::transmute(param1), ::core::mem::transmute(lpbuf), ::core::mem::transmute(dwbuflen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetCompositionStringW<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, param1: u32, lpbuf: *mut ::core::ffi::c_void, dwbuflen: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetCompositionStringW(param0: super::super::super::Globalization::HIMC, param1: u32, lpbuf: *mut ::core::ffi::c_void, dwbuflen: u32) -> i32;
        }
        ::core::mem::transmute(ImmGetCompositionStringW(param0.into_param().abi(), ::core::mem::transmute(param1), ::core::mem::transmute(lpbuf), ::core::mem::transmute(dwbuflen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmGetCompositionWindow<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, lpcompform: *mut COMPOSITIONFORM) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetCompositionWindow(param0: super::super::super::Globalization::HIMC, lpcompform: *mut COMPOSITIONFORM) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmGetCompositionWindow(param0.into_param().abi(), ::core::mem::transmute(lpcompform)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmGetContext<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>>(param0: Param0) -> super::super::super::Globalization::HIMC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetContext(param0: super::super::super::Foundation::HWND) -> super::super::super::Globalization::HIMC;
        }
        ::core::mem::transmute(ImmGetContext(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmGetConversionListA<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(param0: Param0, param1: Param1, lpsrc: Param2, lpdst: *mut CANDIDATELIST, dwbuflen: u32, uflag: GET_CONVERSION_LIST_FLAG) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetConversionListA(param0: super::super::TextServices::HKL, param1: super::super::super::Globalization::HIMC, lpsrc: ::windows_core::PCSTR, lpdst: *mut CANDIDATELIST, dwbuflen: u32, uflag: GET_CONVERSION_LIST_FLAG) -> u32;
        }
        ::core::mem::transmute(ImmGetConversionListA(param0.into_param().abi(), param1.into_param().abi(), lpsrc.into_param().abi(), ::core::mem::transmute(lpdst), ::core::mem::transmute(dwbuflen), ::core::mem::transmute(uflag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmGetConversionListW<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(param0: Param0, param1: Param1, lpsrc: Param2, lpdst: *mut CANDIDATELIST, dwbuflen: u32, uflag: GET_CONVERSION_LIST_FLAG) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetConversionListW(param0: super::super::TextServices::HKL, param1: super::super::super::Globalization::HIMC, lpsrc: ::windows_core::PCWSTR, lpdst: *mut CANDIDATELIST, dwbuflen: u32, uflag: GET_CONVERSION_LIST_FLAG) -> u32;
        }
        ::core::mem::transmute(ImmGetConversionListW(param0.into_param().abi(), param1.into_param().abi(), lpsrc.into_param().abi(), ::core::mem::transmute(lpdst), ::core::mem::transmute(dwbuflen), ::core::mem::transmute(uflag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmGetConversionStatus<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, lpfdwconversion: *mut u32, lpfdwsentence: *mut u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetConversionStatus(param0: super::super::super::Globalization::HIMC, lpfdwconversion: *mut u32, lpfdwsentence: *mut u32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmGetConversionStatus(param0.into_param().abi(), ::core::mem::transmute(lpfdwconversion), ::core::mem::transmute(lpfdwsentence)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmGetDefaultIMEWnd<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>>(param0: Param0) -> super::super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetDefaultIMEWnd(param0: super::super::super::Foundation::HWND) -> super::super::super::Foundation::HWND;
        }
        ::core::mem::transmute(ImmGetDefaultIMEWnd(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_TextServices")]
#[inline]
pub unsafe fn ImmGetDescriptionA<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>>(param0: Param0, lpszdescription: &mut [u8]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetDescriptionA(param0: super::super::TextServices::HKL, lpszdescription: ::windows_core::PSTR, ubuflen: u32) -> u32;
        }
        ::core::mem::transmute(ImmGetDescriptionA(param0.into_param().abi(), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(lpszdescription)), lpszdescription.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_TextServices")]
#[inline]
pub unsafe fn ImmGetDescriptionW<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>>(param0: Param0, lpszdescription: &mut [u16]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetDescriptionW(param0: super::super::TextServices::HKL, lpszdescription: ::windows_core::PWSTR, ubuflen: u32) -> u32;
        }
        ::core::mem::transmute(ImmGetDescriptionW(param0.into_param().abi(), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(lpszdescription)), lpszdescription.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetGuideLineA<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, dwindex: GET_GUIDE_LINE_TYPE, lpbuf: ::windows_core::PSTR, dwbuflen: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetGuideLineA(param0: super::super::super::Globalization::HIMC, dwindex: GET_GUIDE_LINE_TYPE, lpbuf: ::windows_core::PSTR, dwbuflen: u32) -> u32;
        }
        ::core::mem::transmute(ImmGetGuideLineA(param0.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(lpbuf), ::core::mem::transmute(dwbuflen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetGuideLineW<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, dwindex: GET_GUIDE_LINE_TYPE, lpbuf: ::windows_core::PWSTR, dwbuflen: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetGuideLineW(param0: super::super::super::Globalization::HIMC, dwindex: GET_GUIDE_LINE_TYPE, lpbuf: ::windows_core::PWSTR, dwbuflen: u32) -> u32;
        }
        ::core::mem::transmute(ImmGetGuideLineW(param0.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(lpbuf), ::core::mem::transmute(dwbuflen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmGetHotKey(param0: u32, lpumodifiers: *mut u32, lpuvkey: *mut u32, phkl: *mut isize) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetHotKey(param0: u32, lpumodifiers: *mut u32, lpuvkey: *mut u32, phkl: *mut isize) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmGetHotKey(::core::mem::transmute(param0), ::core::mem::transmute(lpumodifiers), ::core::mem::transmute(lpuvkey), ::core::mem::transmute(phkl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetIMCCLockCount<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMCC>>(param0: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetIMCCLockCount(param0: super::super::super::Globalization::HIMCC) -> u32;
        }
        ::core::mem::transmute(ImmGetIMCCLockCount(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetIMCCSize<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMCC>>(param0: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetIMCCSize(param0: super::super::super::Globalization::HIMCC) -> u32;
        }
        ::core::mem::transmute(ImmGetIMCCSize(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetIMCLockCount<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetIMCLockCount(param0: super::super::super::Globalization::HIMC) -> u32;
        }
        ::core::mem::transmute(ImmGetIMCLockCount(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_TextServices")]
#[inline]
pub unsafe fn ImmGetIMEFileNameA<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>>(param0: Param0, lpszfilename: &mut [u8]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetIMEFileNameA(param0: super::super::TextServices::HKL, lpszfilename: ::windows_core::PSTR, ubuflen: u32) -> u32;
        }
        ::core::mem::transmute(ImmGetIMEFileNameA(param0.into_param().abi(), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(lpszfilename)), lpszfilename.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_TextServices")]
#[inline]
pub unsafe fn ImmGetIMEFileNameW<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>>(param0: Param0, lpszfilename: &mut [u16]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetIMEFileNameW(param0: super::super::TextServices::HKL, lpszfilename: ::windows_core::PWSTR, ubuflen: u32) -> u32;
        }
        ::core::mem::transmute(ImmGetIMEFileNameW(param0.into_param().abi(), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(lpszfilename)), lpszfilename.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImmGetImeMenuItemsA<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, param1: u32, param2: u32, lpimeparentmenu: *mut IMEMENUITEMINFOA, lpimemenu: *mut IMEMENUITEMINFOA, dwsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetImeMenuItemsA(param0: super::super::super::Globalization::HIMC, param1: u32, param2: u32, lpimeparentmenu: *mut IMEMENUITEMINFOA, lpimemenu: *mut IMEMENUITEMINFOA, dwsize: u32) -> u32;
        }
        ::core::mem::transmute(ImmGetImeMenuItemsA(param0.into_param().abi(), ::core::mem::transmute(param1), ::core::mem::transmute(param2), ::core::mem::transmute(lpimeparentmenu), ::core::mem::transmute(lpimemenu), ::core::mem::transmute(dwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImmGetImeMenuItemsW<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, param1: u32, param2: u32, lpimeparentmenu: *mut IMEMENUITEMINFOW, lpimemenu: *mut IMEMENUITEMINFOW, dwsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetImeMenuItemsW(param0: super::super::super::Globalization::HIMC, param1: u32, param2: u32, lpimeparentmenu: *mut IMEMENUITEMINFOW, lpimemenu: *mut IMEMENUITEMINFOW, dwsize: u32) -> u32;
        }
        ::core::mem::transmute(ImmGetImeMenuItemsW(param0.into_param().abi(), ::core::mem::transmute(param1), ::core::mem::transmute(param2), ::core::mem::transmute(lpimeparentmenu), ::core::mem::transmute(lpimemenu), ::core::mem::transmute(dwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmGetOpenStatus<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetOpenStatus(param0: super::super::super::Globalization::HIMC) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmGetOpenStatus(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_TextServices")]
#[inline]
pub unsafe fn ImmGetProperty<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>>(param0: Param0, param1: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetProperty(param0: super::super::TextServices::HKL, param1: u32) -> u32;
        }
        ::core::mem::transmute(ImmGetProperty(param0.into_param().abi(), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmGetRegisterWordStyleA<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>>(param0: Param0, lpstylebuf: &mut [STYLEBUFA]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetRegisterWordStyleA(param0: super::super::TextServices::HKL, nitem: u32, lpstylebuf: *mut STYLEBUFA) -> u32;
        }
        ::core::mem::transmute(ImmGetRegisterWordStyleA(param0.into_param().abi(), lpstylebuf.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(lpstylebuf))))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_TextServices")]
#[inline]
pub unsafe fn ImmGetRegisterWordStyleW<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>>(param0: Param0, lpstylebuf: &mut [STYLEBUFW]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetRegisterWordStyleW(param0: super::super::TextServices::HKL, nitem: u32, lpstylebuf: *mut STYLEBUFW) -> u32;
        }
        ::core::mem::transmute(ImmGetRegisterWordStyleW(param0.into_param().abi(), lpstylebuf.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(lpstylebuf))))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmGetStatusWindowPos<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, lpptpos: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetStatusWindowPos(param0: super::super::super::Globalization::HIMC, lpptpos: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmGetStatusWindowPos(param0.into_param().abi(), ::core::mem::transmute(lpptpos)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmGetVirtualKey<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>>(param0: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetVirtualKey(param0: super::super::super::Foundation::HWND) -> u32;
        }
        ::core::mem::transmute(ImmGetVirtualKey(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_TextServices")]
#[inline]
pub unsafe fn ImmInstallIMEA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(lpszimefilename: Param0, lpszlayouttext: Param1) -> super::super::TextServices::HKL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmInstallIMEA(lpszimefilename: ::windows_core::PCSTR, lpszlayouttext: ::windows_core::PCSTR) -> super::super::TextServices::HKL;
        }
        ::core::mem::transmute(ImmInstallIMEA(lpszimefilename.into_param().abi(), lpszlayouttext.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_TextServices")]
#[inline]
pub unsafe fn ImmInstallIMEW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(lpszimefilename: Param0, lpszlayouttext: Param1) -> super::super::TextServices::HKL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmInstallIMEW(lpszimefilename: ::windows_core::PCWSTR, lpszlayouttext: ::windows_core::PCWSTR) -> super::super::TextServices::HKL;
        }
        ::core::mem::transmute(ImmInstallIMEW(lpszimefilename.into_param().abi(), lpszlayouttext.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmIsIME<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>>(param0: Param0) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmIsIME(param0: super::super::TextServices::HKL) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmIsIME(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmIsUIMessageA<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>, Param2: ::windows_core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param3: ::windows_core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(param0: Param0, param1: u32, param2: Param2, param3: Param3) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmIsUIMessageA(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmIsUIMessageA(param0.into_param().abi(), ::core::mem::transmute(param1), param2.into_param().abi(), param3.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmIsUIMessageW<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>, Param2: ::windows_core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param3: ::windows_core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(param0: Param0, param1: u32, param2: Param2, param3: Param3) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmIsUIMessageW(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmIsUIMessageW(param0.into_param().abi(), ::core::mem::transmute(param1), param2.into_param().abi(), param3.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImmLockIMC<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0) -> *mut INPUTCONTEXT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmLockIMC(param0: super::super::super::Globalization::HIMC) -> *mut INPUTCONTEXT;
        }
        ::core::mem::transmute(ImmLockIMC(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmLockIMCC<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMCC>>(param0: Param0) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmLockIMCC(param0: super::super::super::Globalization::HIMCC) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(ImmLockIMCC(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmNotifyIME<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, dwaction: NOTIFY_IME_ACTION, dwindex: NOTIFY_IME_INDEX, dwvalue: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmNotifyIME(param0: super::super::super::Globalization::HIMC, dwaction: NOTIFY_IME_ACTION, dwindex: NOTIFY_IME_INDEX, dwvalue: u32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmNotifyIME(param0.into_param().abi(), ::core::mem::transmute(dwaction), ::core::mem::transmute(dwindex), ::core::mem::transmute(dwvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmReSizeIMCC<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMCC>>(param0: Param0, param1: u32) -> super::super::super::Globalization::HIMCC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmReSizeIMCC(param0: super::super::super::Globalization::HIMCC, param1: u32) -> super::super::super::Globalization::HIMCC;
        }
        ::core::mem::transmute(ImmReSizeIMCC(param0.into_param().abi(), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmRegisterWordA<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(param0: Param0, lpszreading: Param1, param2: u32, lpszregister: Param3) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmRegisterWordA(param0: super::super::TextServices::HKL, lpszreading: ::windows_core::PCSTR, param2: u32, lpszregister: ::windows_core::PCSTR) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmRegisterWordA(param0.into_param().abi(), lpszreading.into_param().abi(), ::core::mem::transmute(param2), lpszregister.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmRegisterWordW<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(param0: Param0, lpszreading: Param1, param2: u32, lpszregister: Param3) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmRegisterWordW(param0: super::super::TextServices::HKL, lpszreading: ::windows_core::PCWSTR, param2: u32, lpszregister: ::windows_core::PCWSTR) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmRegisterWordW(param0.into_param().abi(), lpszreading.into_param().abi(), ::core::mem::transmute(param2), lpszregister.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmReleaseContext<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>, Param1: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, param1: Param1) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmReleaseContext(param0: super::super::super::Foundation::HWND, param1: super::super::super::Globalization::HIMC) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmReleaseContext(param0.into_param().abi(), param1.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmRequestMessageA<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param2: ::windows_core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(param0: Param0, param1: Param1, param2: Param2) -> super::super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmRequestMessageA(param0: super::super::super::Globalization::HIMC, param1: super::super::super::Foundation::WPARAM, param2: super::super::super::Foundation::LPARAM) -> super::super::super::Foundation::LRESULT;
        }
        ::core::mem::transmute(ImmRequestMessageA(param0.into_param().abi(), param1.into_param().abi(), param2.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmRequestMessageW<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param2: ::windows_core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(param0: Param0, param1: Param1, param2: Param2) -> super::super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmRequestMessageW(param0: super::super::super::Globalization::HIMC, param1: super::super::super::Foundation::WPARAM, param2: super::super::super::Foundation::LPARAM) -> super::super::super::Foundation::LRESULT;
        }
        ::core::mem::transmute(ImmRequestMessageW(param0.into_param().abi(), param1.into_param().abi(), param2.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmSetCandidateWindow<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, lpcandidate: *const CANDIDATEFORM) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmSetCandidateWindow(param0: super::super::super::Globalization::HIMC, lpcandidate: *const CANDIDATEFORM) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmSetCandidateWindow(param0.into_param().abi(), ::core::mem::transmute(lpcandidate)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImmSetCompositionFontA<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, lplf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmSetCompositionFontA(param0: super::super::super::Globalization::HIMC, lplf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmSetCompositionFontA(param0.into_param().abi(), ::core::mem::transmute(lplf)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImmSetCompositionFontW<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, lplf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmSetCompositionFontW(param0: super::super::super::Globalization::HIMC, lplf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmSetCompositionFontW(param0.into_param().abi(), ::core::mem::transmute(lplf)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmSetCompositionStringA<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, dwindex: SET_COMPOSITION_STRING_TYPE, lpcomp: *const ::core::ffi::c_void, dwcomplen: u32, lpread: *const ::core::ffi::c_void, dwreadlen: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmSetCompositionStringA(param0: super::super::super::Globalization::HIMC, dwindex: SET_COMPOSITION_STRING_TYPE, lpcomp: *const ::core::ffi::c_void, dwcomplen: u32, lpread: *const ::core::ffi::c_void, dwreadlen: u32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmSetCompositionStringA(param0.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(lpcomp), ::core::mem::transmute(dwcomplen), ::core::mem::transmute(lpread), ::core::mem::transmute(dwreadlen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmSetCompositionStringW<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, dwindex: SET_COMPOSITION_STRING_TYPE, lpcomp: *const ::core::ffi::c_void, dwcomplen: u32, lpread: *const ::core::ffi::c_void, dwreadlen: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmSetCompositionStringW(param0: super::super::super::Globalization::HIMC, dwindex: SET_COMPOSITION_STRING_TYPE, lpcomp: *const ::core::ffi::c_void, dwcomplen: u32, lpread: *const ::core::ffi::c_void, dwreadlen: u32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmSetCompositionStringW(param0.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(lpcomp), ::core::mem::transmute(dwcomplen), ::core::mem::transmute(lpread), ::core::mem::transmute(dwreadlen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmSetCompositionWindow<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, lpcompform: *const COMPOSITIONFORM) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmSetCompositionWindow(param0: super::super::super::Globalization::HIMC, lpcompform: *const COMPOSITIONFORM) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmSetCompositionWindow(param0.into_param().abi(), ::core::mem::transmute(lpcompform)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmSetConversionStatus<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, param1: u32, param2: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmSetConversionStatus(param0: super::super::super::Globalization::HIMC, param1: u32, param2: u32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmSetConversionStatus(param0.into_param().abi(), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmSetHotKey<'a, Param3: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>>(param0: u32, param1: u32, param2: u32, param3: Param3) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmSetHotKey(param0: u32, param1: u32, param2: u32, param3: super::super::TextServices::HKL) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmSetHotKey(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2), param3.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmSetOpenStatus<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::BOOL>>(param0: Param0, param1: Param1) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmSetOpenStatus(param0: super::super::super::Globalization::HIMC, param1: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmSetOpenStatus(param0.into_param().abi(), param1.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmSetStatusWindowPos<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, lpptpos: *const super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmSetStatusWindowPos(param0: super::super::super::Globalization::HIMC, lpptpos: *const super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmSetStatusWindowPos(param0.into_param().abi(), ::core::mem::transmute(lpptpos)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmShowSoftKeyboard<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>>(param0: Param0, param1: i32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmShowSoftKeyboard(param0: super::super::super::Foundation::HWND, param1: i32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmShowSoftKeyboard(param0.into_param().abi(), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmSimulateHotKey<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>>(param0: Param0, param1: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmSimulateHotKey(param0: super::super::super::Foundation::HWND, param1: u32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmSimulateHotKey(param0.into_param().abi(), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmUnlockIMC<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmUnlockIMC(param0: super::super::super::Globalization::HIMC) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmUnlockIMC(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmUnlockIMCC<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Globalization::HIMCC>>(param0: Param0) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmUnlockIMCC(param0: super::super::super::Globalization::HIMCC) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmUnlockIMCC(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmUnregisterWordA<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(param0: Param0, lpszreading: Param1, param2: u32, lpszunregister: Param3) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmUnregisterWordA(param0: super::super::TextServices::HKL, lpszreading: ::windows_core::PCSTR, param2: u32, lpszunregister: ::windows_core::PCSTR) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmUnregisterWordA(param0.into_param().abi(), lpszreading.into_param().abi(), ::core::mem::transmute(param2), lpszunregister.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmUnregisterWordW<'a, Param0: ::windows_core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(param0: Param0, lpszreading: Param1, param2: u32, lpszunregister: Param3) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmUnregisterWordW(param0: super::super::TextServices::HKL, lpszreading: ::windows_core::PCWSTR, param2: u32, lpszunregister: ::windows_core::PCWSTR) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmUnregisterWordW(param0.into_param().abi(), lpszreading.into_param().abi(), ::core::mem::transmute(param2), lpszunregister.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const JPOS_1DAN: u32 = 213u32;
pub const JPOS_4DAN_HA: u32 = 212u32;
pub const JPOS_5DAN_AWA: u32 = 200u32;
pub const JPOS_5DAN_AWAUON: u32 = 209u32;
pub const JPOS_5DAN_BA: u32 = 206u32;
pub const JPOS_5DAN_GA: u32 = 202u32;
pub const JPOS_5DAN_KA: u32 = 201u32;
pub const JPOS_5DAN_KASOKUON: u32 = 210u32;
pub const JPOS_5DAN_MA: u32 = 207u32;
pub const JPOS_5DAN_NA: u32 = 205u32;
pub const JPOS_5DAN_RA: u32 = 208u32;
pub const JPOS_5DAN_RAHEN: u32 = 211u32;
pub const JPOS_5DAN_SA: u32 = 203u32;
pub const JPOS_5DAN_TA: u32 = 204u32;
pub const JPOS_BUPPIN: u32 = 122u32;
pub const JPOS_CHIMEI: u32 = 109u32;
pub const JPOS_CHIMEI_EKI: u32 = 117u32;
pub const JPOS_CHIMEI_GUN: u32 = 112u32;
pub const JPOS_CHIMEI_KEN: u32 = 111u32;
pub const JPOS_CHIMEI_KU: u32 = 113u32;
pub const JPOS_CHIMEI_KUNI: u32 = 110u32;
pub const JPOS_CHIMEI_MACHI: u32 = 115u32;
pub const JPOS_CHIMEI_MURA: u32 = 116u32;
pub const JPOS_CHIMEI_SHI: u32 = 114u32;
pub const JPOS_CLOSEBRACE: u32 = 911u32;
pub const JPOS_DAIMEISHI: u32 = 123u32;
pub const JPOS_DAIMEISHI_NINSHOU: u32 = 124u32;
pub const JPOS_DAIMEISHI_SHIJI: u32 = 125u32;
pub const JPOS_DOKURITSUGO: u32 = 903u32;
pub const JPOS_EIJI: u32 = 906u32;
pub const JPOS_FUKUSHI: u32 = 500u32;
pub const JPOS_FUKUSHI_DA: u32 = 504u32;
pub const JPOS_FUKUSHI_NANO: u32 = 503u32;
pub const JPOS_FUKUSHI_NI: u32 = 502u32;
pub const JPOS_FUKUSHI_SAHEN: u32 = 501u32;
pub const JPOS_FUKUSHI_TO: u32 = 505u32;
pub const JPOS_FUKUSHI_TOSURU: u32 = 506u32;
pub const JPOS_FUTEIGO: u32 = 904u32;
pub const JPOS_HUKUSIMEISHI: u32 = 104u32;
pub const JPOS_JINMEI: u32 = 106u32;
pub const JPOS_JINMEI_MEI: u32 = 108u32;
pub const JPOS_JINMEI_SEI: u32 = 107u32;
pub const JPOS_KANDOUSHI: u32 = 670u32;
pub const JPOS_KANJI: u32 = 909u32;
pub const JPOS_KANYOUKU: u32 = 902u32;
pub const JPOS_KAZU: u32 = 126u32;
pub const JPOS_KAZU_SURYOU: u32 = 127u32;
pub const JPOS_KAZU_SUSHI: u32 = 128u32;
pub const JPOS_KEIDOU: u32 = 400u32;
pub const JPOS_KEIDOU_GARU: u32 = 403u32;
pub const JPOS_KEIDOU_NO: u32 = 401u32;
pub const JPOS_KEIDOU_TARU: u32 = 402u32;
pub const JPOS_KEIYOU: u32 = 300u32;
pub const JPOS_KEIYOU_GARU: u32 = 301u32;
pub const JPOS_KEIYOU_GE: u32 = 302u32;
pub const JPOS_KEIYOU_ME: u32 = 303u32;
pub const JPOS_KEIYOU_U: u32 = 305u32;
pub const JPOS_KEIYOU_YUU: u32 = 304u32;
pub const JPOS_KENCHIKU: u32 = 121u32;
pub const JPOS_KIGOU: u32 = 905u32;
pub const JPOS_KURU_KI: u32 = 219u32;
pub const JPOS_KURU_KITA: u32 = 220u32;
pub const JPOS_KURU_KITARA: u32 = 221u32;
pub const JPOS_KURU_KITARI: u32 = 222u32;
pub const JPOS_KURU_KITAROU: u32 = 223u32;
pub const JPOS_KURU_KITE: u32 = 224u32;
pub const JPOS_KURU_KO: u32 = 226u32;
pub const JPOS_KURU_KOI: u32 = 227u32;
pub const JPOS_KURU_KOYOU: u32 = 228u32;
pub const JPOS_KURU_KUREBA: u32 = 225u32;
pub const JPOS_KUTEN: u32 = 907u32;
pub const JPOS_MEISA_KEIDOU: u32 = 105u32;
pub const JPOS_MEISHI_FUTSU: u32 = 100u32;
pub const JPOS_MEISHI_KEIYOUDOUSHI: u32 = 103u32;
pub const JPOS_MEISHI_SAHEN: u32 = 101u32;
pub const JPOS_MEISHI_ZAHEN: u32 = 102u32;
pub const JPOS_OPENBRACE: u32 = 910u32;
pub const JPOS_RENTAISHI: u32 = 600u32;
pub const JPOS_RENTAISHI_SHIJI: u32 = 601u32;
pub const JPOS_RENYOU_SETSUBI: u32 = 826u32;
pub const JPOS_SETSUBI: u32 = 800u32;
pub const JPOS_SETSUBI_CHIMEI: u32 = 811u32;
pub const JPOS_SETSUBI_CHOU: u32 = 818u32;
pub const JPOS_SETSUBI_CHU: u32 = 804u32;
pub const JPOS_SETSUBI_DONO: u32 = 835u32;
pub const JPOS_SETSUBI_EKI: u32 = 821u32;
pub const JPOS_SETSUBI_FU: u32 = 805u32;
pub const JPOS_SETSUBI_FUKUSU: u32 = 836u32;
pub const JPOS_SETSUBI_GUN: u32 = 814u32;
pub const JPOS_SETSUBI_JIKAN: u32 = 829u32;
pub const JPOS_SETSUBI_JIKANPLUS: u32 = 830u32;
pub const JPOS_SETSUBI_JINMEI: u32 = 810u32;
pub const JPOS_SETSUBI_JOSUSHI: u32 = 827u32;
pub const JPOS_SETSUBI_JOSUSHIPLUS: u32 = 828u32;
pub const JPOS_SETSUBI_KA: u32 = 803u32;
pub const JPOS_SETSUBI_KATA: u32 = 808u32;
pub const JPOS_SETSUBI_KEN: u32 = 813u32;
pub const JPOS_SETSUBI_KENCHIKU: u32 = 825u32;
pub const JPOS_SETSUBI_KU: u32 = 815u32;
pub const JPOS_SETSUBI_KUN: u32 = 833u32;
pub const JPOS_SETSUBI_KUNI: u32 = 812u32;
pub const JPOS_SETSUBI_MACHI: u32 = 817u32;
pub const JPOS_SETSUBI_MEISHIRENDAKU: u32 = 809u32;
pub const JPOS_SETSUBI_MURA: u32 = 819u32;
pub const JPOS_SETSUBI_RA: u32 = 838u32;
pub const JPOS_SETSUBI_RYU: u32 = 806u32;
pub const JPOS_SETSUBI_SAMA: u32 = 834u32;
pub const JPOS_SETSUBI_SAN: u32 = 832u32;
pub const JPOS_SETSUBI_SEI: u32 = 802u32;
pub const JPOS_SETSUBI_SHAMEI: u32 = 823u32;
pub const JPOS_SETSUBI_SHI: u32 = 816u32;
pub const JPOS_SETSUBI_SON: u32 = 820u32;
pub const JPOS_SETSUBI_SONOTA: u32 = 822u32;
pub const JPOS_SETSUBI_SOSHIKI: u32 = 824u32;
pub const JPOS_SETSUBI_TACHI: u32 = 837u32;
pub const JPOS_SETSUBI_TEINEI: u32 = 831u32;
pub const JPOS_SETSUBI_TEKI: u32 = 801u32;
pub const JPOS_SETSUBI_YOU: u32 = 807u32;
pub const JPOS_SETSUZOKUSHI: u32 = 650u32;
pub const JPOS_SETTOU: u32 = 700u32;
pub const JPOS_SETTOU_CHIMEI: u32 = 710u32;
pub const JPOS_SETTOU_CHOUTAN: u32 = 707u32;
pub const JPOS_SETTOU_DAISHOU: u32 = 705u32;
pub const JPOS_SETTOU_FUKU: u32 = 703u32;
pub const JPOS_SETTOU_JINMEI: u32 = 709u32;
pub const JPOS_SETTOU_JOSUSHI: u32 = 712u32;
pub const JPOS_SETTOU_KAKU: u32 = 701u32;
pub const JPOS_SETTOU_KOUTEI: u32 = 706u32;
pub const JPOS_SETTOU_MI: u32 = 704u32;
pub const JPOS_SETTOU_SAI: u32 = 702u32;
pub const JPOS_SETTOU_SHINKYU: u32 = 708u32;
pub const JPOS_SETTOU_SONOTA: u32 = 711u32;
pub const JPOS_SETTOU_TEINEI_GO: u32 = 714u32;
pub const JPOS_SETTOU_TEINEI_O: u32 = 713u32;
pub const JPOS_SETTOU_TEINEI_ON: u32 = 715u32;
pub const JPOS_SHAMEI: u32 = 119u32;
pub const JPOS_SONOTA: u32 = 118u32;
pub const JPOS_SOSHIKI: u32 = 120u32;
pub const JPOS_SURU_SA: u32 = 229u32;
pub const JPOS_SURU_SE: u32 = 238u32;
pub const JPOS_SURU_SEYO: u32 = 239u32;
pub const JPOS_SURU_SI: u32 = 230u32;
pub const JPOS_SURU_SIATRI: u32 = 233u32;
pub const JPOS_SURU_SITA: u32 = 231u32;
pub const JPOS_SURU_SITARA: u32 = 232u32;
pub const JPOS_SURU_SITAROU: u32 = 234u32;
pub const JPOS_SURU_SITE: u32 = 235u32;
pub const JPOS_SURU_SIYOU: u32 = 236u32;
pub const JPOS_SURU_SUREBA: u32 = 237u32;
pub const JPOS_TANKANJI: u32 = 900u32;
pub const JPOS_TANKANJI_KAO: u32 = 901u32;
pub const JPOS_TANSHUKU: u32 = 913u32;
pub const JPOS_TOKUSHU_KAHEN: u32 = 214u32;
pub const JPOS_TOKUSHU_NAHEN: u32 = 218u32;
pub const JPOS_TOKUSHU_SAHEN: u32 = 216u32;
pub const JPOS_TOKUSHU_SAHENSURU: u32 = 215u32;
pub const JPOS_TOKUSHU_ZAHEN: u32 = 217u32;
pub const JPOS_TOUTEN: u32 = 908u32;
pub const JPOS_UNDEFINED: u32 = 0u32;
pub const JPOS_YOKUSEI: u32 = 912u32;
pub const MAX_APPLETTITLE: u32 = 64u32;
pub const MAX_FONTFACE: u32 = 32u32;
pub const MODEBIASMODE_DEFAULT: u32 = 0u32;
pub const MODEBIASMODE_DIGIT: u32 = 4u32;
pub const MODEBIASMODE_FILENAME: u32 = 1u32;
pub const MODEBIASMODE_READING: u32 = 2u32;
pub const MODEBIAS_GETVALUE: u32 = 2u32;
pub const MODEBIAS_GETVERSION: u32 = 0u32;
pub const MODEBIAS_SETVALUE: u32 = 1u32;
pub const MOD_IGNORE_ALL_MODIFIER: u32 = 1024u32;
pub const MOD_LEFT: u32 = 32768u32;
pub const MOD_ON_KEYUP: u32 = 2048u32;
pub const MOD_RIGHT: u32 = 16384u32;
#[repr(C, packed(1))]
pub struct MORRSLT {
    pub dwSize: u32,
    pub pwchOutput: ::windows_core::PWSTR,
    pub cchOutput: u16,
    pub Anonymous1: MORRSLT_0,
    pub Anonymous2: MORRSLT_1,
    pub pchInputPos: *mut u16,
    pub pchOutputIdxWDD: *mut u16,
    pub Anonymous3: MORRSLT_2,
    pub paMonoRubyPos: *mut u16,
    pub pWDD: *mut WDD,
    pub cWDD: i32,
    pub pPrivate: *mut ::core::ffi::c_void,
    pub BLKBuff: [u16; 1],
}
impl ::core::marker::Copy for MORRSLT {}
impl ::core::clone::Clone for MORRSLT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for MORRSLT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MORRSLT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MORRSLT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MORRSLT {}
impl ::core::default::Default for MORRSLT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub union MORRSLT_0 {
    pub pwchRead: ::windows_core::PWSTR,
    pub pwchComp: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for MORRSLT_0 {}
impl ::core::clone::Clone for MORRSLT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for MORRSLT_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MORRSLT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MORRSLT_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MORRSLT_0 {}
impl ::core::default::Default for MORRSLT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub union MORRSLT_1 {
    pub cchRead: u16,
    pub cchComp: u16,
}
impl ::core::marker::Copy for MORRSLT_1 {}
impl ::core::clone::Clone for MORRSLT_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for MORRSLT_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MORRSLT_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MORRSLT_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for MORRSLT_1 {}
impl ::core::default::Default for MORRSLT_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub union MORRSLT_2 {
    pub pchReadIdxWDD: *mut u16,
    pub pchCompIdxWDD: *mut u16,
}
impl ::core::marker::Copy for MORRSLT_2 {}
impl ::core::clone::Clone for MORRSLT_2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for MORRSLT_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MORRSLT_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MORRSLT_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for MORRSLT_2 {}
impl ::core::default::Default for MORRSLT_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const NI_CONTEXTUPDATED: u32 = 3u32;
pub const NI_FINALIZECONVERSIONRESULT: u32 = 20u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NOTIFY_IME_ACTION(pub u32);
pub const NI_CHANGECANDIDATELIST: NOTIFY_IME_ACTION = NOTIFY_IME_ACTION(19u32);
pub const NI_CLOSECANDIDATE: NOTIFY_IME_ACTION = NOTIFY_IME_ACTION(17u32);
pub const NI_COMPOSITIONSTR: NOTIFY_IME_ACTION = NOTIFY_IME_ACTION(21u32);
pub const NI_IMEMENUSELECTED: NOTIFY_IME_ACTION = NOTIFY_IME_ACTION(24u32);
pub const NI_OPENCANDIDATE: NOTIFY_IME_ACTION = NOTIFY_IME_ACTION(16u32);
pub const NI_SELECTCANDIDATESTR: NOTIFY_IME_ACTION = NOTIFY_IME_ACTION(18u32);
pub const NI_SETCANDIDATE_PAGESIZE: NOTIFY_IME_ACTION = NOTIFY_IME_ACTION(23u32);
pub const NI_SETCANDIDATE_PAGESTART: NOTIFY_IME_ACTION = NOTIFY_IME_ACTION(22u32);
impl ::core::marker::Copy for NOTIFY_IME_ACTION {}
impl ::core::clone::Clone for NOTIFY_IME_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NOTIFY_IME_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NOTIFY_IME_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for NOTIFY_IME_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NOTIFY_IME_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NOTIFY_IME_INDEX(pub u32);
pub const CPS_CANCEL: NOTIFY_IME_INDEX = NOTIFY_IME_INDEX(4u32);
pub const CPS_COMPLETE: NOTIFY_IME_INDEX = NOTIFY_IME_INDEX(1u32);
pub const CPS_CONVERT: NOTIFY_IME_INDEX = NOTIFY_IME_INDEX(2u32);
pub const CPS_REVERT: NOTIFY_IME_INDEX = NOTIFY_IME_INDEX(3u32);
impl ::core::marker::Copy for NOTIFY_IME_INDEX {}
impl ::core::clone::Clone for NOTIFY_IME_INDEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NOTIFY_IME_INDEX {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NOTIFY_IME_INDEX {
    type Abi = Self;
}
impl ::core::fmt::Debug for NOTIFY_IME_INDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NOTIFY_IME_INDEX").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
pub type PFNLOG = ::core::option::Option<unsafe extern "system" fn(param0: *mut IMEDP, param1: ::windows_core::HRESULT) -> super::super::super::Foundation::BOOL>;
#[repr(C, packed(1))]
pub struct POSTBL {
    pub nPos: u16,
    pub szName: *mut u8,
}
impl ::core::marker::Copy for POSTBL {}
impl ::core::clone::Clone for POSTBL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for POSTBL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for POSTBL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<POSTBL>()) == 0 }
    }
}
impl ::core::cmp::Eq for POSTBL {}
impl ::core::default::Default for POSTBL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const POS_UNDEFINED: u32 = 0u32;
#[repr(C)]
pub struct RECONVERTSTRING {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwStrLen: u32,
    pub dwStrOffset: u32,
    pub dwCompStrLen: u32,
    pub dwCompStrOffset: u32,
    pub dwTargetStrLen: u32,
    pub dwTargetStrOffset: u32,
}
impl ::core::marker::Copy for RECONVERTSTRING {}
impl ::core::clone::Clone for RECONVERTSTRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RECONVERTSTRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RECONVERTSTRING").field("dwSize", &self.dwSize).field("dwVersion", &self.dwVersion).field("dwStrLen", &self.dwStrLen).field("dwStrOffset", &self.dwStrOffset).field("dwCompStrLen", &self.dwCompStrLen).field("dwCompStrOffset", &self.dwCompStrOffset).field("dwTargetStrLen", &self.dwTargetStrLen).field("dwTargetStrOffset", &self.dwTargetStrOffset).finish()
    }
}
unsafe impl ::windows_core::Abi for RECONVERTSTRING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RECONVERTSTRING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RECONVERTSTRING>()) == 0 }
    }
}
impl ::core::cmp::Eq for RECONVERTSTRING {}
impl ::core::default::Default for RECONVERTSTRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const RECONVOPT_NONE: u32 = 0u32;
pub const RECONVOPT_USECANCELNOTIFY: u32 = 1u32;
#[repr(C)]
pub struct REGISTERWORDA {
    pub lpReading: ::windows_core::PSTR,
    pub lpWord: ::windows_core::PSTR,
}
impl ::core::marker::Copy for REGISTERWORDA {}
impl ::core::clone::Clone for REGISTERWORDA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REGISTERWORDA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REGISTERWORDA").field("lpReading", &self.lpReading).field("lpWord", &self.lpWord).finish()
    }
}
unsafe impl ::windows_core::Abi for REGISTERWORDA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REGISTERWORDA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REGISTERWORDA>()) == 0 }
    }
}
impl ::core::cmp::Eq for REGISTERWORDA {}
impl ::core::default::Default for REGISTERWORDA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type REGISTERWORDENUMPROCA = ::core::option::Option<unsafe extern "system" fn(lpszreading: ::windows_core::PCSTR, param1: u32, lpszstring: ::windows_core::PCSTR, param3: *mut ::core::ffi::c_void) -> i32>;
pub type REGISTERWORDENUMPROCW = ::core::option::Option<unsafe extern "system" fn(lpszreading: ::windows_core::PCWSTR, param1: u32, lpszstring: ::windows_core::PCWSTR, param3: *mut ::core::ffi::c_void) -> i32>;
#[repr(C)]
pub struct REGISTERWORDW {
    pub lpReading: ::windows_core::PWSTR,
    pub lpWord: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for REGISTERWORDW {}
impl ::core::clone::Clone for REGISTERWORDW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REGISTERWORDW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REGISTERWORDW").field("lpReading", &self.lpReading).field("lpWord", &self.lpWord).finish()
    }
}
unsafe impl ::windows_core::Abi for REGISTERWORDW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REGISTERWORDW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REGISTERWORDW>()) == 0 }
    }
}
impl ::core::cmp::Eq for REGISTERWORDW {}
impl ::core::default::Default for REGISTERWORDW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const RWM_CHGKEYMAP: &str = "MSIMEChangeKeyMap";
pub const RWM_DOCUMENTFEED: &str = "MSIMEDocumentFeed";
pub const RWM_KEYMAP: &str = "MSIMEKeyMap";
pub const RWM_MODEBIAS: &str = "MSIMEModeBias";
pub const RWM_MOUSE: &str = "MSIMEMouseOperation";
pub const RWM_NTFYKEYMAP: &str = "MSIMENotifyKeyMap";
pub const RWM_QUERYPOSITION: &str = "MSIMEQueryPosition";
pub const RWM_RECONVERT: &str = "MSIMEReconvert";
pub const RWM_RECONVERTOPTIONS: &str = "MSIMEReconvertOptions";
pub const RWM_RECONVERTREQUEST: &str = "MSIMEReconvertRequest";
pub const RWM_SERVICE: &str = "MSIMEService";
pub const RWM_SHOWIMEPAD: &str = "MSIMEShowImePad";
pub const RWM_UIREADY: &str = "MSIMEUIReady";
pub const SCS_CAP_COMPSTR: u32 = 1u32;
pub const SCS_CAP_MAKEREAD: u32 = 2u32;
pub const SCS_CAP_SETRECONVERTSTRING: u32 = 4u32;
pub const SELECT_CAP_CONVERSION: u32 = 1u32;
pub const SELECT_CAP_SENTENCE: u32 = 2u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SET_COMPOSITION_STRING_TYPE(pub u32);
pub const SCS_SETSTR: SET_COMPOSITION_STRING_TYPE = SET_COMPOSITION_STRING_TYPE(9u32);
pub const SCS_CHANGEATTR: SET_COMPOSITION_STRING_TYPE = SET_COMPOSITION_STRING_TYPE(18u32);
pub const SCS_CHANGECLAUSE: SET_COMPOSITION_STRING_TYPE = SET_COMPOSITION_STRING_TYPE(36u32);
pub const SCS_SETRECONVERTSTRING: SET_COMPOSITION_STRING_TYPE = SET_COMPOSITION_STRING_TYPE(65536u32);
pub const SCS_QUERYRECONVERTSTRING: SET_COMPOSITION_STRING_TYPE = SET_COMPOSITION_STRING_TYPE(131072u32);
impl ::core::marker::Copy for SET_COMPOSITION_STRING_TYPE {}
impl ::core::clone::Clone for SET_COMPOSITION_STRING_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SET_COMPOSITION_STRING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SET_COMPOSITION_STRING_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SET_COMPOSITION_STRING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SET_COMPOSITION_STRING_TYPE").field(&self.0).finish()
    }
}
pub const SHOWIMEPAD_CATEGORY: u32 = 1u32;
pub const SHOWIMEPAD_DEFAULT: u32 = 0u32;
pub const SHOWIMEPAD_GUID: u32 = 2u32;
#[repr(C)]
pub struct SOFTKBDDATA {
    pub uCount: u32,
    pub wCode: [u16; 256],
}
impl ::core::marker::Copy for SOFTKBDDATA {}
impl ::core::clone::Clone for SOFTKBDDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOFTKBDDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOFTKBDDATA").field("uCount", &self.uCount).field("wCode", &self.wCode).finish()
    }
}
unsafe impl ::windows_core::Abi for SOFTKBDDATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SOFTKBDDATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SOFTKBDDATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for SOFTKBDDATA {}
impl ::core::default::Default for SOFTKBDDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const SOFTKEYBOARD_TYPE_C1: u32 = 2u32;
pub const SOFTKEYBOARD_TYPE_T1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STYLEBUFA {
    pub dwStyle: u32,
    pub szDescription: [super::super::super::Foundation::CHAR; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STYLEBUFA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STYLEBUFA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STYLEBUFA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STYLEBUFA").field("dwStyle", &self.dwStyle).field("szDescription", &self.szDescription).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows_core::Abi for STYLEBUFA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STYLEBUFA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STYLEBUFA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STYLEBUFA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STYLEBUFA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct STYLEBUFW {
    pub dwStyle: u32,
    pub szDescription: [u16; 32],
}
impl ::core::marker::Copy for STYLEBUFW {}
impl ::core::clone::Clone for STYLEBUFW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STYLEBUFW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STYLEBUFW").field("dwStyle", &self.dwStyle).field("szDescription", &self.szDescription).finish()
    }
}
unsafe impl ::windows_core::Abi for STYLEBUFW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STYLEBUFW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STYLEBUFW>()) == 0 }
    }
}
impl ::core::cmp::Eq for STYLEBUFW {}
impl ::core::default::Default for STYLEBUFW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const STYLE_DESCRIPTION_SIZE: u32 = 32u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TRANSMSG {
    pub message: u32,
    pub wParam: super::super::super::Foundation::WPARAM,
    pub lParam: super::super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TRANSMSG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TRANSMSG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TRANSMSG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSMSG").field("message", &self.message).field("wParam", &self.wParam).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows_core::Abi for TRANSMSG {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TRANSMSG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRANSMSG>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TRANSMSG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TRANSMSG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TRANSMSGLIST {
    pub uMsgCount: u32,
    pub TransMsg: [TRANSMSG; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TRANSMSGLIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TRANSMSGLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TRANSMSGLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSMSGLIST").field("uMsgCount", &self.uMsgCount).field("TransMsg", &self.TransMsg).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows_core::Abi for TRANSMSGLIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TRANSMSGLIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRANSMSGLIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TRANSMSGLIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TRANSMSGLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const UI_CAP_2700: u32 = 1u32;
pub const UI_CAP_ROT90: u32 = 2u32;
pub const UI_CAP_ROTANY: u32 = 4u32;
pub const UI_CAP_SOFTKBD: u32 = 65536u32;
pub const VERSION_DOCUMENTFEED: u32 = 1u32;
pub const VERSION_ID_CHINESE_SIMPLIFIED: u32 = 134217728u32;
pub const VERSION_ID_CHINESE_TRADITIONAL: u32 = 67108864u32;
pub const VERSION_ID_JAPANESE: u32 = 16777216u32;
pub const VERSION_ID_KOREAN: u32 = 33554432u32;
pub const VERSION_MODEBIAS: u32 = 1u32;
pub const VERSION_MOUSE_OPERATION: u32 = 1u32;
pub const VERSION_QUERYPOSITION: u32 = 1u32;
pub const VERSION_RECONVERSION: u32 = 1u32;
#[repr(C, packed(1))]
pub struct WDD {
    pub wDispPos: u16,
    pub Anonymous1: WDD_0,
    pub cchDisp: u16,
    pub Anonymous2: WDD_1,
    pub WDD_nReserve1: u32,
    pub nPos: u16,
    pub _bitfield: u16,
    pub pReserved: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WDD {}
impl ::core::clone::Clone for WDD {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for WDD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WDD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WDD>()) == 0 }
    }
}
impl ::core::cmp::Eq for WDD {}
impl ::core::default::Default for WDD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub union WDD_0 {
    pub wReadPos: u16,
    pub wCompPos: u16,
}
impl ::core::marker::Copy for WDD_0 {}
impl ::core::clone::Clone for WDD_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for WDD_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WDD_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WDD_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WDD_0 {}
impl ::core::default::Default for WDD_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub union WDD_1 {
    pub cchRead: u16,
    pub cchComp: u16,
}
impl ::core::marker::Copy for WDD_1 {}
impl ::core::clone::Clone for WDD_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for WDD_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WDD_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WDD_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for WDD_1 {}
impl ::core::default::Default for WDD_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const cbCommentMax: u32 = 256u32;
pub type fpCreateIFECommonInstanceType = ::core::option::Option<unsafe extern "system" fn(ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT>;
pub type fpCreateIFEDictionaryInstanceType = ::core::option::Option<unsafe extern "system" fn(ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT>;
pub type fpCreateIFELanguageInstanceType = ::core::option::Option<unsafe extern "system" fn(clsid: *const ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT>;
pub const szImeChina: &str = "MSIME.China";
pub const szImeJapan: &str = "MSIME.Japan";
pub const szImeKorea: &str = "MSIME.Korea";
pub const szImeTaiwan: &str = "MSIME.Taiwan";
#[repr(C)]
pub struct tabIMEFAREASTINFO {
    pub dwSize: u32,
    pub dwType: u32,
    pub dwData: [u32; 1],
}
impl ::core::marker::Copy for tabIMEFAREASTINFO {}
impl ::core::clone::Clone for tabIMEFAREASTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for tabIMEFAREASTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("tabIMEFAREASTINFO").field("dwSize", &self.dwSize).field("dwType", &self.dwType).field("dwData", &self.dwData).finish()
    }
}
unsafe impl ::windows_core::Abi for tabIMEFAREASTINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for tabIMEFAREASTINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<tabIMEFAREASTINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for tabIMEFAREASTINFO {}
impl ::core::default::Default for tabIMEFAREASTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct tabIMESTRINGINFO {
    pub dwFarEastId: u32,
    pub lpwstr: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for tabIMESTRINGINFO {}
impl ::core::clone::Clone for tabIMESTRINGINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for tabIMESTRINGINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("tabIMESTRINGINFO").field("dwFarEastId", &self.dwFarEastId).field("lpwstr", &self.lpwstr).finish()
    }
}
unsafe impl ::windows_core::Abi for tabIMESTRINGINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for tabIMESTRINGINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<tabIMESTRINGINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for tabIMESTRINGINFO {}
impl ::core::default::Default for tabIMESTRINGINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const wchPrivate1: u32 = 57344u32;
