#[inline]
pub unsafe fn AcquireDeveloperLicense<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwndparent: Param0) -> ::windows_core::Result<::win32_foundation::FILETIME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AcquireDeveloperLicense(hwndparent: ::win32_foundation::HWND, pexpiration: *mut ::win32_foundation::FILETIME) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::FILETIME>::zeroed();
        AcquireDeveloperLicense(hwndparent.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::FILETIME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CheckDeveloperLicense() -> ::windows_core::Result<::win32_foundation::FILETIME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckDeveloperLicense(pexpiration: *mut ::win32_foundation::FILETIME) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::FILETIME>::zeroed();
        CheckDeveloperLicense(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::FILETIME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RemoveDeveloperLicense<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwndparent: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RemoveDeveloperLicense(hwndparent: ::win32_foundation::HWND) -> ::windows_core::HRESULT;
        }
        RemoveDeveloperLicense(hwndparent.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
