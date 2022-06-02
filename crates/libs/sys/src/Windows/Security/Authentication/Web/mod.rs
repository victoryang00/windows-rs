#[cfg(feature = "Security_Authentication_Web_Core")]
pub mod Core;
#[cfg(feature = "Security_Authentication_Web_Provider")]
pub mod Provider;
#[repr(C)]
pub struct IWebAuthenticationBrokerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AuthenticateWithCallbackUriAsync: unsafe extern "system" fn(this: *mut *mut Self, options: WebAuthenticationOptions, requesturi: *mut ::core::ffi::c_void, callbackuri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AuthenticateWithCallbackUriAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AuthenticateWithoutCallbackUriAsync: unsafe extern "system" fn(this: *mut *mut Self, options: WebAuthenticationOptions, requesturi: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AuthenticateWithoutCallbackUriAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetCurrentApplicationCallbackUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCurrentApplicationCallbackUri: usize,
}
#[repr(C)]
pub struct IWebAuthenticationBrokerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AuthenticateAndContinue: unsafe extern "system" fn(this: *mut *mut Self, requesturi: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AuthenticateAndContinue: usize,
    #[cfg(feature = "Foundation")]
    pub AuthenticateWithCallbackUriAndContinue: unsafe extern "system" fn(this: *mut *mut Self, requesturi: *mut ::core::ffi::c_void, callbackuri: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AuthenticateWithCallbackUriAndContinue: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AuthenticateWithCallbackUriContinuationDataAndOptionsAndContinue: unsafe extern "system" fn(this: *mut *mut Self, requesturi: *mut ::core::ffi::c_void, callbackuri: *mut ::core::ffi::c_void, continuationdata: *mut ::core::ffi::c_void, options: WebAuthenticationOptions) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AuthenticateWithCallbackUriContinuationDataAndOptionsAndContinue: usize,
    #[cfg(feature = "Foundation")]
    pub AuthenticateSilentlyAsync: unsafe extern "system" fn(this: *mut *mut Self, requesturi: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AuthenticateSilentlyAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AuthenticateSilentlyWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, requesturi: *mut ::core::ffi::c_void, options: WebAuthenticationOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AuthenticateSilentlyWithOptionsAsync: usize,
}
#[repr(C)]
pub struct IWebAuthenticationResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub ResponseData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ResponseStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WebAuthenticationStatus) -> ::windows_sys::core::HRESULT,
    pub ResponseErrorDetail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[doc = "*Required features: `\"Security_Authentication_Web\"`*"]
#[repr(transparent)]
pub struct TokenBindingKeyType(pub i32);
impl TokenBindingKeyType {
    pub const Rsa2048: Self = Self(0i32);
    pub const EcdsaP256: Self = Self(1i32);
    pub const AnyExisting: Self = Self(2i32);
}
impl ::core::marker::Copy for TokenBindingKeyType {}
impl ::core::clone::Clone for TokenBindingKeyType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_Authentication_Web\"`*"]
#[repr(transparent)]
pub struct WebAuthenticationOptions(pub u32);
impl WebAuthenticationOptions {
    pub const None: Self = Self(0u32);
    pub const SilentMode: Self = Self(1u32);
    pub const UseTitle: Self = Self(2u32);
    pub const UseHttpPost: Self = Self(4u32);
    pub const UseCorporateNetwork: Self = Self(8u32);
}
impl ::core::marker::Copy for WebAuthenticationOptions {}
impl ::core::clone::Clone for WebAuthenticationOptions {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WebAuthenticationResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Authentication_Web\"`*"]
#[repr(transparent)]
pub struct WebAuthenticationStatus(pub i32);
impl WebAuthenticationStatus {
    pub const Success: Self = Self(0i32);
    pub const UserCancel: Self = Self(1i32);
    pub const ErrorHttp: Self = Self(2i32);
}
impl ::core::marker::Copy for WebAuthenticationStatus {}
impl ::core::clone::Clone for WebAuthenticationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
