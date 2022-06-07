#[cfg(feature = "Win32_System_WinRT_AllJoyn")]
pub mod AllJoyn;
#[cfg(feature = "Win32_System_WinRT_Composition")]
pub mod Composition;
#[cfg(feature = "Win32_System_WinRT_CoreInputView")]
pub mod CoreInputView;
#[cfg(feature = "Win32_System_WinRT_Direct3D11")]
pub mod Direct3D11;
#[cfg(feature = "Win32_System_WinRT_Display")]
pub mod Display;
#[cfg(feature = "Win32_System_WinRT_Graphics")]
pub mod Graphics;
#[cfg(feature = "Win32_System_WinRT_Holographic")]
pub mod Holographic;
#[cfg(feature = "Win32_System_WinRT_Isolation")]
pub mod Isolation;
#[cfg(feature = "Win32_System_WinRT_ML")]
pub mod ML;
#[cfg(feature = "Win32_System_WinRT_Media")]
pub mod Media;
#[cfg(feature = "Win32_System_WinRT_Pdf")]
pub mod Pdf;
#[cfg(feature = "Win32_System_WinRT_Printing")]
pub mod Printing;
#[cfg(feature = "Win32_System_WinRT_Shell")]
pub mod Shell;
#[cfg(feature = "Win32_System_WinRT_Storage")]
pub mod Storage;
#[cfg(feature = "Win32_System_WinRT_Xaml")]
pub mod Xaml;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn CoDecodeProxy(dwclientpid: u32, ui64proxyaddress: u64, pserverinformation: *mut ServerInformation) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn CreateControlInput(riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn CreateControlInputEx(pcorewindow: *mut *mut ::windows_sys::core::IUnknown, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn CreateDispatcherQueueController(options: DispatcherQueueOptions, dispatcherqueuecontroller: *mut *mut *mut super::super::super::System::DispatcherQueueController) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn CreateRandomAccessStreamOnFile(filepath: ::windows_sys::core::PCWSTR, accessmode: u32, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn CreateRandomAccessStreamOverStream(stream: *mut *mut super::Com::IStream, options: BSOS_OPTIONS, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn CreateStreamOverRandomAccessStream(randomaccessstream: *mut *mut ::windows_sys::core::IUnknown, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn GetRestrictedErrorInfo(pprestrictederrorinfo: *mut *mut *mut IRestrictedErrorInfo) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn HSTRING_UserFree(param0: *const u32, param1: *const ::windows_sys::core::HSTRING);
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn HSTRING_UserFree64(param0: *const u32, param1: *const ::windows_sys::core::HSTRING);
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn HSTRING_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::windows_sys::core::HSTRING) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn HSTRING_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::windows_sys::core::HSTRING) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn HSTRING_UserSize(param0: *const u32, param1: u32, param2: *const ::windows_sys::core::HSTRING) -> u32;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn HSTRING_UserSize64(param0: *const u32, param1: u32, param2: *const ::windows_sys::core::HSTRING) -> u32;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn HSTRING_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::windows_sys::core::HSTRING) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn HSTRING_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::windows_sys::core::HSTRING) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsErrorPropagationEnabled() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn MetaDataGetDispenser(rclsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn RoActivateInstance(activatableclassid: ::windows_sys::core::HSTRING, instance: *mut *mut *mut ::windows_sys::core::IInspectable) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn RoCaptureErrorContext(hr: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn RoClearError();
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn RoFailFastWithErrorContext(hrerror: ::windows_sys::core::HRESULT);
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn RoFreeParameterizedTypeExtra(extra: ROPARAMIIDHANDLE);
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn RoGetActivationFactory(activatableclassid: ::windows_sys::core::HSTRING, iid: *const ::windows_sys::core::GUID, factory: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn RoGetAgileReference(options: AgileReferenceOptions, riid: *const ::windows_sys::core::GUID, punk: *mut *mut ::windows_sys::core::IUnknown, ppagilereference: *mut *mut *mut IAgileReference) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn RoGetApartmentIdentifier(apartmentidentifier: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_System_Com_Marshal\"`*"]
    #[cfg(feature = "Win32_System_Com_Marshal")]
    pub fn RoGetBufferMarshaler(buffermarshaler: *mut *mut *mut super::Com::Marshal::IMarshal) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn RoGetErrorReportingFlags(pflags: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn RoGetMatchingRestrictedErrorInfo(hrin: ::windows_sys::core::HRESULT, pprestrictederrorinfo: *mut *mut *mut IRestrictedErrorInfo) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn RoGetParameterizedTypeInstanceIID(nameelementcount: u32, nameelements: *const ::windows_sys::core::PWSTR, metadatalocator: *mut *mut IRoMetaDataLocator, iid: *mut ::windows_sys::core::GUID, pextra: *mut ROPARAMIIDHANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn RoGetServerActivatableClasses(servername: ::windows_sys::core::HSTRING, activatableclassids: *mut *mut ::windows_sys::core::HSTRING, count: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn RoInitialize(inittype: RO_INIT_TYPE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn RoInspectCapturedStackBackTrace(targeterrorinfoaddress: usize, machine: u16, readmemorycallback: PINSPECT_MEMORY_CALLBACK, context: *const ::core::ffi::c_void, framecount: *mut u32, targetbacktraceaddress: *mut usize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn RoInspectThreadErrorInfo(targettebaddress: usize, machine: u16, readmemorycallback: PINSPECT_MEMORY_CALLBACK, context: *const ::core::ffi::c_void, targeterrorinfoaddress: *mut usize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoOriginateError(error: ::windows_sys::core::HRESULT, message: ::windows_sys::core::HSTRING) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoOriginateErrorW(error: ::windows_sys::core::HRESULT, cchmax: u32, message: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoOriginateLanguageException(error: ::windows_sys::core::HRESULT, message: ::windows_sys::core::HSTRING, languageexception: *mut *mut ::windows_sys::core::IUnknown) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn RoParameterizedTypeExtraGetTypeSignature(extra: ROPARAMIIDHANDLE) -> ::windows_sys::core::PSTR;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn RoRegisterActivationFactories(activatableclassids: *const ::windows_sys::core::HSTRING, activationfactorycallbacks: *const isize, count: u32, cookie: *mut isize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn RoRegisterForApartmentShutdown(callbackobject: *mut *mut IApartmentShutdown, apartmentidentifier: *mut u64, regcookie: *mut APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn RoReportFailedDelegate(punkdelegate: *mut *mut ::windows_sys::core::IUnknown, prestrictederrorinfo: *mut *mut IRestrictedErrorInfo) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn RoReportUnhandledError(prestrictederrorinfo: *mut *mut IRestrictedErrorInfo) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn RoResolveRestrictedErrorInfoReference(reference: ::windows_sys::core::PCWSTR, pprestrictederrorinfo: *mut *mut *mut IRestrictedErrorInfo) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn RoRevokeActivationFactories(cookie: isize);
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn RoSetErrorReportingFlags(flags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoTransformError(olderror: ::windows_sys::core::HRESULT, newerror: ::windows_sys::core::HRESULT, message: ::windows_sys::core::HSTRING) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoTransformErrorW(olderror: ::windows_sys::core::HRESULT, newerror: ::windows_sys::core::HRESULT, cchmax: u32, message: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn RoUninitialize();
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn RoUnregisterForApartmentShutdown(regcookie: APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn SetRestrictedErrorInfo(prestrictederrorinfo: *mut *mut IRestrictedErrorInfo) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn WindowsCompareStringOrdinal(string1: ::windows_sys::core::HSTRING, string2: ::windows_sys::core::HSTRING, result: *mut i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn WindowsConcatString(string1: ::windows_sys::core::HSTRING, string2: ::windows_sys::core::HSTRING, newstring: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn WindowsCreateString(sourcestring: ::windows_sys::core::PCWSTR, length: u32, string: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn WindowsCreateStringReference(sourcestring: ::windows_sys::core::PCWSTR, length: u32, hstringheader: *mut HSTRING_HEADER, string: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn WindowsDeleteString(string: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn WindowsDeleteStringBuffer(bufferhandle: HSTRING_BUFFER) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn WindowsDuplicateString(string: ::windows_sys::core::HSTRING, newstring: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn WindowsGetStringLen(string: ::windows_sys::core::HSTRING) -> u32;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn WindowsGetStringRawBuffer(string: ::windows_sys::core::HSTRING, length: *mut u32) -> ::windows_sys::core::PWSTR;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn WindowsInspectString(targethstring: usize, machine: u16, callback: PINSPECT_HSTRING_CALLBACK, context: *const ::core::ffi::c_void, length: *mut u32, targetstringaddress: *mut usize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn WindowsInspectString2(targethstring: u64, machine: u16, callback: PINSPECT_HSTRING_CALLBACK2, context: *const ::core::ffi::c_void, length: *mut u32, targetstringaddress: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowsIsStringEmpty(string: ::windows_sys::core::HSTRING) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn WindowsPreallocateStringBuffer(length: u32, charbuffer: *mut *mut u16, bufferhandle: *mut HSTRING_BUFFER) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn WindowsPromoteStringBuffer(bufferhandle: HSTRING_BUFFER, string: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn WindowsReplaceString(string: ::windows_sys::core::HSTRING, stringreplaced: ::windows_sys::core::HSTRING, stringreplacewith: ::windows_sys::core::HSTRING, newstring: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowsStringHasEmbeddedNull(string: ::windows_sys::core::HSTRING, hasembednull: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn WindowsSubstring(string: ::windows_sys::core::HSTRING, startindex: u32, newstring: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn WindowsSubstringWithSpecifiedLength(string: ::windows_sys::core::HSTRING, startindex: u32, length: u32, newstring: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn WindowsTrimStringEnd(string: ::windows_sys::core::HSTRING, trimstring: ::windows_sys::core::HSTRING, newstring: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
    pub fn WindowsTrimStringStart(string: ::windows_sys::core::HSTRING, trimstring: ::windows_sys::core::HSTRING, newstring: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT;
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub type ACTIVATIONTYPE = i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const ACTIVATIONTYPE_UNCATEGORIZED: ACTIVATIONTYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const ACTIVATIONTYPE_FROM_MONIKER: ACTIVATIONTYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const ACTIVATIONTYPE_FROM_DATA: ACTIVATIONTYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const ACTIVATIONTYPE_FROM_STORAGE: ACTIVATIONTYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const ACTIVATIONTYPE_FROM_STREAM: ACTIVATIONTYPE = 8i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const ACTIVATIONTYPE_FROM_FILE: ACTIVATIONTYPE = 16i32;
pub type APARTMENT_SHUTDOWN_REGISTRATION_COOKIE = isize;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub type AgileReferenceOptions = i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const AGILEREFERENCE_DEFAULT: AgileReferenceOptions = 0i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const AGILEREFERENCE_DELAYEDMARSHAL: AgileReferenceOptions = 1i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub type BSOS_OPTIONS = i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const BSOS_DEFAULT: BSOS_OPTIONS = 0i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const BSOS_PREFERDESTINATIONSTREAM: BSOS_OPTIONS = 1i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub type CASTING_CONNECTION_ERROR_STATUS = i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_ERROR_STATUS_SUCCEEDED: CASTING_CONNECTION_ERROR_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_ERROR_STATUS_DEVICE_DID_NOT_RESPOND: CASTING_CONNECTION_ERROR_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_ERROR_STATUS_DEVICE_ERROR: CASTING_CONNECTION_ERROR_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_ERROR_STATUS_DEVICE_LOCKED: CASTING_CONNECTION_ERROR_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_ERROR_STATUS_PROTECTED_PLAYBACK_FAILED: CASTING_CONNECTION_ERROR_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_ERROR_STATUS_INVALID_CASTING_SOURCE: CASTING_CONNECTION_ERROR_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_ERROR_STATUS_UNKNOWN: CASTING_CONNECTION_ERROR_STATUS = 6i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub type CASTING_CONNECTION_STATE = i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_STATE_DISCONNECTED: CASTING_CONNECTION_STATE = 0i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_STATE_CONNECTED: CASTING_CONNECTION_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_STATE_RENDERING: CASTING_CONNECTION_STATE = 2i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_STATE_DISCONNECTING: CASTING_CONNECTION_STATE = 3i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CASTING_CONNECTION_STATE_CONNECTING: CASTING_CONNECTION_STATE = 4i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CastingSourceInfo_Property_CastingTypes: &str = "CastingTypes";
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CastingSourceInfo_Property_PreferredSourceUriScheme: &str = "PreferredSourceUriScheme";
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const CastingSourceInfo_Property_ProtectedMedia: &str = "ProtectedMedia";
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub type DISPATCHERQUEUE_THREAD_APARTMENTTYPE = i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const DQTAT_COM_NONE: DISPATCHERQUEUE_THREAD_APARTMENTTYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const DQTAT_COM_ASTA: DISPATCHERQUEUE_THREAD_APARTMENTTYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const DQTAT_COM_STA: DISPATCHERQUEUE_THREAD_APARTMENTTYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub type DISPATCHERQUEUE_THREAD_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const DQTYPE_THREAD_DEDICATED: DISPATCHERQUEUE_THREAD_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const DQTYPE_THREAD_CURRENT: DISPATCHERQUEUE_THREAD_TYPE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub struct DispatcherQueueOptions {
    pub dwSize: u32,
    pub threadType: DISPATCHERQUEUE_THREAD_TYPE,
    pub apartmentType: DISPATCHERQUEUE_THREAD_APARTMENTTYPE,
}
impl ::core::marker::Copy for DispatcherQueueOptions {}
impl ::core::clone::Clone for DispatcherQueueOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub struct EventRegistrationToken {
    pub value: i64,
}
impl ::core::marker::Copy for EventRegistrationToken {}
impl ::core::clone::Clone for EventRegistrationToken {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HSTRING_BUFFER = isize;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub struct HSTRING_HEADER {
    pub flags: u32,
    pub length: u32,
    pub padding1: u32,
    pub padding2: u32,
    pub data: isize,
}
impl ::core::marker::Copy for HSTRING_HEADER {}
impl ::core::clone::Clone for HSTRING_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IAccountsSettingsPaneInterop {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut *mut Self, appwindow: super::super::Foundation::HWND, riid: *const ::windows_sys::core::GUID, accountssettingspane: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowManageAccountsForWindowAsync: unsafe extern "system" fn(this: *mut *mut Self, appwindow: super::super::Foundation::HWND, riid: *const ::windows_sys::core::GUID, asyncaction: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowManageAccountsForWindowAsync: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowAddAccountForWindowAsync: unsafe extern "system" fn(this: *mut *mut Self, appwindow: super::super::Foundation::HWND, riid: *const ::windows_sys::core::GUID, asyncaction: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowAddAccountForWindowAsync: usize,
}
impl ::windows_sys::core::Interface for IAccountsSettingsPaneInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3555594925, data2: 14437, data3: 17250, data4: [151, 70, 183, 90, 104, 45, 240, 230] };
}
#[repr(C)]
pub struct IActivationFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub ActivateInstance: unsafe extern "system" fn(this: *mut *mut Self, instance: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IActivationFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 53, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IAgileReference {
    pub base__: ::windows_sys::core::IUnknown,
    pub Resolve: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppvobjectreference: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAgileReference {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3225381443, data2: 26020, data3: 38936, data4: [152, 126, 224, 184, 16, 210, 166, 242] };
}
#[repr(C)]
pub struct IApartmentShutdown {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnUninitialize: unsafe extern "system" fn(this: *mut *mut Self, ui64apartmentidentifier: u64),
}
impl ::windows_sys::core::Interface for IApartmentShutdown {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2733660681, data2: 10146, data3: 17077, data4: [188, 14, 172, 22, 62, 244, 157, 155] };
}
#[repr(C)]
pub struct IAppServiceConnectionExtendedExecution {
    pub base__: ::windows_sys::core::IUnknown,
    pub OpenForExtendedExecutionAsync: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, operation: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppServiceConnectionExtendedExecution {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1696699780, data2: 63947, data3: 19171, data4: [129, 249, 162, 138, 108, 164, 80, 217] };
}
#[repr(C)]
pub struct IBufferByteAccess {
    pub base__: ::windows_sys::core::IUnknown,
    pub Buffer: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBufferByteAccess {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2421821423, data2: 48211, data3: 4575, data4: [140, 73, 0, 30, 79, 198, 134, 218] };
}
#[repr(C)]
pub struct ICastingController {
    pub base__: ::windows_sys::core::IUnknown,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, castingengine: *mut ::core::ffi::c_void, castingsource: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Connect: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Advise: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows_sys::core::HRESULT,
    pub UnAdvise: unsafe extern "system" fn(this: *mut *mut Self, cookie: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICastingController {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4037370915, data2: 42596, data3: 20413, data4: [139, 67, 64, 154, 69, 232, 217, 161] };
}
#[repr(C)]
pub struct ICastingEventHandler {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnStateChanged: unsafe extern "system" fn(this: *mut *mut Self, newstate: CASTING_CONNECTION_STATE) -> ::windows_sys::core::HRESULT,
    pub OnError: unsafe extern "system" fn(this: *mut *mut Self, errorstatus: CASTING_CONNECTION_ERROR_STATUS, errormessage: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICastingEventHandler {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3348786359, data2: 48829, data3: 18342, data4: [162, 173, 77, 69, 173, 121, 199, 188] };
}
#[repr(C)]
pub struct ICastingSourceInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetController: unsafe extern "system" fn(this: *mut *mut Self, controller: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetProperties: unsafe extern "system" fn(this: *mut *mut Self, props: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetProperties: usize,
}
impl ::windows_sys::core::Interface for ICastingSourceInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1158683319, data2: 31802, data3: 19406, data4: [149, 0, 18, 192, 144, 36, 178, 152] };
}
#[repr(C)]
pub struct ICoreInputInterop {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetInputSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetMessageHandled: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreInputInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1086317539, data2: 46938, data3: 17529, data4: [172, 150, 71, 83, 101, 116, 155, 184] };
}
#[repr(C)]
pub struct ICoreWindowAdapterInterop {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppActivationClientAdapter: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ApplicationViewClientAdapter: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CoreApplicationViewClientAdapter: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HoloViewClientAdapter: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PositionerClientAdapter: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SystemNavigationClientAdapter: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TitleBarClientAdapter: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetWindowClientAdapter: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreWindowAdapterInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2052812753, data2: 52595, data3: 19308, data4: [156, 244, 46, 134, 158, 175, 71, 10] };
}
#[repr(C)]
pub struct ICoreWindowComponentInterop {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub ConfigureComponentInput: unsafe extern "system" fn(this: *mut *mut Self, hostviewinstanceid: u32, hwndhost: super::super::Foundation::HWND, inputsourcevisual: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConfigureComponentInput: usize,
    pub GetViewInstanceId: unsafe extern "system" fn(this: *mut *mut Self, componentviewinstanceid: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreWindowComponentInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 91663153, data2: 41744, data3: 19520, data4: [186, 49, 253, 55, 224, 41, 141, 250] };
}
#[repr(C)]
pub struct ICoreWindowInterop {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub WindowHandle: unsafe extern "system" fn(this: *mut *mut Self, hwnd: *mut super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WindowHandle: usize,
    pub SetMessageHandled: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreWindowInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1171671593, data2: 42558, data3: 19638, data4: [180, 152, 87, 129, 210, 152, 203, 79] };
}
#[repr(C)]
pub struct ICorrelationVectorInformation {
    pub base__: ::windows_sys::core::IInspectable,
    pub LastCorrelationVectorForThread: unsafe extern "system" fn(this: *mut *mut Self, cv: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NextCorrelationVectorForThread: unsafe extern "system" fn(this: *mut *mut Self, cv: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetNextCorrelationVectorForThread: unsafe extern "system" fn(this: *mut *mut Self, cv: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICorrelationVectorInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2210892604, data2: 55435, data3: 18768, data4: [170, 110, 34, 184, 210, 42, 171, 211] };
}
#[repr(C)]
pub struct ICorrelationVectorSource {
    pub base__: ::windows_sys::core::IUnknown,
    pub CorrelationVector: unsafe extern "system" fn(this: *mut *mut Self, cv: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICorrelationVectorSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 355174971, data2: 47545, data3: 18053, data4: [181, 110, 151, 72, 71, 188, 117, 69] };
}
#[repr(C)]
pub struct IDragDropManagerInterop {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut *mut Self, hwnd: super::super::Foundation::HWND, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
impl ::windows_sys::core::Interface for IDragDropManagerInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1524157351, data2: 19457, data3: 19884, data4: [144, 116, 130, 120, 148, 41, 45, 99] };
}
#[repr(C)]
pub struct IHolographicSpaceInterop {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateForWindow: unsafe extern "system" fn(this: *mut *mut Self, window: super::super::Foundation::HWND, riid: *const ::windows_sys::core::GUID, holographicspace: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateForWindow: usize,
}
impl ::windows_sys::core::Interface for IHolographicSpaceInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1548674358, data2: 27288, data3: 19334, data4: [161, 112, 88, 112, 19, 214, 253, 75] };
}
#[repr(C)]
pub struct IInputPaneInterop {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut *mut Self, appwindow: super::super::Foundation::HWND, riid: *const ::windows_sys::core::GUID, inputpane: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
impl ::windows_sys::core::Interface for IInputPaneInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1976511575, data2: 37269, data3: 18737, data4: [131, 50, 240, 180, 9, 233, 22, 175] };
}
#[repr(C)]
pub struct ILanguageExceptionErrorInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetLanguageException: unsafe extern "system" fn(this: *mut *mut Self, languageexception: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILanguageExceptionErrorInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 77782003, data2: 57219, data3: 4460, data4: [9, 70, 8, 18, 171, 246, 224, 125] };
}
#[repr(C)]
pub struct ILanguageExceptionErrorInfo2 {
    pub base__: ILanguageExceptionErrorInfo,
    pub GetPreviousLanguageExceptionErrorInfo: unsafe extern "system" fn(this: *mut *mut Self, previouslanguageexceptionerrorinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CapturePropagationContext: unsafe extern "system" fn(this: *mut *mut Self, languageexception: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPropagationContextHead: unsafe extern "system" fn(this: *mut *mut Self, propagatedlanguageexceptionerrorinfohead: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILanguageExceptionErrorInfo2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1464264132, data2: 23447, data3: 16972, data4: [182, 32, 40, 34, 145, 87, 52, 221] };
}
#[repr(C)]
pub struct ILanguageExceptionStackBackTrace {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetStackBackTrace: unsafe extern "system" fn(this: *mut *mut Self, maxframestocapture: u32, stackbacktrace: *mut usize, framescaptured: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILanguageExceptionStackBackTrace {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3420798901, data2: 63847, data3: 16984, data4: [141, 52, 66, 245, 226, 88, 51, 222] };
}
#[repr(C)]
pub struct ILanguageExceptionTransform {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetTransformedRestrictedErrorInfo: unsafe extern "system" fn(this: *mut *mut Self, restrictederrorinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILanguageExceptionTransform {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4273316465, data2: 42701, data3: 17870, data4: [136, 10, 105, 103, 6, 186, 220, 101] };
}
#[repr(C)]
pub struct IMemoryBufferByteAccess {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetBuffer: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut u8, capacity: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMemoryBufferByteAccess {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1527591477, data2: 19898, data3: 19780, data4: [134, 94, 143, 29, 14, 79, 208, 77] };
}
#[repr(C)]
pub struct IMessageDispatcher {
    pub base__: ::windows_sys::core::IInspectable,
    pub PumpMessages: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMessageDispatcher {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4126690447, data2: 53200, data3: 19670, data4: [182, 107, 197, 210, 111, 241, 104, 157] };
}
#[repr(C)]
pub struct IPlayToManagerInterop {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut *mut Self, appwindow: super::super::Foundation::HWND, riid: *const ::windows_sys::core::GUID, playtomanager: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowPlayToUIForWindow: unsafe extern "system" fn(this: *mut *mut Self, appwindow: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowPlayToUIForWindow: usize,
}
impl ::windows_sys::core::Interface for IPlayToManagerInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 607733401, data2: 7980, data3: 20147, data4: [140, 215, 14, 193, 218, 66, 165, 64] };
}
#[repr(C)]
pub struct IRestrictedErrorInfo {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetErrorDetails: unsafe extern "system" fn(this: *mut *mut Self, description: *mut super::super::Foundation::BSTR, error: *mut ::windows_sys::core::HRESULT, restricteddescription: *mut super::super::Foundation::BSTR, capabilitysid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetErrorDetails: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetReference: unsafe extern "system" fn(this: *mut *mut Self, reference: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetReference: usize,
}
impl ::windows_sys::core::Interface for IRestrictedErrorInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2193256594, data2: 19592, data3: 17021, data4: [167, 188, 22, 221, 147, 254, 182, 126] };
}
#[repr(C)]
pub struct IRoMetaDataLocator {
    pub Locate: unsafe extern "system" fn(this: *mut *mut Self, nameelement: ::windows_sys::core::PCWSTR, metadatadestination: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRoMetaDataLocator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::zeroed();
}
#[repr(C)]
pub struct IRoSimpleMetaDataBuilder {
    pub SetWinRtInterface: unsafe extern "system" fn(this: *mut *mut Self, iid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetDelegate: unsafe extern "system" fn(this: *mut *mut Self, iid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetInterfaceGroupSimpleDefault: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR, defaultinterfacename: ::windows_sys::core::PCWSTR, defaultinterfaceiid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetInterfaceGroupParameterizedDefault: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR, elementcount: u32, defaultinterfacenameelements: *const ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetRuntimeClassSimpleDefault: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR, defaultinterfacename: ::windows_sys::core::PCWSTR, defaultinterfaceiid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetRuntimeClassParameterizedDefault: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR, elementcount: u32, defaultinterfacenameelements: *const ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetStruct: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR, numfields: u32, fieldtypenames: *const ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetEnum: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR, basetype: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SetParameterizedInterface: unsafe extern "system" fn(this: *mut *mut Self, piid: ::windows_sys::core::GUID, numargs: u32) -> ::windows_sys::core::HRESULT,
    pub SetParameterizedDelegate: unsafe extern "system" fn(this: *mut *mut Self, piid: ::windows_sys::core::GUID, numargs: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRoSimpleMetaDataBuilder {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::zeroed();
}
#[repr(C)]
pub struct IShareWindowCommandEventArgsInterop {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWindow: unsafe extern "system" fn(this: *mut *mut Self, value: *mut super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWindow: usize,
}
impl ::windows_sys::core::Interface for IShareWindowCommandEventArgsInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1701947169, data2: 25661, data3: 17364, data4: [172, 164, 107, 111, 95, 48, 241, 173] };
}
#[repr(C)]
pub struct IShareWindowCommandSourceInterop {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut *mut Self, appwindow: super::super::Foundation::HWND, riid: *const ::windows_sys::core::GUID, sharewindowcommandsource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
impl ::windows_sys::core::Interface for IShareWindowCommandSourceInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1176115487, data2: 33828, data3: 17318, data4: [160, 250, 52, 81, 162, 47, 86, 171] };
}
#[repr(C)]
pub struct ISpatialInteractionManagerInterop {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut *mut Self, window: super::super::Foundation::HWND, riid: *const ::windows_sys::core::GUID, spatialinteractionmanager: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
impl ::windows_sys::core::Interface for ISpatialInteractionManagerInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1548674358, data2: 27288, data3: 19334, data4: [161, 112, 88, 112, 19, 214, 253, 75] };
}
#[repr(C)]
pub struct ISystemMediaTransportControlsInterop {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut *mut Self, appwindow: super::super::Foundation::HWND, riid: *const ::windows_sys::core::GUID, mediatransportcontrol: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
impl ::windows_sys::core::Interface for ISystemMediaTransportControlsInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3719317293, data2: 51473, data3: 18975, data4: [134, 217, 220, 61, 113, 169, 95, 90] };
}
#[repr(C)]
pub struct IUIViewSettingsInterop {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut *mut Self, hwnd: super::super::Foundation::HWND, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
impl ::windows_sys::core::Interface for IUIViewSettingsInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 915725305, data2: 36712, data3: 17598, data4: [143, 245, 25, 92, 152, 237, 232, 166] };
}
#[repr(C)]
pub struct IUserActivityInterop {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSessionForWindow: unsafe extern "system" fn(this: *mut *mut Self, window: super::super::Foundation::HWND, iid: *const ::windows_sys::core::GUID, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSessionForWindow: usize,
}
impl ::windows_sys::core::Interface for IUserActivityInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 450769229, data2: 3594, data3: 16601, data4: [130, 76, 154, 8, 138, 80, 5, 159] };
}
#[repr(C)]
pub struct IUserActivityRequestManagerInterop {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut *mut Self, window: super::super::Foundation::HWND, iid: *const ::windows_sys::core::GUID, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
impl ::windows_sys::core::Interface for IUserActivityRequestManagerInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3714709622, data2: 38553, data3: 18197, data4: [144, 149, 227, 126, 163, 13, 250, 27] };
}
#[repr(C)]
pub struct IUserActivitySourceHostInterop {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetActivitySourceHost: unsafe extern "system" fn(this: *mut *mut Self, activitysourcehost: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserActivitySourceHostInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3244161212, data2: 34884, data3: 18554, data4: [184, 91, 117, 120, 224, 246, 20, 25] };
}
#[repr(C)]
pub struct IUserConsentVerifierInterop {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestVerificationForWindowAsync: unsafe extern "system" fn(this: *mut *mut Self, appwindow: super::super::Foundation::HWND, message: ::windows_sys::core::HSTRING, riid: *const ::windows_sys::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestVerificationForWindowAsync: usize,
}
impl ::windows_sys::core::Interface for IUserConsentVerifierInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 971002051, data2: 20084, data3: 17434, data4: [141, 192, 184, 17, 4, 223, 148, 156] };
}
#[repr(C)]
pub struct IWeakReference {
    pub base__: ::windows_sys::core::IUnknown,
    pub Resolve: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, objectreference: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWeakReference {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 55, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IWeakReferenceSource {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetWeakReference: unsafe extern "system" fn(this: *mut *mut Self, weakreference: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWeakReferenceSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 56, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IWebAuthenticationCoreManagerInterop {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestTokenForWindowAsync: unsafe extern "system" fn(this: *mut *mut Self, appwindow: super::super::Foundation::HWND, request: *mut ::core::ffi::c_void, riid: *const ::windows_sys::core::GUID, asyncinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestTokenForWindowAsync: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestTokenWithWebAccountForWindowAsync: unsafe extern "system" fn(this: *mut *mut Self, appwindow: super::super::Foundation::HWND, request: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, riid: *const ::windows_sys::core::GUID, asyncinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestTokenWithWebAccountForWindowAsync: usize,
}
impl ::windows_sys::core::Interface for IWebAuthenticationCoreManagerInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4105758724, data2: 33054, data3: 17462, data4: [182, 156, 68, 203, 103, 183, 32, 132] };
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const MAX_ERROR_MESSAGE_CHARS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub type PINSPECT_HSTRING_CALLBACK = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, readaddress: usize, length: u32, buffer: *mut u8) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub type PINSPECT_HSTRING_CALLBACK2 = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, readaddress: u64, length: u32, buffer: *mut u8) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub type PINSPECT_MEMORY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, readaddress: usize, length: u32, buffer: *mut u8) -> ::windows_sys::core::HRESULT>;
pub type ROPARAMIIDHANDLE = isize;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub type RO_ERROR_REPORTING_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const RO_ERROR_REPORTING_NONE: RO_ERROR_REPORTING_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const RO_ERROR_REPORTING_SUPPRESSEXCEPTIONS: RO_ERROR_REPORTING_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const RO_ERROR_REPORTING_FORCEEXCEPTIONS: RO_ERROR_REPORTING_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const RO_ERROR_REPORTING_USESETERRORINFO: RO_ERROR_REPORTING_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const RO_ERROR_REPORTING_SUPPRESSSETERRORINFO: RO_ERROR_REPORTING_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub type RO_INIT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const RO_INIT_SINGLETHREADED: RO_INIT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const RO_INIT_MULTITHREADED: RO_INIT_TYPE = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub struct ServerInformation {
    pub dwServerPid: u32,
    pub dwServerTid: u32,
    pub ui64ServerAddress: u64,
}
impl ::core::marker::Copy for ServerInformation {}
impl ::core::clone::Clone for ServerInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub type TrustLevel = i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const BaseTrust: TrustLevel = 0i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const PartialTrust: TrustLevel = 1i32;
#[doc = "*Required features: `\"Win32_System_WinRT\"`*"]
pub const FullTrust: TrustLevel = 2i32;
#[repr(C)]
pub struct _RO_REGISTRATION_COOKIE(pub u8);
