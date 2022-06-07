#[cfg(feature = "System_Profile_SystemManufacturers")]
pub mod SystemManufacturers;
pub type AnalyticsVersionInfo = *mut ::core::ffi::c_void;
pub type HardwareToken = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAnalyticsInfoStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub VersionInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DeviceForm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAnalyticsInfoStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 492757094, data2: 6285, data3: 23465, data4: [67, 135, 172, 174, 176, 231, 227, 5] };
}
#[repr(C)]
pub struct IAnalyticsInfoStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSystemPropertiesAsync: unsafe extern "system" fn(this: *mut *mut Self, attributenames: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSystemPropertiesAsync: usize,
}
impl ::windows_sys::core::Interface for IAnalyticsInfoStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 269944042, data2: 43001, data3: 18130, data4: [171, 148, 1, 104, 101, 175, 219, 37] };
}
#[repr(C)]
pub struct IAnalyticsVersionInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceFamily: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DeviceFamilyVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAnalyticsVersionInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2455843000, data2: 39253, data3: 19572, data4: [189, 193, 124, 208, 222, 207, 155, 3] };
}
#[repr(C)]
pub struct IAnalyticsVersionInfo2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProductName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAnalyticsVersionInfo2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1994986929, data2: 65334, data3: 16508, data4: [159, 87, 22, 13, 62, 84, 7, 71] };
}
#[repr(C)]
pub struct IAppApplicabilityStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUnsupportedAppRequirements: unsafe extern "system" fn(this: *mut *mut Self, capabilities: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUnsupportedAppRequirements: usize,
}
impl ::windows_sys::core::Interface for IAppApplicabilityStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 375693442, data2: 3896, data3: 23705, data4: [131, 228, 72, 153, 89, 112, 134, 28] };
}
#[repr(C)]
pub struct IEducationSettingsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsEducationEnvironment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEducationSettingsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4233359599, data2: 19774, data3: 19987, data4: [155, 35, 80, 95, 77, 9, 30, 146] };
}
#[repr(C)]
pub struct IHardwareIdentificationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub GetPackageSpecificToken: unsafe extern "system" fn(this: *mut *mut Self, nonce: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetPackageSpecificToken: usize,
}
impl ::windows_sys::core::Interface for IHardwareIdentificationStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2534564064, data2: 61808, data3: 19010, data4: [189, 85, 169, 0, 178, 18, 218, 226] };
}
#[repr(C)]
pub struct IHardwareToken {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Id: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Signature: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Signature: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Certificate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Certificate: usize,
}
impl ::windows_sys::core::Interface for IHardwareToken {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 687264960, data2: 64274, data3: 16548, data4: [129, 103, 127, 78, 3, 210, 114, 76] };
}
#[repr(C)]
pub struct IKnownRetailInfoPropertiesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub RetailAccessCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ManufacturerName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ModelName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayModelName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Price: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsFeatured: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FormFactor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ScreenSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Weight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub BatteryLifeDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ProcessorDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Memory: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub StorageDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GraphicsDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FrontCameraDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RearCameraDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HasNfc: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HasSdSlot: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HasOpticalDrive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsOfficeInstalled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WindowsEdition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IKnownRetailInfoPropertiesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2572620152, data2: 20495, data3: 18558, data4: [142, 117, 41, 229, 81, 114, 135, 18] };
}
#[repr(C)]
pub struct IPlatformDiagnosticsAndUsageDataSettingsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CollectionLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PlatformDataCollectionLevel) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CollectionLevelChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CollectionLevelChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCollectionLevelChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCollectionLevelChanged: usize,
    pub CanCollectDiagnostics: unsafe extern "system" fn(this: *mut *mut Self, level: PlatformDataCollectionLevel, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlatformDiagnosticsAndUsageDataSettingsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3068283931, data2: 31516, data3: 19250, data4: [140, 98, 166, 101, 151, 206, 114, 58] };
}
#[repr(C)]
pub struct IRetailInfoStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsDemoModeEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
impl ::windows_sys::core::Interface for IRetailInfoStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 118671032, data2: 35730, data3: 20266, data4: [132, 153, 3, 31, 23, 152, 214, 239] };
}
#[repr(C)]
pub struct ISharedModeSettingsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISharedModeSettingsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2302538766, data2: 51926, data3: 19792, data4: [140, 73, 111, 207, 192, 62, 219, 41] };
}
#[repr(C)]
pub struct ISharedModeSettingsStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShouldAvoidLocalStorage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISharedModeSettingsStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1619626148, data2: 52465, data3: 20200, data4: [165, 226, 253, 106, 29, 12, 250, 200] };
}
#[repr(C)]
pub struct ISystemIdentificationInfo {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Id: usize,
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SystemIdentificationSource) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemIdentificationInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 207986301, data2: 50114, data3: 19763, data4: [162, 223, 33, 188, 65, 145, 110, 179] };
}
#[repr(C)]
pub struct ISystemIdentificationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetSystemIdForPublisher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSystemIdForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemIdentificationStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1434580010, data2: 54239, data3: 19859, data4: [163, 125, 196, 26, 97, 108, 109, 1] };
}
#[repr(C)]
pub struct ISystemSetupInfoStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub OutOfBoxExperienceState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SystemOutOfBoxExperienceState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub OutOfBoxExperienceStateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OutOfBoxExperienceStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOutOfBoxExperienceStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOutOfBoxExperienceStateChanged: usize,
}
impl ::windows_sys::core::Interface for ISystemSetupInfoStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 748036264, data2: 7560, data3: 24109, data4: [163, 36, 165, 67, 175, 66, 71, 238] };
}
#[repr(C)]
pub struct IUnsupportedAppRequirement {
    pub base__: ::windows_sys::core::IInspectable,
    pub Requirement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Reasons: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UnsupportedAppRequirementReasons) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUnsupportedAppRequirement {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1635927132, data2: 35147, data3: 23740, data4: [137, 118, 169, 142, 10, 155, 153, 141] };
}
#[repr(C)]
pub struct IWindowsIntegrityPolicyStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsEnabledForTrial: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanDisable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsDisableSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PolicyChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PolicyChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePolicyChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePolicyChanged: usize,
}
impl ::windows_sys::core::Interface for IWindowsIntegrityPolicyStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2099085787, data2: 36195, data3: 18313, data4: [158, 165, 221, 207, 101, 169, 79, 60] };
}
#[doc = "*Required features: `\"System_Profile\"`*"]
#[repr(transparent)]
pub struct PlatformDataCollectionLevel(pub i32);
impl PlatformDataCollectionLevel {
    pub const Security: Self = Self(0i32);
    pub const Basic: Self = Self(1i32);
    pub const Enhanced: Self = Self(2i32);
    pub const Full: Self = Self(3i32);
}
impl ::core::marker::Copy for PlatformDataCollectionLevel {}
impl ::core::clone::Clone for PlatformDataCollectionLevel {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SystemIdentificationInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System_Profile\"`*"]
#[repr(transparent)]
pub struct SystemIdentificationSource(pub i32);
impl SystemIdentificationSource {
    pub const None: Self = Self(0i32);
    pub const Tpm: Self = Self(1i32);
    pub const Uefi: Self = Self(2i32);
    pub const Registry: Self = Self(3i32);
}
impl ::core::marker::Copy for SystemIdentificationSource {}
impl ::core::clone::Clone for SystemIdentificationSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"System_Profile\"`*"]
#[repr(transparent)]
pub struct SystemOutOfBoxExperienceState(pub i32);
impl SystemOutOfBoxExperienceState {
    pub const NotStarted: Self = Self(0i32);
    pub const InProgress: Self = Self(1i32);
    pub const Completed: Self = Self(2i32);
}
impl ::core::marker::Copy for SystemOutOfBoxExperienceState {}
impl ::core::clone::Clone for SystemOutOfBoxExperienceState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type UnsupportedAppRequirement = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System_Profile\"`*"]
#[repr(transparent)]
pub struct UnsupportedAppRequirementReasons(pub u32);
impl UnsupportedAppRequirementReasons {
    pub const Unknown: Self = Self(0u32);
    pub const DeniedBySystem: Self = Self(1u32);
}
impl ::core::marker::Copy for UnsupportedAppRequirementReasons {}
impl ::core::clone::Clone for UnsupportedAppRequirementReasons {
    fn clone(&self) -> Self {
        *self
    }
}
