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
#[repr(C)]
pub struct IHttpChallengeHeaderValueCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub ParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpChallengeHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromScheme: unsafe extern "system" fn(this: *mut *mut Self, scheme: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromSchemeWithToken: unsafe extern "system" fn(this: *mut *mut Self, scheme: ::windows_sys::core::HSTRING, token: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpChallengeHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, challengeheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpConnectionOptionHeaderValue {
    pub base__: ::windows_sys::core::IInspectable,
    pub Token: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpConnectionOptionHeaderValueCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub ParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpConnectionOptionHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, token: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpConnectionOptionHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, connectionoptionheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpContentCodingHeaderValue {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContentCoding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpContentCodingHeaderValueCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub ParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpContentCodingHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, contentcoding: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpContentCodingHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, contentcodingheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IHttpContentCodingWithQualityHeaderValueCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub ParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpContentCodingWithQualityHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromValue: unsafe extern "system" fn(this: *mut *mut Self, contentcoding: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromValueWithQuality: unsafe extern "system" fn(this: *mut *mut Self, contentcoding: ::windows_sys::core::HSTRING, quality: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpContentCodingWithQualityHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, contentcodingwithqualityheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IHttpContentDispositionHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, dispositiontype: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpContentDispositionHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, contentdispositionheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IHttpContentRangeHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromLength: unsafe extern "system" fn(this: *mut *mut Self, length: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromRange: unsafe extern "system" fn(this: *mut *mut Self, from: u64, to: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromRangeWithLength: unsafe extern "system" fn(this: *mut *mut Self, from: u64, to: u64, length: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpContentRangeHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, contentrangeheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpCookiePairHeaderValue {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpCookiePairHeaderValueCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub ParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpCookiePairHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromName: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromNameWithValue: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpCookiePairHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, cookiepairheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IHttpCredentialsHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromScheme: unsafe extern "system" fn(this: *mut *mut Self, scheme: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromSchemeWithToken: unsafe extern "system" fn(this: *mut *mut Self, scheme: ::windows_sys::core::HSTRING, token: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpCredentialsHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, credentialsheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IHttpDateOrDeltaHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, dateordeltaheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IHttpExpectationHeaderValueCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub ParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpExpectationHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromName: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromNameWithValue: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpExpectationHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, expectationheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpLanguageHeaderValueCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub ParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IHttpLanguageRangeWithQualityHeaderValueCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub ParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpLanguageRangeWithQualityHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromLanguageRange: unsafe extern "system" fn(this: *mut *mut Self, languagerange: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromLanguageRangeWithQuality: unsafe extern "system" fn(this: *mut *mut Self, languagerange: ::windows_sys::core::HSTRING, quality: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpLanguageRangeWithQualityHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, languagerangewithqualityheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IHttpMediaTypeHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, mediatype: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpMediaTypeHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, mediatypeheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IHttpMediaTypeWithQualityHeaderValueCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub ParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpMediaTypeWithQualityHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromMediaType: unsafe extern "system" fn(this: *mut *mut Self, mediatype: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromMediaTypeWithQuality: unsafe extern "system" fn(this: *mut *mut Self, mediatype: ::windows_sys::core::HSTRING, quality: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpMediaTypeWithQualityHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, mediatypewithqualityheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpMethodHeaderValueCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub ParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpNameValueHeaderValue {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpNameValueHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromName: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromNameWithValue: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpNameValueHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, namevalueheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpProductHeaderValue {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpProductHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromName: unsafe extern "system" fn(this: *mut *mut Self, productname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromNameWithVersion: unsafe extern "system" fn(this: *mut *mut Self, productname: ::windows_sys::core::HSTRING, productversion: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpProductHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, productheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpProductInfoHeaderValue {
    pub base__: ::windows_sys::core::IInspectable,
    pub Product: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Comment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpProductInfoHeaderValueCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub ParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpProductInfoHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromComment: unsafe extern "system" fn(this: *mut *mut Self, productcomment: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromNameWithVersion: unsafe extern "system" fn(this: *mut *mut Self, productname: ::windows_sys::core::HSTRING, productversion: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpProductInfoHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, productinfoheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IHttpTransferCodingHeaderValue {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Parameters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Parameters: usize,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpTransferCodingHeaderValueCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub ParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpTransferCodingHeaderValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHttpTransferCodingHeaderValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, transfercodingheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
