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
#[repr(C)]
pub struct IHttpBaseProtocolFilter2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::HttpVersion) -> ::windows_sys::core::HRESULT,
    pub SetMaxVersion: unsafe extern "system" fn(this: *mut *mut Self, value: super::HttpVersion) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpBaseProtocolFilter3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CookieUsageBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HttpCookieUsageBehavior) -> ::windows_sys::core::HRESULT,
    pub SetCookieUsageBehavior: unsafe extern "system" fn(this: *mut *mut Self, value: HttpCookieUsageBehavior) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IHttpBaseProtocolFilter5 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[repr(C)]
pub struct IHttpBaseProtocolFilterStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub CreateForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    CreateForUser: usize,
}
#[repr(C)]
pub struct IHttpCacheControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReadBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HttpCacheReadBehavior) -> ::windows_sys::core::HRESULT,
    pub SetReadBehavior: unsafe extern "system" fn(this: *mut *mut Self, value: HttpCacheReadBehavior) -> ::windows_sys::core::HRESULT,
    pub WriteBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HttpCacheWriteBehavior) -> ::windows_sys::core::HRESULT,
    pub SetWriteBehavior: unsafe extern "system" fn(this: *mut *mut Self, value: HttpCacheWriteBehavior) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpFilter {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SendRequestAsync: unsafe extern "system" fn(this: *mut *mut Self, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendRequestAsync: usize,
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
