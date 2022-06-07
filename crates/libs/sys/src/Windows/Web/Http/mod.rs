#[cfg(feature = "Web_Http_Diagnostics")]
pub mod Diagnostics;
#[cfg(feature = "Web_Http_Filters")]
pub mod Filters;
#[cfg(feature = "Web_Http_Headers")]
pub mod Headers;
pub type HttpBufferContent = *mut ::core::ffi::c_void;
pub type HttpClient = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Web_Http\"`*"]
#[repr(transparent)]
pub struct HttpCompletionOption(pub i32);
impl HttpCompletionOption {
    pub const ResponseContentRead: Self = Self(0i32);
    pub const ResponseHeadersRead: Self = Self(1i32);
}
impl ::core::marker::Copy for HttpCompletionOption {}
impl ::core::clone::Clone for HttpCompletionOption {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HttpCookie = *mut ::core::ffi::c_void;
pub type HttpCookieCollection = *mut ::core::ffi::c_void;
pub type HttpCookieManager = *mut ::core::ffi::c_void;
pub type HttpFormUrlEncodedContent = *mut ::core::ffi::c_void;
pub type HttpGetBufferResult = *mut ::core::ffi::c_void;
pub type HttpGetInputStreamResult = *mut ::core::ffi::c_void;
pub type HttpGetStringResult = *mut ::core::ffi::c_void;
pub type HttpMethod = *mut ::core::ffi::c_void;
pub type HttpMultipartContent = *mut ::core::ffi::c_void;
pub type HttpMultipartFormDataContent = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"Web_Http\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
pub struct HttpProgress {
    pub Stage: HttpProgressStage,
    pub BytesSent: u64,
    pub TotalBytesToSend: *mut *mut *mut *mut super::super::Foundation::IReference<u64>,
    pub BytesReceived: u64,
    pub TotalBytesToReceive: *mut *mut *mut *mut super::super::Foundation::IReference<u64>,
    pub Retries: u32,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for HttpProgress {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for HttpProgress {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Web_Http\"`*"]
#[repr(transparent)]
pub struct HttpProgressStage(pub i32);
impl HttpProgressStage {
    pub const None: Self = Self(0i32);
    pub const DetectingProxy: Self = Self(10i32);
    pub const ResolvingName: Self = Self(20i32);
    pub const ConnectingToServer: Self = Self(30i32);
    pub const NegotiatingSsl: Self = Self(40i32);
    pub const SendingHeaders: Self = Self(50i32);
    pub const SendingContent: Self = Self(60i32);
    pub const WaitingForResponse: Self = Self(70i32);
    pub const ReceivingHeaders: Self = Self(80i32);
    pub const ReceivingContent: Self = Self(90i32);
}
impl ::core::marker::Copy for HttpProgressStage {}
impl ::core::clone::Clone for HttpProgressStage {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HttpRequestMessage = *mut ::core::ffi::c_void;
pub type HttpRequestResult = *mut ::core::ffi::c_void;
pub type HttpResponseMessage = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Web_Http\"`*"]
#[repr(transparent)]
pub struct HttpResponseMessageSource(pub i32);
impl HttpResponseMessageSource {
    pub const None: Self = Self(0i32);
    pub const Cache: Self = Self(1i32);
    pub const Network: Self = Self(2i32);
}
impl ::core::marker::Copy for HttpResponseMessageSource {}
impl ::core::clone::Clone for HttpResponseMessageSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Web_Http\"`*"]
#[repr(transparent)]
pub struct HttpStatusCode(pub i32);
impl HttpStatusCode {
    pub const None: Self = Self(0i32);
    pub const Continue: Self = Self(100i32);
    pub const SwitchingProtocols: Self = Self(101i32);
    pub const Processing: Self = Self(102i32);
    pub const Ok: Self = Self(200i32);
    pub const Created: Self = Self(201i32);
    pub const Accepted: Self = Self(202i32);
    pub const NonAuthoritativeInformation: Self = Self(203i32);
    pub const NoContent: Self = Self(204i32);
    pub const ResetContent: Self = Self(205i32);
    pub const PartialContent: Self = Self(206i32);
    pub const MultiStatus: Self = Self(207i32);
    pub const AlreadyReported: Self = Self(208i32);
    pub const IMUsed: Self = Self(226i32);
    pub const MultipleChoices: Self = Self(300i32);
    pub const MovedPermanently: Self = Self(301i32);
    pub const Found: Self = Self(302i32);
    pub const SeeOther: Self = Self(303i32);
    pub const NotModified: Self = Self(304i32);
    pub const UseProxy: Self = Self(305i32);
    pub const TemporaryRedirect: Self = Self(307i32);
    pub const PermanentRedirect: Self = Self(308i32);
    pub const BadRequest: Self = Self(400i32);
    pub const Unauthorized: Self = Self(401i32);
    pub const PaymentRequired: Self = Self(402i32);
    pub const Forbidden: Self = Self(403i32);
    pub const NotFound: Self = Self(404i32);
    pub const MethodNotAllowed: Self = Self(405i32);
    pub const NotAcceptable: Self = Self(406i32);
    pub const ProxyAuthenticationRequired: Self = Self(407i32);
    pub const RequestTimeout: Self = Self(408i32);
    pub const Conflict: Self = Self(409i32);
    pub const Gone: Self = Self(410i32);
    pub const LengthRequired: Self = Self(411i32);
    pub const PreconditionFailed: Self = Self(412i32);
    pub const RequestEntityTooLarge: Self = Self(413i32);
    pub const RequestUriTooLong: Self = Self(414i32);
    pub const UnsupportedMediaType: Self = Self(415i32);
    pub const RequestedRangeNotSatisfiable: Self = Self(416i32);
    pub const ExpectationFailed: Self = Self(417i32);
    pub const UnprocessableEntity: Self = Self(422i32);
    pub const Locked: Self = Self(423i32);
    pub const FailedDependency: Self = Self(424i32);
    pub const UpgradeRequired: Self = Self(426i32);
    pub const PreconditionRequired: Self = Self(428i32);
    pub const TooManyRequests: Self = Self(429i32);
    pub const RequestHeaderFieldsTooLarge: Self = Self(431i32);
    pub const InternalServerError: Self = Self(500i32);
    pub const NotImplemented: Self = Self(501i32);
    pub const BadGateway: Self = Self(502i32);
    pub const ServiceUnavailable: Self = Self(503i32);
    pub const GatewayTimeout: Self = Self(504i32);
    pub const HttpVersionNotSupported: Self = Self(505i32);
    pub const VariantAlsoNegotiates: Self = Self(506i32);
    pub const InsufficientStorage: Self = Self(507i32);
    pub const LoopDetected: Self = Self(508i32);
    pub const NotExtended: Self = Self(510i32);
    pub const NetworkAuthenticationRequired: Self = Self(511i32);
}
impl ::core::marker::Copy for HttpStatusCode {}
impl ::core::clone::Clone for HttpStatusCode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HttpStreamContent = *mut ::core::ffi::c_void;
pub type HttpStringContent = *mut ::core::ffi::c_void;
pub type HttpTransportInformation = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Web_Http\"`*"]
#[repr(transparent)]
pub struct HttpVersion(pub i32);
impl HttpVersion {
    pub const None: Self = Self(0i32);
    pub const Http10: Self = Self(1i32);
    pub const Http11: Self = Self(2i32);
    pub const Http20: Self = Self(3i32);
}
impl ::core::marker::Copy for HttpVersion {}
impl ::core::clone::Clone for HttpVersion {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IHttpBufferContentFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, content: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBufferWithOffset: unsafe extern "system" fn(this: *mut *mut Self, content: *mut ::core::ffi::c_void, offset: u32, count: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBufferWithOffset: usize,
}
impl ::windows_sys::core::Interface for IHttpBufferContentFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3156263315, data2: 50207, data3: 20471, data4: [145, 35, 100, 53, 115, 110, 173, 194] };
}
#[repr(C)]
pub struct IHttpClient {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetWithOptionAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, completionoption: HttpCompletionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetWithOptionAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetBufferAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetBufferAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetInputStreamAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetInputStreamAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetStringAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetStringAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PostAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PostAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PutAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PutAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SendRequestAsync: unsafe extern "system" fn(this: *mut *mut Self, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendRequestAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SendRequestWithOptionAsync: unsafe extern "system" fn(this: *mut *mut Self, request: *mut ::core::ffi::c_void, completionoption: HttpCompletionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendRequestWithOptionAsync: usize,
    #[cfg(feature = "Web_Http_Headers")]
    pub DefaultRequestHeaders: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Web_Http_Headers"))]
    DefaultRequestHeaders: usize,
}
impl ::windows_sys::core::Interface for IHttpClient {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2144997713, data2: 13684, data3: 18560, data4: [168, 186, 230, 177, 224, 6, 31, 61] };
}
#[repr(C)]
pub struct IHttpClient2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub TryDeleteAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDeleteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryGetAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryGetAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryGetAsync2: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, completionoption: HttpCompletionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryGetAsync2: usize,
    #[cfg(feature = "Foundation")]
    pub TryGetBufferAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryGetBufferAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryGetInputStreamAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryGetInputStreamAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryGetStringAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryGetStringAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryPostAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryPostAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryPutAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryPutAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySendRequestAsync: unsafe extern "system" fn(this: *mut *mut Self, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySendRequestAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySendRequestAsync2: unsafe extern "system" fn(this: *mut *mut Self, request: *mut ::core::ffi::c_void, completionoption: HttpCompletionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySendRequestAsync2: usize,
}
impl ::windows_sys::core::Interface for IHttpClient2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3453498184, data2: 59575, data3: 19692, data4: [177, 176, 220, 69, 95, 231, 44, 146] };
}
#[repr(C)]
pub struct IHttpClientFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Web_Http_Filters")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, filter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Web_Http_Filters"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for IHttpClientFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3272363722, data2: 58362, data3: 20377, data4: [175, 180, 99, 204, 101, 0, 148, 98] };
}
#[repr(C)]
pub struct IHttpContent {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Web_Http_Headers")]
    pub Headers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Web_Http_Headers"))]
    Headers: usize,
    #[cfg(feature = "Foundation")]
    pub BufferAllAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BufferAllAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ReadAsBufferAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ReadAsBufferAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ReadAsInputStreamAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ReadAsInputStreamAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReadAsStringAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadAsStringAsync: usize,
    pub TryComputeLength: unsafe extern "system" fn(this: *mut *mut Self, length: *mut u64, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub WriteToStreamAsync: unsafe extern "system" fn(this: *mut *mut Self, outputstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    WriteToStreamAsync: usize,
}
impl ::windows_sys::core::Interface for IHttpContent {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1796514881, data2: 64423, data3: 19410, data4: [175, 10, 131, 157, 231, 194, 149, 218] };
}
#[repr(C)]
pub struct IHttpCookie {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Domain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Expires: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Expires: usize,
    #[cfg(feature = "Foundation")]
    pub SetExpires: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExpires: usize,
    pub HttpOnly: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHttpOnly: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Secure: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetSecure: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpCookie {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 525633762, data2: 52269, data3: 18297, data4: [134, 167, 136, 241, 6, 135, 210, 73] };
}
#[repr(C)]
pub struct IHttpCookieFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, domain: ::windows_sys::core::HSTRING, path: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpCookieFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1778746793, data2: 37660, data3: 19665, data4: [169, 109, 194, 23, 1, 120, 92, 95] };
}
#[repr(C)]
pub struct IHttpCookieManager {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetCookie: unsafe extern "system" fn(this: *mut *mut Self, cookie: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCookieWithThirdParty: unsafe extern "system" fn(this: *mut *mut Self, cookie: *mut ::core::ffi::c_void, thirdparty: bool, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub DeleteCookie: unsafe extern "system" fn(this: *mut *mut Self, cookie: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCookies: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCookies: usize,
}
impl ::windows_sys::core::Interface for IHttpCookieManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2051217280, data2: 52559, data3: 20055, data4: [168, 74, 91, 10, 83, 214, 187, 150] };
}
#[repr(C)]
pub struct IHttpFormUrlEncodedContentFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, content: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for IHttpFormUrlEncodedContentFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1139807116, data2: 12147, data3: 17154, data4: [181, 243, 234, 233, 35, 138, 94, 1] };
}
#[repr(C)]
pub struct IHttpGetBufferResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub RequestMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ResponseMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Value: usize,
}
impl ::windows_sys::core::Interface for IHttpGetBufferResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1406176892, data2: 57865, data3: 16462, data4: [154, 73, 116, 45, 130, 54, 253, 58] };
}
#[repr(C)]
pub struct IHttpGetInputStreamResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub RequestMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ResponseMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Value: usize,
}
impl ::windows_sys::core::Interface for IHttpGetInputStreamResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3587585123, data2: 5034, data3: 20192, data4: [190, 149, 160, 195, 159, 233, 18, 3] };
}
#[repr(C)]
pub struct IHttpGetStringResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub RequestMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ResponseMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpGetStringResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2611758701, data2: 34057, data3: 18293, data4: [177, 109, 137, 83, 244, 122, 127, 95] };
}
#[repr(C)]
pub struct IHttpMethod {
    pub base__: ::windows_sys::core::IInspectable,
    pub Method: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpMethod {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921859618, data2: 28685, data3: 20448, data4: [175, 165, 64, 41, 156, 88, 219, 253] };
}
#[repr(C)]
pub struct IHttpMethodFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, method: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpMethodFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1011994893, data2: 14039, data3: 16632, data4: [168, 109, 231, 89, 202, 242, 248, 63] };
}
#[repr(C)]
pub struct IHttpMethodStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Get: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Head: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Options: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Patch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Post: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Put: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpMethodStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1691447792, data2: 55706, data3: 16723, data4: [141, 198, 214, 140, 196, 204, 227, 23] };
}
#[repr(C)]
pub struct IHttpMultipartContent {
    pub base__: ::windows_sys::core::IInspectable,
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, content: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpMultipartContent {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3750849279, data2: 39206, data3: 19145, data4: [170, 241, 224, 208, 78, 240, 155, 185] };
}
#[repr(C)]
pub struct IHttpMultipartContentFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWithSubtype: unsafe extern "system" fn(this: *mut *mut Self, subtype: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithSubtypeAndBoundary: unsafe extern "system" fn(this: *mut *mut Self, subtype: ::windows_sys::core::HSTRING, boundary: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpMultipartContentFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2125737570, data2: 546, data3: 20256, data4: [179, 114, 71, 213, 219, 93, 51, 180] };
}
#[repr(C)]
pub struct IHttpMultipartFormDataContent {
    pub base__: ::windows_sys::core::IInspectable,
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, content: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddWithName: unsafe extern "system" fn(this: *mut *mut Self, content: *mut ::core::ffi::c_void, name: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AddWithNameAndFileName: unsafe extern "system" fn(this: *mut *mut Self, content: *mut ::core::ffi::c_void, name: ::windows_sys::core::HSTRING, filename: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpMultipartFormDataContent {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1691564002, data2: 59751, data3: 17956, data4: [182, 209, 207, 116, 96, 74, 74, 66] };
}
#[repr(C)]
pub struct IHttpMultipartFormDataContentFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWithBoundary: unsafe extern "system" fn(this: *mut *mut Self, boundary: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpMultipartFormDataContentFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2689430289, data2: 20503, data3: 17954, data4: [147, 168, 73, 178, 74, 79, 203, 252] };
}
#[repr(C)]
pub struct IHttpRequestMessage {
    pub base__: ::windows_sys::core::IInspectable,
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Web_Http_Headers")]
    pub Headers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Web_Http_Headers"))]
    Headers: usize,
    pub Method: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetMethod: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(feature = "Foundation")]
    pub RequestUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetRequestUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRequestUri: usize,
    pub TransportInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpRequestMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4118162236, data2: 29908, data3: 18449, data4: [181, 220, 159, 139, 78, 47, 154, 191] };
}
#[repr(C)]
pub struct IHttpRequestMessageFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, method: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for IHttpRequestMessageFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1538038094, data2: 14470, data3: 16686, data4: [174, 195, 82, 236, 127, 37, 97, 111] };
}
#[repr(C)]
pub struct IHttpRequestResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub RequestMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ResponseMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpRequestResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1791970728, data2: 46571, data3: 18997, data4: [169, 2, 66, 23, 251, 232, 32, 197] };
}
#[repr(C)]
pub struct IHttpResponseMessage {
    pub base__: ::windows_sys::core::IInspectable,
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Web_Http_Headers")]
    pub Headers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Web_Http_Headers"))]
    Headers: usize,
    pub IsSuccessStatusCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ReasonPhrase: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetReasonPhrase: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RequestMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRequestMessage: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HttpResponseMessageSource) -> ::windows_sys::core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut *mut Self, value: HttpResponseMessageSource) -> ::windows_sys::core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HttpStatusCode) -> ::windows_sys::core::HRESULT,
    pub SetStatusCode: unsafe extern "system" fn(this: *mut *mut Self, value: HttpStatusCode) -> ::windows_sys::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HttpVersion) -> ::windows_sys::core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut *mut Self, value: HttpVersion) -> ::windows_sys::core::HRESULT,
    pub EnsureSuccessStatusCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpResponseMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4276224251, data2: 34404, data3: 17632, data4: [149, 217, 66, 105, 97, 153, 191, 252] };
}
#[repr(C)]
pub struct IHttpResponseMessageFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, statuscode: HttpStatusCode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpResponseMessageFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1386786713, data2: 61589, data3: 17370, data4: [182, 15, 124, 252, 43, 199, 234, 47] };
}
#[repr(C)]
pub struct IHttpStreamContentFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromInputStream: unsafe extern "system" fn(this: *mut *mut Self, content: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromInputStream: usize,
}
impl ::windows_sys::core::Interface for IHttpStreamContentFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4091956637, data2: 63269, data3: 16510, data4: [148, 47, 14, 218, 24, 152, 9, 244] };
}
#[repr(C)]
pub struct IHttpStringContentFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromString: unsafe extern "system" fn(this: *mut *mut Self, content: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStringWithEncoding: unsafe extern "system" fn(this: *mut *mut Self, content: ::windows_sys::core::HSTRING, encoding: super::super::Storage::Streams::UnicodeEncoding, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStringWithEncoding: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStringWithEncodingAndMediaType: unsafe extern "system" fn(this: *mut *mut Self, content: ::windows_sys::core::HSTRING, encoding: super::super::Storage::Streams::UnicodeEncoding, mediatype: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStringWithEncodingAndMediaType: usize,
}
impl ::windows_sys::core::Interface for IHttpStringContentFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1180999003, data2: 11923, data3: 18667, data4: [142, 97, 25, 103, 120, 120, 229, 127] };
}
#[repr(C)]
pub struct IHttpTransportInformation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub ServerCertificate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    ServerCertificate: usize,
    #[cfg(feature = "Networking_Sockets")]
    pub ServerCertificateErrorSeverity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Networking::Sockets::SocketSslErrorSeverity) -> ::windows_sys::core::HRESULT,
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
}
impl ::windows_sys::core::Interface for IHttpTransportInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1880256920, data2: 50855, data3: 20176, data4: [131, 58, 131, 253, 139, 143, 23, 141] };
}
