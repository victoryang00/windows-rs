#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CeipIsOptedIn() -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CeipIsOptedIn() -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CeipIsOptedIn())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
