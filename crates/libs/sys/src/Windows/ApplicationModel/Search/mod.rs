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
impl ::windows_sys::core::Interface for ILocalContentSuggestionSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4004425826, data2: 29757, data3: 17774, data4: [132, 163, 35, 240, 111, 45, 21, 215] };
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
impl ::windows_sys::core::Interface for ISearchPane {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4255968312, data2: 14080, data3: 19827, data4: [145, 161, 47, 153, 134, 116, 35, 138] };
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
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ISearchPaneQueryChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1007046633, data2: 9041, data3: 16968, data4: [165, 41, 113, 16, 244, 100, 167, 133] };
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
impl ::windows_sys::core::Interface for ISearchPaneQueryLinguisticDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2197505550, data2: 2368, data3: 19309, data4: [184, 208, 100, 43, 48, 152, 158, 21] };
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
impl ::windows_sys::core::Interface for ISearchPaneQuerySubmittedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 339453180, data2: 59845, data3: 18230, data4: [145, 178, 232, 235, 156, 184, 131, 86] };
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
impl ::windows_sys::core::Interface for ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1175229157, data2: 19506, data3: 17720, data4: [164, 212, 182, 180, 64, 13, 20, 15] };
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
impl ::windows_sys::core::Interface for ISearchPaneResultSuggestionChosenEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3358682304, data2: 44754, data3: 16864, data4: [188, 224, 194, 108, 167, 79, 133, 236] };
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
impl ::windows_sys::core::Interface for ISearchPaneStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2507320817, data2: 36637, data3: 18463, data4: [161, 91, 198, 22, 85, 241, 106, 14] };
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
impl ::windows_sys::core::Interface for ISearchPaneStaticsWithHideThisApplication {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 7546928, data2: 20721, data3: 19715, data4: [153, 172, 198, 100, 76, 142, 216, 181] };
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
impl ::windows_sys::core::Interface for ISearchPaneSuggestionsRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2175863580, data2: 58721, data3: 16531, data4: [155, 77, 42, 212, 130, 121, 74, 83] };
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
impl ::windows_sys::core::Interface for ISearchPaneSuggestionsRequestDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2697988599, data2: 34632, data3: 20194, data4: [173, 68, 175, 166, 190, 153, 124, 81] };
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
impl ::windows_sys::core::Interface for ISearchPaneSuggestionsRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3365636655, data2: 44118, data3: 17504, data4: [141, 47, 128, 2, 59, 236, 79, 197] };
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
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ISearchPaneVisibilityChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1011691590, data2: 44107, data3: 18930, data4: [151, 214, 2, 14, 97, 130, 203, 156] };
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
impl ::windows_sys::core::Interface for ISearchQueryLinguisticDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1184964699, data2: 27081, data3: 18245, data4: [183, 47, 168, 164, 252, 143, 36, 174] };
}
#[repr(C)]
pub struct ISearchQueryLinguisticDetailsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, querytextalternatives: *mut ::core::ffi::c_void, querytextcompositionstart: u32, querytextcompositionlength: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateInstance: usize,
}
impl ::windows_sys::core::Interface for ISearchQueryLinguisticDetailsFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3402023864, data2: 15460, data3: 19965, data4: [173, 159, 71, 158, 77, 64, 101, 164] };
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
impl ::windows_sys::core::Interface for ISearchSuggestionCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 842697291, data2: 64490, data3: 17478, data4: [171, 188, 61, 167, 145, 95, 221, 58] };
}
#[repr(C)]
pub struct ISearchSuggestionsRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsCanceled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SearchSuggestionCollection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISearchSuggestionsRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1313744551, data2: 17637, data3: 16441, data4: [144, 153, 96, 0, 234, 209, 240, 198] };
}
#[repr(C)]
pub struct ISearchSuggestionsRequestDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISearchSuggestionsRequestDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3071645865, data2: 49253, data3: 17773, data4: [168, 69, 30, 204, 236, 93, 194, 139] };
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
