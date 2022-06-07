#[repr(C)]
pub struct ITargetedContentAction {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub InvokeAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InvokeAsync: usize,
}
impl ::windows_sys::core::Interface for ITargetedContentAction {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3613092126, data2: 27862, data3: 19616, data4: [157, 143, 71, 40, 176, 183, 230, 182] };
}
#[repr(C)]
pub struct ITargetedContentAvailabilityChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for ITargetedContentAvailabilityChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3774192934, data2: 22823, data3: 17488, data4: [150, 92, 28, 235, 123, 236, 222, 101] };
}
#[repr(C)]
pub struct ITargetedContentChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
    pub HasPreviousContentExpired: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITargetedContentChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2580842697, data2: 22654, data3: 17798, data4: [142, 247, 181, 76, 169, 69, 58, 22] };
}
#[repr(C)]
pub struct ITargetedContentCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ReportInteraction: unsafe extern "system" fn(this: *mut *mut Self, interaction: TargetedContentInteraction) -> ::windows_sys::core::HRESULT,
    pub ReportCustomInteraction: unsafe extern "system" fn(this: *mut *mut Self, custominteractionname: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Collections: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Collections: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
}
impl ::windows_sys::core::Interface for ITargetedContentCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 759916229, data2: 61795, data3: 17594, data4: [159, 110, 225, 164, 194, 187, 85, 157] };
}
#[repr(C)]
pub struct ITargetedContentContainer {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub Availability: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TargetedContentAvailability) -> ::windows_sys::core::HRESULT,
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectSingleObject: unsafe extern "system" fn(this: *mut *mut Self, path: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITargetedContentContainer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3156513993, data2: 34871, data3: 18370, data4: [133, 15, 215, 157, 100, 89, 89, 38] };
}
#[repr(C)]
pub struct ITargetedContentContainerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetAsync: unsafe extern "system" fn(this: *mut *mut Self, contentid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAsync: usize,
}
impl ::windows_sys::core::Interface for ITargetedContentContainerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1531439099, data2: 8512, data3: 19487, data4: [167, 54, 197, 149, 131, 242, 39, 216] };
}
#[repr(C)]
pub struct ITargetedContentImage {
    pub base__: ::windows_sys::core::IInspectable,
    pub Height: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITargetedContentImage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2812642777, data2: 30623, data3: 19230, data4: [187, 177, 142, 175, 83, 251, 234, 178] };
}
#[repr(C)]
pub struct ITargetedContentItem {
    pub base__: ::windows_sys::core::IInspectable,
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ReportInteraction: unsafe extern "system" fn(this: *mut *mut Self, interaction: TargetedContentInteraction) -> ::windows_sys::core::HRESULT,
    pub ReportCustomInteraction: unsafe extern "system" fn(this: *mut *mut Self, custominteractionname: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Collections: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Collections: usize,
}
impl ::windows_sys::core::Interface for ITargetedContentItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 941002180, data2: 10092, data3: 19506, data4: [150, 186, 86, 92, 110, 64, 110, 116] };
}
#[repr(C)]
pub struct ITargetedContentItemState {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShouldDisplay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub AppInstallationState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TargetedContentAppInstallationState) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITargetedContentItemState {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1939035220, data2: 19557, data3: 19271, data4: [164, 65, 71, 45, 229, 60, 121, 182] };
}
#[repr(C)]
pub struct ITargetedContentObject {
    pub base__: ::windows_sys::core::IInspectable,
    pub ObjectKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TargetedContentObjectKind) -> ::windows_sys::core::HRESULT,
    pub Collection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITargetedContentObject {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 69040489, data2: 8722, data3: 17105, data4: [157, 250, 136, 168, 227, 3, 58, 163] };
}
#[repr(C)]
pub struct ITargetedContentStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for ITargetedContentStateChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2585587517, data2: 32883, data3: 17430, data4: [141, 242, 84, 104, 53, 166, 65, 79] };
}
#[repr(C)]
pub struct ITargetedContentSubscription {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetContentContainerAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetContentContainerAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ContentChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContentChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContentChanged: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContentChanged: usize,
    #[cfg(feature = "Foundation")]
    pub AvailabilityChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AvailabilityChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAvailabilityChanged: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAvailabilityChanged: usize,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
}
impl ::windows_sys::core::Interface for ITargetedContentSubscription {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2284596297, data2: 50770, data3: 19578, data4: [172, 173, 31, 127, 162, 152, 108, 115] };
}
#[repr(C)]
pub struct ITargetedContentSubscriptionOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub SubscriptionId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AllowPartialContentAvailability: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowPartialContentAvailability: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CloudQueryParameters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CloudQueryParameters: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LocalFilters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LocalFilters: usize,
    pub Update: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITargetedContentSubscriptionOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1643014864, data2: 11395, data3: 16923, data4: [132, 103, 65, 62, 175, 26, 235, 151] };
}
#[repr(C)]
pub struct ITargetedContentSubscriptionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetAsync: unsafe extern "system" fn(this: *mut *mut Self, subscriptionid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAsync: usize,
    pub GetOptions: unsafe extern "system" fn(this: *mut *mut Self, subscriptionid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITargetedContentSubscriptionStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4208852608, data2: 13837, data3: 18710, data4: [181, 60, 126, 162, 112, 144, 208, 42] };
}
#[repr(C)]
pub struct ITargetedContentValue {
    pub base__: ::windows_sys::core::IInspectable,
    pub ValueKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TargetedContentValueKind) -> ::windows_sys::core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub String: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub Number: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub Boolean: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub File: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    File: usize,
    pub ImageFile: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Action: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Strings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Strings: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Uris: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Uris: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Numbers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Numbers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Booleans: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Booleans: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub Files: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    Files: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ImageFiles: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ImageFiles: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Actions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Actions: usize,
}
impl ::windows_sys::core::Interface for ITargetedContentValue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2868765875, data2: 16917, data3: 19448, data4: [134, 127, 67, 240, 72, 101, 249, 191] };
}
pub type TargetedContentAction = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Services_TargetedContent\"`*"]
#[repr(transparent)]
pub struct TargetedContentAppInstallationState(pub i32);
impl TargetedContentAppInstallationState {
    pub const NotApplicable: Self = Self(0i32);
    pub const NotInstalled: Self = Self(1i32);
    pub const Installed: Self = Self(2i32);
}
impl ::core::marker::Copy for TargetedContentAppInstallationState {}
impl ::core::clone::Clone for TargetedContentAppInstallationState {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Services_TargetedContent\"`*"]
#[repr(transparent)]
pub struct TargetedContentAvailability(pub i32);
impl TargetedContentAvailability {
    pub const None: Self = Self(0i32);
    pub const Partial: Self = Self(1i32);
    pub const All: Self = Self(2i32);
}
impl ::core::marker::Copy for TargetedContentAvailability {}
impl ::core::clone::Clone for TargetedContentAvailability {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TargetedContentAvailabilityChangedEventArgs = *mut ::core::ffi::c_void;
pub type TargetedContentChangedEventArgs = *mut ::core::ffi::c_void;
pub type TargetedContentCollection = *mut ::core::ffi::c_void;
pub type TargetedContentContainer = *mut ::core::ffi::c_void;
pub type TargetedContentFile = *mut ::core::ffi::c_void;
pub type TargetedContentImage = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Services_TargetedContent\"`*"]
#[repr(transparent)]
pub struct TargetedContentInteraction(pub i32);
impl TargetedContentInteraction {
    pub const Impression: Self = Self(0i32);
    pub const ClickThrough: Self = Self(1i32);
    pub const Hover: Self = Self(2i32);
    pub const Like: Self = Self(3i32);
    pub const Dislike: Self = Self(4i32);
    pub const Dismiss: Self = Self(5i32);
    pub const Ineligible: Self = Self(6i32);
    pub const Accept: Self = Self(7i32);
    pub const Decline: Self = Self(8i32);
    pub const Defer: Self = Self(9i32);
    pub const Canceled: Self = Self(10i32);
    pub const Conversion: Self = Self(11i32);
    pub const Opportunity: Self = Self(12i32);
}
impl ::core::marker::Copy for TargetedContentInteraction {}
impl ::core::clone::Clone for TargetedContentInteraction {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TargetedContentItem = *mut ::core::ffi::c_void;
pub type TargetedContentItemState = *mut ::core::ffi::c_void;
pub type TargetedContentObject = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Services_TargetedContent\"`*"]
#[repr(transparent)]
pub struct TargetedContentObjectKind(pub i32);
impl TargetedContentObjectKind {
    pub const Collection: Self = Self(0i32);
    pub const Item: Self = Self(1i32);
    pub const Value: Self = Self(2i32);
}
impl ::core::marker::Copy for TargetedContentObjectKind {}
impl ::core::clone::Clone for TargetedContentObjectKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TargetedContentStateChangedEventArgs = *mut ::core::ffi::c_void;
pub type TargetedContentSubscription = *mut ::core::ffi::c_void;
pub type TargetedContentSubscriptionOptions = *mut ::core::ffi::c_void;
pub type TargetedContentValue = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Services_TargetedContent\"`*"]
#[repr(transparent)]
pub struct TargetedContentValueKind(pub i32);
impl TargetedContentValueKind {
    pub const String: Self = Self(0i32);
    pub const Uri: Self = Self(1i32);
    pub const Number: Self = Self(2i32);
    pub const Boolean: Self = Self(3i32);
    pub const File: Self = Self(4i32);
    pub const ImageFile: Self = Self(5i32);
    pub const Action: Self = Self(6i32);
    pub const Strings: Self = Self(7i32);
    pub const Uris: Self = Self(8i32);
    pub const Numbers: Self = Self(9i32);
    pub const Booleans: Self = Self(10i32);
    pub const Files: Self = Self(11i32);
    pub const ImageFiles: Self = Self(12i32);
    pub const Actions: Self = Self(13i32);
}
impl ::core::marker::Copy for TargetedContentValueKind {}
impl ::core::clone::Clone for TargetedContentValueKind {
    fn clone(&self) -> Self {
        *self
    }
}
