pub type HttpDiagnosticProvider = *mut ::core::ffi::c_void;
pub type HttpDiagnosticProviderRequestResponseCompletedEventArgs = *mut ::core::ffi::c_void;
pub type HttpDiagnosticProviderRequestResponseTimestamps = *mut ::core::ffi::c_void;
pub type HttpDiagnosticProviderRequestSentEventArgs = *mut ::core::ffi::c_void;
pub type HttpDiagnosticProviderResponseReceivedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Web_Http_Diagnostics\"`*"]
#[repr(transparent)]
pub struct HttpDiagnosticRequestInitiator(pub i32);
impl HttpDiagnosticRequestInitiator {
    pub const ParsedElement: Self = Self(0i32);
    pub const Script: Self = Self(1i32);
    pub const Image: Self = Self(2i32);
    pub const Link: Self = Self(3i32);
    pub const Style: Self = Self(4i32);
    pub const XmlHttpRequest: Self = Self(5i32);
    pub const Media: Self = Self(6i32);
    pub const HtmlDownload: Self = Self(7i32);
    pub const Prefetch: Self = Self(8i32);
    pub const Other: Self = Self(9i32);
    pub const CrossOriginPreFlight: Self = Self(10i32);
    pub const Fetch: Self = Self(11i32);
    pub const Beacon: Self = Self(12i32);
}
impl ::core::marker::Copy for HttpDiagnosticRequestInitiator {}
impl ::core::clone::Clone for HttpDiagnosticRequestInitiator {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HttpDiagnosticSourceLocation = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IHttpDiagnosticProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestSent: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSent: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRequestSent: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRequestSent: usize,
    #[cfg(feature = "Foundation")]
    pub ResponseReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResponseReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveResponseReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveResponseReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RequestResponseCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestResponseCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRequestResponseCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRequestResponseCompleted: usize,
}
impl ::windows_sys::core::Interface for IHttpDiagnosticProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3179353345, data2: 41046, data3: 19769, data4: [177, 116, 131, 59, 123, 3, 176, 44] };
}
#[repr(C)]
pub struct IHttpDiagnosticProviderRequestResponseCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ActivityId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Timestamps: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestedUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestedUri: usize,
    pub ProcessId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ThreadId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Initiator: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HttpDiagnosticRequestInitiator) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SourceLocations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SourceLocations: usize,
}
impl ::windows_sys::core::Interface for IHttpDiagnosticProviderRequestResponseCompletedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1935644910, data2: 38134, data3: 17714, data4: [178, 110, 97, 225, 177, 228, 239, 212] };
}
#[repr(C)]
pub struct IHttpDiagnosticProviderRequestResponseTimestamps {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CacheCheckedTimestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CacheCheckedTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectionInitiatedTimestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionInitiatedTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub NameResolvedTimestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NameResolvedTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub SslNegotiatedTimestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SslNegotiatedTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectionCompletedTimestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionCompletedTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub RequestSentTimestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSentTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub RequestCompletedTimestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestCompletedTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub ResponseReceivedTimestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResponseReceivedTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub ResponseCompletedTimestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResponseCompletedTimestamp: usize,
}
impl ::windows_sys::core::Interface for IHttpDiagnosticProviderRequestResponseTimestamps {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3769622032, data2: 21967, data3: 19457, data4: [145, 212, 162, 5, 87, 216, 73, 240] };
}
#[repr(C)]
pub struct IHttpDiagnosticProviderRequestSentEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub ActivityId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ProcessId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ThreadId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Initiator: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HttpDiagnosticRequestInitiator) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SourceLocations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SourceLocations: usize,
}
impl ::windows_sys::core::Interface for IHttpDiagnosticProviderRequestSentEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1062311632, data2: 19487, data3: 20158, data4: [165, 122, 6, 147, 7, 113, 197, 13] };
}
#[repr(C)]
pub struct IHttpDiagnosticProviderResponseReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub ActivityId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpDiagnosticProviderResponseReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2694993516, data2: 43871, data3: 19814, data4: [187, 45, 8, 76, 244, 22, 53, 208] };
}
#[repr(C)]
pub struct IHttpDiagnosticProviderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System_Diagnostics")]
    pub CreateFromProcessDiagnosticInfo: unsafe extern "system" fn(this: *mut *mut Self, processdiagnosticinfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System_Diagnostics"))]
    CreateFromProcessDiagnosticInfo: usize,
}
impl ::windows_sys::core::Interface for IHttpDiagnosticProviderStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1535266497, data2: 27244, data3: 18380, data4: [175, 236, 30, 134, 188, 38, 5, 59] };
}
#[repr(C)]
pub struct IHttpDiagnosticSourceLocation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SourceUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceUri: usize,
    pub LineNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub ColumnNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpDiagnosticSourceLocation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1420415584, data2: 34912, data3: 16959, data4: [182, 250, 215, 119, 22, 246, 71, 167] };
}
