#[repr(C)]
pub struct IPrintBindingOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintBorderingOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintCollationOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintColorModeOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintCopiesOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintCustomItemDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub ItemId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetItemDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ItemDisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintCustomItemListOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub AddItem: unsafe extern "system" fn(this: *mut *mut Self, itemid: ::windows_sys::core::HSTRING, displayname: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintCustomItemListOptionDetails2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub AddItem: unsafe extern "system" fn(this: *mut *mut Self, itemid: ::windows_sys::core::HSTRING, displayname: ::windows_sys::core::HSTRING, description: ::windows_sys::core::HSTRING, icon: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AddItem: usize,
}
#[repr(C)]
pub struct IPrintCustomItemListOptionDetails3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintCustomOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintCustomTextOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetMaxCharacters: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub MaxCharacters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintCustomTextOptionDetails2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintCustomToggleOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintDuplexOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintHolePunchOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintItemListOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
}
#[repr(C)]
pub struct IPrintMediaSizeOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintMediaTypeOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintNumberOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub MinValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MaxValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub OptionId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub OptionType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PrintOptionType) -> ::windows_sys::core::HRESULT,
    pub SetErrorText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ErrorText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetState: unsafe extern "system" fn(this: *mut *mut Self, value: PrintOptionStates) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PrintOptionStates) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TrySetValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintOrientationOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintPageRangeOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintQualityOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintStapleOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintTaskOptionChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub OptionId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintTaskOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Options: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Options: usize,
    pub CreateItemListOption: unsafe extern "system" fn(this: *mut *mut Self, optionid: ::windows_sys::core::HSTRING, displayname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateTextOption: unsafe extern "system" fn(this: *mut *mut Self, optionid: ::windows_sys::core::HSTRING, displayname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub OptionChanged: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OptionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOptionChanged: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOptionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub BeginValidation: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BeginValidation: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBeginValidation: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBeginValidation: usize,
}
#[repr(C)]
pub struct IPrintTaskOptionDetails2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateToggleOption: unsafe extern "system" fn(this: *mut *mut Self, optionid: ::windows_sys::core::HSTRING, displayname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintTaskOptionDetailsStatic {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetFromPrintTaskOptions: unsafe extern "system" fn(this: *mut *mut Self, printtaskoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintTextOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxCharacters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
pub type PrintBindingOptionDetails = *mut ::core::ffi::c_void;
pub type PrintBorderingOptionDetails = *mut ::core::ffi::c_void;
pub type PrintCollationOptionDetails = *mut ::core::ffi::c_void;
pub type PrintColorModeOptionDetails = *mut ::core::ffi::c_void;
pub type PrintCopiesOptionDetails = *mut ::core::ffi::c_void;
pub type PrintCustomItemDetails = *mut ::core::ffi::c_void;
pub type PrintCustomItemListOptionDetails = *mut ::core::ffi::c_void;
pub type PrintCustomTextOptionDetails = *mut ::core::ffi::c_void;
pub type PrintCustomToggleOptionDetails = *mut ::core::ffi::c_void;
pub type PrintDuplexOptionDetails = *mut ::core::ffi::c_void;
pub type PrintHolePunchOptionDetails = *mut ::core::ffi::c_void;
pub type PrintMediaSizeOptionDetails = *mut ::core::ffi::c_void;
pub type PrintMediaTypeOptionDetails = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Printing_OptionDetails\"`*"]
#[repr(transparent)]
pub struct PrintOptionStates(pub u32);
impl PrintOptionStates {
    pub const None: Self = Self(0u32);
    pub const Enabled: Self = Self(1u32);
    pub const Constrained: Self = Self(2u32);
}
impl ::core::marker::Copy for PrintOptionStates {}
impl ::core::clone::Clone for PrintOptionStates {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Graphics_Printing_OptionDetails\"`*"]
#[repr(transparent)]
pub struct PrintOptionType(pub i32);
impl PrintOptionType {
    pub const Unknown: Self = Self(0i32);
    pub const Number: Self = Self(1i32);
    pub const Text: Self = Self(2i32);
    pub const ItemList: Self = Self(3i32);
    pub const Toggle: Self = Self(4i32);
}
impl ::core::marker::Copy for PrintOptionType {}
impl ::core::clone::Clone for PrintOptionType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PrintOrientationOptionDetails = *mut ::core::ffi::c_void;
pub type PrintPageRangeOptionDetails = *mut ::core::ffi::c_void;
pub type PrintQualityOptionDetails = *mut ::core::ffi::c_void;
pub type PrintStapleOptionDetails = *mut ::core::ffi::c_void;
pub type PrintTaskOptionChangedEventArgs = *mut ::core::ffi::c_void;
pub type PrintTaskOptionDetails = *mut ::core::ffi::c_void;
