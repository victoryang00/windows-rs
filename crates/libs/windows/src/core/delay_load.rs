use super::*;
use windows_sys::Win32::System::LibraryLoader::*;

/// Load a function from a given library.
///
/// This is a small wrapper around `LoadLibrary` and `GetProcAddress`.
///
/// # Safety
///
/// * Both the library and function names must be valid PCSTR representations
pub unsafe fn delay_load(library: &[u8], function: &[u8]) -> Result<*mut core::ffi::c_void> {
    let library = LoadLibraryA(library.as_ptr());
    if library == 0 {
        return Err(Error::from_win32());
    }
    if let Some(address) = GetProcAddress(library, function.as_ptr()) {
        Ok(address as _)
    } else {
        FreeLibrary(library);
        Err(Error::from_win32())
    }
}
