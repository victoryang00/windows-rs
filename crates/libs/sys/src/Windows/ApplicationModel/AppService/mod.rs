pub type AppServiceClosedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_AppService\"`*"]
#[repr(transparent)]
pub struct AppServiceClosedStatus(pub i32);
impl AppServiceClosedStatus {
    pub const Completed: Self = Self(0i32);
    pub const Canceled: Self = Self(1i32);
    pub const ResourceLimitsExceeded: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
}
impl ::core::marker::Copy for AppServiceClosedStatus {}
impl ::core::clone::Clone for AppServiceClosedStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppServiceConnection = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_AppService\"`*"]
#[repr(transparent)]
pub struct AppServiceConnectionStatus(pub i32);
impl AppServiceConnectionStatus {
    pub const Success: Self = Self(0i32);
    pub const AppNotInstalled: Self = Self(1i32);
    pub const AppUnavailable: Self = Self(2i32);
    pub const AppServiceUnavailable: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
    pub const RemoteSystemUnavailable: Self = Self(5i32);
    pub const RemoteSystemNotSupportedByApp: Self = Self(6i32);
    pub const NotAuthorized: Self = Self(7i32);
    pub const AuthenticationError: Self = Self(8i32);
    pub const NetworkNotAvailable: Self = Self(9i32);
    pub const DisabledByPolicy: Self = Self(10i32);
    pub const WebServiceUnavailable: Self = Self(11i32);
}
impl ::core::marker::Copy for AppServiceConnectionStatus {}
impl ::core::clone::Clone for AppServiceConnectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppServiceDeferral = *mut ::core::ffi::c_void;
pub type AppServiceRequest = *mut ::core::ffi::c_void;
pub type AppServiceRequestReceivedEventArgs = *mut ::core::ffi::c_void;
pub type AppServiceResponse = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_AppService\"`*"]
#[repr(transparent)]
pub struct AppServiceResponseStatus(pub i32);
impl AppServiceResponseStatus {
    pub const Success: Self = Self(0i32);
    pub const Failure: Self = Self(1i32);
    pub const ResourceLimitsExceeded: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
    pub const RemoteSystemUnavailable: Self = Self(4i32);
    pub const MessageSizeTooLarge: Self = Self(5i32);
    pub const AppUnavailable: Self = Self(6i32);
    pub const AuthenticationError: Self = Self(7i32);
    pub const NetworkNotAvailable: Self = Self(8i32);
    pub const DisabledByPolicy: Self = Self(9i32);
    pub const WebServiceUnavailable: Self = Self(10i32);
}
impl ::core::marker::Copy for AppServiceResponseStatus {}
impl ::core::clone::Clone for AppServiceResponseStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppServiceTriggerDetails = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAppServiceCatalogStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAppServiceProvidersAsync: unsafe extern "system" fn(this: *mut *mut Self, appservicename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAppServiceProvidersAsync: usize,
}
impl ::windows_sys::core::Interface for IAppServiceCatalogStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4010616071, data2: 53554, data3: 19589, data4: [131, 149, 60, 49, 213, 161, 233, 65] };
}
#[repr(C)]
pub struct IAppServiceClosedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppServiceClosedStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppServiceClosedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3730839286, data2: 51971, data3: 19765, data4: [172, 141, 204, 99, 3, 35, 151, 49] };
}
#[repr(C)]
pub struct IAppServiceConnection {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppServiceName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetAppServiceName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub OpenAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SendMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SendMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRequestReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRequestReceived: usize,
    #[cfg(feature = "Foundation")]
    pub ServiceClosed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServiceClosed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveServiceClosed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveServiceClosed: usize,
}
impl ::windows_sys::core::Interface for IAppServiceConnection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2647946402, data2: 34591, data3: 19794, data4: [137, 169, 158, 9, 5, 49, 189, 39] };
}
#[repr(C)]
pub struct IAppServiceConnection2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))]
    pub OpenRemoteAsync: unsafe extern "system" fn(this: *mut *mut Self, remotesystemconnectionrequest: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System_RemoteSystems")))]
    OpenRemoteAsync: usize,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
    #[cfg(feature = "System")]
    pub SetUser: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetUser: usize,
}
impl ::windows_sys::core::Interface for IAppServiceConnection2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2346700127, data2: 8962, data3: 20413, data4: [128, 97, 82, 81, 28, 47, 139, 249] };
}
#[repr(C)]
pub struct IAppServiceConnectionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "System_RemoteSystems"))]
    pub SendStatelessMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, connection: *mut ::core::ffi::c_void, connectionrequest: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System_RemoteSystems")))]
    SendStatelessMessageAsync: usize,
}
impl ::windows_sys::core::Interface for IAppServiceConnectionStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2915396841, data2: 54280, data3: 22131, data4: [134, 55, 130, 122, 75, 39, 65, 104] };
}
#[repr(C)]
pub struct IAppServiceDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppServiceDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2115719970, data2: 60080, data3: 16968, data4: [174, 4, 253, 249, 56, 56, 228, 114] };
}
#[repr(C)]
pub struct IAppServiceRequest {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Message: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SendResponseAsync: unsafe extern "system" fn(this: *mut *mut Self, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SendResponseAsync: usize,
}
impl ::windows_sys::core::Interface for IAppServiceRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 551914909, data2: 6366, data3: 19201, data4: [128, 186, 144, 167, 98, 4, 227, 200] };
}
#[repr(C)]
pub struct IAppServiceRequestReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppServiceRequestReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1846682464, data2: 65381, data3: 17582, data4: [158, 69, 133, 127, 228, 24, 6, 129] };
}
#[repr(C)]
pub struct IAppServiceResponse {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Message: usize,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppServiceResponseStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppServiceResponse {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2370845932, data2: 39587, data3: 20072, data4: [149, 89, 157, 230, 62, 55, 44, 228] };
}
#[repr(C)]
pub struct IAppServiceTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CallerPackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AppServiceConnection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppServiceTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2292374700, data2: 44328, data3: 16824, data4: [128, 187, 189, 241, 178, 22, 158, 25] };
}
#[repr(C)]
pub struct IAppServiceTriggerDetails2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsRemoteSystemConnection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppServiceTriggerDetails2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3896333490, data2: 10444, data3: 17394, data4: [180, 101, 192, 72, 46, 89, 226, 220] };
}
#[repr(C)]
pub struct IAppServiceTriggerDetails3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CheckCallerForCapabilityAsync: unsafe extern "system" fn(this: *mut *mut Self, capabilityname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckCallerForCapabilityAsync: usize,
}
impl ::windows_sys::core::Interface for IAppServiceTriggerDetails3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4225179169, data2: 31033, data3: 20072, data4: [158, 60, 119, 128, 20, 122, 171, 182] };
}
#[repr(C)]
pub struct IAppServiceTriggerDetails4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CallerRemoteConnectionToken: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppServiceTriggerDetails4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 293974400, data2: 34913, data3: 24112, data4: [171, 85, 28, 244, 208, 139, 191, 109] };
}
#[repr(C)]
pub struct IStatelessAppServiceResponse {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Message: usize,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StatelessAppServiceResponseStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStatelessAppServiceResponse {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1131760375, data2: 43500, data3: 21246, data4: [130, 231, 147, 155, 104, 220, 147, 136] };
}
pub type StatelessAppServiceResponse = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_AppService\"`*"]
#[repr(transparent)]
pub struct StatelessAppServiceResponseStatus(pub i32);
impl StatelessAppServiceResponseStatus {
    pub const Success: Self = Self(0i32);
    pub const AppNotInstalled: Self = Self(1i32);
    pub const AppUnavailable: Self = Self(2i32);
    pub const AppServiceUnavailable: Self = Self(3i32);
    pub const RemoteSystemUnavailable: Self = Self(4i32);
    pub const RemoteSystemNotSupportedByApp: Self = Self(5i32);
    pub const NotAuthorized: Self = Self(6i32);
    pub const ResourceLimitsExceeded: Self = Self(7i32);
    pub const MessageSizeTooLarge: Self = Self(8i32);
    pub const Failure: Self = Self(9i32);
    pub const Unknown: Self = Self(10i32);
    pub const AuthenticationError: Self = Self(11i32);
    pub const NetworkNotAvailable: Self = Self(12i32);
    pub const DisabledByPolicy: Self = Self(13i32);
    pub const WebServiceUnavailable: Self = Self(14i32);
}
impl ::core::marker::Copy for StatelessAppServiceResponseStatus {}
impl ::core::clone::Clone for StatelessAppServiceResponseStatus {
    fn clone(&self) -> Self {
        *self
    }
}
