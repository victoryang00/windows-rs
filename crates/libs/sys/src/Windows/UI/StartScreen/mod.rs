#[doc = "*Required features: `\"UI_StartScreen\"`*"]
#[repr(transparent)]
pub struct ForegroundText(pub i32);
impl ForegroundText {
    pub const Dark: Self = Self(0i32);
    pub const Light: Self = Self(1i32);
}
impl ::core::marker::Copy for ForegroundText {}
impl ::core::clone::Clone for ForegroundText {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IJumpList {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
    pub SystemGroupKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut JumpListSystemGroupKind) -> ::windows_sys::core::HRESULT,
    pub SetSystemGroupKind: unsafe extern "system" fn(this: *mut *mut Self, value: JumpListSystemGroupKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
}
#[repr(C)]
pub struct IJumpListItem {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut JumpListItemKind) -> ::windows_sys::core::HRESULT,
    pub Arguments: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RemovedByUser: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GroupName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetGroupName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Logo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Logo: usize,
    #[cfg(feature = "Foundation")]
    pub SetLogo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLogo: usize,
}
#[repr(C)]
pub struct IJumpListItemStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWithArguments: unsafe extern "system" fn(this: *mut *mut Self, arguments: ::windows_sys::core::HSTRING, displayname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSeparator: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IJumpListStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub LoadCurrentAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadCurrentAsync: usize,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISecondaryTile {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetTileId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TileId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetArguments: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Arguments: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub SetShortName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetShortName: usize,
    #[cfg(feature = "deprecated")]
    pub ShortName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ShortName: usize,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetLogo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetLogo: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Logo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Logo: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetSmallLogo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetSmallLogo: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SmallLogo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SmallLogo: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetWideLogo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetWideLogo: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub WideLogo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    WideLogo: usize,
    #[cfg(feature = "Foundation")]
    pub SetLockScreenBadgeLogo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLockScreenBadgeLogo: usize,
    #[cfg(feature = "Foundation")]
    pub LockScreenBadgeLogo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LockScreenBadgeLogo: usize,
    pub SetLockScreenDisplayBadgeAndTileText: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub LockScreenDisplayBadgeAndTileText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub SetTileOptions: unsafe extern "system" fn(this: *mut *mut Self, value: TileOptions) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetTileOptions: usize,
    #[cfg(feature = "deprecated")]
    pub TileOptions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TileOptions) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TileOptions: usize,
    #[cfg(feature = "deprecated")]
    pub SetForegroundText: unsafe extern "system" fn(this: *mut *mut Self, value: ForegroundText) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetForegroundText: usize,
    #[cfg(feature = "deprecated")]
    pub ForegroundText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ForegroundText) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ForegroundText: usize,
    #[cfg(feature = "deprecated")]
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetBackgroundColor: usize,
    #[cfg(feature = "deprecated")]
    pub BackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub RequestCreateAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestCreateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestCreateAsyncWithPoint: unsafe extern "system" fn(this: *mut *mut Self, invocationpoint: super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestCreateAsyncWithPoint: usize,
    #[cfg(feature = "Foundation")]
    pub RequestCreateAsyncWithRect: unsafe extern "system" fn(this: *mut *mut Self, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestCreateAsyncWithRect: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub RequestCreateAsyncWithRectAndPlacement: unsafe extern "system" fn(this: *mut *mut Self, selection: super::super::Foundation::Rect, preferredplacement: super::Popups::Placement, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    RequestCreateAsyncWithRectAndPlacement: usize,
    #[cfg(feature = "Foundation")]
    pub RequestDeleteAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestDeleteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestDeleteAsyncWithPoint: unsafe extern "system" fn(this: *mut *mut Self, invocationpoint: super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestDeleteAsyncWithPoint: usize,
    #[cfg(feature = "Foundation")]
    pub RequestDeleteAsyncWithRect: unsafe extern "system" fn(this: *mut *mut Self, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestDeleteAsyncWithRect: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub RequestDeleteAsyncWithRectAndPlacement: unsafe extern "system" fn(this: *mut *mut Self, selection: super::super::Foundation::Rect, preferredplacement: super::Popups::Placement, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    RequestDeleteAsyncWithRectAndPlacement: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateAsync: usize,
}
#[repr(C)]
pub struct ISecondaryTile2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetPhoneticName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PhoneticName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VisualElements: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRoamingEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub RoamingEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub VisualElementsRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VisualElementsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVisualElementsRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVisualElementsRequested: usize,
}
#[repr(C)]
pub struct ISecondaryTileFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CreateTile: unsafe extern "system" fn(this: *mut *mut Self, tileid: ::windows_sys::core::HSTRING, shortname: ::windows_sys::core::HSTRING, displayname: ::windows_sys::core::HSTRING, arguments: ::windows_sys::core::HSTRING, tileoptions: TileOptions, logoreference: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CreateTile: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CreateWideTile: unsafe extern "system" fn(this: *mut *mut Self, tileid: ::windows_sys::core::HSTRING, shortname: ::windows_sys::core::HSTRING, displayname: ::windows_sys::core::HSTRING, arguments: ::windows_sys::core::HSTRING, tileoptions: TileOptions, logoreference: *mut ::core::ffi::c_void, widelogoreference: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CreateWideTile: usize,
    pub CreateWithId: unsafe extern "system" fn(this: *mut *mut Self, tileid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISecondaryTileFactory2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateMinimalTile: unsafe extern "system" fn(this: *mut *mut Self, tileid: ::windows_sys::core::HSTRING, displayname: ::windows_sys::core::HSTRING, arguments: ::windows_sys::core::HSTRING, square150x150logo: *mut ::core::ffi::c_void, desiredsize: TileSize, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateMinimalTile: usize,
}
#[repr(C)]
pub struct ISecondaryTileStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Exists: unsafe extern "system" fn(this: *mut *mut Self, tileid: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllForApplicationAsync: unsafe extern "system" fn(this: *mut *mut Self, applicationid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllForApplicationAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllForPackageAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllForPackageAsync: usize,
}
#[repr(C)]
pub struct ISecondaryTileVisualElements {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetSquare30x30Logo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetSquare30x30Logo: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Square30x30Logo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Square30x30Logo: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetSquare70x70Logo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetSquare70x70Logo: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Square70x70Logo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Square70x70Logo: usize,
    #[cfg(feature = "Foundation")]
    pub SetSquare150x150Logo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSquare150x150Logo: usize,
    #[cfg(feature = "Foundation")]
    pub Square150x150Logo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Square150x150Logo: usize,
    #[cfg(feature = "Foundation")]
    pub SetWide310x150Logo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetWide310x150Logo: usize,
    #[cfg(feature = "Foundation")]
    pub Wide310x150Logo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Wide310x150Logo: usize,
    #[cfg(feature = "Foundation")]
    pub SetSquare310x310Logo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSquare310x310Logo: usize,
    #[cfg(feature = "Foundation")]
    pub Square310x310Logo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Square310x310Logo: usize,
    pub SetForegroundText: unsafe extern "system" fn(this: *mut *mut Self, value: ForegroundText) -> ::windows_sys::core::HRESULT,
    pub ForegroundText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ForegroundText) -> ::windows_sys::core::HRESULT,
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::Color) -> ::windows_sys::core::HRESULT,
    pub BackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Color) -> ::windows_sys::core::HRESULT,
    pub SetShowNameOnSquare150x150Logo: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ShowNameOnSquare150x150Logo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetShowNameOnWide310x150Logo: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ShowNameOnWide310x150Logo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetShowNameOnSquare310x310Logo: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ShowNameOnSquare310x310Logo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISecondaryTileVisualElements2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SetSquare71x71Logo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSquare71x71Logo: usize,
    #[cfg(feature = "Foundation")]
    pub Square71x71Logo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Square71x71Logo: usize,
}
#[repr(C)]
pub struct ISecondaryTileVisualElements3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SetSquare44x44Logo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSquare44x44Logo: usize,
    #[cfg(feature = "Foundation")]
    pub Square44x44Logo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Square44x44Logo: usize,
}
#[repr(C)]
pub struct ISecondaryTileVisualElements4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MixedRealityModel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStartScreenManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
    #[cfg(feature = "ApplicationModel_Core")]
    pub SupportsAppListEntry: unsafe extern "system" fn(this: *mut *mut Self, applistentry: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Core"))]
    SupportsAppListEntry: usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub ContainsAppListEntryAsync: unsafe extern "system" fn(this: *mut *mut Self, applistentry: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation")))]
    ContainsAppListEntryAsync: usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub RequestAddAppListEntryAsync: unsafe extern "system" fn(this: *mut *mut Self, applistentry: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation")))]
    RequestAddAppListEntryAsync: usize,
}
#[repr(C)]
pub struct IStartScreenManager2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ContainsSecondaryTileAsync: unsafe extern "system" fn(this: *mut *mut Self, tileid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContainsSecondaryTileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryRemoveSecondaryTileAsync: unsafe extern "system" fn(this: *mut *mut Self, tileid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryRemoveSecondaryTileAsync: usize,
}
#[repr(C)]
pub struct IStartScreenManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[repr(C)]
pub struct ITileMixedRealityModel {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub SetBoundingBox: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    SetBoundingBox: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub BoundingBox: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    BoundingBox: usize,
}
#[repr(C)]
pub struct ITileMixedRealityModel2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetActivationBehavior: unsafe extern "system" fn(this: *mut *mut Self, value: TileMixedRealityModelActivationBehavior) -> ::windows_sys::core::HRESULT,
    pub ActivationBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TileMixedRealityModelActivationBehavior) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVisualElementsRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub VisualElements: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AlternateVisualElements: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AlternateVisualElements: usize,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVisualElementsRequestDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVisualElementsRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
pub type JumpList = *mut ::core::ffi::c_void;
pub type JumpListItem = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_StartScreen\"`*"]
#[repr(transparent)]
pub struct JumpListItemKind(pub i32);
impl JumpListItemKind {
    pub const Arguments: Self = Self(0i32);
    pub const Separator: Self = Self(1i32);
}
impl ::core::marker::Copy for JumpListItemKind {}
impl ::core::clone::Clone for JumpListItemKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_StartScreen\"`*"]
#[repr(transparent)]
pub struct JumpListSystemGroupKind(pub i32);
impl JumpListSystemGroupKind {
    pub const None: Self = Self(0i32);
    pub const Frequent: Self = Self(1i32);
    pub const Recent: Self = Self(2i32);
}
impl ::core::marker::Copy for JumpListSystemGroupKind {}
impl ::core::clone::Clone for JumpListSystemGroupKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SecondaryTile = *mut ::core::ffi::c_void;
pub type SecondaryTileVisualElements = *mut ::core::ffi::c_void;
pub type StartScreenManager = *mut ::core::ffi::c_void;
pub type TileMixedRealityModel = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_StartScreen\"`*"]
#[repr(transparent)]
pub struct TileMixedRealityModelActivationBehavior(pub i32);
impl TileMixedRealityModelActivationBehavior {
    pub const Default: Self = Self(0i32);
    pub const None: Self = Self(1i32);
}
impl ::core::marker::Copy for TileMixedRealityModelActivationBehavior {}
impl ::core::clone::Clone for TileMixedRealityModelActivationBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_StartScreen\"`*"]
#[repr(transparent)]
pub struct TileOptions(pub u32);
impl TileOptions {
    pub const None: Self = Self(0u32);
    pub const ShowNameOnLogo: Self = Self(1u32);
    pub const ShowNameOnWideLogo: Self = Self(2u32);
    pub const CopyOnDeployment: Self = Self(4u32);
}
impl ::core::marker::Copy for TileOptions {}
impl ::core::clone::Clone for TileOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_StartScreen\"`*"]
#[repr(transparent)]
pub struct TileSize(pub i32);
impl TileSize {
    pub const Default: Self = Self(0i32);
    pub const Square30x30: Self = Self(1i32);
    pub const Square70x70: Self = Self(2i32);
    pub const Square150x150: Self = Self(3i32);
    pub const Wide310x150: Self = Self(4i32);
    pub const Square310x310: Self = Self(5i32);
    pub const Square71x71: Self = Self(6i32);
    pub const Square44x44: Self = Self(7i32);
}
impl ::core::marker::Copy for TileSize {}
impl ::core::clone::Clone for TileSize {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VisualElementsRequest = *mut ::core::ffi::c_void;
pub type VisualElementsRequestDeferral = *mut ::core::ffi::c_void;
pub type VisualElementsRequestedEventArgs = *mut ::core::ffi::c_void;
