#[cfg(feature = "UI_Notifications_Management")]
pub mod Management;
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct AdaptiveNotificationContentKind(pub i32);
impl AdaptiveNotificationContentKind {
    pub const Text: Self = Self(0i32);
}
impl ::core::marker::Copy for AdaptiveNotificationContentKind {}
impl ::core::clone::Clone for AdaptiveNotificationContentKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AdaptiveNotificationText = *mut ::core::ffi::c_void;
pub type BadgeNotification = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct BadgeTemplateType(pub i32);
impl BadgeTemplateType {
    pub const BadgeGlyph: Self = Self(0i32);
    pub const BadgeNumber: Self = Self(1i32);
}
impl ::core::marker::Copy for BadgeTemplateType {}
impl ::core::clone::Clone for BadgeTemplateType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BadgeUpdateManagerForUser = *mut ::core::ffi::c_void;
pub type BadgeUpdater = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAdaptiveNotificationContent {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AdaptiveNotificationContentKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Hints: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Hints: usize,
}
impl ::windows_sys::core::Interface for IAdaptiveNotificationContent {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3943546470, data2: 29768, data3: 17549, data4: [157, 184, 215, 138, 205, 42, 187, 169] };
}
#[repr(C)]
pub struct IAdaptiveNotificationText {
    pub base__: ::windows_sys::core::IInspectable,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAdaptiveNotificationText {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1188340670, data2: 24730, data3: 17190, data4: [164, 11, 191, 222, 135, 32, 52, 163] };
}
#[repr(C)]
pub struct IBadgeNotification {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Data_Xml_Dom")]
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    Content: usize,
    #[cfg(feature = "Foundation")]
    pub SetExpirationTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExpirationTime: usize,
    #[cfg(feature = "Foundation")]
    pub ExpirationTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationTime: usize,
}
impl ::windows_sys::core::Interface for IBadgeNotification {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 123516106, data2: 53386, data3: 20015, data4: [146, 51, 126, 40, 156, 31, 119, 34] };
}
#[repr(C)]
pub struct IBadgeNotificationFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Data_Xml_Dom")]
    pub CreateBadgeNotification: unsafe extern "system" fn(this: *mut *mut Self, content: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    CreateBadgeNotification: usize,
}
impl ::windows_sys::core::Interface for IBadgeNotificationFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3992081870, data2: 1560, data3: 19801, data4: [148, 138, 90, 97, 4, 12, 82, 249] };
}
#[repr(C)]
pub struct IBadgeUpdateManagerForUser {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateBadgeUpdaterForApplication: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateBadgeUpdaterForApplicationWithId: unsafe extern "system" fn(this: *mut *mut Self, applicationid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateBadgeUpdaterForSecondaryTile: unsafe extern "system" fn(this: *mut *mut Self, tileid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
impl ::windows_sys::core::Interface for IBadgeUpdateManagerForUser {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2573935036, data2: 902, data3: 17637, data4: [186, 141, 12, 16, 119, 166, 46, 146] };
}
#[repr(C)]
pub struct IBadgeUpdateManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateBadgeUpdaterForApplication: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateBadgeUpdaterForApplicationWithId: unsafe extern "system" fn(this: *mut *mut Self, applicationid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateBadgeUpdaterForSecondaryTile: unsafe extern "system" fn(this: *mut *mut Self, tileid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetTemplateContent: unsafe extern "system" fn(this: *mut *mut Self, r#type: BadgeTemplateType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetTemplateContent: usize,
}
impl ::windows_sys::core::Interface for IBadgeUpdateManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 859836330, data2: 28117, data3: 16645, data4: [174, 188, 155, 80, 252, 164, 146, 218] };
}
#[repr(C)]
pub struct IBadgeUpdateManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
impl ::windows_sys::core::Interface for IBadgeUpdateManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2543465934, data2: 63808, data3: 18623, data4: [148, 232, 202, 36, 77, 64, 11, 65] };
}
#[repr(C)]
pub struct IBadgeUpdater {
    pub base__: ::windows_sys::core::IInspectable,
    pub Update: unsafe extern "system" fn(this: *mut *mut Self, notification: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartPeriodicUpdate: unsafe extern "system" fn(this: *mut *mut Self, badgecontent: *mut ::core::ffi::c_void, requestedinterval: PeriodicUpdateRecurrence) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartPeriodicUpdate: usize,
    #[cfg(feature = "Foundation")]
    pub StartPeriodicUpdateAtTime: unsafe extern "system" fn(this: *mut *mut Self, badgecontent: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartPeriodicUpdateAtTime: usize,
    pub StopPeriodicUpdate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBadgeUpdater {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3053068244, data2: 30050, data3: 20332, data4: [191, 163, 27, 110, 210, 229, 127, 47] };
}
#[repr(C)]
pub struct IKnownAdaptiveNotificationHintsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Style: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Wrap: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MaxLines: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MinLines: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TextStacking: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Align: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IKnownAdaptiveNotificationHintsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 102786456, data2: 54422, data3: 18813, data4: [134, 146, 79, 125, 124, 39, 112, 223] };
}
#[repr(C)]
pub struct IKnownAdaptiveNotificationTextStylesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Caption: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Body: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Base: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Subheader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Header: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TitleNumeral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SubheaderNumeral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HeaderNumeral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CaptionSubtle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub BodySubtle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub BaseSubtle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SubtitleSubtle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TitleSubtle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SubheaderSubtle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SubheaderNumeralSubtle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HeaderSubtle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HeaderNumeralSubtle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IKnownAdaptiveNotificationTextStylesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 539071191, data2: 35222, data3: 17834, data4: [139, 161, 212, 97, 215, 44, 42, 27] };
}
#[repr(C)]
pub struct IKnownNotificationBindingsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ToastGeneric: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IKnownNotificationBindingsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2034400174, data2: 43191, data3: 19800, data4: [137, 234, 118, 167, 183, 188, 205, 237] };
}
#[repr(C)]
pub struct INotification {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ExpirationTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetExpirationTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExpirationTime: usize,
    pub Visual: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetVisual: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for INotification {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 276838398, data2: 60278, data3: 20354, data4: [151, 188, 218, 7, 83, 10, 46, 32] };
}
#[repr(C)]
pub struct INotificationBinding {
    pub base__: ::windows_sys::core::IInspectable,
    pub Template: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Hints: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Hints: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTextElements: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTextElements: usize,
}
impl ::windows_sys::core::Interface for INotificationBinding {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4070460293, data2: 880, data3: 19155, data4: [180, 234, 218, 158, 53, 231, 234, 191] };
}
#[repr(C)]
pub struct INotificationData {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Values: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Values: usize,
    pub SequenceNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetSequenceNumber: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for INotificationData {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2684166930, data2: 40298, data3: 19119, data4: [182, 172, 255, 23, 240, 193, 242, 128] };
}
#[repr(C)]
pub struct INotificationDataFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateNotificationDataWithValuesAndSequenceNumber: unsafe extern "system" fn(this: *mut *mut Self, initialvalues: *mut ::core::ffi::c_void, sequencenumber: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateNotificationDataWithValuesAndSequenceNumber: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateNotificationDataWithValues: unsafe extern "system" fn(this: *mut *mut Self, initialvalues: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateNotificationDataWithValues: usize,
}
impl ::windows_sys::core::Interface for INotificationDataFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 599909178, data2: 7184, data3: 18171, data4: [128, 64, 222, 195, 132, 98, 28, 248] };
}
#[repr(C)]
pub struct INotificationVisual {
    pub base__: ::windows_sys::core::IInspectable,
    pub Language: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Bindings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Bindings: usize,
    pub GetBinding: unsafe extern "system" fn(this: *mut *mut Self, templatename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for INotificationVisual {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1753439118, data2: 43606, data3: 19985, data4: [134, 211, 95, 154, 105, 87, 188, 91] };
}
#[repr(C)]
pub struct IScheduledTileNotification {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Data_Xml_Dom")]
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    Content: usize,
    #[cfg(feature = "Foundation")]
    pub DeliveryTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeliveryTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetExpirationTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExpirationTime: usize,
    #[cfg(feature = "Foundation")]
    pub ExpirationTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationTime: usize,
    pub SetTag: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IScheduledTileNotification {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 180135637, data2: 39388, data3: 19576, data4: [161, 28, 201, 231, 248, 109, 126, 247] };
}
#[repr(C)]
pub struct IScheduledTileNotificationFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation"))]
    pub CreateScheduledTileNotification: unsafe extern "system" fn(this: *mut *mut Self, content: *mut ::core::ffi::c_void, deliverytime: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Data_Xml_Dom", feature = "Foundation")))]
    CreateScheduledTileNotification: usize,
}
impl ::windows_sys::core::Interface for IScheduledTileNotificationFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 864228234, data2: 39104, data3: 19515, data4: [187, 214, 74, 99, 60, 124, 252, 41] };
}
#[repr(C)]
pub struct IScheduledToastNotification {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Data_Xml_Dom")]
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    Content: usize,
    #[cfg(feature = "Foundation")]
    pub DeliveryTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeliveryTime: usize,
    #[cfg(feature = "Foundation")]
    pub SnoozeInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SnoozeInterval: usize,
    pub MaximumSnoozeCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IScheduledToastNotification {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2046130168, data2: 3559, data3: 18637, data4: [151, 64, 155, 55, 4, 144, 200, 56] };
}
#[repr(C)]
pub struct IScheduledToastNotification2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetTag: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetGroup: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Group: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSuppressPopup: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SuppressPopup: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IScheduledToastNotification2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2792267932, data2: 12724, data3: 17328, data4: [181, 221, 122, 64, 232, 83, 99, 177] };
}
#[repr(C)]
pub struct IScheduledToastNotification3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub NotificationMirroring: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NotificationMirroring) -> ::windows_sys::core::HRESULT,
    pub SetNotificationMirroring: unsafe extern "system" fn(this: *mut *mut Self, value: NotificationMirroring) -> ::windows_sys::core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IScheduledToastNotification3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2554502795, data2: 48434, data3: 19003, data4: [157, 21, 34, 174, 164, 148, 98, 161] };
}
#[repr(C)]
pub struct IScheduledToastNotification4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ExpirationTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetExpirationTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExpirationTime: usize,
}
impl ::windows_sys::core::Interface for IScheduledToastNotification4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 491217405, data2: 48623, data3: 20042, data4: [150, 190, 1, 1, 54, 155, 88, 210] };
}
#[repr(C)]
pub struct IScheduledToastNotificationFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation"))]
    pub CreateScheduledToastNotification: unsafe extern "system" fn(this: *mut *mut Self, content: *mut ::core::ffi::c_void, deliverytime: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Data_Xml_Dom", feature = "Foundation")))]
    CreateScheduledToastNotification: usize,
    #[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation"))]
    pub CreateScheduledToastNotificationRecurring: unsafe extern "system" fn(this: *mut *mut Self, content: *mut ::core::ffi::c_void, deliverytime: super::super::Foundation::DateTime, snoozeinterval: super::super::Foundation::TimeSpan, maximumsnoozecount: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Data_Xml_Dom", feature = "Foundation")))]
    CreateScheduledToastNotificationRecurring: usize,
}
impl ::windows_sys::core::Interface for IScheduledToastNotificationFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3888042385, data2: 3001, data3: 16777, data4: [131, 148, 49, 118, 27, 71, 111, 215] };
}
#[repr(C)]
pub struct IScheduledToastNotificationShowingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ScheduledToastNotification: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IScheduledToastNotificationShowingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1634989748, data2: 16682, data3: 24108, data4: [166, 237, 160, 32, 154, 239, 154, 9] };
}
#[repr(C)]
pub struct IShownTileNotification {
    pub base__: ::windows_sys::core::IInspectable,
    pub Arguments: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IShownTileNotification {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 875399560, data2: 23282, data3: 18458, data4: [166, 163, 242, 253, 199, 141, 232, 142] };
}
#[repr(C)]
pub struct ITileFlyoutNotification {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Data_Xml_Dom")]
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    Content: usize,
    #[cfg(feature = "Foundation")]
    pub SetExpirationTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExpirationTime: usize,
    #[cfg(feature = "Foundation")]
    pub ExpirationTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationTime: usize,
}
impl ::windows_sys::core::Interface for ITileFlyoutNotification {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2589176417, data2: 50956, data3: 17086, data4: [178, 243, 244, 42, 169, 125, 52, 229] };
}
#[repr(C)]
pub struct ITileFlyoutNotificationFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Data_Xml_Dom")]
    pub CreateTileFlyoutNotification: unsafe extern "system" fn(this: *mut *mut Self, content: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    CreateTileFlyoutNotification: usize,
}
impl ::windows_sys::core::Interface for ITileFlyoutNotificationFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4015353845, data2: 21030, data3: 20267, data4: [178, 120, 136, 163, 93, 254, 86, 159] };
}
#[repr(C)]
pub struct ITileFlyoutUpdateManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateTileFlyoutUpdaterForApplication: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateTileFlyoutUpdaterForApplicationWithId: unsafe extern "system" fn(this: *mut *mut Self, applicationid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateTileFlyoutUpdaterForSecondaryTile: unsafe extern "system" fn(this: *mut *mut Self, tileid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetTemplateContent: unsafe extern "system" fn(this: *mut *mut Self, r#type: TileFlyoutTemplateType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetTemplateContent: usize,
}
impl ::windows_sys::core::Interface for ITileFlyoutUpdateManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 70662923, data2: 6848, data3: 19353, data4: [136, 231, 173, 168, 62, 149, 61, 72] };
}
#[repr(C)]
pub struct ITileFlyoutUpdater {
    pub base__: ::windows_sys::core::IInspectable,
    pub Update: unsafe extern "system" fn(this: *mut *mut Self, notification: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartPeriodicUpdate: unsafe extern "system" fn(this: *mut *mut Self, tileflyoutcontent: *mut ::core::ffi::c_void, requestedinterval: PeriodicUpdateRecurrence) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartPeriodicUpdate: usize,
    #[cfg(feature = "Foundation")]
    pub StartPeriodicUpdateAtTime: unsafe extern "system" fn(this: *mut *mut Self, tileflyoutcontent: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartPeriodicUpdateAtTime: usize,
    pub StopPeriodicUpdate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Setting: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NotificationSetting) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITileFlyoutUpdater {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2369832810, data2: 50277, data3: 16466, data4: [167, 64, 92, 38, 84, 193, 160, 137] };
}
#[repr(C)]
pub struct ITileNotification {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Data_Xml_Dom")]
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    Content: usize,
    #[cfg(feature = "Foundation")]
    pub SetExpirationTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExpirationTime: usize,
    #[cfg(feature = "Foundation")]
    pub ExpirationTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationTime: usize,
    pub SetTag: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITileNotification {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3954100474, data2: 20716, data3: 19480, data4: [180, 208, 58, 240, 46, 85, 64, 171] };
}
#[repr(C)]
pub struct ITileNotificationFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Data_Xml_Dom")]
    pub CreateTileNotification: unsafe extern "system" fn(this: *mut *mut Self, content: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    CreateTileNotification: usize,
}
impl ::windows_sys::core::Interface for ITileNotificationFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3333152110, data2: 18728, data3: 18120, data4: [189, 191, 129, 160, 71, 222, 160, 212] };
}
#[repr(C)]
pub struct ITileUpdateManagerForUser {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateTileUpdaterForApplication: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateTileUpdaterForApplicationWithId: unsafe extern "system" fn(this: *mut *mut Self, applicationid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateTileUpdaterForSecondaryTile: unsafe extern "system" fn(this: *mut *mut Self, tileid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
impl ::windows_sys::core::Interface for ITileUpdateManagerForUser {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1427379016, data2: 12002, data3: 20013, data4: [156, 193, 33, 106, 32, 222, 204, 159] };
}
#[repr(C)]
pub struct ITileUpdateManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateTileUpdaterForApplication: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateTileUpdaterForApplicationWithId: unsafe extern "system" fn(this: *mut *mut Self, applicationid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateTileUpdaterForSecondaryTile: unsafe extern "system" fn(this: *mut *mut Self, tileid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetTemplateContent: unsafe extern "system" fn(this: *mut *mut Self, r#type: TileTemplateType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetTemplateContent: usize,
}
impl ::windows_sys::core::Interface for ITileUpdateManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3658849885, data2: 16041, data3: 18822, data4: [141, 132, 176, 157, 94, 18, 39, 109] };
}
#[repr(C)]
pub struct ITileUpdateManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
impl ::windows_sys::core::Interface for ITileUpdateManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1931222492, data2: 36372, data3: 19324, data4: [163, 75, 157, 34, 222, 118, 200, 77] };
}
#[repr(C)]
pub struct ITileUpdater {
    pub base__: ::windows_sys::core::IInspectable,
    pub Update: unsafe extern "system" fn(this: *mut *mut Self, notification: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub EnableNotificationQueue: unsafe extern "system" fn(this: *mut *mut Self, enable: bool) -> ::windows_sys::core::HRESULT,
    pub Setting: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NotificationSetting) -> ::windows_sys::core::HRESULT,
    pub AddToSchedule: unsafe extern "system" fn(this: *mut *mut Self, scheduledtile: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveFromSchedule: unsafe extern "system" fn(this: *mut *mut Self, scheduledtile: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetScheduledTileNotifications: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetScheduledTileNotifications: usize,
    #[cfg(feature = "Foundation")]
    pub StartPeriodicUpdate: unsafe extern "system" fn(this: *mut *mut Self, tilecontent: *mut ::core::ffi::c_void, requestedinterval: PeriodicUpdateRecurrence) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartPeriodicUpdate: usize,
    #[cfg(feature = "Foundation")]
    pub StartPeriodicUpdateAtTime: unsafe extern "system" fn(this: *mut *mut Self, tilecontent: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartPeriodicUpdateAtTime: usize,
    pub StopPeriodicUpdate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub StartPeriodicUpdateBatch: unsafe extern "system" fn(this: *mut *mut Self, tilecontents: *mut ::core::ffi::c_void, requestedinterval: PeriodicUpdateRecurrence) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StartPeriodicUpdateBatch: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StartPeriodicUpdateBatchAtTime: unsafe extern "system" fn(this: *mut *mut Self, tilecontents: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StartPeriodicUpdateBatchAtTime: usize,
}
impl ::windows_sys::core::Interface for ITileUpdater {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 155362443, data2: 7569, data3: 17644, data4: [146, 67, 193, 232, 33, 194, 154, 32] };
}
#[repr(C)]
pub struct ITileUpdater2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub EnableNotificationQueueForSquare150x150: unsafe extern "system" fn(this: *mut *mut Self, enable: bool) -> ::windows_sys::core::HRESULT,
    pub EnableNotificationQueueForWide310x150: unsafe extern "system" fn(this: *mut *mut Self, enable: bool) -> ::windows_sys::core::HRESULT,
    pub EnableNotificationQueueForSquare310x310: unsafe extern "system" fn(this: *mut *mut Self, enable: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITileUpdater2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2720427538, data2: 5614, data3: 17389, data4: [131, 245, 101, 179, 82, 187, 26, 132] };
}
#[repr(C)]
pub struct IToastActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Arguments: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IToastActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3820983027, data2: 49559, data3: 17263, data4: [130, 101, 6, 37, 130, 79, 141, 172] };
}
#[repr(C)]
pub struct IToastActivatedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub UserInput: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UserInput: usize,
}
impl ::windows_sys::core::Interface for IToastActivatedEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2877138194, data2: 52321, data3: 22158, data4: [129, 190, 48, 74, 195, 16, 56, 250] };
}
#[repr(C)]
pub struct IToastCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LaunchArgs: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLaunchArgs: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Icon: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Icon: usize,
    #[cfg(feature = "Foundation")]
    pub SetIcon: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetIcon: usize,
}
impl ::windows_sys::core::Interface for IToastCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 176931760, data2: 57534, data3: 18520, data4: [188, 42, 137, 223, 224, 179, 40, 99] };
}
#[repr(C)]
pub struct IToastCollectionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, collectionid: ::windows_sys::core::HSTRING, displayname: ::windows_sys::core::HSTRING, launchargs: ::windows_sys::core::HSTRING, iconuri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInstance: usize,
}
impl ::windows_sys::core::Interface for IToastCollectionFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 374199255, data2: 29636, data3: 17655, data4: [180, 255, 251, 109, 75, 241, 244, 198] };
}
#[repr(C)]
pub struct IToastCollectionManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SaveToastCollectionAsync: unsafe extern "system" fn(this: *mut *mut Self, collection: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveToastCollectionAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllToastCollectionsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllToastCollectionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetToastCollectionAsync: unsafe extern "system" fn(this: *mut *mut Self, collectionid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetToastCollectionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveToastCollectionAsync: unsafe extern "system" fn(this: *mut *mut Self, collectionid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveToastCollectionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAllToastCollectionsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAllToastCollectionsAsync: usize,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
    pub AppId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IToastCollectionManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 706224638, data2: 6045, data3: 18876, data4: [183, 157, 165, 39, 146, 13, 54, 101] };
}
#[repr(C)]
pub struct IToastDismissedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ToastDismissalReason) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IToastDismissedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1065998645, data2: 55755, data3: 17720, data4: [160, 240, 255, 231, 101, 153, 56, 248] };
}
#[repr(C)]
pub struct IToastFailedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IToastFailedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 890726498, data2: 53204, data3: 17656, data4: [173, 100, 245, 0, 253, 137, 108, 59] };
}
#[repr(C)]
pub struct IToastNotification {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Data_Xml_Dom")]
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    Content: usize,
    #[cfg(feature = "Foundation")]
    pub SetExpirationTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExpirationTime: usize,
    #[cfg(feature = "Foundation")]
    pub ExpirationTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationTime: usize,
    #[cfg(feature = "Foundation")]
    pub Dismissed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Dismissed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDismissed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDismissed: usize,
    #[cfg(feature = "Foundation")]
    pub Activated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Activated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveActivated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveActivated: usize,
    #[cfg(feature = "Foundation")]
    pub Failed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Failed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFailed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFailed: usize,
}
impl ::windows_sys::core::Interface for IToastNotification {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2575181429, data2: 1438, data3: 20064, data4: [139, 6, 23, 96, 145, 124, 139, 128] };
}
#[repr(C)]
pub struct IToastNotification2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetTag: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetGroup: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Group: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSuppressPopup: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SuppressPopup: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IToastNotification2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2650513361, data2: 5178, data3: 18702, data4: [144, 191, 185, 251, 167, 19, 45, 231] };
}
#[repr(C)]
pub struct IToastNotification3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub NotificationMirroring: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NotificationMirroring) -> ::windows_sys::core::HRESULT,
    pub SetNotificationMirroring: unsafe extern "system" fn(this: *mut *mut Self, value: NotificationMirroring) -> ::windows_sys::core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IToastNotification3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 837332696, data2: 33089, data3: 20377, data4: [188, 10, 196, 237, 33, 41, 125, 119] };
}
#[repr(C)]
pub struct IToastNotification4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ToastNotificationPriority) -> ::windows_sys::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut *mut Self, value: ToastNotificationPriority) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IToastNotification4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 353716533, data2: 10474, data3: 18215, data4: [136, 233, 197, 134, 128, 226, 209, 24] };
}
#[repr(C)]
pub struct IToastNotification6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExpiresOnReboot: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetExpiresOnReboot: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IToastNotification6 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1139539539, data2: 35246, data3: 23582, data4: [162, 121, 58, 236, 254, 155, 111, 84] };
}
#[repr(C)]
pub struct IToastNotificationActionTriggerDetail {
    pub base__: ::windows_sys::core::IInspectable,
    pub Argument: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub UserInput: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UserInput: usize,
}
impl ::windows_sys::core::Interface for IToastNotificationActionTriggerDetail {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2487554906, data2: 14579, data3: 17142, data4: [150, 170, 121, 85, 176, 240, 61, 162] };
}
#[repr(C)]
pub struct IToastNotificationFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Data_Xml_Dom")]
    pub CreateToastNotification: unsafe extern "system" fn(this: *mut *mut Self, content: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    CreateToastNotification: usize,
}
impl ::windows_sys::core::Interface for IToastNotificationFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 68307744, data2: 33478, data3: 16937, data4: [177, 9, 253, 158, 212, 102, 43, 83] };
}
#[repr(C)]
pub struct IToastNotificationHistory {
    pub base__: ::windows_sys::core::IInspectable,
    pub RemoveGroup: unsafe extern "system" fn(this: *mut *mut Self, group: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RemoveGroupWithId: unsafe extern "system" fn(this: *mut *mut Self, group: ::windows_sys::core::HSTRING, applicationid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RemoveGroupedTagWithId: unsafe extern "system" fn(this: *mut *mut Self, tag: ::windows_sys::core::HSTRING, group: ::windows_sys::core::HSTRING, applicationid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RemoveGroupedTag: unsafe extern "system" fn(this: *mut *mut Self, tag: ::windows_sys::core::HSTRING, group: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, tag: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ClearWithId: unsafe extern "system" fn(this: *mut *mut Self, applicationid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IToastNotificationHistory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1554898019, data2: 467, data3: 19607, data4: [152, 111, 5, 51, 72, 63, 238, 20] };
}
#[repr(C)]
pub struct IToastNotificationHistory2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetHistory: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetHistory: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetHistoryWithId: unsafe extern "system" fn(this: *mut *mut Self, applicationid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetHistoryWithId: usize,
}
impl ::windows_sys::core::Interface for IToastNotificationHistory2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1002689107, data2: 12081, data3: 16530, data4: [145, 41, 138, 213, 171, 240, 103, 218] };
}
#[repr(C)]
pub struct IToastNotificationHistoryChangedTriggerDetail {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChangeType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ToastHistoryChangedType) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IToastNotificationHistoryChangedTriggerDetail {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3674439674, data2: 104, data3: 16684, data4: [156, 131, 38, 124, 55, 246, 86, 112] };
}
#[repr(C)]
pub struct IToastNotificationHistoryChangedTriggerDetail2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CollectionId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IToastNotificationHistoryChangedTriggerDetail2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 188148098, data2: 51313, data3: 18939, data4: [186, 187, 37, 189, 188, 76, 196, 91] };
}
#[repr(C)]
pub struct IToastNotificationManagerForUser {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateToastNotifier: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateToastNotifierWithId: unsafe extern "system" fn(this: *mut *mut Self, applicationid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub History: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
impl ::windows_sys::core::Interface for IToastNotificationManagerForUser {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2041272310, data2: 17406, data3: 18555, data4: [138, 127, 153, 86, 114, 0, 174, 148] };
}
#[repr(C)]
pub struct IToastNotificationManagerForUser2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetToastNotifierForToastCollectionIdAsync: unsafe extern "system" fn(this: *mut *mut Self, collectionid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetToastNotifierForToastCollectionIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetHistoryForToastCollectionIdAsync: unsafe extern "system" fn(this: *mut *mut Self, collectionid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetHistoryForToastCollectionIdAsync: usize,
    pub GetToastCollectionManager: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetToastCollectionManagerWithAppId: unsafe extern "system" fn(this: *mut *mut Self, appid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IToastNotificationManagerForUser2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1738302647, data2: 33195, data3: 17090, data4: [136, 25, 201, 88, 118, 119, 83, 244] };
}
#[repr(C)]
pub struct IToastNotificationManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateToastNotifier: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateToastNotifierWithId: unsafe extern "system" fn(this: *mut *mut Self, applicationid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetTemplateContent: unsafe extern "system" fn(this: *mut *mut Self, r#type: ToastTemplateType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetTemplateContent: usize,
}
impl ::windows_sys::core::Interface for IToastNotificationManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1353453631, data2: 53813, data3: 17816, data4: [187, 239, 152, 254, 77, 26, 58, 212] };
}
#[repr(C)]
pub struct IToastNotificationManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub History: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IToastNotificationManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2058959954, data2: 3656, data3: 18256, data4: [186, 157, 26, 65, 19, 152, 24, 71] };
}
#[repr(C)]
pub struct IToastNotificationManagerStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
    pub ConfigureNotificationMirroring: unsafe extern "system" fn(this: *mut *mut Self, value: NotificationMirroring) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IToastNotificationManagerStatics4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2409185235, data2: 58646, data3: 17915, data4: [129, 48, 57, 142, 147, 250, 82, 195] };
}
#[repr(C)]
pub struct IToastNotificationManagerStatics5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IToastNotificationManagerStatics5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3606443369, data2: 54285, data3: 16508, data4: [137, 137, 136, 202, 180, 44, 253, 20] };
}
#[repr(C)]
pub struct IToastNotifier {
    pub base__: ::windows_sys::core::IInspectable,
    pub Show: unsafe extern "system" fn(this: *mut *mut Self, notification: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Hide: unsafe extern "system" fn(this: *mut *mut Self, notification: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Setting: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NotificationSetting) -> ::windows_sys::core::HRESULT,
    pub AddToSchedule: unsafe extern "system" fn(this: *mut *mut Self, scheduledtoast: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveFromSchedule: unsafe extern "system" fn(this: *mut *mut Self, scheduledtoast: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetScheduledToastNotifications: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetScheduledToastNotifications: usize,
}
impl ::windows_sys::core::Interface for IToastNotifier {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1972534163, data2: 1011, data3: 16876, data4: [145, 211, 110, 91, 172, 27, 56, 231] };
}
#[repr(C)]
pub struct IToastNotifier2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub UpdateWithTagAndGroup: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void, tag: ::windows_sys::core::HSTRING, group: ::windows_sys::core::HSTRING, result__: *mut NotificationUpdateResult) -> ::windows_sys::core::HRESULT,
    pub UpdateWithTag: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void, tag: ::windows_sys::core::HSTRING, result__: *mut NotificationUpdateResult) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IToastNotifier2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 893618630, data2: 31745, data3: 19413, data4: [156, 32, 96, 67, 64, 205, 43, 116] };
}
#[repr(C)]
pub struct IToastNotifier3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ScheduledToastNotificationShowing: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScheduledToastNotificationShowing: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveScheduledToastNotificationShowing: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveScheduledToastNotificationShowing: usize,
}
impl ::windows_sys::core::Interface for IToastNotifier3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2926944330, data2: 15116, data3: 20909, data4: [183, 232, 176, 138, 182, 5, 37, 73] };
}
#[repr(C)]
pub struct IUserNotification {
    pub base__: ::windows_sys::core::IInspectable,
    pub Notification: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "ApplicationModel")]
    pub AppInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    AppInfo: usize,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreationTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreationTime: usize,
}
impl ::windows_sys::core::Interface for IUserNotification {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2918704431, data2: 20051, data3: 17109, data4: [156, 51, 235, 94, 165, 21, 178, 62] };
}
#[repr(C)]
pub struct IUserNotificationChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChangeKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UserNotificationChangedKind) -> ::windows_sys::core::HRESULT,
    pub UserNotificationId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserNotificationChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3065866297, data2: 31183, data3: 19237, data4: [130, 192, 12, 225, 238, 248, 31, 140] };
}
pub type Notification = *mut ::core::ffi::c_void;
pub type NotificationBinding = *mut ::core::ffi::c_void;
pub type NotificationData = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct NotificationKinds(pub u32);
impl NotificationKinds {
    pub const Unknown: Self = Self(0u32);
    pub const Toast: Self = Self(1u32);
}
impl ::core::marker::Copy for NotificationKinds {}
impl ::core::clone::Clone for NotificationKinds {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct NotificationMirroring(pub i32);
impl NotificationMirroring {
    pub const Allowed: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
}
impl ::core::marker::Copy for NotificationMirroring {}
impl ::core::clone::Clone for NotificationMirroring {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct NotificationSetting(pub i32);
impl NotificationSetting {
    pub const Enabled: Self = Self(0i32);
    pub const DisabledForApplication: Self = Self(1i32);
    pub const DisabledForUser: Self = Self(2i32);
    pub const DisabledByGroupPolicy: Self = Self(3i32);
    pub const DisabledByManifest: Self = Self(4i32);
}
impl ::core::marker::Copy for NotificationSetting {}
impl ::core::clone::Clone for NotificationSetting {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct NotificationUpdateResult(pub i32);
impl NotificationUpdateResult {
    pub const Succeeded: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
    pub const NotificationNotFound: Self = Self(2i32);
}
impl ::core::marker::Copy for NotificationUpdateResult {}
impl ::core::clone::Clone for NotificationUpdateResult {
    fn clone(&self) -> Self {
        *self
    }
}
pub type NotificationVisual = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct PeriodicUpdateRecurrence(pub i32);
impl PeriodicUpdateRecurrence {
    pub const HalfHour: Self = Self(0i32);
    pub const Hour: Self = Self(1i32);
    pub const SixHours: Self = Self(2i32);
    pub const TwelveHours: Self = Self(3i32);
    pub const Daily: Self = Self(4i32);
}
impl ::core::marker::Copy for PeriodicUpdateRecurrence {}
impl ::core::clone::Clone for PeriodicUpdateRecurrence {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ScheduledTileNotification = *mut ::core::ffi::c_void;
pub type ScheduledToastNotification = *mut ::core::ffi::c_void;
pub type ScheduledToastNotificationShowingEventArgs = *mut ::core::ffi::c_void;
pub type ShownTileNotification = *mut ::core::ffi::c_void;
pub type TileFlyoutNotification = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct TileFlyoutTemplateType(pub i32);
impl TileFlyoutTemplateType {
    pub const TileFlyoutTemplate01: Self = Self(0i32);
}
impl ::core::marker::Copy for TileFlyoutTemplateType {}
impl ::core::clone::Clone for TileFlyoutTemplateType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TileFlyoutUpdater = *mut ::core::ffi::c_void;
pub type TileNotification = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct TileTemplateType(pub i32);
impl TileTemplateType {
    pub const TileSquareImage: Self = Self(0i32);
    pub const TileSquareBlock: Self = Self(1i32);
    pub const TileSquareText01: Self = Self(2i32);
    pub const TileSquareText02: Self = Self(3i32);
    pub const TileSquareText03: Self = Self(4i32);
    pub const TileSquareText04: Self = Self(5i32);
    pub const TileSquarePeekImageAndText01: Self = Self(6i32);
    pub const TileSquarePeekImageAndText02: Self = Self(7i32);
    pub const TileSquarePeekImageAndText03: Self = Self(8i32);
    pub const TileSquarePeekImageAndText04: Self = Self(9i32);
    pub const TileWideImage: Self = Self(10i32);
    pub const TileWideImageCollection: Self = Self(11i32);
    pub const TileWideImageAndText01: Self = Self(12i32);
    pub const TileWideImageAndText02: Self = Self(13i32);
    pub const TileWideBlockAndText01: Self = Self(14i32);
    pub const TileWideBlockAndText02: Self = Self(15i32);
    pub const TileWidePeekImageCollection01: Self = Self(16i32);
    pub const TileWidePeekImageCollection02: Self = Self(17i32);
    pub const TileWidePeekImageCollection03: Self = Self(18i32);
    pub const TileWidePeekImageCollection04: Self = Self(19i32);
    pub const TileWidePeekImageCollection05: Self = Self(20i32);
    pub const TileWidePeekImageCollection06: Self = Self(21i32);
    pub const TileWidePeekImageAndText01: Self = Self(22i32);
    pub const TileWidePeekImageAndText02: Self = Self(23i32);
    pub const TileWidePeekImage01: Self = Self(24i32);
    pub const TileWidePeekImage02: Self = Self(25i32);
    pub const TileWidePeekImage03: Self = Self(26i32);
    pub const TileWidePeekImage04: Self = Self(27i32);
    pub const TileWidePeekImage05: Self = Self(28i32);
    pub const TileWidePeekImage06: Self = Self(29i32);
    pub const TileWideSmallImageAndText01: Self = Self(30i32);
    pub const TileWideSmallImageAndText02: Self = Self(31i32);
    pub const TileWideSmallImageAndText03: Self = Self(32i32);
    pub const TileWideSmallImageAndText04: Self = Self(33i32);
    pub const TileWideSmallImageAndText05: Self = Self(34i32);
    pub const TileWideText01: Self = Self(35i32);
    pub const TileWideText02: Self = Self(36i32);
    pub const TileWideText03: Self = Self(37i32);
    pub const TileWideText04: Self = Self(38i32);
    pub const TileWideText05: Self = Self(39i32);
    pub const TileWideText06: Self = Self(40i32);
    pub const TileWideText07: Self = Self(41i32);
    pub const TileWideText08: Self = Self(42i32);
    pub const TileWideText09: Self = Self(43i32);
    pub const TileWideText10: Self = Self(44i32);
    pub const TileWideText11: Self = Self(45i32);
    pub const TileSquare150x150Image: Self = Self(0i32);
    pub const TileSquare150x150Block: Self = Self(1i32);
    pub const TileSquare150x150Text01: Self = Self(2i32);
    pub const TileSquare150x150Text02: Self = Self(3i32);
    pub const TileSquare150x150Text03: Self = Self(4i32);
    pub const TileSquare150x150Text04: Self = Self(5i32);
    pub const TileSquare150x150PeekImageAndText01: Self = Self(6i32);
    pub const TileSquare150x150PeekImageAndText02: Self = Self(7i32);
    pub const TileSquare150x150PeekImageAndText03: Self = Self(8i32);
    pub const TileSquare150x150PeekImageAndText04: Self = Self(9i32);
    pub const TileWide310x150Image: Self = Self(10i32);
    pub const TileWide310x150ImageCollection: Self = Self(11i32);
    pub const TileWide310x150ImageAndText01: Self = Self(12i32);
    pub const TileWide310x150ImageAndText02: Self = Self(13i32);
    pub const TileWide310x150BlockAndText01: Self = Self(14i32);
    pub const TileWide310x150BlockAndText02: Self = Self(15i32);
    pub const TileWide310x150PeekImageCollection01: Self = Self(16i32);
    pub const TileWide310x150PeekImageCollection02: Self = Self(17i32);
    pub const TileWide310x150PeekImageCollection03: Self = Self(18i32);
    pub const TileWide310x150PeekImageCollection04: Self = Self(19i32);
    pub const TileWide310x150PeekImageCollection05: Self = Self(20i32);
    pub const TileWide310x150PeekImageCollection06: Self = Self(21i32);
    pub const TileWide310x150PeekImageAndText01: Self = Self(22i32);
    pub const TileWide310x150PeekImageAndText02: Self = Self(23i32);
    pub const TileWide310x150PeekImage01: Self = Self(24i32);
    pub const TileWide310x150PeekImage02: Self = Self(25i32);
    pub const TileWide310x150PeekImage03: Self = Self(26i32);
    pub const TileWide310x150PeekImage04: Self = Self(27i32);
    pub const TileWide310x150PeekImage05: Self = Self(28i32);
    pub const TileWide310x150PeekImage06: Self = Self(29i32);
    pub const TileWide310x150SmallImageAndText01: Self = Self(30i32);
    pub const TileWide310x150SmallImageAndText02: Self = Self(31i32);
    pub const TileWide310x150SmallImageAndText03: Self = Self(32i32);
    pub const TileWide310x150SmallImageAndText04: Self = Self(33i32);
    pub const TileWide310x150SmallImageAndText05: Self = Self(34i32);
    pub const TileWide310x150Text01: Self = Self(35i32);
    pub const TileWide310x150Text02: Self = Self(36i32);
    pub const TileWide310x150Text03: Self = Self(37i32);
    pub const TileWide310x150Text04: Self = Self(38i32);
    pub const TileWide310x150Text05: Self = Self(39i32);
    pub const TileWide310x150Text06: Self = Self(40i32);
    pub const TileWide310x150Text07: Self = Self(41i32);
    pub const TileWide310x150Text08: Self = Self(42i32);
    pub const TileWide310x150Text09: Self = Self(43i32);
    pub const TileWide310x150Text10: Self = Self(44i32);
    pub const TileWide310x150Text11: Self = Self(45i32);
    pub const TileSquare310x310BlockAndText01: Self = Self(46i32);
    pub const TileSquare310x310BlockAndText02: Self = Self(47i32);
    pub const TileSquare310x310Image: Self = Self(48i32);
    pub const TileSquare310x310ImageAndText01: Self = Self(49i32);
    pub const TileSquare310x310ImageAndText02: Self = Self(50i32);
    pub const TileSquare310x310ImageAndTextOverlay01: Self = Self(51i32);
    pub const TileSquare310x310ImageAndTextOverlay02: Self = Self(52i32);
    pub const TileSquare310x310ImageAndTextOverlay03: Self = Self(53i32);
    pub const TileSquare310x310ImageCollectionAndText01: Self = Self(54i32);
    pub const TileSquare310x310ImageCollectionAndText02: Self = Self(55i32);
    pub const TileSquare310x310ImageCollection: Self = Self(56i32);
    pub const TileSquare310x310SmallImagesAndTextList01: Self = Self(57i32);
    pub const TileSquare310x310SmallImagesAndTextList02: Self = Self(58i32);
    pub const TileSquare310x310SmallImagesAndTextList03: Self = Self(59i32);
    pub const TileSquare310x310SmallImagesAndTextList04: Self = Self(60i32);
    pub const TileSquare310x310Text01: Self = Self(61i32);
    pub const TileSquare310x310Text02: Self = Self(62i32);
    pub const TileSquare310x310Text03: Self = Self(63i32);
    pub const TileSquare310x310Text04: Self = Self(64i32);
    pub const TileSquare310x310Text05: Self = Self(65i32);
    pub const TileSquare310x310Text06: Self = Self(66i32);
    pub const TileSquare310x310Text07: Self = Self(67i32);
    pub const TileSquare310x310Text08: Self = Self(68i32);
    pub const TileSquare310x310TextList01: Self = Self(69i32);
    pub const TileSquare310x310TextList02: Self = Self(70i32);
    pub const TileSquare310x310TextList03: Self = Self(71i32);
    pub const TileSquare310x310SmallImageAndText01: Self = Self(72i32);
    pub const TileSquare310x310SmallImagesAndTextList05: Self = Self(73i32);
    pub const TileSquare310x310Text09: Self = Self(74i32);
    pub const TileSquare71x71IconWithBadge: Self = Self(75i32);
    pub const TileSquare150x150IconWithBadge: Self = Self(76i32);
    pub const TileWide310x150IconWithBadgeAndText: Self = Self(77i32);
    pub const TileSquare71x71Image: Self = Self(78i32);
    pub const TileTall150x310Image: Self = Self(79i32);
}
impl ::core::marker::Copy for TileTemplateType {}
impl ::core::clone::Clone for TileTemplateType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TileUpdateManagerForUser = *mut ::core::ffi::c_void;
pub type TileUpdater = *mut ::core::ffi::c_void;
pub type ToastActivatedEventArgs = *mut ::core::ffi::c_void;
pub type ToastCollection = *mut ::core::ffi::c_void;
pub type ToastCollectionManager = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct ToastDismissalReason(pub i32);
impl ToastDismissalReason {
    pub const UserCanceled: Self = Self(0i32);
    pub const ApplicationHidden: Self = Self(1i32);
    pub const TimedOut: Self = Self(2i32);
}
impl ::core::marker::Copy for ToastDismissalReason {}
impl ::core::clone::Clone for ToastDismissalReason {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ToastDismissedEventArgs = *mut ::core::ffi::c_void;
pub type ToastFailedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct ToastHistoryChangedType(pub i32);
impl ToastHistoryChangedType {
    pub const Cleared: Self = Self(0i32);
    pub const Removed: Self = Self(1i32);
    pub const Expired: Self = Self(2i32);
    pub const Added: Self = Self(3i32);
}
impl ::core::marker::Copy for ToastHistoryChangedType {}
impl ::core::clone::Clone for ToastHistoryChangedType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ToastNotification = *mut ::core::ffi::c_void;
pub type ToastNotificationActionTriggerDetail = *mut ::core::ffi::c_void;
pub type ToastNotificationHistory = *mut ::core::ffi::c_void;
pub type ToastNotificationHistoryChangedTriggerDetail = *mut ::core::ffi::c_void;
pub type ToastNotificationManagerForUser = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct ToastNotificationPriority(pub i32);
impl ToastNotificationPriority {
    pub const Default: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl ::core::marker::Copy for ToastNotificationPriority {}
impl ::core::clone::Clone for ToastNotificationPriority {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ToastNotifier = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct ToastTemplateType(pub i32);
impl ToastTemplateType {
    pub const ToastImageAndText01: Self = Self(0i32);
    pub const ToastImageAndText02: Self = Self(1i32);
    pub const ToastImageAndText03: Self = Self(2i32);
    pub const ToastImageAndText04: Self = Self(3i32);
    pub const ToastText01: Self = Self(4i32);
    pub const ToastText02: Self = Self(5i32);
    pub const ToastText03: Self = Self(6i32);
    pub const ToastText04: Self = Self(7i32);
}
impl ::core::marker::Copy for ToastTemplateType {}
impl ::core::clone::Clone for ToastTemplateType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type UserNotification = *mut ::core::ffi::c_void;
pub type UserNotificationChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Notifications\"`*"]
#[repr(transparent)]
pub struct UserNotificationChangedKind(pub i32);
impl UserNotificationChangedKind {
    pub const Added: Self = Self(0i32);
    pub const Removed: Self = Self(1i32);
}
impl ::core::marker::Copy for UserNotificationChangedKind {}
impl ::core::clone::Clone for UserNotificationChangedKind {
    fn clone(&self) -> Self {
        *self
    }
}
