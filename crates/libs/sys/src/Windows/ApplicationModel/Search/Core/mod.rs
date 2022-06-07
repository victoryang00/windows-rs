#[repr(C)]
pub struct IRequestingFocusOnKeyboardInputEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IRequestingFocusOnKeyboardInputEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2702794535, data2: 45479, data3: 16802, data4: [135, 157, 106, 104, 104, 126, 89, 133] };
}
#[repr(C)]
pub struct ISearchSuggestion {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SearchSuggestionKind) -> ::windows_sys::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DetailText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Image: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Image: usize,
    pub ImageAlternateText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISearchSuggestion {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1532318896, data2: 5415, data3: 17275, data4: [149, 197, 141, 24, 210, 184, 175, 85] };
}
#[repr(C)]
pub struct ISearchSuggestionManager {
    pub base__: ::windows_sys::core::IInspectable,
    pub SearchHistoryEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetSearchHistoryEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SearchHistoryContext: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSearchHistoryContext: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLocalContentSuggestionSettings: unsafe extern "system" fn(this: *mut *mut Self, settings: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetQuery: unsafe extern "system" fn(this: *mut *mut Self, querytext: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetQueryWithLanguage: unsafe extern "system" fn(this: *mut *mut Self, querytext: ::windows_sys::core::HSTRING, language: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetQueryWithSearchQueryLinguisticDetails: unsafe extern "system" fn(this: *mut *mut Self, querytext: ::windows_sys::core::HSTRING, language: ::windows_sys::core::HSTRING, linguisticdetails: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Suggestions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Suggestions: usize,
    pub AddToHistory: unsafe extern "system" fn(this: *mut *mut Self, querytext: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AddToHistoryWithLanguage: unsafe extern "system" fn(this: *mut *mut Self, querytext: ::windows_sys::core::HSTRING, language: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ClearHistory: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SuggestionsRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SuggestionsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSuggestionsRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSuggestionsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RequestingFocusOnKeyboardInput: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestingFocusOnKeyboardInput: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRequestingFocusOnKeyboardInput: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRequestingFocusOnKeyboardInput: usize,
}
impl ::windows_sys::core::Interface for ISearchSuggestionManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1057771681, data2: 52125, data3: 18811, data4: [181, 0, 60, 4, 172, 149, 154, 210] };
}
#[repr(C)]
pub struct ISearchSuggestionsRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub QueryText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LinguisticDetails: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISearchSuggestionsRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1876236773, data2: 40574, data3: 19124, data4: [139, 227, 199, 107, 27, 212, 52, 74] };
}
pub type RequestingFocusOnKeyboardInputEventArgs = *mut ::core::ffi::c_void;
pub type SearchSuggestion = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Search_Core\"`*"]
#[repr(transparent)]
pub struct SearchSuggestionKind(pub i32);
impl SearchSuggestionKind {
    pub const Query: Self = Self(0i32);
    pub const Result: Self = Self(1i32);
    pub const Separator: Self = Self(2i32);
}
impl ::core::marker::Copy for SearchSuggestionKind {}
impl ::core::clone::Clone for SearchSuggestionKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SearchSuggestionManager = *mut ::core::ffi::c_void;
pub type SearchSuggestionsRequestedEventArgs = *mut ::core::ffi::c_void;
