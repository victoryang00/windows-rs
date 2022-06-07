#[doc = "*Required features: `\"Storage_AccessCache\"`*"]
#[repr(transparent)]
pub struct AccessCacheOptions(pub u32);
impl AccessCacheOptions {
    pub const None: Self = Self(0u32);
    pub const DisallowUserInput: Self = Self(1u32);
    pub const FastLocationsOnly: Self = Self(2u32);
    pub const UseReadOnlyCachedCopy: Self = Self(4u32);
    pub const SuppressAccessTimeUpdate: Self = Self(8u32);
}
impl ::core::marker::Copy for AccessCacheOptions {}
impl ::core::clone::Clone for AccessCacheOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Storage_AccessCache\"`*"]
pub struct AccessListEntry {
    pub Token: ::windows_sys::core::HSTRING,
    pub Metadata: ::windows_sys::core::HSTRING,
}
impl ::core::marker::Copy for AccessListEntry {}
impl ::core::clone::Clone for AccessListEntry {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AccessListEntryView = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IItemRemovedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub RemovedEntry: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::core::mem::ManuallyDrop<AccessListEntry>) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IItemRemovedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1499954780, data2: 21950, data3: 19558, data4: [186, 102, 94, 174, 167, 157, 38, 49] };
}
#[repr(C)]
pub struct IStorageApplicationPermissionsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FutureAccessList: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MostRecentlyUsedList: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorageApplicationPermissionsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1133633450, data2: 53299, data3: 18681, data4: [128, 96, 62, 200, 71, 210, 227, 241] };
}
#[repr(C)]
pub struct IStorageApplicationPermissionsStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub GetFutureAccessListForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetFutureAccessListForUser: usize,
    #[cfg(feature = "System")]
    pub GetMostRecentlyUsedListForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetMostRecentlyUsedListForUser: usize,
}
impl ::windows_sys::core::Interface for IStorageApplicationPermissionsStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 120002284, data2: 43525, data3: 17044, data4: [154, 17, 26, 61, 4, 81, 154, 208] };
}
#[repr(C)]
pub struct IStorageItemAccessList {
    pub base__: ::windows_sys::core::IInspectable,
    pub AddOverloadDefaultMetadata: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, metadata: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AddOrReplaceOverloadDefaultMetadata: unsafe extern "system" fn(this: *mut *mut Self, token: ::windows_sys::core::HSTRING, file: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddOrReplace: unsafe extern "system" fn(this: *mut *mut Self, token: ::windows_sys::core::HSTRING, file: *mut ::core::ffi::c_void, metadata: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetItemAsync: unsafe extern "system" fn(this: *mut *mut Self, token: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetItemAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFileAsync: unsafe extern "system" fn(this: *mut *mut Self, token: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFolderAsync: unsafe extern "system" fn(this: *mut *mut Self, token: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetItemWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, token: ::windows_sys::core::HSTRING, options: AccessCacheOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetItemWithOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFileWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, token: ::windows_sys::core::HSTRING, options: AccessCacheOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFileWithOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFolderWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, token: ::windows_sys::core::HSTRING, options: AccessCacheOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFolderWithOptionsAsync: usize,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, token: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ContainsItem: unsafe extern "system" fn(this: *mut *mut Self, token: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub CheckAccess: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Entries: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Entries: usize,
    pub MaximumItemsAllowed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorageItemAccessList {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 749729453, data2: 56976, data3: 18421, data4: [178, 195, 221, 54, 201, 253, 212, 83] };
}
#[repr(C)]
pub struct IStorageItemMostRecentlyUsedList {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ItemRemoved: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveItemRemoved: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveItemRemoved: usize,
}
impl ::windows_sys::core::Interface for IStorageItemMostRecentlyUsedList {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 23214549, data2: 20749, data3: 16670, data4: [140, 241, 195, 209, 239, 250, 76, 51] };
}
#[repr(C)]
pub struct IStorageItemMostRecentlyUsedList2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AddWithMetadataAndVisibility: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, metadata: ::windows_sys::core::HSTRING, visibility: RecentStorageItemVisibility, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AddOrReplaceWithMetadataAndVisibility: unsafe extern "system" fn(this: *mut *mut Self, token: ::windows_sys::core::HSTRING, file: *mut ::core::ffi::c_void, metadata: ::windows_sys::core::HSTRING, visibility: RecentStorageItemVisibility) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorageItemMostRecentlyUsedList2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3662159520, data2: 60813, data3: 18225, data4: [161, 219, 228, 78, 226, 32, 64, 147] };
}
pub type ItemRemovedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Storage_AccessCache\"`*"]
#[repr(transparent)]
pub struct RecentStorageItemVisibility(pub i32);
impl RecentStorageItemVisibility {
    pub const AppOnly: Self = Self(0i32);
    pub const AppAndSystem: Self = Self(1i32);
}
impl ::core::marker::Copy for RecentStorageItemVisibility {}
impl ::core::clone::Clone for RecentStorageItemVisibility {
    fn clone(&self) -> Self {
        *self
    }
}
pub type StorageItemAccessList = *mut ::core::ffi::c_void;
pub type StorageItemMostRecentlyUsedList = *mut ::core::ffi::c_void;
