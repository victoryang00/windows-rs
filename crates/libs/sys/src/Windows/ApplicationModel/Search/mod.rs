#[cfg(feature = "ApplicationModel_Search_Core")]
pub mod Core;
#[repr(C)]
pub struct ILocalContentSuggestionSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub Locations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    Locations: usize,
    pub SetAqsFilter: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AqsFilter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PropertiesToMatch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PropertiesToMatch: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISearchPane {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub SetSearchHistoryEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetSearchHistoryEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub SearchHistoryEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SearchHistoryEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub SetSearchHistoryContext: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetSearchHistoryContext: usize,
    #[cfg(feature = "deprecated")]
    pub SearchHistoryContext: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SearchHistoryContext: usize,
    #[cfg(feature = "deprecated")]
    pub SetPlaceholderText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetPlaceholderText: usize,
    #[cfg(feature = "deprecated")]
    pub PlaceholderText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PlaceholderText: usize,
    #[cfg(feature = "deprecated")]
    pub QueryText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    QueryText: usize,
    #[cfg(feature = "deprecated")]
    pub Language: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Language: usize,
    #[cfg(feature = "deprecated")]
    pub Visible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Visible: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub VisibilityChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    VisibilityChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveVisibilityChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveVisibilityChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub QueryChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    QueryChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveQueryChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveQueryChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SuggestionsRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SuggestionsRequested: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSuggestionsRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSuggestionsRequested: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub QuerySubmitted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    QuerySubmitted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveQuerySubmitted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveQuerySubmitted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ResultSuggestionChosen: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ResultSuggestionChosen: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveResultSuggestionChosen: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveResultSuggestionChosen: usize,
    #[cfg(feature = "deprecated")]
    pub SetLocalContentSuggestionSettings: unsafe extern "system" fn(this: *mut *mut Self, settings: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetLocalContentSuggestionSettings: usize,
    #[cfg(feature = "deprecated")]
    pub ShowOverloadDefault: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ShowOverloadDefault: usize,
    #[cfg(feature = "deprecated")]
    pub ShowOverloadWithQuery: unsafe extern "system" fn(this: *mut *mut Self, query: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ShowOverloadWithQuery: usize,
    #[cfg(feature = "deprecated")]
    pub SetShowOnKeyboardInput: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetShowOnKeyboardInput: usize,
    #[cfg(feature = "deprecated")]
    pub ShowOnKeyboardInput: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ShowOnKeyboardInput: usize,
    #[cfg(feature = "deprecated")]
    pub TrySetQueryText: unsafe extern "system" fn(this: *mut *mut Self, query: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TrySetQueryText: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISearchPaneQueryChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub QueryText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    QueryText: usize,
    #[cfg(feature = "deprecated")]
    pub Language: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Language: usize,
    #[cfg(feature = "deprecated")]
    pub LinguisticDetails: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LinguisticDetails: usize,
}
#[repr(C)]
pub struct ISearchPaneQueryLinguisticDetails {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub QueryTextAlternatives: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    QueryTextAlternatives: usize,
    pub QueryTextCompositionStart: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub QueryTextCompositionLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISearchPaneQuerySubmittedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub QueryText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    QueryText: usize,
    #[cfg(feature = "deprecated")]
    pub Language: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Language: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub LinguisticDetails: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LinguisticDetails: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISearchPaneResultSuggestionChosenEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Tag: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Tag: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISearchPaneStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetForCurrentView: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISearchPaneStaticsWithHideThisApplication {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub HideThisApplication: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    HideThisApplication: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISearchPaneSuggestionsRequest {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub IsCanceled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsCanceled: usize,
    #[cfg(feature = "deprecated")]
    pub SearchSuggestionCollection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SearchSuggestionCollection: usize,
    #[cfg(feature = "deprecated")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetDeferral: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISearchPaneSuggestionsRequestDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Complete: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISearchPaneSuggestionsRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Request: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISearchPaneVisibilityChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Visible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Visible: usize,
}
#[repr(C)]
pub struct ISearchQueryLinguisticDetails {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub QueryTextAlternatives: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    QueryTextAlternatives: usize,
    pub QueryTextCompositionStart: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub QueryTextCompositionLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISearchQueryLinguisticDetailsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, querytextalternatives: *mut ::core::ffi::c_void, querytextcompositionstart: u32, querytextcompositionlength: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateInstance: usize,
}
#[repr(C)]
pub struct ISearchSuggestionCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub AppendQuerySuggestion: unsafe extern "system" fn(this: *mut *mut Self, text: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AppendQuerySuggestions: unsafe extern "system" fn(this: *mut *mut Self, suggestions: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppendQuerySuggestions: usize,
    #[cfg(feature = "Storage_Streams")]
    pub AppendResultSuggestion: unsafe extern "system" fn(this: *mut *mut Self, text: ::windows_sys::core::HSTRING, detailtext: ::windows_sys::core::HSTRING, tag: ::windows_sys::core::HSTRING, image: *mut ::core::ffi::c_void, imagealternatetext: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AppendResultSuggestion: usize,
    pub AppendSearchSeparator: unsafe extern "system" fn(this: *mut *mut Self, label: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISearchSuggestionsRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsCanceled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SearchSuggestionCollection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISearchSuggestionsRequestDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
pub type LocalContentSuggestionSettings = *mut ::core::ffi::c_void;
pub type SearchPane = *mut ::core::ffi::c_void;
pub type SearchPaneQueryChangedEventArgs = *mut ::core::ffi::c_void;
pub type SearchPaneQueryLinguisticDetails = *mut ::core::ffi::c_void;
pub type SearchPaneQuerySubmittedEventArgs = *mut ::core::ffi::c_void;
pub type SearchPaneResultSuggestionChosenEventArgs = *mut ::core::ffi::c_void;
pub type SearchPaneSuggestionsRequest = *mut ::core::ffi::c_void;
pub type SearchPaneSuggestionsRequestDeferral = *mut ::core::ffi::c_void;
pub type SearchPaneSuggestionsRequestedEventArgs = *mut ::core::ffi::c_void;
pub type SearchPaneVisibilityChangedEventArgs = *mut ::core::ffi::c_void;
pub type SearchQueryLinguisticDetails = *mut ::core::ffi::c_void;
pub type SearchSuggestionCollection = *mut ::core::ffi::c_void;
pub type SearchSuggestionsRequest = *mut ::core::ffi::c_void;
pub type SearchSuggestionsRequestDeferral = *mut ::core::ffi::c_void;
