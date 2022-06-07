#[repr(C)]
pub struct IPrintBindingOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintBindingOptionDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3287600280, data2: 38244, data3: 20246, data4: [160, 85, 169, 139, 154, 73, 233, 211] };
}
#[repr(C)]
pub struct IPrintBorderingOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintBorderingOptionDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1299430543, data2: 64339, data3: 20146, data4: [152, 95, 29, 145, 222, 11, 118, 57] };
}
#[repr(C)]
pub struct IPrintCollationOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintCollationOptionDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3601576294, data2: 42406, data3: 16604, data4: [172, 195, 115, 159, 40, 241, 229, 211] };
}
#[repr(C)]
pub struct IPrintColorModeOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintColorModeOptionDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3685316356, data2: 61910, data3: 18499, data4: [164, 132, 155, 68, 124, 220, 243, 182] };
}
#[repr(C)]
pub struct IPrintCopiesOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintCopiesOptionDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1107636377, data2: 17209, data3: 17219, data4: [137, 141, 44, 71, 181, 224, 195, 65] };
}
#[repr(C)]
pub struct IPrintCustomItemDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub ItemId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetItemDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ItemDisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintCustomItemDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1459926583, data2: 23610, data3: 17562, data4: [170, 54, 179, 41, 27, 17, 146, 253] };
}
#[repr(C)]
pub struct IPrintCustomItemListOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub AddItem: unsafe extern "system" fn(this: *mut *mut Self, itemid: ::windows_sys::core::HSTRING, displayname: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintCustomItemListOptionDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2784689544, data2: 22770, data3: 20157, data4: [185, 15, 81, 228, 242, 148, 76, 93] };
}
#[repr(C)]
pub struct IPrintCustomItemListOptionDetails2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub AddItem: unsafe extern "system" fn(this: *mut *mut Self, itemid: ::windows_sys::core::HSTRING, displayname: ::windows_sys::core::HSTRING, description: ::windows_sys::core::HSTRING, icon: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AddItem: usize,
}
impl ::windows_sys::core::Interface for IPrintCustomItemListOptionDetails2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3386258749, data2: 25884, data3: 19001, data4: [144, 110, 16, 145, 161, 128, 27, 241] };
}
#[repr(C)]
pub struct IPrintCustomItemListOptionDetails3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintCustomItemListOptionDetails3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1335997759, data2: 15412, data3: 18536, data4: [164, 7, 252, 94, 171, 37, 155, 33] };
}
#[repr(C)]
pub struct IPrintCustomOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintCustomOptionDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3811302940, data2: 10415, data3: 19344, data4: [149, 218, 163, 172, 243, 32, 185, 41] };
}
#[repr(C)]
pub struct IPrintCustomTextOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetMaxCharacters: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub MaxCharacters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintCustomTextOptionDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 718369272, data2: 51389, data3: 18693, data4: [145, 146, 13, 117, 19, 110, 139, 49] };
}
#[repr(C)]
pub struct IPrintCustomTextOptionDetails2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintCustomTextOptionDetails2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3467053908, data2: 47479, data3: 18200, data4: [131, 56, 126, 210, 176, 216, 111, 227] };
}
#[repr(C)]
pub struct IPrintCustomToggleOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintCustomToggleOptionDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2645873940, data2: 58465, data3: 17928, data4: [142, 233, 219, 111, 94, 208, 115, 198] };
}
#[repr(C)]
pub struct IPrintDuplexOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintDuplexOptionDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4242097553, data2: 54436, data3: 17658, data4: [179, 254, 66, 224, 186, 40, 213, 173] };
}
#[repr(C)]
pub struct IPrintHolePunchOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintHolePunchOptionDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2799574808, data2: 18476, data3: 18007, data4: [157, 113, 141, 221, 219, 234, 30, 30] };
}
#[repr(C)]
pub struct IPrintItemListOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
}
impl ::windows_sys::core::Interface for IPrintItemListOptionDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2585941951, data2: 65121, data3: 17368, data4: [162, 79, 163, 246, 171, 115, 32, 231] };
}
#[repr(C)]
pub struct IPrintMediaSizeOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintMediaSizeOptionDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1821203407, data2: 49343, data3: 18376, data4: [184, 74, 98, 142, 125, 13, 26, 29] };
}
#[repr(C)]
pub struct IPrintMediaTypeOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintMediaTypeOptionDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4173791243, data2: 44019, data3: 19132, data4: [142, 134, 34, 171, 197, 116, 74, 67] };
}
#[repr(C)]
pub struct IPrintNumberOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub MinValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MaxValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintNumberOptionDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1291959215, data2: 25692, data3: 19945, data4: [150, 95, 111, 198, 187, 196, 124, 171] };
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
impl ::windows_sys::core::Interface for IPrintOptionDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 956729039, data2: 54914, data3: 18783, data4: [173, 254, 215, 51, 63, 92, 24, 8] };
}
#[repr(C)]
pub struct IPrintOrientationOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintOrientationOptionDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1187219577, data2: 26336, data3: 19872, data4: [135, 180, 210, 84, 87, 130, 78, 183] };
}
#[repr(C)]
pub struct IPrintPageRangeOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintPageRangeOptionDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1511646391, data2: 11240, data3: 19111, data4: [158, 165, 222, 251, 232, 113, 59, 78] };
}
#[repr(C)]
pub struct IPrintQualityOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintQualityOptionDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 768633761, data2: 52762, data3: 17638, data4: [132, 249, 58, 146, 234, 30, 48, 68] };
}
#[repr(C)]
pub struct IPrintStapleOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWarningText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintStapleOptionDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3560011197, data2: 39947, data3: 17632, data4: [132, 246, 206, 235, 206, 101, 56, 0] };
}
#[repr(C)]
pub struct IPrintTaskOptionChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub OptionId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintTaskOptionChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1696169221, data2: 42478, data3: 17159, data4: [148, 7, 154, 202, 209, 71, 103, 156] };
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
impl ::windows_sys::core::Interface for IPrintTaskOptionDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4117891825, data2: 43166, data3: 17062, data4: [129, 175, 248, 224, 16, 179, 138, 104] };
}
#[repr(C)]
pub struct IPrintTaskOptionDetails2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateToggleOption: unsafe extern "system" fn(this: *mut *mut Self, optionid: ::windows_sys::core::HSTRING, displayname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintTaskOptionDetails2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1400048137, data2: 63848, data3: 18066, data4: [161, 119, 192, 116, 89, 113, 134, 219] };
}
#[repr(C)]
pub struct IPrintTaskOptionDetailsStatic {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetFromPrintTaskOptions: unsafe extern "system" fn(this: *mut *mut Self, printtaskoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintTaskOptionDetailsStatic {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 324903315, data2: 2401, data3: 19310, data4: [135, 102, 241, 59, 127, 188, 205, 88] };
}
#[repr(C)]
pub struct IPrintTextOptionDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxCharacters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintTextOptionDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2910184803, data2: 23780, data3: 18108, data4: [153, 24, 171, 159, 173, 20, 76, 91] };
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
