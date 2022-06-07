use super::*;
use windows_sys::core::Interface;
use windows_sys::Win32::System::Com::{GetErrorInfo,IErrorInfo, SetErrorInfo};
use windows_sys::Win32::System::WinRT::{ILanguageExceptionErrorInfo2, IRestrictedErrorInfo};
use windows_sys::Win32::Foundation::{GetLastError, S_OK, SysFreeString, BOOL, SysStringLen};

/// An error object consists of both an error code as well as detailed error information for debugging.
#[derive(Clone, PartialEq)]
pub struct Error {
    pub(crate) code: HRESULT,
    pub(crate) info: ComPtr<IRestrictedErrorInfo>,
}

impl Error {
    /// An error object without any failure information.
    pub const OK: Self = Self { code: HRESULT(S_OK), info: ComPtr::null() };

    /// This creates a new WinRT error object, capturing the stack and other information about the
    /// point of failure.
    pub fn new(code: HRESULT, message: HSTRING) -> Self {
        unsafe {
            if let Ok(function) = delay_load(b"combase.dll\0", b"RoOriginateError\0") {
                let function: RoOriginateError = core::mem::transmute(function);
                function(code, core::mem::transmute_copy(&message));
            }
            let mut first = ComPtr::<IErrorInfo>::null();
            GetErrorInfo(0, first.put());
            let mut info = ComPtr::<IRestrictedErrorInfo>::null();
            if !first.is_null() {
                // TODO: wrap in ComPtr?
                (first.get().base__.QueryInterface)(first.this() as _, &IRestrictedErrorInfo::IID, info.put_void() as _);
            }
            Self { code, info }
        }
    }

    pub fn from_win32() -> Self {
        let win32_error = GetLastError();
        let code = if win32_error == 0 { HRESULT(S_OK) } else { HRESULT(((win32_error & 0x0000_FFFF) | (7 << 16) | 0x8000_0000) as _) };
        unsafe { Self { code, info: ComPtr::null() } }
    }

    /// The error code describing the error.
    pub const fn code(&self) -> HRESULT {
        self.code
    }

    /// The error information describing the error.
    pub const fn info(&self) -> &Option<IUnknown> {
        std::mem::transmute(&self.info)
    }

    /// The error message describing the error.
    pub fn message(&self) -> HSTRING {
        // First attempt to retrieve the restricted error information.
        if !self.info.is_null(){
            let mut fallback = BSTR::new();
            let mut message = BSTR::new();
            let mut unused = BSTR::new();
            let mut code = HRESULT(0);

            unsafe {
                (self.info.get().GetErrorDetails)(self.info.this(), fallback.put(), &mut code.0, message.put(), unused.put());
            }

            if self.code == code {
                let message = if !message.is_empty() { message } else { fallback };
                return HSTRING::from_wide(wide_trim_end(message.as_wide()));
            }
        }

        self.code.message()
    }

    /// Returns the win32 error code if the underlying HRESULT's facility is win32
    #[cfg(feature = "Win32_Foundation")]
    pub fn win32_error(&self) -> Option<crate::Win32::Foundation::WIN32_ERROR> {
        let hresult = self.code.0 as u32;
        if ((hresult >> 16) & 0x7FF) == 7 {
            Some(crate::Win32::Foundation::WIN32_ERROR(hresult & 0xFFFF))
        } else {
            None
        }
    }
}

impl core::convert::From<Error> for HRESULT {
    fn from(error: Error) -> Self {
        let code = error.code;
        let info = error.info.and_then(|info| info.cast().ok());

        unsafe {
            let _ = SetErrorInfo(0, info);
        }

        code
    }
}

impl core::convert::From<Error> for std::io::Error {
    fn from(from: Error) -> Self {
        Self::from_raw_os_error((from.code.0 & 0xFFFF) as _)
    }
}

impl core::convert::From<HRESULT> for Error {
    fn from(code: HRESULT) -> Self {
        let info: Option<IRestrictedErrorInfo> = unsafe { GetErrorInfo(0).and_then(|e| e.cast()).ok() };

        if let Some(info) = info {
            // If it does (and therefore running on a recent version of Windows)
            // then capture_propagation_context adds a breadcrumb to the error
            // info to make debugging easier.
            if let Ok(capture) = info.cast::<ILanguageExceptionErrorInfo2>() {
                unsafe {
                    let _ = capture.CapturePropagationContext(None);
                }
            }

            return Self { code, info: Some(info) };
        }

        if let Ok(info) = unsafe { GetErrorInfo(0) } {
            let message = unsafe { info.GetDescription().unwrap_or_default() };
            Self::new(code, HSTRING::from_wide(message.as_wide()))
        } else {
            Self { code, info: None }
        }
    }
}

impl core::fmt::Debug for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut debug = fmt.debug_struct("Error");
        debug.field("code", &format_args!("{:#010X}", self.code.0)).field("message", &self.message()).finish()
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::write!(fmt, "{}", self.message())
    }
}

impl std::error::Error for Error {}

type RoOriginateError = extern "system" fn(code: HRESULT, message: core::mem::ManuallyDrop<HSTRING>) -> BOOL;

struct BSTR(*mut u16);

impl BSTR {
    pub fn new() -> Self {
        Self(std::ptr::null_mut())
    }
    pub unsafe fn put(&mut self) -> *mut *mut u16 {
        debug_assert!(self.0.is_null());
        &mut self
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn len(&self) -> usize {
        if self.0.is_null() {
            0
        } else {
            unsafe { SysStringLen(self) as usize }
        }
    }
    pub fn as_wide(&self) -> &[u16] {
        if self.0.is_null() {
            return &[];
        }

        unsafe { std::slice::from_raw_parts(self.0, self.len()) }
    }
}

impl Drop for BSTR {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { SysFreeString(self as &Self) }
        }
    }
}