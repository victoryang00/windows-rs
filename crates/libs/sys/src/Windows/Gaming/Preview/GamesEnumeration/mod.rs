#[doc = "*Required features: `\"Gaming_Preview_GamesEnumeration\"`*"]
#[repr(transparent)]
pub struct GameListCategory(pub i32);
impl GameListCategory {
    pub const Candidate: Self = Self(0i32);
    pub const ConfirmedBySystem: Self = Self(1i32);
    pub const ConfirmedByUser: Self = Self(2i32);
}
impl ::core::marker::Copy for GameListCategory {}
impl ::core::clone::Clone for GameListCategory {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GameListChangedEventHandler = *mut ::core::ffi::c_void;
pub type GameListEntry = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Gaming_Preview_GamesEnumeration\"`*"]
#[repr(transparent)]
pub struct GameListEntryLaunchableState(pub i32);
impl GameListEntryLaunchableState {
    pub const NotLaunchable: Self = Self(0i32);
    pub const ByLastRunningFullPath: Self = Self(1i32);
    pub const ByUserProvidedPath: Self = Self(2i32);
    pub const ByTile: Self = Self(3i32);
}
impl ::core::marker::Copy for GameListEntryLaunchableState {}
impl ::core::clone::Clone for GameListEntryLaunchableState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GameListRemovedEventHandler = *mut ::core::ffi::c_void;
pub type GameModeConfiguration = *mut ::core::ffi::c_void;
pub type GameModeUserConfiguration = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IGameListEntry {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel")]
    pub DisplayInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    DisplayInfo: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchAsync: usize,
    pub Category: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GameListCategory) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(feature = "Foundation")]
    pub SetCategoryAsync: unsafe extern "system" fn(this: *mut *mut Self, value: GameListCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCategoryAsync: usize,
}
impl ::windows_sys::core::Interface for IGameListEntry {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1935221971, data2: 33055, data3: 17556, data4: [182, 156, 198, 65, 160, 198, 21, 67] };
}
#[repr(C)]
pub struct IGameListEntry2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LaunchableState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GameListEntryLaunchableState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage")]
    pub LauncherExecutable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    LauncherExecutable: usize,
    pub LaunchParameters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub SetLauncherExecutableFileAsync: unsafe extern "system" fn(this: *mut *mut Self, executablefile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    SetLauncherExecutableFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub SetLauncherExecutableFileWithParamsAsync: unsafe extern "system" fn(this: *mut *mut Self, executablefile: *mut ::core::ffi::c_void, launchparams: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    SetLauncherExecutableFileWithParamsAsync: usize,
    pub TitleId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetTitleIdAsync: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTitleIdAsync: usize,
    pub GameModeConfiguration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGameListEntry2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3628765067, data2: 34633, data3: 18981, data4: [144, 211, 246, 197, 164, 39, 136, 109] };
}
#[repr(C)]
pub struct IGameListStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsyncPackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, packagefamilyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsyncPackageFamilyName: usize,
    #[cfg(feature = "Foundation")]
    pub GameAdded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GameAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGameAdded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGameAdded: usize,
    #[cfg(feature = "Foundation")]
    pub GameRemoved: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GameRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGameRemoved: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGameRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub GameUpdated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GameUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGameUpdated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGameUpdated: usize,
}
impl ::windows_sys::core::Interface for IGameListStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 769462127, data2: 40038, data3: 19205, data4: [148, 92, 214, 237, 120, 73, 27, 140] };
}
#[repr(C)]
pub struct IGameListStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub MergeEntriesAsync: unsafe extern "system" fn(this: *mut *mut Self, left: *mut ::core::ffi::c_void, right: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MergeEntriesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UnmergeEntryAsync: unsafe extern "system" fn(this: *mut *mut Self, mergedentry: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UnmergeEntryAsync: usize,
}
impl ::windows_sys::core::Interface for IGameListStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 962535576, data2: 59930, data3: 17834, data4: [146, 104, 168, 57, 5, 104, 111, 39] };
}
#[repr(C)]
pub struct IGameModeConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RelatedProcessNames: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RelatedProcessNames: usize,
    #[cfg(feature = "Foundation")]
    pub PercentGpuTimeAllocatedToGame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PercentGpuTimeAllocatedToGame: usize,
    #[cfg(feature = "Foundation")]
    pub SetPercentGpuTimeAllocatedToGame: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPercentGpuTimeAllocatedToGame: usize,
    #[cfg(feature = "Foundation")]
    pub PercentGpuMemoryAllocatedToGame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PercentGpuMemoryAllocatedToGame: usize,
    #[cfg(feature = "Foundation")]
    pub SetPercentGpuMemoryAllocatedToGame: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPercentGpuMemoryAllocatedToGame: usize,
    #[cfg(feature = "Foundation")]
    pub PercentGpuMemoryAllocatedToSystemCompositor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PercentGpuMemoryAllocatedToSystemCompositor: usize,
    #[cfg(feature = "Foundation")]
    pub SetPercentGpuMemoryAllocatedToSystemCompositor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPercentGpuMemoryAllocatedToSystemCompositor: usize,
    #[cfg(feature = "Foundation")]
    pub MaxCpuCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxCpuCount: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxCpuCount: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxCpuCount: usize,
    #[cfg(feature = "Foundation")]
    pub CpuExclusivityMaskLow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CpuExclusivityMaskLow: usize,
    #[cfg(feature = "Foundation")]
    pub SetCpuExclusivityMaskLow: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCpuExclusivityMaskLow: usize,
    #[cfg(feature = "Foundation")]
    pub CpuExclusivityMaskHigh: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CpuExclusivityMaskHigh: usize,
    #[cfg(feature = "Foundation")]
    pub SetCpuExclusivityMaskHigh: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCpuExclusivityMaskHigh: usize,
    pub AffinitizeToExclusiveCpus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAffinitizeToExclusiveCpus: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
}
impl ::windows_sys::core::Interface for IGameModeConfiguration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2028310959, data2: 45378, data3: 20208, data4: [136, 48, 85, 188, 43, 228, 245, 234] };
}
#[repr(C)]
pub struct IGameModeUserConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GamingRelatedProcessNames: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GamingRelatedProcessNames: usize,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
}
impl ::windows_sys::core::Interface for IGameModeUserConfiguration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1926449908, data2: 30059, data3: 18191, data4: [160, 194, 186, 98, 169, 7, 149, 219] };
}
#[repr(C)]
pub struct IGameModeUserConfigurationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGameModeUserConfigurationStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1850792316, data2: 26346, data3: 18318, data4: [164, 161, 245, 124, 14, 141, 0, 231] };
}
