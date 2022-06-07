pub type AppExtension = *mut ::core::ffi::c_void;
pub type AppExtensionCatalog = *mut ::core::ffi::c_void;
pub type AppExtensionPackageInstalledEventArgs = *mut ::core::ffi::c_void;
pub type AppExtensionPackageStatusChangedEventArgs = *mut ::core::ffi::c_void;
pub type AppExtensionPackageUninstallingEventArgs = *mut ::core::ffi::c_void;
pub type AppExtensionPackageUpdatedEventArgs = *mut ::core::ffi::c_void;
pub type AppExtensionPackageUpdatingEventArgs = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAppExtension {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AppInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetExtensionPropertiesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetExtensionPropertiesAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub GetPublicFolderAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    GetPublicFolderAsync: usize,
}
impl ::windows_sys::core::Interface for IAppExtension {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2219872300, data2: 5613, data3: 20399, data4: [147, 234, 34, 55, 187, 248, 203, 214] };
}
#[repr(C)]
pub struct IAppExtension2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppUserModelId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppExtension2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2872776176, data2: 5369, data3: 19359, data4: [148, 25, 163, 73, 162, 66, 239, 56] };
}
#[repr(C)]
pub struct IAppExtensionCatalog {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestRemovePackageAsync: unsafe extern "system" fn(this: *mut *mut Self, packagefullname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestRemovePackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PackageInstalled: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageInstalled: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageInstalled: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageInstalled: usize,
    #[cfg(feature = "Foundation")]
    pub PackageUpdating: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageUpdating: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageUpdating: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageUpdating: usize,
    #[cfg(feature = "Foundation")]
    pub PackageUpdated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageUpdated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub PackageUninstalling: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageUninstalling: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageUninstalling: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageUninstalling: usize,
    #[cfg(feature = "Foundation")]
    pub PackageStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageStatusChanged: usize,
}
impl ::windows_sys::core::Interface for IAppExtensionCatalog {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2542215218, data2: 33830, data3: 19153, data4: [144, 132, 146, 232, 140, 45, 162, 0] };
}
#[repr(C)]
pub struct IAppExtensionCatalogStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Open: unsafe extern "system" fn(this: *mut *mut Self, appextensionname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppExtensionCatalogStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1010198154, data2: 24344, data3: 20235, data4: [156, 229, 202, 182, 29, 25, 111, 17] };
}
#[repr(C)]
pub struct IAppExtensionPackageInstalledEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppExtensionName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Extensions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Extensions: usize,
}
impl ::windows_sys::core::Interface for IAppExtensionPackageInstalledEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 971346484, data2: 13137, data3: 19085, data4: [151, 69, 231, 211, 221, 69, 188, 72] };
}
#[repr(C)]
pub struct IAppExtensionPackageStatusChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppExtensionName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppExtensionPackageStatusChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 484537395, data2: 4435, data3: 17661, data4: [135, 177, 138, 225, 5, 3, 3, 223] };
}
#[repr(C)]
pub struct IAppExtensionPackageUninstallingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppExtensionName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppExtensionPackageUninstallingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1626431685, data2: 5918, data3: 16639, data4: [174, 152, 171, 44, 32, 221, 77, 117] };
}
#[repr(C)]
pub struct IAppExtensionPackageUpdatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppExtensionName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Extensions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Extensions: usize,
}
impl ::windows_sys::core::Interface for IAppExtensionPackageUpdatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 981713983, data2: 31102, data3: 17589, data4: [186, 36, 164, 200, 181, 165, 67, 215] };
}
#[repr(C)]
pub struct IAppExtensionPackageUpdatingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppExtensionName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppExtensionPackageUpdatingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2127926057, data2: 6757, data3: 18432, data4: [167, 0, 179, 33, 0, 158, 48, 106] };
}
