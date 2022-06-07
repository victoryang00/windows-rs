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
impl ::windows_sys::core::Interface for IJumpList {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2955103294, data2: 52591, data3: 19638, data4: [166, 17, 97, 253, 80, 95, 62, 209] };
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
impl ::windows_sys::core::Interface for IJumpListItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2061199127, data2: 35677, data3: 18464, data4: [153, 91, 155, 65, 141, 190, 72, 176] };
}
#[repr(C)]
pub struct IJumpListItemStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWithArguments: unsafe extern "system" fn(this: *mut *mut Self, arguments: ::windows_sys::core::HSTRING, displayname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSeparator: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IJumpListItemStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4055876840, data2: 51114, data3: 18891, data4: [141, 222, 236, 252, 205, 122, 215, 228] };
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
impl ::windows_sys::core::Interface for IJumpListStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2816525953, data2: 59006, data3: 19316, data4: [130, 80, 63, 50, 44, 77, 146, 195] };
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
impl ::windows_sys::core::Interface for ISecondaryTile {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2661175776, data2: 11189, data3: 19392, data4: [187, 141, 66, 178, 58, 188, 200, 141] };
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
impl ::windows_sys::core::Interface for ISecondaryTile2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3002518581, data2: 12880, data3: 18832, data4: [146, 60, 41, 74, 180, 182, 148, 221] };
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
impl ::windows_sys::core::Interface for ISecondaryTileFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1475685536, data2: 20924, data3: 19135, data4: [142, 191, 98, 122, 3, 152, 176, 90] };
}
#[repr(C)]
pub struct ISecondaryTileFactory2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateMinimalTile: unsafe extern "system" fn(this: *mut *mut Self, tileid: ::windows_sys::core::HSTRING, displayname: ::windows_sys::core::HSTRING, arguments: ::windows_sys::core::HSTRING, square150x150logo: *mut ::core::ffi::c_void, desiredsize: TileSize, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateMinimalTile: usize,
}
impl ::windows_sys::core::Interface for ISecondaryTileFactory2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 659262011, data2: 21037, data3: 17550, data4: [158, 178, 208, 103, 42, 179, 69, 200] };
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
impl ::windows_sys::core::Interface for ISecondaryTileStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2576387502, data2: 53329, data3: 18038, data4: [135, 254, 158, 194, 66, 216, 60, 116] };
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
impl ::windows_sys::core::Interface for ISecondaryTileVisualElements {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 495842099, data2: 33118, data3: 16703, data4: [159, 80, 168, 29, 167, 10, 150, 178] };
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
impl ::windows_sys::core::Interface for ISecondaryTileVisualElements2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4247663056, data2: 22492, data3: 18324, data4: [142, 207, 86, 130, 245, 243, 230, 239] };
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
impl ::windows_sys::core::Interface for ISecondaryTileVisualElements3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1454725846, data2: 53596, data3: 16628, data4: [129, 231, 87, 255, 216, 248, 164, 233] };
}
#[repr(C)]
pub struct ISecondaryTileVisualElements4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MixedRealityModel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISecondaryTileVisualElements4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1716936983, data2: 46404, data3: 16594, data4: [141, 18, 116, 212, 236, 36, 208, 76] };
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
impl ::windows_sys::core::Interface for IStartScreenManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1243466699, data2: 9961, data3: 20148, data4: [137, 51, 133, 158, 182, 236, 219, 41] };
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
impl ::windows_sys::core::Interface for IStartScreenManager2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 145168054, data2: 12651, data3: 19161, data4: [172, 184, 254, 156, 240, 11, 214, 8] };
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
impl ::windows_sys::core::Interface for IStartScreenManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2019946255, data2: 46469, data3: 17998, data4: [137, 147, 52, 232, 248, 115, 141, 72] };
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
impl ::windows_sys::core::Interface for ITileMixedRealityModel {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2960543323, data2: 34941, data3: 16962, data4: [154, 25, 61, 10, 78, 167, 128, 49] };
}
#[repr(C)]
pub struct ITileMixedRealityModel2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetActivationBehavior: unsafe extern "system" fn(this: *mut *mut Self, value: TileMixedRealityModelActivationBehavior) -> ::windows_sys::core::HRESULT,
    pub ActivationBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TileMixedRealityModelActivationBehavior) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITileMixedRealityModel2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1133801650, data2: 55237, data3: 16651, data4: [131, 25, 148, 134, 162, 123, 108, 103] };
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
impl ::windows_sys::core::Interface for IVisualElementsRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3241685818, data2: 37640, data3: 16498, data4: [136, 204, 208, 104, 219, 52, 124, 104] };
}
#[repr(C)]
pub struct IVisualElementsRequestDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVisualElementsRequestDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2707779248, data2: 294, data3: 17239, data4: [130, 4, 189, 130, 187, 42, 4, 109] };
}
#[repr(C)]
pub struct IVisualElementsRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVisualElementsRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2070923650, data2: 14861, data3: 20174, data4: [175, 150, 205, 23, 225, 176, 11, 45] };
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
