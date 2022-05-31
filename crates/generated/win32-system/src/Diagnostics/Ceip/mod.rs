#[inline]
pub unsafe fn CeipIsOptedIn() -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CeipIsOptedIn() -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CeipIsOptedIn())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
