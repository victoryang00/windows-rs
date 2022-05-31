#[repr(transparent)]
pub struct IWebApplicationActivation(::windows_core::IUnknown);
impl IWebApplicationActivation {
    pub unsafe fn CancelPendingActivation(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CancelPendingActivation)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWebApplicationActivation> for ::windows_core::IUnknown {
    fn from(value: IWebApplicationActivation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebApplicationActivation> for ::windows_core::IUnknown {
    fn from(value: &IWebApplicationActivation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWebApplicationActivation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWebApplicationActivation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWebApplicationActivation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebApplicationActivation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebApplicationActivation {}
impl ::core::fmt::Debug for IWebApplicationActivation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebApplicationActivation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWebApplicationActivation {
    type Vtable = IWebApplicationActivation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbcdcd0de_330e_481b_b843_4898a6a8ebac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebApplicationActivation_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CancelPendingActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWebApplicationAuthoringMode(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWebApplicationAuthoringMode {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryService(&self, guidservice: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.QueryService)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidservice), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobject)).ok()
    }
    pub unsafe fn AuthoringClientBinary(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).AuthoringClientBinary)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWebApplicationAuthoringMode> for ::windows_core::IUnknown {
    fn from(value: IWebApplicationAuthoringMode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWebApplicationAuthoringMode> for ::windows_core::IUnknown {
    fn from(value: &IWebApplicationAuthoringMode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWebApplicationAuthoringMode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWebApplicationAuthoringMode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWebApplicationAuthoringMode> for super::super::super::Com::IServiceProvider {
    fn from(value: IWebApplicationAuthoringMode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWebApplicationAuthoringMode> for super::super::super::Com::IServiceProvider {
    fn from(value: &IWebApplicationAuthoringMode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Com::IServiceProvider> for IWebApplicationAuthoringMode {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Com::IServiceProvider> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Com::IServiceProvider> for &'a IWebApplicationAuthoringMode {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Com::IServiceProvider> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWebApplicationAuthoringMode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWebApplicationAuthoringMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWebApplicationAuthoringMode {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWebApplicationAuthoringMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebApplicationAuthoringMode").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWebApplicationAuthoringMode {
    type Vtable = IWebApplicationAuthoringMode_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x720aea93_1964_4db0_b005_29eb9e2b18a9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWebApplicationAuthoringMode_Vtbl {
    pub base__: super::super::super::Com::IServiceProvider_Vtbl,
    pub AuthoringClientBinary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, designmodedllpath: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWebApplicationHost(::windows_core::IUnknown);
impl IWebApplicationHost {
    pub unsafe fn HWND(&self, hwnd: *mut ::win32_foundation::HWND) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).HWND)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hwnd)).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub unsafe fn Document(&self) -> ::windows_core::Result<::win32_web::MsHtml::IHTMLDocument2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Document)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_web::MsHtml::IHTMLDocument2>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Advise<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, interfaceid: *const ::windows_core::GUID, callback: Param1, cookie: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Advise)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(interfaceid), callback.into_param().abi(), ::core::mem::transmute(cookie)).ok()
    }
    pub unsafe fn Unadvise(&self, cookie: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Unadvise)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cookie)).ok()
    }
}
impl ::core::convert::From<IWebApplicationHost> for ::windows_core::IUnknown {
    fn from(value: IWebApplicationHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebApplicationHost> for ::windows_core::IUnknown {
    fn from(value: &IWebApplicationHost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWebApplicationHost {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWebApplicationHost {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWebApplicationHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebApplicationHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebApplicationHost {}
impl ::core::fmt::Debug for IWebApplicationHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebApplicationHost").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWebApplicationHost {
    type Vtable = IWebApplicationHost_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcecbd2c3_a3a5_4749_9681_20e9161c6794);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebApplicationHost_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub HWND: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: *mut ::win32_foundation::HWND) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub Document: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, htmldocument: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml")))]
    Document: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaceid: *const ::windows_core::GUID, callback: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows_core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWebApplicationNavigationEvents(::windows_core::IUnknown);
impl IWebApplicationNavigationEvents {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub unsafe fn BeforeNavigate<'a, Param0: ::windows_core::IntoParam<'a, ::win32_web::MsHtml::IHTMLWindow2>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, htmlwindow: Param0, url: Param1, navigationflags: u32, targetframename: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeforeNavigate)(::windows_core::Interface::as_raw(self), htmlwindow.into_param().abi(), url.into_param().abi(), ::core::mem::transmute(navigationflags), targetframename.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub unsafe fn NavigateComplete<'a, Param0: ::windows_core::IntoParam<'a, ::win32_web::MsHtml::IHTMLWindow2>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, htmlwindow: Param0, url: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NavigateComplete)(::windows_core::Interface::as_raw(self), htmlwindow.into_param().abi(), url.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub unsafe fn NavigateError<'a, Param0: ::windows_core::IntoParam<'a, ::win32_web::MsHtml::IHTMLWindow2>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, htmlwindow: Param0, url: Param1, targetframename: Param2, statuscode: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NavigateError)(::windows_core::Interface::as_raw(self), htmlwindow.into_param().abi(), url.into_param().abi(), targetframename.into_param().abi(), ::core::mem::transmute(statuscode)).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub unsafe fn DocumentComplete<'a, Param0: ::windows_core::IntoParam<'a, ::win32_web::MsHtml::IHTMLWindow2>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, htmlwindow: Param0, url: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DocumentComplete)(::windows_core::Interface::as_raw(self), htmlwindow.into_param().abi(), url.into_param().abi()).ok()
    }
    pub unsafe fn DownloadBegin(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DownloadBegin)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DownloadComplete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DownloadComplete)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWebApplicationNavigationEvents> for ::windows_core::IUnknown {
    fn from(value: IWebApplicationNavigationEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebApplicationNavigationEvents> for ::windows_core::IUnknown {
    fn from(value: &IWebApplicationNavigationEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWebApplicationNavigationEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWebApplicationNavigationEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWebApplicationNavigationEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebApplicationNavigationEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebApplicationNavigationEvents {}
impl ::core::fmt::Debug for IWebApplicationNavigationEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebApplicationNavigationEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWebApplicationNavigationEvents {
    type Vtable = IWebApplicationNavigationEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc22615d2_d318_4da2_8422_1fcaf77b10e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebApplicationNavigationEvents_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub BeforeNavigate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, htmlwindow: ::windows_core::RawPtr, url: ::windows_core::PCWSTR, navigationflags: u32, targetframename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml")))]
    BeforeNavigate: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub NavigateComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, htmlwindow: ::windows_core::RawPtr, url: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml")))]
    NavigateComplete: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub NavigateError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, htmlwindow: ::windows_core::RawPtr, url: ::windows_core::PCWSTR, targetframename: ::windows_core::PCWSTR, statuscode: u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml")))]
    NavigateError: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub DocumentComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, htmlwindow: ::windows_core::RawPtr, url: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml")))]
    DocumentComplete: usize,
    pub DownloadBegin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DownloadComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWebApplicationScriptEvents(::windows_core::IUnknown);
impl IWebApplicationScriptEvents {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub unsafe fn BeforeScriptExecute<'a, Param0: ::windows_core::IntoParam<'a, ::win32_web::MsHtml::IHTMLWindow2>>(&self, htmlwindow: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeforeScriptExecute)(::windows_core::Interface::as_raw(self), htmlwindow.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub unsafe fn ScriptError<'a, Param0: ::windows_core::IntoParam<'a, ::win32_web::MsHtml::IHTMLWindow2>, Param1: ::windows_core::IntoParam<'a, super::IActiveScriptError>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, htmlwindow: Param0, scripterror: Param1, url: Param2, errorhandled: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ScriptError)(::windows_core::Interface::as_raw(self), htmlwindow.into_param().abi(), scripterror.into_param().abi(), url.into_param().abi(), errorhandled.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWebApplicationScriptEvents> for ::windows_core::IUnknown {
    fn from(value: IWebApplicationScriptEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebApplicationScriptEvents> for ::windows_core::IUnknown {
    fn from(value: &IWebApplicationScriptEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWebApplicationScriptEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWebApplicationScriptEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWebApplicationScriptEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebApplicationScriptEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebApplicationScriptEvents {}
impl ::core::fmt::Debug for IWebApplicationScriptEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebApplicationScriptEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWebApplicationScriptEvents {
    type Vtable = IWebApplicationScriptEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7c3f6998_1567_4bba_b52b_48d32141d613);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebApplicationScriptEvents_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub BeforeScriptExecute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, htmlwindow: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml")))]
    BeforeScriptExecute: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub ScriptError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, htmlwindow: ::windows_core::RawPtr, scripterror: ::windows_core::RawPtr, url: ::windows_core::PCWSTR, errorhandled: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml")))]
    ScriptError: usize,
}
#[repr(transparent)]
pub struct IWebApplicationUIEvents(::windows_core::IUnknown);
impl IWebApplicationUIEvents {
    pub unsafe fn SecurityProblem(&self, securityproblem: u32, result: *mut ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SecurityProblem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(securityproblem), ::core::mem::transmute(result)).ok()
    }
}
impl ::core::convert::From<IWebApplicationUIEvents> for ::windows_core::IUnknown {
    fn from(value: IWebApplicationUIEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebApplicationUIEvents> for ::windows_core::IUnknown {
    fn from(value: &IWebApplicationUIEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWebApplicationUIEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWebApplicationUIEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWebApplicationUIEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebApplicationUIEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebApplicationUIEvents {}
impl ::core::fmt::Debug for IWebApplicationUIEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebApplicationUIEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWebApplicationUIEvents {
    type Vtable = IWebApplicationUIEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5b2b3f99_328c_41d5_a6f7_7483ed8e71dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebApplicationUIEvents_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SecurityProblem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, securityproblem: u32, result: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWebApplicationUpdateEvents(::windows_core::IUnknown);
impl IWebApplicationUpdateEvents {
    pub unsafe fn OnPaint(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnPaint)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnCssChanged(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnCssChanged)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWebApplicationUpdateEvents> for ::windows_core::IUnknown {
    fn from(value: IWebApplicationUpdateEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebApplicationUpdateEvents> for ::windows_core::IUnknown {
    fn from(value: &IWebApplicationUpdateEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWebApplicationUpdateEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWebApplicationUpdateEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWebApplicationUpdateEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebApplicationUpdateEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebApplicationUpdateEvents {}
impl ::core::fmt::Debug for IWebApplicationUpdateEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebApplicationUpdateEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWebApplicationUpdateEvents {
    type Vtable = IWebApplicationUpdateEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3e59e6b7_c652_4daf_ad5e_16feb350cde3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebApplicationUpdateEvents_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnPaint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OnCssChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub type RegisterAuthoringClientFunctionType = ::core::option::Option<unsafe extern "system" fn(authoringmodeobject: ::core::option::Option<IWebApplicationAuthoringMode>, host: ::core::option::Option<IWebApplicationHost>) -> ::windows_core::HRESULT>;
pub type UnregisterAuthoringClientFunctionType = ::core::option::Option<unsafe extern "system" fn(host: ::core::option::Option<IWebApplicationHost>) -> ::windows_core::HRESULT>;
