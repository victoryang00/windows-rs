#[link(name = "windows")]
extern "system" {
    pub fn MagGetColorEffect(hwnd: ::win32_foundation_sys::HWND, peffect: *mut MAGCOLOREFFECT) -> ::win32_foundation_sys::BOOL;
    pub fn MagGetFullscreenColorEffect(peffect: *mut MAGCOLOREFFECT) -> ::win32_foundation_sys::BOOL;
    pub fn MagGetFullscreenTransform(pmaglevel: *mut f32, pxoffset: *mut i32, pyoffset: *mut i32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn MagGetImageScalingCallback(hwnd: ::win32_foundation_sys::HWND) -> MagImageScalingCallback;
    pub fn MagGetInputTransform(pfenabled: *mut ::win32_foundation_sys::BOOL, prectsource: *mut ::win32_foundation_sys::RECT, prectdest: *mut ::win32_foundation_sys::RECT) -> ::win32_foundation_sys::BOOL;
    pub fn MagGetWindowFilterList(hwnd: ::win32_foundation_sys::HWND, pdwfiltermode: *mut u32, count: i32, phwnd: *mut ::win32_foundation_sys::HWND) -> i32;
    pub fn MagGetWindowSource(hwnd: ::win32_foundation_sys::HWND, prect: *mut ::win32_foundation_sys::RECT) -> ::win32_foundation_sys::BOOL;
    pub fn MagGetWindowTransform(hwnd: ::win32_foundation_sys::HWND, ptransform: *mut MAGTRANSFORM) -> ::win32_foundation_sys::BOOL;
    pub fn MagInitialize() -> ::win32_foundation_sys::BOOL;
    pub fn MagSetColorEffect(hwnd: ::win32_foundation_sys::HWND, peffect: *mut MAGCOLOREFFECT) -> ::win32_foundation_sys::BOOL;
    pub fn MagSetFullscreenColorEffect(peffect: *const MAGCOLOREFFECT) -> ::win32_foundation_sys::BOOL;
    pub fn MagSetFullscreenTransform(maglevel: f32, xoffset: i32, yoffset: i32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn MagSetImageScalingCallback(hwnd: ::win32_foundation_sys::HWND, callback: MagImageScalingCallback) -> ::win32_foundation_sys::BOOL;
    pub fn MagSetInputTransform(fenabled: ::win32_foundation_sys::BOOL, prectsource: *const ::win32_foundation_sys::RECT, prectdest: *const ::win32_foundation_sys::RECT) -> ::win32_foundation_sys::BOOL;
    pub fn MagSetWindowFilterList(hwnd: ::win32_foundation_sys::HWND, dwfiltermode: u32, count: i32, phwnd: *mut ::win32_foundation_sys::HWND) -> ::win32_foundation_sys::BOOL;
    pub fn MagSetWindowSource(hwnd: ::win32_foundation_sys::HWND, rect: ::win32_foundation_sys::RECT) -> ::win32_foundation_sys::BOOL;
    pub fn MagSetWindowTransform(hwnd: ::win32_foundation_sys::HWND, ptransform: *mut MAGTRANSFORM) -> ::win32_foundation_sys::BOOL;
    pub fn MagShowSystemCursor(fshowcursor: ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    pub fn MagUninitialize() -> ::win32_foundation_sys::BOOL;
}
#[repr(C)]
pub struct MAGCOLOREFFECT {
    pub transform: [f32; 25],
}
impl ::core::marker::Copy for MAGCOLOREFFECT {}
impl ::core::clone::Clone for MAGCOLOREFFECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MAGIMAGEHEADER {
    pub width: u32,
    pub height: u32,
    pub format: ::windows_core_sys::GUID,
    pub stride: u32,
    pub offset: u32,
    pub cbSize: usize,
}
impl ::core::marker::Copy for MAGIMAGEHEADER {}
impl ::core::clone::Clone for MAGIMAGEHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MAGTRANSFORM {
    pub v: [f32; 9],
}
impl ::core::marker::Copy for MAGTRANSFORM {}
impl ::core::clone::Clone for MAGTRANSFORM {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MS_CLIPAROUNDCURSOR: i32 = 2i32;
pub const MS_INVERTCOLORS: i32 = 4i32;
pub const MS_SHOWMAGNIFIEDCURSOR: i32 = 1i32;
pub const MW_FILTERMODE_EXCLUDE: u32 = 0u32;
pub const MW_FILTERMODE_INCLUDE: u32 = 1u32;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type MagImageScalingCallback = ::core::option::Option<unsafe extern "system" fn(hwnd: ::win32_foundation_sys::HWND, srcdata: *mut ::core::ffi::c_void, srcheader: MAGIMAGEHEADER, destdata: *mut ::core::ffi::c_void, destheader: MAGIMAGEHEADER, unclipped: ::win32_foundation_sys::RECT, clipped: ::win32_foundation_sys::RECT, dirty: ::win32_graphics_sys::Gdi::HRGN) -> ::win32_foundation_sys::BOOL>;
pub const WC_MAGNIFIER: &str = "Magnifier";
pub const WC_MAGNIFIERA: &str = "Magnifier";
pub const WC_MAGNIFIERW: &str = "Magnifier";
