#[cfg(feature = "ApplicationModel_Store_Preview_InstallControl")]
pub mod InstallControl;
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
#[repr(transparent)]
pub struct DeliveryOptimizationDownloadMode(pub i32);
impl DeliveryOptimizationDownloadMode {
    pub const Simple: Self = Self(0i32);
    pub const HttpOnly: Self = Self(1i32);
    pub const Lan: Self = Self(2i32);
    pub const Group: Self = Self(3i32);
    pub const Internet: Self = Self(4i32);
    pub const Bypass: Self = Self(5i32);
}
impl ::core::marker::Copy for DeliveryOptimizationDownloadMode {}
impl ::core::clone::Clone for DeliveryOptimizationDownloadMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
#[repr(transparent)]
pub struct DeliveryOptimizationDownloadModeSource(pub i32);
impl DeliveryOptimizationDownloadModeSource {
    pub const Default: Self = Self(0i32);
    pub const Policy: Self = Self(1i32);
}
impl ::core::marker::Copy for DeliveryOptimizationDownloadModeSource {}
impl ::core::clone::Clone for DeliveryOptimizationDownloadModeSource {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DeliveryOptimizationSettings = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IDeliveryOptimizationSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub DownloadMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DeliveryOptimizationDownloadMode) -> ::windows_sys::core::HRESULT,
    pub DownloadModeSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DeliveryOptimizationDownloadModeSource) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeliveryOptimizationSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 403766688, data2: 59475, data3: 22110, data4: [184, 116, 122, 138, 123, 154, 14, 15] };
}
#[repr(C)]
pub struct IDeliveryOptimizationSettingsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetCurrentSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeliveryOptimizationSettingsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1551989935, data2: 44757, data3: 22937, data4: [180, 201, 140, 96, 137, 139, 196, 243] };
}
#[repr(C)]
pub struct IStoreConfigurationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SetSystemConfiguration: unsafe extern "system" fn(this: *mut *mut Self, cataloghardwaremanufacturerid: ::windows_sys::core::HSTRING, catalogstorecontentmodifierid: ::windows_sys::core::HSTRING, systemconfigurationexpiration: super::super::super::Foundation::DateTime, cataloghardwaredescriptor: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSystemConfiguration: usize,
    pub SetMobileOperatorConfiguration: unsafe extern "system" fn(this: *mut *mut Self, mobileoperatorid: ::windows_sys::core::HSTRING, appdownloadlimitinmegabytes: u32, updatedownloadlimitinmegabytes: u32) -> ::windows_sys::core::HRESULT,
    pub SetStoreWebAccountId: unsafe extern "system" fn(this: *mut *mut Self, webaccountid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsStoreWebAccountId: unsafe extern "system" fn(this: *mut *mut Self, webaccountid: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub HardwareManufacturerInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FilterUnsupportedSystemFeaturesAsync: unsafe extern "system" fn(this: *mut *mut Self, systemfeatures: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FilterUnsupportedSystemFeaturesAsync: usize,
}
impl ::windows_sys::core::Interface for IStoreConfigurationStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1922006976, data2: 34344, data3: 17132, data4: [132, 162, 7, 120, 14, 180, 77, 139] };
}
#[repr(C)]
pub struct IStoreConfigurationStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PurchasePromptingPolicy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PurchasePromptingPolicy: usize,
    #[cfg(feature = "Foundation")]
    pub SetPurchasePromptingPolicy: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPurchasePromptingPolicy: usize,
}
impl ::windows_sys::core::Interface for IStoreConfigurationStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1702643093, data2: 51383, data3: 20457, data4: [159, 76, 77, 113, 2, 125, 52, 126] };
}
#[repr(C)]
pub struct IStoreConfigurationStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub HasStoreWebAccount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub HasStoreWebAccountForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    HasStoreWebAccountForUser: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetStoreLogDataAsync: unsafe extern "system" fn(this: *mut *mut Self, options: StoreLogOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetStoreLogDataAsync: usize,
    #[cfg(feature = "System")]
    pub SetStoreWebAccountIdForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, webaccountid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetStoreWebAccountIdForUser: usize,
    #[cfg(feature = "System")]
    pub IsStoreWebAccountIdForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, webaccountid: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    IsStoreWebAccountIdForUser: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetPurchasePromptingPolicyForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetPurchasePromptingPolicyForUser: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub SetPurchasePromptingPolicyForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    SetPurchasePromptingPolicyForUser: usize,
}
impl ::windows_sys::core::Interface for IStoreConfigurationStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1833301372, data2: 61764, data3: 19637, data4: [157, 63, 78, 176, 94, 48, 182, 211] };
}
#[repr(C)]
pub struct IStoreConfigurationStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetStoreWebAccountId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetStoreWebAccountIdForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetStoreWebAccountIdForUser: usize,
    pub SetEnterpriseStoreWebAccountId: unsafe extern "system" fn(this: *mut *mut Self, webaccountid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub SetEnterpriseStoreWebAccountIdForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, webaccountid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetEnterpriseStoreWebAccountIdForUser: usize,
    pub GetEnterpriseStoreWebAccountId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetEnterpriseStoreWebAccountIdForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetEnterpriseStoreWebAccountIdForUser: usize,
    pub ShouldRestrictToEnterpriseStoreOnly: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub ShouldRestrictToEnterpriseStoreOnlyForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ShouldRestrictToEnterpriseStoreOnlyForUser: usize,
}
impl ::windows_sys::core::Interface for IStoreConfigurationStatics4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 553604818, data2: 20195, data3: 19696, data4: [155, 18, 85, 44, 3, 49, 15, 117] };
}
#[repr(C)]
pub struct IStoreConfigurationStatics5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsPinToDesktopSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsPinToTaskbarSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsPinToStartSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub PinToDesktop: unsafe extern "system" fn(this: *mut *mut Self, apppackagefamilyname: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub PinToDesktopForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, apppackagefamilyname: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    PinToDesktopForUser: usize,
}
impl ::windows_sys::core::Interface for IStoreConfigurationStatics5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4150342033, data2: 36777, data3: 18907, data4: [130, 43, 1, 96, 231, 228, 229, 197] };
}
#[repr(C)]
pub struct IStoreHardwareManufacturerInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub HardwareManufacturerId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub StoreContentModifierId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ModelName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ManufacturerName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStoreHardwareManufacturerInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4069710856, data2: 50772, data3: 17324, data4: [162, 31, 52, 128, 28, 157, 51, 136] };
}
#[repr(C)]
pub struct IStorePreview {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestProductPurchaseByProductIdAndSkuIdAsync: unsafe extern "system" fn(this: *mut *mut Self, productid: ::windows_sys::core::HSTRING, skuid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestProductPurchaseByProductIdAndSkuIdAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LoadAddOnProductInfosAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LoadAddOnProductInfosAsync: usize,
}
impl ::windows_sys::core::Interface for IStorePreview {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2316661313, data2: 33806, data3: 18857, data4: [188, 1, 93, 91, 1, 251, 200, 233] };
}
#[repr(C)]
pub struct IStorePreviewProductInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProductId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ProductType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SkuInfoList: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SkuInfoList: usize,
}
impl ::windows_sys::core::Interface for IStorePreviewProductInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 423091123, data2: 27649, data3: 19613, data4: [133, 205, 91, 171, 170, 194, 179, 81] };
}
#[repr(C)]
pub struct IStorePreviewPurchaseResults {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProductPurchaseStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StorePreviewProductPurchaseStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorePreviewPurchaseResults {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2967121617, data2: 54981, data3: 20051, data4: [160, 67, 251, 160, 216, 230, 18, 49] };
}
#[repr(C)]
pub struct IStorePreviewSkuInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProductId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SkuId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SkuType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CustomDeveloperData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CurrencyCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FormattedListPrice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ExtendedData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorePreviewSkuInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2180871906, data2: 2854, data3: 18649, data4: [152, 206, 39, 70, 28, 102, 157, 108] };
}
#[repr(C)]
pub struct IWebAuthenticationCoreManagerHelper {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "UI_Xaml"))]
    pub RequestTokenWithUIElementHostingAsync: unsafe extern "system" fn(this: *mut *mut Self, request: *mut ::core::ffi::c_void, uielement: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "UI_Xaml")))]
    RequestTokenWithUIElementHostingAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "Security_Credentials", feature = "UI_Xaml"))]
    pub RequestTokenWithUIElementHostingAndWebAccountAsync: unsafe extern "system" fn(this: *mut *mut Self, request: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, uielement: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "Security_Credentials", feature = "UI_Xaml")))]
    RequestTokenWithUIElementHostingAndWebAccountAsync: usize,
}
impl ::windows_sys::core::Interface for IWebAuthenticationCoreManagerHelper {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 111478053, data2: 59157, data3: 16675, data4: [146, 118, 157, 111, 134, 91, 165, 95] };
}
pub type StoreHardwareManufacturerInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
#[repr(transparent)]
pub struct StoreLogOptions(pub u32);
impl StoreLogOptions {
    pub const None: Self = Self(0u32);
    pub const TryElevate: Self = Self(1u32);
}
impl ::core::marker::Copy for StoreLogOptions {}
impl ::core::clone::Clone for StoreLogOptions {
    fn clone(&self) -> Self {
        *self
    }
}
pub type StorePreviewProductInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
#[repr(transparent)]
pub struct StorePreviewProductPurchaseStatus(pub i32);
impl StorePreviewProductPurchaseStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const AlreadyPurchased: Self = Self(1i32);
    pub const NotFulfilled: Self = Self(2i32);
    pub const NotPurchased: Self = Self(3i32);
}
impl ::core::marker::Copy for StorePreviewProductPurchaseStatus {}
impl ::core::clone::Clone for StorePreviewProductPurchaseStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type StorePreviewPurchaseResults = *mut ::core::ffi::c_void;
pub type StorePreviewSkuInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
#[repr(transparent)]
pub struct StoreSystemFeature(pub i32);
impl StoreSystemFeature {
    pub const ArchitectureX86: Self = Self(0i32);
    pub const ArchitectureX64: Self = Self(1i32);
    pub const ArchitectureArm: Self = Self(2i32);
    pub const DirectX9: Self = Self(3i32);
    pub const DirectX10: Self = Self(4i32);
    pub const DirectX11: Self = Self(5i32);
    pub const D3D12HardwareFL11: Self = Self(6i32);
    pub const D3D12HardwareFL12: Self = Self(7i32);
    pub const Memory300MB: Self = Self(8i32);
    pub const Memory750MB: Self = Self(9i32);
    pub const Memory1GB: Self = Self(10i32);
    pub const Memory2GB: Self = Self(11i32);
    pub const CameraFront: Self = Self(12i32);
    pub const CameraRear: Self = Self(13i32);
    pub const Gyroscope: Self = Self(14i32);
    pub const Hover: Self = Self(15i32);
    pub const Magnetometer: Self = Self(16i32);
    pub const Nfc: Self = Self(17i32);
    pub const Resolution720P: Self = Self(18i32);
    pub const ResolutionWvga: Self = Self(19i32);
    pub const ResolutionWvgaOr720P: Self = Self(20i32);
    pub const ResolutionWxga: Self = Self(21i32);
    pub const ResolutionWvgaOrWxga: Self = Self(22i32);
    pub const ResolutionWxgaOr720P: Self = Self(23i32);
    pub const Memory4GB: Self = Self(24i32);
    pub const Memory6GB: Self = Self(25i32);
    pub const Memory8GB: Self = Self(26i32);
    pub const Memory12GB: Self = Self(27i32);
    pub const Memory16GB: Self = Self(28i32);
    pub const Memory20GB: Self = Self(29i32);
    pub const VideoMemory2GB: Self = Self(30i32);
    pub const VideoMemory4GB: Self = Self(31i32);
    pub const VideoMemory6GB: Self = Self(32i32);
    pub const VideoMemory1GB: Self = Self(33i32);
    pub const ArchitectureArm64: Self = Self(34i32);
}
impl ::core::marker::Copy for StoreSystemFeature {}
impl ::core::clone::Clone for StoreSystemFeature {
    fn clone(&self) -> Self {
        *self
    }
}
