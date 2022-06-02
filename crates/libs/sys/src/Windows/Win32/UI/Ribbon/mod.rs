#[repr(C)]
pub struct IUIApplication {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnViewChanged: unsafe extern "system" fn(this: *mut *mut Self, viewid: u32, typeid: UI_VIEWTYPE, view: *mut ::core::ffi::c_void, verb: UI_VIEWVERB, ureasoncode: i32) -> ::windows_sys::core::HRESULT,
    pub OnCreateUICommand: unsafe extern "system" fn(this: *mut *mut Self, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnDestroyUICommand: unsafe extern "system" fn(this: *mut *mut Self, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUICollection {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, count: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetItem: unsafe extern "system" fn(this: *mut *mut Self, index: u32, item: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Insert: unsafe extern "system" fn(this: *mut *mut Self, index: u32, item: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32) -> ::windows_sys::core::HRESULT,
    pub Replace: unsafe extern "system" fn(this: *mut *mut Self, indexreplaced: u32, itemreplacewith: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUICollectionChangedEvent {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnChanged: unsafe extern "system" fn(this: *mut *mut Self, action: UI_COLLECTIONCHANGE, oldindex: u32, olditem: *mut ::core::ffi::c_void, newindex: u32, newitem: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUICommandHandler {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub Execute: unsafe extern "system" fn(this: *mut *mut Self, commandid: u32, verb: UI_EXECUTIONVERB, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT, commandexecutionproperties: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    Execute: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub UpdateProperty: unsafe extern "system" fn(this: *mut *mut Self, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT, newvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    UpdateProperty: usize,
}
#[repr(C)]
pub struct IUIContextualUI {
    pub base__: ::windows_sys::core::IUnknown,
    pub ShowAtLocation: unsafe extern "system" fn(this: *mut *mut Self, x: i32, y: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUIEventLogger {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnUIEvent: unsafe extern "system" fn(this: *mut *mut Self, peventparams: *const UI_EVENTPARAMS),
}
#[repr(C)]
pub struct IUIEventingManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetEventLogger: unsafe extern "system" fn(this: *mut *mut Self, eventlogger: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUIFramework {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, framewnd: super::super::Foundation::HWND, application: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub Destroy: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LoadUI: unsafe extern "system" fn(this: *mut *mut Self, instance: super::super::Foundation::HINSTANCE, resourcename: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LoadUI: usize,
    pub GetView: unsafe extern "system" fn(this: *mut *mut Self, viewid: u32, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetUICommandProperty: unsafe extern "system" fn(this: *mut *mut Self, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetUICommandProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub SetUICommandProperty: unsafe extern "system" fn(this: *mut *mut Self, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    SetUICommandProperty: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub InvalidateUICommand: unsafe extern "system" fn(this: *mut *mut Self, commandid: u32, flags: UI_INVALIDATIONS, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    InvalidateUICommand: usize,
    pub FlushPendingInvalidations: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetModes: unsafe extern "system" fn(this: *mut *mut Self, imodes: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUIImage {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetBitmap: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetBitmap: usize,
}
#[repr(C)]
pub struct IUIImageFromBitmap {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CreateImage: unsafe extern "system" fn(this: *mut *mut Self, bitmap: super::super::Graphics::Gdi::HBITMAP, options: UI_OWNERSHIP, image: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CreateImage: usize,
}
#[repr(C)]
pub struct IUIRibbon {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetHeight: unsafe extern "system" fn(this: *mut *mut Self, cy: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub LoadSettingsFromStream: unsafe extern "system" fn(this: *mut *mut Self, pstream: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LoadSettingsFromStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SaveSettingsToStream: unsafe extern "system" fn(this: *mut *mut Self, pstream: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SaveSettingsToStream: usize,
}
#[repr(C)]
pub struct IUISimplePropertySet {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetValue: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetValue: usize,
}
pub const LIBID_UIRibbon: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2486121922, data2: 59451, data3: 17903, data4: [176, 133, 172, 41, 93, 214, 61, 91] };
pub const UIRibbonFramework: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2456242682, data2: 9749, data3: 18823, data4: [136, 69, 195, 62, 101, 242, 185, 87] };
pub const UIRibbonImageFromBitmapFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 259273910, data2: 22966, data3: 16976, data4: [153, 158, 209, 104, 214, 174, 66, 147] };
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_ALL_COMMANDS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub type UI_COLLECTIONCHANGE = i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COLLECTIONCHANGE_INSERT: UI_COLLECTIONCHANGE = 0i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COLLECTIONCHANGE_REMOVE: UI_COLLECTIONCHANGE = 1i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COLLECTIONCHANGE_REPLACE: UI_COLLECTIONCHANGE = 2i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COLLECTIONCHANGE_RESET: UI_COLLECTIONCHANGE = 3i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COLLECTION_INVALIDINDEX: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub type UI_COMMANDTYPE = i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COMMANDTYPE_UNKNOWN: UI_COMMANDTYPE = 0i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COMMANDTYPE_GROUP: UI_COMMANDTYPE = 1i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COMMANDTYPE_ACTION: UI_COMMANDTYPE = 2i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COMMANDTYPE_ANCHOR: UI_COMMANDTYPE = 3i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COMMANDTYPE_CONTEXT: UI_COMMANDTYPE = 4i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COMMANDTYPE_COLLECTION: UI_COMMANDTYPE = 5i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COMMANDTYPE_COMMANDCOLLECTION: UI_COMMANDTYPE = 6i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COMMANDTYPE_DECIMAL: UI_COMMANDTYPE = 7i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COMMANDTYPE_BOOLEAN: UI_COMMANDTYPE = 8i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COMMANDTYPE_FONT: UI_COMMANDTYPE = 9i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COMMANDTYPE_RECENTITEMS: UI_COMMANDTYPE = 10i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COMMANDTYPE_COLORANCHOR: UI_COMMANDTYPE = 11i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_COMMANDTYPE_COLORCOLLECTION: UI_COMMANDTYPE = 12i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub type UI_CONTEXTAVAILABILITY = i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_CONTEXTAVAILABILITY_NOTAVAILABLE: UI_CONTEXTAVAILABILITY = 0i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_CONTEXTAVAILABILITY_AVAILABLE: UI_CONTEXTAVAILABILITY = 1i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_CONTEXTAVAILABILITY_ACTIVE: UI_CONTEXTAVAILABILITY = 2i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub type UI_CONTROLDOCK = i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_CONTROLDOCK_TOP: UI_CONTROLDOCK = 1i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_CONTROLDOCK_BOTTOM: UI_CONTROLDOCK = 3i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub type UI_EVENTLOCATION = i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_EVENTLOCATION_Ribbon: UI_EVENTLOCATION = 0i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_EVENTLOCATION_QAT: UI_EVENTLOCATION = 1i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_EVENTLOCATION_ApplicationMenu: UI_EVENTLOCATION = 2i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_EVENTLOCATION_ContextPopup: UI_EVENTLOCATION = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub struct UI_EVENTPARAMS {
    pub EventType: UI_EVENTTYPE,
    pub Anonymous: UI_EVENTPARAMS_0,
}
impl ::core::marker::Copy for UI_EVENTPARAMS {}
impl ::core::clone::Clone for UI_EVENTPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub union UI_EVENTPARAMS_0 {
    pub Modes: i32,
    pub Params: UI_EVENTPARAMS_COMMAND,
}
impl ::core::marker::Copy for UI_EVENTPARAMS_0 {}
impl ::core::clone::Clone for UI_EVENTPARAMS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub struct UI_EVENTPARAMS_COMMAND {
    pub CommandID: u32,
    pub CommandName: ::windows_sys::core::PCWSTR,
    pub ParentCommandID: u32,
    pub ParentCommandName: ::windows_sys::core::PCWSTR,
    pub SelectionIndex: u32,
    pub Location: UI_EVENTLOCATION,
}
impl ::core::marker::Copy for UI_EVENTPARAMS_COMMAND {}
impl ::core::clone::Clone for UI_EVENTPARAMS_COMMAND {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub type UI_EVENTTYPE = i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_EVENTTYPE_ApplicationMenuOpened: UI_EVENTTYPE = 0i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_EVENTTYPE_RibbonMinimized: UI_EVENTTYPE = 1i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_EVENTTYPE_RibbonExpanded: UI_EVENTTYPE = 2i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_EVENTTYPE_ApplicationModeSwitched: UI_EVENTTYPE = 3i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_EVENTTYPE_TabActivated: UI_EVENTTYPE = 4i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_EVENTTYPE_MenuOpened: UI_EVENTTYPE = 5i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_EVENTTYPE_CommandExecuted: UI_EVENTTYPE = 6i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_EVENTTYPE_TooltipShown: UI_EVENTTYPE = 7i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub type UI_EXECUTIONVERB = i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_EXECUTIONVERB_EXECUTE: UI_EXECUTIONVERB = 0i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_EXECUTIONVERB_PREVIEW: UI_EXECUTIONVERB = 1i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_EXECUTIONVERB_CANCELPREVIEW: UI_EXECUTIONVERB = 2i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub type UI_FONTDELTASIZE = i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_FONTDELTASIZE_GROW: UI_FONTDELTASIZE = 0i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_FONTDELTASIZE_SHRINK: UI_FONTDELTASIZE = 1i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub type UI_FONTPROPERTIES = i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_FONTPROPERTIES_NOTAVAILABLE: UI_FONTPROPERTIES = 0i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_FONTPROPERTIES_NOTSET: UI_FONTPROPERTIES = 1i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_FONTPROPERTIES_SET: UI_FONTPROPERTIES = 2i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub type UI_FONTUNDERLINE = i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_FONTUNDERLINE_NOTAVAILABLE: UI_FONTUNDERLINE = 0i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_FONTUNDERLINE_NOTSET: UI_FONTUNDERLINE = 1i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_FONTUNDERLINE_SET: UI_FONTUNDERLINE = 2i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub type UI_FONTVERTICALPOSITION = i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_FONTVERTICALPOSITION_NOTAVAILABLE: UI_FONTVERTICALPOSITION = 0i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_FONTVERTICALPOSITION_NOTSET: UI_FONTVERTICALPOSITION = 1i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_FONTVERTICALPOSITION_SUPERSCRIPT: UI_FONTVERTICALPOSITION = 2i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_FONTVERTICALPOSITION_SUBSCRIPT: UI_FONTVERTICALPOSITION = 3i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub type UI_INVALIDATIONS = i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_INVALIDATIONS_STATE: UI_INVALIDATIONS = 1i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_INVALIDATIONS_VALUE: UI_INVALIDATIONS = 2i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_INVALIDATIONS_PROPERTY: UI_INVALIDATIONS = 4i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_INVALIDATIONS_ALLPROPERTIES: UI_INVALIDATIONS = 8i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub type UI_OWNERSHIP = i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_OWNERSHIP_TRANSFER: UI_OWNERSHIP = 0i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_OWNERSHIP_COPY: UI_OWNERSHIP = 1i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub type UI_SWATCHCOLORMODE = i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_SWATCHCOLORMODE_NORMAL: UI_SWATCHCOLORMODE = 0i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_SWATCHCOLORMODE_MONOCHROME: UI_SWATCHCOLORMODE = 1i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub type UI_SWATCHCOLORTYPE = i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_SWATCHCOLORTYPE_NOCOLOR: UI_SWATCHCOLORTYPE = 0i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_SWATCHCOLORTYPE_AUTOMATIC: UI_SWATCHCOLORTYPE = 1i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_SWATCHCOLORTYPE_RGB: UI_SWATCHCOLORTYPE = 2i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub type UI_VIEWTYPE = i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_VIEWTYPE_RIBBON: UI_VIEWTYPE = 1i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub type UI_VIEWVERB = i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_VIEWVERB_CREATE: UI_VIEWVERB = 0i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_VIEWVERB_DESTROY: UI_VIEWVERB = 1i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_VIEWVERB_SIZE: UI_VIEWVERB = 2i32;
#[doc = "*Required features: `\"Win32_UI_Ribbon\"`*"]
pub const UI_VIEWVERB_ERROR: UI_VIEWVERB = 3i32;
