pub type HttpBaseProtocolFilter = *mut ::core::ffi::c_void;
pub type HttpCacheControl = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Web_Http_Filters\"`*"]
#[repr(transparent)]
pub struct HttpCacheReadBehavior(pub i32);
impl HttpCacheReadBehavior {
    pub const Default: Self = Self(0i32);
    pub const MostRecent: Self = Self(1i32);
    pub const OnlyFromCache: Self = Self(2i32);
    pub const NoCache: Self = Self(3i32);
}
impl ::core::marker::Copy for HttpCacheReadBehavior {}
impl ::core::clone::Clone for HttpCacheReadBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Web_Http_Filters\"`*"]
#[repr(transparent)]
pub struct HttpCacheWriteBehavior(pub i32);
impl HttpCacheWriteBehavior {
    pub const Default: Self = Self(0i32);
    pub const NoCache: Self = Self(1i32);
}
impl ::core::marker::Copy for HttpCacheWriteBehavior {}
impl ::core::clone::Clone for HttpCacheWriteBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Web_Http_Filters\"`*"]
#[repr(transparent)]
pub struct HttpCookieUsageBehavior(pub i32);
impl HttpCookieUsageBehavior {
    pub const Default: Self = Self(0i32);
    pub const NoCookies: Self = Self(1i32);
}
impl ::core::marker::Copy for HttpCookieUsageBehavior {}
impl ::core::clone::Clone for HttpCookieUsageBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HttpServerCustomValidationRequestedEventArgs = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IHttpBaseProtocolFilter {
    pub base__: ::windows_sys::core::IInspectable,
    pub AllowAutoRedirect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowAutoRedirect: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AllowUI: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowUI: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AutomaticDecompression: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAutomaticDecompression: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CacheControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CookieManager: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub ClientCertificate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    ClientCertificate: usize,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub SetClientCertificate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    SetClientCertificate: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub IgnorableServerCertificateErrors: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    IgnorableServerCertificateErrors: usize,
    pub MaxConnectionsPerServer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMaxConnectionsPerServer: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub ProxyCredential: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    ProxyCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetProxyCredential: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetProxyCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub ServerCredential: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    ServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetServerCredential: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetServerCredential: usize,
    pub UseProxy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetUseProxy: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpBaseProtocolFilter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1908972297, data2: 57649, data3: 19284, data4: [165, 60, 235, 67, 255, 55, 233, 187] };
}
#[repr(C)]
pub struct IHttpBaseProtocolFilter2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::HttpVersion) -> ::windows_sys::core::HRESULT,
    pub SetMaxVersion: unsafe extern "system" fn(this: *mut *mut Self, value: super::HttpVersion) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpBaseProtocolFilter2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 784531475, data2: 37927, data3: 18688, data4: [160, 23, 250, 125, 163, 181, 201, 174] };
}
#[repr(C)]
pub struct IHttpBaseProtocolFilter3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CookieUsageBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HttpCookieUsageBehavior) -> ::windows_sys::core::HRESULT,
    pub SetCookieUsageBehavior: unsafe extern "system" fn(this: *mut *mut Self, value: HttpCookieUsageBehavior) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpBaseProtocolFilter3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3560918348, data2: 48450, data3: 17326, data4: [135, 23, 173, 44, 143, 75, 41, 55] };
}
#[repr(C)]
pub struct IHttpBaseProtocolFilter4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ServerCustomValidationRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServerCustomValidationRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveServerCustomValidationRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveServerCustomValidationRequested: usize,
    pub ClearAuthenticationCache: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpBaseProtocolFilter4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2682481871, data2: 10627, data3: 18579, data4: [148, 31, 235, 81, 140, 168, 206, 249] };
}
#[repr(C)]
pub struct IHttpBaseProtocolFilter5 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
impl ::windows_sys::core::Interface for IHttpBaseProtocolFilter5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1097746835, data2: 12771, data3: 18454, data4: [191, 9, 224, 24, 238, 141, 193, 245] };
}
#[repr(C)]
pub struct IHttpBaseProtocolFilterStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub CreateForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    CreateForUser: usize,
}
impl ::windows_sys::core::Interface for IHttpBaseProtocolFilterStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1833823756, data2: 59656, data3: 18766, data4: [181, 163, 18, 99, 201, 184, 36, 42] };
}
#[repr(C)]
pub struct IHttpCacheControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReadBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HttpCacheReadBehavior) -> ::windows_sys::core::HRESULT,
    pub SetReadBehavior: unsafe extern "system" fn(this: *mut *mut Self, value: HttpCacheReadBehavior) -> ::windows_sys::core::HRESULT,
    pub WriteBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HttpCacheWriteBehavior) -> ::windows_sys::core::HRESULT,
    pub SetWriteBehavior: unsafe extern "system" fn(this: *mut *mut Self, value: HttpCacheWriteBehavior) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpCacheControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3346930868, data2: 15594, data3: 20149, data4: [172, 133, 4, 225, 134, 230, 58, 183] };
}
#[repr(C)]
pub struct IHttpFilter {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SendRequestAsync: unsafe extern "system" fn(this: *mut *mut Self, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendRequestAsync: usize,
}
impl ::windows_sys::core::Interface for IHttpFilter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2764795349, data2: 2306, data3: 17310, data4: [191, 215, 225, 37, 82, 177, 101, 206] };
}
#[repr(C)]
pub struct IHttpServerCustomValidationRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequestMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub ServerCertificate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    ServerCertificate: usize,
    #[cfg(feature = "Networking_Sockets")]
    pub ServerCertificateErrorSeverity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Networking::Sockets::SocketSslErrorSeverity) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    ServerCertificateErrorSeverity: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ServerCertificateErrors: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ServerCertificateErrors: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ServerIntermediateCertificates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ServerIntermediateCertificates: usize,
    pub Reject: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IHttpServerCustomValidationRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 828767794, data2: 59357, data3: 18615, data4: [163, 97, 147, 156, 117, 14, 99, 204] };
}
