#[doc = "*Required features: `\"System_UserProfile\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct AccountPictureKind(pub i32);
#[cfg(feature = "deprecated")]
impl AccountPictureKind {
    pub const SmallImage: Self = Self(0i32);
    pub const LargeImage: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for AccountPictureKind {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AccountPictureKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AdvertisingManagerForUser = *mut ::core::ffi::c_void;
pub type AssignedAccessSettings = *mut ::core::ffi::c_void;
pub type DiagnosticsSettings = *mut ::core::ffi::c_void;
pub type FirstSignInSettings = *mut ::core::ffi::c_void;
pub type GlobalizationPreferencesForUser = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAdvertisingManagerForUser {
    pub base__: ::windows_sys::core::IInspectable,
    pub AdvertisingId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAdvertisingManagerForUser {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2458645456, data2: 53116, data3: 19120, data4: [167, 220, 109, 197, 188, 212, 66, 82] };
}
#[repr(C)]
pub struct IAdvertisingManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub AdvertisingId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAdvertisingManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2916304524, data2: 41587, data3: 18635, data4: [179, 70, 53, 68, 82, 45, 85, 129] };
}
#[repr(C)]
pub struct IAdvertisingManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAdvertisingManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3708372911, data2: 6765, data3: 18096, data4: [149, 188, 243, 249, 214, 190, 185, 251] };
}
#[repr(C)]
pub struct IAssignedAccessSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsSingleAppKioskMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAssignedAccessSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 465927964, data2: 59761, data3: 22359, data4: [184, 224, 81, 47, 139, 140, 70, 210] };
}
#[repr(C)]
pub struct IAssignedAccessSettingsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAssignedAccessSettingsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 883432717, data2: 35369, data3: 24307, data4: [167, 190, 97, 142, 106, 195, 189, 1] };
}
#[repr(C)]
pub struct IDiagnosticsSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanUseDiagnosticsToTailorExperiences: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDiagnosticsSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3857312973, data2: 10001, data3: 17632, data4: [151, 60, 73, 29, 120, 4, 141, 36] };
}
#[repr(C)]
pub struct IDiagnosticsSettingsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDiagnosticsSettingsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1926424591, data2: 21392, data3: 18323, data4: [153, 11, 60, 204, 125, 106, 201, 200] };
}
#[repr(C)]
pub struct IFirstSignInSettings {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IFirstSignInSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1049907539, data2: 14942, data3: 17710, data4: [166, 1, 245, 186, 173, 42, 72, 112] };
}
#[repr(C)]
pub struct IFirstSignInSettingsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFirstSignInSettingsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 484544271, data2: 7233, data3: 20128, data4: [183, 162, 111, 12, 28, 126, 132, 56] };
}
#[repr(C)]
pub struct IGlobalizationPreferencesForUser {
    pub base__: ::windows_sys::core::IInspectable,
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Calendars: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Calendars: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Clocks: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Clocks: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Currencies: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Currencies: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Languages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Languages: usize,
    pub HomeGeographicRegion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Globalization")]
    pub WeekStartsOn: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Globalization::DayOfWeek) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    WeekStartsOn: usize,
}
impl ::windows_sys::core::Interface for IGlobalizationPreferencesForUser {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 353306517, data2: 20334, data3: 16570, data4: [160, 16, 226, 125, 129, 189, 167, 245] };
}
#[repr(C)]
pub struct IGlobalizationPreferencesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Calendars: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Calendars: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Clocks: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Clocks: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Currencies: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Currencies: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Languages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Languages: usize,
    pub HomeGeographicRegion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Globalization")]
    pub WeekStartsOn: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Globalization::DayOfWeek) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    WeekStartsOn: usize,
}
impl ::windows_sys::core::Interface for IGlobalizationPreferencesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 29311782, data2: 60727, data3: 20118, data4: [176, 233, 193, 52, 13, 30, 161, 88] };
}
#[repr(C)]
pub struct IGlobalizationPreferencesStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TrySetHomeGeographicRegion: unsafe extern "system" fn(this: *mut *mut Self, region: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TrySetLanguages: unsafe extern "system" fn(this: *mut *mut Self, languagetags: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TrySetLanguages: usize,
}
impl ::windows_sys::core::Interface for IGlobalizationPreferencesStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4241393137, data2: 17152, data3: 19664, data4: [156, 172, 26, 142, 123, 126, 24, 244] };
}
#[repr(C)]
pub struct IGlobalizationPreferencesStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGlobalizationPreferencesStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 503682867, data2: 13813, data3: 16600, data4: [185, 232, 174, 243, 239, 133, 111, 206] };
}
#[repr(C)]
pub struct ILockScreenImageFeedStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestSetImageFeedAsync: unsafe extern "system" fn(this: *mut *mut Self, syndicationfeeduri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSetImageFeedAsync: usize,
    pub TryRemoveImageFeed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILockScreenImageFeedStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 739079158, data2: 937, data3: 16806, data4: [155, 1, 73, 82, 81, 255, 81, 213] };
}
#[repr(C)]
pub struct ILockScreenStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub OriginalImageFile: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OriginalImageFile: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GetImageStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetImageStream: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub SetImageFileAsync: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    SetImageFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SetImageStreamAsync: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SetImageStreamAsync: usize,
}
impl ::windows_sys::core::Interface for ILockScreenStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1055511469, data2: 46599, data3: 16558, data4: [180, 38, 118, 49, 217, 130, 18, 105] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IUserInformationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub AccountPictureChangeEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AccountPictureChangeEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub NameAccessAllowed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    NameAccessAllowed: usize,
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub GetAccountPicture: unsafe extern "system" fn(this: *mut *mut Self, kind: AccountPictureKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Storage", feature = "deprecated")))]
    GetAccountPicture: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "deprecated"))]
    pub SetAccountPictureAsync: unsafe extern "system" fn(this: *mut *mut Self, image: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage", feature = "deprecated")))]
    SetAccountPictureAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "deprecated"))]
    pub SetAccountPicturesAsync: unsafe extern "system" fn(this: *mut *mut Self, smallimage: *mut ::core::ffi::c_void, largeimage: *mut ::core::ffi::c_void, video: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage", feature = "deprecated")))]
    SetAccountPicturesAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub SetAccountPictureFromStreamAsync: unsafe extern "system" fn(this: *mut *mut Self, image: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated")))]
    SetAccountPictureFromStreamAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub SetAccountPicturesFromStreamsAsync: unsafe extern "system" fn(this: *mut *mut Self, smallimage: *mut ::core::ffi::c_void, largeimage: *mut ::core::ffi::c_void, video: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated")))]
    SetAccountPicturesFromStreamsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub AccountPictureChanged: unsafe extern "system" fn(this: *mut *mut Self, changehandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    AccountPictureChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveAccountPictureChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveAccountPictureChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetDisplayNameAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetDisplayNameAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetFirstNameAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetFirstNameAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetLastNameAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetLastNameAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetPrincipalNameAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetPrincipalNameAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetSessionInitiationProtocolUriAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetSessionInitiationProtocolUriAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetDomainNameAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetDomainNameAsync: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IUserInformationStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2012457232, data2: 18682, data3: 18588, data4: [147, 78, 42, 232, 91, 168, 247, 114] };
}
#[repr(C)]
pub struct IUserProfilePersonalizationSettings {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub TrySetLockScreenImageAsync: unsafe extern "system" fn(this: *mut *mut Self, imagefile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    TrySetLockScreenImageAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub TrySetWallpaperImageAsync: unsafe extern "system" fn(this: *mut *mut Self, imagefile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    TrySetWallpaperImageAsync: usize,
}
impl ::windows_sys::core::Interface for IUserProfilePersonalizationSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2364398260, data2: 31128, data3: 18133, data4: [141, 211, 24, 79, 28, 95, 154, 185] };
}
#[repr(C)]
pub struct IUserProfilePersonalizationSettingsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Current: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserProfilePersonalizationSettingsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2444015681, data2: 20535, data3: 17739, data4: [152, 131, 187, 119, 45, 8, 221, 22] };
}
#[doc = "*Required features: `\"System_UserProfile\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SetAccountPictureResult(pub i32);
#[cfg(feature = "deprecated")]
impl SetAccountPictureResult {
    pub const Success: Self = Self(0i32);
    pub const ChangeDisabled: Self = Self(1i32);
    pub const LargeOrDynamicError: Self = Self(2i32);
    pub const VideoFrameSizeError: Self = Self(3i32);
    pub const FileSizeError: Self = Self(4i32);
    pub const Failure: Self = Self(5i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SetAccountPictureResult {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SetAccountPictureResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"System_UserProfile\"`*"]
#[repr(transparent)]
pub struct SetImageFeedResult(pub i32);
impl SetImageFeedResult {
    pub const Success: Self = Self(0i32);
    pub const ChangeDisabled: Self = Self(1i32);
    pub const UserCanceled: Self = Self(2i32);
}
impl ::core::marker::Copy for SetImageFeedResult {}
impl ::core::clone::Clone for SetImageFeedResult {
    fn clone(&self) -> Self {
        *self
    }
}
pub type UserProfilePersonalizationSettings = *mut ::core::ffi::c_void;
