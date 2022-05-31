#[link(name = "windows")]
extern "system" {
    pub fn CeipIsOptedIn() -> ::win32_foundation_sys::BOOL;
}
