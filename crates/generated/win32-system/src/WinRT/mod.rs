#[cfg(feature = "AllJoyn")]
pub mod AllJoyn;
#[cfg(feature = "Composition")]
pub mod Composition;
#[cfg(feature = "CoreInputView")]
pub mod CoreInputView;
#[cfg(feature = "Direct3D11")]
pub mod Direct3D11;
#[cfg(feature = "Display")]
pub mod Display;
#[cfg(feature = "Graphics")]
pub mod Graphics;
#[cfg(feature = "Holographic")]
pub mod Holographic;
#[cfg(feature = "Isolation")]
pub mod Isolation;
#[cfg(feature = "ML")]
pub mod ML;
#[cfg(feature = "Media")]
pub mod Media;
#[cfg(feature = "Pdf")]
pub mod Pdf;
#[cfg(feature = "Printing")]
pub mod Printing;
#[cfg(feature = "Shell")]
pub mod Shell;
#[cfg(feature = "Storage")]
pub mod Storage;
#[cfg(feature = "Xaml")]
pub mod Xaml;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ACTIVATIONTYPE(pub i32);
pub const ACTIVATIONTYPE_UNCATEGORIZED: ACTIVATIONTYPE = ACTIVATIONTYPE(0i32);
pub const ACTIVATIONTYPE_FROM_MONIKER: ACTIVATIONTYPE = ACTIVATIONTYPE(1i32);
pub const ACTIVATIONTYPE_FROM_DATA: ACTIVATIONTYPE = ACTIVATIONTYPE(2i32);
pub const ACTIVATIONTYPE_FROM_STORAGE: ACTIVATIONTYPE = ACTIVATIONTYPE(4i32);
pub const ACTIVATIONTYPE_FROM_STREAM: ACTIVATIONTYPE = ACTIVATIONTYPE(8i32);
pub const ACTIVATIONTYPE_FROM_FILE: ACTIVATIONTYPE = ACTIVATIONTYPE(16i32);
impl ::core::marker::Copy for ACTIVATIONTYPE {}
impl ::core::clone::Clone for ACTIVATIONTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ACTIVATIONTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ACTIVATIONTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ACTIVATIONTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACTIVATIONTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct APARTMENT_SHUTDOWN_REGISTRATION_COOKIE(pub isize);
impl APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {}
impl ::core::fmt::Debug for APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APARTMENT_SHUTDOWN_REGISTRATION_COOKIE").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Abi for APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AgileReferenceOptions(pub i32);
pub const AGILEREFERENCE_DEFAULT: AgileReferenceOptions = AgileReferenceOptions(0i32);
pub const AGILEREFERENCE_DELAYEDMARSHAL: AgileReferenceOptions = AgileReferenceOptions(1i32);
impl ::core::marker::Copy for AgileReferenceOptions {}
impl ::core::clone::Clone for AgileReferenceOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AgileReferenceOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AgileReferenceOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for AgileReferenceOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AgileReferenceOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BSOS_OPTIONS(pub i32);
pub const BSOS_DEFAULT: BSOS_OPTIONS = BSOS_OPTIONS(0i32);
pub const BSOS_PREFERDESTINATIONSTREAM: BSOS_OPTIONS = BSOS_OPTIONS(1i32);
impl ::core::marker::Copy for BSOS_OPTIONS {}
impl ::core::clone::Clone for BSOS_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BSOS_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BSOS_OPTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for BSOS_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BSOS_OPTIONS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CASTING_CONNECTION_ERROR_STATUS(pub i32);
pub const CASTING_CONNECTION_ERROR_STATUS_SUCCEEDED: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(0i32);
pub const CASTING_CONNECTION_ERROR_STATUS_DEVICE_DID_NOT_RESPOND: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(1i32);
pub const CASTING_CONNECTION_ERROR_STATUS_DEVICE_ERROR: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(2i32);
pub const CASTING_CONNECTION_ERROR_STATUS_DEVICE_LOCKED: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(3i32);
pub const CASTING_CONNECTION_ERROR_STATUS_PROTECTED_PLAYBACK_FAILED: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(4i32);
pub const CASTING_CONNECTION_ERROR_STATUS_INVALID_CASTING_SOURCE: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(5i32);
pub const CASTING_CONNECTION_ERROR_STATUS_UNKNOWN: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(6i32);
impl ::core::marker::Copy for CASTING_CONNECTION_ERROR_STATUS {}
impl ::core::clone::Clone for CASTING_CONNECTION_ERROR_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CASTING_CONNECTION_ERROR_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CASTING_CONNECTION_ERROR_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CASTING_CONNECTION_ERROR_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CASTING_CONNECTION_ERROR_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CASTING_CONNECTION_STATE(pub i32);
pub const CASTING_CONNECTION_STATE_DISCONNECTED: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(0i32);
pub const CASTING_CONNECTION_STATE_CONNECTED: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(1i32);
pub const CASTING_CONNECTION_STATE_RENDERING: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(2i32);
pub const CASTING_CONNECTION_STATE_DISCONNECTING: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(3i32);
pub const CASTING_CONNECTION_STATE_CONNECTING: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(4i32);
impl ::core::marker::Copy for CASTING_CONNECTION_STATE {}
impl ::core::clone::Clone for CASTING_CONNECTION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CASTING_CONNECTION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CASTING_CONNECTION_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for CASTING_CONNECTION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CASTING_CONNECTION_STATE").field(&self.0).finish()
    }
}
pub const CastingSourceInfo_Property_CastingTypes: &str = "CastingTypes";
pub const CastingSourceInfo_Property_PreferredSourceUriScheme: &str = "PreferredSourceUriScheme";
pub const CastingSourceInfo_Property_ProtectedMedia: &str = "ProtectedMedia";
#[inline]
pub unsafe fn CoDecodeProxy(dwclientpid: u32, ui64proxyaddress: u64) -> ::windows_core::Result<ServerInformation> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoDecodeProxy(dwclientpid: u32, ui64proxyaddress: u64, pserverinformation: *mut ServerInformation) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<ServerInformation>::zeroed();
        CoDecodeProxy(::core::mem::transmute(dwclientpid), ::core::mem::transmute(ui64proxyaddress), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ServerInformation>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateControlInput<T: ::windows_core::Interface>() -> ::windows_core::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateControlInput(riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::option::Option::None;
        CreateControlInput(&<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateControlInputEx<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, T: ::windows_core::Interface>(pcorewindow: Param0) -> ::windows_core::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateControlInputEx(pcorewindow: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::option::Option::None;
        CreateControlInputEx(pcorewindow.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "winrt-system")]
#[inline]
pub unsafe fn CreateDispatcherQueueController<'a, Param0: ::windows_core::IntoParam<'a, DispatcherQueueOptions>>(options: Param0) -> ::windows_core::Result<::winrt_system::DispatcherQueueController> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDispatcherQueueController(options: DispatcherQueueOptions, dispatcherqueuecontroller: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        CreateDispatcherQueueController(options.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::winrt_system::DispatcherQueueController>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateRandomAccessStreamOnFile<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, T: ::windows_core::Interface>(filepath: Param0, accessmode: u32) -> ::windows_core::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateRandomAccessStreamOnFile(filepath: ::windows_core::PCWSTR, accessmode: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::option::Option::None;
        CreateRandomAccessStreamOnFile(filepath.into_param().abi(), ::core::mem::transmute(accessmode), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-system")]
#[inline]
pub unsafe fn CreateRandomAccessStreamOverStream<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IStream>, T: ::windows_core::Interface>(stream: Param0, options: BSOS_OPTIONS) -> ::windows_core::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateRandomAccessStreamOverStream(stream: ::windows_core::RawPtr, options: BSOS_OPTIONS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::option::Option::None;
        CreateRandomAccessStreamOverStream(stream.into_param().abi(), ::core::mem::transmute(options), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateStreamOverRandomAccessStream<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, T: ::windows_core::Interface>(randomaccessstream: Param0) -> ::windows_core::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateStreamOverRandomAccessStream(randomaccessstream: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::option::Option::None;
        CreateStreamOverRandomAccessStream(randomaccessstream.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPATCHERQUEUE_THREAD_APARTMENTTYPE(pub i32);
pub const DQTAT_COM_NONE: DISPATCHERQUEUE_THREAD_APARTMENTTYPE = DISPATCHERQUEUE_THREAD_APARTMENTTYPE(0i32);
pub const DQTAT_COM_ASTA: DISPATCHERQUEUE_THREAD_APARTMENTTYPE = DISPATCHERQUEUE_THREAD_APARTMENTTYPE(1i32);
pub const DQTAT_COM_STA: DISPATCHERQUEUE_THREAD_APARTMENTTYPE = DISPATCHERQUEUE_THREAD_APARTMENTTYPE(2i32);
impl ::core::marker::Copy for DISPATCHERQUEUE_THREAD_APARTMENTTYPE {}
impl ::core::clone::Clone for DISPATCHERQUEUE_THREAD_APARTMENTTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPATCHERQUEUE_THREAD_APARTMENTTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPATCHERQUEUE_THREAD_APARTMENTTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPATCHERQUEUE_THREAD_APARTMENTTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPATCHERQUEUE_THREAD_APARTMENTTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISPATCHERQUEUE_THREAD_TYPE(pub i32);
pub const DQTYPE_THREAD_DEDICATED: DISPATCHERQUEUE_THREAD_TYPE = DISPATCHERQUEUE_THREAD_TYPE(1i32);
pub const DQTYPE_THREAD_CURRENT: DISPATCHERQUEUE_THREAD_TYPE = DISPATCHERQUEUE_THREAD_TYPE(2i32);
impl ::core::marker::Copy for DISPATCHERQUEUE_THREAD_TYPE {}
impl ::core::clone::Clone for DISPATCHERQUEUE_THREAD_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPATCHERQUEUE_THREAD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DISPATCHERQUEUE_THREAD_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISPATCHERQUEUE_THREAD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPATCHERQUEUE_THREAD_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
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
impl ::core::fmt::Debug for DispatcherQueueOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DispatcherQueueOptions").field("dwSize", &self.dwSize).field("threadType", &self.threadType).field("apartmentType", &self.apartmentType).finish()
    }
}
unsafe impl ::windows_core::Abi for DispatcherQueueOptions {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DispatcherQueueOptions {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DispatcherQueueOptions>()) == 0 }
    }
}
impl ::core::cmp::Eq for DispatcherQueueOptions {}
impl ::core::default::Default for DispatcherQueueOptions {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct EventRegistrationToken {
    pub value: i64,
}
impl ::core::marker::Copy for EventRegistrationToken {}
impl ::core::clone::Clone for EventRegistrationToken {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EventRegistrationToken {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EventRegistrationToken").field("value", &self.value).finish()
    }
}
unsafe impl ::windows_core::Abi for EventRegistrationToken {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EventRegistrationToken {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EventRegistrationToken>()) == 0 }
    }
}
impl ::core::cmp::Eq for EventRegistrationToken {}
impl ::core::default::Default for EventRegistrationToken {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[inline]
pub unsafe fn GetRestrictedErrorInfo() -> ::windows_core::Result<IRestrictedErrorInfo> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRestrictedErrorInfo(pprestrictederrorinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        GetRestrictedErrorInfo(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRestrictedErrorInfo>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HSTRING_BUFFER(pub isize);
impl HSTRING_BUFFER {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HSTRING_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HSTRING_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HSTRING_BUFFER {}
impl ::core::fmt::Debug for HSTRING_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HSTRING_BUFFER").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Abi for HSTRING_BUFFER {
    type Abi = Self;
}
#[repr(C)]
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
impl ::core::fmt::Debug for HSTRING_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSTRING_HEADER").field("flags", &self.flags).field("length", &self.length).field("padding1", &self.padding1).field("padding2", &self.padding2).field("data", &self.data).finish()
    }
}
unsafe impl ::windows_core::Abi for HSTRING_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HSTRING_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HSTRING_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for HSTRING_HEADER {}
impl ::core::default::Default for HSTRING_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[inline]
pub unsafe fn HSTRING_UserFree(param0: *const u32, param1: *const ::windows_core::HSTRING) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HSTRING_UserFree(param0: *const u32, param1: *const ::core::mem::ManuallyDrop<::windows_core::HSTRING>);
        }
        HSTRING_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HSTRING_UserFree64(param0: *const u32, param1: *const ::windows_core::HSTRING) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HSTRING_UserFree64(param0: *const u32, param1: *const ::core::mem::ManuallyDrop<::windows_core::HSTRING>);
        }
        HSTRING_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HSTRING_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::windows_core::HSTRING) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HSTRING_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> *mut u8;
        }
        ::core::mem::transmute(HSTRING_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HSTRING_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::windows_core::HSTRING) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HSTRING_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> *mut u8;
        }
        ::core::mem::transmute(HSTRING_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HSTRING_UserSize(param0: *const u32, param1: u32, param2: *const ::windows_core::HSTRING) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HSTRING_UserSize(param0: *const u32, param1: u32, param2: *const ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> u32;
        }
        ::core::mem::transmute(HSTRING_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HSTRING_UserSize64(param0: *const u32, param1: u32, param2: *const ::windows_core::HSTRING) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HSTRING_UserSize64(param0: *const u32, param1: u32, param2: *const ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> u32;
        }
        ::core::mem::transmute(HSTRING_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HSTRING_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::windows_core::HSTRING) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HSTRING_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> *mut u8;
        }
        ::core::mem::transmute(HSTRING_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HSTRING_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::windows_core::HSTRING) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HSTRING_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> *mut u8;
        }
        ::core::mem::transmute(HSTRING_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
pub struct IAccountsSettingsPaneInterop(::windows_core::IUnknown);
impl IAccountsSettingsPaneInterop {
    pub unsafe fn GetForWindow<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, T: ::windows_core::Interface>(&self, appwindow: Param0) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetForWindow)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn ShowManageAccountsForWindowAsync<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, T: ::windows_core::Interface>(&self, appwindow: Param0) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).ShowManageAccountsForWindowAsync)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn ShowAddAccountForWindowAsync<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, T: ::windows_core::Interface>(&self, appwindow: Param0) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).ShowAddAccountForWindowAsync)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IAccountsSettingsPaneInterop> for ::windows_core::IUnknown {
    fn from(value: IAccountsSettingsPaneInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAccountsSettingsPaneInterop> for ::windows_core::IUnknown {
    fn from(value: &IAccountsSettingsPaneInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAccountsSettingsPaneInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAccountsSettingsPaneInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAccountsSettingsPaneInterop> for ::windows_core::IInspectable {
    fn from(value: IAccountsSettingsPaneInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAccountsSettingsPaneInterop> for ::windows_core::IInspectable {
    fn from(value: &IAccountsSettingsPaneInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IAccountsSettingsPaneInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IAccountsSettingsPaneInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAccountsSettingsPaneInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAccountsSettingsPaneInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAccountsSettingsPaneInterop {}
impl ::core::fmt::Debug for IAccountsSettingsPaneInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAccountsSettingsPaneInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAccountsSettingsPaneInterop {
    type Vtable = IAccountsSettingsPaneInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd3ee12ad_3865_4362_9746_b75a682df0e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneInterop_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::win32_foundation::HWND, riid: *const ::windows_core::GUID, accountssettingspane: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ShowManageAccountsForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::win32_foundation::HWND, riid: *const ::windows_core::GUID, asyncaction: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ShowAddAccountForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::win32_foundation::HWND, riid: *const ::windows_core::GUID, asyncaction: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IActivationFactory(::windows_core::IUnknown);
impl IActivationFactory {
    pub unsafe fn ActivateInstance(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).ActivateInstance)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IInspectable>(result__)
    }
}
impl ::core::convert::From<IActivationFactory> for ::windows_core::IUnknown {
    fn from(value: IActivationFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActivationFactory> for ::windows_core::IUnknown {
    fn from(value: &IActivationFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IActivationFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IActivationFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IActivationFactory> for ::windows_core::IInspectable {
    fn from(value: IActivationFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActivationFactory> for ::windows_core::IInspectable {
    fn from(value: &IActivationFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IActivationFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IActivationFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IActivationFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActivationFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActivationFactory {}
impl ::core::fmt::Debug for IActivationFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActivationFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IActivationFactory {
    type Vtable = IActivationFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000035_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ActivateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instance: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAgileReference(::windows_core::IUnknown);
impl IAgileReference {
    pub unsafe fn Resolve<T: ::windows_core::Interface>(&self) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).Resolve)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IAgileReference> for ::windows_core::IUnknown {
    fn from(value: IAgileReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAgileReference> for ::windows_core::IUnknown {
    fn from(value: &IAgileReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAgileReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAgileReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAgileReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAgileReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAgileReference {}
impl ::core::fmt::Debug for IAgileReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAgileReference").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAgileReference {
    type Vtable = IAgileReference_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc03f6a43_65a4_9818_987e_e0b810d2a6f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAgileReference_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Resolve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvobjectreference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IApartmentShutdown(::windows_core::IUnknown);
impl IApartmentShutdown {
    pub unsafe fn OnUninitialize(&self, ui64apartmentidentifier: u64) {
        (::windows_core::Interface::vtable(self).OnUninitialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ui64apartmentidentifier))
    }
}
impl ::core::convert::From<IApartmentShutdown> for ::windows_core::IUnknown {
    fn from(value: IApartmentShutdown) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IApartmentShutdown> for ::windows_core::IUnknown {
    fn from(value: &IApartmentShutdown) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IApartmentShutdown {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IApartmentShutdown {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IApartmentShutdown {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IApartmentShutdown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IApartmentShutdown {}
impl ::core::fmt::Debug for IApartmentShutdown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IApartmentShutdown").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IApartmentShutdown {
    type Vtable = IApartmentShutdown_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa2f05a09_27a2_42b5_bc0e_ac163ef49d9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApartmentShutdown_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnUninitialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ui64apartmentidentifier: u64),
}
#[repr(transparent)]
pub struct IAppServiceConnectionExtendedExecution(::windows_core::IUnknown);
impl IAppServiceConnectionExtendedExecution {
    pub unsafe fn OpenForExtendedExecutionAsync<T: ::windows_core::Interface>(&self) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).OpenForExtendedExecutionAsync)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IAppServiceConnectionExtendedExecution> for ::windows_core::IUnknown {
    fn from(value: IAppServiceConnectionExtendedExecution) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppServiceConnectionExtendedExecution> for ::windows_core::IUnknown {
    fn from(value: &IAppServiceConnectionExtendedExecution) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAppServiceConnectionExtendedExecution {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAppServiceConnectionExtendedExecution {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAppServiceConnectionExtendedExecution {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppServiceConnectionExtendedExecution {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppServiceConnectionExtendedExecution {}
impl ::core::fmt::Debug for IAppServiceConnectionExtendedExecution {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppServiceConnectionExtendedExecution").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAppServiceConnectionExtendedExecution {
    type Vtable = IAppServiceConnectionExtendedExecution_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x65219584_f9cb_4ae3_81f9_a28a6ca450d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceConnectionExtendedExecution_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OpenForExtendedExecutionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, operation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IBufferByteAccess(::windows_core::IUnknown);
impl IBufferByteAccess {
    pub unsafe fn Buffer(&self) -> ::windows_core::Result<*mut u8> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut u8>::zeroed();
        (::windows_core::Interface::vtable(self).Buffer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut u8>(result__)
    }
}
impl ::core::convert::From<IBufferByteAccess> for ::windows_core::IUnknown {
    fn from(value: IBufferByteAccess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBufferByteAccess> for ::windows_core::IUnknown {
    fn from(value: &IBufferByteAccess) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IBufferByteAccess {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IBufferByteAccess {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBufferByteAccess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBufferByteAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBufferByteAccess {}
impl ::core::fmt::Debug for IBufferByteAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBufferByteAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IBufferByteAccess {
    type Vtable = IBufferByteAccess_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x905a0fef_bc53_11df_8c49_001e4fc686da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBufferByteAccess_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Buffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut u8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICastingController(::windows_core::IUnknown);
impl ICastingController {
    pub unsafe fn Initialize<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, castingengine: Param0, castingsource: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), castingengine.into_param().abi(), castingsource.into_param().abi()).ok()
    }
    pub unsafe fn Connect(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Connect)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Disconnect(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Disconnect)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Advise<'a, Param0: ::windows_core::IntoParam<'a, ICastingEventHandler>>(&self, eventhandler: Param0) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).Advise)(::windows_core::Interface::as_raw(self), eventhandler.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn UnAdvise(&self, cookie: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnAdvise)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cookie)).ok()
    }
}
impl ::core::convert::From<ICastingController> for ::windows_core::IUnknown {
    fn from(value: ICastingController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICastingController> for ::windows_core::IUnknown {
    fn from(value: &ICastingController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICastingController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICastingController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICastingController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICastingController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICastingController {}
impl ::core::fmt::Debug for ICastingController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICastingController").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICastingController {
    type Vtable = ICastingController_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf0a56423_a664_4fbd_8b43_409a45e8d9a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingController_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, castingengine: *mut ::core::ffi::c_void, castingsource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: ::windows_core::RawPtr, cookie: *mut u32) -> ::windows_core::HRESULT,
    pub UnAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICastingEventHandler(::windows_core::IUnknown);
impl ICastingEventHandler {
    pub unsafe fn OnStateChanged(&self, newstate: CASTING_CONNECTION_STATE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnStateChanged)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(newstate)).ok()
    }
    pub unsafe fn OnError<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, errorstatus: CASTING_CONNECTION_ERROR_STATUS, errormessage: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnError)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(errorstatus), errormessage.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ICastingEventHandler> for ::windows_core::IUnknown {
    fn from(value: ICastingEventHandler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICastingEventHandler> for ::windows_core::IUnknown {
    fn from(value: &ICastingEventHandler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICastingEventHandler {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICastingEventHandler {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICastingEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICastingEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICastingEventHandler {}
impl ::core::fmt::Debug for ICastingEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICastingEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICastingEventHandler {
    type Vtable = ICastingEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc79a6cb7_bebd_47a6_a2ad_4d45ad79c7bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newstate: CASTING_CONNECTION_STATE) -> ::windows_core::HRESULT,
    pub OnError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorstatus: CASTING_CONNECTION_ERROR_STATUS, errormessage: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICastingSourceInfo(::windows_core::IUnknown);
impl ICastingSourceInfo {
    pub unsafe fn GetController(&self) -> ::windows_core::Result<ICastingController> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetController)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ICastingController>(result__)
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn GetProperties(&self) -> ::windows_core::Result<::win32_ui::Shell::PropertiesSystem::INamedPropertyStore> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_ui::Shell::PropertiesSystem::INamedPropertyStore>(result__)
    }
}
impl ::core::convert::From<ICastingSourceInfo> for ::windows_core::IUnknown {
    fn from(value: ICastingSourceInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICastingSourceInfo> for ::windows_core::IUnknown {
    fn from(value: &ICastingSourceInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICastingSourceInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICastingSourceInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICastingSourceInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICastingSourceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICastingSourceInfo {}
impl ::core::fmt::Debug for ICastingSourceInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICastingSourceInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICastingSourceInfo {
    type Vtable = ICastingSourceInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x45101ab7_7c3a_4bce_9500_12c09024b298);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingSourceInfo_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controller: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-ui")]
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, props: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    GetProperties: usize,
}
#[repr(transparent)]
pub struct ICoreInputInterop(::windows_core::IUnknown);
impl ICoreInputInterop {
    pub unsafe fn SetInputSource<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInputSource)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    pub unsafe fn SetMessageHandled(&self, value: u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMessageHandled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
}
impl ::core::convert::From<ICoreInputInterop> for ::windows_core::IUnknown {
    fn from(value: ICoreInputInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreInputInterop> for ::windows_core::IUnknown {
    fn from(value: &ICoreInputInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICoreInputInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICoreInputInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICoreInputInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICoreInputInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreInputInterop {}
impl ::core::fmt::Debug for ICoreInputInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreInputInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICoreInputInterop {
    type Vtable = ICoreInputInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x40bfe3e3_b75a_4479_ac96_475365749bb8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputInterop_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetInputSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetMessageHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICoreWindowAdapterInterop(::windows_core::IUnknown);
impl ICoreWindowAdapterInterop {
    pub unsafe fn AppActivationClientAdapter(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).AppActivationClientAdapter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn ApplicationViewClientAdapter(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).ApplicationViewClientAdapter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn CoreApplicationViewClientAdapter(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).CoreApplicationViewClientAdapter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn HoloViewClientAdapter(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).HoloViewClientAdapter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn PositionerClientAdapter(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).PositionerClientAdapter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn SystemNavigationClientAdapter(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).SystemNavigationClientAdapter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn TitleBarClientAdapter(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).TitleBarClientAdapter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn SetWindowClientAdapter<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWindowClientAdapter)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ICoreWindowAdapterInterop> for ::windows_core::IUnknown {
    fn from(value: ICoreWindowAdapterInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreWindowAdapterInterop> for ::windows_core::IUnknown {
    fn from(value: &ICoreWindowAdapterInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICoreWindowAdapterInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICoreWindowAdapterInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICoreWindowAdapterInterop> for ::windows_core::IInspectable {
    fn from(value: ICoreWindowAdapterInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreWindowAdapterInterop> for ::windows_core::IInspectable {
    fn from(value: &ICoreWindowAdapterInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ICoreWindowAdapterInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ICoreWindowAdapterInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICoreWindowAdapterInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICoreWindowAdapterInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreWindowAdapterInterop {}
impl ::core::fmt::Debug for ICoreWindowAdapterInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreWindowAdapterInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICoreWindowAdapterInterop {
    type Vtable = ICoreWindowAdapterInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a5b6fd1_cd73_4b6c_9cf4_2e869eaf470a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowAdapterInterop_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AppActivationClientAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ApplicationViewClientAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CoreApplicationViewClientAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub HoloViewClientAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PositionerClientAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SystemNavigationClientAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TitleBarClientAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetWindowClientAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICoreWindowComponentInterop(::windows_core::IUnknown);
impl ICoreWindowComponentInterop {
    pub unsafe fn ConfigureComponentInput<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param2: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, hostviewinstanceid: u32, hwndhost: Param1, inputsourcevisual: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ConfigureComponentInput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hostviewinstanceid), hwndhost.into_param().abi(), inputsourcevisual.into_param().abi()).ok()
    }
    pub unsafe fn GetViewInstanceId(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetViewInstanceId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<ICoreWindowComponentInterop> for ::windows_core::IUnknown {
    fn from(value: ICoreWindowComponentInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreWindowComponentInterop> for ::windows_core::IUnknown {
    fn from(value: &ICoreWindowComponentInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICoreWindowComponentInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICoreWindowComponentInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICoreWindowComponentInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICoreWindowComponentInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreWindowComponentInterop {}
impl ::core::fmt::Debug for ICoreWindowComponentInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreWindowComponentInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICoreWindowComponentInterop {
    type Vtable = ICoreWindowComponentInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0576ab31_a310_4c40_ba31_fd37e0298dfa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowComponentInterop_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ConfigureComponentInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostviewinstanceid: u32, hwndhost: ::win32_foundation::HWND, inputsourcevisual: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetViewInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, componentviewinstanceid: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICoreWindowInterop(::windows_core::IUnknown);
impl ICoreWindowInterop {
    pub unsafe fn WindowHandle(&self) -> ::windows_core::Result<::win32_foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::HWND>::zeroed();
        (::windows_core::Interface::vtable(self).WindowHandle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::HWND>(result__)
    }
    pub unsafe fn SetMessageHandled(&self, value: u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMessageHandled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
}
impl ::core::convert::From<ICoreWindowInterop> for ::windows_core::IUnknown {
    fn from(value: ICoreWindowInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreWindowInterop> for ::windows_core::IUnknown {
    fn from(value: &ICoreWindowInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICoreWindowInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICoreWindowInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICoreWindowInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICoreWindowInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreWindowInterop {}
impl ::core::fmt::Debug for ICoreWindowInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreWindowInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICoreWindowInterop {
    type Vtable = ICoreWindowInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x45d64a29_a63e_4cb6_b498_5781d298cb4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowInterop_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub WindowHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: *mut ::win32_foundation::HWND) -> ::windows_core::HRESULT,
    pub SetMessageHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICorrelationVectorInformation(::windows_core::IUnknown);
impl ICorrelationVectorInformation {
    pub unsafe fn LastCorrelationVectorForThread(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
        (::windows_core::Interface::vtable(self).LastCorrelationVectorForThread)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::HSTRING>(result__)
    }
    pub unsafe fn NextCorrelationVectorForThread(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
        (::windows_core::Interface::vtable(self).NextCorrelationVectorForThread)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::HSTRING>(result__)
    }
    pub unsafe fn SetNextCorrelationVectorForThread<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, cv: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNextCorrelationVectorForThread)(::windows_core::Interface::as_raw(self), cv.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ICorrelationVectorInformation> for ::windows_core::IUnknown {
    fn from(value: ICorrelationVectorInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICorrelationVectorInformation> for ::windows_core::IUnknown {
    fn from(value: &ICorrelationVectorInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICorrelationVectorInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICorrelationVectorInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICorrelationVectorInformation> for ::windows_core::IInspectable {
    fn from(value: ICorrelationVectorInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICorrelationVectorInformation> for ::windows_core::IInspectable {
    fn from(value: &ICorrelationVectorInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ICorrelationVectorInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ICorrelationVectorInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICorrelationVectorInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICorrelationVectorInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorrelationVectorInformation {}
impl ::core::fmt::Debug for ICorrelationVectorInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorrelationVectorInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICorrelationVectorInformation {
    type Vtable = ICorrelationVectorInformation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x83c78b3c_d88b_4950_aa6e_22b8d22aabd3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorrelationVectorInformation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LastCorrelationVectorForThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cv: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NextCorrelationVectorForThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cv: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetNextCorrelationVectorForThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cv: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICorrelationVectorSource(::windows_core::IUnknown);
impl ICorrelationVectorSource {
    pub unsafe fn CorrelationVector(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
        (::windows_core::Interface::vtable(self).CorrelationVector)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::HSTRING>(result__)
    }
}
impl ::core::convert::From<ICorrelationVectorSource> for ::windows_core::IUnknown {
    fn from(value: ICorrelationVectorSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICorrelationVectorSource> for ::windows_core::IUnknown {
    fn from(value: &ICorrelationVectorSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICorrelationVectorSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICorrelationVectorSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICorrelationVectorSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICorrelationVectorSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorrelationVectorSource {}
impl ::core::fmt::Debug for ICorrelationVectorSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorrelationVectorSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICorrelationVectorSource {
    type Vtable = ICorrelationVectorSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x152b8a3b_b9b9_4685_b56e_974847bc7545);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorrelationVectorSource_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CorrelationVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cv: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDragDropManagerInterop(::windows_core::IUnknown);
impl IDragDropManagerInterop {
    pub unsafe fn GetForWindow<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, T: ::windows_core::Interface>(&self, hwnd: Param0) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetForWindow)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IDragDropManagerInterop> for ::windows_core::IUnknown {
    fn from(value: IDragDropManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDragDropManagerInterop> for ::windows_core::IUnknown {
    fn from(value: &IDragDropManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDragDropManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDragDropManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDragDropManagerInterop> for ::windows_core::IInspectable {
    fn from(value: IDragDropManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDragDropManagerInterop> for ::windows_core::IInspectable {
    fn from(value: &IDragDropManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IDragDropManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IDragDropManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDragDropManagerInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDragDropManagerInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDragDropManagerInterop {}
impl ::core::fmt::Debug for IDragDropManagerInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDragDropManagerInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDragDropManagerInterop {
    type Vtable = IDragDropManagerInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5ad8cba7_4c01_4dac_9074_827894292d63);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragDropManagerInterop_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: ::win32_foundation::HWND, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IHolographicSpaceInterop(::windows_core::IUnknown);
impl IHolographicSpaceInterop {
    pub unsafe fn CreateForWindow<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, T: ::windows_core::Interface>(&self, window: Param0) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).CreateForWindow)(::windows_core::Interface::as_raw(self), window.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IHolographicSpaceInterop> for ::windows_core::IUnknown {
    fn from(value: IHolographicSpaceInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHolographicSpaceInterop> for ::windows_core::IUnknown {
    fn from(value: &IHolographicSpaceInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IHolographicSpaceInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IHolographicSpaceInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IHolographicSpaceInterop> for ::windows_core::IInspectable {
    fn from(value: IHolographicSpaceInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHolographicSpaceInterop> for ::windows_core::IInspectable {
    fn from(value: &IHolographicSpaceInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IHolographicSpaceInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IHolographicSpaceInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IHolographicSpaceInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IHolographicSpaceInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHolographicSpaceInterop {}
impl ::core::fmt::Debug for IHolographicSpaceInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHolographicSpaceInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IHolographicSpaceInterop {
    type Vtable = IHolographicSpaceInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c4ee536_6a98_4b86_a170_587013d6fd4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicSpaceInterop_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: ::win32_foundation::HWND, riid: *const ::windows_core::GUID, holographicspace: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IInputPaneInterop(::windows_core::IUnknown);
impl IInputPaneInterop {
    pub unsafe fn GetForWindow<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, T: ::windows_core::Interface>(&self, appwindow: Param0) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetForWindow)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IInputPaneInterop> for ::windows_core::IUnknown {
    fn from(value: IInputPaneInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInputPaneInterop> for ::windows_core::IUnknown {
    fn from(value: &IInputPaneInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInputPaneInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInputPaneInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IInputPaneInterop> for ::windows_core::IInspectable {
    fn from(value: IInputPaneInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInputPaneInterop> for ::windows_core::IInspectable {
    fn from(value: &IInputPaneInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IInputPaneInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IInputPaneInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInputPaneInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInputPaneInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInputPaneInterop {}
impl ::core::fmt::Debug for IInputPaneInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInputPaneInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IInputPaneInterop {
    type Vtable = IInputPaneInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75cf2c57_9195_4931_8332_f0b409e916af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputPaneInterop_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::win32_foundation::HWND, riid: *const ::windows_core::GUID, inputpane: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ILanguageExceptionErrorInfo(::windows_core::IUnknown);
impl ILanguageExceptionErrorInfo {
    pub unsafe fn GetLanguageException(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).GetLanguageException)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
impl ::core::convert::From<ILanguageExceptionErrorInfo> for ::windows_core::IUnknown {
    fn from(value: ILanguageExceptionErrorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILanguageExceptionErrorInfo> for ::windows_core::IUnknown {
    fn from(value: &ILanguageExceptionErrorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ILanguageExceptionErrorInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ILanguageExceptionErrorInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILanguageExceptionErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILanguageExceptionErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILanguageExceptionErrorInfo {}
impl ::core::fmt::Debug for ILanguageExceptionErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILanguageExceptionErrorInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ILanguageExceptionErrorInfo {
    type Vtable = ILanguageExceptionErrorInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x04a2dbf3_df83_116c_0946_0812abf6e07d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionErrorInfo_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetLanguageException: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languageexception: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ILanguageExceptionErrorInfo2(::windows_core::IUnknown);
impl ILanguageExceptionErrorInfo2 {
    pub unsafe fn GetLanguageException(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLanguageException)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn GetPreviousLanguageExceptionErrorInfo(&self) -> ::windows_core::Result<ILanguageExceptionErrorInfo2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetPreviousLanguageExceptionErrorInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ILanguageExceptionErrorInfo2>(result__)
    }
    pub unsafe fn CapturePropagationContext<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, languageexception: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CapturePropagationContext)(::windows_core::Interface::as_raw(self), languageexception.into_param().abi()).ok()
    }
    pub unsafe fn GetPropagationContextHead(&self) -> ::windows_core::Result<ILanguageExceptionErrorInfo2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetPropagationContextHead)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ILanguageExceptionErrorInfo2>(result__)
    }
}
impl ::core::convert::From<ILanguageExceptionErrorInfo2> for ::windows_core::IUnknown {
    fn from(value: ILanguageExceptionErrorInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILanguageExceptionErrorInfo2> for ::windows_core::IUnknown {
    fn from(value: &ILanguageExceptionErrorInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ILanguageExceptionErrorInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ILanguageExceptionErrorInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ILanguageExceptionErrorInfo2> for ILanguageExceptionErrorInfo {
    fn from(value: ILanguageExceptionErrorInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILanguageExceptionErrorInfo2> for ILanguageExceptionErrorInfo {
    fn from(value: &ILanguageExceptionErrorInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILanguageExceptionErrorInfo> for ILanguageExceptionErrorInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, ILanguageExceptionErrorInfo> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILanguageExceptionErrorInfo> for &'a ILanguageExceptionErrorInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, ILanguageExceptionErrorInfo> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILanguageExceptionErrorInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILanguageExceptionErrorInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILanguageExceptionErrorInfo2 {}
impl ::core::fmt::Debug for ILanguageExceptionErrorInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILanguageExceptionErrorInfo2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ILanguageExceptionErrorInfo2 {
    type Vtable = ILanguageExceptionErrorInfo2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5746e5c4_5b97_424c_b620_2822915734dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionErrorInfo2_Vtbl {
    pub base__: ILanguageExceptionErrorInfo_Vtbl,
    pub GetPreviousLanguageExceptionErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, previouslanguageexceptionerrorinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CapturePropagationContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languageexception: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetPropagationContextHead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propagatedlanguageexceptionerrorinfohead: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ILanguageExceptionStackBackTrace(::windows_core::IUnknown);
impl ILanguageExceptionStackBackTrace {
    pub unsafe fn GetStackBackTrace(&self, maxframestocapture: u32, stackbacktrace: *mut usize, framescaptured: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetStackBackTrace)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(maxframestocapture), ::core::mem::transmute(stackbacktrace), ::core::mem::transmute(framescaptured)).ok()
    }
}
impl ::core::convert::From<ILanguageExceptionStackBackTrace> for ::windows_core::IUnknown {
    fn from(value: ILanguageExceptionStackBackTrace) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILanguageExceptionStackBackTrace> for ::windows_core::IUnknown {
    fn from(value: &ILanguageExceptionStackBackTrace) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ILanguageExceptionStackBackTrace {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ILanguageExceptionStackBackTrace {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILanguageExceptionStackBackTrace {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILanguageExceptionStackBackTrace {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILanguageExceptionStackBackTrace {}
impl ::core::fmt::Debug for ILanguageExceptionStackBackTrace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILanguageExceptionStackBackTrace").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ILanguageExceptionStackBackTrace {
    type Vtable = ILanguageExceptionStackBackTrace_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcbe53fb5_f967_4258_8d34_42f5e25833de);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionStackBackTrace_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetStackBackTrace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxframestocapture: u32, stackbacktrace: *mut usize, framescaptured: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ILanguageExceptionTransform(::windows_core::IUnknown);
impl ILanguageExceptionTransform {
    pub unsafe fn GetTransformedRestrictedErrorInfo(&self) -> ::windows_core::Result<IRestrictedErrorInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetTransformedRestrictedErrorInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRestrictedErrorInfo>(result__)
    }
}
impl ::core::convert::From<ILanguageExceptionTransform> for ::windows_core::IUnknown {
    fn from(value: ILanguageExceptionTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILanguageExceptionTransform> for ::windows_core::IUnknown {
    fn from(value: &ILanguageExceptionTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ILanguageExceptionTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ILanguageExceptionTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILanguageExceptionTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILanguageExceptionTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILanguageExceptionTransform {}
impl ::core::fmt::Debug for ILanguageExceptionTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILanguageExceptionTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ILanguageExceptionTransform {
    type Vtable = ILanguageExceptionTransform_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfeb5a271_a6cd_45ce_880a_696706badc65);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionTransform_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetTransformedRestrictedErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restrictederrorinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IMemoryBufferByteAccess(::windows_core::IUnknown);
impl IMemoryBufferByteAccess {
    pub unsafe fn GetBuffer(&self, value: *mut *mut u8, capacity: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetBuffer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value), ::core::mem::transmute(capacity)).ok()
    }
}
impl ::core::convert::From<IMemoryBufferByteAccess> for ::windows_core::IUnknown {
    fn from(value: IMemoryBufferByteAccess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMemoryBufferByteAccess> for ::windows_core::IUnknown {
    fn from(value: &IMemoryBufferByteAccess) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMemoryBufferByteAccess {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMemoryBufferByteAccess {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMemoryBufferByteAccess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMemoryBufferByteAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMemoryBufferByteAccess {}
impl ::core::fmt::Debug for IMemoryBufferByteAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMemoryBufferByteAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IMemoryBufferByteAccess {
    type Vtable = IMemoryBufferByteAccess_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5b0d3235_4dba_4d44_865e_8f1d0e4fd04d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryBufferByteAccess_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut u8, capacity: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IMessageDispatcher(::windows_core::IUnknown);
impl IMessageDispatcher {
    pub unsafe fn PumpMessages(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PumpMessages)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IMessageDispatcher> for ::windows_core::IUnknown {
    fn from(value: IMessageDispatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMessageDispatcher> for ::windows_core::IUnknown {
    fn from(value: &IMessageDispatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMessageDispatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMessageDispatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMessageDispatcher> for ::windows_core::IInspectable {
    fn from(value: IMessageDispatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMessageDispatcher> for ::windows_core::IInspectable {
    fn from(value: &IMessageDispatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IMessageDispatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IMessageDispatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMessageDispatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMessageDispatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMessageDispatcher {}
impl ::core::fmt::Debug for IMessageDispatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMessageDispatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IMessageDispatcher {
    type Vtable = IMessageDispatcher_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5f84c8f_cfd0_4cd6_b66b_c5d26ff1689d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageDispatcher_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PumpMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPlayToManagerInterop(::windows_core::IUnknown);
impl IPlayToManagerInterop {
    pub unsafe fn GetForWindow<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, T: ::windows_core::Interface>(&self, appwindow: Param0) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetForWindow)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn ShowPlayToUIForWindow<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(&self, appwindow: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ShowPlayToUIForWindow)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IPlayToManagerInterop> for ::windows_core::IUnknown {
    fn from(value: IPlayToManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPlayToManagerInterop> for ::windows_core::IUnknown {
    fn from(value: &IPlayToManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPlayToManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPlayToManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPlayToManagerInterop> for ::windows_core::IInspectable {
    fn from(value: IPlayToManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPlayToManagerInterop> for ::windows_core::IInspectable {
    fn from(value: &IPlayToManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IPlayToManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IPlayToManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPlayToManagerInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPlayToManagerInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPlayToManagerInterop {}
impl ::core::fmt::Debug for IPlayToManagerInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPlayToManagerInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPlayToManagerInterop {
    type Vtable = IPlayToManagerInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x24394699_1f2c_4eb3_8cd7_0ec1da42a540);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToManagerInterop_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::win32_foundation::HWND, riid: *const ::windows_core::GUID, playtomanager: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ShowPlayToUIForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::win32_foundation::HWND) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRestrictedErrorInfo(::windows_core::IUnknown);
impl IRestrictedErrorInfo {
    pub unsafe fn GetErrorDetails(&self, description: *mut ::win32_foundation::BSTR, error: *mut ::windows_core::HRESULT, restricteddescription: *mut ::win32_foundation::BSTR, capabilitysid: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetErrorDetails)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(description), ::core::mem::transmute(error), ::core::mem::transmute(restricteddescription), ::core::mem::transmute(capabilitysid)).ok()
    }
    pub unsafe fn GetReference(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetReference)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IRestrictedErrorInfo> for ::windows_core::IUnknown {
    fn from(value: IRestrictedErrorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRestrictedErrorInfo> for ::windows_core::IUnknown {
    fn from(value: &IRestrictedErrorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRestrictedErrorInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRestrictedErrorInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRestrictedErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRestrictedErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRestrictedErrorInfo {}
impl ::core::fmt::Debug for IRestrictedErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRestrictedErrorInfo").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for IRestrictedErrorInfo {}
unsafe impl ::core::marker::Sync for IRestrictedErrorInfo {}
unsafe impl ::windows_core::Interface for IRestrictedErrorInfo {
    type Vtable = IRestrictedErrorInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x82ba7092_4c88_427d_a7bc_16dd93feb67e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRestrictedErrorInfo_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetErrorDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut ::win32_foundation::BSTR, error: *mut ::windows_core::HRESULT, restricteddescription: *mut ::win32_foundation::BSTR, capabilitysid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reference: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRoMetaDataLocator(::windows_core::IUnknown);
impl IRoMetaDataLocator {
    pub unsafe fn Locate<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, IRoSimpleMetaDataBuilder>>(&self, nameelement: Param0, metadatadestination: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Locate)(::windows_core::Interface::as_raw(self), nameelement.into_param().abi(), metadatadestination.into_param().abi()).ok()
    }
}
impl ::core::clone::Clone for IRoMetaDataLocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRoMetaDataLocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRoMetaDataLocator {}
impl ::core::fmt::Debug for IRoMetaDataLocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRoMetaDataLocator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRoMetaDataLocator {
    type Vtable = IRoMetaDataLocator_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoMetaDataLocator_Vtbl {
    pub Locate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nameelement: ::windows_core::PCWSTR, metadatadestination: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRoSimpleMetaDataBuilder(::windows_core::IUnknown);
impl IRoSimpleMetaDataBuilder {
    pub unsafe fn SetWinRtInterface<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, iid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWinRtInterface)(::windows_core::Interface::as_raw(self), iid.into_param().abi()).ok()
    }
    pub unsafe fn SetDelegate<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, iid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDelegate)(::windows_core::Interface::as_raw(self), iid.into_param().abi()).ok()
    }
    pub unsafe fn SetInterfaceGroupSimpleDefault<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, name: Param0, defaultinterfacename: Param1, defaultinterfaceiid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInterfaceGroupSimpleDefault)(::windows_core::Interface::as_raw(self), name.into_param().abi(), defaultinterfacename.into_param().abi(), ::core::mem::transmute(defaultinterfaceiid)).ok()
    }
    pub unsafe fn SetInterfaceGroupParameterizedDefault<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, name: Param0, defaultinterfacenameelements: &[::windows_core::PWSTR]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInterfaceGroupParameterizedDefault)(::windows_core::Interface::as_raw(self), name.into_param().abi(), defaultinterfacenameelements.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(defaultinterfacenameelements))).ok()
    }
    pub unsafe fn SetRuntimeClassSimpleDefault<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, name: Param0, defaultinterfacename: Param1, defaultinterfaceiid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRuntimeClassSimpleDefault)(::windows_core::Interface::as_raw(self), name.into_param().abi(), defaultinterfacename.into_param().abi(), ::core::mem::transmute(defaultinterfaceiid)).ok()
    }
    pub unsafe fn SetRuntimeClassParameterizedDefault<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, name: Param0, defaultinterfacenameelements: &[::windows_core::PWSTR]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRuntimeClassParameterizedDefault)(::windows_core::Interface::as_raw(self), name.into_param().abi(), defaultinterfacenameelements.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(defaultinterfacenameelements))).ok()
    }
    pub unsafe fn SetStruct<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, name: Param0, fieldtypenames: &[::windows_core::PWSTR]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStruct)(::windows_core::Interface::as_raw(self), name.into_param().abi(), fieldtypenames.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(fieldtypenames))).ok()
    }
    pub unsafe fn SetEnum<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, name: Param0, basetype: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnum)(::windows_core::Interface::as_raw(self), name.into_param().abi(), basetype.into_param().abi()).ok()
    }
    pub unsafe fn SetParameterizedInterface<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, piid: Param0, numargs: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetParameterizedInterface)(::windows_core::Interface::as_raw(self), piid.into_param().abi(), ::core::mem::transmute(numargs)).ok()
    }
    pub unsafe fn SetParameterizedDelegate<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, piid: Param0, numargs: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetParameterizedDelegate)(::windows_core::Interface::as_raw(self), piid.into_param().abi(), ::core::mem::transmute(numargs)).ok()
    }
}
impl ::core::clone::Clone for IRoSimpleMetaDataBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRoSimpleMetaDataBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRoSimpleMetaDataBuilder {}
impl ::core::fmt::Debug for IRoSimpleMetaDataBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRoSimpleMetaDataBuilder").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRoSimpleMetaDataBuilder {
    type Vtable = IRoSimpleMetaDataBuilder_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoSimpleMetaDataBuilder_Vtbl {
    pub SetWinRtInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetDelegate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetInterfaceGroupSimpleDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, defaultinterfacename: ::windows_core::PCWSTR, defaultinterfaceiid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetInterfaceGroupParameterizedDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, elementcount: u32, defaultinterfacenameelements: *const ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetRuntimeClassSimpleDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, defaultinterfacename: ::windows_core::PCWSTR, defaultinterfaceiid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetRuntimeClassParameterizedDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, elementcount: u32, defaultinterfacenameelements: *const ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetStruct: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, numfields: u32, fieldtypenames: *const ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, basetype: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SetParameterizedInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piid: ::windows_core::GUID, numargs: u32) -> ::windows_core::HRESULT,
    pub SetParameterizedDelegate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piid: ::windows_core::GUID, numargs: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IShareWindowCommandEventArgsInterop(::windows_core::IUnknown);
impl IShareWindowCommandEventArgsInterop {
    pub unsafe fn GetWindow(&self) -> ::windows_core::Result<::win32_foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::HWND>::zeroed();
        (::windows_core::Interface::vtable(self).GetWindow)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::HWND>(result__)
    }
}
impl ::core::convert::From<IShareWindowCommandEventArgsInterop> for ::windows_core::IUnknown {
    fn from(value: IShareWindowCommandEventArgsInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IShareWindowCommandEventArgsInterop> for ::windows_core::IUnknown {
    fn from(value: &IShareWindowCommandEventArgsInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IShareWindowCommandEventArgsInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IShareWindowCommandEventArgsInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IShareWindowCommandEventArgsInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IShareWindowCommandEventArgsInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShareWindowCommandEventArgsInterop {}
impl ::core::fmt::Debug for IShareWindowCommandEventArgsInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShareWindowCommandEventArgsInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IShareWindowCommandEventArgsInterop {
    type Vtable = IShareWindowCommandEventArgsInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6571a721_643d_43d4_aca4_6b6f5f30f1ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareWindowCommandEventArgsInterop_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::win32_foundation::HWND) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IShareWindowCommandSourceInterop(::windows_core::IUnknown);
impl IShareWindowCommandSourceInterop {
    pub unsafe fn GetForWindow<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, T: ::windows_core::Interface>(&self, appwindow: Param0) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetForWindow)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IShareWindowCommandSourceInterop> for ::windows_core::IUnknown {
    fn from(value: IShareWindowCommandSourceInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IShareWindowCommandSourceInterop> for ::windows_core::IUnknown {
    fn from(value: &IShareWindowCommandSourceInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IShareWindowCommandSourceInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IShareWindowCommandSourceInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IShareWindowCommandSourceInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IShareWindowCommandSourceInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShareWindowCommandSourceInterop {}
impl ::core::fmt::Debug for IShareWindowCommandSourceInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShareWindowCommandSourceInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IShareWindowCommandSourceInterop {
    type Vtable = IShareWindowCommandSourceInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x461a191f_8424_43a6_a0fa_3451a22f56ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareWindowCommandSourceInterop_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::win32_foundation::HWND, riid: *const ::windows_core::GUID, sharewindowcommandsource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISpatialInteractionManagerInterop(::windows_core::IUnknown);
impl ISpatialInteractionManagerInterop {
    pub unsafe fn GetForWindow<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, T: ::windows_core::Interface>(&self, window: Param0) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetForWindow)(::windows_core::Interface::as_raw(self), window.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<ISpatialInteractionManagerInterop> for ::windows_core::IUnknown {
    fn from(value: ISpatialInteractionManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpatialInteractionManagerInterop> for ::windows_core::IUnknown {
    fn from(value: &ISpatialInteractionManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISpatialInteractionManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISpatialInteractionManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISpatialInteractionManagerInterop> for ::windows_core::IInspectable {
    fn from(value: ISpatialInteractionManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpatialInteractionManagerInterop> for ::windows_core::IInspectable {
    fn from(value: &ISpatialInteractionManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ISpatialInteractionManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ISpatialInteractionManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISpatialInteractionManagerInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISpatialInteractionManagerInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialInteractionManagerInterop {}
impl ::core::fmt::Debug for ISpatialInteractionManagerInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialInteractionManagerInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISpatialInteractionManagerInterop {
    type Vtable = ISpatialInteractionManagerInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c4ee536_6a98_4b86_a170_587013d6fd4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionManagerInterop_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: ::win32_foundation::HWND, riid: *const ::windows_core::GUID, spatialinteractionmanager: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISystemMediaTransportControlsInterop(::windows_core::IUnknown);
impl ISystemMediaTransportControlsInterop {
    pub unsafe fn GetForWindow<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, T: ::windows_core::Interface>(&self, appwindow: Param0) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetForWindow)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<ISystemMediaTransportControlsInterop> for ::windows_core::IUnknown {
    fn from(value: ISystemMediaTransportControlsInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISystemMediaTransportControlsInterop> for ::windows_core::IUnknown {
    fn from(value: &ISystemMediaTransportControlsInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISystemMediaTransportControlsInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISystemMediaTransportControlsInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISystemMediaTransportControlsInterop> for ::windows_core::IInspectable {
    fn from(value: ISystemMediaTransportControlsInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISystemMediaTransportControlsInterop> for ::windows_core::IInspectable {
    fn from(value: &ISystemMediaTransportControlsInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ISystemMediaTransportControlsInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ISystemMediaTransportControlsInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISystemMediaTransportControlsInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISystemMediaTransportControlsInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISystemMediaTransportControlsInterop {}
impl ::core::fmt::Debug for ISystemMediaTransportControlsInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISystemMediaTransportControlsInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISystemMediaTransportControlsInterop {
    type Vtable = ISystemMediaTransportControlsInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xddb0472d_c911_4a1f_86d9_dc3d71a95f5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControlsInterop_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::win32_foundation::HWND, riid: *const ::windows_core::GUID, mediatransportcontrol: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IUIViewSettingsInterop(::windows_core::IUnknown);
impl IUIViewSettingsInterop {
    pub unsafe fn GetForWindow<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, T: ::windows_core::Interface>(&self, hwnd: Param0) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetForWindow)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IUIViewSettingsInterop> for ::windows_core::IUnknown {
    fn from(value: IUIViewSettingsInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIViewSettingsInterop> for ::windows_core::IUnknown {
    fn from(value: &IUIViewSettingsInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUIViewSettingsInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUIViewSettingsInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IUIViewSettingsInterop> for ::windows_core::IInspectable {
    fn from(value: IUIViewSettingsInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIViewSettingsInterop> for ::windows_core::IInspectable {
    fn from(value: &IUIViewSettingsInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IUIViewSettingsInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IUIViewSettingsInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIViewSettingsInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIViewSettingsInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIViewSettingsInterop {}
impl ::core::fmt::Debug for IUIViewSettingsInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIViewSettingsInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUIViewSettingsInterop {
    type Vtable = IUIViewSettingsInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3694dbf9_8f68_44be_8ff5_195c98ede8a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIViewSettingsInterop_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: ::win32_foundation::HWND, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IUserActivityInterop(::windows_core::IUnknown);
impl IUserActivityInterop {
    pub unsafe fn CreateSessionForWindow<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, T: ::windows_core::Interface>(&self, window: Param0) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).CreateSessionForWindow)(::windows_core::Interface::as_raw(self), window.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IUserActivityInterop> for ::windows_core::IUnknown {
    fn from(value: IUserActivityInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserActivityInterop> for ::windows_core::IUnknown {
    fn from(value: &IUserActivityInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUserActivityInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUserActivityInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IUserActivityInterop> for ::windows_core::IInspectable {
    fn from(value: IUserActivityInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserActivityInterop> for ::windows_core::IInspectable {
    fn from(value: &IUserActivityInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IUserActivityInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IUserActivityInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUserActivityInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUserActivityInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUserActivityInterop {}
impl ::core::fmt::Debug for IUserActivityInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUserActivityInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUserActivityInterop {
    type Vtable = IUserActivityInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1ade314d_0e0a_40d9_824c_9a088a50059f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityInterop_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateSessionForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: ::win32_foundation::HWND, iid: *const ::windows_core::GUID, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IUserActivityRequestManagerInterop(::windows_core::IUnknown);
impl IUserActivityRequestManagerInterop {
    pub unsafe fn GetForWindow<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, T: ::windows_core::Interface>(&self, window: Param0) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetForWindow)(::windows_core::Interface::as_raw(self), window.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IUserActivityRequestManagerInterop> for ::windows_core::IUnknown {
    fn from(value: IUserActivityRequestManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserActivityRequestManagerInterop> for ::windows_core::IUnknown {
    fn from(value: &IUserActivityRequestManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUserActivityRequestManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUserActivityRequestManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IUserActivityRequestManagerInterop> for ::windows_core::IInspectable {
    fn from(value: IUserActivityRequestManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserActivityRequestManagerInterop> for ::windows_core::IInspectable {
    fn from(value: &IUserActivityRequestManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IUserActivityRequestManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IUserActivityRequestManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUserActivityRequestManagerInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUserActivityRequestManagerInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUserActivityRequestManagerInterop {}
impl ::core::fmt::Debug for IUserActivityRequestManagerInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUserActivityRequestManagerInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUserActivityRequestManagerInterop {
    type Vtable = IUserActivityRequestManagerInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdd69f876_9699_4715_9095_e37ea30dfa1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityRequestManagerInterop_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: ::win32_foundation::HWND, iid: *const ::windows_core::GUID, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IUserActivitySourceHostInterop(::windows_core::IUnknown);
impl IUserActivitySourceHostInterop {
    pub unsafe fn SetActivitySourceHost<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, activitysourcehost: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetActivitySourceHost)(::windows_core::Interface::as_raw(self), activitysourcehost.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IUserActivitySourceHostInterop> for ::windows_core::IUnknown {
    fn from(value: IUserActivitySourceHostInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserActivitySourceHostInterop> for ::windows_core::IUnknown {
    fn from(value: &IUserActivitySourceHostInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUserActivitySourceHostInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUserActivitySourceHostInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IUserActivitySourceHostInterop> for ::windows_core::IInspectable {
    fn from(value: IUserActivitySourceHostInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserActivitySourceHostInterop> for ::windows_core::IInspectable {
    fn from(value: &IUserActivitySourceHostInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IUserActivitySourceHostInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IUserActivitySourceHostInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUserActivitySourceHostInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUserActivitySourceHostInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUserActivitySourceHostInterop {}
impl ::core::fmt::Debug for IUserActivitySourceHostInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUserActivitySourceHostInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUserActivitySourceHostInterop {
    type Vtable = IUserActivitySourceHostInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc15df8bc_8844_487a_b85b_7578e0f61419);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivitySourceHostInterop_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetActivitySourceHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activitysourcehost: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IUserConsentVerifierInterop(::windows_core::IUnknown);
impl IUserConsentVerifierInterop {
    pub unsafe fn RequestVerificationForWindowAsync<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, T: ::windows_core::Interface>(&self, appwindow: Param0, message: Param1) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).RequestVerificationForWindowAsync)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), message.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IUserConsentVerifierInterop> for ::windows_core::IUnknown {
    fn from(value: IUserConsentVerifierInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserConsentVerifierInterop> for ::windows_core::IUnknown {
    fn from(value: &IUserConsentVerifierInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUserConsentVerifierInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUserConsentVerifierInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IUserConsentVerifierInterop> for ::windows_core::IInspectable {
    fn from(value: IUserConsentVerifierInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserConsentVerifierInterop> for ::windows_core::IInspectable {
    fn from(value: &IUserConsentVerifierInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IUserConsentVerifierInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IUserConsentVerifierInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUserConsentVerifierInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUserConsentVerifierInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUserConsentVerifierInterop {}
impl ::core::fmt::Debug for IUserConsentVerifierInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUserConsentVerifierInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUserConsentVerifierInterop {
    type Vtable = IUserConsentVerifierInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x39e050c3_4e74_441a_8dc0_b81104df949c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserConsentVerifierInterop_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestVerificationForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::win32_foundation::HWND, message: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWeakReference(::windows_core::IUnknown);
impl IWeakReference {
    pub unsafe fn Resolve<T: ::windows_core::Interface>(&self) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).Resolve)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IWeakReference> for ::windows_core::IUnknown {
    fn from(value: IWeakReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWeakReference> for ::windows_core::IUnknown {
    fn from(value: &IWeakReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWeakReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWeakReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWeakReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWeakReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWeakReference {}
impl ::core::fmt::Debug for IWeakReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWeakReference").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWeakReference {
    type Vtable = IWeakReference_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000037_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWeakReference_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Resolve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, objectreference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWeakReferenceSource(::windows_core::IUnknown);
impl IWeakReferenceSource {
    pub unsafe fn GetWeakReference(&self) -> ::windows_core::Result<IWeakReference> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetWeakReference)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWeakReference>(result__)
    }
}
impl ::core::convert::From<IWeakReferenceSource> for ::windows_core::IUnknown {
    fn from(value: IWeakReferenceSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWeakReferenceSource> for ::windows_core::IUnknown {
    fn from(value: &IWeakReferenceSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWeakReferenceSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWeakReferenceSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWeakReferenceSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWeakReferenceSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWeakReferenceSource {}
impl ::core::fmt::Debug for IWeakReferenceSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWeakReferenceSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWeakReferenceSource {
    type Vtable = IWeakReferenceSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000038_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWeakReferenceSource_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetWeakReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, weakreference: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWebAuthenticationCoreManagerInterop(::windows_core::IUnknown);
impl IWebAuthenticationCoreManagerInterop {
    pub unsafe fn RequestTokenForWindowAsync<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, T: ::windows_core::Interface>(&self, appwindow: Param0, request: Param1) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).RequestTokenForWindowAsync)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), request.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn RequestTokenWithWebAccountForWindowAsync<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param2: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, T: ::windows_core::Interface>(&self, appwindow: Param0, request: Param1, webaccount: Param2) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).RequestTokenWithWebAccountForWindowAsync)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), request.into_param().abi(), webaccount.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IWebAuthenticationCoreManagerInterop> for ::windows_core::IUnknown {
    fn from(value: IWebAuthenticationCoreManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAuthenticationCoreManagerInterop> for ::windows_core::IUnknown {
    fn from(value: &IWebAuthenticationCoreManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWebAuthenticationCoreManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWebAuthenticationCoreManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWebAuthenticationCoreManagerInterop> for ::windows_core::IInspectable {
    fn from(value: IWebAuthenticationCoreManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAuthenticationCoreManagerInterop> for ::windows_core::IInspectable {
    fn from(value: &IWebAuthenticationCoreManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IWebAuthenticationCoreManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IWebAuthenticationCoreManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWebAuthenticationCoreManagerInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebAuthenticationCoreManagerInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebAuthenticationCoreManagerInterop {}
impl ::core::fmt::Debug for IWebAuthenticationCoreManagerInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebAuthenticationCoreManagerInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWebAuthenticationCoreManagerInterop {
    type Vtable = IWebAuthenticationCoreManagerInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf4b8e804_811e_4436_b69c_44cb67b72084);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationCoreManagerInterop_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestTokenForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::win32_foundation::HWND, request: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, asyncinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RequestTokenWithWebAccountForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::win32_foundation::HWND, request: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, asyncinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[inline]
pub unsafe fn IsErrorPropagationEnabled() -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsErrorPropagationEnabled() -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(IsErrorPropagationEnabled())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const MAX_ERROR_MESSAGE_CHARS: u32 = 512u32;
#[inline]
pub unsafe fn MetaDataGetDispenser(rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MetaDataGetDispenser(rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        MetaDataGetDispenser(::core::mem::transmute(rclsid), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub type PINSPECT_HSTRING_CALLBACK = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, readaddress: usize, length: u32, buffer: *mut u8) -> ::windows_core::HRESULT>;
pub type PINSPECT_HSTRING_CALLBACK2 = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, readaddress: u64, length: u32, buffer: *mut u8) -> ::windows_core::HRESULT>;
pub type PINSPECT_MEMORY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, readaddress: usize, length: u32, buffer: *mut u8) -> ::windows_core::HRESULT>;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ROPARAMIIDHANDLE(pub isize);
impl ROPARAMIIDHANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for ROPARAMIIDHANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for ROPARAMIIDHANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for ROPARAMIIDHANDLE {}
impl ::core::fmt::Debug for ROPARAMIIDHANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ROPARAMIIDHANDLE").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Abi for ROPARAMIIDHANDLE {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RO_ERROR_REPORTING_FLAGS(pub u32);
pub const RO_ERROR_REPORTING_NONE: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(0u32);
pub const RO_ERROR_REPORTING_SUPPRESSEXCEPTIONS: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(1u32);
pub const RO_ERROR_REPORTING_FORCEEXCEPTIONS: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(2u32);
pub const RO_ERROR_REPORTING_USESETERRORINFO: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(4u32);
pub const RO_ERROR_REPORTING_SUPPRESSSETERRORINFO: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(8u32);
impl ::core::marker::Copy for RO_ERROR_REPORTING_FLAGS {}
impl ::core::clone::Clone for RO_ERROR_REPORTING_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RO_ERROR_REPORTING_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RO_ERROR_REPORTING_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for RO_ERROR_REPORTING_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RO_ERROR_REPORTING_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for RO_ERROR_REPORTING_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RO_ERROR_REPORTING_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RO_ERROR_REPORTING_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RO_ERROR_REPORTING_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RO_ERROR_REPORTING_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RO_INIT_TYPE(pub i32);
pub const RO_INIT_SINGLETHREADED: RO_INIT_TYPE = RO_INIT_TYPE(0i32);
pub const RO_INIT_MULTITHREADED: RO_INIT_TYPE = RO_INIT_TYPE(1i32);
impl ::core::marker::Copy for RO_INIT_TYPE {}
impl ::core::clone::Clone for RO_INIT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RO_INIT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RO_INIT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RO_INIT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RO_INIT_TYPE").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn RoActivateInstance<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(activatableclassid: Param0) -> ::windows_core::Result<::windows_core::IInspectable> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoActivateInstance(activatableclassid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, instance: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        RoActivateInstance(activatableclassid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IInspectable>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RoCaptureErrorContext(hr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoCaptureErrorContext(hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT;
        }
        RoCaptureErrorContext(::core::mem::transmute(hr)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RoClearError() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoClearError();
        }
        RoClearError()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RoFailFastWithErrorContext(hrerror: ::windows_core::HRESULT) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoFailFastWithErrorContext(hrerror: ::windows_core::HRESULT);
        }
        RoFailFastWithErrorContext(::core::mem::transmute(hrerror))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RoFreeParameterizedTypeExtra<'a, Param0: ::windows_core::IntoParam<'a, ROPARAMIIDHANDLE>>(extra: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoFreeParameterizedTypeExtra(extra: ROPARAMIIDHANDLE);
        }
        RoFreeParameterizedTypeExtra(extra.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RoGetActivationFactory<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, T: ::windows_core::Interface>(activatableclassid: Param0) -> ::windows_core::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoGetActivationFactory(activatableclassid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, iid: *const ::windows_core::GUID, factory: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::option::Option::None;
        RoGetActivationFactory(activatableclassid.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RoGetAgileReference<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(options: AgileReferenceOptions, riid: *const ::windows_core::GUID, punk: Param2) -> ::windows_core::Result<IAgileReference> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoGetAgileReference(options: AgileReferenceOptions, riid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void, ppagilereference: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        RoGetAgileReference(::core::mem::transmute(options), ::core::mem::transmute(riid), punk.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAgileReference>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RoGetApartmentIdentifier() -> ::windows_core::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoGetApartmentIdentifier(apartmentidentifier: *mut u64) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        RoGetApartmentIdentifier(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-system")]
#[inline]
pub unsafe fn RoGetBufferMarshaler() -> ::windows_core::Result<super::Com::Marshal::IMarshal> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoGetBufferMarshaler(buffermarshaler: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        RoGetBufferMarshaler(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::Marshal::IMarshal>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RoGetErrorReportingFlags() -> ::windows_core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoGetErrorReportingFlags(pflags: *mut u32) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        RoGetErrorReportingFlags(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RoGetMatchingRestrictedErrorInfo(hrin: ::windows_core::HRESULT) -> ::windows_core::Result<IRestrictedErrorInfo> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoGetMatchingRestrictedErrorInfo(hrin: ::windows_core::HRESULT, pprestrictederrorinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        RoGetMatchingRestrictedErrorInfo(::core::mem::transmute(hrin), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRestrictedErrorInfo>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RoGetParameterizedTypeInstanceIID<'a, Param2: ::windows_core::IntoParam<'a, IRoMetaDataLocator>>(nameelements: &[::windows_core::PWSTR], metadatalocator: Param2, iid: *mut ::windows_core::GUID, pextra: *mut ROPARAMIIDHANDLE) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoGetParameterizedTypeInstanceIID(nameelementcount: u32, nameelements: *const ::windows_core::PWSTR, metadatalocator: ::windows_core::RawPtr, iid: *mut ::windows_core::GUID, pextra: *mut ROPARAMIIDHANDLE) -> ::windows_core::HRESULT;
        }
        RoGetParameterizedTypeInstanceIID(nameelements.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(nameelements)), metadatalocator.into_param().abi(), ::core::mem::transmute(iid), ::core::mem::transmute(pextra)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RoGetServerActivatableClasses<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(servername: Param0, activatableclassids: *mut *mut ::windows_core::HSTRING, count: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoGetServerActivatableClasses(servername: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, activatableclassids: *mut *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>, count: *mut u32) -> ::windows_core::HRESULT;
        }
        RoGetServerActivatableClasses(servername.into_param().abi(), ::core::mem::transmute(activatableclassids), ::core::mem::transmute(count)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RoInitialize(inittype: RO_INIT_TYPE) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoInitialize(inittype: RO_INIT_TYPE) -> ::windows_core::HRESULT;
        }
        RoInitialize(::core::mem::transmute(inittype)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RoInspectCapturedStackBackTrace(targeterrorinfoaddress: usize, machine: u16, readmemorycallback: PINSPECT_MEMORY_CALLBACK, context: *const ::core::ffi::c_void, framecount: *mut u32, targetbacktraceaddress: *mut usize) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoInspectCapturedStackBackTrace(targeterrorinfoaddress: usize, machine: u16, readmemorycallback: ::windows_core::RawPtr, context: *const ::core::ffi::c_void, framecount: *mut u32, targetbacktraceaddress: *mut usize) -> ::windows_core::HRESULT;
        }
        RoInspectCapturedStackBackTrace(::core::mem::transmute(targeterrorinfoaddress), ::core::mem::transmute(machine), ::core::mem::transmute(readmemorycallback), ::core::mem::transmute(context), ::core::mem::transmute(framecount), ::core::mem::transmute(targetbacktraceaddress)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RoInspectThreadErrorInfo(targettebaddress: usize, machine: u16, readmemorycallback: PINSPECT_MEMORY_CALLBACK, context: *const ::core::ffi::c_void) -> ::windows_core::Result<usize> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoInspectThreadErrorInfo(targettebaddress: usize, machine: u16, readmemorycallback: ::windows_core::RawPtr, context: *const ::core::ffi::c_void, targeterrorinfoaddress: *mut usize) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<usize>::zeroed();
        RoInspectThreadErrorInfo(::core::mem::transmute(targettebaddress), ::core::mem::transmute(machine), ::core::mem::transmute(readmemorycallback), ::core::mem::transmute(context), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<usize>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RoOriginateError<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(error: ::windows_core::HRESULT, message: Param1) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoOriginateError(error: ::windows_core::HRESULT, message: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(RoOriginateError(::core::mem::transmute(error), message.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RoOriginateErrorW(error: ::windows_core::HRESULT, cchmax: u32, message: &[u16; 512]) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoOriginateErrorW(error: ::windows_core::HRESULT, cchmax: u32, message: ::windows_core::PCWSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(RoOriginateErrorW(::core::mem::transmute(error), ::core::mem::transmute(cchmax), ::core::mem::transmute(::windows_core::as_ptr_or_null(message))))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RoOriginateLanguageException<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(error: ::windows_core::HRESULT, message: Param1, languageexception: Param2) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoOriginateLanguageException(error: ::windows_core::HRESULT, message: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, languageexception: *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(RoOriginateLanguageException(::core::mem::transmute(error), message.into_param().abi(), languageexception.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RoParameterizedTypeExtraGetTypeSignature<'a, Param0: ::windows_core::IntoParam<'a, ROPARAMIIDHANDLE>>(extra: Param0) -> ::windows_core::PSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoParameterizedTypeExtraGetTypeSignature(extra: ROPARAMIIDHANDLE) -> ::windows_core::PSTR;
        }
        ::core::mem::transmute(RoParameterizedTypeExtraGetTypeSignature(extra.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RoRegisterActivationFactories(activatableclassids: *const ::windows_core::HSTRING, activationfactorycallbacks: *const isize, count: u32) -> ::windows_core::Result<isize> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoRegisterActivationFactories(activatableclassids: *const ::core::mem::ManuallyDrop<::windows_core::HSTRING>, activationfactorycallbacks: *const isize, count: u32, cookie: *mut isize) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<isize>::zeroed();
        RoRegisterActivationFactories(::core::mem::transmute(activatableclassids), ::core::mem::transmute(activationfactorycallbacks), ::core::mem::transmute(count), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<isize>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RoRegisterForApartmentShutdown<'a, Param0: ::windows_core::IntoParam<'a, IApartmentShutdown>>(callbackobject: Param0, apartmentidentifier: *mut u64, regcookie: *mut APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoRegisterForApartmentShutdown(callbackobject: ::windows_core::RawPtr, apartmentidentifier: *mut u64, regcookie: *mut APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> ::windows_core::HRESULT;
        }
        RoRegisterForApartmentShutdown(callbackobject.into_param().abi(), ::core::mem::transmute(apartmentidentifier), ::core::mem::transmute(regcookie)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RoReportFailedDelegate<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param1: ::windows_core::IntoParam<'a, IRestrictedErrorInfo>>(punkdelegate: Param0, prestrictederrorinfo: Param1) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoReportFailedDelegate(punkdelegate: *mut ::core::ffi::c_void, prestrictederrorinfo: ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        RoReportFailedDelegate(punkdelegate.into_param().abi(), prestrictederrorinfo.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RoReportUnhandledError<'a, Param0: ::windows_core::IntoParam<'a, IRestrictedErrorInfo>>(prestrictederrorinfo: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoReportUnhandledError(prestrictederrorinfo: ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        RoReportUnhandledError(prestrictederrorinfo.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RoResolveRestrictedErrorInfoReference<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(reference: Param0) -> ::windows_core::Result<IRestrictedErrorInfo> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoResolveRestrictedErrorInfoReference(reference: ::windows_core::PCWSTR, pprestrictederrorinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        RoResolveRestrictedErrorInfoReference(reference.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRestrictedErrorInfo>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RoRevokeActivationFactories(cookie: isize) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoRevokeActivationFactories(cookie: isize);
        }
        RoRevokeActivationFactories(::core::mem::transmute(cookie))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RoSetErrorReportingFlags(flags: u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoSetErrorReportingFlags(flags: u32) -> ::windows_core::HRESULT;
        }
        RoSetErrorReportingFlags(::core::mem::transmute(flags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RoTransformError<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(olderror: ::windows_core::HRESULT, newerror: ::windows_core::HRESULT, message: Param2) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoTransformError(olderror: ::windows_core::HRESULT, newerror: ::windows_core::HRESULT, message: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(RoTransformError(::core::mem::transmute(olderror), ::core::mem::transmute(newerror), message.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RoTransformErrorW(olderror: ::windows_core::HRESULT, newerror: ::windows_core::HRESULT, cchmax: u32, message: &[u16; 512]) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoTransformErrorW(olderror: ::windows_core::HRESULT, newerror: ::windows_core::HRESULT, cchmax: u32, message: ::windows_core::PCWSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(RoTransformErrorW(::core::mem::transmute(olderror), ::core::mem::transmute(newerror), ::core::mem::transmute(cchmax), ::core::mem::transmute(::windows_core::as_ptr_or_null(message))))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RoUninitialize() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoUninitialize();
        }
        RoUninitialize()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RoUnregisterForApartmentShutdown<'a, Param0: ::windows_core::IntoParam<'a, APARTMENT_SHUTDOWN_REGISTRATION_COOKIE>>(regcookie: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoUnregisterForApartmentShutdown(regcookie: APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> ::windows_core::HRESULT;
        }
        RoUnregisterForApartmentShutdown(regcookie.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
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
impl ::core::fmt::Debug for ServerInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ServerInformation").field("dwServerPid", &self.dwServerPid).field("dwServerTid", &self.dwServerTid).field("ui64ServerAddress", &self.ui64ServerAddress).finish()
    }
}
unsafe impl ::windows_core::Abi for ServerInformation {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ServerInformation {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ServerInformation>()) == 0 }
    }
}
impl ::core::cmp::Eq for ServerInformation {}
impl ::core::default::Default for ServerInformation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[inline]
pub unsafe fn SetRestrictedErrorInfo<'a, Param0: ::windows_core::IntoParam<'a, IRestrictedErrorInfo>>(prestrictederrorinfo: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetRestrictedErrorInfo(prestrictederrorinfo: ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        SetRestrictedErrorInfo(prestrictederrorinfo.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TrustLevel(pub i32);
pub const BaseTrust: TrustLevel = TrustLevel(0i32);
pub const PartialTrust: TrustLevel = TrustLevel(1i32);
pub const FullTrust: TrustLevel = TrustLevel(2i32);
impl ::core::marker::Copy for TrustLevel {}
impl ::core::clone::Clone for TrustLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TrustLevel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TrustLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for TrustLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TrustLevel").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn WindowsCompareStringOrdinal<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(string1: Param0, string2: Param1) -> ::windows_core::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsCompareStringOrdinal(string1: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, string2: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result: *mut i32) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        WindowsCompareStringOrdinal(string1.into_param().abi(), string2.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WindowsConcatString<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(string1: Param0, string2: Param1) -> ::windows_core::Result<::windows_core::HSTRING> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsConcatString(string1: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, string2: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, newstring: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
        WindowsConcatString(string1.into_param().abi(), string2.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::HSTRING>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WindowsCreateString(sourcestring: &[u16]) -> ::windows_core::Result<::windows_core::HSTRING> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsCreateString(sourcestring: ::windows_core::PCWSTR, length: u32, string: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
        WindowsCreateString(::core::mem::transmute(::windows_core::as_ptr_or_null(sourcestring)), sourcestring.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::HSTRING>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WindowsCreateStringReference<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(sourcestring: Param0, length: u32, hstringheader: *mut HSTRING_HEADER, string: *mut ::windows_core::HSTRING) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsCreateStringReference(sourcestring: ::windows_core::PCWSTR, length: u32, hstringheader: *mut HSTRING_HEADER, string: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT;
        }
        WindowsCreateStringReference(sourcestring.into_param().abi(), ::core::mem::transmute(length), ::core::mem::transmute(hstringheader), ::core::mem::transmute(string)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WindowsDeleteString<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(string: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsDeleteString(string: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT;
        }
        WindowsDeleteString(string.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WindowsDeleteStringBuffer<'a, Param0: ::windows_core::IntoParam<'a, HSTRING_BUFFER>>(bufferhandle: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsDeleteStringBuffer(bufferhandle: HSTRING_BUFFER) -> ::windows_core::HRESULT;
        }
        WindowsDeleteStringBuffer(bufferhandle.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WindowsDuplicateString<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(string: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsDuplicateString(string: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, newstring: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
        WindowsDuplicateString(string.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::HSTRING>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WindowsGetStringLen<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(string: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsGetStringLen(string: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> u32;
        }
        ::core::mem::transmute(WindowsGetStringLen(string.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WindowsGetStringRawBuffer<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(string: Param0, length: *mut u32) -> ::windows_core::PWSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsGetStringRawBuffer(string: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, length: *mut u32) -> ::windows_core::PWSTR;
        }
        ::core::mem::transmute(WindowsGetStringRawBuffer(string.into_param().abi(), ::core::mem::transmute(length)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WindowsInspectString(targethstring: usize, machine: u16, callback: PINSPECT_HSTRING_CALLBACK, context: *const ::core::ffi::c_void, length: *mut u32, targetstringaddress: *mut usize) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsInspectString(targethstring: usize, machine: u16, callback: ::windows_core::RawPtr, context: *const ::core::ffi::c_void, length: *mut u32, targetstringaddress: *mut usize) -> ::windows_core::HRESULT;
        }
        WindowsInspectString(::core::mem::transmute(targethstring), ::core::mem::transmute(machine), ::core::mem::transmute(callback), ::core::mem::transmute(context), ::core::mem::transmute(length), ::core::mem::transmute(targetstringaddress)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WindowsInspectString2(targethstring: u64, machine: u16, callback: PINSPECT_HSTRING_CALLBACK2, context: *const ::core::ffi::c_void, length: *mut u32, targetstringaddress: *mut u64) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsInspectString2(targethstring: u64, machine: u16, callback: ::windows_core::RawPtr, context: *const ::core::ffi::c_void, length: *mut u32, targetstringaddress: *mut u64) -> ::windows_core::HRESULT;
        }
        WindowsInspectString2(::core::mem::transmute(targethstring), ::core::mem::transmute(machine), ::core::mem::transmute(callback), ::core::mem::transmute(context), ::core::mem::transmute(length), ::core::mem::transmute(targetstringaddress)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WindowsIsStringEmpty<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(string: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsIsStringEmpty(string: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(WindowsIsStringEmpty(string.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WindowsPreallocateStringBuffer(length: u32, charbuffer: *mut *mut u16, bufferhandle: *mut HSTRING_BUFFER) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsPreallocateStringBuffer(length: u32, charbuffer: *mut *mut u16, bufferhandle: *mut HSTRING_BUFFER) -> ::windows_core::HRESULT;
        }
        WindowsPreallocateStringBuffer(::core::mem::transmute(length), ::core::mem::transmute(charbuffer), ::core::mem::transmute(bufferhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WindowsPromoteStringBuffer<'a, Param0: ::windows_core::IntoParam<'a, HSTRING_BUFFER>>(bufferhandle: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsPromoteStringBuffer(bufferhandle: HSTRING_BUFFER, string: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
        WindowsPromoteStringBuffer(bufferhandle.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::HSTRING>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WindowsReplaceString<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(string: Param0, stringreplaced: Param1, stringreplacewith: Param2) -> ::windows_core::Result<::windows_core::HSTRING> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsReplaceString(string: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, stringreplaced: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, stringreplacewith: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, newstring: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
        WindowsReplaceString(string.into_param().abi(), stringreplaced.into_param().abi(), stringreplacewith.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::HSTRING>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WindowsStringHasEmbeddedNull<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(string: Param0) -> ::windows_core::Result<::win32_foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsStringHasEmbeddedNull(string: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, hasembednull: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        WindowsStringHasEmbeddedNull(string.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WindowsSubstring<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(string: Param0, startindex: u32) -> ::windows_core::Result<::windows_core::HSTRING> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsSubstring(string: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, startindex: u32, newstring: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
        WindowsSubstring(string.into_param().abi(), ::core::mem::transmute(startindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::HSTRING>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WindowsSubstringWithSpecifiedLength<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(string: Param0, startindex: u32, length: u32) -> ::windows_core::Result<::windows_core::HSTRING> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsSubstringWithSpecifiedLength(string: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, startindex: u32, length: u32, newstring: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
        WindowsSubstringWithSpecifiedLength(string.into_param().abi(), ::core::mem::transmute(startindex), ::core::mem::transmute(length), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::HSTRING>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WindowsTrimStringEnd<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(string: Param0, trimstring: Param1) -> ::windows_core::Result<::windows_core::HSTRING> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsTrimStringEnd(string: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, trimstring: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, newstring: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
        WindowsTrimStringEnd(string.into_param().abi(), trimstring.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::HSTRING>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WindowsTrimStringStart<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(string: Param0, trimstring: Param1) -> ::windows_core::Result<::windows_core::HSTRING> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsTrimStringStart(string: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, trimstring: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, newstring: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
        WindowsTrimStringStart(string.into_param().abi(), trimstring.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::HSTRING>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct _RO_REGISTRATION_COOKIE(pub u8);
