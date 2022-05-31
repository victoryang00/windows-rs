#[cfg(feature = "win32-security")]
#[inline]
pub unsafe fn CreateMailslotA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(lpname: Param0, nmaxmessagesize: u32, lreadtimeout: u32, lpsecurityattributes: *const ::win32_security::SECURITY_ATTRIBUTES) -> ::windows_core::Result<::win32_foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateMailslotA(lpname: ::windows_core::PCSTR, nmaxmessagesize: u32, lreadtimeout: u32, lpsecurityattributes: *const ::win32_security::SECURITY_ATTRIBUTES) -> ::win32_foundation::HANDLE;
        }
        let result__ = CreateMailslotA(lpname.into_param().abi(), ::core::mem::transmute(nmaxmessagesize), ::core::mem::transmute(lreadtimeout), ::core::mem::transmute(lpsecurityattributes));
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows_core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-security")]
#[inline]
pub unsafe fn CreateMailslotW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(lpname: Param0, nmaxmessagesize: u32, lreadtimeout: u32, lpsecurityattributes: *const ::win32_security::SECURITY_ATTRIBUTES) -> ::windows_core::Result<::win32_foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateMailslotW(lpname: ::windows_core::PCWSTR, nmaxmessagesize: u32, lreadtimeout: u32, lpsecurityattributes: *const ::win32_security::SECURITY_ATTRIBUTES) -> ::win32_foundation::HANDLE;
        }
        let result__ = CreateMailslotW(lpname.into_param().abi(), ::core::mem::transmute(nmaxmessagesize), ::core::mem::transmute(lreadtimeout), ::core::mem::transmute(lpsecurityattributes));
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows_core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetMailslotInfo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hmailslot: Param0, lpmaxmessagesize: *mut u32, lpnextsize: *mut u32, lpmessagecount: *mut u32, lpreadtimeout: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMailslotInfo(hmailslot: ::win32_foundation::HANDLE, lpmaxmessagesize: *mut u32, lpnextsize: *mut u32, lpmessagecount: *mut u32, lpreadtimeout: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetMailslotInfo(hmailslot.into_param().abi(), ::core::mem::transmute(lpmaxmessagesize), ::core::mem::transmute(lpnextsize), ::core::mem::transmute(lpmessagecount), ::core::mem::transmute(lpreadtimeout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetMailslotInfo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hmailslot: Param0, lreadtimeout: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetMailslotInfo(hmailslot: ::win32_foundation::HANDLE, lreadtimeout: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetMailslotInfo(hmailslot.into_param().abi(), ::core::mem::transmute(lreadtimeout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
