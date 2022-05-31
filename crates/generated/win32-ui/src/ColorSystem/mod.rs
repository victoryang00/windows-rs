pub const ATTRIB_MATTE: u32 = 2u32;
pub const ATTRIB_TRANSPARENCY: u32 = 1u32;
#[inline]
pub unsafe fn AssociateColorProfileWithDeviceA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(pmachinename: Param0, pprofilename: Param1, pdevicename: Param2) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AssociateColorProfileWithDeviceA(pmachinename: ::windows_core::PCSTR, pprofilename: ::windows_core::PCSTR, pdevicename: ::windows_core::PCSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AssociateColorProfileWithDeviceA(pmachinename.into_param().abi(), pprofilename.into_param().abi(), pdevicename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AssociateColorProfileWithDeviceW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pmachinename: Param0, pprofilename: Param1, pdevicename: Param2) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AssociateColorProfileWithDeviceW(pmachinename: ::windows_core::PCWSTR, pprofilename: ::windows_core::PCWSTR, pdevicename: ::windows_core::PCWSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AssociateColorProfileWithDeviceW(pmachinename.into_param().abi(), pprofilename.into_param().abi(), pdevicename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const BEST_MODE: u32 = 3u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BMFORMAT(pub i32);
pub const BM_x555RGB: BMFORMAT = BMFORMAT(0i32);
pub const BM_x555XYZ: BMFORMAT = BMFORMAT(257i32);
pub const BM_x555Yxy: BMFORMAT = BMFORMAT(258i32);
pub const BM_x555Lab: BMFORMAT = BMFORMAT(259i32);
pub const BM_x555G3CH: BMFORMAT = BMFORMAT(260i32);
pub const BM_RGBTRIPLETS: BMFORMAT = BMFORMAT(2i32);
pub const BM_BGRTRIPLETS: BMFORMAT = BMFORMAT(4i32);
pub const BM_XYZTRIPLETS: BMFORMAT = BMFORMAT(513i32);
pub const BM_YxyTRIPLETS: BMFORMAT = BMFORMAT(514i32);
pub const BM_LabTRIPLETS: BMFORMAT = BMFORMAT(515i32);
pub const BM_G3CHTRIPLETS: BMFORMAT = BMFORMAT(516i32);
pub const BM_5CHANNEL: BMFORMAT = BMFORMAT(517i32);
pub const BM_6CHANNEL: BMFORMAT = BMFORMAT(518i32);
pub const BM_7CHANNEL: BMFORMAT = BMFORMAT(519i32);
pub const BM_8CHANNEL: BMFORMAT = BMFORMAT(520i32);
pub const BM_GRAY: BMFORMAT = BMFORMAT(521i32);
pub const BM_xRGBQUADS: BMFORMAT = BMFORMAT(8i32);
pub const BM_xBGRQUADS: BMFORMAT = BMFORMAT(16i32);
pub const BM_xG3CHQUADS: BMFORMAT = BMFORMAT(772i32);
pub const BM_KYMCQUADS: BMFORMAT = BMFORMAT(773i32);
pub const BM_CMYKQUADS: BMFORMAT = BMFORMAT(32i32);
pub const BM_10b_RGB: BMFORMAT = BMFORMAT(9i32);
pub const BM_10b_XYZ: BMFORMAT = BMFORMAT(1025i32);
pub const BM_10b_Yxy: BMFORMAT = BMFORMAT(1026i32);
pub const BM_10b_Lab: BMFORMAT = BMFORMAT(1027i32);
pub const BM_10b_G3CH: BMFORMAT = BMFORMAT(1028i32);
pub const BM_NAMED_INDEX: BMFORMAT = BMFORMAT(1029i32);
pub const BM_16b_RGB: BMFORMAT = BMFORMAT(10i32);
pub const BM_16b_XYZ: BMFORMAT = BMFORMAT(1281i32);
pub const BM_16b_Yxy: BMFORMAT = BMFORMAT(1282i32);
pub const BM_16b_Lab: BMFORMAT = BMFORMAT(1283i32);
pub const BM_16b_G3CH: BMFORMAT = BMFORMAT(1284i32);
pub const BM_16b_GRAY: BMFORMAT = BMFORMAT(1285i32);
pub const BM_565RGB: BMFORMAT = BMFORMAT(1i32);
pub const BM_32b_scRGB: BMFORMAT = BMFORMAT(1537i32);
pub const BM_32b_scARGB: BMFORMAT = BMFORMAT(1538i32);
pub const BM_S2DOT13FIXED_scRGB: BMFORMAT = BMFORMAT(1539i32);
pub const BM_S2DOT13FIXED_scARGB: BMFORMAT = BMFORMAT(1540i32);
pub const BM_R10G10B10A2: BMFORMAT = BMFORMAT(1793i32);
pub const BM_R10G10B10A2_XR: BMFORMAT = BMFORMAT(1794i32);
pub const BM_R16G16B16A16_FLOAT: BMFORMAT = BMFORMAT(1795i32);
impl ::core::marker::Copy for BMFORMAT {}
impl ::core::clone::Clone for BMFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BMFORMAT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BMFORMAT {
    type Abi = Self;
}
impl ::core::fmt::Debug for BMFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BMFORMAT").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct BlackInformation {
    pub fBlackOnly: ::win32_foundation::BOOL,
    pub blackWeight: f32,
}
impl ::core::marker::Copy for BlackInformation {}
impl ::core::clone::Clone for BlackInformation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BlackInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BlackInformation").field("fBlackOnly", &self.fBlackOnly).field("blackWeight", &self.blackWeight).finish()
    }
}
unsafe impl ::windows_core::Abi for BlackInformation {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BlackInformation {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BlackInformation>()) == 0 }
    }
}
impl ::core::cmp::Eq for BlackInformation {}
impl ::core::default::Default for BlackInformation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const CATID_WcsPlugin: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa0b402e0_8240_405f_8a16_8a5b4df2f0dd);
#[inline]
pub unsafe fn CMCheckColors(hcmtransform: isize, lpainputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, lparesult: *mut u8) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMCheckColors(hcmtransform: isize, lpainputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, lparesult: *mut u8) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CMCheckColors(::core::mem::transmute(hcmtransform), ::core::mem::transmute(lpainputcolors), ::core::mem::transmute(ncolors), ::core::mem::transmute(ctinput), ::core::mem::transmute(lparesult)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CMCheckColorsInGamut(hcmtransform: isize, lpargbtriple: &[::win32_graphics::Gdi::RGBTRIPLE], lparesult: *mut u8) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMCheckColorsInGamut(hcmtransform: isize, lpargbtriple: *const ::win32_graphics::Gdi::RGBTRIPLE, lparesult: *mut u8, ncount: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CMCheckColorsInGamut(::core::mem::transmute(hcmtransform), ::core::mem::transmute(::windows_core::as_ptr_or_null(lpargbtriple)), ::core::mem::transmute(lparesult), lpargbtriple.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CMCheckRGBs<'a, Param8: ::windows_core::IntoParam<'a, ::win32_foundation::LPARAM>>(hcmtransform: isize, lpsrcbits: *const ::core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwstride: u32, lparesult: *mut u8, pfncallback: LPBMCALLBACKFN, ulcallbackdata: Param8) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMCheckRGBs(hcmtransform: isize, lpsrcbits: *const ::core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwstride: u32, lparesult: *mut u8, pfncallback: ::windows_core::RawPtr, ulcallbackdata: ::win32_foundation::LPARAM) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CMCheckRGBs(::core::mem::transmute(hcmtransform), ::core::mem::transmute(lpsrcbits), ::core::mem::transmute(bminput), ::core::mem::transmute(dwwidth), ::core::mem::transmute(dwheight), ::core::mem::transmute(dwstride), ::core::mem::transmute(lparesult), ::core::mem::transmute(pfncallback), ulcallbackdata.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CMConvertColorNameToIndex(hprofile: isize, pacolorname: *const *const i8, paindex: *mut u32, dwcount: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMConvertColorNameToIndex(hprofile: isize, pacolorname: *const *const i8, paindex: *mut u32, dwcount: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CMConvertColorNameToIndex(::core::mem::transmute(hprofile), ::core::mem::transmute(pacolorname), ::core::mem::transmute(paindex), ::core::mem::transmute(dwcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CMConvertIndexToColorName(hprofile: isize, paindex: *const u32, pacolorname: *mut *mut i8, dwcount: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMConvertIndexToColorName(hprofile: isize, paindex: *const u32, pacolorname: *mut *mut i8, dwcount: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CMConvertIndexToColorName(::core::mem::transmute(hprofile), ::core::mem::transmute(paindex), ::core::mem::transmute(pacolorname), ::core::mem::transmute(dwcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CMCreateDeviceLinkProfile(pahprofiles: &[isize], padwintents: &[u32], dwflags: u32, lpprofiledata: *mut *mut u8) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMCreateDeviceLinkProfile(pahprofiles: *const isize, nprofiles: u32, padwintents: *const u32, nintents: u32, dwflags: u32, lpprofiledata: *mut *mut u8) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CMCreateDeviceLinkProfile(::core::mem::transmute(::windows_core::as_ptr_or_null(pahprofiles)), pahprofiles.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(padwintents)), padwintents.len() as _, ::core::mem::transmute(dwflags), ::core::mem::transmute(lpprofiledata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CMCreateMultiProfileTransform(pahprofiles: &[isize], padwintents: &[u32], dwflags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMCreateMultiProfileTransform(pahprofiles: *const isize, nprofiles: u32, padwintents: *const u32, nintents: u32, dwflags: u32) -> isize;
        }
        ::core::mem::transmute(CMCreateMultiProfileTransform(::core::mem::transmute(::windows_core::as_ptr_or_null(pahprofiles)), pahprofiles.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(padwintents)), padwintents.len() as _, ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CMCreateProfile(lpcolorspace: *mut LOGCOLORSPACEA, lpprofiledata: *mut *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMCreateProfile(lpcolorspace: *mut LOGCOLORSPACEA, lpprofiledata: *mut *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CMCreateProfile(::core::mem::transmute(lpcolorspace), ::core::mem::transmute(lpprofiledata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CMCreateProfileW(lpcolorspace: *mut LOGCOLORSPACEW, lpprofiledata: *mut *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMCreateProfileW(lpcolorspace: *mut LOGCOLORSPACEW, lpprofiledata: *mut *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CMCreateProfileW(::core::mem::transmute(lpcolorspace), ::core::mem::transmute(lpprofiledata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CMCreateTransform(lpcolorspace: *const LOGCOLORSPACEA, lpdevcharacter: *const ::core::ffi::c_void, lptargetdevcharacter: *const ::core::ffi::c_void) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMCreateTransform(lpcolorspace: *const LOGCOLORSPACEA, lpdevcharacter: *const ::core::ffi::c_void, lptargetdevcharacter: *const ::core::ffi::c_void) -> isize;
        }
        ::core::mem::transmute(CMCreateTransform(::core::mem::transmute(lpcolorspace), ::core::mem::transmute(lpdevcharacter), ::core::mem::transmute(lptargetdevcharacter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CMCreateTransformExt(lpcolorspace: *const LOGCOLORSPACEA, lpdevcharacter: *const ::core::ffi::c_void, lptargetdevcharacter: *const ::core::ffi::c_void, dwflags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMCreateTransformExt(lpcolorspace: *const LOGCOLORSPACEA, lpdevcharacter: *const ::core::ffi::c_void, lptargetdevcharacter: *const ::core::ffi::c_void, dwflags: u32) -> isize;
        }
        ::core::mem::transmute(CMCreateTransformExt(::core::mem::transmute(lpcolorspace), ::core::mem::transmute(lpdevcharacter), ::core::mem::transmute(lptargetdevcharacter), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CMCreateTransformExtW(lpcolorspace: *const LOGCOLORSPACEW, lpdevcharacter: *const ::core::ffi::c_void, lptargetdevcharacter: *const ::core::ffi::c_void, dwflags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMCreateTransformExtW(lpcolorspace: *const LOGCOLORSPACEW, lpdevcharacter: *const ::core::ffi::c_void, lptargetdevcharacter: *const ::core::ffi::c_void, dwflags: u32) -> isize;
        }
        ::core::mem::transmute(CMCreateTransformExtW(::core::mem::transmute(lpcolorspace), ::core::mem::transmute(lpdevcharacter), ::core::mem::transmute(lptargetdevcharacter), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CMCreateTransformW(lpcolorspace: *const LOGCOLORSPACEW, lpdevcharacter: *const ::core::ffi::c_void, lptargetdevcharacter: *const ::core::ffi::c_void) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMCreateTransformW(lpcolorspace: *const LOGCOLORSPACEW, lpdevcharacter: *const ::core::ffi::c_void, lptargetdevcharacter: *const ::core::ffi::c_void) -> isize;
        }
        ::core::mem::transmute(CMCreateTransformW(::core::mem::transmute(lpcolorspace), ::core::mem::transmute(lpdevcharacter), ::core::mem::transmute(lptargetdevcharacter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CMDeleteTransform(hcmtransform: isize) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMDeleteTransform(hcmtransform: isize) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CMDeleteTransform(::core::mem::transmute(hcmtransform)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CMGetInfo(dwinfo: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMGetInfo(dwinfo: u32) -> u32;
        }
        ::core::mem::transmute(CMGetInfo(::core::mem::transmute(dwinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CMGetNamedProfileInfo(hprofile: isize, pnamedprofileinfo: *mut NAMED_PROFILE_INFO) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMGetNamedProfileInfo(hprofile: isize, pnamedprofileinfo: *mut NAMED_PROFILE_INFO) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CMGetNamedProfileInfo(::core::mem::transmute(hprofile), ::core::mem::transmute(pnamedprofileinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CMIsProfileValid(hprofile: isize, lpbvalid: *mut i32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMIsProfileValid(hprofile: isize, lpbvalid: *mut i32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CMIsProfileValid(::core::mem::transmute(hprofile), ::core::mem::transmute(lpbvalid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const CMM_DESCRIPTION: u32 = 5u32;
pub const CMM_DLL_VERSION: u32 = 3u32;
pub const CMM_DRIVER_VERSION: u32 = 2u32;
pub const CMM_FROM_PROFILE: u32 = 0u32;
pub const CMM_IDENT: u32 = 1u32;
pub const CMM_LOGOICON: u32 = 6u32;
pub const CMM_VERSION: u32 = 4u32;
pub const CMM_WIN_VERSION: u32 = 0u32;
pub const CMS_BACKWARD: u32 = 1u32;
pub const CMS_DISABLEICM: u32 = 1u32;
pub const CMS_DISABLEINTENT: u32 = 1024u32;
pub const CMS_DISABLERENDERINTENT: u32 = 2048u32;
pub const CMS_ENABLEPROOFING: u32 = 2u32;
pub const CMS_FORWARD: u32 = 0u32;
pub const CMS_MONITOROVERFLOW: i32 = -2147483648i32;
pub const CMS_PRINTEROVERFLOW: i32 = 1073741824i32;
pub const CMS_SETMONITORPROFILE: u32 = 16u32;
pub const CMS_SETPRINTERPROFILE: u32 = 32u32;
pub const CMS_SETPROOFINTENT: u32 = 8u32;
pub const CMS_SETRENDERINTENT: u32 = 4u32;
pub const CMS_SETTARGETPROFILE: u32 = 64u32;
pub const CMS_TARGETOVERFLOW: i32 = 536870912i32;
pub const CMS_USEAPPLYCALLBACK: u32 = 256u32;
pub const CMS_USEDESCRIPTION: u32 = 512u32;
pub const CMS_USEHOOK: u32 = 128u32;
#[inline]
pub unsafe fn CMTranslateColors(hcmtransform: isize, lpainputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, lpaoutputcolors: *mut COLOR, ctoutput: COLORTYPE) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMTranslateColors(hcmtransform: isize, lpainputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, lpaoutputcolors: *mut COLOR, ctoutput: COLORTYPE) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CMTranslateColors(::core::mem::transmute(hcmtransform), ::core::mem::transmute(lpainputcolors), ::core::mem::transmute(ncolors), ::core::mem::transmute(ctinput), ::core::mem::transmute(lpaoutputcolors), ::core::mem::transmute(ctoutput)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CMTranslateRGB(hcmtransform: isize, colorref: u32, lpcolorref: *mut u32, dwflags: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMTranslateRGB(hcmtransform: isize, colorref: u32, lpcolorref: *mut u32, dwflags: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CMTranslateRGB(::core::mem::transmute(hcmtransform), ::core::mem::transmute(colorref), ::core::mem::transmute(lpcolorref), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CMTranslateRGBs(hcmtransform: isize, lpsrcbits: *const ::core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwstride: u32, lpdestbits: *mut ::core::ffi::c_void, bmoutput: BMFORMAT, dwtranslatedirection: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMTranslateRGBs(hcmtransform: isize, lpsrcbits: *const ::core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwstride: u32, lpdestbits: *mut ::core::ffi::c_void, bmoutput: BMFORMAT, dwtranslatedirection: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CMTranslateRGBs(::core::mem::transmute(hcmtransform), ::core::mem::transmute(lpsrcbits), ::core::mem::transmute(bminput), ::core::mem::transmute(dwwidth), ::core::mem::transmute(dwheight), ::core::mem::transmute(dwstride), ::core::mem::transmute(lpdestbits), ::core::mem::transmute(bmoutput), ::core::mem::transmute(dwtranslatedirection)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CMTranslateRGBsExt<'a, Param10: ::windows_core::IntoParam<'a, ::win32_foundation::LPARAM>>(hcmtransform: isize, lpsrcbits: *const ::core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwinputstride: u32, lpdestbits: *mut ::core::ffi::c_void, bmoutput: BMFORMAT, dwoutputstride: u32, lpfncallback: LPBMCALLBACKFN, ulcallbackdata: Param10) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMTranslateRGBsExt(hcmtransform: isize, lpsrcbits: *const ::core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwinputstride: u32, lpdestbits: *mut ::core::ffi::c_void, bmoutput: BMFORMAT, dwoutputstride: u32, lpfncallback: ::windows_core::RawPtr, ulcallbackdata: ::win32_foundation::LPARAM) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CMTranslateRGBsExt(::core::mem::transmute(hcmtransform), ::core::mem::transmute(lpsrcbits), ::core::mem::transmute(bminput), ::core::mem::transmute(dwwidth), ::core::mem::transmute(dwheight), ::core::mem::transmute(dwinputstride), ::core::mem::transmute(lpdestbits), ::core::mem::transmute(bmoutput), ::core::mem::transmute(dwoutputstride), ::core::mem::transmute(lpfncallback), ulcallbackdata.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct CMYKCOLOR {
    pub cyan: u16,
    pub magenta: u16,
    pub yellow: u16,
    pub black: u16,
}
impl ::core::marker::Copy for CMYKCOLOR {}
impl ::core::clone::Clone for CMYKCOLOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CMYKCOLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMYKCOLOR").field("cyan", &self.cyan).field("magenta", &self.magenta).field("yellow", &self.yellow).field("black", &self.black).finish()
    }
}
unsafe impl ::windows_core::Abi for CMYKCOLOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CMYKCOLOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CMYKCOLOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for CMYKCOLOR {}
impl ::core::default::Default for CMYKCOLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union COLOR {
    pub gray: GRAYCOLOR,
    pub rgb: RGBCOLOR,
    pub cmyk: CMYKCOLOR,
    pub XYZ: XYZCOLOR,
    pub Yxy: YxyCOLOR,
    pub Lab: LabCOLOR,
    pub gen3ch: GENERIC3CHANNEL,
    pub named: NAMEDCOLOR,
    pub hifi: HiFiCOLOR,
    pub Anonymous: COLOR_0,
}
impl ::core::marker::Copy for COLOR {}
impl ::core::clone::Clone for COLOR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for COLOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COLOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COLOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for COLOR {}
impl ::core::default::Default for COLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct COLOR_0 {
    pub reserved1: u32,
    pub reserved2: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for COLOR_0 {}
impl ::core::clone::Clone for COLOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COLOR_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COLOR_0").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).finish()
    }
}
unsafe impl ::windows_core::Abi for COLOR_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COLOR_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COLOR_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for COLOR_0 {}
impl ::core::default::Default for COLOR_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COLORDATATYPE(pub i32);
pub const COLOR_BYTE: COLORDATATYPE = COLORDATATYPE(1i32);
pub const COLOR_WORD: COLORDATATYPE = COLORDATATYPE(2i32);
pub const COLOR_FLOAT: COLORDATATYPE = COLORDATATYPE(3i32);
pub const COLOR_S2DOT13FIXED: COLORDATATYPE = COLORDATATYPE(4i32);
pub const COLOR_10b_R10G10B10A2: COLORDATATYPE = COLORDATATYPE(5i32);
pub const COLOR_10b_R10G10B10A2_XR: COLORDATATYPE = COLORDATATYPE(6i32);
pub const COLOR_FLOAT16: COLORDATATYPE = COLORDATATYPE(7i32);
impl ::core::marker::Copy for COLORDATATYPE {}
impl ::core::clone::Clone for COLORDATATYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COLORDATATYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COLORDATATYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for COLORDATATYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COLORDATATYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub struct COLORMATCHSETUPA {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub hwndOwner: ::win32_foundation::HWND,
    pub pSourceName: ::windows_core::PCSTR,
    pub pDisplayName: ::windows_core::PCSTR,
    pub pPrinterName: ::windows_core::PCSTR,
    pub dwRenderIntent: u32,
    pub dwProofingIntent: u32,
    pub pMonitorProfile: ::windows_core::PSTR,
    pub ccMonitorProfile: u32,
    pub pPrinterProfile: ::windows_core::PSTR,
    pub ccPrinterProfile: u32,
    pub pTargetProfile: ::windows_core::PSTR,
    pub ccTargetProfile: u32,
    pub lpfnHook: super::WindowsAndMessaging::DLGPROC,
    pub lParam: ::win32_foundation::LPARAM,
    pub lpfnApplyCallback: PCMSCALLBACKA,
    pub lParamApplyCallback: ::win32_foundation::LPARAM,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::marker::Copy for COLORMATCHSETUPA {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::clone::Clone for COLORMATCHSETUPA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::fmt::Debug for COLORMATCHSETUPA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COLORMATCHSETUPA")
            .field("dwSize", &self.dwSize)
            .field("dwVersion", &self.dwVersion)
            .field("dwFlags", &self.dwFlags)
            .field("hwndOwner", &self.hwndOwner)
            .field("pSourceName", &self.pSourceName)
            .field("pDisplayName", &self.pDisplayName)
            .field("pPrinterName", &self.pPrinterName)
            .field("dwRenderIntent", &self.dwRenderIntent)
            .field("dwProofingIntent", &self.dwProofingIntent)
            .field("pMonitorProfile", &self.pMonitorProfile)
            .field("ccMonitorProfile", &self.ccMonitorProfile)
            .field("pPrinterProfile", &self.pPrinterProfile)
            .field("ccPrinterProfile", &self.ccPrinterProfile)
            .field("pTargetProfile", &self.pTargetProfile)
            .field("ccTargetProfile", &self.ccTargetProfile)
            .field("lpfnHook", &self.lpfnHook.map(|f| f as usize))
            .field("lParam", &self.lParam)
            .field("lpfnApplyCallback", &self.lpfnApplyCallback.map(|f| f as usize))
            .field("lParamApplyCallback", &self.lParamApplyCallback)
            .finish()
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
unsafe impl ::windows_core::Abi for COLORMATCHSETUPA {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::PartialEq for COLORMATCHSETUPA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COLORMATCHSETUPA>()) == 0 }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::Eq for COLORMATCHSETUPA {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for COLORMATCHSETUPA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub struct COLORMATCHSETUPW {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub hwndOwner: ::win32_foundation::HWND,
    pub pSourceName: ::windows_core::PCWSTR,
    pub pDisplayName: ::windows_core::PCWSTR,
    pub pPrinterName: ::windows_core::PCWSTR,
    pub dwRenderIntent: u32,
    pub dwProofingIntent: u32,
    pub pMonitorProfile: ::windows_core::PWSTR,
    pub ccMonitorProfile: u32,
    pub pPrinterProfile: ::windows_core::PWSTR,
    pub ccPrinterProfile: u32,
    pub pTargetProfile: ::windows_core::PWSTR,
    pub ccTargetProfile: u32,
    pub lpfnHook: super::WindowsAndMessaging::DLGPROC,
    pub lParam: ::win32_foundation::LPARAM,
    pub lpfnApplyCallback: PCMSCALLBACKW,
    pub lParamApplyCallback: ::win32_foundation::LPARAM,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::marker::Copy for COLORMATCHSETUPW {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::clone::Clone for COLORMATCHSETUPW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::fmt::Debug for COLORMATCHSETUPW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COLORMATCHSETUPW")
            .field("dwSize", &self.dwSize)
            .field("dwVersion", &self.dwVersion)
            .field("dwFlags", &self.dwFlags)
            .field("hwndOwner", &self.hwndOwner)
            .field("pSourceName", &self.pSourceName)
            .field("pDisplayName", &self.pDisplayName)
            .field("pPrinterName", &self.pPrinterName)
            .field("dwRenderIntent", &self.dwRenderIntent)
            .field("dwProofingIntent", &self.dwProofingIntent)
            .field("pMonitorProfile", &self.pMonitorProfile)
            .field("ccMonitorProfile", &self.ccMonitorProfile)
            .field("pPrinterProfile", &self.pPrinterProfile)
            .field("ccPrinterProfile", &self.ccPrinterProfile)
            .field("pTargetProfile", &self.pTargetProfile)
            .field("ccTargetProfile", &self.ccTargetProfile)
            .field("lpfnHook", &self.lpfnHook.map(|f| f as usize))
            .field("lParam", &self.lParam)
            .field("lpfnApplyCallback", &self.lpfnApplyCallback.map(|f| f as usize))
            .field("lParamApplyCallback", &self.lParamApplyCallback)
            .finish()
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
unsafe impl ::windows_core::Abi for COLORMATCHSETUPW {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::PartialEq for COLORMATCHSETUPW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COLORMATCHSETUPW>()) == 0 }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::Eq for COLORMATCHSETUPW {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for COLORMATCHSETUPW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COLORPROFILESUBTYPE(pub i32);
pub const CPST_PERCEPTUAL: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(0i32);
pub const CPST_RELATIVE_COLORIMETRIC: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(1i32);
pub const CPST_SATURATION: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(2i32);
pub const CPST_ABSOLUTE_COLORIMETRIC: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(3i32);
pub const CPST_NONE: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(4i32);
pub const CPST_RGB_WORKING_SPACE: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(5i32);
pub const CPST_CUSTOM_WORKING_SPACE: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(6i32);
pub const CPST_STANDARD_DISPLAY_COLOR_MODE: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(7i32);
pub const CPST_EXTENDED_DISPLAY_COLOR_MODE: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(8i32);
impl ::core::marker::Copy for COLORPROFILESUBTYPE {}
impl ::core::clone::Clone for COLORPROFILESUBTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COLORPROFILESUBTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COLORPROFILESUBTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for COLORPROFILESUBTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COLORPROFILESUBTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COLORPROFILETYPE(pub i32);
pub const CPT_ICC: COLORPROFILETYPE = COLORPROFILETYPE(0i32);
pub const CPT_DMP: COLORPROFILETYPE = COLORPROFILETYPE(1i32);
pub const CPT_CAMP: COLORPROFILETYPE = COLORPROFILETYPE(2i32);
pub const CPT_GMMP: COLORPROFILETYPE = COLORPROFILETYPE(3i32);
impl ::core::marker::Copy for COLORPROFILETYPE {}
impl ::core::clone::Clone for COLORPROFILETYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COLORPROFILETYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COLORPROFILETYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for COLORPROFILETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COLORPROFILETYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COLORTYPE(pub i32);
pub const COLOR_GRAY: COLORTYPE = COLORTYPE(1i32);
pub const COLOR_RGB: COLORTYPE = COLORTYPE(2i32);
pub const COLOR_XYZ: COLORTYPE = COLORTYPE(3i32);
pub const COLOR_Yxy: COLORTYPE = COLORTYPE(4i32);
pub const COLOR_Lab: COLORTYPE = COLORTYPE(5i32);
pub const COLOR_3_CHANNEL: COLORTYPE = COLORTYPE(6i32);
pub const COLOR_CMYK: COLORTYPE = COLORTYPE(7i32);
pub const COLOR_5_CHANNEL: COLORTYPE = COLORTYPE(8i32);
pub const COLOR_6_CHANNEL: COLORTYPE = COLORTYPE(9i32);
pub const COLOR_7_CHANNEL: COLORTYPE = COLORTYPE(10i32);
pub const COLOR_8_CHANNEL: COLORTYPE = COLORTYPE(11i32);
pub const COLOR_NAMED: COLORTYPE = COLORTYPE(12i32);
impl ::core::marker::Copy for COLORTYPE {}
impl ::core::clone::Clone for COLORTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COLORTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COLORTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for COLORTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COLORTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COLOR_MATCH_TO_TARGET_ACTION(pub i32);
pub const CS_ENABLE: COLOR_MATCH_TO_TARGET_ACTION = COLOR_MATCH_TO_TARGET_ACTION(1i32);
pub const CS_DISABLE: COLOR_MATCH_TO_TARGET_ACTION = COLOR_MATCH_TO_TARGET_ACTION(2i32);
pub const CS_DELETE_TRANSFORM: COLOR_MATCH_TO_TARGET_ACTION = COLOR_MATCH_TO_TARGET_ACTION(3i32);
impl ::core::marker::Copy for COLOR_MATCH_TO_TARGET_ACTION {}
impl ::core::clone::Clone for COLOR_MATCH_TO_TARGET_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COLOR_MATCH_TO_TARGET_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COLOR_MATCH_TO_TARGET_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for COLOR_MATCH_TO_TARGET_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COLOR_MATCH_TO_TARGET_ACTION").field(&self.0).finish()
    }
}
pub const COLOR_MATCH_VERSION: u32 = 512u32;
pub const CSA_A: u32 = 1u32;
pub const CSA_ABC: u32 = 2u32;
pub const CSA_CMYK: u32 = 7u32;
pub const CSA_DEF: u32 = 3u32;
pub const CSA_DEFG: u32 = 4u32;
pub const CSA_GRAY: u32 = 5u32;
pub const CSA_Lab: u32 = 8u32;
pub const CSA_RGB: u32 = 6u32;
#[inline]
pub unsafe fn CheckBitmapBits<'a, Param8: ::windows_core::IntoParam<'a, ::win32_foundation::LPARAM>>(hcolortransform: isize, psrcbits: *const ::core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwstride: u32, paresult: *mut u8, pfncallback: LPBMCALLBACKFN, lpcallbackdata: Param8) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckBitmapBits(hcolortransform: isize, psrcbits: *const ::core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwstride: u32, paresult: *mut u8, pfncallback: ::windows_core::RawPtr, lpcallbackdata: ::win32_foundation::LPARAM) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CheckBitmapBits(::core::mem::transmute(hcolortransform), ::core::mem::transmute(psrcbits), ::core::mem::transmute(bminput), ::core::mem::transmute(dwwidth), ::core::mem::transmute(dwheight), ::core::mem::transmute(dwstride), ::core::mem::transmute(paresult), ::core::mem::transmute(pfncallback), lpcallbackdata.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CheckColors(hcolortransform: isize, painputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, paresult: *mut u8) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckColors(hcolortransform: isize, painputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, paresult: *mut u8) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CheckColors(::core::mem::transmute(hcolortransform), ::core::mem::transmute(painputcolors), ::core::mem::transmute(ncolors), ::core::mem::transmute(ctinput), ::core::mem::transmute(paresult)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CheckColorsInGamut<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(hdc: Param0, lprgbtriple: &[::win32_graphics::Gdi::RGBTRIPLE], dlpbuffer: *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckColorsInGamut(hdc: ::win32_graphics::Gdi::HDC, lprgbtriple: *const ::win32_graphics::Gdi::RGBTRIPLE, dlpbuffer: *mut ::core::ffi::c_void, ncount: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CheckColorsInGamut(hdc.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(lprgbtriple)), ::core::mem::transmute(dlpbuffer), lprgbtriple.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CloseColorProfile(hprofile: isize) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseColorProfile(hprofile: isize) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CloseColorProfile(::core::mem::transmute(hprofile)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ColorCorrectPalette<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HPALETTE>>(hdc: Param0, hpal: Param1, defirst: u32, num: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ColorCorrectPalette(hdc: ::win32_graphics::Gdi::HDC, hpal: ::win32_graphics::Gdi::HPALETTE, defirst: u32, num: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ColorCorrectPalette(hdc.into_param().abi(), hpal.into_param().abi(), ::core::mem::transmute(defirst), ::core::mem::transmute(num)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ColorMatchToTarget<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(hdc: Param0, hdctarget: Param1, action: COLOR_MATCH_TO_TARGET_ACTION) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ColorMatchToTarget(hdc: ::win32_graphics::Gdi::HDC, hdctarget: ::win32_graphics::Gdi::HDC, action: COLOR_MATCH_TO_TARGET_ACTION) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ColorMatchToTarget(hdc.into_param().abi(), hdctarget.into_param().abi(), ::core::mem::transmute(action)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ColorProfileAddDisplayAssociation<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::LUID>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, profilename: Param1, targetadapterid: Param2, sourceid: u32, setasdefault: Param4, associateasadvancedcolor: Param5) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ColorProfileAddDisplayAssociation(scope: WCS_PROFILE_MANAGEMENT_SCOPE, profilename: ::windows_core::PCWSTR, targetadapterid: ::win32_foundation::LUID, sourceid: u32, setasdefault: ::win32_foundation::BOOL, associateasadvancedcolor: ::win32_foundation::BOOL) -> ::windows_core::HRESULT;
        }
        ColorProfileAddDisplayAssociation(::core::mem::transmute(scope), profilename.into_param().abi(), targetadapterid.into_param().abi(), ::core::mem::transmute(sourceid), setasdefault.into_param().abi(), associateasadvancedcolor.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ColorProfileGetDisplayDefault<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::LUID>>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, targetadapterid: Param1, sourceid: u32, profiletype: COLORPROFILETYPE, profilesubtype: COLORPROFILESUBTYPE) -> ::windows_core::Result<::windows_core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ColorProfileGetDisplayDefault(scope: WCS_PROFILE_MANAGEMENT_SCOPE, targetadapterid: ::win32_foundation::LUID, sourceid: u32, profiletype: COLORPROFILETYPE, profilesubtype: COLORPROFILESUBTYPE, profilename: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        ColorProfileGetDisplayDefault(::core::mem::transmute(scope), targetadapterid.into_param().abi(), ::core::mem::transmute(sourceid), ::core::mem::transmute(profiletype), ::core::mem::transmute(profilesubtype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ColorProfileGetDisplayList<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::LUID>>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, targetadapterid: Param1, sourceid: u32, profilelist: *mut *mut ::windows_core::PWSTR, profilecount: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ColorProfileGetDisplayList(scope: WCS_PROFILE_MANAGEMENT_SCOPE, targetadapterid: ::win32_foundation::LUID, sourceid: u32, profilelist: *mut *mut ::windows_core::PWSTR, profilecount: *mut u32) -> ::windows_core::HRESULT;
        }
        ColorProfileGetDisplayList(::core::mem::transmute(scope), targetadapterid.into_param().abi(), ::core::mem::transmute(sourceid), ::core::mem::transmute(profilelist), ::core::mem::transmute(profilecount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ColorProfileGetDisplayUserScope<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::LUID>>(targetadapterid: Param0, sourceid: u32) -> ::windows_core::Result<WCS_PROFILE_MANAGEMENT_SCOPE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ColorProfileGetDisplayUserScope(targetadapterid: ::win32_foundation::LUID, sourceid: u32, scope: *mut WCS_PROFILE_MANAGEMENT_SCOPE) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<WCS_PROFILE_MANAGEMENT_SCOPE>::zeroed();
        ColorProfileGetDisplayUserScope(targetadapterid.into_param().abi(), ::core::mem::transmute(sourceid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WCS_PROFILE_MANAGEMENT_SCOPE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ColorProfileRemoveDisplayAssociation<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::LUID>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, profilename: Param1, targetadapterid: Param2, sourceid: u32, dissociateadvancedcolor: Param4) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ColorProfileRemoveDisplayAssociation(scope: WCS_PROFILE_MANAGEMENT_SCOPE, profilename: ::windows_core::PCWSTR, targetadapterid: ::win32_foundation::LUID, sourceid: u32, dissociateadvancedcolor: ::win32_foundation::BOOL) -> ::windows_core::HRESULT;
        }
        ColorProfileRemoveDisplayAssociation(::core::mem::transmute(scope), profilename.into_param().abi(), targetadapterid.into_param().abi(), ::core::mem::transmute(sourceid), dissociateadvancedcolor.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ColorProfileSetDisplayDefaultAssociation<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::LUID>>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, profilename: Param1, profiletype: COLORPROFILETYPE, profilesubtype: COLORPROFILESUBTYPE, targetadapterid: Param4, sourceid: u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ColorProfileSetDisplayDefaultAssociation(scope: WCS_PROFILE_MANAGEMENT_SCOPE, profilename: ::windows_core::PCWSTR, profiletype: COLORPROFILETYPE, profilesubtype: COLORPROFILESUBTYPE, targetadapterid: ::win32_foundation::LUID, sourceid: u32) -> ::windows_core::HRESULT;
        }
        ColorProfileSetDisplayDefaultAssociation(::core::mem::transmute(scope), profilename.into_param().abi(), ::core::mem::transmute(profiletype), ::core::mem::transmute(profilesubtype), targetadapterid.into_param().abi(), ::core::mem::transmute(sourceid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ConvertColorNameToIndex(hprofile: isize, pacolorname: *const *const i8, paindex: *mut u32, dwcount: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertColorNameToIndex(hprofile: isize, pacolorname: *const *const i8, paindex: *mut u32, dwcount: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ConvertColorNameToIndex(::core::mem::transmute(hprofile), ::core::mem::transmute(pacolorname), ::core::mem::transmute(paindex), ::core::mem::transmute(dwcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ConvertIndexToColorName(hprofile: isize, paindex: *const u32, pacolorname: *mut *mut i8, dwcount: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertIndexToColorName(hprofile: isize, paindex: *const u32, pacolorname: *mut *mut i8, dwcount: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ConvertIndexToColorName(::core::mem::transmute(hprofile), ::core::mem::transmute(paindex), ::core::mem::transmute(pacolorname), ::core::mem::transmute(dwcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CreateColorSpaceA(lplcs: *const LOGCOLORSPACEA) -> HCOLORSPACE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateColorSpaceA(lplcs: *const LOGCOLORSPACEA) -> HCOLORSPACE;
        }
        ::core::mem::transmute(CreateColorSpaceA(::core::mem::transmute(lplcs)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CreateColorSpaceW(lplcs: *const LOGCOLORSPACEW) -> HCOLORSPACE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateColorSpaceW(lplcs: *const LOGCOLORSPACEW) -> HCOLORSPACE;
        }
        ::core::mem::transmute(CreateColorSpaceW(::core::mem::transmute(lplcs)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CreateColorTransformA(plogcolorspace: *const LOGCOLORSPACEA, hdestprofile: isize, htargetprofile: isize, dwflags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateColorTransformA(plogcolorspace: *const LOGCOLORSPACEA, hdestprofile: isize, htargetprofile: isize, dwflags: u32) -> isize;
        }
        ::core::mem::transmute(CreateColorTransformA(::core::mem::transmute(plogcolorspace), ::core::mem::transmute(hdestprofile), ::core::mem::transmute(htargetprofile), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CreateColorTransformW(plogcolorspace: *const LOGCOLORSPACEW, hdestprofile: isize, htargetprofile: isize, dwflags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateColorTransformW(plogcolorspace: *const LOGCOLORSPACEW, hdestprofile: isize, htargetprofile: isize, dwflags: u32) -> isize;
        }
        ::core::mem::transmute(CreateColorTransformW(::core::mem::transmute(plogcolorspace), ::core::mem::transmute(hdestprofile), ::core::mem::transmute(htargetprofile), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateDeviceLinkProfile(hprofile: &[isize], padwintent: &[u32], dwflags: u32, pprofiledata: *mut *mut u8, indexpreferredcmm: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDeviceLinkProfile(hprofile: *const isize, nprofiles: u32, padwintent: *const u32, nintents: u32, dwflags: u32, pprofiledata: *mut *mut u8, indexpreferredcmm: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CreateDeviceLinkProfile(::core::mem::transmute(::windows_core::as_ptr_or_null(hprofile)), hprofile.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(padwintent)), padwintent.len() as _, ::core::mem::transmute(dwflags), ::core::mem::transmute(pprofiledata), ::core::mem::transmute(indexpreferredcmm)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateMultiProfileTransform(pahprofiles: &[isize], padwintent: &[u32], dwflags: u32, indexpreferredcmm: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateMultiProfileTransform(pahprofiles: *const isize, nprofiles: u32, padwintent: *const u32, nintents: u32, dwflags: u32, indexpreferredcmm: u32) -> isize;
        }
        ::core::mem::transmute(CreateMultiProfileTransform(::core::mem::transmute(::windows_core::as_ptr_or_null(pahprofiles)), pahprofiles.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(padwintent)), padwintent.len() as _, ::core::mem::transmute(dwflags), ::core::mem::transmute(indexpreferredcmm)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CreateProfileFromLogColorSpaceA(plogcolorspace: *const LOGCOLORSPACEA, pprofile: *mut *mut u8) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateProfileFromLogColorSpaceA(plogcolorspace: *const LOGCOLORSPACEA, pprofile: *mut *mut u8) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CreateProfileFromLogColorSpaceA(::core::mem::transmute(plogcolorspace), ::core::mem::transmute(pprofile)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CreateProfileFromLogColorSpaceW(plogcolorspace: *const LOGCOLORSPACEW, pprofile: *mut *mut u8) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateProfileFromLogColorSpaceW(plogcolorspace: *const LOGCOLORSPACEW, pprofile: *mut *mut u8) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CreateProfileFromLogColorSpaceW(::core::mem::transmute(plogcolorspace), ::core::mem::transmute(pprofile)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const DONT_USE_EMBEDDED_WCS_PROFILES: i32 = 1i32;
#[inline]
pub unsafe fn DeleteColorSpace<'a, Param0: ::windows_core::IntoParam<'a, HCOLORSPACE>>(hcs: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteColorSpace(hcs: HCOLORSPACE) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(DeleteColorSpace(hcs.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DeleteColorTransform(hxform: isize) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteColorTransform(hxform: isize) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(DeleteColorTransform(::core::mem::transmute(hxform)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DisassociateColorProfileFromDeviceA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(pmachinename: Param0, pprofilename: Param1, pdevicename: Param2) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DisassociateColorProfileFromDeviceA(pmachinename: ::windows_core::PCSTR, pprofilename: ::windows_core::PCSTR, pdevicename: ::windows_core::PCSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(DisassociateColorProfileFromDeviceA(pmachinename.into_param().abi(), pprofilename.into_param().abi(), pdevicename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DisassociateColorProfileFromDeviceW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pmachinename: Param0, pprofilename: Param1, pdevicename: Param2) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DisassociateColorProfileFromDeviceW(pmachinename: ::windows_core::PCWSTR, pprofilename: ::windows_core::PCWSTR, pdevicename: ::windows_core::PCWSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(DisassociateColorProfileFromDeviceW(pmachinename.into_param().abi(), pprofilename.into_param().abi(), pdevicename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct EMRCREATECOLORSPACE {
    pub emr: ::win32_graphics::Gdi::EMR,
    pub ihCS: u32,
    pub lcs: LOGCOLORSPACEA,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for EMRCREATECOLORSPACE {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for EMRCREATECOLORSPACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for EMRCREATECOLORSPACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRCREATECOLORSPACE").field("emr", &self.emr).field("ihCS", &self.ihCS).field("lcs", &self.lcs).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for EMRCREATECOLORSPACE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for EMRCREATECOLORSPACE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRCREATECOLORSPACE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for EMRCREATECOLORSPACE {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for EMRCREATECOLORSPACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct EMRCREATECOLORSPACEW {
    pub emr: ::win32_graphics::Gdi::EMR,
    pub ihCS: u32,
    pub lcs: LOGCOLORSPACEW,
    pub dwFlags: u32,
    pub cbData: u32,
    pub Data: [u8; 1],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for EMRCREATECOLORSPACEW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for EMRCREATECOLORSPACEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for EMRCREATECOLORSPACEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRCREATECOLORSPACEW").field("emr", &self.emr).field("ihCS", &self.ihCS).field("lcs", &self.lcs).field("dwFlags", &self.dwFlags).field("cbData", &self.cbData).field("Data", &self.Data).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for EMRCREATECOLORSPACEW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for EMRCREATECOLORSPACEW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EMRCREATECOLORSPACEW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for EMRCREATECOLORSPACEW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for EMRCREATECOLORSPACEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const ENABLE_GAMUT_CHECKING: u32 = 65536u32;
#[repr(C)]
pub struct ENUMTYPEA {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFields: u32,
    pub pDeviceName: ::windows_core::PCSTR,
    pub dwMediaType: u32,
    pub dwDitheringMode: u32,
    pub dwResolution: [u32; 2],
    pub dwCMMType: u32,
    pub dwClass: u32,
    pub dwDataColorSpace: u32,
    pub dwConnectionSpace: u32,
    pub dwSignature: u32,
    pub dwPlatform: u32,
    pub dwProfileFlags: u32,
    pub dwManufacturer: u32,
    pub dwModel: u32,
    pub dwAttributes: [u32; 2],
    pub dwRenderingIntent: u32,
    pub dwCreator: u32,
    pub dwDeviceClass: u32,
}
impl ::core::marker::Copy for ENUMTYPEA {}
impl ::core::clone::Clone for ENUMTYPEA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENUMTYPEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMTYPEA")
            .field("dwSize", &self.dwSize)
            .field("dwVersion", &self.dwVersion)
            .field("dwFields", &self.dwFields)
            .field("pDeviceName", &self.pDeviceName)
            .field("dwMediaType", &self.dwMediaType)
            .field("dwDitheringMode", &self.dwDitheringMode)
            .field("dwResolution", &self.dwResolution)
            .field("dwCMMType", &self.dwCMMType)
            .field("dwClass", &self.dwClass)
            .field("dwDataColorSpace", &self.dwDataColorSpace)
            .field("dwConnectionSpace", &self.dwConnectionSpace)
            .field("dwSignature", &self.dwSignature)
            .field("dwPlatform", &self.dwPlatform)
            .field("dwProfileFlags", &self.dwProfileFlags)
            .field("dwManufacturer", &self.dwManufacturer)
            .field("dwModel", &self.dwModel)
            .field("dwAttributes", &self.dwAttributes)
            .field("dwRenderingIntent", &self.dwRenderingIntent)
            .field("dwCreator", &self.dwCreator)
            .field("dwDeviceClass", &self.dwDeviceClass)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for ENUMTYPEA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ENUMTYPEA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENUMTYPEA>()) == 0 }
    }
}
impl ::core::cmp::Eq for ENUMTYPEA {}
impl ::core::default::Default for ENUMTYPEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ENUMTYPEW {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFields: u32,
    pub pDeviceName: ::windows_core::PCWSTR,
    pub dwMediaType: u32,
    pub dwDitheringMode: u32,
    pub dwResolution: [u32; 2],
    pub dwCMMType: u32,
    pub dwClass: u32,
    pub dwDataColorSpace: u32,
    pub dwConnectionSpace: u32,
    pub dwSignature: u32,
    pub dwPlatform: u32,
    pub dwProfileFlags: u32,
    pub dwManufacturer: u32,
    pub dwModel: u32,
    pub dwAttributes: [u32; 2],
    pub dwRenderingIntent: u32,
    pub dwCreator: u32,
    pub dwDeviceClass: u32,
}
impl ::core::marker::Copy for ENUMTYPEW {}
impl ::core::clone::Clone for ENUMTYPEW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENUMTYPEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMTYPEW")
            .field("dwSize", &self.dwSize)
            .field("dwVersion", &self.dwVersion)
            .field("dwFields", &self.dwFields)
            .field("pDeviceName", &self.pDeviceName)
            .field("dwMediaType", &self.dwMediaType)
            .field("dwDitheringMode", &self.dwDitheringMode)
            .field("dwResolution", &self.dwResolution)
            .field("dwCMMType", &self.dwCMMType)
            .field("dwClass", &self.dwClass)
            .field("dwDataColorSpace", &self.dwDataColorSpace)
            .field("dwConnectionSpace", &self.dwConnectionSpace)
            .field("dwSignature", &self.dwSignature)
            .field("dwPlatform", &self.dwPlatform)
            .field("dwProfileFlags", &self.dwProfileFlags)
            .field("dwManufacturer", &self.dwManufacturer)
            .field("dwModel", &self.dwModel)
            .field("dwAttributes", &self.dwAttributes)
            .field("dwRenderingIntent", &self.dwRenderingIntent)
            .field("dwCreator", &self.dwCreator)
            .field("dwDeviceClass", &self.dwDeviceClass)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for ENUMTYPEW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ENUMTYPEW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENUMTYPEW>()) == 0 }
    }
}
impl ::core::cmp::Eq for ENUMTYPEW {}
impl ::core::default::Default for ENUMTYPEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const ENUM_TYPE_VERSION: u32 = 768u32;
pub const ET_ATTRIBUTES: u32 = 8192u32;
pub const ET_CLASS: u32 = 32u32;
pub const ET_CMMTYPE: u32 = 16u32;
pub const ET_CONNECTIONSPACE: u32 = 128u32;
pub const ET_CREATOR: u32 = 32768u32;
pub const ET_DATACOLORSPACE: u32 = 64u32;
pub const ET_DEVICECLASS: u32 = 65536u32;
pub const ET_DEVICENAME: u32 = 1u32;
pub const ET_DITHERMODE: u32 = 4u32;
pub const ET_EXTENDEDDISPLAYCOLOR: u32 = 262144u32;
pub const ET_MANUFACTURER: u32 = 2048u32;
pub const ET_MEDIATYPE: u32 = 2u32;
pub const ET_MODEL: u32 = 4096u32;
pub const ET_PLATFORM: u32 = 512u32;
pub const ET_PROFILEFLAGS: u32 = 1024u32;
pub const ET_RENDERINGINTENT: u32 = 16384u32;
pub const ET_RESOLUTION: u32 = 8u32;
pub const ET_SIGNATURE: u32 = 256u32;
pub const ET_STANDARDDISPLAYCOLOR: u32 = 131072u32;
#[inline]
pub unsafe fn EnumColorProfilesA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(pmachinename: Param0, penumrecord: *const ENUMTYPEA, penumerationbuffer: *mut u8, pdwsizeofenumerationbuffer: *mut u32, pnprofiles: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumColorProfilesA(pmachinename: ::windows_core::PCSTR, penumrecord: *const ENUMTYPEA, penumerationbuffer: *mut u8, pdwsizeofenumerationbuffer: *mut u32, pnprofiles: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EnumColorProfilesA(pmachinename.into_param().abi(), ::core::mem::transmute(penumrecord), ::core::mem::transmute(penumerationbuffer), ::core::mem::transmute(pdwsizeofenumerationbuffer), ::core::mem::transmute(pnprofiles)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EnumColorProfilesW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pmachinename: Param0, penumrecord: *const ENUMTYPEW, penumerationbuffer: *mut u8, pdwsizeofenumerationbuffer: *mut u32, pnprofiles: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumColorProfilesW(pmachinename: ::windows_core::PCWSTR, penumrecord: *const ENUMTYPEW, penumerationbuffer: *mut u8, pdwsizeofenumerationbuffer: *mut u32, pnprofiles: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EnumColorProfilesW(pmachinename.into_param().abi(), ::core::mem::transmute(penumrecord), ::core::mem::transmute(penumerationbuffer), ::core::mem::transmute(pdwsizeofenumerationbuffer), ::core::mem::transmute(pnprofiles)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn EnumICMProfilesA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::LPARAM>>(hdc: Param0, proc: ICMENUMPROCA, param2: Param2) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumICMProfilesA(hdc: ::win32_graphics::Gdi::HDC, proc: ::windows_core::RawPtr, param2: ::win32_foundation::LPARAM) -> i32;
        }
        ::core::mem::transmute(EnumICMProfilesA(hdc.into_param().abi(), ::core::mem::transmute(proc), param2.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn EnumICMProfilesW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::LPARAM>>(hdc: Param0, proc: ICMENUMPROCW, param2: Param2) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumICMProfilesW(hdc: ::win32_graphics::Gdi::HDC, proc: ::windows_core::RawPtr, param2: ::win32_foundation::LPARAM) -> i32;
        }
        ::core::mem::transmute(EnumICMProfilesW(hdc.into_param().abi(), ::core::mem::transmute(proc), param2.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FAST_TRANSLATE: u32 = 262144u32;
pub const FLAG_DEPENDENTONDATA: u32 = 2u32;
pub const FLAG_EMBEDDEDPROFILE: u32 = 1u32;
pub const FLAG_ENABLE_CHROMATIC_ADAPTATION: u32 = 33554432u32;
#[repr(C)]
pub struct GENERIC3CHANNEL {
    pub ch1: u16,
    pub ch2: u16,
    pub ch3: u16,
}
impl ::core::marker::Copy for GENERIC3CHANNEL {}
impl ::core::clone::Clone for GENERIC3CHANNEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GENERIC3CHANNEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GENERIC3CHANNEL").field("ch1", &self.ch1).field("ch2", &self.ch2).field("ch3", &self.ch3).finish()
    }
}
unsafe impl ::windows_core::Abi for GENERIC3CHANNEL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GENERIC3CHANNEL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GENERIC3CHANNEL>()) == 0 }
    }
}
impl ::core::cmp::Eq for GENERIC3CHANNEL {}
impl ::core::default::Default for GENERIC3CHANNEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GRAYCOLOR {
    pub gray: u16,
}
impl ::core::marker::Copy for GRAYCOLOR {}
impl ::core::clone::Clone for GRAYCOLOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GRAYCOLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GRAYCOLOR").field("gray", &self.gray).finish()
    }
}
unsafe impl ::windows_core::Abi for GRAYCOLOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GRAYCOLOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GRAYCOLOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for GRAYCOLOR {}
impl ::core::default::Default for GRAYCOLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GamutBoundaryDescription {
    pub pPrimaries: *mut PrimaryJabColors,
    pub cNeutralSamples: u32,
    pub pNeutralSamples: *mut JabColorF,
    pub pReferenceShell: *mut GamutShell,
    pub pPlausibleShell: *mut GamutShell,
    pub pPossibleShell: *mut GamutShell,
}
impl ::core::marker::Copy for GamutBoundaryDescription {}
impl ::core::clone::Clone for GamutBoundaryDescription {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GamutBoundaryDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GamutBoundaryDescription").field("pPrimaries", &self.pPrimaries).field("cNeutralSamples", &self.cNeutralSamples).field("pNeutralSamples", &self.pNeutralSamples).field("pReferenceShell", &self.pReferenceShell).field("pPlausibleShell", &self.pPlausibleShell).field("pPossibleShell", &self.pPossibleShell).finish()
    }
}
unsafe impl ::windows_core::Abi for GamutBoundaryDescription {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GamutBoundaryDescription {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GamutBoundaryDescription>()) == 0 }
    }
}
impl ::core::cmp::Eq for GamutBoundaryDescription {}
impl ::core::default::Default for GamutBoundaryDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GamutShell {
    pub JMin: f32,
    pub JMax: f32,
    pub cVertices: u32,
    pub cTriangles: u32,
    pub pVertices: *mut JabColorF,
    pub pTriangles: *mut GamutShellTriangle,
}
impl ::core::marker::Copy for GamutShell {}
impl ::core::clone::Clone for GamutShell {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GamutShell {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GamutShell").field("JMin", &self.JMin).field("JMax", &self.JMax).field("cVertices", &self.cVertices).field("cTriangles", &self.cTriangles).field("pVertices", &self.pVertices).field("pTriangles", &self.pTriangles).finish()
    }
}
unsafe impl ::windows_core::Abi for GamutShell {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GamutShell {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GamutShell>()) == 0 }
    }
}
impl ::core::cmp::Eq for GamutShell {}
impl ::core::default::Default for GamutShell {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GamutShellTriangle {
    pub aVertexIndex: [u32; 3],
}
impl ::core::marker::Copy for GamutShellTriangle {}
impl ::core::clone::Clone for GamutShellTriangle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GamutShellTriangle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GamutShellTriangle").field("aVertexIndex", &self.aVertexIndex).finish()
    }
}
unsafe impl ::windows_core::Abi for GamutShellTriangle {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GamutShellTriangle {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GamutShellTriangle>()) == 0 }
    }
}
impl ::core::cmp::Eq for GamutShellTriangle {}
impl ::core::default::Default for GamutShellTriangle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[inline]
pub unsafe fn GetCMMInfo(hcolortransform: isize, param1: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCMMInfo(hcolortransform: isize, param1: u32) -> u32;
        }
        ::core::mem::transmute(GetCMMInfo(::core::mem::transmute(hcolortransform), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetColorDirectoryA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(pmachinename: Param0, pbuffer: ::windows_core::PSTR, pdwsize: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetColorDirectoryA(pmachinename: ::windows_core::PCSTR, pbuffer: ::windows_core::PSTR, pdwsize: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetColorDirectoryA(pmachinename.into_param().abi(), ::core::mem::transmute(pbuffer), ::core::mem::transmute(pdwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetColorDirectoryW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pmachinename: Param0, pbuffer: ::windows_core::PWSTR, pdwsize: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetColorDirectoryW(pmachinename: ::windows_core::PCWSTR, pbuffer: ::windows_core::PWSTR, pdwsize: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetColorDirectoryW(pmachinename.into_param().abi(), ::core::mem::transmute(pbuffer), ::core::mem::transmute(pdwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetColorProfileElement(hprofile: isize, tag: u32, dwoffset: u32, pcbelement: *mut u32, pelement: *mut ::core::ffi::c_void, pbreference: *mut ::win32_foundation::BOOL) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetColorProfileElement(hprofile: isize, tag: u32, dwoffset: u32, pcbelement: *mut u32, pelement: *mut ::core::ffi::c_void, pbreference: *mut ::win32_foundation::BOOL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetColorProfileElement(::core::mem::transmute(hprofile), ::core::mem::transmute(tag), ::core::mem::transmute(dwoffset), ::core::mem::transmute(pcbelement), ::core::mem::transmute(pelement), ::core::mem::transmute(pbreference)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetColorProfileElementTag(hprofile: isize, dwindex: u32, ptag: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetColorProfileElementTag(hprofile: isize, dwindex: u32, ptag: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetColorProfileElementTag(::core::mem::transmute(hprofile), ::core::mem::transmute(dwindex), ::core::mem::transmute(ptag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetColorProfileFromHandle(hprofile: isize, pprofile: *mut u8, pcbprofile: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetColorProfileFromHandle(hprofile: isize, pprofile: *mut u8, pcbprofile: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetColorProfileFromHandle(::core::mem::transmute(hprofile), ::core::mem::transmute(pprofile), ::core::mem::transmute(pcbprofile)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetColorProfileHeader(hprofile: isize, pheader: *mut PROFILEHEADER) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetColorProfileHeader(hprofile: isize, pheader: *mut PROFILEHEADER) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetColorProfileHeader(::core::mem::transmute(hprofile), ::core::mem::transmute(pheader)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetColorSpace<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(hdc: Param0) -> HCOLORSPACE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetColorSpace(hdc: ::win32_graphics::Gdi::HDC) -> HCOLORSPACE;
        }
        ::core::mem::transmute(GetColorSpace(hdc.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetCountColorProfileElements(hprofile: isize, pnelementcount: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCountColorProfileElements(hprofile: isize, pnelementcount: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetCountColorProfileElements(::core::mem::transmute(hprofile), ::core::mem::transmute(pnelementcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetDeviceGammaRamp<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(hdc: Param0, lpramp: *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDeviceGammaRamp(hdc: ::win32_graphics::Gdi::HDC, lpramp: *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetDeviceGammaRamp(hdc.into_param().abi(), ::core::mem::transmute(lpramp)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetICMProfileA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(hdc: Param0, pbufsize: *mut u32, pszfilename: ::windows_core::PSTR) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetICMProfileA(hdc: ::win32_graphics::Gdi::HDC, pbufsize: *mut u32, pszfilename: ::windows_core::PSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetICMProfileA(hdc.into_param().abi(), ::core::mem::transmute(pbufsize), ::core::mem::transmute(pszfilename)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetICMProfileW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(hdc: Param0, pbufsize: *mut u32, pszfilename: ::windows_core::PWSTR) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetICMProfileW(hdc: ::win32_graphics::Gdi::HDC, pbufsize: *mut u32, pszfilename: ::windows_core::PWSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetICMProfileW(hdc.into_param().abi(), ::core::mem::transmute(pbufsize), ::core::mem::transmute(pszfilename)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetLogColorSpaceA<'a, Param0: ::windows_core::IntoParam<'a, HCOLORSPACE>>(hcolorspace: Param0, lpbuffer: *mut LOGCOLORSPACEA, nsize: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLogColorSpaceA(hcolorspace: HCOLORSPACE, lpbuffer: *mut LOGCOLORSPACEA, nsize: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetLogColorSpaceA(hcolorspace.into_param().abi(), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetLogColorSpaceW<'a, Param0: ::windows_core::IntoParam<'a, HCOLORSPACE>>(hcolorspace: Param0, lpbuffer: *mut LOGCOLORSPACEW, nsize: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLogColorSpaceW(hcolorspace: HCOLORSPACE, lpbuffer: *mut LOGCOLORSPACEW, nsize: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetLogColorSpaceW(hcolorspace.into_param().abi(), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetNamedProfileInfo(hprofile: isize, pnamedprofileinfo: *mut NAMED_PROFILE_INFO) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNamedProfileInfo(hprofile: isize, pnamedprofileinfo: *mut NAMED_PROFILE_INFO) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetNamedProfileInfo(::core::mem::transmute(hprofile), ::core::mem::transmute(pnamedprofileinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetPS2ColorRenderingDictionary(hprofile: isize, dwintent: u32, pps2colorrenderingdictionary: *mut u8, pcbps2colorrenderingdictionary: *mut u32, pbbinary: *mut ::win32_foundation::BOOL) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPS2ColorRenderingDictionary(hprofile: isize, dwintent: u32, pps2colorrenderingdictionary: *mut u8, pcbps2colorrenderingdictionary: *mut u32, pbbinary: *mut ::win32_foundation::BOOL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetPS2ColorRenderingDictionary(::core::mem::transmute(hprofile), ::core::mem::transmute(dwintent), ::core::mem::transmute(pps2colorrenderingdictionary), ::core::mem::transmute(pcbps2colorrenderingdictionary), ::core::mem::transmute(pbbinary)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetPS2ColorRenderingIntent(hprofile: isize, dwintent: u32, pbuffer: *mut u8, pcbps2colorrenderingintent: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPS2ColorRenderingIntent(hprofile: isize, dwintent: u32, pbuffer: *mut u8, pcbps2colorrenderingintent: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetPS2ColorRenderingIntent(::core::mem::transmute(hprofile), ::core::mem::transmute(dwintent), ::core::mem::transmute(pbuffer), ::core::mem::transmute(pcbps2colorrenderingintent)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetPS2ColorSpaceArray(hprofile: isize, dwintent: u32, dwcsatype: u32, pps2colorspacearray: *mut u8, pcbps2colorspacearray: *mut u32, pbbinary: *mut ::win32_foundation::BOOL) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPS2ColorSpaceArray(hprofile: isize, dwintent: u32, dwcsatype: u32, pps2colorspacearray: *mut u8, pcbps2colorspacearray: *mut u32, pbbinary: *mut ::win32_foundation::BOOL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetPS2ColorSpaceArray(::core::mem::transmute(hprofile), ::core::mem::transmute(dwintent), ::core::mem::transmute(dwcsatype), ::core::mem::transmute(pps2colorspacearray), ::core::mem::transmute(pcbps2colorspacearray), ::core::mem::transmute(pbbinary)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetStandardColorSpaceProfileA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(pmachinename: Param0, dwscs: u32, pbuffer: ::windows_core::PSTR, pcbsize: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetStandardColorSpaceProfileA(pmachinename: ::windows_core::PCSTR, dwscs: u32, pbuffer: ::windows_core::PSTR, pcbsize: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetStandardColorSpaceProfileA(pmachinename.into_param().abi(), ::core::mem::transmute(dwscs), ::core::mem::transmute(pbuffer), ::core::mem::transmute(pcbsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetStandardColorSpaceProfileW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pmachinename: Param0, dwscs: u32, pbuffer: ::windows_core::PWSTR, pcbsize: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetStandardColorSpaceProfileW(pmachinename: ::windows_core::PCWSTR, dwscs: u32, pbuffer: ::windows_core::PWSTR, pcbsize: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetStandardColorSpaceProfileW(pmachinename.into_param().abi(), ::core::mem::transmute(dwscs), ::core::mem::transmute(pbuffer), ::core::mem::transmute(pcbsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HCOLORSPACE(pub isize);
impl HCOLORSPACE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HCOLORSPACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HCOLORSPACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HCOLORSPACE {}
impl ::core::fmt::Debug for HCOLORSPACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCOLORSPACE").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Abi for HCOLORSPACE {
    type Abi = Self;
}
#[repr(C)]
pub struct HiFiCOLOR {
    pub channel: [u8; 8],
}
impl ::core::marker::Copy for HiFiCOLOR {}
impl ::core::clone::Clone for HiFiCOLOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HiFiCOLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HiFiCOLOR").field("channel", &self.channel).finish()
    }
}
unsafe impl ::windows_core::Abi for HiFiCOLOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HiFiCOLOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HiFiCOLOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for HiFiCOLOR {}
impl ::core::default::Default for HiFiCOLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type ICMENUMPROCA = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_core::PCSTR, param1: ::win32_foundation::LPARAM) -> i32>;
pub type ICMENUMPROCW = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_core::PCWSTR, param1: ::win32_foundation::LPARAM) -> i32>;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ICM_COMMAND(pub u32);
pub const ICM_ADDPROFILE: ICM_COMMAND = ICM_COMMAND(1u32);
pub const ICM_DELETEPROFILE: ICM_COMMAND = ICM_COMMAND(2u32);
pub const ICM_QUERYPROFILE: ICM_COMMAND = ICM_COMMAND(3u32);
pub const ICM_SETDEFAULTPROFILE: ICM_COMMAND = ICM_COMMAND(4u32);
pub const ICM_REGISTERICMATCHER: ICM_COMMAND = ICM_COMMAND(5u32);
pub const ICM_UNREGISTERICMATCHER: ICM_COMMAND = ICM_COMMAND(6u32);
pub const ICM_QUERYMATCH: ICM_COMMAND = ICM_COMMAND(7u32);
impl ::core::marker::Copy for ICM_COMMAND {}
impl ::core::clone::Clone for ICM_COMMAND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ICM_COMMAND {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ICM_COMMAND {
    type Abi = Self;
}
impl ::core::fmt::Debug for ICM_COMMAND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICM_COMMAND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ICM_MODE(pub i32);
pub const ICM_OFF: ICM_MODE = ICM_MODE(1i32);
pub const ICM_ON: ICM_MODE = ICM_MODE(2i32);
pub const ICM_QUERY: ICM_MODE = ICM_MODE(3i32);
pub const ICM_DONE_OUTSIDEDC: ICM_MODE = ICM_MODE(4i32);
impl ::core::marker::Copy for ICM_MODE {}
impl ::core::clone::Clone for ICM_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ICM_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ICM_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ICM_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICM_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
pub struct IDeviceModelPlugIn(::windows_core::IUnknown);
impl IDeviceModelPlugIn {
    pub unsafe fn Initialize<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrxml: Param0, cnummodels: u32, imodelposition: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), bstrxml.into_param().abi(), ::core::mem::transmute(cnummodels), ::core::mem::transmute(imodelposition)).ok()
    }
    pub unsafe fn GetNumChannels(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetNumChannels)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn DeviceToColorimetricColors(&self, cchannels: u32, pdevicevalues: *const f32, pxyzcolors: &mut [XYZColorF]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeviceToColorimetricColors)(::windows_core::Interface::as_raw(self), pxyzcolors.len() as _, ::core::mem::transmute(cchannels), ::core::mem::transmute(pdevicevalues), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pxyzcolors))).ok()
    }
    pub unsafe fn ColorimetricToDeviceColors(&self, cchannels: u32, pxyzcolors: &[XYZColorF]) -> ::windows_core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
        (::windows_core::Interface::vtable(self).ColorimetricToDeviceColors)(::windows_core::Interface::as_raw(self), pxyzcolors.len() as _, ::core::mem::transmute(cchannels), ::core::mem::transmute(::windows_core::as_ptr_or_null(pxyzcolors)), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn ColorimetricToDeviceColorsWithBlack(&self, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF, pblackinformation: *const BlackInformation) -> ::windows_core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
        (::windows_core::Interface::vtable(self).ColorimetricToDeviceColorsWithBlack)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ccolors), ::core::mem::transmute(cchannels), ::core::mem::transmute(pxyzcolors), ::core::mem::transmute(pblackinformation), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetTransformDeviceModelInfo<'a, Param1: ::windows_core::IntoParam<'a, IDeviceModelPlugIn>>(&self, imodelposition: u32, pidevicemodelother: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTransformDeviceModelInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(imodelposition), pidevicemodelother.into_param().abi()).ok()
    }
    pub unsafe fn GetPrimarySamples(&self) -> ::windows_core::Result<PrimaryXYZColors> {
        let mut result__ = ::core::mem::MaybeUninit::<PrimaryXYZColors>::zeroed();
        (::windows_core::Interface::vtable(self).GetPrimarySamples)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PrimaryXYZColors>(result__)
    }
    pub unsafe fn GetGamutBoundaryMeshSize(&self, pnumvertices: *mut u32, pnumtriangles: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetGamutBoundaryMeshSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pnumvertices), ::core::mem::transmute(pnumtriangles)).ok()
    }
    pub unsafe fn GetGamutBoundaryMesh(&self, cchannels: u32, cvertices: u32, pvertices: *mut f32, ptriangles: &mut [GamutShellTriangle]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetGamutBoundaryMesh)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cchannels), ::core::mem::transmute(cvertices), ptriangles.len() as _, ::core::mem::transmute(pvertices), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(ptriangles))).ok()
    }
    pub unsafe fn GetNeutralAxisSize(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetNeutralAxisSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetNeutralAxis(&self, pxyzcolors: &mut [XYZColorF]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetNeutralAxis)(::windows_core::Interface::as_raw(self), pxyzcolors.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pxyzcolors))).ok()
    }
}
impl ::core::convert::From<IDeviceModelPlugIn> for ::windows_core::IUnknown {
    fn from(value: IDeviceModelPlugIn) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDeviceModelPlugIn> for ::windows_core::IUnknown {
    fn from(value: &IDeviceModelPlugIn) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDeviceModelPlugIn {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDeviceModelPlugIn {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDeviceModelPlugIn {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDeviceModelPlugIn {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDeviceModelPlugIn {}
impl ::core::fmt::Debug for IDeviceModelPlugIn {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDeviceModelPlugIn").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDeviceModelPlugIn {
    type Vtable = IDeviceModelPlugIn_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1cd63475_07c4_46fe_a903_d655316d11fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceModelPlugIn_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrxml: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, cnummodels: u32, imodelposition: u32) -> ::windows_core::HRESULT,
    pub GetNumChannels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnumchannels: *mut u32) -> ::windows_core::HRESULT,
    pub DeviceToColorimetricColors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccolors: u32, cchannels: u32, pdevicevalues: *const f32, pxyzcolors: *mut XYZColorF) -> ::windows_core::HRESULT,
    pub ColorimetricToDeviceColors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF, pdevicevalues: *mut f32) -> ::windows_core::HRESULT,
    pub ColorimetricToDeviceColorsWithBlack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF, pblackinformation: *const BlackInformation, pdevicevalues: *mut f32) -> ::windows_core::HRESULT,
    pub SetTransformDeviceModelInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imodelposition: u32, pidevicemodelother: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetPrimarySamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprimarycolor: *mut PrimaryXYZColors) -> ::windows_core::HRESULT,
    pub GetGamutBoundaryMeshSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnumvertices: *mut u32, pnumtriangles: *mut u32) -> ::windows_core::HRESULT,
    pub GetGamutBoundaryMesh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchannels: u32, cvertices: u32, ctriangles: u32, pvertices: *mut f32, ptriangles: *mut GamutShellTriangle) -> ::windows_core::HRESULT,
    pub GetNeutralAxisSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pccolors: *mut u32) -> ::windows_core::HRESULT,
    pub GetNeutralAxis: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccolors: u32, pxyzcolors: *mut XYZColorF) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IGamutMapModelPlugIn(::windows_core::IUnknown);
impl IGamutMapModelPlugIn {
    pub unsafe fn Initialize<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, IDeviceModelPlugIn>, Param2: ::windows_core::IntoParam<'a, IDeviceModelPlugIn>>(&self, bstrxml: Param0, psrcplugin: Param1, pdestplugin: Param2, psrcgbd: *const GamutBoundaryDescription, pdestgbd: *const GamutBoundaryDescription) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), bstrxml.into_param().abi(), psrcplugin.into_param().abi(), pdestplugin.into_param().abi(), ::core::mem::transmute(psrcgbd), ::core::mem::transmute(pdestgbd)).ok()
    }
    pub unsafe fn SourceToDestinationAppearanceColors(&self, ccolors: u32, pinputcolors: *const JChColorF, poutputcolors: *mut JChColorF) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SourceToDestinationAppearanceColors)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ccolors), ::core::mem::transmute(pinputcolors), ::core::mem::transmute(poutputcolors)).ok()
    }
}
impl ::core::convert::From<IGamutMapModelPlugIn> for ::windows_core::IUnknown {
    fn from(value: IGamutMapModelPlugIn) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGamutMapModelPlugIn> for ::windows_core::IUnknown {
    fn from(value: &IGamutMapModelPlugIn) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGamutMapModelPlugIn {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGamutMapModelPlugIn {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGamutMapModelPlugIn {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGamutMapModelPlugIn {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGamutMapModelPlugIn {}
impl ::core::fmt::Debug for IGamutMapModelPlugIn {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGamutMapModelPlugIn").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IGamutMapModelPlugIn {
    type Vtable = IGamutMapModelPlugIn_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2dd80115_ad1e_41f6_a219_a4f4b583d1f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGamutMapModelPlugIn_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrxml: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, psrcplugin: ::windows_core::RawPtr, pdestplugin: ::windows_core::RawPtr, psrcgbd: *const GamutBoundaryDescription, pdestgbd: *const GamutBoundaryDescription) -> ::windows_core::HRESULT,
    pub SourceToDestinationAppearanceColors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccolors: u32, pinputcolors: *const JChColorF, poutputcolors: *mut JChColorF) -> ::windows_core::HRESULT,
}
pub const INDEX_DONT_CARE: u32 = 0u32;
pub const INTENT_ABSOLUTE_COLORIMETRIC: u32 = 3u32;
pub const INTENT_PERCEPTUAL: u32 = 0u32;
pub const INTENT_RELATIVE_COLORIMETRIC: u32 = 1u32;
pub const INTENT_SATURATION: u32 = 2u32;
#[inline]
pub unsafe fn InstallColorProfileA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(pmachinename: Param0, pprofilename: Param1) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InstallColorProfileA(pmachinename: ::windows_core::PCSTR, pprofilename: ::windows_core::PCSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(InstallColorProfileA(pmachinename.into_param().abi(), pprofilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn InstallColorProfileW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pmachinename: Param0, pprofilename: Param1) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InstallColorProfileW(pmachinename: ::windows_core::PCWSTR, pprofilename: ::windows_core::PCWSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(InstallColorProfileW(pmachinename.into_param().abi(), pprofilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn IsColorProfileTagPresent(hprofile: isize, tag: u32, pbpresent: *mut ::win32_foundation::BOOL) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsColorProfileTagPresent(hprofile: isize, tag: u32, pbpresent: *mut ::win32_foundation::BOOL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(IsColorProfileTagPresent(::core::mem::transmute(hprofile), ::core::mem::transmute(tag), ::core::mem::transmute(pbpresent)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn IsColorProfileValid(hprofile: isize, pbvalid: *mut ::win32_foundation::BOOL) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsColorProfileValid(hprofile: isize, pbvalid: *mut ::win32_foundation::BOOL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(IsColorProfileValid(::core::mem::transmute(hprofile), ::core::mem::transmute(pbvalid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct JChColorF {
    pub J: f32,
    pub C: f32,
    pub h: f32,
}
impl ::core::marker::Copy for JChColorF {}
impl ::core::clone::Clone for JChColorF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JChColorF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JChColorF").field("J", &self.J).field("C", &self.C).field("h", &self.h).finish()
    }
}
unsafe impl ::windows_core::Abi for JChColorF {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JChColorF {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JChColorF>()) == 0 }
    }
}
impl ::core::cmp::Eq for JChColorF {}
impl ::core::default::Default for JChColorF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct JabColorF {
    pub J: f32,
    pub a: f32,
    pub b: f32,
}
impl ::core::marker::Copy for JabColorF {}
impl ::core::clone::Clone for JabColorF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JabColorF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JabColorF").field("J", &self.J).field("a", &self.a).field("b", &self.b).finish()
    }
}
unsafe impl ::windows_core::Abi for JabColorF {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JabColorF {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JabColorF>()) == 0 }
    }
}
impl ::core::cmp::Eq for JabColorF {}
impl ::core::default::Default for JabColorF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct LOGCOLORSPACEA {
    pub lcsSignature: u32,
    pub lcsVersion: u32,
    pub lcsSize: u32,
    pub lcsCSType: i32,
    pub lcsIntent: i32,
    pub lcsEndpoints: ::win32_graphics::Gdi::CIEXYZTRIPLE,
    pub lcsGammaRed: u32,
    pub lcsGammaGreen: u32,
    pub lcsGammaBlue: u32,
    pub lcsFilename: [::win32_foundation::CHAR; 260],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for LOGCOLORSPACEA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for LOGCOLORSPACEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for LOGCOLORSPACEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOGCOLORSPACEA").field("lcsSignature", &self.lcsSignature).field("lcsVersion", &self.lcsVersion).field("lcsSize", &self.lcsSize).field("lcsCSType", &self.lcsCSType).field("lcsIntent", &self.lcsIntent).field("lcsEndpoints", &self.lcsEndpoints).field("lcsGammaRed", &self.lcsGammaRed).field("lcsGammaGreen", &self.lcsGammaGreen).field("lcsGammaBlue", &self.lcsGammaBlue).field("lcsFilename", &self.lcsFilename).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for LOGCOLORSPACEA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for LOGCOLORSPACEA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LOGCOLORSPACEA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for LOGCOLORSPACEA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for LOGCOLORSPACEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct LOGCOLORSPACEW {
    pub lcsSignature: u32,
    pub lcsVersion: u32,
    pub lcsSize: u32,
    pub lcsCSType: i32,
    pub lcsIntent: i32,
    pub lcsEndpoints: ::win32_graphics::Gdi::CIEXYZTRIPLE,
    pub lcsGammaRed: u32,
    pub lcsGammaGreen: u32,
    pub lcsGammaBlue: u32,
    pub lcsFilename: [u16; 260],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for LOGCOLORSPACEW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for LOGCOLORSPACEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for LOGCOLORSPACEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOGCOLORSPACEW").field("lcsSignature", &self.lcsSignature).field("lcsVersion", &self.lcsVersion).field("lcsSize", &self.lcsSize).field("lcsCSType", &self.lcsCSType).field("lcsIntent", &self.lcsIntent).field("lcsEndpoints", &self.lcsEndpoints).field("lcsGammaRed", &self.lcsGammaRed).field("lcsGammaGreen", &self.lcsGammaGreen).field("lcsGammaBlue", &self.lcsGammaBlue).field("lcsFilename", &self.lcsFilename).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for LOGCOLORSPACEW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for LOGCOLORSPACEW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LOGCOLORSPACEW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for LOGCOLORSPACEW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for LOGCOLORSPACEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type LPBMCALLBACKFN = ::core::option::Option<unsafe extern "system" fn(param0: u32, param1: u32, param2: ::win32_foundation::LPARAM) -> ::win32_foundation::BOOL>;
#[repr(C)]
pub struct LabCOLOR {
    pub L: u16,
    pub a: u16,
    pub b: u16,
}
impl ::core::marker::Copy for LabCOLOR {}
impl ::core::clone::Clone for LabCOLOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LabCOLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LabCOLOR").field("L", &self.L).field("a", &self.a).field("b", &self.b).finish()
    }
}
unsafe impl ::windows_core::Abi for LabCOLOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LabCOLOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LabCOLOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for LabCOLOR {}
impl ::core::default::Default for LabCOLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const MAX_COLOR_CHANNELS: u32 = 8u32;
#[repr(C)]
pub struct NAMEDCOLOR {
    pub dwIndex: u32,
}
impl ::core::marker::Copy for NAMEDCOLOR {}
impl ::core::clone::Clone for NAMEDCOLOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NAMEDCOLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NAMEDCOLOR").field("dwIndex", &self.dwIndex).finish()
    }
}
unsafe impl ::windows_core::Abi for NAMEDCOLOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NAMEDCOLOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NAMEDCOLOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for NAMEDCOLOR {}
impl ::core::default::Default for NAMEDCOLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NAMED_PROFILE_INFO {
    pub dwFlags: u32,
    pub dwCount: u32,
    pub dwCountDevCoordinates: u32,
    pub szPrefix: [i8; 32],
    pub szSuffix: [i8; 32],
}
impl ::core::marker::Copy for NAMED_PROFILE_INFO {}
impl ::core::clone::Clone for NAMED_PROFILE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NAMED_PROFILE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NAMED_PROFILE_INFO").field("dwFlags", &self.dwFlags).field("dwCount", &self.dwCount).field("dwCountDevCoordinates", &self.dwCountDevCoordinates).field("szPrefix", &self.szPrefix).field("szSuffix", &self.szSuffix).finish()
    }
}
unsafe impl ::windows_core::Abi for NAMED_PROFILE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NAMED_PROFILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NAMED_PROFILE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for NAMED_PROFILE_INFO {}
impl ::core::default::Default for NAMED_PROFILE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const NORMAL_MODE: u32 = 2u32;
#[inline]
pub unsafe fn OpenColorProfileA(pprofile: *const PROFILE, dwdesiredaccess: u32, dwsharemode: u32, dwcreationmode: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenColorProfileA(pprofile: *const PROFILE, dwdesiredaccess: u32, dwsharemode: u32, dwcreationmode: u32) -> isize;
        }
        ::core::mem::transmute(OpenColorProfileA(::core::mem::transmute(pprofile), ::core::mem::transmute(dwdesiredaccess), ::core::mem::transmute(dwsharemode), ::core::mem::transmute(dwcreationmode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn OpenColorProfileW(pprofile: *const PROFILE, dwdesiredaccess: u32, dwsharemode: u32, dwcreationmode: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenColorProfileW(pprofile: *const PROFILE, dwdesiredaccess: u32, dwsharemode: u32, dwcreationmode: u32) -> isize;
        }
        ::core::mem::transmute(OpenColorProfileW(::core::mem::transmute(pprofile), ::core::mem::transmute(dwdesiredaccess), ::core::mem::transmute(dwsharemode), ::core::mem::transmute(dwcreationmode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub type PCMSCALLBACKA = ::core::option::Option<unsafe extern "system" fn(param0: *mut COLORMATCHSETUPA, param1: ::win32_foundation::LPARAM) -> ::win32_foundation::BOOL>;
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub type PCMSCALLBACKW = ::core::option::Option<unsafe extern "system" fn(param0: *mut COLORMATCHSETUPW, param1: ::win32_foundation::LPARAM) -> ::win32_foundation::BOOL>;
pub const PRESERVEBLACK: u32 = 1048576u32;
#[repr(C)]
pub struct PROFILE {
    pub dwType: u32,
    pub pProfileData: *mut ::core::ffi::c_void,
    pub cbDataSize: u32,
}
impl ::core::marker::Copy for PROFILE {}
impl ::core::clone::Clone for PROFILE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROFILE").field("dwType", &self.dwType).field("pProfileData", &self.pProfileData).field("cbDataSize", &self.cbDataSize).finish()
    }
}
unsafe impl ::windows_core::Abi for PROFILE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PROFILE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROFILE>()) == 0 }
    }
}
impl ::core::cmp::Eq for PROFILE {}
impl ::core::default::Default for PROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct PROFILEHEADER {
    pub phSize: u32,
    pub phCMMType: u32,
    pub phVersion: u32,
    pub phClass: u32,
    pub phDataColorSpace: u32,
    pub phConnectionSpace: u32,
    pub phDateTime: [u32; 3],
    pub phSignature: u32,
    pub phPlatform: u32,
    pub phProfileFlags: u32,
    pub phManufacturer: u32,
    pub phModel: u32,
    pub phAttributes: [u32; 2],
    pub phRenderingIntent: u32,
    pub phIlluminant: ::win32_graphics::Gdi::CIEXYZ,
    pub phCreator: u32,
    pub phReserved: [u8; 44],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for PROFILEHEADER {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for PROFILEHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for PROFILEHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROFILEHEADER")
            .field("phSize", &self.phSize)
            .field("phCMMType", &self.phCMMType)
            .field("phVersion", &self.phVersion)
            .field("phClass", &self.phClass)
            .field("phDataColorSpace", &self.phDataColorSpace)
            .field("phConnectionSpace", &self.phConnectionSpace)
            .field("phDateTime", &self.phDateTime)
            .field("phSignature", &self.phSignature)
            .field("phPlatform", &self.phPlatform)
            .field("phProfileFlags", &self.phProfileFlags)
            .field("phManufacturer", &self.phManufacturer)
            .field("phModel", &self.phModel)
            .field("phAttributes", &self.phAttributes)
            .field("phRenderingIntent", &self.phRenderingIntent)
            .field("phIlluminant", &self.phIlluminant)
            .field("phCreator", &self.phCreator)
            .field("phReserved", &self.phReserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for PROFILEHEADER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for PROFILEHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROFILEHEADER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for PROFILEHEADER {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for PROFILEHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const PROFILE_FILENAME: u32 = 1u32;
pub const PROFILE_MEMBUFFER: u32 = 2u32;
pub const PROFILE_READ: u32 = 1u32;
pub const PROFILE_READWRITE: u32 = 2u32;
pub const PROOF_MODE: u32 = 1u32;
#[repr(C)]
pub struct PrimaryJabColors {
    pub red: JabColorF,
    pub yellow: JabColorF,
    pub green: JabColorF,
    pub cyan: JabColorF,
    pub blue: JabColorF,
    pub magenta: JabColorF,
    pub black: JabColorF,
    pub white: JabColorF,
}
impl ::core::marker::Copy for PrimaryJabColors {}
impl ::core::clone::Clone for PrimaryJabColors {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PrimaryJabColors {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PrimaryJabColors").field("red", &self.red).field("yellow", &self.yellow).field("green", &self.green).field("cyan", &self.cyan).field("blue", &self.blue).field("magenta", &self.magenta).field("black", &self.black).field("white", &self.white).finish()
    }
}
unsafe impl ::windows_core::Abi for PrimaryJabColors {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PrimaryJabColors {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PrimaryJabColors>()) == 0 }
    }
}
impl ::core::cmp::Eq for PrimaryJabColors {}
impl ::core::default::Default for PrimaryJabColors {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct PrimaryXYZColors {
    pub red: XYZColorF,
    pub yellow: XYZColorF,
    pub green: XYZColorF,
    pub cyan: XYZColorF,
    pub blue: XYZColorF,
    pub magenta: XYZColorF,
    pub black: XYZColorF,
    pub white: XYZColorF,
}
impl ::core::marker::Copy for PrimaryXYZColors {}
impl ::core::clone::Clone for PrimaryXYZColors {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PrimaryXYZColors {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PrimaryXYZColors").field("red", &self.red).field("yellow", &self.yellow).field("green", &self.green).field("cyan", &self.cyan).field("blue", &self.blue).field("magenta", &self.magenta).field("black", &self.black).field("white", &self.white).finish()
    }
}
unsafe impl ::windows_core::Abi for PrimaryXYZColors {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PrimaryXYZColors {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PrimaryXYZColors>()) == 0 }
    }
}
impl ::core::cmp::Eq for PrimaryXYZColors {}
impl ::core::default::Default for PrimaryXYZColors {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const RESERVED: u32 = 2147483648u32;
#[repr(C)]
pub struct RGBCOLOR {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}
impl ::core::marker::Copy for RGBCOLOR {}
impl ::core::clone::Clone for RGBCOLOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RGBCOLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RGBCOLOR").field("red", &self.red).field("green", &self.green).field("blue", &self.blue).finish()
    }
}
unsafe impl ::windows_core::Abi for RGBCOLOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RGBCOLOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RGBCOLOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for RGBCOLOR {}
impl ::core::default::Default for RGBCOLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[inline]
pub unsafe fn RegisterCMMA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(pmachinename: Param0, cmmid: u32, pcmmdll: Param2) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterCMMA(pmachinename: ::windows_core::PCSTR, cmmid: u32, pcmmdll: ::windows_core::PCSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(RegisterCMMA(pmachinename.into_param().abi(), ::core::mem::transmute(cmmid), pcmmdll.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RegisterCMMW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pmachinename: Param0, cmmid: u32, pcmmdll: Param2) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterCMMW(pmachinename: ::windows_core::PCWSTR, cmmid: u32, pcmmdll: ::windows_core::PCWSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(RegisterCMMW(pmachinename.into_param().abi(), ::core::mem::transmute(cmmid), pcmmdll.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const SEQUENTIAL_TRANSFORM: u32 = 2155872256u32;
#[inline]
pub unsafe fn SelectCMM(dwcmmtype: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SelectCMM(dwcmmtype: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SelectCMM(::core::mem::transmute(dwcmmtype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetColorProfileElement(hprofile: isize, tag: u32, dwoffset: u32, pelement: &[u8]) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetColorProfileElement(hprofile: isize, tag: u32, dwoffset: u32, pcbelement: *const u32, pelement: *const ::core::ffi::c_void) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetColorProfileElement(::core::mem::transmute(hprofile), ::core::mem::transmute(tag), ::core::mem::transmute(dwoffset), pelement.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pelement))))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetColorProfileElementReference(hprofile: isize, newtag: u32, reftag: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetColorProfileElementReference(hprofile: isize, newtag: u32, reftag: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetColorProfileElementReference(::core::mem::transmute(hprofile), ::core::mem::transmute(newtag), ::core::mem::transmute(reftag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetColorProfileElementSize(hprofile: isize, tagtype: u32, pcbelement: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetColorProfileElementSize(hprofile: isize, tagtype: u32, pcbelement: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetColorProfileElementSize(::core::mem::transmute(hprofile), ::core::mem::transmute(tagtype), ::core::mem::transmute(pcbelement)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn SetColorProfileHeader(hprofile: isize, pheader: *const PROFILEHEADER) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetColorProfileHeader(hprofile: isize, pheader: *const PROFILEHEADER) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetColorProfileHeader(::core::mem::transmute(hprofile), ::core::mem::transmute(pheader)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn SetColorSpace<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>, Param1: ::windows_core::IntoParam<'a, HCOLORSPACE>>(hdc: Param0, hcs: Param1) -> HCOLORSPACE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetColorSpace(hdc: ::win32_graphics::Gdi::HDC, hcs: HCOLORSPACE) -> HCOLORSPACE;
        }
        ::core::mem::transmute(SetColorSpace(hdc.into_param().abi(), hcs.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn SetDeviceGammaRamp<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(hdc: Param0, lpramp: *const ::core::ffi::c_void) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetDeviceGammaRamp(hdc: ::win32_graphics::Gdi::HDC, lpramp: *const ::core::ffi::c_void) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetDeviceGammaRamp(hdc.into_param().abi(), ::core::mem::transmute(lpramp)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn SetICMMode<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(hdc: Param0, mode: ICM_MODE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetICMMode(hdc: ::win32_graphics::Gdi::HDC, mode: ICM_MODE) -> i32;
        }
        ::core::mem::transmute(SetICMMode(hdc.into_param().abi(), ::core::mem::transmute(mode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn SetICMProfileA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(hdc: Param0, lpfilename: Param1) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetICMProfileA(hdc: ::win32_graphics::Gdi::HDC, lpfilename: ::windows_core::PCSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetICMProfileA(hdc.into_param().abi(), lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn SetICMProfileW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hdc: Param0, lpfilename: Param1) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetICMProfileW(hdc: ::win32_graphics::Gdi::HDC, lpfilename: ::windows_core::PCWSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetICMProfileW(hdc.into_param().abi(), lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetStandardColorSpaceProfileA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(pmachinename: Param0, dwprofileid: u32, pprofilename: Param2) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetStandardColorSpaceProfileA(pmachinename: ::windows_core::PCSTR, dwprofileid: u32, pprofilename: ::windows_core::PCSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetStandardColorSpaceProfileA(pmachinename.into_param().abi(), ::core::mem::transmute(dwprofileid), pprofilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetStandardColorSpaceProfileW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pmachinename: Param0, dwprofileid: u32, pprofilename: Param2) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetStandardColorSpaceProfileW(pmachinename: ::windows_core::PCWSTR, dwprofileid: u32, pprofilename: ::windows_core::PCWSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetStandardColorSpaceProfileW(pmachinename.into_param().abi(), ::core::mem::transmute(dwprofileid), pprofilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn SetupColorMatchingA(pcms: *mut COLORMATCHSETUPA) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupColorMatchingA(pcms: *mut COLORMATCHSETUPA) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetupColorMatchingA(::core::mem::transmute(pcms)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn SetupColorMatchingW(pcms: *mut COLORMATCHSETUPW) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupColorMatchingW(pcms: *mut COLORMATCHSETUPW) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetupColorMatchingW(::core::mem::transmute(pcms)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TranslateBitmapBits<'a, Param10: ::windows_core::IntoParam<'a, ::win32_foundation::LPARAM>>(hcolortransform: isize, psrcbits: *const ::core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwinputstride: u32, pdestbits: *mut ::core::ffi::c_void, bmoutput: BMFORMAT, dwoutputstride: u32, pfncallback: LPBMCALLBACKFN, ulcallbackdata: Param10) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TranslateBitmapBits(hcolortransform: isize, psrcbits: *const ::core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwinputstride: u32, pdestbits: *mut ::core::ffi::c_void, bmoutput: BMFORMAT, dwoutputstride: u32, pfncallback: ::windows_core::RawPtr, ulcallbackdata: ::win32_foundation::LPARAM) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(TranslateBitmapBits(::core::mem::transmute(hcolortransform), ::core::mem::transmute(psrcbits), ::core::mem::transmute(bminput), ::core::mem::transmute(dwwidth), ::core::mem::transmute(dwheight), ::core::mem::transmute(dwinputstride), ::core::mem::transmute(pdestbits), ::core::mem::transmute(bmoutput), ::core::mem::transmute(dwoutputstride), ::core::mem::transmute(pfncallback), ulcallbackdata.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TranslateColors(hcolortransform: isize, painputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, paoutputcolors: *mut COLOR, ctoutput: COLORTYPE) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TranslateColors(hcolortransform: isize, painputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, paoutputcolors: *mut COLOR, ctoutput: COLORTYPE) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(TranslateColors(::core::mem::transmute(hcolortransform), ::core::mem::transmute(painputcolors), ::core::mem::transmute(ncolors), ::core::mem::transmute(ctinput), ::core::mem::transmute(paoutputcolors), ::core::mem::transmute(ctoutput)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const USE_RELATIVE_COLORIMETRIC: u32 = 131072u32;
#[inline]
pub unsafe fn UninstallColorProfileA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(pmachinename: Param0, pprofilename: Param1, bdelete: Param2) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UninstallColorProfileA(pmachinename: ::windows_core::PCSTR, pprofilename: ::windows_core::PCSTR, bdelete: ::win32_foundation::BOOL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(UninstallColorProfileA(pmachinename.into_param().abi(), pprofilename.into_param().abi(), bdelete.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn UninstallColorProfileW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(pmachinename: Param0, pprofilename: Param1, bdelete: Param2) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UninstallColorProfileW(pmachinename: ::windows_core::PCWSTR, pprofilename: ::windows_core::PCWSTR, bdelete: ::win32_foundation::BOOL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(UninstallColorProfileW(pmachinename.into_param().abi(), pprofilename.into_param().abi(), bdelete.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn UnregisterCMMA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(pmachinename: Param0, cmmid: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterCMMA(pmachinename: ::windows_core::PCSTR, cmmid: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(UnregisterCMMA(pmachinename.into_param().abi(), ::core::mem::transmute(cmmid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn UnregisterCMMW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pmachinename: Param0, cmmid: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterCMMW(pmachinename: ::windows_core::PCWSTR, cmmid: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(UnregisterCMMW(pmachinename.into_param().abi(), ::core::mem::transmute(cmmid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn UpdateICMRegKeyA<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(reserved: u32, lpszcmid: Param1, lpszfilename: Param2, command: ICM_COMMAND) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UpdateICMRegKeyA(reserved: u32, lpszcmid: ::windows_core::PCSTR, lpszfilename: ::windows_core::PCSTR, command: ICM_COMMAND) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(UpdateICMRegKeyA(::core::mem::transmute(reserved), lpszcmid.into_param().abi(), lpszfilename.into_param().abi(), ::core::mem::transmute(command)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn UpdateICMRegKeyW<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(reserved: u32, lpszcmid: Param1, lpszfilename: Param2, command: ICM_COMMAND) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UpdateICMRegKeyW(reserved: u32, lpszcmid: ::windows_core::PCWSTR, lpszfilename: ::windows_core::PCWSTR, command: ICM_COMMAND) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(UpdateICMRegKeyW(::core::mem::transmute(reserved), lpszcmid.into_param().abi(), lpszfilename.into_param().abi(), ::core::mem::transmute(command)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const WCS_ALWAYS: u32 = 2097152u32;
pub const WCS_DEFAULT: i32 = 0i32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WCS_DEVICE_CAPABILITIES_TYPE(pub i32);
pub const VideoCardGammaTable: WCS_DEVICE_CAPABILITIES_TYPE = WCS_DEVICE_CAPABILITIES_TYPE(1i32);
pub const MicrosoftHardwareColorV2: WCS_DEVICE_CAPABILITIES_TYPE = WCS_DEVICE_CAPABILITIES_TYPE(2i32);
impl ::core::marker::Copy for WCS_DEVICE_CAPABILITIES_TYPE {}
impl ::core::clone::Clone for WCS_DEVICE_CAPABILITIES_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WCS_DEVICE_CAPABILITIES_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WCS_DEVICE_CAPABILITIES_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WCS_DEVICE_CAPABILITIES_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCS_DEVICE_CAPABILITIES_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct WCS_DEVICE_MHC2_CAPABILITIES {
    pub Size: u32,
    pub SupportsMhc2: ::win32_foundation::BOOL,
    pub RegammaLutEntryCount: u32,
    pub CscXyzMatrixRows: u32,
    pub CscXyzMatrixColumns: u32,
}
impl ::core::marker::Copy for WCS_DEVICE_MHC2_CAPABILITIES {}
impl ::core::clone::Clone for WCS_DEVICE_MHC2_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WCS_DEVICE_MHC2_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCS_DEVICE_MHC2_CAPABILITIES").field("Size", &self.Size).field("SupportsMhc2", &self.SupportsMhc2).field("RegammaLutEntryCount", &self.RegammaLutEntryCount).field("CscXyzMatrixRows", &self.CscXyzMatrixRows).field("CscXyzMatrixColumns", &self.CscXyzMatrixColumns).finish()
    }
}
unsafe impl ::windows_core::Abi for WCS_DEVICE_MHC2_CAPABILITIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WCS_DEVICE_MHC2_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WCS_DEVICE_MHC2_CAPABILITIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WCS_DEVICE_MHC2_CAPABILITIES {}
impl ::core::default::Default for WCS_DEVICE_MHC2_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WCS_DEVICE_VCGT_CAPABILITIES {
    pub Size: u32,
    pub SupportsVcgt: ::win32_foundation::BOOL,
}
impl ::core::marker::Copy for WCS_DEVICE_VCGT_CAPABILITIES {}
impl ::core::clone::Clone for WCS_DEVICE_VCGT_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WCS_DEVICE_VCGT_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCS_DEVICE_VCGT_CAPABILITIES").field("Size", &self.Size).field("SupportsVcgt", &self.SupportsVcgt).finish()
    }
}
unsafe impl ::windows_core::Abi for WCS_DEVICE_VCGT_CAPABILITIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WCS_DEVICE_VCGT_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WCS_DEVICE_VCGT_CAPABILITIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WCS_DEVICE_VCGT_CAPABILITIES {}
impl ::core::default::Default for WCS_DEVICE_VCGT_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WCS_ICCONLY: i32 = 65536i32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WCS_PROFILE_MANAGEMENT_SCOPE(pub i32);
pub const WCS_PROFILE_MANAGEMENT_SCOPE_SYSTEM_WIDE: WCS_PROFILE_MANAGEMENT_SCOPE = WCS_PROFILE_MANAGEMENT_SCOPE(0i32);
pub const WCS_PROFILE_MANAGEMENT_SCOPE_CURRENT_USER: WCS_PROFILE_MANAGEMENT_SCOPE = WCS_PROFILE_MANAGEMENT_SCOPE(1i32);
impl ::core::marker::Copy for WCS_PROFILE_MANAGEMENT_SCOPE {}
impl ::core::clone::Clone for WCS_PROFILE_MANAGEMENT_SCOPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WCS_PROFILE_MANAGEMENT_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WCS_PROFILE_MANAGEMENT_SCOPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WCS_PROFILE_MANAGEMENT_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCS_PROFILE_MANAGEMENT_SCOPE").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn WcsAssociateColorProfileWithDevice<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pprofilename: Param1, pdevicename: Param2) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsAssociateColorProfileWithDevice(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pprofilename: ::windows_core::PCWSTR, pdevicename: ::windows_core::PCWSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(WcsAssociateColorProfileWithDevice(::core::mem::transmute(scope), pprofilename.into_param().abi(), pdevicename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WcsCheckColors(hcolortransform: isize, ninputchannels: u32, cdtinput: COLORDATATYPE, cbinput: u32, pinputdata: *const ::core::ffi::c_void, paresult: &mut [u8]) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsCheckColors(hcolortransform: isize, ncolors: u32, ninputchannels: u32, cdtinput: COLORDATATYPE, cbinput: u32, pinputdata: *const ::core::ffi::c_void, paresult: *mut u8) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(WcsCheckColors(::core::mem::transmute(hcolortransform), paresult.len() as _, ::core::mem::transmute(ninputchannels), ::core::mem::transmute(cdtinput), ::core::mem::transmute(cbinput), ::core::mem::transmute(pinputdata), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(paresult))))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WcsCreateIccProfile(hwcsprofile: isize, dwoptions: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsCreateIccProfile(hwcsprofile: isize, dwoptions: u32) -> isize;
        }
        ::core::mem::transmute(WcsCreateIccProfile(::core::mem::transmute(hwcsprofile), ::core::mem::transmute(dwoptions)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WcsDisassociateColorProfileFromDevice<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pprofilename: Param1, pdevicename: Param2) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsDisassociateColorProfileFromDevice(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pprofilename: ::windows_core::PCWSTR, pdevicename: ::windows_core::PCWSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(WcsDisassociateColorProfileFromDevice(::core::mem::transmute(scope), pprofilename.into_param().abi(), pdevicename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WcsEnumColorProfiles(scope: WCS_PROFILE_MANAGEMENT_SCOPE, penumrecord: *const ENUMTYPEW, pbuffer: *mut u8, dwsize: u32, pnprofiles: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsEnumColorProfiles(scope: WCS_PROFILE_MANAGEMENT_SCOPE, penumrecord: *const ENUMTYPEW, pbuffer: *mut u8, dwsize: u32, pnprofiles: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(WcsEnumColorProfiles(::core::mem::transmute(scope), ::core::mem::transmute(penumrecord), ::core::mem::transmute(pbuffer), ::core::mem::transmute(dwsize), ::core::mem::transmute(pnprofiles)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WcsEnumColorProfilesSize(scope: WCS_PROFILE_MANAGEMENT_SCOPE, penumrecord: *const ENUMTYPEW, pdwsize: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsEnumColorProfilesSize(scope: WCS_PROFILE_MANAGEMENT_SCOPE, penumrecord: *const ENUMTYPEW, pdwsize: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(WcsEnumColorProfilesSize(::core::mem::transmute(scope), ::core::mem::transmute(penumrecord), ::core::mem::transmute(pdwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WcsGetCalibrationManagementState(pbisenabled: *mut ::win32_foundation::BOOL) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsGetCalibrationManagementState(pbisenabled: *mut ::win32_foundation::BOOL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(WcsGetCalibrationManagementState(::core::mem::transmute(pbisenabled)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WcsGetDefaultColorProfile<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename: Param1, cptcolorprofiletype: COLORPROFILETYPE, cpstcolorprofilesubtype: COLORPROFILESUBTYPE, dwprofileid: u32, cbprofilename: u32, pprofilename: ::windows_core::PWSTR) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsGetDefaultColorProfile(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename: ::windows_core::PCWSTR, cptcolorprofiletype: COLORPROFILETYPE, cpstcolorprofilesubtype: COLORPROFILESUBTYPE, dwprofileid: u32, cbprofilename: u32, pprofilename: ::windows_core::PWSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(WcsGetDefaultColorProfile(::core::mem::transmute(scope), pdevicename.into_param().abi(), ::core::mem::transmute(cptcolorprofiletype), ::core::mem::transmute(cpstcolorprofilesubtype), ::core::mem::transmute(dwprofileid), ::core::mem::transmute(cbprofilename), ::core::mem::transmute(pprofilename)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WcsGetDefaultColorProfileSize<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename: Param1, cptcolorprofiletype: COLORPROFILETYPE, cpstcolorprofilesubtype: COLORPROFILESUBTYPE, dwprofileid: u32, pcbprofilename: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsGetDefaultColorProfileSize(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename: ::windows_core::PCWSTR, cptcolorprofiletype: COLORPROFILETYPE, cpstcolorprofilesubtype: COLORPROFILESUBTYPE, dwprofileid: u32, pcbprofilename: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(WcsGetDefaultColorProfileSize(::core::mem::transmute(scope), pdevicename.into_param().abi(), ::core::mem::transmute(cptcolorprofiletype), ::core::mem::transmute(cpstcolorprofilesubtype), ::core::mem::transmute(dwprofileid), ::core::mem::transmute(pcbprofilename)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WcsGetDefaultRenderingIntent(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdwrenderingintent: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsGetDefaultRenderingIntent(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdwrenderingintent: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(WcsGetDefaultRenderingIntent(::core::mem::transmute(scope), ::core::mem::transmute(pdwrenderingintent)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WcsGetUsePerUserProfiles<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pdevicename: Param0, dwdeviceclass: u32, puseperuserprofiles: *mut ::win32_foundation::BOOL) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsGetUsePerUserProfiles(pdevicename: ::windows_core::PCWSTR, dwdeviceclass: u32, puseperuserprofiles: *mut ::win32_foundation::BOOL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(WcsGetUsePerUserProfiles(pdevicename.into_param().abi(), ::core::mem::transmute(dwdeviceclass), ::core::mem::transmute(puseperuserprofiles)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WcsOpenColorProfileA(pcdmpprofile: *const PROFILE, pcampprofile: *const PROFILE, pgmmpprofile: *const PROFILE, dwdesireaccess: u32, dwsharemode: u32, dwcreationmode: u32, dwflags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsOpenColorProfileA(pcdmpprofile: *const PROFILE, pcampprofile: *const PROFILE, pgmmpprofile: *const PROFILE, dwdesireaccess: u32, dwsharemode: u32, dwcreationmode: u32, dwflags: u32) -> isize;
        }
        ::core::mem::transmute(WcsOpenColorProfileA(::core::mem::transmute(pcdmpprofile), ::core::mem::transmute(pcampprofile), ::core::mem::transmute(pgmmpprofile), ::core::mem::transmute(dwdesireaccess), ::core::mem::transmute(dwsharemode), ::core::mem::transmute(dwcreationmode), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WcsOpenColorProfileW(pcdmpprofile: *const PROFILE, pcampprofile: *const PROFILE, pgmmpprofile: *const PROFILE, dwdesireaccess: u32, dwsharemode: u32, dwcreationmode: u32, dwflags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsOpenColorProfileW(pcdmpprofile: *const PROFILE, pcampprofile: *const PROFILE, pgmmpprofile: *const PROFILE, dwdesireaccess: u32, dwsharemode: u32, dwcreationmode: u32, dwflags: u32) -> isize;
        }
        ::core::mem::transmute(WcsOpenColorProfileW(::core::mem::transmute(pcdmpprofile), ::core::mem::transmute(pcampprofile), ::core::mem::transmute(pgmmpprofile), ::core::mem::transmute(dwdesireaccess), ::core::mem::transmute(dwsharemode), ::core::mem::transmute(dwcreationmode), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WcsSetCalibrationManagementState<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(bisenabled: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsSetCalibrationManagementState(bisenabled: ::win32_foundation::BOOL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(WcsSetCalibrationManagementState(bisenabled.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WcsSetDefaultColorProfile<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param5: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename: Param1, cptcolorprofiletype: COLORPROFILETYPE, cpstcolorprofilesubtype: COLORPROFILESUBTYPE, dwprofileid: u32, pprofilename: Param5) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsSetDefaultColorProfile(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename: ::windows_core::PCWSTR, cptcolorprofiletype: COLORPROFILETYPE, cpstcolorprofilesubtype: COLORPROFILESUBTYPE, dwprofileid: u32, pprofilename: ::windows_core::PCWSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(WcsSetDefaultColorProfile(::core::mem::transmute(scope), pdevicename.into_param().abi(), ::core::mem::transmute(cptcolorprofiletype), ::core::mem::transmute(cpstcolorprofilesubtype), ::core::mem::transmute(dwprofileid), pprofilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WcsSetDefaultRenderingIntent(scope: WCS_PROFILE_MANAGEMENT_SCOPE, dwrenderingintent: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsSetDefaultRenderingIntent(scope: WCS_PROFILE_MANAGEMENT_SCOPE, dwrenderingintent: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(WcsSetDefaultRenderingIntent(::core::mem::transmute(scope), ::core::mem::transmute(dwrenderingintent)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WcsSetUsePerUserProfiles<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(pdevicename: Param0, dwdeviceclass: u32, useperuserprofiles: Param2) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsSetUsePerUserProfiles(pdevicename: ::windows_core::PCWSTR, dwdeviceclass: u32, useperuserprofiles: ::win32_foundation::BOOL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(WcsSetUsePerUserProfiles(pdevicename.into_param().abi(), ::core::mem::transmute(dwdeviceclass), useperuserprofiles.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WcsTranslateColors(hcolortransform: isize, ncolors: u32, ninputchannels: u32, cdtinput: COLORDATATYPE, cbinput: u32, pinputdata: *const ::core::ffi::c_void, noutputchannels: u32, cdtoutput: COLORDATATYPE, cboutput: u32, poutputdata: *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsTranslateColors(hcolortransform: isize, ncolors: u32, ninputchannels: u32, cdtinput: COLORDATATYPE, cbinput: u32, pinputdata: *const ::core::ffi::c_void, noutputchannels: u32, cdtoutput: COLORDATATYPE, cboutput: u32, poutputdata: *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(WcsTranslateColors(::core::mem::transmute(hcolortransform), ::core::mem::transmute(ncolors), ::core::mem::transmute(ninputchannels), ::core::mem::transmute(cdtinput), ::core::mem::transmute(cbinput), ::core::mem::transmute(pinputdata), ::core::mem::transmute(noutputchannels), ::core::mem::transmute(cdtoutput), ::core::mem::transmute(cboutput), ::core::mem::transmute(poutputdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct XYZCOLOR {
    pub X: u16,
    pub Y: u16,
    pub Z: u16,
}
impl ::core::marker::Copy for XYZCOLOR {}
impl ::core::clone::Clone for XYZCOLOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XYZCOLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XYZCOLOR").field("X", &self.X).field("Y", &self.Y).field("Z", &self.Z).finish()
    }
}
unsafe impl ::windows_core::Abi for XYZCOLOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for XYZCOLOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<XYZCOLOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for XYZCOLOR {}
impl ::core::default::Default for XYZCOLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct XYZColorF {
    pub X: f32,
    pub Y: f32,
    pub Z: f32,
}
impl ::core::marker::Copy for XYZColorF {}
impl ::core::clone::Clone for XYZColorF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XYZColorF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XYZColorF").field("X", &self.X).field("Y", &self.Y).field("Z", &self.Z).finish()
    }
}
unsafe impl ::windows_core::Abi for XYZColorF {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for XYZColorF {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<XYZColorF>()) == 0 }
    }
}
impl ::core::cmp::Eq for XYZColorF {}
impl ::core::default::Default for XYZColorF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct YxyCOLOR {
    pub Y: u16,
    pub x: u16,
    pub y: u16,
}
impl ::core::marker::Copy for YxyCOLOR {}
impl ::core::clone::Clone for YxyCOLOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for YxyCOLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("YxyCOLOR").field("Y", &self.Y).field("x", &self.x).field("y", &self.y).finish()
    }
}
unsafe impl ::windows_core::Abi for YxyCOLOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for YxyCOLOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<YxyCOLOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for YxyCOLOR {}
impl ::core::default::Default for YxyCOLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
