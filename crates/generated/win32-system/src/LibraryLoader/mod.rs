#[inline]
pub unsafe fn AddDllDirectory<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(newdirectory: Param0) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddDllDirectory(newdirectory: ::windows_core::PCWSTR) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(AddDllDirectory(newdirectory.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BeginUpdateResourceA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(pfilename: Param0, bdeleteexistingresources: Param1) -> ::windows_core::Result<::win32_foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BeginUpdateResourceA(pfilename: ::windows_core::PCSTR, bdeleteexistingresources: ::win32_foundation::BOOL) -> ::win32_foundation::HANDLE;
        }
        let result__ = BeginUpdateResourceA(pfilename.into_param().abi(), bdeleteexistingresources.into_param().abi());
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows_core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BeginUpdateResourceW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(pfilename: Param0, bdeleteexistingresources: Param1) -> ::windows_core::Result<::win32_foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BeginUpdateResourceW(pfilename: ::windows_core::PCWSTR, bdeleteexistingresources: ::win32_foundation::BOOL) -> ::win32_foundation::HANDLE;
        }
        let result__ = BeginUpdateResourceW(pfilename.into_param().abi(), bdeleteexistingresources.into_param().abi());
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows_core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const CURRENT_IMPORT_REDIRECTION_VERSION: u32 = 1u32;
#[inline]
pub unsafe fn DisableThreadLibraryCalls<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>>(hlibmodule: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DisableThreadLibraryCalls(hlibmodule: ::win32_foundation::HINSTANCE) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(DisableThreadLibraryCalls(hlibmodule.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub type ENUMRESLANGPROCA = ::core::option::Option<unsafe extern "system" fn(hmodule: ::win32_foundation::HINSTANCE, lptype: ::windows_core::PCSTR, lpname: ::windows_core::PCSTR, wlanguage: u16, lparam: isize) -> ::win32_foundation::BOOL>;
pub type ENUMRESLANGPROCW = ::core::option::Option<unsafe extern "system" fn(hmodule: ::win32_foundation::HINSTANCE, lptype: ::windows_core::PCWSTR, lpname: ::windows_core::PCWSTR, wlanguage: u16, lparam: isize) -> ::win32_foundation::BOOL>;
pub type ENUMRESNAMEPROCA = ::core::option::Option<unsafe extern "system" fn(hmodule: ::win32_foundation::HINSTANCE, lptype: ::windows_core::PCSTR, lpname: ::windows_core::PCSTR, lparam: isize) -> ::win32_foundation::BOOL>;
pub type ENUMRESNAMEPROCW = ::core::option::Option<unsafe extern "system" fn(hmodule: ::win32_foundation::HINSTANCE, lptype: ::windows_core::PCWSTR, lpname: ::windows_core::PCWSTR, lparam: isize) -> ::win32_foundation::BOOL>;
pub type ENUMRESTYPEPROCA = ::core::option::Option<unsafe extern "system" fn(hmodule: ::win32_foundation::HINSTANCE, lptype: ::windows_core::PCSTR, lparam: isize) -> ::win32_foundation::BOOL>;
pub type ENUMRESTYPEPROCW = ::core::option::Option<unsafe extern "system" fn(hmodule: ::win32_foundation::HINSTANCE, lptype: ::windows_core::PCWSTR, lparam: isize) -> ::win32_foundation::BOOL>;
#[repr(C)]
pub struct ENUMUILANG {
    pub NumOfEnumUILang: u32,
    pub SizeOfEnumUIBuffer: u32,
    pub pEnumUIBuffer: *mut u16,
}
impl ::core::marker::Copy for ENUMUILANG {}
impl ::core::clone::Clone for ENUMUILANG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENUMUILANG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMUILANG").field("NumOfEnumUILang", &self.NumOfEnumUILang).field("SizeOfEnumUIBuffer", &self.SizeOfEnumUIBuffer).field("pEnumUIBuffer", &self.pEnumUIBuffer).finish()
    }
}
unsafe impl ::windows_core::Abi for ENUMUILANG {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ENUMUILANG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENUMUILANG>()) == 0 }
    }
}
impl ::core::cmp::Eq for ENUMUILANG {}
impl ::core::default::Default for ENUMUILANG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[inline]
pub unsafe fn EndUpdateResourceA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(hupdate: Param0, fdiscard: Param1) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EndUpdateResourceA(hupdate: ::win32_foundation::HANDLE, fdiscard: ::win32_foundation::BOOL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EndUpdateResourceA(hupdate.into_param().abi(), fdiscard.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EndUpdateResourceW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(hupdate: Param0, fdiscard: Param1) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EndUpdateResourceW(hupdate: ::win32_foundation::HANDLE, fdiscard: ::win32_foundation::BOOL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EndUpdateResourceW(hupdate.into_param().abi(), fdiscard.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EnumResourceLanguagesA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(hmodule: Param0, lptype: Param1, lpname: Param2, lpenumfunc: ENUMRESLANGPROCA, lparam: isize) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceLanguagesA(hmodule: ::win32_foundation::HINSTANCE, lptype: ::windows_core::PCSTR, lpname: ::windows_core::PCSTR, lpenumfunc: ::windows_core::RawPtr, lparam: isize) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EnumResourceLanguagesA(hmodule.into_param().abi(), lptype.into_param().abi(), lpname.into_param().abi(), ::core::mem::transmute(lpenumfunc), ::core::mem::transmute(lparam)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EnumResourceLanguagesExA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(hmodule: Param0, lptype: Param1, lpname: Param2, lpenumfunc: ENUMRESLANGPROCA, lparam: isize, dwflags: u32, langid: u16) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceLanguagesExA(hmodule: ::win32_foundation::HINSTANCE, lptype: ::windows_core::PCSTR, lpname: ::windows_core::PCSTR, lpenumfunc: ::windows_core::RawPtr, lparam: isize, dwflags: u32, langid: u16) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EnumResourceLanguagesExA(hmodule.into_param().abi(), lptype.into_param().abi(), lpname.into_param().abi(), ::core::mem::transmute(lpenumfunc), ::core::mem::transmute(lparam), ::core::mem::transmute(dwflags), ::core::mem::transmute(langid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EnumResourceLanguagesExW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hmodule: Param0, lptype: Param1, lpname: Param2, lpenumfunc: ENUMRESLANGPROCW, lparam: isize, dwflags: u32, langid: u16) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceLanguagesExW(hmodule: ::win32_foundation::HINSTANCE, lptype: ::windows_core::PCWSTR, lpname: ::windows_core::PCWSTR, lpenumfunc: ::windows_core::RawPtr, lparam: isize, dwflags: u32, langid: u16) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EnumResourceLanguagesExW(hmodule.into_param().abi(), lptype.into_param().abi(), lpname.into_param().abi(), ::core::mem::transmute(lpenumfunc), ::core::mem::transmute(lparam), ::core::mem::transmute(dwflags), ::core::mem::transmute(langid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EnumResourceLanguagesW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hmodule: Param0, lptype: Param1, lpname: Param2, lpenumfunc: ENUMRESLANGPROCW, lparam: isize) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceLanguagesW(hmodule: ::win32_foundation::HINSTANCE, lptype: ::windows_core::PCWSTR, lpname: ::windows_core::PCWSTR, lpenumfunc: ::windows_core::RawPtr, lparam: isize) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EnumResourceLanguagesW(hmodule.into_param().abi(), lptype.into_param().abi(), lpname.into_param().abi(), ::core::mem::transmute(lpenumfunc), ::core::mem::transmute(lparam)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EnumResourceNamesA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(hmodule: Param0, lptype: Param1, lpenumfunc: ENUMRESNAMEPROCA, lparam: isize) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceNamesA(hmodule: ::win32_foundation::HINSTANCE, lptype: ::windows_core::PCSTR, lpenumfunc: ::windows_core::RawPtr, lparam: isize) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EnumResourceNamesA(hmodule.into_param().abi(), lptype.into_param().abi(), ::core::mem::transmute(lpenumfunc), ::core::mem::transmute(lparam)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EnumResourceNamesExA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(hmodule: Param0, lptype: Param1, lpenumfunc: ENUMRESNAMEPROCA, lparam: isize, dwflags: u32, langid: u16) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceNamesExA(hmodule: ::win32_foundation::HINSTANCE, lptype: ::windows_core::PCSTR, lpenumfunc: ::windows_core::RawPtr, lparam: isize, dwflags: u32, langid: u16) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EnumResourceNamesExA(hmodule.into_param().abi(), lptype.into_param().abi(), ::core::mem::transmute(lpenumfunc), ::core::mem::transmute(lparam), ::core::mem::transmute(dwflags), ::core::mem::transmute(langid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EnumResourceNamesExW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hmodule: Param0, lptype: Param1, lpenumfunc: ENUMRESNAMEPROCW, lparam: isize, dwflags: u32, langid: u16) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceNamesExW(hmodule: ::win32_foundation::HINSTANCE, lptype: ::windows_core::PCWSTR, lpenumfunc: ::windows_core::RawPtr, lparam: isize, dwflags: u32, langid: u16) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EnumResourceNamesExW(hmodule.into_param().abi(), lptype.into_param().abi(), ::core::mem::transmute(lpenumfunc), ::core::mem::transmute(lparam), ::core::mem::transmute(dwflags), ::core::mem::transmute(langid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EnumResourceNamesW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hmodule: Param0, lptype: Param1, lpenumfunc: ENUMRESNAMEPROCW, lparam: isize) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceNamesW(hmodule: ::win32_foundation::HINSTANCE, lptype: ::windows_core::PCWSTR, lpenumfunc: ::windows_core::RawPtr, lparam: isize) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EnumResourceNamesW(hmodule.into_param().abi(), lptype.into_param().abi(), ::core::mem::transmute(lpenumfunc), ::core::mem::transmute(lparam)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EnumResourceTypesA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>>(hmodule: Param0, lpenumfunc: ENUMRESTYPEPROCA, lparam: isize) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceTypesA(hmodule: ::win32_foundation::HINSTANCE, lpenumfunc: ::windows_core::RawPtr, lparam: isize) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EnumResourceTypesA(hmodule.into_param().abi(), ::core::mem::transmute(lpenumfunc), ::core::mem::transmute(lparam)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EnumResourceTypesExA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>>(hmodule: Param0, lpenumfunc: ENUMRESTYPEPROCA, lparam: isize, dwflags: u32, langid: u16) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceTypesExA(hmodule: ::win32_foundation::HINSTANCE, lpenumfunc: ::windows_core::RawPtr, lparam: isize, dwflags: u32, langid: u16) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EnumResourceTypesExA(hmodule.into_param().abi(), ::core::mem::transmute(lpenumfunc), ::core::mem::transmute(lparam), ::core::mem::transmute(dwflags), ::core::mem::transmute(langid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EnumResourceTypesExW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>>(hmodule: Param0, lpenumfunc: ENUMRESTYPEPROCW, lparam: isize, dwflags: u32, langid: u16) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceTypesExW(hmodule: ::win32_foundation::HINSTANCE, lpenumfunc: ::windows_core::RawPtr, lparam: isize, dwflags: u32, langid: u16) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EnumResourceTypesExW(hmodule.into_param().abi(), ::core::mem::transmute(lpenumfunc), ::core::mem::transmute(lparam), ::core::mem::transmute(dwflags), ::core::mem::transmute(langid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EnumResourceTypesW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>>(hmodule: Param0, lpenumfunc: ENUMRESTYPEPROCW, lparam: isize) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceTypesW(hmodule: ::win32_foundation::HINSTANCE, lpenumfunc: ::windows_core::RawPtr, lparam: isize) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EnumResourceTypesW(hmodule.into_param().abi(), ::core::mem::transmute(lpenumfunc), ::core::mem::transmute(lparam)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FIND_RESOURCE_DIRECTORY_LANGUAGES: u32 = 1024u32;
pub const FIND_RESOURCE_DIRECTORY_NAMES: u32 = 512u32;
pub const FIND_RESOURCE_DIRECTORY_TYPES: u32 = 256u32;
#[inline]
pub unsafe fn FindResourceA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(hmodule: Param0, lpname: Param1, lptype: Param2) -> ::windows_core::Result<::win32_foundation::HRSRC> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindResourceA(hmodule: ::win32_foundation::HINSTANCE, lpname: ::windows_core::PCSTR, lptype: ::windows_core::PCSTR) -> ::win32_foundation::HRSRC;
        }
        let result__ = FindResourceA(hmodule.into_param().abi(), lpname.into_param().abi(), lptype.into_param().abi());
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows_core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn FindResourceExA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(hmodule: Param0, lptype: Param1, lpname: Param2, wlanguage: u16) -> ::windows_core::Result<::win32_foundation::HRSRC> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindResourceExA(hmodule: ::win32_foundation::HINSTANCE, lptype: ::windows_core::PCSTR, lpname: ::windows_core::PCSTR, wlanguage: u16) -> ::win32_foundation::HRSRC;
        }
        let result__ = FindResourceExA(hmodule.into_param().abi(), lptype.into_param().abi(), lpname.into_param().abi(), ::core::mem::transmute(wlanguage));
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows_core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn FindResourceExW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hmodule: Param0, lptype: Param1, lpname: Param2, wlanguage: u16) -> ::win32_foundation::HRSRC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindResourceExW(hmodule: ::win32_foundation::HINSTANCE, lptype: ::windows_core::PCWSTR, lpname: ::windows_core::PCWSTR, wlanguage: u16) -> ::win32_foundation::HRSRC;
        }
        ::core::mem::transmute(FindResourceExW(hmodule.into_param().abi(), lptype.into_param().abi(), lpname.into_param().abi(), ::core::mem::transmute(wlanguage)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn FindResourceW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hmodule: Param0, lpname: Param1, lptype: Param2) -> ::win32_foundation::HRSRC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindResourceW(hmodule: ::win32_foundation::HINSTANCE, lpname: ::windows_core::PCWSTR, lptype: ::windows_core::PCWSTR) -> ::win32_foundation::HRSRC;
        }
        ::core::mem::transmute(FindResourceW(hmodule.into_param().abi(), lpname.into_param().abi(), lptype.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn FreeLibrary<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>>(hlibmodule: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeLibrary(hlibmodule: ::win32_foundation::HINSTANCE) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(FreeLibrary(hlibmodule.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn FreeLibraryAndExitThread<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>>(hlibmodule: Param0, dwexitcode: u32) -> ! {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeLibraryAndExitThread(hlibmodule: ::win32_foundation::HINSTANCE, dwexitcode: u32) -> !;
        }
        FreeLibraryAndExitThread(hlibmodule.into_param().abi(), ::core::mem::transmute(dwexitcode))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn FreeResource(hresdata: isize) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeResource(hresdata: isize) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(FreeResource(::core::mem::transmute(hresdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: u32 = 4u32;
pub const GET_MODULE_HANDLE_EX_FLAG_PIN: u32 = 1u32;
pub const GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT: u32 = 2u32;
#[inline]
pub unsafe fn GetDllDirectoryA(lpbuffer: &mut [u8]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDllDirectoryA(nbufferlength: u32, lpbuffer: ::windows_core::PSTR) -> u32;
        }
        ::core::mem::transmute(GetDllDirectoryA(lpbuffer.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(lpbuffer))))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetDllDirectoryW(lpbuffer: &mut [u16]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDllDirectoryW(nbufferlength: u32, lpbuffer: ::windows_core::PWSTR) -> u32;
        }
        ::core::mem::transmute(GetDllDirectoryW(lpbuffer.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(lpbuffer))))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetModuleFileNameA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>>(hmodule: Param0, lpfilename: &mut [u8]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetModuleFileNameA(hmodule: ::win32_foundation::HINSTANCE, lpfilename: ::windows_core::PSTR, nsize: u32) -> u32;
        }
        ::core::mem::transmute(GetModuleFileNameA(hmodule.into_param().abi(), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(lpfilename)), lpfilename.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetModuleFileNameW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>>(hmodule: Param0, lpfilename: &mut [u16]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetModuleFileNameW(hmodule: ::win32_foundation::HINSTANCE, lpfilename: ::windows_core::PWSTR, nsize: u32) -> u32;
        }
        ::core::mem::transmute(GetModuleFileNameW(hmodule.into_param().abi(), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(lpfilename)), lpfilename.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetModuleHandleA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(lpmodulename: Param0) -> ::windows_core::Result<::win32_foundation::HINSTANCE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetModuleHandleA(lpmodulename: ::windows_core::PCSTR) -> ::win32_foundation::HINSTANCE;
        }
        let result__ = GetModuleHandleA(lpmodulename.into_param().abi());
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows_core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetModuleHandleExA<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(dwflags: u32, lpmodulename: Param1, phmodule: *mut ::win32_foundation::HINSTANCE) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetModuleHandleExA(dwflags: u32, lpmodulename: ::windows_core::PCSTR, phmodule: *mut ::win32_foundation::HINSTANCE) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetModuleHandleExA(::core::mem::transmute(dwflags), lpmodulename.into_param().abi(), ::core::mem::transmute(phmodule)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetModuleHandleExW<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(dwflags: u32, lpmodulename: Param1, phmodule: *mut ::win32_foundation::HINSTANCE) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetModuleHandleExW(dwflags: u32, lpmodulename: ::windows_core::PCWSTR, phmodule: *mut ::win32_foundation::HINSTANCE) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetModuleHandleExW(::core::mem::transmute(dwflags), lpmodulename.into_param().abi(), ::core::mem::transmute(phmodule)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetModuleHandleW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(lpmodulename: Param0) -> ::windows_core::Result<::win32_foundation::HINSTANCE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetModuleHandleW(lpmodulename: ::windows_core::PCWSTR) -> ::win32_foundation::HINSTANCE;
        }
        let result__ = GetModuleHandleW(lpmodulename.into_param().abi());
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows_core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetProcAddress<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(hmodule: Param0, lpprocname: Param1) -> ::win32_foundation::FARPROC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcAddress(hmodule: ::win32_foundation::HINSTANCE, lpprocname: ::windows_core::PCSTR) -> ::win32_foundation::FARPROC;
        }
        ::core::mem::transmute(GetProcAddress(hmodule.into_param().abi(), lpprocname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LOAD_LIBRARY_FLAGS(pub u32);
pub const DONT_RESOLVE_DLL_REFERENCES: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(1u32);
pub const LOAD_LIBRARY_AS_DATAFILE: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(2u32);
pub const LOAD_WITH_ALTERED_SEARCH_PATH: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(8u32);
pub const LOAD_IGNORE_CODE_AUTHZ_LEVEL: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(16u32);
pub const LOAD_LIBRARY_AS_IMAGE_RESOURCE: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(32u32);
pub const LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(64u32);
pub const LOAD_LIBRARY_REQUIRE_SIGNED_TARGET: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(128u32);
pub const LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(256u32);
pub const LOAD_LIBRARY_SEARCH_APPLICATION_DIR: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(512u32);
pub const LOAD_LIBRARY_SEARCH_USER_DIRS: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(1024u32);
pub const LOAD_LIBRARY_SEARCH_SYSTEM32: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(2048u32);
pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(4096u32);
pub const LOAD_LIBRARY_SAFE_CURRENT_DIRS: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(8192u32);
pub const LOAD_LIBRARY_SEARCH_SYSTEM32_NO_FORWARDER: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(16384u32);
impl ::core::marker::Copy for LOAD_LIBRARY_FLAGS {}
impl ::core::clone::Clone for LOAD_LIBRARY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LOAD_LIBRARY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LOAD_LIBRARY_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for LOAD_LIBRARY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOAD_LIBRARY_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LOAD_LIBRARY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LOAD_LIBRARY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LOAD_LIBRARY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LOAD_LIBRARY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LOAD_LIBRARY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const LOAD_LIBRARY_OS_INTEGRITY_CONTINUITY: u32 = 32768u32;
#[inline]
pub unsafe fn LoadLibraryA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(lplibfilename: Param0) -> ::windows_core::Result<::win32_foundation::HINSTANCE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadLibraryA(lplibfilename: ::windows_core::PCSTR) -> ::win32_foundation::HINSTANCE;
        }
        let result__ = LoadLibraryA(lplibfilename.into_param().abi());
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows_core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LoadLibraryExA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(lplibfilename: Param0, hfile: Param1, dwflags: LOAD_LIBRARY_FLAGS) -> ::windows_core::Result<::win32_foundation::HINSTANCE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadLibraryExA(lplibfilename: ::windows_core::PCSTR, hfile: ::win32_foundation::HANDLE, dwflags: LOAD_LIBRARY_FLAGS) -> ::win32_foundation::HINSTANCE;
        }
        let result__ = LoadLibraryExA(lplibfilename.into_param().abi(), hfile.into_param().abi(), ::core::mem::transmute(dwflags));
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows_core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LoadLibraryExW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(lplibfilename: Param0, hfile: Param1, dwflags: LOAD_LIBRARY_FLAGS) -> ::windows_core::Result<::win32_foundation::HINSTANCE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadLibraryExW(lplibfilename: ::windows_core::PCWSTR, hfile: ::win32_foundation::HANDLE, dwflags: LOAD_LIBRARY_FLAGS) -> ::win32_foundation::HINSTANCE;
        }
        let result__ = LoadLibraryExW(lplibfilename.into_param().abi(), hfile.into_param().abi(), ::core::mem::transmute(dwflags));
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows_core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LoadLibraryW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(lplibfilename: Param0) -> ::windows_core::Result<::win32_foundation::HINSTANCE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadLibraryW(lplibfilename: ::windows_core::PCWSTR) -> ::win32_foundation::HINSTANCE;
        }
        let result__ = LoadLibraryW(lplibfilename.into_param().abi());
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows_core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LoadModule<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(lpmodulename: Param0, lpparameterblock: *const ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadModule(lpmodulename: ::windows_core::PCSTR, lpparameterblock: *const ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(LoadModule(lpmodulename.into_param().abi(), ::core::mem::transmute(lpparameterblock)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LoadPackagedLibrary<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(lpwlibfilename: Param0, reserved: u32) -> ::windows_core::Result<::win32_foundation::HINSTANCE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadPackagedLibrary(lpwlibfilename: ::windows_core::PCWSTR, reserved: u32) -> ::win32_foundation::HINSTANCE;
        }
        let result__ = LoadPackagedLibrary(lpwlibfilename.into_param().abi(), ::core::mem::transmute(reserved));
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows_core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LoadResource<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::HRSRC>>(hmodule: Param0, hresinfo: Param1) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadResource(hmodule: ::win32_foundation::HINSTANCE, hresinfo: ::win32_foundation::HRSRC) -> isize;
        }
        ::core::mem::transmute(LoadResource(hmodule.into_param().abi(), hresinfo.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LockResource(hresdata: isize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LockResource(hresdata: isize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(LockResource(::core::mem::transmute(hresdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub type PGET_MODULE_HANDLE_EXA = ::core::option::Option<unsafe extern "system" fn(dwflags: u32, lpmodulename: ::windows_core::PCSTR, phmodule: *mut ::win32_foundation::HINSTANCE) -> ::win32_foundation::BOOL>;
pub type PGET_MODULE_HANDLE_EXW = ::core::option::Option<unsafe extern "system" fn(dwflags: u32, lpmodulename: ::windows_core::PCWSTR, phmodule: *mut ::win32_foundation::HINSTANCE) -> ::win32_foundation::BOOL>;
#[repr(C)]
pub struct REDIRECTION_DESCRIPTOR {
    pub Version: u32,
    pub FunctionCount: u32,
    pub Redirections: *mut REDIRECTION_FUNCTION_DESCRIPTOR,
}
impl ::core::marker::Copy for REDIRECTION_DESCRIPTOR {}
impl ::core::clone::Clone for REDIRECTION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REDIRECTION_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REDIRECTION_DESCRIPTOR").field("Version", &self.Version).field("FunctionCount", &self.FunctionCount).field("Redirections", &self.Redirections).finish()
    }
}
unsafe impl ::windows_core::Abi for REDIRECTION_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REDIRECTION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REDIRECTION_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for REDIRECTION_DESCRIPTOR {}
impl ::core::default::Default for REDIRECTION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct REDIRECTION_FUNCTION_DESCRIPTOR {
    pub DllName: ::windows_core::PCSTR,
    pub FunctionName: ::windows_core::PCSTR,
    pub RedirectionTarget: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for REDIRECTION_FUNCTION_DESCRIPTOR {}
impl ::core::clone::Clone for REDIRECTION_FUNCTION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REDIRECTION_FUNCTION_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REDIRECTION_FUNCTION_DESCRIPTOR").field("DllName", &self.DllName).field("FunctionName", &self.FunctionName).field("RedirectionTarget", &self.RedirectionTarget).finish()
    }
}
unsafe impl ::windows_core::Abi for REDIRECTION_FUNCTION_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REDIRECTION_FUNCTION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REDIRECTION_FUNCTION_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for REDIRECTION_FUNCTION_DESCRIPTOR {}
impl ::core::default::Default for REDIRECTION_FUNCTION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const RESOURCE_ENUM_LN: u32 = 1u32;
pub const RESOURCE_ENUM_MODULE_EXACT: u32 = 16u32;
pub const RESOURCE_ENUM_MUI: u32 = 2u32;
pub const RESOURCE_ENUM_MUI_SYSTEM: u32 = 4u32;
pub const RESOURCE_ENUM_VALIDATE: u32 = 8u32;
#[inline]
pub unsafe fn RemoveDllDirectory(cookie: *const ::core::ffi::c_void) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RemoveDllDirectory(cookie: *const ::core::ffi::c_void) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(RemoveDllDirectory(::core::mem::transmute(cookie)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const SUPPORT_LANG_NUMBER: u32 = 32u32;
#[inline]
pub unsafe fn SetDefaultDllDirectories(directoryflags: LOAD_LIBRARY_FLAGS) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetDefaultDllDirectories(directoryflags: LOAD_LIBRARY_FLAGS) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetDefaultDllDirectories(::core::mem::transmute(directoryflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetDllDirectoryA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(lppathname: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetDllDirectoryA(lppathname: ::windows_core::PCSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetDllDirectoryA(lppathname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetDllDirectoryW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(lppathname: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetDllDirectoryW(lppathname: ::windows_core::PCWSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetDllDirectoryW(lppathname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SizeofResource<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::HRSRC>>(hmodule: Param0, hresinfo: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SizeofResource(hmodule: ::win32_foundation::HINSTANCE, hresinfo: ::win32_foundation::HRSRC) -> u32;
        }
        ::core::mem::transmute(SizeofResource(hmodule.into_param().abi(), hresinfo.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn UpdateResourceA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(hupdate: Param0, lptype: Param1, lpname: Param2, wlanguage: u16, lpdata: *const ::core::ffi::c_void, cb: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UpdateResourceA(hupdate: ::win32_foundation::HANDLE, lptype: ::windows_core::PCSTR, lpname: ::windows_core::PCSTR, wlanguage: u16, lpdata: *const ::core::ffi::c_void, cb: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(UpdateResourceA(hupdate.into_param().abi(), lptype.into_param().abi(), lpname.into_param().abi(), ::core::mem::transmute(wlanguage), ::core::mem::transmute(lpdata), ::core::mem::transmute(cb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn UpdateResourceW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hupdate: Param0, lptype: Param1, lpname: Param2, wlanguage: u16, lpdata: *const ::core::ffi::c_void, cb: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UpdateResourceW(hupdate: ::win32_foundation::HANDLE, lptype: ::windows_core::PCWSTR, lpname: ::windows_core::PCWSTR, wlanguage: u16, lpdata: *const ::core::ffi::c_void, cb: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(UpdateResourceW(hupdate.into_param().abi(), lptype.into_param().abi(), lpname.into_param().abi(), ::core::mem::transmute(wlanguage), ::core::mem::transmute(lpdata), ::core::mem::transmute(cb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
