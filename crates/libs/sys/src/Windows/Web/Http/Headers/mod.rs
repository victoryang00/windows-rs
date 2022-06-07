pub type HttpCacheDirectiveHeaderValueCollection = *mut ::core::ffi::c_void;
pub type HttpChallengeHeaderValue = *mut ::core::ffi::c_void;
pub type HttpChallengeHeaderValueCollection = *mut ::core::ffi::c_void;
pub type HttpConnectionOptionHeaderValue = *mut ::core::ffi::c_void;
pub type HttpConnectionOptionHeaderValueCollection = *mut ::core::ffi::c_void;
pub type HttpContentCodingHeaderValue = *mut ::core::ffi::c_void;
pub type HttpContentCodingHeaderValueCollection = *mut ::core::ffi::c_void;
pub type HttpContentCodingWithQualityHeaderValue = *mut ::core::ffi::c_void;
pub type HttpContentCodingWithQualityHeaderValueCollection = *mut ::core::ffi::c_void;
pub type HttpContentDispositionHeaderValue = *mut ::core::ffi::c_void;
pub type HttpContentHeaderCollection = *mut ::core::ffi::c_void;
pub type HttpContentRangeHeaderValue = *mut ::core::ffi::c_void;
pub type HttpCookiePairHeaderValue = *mut ::core::ffi::c_void;
pub type HttpCookiePairHeaderValueCollection = *mut ::core::ffi::c_void;
pub type HttpCredentialsHeaderValue = *mut ::core::ffi::c_void;
pub type HttpDateOrDeltaHeaderValue = *mut ::core::ffi::c_void;
pub type HttpExpectationHeaderValue = *mut ::core::ffi::c_void;
pub type HttpExpectationHeaderValueCollection = *mut ::core::ffi::c_void;
pub type HttpLanguageHeaderValueCollection = *mut ::core::ffi::c_void;
pub type HttpLanguageRangeWithQualityHeaderValue = *mut ::core::ffi::c_void;
pub type HttpLanguageRangeWithQualityHeaderValueCollection = *mut ::core::ffi::c_void;
pub type HttpMediaTypeHeaderValue = *mut ::core::ffi::c_void;
pub type HttpMediaTypeWithQualityHeaderValue = *mut ::core::ffi::c_void;
pub type HttpMediaTypeWithQualityHeaderValueCollection = *mut ::core::ffi::c_void;
pub type HttpMethodHeaderValueCollection = *mut ::core::ffi::c_void;
pub type HttpNameValueHeaderValue = *mut ::core::ffi::c_void;
pub type HttpProductHeaderValue = *mut ::core::ffi::c_void;
pub type HttpProductInfoHeaderValue = *mut ::core::ffi::c_void;
pub type HttpProductInfoHeaderValueCollection = *mut ::core::ffi::c_void;
pub type HttpRequestHeaderCollection = *mut ::core::ffi::c_void;
pub type HttpResponseHeaderCollection = *mut ::core::ffi::c_void;
pub type HttpTransferCodingHeaderValue = *mut ::core::ffi::c_void;
pub type HttpTransferCodingHeaderValueCollection = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IHttpCacheDirectiveHeaderValueCollection {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub MaxAge: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxAge: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxAge: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxAge: usize,
    #[cfg(feature = "Foundation")]
    pub MaxStale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxStale: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxStale: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxStale: usize,
    #[cfg(feature = "Foundation")]
    pub MinFresh: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinFresh: usize,
    #[cfg(feature = "Foundation")]
    pub SetMinFresh: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMinFresh: usize,
    #[cfg(feature = "Foundation")]
    pub SharedMaxAge: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SharedMaxAge: usize,
    #[cfg(feature = "Foundation")]
    pub SetSharedMaxAge: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSharedMaxAge: usize,
    pub ParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpCacheDirectiveHeaderValueCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2589485961, data2: 54736, data3: 20414, data4: [189, 157, 181, 179, 99, 104, 17, 180] };
}
#[repr(C)]
pub struct IHttpChallengeHeaderValue {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Parameters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Parameters: usize,
    pub Scheme: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Token: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpChallengeHeaderValue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 959668655, data2: 3965, data3: 18464, data4: [159, 221, 162, 185, 86, 238, 174, 171] };
}
#[repr(C)]
pub struct IHttpChallengeHeaderValueCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub ParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpChallengeHeaderValueCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3399376769, data2: 44768, data3: 17235, data4: [161, 11, 230, 37, 186, 189, 100, 194] };
}
#[repr(C)]
pub struct IHttpChallengeHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromScheme: unsafe extern "system" fn(this: *mut *mut Self, scheme: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromSchemeWithToken: unsafe extern "system" fn(this: *mut *mut Self, scheme: ::windows_sys::core::HSTRING, token: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpChallengeHeaderValueFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3293758545, data2: 55708, data3: 16554, data4: [147, 153, 144, 238, 185, 143, 198, 19] };
}
#[repr(C)]
pub struct IHttpChallengeHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, challengeheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpChallengeHeaderValueStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4090727026, data2: 64513, data3: 19713, data4: [160, 8, 252, 183, 196, 89, 214, 53] };
}
#[repr(C)]
pub struct IHttpConnectionOptionHeaderValue {
    pub base__: ::windows_sys::core::IInspectable,
    pub Token: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpConnectionOptionHeaderValue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3410686586, data2: 20112, data3: 17899, data4: [141, 205, 253, 20, 8, 244, 196, 79] };
}
#[repr(C)]
pub struct IHttpConnectionOptionHeaderValueCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub ParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpConnectionOptionHeaderValueCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3841289245, data2: 20802, data3: 19968, data4: [142, 15, 1, 149, 9, 51, 118, 41] };
}
#[repr(C)]
pub struct IHttpConnectionOptionHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, token: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpConnectionOptionHeaderValueFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3644640286, data2: 2941, data3: 19519, data4: [165, 141, 162, 161, 189, 234, 188, 10] };
}
#[repr(C)]
pub struct IHttpConnectionOptionHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, connectionoptionheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpConnectionOptionHeaderValueStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2863095095, data2: 43334, data3: 19231, data4: [133, 175, 72, 182, 139, 60, 80, 189] };
}
#[repr(C)]
pub struct IHttpContentCodingHeaderValue {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContentCoding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpContentCodingHeaderValue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3170367786, data2: 37750, data3: 19845, data4: [188, 204, 159, 79, 154, 202, 180, 52] };
}
#[repr(C)]
pub struct IHttpContentCodingHeaderValueCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub ParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpContentCodingHeaderValueCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2099386145, data2: 42715, data3: 17262, data4: [142, 131, 145, 89, 97, 146, 129, 156] };
}
#[repr(C)]
pub struct IHttpContentCodingHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, contentcoding: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpContentCodingHeaderValueFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3309120471, data2: 13099, data3: 17232, data4: [133, 16, 46, 103, 162, 40, 154, 90] };
}
#[repr(C)]
pub struct IHttpContentCodingHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, contentcodingheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpContentCodingHeaderValueStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2497208366, data2: 63935, data3: 17143, data4: [170, 70, 237, 39, 42, 65, 226, 18] };
}
#[repr(C)]
pub struct IHttpContentCodingWithQualityHeaderValue {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContentCoding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Quality: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Quality: usize,
}
impl ::windows_sys::core::Interface for IHttpContentCodingWithQualityHeaderValue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2488474837, data2: 35603, data3: 19827, data4: [134, 81, 247, 107, 56, 248, 132, 149] };
}
#[repr(C)]
pub struct IHttpContentCodingWithQualityHeaderValueCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub ParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpContentCodingWithQualityHeaderValueCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2081256766, data2: 59545, data3: 17272, data4: [181, 200, 65, 45, 130, 7, 17, 204] };
}
#[repr(C)]
pub struct IHttpContentCodingWithQualityHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromValue: unsafe extern "system" fn(this: *mut *mut Self, contentcoding: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromValueWithQuality: unsafe extern "system" fn(this: *mut *mut Self, contentcoding: ::windows_sys::core::HSTRING, quality: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpContentCodingWithQualityHeaderValueFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3294555674, data2: 50515, data3: 18172, data4: [173, 226, 215, 92, 29, 83, 223, 123] };
}
#[repr(C)]
pub struct IHttpContentCodingWithQualityHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, contentcodingwithqualityheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpContentCodingWithQualityHeaderValueStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3905500540, data2: 36745, data3: 18433, data4: [142, 117, 76, 154, 191, 195, 222, 113] };
}
#[repr(C)]
pub struct IHttpContentDispositionHeaderValue {
    pub base__: ::windows_sys::core::IInspectable,
    pub DispositionType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDispositionType: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FileName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetFileName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FileNameStar: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetFileNameStar: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Parameters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Parameters: usize,
    #[cfg(feature = "Foundation")]
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Size: usize,
    #[cfg(feature = "Foundation")]
    pub SetSize: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSize: usize,
}
impl ::windows_sys::core::Interface for IHttpContentDispositionHeaderValue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4070764252, data2: 9769, data3: 19273, data4: [153, 8, 150, 161, 104, 233, 54, 94] };
}
#[repr(C)]
pub struct IHttpContentDispositionHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, dispositiontype: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpContentDispositionHeaderValueFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2568338372, data2: 17772, data3: 20097, data4: [130, 149, 178, 171, 60, 188, 245, 69] };
}
#[repr(C)]
pub struct IHttpContentDispositionHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, contentdispositionheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpContentDispositionHeaderValueStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 700801127, data2: 23095, data3: 18148, data4: [176, 116, 197, 23, 125, 105, 202, 102] };
}
#[repr(C)]
pub struct IHttpContentHeaderCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContentDisposition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContentDisposition: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentEncoding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentLanguage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ContentLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContentLength: usize,
    #[cfg(feature = "Foundation")]
    pub SetContentLength: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetContentLength: usize,
    #[cfg(feature = "Foundation")]
    pub ContentLocation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContentLocation: usize,
    #[cfg(feature = "Foundation")]
    pub SetContentLocation: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetContentLocation: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ContentMD5: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ContentMD5: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetContentMD5: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetContentMD5: usize,
    pub ContentRange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContentRange: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContentType: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Expires: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Expires: usize,
    #[cfg(feature = "Foundation")]
    pub SetExpires: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExpires: usize,
    #[cfg(feature = "Foundation")]
    pub LastModified: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastModified: usize,
    #[cfg(feature = "Foundation")]
    pub SetLastModified: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastModified: usize,
    pub Append: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryAppendWithoutValidation: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpContentHeaderCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1080109636, data2: 18350, data3: 19326, data4: [145, 36, 105, 98, 139, 100, 170, 24] };
}
#[repr(C)]
pub struct IHttpContentRangeHeaderValue {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FirstBytePosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FirstBytePosition: usize,
    #[cfg(feature = "Foundation")]
    pub LastBytePosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastBytePosition: usize,
    #[cfg(feature = "Foundation")]
    pub Length: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Length: usize,
    pub Unit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetUnit: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpContentRangeHeaderValue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 81356755, data2: 42230, data3: 18780, data4: [149, 48, 133, 121, 252, 186, 138, 169] };
}
#[repr(C)]
pub struct IHttpContentRangeHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromLength: unsafe extern "system" fn(this: *mut *mut Self, length: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromRange: unsafe extern "system" fn(this: *mut *mut Self, from: u64, to: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromRangeWithLength: unsafe extern "system" fn(this: *mut *mut Self, from: u64, to: u64, length: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpContentRangeHeaderValueFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1062983313, data2: 41020, data3: 17494, data4: [154, 111, 239, 39, 236, 208, 60, 174] };
}
#[repr(C)]
pub struct IHttpContentRangeHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, contentrangeheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpContentRangeHeaderValueStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2158184138, data2: 5964, data3: 20398, data4: [130, 28, 19, 76, 210, 148, 170, 56] };
}
#[repr(C)]
pub struct IHttpCookiePairHeaderValue {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpCookiePairHeaderValue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3419693591, data2: 19241, data3: 16683, data4: [189, 144, 179, 216, 20, 171, 142, 27] };
}
#[repr(C)]
pub struct IHttpCookiePairHeaderValueCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub ParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpCookiePairHeaderValueCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4092871504, data2: 22558, data3: 20172, data4: [159, 89, 229, 7, 208, 79, 6, 230] };
}
#[repr(C)]
pub struct IHttpCookiePairHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromName: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromNameWithValue: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpCookiePairHeaderValueFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1667117679, data2: 5231, data3: 20310, data4: [170, 33, 44, 183, 214, 213, 139, 30] };
}
#[repr(C)]
pub struct IHttpCookiePairHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, cookiepairheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpCookiePairHeaderValueStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1854303560, data2: 1711, data3: 17506, data4: [129, 88, 153, 56, 141, 93, 202, 129] };
}
#[repr(C)]
pub struct IHttpCredentialsHeaderValue {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Parameters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Parameters: usize,
    pub Scheme: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Token: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpCredentialsHeaderValue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3276587979, data2: 21550, data3: 16759, data4: [166, 199, 182, 116, 206, 25, 63, 191] };
}
#[repr(C)]
pub struct IHttpCredentialsHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromScheme: unsafe extern "system" fn(this: *mut *mut Self, scheme: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromSchemeWithToken: unsafe extern "system" fn(this: *mut *mut Self, scheme: ::windows_sys::core::HSTRING, token: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpCredentialsHeaderValueFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4062027409, data2: 19740, data3: 16770, data4: [191, 209, 52, 71, 10, 98, 249, 80] };
}
#[repr(C)]
pub struct IHttpCredentialsHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, credentialsheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpCredentialsHeaderValueStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2795187174, data2: 52876, data3: 17475, data4: [163, 90, 27, 114, 123, 19, 16, 54] };
}
#[repr(C)]
pub struct IHttpDateOrDeltaHeaderValue {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Date: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Date: usize,
    #[cfg(feature = "Foundation")]
    pub Delta: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Delta: usize,
}
impl ::windows_sys::core::Interface for IHttpDateOrDeltaHeaderValue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3942427242, data2: 50396, data3: 18914, data4: [162, 125, 4, 58, 223, 88, 103, 163] };
}
#[repr(C)]
pub struct IHttpDateOrDeltaHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, dateordeltaheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpDateOrDeltaHeaderValueStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2082888104, data2: 26226, data3: 20112, data4: [154, 154, 243, 151, 102, 247, 245, 118] };
}
#[repr(C)]
pub struct IHttpExpectationHeaderValue {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Parameters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Parameters: usize,
}
impl ::windows_sys::core::Interface for IHttpExpectationHeaderValue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1290110413, data2: 15001, data3: 17327, data4: [162, 230, 236, 35, 47, 234, 150, 88] };
}
#[repr(C)]
pub struct IHttpExpectationHeaderValueCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub ParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpExpectationHeaderValueCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3884261811, data2: 41186, data3: 19140, data4: [158, 102, 121, 112, 108, 185, 253, 88] };
}
#[repr(C)]
pub struct IHttpExpectationHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromName: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromNameWithValue: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpExpectationHeaderValueFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1319269835, data2: 54590, data3: 18536, data4: [136, 86, 30, 33, 165, 3, 13, 192] };
}
#[repr(C)]
pub struct IHttpExpectationHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, expectationheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpExpectationHeaderValueStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 806988770, data2: 53221, data3: 18235, data4: [165, 127, 251, 165, 177, 78, 178, 87] };
}
#[repr(C)]
pub struct IHttpLanguageHeaderValueCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub ParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpLanguageHeaderValueCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2663218339, data2: 33305, data3: 17654, data4: [153, 2, 140, 86, 223, 211, 52, 12] };
}
#[repr(C)]
pub struct IHttpLanguageRangeWithQualityHeaderValue {
    pub base__: ::windows_sys::core::IInspectable,
    pub LanguageRange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Quality: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Quality: usize,
}
impl ::windows_sys::core::Interface for IHttpLanguageRangeWithQualityHeaderValue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1918296322, data2: 128, data3: 19892, data4: [160, 131, 125, 231, 178, 229, 186, 76] };
}
#[repr(C)]
pub struct IHttpLanguageRangeWithQualityHeaderValueCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub ParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpLanguageRangeWithQualityHeaderValueCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2287819453, data2: 19279, data3: 18442, data4: [137, 206, 138, 237, 206, 230, 227, 160] };
}
#[repr(C)]
pub struct IHttpLanguageRangeWithQualityHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromLanguageRange: unsafe extern "system" fn(this: *mut *mut Self, languagerange: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromLanguageRangeWithQuality: unsafe extern "system" fn(this: *mut *mut Self, languagerange: ::windows_sys::core::HSTRING, quality: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpLanguageRangeWithQualityHeaderValueFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2075670896, data2: 30735, data3: 19587, data4: [159, 228, 220, 48, 135, 246, 189, 85] };
}
#[repr(C)]
pub struct IHttpLanguageRangeWithQualityHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, languagerangewithqualityheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpLanguageRangeWithQualityHeaderValueStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 625074502, data2: 62216, data3: 18165, data4: [182, 149, 66, 245, 64, 36, 236, 104] };
}
#[repr(C)]
pub struct IHttpMediaTypeHeaderValue {
    pub base__: ::windows_sys::core::IInspectable,
    pub CharSet: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCharSet: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MediaType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetMediaType: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Parameters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Parameters: usize,
}
impl ::windows_sys::core::Interface for IHttpMediaTypeHeaderValue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 380798259, data2: 59176, data3: 20427, data4: [189, 176, 8, 164, 49, 161, 72, 68] };
}
#[repr(C)]
pub struct IHttpMediaTypeHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, mediatype: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpMediaTypeHeaderValueFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3201779624, data2: 52503, data3: 17117, data4: [147, 103, 171, 156, 91, 86, 221, 125] };
}
#[repr(C)]
pub struct IHttpMediaTypeHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, mediatypeheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpMediaTypeHeaderValueStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3763176415, data2: 7489, data3: 19852, data4: [162, 222, 111, 210, 237, 135, 57, 155] };
}
#[repr(C)]
pub struct IHttpMediaTypeWithQualityHeaderValue {
    pub base__: ::windows_sys::core::IInspectable,
    pub CharSet: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCharSet: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MediaType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetMediaType: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Parameters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Parameters: usize,
    #[cfg(feature = "Foundation")]
    pub Quality: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Quality: usize,
    #[cfg(feature = "Foundation")]
    pub SetQuality: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetQuality: usize,
}
impl ::windows_sys::core::Interface for IHttpMediaTypeWithQualityHeaderValue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 411917874, data2: 30398, data3: 17568, data4: [177, 205, 32, 116, 189, 237, 45, 222] };
}
#[repr(C)]
pub struct IHttpMediaTypeWithQualityHeaderValueCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub ParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpMediaTypeWithQualityHeaderValueCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1007446899, data2: 4930, data3: 17799, data4: [160, 86, 24, 208, 47, 246, 113, 101] };
}
#[repr(C)]
pub struct IHttpMediaTypeWithQualityHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromMediaType: unsafe extern "system" fn(this: *mut *mut Self, mediatype: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromMediaTypeWithQuality: unsafe extern "system" fn(this: *mut *mut Self, mediatype: ::windows_sys::core::HSTRING, quality: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpMediaTypeWithQualityHeaderValueFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1282220276, data2: 37975, data3: 17638, data4: [163, 35, 209, 34, 185, 88, 120, 11] };
}
#[repr(C)]
pub struct IHttpMediaTypeWithQualityHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, mediatypewithqualityheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpMediaTypeWithQualityHeaderValueStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1527188697, data2: 46432, data3: 20424, data4: [152, 53, 126, 108, 10, 101, 123, 36] };
}
#[repr(C)]
pub struct IHttpMethodHeaderValueCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub ParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpMethodHeaderValueCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1136410612, data2: 24857, data3: 19167, data4: [147, 140, 52, 191, 255, 207, 146, 237] };
}
#[repr(C)]
pub struct IHttpNameValueHeaderValue {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpNameValueHeaderValue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3636098147, data2: 23450, data3: 19739, data4: [147, 249, 170, 91, 68, 236, 253, 223] };
}
#[repr(C)]
pub struct IHttpNameValueHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromName: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromNameWithValue: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpNameValueHeaderValueFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1997415015, data2: 52216, data3: 18230, data4: [169, 37, 147, 251, 225, 12, 124, 168] };
}
#[repr(C)]
pub struct IHttpNameValueHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, namevalueheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpNameValueHeaderValueStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4292084495, data2: 4400, data3: 16722, data4: [134, 89, 37, 105, 9, 169, 209, 21] };
}
#[repr(C)]
pub struct IHttpProductHeaderValue {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpProductHeaderValue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4110347779, data2: 60372, data3: 16736, data4: [185, 255, 128, 124, 81, 131, 182, 230] };
}
#[repr(C)]
pub struct IHttpProductHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromName: unsafe extern "system" fn(this: *mut *mut Self, productname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromNameWithVersion: unsafe extern "system" fn(this: *mut *mut Self, productname: ::windows_sys::core::HSTRING, productversion: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpProductHeaderValueFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1629136117, data2: 33468, data3: 17147, data4: [151, 123, 220, 0, 83, 110, 94, 134] };
}
#[repr(C)]
pub struct IHttpProductHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, productheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpProductHeaderValueStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2428714537, data2: 48892, data3: 17207, data4: [190, 98, 73, 240, 151, 151, 95, 83] };
}
#[repr(C)]
pub struct IHttpProductInfoHeaderValue {
    pub base__: ::windows_sys::core::IInspectable,
    pub Product: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Comment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpProductInfoHeaderValue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 454723378, data2: 19509, data3: 18538, data4: [150, 111, 100, 100, 137, 25, 142, 77] };
}
#[repr(C)]
pub struct IHttpProductInfoHeaderValueCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub ParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpProductInfoHeaderValueCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2273179466, data2: 54939, data3: 17656, data4: [173, 79, 69, 58, 249, 196, 46, 208] };
}
#[repr(C)]
pub struct IHttpProductInfoHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromComment: unsafe extern "system" fn(this: *mut *mut Self, productcomment: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromNameWithVersion: unsafe extern "system" fn(this: *mut *mut Self, productname: ::windows_sys::core::HSTRING, productversion: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpProductInfoHeaderValueFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 606212030, data2: 60094, data3: 17508, data4: [180, 96, 236, 1, 11, 124, 65, 226] };
}
#[repr(C)]
pub struct IHttpProductInfoHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, productinfoheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpProductInfoHeaderValueStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3682588759, data2: 12922, data3: 20083, data4: [129, 229, 112, 89, 163, 2, 176, 66] };
}
#[repr(C)]
pub struct IHttpRequestHeaderCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub Accept: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AcceptEncoding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AcceptLanguage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Authorization: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAuthorization: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CacheControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Connection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Cookie: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Date: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Date: usize,
    #[cfg(feature = "Foundation")]
    pub SetDate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDate: usize,
    pub Expect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub From: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetFrom: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Networking")]
    pub Host: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    Host: usize,
    #[cfg(feature = "Networking")]
    pub SetHost: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    SetHost: usize,
    #[cfg(feature = "Foundation")]
    pub IfModifiedSince: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IfModifiedSince: usize,
    #[cfg(feature = "Foundation")]
    pub SetIfModifiedSince: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetIfModifiedSince: usize,
    #[cfg(feature = "Foundation")]
    pub IfUnmodifiedSince: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IfUnmodifiedSince: usize,
    #[cfg(feature = "Foundation")]
    pub SetIfUnmodifiedSince: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetIfUnmodifiedSince: usize,
    #[cfg(feature = "Foundation")]
    pub MaxForwards: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxForwards: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxForwards: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxForwards: usize,
    pub ProxyAuthorization: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetProxyAuthorization: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Referer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Referer: usize,
    #[cfg(feature = "Foundation")]
    pub SetReferer: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetReferer: usize,
    pub TransferEncoding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UserAgent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryAppendWithoutValidation: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpRequestHeaderCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2940220059, data2: 46404, data3: 18075, data4: [134, 185, 172, 61, 70, 111, 234, 54] };
}
#[repr(C)]
pub struct IHttpResponseHeaderCollection {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Age: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Age: usize,
    #[cfg(feature = "Foundation")]
    pub SetAge: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAge: usize,
    pub Allow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CacheControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Connection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Date: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Date: usize,
    #[cfg(feature = "Foundation")]
    pub SetDate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDate: usize,
    #[cfg(feature = "Foundation")]
    pub Location: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Location: usize,
    #[cfg(feature = "Foundation")]
    pub SetLocation: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLocation: usize,
    pub ProxyAuthenticate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RetryAfter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRetryAfter: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TransferEncoding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub WwwAuthenticate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryAppendWithoutValidation: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpResponseHeaderCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2056849769, data2: 64063, data3: 16877, data4: [170, 198, 191, 149, 121, 117, 193, 107] };
}
#[repr(C)]
pub struct IHttpTransferCodingHeaderValue {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Parameters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Parameters: usize,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpTransferCodingHeaderValue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1131361017, data2: 15853, data3: 17085, data4: [179, 138, 84, 150, 162, 81, 28, 230] };
}
#[repr(C)]
pub struct IHttpTransferCodingHeaderValueCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub ParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpTransferCodingHeaderValueCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 539790388, data2: 11267, data3: 18872, data4: [150, 101, 115, 226, 124, 178, 252, 121] };
}
#[repr(C)]
pub struct IHttpTransferCodingHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpTransferCodingHeaderValueFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3143819260, data2: 58209, data3: 20232, data4: [142, 79, 201, 231, 35, 222, 112, 59] };
}
#[repr(C)]
pub struct IHttpTransferCodingHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, transfercodingheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHttpTransferCodingHeaderValueStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1790478634, data2: 6808, data3: 19762, data4: [169, 6, 116, 112, 169, 135, 92, 229] };
}
