#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const AUTO_WIDTH: i32 = -1i32;
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct AppEvents {
    pub base__: super::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for AppEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4235870802, data2: 30892, data3: 17714, data4: [140, 90, 86, 60, 254, 19, 136, 99] };
}
pub const AppEventsDHTMLConnector: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2917549131, data2: 51487, data3: 20023, data4: [146, 164, 91, 180, 48, 163, 51, 64] };
pub const Application: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1236433178, data2: 45486, data3: 19600, data4: [155, 142, 232, 96, 186, 7, 248, 137] };
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub type CCM_COMMANDID_MASK_CONSTANTS = u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_COMMANDID_MASK_RESERVED: CCM_COMMANDID_MASK_CONSTANTS = 4294901760u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub type CCM_INSERTIONALLOWED = i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONALLOWED_TOP: CCM_INSERTIONALLOWED = 1i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONALLOWED_NEW: CCM_INSERTIONALLOWED = 2i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONALLOWED_TASK: CCM_INSERTIONALLOWED = 4i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONALLOWED_VIEW: CCM_INSERTIONALLOWED = 8i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub type CCM_INSERTIONPOINTID = i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONPOINTID_MASK_SPECIAL: CCM_INSERTIONPOINTID = -65536i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONPOINTID_MASK_SHARED: CCM_INSERTIONPOINTID = -2147483648i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONPOINTID_MASK_CREATE_PRIMARY: CCM_INSERTIONPOINTID = 1073741824i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONPOINTID_MASK_ADD_PRIMARY: CCM_INSERTIONPOINTID = 536870912i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONPOINTID_MASK_ADD_3RDPARTY: CCM_INSERTIONPOINTID = 268435456i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONPOINTID_MASK_RESERVED: CCM_INSERTIONPOINTID = 268369920i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONPOINTID_MASK_FLAGINDEX: CCM_INSERTIONPOINTID = 31i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONPOINTID_PRIMARY_TOP: CCM_INSERTIONPOINTID = -1610612736i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONPOINTID_PRIMARY_NEW: CCM_INSERTIONPOINTID = -1610612735i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONPOINTID_PRIMARY_TASK: CCM_INSERTIONPOINTID = -1610612734i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONPOINTID_PRIMARY_VIEW: CCM_INSERTIONPOINTID = -1610612733i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONPOINTID_PRIMARY_HELP: CCM_INSERTIONPOINTID = -1610612732i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONPOINTID_3RDPARTY_NEW: CCM_INSERTIONPOINTID = -1879048191i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONPOINTID_3RDPARTY_TASK: CCM_INSERTIONPOINTID = -1879048190i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONPOINTID_ROOT_MENU: CCM_INSERTIONPOINTID = -2147483648i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub type CCM_SPECIAL = i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_SPECIAL_SEPARATOR: CCM_SPECIAL = 1i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_SPECIAL_SUBMENU: CCM_SPECIAL = 2i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_SPECIAL_DEFAULT_ITEM: CCM_SPECIAL = 4i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_SPECIAL_INSERTION_POINT: CCM_SPECIAL = 8i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_SPECIAL_TESTONLY: CCM_SPECIAL = 16i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct CONTEXTMENUITEM {
    pub strName: ::windows_sys::core::PWSTR,
    pub strStatusBarText: ::windows_sys::core::PWSTR,
    pub lCommandID: i32,
    pub lInsertionPointID: i32,
    pub fFlags: i32,
    pub fSpecialFlags: i32,
}
impl ::core::marker::Copy for CONTEXTMENUITEM {}
impl ::core::clone::Clone for CONTEXTMENUITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct CONTEXTMENUITEM2 {
    pub strName: ::windows_sys::core::PWSTR,
    pub strStatusBarText: ::windows_sys::core::PWSTR,
    pub lCommandID: i32,
    pub lInsertionPointID: i32,
    pub fFlags: i32,
    pub fSpecialFlags: i32,
    pub strLanguageIndependentName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for CONTEXTMENUITEM2 {}
impl ::core::clone::Clone for CONTEXTMENUITEM2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct Column {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, width: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetWidth: unsafe extern "system" fn(this: *mut *mut Self, width: i32) -> ::windows_sys::core::HRESULT,
    pub DisplayPosition: unsafe extern "system" fn(this: *mut *mut Self, displayposition: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetDisplayPosition: unsafe extern "system" fn(this: *mut *mut Self, index: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Hidden: unsafe extern "system" fn(this: *mut *mut Self, hidden: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Hidden: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetHidden: unsafe extern "system" fn(this: *mut *mut Self, hidden: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetHidden: usize,
    pub SetAsSortColumn: unsafe extern "system" fn(this: *mut *mut Self, sortorder: _ColumnSortOrder) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSortColumn: unsafe extern "system" fn(this: *mut *mut Self, issortcolumn: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSortColumn: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for Column {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4246495075, data2: 11030, data3: 19718, data4: [154, 179, 244, 83, 80, 185, 64, 171] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct Columns {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, column: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for Columns {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 943541655, data2: 64580, data3: 18315, data4: [177, 57, 99, 35, 220, 72, 97, 28] };
}
pub const ConsolePower: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4029174644, data2: 57329, data3: 4563, data4: [180, 51, 0, 192, 79, 142, 205, 120] };
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ContextMenu {
    pub base__: super::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, indexorpath: ::core::mem::ManuallyDrop<super::Com::VARIANT>, menuitem: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ContextMenu {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3669204192, data2: 9702, data3: 19975, data4: [131, 98, 186, 156, 149, 112, 101, 69] };
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub type DATA_OBJECT_TYPES = i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCT_SCOPE: DATA_OBJECT_TYPES = 32768i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCT_RESULT: DATA_OBJECT_TYPES = 32769i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCT_SNAPIN_MANAGER: DATA_OBJECT_TYPES = 32770i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCT_UNINITIALIZED: DATA_OBJECT_TYPES = 65535i32;
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct Document {
    pub base__: super::Com::IDispatch,
    pub Save: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SaveAs: unsafe extern "system" fn(this: *mut *mut Self, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SaveAs: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Close: unsafe extern "system" fn(this: *mut *mut Self, savechanges: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Close: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Views: unsafe extern "system" fn(this: *mut *mut Self, views: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Views: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SnapIns: unsafe extern "system" fn(this: *mut *mut Self, snapins: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SnapIns: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ActiveView: unsafe extern "system" fn(this: *mut *mut Self, view: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActiveView: usize,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    pub Location: unsafe extern "system" fn(this: *mut *mut Self, location: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSaved: unsafe extern "system" fn(this: *mut *mut Self, issaved: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSaved: usize,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, mode: *mut _DocumentMode) -> ::windows_sys::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut *mut Self, mode: _DocumentMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RootNode: unsafe extern "system" fn(this: *mut *mut Self, node: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RootNode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ScopeNamespace: unsafe extern "system" fn(this: *mut *mut Self, scopenamespace: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ScopeNamespace: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateProperties: unsafe extern "system" fn(this: *mut *mut Self, properties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Application: unsafe extern "system" fn(this: *mut *mut Self, application: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Application: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for Document {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 575742166, data2: 7695, data3: 16547, data4: [147, 254, 16, 121, 230, 168, 1, 123] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct Extension {
    pub base__: super::Com::IDispatch,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    pub Vendor: unsafe extern "system" fn(this: *mut *mut Self, vendor: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut *mut Self, version: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Extensions: unsafe extern "system" fn(this: *mut *mut Self, extensions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Extensions: usize,
    pub SnapinCLSID: unsafe extern "system" fn(this: *mut *mut Self, snapinclsid: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableAllExtensions: unsafe extern "system" fn(this: *mut *mut Self, enable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableAllExtensions: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Enable: unsafe extern "system" fn(this: *mut *mut Self, enable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enable: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for Extension {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2907532454, data2: 37167, data3: 16539, data4: [162, 110, 127, 210, 52, 174, 245, 66] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct Extensions {
    pub base__: super::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, extension: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for Extensions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2195450435, data2: 36004, data3: 17596, data4: [162, 202, 209, 135, 65, 5, 158, 200] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct Frame {
    pub base__: super::Com::IDispatch,
    pub Maximize: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Minimize: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Restore: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Top: unsafe extern "system" fn(this: *mut *mut Self, top: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetTop: unsafe extern "system" fn(this: *mut *mut Self, top: i32) -> ::windows_sys::core::HRESULT,
    pub Bottom: unsafe extern "system" fn(this: *mut *mut Self, bottom: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetBottom: unsafe extern "system" fn(this: *mut *mut Self, bottom: i32) -> ::windows_sys::core::HRESULT,
    pub Left: unsafe extern "system" fn(this: *mut *mut Self, left: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetLeft: unsafe extern "system" fn(this: *mut *mut Self, left: i32) -> ::windows_sys::core::HRESULT,
    pub Right: unsafe extern "system" fn(this: *mut *mut Self, right: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetRight: unsafe extern "system" fn(this: *mut *mut Self, right: i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for Frame {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3856849264, data2: 23475, data3: 17158, data4: [136, 4, 176, 150, 138, 49, 200, 230] };
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const HDI_HIDDEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const HIDE_COLUMN: i32 = -4i32;
#[repr(C)]
pub struct IColumnData {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetColumnConfigData: unsafe extern "system" fn(this: *mut *mut Self, pcolid: *const SColumnSetID, pcolsetdata: *const MMC_COLUMN_SET_DATA) -> ::windows_sys::core::HRESULT,
    pub GetColumnConfigData: unsafe extern "system" fn(this: *mut *mut Self, pcolid: *const SColumnSetID, ppcolsetdata: *mut *mut MMC_COLUMN_SET_DATA) -> ::windows_sys::core::HRESULT,
    pub SetColumnSortData: unsafe extern "system" fn(this: *mut *mut Self, pcolid: *const SColumnSetID, pcolsortdata: *const MMC_SORT_SET_DATA) -> ::windows_sys::core::HRESULT,
    pub GetColumnSortData: unsafe extern "system" fn(this: *mut *mut Self, pcolid: *const SColumnSetID, ppcolsortdata: *mut *mut MMC_SORT_SET_DATA) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IColumnData {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1417417556, data2: 589, data3: 4563, data4: [167, 7, 0, 192, 79, 142, 244, 203] };
}
#[repr(C)]
pub struct IComponent {
    pub base__: ::windows_sys::core::IUnknown,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, lpconsole: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Notify: unsafe extern "system" fn(this: *mut *mut Self, lpdataobject: *mut ::core::ffi::c_void, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param3: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Notify: usize,
    pub Destroy: unsafe extern "system" fn(this: *mut *mut Self, cookie: isize) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryDataObject: unsafe extern "system" fn(this: *mut *mut Self, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryDataObject: usize,
    pub GetResultViewType: unsafe extern "system" fn(this: *mut *mut Self, cookie: isize, ppviewtype: *mut ::windows_sys::core::PWSTR, pviewoptions: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDisplayInfo: unsafe extern "system" fn(this: *mut *mut Self, presultdataitem: *mut RESULTDATAITEM) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDisplayInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CompareObjects: unsafe extern "system" fn(this: *mut *mut Self, lpdataobjecta: *mut ::core::ffi::c_void, lpdataobjectb: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CompareObjects: usize,
}
impl ::windows_sys::core::Interface for IComponent {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1125346994, data2: 54124, data3: 4559, data4: [173, 188, 0, 170, 0, 168, 0, 51] };
}
#[repr(C)]
pub struct IComponent2 {
    pub base__: IComponent,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryDispatch: unsafe extern "system" fn(this: *mut *mut Self, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdispatch: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryDispatch: usize,
    pub GetResultViewType2: unsafe extern "system" fn(this: *mut *mut Self, cookie: isize, presultviewtype: *mut RESULT_VIEW_TYPE_INFO) -> ::windows_sys::core::HRESULT,
    pub RestoreResultView: unsafe extern "system" fn(this: *mut *mut Self, cookie: isize, presultviewtype: *const RESULT_VIEW_TYPE_INFO) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IComponent2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2040714773, data2: 18960, data3: 20180, data4: [140, 101, 134, 51, 249, 51, 80, 149] };
}
#[repr(C)]
pub struct IComponentData {
    pub base__: ::windows_sys::core::IUnknown,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, punknown: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateComponent: unsafe extern "system" fn(this: *mut *mut Self, ppcomponent: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Notify: unsafe extern "system" fn(this: *mut *mut Self, lpdataobject: *mut ::core::ffi::c_void, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param3: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Notify: usize,
    pub Destroy: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryDataObject: unsafe extern "system" fn(this: *mut *mut Self, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryDataObject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDisplayInfo: unsafe extern "system" fn(this: *mut *mut Self, pscopedataitem: *mut SCOPEDATAITEM) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDisplayInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CompareObjects: unsafe extern "system" fn(this: *mut *mut Self, lpdataobjecta: *mut ::core::ffi::c_void, lpdataobjectb: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CompareObjects: usize,
}
impl ::windows_sys::core::Interface for IComponentData {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2505749130, data2: 21016, data3: 4560, data4: [169, 133, 0, 192, 79, 216, 213, 101] };
}
#[repr(C)]
pub struct IComponentData2 {
    pub base__: IComponentData,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryDispatch: unsafe extern "system" fn(this: *mut *mut Self, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdispatch: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryDispatch: usize,
}
impl ::windows_sys::core::Interface for IComponentData2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3433099986, data2: 33502, data3: 16821, data4: [191, 71, 59, 32, 118, 39, 61, 92] };
}
#[repr(C)]
pub struct IConsole {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetHeader: unsafe extern "system" fn(this: *mut *mut Self, pheader: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetToolbar: unsafe extern "system" fn(this: *mut *mut Self, ptoolbar: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub QueryResultView: unsafe extern "system" fn(this: *mut *mut Self, punknown: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub QueryScopeImageList: unsafe extern "system" fn(this: *mut *mut Self, ppimagelist: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub QueryResultImageList: unsafe extern "system" fn(this: *mut *mut Self, ppimagelist: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub UpdateAllViews: unsafe extern "system" fn(this: *mut *mut Self, lpdataobject: *mut ::core::ffi::c_void, data: super::super::Foundation::LPARAM, hint: isize) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    UpdateAllViews: usize,
    pub MessageBox: unsafe extern "system" fn(this: *mut *mut Self, lpsztext: ::windows_sys::core::PCWSTR, lpsztitle: ::windows_sys::core::PCWSTR, fustyle: u32, piretval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub QueryConsoleVerb: unsafe extern "system" fn(this: *mut *mut Self, ppconsoleverb: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectScopeItem: unsafe extern "system" fn(this: *mut *mut Self, hscopeitem: isize) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMainWindow: unsafe extern "system" fn(this: *mut *mut Self, phwnd: *mut super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMainWindow: usize,
    pub NewWindow: unsafe extern "system" fn(this: *mut *mut Self, hscopeitem: isize, loptions: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IConsole {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1125346993, data2: 54124, data3: 4559, data4: [173, 188, 0, 170, 0, 168, 0, 51] };
}
#[repr(C)]
pub struct IConsole2 {
    pub base__: IConsole,
    #[cfg(feature = "Win32_Foundation")]
    pub Expand: unsafe extern "system" fn(this: *mut *mut Self, hitem: isize, bexpand: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Expand: usize,
    pub IsTaskpadViewPreferred: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetStatusText: unsafe extern "system" fn(this: *mut *mut Self, pszstatustext: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IConsole2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 272466986, data2: 43619, data3: 4561, data4: [167, 225, 0, 192, 79, 216, 213, 101] };
}
#[repr(C)]
pub struct IConsole3 {
    pub base__: IConsole2,
    pub RenameScopeItem: unsafe extern "system" fn(this: *mut *mut Self, hscopeitem: isize) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IConsole3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1334177755, data2: 53473, data3: 18828, data4: [141, 74, 208, 16, 223, 221, 64, 79] };
}
#[repr(C)]
pub struct IConsoleNameSpace {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub InsertItem: unsafe extern "system" fn(this: *mut *mut Self, item: *mut SCOPEDATAITEM) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InsertItem: usize,
    pub DeleteItem: unsafe extern "system" fn(this: *mut *mut Self, hitem: isize, fdeletethis: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetItem: unsafe extern "system" fn(this: *mut *mut Self, item: *const SCOPEDATAITEM) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetItem: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetItem: unsafe extern "system" fn(this: *mut *mut Self, item: *mut SCOPEDATAITEM) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetItem: usize,
    pub GetChildItem: unsafe extern "system" fn(this: *mut *mut Self, item: isize, pitemchild: *mut isize, pcookie: *mut isize) -> ::windows_sys::core::HRESULT,
    pub GetNextItem: unsafe extern "system" fn(this: *mut *mut Self, item: isize, pitemnext: *mut isize, pcookie: *mut isize) -> ::windows_sys::core::HRESULT,
    pub GetParentItem: unsafe extern "system" fn(this: *mut *mut Self, item: isize, pitemparent: *mut isize, pcookie: *mut isize) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IConsoleNameSpace {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3202266656, data2: 62029, data3: 4559, data4: [138, 252, 0, 170, 0, 60, 169, 246] };
}
#[repr(C)]
pub struct IConsoleNameSpace2 {
    pub base__: IConsoleNameSpace,
    pub Expand: unsafe extern "system" fn(this: *mut *mut Self, hitem: isize) -> ::windows_sys::core::HRESULT,
    pub AddExtension: unsafe extern "system" fn(this: *mut *mut Self, hitem: isize, lpclsid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IConsoleNameSpace2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 626989260, data2: 26075, data3: 4561, data4: [167, 220, 0, 192, 79, 216, 213, 101] };
}
#[repr(C)]
pub struct IConsolePower {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetExecutionState: unsafe extern "system" fn(this: *mut *mut Self, dwadd: u32, dwremove: u32) -> ::windows_sys::core::HRESULT,
    pub ResetIdleTimer: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IConsolePower {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 486268174, data2: 25290, data3: 18894, data4: [163, 175, 219, 178, 222, 97, 176, 104] };
}
#[repr(C)]
pub struct IConsolePowerSink {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub OnPowerBroadcast: unsafe extern "system" fn(this: *mut *mut Self, nevent: u32, lparam: super::super::Foundation::LPARAM, plreturn: *mut super::super::Foundation::LRESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnPowerBroadcast: usize,
}
impl ::windows_sys::core::Interface for IConsolePowerSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 859010463, data2: 65103, data3: 18805, data4: [177, 67, 254, 192, 165, 221, 109, 101] };
}
#[repr(C)]
pub struct IConsoleVerb {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetVerbState: unsafe extern "system" fn(this: *mut *mut Self, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE, pstate: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetVerbState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetVerbState: unsafe extern "system" fn(this: *mut *mut Self, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetVerbState: usize,
    pub SetDefaultVerb: unsafe extern "system" fn(this: *mut *mut Self, ecmdid: MMC_CONSOLE_VERB) -> ::windows_sys::core::HRESULT,
    pub GetDefaultVerb: unsafe extern "system" fn(this: *mut *mut Self, pecmdid: *mut MMC_CONSOLE_VERB) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IConsoleVerb {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3835656800, data2: 29871, data3: 4560, data4: [162, 134, 0, 192, 79, 216, 254, 147] };
}
#[repr(C)]
pub struct IContextMenuCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddItem: unsafe extern "system" fn(this: *mut *mut Self, pitem: *const CONTEXTMENUITEM) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContextMenuCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1125346999, data2: 54124, data3: 4559, data4: [173, 188, 0, 170, 0, 168, 0, 51] };
}
#[repr(C)]
pub struct IContextMenuCallback2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddItem: unsafe extern "system" fn(this: *mut *mut Self, pitem: *const CONTEXTMENUITEM2) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContextMenuCallback2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3782786062, data2: 11984, data3: 19294, data4: [128, 151, 66, 201, 8, 126, 139, 51] };
}
#[repr(C)]
pub struct IContextMenuProvider {
    pub base__: IContextMenuCallback,
    pub EmptyMenuList: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPrimaryExtensionItems: unsafe extern "system" fn(this: *mut *mut Self, piextension: *mut ::core::ffi::c_void, pidataobject: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPrimaryExtensionItems: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddThirdPartyExtensionItems: unsafe extern "system" fn(this: *mut *mut Self, pidataobject: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddThirdPartyExtensionItems: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowContextMenu: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::Foundation::HWND, xpos: i32, ypos: i32, plselected: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowContextMenu: usize,
}
impl ::windows_sys::core::Interface for IContextMenuProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1125346998, data2: 54124, data3: 4559, data4: [173, 188, 0, 170, 0, 168, 0, 51] };
}
#[repr(C)]
pub struct IControlbar {
    pub base__: ::windows_sys::core::IUnknown,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, ntype: MMC_CONTROL_TYPE, pextendcontrolbar: *mut ::core::ffi::c_void, ppunknown: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Attach: unsafe extern "system" fn(this: *mut *mut Self, ntype: MMC_CONTROL_TYPE, lpunknown: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Detach: unsafe extern "system" fn(this: *mut *mut Self, lpunknown: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IControlbar {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1778090270, data2: 27676, data3: 4560, data4: [162, 203, 0, 192, 79, 217, 9, 221] };
}
#[repr(C)]
pub struct IDisplayHelp {
    pub base__: ::windows_sys::core::IUnknown,
    pub ShowTopic: unsafe extern "system" fn(this: *mut *mut Self, pszhelptopic: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDisplayHelp {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3428399152, data2: 47398, data3: 4561, data4: [128, 99, 0, 0, 248, 117, 169, 206] };
}
#[repr(C)]
pub struct IEnumTASK {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, rgelt: *mut MMC_TASK, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumTASK {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 864458929, data2: 23042, data3: 4561, data4: [159, 236, 0, 96, 8, 50, 219, 74] };
}
#[repr(C)]
pub struct IExtendContextMenu {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub AddMenuItems: unsafe extern "system" fn(this: *mut *mut Self, pidataobject: *mut ::core::ffi::c_void, picallback: *mut ::core::ffi::c_void, pinsertionallowed: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddMenuItems: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Command: unsafe extern "system" fn(this: *mut *mut Self, lcommandid: i32, pidataobject: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Command: usize,
}
impl ::windows_sys::core::Interface for IExtendContextMenu {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1329297999, data2: 53164, data3: 4559, data4: [184, 227, 0, 192, 79, 216, 213, 176] };
}
#[repr(C)]
pub struct IExtendControlbar {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetControlbar: unsafe extern "system" fn(this: *mut *mut Self, pcontrolbar: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ControlbarNotify: unsafe extern "system" fn(this: *mut *mut Self, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param2: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ControlbarNotify: usize,
}
impl ::windows_sys::core::Interface for IExtendControlbar {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1230005536, data2: 28480, data3: 4560, data4: [169, 139, 0, 192, 79, 216, 213, 101] };
}
#[repr(C)]
pub struct IExtendPropertySheet {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePropertyPages: unsafe extern "system" fn(this: *mut *mut Self, lpprovider: *mut ::core::ffi::c_void, handle: isize, lpidataobject: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePropertyPages: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryPagesFor: unsafe extern "system" fn(this: *mut *mut Self, lpdataobject: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryPagesFor: usize,
}
impl ::windows_sys::core::Interface for IExtendPropertySheet {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2245944540, data2: 61217, data3: 4559, data4: [162, 133, 0, 192, 79, 216, 219, 230] };
}
#[repr(C)]
pub struct IExtendPropertySheet2 {
    pub base__: IExtendPropertySheet,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub GetWatermarks: unsafe extern "system" fn(this: *mut *mut Self, lpidataobject: *mut ::core::ffi::c_void, lphwatermark: *mut super::super::Graphics::Gdi::HBITMAP, lphheader: *mut super::super::Graphics::Gdi::HBITMAP, lphpalette: *mut super::super::Graphics::Gdi::HPALETTE, bstretch: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    GetWatermarks: usize,
}
impl ::windows_sys::core::Interface for IExtendPropertySheet2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3081269810, data2: 19025, data3: 4561, data4: [167, 234, 0, 192, 79, 217, 9, 221] };
}
#[repr(C)]
pub struct IExtendTaskPad {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub TaskNotify: unsafe extern "system" fn(this: *mut *mut Self, pdo: *mut ::core::ffi::c_void, arg: *const super::Com::VARIANT, param2: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    TaskNotify: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumTasks: unsafe extern "system" fn(this: *mut *mut Self, pdo: *mut ::core::ffi::c_void, sztaskgroup: ::windows_sys::core::PCWSTR, ppenumtask: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumTasks: usize,
    pub GetTitle: unsafe extern "system" fn(this: *mut *mut Self, pszgroup: ::windows_sys::core::PCWSTR, psztitle: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetDescriptiveText: unsafe extern "system" fn(this: *mut *mut Self, pszgroup: ::windows_sys::core::PCWSTR, pszdescriptivetext: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetBackground: unsafe extern "system" fn(this: *mut *mut Self, pszgroup: ::windows_sys::core::PCWSTR, ptdo: *mut MMC_TASK_DISPLAY_OBJECT) -> ::windows_sys::core::HRESULT,
    pub GetListPadInfo: unsafe extern "system" fn(this: *mut *mut Self, pszgroup: ::windows_sys::core::PCWSTR, lplistpadinfo: *mut MMC_LISTPAD_INFO) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IExtendTaskPad {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2381210897, data2: 21837, data3: 4561, data4: [159, 234, 0, 96, 8, 50, 219, 74] };
}
#[repr(C)]
pub struct IExtendView {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub GetViews: unsafe extern "system" fn(this: *mut *mut Self, pdataobject: *mut ::core::ffi::c_void, pviewextensioncallback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetViews: usize,
}
impl ::windows_sys::core::Interface for IExtendView {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2308529390, data2: 53997, data3: 19470, data4: [174, 94, 223, 126, 118, 243, 250, 83] };
}
#[repr(C)]
pub struct IHeaderCtrl {
    pub base__: ::windows_sys::core::IUnknown,
    pub InsertColumn: unsafe extern "system" fn(this: *mut *mut Self, ncol: i32, title: ::windows_sys::core::PCWSTR, nformat: i32, nwidth: i32) -> ::windows_sys::core::HRESULT,
    pub DeleteColumn: unsafe extern "system" fn(this: *mut *mut Self, ncol: i32) -> ::windows_sys::core::HRESULT,
    pub SetColumnText: unsafe extern "system" fn(this: *mut *mut Self, ncol: i32, title: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetColumnText: unsafe extern "system" fn(this: *mut *mut Self, ncol: i32, ptext: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetColumnWidth: unsafe extern "system" fn(this: *mut *mut Self, ncol: i32, nwidth: i32) -> ::windows_sys::core::HRESULT,
    pub GetColumnWidth: unsafe extern "system" fn(this: *mut *mut Self, ncol: i32, pwidth: *mut i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHeaderCtrl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1125346995, data2: 54124, data3: 4559, data4: [173, 188, 0, 170, 0, 168, 0, 51] };
}
#[repr(C)]
pub struct IHeaderCtrl2 {
    pub base__: IHeaderCtrl,
    pub SetChangeTimeOut: unsafe extern "system" fn(this: *mut *mut Self, utimeout: u32) -> ::windows_sys::core::HRESULT,
    pub SetColumnFilter: unsafe extern "system" fn(this: *mut *mut Self, ncolumn: u32, dwtype: u32, pfilterdata: *const MMC_FILTERDATA) -> ::windows_sys::core::HRESULT,
    pub GetColumnFilter: unsafe extern "system" fn(this: *mut *mut Self, ncolumn: u32, pdwtype: *mut u32, pfilterdata: *mut MMC_FILTERDATA) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHeaderCtrl2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2539105208, data2: 6962, data3: 4561, data4: [167, 206, 0, 192, 79, 216, 213, 101] };
}
#[repr(C)]
pub struct IImageList {
    pub base__: ::windows_sys::core::IUnknown,
    pub ImageListSetIcon: unsafe extern "system" fn(this: *mut *mut Self, picon: *const isize, nloc: i32) -> ::windows_sys::core::HRESULT,
    pub ImageListSetStrip: unsafe extern "system" fn(this: *mut *mut Self, pbmapsm: *const isize, pbmaplg: *const isize, nstartloc: i32, cmask: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IImageList {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1125347000, data2: 54124, data3: 4559, data4: [173, 188, 0, 170, 0, 168, 0, 51] };
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ILSIF_LEAVE_LARGE_ICON: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ILSIF_LEAVE_SMALL_ICON: u32 = 536870912u32;
#[repr(C)]
pub struct IMMCVersionInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetMMCVersion: unsafe extern "system" fn(this: *mut *mut Self, pversionmajor: *mut i32, pversionminor: *mut i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMMCVersionInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2832385534, data2: 52683, data3: 19357, data4: [189, 229, 162, 115, 67, 255, 84, 188] };
}
#[repr(C)]
pub struct IMenuButton {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddButton: unsafe extern "system" fn(this: *mut *mut Self, idcommand: i32, lpbuttontext: ::windows_sys::core::PCWSTR, lptooltiptext: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SetButton: unsafe extern "system" fn(this: *mut *mut Self, idcommand: i32, lpbuttontext: ::windows_sys::core::PCWSTR, lptooltiptext: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetButtonState: unsafe extern "system" fn(this: *mut *mut Self, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetButtonState: usize,
}
impl ::windows_sys::core::Interface for IMenuButton {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2501826384, data2: 53376, data3: 4560, data4: [177, 151, 0, 0, 0, 0, 0, 0] };
}
#[repr(C)]
pub struct IMessageView {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetTitleText: unsafe extern "system" fn(this: *mut *mut Self, psztitletext: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SetBodyText: unsafe extern "system" fn(this: *mut *mut Self, pszbodytext: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SetIcon: unsafe extern "system" fn(this: *mut *mut Self, id: IconIdentifier) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMessageView {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2163818868, data2: 64716, data3: 4562, data4: [185, 145, 0, 192, 79, 142, 205, 120] };
}
#[repr(C)]
pub struct INodeProperties {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut *mut Self, pdataobject: *mut ::core::ffi::c_void, szpropertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrproperty: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetProperty: usize,
}
impl ::windows_sys::core::Interface for INodeProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 364662052, data2: 42274, data3: 17414, data4: [170, 85, 7, 73, 83, 122, 104, 101] };
}
#[repr(C)]
pub struct IPropertySheetCallback {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_UI_Controls")]
    pub AddPage: unsafe extern "system" fn(this: *mut *mut Self, hpage: super::super::UI::Controls::HPROPSHEETPAGE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Controls"))]
    AddPage: usize,
    #[cfg(feature = "Win32_UI_Controls")]
    pub RemovePage: unsafe extern "system" fn(this: *mut *mut Self, hpage: super::super::UI::Controls::HPROPSHEETPAGE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Controls"))]
    RemovePage: usize,
}
impl ::windows_sys::core::Interface for IPropertySheetCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2245944541, data2: 61217, data3: 4559, data4: [162, 133, 0, 192, 79, 216, 219, 230] };
}
#[repr(C)]
pub struct IPropertySheetProvider {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePropertySheet: unsafe extern "system" fn(this: *mut *mut Self, title: ::windows_sys::core::PCWSTR, r#type: u8, cookie: isize, pidataobjectm: *mut ::core::ffi::c_void, dwoptions: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePropertySheet: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub FindPropertySheet: unsafe extern "system" fn(this: *mut *mut Self, hitem: isize, lpcomponent: *mut ::core::ffi::c_void, lpdataobject: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FindPropertySheet: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AddPrimaryPages: unsafe extern "system" fn(this: *mut *mut Self, lpunknown: *mut ::core::ffi::c_void, bcreatehandle: super::super::Foundation::BOOL, hnotifywindow: super::super::Foundation::HWND, bscopepane: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddPrimaryPages: usize,
    pub AddExtensionPages: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Show: unsafe extern "system" fn(this: *mut *mut Self, window: isize, page: i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPropertySheetProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2245944542, data2: 61217, data3: 4559, data4: [162, 133, 0, 192, 79, 216, 219, 230] };
}
#[repr(C)]
pub struct IRequiredExtensions {
    pub base__: ::windows_sys::core::IUnknown,
    pub EnableAllExtensions: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetFirstExtension: unsafe extern "system" fn(this: *mut *mut Self, pextclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetNextExtension: unsafe extern "system" fn(this: *mut *mut Self, pextclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRequiredExtensions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1920478586, data2: 42144, data3: 4561, data4: [175, 15, 0, 192, 79, 182, 221, 44] };
}
#[repr(C)]
pub struct IResultData {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub InsertItem: unsafe extern "system" fn(this: *mut *mut Self, item: *mut RESULTDATAITEM) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InsertItem: usize,
    pub DeleteItem: unsafe extern "system" fn(this: *mut *mut Self, itemid: isize, ncol: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FindItemByLParam: unsafe extern "system" fn(this: *mut *mut Self, lparam: super::super::Foundation::LPARAM, pitemid: *mut isize) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindItemByLParam: usize,
    pub DeleteAllRsltItems: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetItem: unsafe extern "system" fn(this: *mut *mut Self, item: *const RESULTDATAITEM) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetItem: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetItem: unsafe extern "system" fn(this: *mut *mut Self, item: *mut RESULTDATAITEM) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetItem: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetNextItem: unsafe extern "system" fn(this: *mut *mut Self, item: *mut RESULTDATAITEM) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetNextItem: usize,
    pub ModifyItemState: unsafe extern "system" fn(this: *mut *mut Self, nindex: i32, itemid: isize, uadd: u32, uremove: u32) -> ::windows_sys::core::HRESULT,
    pub ModifyViewStyle: unsafe extern "system" fn(this: *mut *mut Self, add: MMC_RESULT_VIEW_STYLE, remove: MMC_RESULT_VIEW_STYLE) -> ::windows_sys::core::HRESULT,
    pub SetViewMode: unsafe extern "system" fn(this: *mut *mut Self, lviewmode: i32) -> ::windows_sys::core::HRESULT,
    pub GetViewMode: unsafe extern "system" fn(this: *mut *mut Self, lviewmode: *mut i32) -> ::windows_sys::core::HRESULT,
    pub UpdateItem: unsafe extern "system" fn(this: *mut *mut Self, itemid: isize) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Sort: unsafe extern "system" fn(this: *mut *mut Self, ncolumn: i32, dwsortoptions: u32, luserparam: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Sort: usize,
    pub SetDescBarText: unsafe extern "system" fn(this: *mut *mut Self, desctext: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SetItemCount: unsafe extern "system" fn(this: *mut *mut Self, nitemcount: i32, dwoptions: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IResultData {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 836394912, data2: 57579, data3: 4559, data4: [159, 33, 0, 170, 0, 60, 169, 246] };
}
#[repr(C)]
pub struct IResultData2 {
    pub base__: IResultData,
    pub RenameResultItem: unsafe extern "system" fn(this: *mut *mut Self, itemid: isize) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IResultData2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 255254763, data2: 42993, data3: 19073, data4: [190, 90, 146, 71, 247, 222, 75, 27] };
}
#[repr(C)]
pub struct IResultDataCompare {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Compare: unsafe extern "system" fn(this: *mut *mut Self, luserparam: super::super::Foundation::LPARAM, cookiea: isize, cookieb: isize, pnresult: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Compare: usize,
}
impl ::windows_sys::core::Interface for IResultDataCompare {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3895548498, data2: 31258, data3: 4560, data4: [162, 210, 0, 192, 79, 217, 9, 221] };
}
#[repr(C)]
pub struct IResultDataCompareEx {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Compare: unsafe extern "system" fn(this: *mut *mut Self, prdc: *const RDCOMPARE, pnresult: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Compare: usize,
}
impl ::windows_sys::core::Interface for IResultDataCompareEx {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2526229622, data2: 593, data3: 4563, data4: [174, 176, 0, 192, 79, 142, 205, 120] };
}
#[repr(C)]
pub struct IResultOwnerData {
    pub base__: ::windows_sys::core::IUnknown,
    pub FindItem: unsafe extern "system" fn(this: *mut *mut Self, pfindinfo: *const RESULTFINDINFO, pnfoundindex: *mut i32) -> ::windows_sys::core::HRESULT,
    pub CacheHint: unsafe extern "system" fn(this: *mut *mut Self, nstartindex: i32, nendindex: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SortItems: unsafe extern "system" fn(this: *mut *mut Self, ncolumn: i32, dwsortoptions: u32, luserparam: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SortItems: usize,
}
impl ::windows_sys::core::Interface for IResultOwnerData {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2629015256, data2: 60035, data3: 4560, data4: [174, 241, 0, 192, 79, 182, 221, 44] };
}
#[repr(C)]
pub struct ISnapinAbout {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetSnapinDescription: unsafe extern "system" fn(this: *mut *mut Self, lpdescription: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetProvider: unsafe extern "system" fn(this: *mut *mut Self, lpname: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetSnapinVersion: unsafe extern "system" fn(this: *mut *mut Self, lpversion: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetSnapinImage: unsafe extern "system" fn(this: *mut *mut Self, happicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetSnapinImage: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetStaticFolderImage: unsafe extern "system" fn(this: *mut *mut Self, hsmallimage: *mut super::super::Graphics::Gdi::HBITMAP, hsmallimageopen: *mut super::super::Graphics::Gdi::HBITMAP, hlargeimage: *mut super::super::Graphics::Gdi::HBITMAP, cmask: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetStaticFolderImage: usize,
}
impl ::windows_sys::core::Interface for ISnapinAbout {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 306520204, data2: 41297, data3: 4560, data4: [167, 215, 0, 192, 79, 217, 9, 221] };
}
#[repr(C)]
pub struct ISnapinHelp {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetHelpTopic: unsafe extern "system" fn(this: *mut *mut Self, lpcompiledhelpfile: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISnapinHelp {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2796640974, data2: 57177, data3: 4560, data4: [167, 221, 0, 192, 79, 217, 9, 221] };
}
#[repr(C)]
pub struct ISnapinHelp2 {
    pub base__: ISnapinHelp,
    pub GetLinkedTopics: unsafe extern "system" fn(this: *mut *mut Self, lpcompiledhelpfiles: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISnapinHelp2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1214357520, data2: 8441, data3: 4562, data4: [165, 16, 0, 192, 79, 182, 221, 44] };
}
#[repr(C)]
pub struct ISnapinProperties {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, pproperties: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    pub QueryPropertyNames: unsafe extern "system" fn(this: *mut *mut Self, pcallback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PropertiesChanged: unsafe extern "system" fn(this: *mut *mut Self, cproperties: i32, pproperties: *const MMC_SNAPIN_PROPERTY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PropertiesChanged: usize,
}
impl ::windows_sys::core::Interface for ISnapinProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4152925609, data2: 18946, data3: 18487, data4: [191, 137, 26, 111, 42, 2, 16, 16] };
}
#[repr(C)]
pub struct ISnapinPropertiesCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddPropertyName: unsafe extern "system" fn(this: *mut *mut Self, pszpropname: ::windows_sys::core::PCWSTR, dwflags: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISnapinPropertiesCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2769265381, data2: 32353, data3: 17899, data4: [168, 212, 154, 7, 179, 232, 81, 168] };
}
#[repr(C)]
pub struct IStringTable {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddString: unsafe extern "system" fn(this: *mut *mut Self, pszadd: ::windows_sys::core::PCWSTR, pstringid: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetString: unsafe extern "system" fn(this: *mut *mut Self, stringid: u32, cchbuffer: u32, lpbuffer: ::windows_sys::core::PWSTR, pcchout: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetStringLength: unsafe extern "system" fn(this: *mut *mut Self, stringid: u32, pcchstring: *mut u32) -> ::windows_sys::core::HRESULT,
    pub DeleteString: unsafe extern "system" fn(this: *mut *mut Self, stringid: u32) -> ::windows_sys::core::HRESULT,
    pub DeleteAllStrings: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub FindString: unsafe extern "system" fn(this: *mut *mut Self, pszfind: ::windows_sys::core::PCWSTR, pstringid: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Enumerate: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Enumerate: usize,
}
impl ::windows_sys::core::Interface for IStringTable {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3728783268, data2: 3941, data3: 4562, data4: [142, 37, 0, 192, 79, 142, 205, 120] };
}
#[repr(C)]
pub struct IToolbar {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub AddBitmap: unsafe extern "system" fn(this: *mut *mut Self, nimages: i32, hbmp: super::super::Graphics::Gdi::HBITMAP, cxsize: i32, cysize: i32, crmask: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    AddBitmap: usize,
    pub AddButtons: unsafe extern "system" fn(this: *mut *mut Self, nbuttons: i32, lpbuttons: *const MMCBUTTON) -> ::windows_sys::core::HRESULT,
    pub InsertButton: unsafe extern "system" fn(this: *mut *mut Self, nindex: i32, lpbutton: *const MMCBUTTON) -> ::windows_sys::core::HRESULT,
    pub DeleteButton: unsafe extern "system" fn(this: *mut *mut Self, nindex: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetButtonState: unsafe extern "system" fn(this: *mut *mut Self, idcommand: i32, nstate: MMC_BUTTON_STATE, pstate: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetButtonState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetButtonState: unsafe extern "system" fn(this: *mut *mut Self, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetButtonState: usize,
}
impl ::windows_sys::core::Interface for IToolbar {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1125347001, data2: 54124, data3: 4559, data4: [173, 188, 0, 170, 0, 168, 0, 51] };
}
#[repr(C)]
pub struct IViewExtensionCallback {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub AddView: unsafe extern "system" fn(this: *mut *mut Self, pextviewdata: *const MMC_EXT_VIEW_DATA) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddView: usize,
}
impl ::windows_sys::core::Interface for IViewExtensionCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 886936202, data2: 30105, data3: 16869, data4: [159, 94, 214, 188, 48, 98, 194, 218] };
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub type IconIdentifier = i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const Icon_None: IconIdentifier = 0i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const Icon_Error: IconIdentifier = 32513i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const Icon_Question: IconIdentifier = 32514i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const Icon_Warning: IconIdentifier = 32515i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const Icon_Information: IconIdentifier = 32516i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const Icon_First: IconIdentifier = 32513i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const Icon_Last: IconIdentifier = 32516i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct MENUBUTTONDATA {
    pub idCommand: i32,
    pub x: i32,
    pub y: i32,
}
impl ::core::marker::Copy for MENUBUTTONDATA {}
impl ::core::clone::Clone for MENUBUTTONDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct MMCBUTTON {
    pub nBitmap: i32,
    pub idCommand: i32,
    pub fsState: u8,
    pub fsType: u8,
    pub lpButtonText: ::windows_sys::core::PWSTR,
    pub lpTooltipText: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for MMCBUTTON {}
impl ::core::clone::Clone for MMCBUTTON {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCLV_AUTO: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCLV_NOICON: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCLV_NOPARAM: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCLV_NOPTR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCLV_UPDATE_NOINVALIDATEALL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCLV_UPDATE_NOSCROLL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCLV_VIEWSTYLE_FILTERED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCLV_VIEWSTYLE_ICON: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCLV_VIEWSTYLE_LIST: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCLV_VIEWSTYLE_REPORT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCLV_VIEWSTYLE_SMALLICON: u32 = 2u32;
pub const MMCVersionInfo: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3607026461, data2: 53025, data3: 19417, data4: [175, 59, 197, 70, 142, 156, 102, 132] };
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub type MMC_ACTION_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_ACTION_UNINITIALIZED: MMC_ACTION_TYPE = -1i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_ACTION_ID: MMC_ACTION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_ACTION_LINK: MMC_ACTION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_ACTION_SCRIPT: MMC_ACTION_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub type MMC_BUTTON_STATE = i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ENABLED: MMC_BUTTON_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CHECKED: MMC_BUTTON_STATE = 2i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const HIDDEN: MMC_BUTTON_STATE = 4i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const INDETERMINATE: MMC_BUTTON_STATE = 8i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const BUTTONPRESSED: MMC_BUTTON_STATE = 16i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct MMC_COLUMN_DATA {
    pub nColIndex: i32,
    pub dwFlags: u32,
    pub nWidth: i32,
    pub ulReserved: usize,
}
impl ::core::marker::Copy for MMC_COLUMN_DATA {}
impl ::core::clone::Clone for MMC_COLUMN_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct MMC_COLUMN_SET_DATA {
    pub cbSize: i32,
    pub nNumCols: i32,
    pub pColData: *mut MMC_COLUMN_DATA,
}
impl ::core::marker::Copy for MMC_COLUMN_SET_DATA {}
impl ::core::clone::Clone for MMC_COLUMN_SET_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub type MMC_CONSOLE_VERB = i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VERB_NONE: MMC_CONSOLE_VERB = 0i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VERB_OPEN: MMC_CONSOLE_VERB = 32768i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VERB_COPY: MMC_CONSOLE_VERB = 32769i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VERB_PASTE: MMC_CONSOLE_VERB = 32770i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VERB_DELETE: MMC_CONSOLE_VERB = 32771i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VERB_PROPERTIES: MMC_CONSOLE_VERB = 32772i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VERB_RENAME: MMC_CONSOLE_VERB = 32773i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VERB_REFRESH: MMC_CONSOLE_VERB = 32774i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VERB_PRINT: MMC_CONSOLE_VERB = 32775i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VERB_CUT: MMC_CONSOLE_VERB = 32776i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VERB_MAX: MMC_CONSOLE_VERB = 32777i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VERB_FIRST: MMC_CONSOLE_VERB = 32768i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VERB_LAST: MMC_CONSOLE_VERB = 32776i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub type MMC_CONTROL_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const TOOLBAR: MMC_CONTROL_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MENUBUTTON: MMC_CONTROL_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const COMBOBOXBAR: MMC_CONTROL_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_DEFAULT_OPERATION_COPY: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_EXPANDSYNC_STRUCT {
    pub bHandled: super::super::Foundation::BOOL,
    pub bExpanding: super::super::Foundation::BOOL,
    pub hItem: isize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MMC_EXPANDSYNC_STRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MMC_EXPANDSYNC_STRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_EXT_VIEW_DATA {
    pub viewID: ::windows_sys::core::GUID,
    pub pszURL: ::windows_sys::core::PCWSTR,
    pub pszViewTitle: ::windows_sys::core::PCWSTR,
    pub pszTooltipText: ::windows_sys::core::PCWSTR,
    pub bReplacesDefaultView: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MMC_EXT_VIEW_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MMC_EXT_VIEW_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct MMC_FILTERDATA {
    pub pszText: ::windows_sys::core::PWSTR,
    pub cchTextMax: i32,
    pub lValue: i32,
}
impl ::core::marker::Copy for MMC_FILTERDATA {}
impl ::core::clone::Clone for MMC_FILTERDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub type MMC_FILTER_CHANGE_CODE = i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MFCC_DISABLE: MMC_FILTER_CHANGE_CODE = 0i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MFCC_ENABLE: MMC_FILTER_CHANGE_CODE = 1i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MFCC_VALUE_CHANGE: MMC_FILTER_CHANGE_CODE = 2i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub type MMC_FILTER_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_STRING_FILTER: MMC_FILTER_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_INT_FILTER: MMC_FILTER_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_FILTER_NOVALUE: MMC_FILTER_TYPE = 32768i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_IMAGECALLBACK: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_ITEM_OVERLAY_STATE_MASK: u32 = 3840u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_ITEM_OVERLAY_STATE_SHIFT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_ITEM_STATE_MASK: u32 = 255u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct MMC_LISTPAD_INFO {
    pub szTitle: ::windows_sys::core::PWSTR,
    pub szButtonText: ::windows_sys::core::PWSTR,
    pub nCommandID: isize,
}
impl ::core::marker::Copy for MMC_LISTPAD_INFO {}
impl ::core::clone::Clone for MMC_LISTPAD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub type MMC_MENU_COMMAND_IDS = i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCC_STANDARD_VIEW_SELECT: MMC_MENU_COMMAND_IDS = -1i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_MULTI_SELECT_COOKIE: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_NODEID_SLOW_RETRIEVAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub type MMC_NOTIFY_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_ACTIVATE: MMC_NOTIFY_TYPE = 32769i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_ADD_IMAGES: MMC_NOTIFY_TYPE = 32770i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_BTN_CLICK: MMC_NOTIFY_TYPE = 32771i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_CLICK: MMC_NOTIFY_TYPE = 32772i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_COLUMN_CLICK: MMC_NOTIFY_TYPE = 32773i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_CONTEXTMENU: MMC_NOTIFY_TYPE = 32774i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_CUTORMOVE: MMC_NOTIFY_TYPE = 32775i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_DBLCLICK: MMC_NOTIFY_TYPE = 32776i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_DELETE: MMC_NOTIFY_TYPE = 32777i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_DESELECT_ALL: MMC_NOTIFY_TYPE = 32778i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_EXPAND: MMC_NOTIFY_TYPE = 32779i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_HELP: MMC_NOTIFY_TYPE = 32780i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_MENU_BTNCLICK: MMC_NOTIFY_TYPE = 32781i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_MINIMIZED: MMC_NOTIFY_TYPE = 32782i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_PASTE: MMC_NOTIFY_TYPE = 32783i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_PROPERTY_CHANGE: MMC_NOTIFY_TYPE = 32784i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_QUERY_PASTE: MMC_NOTIFY_TYPE = 32785i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_REFRESH: MMC_NOTIFY_TYPE = 32786i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_REMOVE_CHILDREN: MMC_NOTIFY_TYPE = 32787i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_RENAME: MMC_NOTIFY_TYPE = 32788i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_SELECT: MMC_NOTIFY_TYPE = 32789i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_SHOW: MMC_NOTIFY_TYPE = 32790i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_VIEW_CHANGE: MMC_NOTIFY_TYPE = 32791i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_SNAPINHELP: MMC_NOTIFY_TYPE = 32792i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_CONTEXTHELP: MMC_NOTIFY_TYPE = 32793i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_INITOCX: MMC_NOTIFY_TYPE = 32794i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_FILTER_CHANGE: MMC_NOTIFY_TYPE = 32795i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_FILTERBTN_CLICK: MMC_NOTIFY_TYPE = 32796i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_RESTORE_VIEW: MMC_NOTIFY_TYPE = 32797i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_PRINT: MMC_NOTIFY_TYPE = 32798i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_PRELOAD: MMC_NOTIFY_TYPE = 32799i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_LISTPAD: MMC_NOTIFY_TYPE = 32800i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_EXPANDSYNC: MMC_NOTIFY_TYPE = 32801i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_COLUMNS_CHANGED: MMC_NOTIFY_TYPE = 32802i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_CANPASTE_OUTOFPROC: MMC_NOTIFY_TYPE = 32803i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_NW_OPTION_CUSTOMTITLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_NW_OPTION_NOACTIONPANE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_NW_OPTION_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_NW_OPTION_NOPERSIST: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_NW_OPTION_NOSCOPEPANE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_NW_OPTION_NOTOOLBARS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_NW_OPTION_SHORTTITLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub type MMC_PROPERTY_ACTION = i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_PROPACT_DELETING: MMC_PROPERTY_ACTION = 1i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_PROPACT_CHANGING: MMC_PROPERTY_ACTION = 2i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_PROPACT_INITIALIZED: MMC_PROPERTY_ACTION = 3i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_PROP_CHANGEAFFECTSUI: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_PROP_MODIFIABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_PROP_PERSIST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_PROP_REMOVABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_PSO_HASHELP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_PSO_NEWWIZARDTYPE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_PSO_NOAPPLYNOW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_PSO_NO_PROPTITLE: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct MMC_RESTORE_VIEW {
    pub dwSize: u32,
    pub cookie: isize,
    pub pViewType: ::windows_sys::core::PWSTR,
    pub lViewOptions: i32,
}
impl ::core::marker::Copy for MMC_RESTORE_VIEW {}
impl ::core::clone::Clone for MMC_RESTORE_VIEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub type MMC_RESULT_VIEW_STYLE = i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_SINGLESEL: MMC_RESULT_VIEW_STYLE = 1i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_SHOWSELALWAYS: MMC_RESULT_VIEW_STYLE = 2i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_NOSORTHEADER: MMC_RESULT_VIEW_STYLE = 4i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_ENSUREFOCUSVISIBLE: MMC_RESULT_VIEW_STYLE = 8i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub type MMC_SCOPE_ITEM_STATE = i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_SCOPE_ITEM_STATE_NORMAL: MMC_SCOPE_ITEM_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_SCOPE_ITEM_STATE_BOLD: MMC_SCOPE_ITEM_STATE = 2i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_SCOPE_ITEM_STATE_EXPANDEDONCE: MMC_SCOPE_ITEM_STATE = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct MMC_SNAPIN_PROPERTY {
    pub pszPropName: ::windows_sys::core::PCWSTR,
    pub varValue: super::Com::VARIANT,
    pub eAction: MMC_PROPERTY_ACTION,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for MMC_SNAPIN_PROPERTY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for MMC_SNAPIN_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct MMC_SORT_DATA {
    pub nColIndex: i32,
    pub dwSortOptions: u32,
    pub ulReserved: usize,
}
impl ::core::marker::Copy for MMC_SORT_DATA {}
impl ::core::clone::Clone for MMC_SORT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct MMC_SORT_SET_DATA {
    pub cbSize: i32,
    pub nNumItems: i32,
    pub pSortData: *mut MMC_SORT_DATA,
}
impl ::core::marker::Copy for MMC_SORT_SET_DATA {}
impl ::core::clone::Clone for MMC_SORT_SET_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct MMC_TASK {
    pub sDisplayObject: MMC_TASK_DISPLAY_OBJECT,
    pub szText: ::windows_sys::core::PWSTR,
    pub szHelpString: ::windows_sys::core::PWSTR,
    pub eActionType: MMC_ACTION_TYPE,
    pub Anonymous: MMC_TASK_0,
}
impl ::core::marker::Copy for MMC_TASK {}
impl ::core::clone::Clone for MMC_TASK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub union MMC_TASK_0 {
    pub nCommandID: isize,
    pub szActionURL: ::windows_sys::core::PWSTR,
    pub szScript: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for MMC_TASK_0 {}
impl ::core::clone::Clone for MMC_TASK_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct MMC_TASK_DISPLAY_BITMAP {
    pub szMouseOverBitmap: ::windows_sys::core::PWSTR,
    pub szMouseOffBitmap: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for MMC_TASK_DISPLAY_BITMAP {}
impl ::core::clone::Clone for MMC_TASK_DISPLAY_BITMAP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct MMC_TASK_DISPLAY_OBJECT {
    pub eDisplayType: MMC_TASK_DISPLAY_TYPE,
    pub Anonymous: MMC_TASK_DISPLAY_OBJECT_0,
}
impl ::core::marker::Copy for MMC_TASK_DISPLAY_OBJECT {}
impl ::core::clone::Clone for MMC_TASK_DISPLAY_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub union MMC_TASK_DISPLAY_OBJECT_0 {
    pub uBitmap: MMC_TASK_DISPLAY_BITMAP,
    pub uSymbol: MMC_TASK_DISPLAY_SYMBOL,
}
impl ::core::marker::Copy for MMC_TASK_DISPLAY_OBJECT_0 {}
impl ::core::clone::Clone for MMC_TASK_DISPLAY_OBJECT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct MMC_TASK_DISPLAY_SYMBOL {
    pub szFontFamilyName: ::windows_sys::core::PWSTR,
    pub szURLtoEOT: ::windows_sys::core::PWSTR,
    pub szSymbolString: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for MMC_TASK_DISPLAY_SYMBOL {}
impl ::core::clone::Clone for MMC_TASK_DISPLAY_SYMBOL {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub type MMC_TASK_DISPLAY_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_TASK_DISPLAY_UNINITIALIZED: MMC_TASK_DISPLAY_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_TASK_DISPLAY_TYPE_SYMBOL: MMC_TASK_DISPLAY_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_TASK_DISPLAY_TYPE_VANILLA_GIF: MMC_TASK_DISPLAY_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_TASK_DISPLAY_TYPE_CHOCOLATE_GIF: MMC_TASK_DISPLAY_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_TASK_DISPLAY_TYPE_BITMAP: MMC_TASK_DISPLAY_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VER: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VIEW_OPTIONS_CREATENEW: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VIEW_OPTIONS_EXCLUDE_SCOPE_ITEMS_FROM_LIST: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VIEW_OPTIONS_FILTERED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VIEW_OPTIONS_LEXICAL_SORT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VIEW_OPTIONS_MULTISELECT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VIEW_OPTIONS_NOLISTVIEWS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VIEW_OPTIONS_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VIEW_OPTIONS_OWNERDATALIST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VIEW_OPTIONS_USEFONTLINKING: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub type MMC_VIEW_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VIEW_TYPE_LIST: MMC_VIEW_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VIEW_TYPE_HTML: MMC_VIEW_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VIEW_TYPE_OCX: MMC_VIEW_TYPE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct MMC_VISIBLE_COLUMNS {
    pub nVisibleColumns: i32,
    pub rgVisibleCols: [i32; 1],
}
impl ::core::marker::Copy for MMC_VISIBLE_COLUMNS {}
impl ::core::clone::Clone for MMC_VISIBLE_COLUMNS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_WINDOW_COOKIE: i32 = -3i32;
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct MenuItem {
    pub base__: super::Com::IDispatch,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, displayname: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    pub LanguageIndependentName: unsafe extern "system" fn(this: *mut *mut Self, languageindependentname: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, path: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    pub LanguageIndependentPath: unsafe extern "system" fn(this: *mut *mut Self, languageindependentpath: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    pub Execute: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, enabled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enabled: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for MenuItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 24705745, data2: 45921, data3: 19239, data4: [150, 173, 103, 197, 126, 191, 46, 29] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct Node {
    pub base__: super::Com::IDispatch,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_Property: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyvalue: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_Property: usize,
    pub Bookmark: unsafe extern "system" fn(this: *mut *mut Self, bookmark: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsScopeNode: unsafe extern "system" fn(this: *mut *mut Self, isscopenode: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsScopeNode: usize,
    pub Nodetype: unsafe extern "system" fn(this: *mut *mut Self, nodetype: *mut *mut u16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for Node {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4162770944, data2: 30777, data3: 17479, data4: [148, 93, 142, 21, 218, 89, 202, 85] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct Nodes {
    pub base__: super::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, node: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for Nodes {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 825950687, data2: 45615, data3: 19778, data4: [177, 184, 72, 60, 220, 245, 29, 53] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct Properties {
    pub base__: super::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, property: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Remove: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for Properties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 679914434, data2: 42021, data3: 17074, data4: [145, 198, 226, 92, 14, 4, 88, 28] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct Property {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, value: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut *mut u16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for Property {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1174455205, data2: 58113, data3: 16856, data4: [182, 208, 239, 46, 66, 18, 224, 202] };
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RDCI_ScopeItem: u32 = 2147483648u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RDCOMPARE {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub nColumn: i32,
    pub lUserParam: super::super::Foundation::LPARAM,
    pub prdch1: *mut RDITEMHDR,
    pub prdch2: *mut RDITEMHDR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RDCOMPARE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RDCOMPARE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RDITEMHDR {
    pub dwFlags: u32,
    pub cookie: isize,
    pub lpReserved: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RDITEMHDR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RDITEMHDR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RDI_IMAGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RDI_INDENT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RDI_INDEX: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RDI_PARAM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RDI_STATE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RDI_STR: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RESULTDATAITEM {
    pub mask: u32,
    pub bScopeItem: super::super::Foundation::BOOL,
    pub itemID: isize,
    pub nIndex: i32,
    pub nCol: i32,
    pub str: ::windows_sys::core::PWSTR,
    pub nImage: i32,
    pub nState: u32,
    pub lParam: super::super::Foundation::LPARAM,
    pub iIndent: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESULTDATAITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESULTDATAITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct RESULTFINDINFO {
    pub psz: ::windows_sys::core::PWSTR,
    pub nStart: i32,
    pub dwOptions: u32,
}
impl ::core::marker::Copy for RESULTFINDINFO {}
impl ::core::clone::Clone for RESULTFINDINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct RESULT_VIEW_TYPE_INFO {
    pub pstrPersistableViewDescription: ::windows_sys::core::PWSTR,
    pub eViewType: MMC_VIEW_TYPE,
    pub dwMiscOptions: u32,
    pub Anonymous: RESULT_VIEW_TYPE_INFO_0,
}
impl ::core::marker::Copy for RESULT_VIEW_TYPE_INFO {}
impl ::core::clone::Clone for RESULT_VIEW_TYPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub union RESULT_VIEW_TYPE_INFO_0 {
    pub dwListOptions: u32,
    pub Anonymous1: RESULT_VIEW_TYPE_INFO_0_0,
    pub Anonymous2: RESULT_VIEW_TYPE_INFO_0_1,
}
impl ::core::marker::Copy for RESULT_VIEW_TYPE_INFO_0 {}
impl ::core::clone::Clone for RESULT_VIEW_TYPE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct RESULT_VIEW_TYPE_INFO_0_0 {
    pub dwHTMLOptions: u32,
    pub pstrURL: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for RESULT_VIEW_TYPE_INFO_0_0 {}
impl ::core::clone::Clone for RESULT_VIEW_TYPE_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct RESULT_VIEW_TYPE_INFO_0_1 {
    pub dwOCXOptions: u32,
    pub pUnkControl: *mut *mut *mut *mut ::windows_sys::core::IUnknown,
}
impl ::core::marker::Copy for RESULT_VIEW_TYPE_INFO_0_1 {}
impl ::core::clone::Clone for RESULT_VIEW_TYPE_INFO_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RFI_PARTIAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RFI_WRAP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RSI_DESCENDING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RSI_NOSORTICON: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RVTI_HTML_OPTIONS_NOLISTVIEW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RVTI_HTML_OPTIONS_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RVTI_LIST_OPTIONS_ALLOWPASTE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RVTI_LIST_OPTIONS_EXCLUDE_SCOPE_ITEMS_FROM_LIST: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RVTI_LIST_OPTIONS_FILTERED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RVTI_LIST_OPTIONS_LEXICAL_SORT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RVTI_LIST_OPTIONS_MULTISELECT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RVTI_LIST_OPTIONS_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RVTI_LIST_OPTIONS_OWNERDATALIST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RVTI_LIST_OPTIONS_USEFONTLINKING: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RVTI_MISC_OPTIONS_NOLISTVIEWS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RVTI_OCX_OPTIONS_CACHE_OCX: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RVTI_OCX_OPTIONS_NOLISTVIEW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RVTI_OCX_OPTIONS_NONE: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SCOPEDATAITEM {
    pub mask: u32,
    pub displayname: ::windows_sys::core::PWSTR,
    pub nImage: i32,
    pub nOpenImage: i32,
    pub nState: u32,
    pub cChildren: i32,
    pub lParam: super::super::Foundation::LPARAM,
    pub relativeID: isize,
    pub ID: isize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SCOPEDATAITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SCOPEDATAITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct SColumnSetID {
    pub dwFlags: u32,
    pub cBytes: u32,
    pub id: [u8; 1],
}
impl ::core::marker::Copy for SColumnSetID {}
impl ::core::clone::Clone for SColumnSetID {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SDI_CHILDREN: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SDI_FIRST: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SDI_IMAGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SDI_NEXT: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SDI_OPENIMAGE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SDI_PARAM: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SDI_PARENT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SDI_PREVIOUS: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SDI_STATE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SDI_STR: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct SMMCDataObjects {
    pub count: u32,
    pub lpDataObject: [*mut *mut super::Com::IDataObject; 1],
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for SMMCDataObjects {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for SMMCDataObjects {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct SMMCObjectTypes {
    pub count: u32,
    pub guid: [::windows_sys::core::GUID; 1],
}
impl ::core::marker::Copy for SMMCObjectTypes {}
impl ::core::clone::Clone for SMMCObjectTypes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct SNodeID {
    pub cBytes: u32,
    pub id: [u8; 1],
}
impl ::core::marker::Copy for SNodeID {}
impl ::core::clone::Clone for SNodeID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct SNodeID2 {
    pub dwFlags: u32,
    pub cBytes: u32,
    pub id: [u8; 1],
}
impl ::core::marker::Copy for SNodeID2 {}
impl ::core::clone::Clone for SNodeID2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SPECIAL_COOKIE_MAX: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SPECIAL_COOKIE_MIN: i32 = -10i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SPECIAL_DOBJ_MAX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SPECIAL_DOBJ_MIN: i32 = -10i32;
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ScopeNamespace {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub GetParent: unsafe extern "system" fn(this: *mut *mut Self, node: *mut ::core::ffi::c_void, parent: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetParent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetChild: unsafe extern "system" fn(this: *mut *mut Self, node: *mut ::core::ffi::c_void, child: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetChild: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNext: unsafe extern "system" fn(this: *mut *mut Self, node: *mut ::core::ffi::c_void, next: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNext: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRoot: unsafe extern "system" fn(this: *mut *mut Self, root: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRoot: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Expand: unsafe extern "system" fn(this: *mut *mut Self, node: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Expand: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ScopeNamespace {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3954919644, data2: 6715, data3: 19846, data4: [183, 134, 194, 27, 40, 56, 144, 18] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct SnapIn {
    pub base__: super::Com::IDispatch,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    pub Vendor: unsafe extern "system" fn(this: *mut *mut Self, vendor: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut *mut Self, version: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Extensions: unsafe extern "system" fn(this: *mut *mut Self, extensions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Extensions: usize,
    pub SnapinCLSID: unsafe extern "system" fn(this: *mut *mut Self, snapinclsid: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, properties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableAllExtensions: unsafe extern "system" fn(this: *mut *mut Self, enable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableAllExtensions: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for SnapIn {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1005129974, data2: 13401, data3: 18886, data4: [161, 187, 65, 230, 190, 157, 243, 234] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct SnapIns {
    pub base__: super::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, snapin: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, snapinnameorclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, parentsnapin: ::core::mem::ManuallyDrop<super::Com::VARIANT>, properties: ::core::mem::ManuallyDrop<super::Com::VARIANT>, snapin: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Add: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, snapin: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Remove: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for SnapIns {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 787734045, data2: 45354, data3: 18897, data4: [146, 197, 11, 0, 121, 135, 104, 241] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct View {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub ActiveScopeNode: unsafe extern "system" fn(this: *mut *mut Self, node: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActiveScopeNode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetActiveScopeNode: unsafe extern "system" fn(this: *mut *mut Self, node: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetActiveScopeNode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Selection: unsafe extern "system" fn(this: *mut *mut Self, nodes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Selection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ListItems: unsafe extern "system" fn(this: *mut *mut Self, nodes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ListItems: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SnapinScopeObject: unsafe extern "system" fn(this: *mut *mut Self, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>, scopenodeobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SnapinScopeObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SnapinSelectionObject: unsafe extern "system" fn(this: *mut *mut Self, selectionobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SnapinSelectionObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Is: unsafe extern "system" fn(this: *mut *mut Self, view: *mut ::core::ffi::c_void, thesame: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Is: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Document: unsafe extern "system" fn(this: *mut *mut Self, document: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Document: usize,
    pub SelectAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Select: unsafe extern "system" fn(this: *mut *mut Self, node: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Select: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Deselect: unsafe extern "system" fn(this: *mut *mut Self, node: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Deselect: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub IsSelected: unsafe extern "system" fn(this: *mut *mut Self, node: *mut ::core::ffi::c_void, isselected: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    IsSelected: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DisplayScopeNodePropertySheet: unsafe extern "system" fn(this: *mut *mut Self, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DisplayScopeNodePropertySheet: usize,
    pub DisplaySelectionPropertySheet: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CopyScopeNode: unsafe extern "system" fn(this: *mut *mut Self, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CopyScopeNode: usize,
    pub CopySelection: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteScopeNode: unsafe extern "system" fn(this: *mut *mut Self, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteScopeNode: usize,
    pub DeleteSelection: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RenameScopeNode: unsafe extern "system" fn(this: *mut *mut Self, newname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RenameScopeNode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RenameSelectedItem: unsafe extern "system" fn(this: *mut *mut Self, newname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RenameSelectedItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_ScopeNodeContextMenu: unsafe extern "system" fn(this: *mut *mut Self, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>, contextmenu: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_ScopeNodeContextMenu: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SelectionContextMenu: unsafe extern "system" fn(this: *mut *mut Self, contextmenu: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SelectionContextMenu: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RefreshScopeNode: unsafe extern "system" fn(this: *mut *mut Self, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RefreshScopeNode: usize,
    pub RefreshSelection: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ExecuteSelectionMenuItem: unsafe extern "system" fn(this: *mut *mut Self, menuitempath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExecuteSelectionMenuItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ExecuteScopeNodeMenuItem: unsafe extern "system" fn(this: *mut *mut Self, menuitempath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, scopenode: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ExecuteScopeNodeMenuItem: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExecuteShellCommand: unsafe extern "system" fn(this: *mut *mut Self, command: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, directory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, parameters: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, windowstate: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExecuteShellCommand: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Frame: unsafe extern "system" fn(this: *mut *mut Self, frame: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Frame: usize,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ScopeTreeVisible: unsafe extern "system" fn(this: *mut *mut Self, visible: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ScopeTreeVisible: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetScopeTreeVisible: unsafe extern "system" fn(this: *mut *mut Self, visible: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetScopeTreeVisible: usize,
    pub Back: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Forward: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetStatusBarText: unsafe extern "system" fn(this: *mut *mut Self, statusbartext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetStatusBarText: usize,
    pub Memento: unsafe extern "system" fn(this: *mut *mut Self, memento: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ViewMemento: unsafe extern "system" fn(this: *mut *mut Self, memento: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ViewMemento: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Columns: unsafe extern "system" fn(this: *mut *mut Self, columns: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Columns: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_CellContents: unsafe extern "system" fn(this: *mut *mut Self, node: *mut ::core::ffi::c_void, column: i32, cellcontents: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_CellContents: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExportList: unsafe extern "system" fn(this: *mut *mut Self, file: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, exportoptions: _ExportListOptions) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExportList: usize,
    pub ListViewMode: unsafe extern "system" fn(this: *mut *mut Self, mode: *mut _ListViewMode) -> ::windows_sys::core::HRESULT,
    pub SetListViewMode: unsafe extern "system" fn(this: *mut *mut Self, mode: _ListViewMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ControlObject: unsafe extern "system" fn(this: *mut *mut Self, control: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ControlObject: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for View {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1862020514, data2: 45964, data3: 17790, data4: [154, 187, 237, 45, 24, 155, 140, 56] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct Views {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, view: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, node: *mut ::core::ffi::c_void, viewoptions: _ViewOptions) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for Views {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3602432669, data2: 41471, data3: 19826, data4: [170, 176, 227, 129, 233, 185, 51, 141] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _AppEvents {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub OnQuit: unsafe extern "system" fn(this: *mut *mut Self, application: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnQuit: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnDocumentOpen: unsafe extern "system" fn(this: *mut *mut Self, document: *mut ::core::ffi::c_void, new: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnDocumentOpen: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnDocumentClose: unsafe extern "system" fn(this: *mut *mut Self, document: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnDocumentClose: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnSnapInAdded: unsafe extern "system" fn(this: *mut *mut Self, document: *mut ::core::ffi::c_void, snapin: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnSnapInAdded: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnSnapInRemoved: unsafe extern "system" fn(this: *mut *mut Self, document: *mut ::core::ffi::c_void, snapin: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnSnapInRemoved: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnNewView: unsafe extern "system" fn(this: *mut *mut Self, view: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnNewView: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnViewClose: unsafe extern "system" fn(this: *mut *mut Self, view: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnViewClose: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnViewChange: unsafe extern "system" fn(this: *mut *mut Self, view: *mut ::core::ffi::c_void, newownernode: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnViewChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnSelectionChange: unsafe extern "system" fn(this: *mut *mut Self, view: *mut ::core::ffi::c_void, newnodes: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnSelectionChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnContextMenuExecuted: unsafe extern "system" fn(this: *mut *mut Self, menuitem: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnContextMenuExecuted: usize,
    pub OnToolbarButtonClicked: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OnListUpdated: unsafe extern "system" fn(this: *mut *mut Self, view: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnListUpdated: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for _AppEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3729181661, data2: 21493, data3: 17973, data4: [175, 84, 79, 231, 30, 146, 61, 63] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _Application {
    pub base__: super::Com::IDispatch,
    pub Help: unsafe extern "system" fn(this: *mut *mut Self),
    pub Quit: unsafe extern "system" fn(this: *mut *mut Self),
    #[cfg(feature = "Win32_System_Com")]
    pub Document: unsafe extern "system" fn(this: *mut *mut Self, document: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Document: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Load: unsafe extern "system" fn(this: *mut *mut Self, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Load: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Frame: unsafe extern "system" fn(this: *mut *mut Self, frame: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Frame: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Visible: unsafe extern "system" fn(this: *mut *mut Self, visible: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Visible: usize,
    pub Show: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Hide: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UserControl: unsafe extern "system" fn(this: *mut *mut Self, usercontrol: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserControl: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUserControl: unsafe extern "system" fn(this: *mut *mut Self, usercontrol: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUserControl: usize,
    pub VersionMajor: unsafe extern "system" fn(this: *mut *mut Self, versionmajor: *mut i32) -> ::windows_sys::core::HRESULT,
    pub VersionMinor: unsafe extern "system" fn(this: *mut *mut Self, versionminor: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for _Application {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2746202572, data2: 46675, data3: 18241, data4: [134, 171, 240, 71, 14, 193, 56, 76] };
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub type _ColumnSortOrder = i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SortOrder_Ascending: _ColumnSortOrder = 0i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SortOrder_Descending: _ColumnSortOrder = 1i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub type _DocumentMode = i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const DocumentMode_Author: _DocumentMode = 0i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const DocumentMode_User: _DocumentMode = 1i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const DocumentMode_User_MDI: _DocumentMode = 2i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const DocumentMode_User_SDI: _DocumentMode = 3i32;
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _EventConnector {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub ConnectTo: unsafe extern "system" fn(this: *mut *mut Self, application: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ConnectTo: usize,
    pub Disconnect: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for _EventConnector {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3233598768, data2: 56900, data3: 17704, data4: [132, 3, 160, 90, 106, 28, 200, 234] };
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub type _ExportListOptions = i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ExportListOptions_Default: _ExportListOptions = 0i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ExportListOptions_Unicode: _ExportListOptions = 1i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ExportListOptions_TabDelimited: _ExportListOptions = 2i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ExportListOptions_SelectedItemsOnly: _ExportListOptions = 4i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub type _ListViewMode = i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ListMode_Small_Icons: _ListViewMode = 0i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ListMode_Large_Icons: _ListViewMode = 1i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ListMode_List: _ListViewMode = 2i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ListMode_Detail: _ListViewMode = 3i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ListMode_Filtered: _ListViewMode = 4i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub type _ViewOptions = i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ViewOption_Default: _ViewOptions = 0i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ViewOption_ScopeTreeHidden: _ViewOptions = 1i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ViewOption_NoToolBars: _ViewOptions = 2i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ViewOption_NotPersistable: _ViewOptions = 4i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ViewOption_ActionPaneHidden: _ViewOptions = 8i32;
