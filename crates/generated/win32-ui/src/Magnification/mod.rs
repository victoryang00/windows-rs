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
impl ::core::fmt::Debug for MAGCOLOREFFECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MAGCOLOREFFECT").field("transform", &self.transform).finish()
    }
}
unsafe impl ::windows_core::Abi for MAGCOLOREFFECT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MAGCOLOREFFECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MAGCOLOREFFECT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MAGCOLOREFFECT {}
impl ::core::default::Default for MAGCOLOREFFECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MAGIMAGEHEADER {
    pub width: u32,
    pub height: u32,
    pub format: ::windows_core::GUID,
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
impl ::core::fmt::Debug for MAGIMAGEHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MAGIMAGEHEADER").field("width", &self.width).field("height", &self.height).field("format", &self.format).field("stride", &self.stride).field("offset", &self.offset).field("cbSize", &self.cbSize).finish()
    }
}
unsafe impl ::windows_core::Abi for MAGIMAGEHEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MAGIMAGEHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MAGIMAGEHEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for MAGIMAGEHEADER {}
impl ::core::default::Default for MAGIMAGEHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for MAGTRANSFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MAGTRANSFORM").field("v", &self.v).finish()
    }
}
unsafe impl ::windows_core::Abi for MAGTRANSFORM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MAGTRANSFORM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MAGTRANSFORM>()) == 0 }
    }
}
impl ::core::cmp::Eq for MAGTRANSFORM {}
impl ::core::default::Default for MAGTRANSFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const MS_CLIPAROUNDCURSOR: i32 = 2i32;
pub const MS_INVERTCOLORS: i32 = 4i32;
pub const MS_SHOWMAGNIFIEDCURSOR: i32 = 1i32;
pub const MW_FILTERMODE_EXCLUDE: u32 = 0u32;
pub const MW_FILTERMODE_INCLUDE: u32 = 1u32;
#[inline]
pub unsafe fn MagGetColorEffect<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnd: Param0, peffect: *mut MAGCOLOREFFECT) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagGetColorEffect(hwnd: ::win32_foundation::HWND, peffect: *mut MAGCOLOREFFECT) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(MagGetColorEffect(hwnd.into_param().abi(), ::core::mem::transmute(peffect)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MagGetFullscreenColorEffect(peffect: *mut MAGCOLOREFFECT) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagGetFullscreenColorEffect(peffect: *mut MAGCOLOREFFECT) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(MagGetFullscreenColorEffect(::core::mem::transmute(peffect)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MagGetFullscreenTransform(pmaglevel: *mut f32, pxoffset: *mut i32, pyoffset: *mut i32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagGetFullscreenTransform(pmaglevel: *mut f32, pxoffset: *mut i32, pyoffset: *mut i32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(MagGetFullscreenTransform(::core::mem::transmute(pmaglevel), ::core::mem::transmute(pxoffset), ::core::mem::transmute(pyoffset)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn MagGetImageScalingCallback<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnd: Param0) -> MagImageScalingCallback {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagGetImageScalingCallback(hwnd: ::win32_foundation::HWND) -> MagImageScalingCallback;
        }
        ::core::mem::transmute(MagGetImageScalingCallback(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MagGetInputTransform(pfenabled: *mut ::win32_foundation::BOOL, prectsource: *mut ::win32_foundation::RECT, prectdest: *mut ::win32_foundation::RECT) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagGetInputTransform(pfenabled: *mut ::win32_foundation::BOOL, prectsource: *mut ::win32_foundation::RECT, prectdest: *mut ::win32_foundation::RECT) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(MagGetInputTransform(::core::mem::transmute(pfenabled), ::core::mem::transmute(prectsource), ::core::mem::transmute(prectdest)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MagGetWindowFilterList<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnd: Param0, pdwfiltermode: *mut u32, count: i32, phwnd: *mut ::win32_foundation::HWND) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagGetWindowFilterList(hwnd: ::win32_foundation::HWND, pdwfiltermode: *mut u32, count: i32, phwnd: *mut ::win32_foundation::HWND) -> i32;
        }
        ::core::mem::transmute(MagGetWindowFilterList(hwnd.into_param().abi(), ::core::mem::transmute(pdwfiltermode), ::core::mem::transmute(count), ::core::mem::transmute(phwnd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MagGetWindowSource<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnd: Param0, prect: *mut ::win32_foundation::RECT) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagGetWindowSource(hwnd: ::win32_foundation::HWND, prect: *mut ::win32_foundation::RECT) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(MagGetWindowSource(hwnd.into_param().abi(), ::core::mem::transmute(prect)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MagGetWindowTransform<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnd: Param0, ptransform: *mut MAGTRANSFORM) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagGetWindowTransform(hwnd: ::win32_foundation::HWND, ptransform: *mut MAGTRANSFORM) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(MagGetWindowTransform(hwnd.into_param().abi(), ::core::mem::transmute(ptransform)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type MagImageScalingCallback = ::core::option::Option<unsafe extern "system" fn(hwnd: ::win32_foundation::HWND, srcdata: *mut ::core::ffi::c_void, srcheader: MAGIMAGEHEADER, destdata: *mut ::core::ffi::c_void, destheader: MAGIMAGEHEADER, unclipped: ::win32_foundation::RECT, clipped: ::win32_foundation::RECT, dirty: ::win32_graphics::Gdi::HRGN) -> ::win32_foundation::BOOL>;
#[inline]
pub unsafe fn MagInitialize() -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagInitialize() -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(MagInitialize())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MagSetColorEffect<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnd: Param0, peffect: *mut MAGCOLOREFFECT) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagSetColorEffect(hwnd: ::win32_foundation::HWND, peffect: *mut MAGCOLOREFFECT) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(MagSetColorEffect(hwnd.into_param().abi(), ::core::mem::transmute(peffect)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MagSetFullscreenColorEffect(peffect: *const MAGCOLOREFFECT) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagSetFullscreenColorEffect(peffect: *const MAGCOLOREFFECT) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(MagSetFullscreenColorEffect(::core::mem::transmute(peffect)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MagSetFullscreenTransform(maglevel: f32, xoffset: i32, yoffset: i32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagSetFullscreenTransform(maglevel: f32, xoffset: i32, yoffset: i32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(MagSetFullscreenTransform(::core::mem::transmute(maglevel), ::core::mem::transmute(xoffset), ::core::mem::transmute(yoffset)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn MagSetImageScalingCallback<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnd: Param0, callback: MagImageScalingCallback) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagSetImageScalingCallback(hwnd: ::win32_foundation::HWND, callback: ::windows_core::RawPtr) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(MagSetImageScalingCallback(hwnd.into_param().abi(), ::core::mem::transmute(callback)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MagSetInputTransform<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(fenabled: Param0, prectsource: *const ::win32_foundation::RECT, prectdest: *const ::win32_foundation::RECT) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagSetInputTransform(fenabled: ::win32_foundation::BOOL, prectsource: *const ::win32_foundation::RECT, prectdest: *const ::win32_foundation::RECT) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(MagSetInputTransform(fenabled.into_param().abi(), ::core::mem::transmute(prectsource), ::core::mem::transmute(prectdest)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MagSetWindowFilterList<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnd: Param0, dwfiltermode: u32, count: i32, phwnd: *mut ::win32_foundation::HWND) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagSetWindowFilterList(hwnd: ::win32_foundation::HWND, dwfiltermode: u32, count: i32, phwnd: *mut ::win32_foundation::HWND) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(MagSetWindowFilterList(hwnd.into_param().abi(), ::core::mem::transmute(dwfiltermode), ::core::mem::transmute(count), ::core::mem::transmute(phwnd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MagSetWindowSource<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::RECT>>(hwnd: Param0, rect: Param1) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagSetWindowSource(hwnd: ::win32_foundation::HWND, rect: ::win32_foundation::RECT) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(MagSetWindowSource(hwnd.into_param().abi(), rect.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MagSetWindowTransform<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnd: Param0, ptransform: *mut MAGTRANSFORM) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagSetWindowTransform(hwnd: ::win32_foundation::HWND, ptransform: *mut MAGTRANSFORM) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(MagSetWindowTransform(hwnd.into_param().abi(), ::core::mem::transmute(ptransform)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MagShowSystemCursor<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(fshowcursor: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagShowSystemCursor(fshowcursor: ::win32_foundation::BOOL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(MagShowSystemCursor(fshowcursor.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MagUninitialize() -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MagUninitialize() -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(MagUninitialize())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const WC_MAGNIFIER: &str = "Magnifier";
pub const WC_MAGNIFIERA: &str = "Magnifier";
pub const WC_MAGNIFIERW: &str = "Magnifier";
