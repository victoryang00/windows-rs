pub type CoreTextCompositionCompletedEventArgs = *mut ::core::ffi::c_void;
pub type CoreTextCompositionSegment = *mut ::core::ffi::c_void;
pub type CoreTextCompositionStartedEventArgs = *mut ::core::ffi::c_void;
pub type CoreTextEditContext = *mut ::core::ffi::c_void;
pub type CoreTextFormatUpdatingEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextFormatUpdatingReason(pub i32);
impl CoreTextFormatUpdatingReason {
    pub const None: Self = Self(0i32);
    pub const CompositionUnconverted: Self = Self(1i32);
    pub const CompositionConverted: Self = Self(2i32);
    pub const CompositionTargetUnconverted: Self = Self(3i32);
    pub const CompositionTargetConverted: Self = Self(4i32);
}
impl ::core::marker::Copy for CoreTextFormatUpdatingReason {}
impl ::core::clone::Clone for CoreTextFormatUpdatingReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextFormatUpdatingResult(pub i32);
impl CoreTextFormatUpdatingResult {
    pub const Succeeded: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreTextFormatUpdatingResult {}
impl ::core::clone::Clone for CoreTextFormatUpdatingResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextInputPaneDisplayPolicy(pub i32);
impl CoreTextInputPaneDisplayPolicy {
    pub const Automatic: Self = Self(0i32);
    pub const Manual: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreTextInputPaneDisplayPolicy {}
impl ::core::clone::Clone for CoreTextInputPaneDisplayPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextInputScope(pub i32);
impl CoreTextInputScope {
    pub const Default: Self = Self(0i32);
    pub const Url: Self = Self(1i32);
    pub const FilePath: Self = Self(2i32);
    pub const FileName: Self = Self(3i32);
    pub const EmailUserName: Self = Self(4i32);
    pub const EmailAddress: Self = Self(5i32);
    pub const UserName: Self = Self(6i32);
    pub const PersonalFullName: Self = Self(7i32);
    pub const PersonalNamePrefix: Self = Self(8i32);
    pub const PersonalGivenName: Self = Self(9i32);
    pub const PersonalMiddleName: Self = Self(10i32);
    pub const PersonalSurname: Self = Self(11i32);
    pub const PersonalNameSuffix: Self = Self(12i32);
    pub const Address: Self = Self(13i32);
    pub const AddressPostalCode: Self = Self(14i32);
    pub const AddressStreet: Self = Self(15i32);
    pub const AddressStateOrProvince: Self = Self(16i32);
    pub const AddressCity: Self = Self(17i32);
    pub const AddressCountryName: Self = Self(18i32);
    pub const AddressCountryShortName: Self = Self(19i32);
    pub const CurrencyAmountAndSymbol: Self = Self(20i32);
    pub const CurrencyAmount: Self = Self(21i32);
    pub const Date: Self = Self(22i32);
    pub const DateMonth: Self = Self(23i32);
    pub const DateDay: Self = Self(24i32);
    pub const DateYear: Self = Self(25i32);
    pub const DateMonthName: Self = Self(26i32);
    pub const DateDayName: Self = Self(27i32);
    pub const Number: Self = Self(29i32);
    pub const SingleCharacter: Self = Self(30i32);
    pub const Password: Self = Self(31i32);
    pub const TelephoneNumber: Self = Self(32i32);
    pub const TelephoneCountryCode: Self = Self(33i32);
    pub const TelephoneAreaCode: Self = Self(34i32);
    pub const TelephoneLocalNumber: Self = Self(35i32);
    pub const Time: Self = Self(36i32);
    pub const TimeHour: Self = Self(37i32);
    pub const TimeMinuteOrSecond: Self = Self(38i32);
    pub const NumberFullWidth: Self = Self(39i32);
    pub const AlphanumericHalfWidth: Self = Self(40i32);
    pub const AlphanumericFullWidth: Self = Self(41i32);
    pub const CurrencyChinese: Self = Self(42i32);
    pub const Bopomofo: Self = Self(43i32);
    pub const Hiragana: Self = Self(44i32);
    pub const KatakanaHalfWidth: Self = Self(45i32);
    pub const KatakanaFullWidth: Self = Self(46i32);
    pub const Hanja: Self = Self(47i32);
    pub const HangulHalfWidth: Self = Self(48i32);
    pub const HangulFullWidth: Self = Self(49i32);
    pub const Search: Self = Self(50i32);
    pub const Formula: Self = Self(51i32);
    pub const SearchIncremental: Self = Self(52i32);
    pub const ChineseHalfWidth: Self = Self(53i32);
    pub const ChineseFullWidth: Self = Self(54i32);
    pub const NativeScript: Self = Self(55i32);
    pub const Text: Self = Self(57i32);
    pub const Chat: Self = Self(58i32);
    pub const NameOrPhoneNumber: Self = Self(59i32);
    pub const EmailUserNameOrAddress: Self = Self(60i32);
    pub const Private: Self = Self(61i32);
    pub const Maps: Self = Self(62i32);
    pub const PasswordNumeric: Self = Self(63i32);
    pub const FormulaNumber: Self = Self(67i32);
    pub const ChatWithoutEmoji: Self = Self(68i32);
    pub const Digits: Self = Self(28i32);
    pub const PinNumeric: Self = Self(64i32);
    pub const PinAlphanumeric: Self = Self(65i32);
}
impl ::core::marker::Copy for CoreTextInputScope {}
impl ::core::clone::Clone for CoreTextInputScope {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CoreTextLayoutBounds = *mut ::core::ffi::c_void;
pub type CoreTextLayoutRequest = *mut ::core::ffi::c_void;
pub type CoreTextLayoutRequestedEventArgs = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
pub struct CoreTextRange {
    pub StartCaretPosition: i32,
    pub EndCaretPosition: i32,
}
impl ::core::marker::Copy for CoreTextRange {}
impl ::core::clone::Clone for CoreTextRange {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CoreTextSelectionRequest = *mut ::core::ffi::c_void;
pub type CoreTextSelectionRequestedEventArgs = *mut ::core::ffi::c_void;
pub type CoreTextSelectionUpdatingEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextSelectionUpdatingResult(pub i32);
impl CoreTextSelectionUpdatingResult {
    pub const Succeeded: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreTextSelectionUpdatingResult {}
impl ::core::clone::Clone for CoreTextSelectionUpdatingResult {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CoreTextServicesManager = *mut ::core::ffi::c_void;
pub type CoreTextTextRequest = *mut ::core::ffi::c_void;
pub type CoreTextTextRequestedEventArgs = *mut ::core::ffi::c_void;
pub type CoreTextTextUpdatingEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Text_Core\"`*"]
#[repr(transparent)]
pub struct CoreTextTextUpdatingResult(pub i32);
impl CoreTextTextUpdatingResult {
    pub const Succeeded: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreTextTextUpdatingResult {}
impl ::core::clone::Clone for CoreTextTextUpdatingResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ICoreTextCompositionCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsCanceled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CompositionSegments: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CompositionSegments: usize,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[repr(C)]
pub struct ICoreTextCompositionSegment {
    pub base__: ::windows_sys::core::IInspectable,
    pub PreconversionString: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Range: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CoreTextRange) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreTextCompositionStartedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsCanceled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[repr(C)]
pub struct ICoreTextEditContext {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub InputScope: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CoreTextInputScope) -> ::windows_sys::core::HRESULT,
    pub SetInputScope: unsafe extern "system" fn(this: *mut *mut Self, value: CoreTextInputScope) -> ::windows_sys::core::HRESULT,
    pub IsReadOnly: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsReadOnly: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub InputPaneDisplayPolicy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CoreTextInputPaneDisplayPolicy) -> ::windows_sys::core::HRESULT,
    pub SetInputPaneDisplayPolicy: unsafe extern "system" fn(this: *mut *mut Self, value: CoreTextInputPaneDisplayPolicy) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TextRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TextRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTextRequested: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTextRequested: usize,
    #[cfg(feature = "Foundation")]
    pub SelectionRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectionRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSelectionRequested: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSelectionRequested: usize,
    #[cfg(feature = "Foundation")]
    pub LayoutRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LayoutRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLayoutRequested: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLayoutRequested: usize,
    #[cfg(feature = "Foundation")]
    pub TextUpdating: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TextUpdating: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTextUpdating: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTextUpdating: usize,
    #[cfg(feature = "Foundation")]
    pub SelectionUpdating: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectionUpdating: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSelectionUpdating: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSelectionUpdating: usize,
    #[cfg(feature = "Foundation")]
    pub FormatUpdating: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FormatUpdating: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFormatUpdating: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFormatUpdating: usize,
    #[cfg(feature = "Foundation")]
    pub CompositionStarted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CompositionStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompositionStarted: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompositionStarted: usize,
    #[cfg(feature = "Foundation")]
    pub CompositionCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CompositionCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompositionCompleted: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompositionCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub FocusRemoved: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FocusRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFocusRemoved: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFocusRemoved: usize,
    pub NotifyFocusEnter: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub NotifyFocusLeave: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub NotifyTextChanged: unsafe extern "system" fn(this: *mut *mut Self, modifiedrange: CoreTextRange, newlength: i32, newselection: CoreTextRange) -> ::windows_sys::core::HRESULT,
    pub NotifySelectionChanged: unsafe extern "system" fn(this: *mut *mut Self, selection: CoreTextRange) -> ::windows_sys::core::HRESULT,
    pub NotifyLayoutChanged: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreTextEditContext2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub NotifyFocusLeaveCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NotifyFocusLeaveCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNotifyFocusLeaveCompleted: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNotifyFocusLeaveCompleted: usize,
}
#[repr(C)]
pub struct ICoreTextFormatUpdatingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Range: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CoreTextRange) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_ViewManagement"))]
    pub TextColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_ViewManagement")))]
    TextColor: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_ViewManagement"))]
    pub BackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_ViewManagement")))]
    BackgroundColor: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_ViewManagement"))]
    pub UnderlineColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_ViewManagement")))]
    UnderlineColor: usize,
    #[cfg(feature = "Foundation")]
    pub UnderlineType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnderlineType: usize,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CoreTextFormatUpdatingReason) -> ::windows_sys::core::HRESULT,
    pub Result: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CoreTextFormatUpdatingResult) -> ::windows_sys::core::HRESULT,
    pub SetResult: unsafe extern "system" fn(this: *mut *mut Self, value: CoreTextFormatUpdatingResult) -> ::windows_sys::core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[repr(C)]
pub struct ICoreTextLayoutBounds {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub TextBounds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TextBounds: usize,
    #[cfg(feature = "Foundation")]
    pub SetTextBounds: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTextBounds: usize,
    #[cfg(feature = "Foundation")]
    pub ControlBounds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ControlBounds: usize,
    #[cfg(feature = "Foundation")]
    pub SetControlBounds: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetControlBounds: usize,
}
#[repr(C)]
pub struct ICoreTextLayoutRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub Range: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CoreTextRange) -> ::windows_sys::core::HRESULT,
    pub LayoutBounds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[repr(C)]
pub struct ICoreTextLayoutRequest2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LayoutBoundsVisualPixels: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreTextLayoutRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreTextSelectionRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub Selection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CoreTextRange) -> ::windows_sys::core::HRESULT,
    pub SetSelection: unsafe extern "system" fn(this: *mut *mut Self, value: CoreTextRange) -> ::windows_sys::core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[repr(C)]
pub struct ICoreTextSelectionRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreTextSelectionUpdatingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Selection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CoreTextRange) -> ::windows_sys::core::HRESULT,
    pub Result: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CoreTextSelectionUpdatingResult) -> ::windows_sys::core::HRESULT,
    pub SetResult: unsafe extern "system" fn(this: *mut *mut Self, value: CoreTextSelectionUpdatingResult) -> ::windows_sys::core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[repr(C)]
pub struct ICoreTextServicesManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Globalization")]
    pub InputLanguage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    InputLanguage: usize,
    #[cfg(feature = "Foundation")]
    pub InputLanguageChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InputLanguageChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInputLanguageChanged: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInputLanguageChanged: usize,
    pub CreateEditContext: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreTextServicesManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreTextServicesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub HiddenCharacter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreTextTextRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub Range: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CoreTextRange) -> ::windows_sys::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[repr(C)]
pub struct ICoreTextTextRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreTextTextUpdatingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Range: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CoreTextRange) -> ::windows_sys::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NewSelection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CoreTextRange) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Globalization")]
    pub InputLanguage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    InputLanguage: usize,
    pub Result: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CoreTextTextUpdatingResult) -> ::windows_sys::core::HRESULT,
    pub SetResult: unsafe extern "system" fn(this: *mut *mut Self, value: CoreTextTextUpdatingResult) -> ::windows_sys::core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
