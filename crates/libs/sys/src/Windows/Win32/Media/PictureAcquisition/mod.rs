#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub type DEVICE_SELECTION_DEVICE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DST_UNKNOWN_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DST_WPD_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DST_WIA_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DST_STI_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DSF_TWAIN_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DST_FS_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DST_DV_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DSF_ALL_DEVICES: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DSF_CPL_MODE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DSF_DV_DEVICES: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DSF_FS_DEVICES: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DSF_SHOW_OFFLINE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DSF_STI_DEVICES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DSF_TWAIN_DEVICES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DSF_WIA_CAMERAS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DSF_WIA_SCANNERS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DSF_WPD_DEVICES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub type ERROR_ADVISE_MESSAGE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_ERROR_SKIPRETRYCANCEL: ERROR_ADVISE_MESSAGE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_ERROR_RETRYCANCEL: ERROR_ADVISE_MESSAGE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_ERROR_YESNO: ERROR_ADVISE_MESSAGE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_ERROR_OK: ERROR_ADVISE_MESSAGE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub type ERROR_ADVISE_RESULT = i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_RESULT_YES: ERROR_ADVISE_RESULT = 0i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_RESULT_NO: ERROR_ADVISE_RESULT = 1i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_RESULT_OK: ERROR_ADVISE_RESULT = 2i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_RESULT_SKIP: ERROR_ADVISE_RESULT = 3i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_RESULT_SKIP_ALL: ERROR_ADVISE_RESULT = 4i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_RESULT_RETRY: ERROR_ADVISE_RESULT = 5i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_RESULT_ABORT: ERROR_ADVISE_RESULT = 6i32;
#[repr(C)]
pub struct IPhotoAcquire {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreatePhotoSource: unsafe extern "system" fn(this: *mut *mut Self, pszdevice: ::windows_sys::core::PCWSTR, ppphotoacquiresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Acquire: unsafe extern "system" fn(this: *mut *mut Self, pphotoacquiresource: *mut ::core::ffi::c_void, fshowprogress: super::super::Foundation::BOOL, hwndparent: super::super::Foundation::HWND, pszapplicationname: ::windows_sys::core::PCWSTR, pphotoacquireprogresscb: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Acquire: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumResults: unsafe extern "system" fn(this: *mut *mut Self, ppenumfilepaths: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumResults: usize,
}
#[repr(C)]
pub struct IPhotoAcquireDeviceSelectionDialog {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, psztitle: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SetSubmitButtonText: unsafe extern "system" fn(this: *mut *mut Self, pszsubmitbuttontext: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DoModal: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::Foundation::HWND, dwdeviceflags: u32, pbstrdeviceid: *mut super::super::Foundation::BSTR, pndevicetype: *mut DEVICE_SELECTION_DEVICE_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DoModal: usize,
}
#[repr(C)]
pub struct IPhotoAcquireItem {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetItemName: unsafe extern "system" fn(this: *mut *mut Self, pbstritemname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetItemName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub GetThumbnail: unsafe extern "system" fn(this: *mut *mut Self, sizethumbnail: super::super::Foundation::SIZE, phbmpthumbnail: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    GetThumbnail: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    SetProperty: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut *mut Self, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CanDelete: unsafe extern "system" fn(this: *mut *mut Self, pfcandelete: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CanDelete: usize,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetSubItemCount: unsafe extern "system" fn(this: *mut *mut Self, pncount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetSubItemAt: unsafe extern "system" fn(this: *mut *mut Self, nitemindex: u32, ppphotoacquireitem: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPhotoAcquireOptionsDialog {
    pub base__: ::windows_sys::core::IUnknown,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, pszregistryroot: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::Foundation::HWND, phwnddialog: *mut super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Create: usize,
    pub Destroy: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DoModal: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::Foundation::HWND, ppnreturncode: *mut isize) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DoModal: usize,
    pub SaveData: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPhotoAcquirePlugin {
    pub base__: ::windows_sys::core::IUnknown,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, pphotoacquiresource: *mut ::core::ffi::c_void, pphotoacquireprogresscb: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub ProcessItem: unsafe extern "system" fn(this: *mut *mut Self, dwacquirestage: u32, pphotoacquireitem: *mut ::core::ffi::c_void, poriginalitemstream: *mut ::core::ffi::c_void, pszfinalfilename: ::windows_sys::core::PCWSTR, ppropertystore: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem")))]
    ProcessItem: usize,
    pub TransferComplete: unsafe extern "system" fn(this: *mut *mut Self, hr: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayConfigureDialog: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayConfigureDialog: usize,
}
#[repr(C)]
pub struct IPhotoAcquireProgressCB {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Cancelled: unsafe extern "system" fn(this: *mut *mut Self, pfcancelled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Cancelled: usize,
    pub StartEnumeration: unsafe extern "system" fn(this: *mut *mut Self, pphotoacquiresource: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FoundItem: unsafe extern "system" fn(this: *mut *mut Self, pphotoacquireitem: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EndEnumeration: unsafe extern "system" fn(this: *mut *mut Self, hr: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub StartTransfer: unsafe extern "system" fn(this: *mut *mut Self, pphotoacquiresource: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StartItemTransfer: unsafe extern "system" fn(this: *mut *mut Self, nitemindex: u32, pphotoacquireitem: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DirectoryCreated: unsafe extern "system" fn(this: *mut *mut Self, pszdirectory: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UpdateTransferPercent: unsafe extern "system" fn(this: *mut *mut Self, foverall: super::super::Foundation::BOOL, npercent: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UpdateTransferPercent: usize,
    pub EndItemTransfer: unsafe extern "system" fn(this: *mut *mut Self, nitemindex: u32, pphotoacquireitem: *mut ::core::ffi::c_void, hr: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub EndTransfer: unsafe extern "system" fn(this: *mut *mut Self, hr: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub StartDelete: unsafe extern "system" fn(this: *mut *mut Self, pphotoacquiresource: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StartItemDelete: unsafe extern "system" fn(this: *mut *mut Self, nitemindex: u32, pphotoacquireitem: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UpdateDeletePercent: unsafe extern "system" fn(this: *mut *mut Self, npercent: u32) -> ::windows_sys::core::HRESULT,
    pub EndItemDelete: unsafe extern "system" fn(this: *mut *mut Self, nitemindex: u32, pphotoacquireitem: *mut ::core::ffi::c_void, hr: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub EndDelete: unsafe extern "system" fn(this: *mut *mut Self, hr: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub EndSession: unsafe extern "system" fn(this: *mut *mut Self, hr: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDeleteAfterAcquire: unsafe extern "system" fn(this: *mut *mut Self, pfdeleteafteracquire: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDeleteAfterAcquire: usize,
    pub ErrorAdvise: unsafe extern "system" fn(this: *mut *mut Self, hr: ::windows_sys::core::HRESULT, pszerrormessage: ::windows_sys::core::PCWSTR, nmessagetype: ERROR_ADVISE_MESSAGE_TYPE, pnerroradviseresult: *mut ERROR_ADVISE_RESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetUserInput: unsafe extern "system" fn(this: *mut *mut Self, riidtype: *const ::windows_sys::core::GUID, punknown: *mut ::core::ffi::c_void, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetUserInput: usize,
}
#[repr(C)]
pub struct IPhotoAcquireSettings {
    pub base__: ::windows_sys::core::IUnknown,
    pub InitializeFromRegistry: unsafe extern "system" fn(this: *mut *mut Self, pszregistrykey: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(this: *mut *mut Self, dwphotoacquireflags: u32) -> ::windows_sys::core::HRESULT,
    pub SetOutputFilenameTemplate: unsafe extern "system" fn(this: *mut *mut Self, psztemplate: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SetSequencePaddingWidth: unsafe extern "system" fn(this: *mut *mut Self, dwwidth: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSequenceZeroPadding: unsafe extern "system" fn(this: *mut *mut Self, fzeropad: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSequenceZeroPadding: usize,
    pub SetGroupTag: unsafe extern "system" fn(this: *mut *mut Self, pszgrouptag: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAcquisitionTime: unsafe extern "system" fn(this: *mut *mut Self, pftacquisitiontime: *const super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAcquisitionTime: usize,
    pub GetFlags: unsafe extern "system" fn(this: *mut *mut Self, pdwphotoacquireflags: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetOutputFilenameTemplate: unsafe extern "system" fn(this: *mut *mut Self, pbstrtemplate: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetOutputFilenameTemplate: usize,
    pub GetSequencePaddingWidth: unsafe extern "system" fn(this: *mut *mut Self, pdwwidth: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSequenceZeroPadding: unsafe extern "system" fn(this: *mut *mut Self, pfzeropad: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSequenceZeroPadding: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetGroupTag: unsafe extern "system" fn(this: *mut *mut Self, pbstrgrouptag: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetGroupTag: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAcquisitionTime: unsafe extern "system" fn(this: *mut *mut Self, pftacquisitiontime: *mut super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAcquisitionTime: usize,
}
#[repr(C)]
pub struct IPhotoAcquireSource {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFriendlyName: unsafe extern "system" fn(this: *mut *mut Self, pbstrfriendlyname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFriendlyName: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetDeviceIcons: unsafe extern "system" fn(this: *mut *mut Self, nsize: u32, phlargeicon: *mut super::super::UI::WindowsAndMessaging::HICON, phsmallicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetDeviceIcons: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeItemList: unsafe extern "system" fn(this: *mut *mut Self, fforceenumeration: super::super::Foundation::BOOL, pphotoacquireprogresscb: *mut ::core::ffi::c_void, pnitemcount: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeItemList: usize,
    pub GetItemCount: unsafe extern "system" fn(this: *mut *mut Self, pnitemcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetItemAt: unsafe extern "system" fn(this: *mut *mut Self, nindex: u32, ppphotoacquireitem: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPhotoAcquireSettings: unsafe extern "system" fn(this: *mut *mut Self, ppphotoacquiresettings: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDeviceId: unsafe extern "system" fn(this: *mut *mut Self, pbstrdeviceid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDeviceId: usize,
    pub BindToObject: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPhotoProgressActionCB {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub DoAction: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DoAction: usize,
}
#[repr(C)]
pub struct IPhotoProgressDialog {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Create: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWindow: unsafe extern "system" fn(this: *mut *mut Self, phwndprogressdialog: *mut super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWindow: usize,
    pub Destroy: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, psztitle: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowCheckbox: unsafe extern "system" fn(this: *mut *mut Self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fshow: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowCheckbox: usize,
    pub SetCheckboxText: unsafe extern "system" fn(this: *mut *mut Self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtext: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCheckboxCheck: unsafe extern "system" fn(this: *mut *mut Self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fchecked: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCheckboxCheck: usize,
    pub SetCheckboxTooltip: unsafe extern "system" fn(this: *mut *mut Self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtooltiptext: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsCheckboxChecked: unsafe extern "system" fn(this: *mut *mut Self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pfchecked: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsCheckboxChecked: usize,
    pub SetCaption: unsafe extern "system" fn(this: *mut *mut Self, psztitle: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub SetImage: unsafe extern "system" fn(this: *mut *mut Self, nimagetype: PROGRESS_DIALOG_IMAGE_TYPE, hicon: super::super::UI::WindowsAndMessaging::HICON, hbitmap: super::super::Graphics::Gdi::HBITMAP) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging")))]
    SetImage: usize,
    pub SetPercentComplete: unsafe extern "system" fn(this: *mut *mut Self, npercent: i32) -> ::windows_sys::core::HRESULT,
    pub SetProgressText: unsafe extern "system" fn(this: *mut *mut Self, pszprogresstext: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SetActionLinkCallback: unsafe extern "system" fn(this: *mut *mut Self, pphotoprogressactioncb: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetActionLinkText: unsafe extern "system" fn(this: *mut *mut Self, pszcaption: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowActionLink: unsafe extern "system" fn(this: *mut *mut Self, fshow: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowActionLink: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsCancelled: unsafe extern "system" fn(this: *mut *mut Self, pfcancelled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsCancelled: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetUserInput: unsafe extern "system" fn(this: *mut *mut Self, riidtype: *const ::windows_sys::core::GUID, punknown: *mut ::core::ffi::c_void, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetUserInput: usize,
}
#[repr(C)]
pub struct IUserInputString {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSubmitButtonText: unsafe extern "system" fn(this: *mut *mut Self, pbstrsubmitbuttontext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSubmitButtonText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPrompt: unsafe extern "system" fn(this: *mut *mut Self, pbstrprompttitle: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPrompt: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetStringId: unsafe extern "system" fn(this: *mut *mut Self, pbstrstringid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetStringId: usize,
    pub GetStringType: unsafe extern "system" fn(this: *mut *mut Self, pnstringtype: *mut USER_INPUT_STRING_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTooltipText: unsafe extern "system" fn(this: *mut *mut Self, pbstrtooltiptext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTooltipText: usize,
    pub GetMaxLength: unsafe extern "system" fn(this: *mut *mut Self, pcchmaxlength: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, pbstrdefault: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDefault: usize,
    pub GetMruCount: unsafe extern "system" fn(this: *mut *mut Self, pnmrucount: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMruEntryAt: unsafe extern "system" fn(this: *mut *mut Self, nindex: u32, pbstrmruentry: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMruEntryAt: usize,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub GetImage: unsafe extern "system" fn(this: *mut *mut Self, nsize: u32, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP, phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging")))]
    GetImage: usize,
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PAPS_CLEANUP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PAPS_POSTSAVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PAPS_PRESAVE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_ABORT_ON_SETTINGS_UPDATE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_DELETE_AFTER_ACQUIRE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_DISABLE_AUTO_ROTATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_DISABLE_DB_INTEGRATION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_DISABLE_DUPLICATE_DETECTION: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_DISABLE_GROUP_TAG_PROMPT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_DISABLE_METADATA_WRITE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_DISABLE_PLUGINS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_DISABLE_SETTINGS_LINK: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_DISABLE_THUMBNAIL_PROGRESS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_ENABLE_THUMBNAIL_CACHING: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_ERROR_RESTART_REQUIRED: ::windows_sys::core::HRESULT = -2147180543i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_IMPORT_VIDEO_AS_MULTIPLE_FILES: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_NO_GALLERY_LAUNCH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_RUN_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_CameraSequenceNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 15872887, data2: 31430, data3: 19322, data4: [132, 67, 52, 94, 115, 31, 165, 122] }, pid: 7u32 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_DuplicateDetectionID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 15872887, data2: 31430, data3: 19322, data4: [132, 67, 52, 94, 115, 31, 165, 122] }, pid: 10u32 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_FinalFilename: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 15872887, data2: 31430, data3: 19322, data4: [132, 67, 52, 94, 115, 31, 165, 122] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_GroupTag: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 15872887, data2: 31430, data3: 19322, data4: [132, 67, 52, 94, 115, 31, 165, 122] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_IntermediateFile: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 15872887, data2: 31430, data3: 19322, data4: [132, 67, 52, 94, 115, 31, 165, 122] }, pid: 8u32 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_OriginalFilename: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 15872887, data2: 31430, data3: 19322, data4: [132, 67, 52, 94, 115, 31, 165, 122] }, pid: 6u32 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_RelativePathname: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 15872887, data2: 31430, data3: 19322, data4: [132, 67, 52, 94, 115, 31, 165, 122] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_SkipImport: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 15872887, data2: 31430, data3: 19322, data4: [132, 67, 52, 94, 115, 31, 165, 122] }, pid: 9u32 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_TransferResult: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 15872887, data2: 31430, data3: 19322, data4: [132, 67, 52, 94, 115, 31, 165, 122] }, pid: 5u32 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub type PROGRESS_DIALOG_CHECKBOX_ID = i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PROGRESS_DIALOG_CHECKBOX_ID_DEFAULT: PROGRESS_DIALOG_CHECKBOX_ID = 0i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub type PROGRESS_DIALOG_IMAGE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PROGRESS_DIALOG_ICON_SMALL: PROGRESS_DIALOG_IMAGE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PROGRESS_DIALOG_ICON_LARGE: PROGRESS_DIALOG_IMAGE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PROGRESS_DIALOG_ICON_THUMBNAIL: PROGRESS_DIALOG_IMAGE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PROGRESS_DIALOG_BITMAP_THUMBNAIL: PROGRESS_DIALOG_IMAGE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PROGRESS_INDETERMINATE: i32 = -1i32;
pub const PhotoAcquire: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 15887874, data2: 59890, data3: 19103, data4: [159, 221, 90, 150, 47, 178, 106, 152] };
pub const PhotoAcquireAutoPlayDropTarget: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 15863477, data2: 36822, data3: 19869, data4: [183, 94, 54, 128, 23, 102, 200, 241] };
pub const PhotoAcquireAutoPlayHWEventHandler: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 15905843, data2: 17636, data3: 19848, data4: [178, 176, 38, 152, 160, 169, 29, 186] };
pub const PhotoAcquireDeviceSelectionDialog: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 15899188, data2: 47265, data3: 18476, data4: [188, 248, 58, 199, 176, 254, 143, 98] };
pub const PhotoAcquireOptionsDialog: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 15863969, data2: 25328, data3: 17291, data4: [159, 126, 150, 24, 215, 42, 24, 49] };
pub const PhotoProgressDialog: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 15879328, data2: 29839, data3: 20106, data4: [137, 79, 14, 3, 87, 198, 121, 159] };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub type USER_INPUT_STRING_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const USER_INPUT_DEFAULT: USER_INPUT_STRING_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const USER_INPUT_PATH_ELEMENT: USER_INPUT_STRING_TYPE = 1i32;
