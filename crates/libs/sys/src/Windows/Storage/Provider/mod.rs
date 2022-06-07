#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct CachedFileOptions(pub u32);
impl CachedFileOptions {
    pub const None: Self = Self(0u32);
    pub const RequireUpdateOnAccess: Self = Self(1u32);
    pub const UseCachedFileWhenOffline: Self = Self(2u32);
    pub const DenyAccessWhenOffline: Self = Self(4u32);
}
impl ::core::marker::Copy for CachedFileOptions {}
impl ::core::clone::Clone for CachedFileOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct CachedFileTarget(pub i32);
impl CachedFileTarget {
    pub const Local: Self = Self(0i32);
    pub const Remote: Self = Self(1i32);
}
impl ::core::marker::Copy for CachedFileTarget {}
impl ::core::clone::Clone for CachedFileTarget {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CachedFileUpdaterUI = *mut ::core::ffi::c_void;
pub type FileUpdateRequest = *mut ::core::ffi::c_void;
pub type FileUpdateRequestDeferral = *mut ::core::ffi::c_void;
pub type FileUpdateRequestedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct FileUpdateStatus(pub i32);
impl FileUpdateStatus {
    pub const Incomplete: Self = Self(0i32);
    pub const Complete: Self = Self(1i32);
    pub const UserInputNeeded: Self = Self(2i32);
    pub const CurrentlyUnavailable: Self = Self(3i32);
    pub const Failed: Self = Self(4i32);
    pub const CompleteAndRenamed: Self = Self(5i32);
}
impl ::core::marker::Copy for FileUpdateStatus {}
impl ::core::clone::Clone for FileUpdateStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ICachedFileUpdaterStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetUpdateInformation: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, contentid: ::windows_sys::core::HSTRING, readmode: ReadActivationMode, writemode: WriteActivationMode, options: CachedFileOptions) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICachedFileUpdaterStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2680752416, data2: 31695, data3: 18568, data4: [168, 30, 16, 45, 112, 52, 215, 206] };
}
#[repr(C)]
pub struct ICachedFileUpdaterUI {
    pub base__: ::windows_sys::core::IInspectable,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub UpdateTarget: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CachedFileTarget) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FileUpdateRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FileUpdateRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFileUpdateRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFileUpdateRequested: usize,
    #[cfg(feature = "Foundation")]
    pub UIRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UIRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUIRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUIRequested: usize,
    pub UIStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UIStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICachedFileUpdaterUI {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2658091494, data2: 47858, data3: 19095, data4: [182, 0, 147, 51, 245, 223, 128, 253] };
}
#[repr(C)]
pub struct ICachedFileUpdaterUI2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub UpdateRequest: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICachedFileUpdaterUI2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2287378972, data2: 34457, data3: 17216, data4: [159, 73, 247, 202, 215, 254, 137, 145] };
}
#[repr(C)]
pub struct IFileUpdateRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContentId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub File: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FileUpdateStatus) -> ::windows_sys::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut *mut Self, value: FileUpdateStatus) -> ::windows_sys::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UpdateLocalFile: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFileUpdateRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1086858550, data2: 49662, data3: 19859, data4: [167, 146, 30, 115, 107, 199, 8, 55] };
}
#[repr(C)]
pub struct IFileUpdateRequest2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub UserInputNeededMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetUserInputNeededMessage: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFileUpdateRequest2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2185774664, data2: 48574, data3: 17531, data4: [162, 238, 122, 254, 106, 3, 42, 148] };
}
#[repr(C)]
pub struct IFileUpdateRequestDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFileUpdateRequestDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4291746603, data2: 35550, data3: 17573, data4: [187, 0, 22, 76, 78, 114, 241, 58] };
}
#[repr(C)]
pub struct IFileUpdateRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFileUpdateRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2064290626, data2: 14597, data3: 17293, data4: [170, 239, 120, 174, 38, 95, 141, 210] };
}
#[repr(C)]
pub struct IStorageProviderError {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FilePath: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetFilePath: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PrimaryAction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPrimaryAction: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SecondaryAction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSecondaryAction: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InformationalLink: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetInformationalLink: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorageProviderError {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1207072779, data2: 61311, data3: 22800, data4: [191, 131, 51, 29, 137, 37, 102, 21] };
}
#[repr(C)]
pub struct IStorageProviderErrorCommand {
    pub base__: ::windows_sys::core::IInspectable,
    pub Label: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ActionUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ActionUri: usize,
}
impl ::windows_sys::core::Interface for IStorageProviderErrorCommand {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3065088749, data2: 47973, data3: 24358, data4: [134, 228, 29, 62, 52, 213, 68, 119] };
}
#[repr(C)]
pub struct IStorageProviderErrorCommandFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, label: ::windows_sys::core::HSTRING, actionuri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInstance: usize,
}
impl ::windows_sys::core::Interface for IStorageProviderErrorCommandFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3972134229, data2: 15028, data3: 21871, data4: [139, 178, 126, 85, 21, 238, 216, 220] };
}
#[repr(C)]
pub struct IStorageProviderErrorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, title: ::windows_sys::core::HSTRING, message: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorageProviderErrorFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2547446336, data2: 25003, data3: 20956, data4: [153, 33, 24, 189, 13, 190, 247, 158] };
}
#[repr(C)]
pub struct IStorageProviderFileTypeInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub FileExtension: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IconResource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorageProviderFileTypeInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 425048513, data2: 388, data3: 23176, data4: [135, 223, 69, 68, 244, 100, 54, 93] };
}
#[repr(C)]
pub struct IStorageProviderFileTypeInfoFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, fileextension: ::windows_sys::core::HSTRING, iconresource: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorageProviderFileTypeInfoFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1067527279, data2: 52454, data3: 23901, data4: [128, 177, 56, 158, 124, 249, 45, 191] };
}
#[repr(C)]
pub struct IStorageProviderGetContentInfoForPathResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StorageProviderUriSourceStatus) -> ::windows_sys::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut *mut Self, value: StorageProviderUriSourceStatus) -> ::windows_sys::core::HRESULT,
    pub ContentUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetContentUri: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ContentId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetContentId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorageProviderGetContentInfoForPathResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 627339549, data2: 43657, data3: 19730, data4: [130, 227, 247, 42, 146, 227, 57, 102] };
}
#[repr(C)]
pub struct IStorageProviderGetPathForContentUriResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StorageProviderUriSourceStatus) -> ::windows_sys::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut *mut Self, value: StorageProviderUriSourceStatus) -> ::windows_sys::core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPath: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorageProviderGetPathForContentUriResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1668356765, data2: 16664, data3: 17830, data4: [172, 182, 34, 196, 157, 1, 159, 64] };
}
#[repr(C)]
pub struct IStorageProviderHandlerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetStatusSource: unsafe extern "system" fn(this: *mut *mut Self, syncrootid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorageProviderHandlerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1632951354, data2: 64541, data3: 23214, data4: [158, 35, 232, 101, 154, 34, 197, 246] };
}
#[repr(C)]
pub struct IStorageProviderItemPropertiesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub SetAsync: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, itemproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetAsync: usize,
}
impl ::windows_sys::core::Interface for IStorageProviderItemPropertiesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 757865623, data2: 9988, data3: 18217, data4: [143, 169, 126, 107, 142, 21, 140, 47] };
}
#[repr(C)]
pub struct IStorageProviderItemProperty {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetId: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetIconResource: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IconResource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorageProviderItemProperty {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1198306648, data2: 29451, data3: 16776, data4: [183, 181, 99, 183, 22, 237, 71, 109] };
}
#[repr(C)]
pub struct IStorageProviderItemPropertyDefinition {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub DisplayNameResource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayNameResource: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorageProviderItemPropertyDefinition {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3316876219, data2: 65311, data3: 17048, data4: [131, 30, 255, 28, 8, 8, 150, 144] };
}
#[repr(C)]
pub struct IStorageProviderItemPropertySource {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemProperties: unsafe extern "system" fn(this: *mut *mut Self, itempath: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemProperties: usize,
}
impl ::windows_sys::core::Interface for IStorageProviderItemPropertySource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2406456382, data2: 63026, data3: 19099, data4: [141, 153, 210, 215, 161, 29, 245, 106] };
}
#[repr(C)]
pub struct IStorageProviderPropertyCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsPropertySupported: unsafe extern "system" fn(this: *mut *mut Self, propertycanonicalname: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorageProviderPropertyCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1703751438, data2: 25527, data3: 17767, data4: [172, 249, 81, 171, 227, 1, 221, 165] };
}
#[repr(C)]
pub struct IStorageProviderStatus {
    pub base__: ::windows_sys::core::IInspectable,
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StorageProviderState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ErrorMessages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ErrorMessages: usize,
}
impl ::windows_sys::core::Interface for IStorageProviderStatus {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4285429277, data2: 64395, data3: 22211, data4: [158, 122, 5, 48, 157, 25, 31, 180] };
}
#[repr(C)]
pub struct IStorageProviderStatusFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, state: StorageProviderState, message: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateInstance2: unsafe extern "system" fn(this: *mut *mut Self, state: StorageProviderState, message: ::windows_sys::core::HSTRING, errormessages: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateInstance2: usize,
}
impl ::windows_sys::core::Interface for IStorageProviderStatusFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3595053253, data2: 39802, data3: 24484, data4: [177, 38, 144, 189, 24, 147, 108, 127] };
}
#[repr(C)]
pub struct IStorageProviderStatusSource {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Changed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Changed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChanged: usize,
}
impl ::windows_sys::core::Interface for IStorageProviderStatusSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 774990770, data2: 64835, data3: 21301, data4: [179, 196, 169, 98, 238, 49, 209, 126] };
}
#[repr(C)]
pub struct IStorageProviderSyncRootInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Context: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Context: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetContext: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetContext: usize,
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPath: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DisplayNameResource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayNameResource: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IconResource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetIconResource: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HydrationPolicy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StorageProviderHydrationPolicy) -> ::windows_sys::core::HRESULT,
    pub SetHydrationPolicy: unsafe extern "system" fn(this: *mut *mut Self, value: StorageProviderHydrationPolicy) -> ::windows_sys::core::HRESULT,
    pub HydrationPolicyModifier: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StorageProviderHydrationPolicyModifier) -> ::windows_sys::core::HRESULT,
    pub SetHydrationPolicyModifier: unsafe extern "system" fn(this: *mut *mut Self, value: StorageProviderHydrationPolicyModifier) -> ::windows_sys::core::HRESULT,
    pub PopulationPolicy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StorageProviderPopulationPolicy) -> ::windows_sys::core::HRESULT,
    pub SetPopulationPolicy: unsafe extern "system" fn(this: *mut *mut Self, value: StorageProviderPopulationPolicy) -> ::windows_sys::core::HRESULT,
    pub InSyncPolicy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StorageProviderInSyncPolicy) -> ::windows_sys::core::HRESULT,
    pub SetInSyncPolicy: unsafe extern "system" fn(this: *mut *mut Self, value: StorageProviderInSyncPolicy) -> ::windows_sys::core::HRESULT,
    pub HardlinkPolicy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StorageProviderHardlinkPolicy) -> ::windows_sys::core::HRESULT,
    pub SetHardlinkPolicy: unsafe extern "system" fn(this: *mut *mut Self, value: StorageProviderHardlinkPolicy) -> ::windows_sys::core::HRESULT,
    pub ShowSiblingsAsGroup: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetShowSiblingsAsGroup: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ProtectionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StorageProviderProtectionMode) -> ::windows_sys::core::HRESULT,
    pub SetProtectionMode: unsafe extern "system" fn(this: *mut *mut Self, value: StorageProviderProtectionMode) -> ::windows_sys::core::HRESULT,
    pub AllowPinning: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowPinning: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub StorageProviderItemPropertyDefinitions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StorageProviderItemPropertyDefinitions: usize,
    #[cfg(feature = "Foundation")]
    pub RecycleBinUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecycleBinUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetRecycleBinUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRecycleBinUri: usize,
}
impl ::windows_sys::core::Interface for IStorageProviderSyncRootInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2081621444, data2: 39417, data3: 16812, data4: [137, 4, 171, 5, 93, 101, 73, 38] };
}
#[repr(C)]
pub struct IStorageProviderSyncRootInfo2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProviderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetProviderId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorageProviderSyncRootInfo2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3478237219, data2: 31985, data3: 20838, data4: [189, 186, 239, 217, 95, 82, 158, 49] };
}
#[repr(C)]
pub struct IStorageProviderSyncRootInfo3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub FallbackFileTypeInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FallbackFileTypeInfo: usize,
}
impl ::windows_sys::core::Interface for IStorageProviderSyncRootInfo3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1350198807, data2: 48886, data3: 22269, data4: [133, 94, 117, 172, 226, 228, 92, 245] };
}
#[repr(C)]
pub struct IStorageProviderSyncRootManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Register: unsafe extern "system" fn(this: *mut *mut Self, syncrootinformation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Unregister: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetSyncRootInformationForFolder: unsafe extern "system" fn(this: *mut *mut Self, folder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSyncRootInformationForId: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCurrentSyncRoots: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCurrentSyncRoots: usize,
}
impl ::windows_sys::core::Interface for IStorageProviderSyncRootManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1050278847, data2: 36835, data3: 19264, data4: [171, 199, 246, 252, 61, 116, 201, 142] };
}
#[repr(C)]
pub struct IStorageProviderSyncRootManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorageProviderSyncRootManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4021735406, data2: 4980, data3: 21582, data4: [157, 241, 85, 152, 210, 233, 207, 221] };
}
#[repr(C)]
pub struct IStorageProviderUriSource {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetPathForContentUri: unsafe extern "system" fn(this: *mut *mut Self, contenturi: ::windows_sys::core::HSTRING, result: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetContentInfoForPath: unsafe extern "system" fn(this: *mut *mut Self, path: ::windows_sys::core::HSTRING, result: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorageProviderUriSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2996307665, data2: 35808, data3: 18786, data4: [139, 182, 13, 76, 46, 20, 212, 122] };
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct ReadActivationMode(pub i32);
impl ReadActivationMode {
    pub const NotNeeded: Self = Self(0i32);
    pub const BeforeAccess: Self = Self(1i32);
}
impl ::core::marker::Copy for ReadActivationMode {}
impl ::core::clone::Clone for ReadActivationMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type StorageProviderError = *mut ::core::ffi::c_void;
pub type StorageProviderErrorCommand = *mut ::core::ffi::c_void;
pub type StorageProviderFileTypeInfo = *mut ::core::ffi::c_void;
pub type StorageProviderGetContentInfoForPathResult = *mut ::core::ffi::c_void;
pub type StorageProviderGetPathForContentUriResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderHardlinkPolicy(pub u32);
impl StorageProviderHardlinkPolicy {
    pub const None: Self = Self(0u32);
    pub const Allowed: Self = Self(1u32);
}
impl ::core::marker::Copy for StorageProviderHardlinkPolicy {}
impl ::core::clone::Clone for StorageProviderHardlinkPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderHydrationPolicy(pub i32);
impl StorageProviderHydrationPolicy {
    pub const Partial: Self = Self(0i32);
    pub const Progressive: Self = Self(1i32);
    pub const Full: Self = Self(2i32);
    pub const AlwaysFull: Self = Self(3i32);
}
impl ::core::marker::Copy for StorageProviderHydrationPolicy {}
impl ::core::clone::Clone for StorageProviderHydrationPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderHydrationPolicyModifier(pub u32);
impl StorageProviderHydrationPolicyModifier {
    pub const None: Self = Self(0u32);
    pub const ValidationRequired: Self = Self(1u32);
    pub const StreamingAllowed: Self = Self(2u32);
    pub const AutoDehydrationAllowed: Self = Self(4u32);
    pub const AllowFullRestartHydration: Self = Self(8u32);
}
impl ::core::marker::Copy for StorageProviderHydrationPolicyModifier {}
impl ::core::clone::Clone for StorageProviderHydrationPolicyModifier {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderInSyncPolicy(pub u32);
impl StorageProviderInSyncPolicy {
    pub const Default: Self = Self(0u32);
    pub const FileCreationTime: Self = Self(1u32);
    pub const FileReadOnlyAttribute: Self = Self(2u32);
    pub const FileHiddenAttribute: Self = Self(4u32);
    pub const FileSystemAttribute: Self = Self(8u32);
    pub const DirectoryCreationTime: Self = Self(16u32);
    pub const DirectoryReadOnlyAttribute: Self = Self(32u32);
    pub const DirectoryHiddenAttribute: Self = Self(64u32);
    pub const DirectorySystemAttribute: Self = Self(128u32);
    pub const FileLastWriteTime: Self = Self(256u32);
    pub const DirectoryLastWriteTime: Self = Self(512u32);
    pub const PreserveInsyncForSyncEngine: Self = Self(2147483648u32);
}
impl ::core::marker::Copy for StorageProviderInSyncPolicy {}
impl ::core::clone::Clone for StorageProviderInSyncPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
pub type StorageProviderItemProperty = *mut ::core::ffi::c_void;
pub type StorageProviderItemPropertyDefinition = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderPopulationPolicy(pub i32);
impl StorageProviderPopulationPolicy {
    pub const Full: Self = Self(1i32);
    pub const AlwaysFull: Self = Self(2i32);
}
impl ::core::marker::Copy for StorageProviderPopulationPolicy {}
impl ::core::clone::Clone for StorageProviderPopulationPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderProtectionMode(pub i32);
impl StorageProviderProtectionMode {
    pub const Unknown: Self = Self(0i32);
    pub const Personal: Self = Self(1i32);
}
impl ::core::marker::Copy for StorageProviderProtectionMode {}
impl ::core::clone::Clone for StorageProviderProtectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderState(pub i32);
impl StorageProviderState {
    pub const InSync: Self = Self(0i32);
    pub const Syncing: Self = Self(1i32);
    pub const Paused: Self = Self(2i32);
    pub const Error: Self = Self(3i32);
    pub const Warning: Self = Self(4i32);
    pub const Offline: Self = Self(5i32);
}
impl ::core::marker::Copy for StorageProviderState {}
impl ::core::clone::Clone for StorageProviderState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type StorageProviderStatus = *mut ::core::ffi::c_void;
pub type StorageProviderSyncRootInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderUriSourceStatus(pub i32);
impl StorageProviderUriSourceStatus {
    pub const Success: Self = Self(0i32);
    pub const NoSyncRoot: Self = Self(1i32);
    pub const FileNotFound: Self = Self(2i32);
}
impl ::core::marker::Copy for StorageProviderUriSourceStatus {}
impl ::core::clone::Clone for StorageProviderUriSourceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct UIStatus(pub i32);
impl UIStatus {
    pub const Unavailable: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
    pub const Visible: Self = Self(2i32);
    pub const Complete: Self = Self(3i32);
}
impl ::core::marker::Copy for UIStatus {}
impl ::core::clone::Clone for UIStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct WriteActivationMode(pub i32);
impl WriteActivationMode {
    pub const ReadOnly: Self = Self(0i32);
    pub const NotNeeded: Self = Self(1i32);
    pub const AfterWrite: Self = Self(2i32);
}
impl ::core::marker::Copy for WriteActivationMode {}
impl ::core::clone::Clone for WriteActivationMode {
    fn clone(&self) -> Self {
        *self
    }
}
