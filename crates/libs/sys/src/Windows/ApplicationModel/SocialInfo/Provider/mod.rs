#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISocialDashboardItemUpdater {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub OwnerRemoteId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OwnerRemoteId: usize,
    #[cfg(feature = "deprecated")]
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Content: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Timestamp: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetTimestamp: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetTimestamp: usize,
    #[cfg(feature = "deprecated")]
    pub SetThumbnail: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetThumbnail: usize,
    #[cfg(feature = "deprecated")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Thumbnail: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CommitAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CommitAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub TargetUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    TargetUri: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetTargetUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetTargetUri: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISocialFeedUpdater {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub OwnerRemoteId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OwnerRemoteId: usize,
    #[cfg(feature = "deprecated")]
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::SocialFeedKind) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Kind: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Items: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Items: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CommitAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CommitAsync: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISocialInfoProviderManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CreateSocialFeedUpdaterAsync: unsafe extern "system" fn(this: *mut *mut Self, kind: super::SocialFeedKind, mode: super::SocialFeedUpdateMode, ownerremoteid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CreateSocialFeedUpdaterAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CreateDashboardItemUpdaterAsync: unsafe extern "system" fn(this: *mut *mut Self, ownerremoteid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CreateDashboardItemUpdaterAsync: usize,
    #[cfg(feature = "deprecated")]
    pub UpdateBadgeCountValue: unsafe extern "system" fn(this: *mut *mut Self, itemremoteid: ::windows_sys::core::HSTRING, newcount: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    UpdateBadgeCountValue: usize,
    #[cfg(feature = "deprecated")]
    pub ReportNewContentAvailable: unsafe extern "system" fn(this: *mut *mut Self, contactremoteid: ::windows_sys::core::HSTRING, kind: super::SocialFeedKind) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ReportNewContentAvailable: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ProvisionAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ProvisionAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub DeprovisionAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    DeprovisionAsync: usize,
}
pub type SocialDashboardItemUpdater = *mut ::core::ffi::c_void;
pub type SocialFeedUpdater = *mut ::core::ffi::c_void;
