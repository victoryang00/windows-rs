#[cfg(feature = "Core")]
pub mod Core;
#[doc(hidden)]
#[repr(transparent)]
pub struct ILocalContentSuggestionSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILocalContentSuggestionSettings {
    type Vtable = ILocalContentSuggestionSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeeaeb062_743d_456e_84a3_23f06f2d15d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalContentSuggestionSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub Locations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-storage")))]
    Locations: usize,
    pub SetAqsFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AqsFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub PropertiesToMatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    PropertiesToMatch: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISearchPane(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISearchPane {
    type Vtable = ISearchPane_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfdacec38_3700_4d73_91a1_2f998674238a);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPane_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub SetSearchHistoryEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetSearchHistoryEnabled: usize,
    #[cfg(feature = "winrt-")]
    pub SearchHistoryEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SearchHistoryEnabled: usize,
    #[cfg(feature = "winrt-")]
    pub SetSearchHistoryContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetSearchHistoryContext: usize,
    #[cfg(feature = "winrt-")]
    pub SearchHistoryContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SearchHistoryContext: usize,
    #[cfg(feature = "winrt-")]
    pub SetPlaceholderText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetPlaceholderText: usize,
    #[cfg(feature = "winrt-")]
    pub PlaceholderText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    PlaceholderText: usize,
    #[cfg(feature = "winrt-")]
    pub QueryText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    QueryText: usize,
    #[cfg(feature = "winrt-")]
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Language: usize,
    #[cfg(feature = "winrt-")]
    pub Visible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Visible: usize,
    #[cfg(feature = "winrt-")]
    pub VisibilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    VisibilityChanged: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveVisibilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveVisibilityChanged: usize,
    #[cfg(feature = "winrt-")]
    pub QueryChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    QueryChanged: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveQueryChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveQueryChanged: usize,
    #[cfg(feature = "winrt-")]
    pub SuggestionsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SuggestionsRequested: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveSuggestionsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveSuggestionsRequested: usize,
    #[cfg(feature = "winrt-")]
    pub QuerySubmitted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    QuerySubmitted: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveQuerySubmitted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveQuerySubmitted: usize,
    #[cfg(feature = "winrt-")]
    pub ResultSuggestionChosen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ResultSuggestionChosen: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveResultSuggestionChosen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveResultSuggestionChosen: usize,
    #[cfg(feature = "winrt-")]
    pub SetLocalContentSuggestionSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetLocalContentSuggestionSettings: usize,
    #[cfg(feature = "winrt-")]
    pub ShowOverloadDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ShowOverloadDefault: usize,
    #[cfg(feature = "winrt-")]
    pub ShowOverloadWithQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ShowOverloadWithQuery: usize,
    #[cfg(feature = "winrt-")]
    pub SetShowOnKeyboardInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetShowOnKeyboardInput: usize,
    #[cfg(feature = "winrt-")]
    pub ShowOnKeyboardInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ShowOnKeyboardInput: usize,
    #[cfg(feature = "winrt-")]
    pub TrySetQueryText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    TrySetQueryText: usize,
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISearchPaneQueryChangedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl ISearchPaneQueryChangedEventArgs {
    #[cfg(feature = "winrt-")]
    pub fn QueryText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).QueryText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn LinguisticDetails(&self) -> ::windows_core::Result<SearchPaneQueryLinguisticDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LinguisticDetails)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SearchPaneQueryLinguisticDetails>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<ISearchPaneQueryChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ISearchPaneQueryChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&ISearchPaneQueryChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ISearchPaneQueryChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISearchPaneQueryChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISearchPaneQueryChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<ISearchPaneQueryChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ISearchPaneQueryChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&ISearchPaneQueryChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ISearchPaneQueryChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ISearchPaneQueryChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ISearchPaneQueryChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for ISearchPaneQueryChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for ISearchPaneQueryChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for ISearchPaneQueryChangedEventArgs {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for ISearchPaneQueryChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchPaneQueryChangedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for ISearchPaneQueryChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{3c064fe9-2351-4248-a529-7110f464a785}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISearchPaneQueryChangedEventArgs {
    type Vtable = ISearchPaneQueryChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3c064fe9_2351_4248_a529_7110f464a785);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneQueryChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub QueryText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    QueryText: usize,
    #[cfg(feature = "winrt-")]
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Language: usize,
    #[cfg(feature = "winrt-")]
    pub LinguisticDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    LinguisticDetails: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchPaneQueryLinguisticDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISearchPaneQueryLinguisticDetails {
    type Vtable = ISearchPaneQueryLinguisticDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x82fb460e_0940_4b6d_b8d0_642b30989e15);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneQueryLinguisticDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub QueryTextAlternatives: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    QueryTextAlternatives: usize,
    pub QueryTextCompositionStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub QueryTextCompositionLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISearchPaneQuerySubmittedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISearchPaneQuerySubmittedEventArgs {
    type Vtable = ISearchPaneQuerySubmittedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x143ba4fc_e9c5_4736_91b2_e8eb9cb88356);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneQuerySubmittedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub QueryText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    QueryText: usize,
    #[cfg(feature = "winrt-")]
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Language: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails {
    type Vtable = ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x460c92e5_4c32_4538_a4d4_b6b4400d140f);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub LinguisticDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    LinguisticDetails: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISearchPaneResultSuggestionChosenEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISearchPaneResultSuggestionChosenEventArgs {
    type Vtable = ISearchPaneResultSuggestionChosenEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc8316cc0_aed2_41e0_bce0_c26ca74f85ec);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneResultSuggestionChosenEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub Tag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Tag: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISearchPaneStatics(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISearchPaneStatics {
    type Vtable = ISearchPaneStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9572adf1_8f1d_481f_a15b_c61655f16a0e);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    GetForCurrentView: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISearchPaneStaticsWithHideThisApplication(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISearchPaneStaticsWithHideThisApplication {
    type Vtable = ISearchPaneStaticsWithHideThisApplication_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00732830_50f1_4d03_99ac_c6644c8ed8b5);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneStaticsWithHideThisApplication_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub HideThisApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    HideThisApplication: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISearchPaneSuggestionsRequest(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISearchPaneSuggestionsRequest {
    type Vtable = ISearchPaneSuggestionsRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x81b10b1c_e561_4093_9b4d_2ad482794a53);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneSuggestionsRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub IsCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    IsCanceled: usize,
    #[cfg(feature = "winrt-")]
    pub SearchSuggestionCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SearchSuggestionCollection: usize,
    #[cfg(feature = "winrt-")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISearchPaneSuggestionsRequestDeferral(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISearchPaneSuggestionsRequestDeferral {
    type Vtable = ISearchPaneSuggestionsRequestDeferral_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa0d009f7_8748_4ee2_ad44_afa6be997c51);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneSuggestionsRequestDeferral_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Complete: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISearchPaneSuggestionsRequestedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISearchPaneSuggestionsRequestedEventArgs {
    type Vtable = ISearchPaneSuggestionsRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc89b8a2f_ac56_4460_8d2f_80023bec4fc5);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneSuggestionsRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Request: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISearchPaneVisibilityChangedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISearchPaneVisibilityChangedEventArgs {
    type Vtable = ISearchPaneVisibilityChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3c4d3046_ac4b_49f2_97d6_020e6182cb9c);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneVisibilityChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub Visible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Visible: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchQueryLinguisticDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISearchQueryLinguisticDetails {
    type Vtable = ISearchQueryLinguisticDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x46a1205b_69c9_4745_b72f_a8a4fc8f24ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchQueryLinguisticDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub QueryTextAlternatives: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    QueryTextAlternatives: usize,
    pub QueryTextCompositionStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub QueryTextCompositionLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchQueryLinguisticDetailsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISearchQueryLinguisticDetailsFactory {
    type Vtable = ISearchQueryLinguisticDetailsFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcac6c3b8_3c64_4dfd_ad9f_479e4d4065a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchQueryLinguisticDetailsFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, querytextalternatives: ::windows_core::RawPtr, querytextcompositionstart: u32, querytextcompositionlength: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    CreateInstance: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchSuggestionCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISearchSuggestionCollection {
    type Vtable = ISearchSuggestionCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x323a8a4b_fbea_4446_abbc_3da7915fdd3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchSuggestionCollection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub AppendQuerySuggestion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub AppendQuerySuggestions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, suggestions: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    AppendQuerySuggestions: usize,
    #[cfg(feature = "winrt-storage")]
    pub AppendResultSuggestion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, detailtext: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, tag: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, image: ::windows_core::RawPtr, imagealternatetext: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    AppendResultSuggestion: usize,
    pub AppendSearchSeparator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, label: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchSuggestionsRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISearchSuggestionsRequest {
    type Vtable = ISearchSuggestionsRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e4e26a7_44e5_4039_9099_6000ead1f0c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchSuggestionsRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SearchSuggestionCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchSuggestionsRequestDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISearchSuggestionsRequestDeferral {
    type Vtable = ISearchSuggestionsRequestDeferral_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb71598a9_c065_456d_a845_1eccec5dc28b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchSuggestionsRequestDeferral_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct LocalContentSuggestionSettings(::windows_core::IUnknown);
impl LocalContentSuggestionSettings {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LocalContentSuggestionSettings, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn SetEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Enabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Enabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub fn Locations(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::winrt_storage::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Locations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::winrt_storage::StorageFolder>>(result__)
        }
    }
    pub fn SetAqsFilter<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAqsFilter)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn AqsFilter(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AqsFilter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn PropertiesToMatch(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PropertiesToMatch)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
}
impl ::core::clone::Clone for LocalContentSuggestionSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LocalContentSuggestionSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LocalContentSuggestionSettings {}
impl ::core::fmt::Debug for LocalContentSuggestionSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LocalContentSuggestionSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LocalContentSuggestionSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.LocalContentSuggestionSettings;{eeaeb062-743d-456e-84a3-23f06f2d15d7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LocalContentSuggestionSettings {
    type Vtable = ILocalContentSuggestionSettings_Vtbl;
    const IID: ::windows_core::GUID = <ILocalContentSuggestionSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LocalContentSuggestionSettings {
    const NAME: &'static str = "Windows.ApplicationModel.Search.LocalContentSuggestionSettings";
}
impl ::core::convert::From<LocalContentSuggestionSettings> for ::windows_core::IUnknown {
    fn from(value: LocalContentSuggestionSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LocalContentSuggestionSettings> for ::windows_core::IUnknown {
    fn from(value: &LocalContentSuggestionSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LocalContentSuggestionSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LocalContentSuggestionSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LocalContentSuggestionSettings> for ::windows_core::IInspectable {
    fn from(value: LocalContentSuggestionSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LocalContentSuggestionSettings> for ::windows_core::IInspectable {
    fn from(value: &LocalContentSuggestionSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LocalContentSuggestionSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LocalContentSuggestionSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct SearchPane(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl SearchPane {
    #[cfg(feature = "winrt-")]
    pub fn SetSearchHistoryEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSearchHistoryEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn SearchHistoryEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).SearchHistoryEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetSearchHistoryContext<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSearchHistoryContext)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn SearchHistoryContext(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SearchHistoryContext)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetPlaceholderText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPlaceholderText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn PlaceholderText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PlaceholderText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn QueryText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).QueryText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Visible(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Visible)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn VisibilityChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SearchPane, SearchPaneVisibilityChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).VisibilityChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveVisibilityChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveVisibilityChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn QueryChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SearchPane, SearchPaneQueryChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).QueryChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveQueryChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveQueryChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn SuggestionsRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SearchPane, SearchPaneSuggestionsRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SuggestionsRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveSuggestionsRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSuggestionsRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn QuerySubmitted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SearchPane, SearchPaneQuerySubmittedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).QuerySubmitted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveQuerySubmitted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveQuerySubmitted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn ResultSuggestionChosen<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SearchPane, SearchPaneResultSuggestionChosenEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ResultSuggestionChosen)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveResultSuggestionChosen<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveResultSuggestionChosen)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetLocalContentSuggestionSettings<'a, Param0: ::windows_core::IntoParam<'a, LocalContentSuggestionSettings>>(&self, settings: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLocalContentSuggestionSettings)(::windows_core::Interface::as_raw(this), settings.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn ShowOverloadDefault(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ShowOverloadDefault)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn ShowOverloadWithQuery<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, query: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ShowOverloadWithQuery)(::windows_core::Interface::as_raw(this), query.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetShowOnKeyboardInput(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetShowOnKeyboardInput)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn ShowOnKeyboardInput(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ShowOnKeyboardInput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn TrySetQueryText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, query: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TrySetQueryText)(::windows_core::Interface::as_raw(this), query.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn GetForCurrentView() -> ::windows_core::Result<SearchPane> {
        Self::ISearchPaneStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SearchPane>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn HideThisApplication() -> ::windows_core::Result<()> {
        Self::ISearchPaneStaticsWithHideThisApplication(|this| unsafe { (::windows_core::Interface::vtable(this).HideThisApplication)(::windows_core::Interface::as_raw(this)).ok() })
    }
    #[cfg(feature = "winrt-")]
    pub fn ISearchPaneStatics<R, F: FnOnce(&ISearchPaneStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SearchPane, ISearchPaneStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-")]
    pub fn ISearchPaneStaticsWithHideThisApplication<R, F: FnOnce(&ISearchPaneStaticsWithHideThisApplication) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SearchPane, ISearchPaneStaticsWithHideThisApplication> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SearchPane {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for SearchPane {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for SearchPane {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SearchPane {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchPane").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SearchPane {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPane;{fdacec38-3700-4d73-91a1-2f998674238a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for SearchPane {
    type Vtable = ISearchPane_Vtbl;
    const IID: ::windows_core::GUID = <ISearchPane as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for SearchPane {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPane";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SearchPane> for ::windows_core::IUnknown {
    fn from(value: SearchPane) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SearchPane> for ::windows_core::IUnknown {
    fn from(value: &SearchPane) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SearchPane {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SearchPane {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SearchPane> for ::windows_core::IInspectable {
    fn from(value: SearchPane) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SearchPane> for ::windows_core::IInspectable {
    fn from(value: &SearchPane) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SearchPane {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SearchPane {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct SearchPaneQueryChangedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl SearchPaneQueryChangedEventArgs {
    #[cfg(feature = "winrt-")]
    pub fn QueryText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).QueryText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn LinguisticDetails(&self) -> ::windows_core::Result<SearchPaneQueryLinguisticDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LinguisticDetails)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SearchPaneQueryLinguisticDetails>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SearchPaneQueryChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for SearchPaneQueryChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for SearchPaneQueryChangedEventArgs {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SearchPaneQueryChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchPaneQueryChangedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SearchPaneQueryChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneQueryChangedEventArgs;{3c064fe9-2351-4248-a529-7110f464a785})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for SearchPaneQueryChangedEventArgs {
    type Vtable = ISearchPaneQueryChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISearchPaneQueryChangedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for SearchPaneQueryChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneQueryChangedEventArgs";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SearchPaneQueryChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SearchPaneQueryChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SearchPaneQueryChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SearchPaneQueryChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SearchPaneQueryChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SearchPaneQueryChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SearchPaneQueryChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SearchPaneQueryChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SearchPaneQueryChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SearchPaneQueryChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SearchPaneQueryChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SearchPaneQueryChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<SearchPaneQueryChangedEventArgs> for ISearchPaneQueryChangedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: SearchPaneQueryChangedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&SearchPaneQueryChangedEventArgs> for ISearchPaneQueryChangedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &SearchPaneQueryChangedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ISearchPaneQueryChangedEventArgs> for SearchPaneQueryChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ISearchPaneQueryChangedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ISearchPaneQueryChangedEventArgs> for &SearchPaneQueryChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ISearchPaneQueryChangedEventArgs> {
        ::core::convert::TryInto::<ISearchPaneQueryChangedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for SearchPaneQueryChangedEventArgs {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for SearchPaneQueryChangedEventArgs {}
#[repr(transparent)]
pub struct SearchPaneQueryLinguisticDetails(::windows_core::IUnknown);
impl SearchPaneQueryLinguisticDetails {
    #[cfg(feature = "winrt-foundation")]
    pub fn QueryTextAlternatives(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).QueryTextAlternatives)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn QueryTextCompositionStart(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).QueryTextCompositionStart)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn QueryTextCompositionLength(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).QueryTextCompositionLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for SearchPaneQueryLinguisticDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SearchPaneQueryLinguisticDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchPaneQueryLinguisticDetails {}
impl ::core::fmt::Debug for SearchPaneQueryLinguisticDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchPaneQueryLinguisticDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SearchPaneQueryLinguisticDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneQueryLinguisticDetails;{82fb460e-0940-4b6d-b8d0-642b30989e15})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SearchPaneQueryLinguisticDetails {
    type Vtable = ISearchPaneQueryLinguisticDetails_Vtbl;
    const IID: ::windows_core::GUID = <ISearchPaneQueryLinguisticDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SearchPaneQueryLinguisticDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneQueryLinguisticDetails";
}
impl ::core::convert::From<SearchPaneQueryLinguisticDetails> for ::windows_core::IUnknown {
    fn from(value: SearchPaneQueryLinguisticDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchPaneQueryLinguisticDetails> for ::windows_core::IUnknown {
    fn from(value: &SearchPaneQueryLinguisticDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SearchPaneQueryLinguisticDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SearchPaneQueryLinguisticDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SearchPaneQueryLinguisticDetails> for ::windows_core::IInspectable {
    fn from(value: SearchPaneQueryLinguisticDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchPaneQueryLinguisticDetails> for ::windows_core::IInspectable {
    fn from(value: &SearchPaneQueryLinguisticDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SearchPaneQueryLinguisticDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SearchPaneQueryLinguisticDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SearchPaneQueryLinguisticDetails {}
unsafe impl ::core::marker::Sync for SearchPaneQueryLinguisticDetails {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct SearchPaneQuerySubmittedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl SearchPaneQuerySubmittedEventArgs {
    #[cfg(feature = "winrt-")]
    pub fn QueryText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).QueryText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn LinguisticDetails(&self) -> ::windows_core::Result<SearchPaneQueryLinguisticDetails> {
        let this = &::windows_core::Interface::cast::<ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LinguisticDetails)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SearchPaneQueryLinguisticDetails>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SearchPaneQuerySubmittedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for SearchPaneQuerySubmittedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for SearchPaneQuerySubmittedEventArgs {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SearchPaneQuerySubmittedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchPaneQuerySubmittedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SearchPaneQuerySubmittedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneQuerySubmittedEventArgs;{143ba4fc-e9c5-4736-91b2-e8eb9cb88356})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for SearchPaneQuerySubmittedEventArgs {
    type Vtable = ISearchPaneQuerySubmittedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISearchPaneQuerySubmittedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for SearchPaneQuerySubmittedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneQuerySubmittedEventArgs";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SearchPaneQuerySubmittedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SearchPaneQuerySubmittedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SearchPaneQuerySubmittedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SearchPaneQuerySubmittedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SearchPaneQuerySubmittedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SearchPaneQuerySubmittedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SearchPaneQuerySubmittedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SearchPaneQuerySubmittedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SearchPaneQuerySubmittedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SearchPaneQuerySubmittedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SearchPaneQuerySubmittedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SearchPaneQuerySubmittedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for SearchPaneQuerySubmittedEventArgs {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for SearchPaneQuerySubmittedEventArgs {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct SearchPaneResultSuggestionChosenEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl SearchPaneResultSuggestionChosenEventArgs {
    #[cfg(feature = "winrt-")]
    pub fn Tag(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Tag)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SearchPaneResultSuggestionChosenEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for SearchPaneResultSuggestionChosenEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for SearchPaneResultSuggestionChosenEventArgs {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SearchPaneResultSuggestionChosenEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchPaneResultSuggestionChosenEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SearchPaneResultSuggestionChosenEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneResultSuggestionChosenEventArgs;{c8316cc0-aed2-41e0-bce0-c26ca74f85ec})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for SearchPaneResultSuggestionChosenEventArgs {
    type Vtable = ISearchPaneResultSuggestionChosenEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISearchPaneResultSuggestionChosenEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for SearchPaneResultSuggestionChosenEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneResultSuggestionChosenEventArgs";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SearchPaneResultSuggestionChosenEventArgs> for ::windows_core::IUnknown {
    fn from(value: SearchPaneResultSuggestionChosenEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SearchPaneResultSuggestionChosenEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SearchPaneResultSuggestionChosenEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SearchPaneResultSuggestionChosenEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SearchPaneResultSuggestionChosenEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SearchPaneResultSuggestionChosenEventArgs> for ::windows_core::IInspectable {
    fn from(value: SearchPaneResultSuggestionChosenEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SearchPaneResultSuggestionChosenEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SearchPaneResultSuggestionChosenEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SearchPaneResultSuggestionChosenEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SearchPaneResultSuggestionChosenEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for SearchPaneResultSuggestionChosenEventArgs {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for SearchPaneResultSuggestionChosenEventArgs {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct SearchPaneSuggestionsRequest(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl SearchPaneSuggestionsRequest {
    #[cfg(feature = "winrt-")]
    pub fn IsCanceled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCanceled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SearchSuggestionCollection(&self) -> ::windows_core::Result<SearchSuggestionCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SearchSuggestionCollection)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SearchSuggestionCollection>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<SearchPaneSuggestionsRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SearchPaneSuggestionsRequestDeferral>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SearchPaneSuggestionsRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for SearchPaneSuggestionsRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for SearchPaneSuggestionsRequest {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SearchPaneSuggestionsRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchPaneSuggestionsRequest").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SearchPaneSuggestionsRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneSuggestionsRequest;{81b10b1c-e561-4093-9b4d-2ad482794a53})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for SearchPaneSuggestionsRequest {
    type Vtable = ISearchPaneSuggestionsRequest_Vtbl;
    const IID: ::windows_core::GUID = <ISearchPaneSuggestionsRequest as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for SearchPaneSuggestionsRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneSuggestionsRequest";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SearchPaneSuggestionsRequest> for ::windows_core::IUnknown {
    fn from(value: SearchPaneSuggestionsRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SearchPaneSuggestionsRequest> for ::windows_core::IUnknown {
    fn from(value: &SearchPaneSuggestionsRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SearchPaneSuggestionsRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SearchPaneSuggestionsRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SearchPaneSuggestionsRequest> for ::windows_core::IInspectable {
    fn from(value: SearchPaneSuggestionsRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SearchPaneSuggestionsRequest> for ::windows_core::IInspectable {
    fn from(value: &SearchPaneSuggestionsRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SearchPaneSuggestionsRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SearchPaneSuggestionsRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for SearchPaneSuggestionsRequest {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for SearchPaneSuggestionsRequest {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct SearchPaneSuggestionsRequestDeferral(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl SearchPaneSuggestionsRequestDeferral {
    #[cfg(feature = "winrt-")]
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SearchPaneSuggestionsRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for SearchPaneSuggestionsRequestDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for SearchPaneSuggestionsRequestDeferral {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SearchPaneSuggestionsRequestDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchPaneSuggestionsRequestDeferral").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SearchPaneSuggestionsRequestDeferral {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestDeferral;{a0d009f7-8748-4ee2-ad44-afa6be997c51})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for SearchPaneSuggestionsRequestDeferral {
    type Vtable = ISearchPaneSuggestionsRequestDeferral_Vtbl;
    const IID: ::windows_core::GUID = <ISearchPaneSuggestionsRequestDeferral as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for SearchPaneSuggestionsRequestDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestDeferral";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SearchPaneSuggestionsRequestDeferral> for ::windows_core::IUnknown {
    fn from(value: SearchPaneSuggestionsRequestDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SearchPaneSuggestionsRequestDeferral> for ::windows_core::IUnknown {
    fn from(value: &SearchPaneSuggestionsRequestDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SearchPaneSuggestionsRequestDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SearchPaneSuggestionsRequestDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SearchPaneSuggestionsRequestDeferral> for ::windows_core::IInspectable {
    fn from(value: SearchPaneSuggestionsRequestDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SearchPaneSuggestionsRequestDeferral> for ::windows_core::IInspectable {
    fn from(value: &SearchPaneSuggestionsRequestDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SearchPaneSuggestionsRequestDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SearchPaneSuggestionsRequestDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for SearchPaneSuggestionsRequestDeferral {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for SearchPaneSuggestionsRequestDeferral {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct SearchPaneSuggestionsRequestedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl SearchPaneSuggestionsRequestedEventArgs {
    #[cfg(feature = "winrt-")]
    pub fn QueryText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISearchPaneQueryChangedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).QueryText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISearchPaneQueryChangedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn LinguisticDetails(&self) -> ::windows_core::Result<SearchPaneQueryLinguisticDetails> {
        let this = &::windows_core::Interface::cast::<ISearchPaneQueryChangedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LinguisticDetails)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SearchPaneQueryLinguisticDetails>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Request(&self) -> ::windows_core::Result<SearchPaneSuggestionsRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SearchPaneSuggestionsRequest>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SearchPaneSuggestionsRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for SearchPaneSuggestionsRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for SearchPaneSuggestionsRequestedEventArgs {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SearchPaneSuggestionsRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchPaneSuggestionsRequestedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SearchPaneSuggestionsRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestedEventArgs;{c89b8a2f-ac56-4460-8d2f-80023bec4fc5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for SearchPaneSuggestionsRequestedEventArgs {
    type Vtable = ISearchPaneSuggestionsRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISearchPaneSuggestionsRequestedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for SearchPaneSuggestionsRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestedEventArgs";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SearchPaneSuggestionsRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SearchPaneSuggestionsRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SearchPaneSuggestionsRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SearchPaneSuggestionsRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SearchPaneSuggestionsRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SearchPaneSuggestionsRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SearchPaneSuggestionsRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SearchPaneSuggestionsRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SearchPaneSuggestionsRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SearchPaneSuggestionsRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SearchPaneSuggestionsRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SearchPaneSuggestionsRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<SearchPaneSuggestionsRequestedEventArgs> for ISearchPaneQueryChangedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: SearchPaneSuggestionsRequestedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&SearchPaneSuggestionsRequestedEventArgs> for ISearchPaneQueryChangedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &SearchPaneSuggestionsRequestedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ISearchPaneQueryChangedEventArgs> for SearchPaneSuggestionsRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ISearchPaneQueryChangedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ISearchPaneQueryChangedEventArgs> for &SearchPaneSuggestionsRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ISearchPaneQueryChangedEventArgs> {
        ::core::convert::TryInto::<ISearchPaneQueryChangedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for SearchPaneSuggestionsRequestedEventArgs {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for SearchPaneSuggestionsRequestedEventArgs {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct SearchPaneVisibilityChangedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl SearchPaneVisibilityChangedEventArgs {
    #[cfg(feature = "winrt-")]
    pub fn Visible(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Visible)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SearchPaneVisibilityChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for SearchPaneVisibilityChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for SearchPaneVisibilityChangedEventArgs {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SearchPaneVisibilityChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchPaneVisibilityChangedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SearchPaneVisibilityChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneVisibilityChangedEventArgs;{3c4d3046-ac4b-49f2-97d6-020e6182cb9c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for SearchPaneVisibilityChangedEventArgs {
    type Vtable = ISearchPaneVisibilityChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISearchPaneVisibilityChangedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for SearchPaneVisibilityChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneVisibilityChangedEventArgs";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SearchPaneVisibilityChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SearchPaneVisibilityChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SearchPaneVisibilityChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SearchPaneVisibilityChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SearchPaneVisibilityChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SearchPaneVisibilityChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SearchPaneVisibilityChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SearchPaneVisibilityChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SearchPaneVisibilityChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SearchPaneVisibilityChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SearchPaneVisibilityChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SearchPaneVisibilityChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for SearchPaneVisibilityChangedEventArgs {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for SearchPaneVisibilityChangedEventArgs {}
#[repr(transparent)]
pub struct SearchQueryLinguisticDetails(::windows_core::IUnknown);
impl SearchQueryLinguisticDetails {
    #[cfg(feature = "winrt-foundation")]
    pub fn QueryTextAlternatives(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).QueryTextAlternatives)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn QueryTextCompositionStart(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).QueryTextCompositionStart)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn QueryTextCompositionLength(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).QueryTextCompositionLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn CreateInstance<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(querytextalternatives: Param0, querytextcompositionstart: u32, querytextcompositionlength: u32) -> ::windows_core::Result<SearchQueryLinguisticDetails> {
        Self::ISearchQueryLinguisticDetailsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), querytextalternatives.into_param().abi(), querytextcompositionstart, querytextcompositionlength, result__.as_mut_ptr()).from_abi::<SearchQueryLinguisticDetails>(result__)
        })
    }
    pub fn ISearchQueryLinguisticDetailsFactory<R, F: FnOnce(&ISearchQueryLinguisticDetailsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SearchQueryLinguisticDetails, ISearchQueryLinguisticDetailsFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SearchQueryLinguisticDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SearchQueryLinguisticDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchQueryLinguisticDetails {}
impl ::core::fmt::Debug for SearchQueryLinguisticDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchQueryLinguisticDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SearchQueryLinguisticDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchQueryLinguisticDetails;{46a1205b-69c9-4745-b72f-a8a4fc8f24ae})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SearchQueryLinguisticDetails {
    type Vtable = ISearchQueryLinguisticDetails_Vtbl;
    const IID: ::windows_core::GUID = <ISearchQueryLinguisticDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SearchQueryLinguisticDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchQueryLinguisticDetails";
}
impl ::core::convert::From<SearchQueryLinguisticDetails> for ::windows_core::IUnknown {
    fn from(value: SearchQueryLinguisticDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchQueryLinguisticDetails> for ::windows_core::IUnknown {
    fn from(value: &SearchQueryLinguisticDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SearchQueryLinguisticDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SearchQueryLinguisticDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SearchQueryLinguisticDetails> for ::windows_core::IInspectable {
    fn from(value: SearchQueryLinguisticDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchQueryLinguisticDetails> for ::windows_core::IInspectable {
    fn from(value: &SearchQueryLinguisticDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SearchQueryLinguisticDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SearchQueryLinguisticDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SearchQueryLinguisticDetails {}
unsafe impl ::core::marker::Sync for SearchQueryLinguisticDetails {}
#[repr(transparent)]
pub struct SearchSuggestionCollection(::windows_core::IUnknown);
impl SearchSuggestionCollection {
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn AppendQuerySuggestion<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, text: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AppendQuerySuggestion)(::windows_core::Interface::as_raw(this), text.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn AppendQuerySuggestions<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, suggestions: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AppendQuerySuggestions)(::windows_core::Interface::as_raw(this), suggestions.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn AppendResultSuggestion<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStreamReference>, Param4: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, text: Param0, detailtext: Param1, tag: Param2, image: Param3, imagealternatetext: Param4) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AppendResultSuggestion)(::windows_core::Interface::as_raw(this), text.into_param().abi(), detailtext.into_param().abi(), tag.into_param().abi(), image.into_param().abi(), imagealternatetext.into_param().abi()).ok() }
    }
    pub fn AppendSearchSeparator<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, label: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AppendSearchSeparator)(::windows_core::Interface::as_raw(this), label.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for SearchSuggestionCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SearchSuggestionCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchSuggestionCollection {}
impl ::core::fmt::Debug for SearchSuggestionCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchSuggestionCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SearchSuggestionCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchSuggestionCollection;{323a8a4b-fbea-4446-abbc-3da7915fdd3a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SearchSuggestionCollection {
    type Vtable = ISearchSuggestionCollection_Vtbl;
    const IID: ::windows_core::GUID = <ISearchSuggestionCollection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SearchSuggestionCollection {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchSuggestionCollection";
}
impl ::core::convert::From<SearchSuggestionCollection> for ::windows_core::IUnknown {
    fn from(value: SearchSuggestionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchSuggestionCollection> for ::windows_core::IUnknown {
    fn from(value: &SearchSuggestionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SearchSuggestionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SearchSuggestionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SearchSuggestionCollection> for ::windows_core::IInspectable {
    fn from(value: SearchSuggestionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchSuggestionCollection> for ::windows_core::IInspectable {
    fn from(value: &SearchSuggestionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SearchSuggestionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SearchSuggestionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SearchSuggestionCollection {}
unsafe impl ::core::marker::Sync for SearchSuggestionCollection {}
#[repr(transparent)]
pub struct SearchSuggestionsRequest(::windows_core::IUnknown);
impl SearchSuggestionsRequest {
    pub fn IsCanceled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCanceled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SearchSuggestionCollection(&self) -> ::windows_core::Result<SearchSuggestionCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SearchSuggestionCollection)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SearchSuggestionCollection>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<SearchSuggestionsRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SearchSuggestionsRequestDeferral>(result__)
        }
    }
}
impl ::core::clone::Clone for SearchSuggestionsRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SearchSuggestionsRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchSuggestionsRequest {}
impl ::core::fmt::Debug for SearchSuggestionsRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchSuggestionsRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SearchSuggestionsRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchSuggestionsRequest;{4e4e26a7-44e5-4039-9099-6000ead1f0c6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SearchSuggestionsRequest {
    type Vtable = ISearchSuggestionsRequest_Vtbl;
    const IID: ::windows_core::GUID = <ISearchSuggestionsRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SearchSuggestionsRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchSuggestionsRequest";
}
impl ::core::convert::From<SearchSuggestionsRequest> for ::windows_core::IUnknown {
    fn from(value: SearchSuggestionsRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchSuggestionsRequest> for ::windows_core::IUnknown {
    fn from(value: &SearchSuggestionsRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SearchSuggestionsRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SearchSuggestionsRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SearchSuggestionsRequest> for ::windows_core::IInspectable {
    fn from(value: SearchSuggestionsRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchSuggestionsRequest> for ::windows_core::IInspectable {
    fn from(value: &SearchSuggestionsRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SearchSuggestionsRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SearchSuggestionsRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SearchSuggestionsRequest {}
unsafe impl ::core::marker::Sync for SearchSuggestionsRequest {}
#[repr(transparent)]
pub struct SearchSuggestionsRequestDeferral(::windows_core::IUnknown);
impl SearchSuggestionsRequestDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for SearchSuggestionsRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SearchSuggestionsRequestDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchSuggestionsRequestDeferral {}
impl ::core::fmt::Debug for SearchSuggestionsRequestDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchSuggestionsRequestDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SearchSuggestionsRequestDeferral {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchSuggestionsRequestDeferral;{b71598a9-c065-456d-a845-1eccec5dc28b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SearchSuggestionsRequestDeferral {
    type Vtable = ISearchSuggestionsRequestDeferral_Vtbl;
    const IID: ::windows_core::GUID = <ISearchSuggestionsRequestDeferral as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SearchSuggestionsRequestDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchSuggestionsRequestDeferral";
}
impl ::core::convert::From<SearchSuggestionsRequestDeferral> for ::windows_core::IUnknown {
    fn from(value: SearchSuggestionsRequestDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchSuggestionsRequestDeferral> for ::windows_core::IUnknown {
    fn from(value: &SearchSuggestionsRequestDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SearchSuggestionsRequestDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SearchSuggestionsRequestDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SearchSuggestionsRequestDeferral> for ::windows_core::IInspectable {
    fn from(value: SearchSuggestionsRequestDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchSuggestionsRequestDeferral> for ::windows_core::IInspectable {
    fn from(value: &SearchSuggestionsRequestDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SearchSuggestionsRequestDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SearchSuggestionsRequestDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SearchSuggestionsRequestDeferral {}
unsafe impl ::core::marker::Sync for SearchSuggestionsRequestDeferral {}
