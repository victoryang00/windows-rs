#[link(name = "windows")]
extern "system" {
    pub fn AssociateColorProfileWithDeviceA(pmachinename: ::windows_core_sys::PCSTR, pprofilename: ::windows_core_sys::PCSTR, pdevicename: ::windows_core_sys::PCSTR) -> ::win32_foundation_sys::BOOL;
    pub fn AssociateColorProfileWithDeviceW(pmachinename: ::windows_core_sys::PCWSTR, pprofilename: ::windows_core_sys::PCWSTR, pdevicename: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::BOOL;
    pub fn CMCheckColors(hcmtransform: isize, lpainputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, lparesult: *mut u8) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn CMCheckColorsInGamut(hcmtransform: isize, lpargbtriple: *const ::win32_graphics_sys::Gdi::RGBTRIPLE, lparesult: *mut u8, ncount: u32) -> ::win32_foundation_sys::BOOL;
    pub fn CMCheckRGBs(hcmtransform: isize, lpsrcbits: *const ::core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwstride: u32, lparesult: *mut u8, pfncallback: LPBMCALLBACKFN, ulcallbackdata: ::win32_foundation_sys::LPARAM) -> ::win32_foundation_sys::BOOL;
    pub fn CMConvertColorNameToIndex(hprofile: isize, pacolorname: *const *const i8, paindex: *mut u32, dwcount: u32) -> ::win32_foundation_sys::BOOL;
    pub fn CMConvertIndexToColorName(hprofile: isize, paindex: *const u32, pacolorname: *mut *mut i8, dwcount: u32) -> ::win32_foundation_sys::BOOL;
    pub fn CMCreateDeviceLinkProfile(pahprofiles: *const isize, nprofiles: u32, padwintents: *const u32, nintents: u32, dwflags: u32, lpprofiledata: *mut *mut u8) -> ::win32_foundation_sys::BOOL;
    pub fn CMCreateMultiProfileTransform(pahprofiles: *const isize, nprofiles: u32, padwintents: *const u32, nintents: u32, dwflags: u32) -> isize;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn CMCreateProfile(lpcolorspace: *mut LOGCOLORSPACEA, lpprofiledata: *mut *mut ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn CMCreateProfileW(lpcolorspace: *mut LOGCOLORSPACEW, lpprofiledata: *mut *mut ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn CMCreateTransform(lpcolorspace: *const LOGCOLORSPACEA, lpdevcharacter: *const ::core::ffi::c_void, lptargetdevcharacter: *const ::core::ffi::c_void) -> isize;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn CMCreateTransformExt(lpcolorspace: *const LOGCOLORSPACEA, lpdevcharacter: *const ::core::ffi::c_void, lptargetdevcharacter: *const ::core::ffi::c_void, dwflags: u32) -> isize;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn CMCreateTransformExtW(lpcolorspace: *const LOGCOLORSPACEW, lpdevcharacter: *const ::core::ffi::c_void, lptargetdevcharacter: *const ::core::ffi::c_void, dwflags: u32) -> isize;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn CMCreateTransformW(lpcolorspace: *const LOGCOLORSPACEW, lpdevcharacter: *const ::core::ffi::c_void, lptargetdevcharacter: *const ::core::ffi::c_void) -> isize;
    pub fn CMDeleteTransform(hcmtransform: isize) -> ::win32_foundation_sys::BOOL;
    pub fn CMGetInfo(dwinfo: u32) -> u32;
    pub fn CMGetNamedProfileInfo(hprofile: isize, pnamedprofileinfo: *mut NAMED_PROFILE_INFO) -> ::win32_foundation_sys::BOOL;
    pub fn CMIsProfileValid(hprofile: isize, lpbvalid: *mut i32) -> ::win32_foundation_sys::BOOL;
    pub fn CMTranslateColors(hcmtransform: isize, lpainputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, lpaoutputcolors: *mut COLOR, ctoutput: COLORTYPE) -> ::win32_foundation_sys::BOOL;
    pub fn CMTranslateRGB(hcmtransform: isize, colorref: u32, lpcolorref: *mut u32, dwflags: u32) -> ::win32_foundation_sys::BOOL;
    pub fn CMTranslateRGBs(hcmtransform: isize, lpsrcbits: *const ::core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwstride: u32, lpdestbits: *mut ::core::ffi::c_void, bmoutput: BMFORMAT, dwtranslatedirection: u32) -> ::win32_foundation_sys::BOOL;
    pub fn CMTranslateRGBsExt(hcmtransform: isize, lpsrcbits: *const ::core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwinputstride: u32, lpdestbits: *mut ::core::ffi::c_void, bmoutput: BMFORMAT, dwoutputstride: u32, lpfncallback: LPBMCALLBACKFN, ulcallbackdata: ::win32_foundation_sys::LPARAM) -> ::win32_foundation_sys::BOOL;
    pub fn CheckBitmapBits(hcolortransform: isize, psrcbits: *const ::core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwstride: u32, paresult: *mut u8, pfncallback: LPBMCALLBACKFN, lpcallbackdata: ::win32_foundation_sys::LPARAM) -> ::win32_foundation_sys::BOOL;
    pub fn CheckColors(hcolortransform: isize, painputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, paresult: *mut u8) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn CheckColorsInGamut(hdc: ::win32_graphics_sys::Gdi::HDC, lprgbtriple: *const ::win32_graphics_sys::Gdi::RGBTRIPLE, dlpbuffer: *mut ::core::ffi::c_void, ncount: u32) -> ::win32_foundation_sys::BOOL;
    pub fn CloseColorProfile(hprofile: isize) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ColorCorrectPalette(hdc: ::win32_graphics_sys::Gdi::HDC, hpal: ::win32_graphics_sys::Gdi::HPALETTE, defirst: u32, num: u32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ColorMatchToTarget(hdc: ::win32_graphics_sys::Gdi::HDC, hdctarget: ::win32_graphics_sys::Gdi::HDC, action: COLOR_MATCH_TO_TARGET_ACTION) -> ::win32_foundation_sys::BOOL;
    pub fn ColorProfileAddDisplayAssociation(scope: WCS_PROFILE_MANAGEMENT_SCOPE, profilename: ::windows_core_sys::PCWSTR, targetadapterid: ::win32_foundation_sys::LUID, sourceid: u32, setasdefault: ::win32_foundation_sys::BOOL, associateasadvancedcolor: ::win32_foundation_sys::BOOL) -> ::windows_core_sys::HRESULT;
    pub fn ColorProfileGetDisplayDefault(scope: WCS_PROFILE_MANAGEMENT_SCOPE, targetadapterid: ::win32_foundation_sys::LUID, sourceid: u32, profiletype: COLORPROFILETYPE, profilesubtype: COLORPROFILESUBTYPE, profilename: *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn ColorProfileGetDisplayList(scope: WCS_PROFILE_MANAGEMENT_SCOPE, targetadapterid: ::win32_foundation_sys::LUID, sourceid: u32, profilelist: *mut *mut ::windows_core_sys::PWSTR, profilecount: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn ColorProfileGetDisplayUserScope(targetadapterid: ::win32_foundation_sys::LUID, sourceid: u32, scope: *mut WCS_PROFILE_MANAGEMENT_SCOPE) -> ::windows_core_sys::HRESULT;
    pub fn ColorProfileRemoveDisplayAssociation(scope: WCS_PROFILE_MANAGEMENT_SCOPE, profilename: ::windows_core_sys::PCWSTR, targetadapterid: ::win32_foundation_sys::LUID, sourceid: u32, dissociateadvancedcolor: ::win32_foundation_sys::BOOL) -> ::windows_core_sys::HRESULT;
    pub fn ColorProfileSetDisplayDefaultAssociation(scope: WCS_PROFILE_MANAGEMENT_SCOPE, profilename: ::windows_core_sys::PCWSTR, profiletype: COLORPROFILETYPE, profilesubtype: COLORPROFILESUBTYPE, targetadapterid: ::win32_foundation_sys::LUID, sourceid: u32) -> ::windows_core_sys::HRESULT;
    pub fn ConvertColorNameToIndex(hprofile: isize, pacolorname: *const *const i8, paindex: *mut u32, dwcount: u32) -> ::win32_foundation_sys::BOOL;
    pub fn ConvertIndexToColorName(hprofile: isize, paindex: *const u32, pacolorname: *mut *mut i8, dwcount: u32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn CreateColorSpaceA(lplcs: *const LOGCOLORSPACEA) -> HCOLORSPACE;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn CreateColorSpaceW(lplcs: *const LOGCOLORSPACEW) -> HCOLORSPACE;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn CreateColorTransformA(plogcolorspace: *const LOGCOLORSPACEA, hdestprofile: isize, htargetprofile: isize, dwflags: u32) -> isize;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn CreateColorTransformW(plogcolorspace: *const LOGCOLORSPACEW, hdestprofile: isize, htargetprofile: isize, dwflags: u32) -> isize;
    pub fn CreateDeviceLinkProfile(hprofile: *const isize, nprofiles: u32, padwintent: *const u32, nintents: u32, dwflags: u32, pprofiledata: *mut *mut u8, indexpreferredcmm: u32) -> ::win32_foundation_sys::BOOL;
    pub fn CreateMultiProfileTransform(pahprofiles: *const isize, nprofiles: u32, padwintent: *const u32, nintents: u32, dwflags: u32, indexpreferredcmm: u32) -> isize;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn CreateProfileFromLogColorSpaceA(plogcolorspace: *const LOGCOLORSPACEA, pprofile: *mut *mut u8) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn CreateProfileFromLogColorSpaceW(plogcolorspace: *const LOGCOLORSPACEW, pprofile: *mut *mut u8) -> ::win32_foundation_sys::BOOL;
    pub fn DeleteColorSpace(hcs: HCOLORSPACE) -> ::win32_foundation_sys::BOOL;
    pub fn DeleteColorTransform(hxform: isize) -> ::win32_foundation_sys::BOOL;
    pub fn DisassociateColorProfileFromDeviceA(pmachinename: ::windows_core_sys::PCSTR, pprofilename: ::windows_core_sys::PCSTR, pdevicename: ::windows_core_sys::PCSTR) -> ::win32_foundation_sys::BOOL;
    pub fn DisassociateColorProfileFromDeviceW(pmachinename: ::windows_core_sys::PCWSTR, pprofilename: ::windows_core_sys::PCWSTR, pdevicename: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::BOOL;
    pub fn EnumColorProfilesA(pmachinename: ::windows_core_sys::PCSTR, penumrecord: *const ENUMTYPEA, penumerationbuffer: *mut u8, pdwsizeofenumerationbuffer: *mut u32, pnprofiles: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn EnumColorProfilesW(pmachinename: ::windows_core_sys::PCWSTR, penumrecord: *const ENUMTYPEW, penumerationbuffer: *mut u8, pdwsizeofenumerationbuffer: *mut u32, pnprofiles: *mut u32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn EnumICMProfilesA(hdc: ::win32_graphics_sys::Gdi::HDC, proc: ICMENUMPROCA, param2: ::win32_foundation_sys::LPARAM) -> i32;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn EnumICMProfilesW(hdc: ::win32_graphics_sys::Gdi::HDC, proc: ICMENUMPROCW, param2: ::win32_foundation_sys::LPARAM) -> i32;
    pub fn GetCMMInfo(hcolortransform: isize, param1: u32) -> u32;
    pub fn GetColorDirectoryA(pmachinename: ::windows_core_sys::PCSTR, pbuffer: ::windows_core_sys::PSTR, pdwsize: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn GetColorDirectoryW(pmachinename: ::windows_core_sys::PCWSTR, pbuffer: ::windows_core_sys::PWSTR, pdwsize: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn GetColorProfileElement(hprofile: isize, tag: u32, dwoffset: u32, pcbelement: *mut u32, pelement: *mut ::core::ffi::c_void, pbreference: *mut ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    pub fn GetColorProfileElementTag(hprofile: isize, dwindex: u32, ptag: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn GetColorProfileFromHandle(hprofile: isize, pprofile: *mut u8, pcbprofile: *mut u32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetColorProfileHeader(hprofile: isize, pheader: *mut PROFILEHEADER) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetColorSpace(hdc: ::win32_graphics_sys::Gdi::HDC) -> HCOLORSPACE;
    pub fn GetCountColorProfileElements(hprofile: isize, pnelementcount: *mut u32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetDeviceGammaRamp(hdc: ::win32_graphics_sys::Gdi::HDC, lpramp: *mut ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetICMProfileA(hdc: ::win32_graphics_sys::Gdi::HDC, pbufsize: *mut u32, pszfilename: ::windows_core_sys::PSTR) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetICMProfileW(hdc: ::win32_graphics_sys::Gdi::HDC, pbufsize: *mut u32, pszfilename: ::windows_core_sys::PWSTR) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetLogColorSpaceA(hcolorspace: HCOLORSPACE, lpbuffer: *mut LOGCOLORSPACEA, nsize: u32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetLogColorSpaceW(hcolorspace: HCOLORSPACE, lpbuffer: *mut LOGCOLORSPACEW, nsize: u32) -> ::win32_foundation_sys::BOOL;
    pub fn GetNamedProfileInfo(hprofile: isize, pnamedprofileinfo: *mut NAMED_PROFILE_INFO) -> ::win32_foundation_sys::BOOL;
    pub fn GetPS2ColorRenderingDictionary(hprofile: isize, dwintent: u32, pps2colorrenderingdictionary: *mut u8, pcbps2colorrenderingdictionary: *mut u32, pbbinary: *mut ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    pub fn GetPS2ColorRenderingIntent(hprofile: isize, dwintent: u32, pbuffer: *mut u8, pcbps2colorrenderingintent: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn GetPS2ColorSpaceArray(hprofile: isize, dwintent: u32, dwcsatype: u32, pps2colorspacearray: *mut u8, pcbps2colorspacearray: *mut u32, pbbinary: *mut ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    pub fn GetStandardColorSpaceProfileA(pmachinename: ::windows_core_sys::PCSTR, dwscs: u32, pbuffer: ::windows_core_sys::PSTR, pcbsize: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn GetStandardColorSpaceProfileW(pmachinename: ::windows_core_sys::PCWSTR, dwscs: u32, pbuffer: ::windows_core_sys::PWSTR, pcbsize: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn InstallColorProfileA(pmachinename: ::windows_core_sys::PCSTR, pprofilename: ::windows_core_sys::PCSTR) -> ::win32_foundation_sys::BOOL;
    pub fn InstallColorProfileW(pmachinename: ::windows_core_sys::PCWSTR, pprofilename: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::BOOL;
    pub fn IsColorProfileTagPresent(hprofile: isize, tag: u32, pbpresent: *mut ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    pub fn IsColorProfileValid(hprofile: isize, pbvalid: *mut ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    pub fn OpenColorProfileA(pprofile: *const PROFILE, dwdesiredaccess: u32, dwsharemode: u32, dwcreationmode: u32) -> isize;
    pub fn OpenColorProfileW(pprofile: *const PROFILE, dwdesiredaccess: u32, dwsharemode: u32, dwcreationmode: u32) -> isize;
    pub fn RegisterCMMA(pmachinename: ::windows_core_sys::PCSTR, cmmid: u32, pcmmdll: ::windows_core_sys::PCSTR) -> ::win32_foundation_sys::BOOL;
    pub fn RegisterCMMW(pmachinename: ::windows_core_sys::PCWSTR, cmmid: u32, pcmmdll: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::BOOL;
    pub fn SelectCMM(dwcmmtype: u32) -> ::win32_foundation_sys::BOOL;
    pub fn SetColorProfileElement(hprofile: isize, tag: u32, dwoffset: u32, pcbelement: *const u32, pelement: *const ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    pub fn SetColorProfileElementReference(hprofile: isize, newtag: u32, reftag: u32) -> ::win32_foundation_sys::BOOL;
    pub fn SetColorProfileElementSize(hprofile: isize, tagtype: u32, pcbelement: u32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn SetColorProfileHeader(hprofile: isize, pheader: *const PROFILEHEADER) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn SetColorSpace(hdc: ::win32_graphics_sys::Gdi::HDC, hcs: HCOLORSPACE) -> HCOLORSPACE;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn SetDeviceGammaRamp(hdc: ::win32_graphics_sys::Gdi::HDC, lpramp: *const ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn SetICMMode(hdc: ::win32_graphics_sys::Gdi::HDC, mode: ICM_MODE) -> i32;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn SetICMProfileA(hdc: ::win32_graphics_sys::Gdi::HDC, lpfilename: ::windows_core_sys::PCSTR) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn SetICMProfileW(hdc: ::win32_graphics_sys::Gdi::HDC, lpfilename: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::BOOL;
    pub fn SetStandardColorSpaceProfileA(pmachinename: ::windows_core_sys::PCSTR, dwprofileid: u32, pprofilename: ::windows_core_sys::PCSTR) -> ::win32_foundation_sys::BOOL;
    pub fn SetStandardColorSpaceProfileW(pmachinename: ::windows_core_sys::PCWSTR, dwprofileid: u32, pprofilename: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn SetupColorMatchingA(pcms: *mut COLORMATCHSETUPA) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn SetupColorMatchingW(pcms: *mut COLORMATCHSETUPW) -> ::win32_foundation_sys::BOOL;
    pub fn TranslateBitmapBits(hcolortransform: isize, psrcbits: *const ::core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwinputstride: u32, pdestbits: *mut ::core::ffi::c_void, bmoutput: BMFORMAT, dwoutputstride: u32, pfncallback: LPBMCALLBACKFN, ulcallbackdata: ::win32_foundation_sys::LPARAM) -> ::win32_foundation_sys::BOOL;
    pub fn TranslateColors(hcolortransform: isize, painputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, paoutputcolors: *mut COLOR, ctoutput: COLORTYPE) -> ::win32_foundation_sys::BOOL;
    pub fn UninstallColorProfileA(pmachinename: ::windows_core_sys::PCSTR, pprofilename: ::windows_core_sys::PCSTR, bdelete: ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    pub fn UninstallColorProfileW(pmachinename: ::windows_core_sys::PCWSTR, pprofilename: ::windows_core_sys::PCWSTR, bdelete: ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    pub fn UnregisterCMMA(pmachinename: ::windows_core_sys::PCSTR, cmmid: u32) -> ::win32_foundation_sys::BOOL;
    pub fn UnregisterCMMW(pmachinename: ::windows_core_sys::PCWSTR, cmmid: u32) -> ::win32_foundation_sys::BOOL;
    pub fn UpdateICMRegKeyA(reserved: u32, lpszcmid: ::windows_core_sys::PCSTR, lpszfilename: ::windows_core_sys::PCSTR, command: ICM_COMMAND) -> ::win32_foundation_sys::BOOL;
    pub fn UpdateICMRegKeyW(reserved: u32, lpszcmid: ::windows_core_sys::PCWSTR, lpszfilename: ::windows_core_sys::PCWSTR, command: ICM_COMMAND) -> ::win32_foundation_sys::BOOL;
    pub fn WcsAssociateColorProfileWithDevice(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pprofilename: ::windows_core_sys::PCWSTR, pdevicename: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::BOOL;
    pub fn WcsCheckColors(hcolortransform: isize, ncolors: u32, ninputchannels: u32, cdtinput: COLORDATATYPE, cbinput: u32, pinputdata: *const ::core::ffi::c_void, paresult: *mut u8) -> ::win32_foundation_sys::BOOL;
    pub fn WcsCreateIccProfile(hwcsprofile: isize, dwoptions: u32) -> isize;
    pub fn WcsDisassociateColorProfileFromDevice(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pprofilename: ::windows_core_sys::PCWSTR, pdevicename: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::BOOL;
    pub fn WcsEnumColorProfiles(scope: WCS_PROFILE_MANAGEMENT_SCOPE, penumrecord: *const ENUMTYPEW, pbuffer: *mut u8, dwsize: u32, pnprofiles: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn WcsEnumColorProfilesSize(scope: WCS_PROFILE_MANAGEMENT_SCOPE, penumrecord: *const ENUMTYPEW, pdwsize: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn WcsGetCalibrationManagementState(pbisenabled: *mut ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    pub fn WcsGetDefaultColorProfile(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename: ::windows_core_sys::PCWSTR, cptcolorprofiletype: COLORPROFILETYPE, cpstcolorprofilesubtype: COLORPROFILESUBTYPE, dwprofileid: u32, cbprofilename: u32, pprofilename: ::windows_core_sys::PWSTR) -> ::win32_foundation_sys::BOOL;
    pub fn WcsGetDefaultColorProfileSize(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename: ::windows_core_sys::PCWSTR, cptcolorprofiletype: COLORPROFILETYPE, cpstcolorprofilesubtype: COLORPROFILESUBTYPE, dwprofileid: u32, pcbprofilename: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn WcsGetDefaultRenderingIntent(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdwrenderingintent: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn WcsGetUsePerUserProfiles(pdevicename: ::windows_core_sys::PCWSTR, dwdeviceclass: u32, puseperuserprofiles: *mut ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    pub fn WcsOpenColorProfileA(pcdmpprofile: *const PROFILE, pcampprofile: *const PROFILE, pgmmpprofile: *const PROFILE, dwdesireaccess: u32, dwsharemode: u32, dwcreationmode: u32, dwflags: u32) -> isize;
    pub fn WcsOpenColorProfileW(pcdmpprofile: *const PROFILE, pcampprofile: *const PROFILE, pgmmpprofile: *const PROFILE, dwdesireaccess: u32, dwsharemode: u32, dwcreationmode: u32, dwflags: u32) -> isize;
    pub fn WcsSetCalibrationManagementState(bisenabled: ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    pub fn WcsSetDefaultColorProfile(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename: ::windows_core_sys::PCWSTR, cptcolorprofiletype: COLORPROFILETYPE, cpstcolorprofilesubtype: COLORPROFILESUBTYPE, dwprofileid: u32, pprofilename: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::BOOL;
    pub fn WcsSetDefaultRenderingIntent(scope: WCS_PROFILE_MANAGEMENT_SCOPE, dwrenderingintent: u32) -> ::win32_foundation_sys::BOOL;
    pub fn WcsSetUsePerUserProfiles(pdevicename: ::windows_core_sys::PCWSTR, dwdeviceclass: u32, useperuserprofiles: ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    pub fn WcsTranslateColors(hcolortransform: isize, ncolors: u32, ninputchannels: u32, cdtinput: COLORDATATYPE, cbinput: u32, pinputdata: *const ::core::ffi::c_void, noutputchannels: u32, cdtoutput: COLORDATATYPE, cboutput: u32, poutputdata: *mut ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
}
pub const ATTRIB_MATTE: u32 = 2u32;
pub const ATTRIB_TRANSPARENCY: u32 = 1u32;
pub const BEST_MODE: u32 = 3u32;
pub type BMFORMAT = i32;
pub const BM_x555RGB: BMFORMAT = 0i32;
pub const BM_x555XYZ: BMFORMAT = 257i32;
pub const BM_x555Yxy: BMFORMAT = 258i32;
pub const BM_x555Lab: BMFORMAT = 259i32;
pub const BM_x555G3CH: BMFORMAT = 260i32;
pub const BM_RGBTRIPLETS: BMFORMAT = 2i32;
pub const BM_BGRTRIPLETS: BMFORMAT = 4i32;
pub const BM_XYZTRIPLETS: BMFORMAT = 513i32;
pub const BM_YxyTRIPLETS: BMFORMAT = 514i32;
pub const BM_LabTRIPLETS: BMFORMAT = 515i32;
pub const BM_G3CHTRIPLETS: BMFORMAT = 516i32;
pub const BM_5CHANNEL: BMFORMAT = 517i32;
pub const BM_6CHANNEL: BMFORMAT = 518i32;
pub const BM_7CHANNEL: BMFORMAT = 519i32;
pub const BM_8CHANNEL: BMFORMAT = 520i32;
pub const BM_GRAY: BMFORMAT = 521i32;
pub const BM_xRGBQUADS: BMFORMAT = 8i32;
pub const BM_xBGRQUADS: BMFORMAT = 16i32;
pub const BM_xG3CHQUADS: BMFORMAT = 772i32;
pub const BM_KYMCQUADS: BMFORMAT = 773i32;
pub const BM_CMYKQUADS: BMFORMAT = 32i32;
pub const BM_10b_RGB: BMFORMAT = 9i32;
pub const BM_10b_XYZ: BMFORMAT = 1025i32;
pub const BM_10b_Yxy: BMFORMAT = 1026i32;
pub const BM_10b_Lab: BMFORMAT = 1027i32;
pub const BM_10b_G3CH: BMFORMAT = 1028i32;
pub const BM_NAMED_INDEX: BMFORMAT = 1029i32;
pub const BM_16b_RGB: BMFORMAT = 10i32;
pub const BM_16b_XYZ: BMFORMAT = 1281i32;
pub const BM_16b_Yxy: BMFORMAT = 1282i32;
pub const BM_16b_Lab: BMFORMAT = 1283i32;
pub const BM_16b_G3CH: BMFORMAT = 1284i32;
pub const BM_16b_GRAY: BMFORMAT = 1285i32;
pub const BM_565RGB: BMFORMAT = 1i32;
pub const BM_32b_scRGB: BMFORMAT = 1537i32;
pub const BM_32b_scARGB: BMFORMAT = 1538i32;
pub const BM_S2DOT13FIXED_scRGB: BMFORMAT = 1539i32;
pub const BM_S2DOT13FIXED_scARGB: BMFORMAT = 1540i32;
pub const BM_R10G10B10A2: BMFORMAT = 1793i32;
pub const BM_R10G10B10A2_XR: BMFORMAT = 1794i32;
pub const BM_R16G16B16A16_FLOAT: BMFORMAT = 1795i32;
#[repr(C)]
pub struct BlackInformation {
    pub fBlackOnly: ::win32_foundation_sys::BOOL,
    pub blackWeight: f32,
}
impl ::core::marker::Copy for BlackInformation {}
impl ::core::clone::Clone for BlackInformation {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CATID_WcsPlugin: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 2696151776, data2: 33344, data3: 16479, data4: [138, 22, 138, 91, 77, 242, 240, 221] };
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
pub type COLORDATATYPE = i32;
pub const COLOR_BYTE: COLORDATATYPE = 1i32;
pub const COLOR_WORD: COLORDATATYPE = 2i32;
pub const COLOR_FLOAT: COLORDATATYPE = 3i32;
pub const COLOR_S2DOT13FIXED: COLORDATATYPE = 4i32;
pub const COLOR_10b_R10G10B10A2: COLORDATATYPE = 5i32;
pub const COLOR_10b_R10G10B10A2_XR: COLORDATATYPE = 6i32;
pub const COLOR_FLOAT16: COLORDATATYPE = 7i32;
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub struct COLORMATCHSETUPA {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub hwndOwner: ::win32_foundation_sys::HWND,
    pub pSourceName: ::windows_core_sys::PCSTR,
    pub pDisplayName: ::windows_core_sys::PCSTR,
    pub pPrinterName: ::windows_core_sys::PCSTR,
    pub dwRenderIntent: u32,
    pub dwProofingIntent: u32,
    pub pMonitorProfile: ::windows_core_sys::PSTR,
    pub ccMonitorProfile: u32,
    pub pPrinterProfile: ::windows_core_sys::PSTR,
    pub ccPrinterProfile: u32,
    pub pTargetProfile: ::windows_core_sys::PSTR,
    pub ccTargetProfile: u32,
    pub lpfnHook: super::WindowsAndMessaging::DLGPROC,
    pub lParam: ::win32_foundation_sys::LPARAM,
    pub lpfnApplyCallback: PCMSCALLBACKA,
    pub lParamApplyCallback: ::win32_foundation_sys::LPARAM,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::marker::Copy for COLORMATCHSETUPA {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::clone::Clone for COLORMATCHSETUPA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub struct COLORMATCHSETUPW {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub hwndOwner: ::win32_foundation_sys::HWND,
    pub pSourceName: ::windows_core_sys::PCWSTR,
    pub pDisplayName: ::windows_core_sys::PCWSTR,
    pub pPrinterName: ::windows_core_sys::PCWSTR,
    pub dwRenderIntent: u32,
    pub dwProofingIntent: u32,
    pub pMonitorProfile: ::windows_core_sys::PWSTR,
    pub ccMonitorProfile: u32,
    pub pPrinterProfile: ::windows_core_sys::PWSTR,
    pub ccPrinterProfile: u32,
    pub pTargetProfile: ::windows_core_sys::PWSTR,
    pub ccTargetProfile: u32,
    pub lpfnHook: super::WindowsAndMessaging::DLGPROC,
    pub lParam: ::win32_foundation_sys::LPARAM,
    pub lpfnApplyCallback: PCMSCALLBACKW,
    pub lParamApplyCallback: ::win32_foundation_sys::LPARAM,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::marker::Copy for COLORMATCHSETUPW {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::clone::Clone for COLORMATCHSETUPW {
    fn clone(&self) -> Self {
        *self
    }
}
pub type COLORPROFILESUBTYPE = i32;
pub const CPST_PERCEPTUAL: COLORPROFILESUBTYPE = 0i32;
pub const CPST_RELATIVE_COLORIMETRIC: COLORPROFILESUBTYPE = 1i32;
pub const CPST_SATURATION: COLORPROFILESUBTYPE = 2i32;
pub const CPST_ABSOLUTE_COLORIMETRIC: COLORPROFILESUBTYPE = 3i32;
pub const CPST_NONE: COLORPROFILESUBTYPE = 4i32;
pub const CPST_RGB_WORKING_SPACE: COLORPROFILESUBTYPE = 5i32;
pub const CPST_CUSTOM_WORKING_SPACE: COLORPROFILESUBTYPE = 6i32;
pub const CPST_STANDARD_DISPLAY_COLOR_MODE: COLORPROFILESUBTYPE = 7i32;
pub const CPST_EXTENDED_DISPLAY_COLOR_MODE: COLORPROFILESUBTYPE = 8i32;
pub type COLORPROFILETYPE = i32;
pub const CPT_ICC: COLORPROFILETYPE = 0i32;
pub const CPT_DMP: COLORPROFILETYPE = 1i32;
pub const CPT_CAMP: COLORPROFILETYPE = 2i32;
pub const CPT_GMMP: COLORPROFILETYPE = 3i32;
pub type COLORTYPE = i32;
pub const COLOR_GRAY: COLORTYPE = 1i32;
pub const COLOR_RGB: COLORTYPE = 2i32;
pub const COLOR_XYZ: COLORTYPE = 3i32;
pub const COLOR_Yxy: COLORTYPE = 4i32;
pub const COLOR_Lab: COLORTYPE = 5i32;
pub const COLOR_3_CHANNEL: COLORTYPE = 6i32;
pub const COLOR_CMYK: COLORTYPE = 7i32;
pub const COLOR_5_CHANNEL: COLORTYPE = 8i32;
pub const COLOR_6_CHANNEL: COLORTYPE = 9i32;
pub const COLOR_7_CHANNEL: COLORTYPE = 10i32;
pub const COLOR_8_CHANNEL: COLORTYPE = 11i32;
pub const COLOR_NAMED: COLORTYPE = 12i32;
pub type COLOR_MATCH_TO_TARGET_ACTION = i32;
pub const CS_ENABLE: COLOR_MATCH_TO_TARGET_ACTION = 1i32;
pub const CS_DISABLE: COLOR_MATCH_TO_TARGET_ACTION = 2i32;
pub const CS_DELETE_TRANSFORM: COLOR_MATCH_TO_TARGET_ACTION = 3i32;
pub const COLOR_MATCH_VERSION: u32 = 512u32;
pub const CSA_A: u32 = 1u32;
pub const CSA_ABC: u32 = 2u32;
pub const CSA_CMYK: u32 = 7u32;
pub const CSA_DEF: u32 = 3u32;
pub const CSA_DEFG: u32 = 4u32;
pub const CSA_GRAY: u32 = 5u32;
pub const CSA_Lab: u32 = 8u32;
pub const CSA_RGB: u32 = 6u32;
pub const DONT_USE_EMBEDDED_WCS_PROFILES: i32 = 1i32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct EMRCREATECOLORSPACE {
    pub emr: ::win32_graphics_sys::Gdi::EMR,
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
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct EMRCREATECOLORSPACEW {
    pub emr: ::win32_graphics_sys::Gdi::EMR,
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
pub const ENABLE_GAMUT_CHECKING: u32 = 65536u32;
#[repr(C)]
pub struct ENUMTYPEA {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFields: u32,
    pub pDeviceName: ::windows_core_sys::PCSTR,
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
#[repr(C)]
pub struct ENUMTYPEW {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFields: u32,
    pub pDeviceName: ::windows_core_sys::PCWSTR,
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
pub type HCOLORSPACE = isize;
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
pub type ICMENUMPROCA = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_core_sys::PCSTR, param1: ::win32_foundation_sys::LPARAM) -> i32>;
pub type ICMENUMPROCW = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_core_sys::PCWSTR, param1: ::win32_foundation_sys::LPARAM) -> i32>;
pub type ICM_COMMAND = u32;
pub const ICM_ADDPROFILE: ICM_COMMAND = 1u32;
pub const ICM_DELETEPROFILE: ICM_COMMAND = 2u32;
pub const ICM_QUERYPROFILE: ICM_COMMAND = 3u32;
pub const ICM_SETDEFAULTPROFILE: ICM_COMMAND = 4u32;
pub const ICM_REGISTERICMATCHER: ICM_COMMAND = 5u32;
pub const ICM_UNREGISTERICMATCHER: ICM_COMMAND = 6u32;
pub const ICM_QUERYMATCH: ICM_COMMAND = 7u32;
pub type ICM_MODE = i32;
pub const ICM_OFF: ICM_MODE = 1i32;
pub const ICM_ON: ICM_MODE = 2i32;
pub const ICM_QUERY: ICM_MODE = 3i32;
pub const ICM_DONE_OUTSIDEDC: ICM_MODE = 4i32;
pub type IDeviceModelPlugIn = *mut ::core::ffi::c_void;
pub type IGamutMapModelPlugIn = *mut ::core::ffi::c_void;
pub const INDEX_DONT_CARE: u32 = 0u32;
pub const INTENT_ABSOLUTE_COLORIMETRIC: u32 = 3u32;
pub const INTENT_PERCEPTUAL: u32 = 0u32;
pub const INTENT_RELATIVE_COLORIMETRIC: u32 = 1u32;
pub const INTENT_SATURATION: u32 = 2u32;
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
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct LOGCOLORSPACEA {
    pub lcsSignature: u32,
    pub lcsVersion: u32,
    pub lcsSize: u32,
    pub lcsCSType: i32,
    pub lcsIntent: i32,
    pub lcsEndpoints: ::win32_graphics_sys::Gdi::CIEXYZTRIPLE,
    pub lcsGammaRed: u32,
    pub lcsGammaGreen: u32,
    pub lcsGammaBlue: u32,
    pub lcsFilename: [::win32_foundation_sys::CHAR; 260],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for LOGCOLORSPACEA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for LOGCOLORSPACEA {
    fn clone(&self) -> Self {
        *self
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
    pub lcsEndpoints: ::win32_graphics_sys::Gdi::CIEXYZTRIPLE,
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
pub type LPBMCALLBACKFN = ::core::option::Option<unsafe extern "system" fn(param0: u32, param1: u32, param2: ::win32_foundation_sys::LPARAM) -> ::win32_foundation_sys::BOOL>;
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
pub const NORMAL_MODE: u32 = 2u32;
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub type PCMSCALLBACKA = ::core::option::Option<unsafe extern "system" fn(param0: *mut COLORMATCHSETUPA, param1: ::win32_foundation_sys::LPARAM) -> ::win32_foundation_sys::BOOL>;
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub type PCMSCALLBACKW = ::core::option::Option<unsafe extern "system" fn(param0: *mut COLORMATCHSETUPW, param1: ::win32_foundation_sys::LPARAM) -> ::win32_foundation_sys::BOOL>;
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
    pub phIlluminant: ::win32_graphics_sys::Gdi::CIEXYZ,
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
pub const SEQUENTIAL_TRANSFORM: u32 = 2155872256u32;
pub const USE_RELATIVE_COLORIMETRIC: u32 = 131072u32;
pub const WCS_ALWAYS: u32 = 2097152u32;
pub const WCS_DEFAULT: i32 = 0i32;
pub type WCS_DEVICE_CAPABILITIES_TYPE = i32;
pub const VideoCardGammaTable: WCS_DEVICE_CAPABILITIES_TYPE = 1i32;
pub const MicrosoftHardwareColorV2: WCS_DEVICE_CAPABILITIES_TYPE = 2i32;
#[repr(C)]
pub struct WCS_DEVICE_MHC2_CAPABILITIES {
    pub Size: u32,
    pub SupportsMhc2: ::win32_foundation_sys::BOOL,
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
#[repr(C)]
pub struct WCS_DEVICE_VCGT_CAPABILITIES {
    pub Size: u32,
    pub SupportsVcgt: ::win32_foundation_sys::BOOL,
}
impl ::core::marker::Copy for WCS_DEVICE_VCGT_CAPABILITIES {}
impl ::core::clone::Clone for WCS_DEVICE_VCGT_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WCS_ICCONLY: i32 = 65536i32;
pub type WCS_PROFILE_MANAGEMENT_SCOPE = i32;
pub const WCS_PROFILE_MANAGEMENT_SCOPE_SYSTEM_WIDE: WCS_PROFILE_MANAGEMENT_SCOPE = 0i32;
pub const WCS_PROFILE_MANAGEMENT_SCOPE_CURRENT_USER: WCS_PROFILE_MANAGEMENT_SCOPE = 1i32;
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
