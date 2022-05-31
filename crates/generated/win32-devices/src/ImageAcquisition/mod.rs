pub const ADVANCED_DUP: u32 = 8192u32;
pub const ADVANCED_DUPLEX: u32 = 1024u32;
pub const ALL_PAGES: u32 = 0u32;
pub const AUTO_ADVANCE: u32 = 512u32;
pub const AUTO_SOURCE: u32 = 32768u32;
pub const BACK_FIRST: u32 = 16u32;
pub const BACK_ONLY: u32 = 64u32;
pub const BARCODE_READER: u32 = 262144u32;
pub const BARCODE_READER_READY: u32 = 16384u32;
pub const BASE_VAL_WIA_ERROR: u32 = 0u32;
pub const BASE_VAL_WIA_SUCCESS: u32 = 0u32;
pub const BOTTOM_JUSTIFIED: u32 = 2u32;
pub const BUS_TYPE_FIREWIRE: u32 = 203u32;
pub const BUS_TYPE_PARALLEL: u32 = 202u32;
pub const BUS_TYPE_SCSI: u32 = 200u32;
pub const BUS_TYPE_USB: u32 = 201u32;
pub const CAPTUREMODE_BURST: u32 = 2u32;
pub const CAPTUREMODE_NORMAL: u32 = 1u32;
pub const CAPTUREMODE_TIMELAPSE: u32 = 3u32;
pub const CENTERED: u32 = 1u32;
pub const CFSTR_WIAITEMNAMES: &str = "WIAItemNames";
pub const CFSTR_WIAITEMPTR: &str = "WIAItemPointer";
pub const CLSID_WiaDefaultSegFilter: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd4f4d30b_0b29_4508_8922_0c5797d42765);
pub const CMD_GETADFAVAILABLE: u32 = 117u32;
pub const CMD_GETADFHASPAPER: u32 = 120u32;
pub const CMD_GETADFOPEN: u32 = 118u32;
pub const CMD_GETADFREADY: u32 = 119u32;
pub const CMD_GETADFSTATUS: u32 = 121u32;
pub const CMD_GETADFUNLOADREADY: u32 = 122u32;
pub const CMD_GETCAPABILITIES: u32 = 132u32;
pub const CMD_GETSUPPORTEDFILEFORMATS: u32 = 138u32;
pub const CMD_GETSUPPORTEDMEMORYFORMATS: u32 = 139u32;
pub const CMD_GETTPAAVAILABLE: u32 = 123u32;
pub const CMD_GETTPAOPENED: u32 = 124u32;
pub const CMD_GET_INTERRUPT_EVENT: u32 = 133u32;
pub const CMD_INITIALIZE: u32 = 100u32;
pub const CMD_LOAD_ADF: u32 = 115u32;
pub const CMD_RESETSCANNER: u32 = 131u32;
pub const CMD_SENDSCSICOMMAND: u32 = 127u32;
pub const CMD_SETCOLORDITHER: u32 = 111u32;
pub const CMD_SETCONTRAST: u32 = 104u32;
pub const CMD_SETDATATYPE: u32 = 106u32;
pub const CMD_SETDITHER: u32 = 107u32;
pub const CMD_SETFILTER: u32 = 114u32;
pub const CMD_SETFORMAT: u32 = 140u32;
pub const CMD_SETGSDNAME: u32 = 134u32;
pub const CMD_SETINTENSITY: u32 = 105u32;
pub const CMD_SETLAMP: u32 = 126u32;
pub const CMD_SETMATRIX: u32 = 112u32;
pub const CMD_SETMIRROR: u32 = 108u32;
pub const CMD_SETNEGATIVE: u32 = 109u32;
pub const CMD_SETSCANMODE: u32 = 135u32;
pub const CMD_SETSPEED: u32 = 113u32;
pub const CMD_SETSTIDEVICEHKEY: u32 = 136u32;
pub const CMD_SETTONEMAP: u32 = 110u32;
pub const CMD_SETXRESOLUTION: u32 = 102u32;
pub const CMD_SETYRESOLUTION: u32 = 103u32;
pub const CMD_STI_DEVICERESET: u32 = 128u32;
pub const CMD_STI_DIAGNOSTIC: u32 = 130u32;
pub const CMD_STI_GETSTATUS: u32 = 129u32;
pub const CMD_TPAREADY: u32 = 125u32;
pub const CMD_UNINITIALIZE: u32 = 101u32;
pub const CMD_UNLOAD_ADF: u32 = 116u32;
pub const COPY_PARENT_PROPERTY_VALUES: u32 = 1073741824u32;
pub const DETECT_DUP: u32 = 64u32;
pub const DETECT_DUP_AVAIL: u32 = 256u32;
pub const DETECT_FEED: u32 = 32u32;
pub const DETECT_FEED_AVAIL: u32 = 128u32;
pub const DETECT_FILM_TPA: u32 = 1024u32;
pub const DETECT_FLAT: u32 = 8u32;
pub const DETECT_SCAN: u32 = 16u32;
pub const DETECT_STOR: u32 = 4096u32;
#[repr(C)]
pub struct DEVICEDIALOGDATA {
    pub cbSize: u32,
    pub hwndParent: ::win32_foundation::HWND,
    pub pIWiaItemRoot: ::core::option::Option<IWiaItem>,
    pub dwFlags: u32,
    pub lIntent: i32,
    pub lItemCount: i32,
    pub ppWiaItems: *mut ::core::option::Option<IWiaItem>,
}
impl ::core::clone::Clone for DEVICEDIALOGDATA {
    fn clone(&self) -> Self {
        Self {
            cbSize: self.cbSize,
            hwndParent: self.hwndParent,
            pIWiaItemRoot: self.pIWiaItemRoot.clone(),
            dwFlags: self.dwFlags,
            lIntent: self.lIntent,
            lItemCount: self.lItemCount,
            ppWiaItems: self.ppWiaItems,
        }
    }
}
impl ::core::fmt::Debug for DEVICEDIALOGDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICEDIALOGDATA").field("cbSize", &self.cbSize).field("hwndParent", &self.hwndParent).field("pIWiaItemRoot", &self.pIWiaItemRoot).field("dwFlags", &self.dwFlags).field("lIntent", &self.lIntent).field("lItemCount", &self.lItemCount).field("ppWiaItems", &self.ppWiaItems).finish()
    }
}
unsafe impl ::windows_core::Abi for DEVICEDIALOGDATA {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for DEVICEDIALOGDATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.hwndParent == other.hwndParent && self.pIWiaItemRoot == other.pIWiaItemRoot && self.dwFlags == other.dwFlags && self.lIntent == other.lIntent && self.lItemCount == other.lItemCount && self.ppWiaItems == other.ppWiaItems
    }
}
impl ::core::cmp::Eq for DEVICEDIALOGDATA {}
impl ::core::default::Default for DEVICEDIALOGDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DEVICEDIALOGDATA2 {
    pub cbSize: u32,
    pub pIWiaItemRoot: ::core::option::Option<IWiaItem2>,
    pub dwFlags: u32,
    pub hwndParent: ::win32_foundation::HWND,
    pub bstrFolderName: ::win32_foundation::BSTR,
    pub bstrFilename: ::win32_foundation::BSTR,
    pub lNumFiles: i32,
    pub pbstrFilePaths: *mut ::win32_foundation::BSTR,
    pub pWiaItem: ::core::option::Option<IWiaItem2>,
}
impl ::core::clone::Clone for DEVICEDIALOGDATA2 {
    fn clone(&self) -> Self {
        Self {
            cbSize: self.cbSize,
            pIWiaItemRoot: self.pIWiaItemRoot.clone(),
            dwFlags: self.dwFlags,
            hwndParent: self.hwndParent,
            bstrFolderName: self.bstrFolderName.clone(),
            bstrFilename: self.bstrFilename.clone(),
            lNumFiles: self.lNumFiles,
            pbstrFilePaths: self.pbstrFilePaths,
            pWiaItem: self.pWiaItem.clone(),
        }
    }
}
impl ::core::fmt::Debug for DEVICEDIALOGDATA2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICEDIALOGDATA2").field("cbSize", &self.cbSize).field("pIWiaItemRoot", &self.pIWiaItemRoot).field("dwFlags", &self.dwFlags).field("hwndParent", &self.hwndParent).field("bstrFolderName", &self.bstrFolderName).field("bstrFilename", &self.bstrFilename).field("lNumFiles", &self.lNumFiles).field("pbstrFilePaths", &self.pbstrFilePaths).field("pWiaItem", &self.pWiaItem).finish()
    }
}
unsafe impl ::windows_core::Abi for DEVICEDIALOGDATA2 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for DEVICEDIALOGDATA2 {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pIWiaItemRoot == other.pIWiaItemRoot && self.dwFlags == other.dwFlags && self.hwndParent == other.hwndParent && self.bstrFolderName == other.bstrFolderName && self.bstrFilename == other.bstrFilename && self.lNumFiles == other.lNumFiles && self.pbstrFilePaths == other.pbstrFilePaths && self.pWiaItem == other.pWiaItem
    }
}
impl ::core::cmp::Eq for DEVICEDIALOGDATA2 {}
impl ::core::default::Default for DEVICEDIALOGDATA2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DEVICE_ATTENTION: u32 = 1024u32;
pub const DUP: u32 = 4u32;
pub const DUPLEX: u32 = 4u32;
pub const DUP_READY: u32 = 4u32;
pub type DeviceDialogFunction = ::core::option::Option<unsafe extern "system" fn(param0: *mut DEVICEDIALOGDATA) -> ::windows_core::HRESULT>;
pub const EFFECTMODE_BW: u32 = 2u32;
pub const EFFECTMODE_SEPIA: u32 = 3u32;
pub const EFFECTMODE_STANDARD: u32 = 1u32;
pub const ENDORSER: u32 = 131072u32;
pub const ENDORSER_READY: u32 = 8192u32;
pub const ESC_TWAIN_CAPABILITY: u32 = 2001u32;
pub const ESC_TWAIN_PRIVATE_SUPPORTED_CAPS: u32 = 2002u32;
pub const EXPOSUREMETERING_AVERAGE: u32 = 1u32;
pub const EXPOSUREMETERING_CENTERSPOT: u32 = 4u32;
pub const EXPOSUREMETERING_CENTERWEIGHT: u32 = 2u32;
pub const EXPOSUREMETERING_MULTISPOT: u32 = 3u32;
pub const EXPOSUREMODE_APERTURE_PRIORITY: u32 = 3u32;
pub const EXPOSUREMODE_AUTO: u32 = 2u32;
pub const EXPOSUREMODE_MANUAL: u32 = 1u32;
pub const EXPOSUREMODE_PORTRAIT: u32 = 7u32;
pub const EXPOSUREMODE_PROGRAM_ACTION: u32 = 6u32;
pub const EXPOSUREMODE_PROGRAM_CREATIVE: u32 = 5u32;
pub const EXPOSUREMODE_SHUTTER_PRIORITY: u32 = 4u32;
pub const FEED: u32 = 1u32;
pub const FEEDER: u32 = 1u32;
pub const FEED_READY: u32 = 1u32;
pub const FILM_TPA: u32 = 512u32;
pub const FILM_TPA_READY: u32 = 64u32;
pub const FLASHMODE_AUTO: u32 = 1u32;
pub const FLASHMODE_EXTERNALSYNC: u32 = 6u32;
pub const FLASHMODE_FILL: u32 = 3u32;
pub const FLASHMODE_OFF: u32 = 2u32;
pub const FLASHMODE_REDEYE_AUTO: u32 = 4u32;
pub const FLASHMODE_REDEYE_FILL: u32 = 5u32;
pub const FLAT: u32 = 2u32;
pub const FLATBED: u32 = 2u32;
pub const FLAT_COVER_UP: u32 = 8u32;
pub const FLAT_READY: u32 = 2u32;
pub const FOCUSMETERING_CENTERSPOT: u32 = 1u32;
pub const FOCUSMETERING_MULTISPOT: u32 = 2u32;
pub const FOCUSMODE_AUTO: u32 = 2u32;
pub const FOCUSMODE_MACROAUTO: u32 = 3u32;
pub const FOCUSMODE_MANUAL: u32 = 1u32;
pub const FRONT_FIRST: u32 = 8u32;
pub const FRONT_ONLY: u32 = 32u32;
pub const GUID_DEVINTERFACE_IMAGE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6bdd1fc6_810f_11d0_bec7_08002be2092f);
#[repr(transparent)]
pub struct IEnumWIA_DEV_CAPS(::windows_core::IUnknown);
impl IEnumWIA_DEV_CAPS {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut WIA_DEV_CAP, pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumWIA_DEV_CAPS> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumWIA_DEV_CAPS>(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IEnumWIA_DEV_CAPS> for ::windows_core::IUnknown {
    fn from(value: IEnumWIA_DEV_CAPS) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumWIA_DEV_CAPS> for ::windows_core::IUnknown {
    fn from(value: &IEnumWIA_DEV_CAPS) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumWIA_DEV_CAPS {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumWIA_DEV_CAPS {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumWIA_DEV_CAPS {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumWIA_DEV_CAPS {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumWIA_DEV_CAPS {}
impl ::core::fmt::Debug for IEnumWIA_DEV_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumWIA_DEV_CAPS").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumWIA_DEV_CAPS {
    type Vtable = IEnumWIA_DEV_CAPS_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1fcc4287_aca6_11d2_a093_00c04f72dc3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWIA_DEV_CAPS_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut WIA_DEV_CAP, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumWIA_DEV_INFO(::windows_core::IUnknown);
impl IEnumWIA_DEV_INFO {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IWiaPropertyStorage>, pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumWIA_DEV_INFO> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumWIA_DEV_INFO>(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IEnumWIA_DEV_INFO> for ::windows_core::IUnknown {
    fn from(value: IEnumWIA_DEV_INFO) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumWIA_DEV_INFO> for ::windows_core::IUnknown {
    fn from(value: &IEnumWIA_DEV_INFO) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumWIA_DEV_INFO {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumWIA_DEV_INFO {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumWIA_DEV_INFO {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumWIA_DEV_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumWIA_DEV_INFO {}
impl ::core::fmt::Debug for IEnumWIA_DEV_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumWIA_DEV_INFO").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumWIA_DEV_INFO {
    type Vtable = IEnumWIA_DEV_INFO_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e38b83c_8cf1_11d1_bf92_0060081ed811);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWIA_DEV_INFO_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows_core::RawPtr, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumWIA_FORMAT_INFO(::windows_core::IUnknown);
impl IEnumWIA_FORMAT_INFO {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut WIA_FORMAT_INFO, pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumWIA_FORMAT_INFO> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumWIA_FORMAT_INFO>(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IEnumWIA_FORMAT_INFO> for ::windows_core::IUnknown {
    fn from(value: IEnumWIA_FORMAT_INFO) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumWIA_FORMAT_INFO> for ::windows_core::IUnknown {
    fn from(value: &IEnumWIA_FORMAT_INFO) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumWIA_FORMAT_INFO {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumWIA_FORMAT_INFO {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumWIA_FORMAT_INFO {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumWIA_FORMAT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumWIA_FORMAT_INFO {}
impl ::core::fmt::Debug for IEnumWIA_FORMAT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumWIA_FORMAT_INFO").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumWIA_FORMAT_INFO {
    type Vtable = IEnumWIA_FORMAT_INFO_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x81befc5b_656d_44f1_b24c_d41d51b4dc81);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWIA_FORMAT_INFO_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut WIA_FORMAT_INFO, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumWiaItem(::windows_core::IUnknown);
impl IEnumWiaItem {
    pub unsafe fn Next(&self, celt: u32, ppiwiaitem: *mut ::core::option::Option<IWiaItem>, pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt), ::core::mem::transmute(ppiwiaitem), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumWiaItem> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumWiaItem>(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IEnumWiaItem> for ::windows_core::IUnknown {
    fn from(value: IEnumWiaItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumWiaItem> for ::windows_core::IUnknown {
    fn from(value: &IEnumWiaItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumWiaItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumWiaItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumWiaItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumWiaItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumWiaItem {}
impl ::core::fmt::Debug for IEnumWiaItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumWiaItem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumWiaItem {
    type Vtable = IEnumWiaItem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e8383fc_3391_11d2_9a33_00c04fa36145);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWiaItem_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppiwiaitem: *mut ::windows_core::RawPtr, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumWiaItem2(::windows_core::IUnknown);
impl IEnumWiaItem2 {
    pub unsafe fn Next(&self, celt: u32, ppiwiaitem2: *mut ::core::option::Option<IWiaItem2>, pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt), ::core::mem::transmute(ppiwiaitem2), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumWiaItem2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumWiaItem2>(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IEnumWiaItem2> for ::windows_core::IUnknown {
    fn from(value: IEnumWiaItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumWiaItem2> for ::windows_core::IUnknown {
    fn from(value: &IEnumWiaItem2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumWiaItem2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumWiaItem2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumWiaItem2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumWiaItem2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumWiaItem2 {}
impl ::core::fmt::Debug for IEnumWiaItem2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumWiaItem2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumWiaItem2 {
    type Vtable = IEnumWiaItem2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x59970af4_cd0d_44d9_ab24_52295630e582);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWiaItem2_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppiwiaitem2: *mut ::windows_core::RawPtr, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: *mut u32) -> ::windows_core::HRESULT,
}
pub const IMPRINTER: u32 = 65536u32;
pub const IMPRINTER_READY: u32 = 4096u32;
pub const IT_MSG_DATA: u32 = 2u32;
pub const IT_MSG_DATA_HEADER: u32 = 1u32;
pub const IT_MSG_FILE_PREVIEW_DATA: u32 = 6u32;
pub const IT_MSG_FILE_PREVIEW_DATA_HEADER: u32 = 7u32;
pub const IT_MSG_NEW_PAGE: u32 = 5u32;
pub const IT_MSG_STATUS: u32 = 3u32;
pub const IT_MSG_TERMINATION: u32 = 4u32;
pub const IT_STATUS_MASK: u32 = 7u32;
pub const IT_STATUS_PROCESSING_DATA: u32 = 2u32;
pub const IT_STATUS_TRANSFER_FROM_DEVICE: u32 = 1u32;
pub const IT_STATUS_TRANSFER_TO_CLIENT: u32 = 4u32;
#[repr(transparent)]
pub struct IWiaAppErrorHandler(::windows_core::IUnknown);
impl IWiaAppErrorHandler {
    pub unsafe fn GetWindow(&self) -> ::windows_core::Result<::win32_foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::HWND>::zeroed();
        (::windows_core::Interface::vtable(self).GetWindow)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::HWND>(result__)
    }
    pub unsafe fn ReportStatus<'a, Param1: ::windows_core::IntoParam<'a, IWiaItem2>>(&self, lflags: i32, pwiaitem2: Param1, hrstatus: ::windows_core::HRESULT, lpercentcomplete: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReportStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), pwiaitem2.into_param().abi(), ::core::mem::transmute(hrstatus), ::core::mem::transmute(lpercentcomplete)).ok()
    }
}
impl ::core::convert::From<IWiaAppErrorHandler> for ::windows_core::IUnknown {
    fn from(value: IWiaAppErrorHandler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaAppErrorHandler> for ::windows_core::IUnknown {
    fn from(value: &IWiaAppErrorHandler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWiaAppErrorHandler {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWiaAppErrorHandler {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaAppErrorHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaAppErrorHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaAppErrorHandler {}
impl ::core::fmt::Debug for IWiaAppErrorHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaAppErrorHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWiaAppErrorHandler {
    type Vtable = IWiaAppErrorHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6c16186c_d0a6_400c_80f4_d26986a0e734);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaAppErrorHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut ::win32_foundation::HWND) -> ::windows_core::HRESULT,
    pub ReportStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pwiaitem2: ::windows_core::RawPtr, hrstatus: ::windows_core::HRESULT, lpercentcomplete: i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWiaDataCallback(::windows_core::IUnknown);
impl IWiaDataCallback {
    pub unsafe fn BandedDataCallback(&self, lmessage: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, lreserved: i32, lreslength: i32, pbbuffer: *mut u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BandedDataCallback)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lmessage), ::core::mem::transmute(lstatus), ::core::mem::transmute(lpercentcomplete), ::core::mem::transmute(loffset), ::core::mem::transmute(llength), ::core::mem::transmute(lreserved), ::core::mem::transmute(lreslength), ::core::mem::transmute(pbbuffer)).ok()
    }
}
impl ::core::convert::From<IWiaDataCallback> for ::windows_core::IUnknown {
    fn from(value: IWiaDataCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaDataCallback> for ::windows_core::IUnknown {
    fn from(value: &IWiaDataCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWiaDataCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWiaDataCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaDataCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaDataCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaDataCallback {}
impl ::core::fmt::Debug for IWiaDataCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaDataCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWiaDataCallback {
    type Vtable = IWiaDataCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa558a866_a5b0_11d2_a08f_00c04f72dc3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaDataCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub BandedDataCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmessage: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, lreserved: i32, lreslength: i32, pbbuffer: *mut u8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWiaDataTransfer(::windows_core::IUnknown);
impl IWiaDataTransfer {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn idtGetData<'a, Param1: ::windows_core::IntoParam<'a, IWiaDataCallback>>(&self, pmedium: *mut ::win32_system::Com::STGMEDIUM, piwiadatacallback: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).idtGetData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmedium), piwiadatacallback.into_param().abi()).ok()
    }
    pub unsafe fn idtGetBandedData<'a, Param1: ::windows_core::IntoParam<'a, IWiaDataCallback>>(&self, pwiadatatransinfo: *mut WIA_DATA_TRANSFER_INFO, piwiadatacallback: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).idtGetBandedData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwiadatatransinfo), piwiadatacallback.into_param().abi()).ok()
    }
    pub unsafe fn idtQueryGetData(&self, pfe: *const WIA_FORMAT_INFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).idtQueryGetData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pfe)).ok()
    }
    pub unsafe fn idtEnumWIA_FORMAT_INFO(&self) -> ::windows_core::Result<IEnumWIA_FORMAT_INFO> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).idtEnumWIA_FORMAT_INFO)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumWIA_FORMAT_INFO>(result__)
    }
    pub unsafe fn idtGetExtendedTransferInfo(&self) -> ::windows_core::Result<WIA_EXTENDED_TRANSFER_INFO> {
        let mut result__ = ::core::mem::MaybeUninit::<WIA_EXTENDED_TRANSFER_INFO>::zeroed();
        (::windows_core::Interface::vtable(self).idtGetExtendedTransferInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WIA_EXTENDED_TRANSFER_INFO>(result__)
    }
}
impl ::core::convert::From<IWiaDataTransfer> for ::windows_core::IUnknown {
    fn from(value: IWiaDataTransfer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaDataTransfer> for ::windows_core::IUnknown {
    fn from(value: &IWiaDataTransfer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWiaDataTransfer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWiaDataTransfer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaDataTransfer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaDataTransfer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaDataTransfer {}
impl ::core::fmt::Debug for IWiaDataTransfer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaDataTransfer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWiaDataTransfer {
    type Vtable = IWiaDataTransfer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa6cef998_a5b0_11d2_a08f_00c04f72dc3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaDataTransfer_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub idtGetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmedium: *mut ::win32_system::Com::STGMEDIUM, piwiadatacallback: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    idtGetData: usize,
    pub idtGetBandedData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwiadatatransinfo: *mut WIA_DATA_TRANSFER_INFO, piwiadatacallback: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub idtQueryGetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfe: *const WIA_FORMAT_INFO) -> ::windows_core::HRESULT,
    pub idtEnumWIA_FORMAT_INFO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub idtGetExtendedTransferInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pextendedtransferinfo: *mut WIA_EXTENDED_TRANSFER_INFO) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWiaDevMgr(::windows_core::IUnknown);
impl IWiaDevMgr {
    pub unsafe fn EnumDeviceInfo(&self, lflag: i32) -> ::windows_core::Result<IEnumWIA_DEV_INFO> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumDeviceInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflag), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumWIA_DEV_INFO>(result__)
    }
    pub unsafe fn CreateDevice<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdeviceid: Param0) -> ::windows_core::Result<IWiaItem> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateDevice)(::windows_core::Interface::as_raw(self), bstrdeviceid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWiaItem>(result__)
    }
    pub unsafe fn SelectDeviceDlg<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(&self, hwndparent: Param0, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut ::win32_foundation::BSTR, ppitemroot: *mut ::core::option::Option<IWiaItem>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SelectDeviceDlg)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), ::core::mem::transmute(ldevicetype), ::core::mem::transmute(lflags), ::core::mem::transmute(pbstrdeviceid), ::core::mem::transmute(ppitemroot)).ok()
    }
    pub unsafe fn SelectDeviceDlgID<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(&self, hwndparent: Param0, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SelectDeviceDlgID)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), ::core::mem::transmute(ldevicetype), ::core::mem::transmute(lflags), ::core::mem::transmute(pbstrdeviceid)).ok()
    }
    pub unsafe fn GetImageDlg<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param4: ::windows_core::IntoParam<'a, IWiaItem>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, hwndparent: Param0, ldevicetype: i32, lflags: i32, lintent: i32, pitemroot: Param4, bstrfilename: Param5, pguidformat: *mut ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetImageDlg)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), ::core::mem::transmute(ldevicetype), ::core::mem::transmute(lflags), ::core::mem::transmute(lintent), pitemroot.into_param().abi(), bstrfilename.into_param().abi(), ::core::mem::transmute(pguidformat)).ok()
    }
    pub unsafe fn RegisterEventCallbackProgram<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param6: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, lflags: i32, bstrdeviceid: Param1, peventguid: *const ::windows_core::GUID, bstrcommandline: Param3, bstrname: Param4, bstrdescription: Param5, bstricon: Param6) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterEventCallbackProgram)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), bstrdeviceid.into_param().abi(), ::core::mem::transmute(peventguid), bstrcommandline.into_param().abi(), bstrname.into_param().abi(), bstrdescription.into_param().abi(), bstricon.into_param().abi()).ok()
    }
    pub unsafe fn RegisterEventCallbackInterface<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, IWiaEventCallback>>(&self, lflags: i32, bstrdeviceid: Param1, peventguid: *const ::windows_core::GUID, piwiaeventcallback: Param3) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).RegisterEventCallbackInterface)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), bstrdeviceid.into_param().abi(), ::core::mem::transmute(peventguid), piwiaeventcallback.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn RegisterEventCallbackCLSID<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param6: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, lflags: i32, bstrdeviceid: Param1, peventguid: *const ::windows_core::GUID, pclsid: *const ::windows_core::GUID, bstrname: Param4, bstrdescription: Param5, bstricon: Param6) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterEventCallbackCLSID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), bstrdeviceid.into_param().abi(), ::core::mem::transmute(peventguid), ::core::mem::transmute(pclsid), bstrname.into_param().abi(), bstrdescription.into_param().abi(), bstricon.into_param().abi()).ok()
    }
    pub unsafe fn AddDeviceDlg<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(&self, hwndparent: Param0, lflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddDeviceDlg)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), ::core::mem::transmute(lflags)).ok()
    }
}
impl ::core::convert::From<IWiaDevMgr> for ::windows_core::IUnknown {
    fn from(value: IWiaDevMgr) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaDevMgr> for ::windows_core::IUnknown {
    fn from(value: &IWiaDevMgr) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWiaDevMgr {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWiaDevMgr {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaDevMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaDevMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaDevMgr {}
impl ::core::fmt::Debug for IWiaDevMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaDevMgr").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWiaDevMgr {
    type Vtable = IWiaDevMgr_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5eb2502a_8cf1_11d1_bf92_0060081ed811);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaDevMgr_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub EnumDeviceInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflag: i32, ppienum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdeviceid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppwiaitemroot: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectDeviceDlg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: ::win32_foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut ::win32_foundation::BSTR, ppitemroot: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectDeviceDlgID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: ::win32_foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetImageDlg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: ::win32_foundation::HWND, ldevicetype: i32, lflags: i32, lintent: i32, pitemroot: ::windows_core::RawPtr, bstrfilename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pguidformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub RegisterEventCallbackProgram: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, peventguid: *const ::windows_core::GUID, bstrcommandline: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrdescription: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstricon: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub RegisterEventCallbackInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, peventguid: *const ::windows_core::GUID, piwiaeventcallback: ::windows_core::RawPtr, peventobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RegisterEventCallbackCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, peventguid: *const ::windows_core::GUID, pclsid: *const ::windows_core::GUID, bstrname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrdescription: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstricon: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub AddDeviceDlg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: ::win32_foundation::HWND, lflags: i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWiaDevMgr2(::windows_core::IUnknown);
impl IWiaDevMgr2 {
    pub unsafe fn EnumDeviceInfo(&self, lflags: i32) -> ::windows_core::Result<IEnumWIA_DEV_INFO> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumDeviceInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumWIA_DEV_INFO>(result__)
    }
    pub unsafe fn CreateDevice<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, lflags: i32, bstrdeviceid: Param1) -> ::windows_core::Result<IWiaItem2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateDevice)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), bstrdeviceid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWiaItem2>(result__)
    }
    pub unsafe fn SelectDeviceDlg<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(&self, hwndparent: Param0, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut ::win32_foundation::BSTR, ppitemroot: *mut ::core::option::Option<IWiaItem2>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SelectDeviceDlg)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), ::core::mem::transmute(ldevicetype), ::core::mem::transmute(lflags), ::core::mem::transmute(pbstrdeviceid), ::core::mem::transmute(ppitemroot)).ok()
    }
    pub unsafe fn SelectDeviceDlgID<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(&self, hwndparent: Param0, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SelectDeviceDlgID)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), ::core::mem::transmute(ldevicetype), ::core::mem::transmute(lflags), ::core::mem::transmute(pbstrdeviceid)).ok()
    }
    pub unsafe fn RegisterEventCallbackInterface<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, IWiaEventCallback>>(&self, lflags: i32, bstrdeviceid: Param1, peventguid: *const ::windows_core::GUID, piwiaeventcallback: Param3) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).RegisterEventCallbackInterface)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), bstrdeviceid.into_param().abi(), ::core::mem::transmute(peventguid), piwiaeventcallback.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn RegisterEventCallbackProgram<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param6: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param7: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, lflags: i32, bstrdeviceid: Param1, peventguid: *const ::windows_core::GUID, bstrfullappname: Param3, bstrcommandlinearg: Param4, bstrname: Param5, bstrdescription: Param6, bstricon: Param7) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterEventCallbackProgram)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), bstrdeviceid.into_param().abi(), ::core::mem::transmute(peventguid), bstrfullappname.into_param().abi(), bstrcommandlinearg.into_param().abi(), bstrname.into_param().abi(), bstrdescription.into_param().abi(), bstricon.into_param().abi()).ok()
    }
    pub unsafe fn RegisterEventCallbackCLSID<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param6: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, lflags: i32, bstrdeviceid: Param1, peventguid: *const ::windows_core::GUID, pclsid: *const ::windows_core::GUID, bstrname: Param4, bstrdescription: Param5, bstricon: Param6) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterEventCallbackCLSID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), bstrdeviceid.into_param().abi(), ::core::mem::transmute(peventguid), ::core::mem::transmute(pclsid), bstrname.into_param().abi(), bstrdescription.into_param().abi(), bstricon.into_param().abi()).ok()
    }
    pub unsafe fn GetImageDlg<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, lflags: i32, bstrdeviceid: Param1, hwndparent: Param2, bstrfoldername: Param3, bstrfilename: Param4, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut ::win32_foundation::BSTR, ppitem: *mut ::core::option::Option<IWiaItem2>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetImageDlg)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), bstrdeviceid.into_param().abi(), hwndparent.into_param().abi(), bstrfoldername.into_param().abi(), bstrfilename.into_param().abi(), ::core::mem::transmute(plnumfiles), ::core::mem::transmute(ppbstrfilepaths), ::core::mem::transmute(ppitem)).ok()
    }
}
impl ::core::convert::From<IWiaDevMgr2> for ::windows_core::IUnknown {
    fn from(value: IWiaDevMgr2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaDevMgr2> for ::windows_core::IUnknown {
    fn from(value: &IWiaDevMgr2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWiaDevMgr2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWiaDevMgr2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaDevMgr2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaDevMgr2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaDevMgr2 {}
impl ::core::fmt::Debug for IWiaDevMgr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaDevMgr2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWiaDevMgr2 {
    type Vtable = IWiaDevMgr2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79c07cf1_cbdd_41ee_8ec3_f00080cada7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaDevMgr2_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub EnumDeviceInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, ppienum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppwiaitem2root: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectDeviceDlg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: ::win32_foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut ::win32_foundation::BSTR, ppitemroot: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectDeviceDlgID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: ::win32_foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub RegisterEventCallbackInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, peventguid: *const ::windows_core::GUID, piwiaeventcallback: ::windows_core::RawPtr, peventobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RegisterEventCallbackProgram: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, peventguid: *const ::windows_core::GUID, bstrfullappname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrcommandlinearg: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrdescription: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstricon: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub RegisterEventCallbackCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, peventguid: *const ::windows_core::GUID, pclsid: *const ::windows_core::GUID, bstrname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrdescription: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstricon: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub GetImageDlg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, hwndparent: ::win32_foundation::HWND, bstrfoldername: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrfilename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut ::win32_foundation::BSTR, ppitem: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWiaDrvItem(::windows_core::IUnknown);
impl IWiaDrvItem {
    pub unsafe fn GetItemFlags(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).GetItemFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn GetDeviceSpecContext(&self) -> ::windows_core::Result<*mut u8> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut u8>::zeroed();
        (::windows_core::Interface::vtable(self).GetDeviceSpecContext)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut u8>(result__)
    }
    pub unsafe fn GetFullItemName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetFullItemName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetItemName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetItemName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn AddItemToFolder<'a, Param0: ::windows_core::IntoParam<'a, IWiaDrvItem>>(&self, __midl__iwiadrvitem0004: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddItemToFolder)(::windows_core::Interface::as_raw(self), __midl__iwiadrvitem0004.into_param().abi()).ok()
    }
    pub unsafe fn UnlinkItemTree(&self, __midl__iwiadrvitem0005: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnlinkItemTree)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(__midl__iwiadrvitem0005)).ok()
    }
    pub unsafe fn RemoveItemFromFolder(&self, __midl__iwiadrvitem0006: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveItemFromFolder)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(__midl__iwiadrvitem0006)).ok()
    }
    pub unsafe fn FindItemByName<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, __midl__iwiadrvitem0007: i32, __midl__iwiadrvitem0008: Param1) -> ::windows_core::Result<IWiaDrvItem> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).FindItemByName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(__midl__iwiadrvitem0007), __midl__iwiadrvitem0008.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWiaDrvItem>(result__)
    }
    pub unsafe fn FindChildItemByName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, __midl__iwiadrvitem0010: Param0) -> ::windows_core::Result<IWiaDrvItem> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).FindChildItemByName)(::windows_core::Interface::as_raw(self), __midl__iwiadrvitem0010.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWiaDrvItem>(result__)
    }
    pub unsafe fn GetParentItem(&self) -> ::windows_core::Result<IWiaDrvItem> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetParentItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWiaDrvItem>(result__)
    }
    pub unsafe fn GetFirstChildItem(&self) -> ::windows_core::Result<IWiaDrvItem> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetFirstChildItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWiaDrvItem>(result__)
    }
    pub unsafe fn GetNextSiblingItem(&self) -> ::windows_core::Result<IWiaDrvItem> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetNextSiblingItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWiaDrvItem>(result__)
    }
    pub unsafe fn DumpItemData(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DumpItemData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IWiaDrvItem> for ::windows_core::IUnknown {
    fn from(value: IWiaDrvItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaDrvItem> for ::windows_core::IUnknown {
    fn from(value: &IWiaDrvItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWiaDrvItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWiaDrvItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaDrvItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaDrvItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaDrvItem {}
impl ::core::fmt::Debug for IWiaDrvItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaDrvItem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWiaDrvItem {
    type Vtable = IWiaDrvItem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1f02b5c5_b00c_11d2_a094_00c04f72dc3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaDrvItem_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetItemFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0000: *mut i32) -> ::windows_core::HRESULT,
    pub GetDeviceSpecContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0001: *mut *mut u8) -> ::windows_core::HRESULT,
    pub GetFullItemName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0002: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetItemName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0003: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub AddItemToFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0004: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UnlinkItemTree: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0005: i32) -> ::windows_core::HRESULT,
    pub RemoveItemFromFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0006: i32) -> ::windows_core::HRESULT,
    pub FindItemByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0007: i32, __midl__iwiadrvitem0008: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, __midl__iwiadrvitem0009: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FindChildItemByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0010: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, __midl__iwiadrvitem0011: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetParentItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0012: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetFirstChildItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0013: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetNextSiblingItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0014: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DumpItemData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0015: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWiaErrorHandler(::windows_core::IUnknown);
impl IWiaErrorHandler {
    pub unsafe fn ReportStatus<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param2: ::windows_core::IntoParam<'a, IWiaItem2>>(&self, lflags: i32, hwndparent: Param1, pwiaitem2: Param2, hrstatus: ::windows_core::HRESULT, lpercentcomplete: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReportStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), hwndparent.into_param().abi(), pwiaitem2.into_param().abi(), ::core::mem::transmute(hrstatus), ::core::mem::transmute(lpercentcomplete)).ok()
    }
    pub unsafe fn GetStatusDescription<'a, Param1: ::windows_core::IntoParam<'a, IWiaItem2>>(&self, lflags: i32, pwiaitem2: Param1, hrstatus: ::windows_core::HRESULT) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetStatusDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), pwiaitem2.into_param().abi(), ::core::mem::transmute(hrstatus), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IWiaErrorHandler> for ::windows_core::IUnknown {
    fn from(value: IWiaErrorHandler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaErrorHandler> for ::windows_core::IUnknown {
    fn from(value: &IWiaErrorHandler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWiaErrorHandler {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWiaErrorHandler {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaErrorHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaErrorHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaErrorHandler {}
impl ::core::fmt::Debug for IWiaErrorHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaErrorHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWiaErrorHandler {
    type Vtable = IWiaErrorHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0e4a51b1_bc1f_443d_a835_72e890759ef3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaErrorHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ReportStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, hwndparent: ::win32_foundation::HWND, pwiaitem2: ::windows_core::RawPtr, hrstatus: ::windows_core::HRESULT, lpercentcomplete: i32) -> ::windows_core::HRESULT,
    pub GetStatusDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pwiaitem2: ::windows_core::RawPtr, hrstatus: ::windows_core::HRESULT, pbstrdescription: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWiaEventCallback(::windows_core::IUnknown);
impl IWiaEventCallback {
    pub unsafe fn ImageEventCallback<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, peventguid: *const ::windows_core::GUID, bstreventdescription: Param1, bstrdeviceid: Param2, bstrdevicedescription: Param3, dwdevicetype: u32, bstrfullitemname: Param5, puleventtype: *mut u32, ulreserved: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ImageEventCallback)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(peventguid), bstreventdescription.into_param().abi(), bstrdeviceid.into_param().abi(), bstrdevicedescription.into_param().abi(), ::core::mem::transmute(dwdevicetype), bstrfullitemname.into_param().abi(), ::core::mem::transmute(puleventtype), ::core::mem::transmute(ulreserved)).ok()
    }
}
impl ::core::convert::From<IWiaEventCallback> for ::windows_core::IUnknown {
    fn from(value: IWiaEventCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaEventCallback> for ::windows_core::IUnknown {
    fn from(value: &IWiaEventCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWiaEventCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWiaEventCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaEventCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaEventCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaEventCallback {}
impl ::core::fmt::Debug for IWiaEventCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaEventCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWiaEventCallback {
    type Vtable = IWiaEventCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae6287b0_0084_11d2_973b_00a0c9068f2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaEventCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ImageEventCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peventguid: *const ::windows_core::GUID, bstreventdescription: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrdeviceid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrdevicedescription: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, dwdevicetype: u32, bstrfullitemname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, puleventtype: *mut u32, ulreserved: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWiaImageFilter(::windows_core::IUnknown);
impl IWiaImageFilter {
    pub unsafe fn InitializeFilter<'a, Param0: ::windows_core::IntoParam<'a, IWiaItem2>, Param1: ::windows_core::IntoParam<'a, IWiaTransferCallback>>(&self, pwiaitem2: Param0, pwiatransfercallback: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InitializeFilter)(::windows_core::Interface::as_raw(self), pwiaitem2.into_param().abi(), pwiatransfercallback.into_param().abi()).ok()
    }
    pub unsafe fn SetNewCallback<'a, Param0: ::windows_core::IntoParam<'a, IWiaTransferCallback>>(&self, pwiatransfercallback: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNewCallback)(::windows_core::Interface::as_raw(self), pwiatransfercallback.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FilterPreviewImage<'a, Param1: ::windows_core::IntoParam<'a, IWiaItem2>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::RECT>, Param3: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>>(&self, lflags: i32, pwiachilditem2: Param1, inputimageextents: Param2, pinputstream: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FilterPreviewImage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), pwiachilditem2.into_param().abi(), inputimageextents.into_param().abi(), pinputstream.into_param().abi()).ok()
    }
    pub unsafe fn ApplyProperties<'a, Param0: ::windows_core::IntoParam<'a, IWiaPropertyStorage>>(&self, pwiapropertystorage: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ApplyProperties)(::windows_core::Interface::as_raw(self), pwiapropertystorage.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWiaImageFilter> for ::windows_core::IUnknown {
    fn from(value: IWiaImageFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaImageFilter> for ::windows_core::IUnknown {
    fn from(value: &IWiaImageFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWiaImageFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWiaImageFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaImageFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaImageFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaImageFilter {}
impl ::core::fmt::Debug for IWiaImageFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaImageFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWiaImageFilter {
    type Vtable = IWiaImageFilter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa8a79ffa_450b_41f1_8f87_849ccd94ebf6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaImageFilter_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub InitializeFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwiaitem2: ::windows_core::RawPtr, pwiatransfercallback: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetNewCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwiatransfercallback: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub FilterPreviewImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pwiachilditem2: ::windows_core::RawPtr, inputimageextents: ::win32_foundation::RECT, pinputstream: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FilterPreviewImage: usize,
    pub ApplyProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwiapropertystorage: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWiaItem(::windows_core::IUnknown);
impl IWiaItem {
    pub unsafe fn GetItemType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).GetItemType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn AnalyzeItem(&self, lflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AnalyzeItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags)).ok()
    }
    pub unsafe fn EnumChildItems(&self) -> ::windows_core::Result<IEnumWiaItem> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumChildItems)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumWiaItem>(result__)
    }
    pub unsafe fn DeleteItem(&self, lflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags)).ok()
    }
    pub unsafe fn CreateChildItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, lflags: i32, bstritemname: Param1, bstrfullitemname: Param2) -> ::windows_core::Result<IWiaItem> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateChildItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), bstritemname.into_param().abi(), bstrfullitemname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWiaItem>(result__)
    }
    pub unsafe fn EnumRegisterEventInfo(&self, lflags: i32, peventguid: *const ::windows_core::GUID) -> ::windows_core::Result<IEnumWIA_DEV_CAPS> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumRegisterEventInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(peventguid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumWIA_DEV_CAPS>(result__)
    }
    pub unsafe fn FindItemByName<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, lflags: i32, bstrfullitemname: Param1) -> ::windows_core::Result<IWiaItem> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).FindItemByName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), bstrfullitemname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWiaItem>(result__)
    }
    pub unsafe fn DeviceDlg<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(&self, hwndparent: Param0, lflags: i32, lintent: i32, plitemcount: *mut i32, ppiwiaitem: *mut *mut ::core::option::Option<IWiaItem>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeviceDlg)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(lintent), ::core::mem::transmute(plitemcount), ::core::mem::transmute(ppiwiaitem)).ok()
    }
    pub unsafe fn DeviceCommand(&self, lflags: i32, pcmdguid: *const ::windows_core::GUID, piwiaitem: *mut ::core::option::Option<IWiaItem>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeviceCommand)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(pcmdguid), ::core::mem::transmute(piwiaitem)).ok()
    }
    pub unsafe fn GetRootItem(&self) -> ::windows_core::Result<IWiaItem> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetRootItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWiaItem>(result__)
    }
    pub unsafe fn EnumDeviceCapabilities(&self, lflags: i32) -> ::windows_core::Result<IEnumWIA_DEV_CAPS> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumDeviceCapabilities)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumWIA_DEV_CAPS>(result__)
    }
    pub unsafe fn DumpItemData(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DumpItemData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn DumpDrvItemData(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DumpDrvItemData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn DumpTreeItemData(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DumpTreeItemData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Diagnostic(&self, pbuffer: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Diagnostic)(::windows_core::Interface::as_raw(self), pbuffer.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pbuffer))).ok()
    }
}
impl ::core::convert::From<IWiaItem> for ::windows_core::IUnknown {
    fn from(value: IWiaItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaItem> for ::windows_core::IUnknown {
    fn from(value: &IWiaItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWiaItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWiaItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaItem {}
impl ::core::fmt::Debug for IWiaItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaItem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWiaItem {
    type Vtable = IWiaItem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4db1ad10_3391_11d2_9a33_00c04fa36145);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaItem_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetItemType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitemtype: *mut i32) -> ::windows_core::HRESULT,
    pub AnalyzeItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows_core::HRESULT,
    pub EnumChildItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppienumwiaitem: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeleteItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows_core::HRESULT,
    pub CreateChildItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, bstritemname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrfullitemname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppiwiaitem: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EnumRegisterEventInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, peventguid: *const ::windows_core::GUID, ppienum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FindItemByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, bstrfullitemname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppiwiaitem: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeviceDlg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: ::win32_foundation::HWND, lflags: i32, lintent: i32, plitemcount: *mut i32, ppiwiaitem: *mut *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeviceCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pcmdguid: *const ::windows_core::GUID, piwiaitem: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetRootItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiwiaitem: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EnumDeviceCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, ppienumwia_dev_caps: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DumpItemData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdata: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub DumpDrvItemData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdata: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub DumpTreeItemData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdata: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Diagnostic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulsize: u32, pbuffer: *const u8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWiaItem2(::windows_core::IUnknown);
impl IWiaItem2 {
    pub unsafe fn CreateChildItem<'a, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, litemflags: i32, lcreationflags: i32, bstritemname: Param2) -> ::windows_core::Result<IWiaItem2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateChildItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(litemflags), ::core::mem::transmute(lcreationflags), bstritemname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWiaItem2>(result__)
    }
    pub unsafe fn DeleteItem(&self, lflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags)).ok()
    }
    pub unsafe fn EnumChildItems(&self, pcategoryguid: *const ::windows_core::GUID) -> ::windows_core::Result<IEnumWiaItem2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumChildItems)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcategoryguid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumWiaItem2>(result__)
    }
    pub unsafe fn FindItemByName<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, lflags: i32, bstrfullitemname: Param1) -> ::windows_core::Result<IWiaItem2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).FindItemByName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), bstrfullitemname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWiaItem2>(result__)
    }
    pub unsafe fn GetItemCategory(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).GetItemCategory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn GetItemType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).GetItemType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn DeviceDlg<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, lflags: i32, hwndparent: Param1, bstrfoldername: Param2, bstrfilename: Param3, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut ::win32_foundation::BSTR, ppitem: *mut ::core::option::Option<IWiaItem2>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeviceDlg)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), hwndparent.into_param().abi(), bstrfoldername.into_param().abi(), bstrfilename.into_param().abi(), ::core::mem::transmute(plnumfiles), ::core::mem::transmute(ppbstrfilepaths), ::core::mem::transmute(ppitem)).ok()
    }
    pub unsafe fn DeviceCommand(&self, lflags: i32, pcmdguid: *const ::windows_core::GUID, ppiwiaitem2: *mut ::core::option::Option<IWiaItem2>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeviceCommand)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(pcmdguid), ::core::mem::transmute(ppiwiaitem2)).ok()
    }
    pub unsafe fn EnumDeviceCapabilities(&self, lflags: i32) -> ::windows_core::Result<IEnumWIA_DEV_CAPS> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumDeviceCapabilities)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumWIA_DEV_CAPS>(result__)
    }
    pub unsafe fn CheckExtension<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, lflags: i32, bstrname: Param1, riidextensioninterface: *const ::windows_core::GUID, pbextensionexists: *mut ::win32_foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CheckExtension)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), bstrname.into_param().abi(), ::core::mem::transmute(riidextensioninterface), ::core::mem::transmute(pbextensionexists)).ok()
    }
    pub unsafe fn GetExtension<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, lflags: i32, bstrname: Param1, riidextensioninterface: *const ::windows_core::GUID, ppout: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetExtension)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), bstrname.into_param().abi(), ::core::mem::transmute(riidextensioninterface), ::core::mem::transmute(ppout)).ok()
    }
    pub unsafe fn GetParentItem(&self) -> ::windows_core::Result<IWiaItem2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetParentItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWiaItem2>(result__)
    }
    pub unsafe fn GetRootItem(&self) -> ::windows_core::Result<IWiaItem2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetRootItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWiaItem2>(result__)
    }
    pub unsafe fn GetPreviewComponent(&self, lflags: i32) -> ::windows_core::Result<IWiaPreview> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetPreviewComponent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWiaPreview>(result__)
    }
    pub unsafe fn EnumRegisterEventInfo(&self, lflags: i32, peventguid: *const ::windows_core::GUID) -> ::windows_core::Result<IEnumWIA_DEV_CAPS> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumRegisterEventInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(peventguid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumWIA_DEV_CAPS>(result__)
    }
    pub unsafe fn Diagnostic(&self, pbuffer: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Diagnostic)(::windows_core::Interface::as_raw(self), pbuffer.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pbuffer))).ok()
    }
}
impl ::core::convert::From<IWiaItem2> for ::windows_core::IUnknown {
    fn from(value: IWiaItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaItem2> for ::windows_core::IUnknown {
    fn from(value: &IWiaItem2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWiaItem2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWiaItem2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaItem2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaItem2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaItem2 {}
impl ::core::fmt::Debug for IWiaItem2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaItem2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWiaItem2 {
    type Vtable = IWiaItem2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6cba0075_1287_407d_9b77_cf0e030435cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaItem2_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CreateChildItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, litemflags: i32, lcreationflags: i32, bstritemname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppiwiaitem2: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeleteItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows_core::HRESULT,
    pub EnumChildItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcategoryguid: *const ::windows_core::GUID, ppienumwiaitem2: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FindItemByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, bstrfullitemname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppiwiaitem2: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetItemCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitemcategoryguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetItemType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitemtype: *mut i32) -> ::windows_core::HRESULT,
    pub DeviceDlg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, hwndparent: ::win32_foundation::HWND, bstrfoldername: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrfilename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut ::win32_foundation::BSTR, ppitem: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeviceCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pcmdguid: *const ::windows_core::GUID, ppiwiaitem2: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EnumDeviceCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, ppienumwia_dev_caps: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CheckExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, bstrname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, riidextensioninterface: *const ::windows_core::GUID, pbextensionexists: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, bstrname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, riidextensioninterface: *const ::windows_core::GUID, ppout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetParentItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiwiaitem2: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetRootItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiwiaitem2: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetPreviewComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, ppwiapreview: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EnumRegisterEventInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, peventguid: *const ::windows_core::GUID, ppienum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Diagnostic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulsize: u32, pbuffer: *const u8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWiaItemExtras(::windows_core::IUnknown);
impl IWiaItemExtras {
    pub unsafe fn GetExtendedErrorInfo(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetExtendedErrorInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Escape(&self, dwescapecode: u32, lpindata: &[u8], poutdata: *mut u8, dwoutdatasize: u32, pdwactualdatasize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Escape)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwescapecode), ::core::mem::transmute(::windows_core::as_ptr_or_null(lpindata)), lpindata.len() as _, ::core::mem::transmute(poutdata), ::core::mem::transmute(dwoutdatasize), ::core::mem::transmute(pdwactualdatasize)).ok()
    }
    pub unsafe fn CancelPendingIO(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CancelPendingIO)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWiaItemExtras> for ::windows_core::IUnknown {
    fn from(value: IWiaItemExtras) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaItemExtras> for ::windows_core::IUnknown {
    fn from(value: &IWiaItemExtras) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWiaItemExtras {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWiaItemExtras {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaItemExtras {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaItemExtras {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaItemExtras {}
impl ::core::fmt::Debug for IWiaItemExtras {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaItemExtras").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWiaItemExtras {
    type Vtable = IWiaItemExtras_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6291ef2c_36ef_4532_876a_8e132593778d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaItemExtras_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetExtendedErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrerrortext: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Escape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwescapecode: u32, lpindata: *const u8, cbindatasize: u32, poutdata: *mut u8, dwoutdatasize: u32, pdwactualdatasize: *mut u32) -> ::windows_core::HRESULT,
    pub CancelPendingIO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWiaLog(::windows_core::IUnknown);
impl IWiaLog {
    pub unsafe fn InitializeLog(&self, hinstance: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InitializeLog)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hinstance)).ok()
    }
    pub unsafe fn hResult(&self, hresult: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).hResult)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hresult)).ok()
    }
    pub unsafe fn Log<'a, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, lflags: i32, lresid: i32, ldetail: i32, bstrtext: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Log)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(lresid), ::core::mem::transmute(ldetail), bstrtext.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWiaLog> for ::windows_core::IUnknown {
    fn from(value: IWiaLog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaLog> for ::windows_core::IUnknown {
    fn from(value: &IWiaLog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWiaLog {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWiaLog {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaLog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaLog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaLog {}
impl ::core::fmt::Debug for IWiaLog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaLog").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWiaLog {
    type Vtable = IWiaLog_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa00c10b6_82a1_452f_8b6c_86062aad6890);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaLog_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub InitializeLog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hinstance: i32) -> ::windows_core::HRESULT,
    pub hResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub Log: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, lresid: i32, ldetail: i32, bstrtext: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWiaLogEx(::windows_core::IUnknown);
impl IWiaLogEx {
    pub unsafe fn InitializeLogEx(&self, hinstance: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InitializeLogEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hinstance)).ok()
    }
    pub unsafe fn hResult(&self, hresult: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).hResult)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hresult)).ok()
    }
    pub unsafe fn Log<'a, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, lflags: i32, lresid: i32, ldetail: i32, bstrtext: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Log)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(lresid), ::core::mem::transmute(ldetail), bstrtext.into_param().abi()).ok()
    }
    pub unsafe fn hResultEx(&self, lmethodid: i32, hresult: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).hResultEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lmethodid), ::core::mem::transmute(hresult)).ok()
    }
    pub unsafe fn LogEx<'a, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, lmethodid: i32, lflags: i32, lresid: i32, ldetail: i32, bstrtext: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LogEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lmethodid), ::core::mem::transmute(lflags), ::core::mem::transmute(lresid), ::core::mem::transmute(ldetail), bstrtext.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWiaLogEx> for ::windows_core::IUnknown {
    fn from(value: IWiaLogEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaLogEx> for ::windows_core::IUnknown {
    fn from(value: &IWiaLogEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWiaLogEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWiaLogEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaLogEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaLogEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaLogEx {}
impl ::core::fmt::Debug for IWiaLogEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaLogEx").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWiaLogEx {
    type Vtable = IWiaLogEx_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaf1f22ac_7a40_4787_b421_aeb47a1fbd0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaLogEx_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub InitializeLogEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hinstance: *const u8) -> ::windows_core::HRESULT,
    pub hResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub Log: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, lresid: i32, ldetail: i32, bstrtext: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub hResultEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmethodid: i32, hresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub LogEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmethodid: i32, lflags: i32, lresid: i32, ldetail: i32, bstrtext: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWiaMiniDrv(::windows_core::IUnknown);
impl IWiaMiniDrv {
    pub unsafe fn drvInitializeWia<'a, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param5: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, __midl__iwiaminidrv0000: *const u8, __midl__iwiaminidrv0001: i32, __midl__iwiaminidrv0002: Param2, __midl__iwiaminidrv0003: Param3, __midl__iwiaminidrv0004: Param4, __midl__iwiaminidrv0005: Param5, __midl__iwiaminidrv0006: *mut ::core::option::Option<IWiaDrvItem>, __midl__iwiaminidrv0007: *mut ::core::option::Option<::windows_core::IUnknown>, __midl__iwiaminidrv0008: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).drvInitializeWia)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(__midl__iwiaminidrv0000), ::core::mem::transmute(__midl__iwiaminidrv0001), __midl__iwiaminidrv0002.into_param().abi(), __midl__iwiaminidrv0003.into_param().abi(), __midl__iwiaminidrv0004.into_param().abi(), __midl__iwiaminidrv0005.into_param().abi(), ::core::mem::transmute(__midl__iwiaminidrv0006), ::core::mem::transmute(__midl__iwiaminidrv0007), ::core::mem::transmute(__midl__iwiaminidrv0008)).ok()
    }
    pub unsafe fn drvAcquireItemData(&self, __midl__iwiaminidrv0009: *const u8, __midl__iwiaminidrv0010: i32, __midl__iwiaminidrv0011: *mut MINIDRV_TRANSFER_CONTEXT, __midl__iwiaminidrv0012: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).drvAcquireItemData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(__midl__iwiaminidrv0009), ::core::mem::transmute(__midl__iwiaminidrv0010), ::core::mem::transmute(__midl__iwiaminidrv0011), ::core::mem::transmute(__midl__iwiaminidrv0012)).ok()
    }
    pub unsafe fn drvInitItemProperties(&self, __midl__iwiaminidrv0013: *const u8, __midl__iwiaminidrv0014: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).drvInitItemProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(__midl__iwiaminidrv0013), ::core::mem::transmute(__midl__iwiaminidrv0014), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn drvValidateItemProperties(&self, __midl__iwiaminidrv0016: *const u8, __midl__iwiaminidrv0017: i32, __midl__iwiaminidrv0018: u32, __midl__iwiaminidrv0019: *const ::win32_system::Com::StructuredStorage::PROPSPEC) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).drvValidateItemProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(__midl__iwiaminidrv0016), ::core::mem::transmute(__midl__iwiaminidrv0017), ::core::mem::transmute(__midl__iwiaminidrv0018), ::core::mem::transmute(__midl__iwiaminidrv0019), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn drvWriteItemProperties(&self, __midl__iwiaminidrv0021: *const u8, __midl__iwiaminidrv0022: i32, __midl__iwiaminidrv0023: *const MINIDRV_TRANSFER_CONTEXT) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).drvWriteItemProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(__midl__iwiaminidrv0021), ::core::mem::transmute(__midl__iwiaminidrv0022), ::core::mem::transmute(__midl__iwiaminidrv0023), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn drvReadItemProperties(&self, __midl__iwiaminidrv0025: *const u8, __midl__iwiaminidrv0026: i32, __midl__iwiaminidrv0027: u32, __midl__iwiaminidrv0028: *const ::win32_system::Com::StructuredStorage::PROPSPEC) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).drvReadItemProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(__midl__iwiaminidrv0025), ::core::mem::transmute(__midl__iwiaminidrv0026), ::core::mem::transmute(__midl__iwiaminidrv0027), ::core::mem::transmute(__midl__iwiaminidrv0028), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn drvLockWiaDevice(&self, __midl__iwiaminidrv0030: *const u8, __midl__iwiaminidrv0031: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).drvLockWiaDevice)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(__midl__iwiaminidrv0030), ::core::mem::transmute(__midl__iwiaminidrv0031), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn drvUnLockWiaDevice(&self, __midl__iwiaminidrv0033: *const u8, __midl__iwiaminidrv0034: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).drvUnLockWiaDevice)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(__midl__iwiaminidrv0033), ::core::mem::transmute(__midl__iwiaminidrv0034), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn drvAnalyzeItem(&self, __midl__iwiaminidrv0036: *const u8, __midl__iwiaminidrv0037: i32, __midl__iwiaminidrv0038: *const i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).drvAnalyzeItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(__midl__iwiaminidrv0036), ::core::mem::transmute(__midl__iwiaminidrv0037), ::core::mem::transmute(__midl__iwiaminidrv0038)).ok()
    }
    pub unsafe fn drvGetDeviceErrorStr(&self, __midl__iwiaminidrv0039: i32, __midl__iwiaminidrv0040: i32, __midl__iwiaminidrv0041: *mut ::windows_core::PWSTR, __midl__iwiaminidrv0042: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).drvGetDeviceErrorStr)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(__midl__iwiaminidrv0039), ::core::mem::transmute(__midl__iwiaminidrv0040), ::core::mem::transmute(__midl__iwiaminidrv0041), ::core::mem::transmute(__midl__iwiaminidrv0042)).ok()
    }
    pub unsafe fn drvDeviceCommand(&self, __midl__iwiaminidrv0043: *const u8, __midl__iwiaminidrv0044: i32, __midl__iwiaminidrv0045: *const ::windows_core::GUID, __midl__iwiaminidrv0046: *mut ::core::option::Option<IWiaDrvItem>, __midl__iwiaminidrv0047: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).drvDeviceCommand)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(__midl__iwiaminidrv0043), ::core::mem::transmute(__midl__iwiaminidrv0044), ::core::mem::transmute(__midl__iwiaminidrv0045), ::core::mem::transmute(__midl__iwiaminidrv0046), ::core::mem::transmute(__midl__iwiaminidrv0047)).ok()
    }
    pub unsafe fn drvGetCapabilities(&self, __midl__iwiaminidrv0048: *const u8, __midl__iwiaminidrv0049: i32, __midl__iwiaminidrv0050: *mut i32, __midl__iwiaminidrv0051: *mut *mut WIA_DEV_CAP_DRV, __midl__iwiaminidrv0052: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).drvGetCapabilities)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(__midl__iwiaminidrv0048), ::core::mem::transmute(__midl__iwiaminidrv0049), ::core::mem::transmute(__midl__iwiaminidrv0050), ::core::mem::transmute(__midl__iwiaminidrv0051), ::core::mem::transmute(__midl__iwiaminidrv0052)).ok()
    }
    pub unsafe fn drvDeleteItem(&self, __midl__iwiaminidrv0053: *const u8, __midl__iwiaminidrv0054: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).drvDeleteItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(__midl__iwiaminidrv0053), ::core::mem::transmute(__midl__iwiaminidrv0054), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn drvFreeDrvItemContext(&self, __midl__iwiaminidrv0056: i32, __midl__iwiaminidrv0057: *const u8) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).drvFreeDrvItemContext)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(__midl__iwiaminidrv0056), ::core::mem::transmute(__midl__iwiaminidrv0057), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn drvGetWiaFormatInfo(&self, __midl__iwiaminidrv0059: *const u8, __midl__iwiaminidrv0060: i32, __midl__iwiaminidrv0061: *mut i32, __midl__iwiaminidrv0062: *mut *mut WIA_FORMAT_INFO, __midl__iwiaminidrv0063: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).drvGetWiaFormatInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(__midl__iwiaminidrv0059), ::core::mem::transmute(__midl__iwiaminidrv0060), ::core::mem::transmute(__midl__iwiaminidrv0061), ::core::mem::transmute(__midl__iwiaminidrv0062), ::core::mem::transmute(__midl__iwiaminidrv0063)).ok()
    }
    pub unsafe fn drvNotifyPnpEvent<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, peventguid: *const ::windows_core::GUID, bstrdeviceid: Param1, ulreserved: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).drvNotifyPnpEvent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(peventguid), bstrdeviceid.into_param().abi(), ::core::mem::transmute(ulreserved)).ok()
    }
    pub unsafe fn drvUnInitializeWia(&self, __midl__iwiaminidrv0064: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).drvUnInitializeWia)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(__midl__iwiaminidrv0064)).ok()
    }
}
impl ::core::convert::From<IWiaMiniDrv> for ::windows_core::IUnknown {
    fn from(value: IWiaMiniDrv) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaMiniDrv> for ::windows_core::IUnknown {
    fn from(value: &IWiaMiniDrv) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWiaMiniDrv {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWiaMiniDrv {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaMiniDrv {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaMiniDrv {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaMiniDrv {}
impl ::core::fmt::Debug for IWiaMiniDrv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaMiniDrv").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWiaMiniDrv {
    type Vtable = IWiaMiniDrv_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd8cdee14_3c6c_11d2_9a35_00c04fa36145);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaMiniDrv_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub drvInitializeWia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0000: *const u8, __midl__iwiaminidrv0001: i32, __midl__iwiaminidrv0002: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, __midl__iwiaminidrv0003: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, __midl__iwiaminidrv0004: *mut ::core::ffi::c_void, __midl__iwiaminidrv0005: *mut ::core::ffi::c_void, __midl__iwiaminidrv0006: *mut ::windows_core::RawPtr, __midl__iwiaminidrv0007: *mut *mut ::core::ffi::c_void, __midl__iwiaminidrv0008: *mut i32) -> ::windows_core::HRESULT,
    pub drvAcquireItemData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0009: *const u8, __midl__iwiaminidrv0010: i32, __midl__iwiaminidrv0011: *mut MINIDRV_TRANSFER_CONTEXT, __midl__iwiaminidrv0012: *mut i32) -> ::windows_core::HRESULT,
    pub drvInitItemProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0013: *const u8, __midl__iwiaminidrv0014: i32, __midl__iwiaminidrv0015: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub drvValidateItemProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0016: *const u8, __midl__iwiaminidrv0017: i32, __midl__iwiaminidrv0018: u32, __midl__iwiaminidrv0019: *const ::win32_system::Com::StructuredStorage::PROPSPEC, __midl__iwiaminidrv0020: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    drvValidateItemProperties: usize,
    pub drvWriteItemProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0021: *const u8, __midl__iwiaminidrv0022: i32, __midl__iwiaminidrv0023: *const MINIDRV_TRANSFER_CONTEXT, __midl__iwiaminidrv0024: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub drvReadItemProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0025: *const u8, __midl__iwiaminidrv0026: i32, __midl__iwiaminidrv0027: u32, __midl__iwiaminidrv0028: *const ::win32_system::Com::StructuredStorage::PROPSPEC, __midl__iwiaminidrv0029: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    drvReadItemProperties: usize,
    pub drvLockWiaDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0030: *const u8, __midl__iwiaminidrv0031: i32, __midl__iwiaminidrv0032: *mut i32) -> ::windows_core::HRESULT,
    pub drvUnLockWiaDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0033: *const u8, __midl__iwiaminidrv0034: i32, __midl__iwiaminidrv0035: *mut i32) -> ::windows_core::HRESULT,
    pub drvAnalyzeItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0036: *const u8, __midl__iwiaminidrv0037: i32, __midl__iwiaminidrv0038: *const i32) -> ::windows_core::HRESULT,
    pub drvGetDeviceErrorStr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0039: i32, __midl__iwiaminidrv0040: i32, __midl__iwiaminidrv0041: *mut ::windows_core::PWSTR, __midl__iwiaminidrv0042: *mut i32) -> ::windows_core::HRESULT,
    pub drvDeviceCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0043: *const u8, __midl__iwiaminidrv0044: i32, __midl__iwiaminidrv0045: *const ::windows_core::GUID, __midl__iwiaminidrv0046: *mut ::windows_core::RawPtr, __midl__iwiaminidrv0047: *mut i32) -> ::windows_core::HRESULT,
    pub drvGetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0048: *const u8, __midl__iwiaminidrv0049: i32, __midl__iwiaminidrv0050: *mut i32, __midl__iwiaminidrv0051: *mut *mut WIA_DEV_CAP_DRV, __midl__iwiaminidrv0052: *mut i32) -> ::windows_core::HRESULT,
    pub drvDeleteItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0053: *const u8, __midl__iwiaminidrv0054: i32, __midl__iwiaminidrv0055: *mut i32) -> ::windows_core::HRESULT,
    pub drvFreeDrvItemContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0056: i32, __midl__iwiaminidrv0057: *const u8, __midl__iwiaminidrv0058: *mut i32) -> ::windows_core::HRESULT,
    pub drvGetWiaFormatInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0059: *const u8, __midl__iwiaminidrv0060: i32, __midl__iwiaminidrv0061: *mut i32, __midl__iwiaminidrv0062: *mut *mut WIA_FORMAT_INFO, __midl__iwiaminidrv0063: *mut i32) -> ::windows_core::HRESULT,
    pub drvNotifyPnpEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peventguid: *const ::windows_core::GUID, bstrdeviceid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ulreserved: u32) -> ::windows_core::HRESULT,
    pub drvUnInitializeWia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0064: *const u8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWiaMiniDrvCallBack(::windows_core::IUnknown);
impl IWiaMiniDrvCallBack {
    pub unsafe fn MiniDrvCallback(&self, lreason: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, ptranctx: *const MINIDRV_TRANSFER_CONTEXT, lreserved: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MiniDrvCallback)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lreason), ::core::mem::transmute(lstatus), ::core::mem::transmute(lpercentcomplete), ::core::mem::transmute(loffset), ::core::mem::transmute(llength), ::core::mem::transmute(ptranctx), ::core::mem::transmute(lreserved)).ok()
    }
}
impl ::core::convert::From<IWiaMiniDrvCallBack> for ::windows_core::IUnknown {
    fn from(value: IWiaMiniDrvCallBack) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaMiniDrvCallBack> for ::windows_core::IUnknown {
    fn from(value: &IWiaMiniDrvCallBack) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWiaMiniDrvCallBack {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWiaMiniDrvCallBack {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaMiniDrvCallBack {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaMiniDrvCallBack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaMiniDrvCallBack {}
impl ::core::fmt::Debug for IWiaMiniDrvCallBack {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaMiniDrvCallBack").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWiaMiniDrvCallBack {
    type Vtable = IWiaMiniDrvCallBack_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33a57d5a_3de8_11d2_9a36_00c04fa36145);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaMiniDrvCallBack_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub MiniDrvCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lreason: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, ptranctx: *const MINIDRV_TRANSFER_CONTEXT, lreserved: i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWiaMiniDrvTransferCallback(::windows_core::IUnknown);
impl IWiaMiniDrvTransferCallback {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNextStream<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, lflags: i32, bstritemname: Param1, bstrfullitemname: Param2) -> ::windows_core::Result<::win32_system::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetNextStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), bstritemname.into_param().abi(), bstrfullitemname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::IStream>(result__)
    }
    pub unsafe fn SendMessage(&self, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendMessage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(pwiatransferparams)).ok()
    }
}
impl ::core::convert::From<IWiaMiniDrvTransferCallback> for ::windows_core::IUnknown {
    fn from(value: IWiaMiniDrvTransferCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaMiniDrvTransferCallback> for ::windows_core::IUnknown {
    fn from(value: &IWiaMiniDrvTransferCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWiaMiniDrvTransferCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWiaMiniDrvTransferCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaMiniDrvTransferCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaMiniDrvTransferCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaMiniDrvTransferCallback {}
impl ::core::fmt::Debug for IWiaMiniDrvTransferCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaMiniDrvTransferCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWiaMiniDrvTransferCallback {
    type Vtable = IWiaMiniDrvTransferCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa9d2ee89_2ce5_4ff0_8adb_c961d1d774ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaMiniDrvTransferCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNextStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, bstritemname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrfullitemname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppistream: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNextStream: usize,
    pub SendMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWiaNotifyDevMgr(::windows_core::IUnknown);
impl IWiaNotifyDevMgr {
    pub unsafe fn NewDeviceArrival(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NewDeviceArrival)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWiaNotifyDevMgr> for ::windows_core::IUnknown {
    fn from(value: IWiaNotifyDevMgr) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaNotifyDevMgr> for ::windows_core::IUnknown {
    fn from(value: &IWiaNotifyDevMgr) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWiaNotifyDevMgr {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWiaNotifyDevMgr {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaNotifyDevMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaNotifyDevMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaNotifyDevMgr {}
impl ::core::fmt::Debug for IWiaNotifyDevMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaNotifyDevMgr").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWiaNotifyDevMgr {
    type Vtable = IWiaNotifyDevMgr_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x70681ea0_e7bf_4291_9fb1_4e8813a3f78e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaNotifyDevMgr_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub NewDeviceArrival: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWiaPreview(::windows_core::IUnknown);
impl IWiaPreview {
    pub unsafe fn GetNewPreview<'a, Param1: ::windows_core::IntoParam<'a, IWiaItem2>, Param2: ::windows_core::IntoParam<'a, IWiaTransferCallback>>(&self, lflags: i32, pwiaitem2: Param1, pwiatransfercallback: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetNewPreview)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), pwiaitem2.into_param().abi(), pwiatransfercallback.into_param().abi()).ok()
    }
    pub unsafe fn UpdatePreview<'a, Param1: ::windows_core::IntoParam<'a, IWiaItem2>, Param2: ::windows_core::IntoParam<'a, IWiaTransferCallback>>(&self, lflags: i32, pchildwiaitem2: Param1, pwiatransfercallback: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UpdatePreview)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), pchildwiaitem2.into_param().abi(), pwiatransfercallback.into_param().abi()).ok()
    }
    pub unsafe fn DetectRegions(&self, lflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DetectRegions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags)).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Clear)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWiaPreview> for ::windows_core::IUnknown {
    fn from(value: IWiaPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaPreview> for ::windows_core::IUnknown {
    fn from(value: &IWiaPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWiaPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWiaPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaPreview {}
impl ::core::fmt::Debug for IWiaPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaPreview").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWiaPreview {
    type Vtable = IWiaPreview_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x95c2b4fd_33f2_4d86_ad40_9431f0df08f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaPreview_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetNewPreview: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pwiaitem2: ::windows_core::RawPtr, pwiatransfercallback: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UpdatePreview: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pchildwiaitem2: ::windows_core::RawPtr, pwiatransfercallback: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DetectRegions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWiaPropertyStorage(::windows_core::IUnknown);
impl IWiaPropertyStorage {
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn ReadMultiple(&self, cpspec: u32, rgpspec: *const ::win32_system::Com::StructuredStorage::PROPSPEC, rgpropvar: *mut ::win32_system::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReadMultiple)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cpspec), ::core::mem::transmute(rgpspec), ::core::mem::transmute(rgpropvar)).ok()
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn WriteMultiple(&self, cpspec: u32, rgpspec: *const ::win32_system::Com::StructuredStorage::PROPSPEC, rgpropvar: *const ::win32_system::Com::StructuredStorage::PROPVARIANT, propidnamefirst: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteMultiple)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cpspec), ::core::mem::transmute(rgpspec), ::core::mem::transmute(rgpropvar), ::core::mem::transmute(propidnamefirst)).ok()
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn DeleteMultiple(&self, rgpspec: &[::win32_system::Com::StructuredStorage::PROPSPEC]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteMultiple)(::windows_core::Interface::as_raw(self), rgpspec.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(rgpspec))).ok()
    }
    pub unsafe fn ReadPropertyNames(&self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReadPropertyNames)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cpropid), ::core::mem::transmute(rgpropid), ::core::mem::transmute(rglpwstrname)).ok()
    }
    pub unsafe fn WritePropertyNames(&self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WritePropertyNames)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cpropid), ::core::mem::transmute(rgpropid), ::core::mem::transmute(rglpwstrname)).ok()
    }
    pub unsafe fn DeletePropertyNames(&self, rgpropid: &[u32]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeletePropertyNames)(::windows_core::Interface::as_raw(self), rgpropid.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(rgpropid))).ok()
    }
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Commit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(grfcommitflags)).ok()
    }
    pub unsafe fn Revert(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Revert)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Enum(&self) -> ::windows_core::Result<::win32_system::Com::StructuredStorage::IEnumSTATPROPSTG> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Enum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::StructuredStorage::IEnumSTATPROPSTG>(result__)
    }
    pub unsafe fn SetTimes(&self, pctime: *const ::win32_foundation::FILETIME, patime: *const ::win32_foundation::FILETIME, pmtime: *const ::win32_foundation::FILETIME) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTimes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pctime), ::core::mem::transmute(patime), ::core::mem::transmute(pmtime)).ok()
    }
    pub unsafe fn SetClass(&self, clsid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetClass)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(clsid)).ok()
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Stat(&self) -> ::windows_core::Result<::win32_system::Com::StructuredStorage::STATPROPSETSTG> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_system::Com::StructuredStorage::STATPROPSETSTG>::zeroed();
        (::windows_core::Interface::vtable(self).Stat)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::StructuredStorage::STATPROPSETSTG>(result__)
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn GetPropertyAttributes(&self, cpspec: u32, rgpspec: *const ::win32_system::Com::StructuredStorage::PROPSPEC, rgflags: *mut u32, rgpropvar: *mut ::win32_system::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertyAttributes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cpspec), ::core::mem::transmute(rgpspec), ::core::mem::transmute(rgflags), ::core::mem::transmute(rgpropvar)).ok()
    }
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPropertyStream(&self, pcompatibilityid: *mut ::windows_core::GUID, ppistream: *mut ::core::option::Option<::win32_system::Com::IStream>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertyStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcompatibilityid), ::core::mem::transmute(ppistream)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPropertyStream<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>>(&self, pcompatibilityid: *mut ::windows_core::GUID, pistream: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPropertyStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcompatibilityid), pistream.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWiaPropertyStorage> for ::windows_core::IUnknown {
    fn from(value: IWiaPropertyStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaPropertyStorage> for ::windows_core::IUnknown {
    fn from(value: &IWiaPropertyStorage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWiaPropertyStorage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWiaPropertyStorage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaPropertyStorage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaPropertyStorage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaPropertyStorage {}
impl ::core::fmt::Debug for IWiaPropertyStorage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaPropertyStorage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWiaPropertyStorage {
    type Vtable = IWiaPropertyStorage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x98b5e8a0_29cc_491a_aac0_e6db4fdcceb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaPropertyStorage_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub ReadMultiple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const ::win32_system::Com::StructuredStorage::PROPSPEC, rgpropvar: *mut ::win32_system::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    ReadMultiple: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub WriteMultiple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const ::win32_system::Com::StructuredStorage::PROPSPEC, rgpropvar: *const ::win32_system::Com::StructuredStorage::PROPVARIANT, propidnamefirst: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    WriteMultiple: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub DeleteMultiple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const ::win32_system::Com::StructuredStorage::PROPSPEC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    DeleteMultiple: usize,
    pub ReadPropertyNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub WritePropertyNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub DeletePropertyNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32) -> ::windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfcommitflags: u32) -> ::windows_core::HRESULT,
    pub Revert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub Enum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    Enum: usize,
    pub SetTimes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctime: *const ::win32_foundation::FILETIME, patime: *const ::win32_foundation::FILETIME, pmtime: *const ::win32_foundation::FILETIME) -> ::windows_core::HRESULT,
    pub SetClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub Stat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatpsstg: *mut ::win32_system::Com::StructuredStorage::STATPROPSETSTG) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    Stat: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub GetPropertyAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const ::win32_system::Com::StructuredStorage::PROPSPEC, rgflags: *mut u32, rgpropvar: *mut ::win32_system::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    GetPropertyAttributes: usize,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulnumprops: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPropertyStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcompatibilityid: *mut ::windows_core::GUID, ppistream: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPropertyStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetPropertyStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcompatibilityid: *mut ::windows_core::GUID, pistream: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetPropertyStream: usize,
}
#[repr(transparent)]
pub struct IWiaSegmentationFilter(::windows_core::IUnknown);
impl IWiaSegmentationFilter {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DetectRegions<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>, Param2: ::windows_core::IntoParam<'a, IWiaItem2>>(&self, lflags: i32, pinputstream: Param1, pwiaitem2: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DetectRegions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), pinputstream.into_param().abi(), pwiaitem2.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWiaSegmentationFilter> for ::windows_core::IUnknown {
    fn from(value: IWiaSegmentationFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaSegmentationFilter> for ::windows_core::IUnknown {
    fn from(value: &IWiaSegmentationFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWiaSegmentationFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWiaSegmentationFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaSegmentationFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaSegmentationFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaSegmentationFilter {}
impl ::core::fmt::Debug for IWiaSegmentationFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaSegmentationFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWiaSegmentationFilter {
    type Vtable = IWiaSegmentationFilter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xec46a697_ac04_4447_8f65_ff63d5154b21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaSegmentationFilter_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub DetectRegions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pinputstream: ::windows_core::RawPtr, pwiaitem2: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DetectRegions: usize,
}
#[repr(transparent)]
pub struct IWiaTransfer(::windows_core::IUnknown);
impl IWiaTransfer {
    pub unsafe fn Download<'a, Param1: ::windows_core::IntoParam<'a, IWiaTransferCallback>>(&self, lflags: i32, piwiatransfercallback: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Download)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), piwiatransfercallback.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Upload<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>, Param2: ::windows_core::IntoParam<'a, IWiaTransferCallback>>(&self, lflags: i32, psource: Param1, piwiatransfercallback: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Upload)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), psource.into_param().abi(), piwiatransfercallback.into_param().abi()).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnumWIA_FORMAT_INFO(&self) -> ::windows_core::Result<IEnumWIA_FORMAT_INFO> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumWIA_FORMAT_INFO)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumWIA_FORMAT_INFO>(result__)
    }
}
impl ::core::convert::From<IWiaTransfer> for ::windows_core::IUnknown {
    fn from(value: IWiaTransfer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaTransfer> for ::windows_core::IUnknown {
    fn from(value: &IWiaTransfer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWiaTransfer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWiaTransfer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaTransfer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaTransfer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaTransfer {}
impl ::core::fmt::Debug for IWiaTransfer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaTransfer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWiaTransfer {
    type Vtable = IWiaTransfer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc39d6942_2f4e_4d04_92fe_4ef4d3a1de5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaTransfer_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Download: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, piwiatransfercallback: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Upload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, psource: ::windows_core::RawPtr, piwiatransfercallback: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Upload: usize,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EnumWIA_FORMAT_INFO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWiaTransferCallback(::windows_core::IUnknown);
impl IWiaTransferCallback {
    pub unsafe fn TransferCallback(&self, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TransferCallback)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(pwiatransferparams)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNextStream<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, lflags: i32, bstritemname: Param1, bstrfullitemname: Param2) -> ::windows_core::Result<::win32_system::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetNextStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), bstritemname.into_param().abi(), bstrfullitemname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::IStream>(result__)
    }
}
impl ::core::convert::From<IWiaTransferCallback> for ::windows_core::IUnknown {
    fn from(value: IWiaTransferCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaTransferCallback> for ::windows_core::IUnknown {
    fn from(value: &IWiaTransferCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWiaTransferCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWiaTransferCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaTransferCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaTransferCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaTransferCallback {}
impl ::core::fmt::Debug for IWiaTransferCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaTransferCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWiaTransferCallback {
    type Vtable = IWiaTransferCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27d4eaaf_28a6_4ca5_9aab_e678168b9527);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaTransferCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub TransferCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNextStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, bstritemname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrfullitemname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppdestination: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNextStream: usize,
}
#[repr(transparent)]
pub struct IWiaUIExtension(::windows_core::IUnknown);
impl IWiaUIExtension {
    pub unsafe fn DeviceDialog(&self, pdevicedialogdata: *const DEVICEDIALOGDATA) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeviceDialog)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdevicedialogdata)).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetDeviceIcon<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdeviceid: Param0, phicon: *mut ::win32_ui::WindowsAndMessaging::HICON, nsize: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDeviceIcon)(::windows_core::Interface::as_raw(self), bstrdeviceid.into_param().abi(), ::core::mem::transmute(phicon), ::core::mem::transmute(nsize)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetDeviceBitmapLogo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdeviceid: Param0, phbitmap: *mut ::win32_graphics::Gdi::HBITMAP, nmaxwidth: u32, nmaxheight: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDeviceBitmapLogo)(::windows_core::Interface::as_raw(self), bstrdeviceid.into_param().abi(), ::core::mem::transmute(phbitmap), ::core::mem::transmute(nmaxwidth), ::core::mem::transmute(nmaxheight)).ok()
    }
}
impl ::core::convert::From<IWiaUIExtension> for ::windows_core::IUnknown {
    fn from(value: IWiaUIExtension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaUIExtension> for ::windows_core::IUnknown {
    fn from(value: &IWiaUIExtension) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWiaUIExtension {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWiaUIExtension {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaUIExtension {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaUIExtension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaUIExtension {}
impl ::core::fmt::Debug for IWiaUIExtension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaUIExtension").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWiaUIExtension {
    type Vtable = IWiaUIExtension_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xda319113_50ee_4c80_b460_57d005d44a2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaUIExtension_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub DeviceDialog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevicedialogdata: *const DEVICEDIALOGDATA) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetDeviceIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdeviceid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, phicon: *mut ::win32_ui::WindowsAndMessaging::HICON, nsize: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetDeviceIcon: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetDeviceBitmapLogo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdeviceid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, phbitmap: *mut ::win32_graphics::Gdi::HBITMAP, nmaxwidth: u32, nmaxheight: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetDeviceBitmapLogo: usize,
}
#[repr(transparent)]
pub struct IWiaUIExtension2(::windows_core::IUnknown);
impl IWiaUIExtension2 {
    pub unsafe fn DeviceDialog(&self, pdevicedialogdata: *const DEVICEDIALOGDATA2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeviceDialog)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdevicedialogdata)).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetDeviceIcon<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdeviceid: Param0, phicon: *mut ::win32_ui::WindowsAndMessaging::HICON, nsize: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDeviceIcon)(::windows_core::Interface::as_raw(self), bstrdeviceid.into_param().abi(), ::core::mem::transmute(phicon), ::core::mem::transmute(nsize)).ok()
    }
}
impl ::core::convert::From<IWiaUIExtension2> for ::windows_core::IUnknown {
    fn from(value: IWiaUIExtension2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaUIExtension2> for ::windows_core::IUnknown {
    fn from(value: &IWiaUIExtension2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWiaUIExtension2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWiaUIExtension2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaUIExtension2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaUIExtension2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaUIExtension2 {}
impl ::core::fmt::Debug for IWiaUIExtension2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaUIExtension2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWiaUIExtension2 {
    type Vtable = IWiaUIExtension2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x305600d7_5088_46d7_9a15_b77b09cdba7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaUIExtension2_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub DeviceDialog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevicedialogdata: *const DEVICEDIALOGDATA2) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetDeviceIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdeviceid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, phicon: *mut ::win32_ui::WindowsAndMessaging::HICON, nsize: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetDeviceIcon: usize,
}
#[repr(transparent)]
pub struct IWiaVideo(::windows_core::IUnknown);
impl IWiaVideo {
    pub unsafe fn PreviewVisible(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).PreviewVisible)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetPreviewVisible<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bpreviewvisible: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPreviewVisible)(::windows_core::Interface::as_raw(self), bpreviewvisible.into_param().abi()).ok()
    }
    pub unsafe fn ImagesDirectory(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ImagesDirectory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetImagesDirectory<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrimagedirectory: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetImagesDirectory)(::windows_core::Interface::as_raw(self), bstrimagedirectory.into_param().abi()).ok()
    }
    pub unsafe fn CreateVideoByWiaDevID<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bstrwiadeviceid: Param0, hwndparent: Param1, bstretchtofitparent: Param2, bautobeginplayback: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateVideoByWiaDevID)(::windows_core::Interface::as_raw(self), bstrwiadeviceid.into_param().abi(), hwndparent.into_param().abi(), bstretchtofitparent.into_param().abi(), bautobeginplayback.into_param().abi()).ok()
    }
    pub unsafe fn CreateVideoByDevNum<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, uidevicenumber: u32, hwndparent: Param1, bstretchtofitparent: Param2, bautobeginplayback: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateVideoByDevNum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uidevicenumber), hwndparent.into_param().abi(), bstretchtofitparent.into_param().abi(), bautobeginplayback.into_param().abi()).ok()
    }
    pub unsafe fn CreateVideoByName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bstrfriendlyname: Param0, hwndparent: Param1, bstretchtofitparent: Param2, bautobeginplayback: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateVideoByName)(::windows_core::Interface::as_raw(self), bstrfriendlyname.into_param().abi(), hwndparent.into_param().abi(), bstretchtofitparent.into_param().abi(), bautobeginplayback.into_param().abi()).ok()
    }
    pub unsafe fn DestroyVideo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DestroyVideo)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Play(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Play)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Pause)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn TakePicture(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).TakePicture)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ResizeVideo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bstretchtofitparent: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ResizeVideo)(::windows_core::Interface::as_raw(self), bstretchtofitparent.into_param().abi()).ok()
    }
    pub unsafe fn GetCurrentState(&self) -> ::windows_core::Result<WIAVIDEO_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::<WIAVIDEO_STATE>::zeroed();
        (::windows_core::Interface::vtable(self).GetCurrentState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WIAVIDEO_STATE>(result__)
    }
}
impl ::core::convert::From<IWiaVideo> for ::windows_core::IUnknown {
    fn from(value: IWiaVideo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaVideo> for ::windows_core::IUnknown {
    fn from(value: &IWiaVideo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWiaVideo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWiaVideo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaVideo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaVideo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaVideo {}
impl ::core::fmt::Debug for IWiaVideo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaVideo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWiaVideo {
    type Vtable = IWiaVideo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd52920aa_db88_41f0_946c_e00dc0a19cfa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaVideo_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub PreviewVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbpreviewvisible: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetPreviewVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bpreviewvisible: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub ImagesDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrimagedirectory: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetImagesDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrimagedirectory: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub CreateVideoByWiaDevID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrwiadeviceid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, hwndparent: ::win32_foundation::HWND, bstretchtofitparent: ::win32_foundation::BOOL, bautobeginplayback: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub CreateVideoByDevNum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uidevicenumber: u32, hwndparent: ::win32_foundation::HWND, bstretchtofitparent: ::win32_foundation::BOOL, bautobeginplayback: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub CreateVideoByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfriendlyname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, hwndparent: ::win32_foundation::HWND, bstretchtofitparent: ::win32_foundation::BOOL, bautobeginplayback: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub DestroyVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Play: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TakePicture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrnewimagefilename: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ResizeVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstretchtofitparent: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetCurrentState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut WIAVIDEO_STATE) -> ::windows_core::HRESULT,
}
pub const LAMP_ERR: u32 = 2048u32;
pub const LANDSCAPE: u32 = 1u32;
pub const LANSCAPE: u32 = 1u32;
pub const LEFT_JUSTIFIED: u32 = 0u32;
pub const LIGHT_SOURCE_DETECT_READY: u32 = 4u32;
pub const LIGHT_SOURCE_NEGATIVE: u32 = 4u32;
pub const LIGHT_SOURCE_POSITIVE: u32 = 2u32;
pub const LIGHT_SOURCE_PRESENT: u32 = 2u32;
pub const LIGHT_SOURCE_PRESENT_DETECT: u32 = 1u32;
pub const LIGHT_SOURCE_READY: u32 = 8u32;
pub const LIGHT_SOURCE_SELECT: u32 = 1u32;
pub const MAX_ANSI_CHAR: u32 = 255u32;
pub const MAX_IO_HANDLES: u32 = 16u32;
pub const MAX_RESERVED: u32 = 4u32;
pub const MCRO_ERROR_GENERAL_ERROR: u32 = 0u32;
pub const MCRO_ERROR_OFFLINE: u32 = 5u32;
pub const MCRO_ERROR_PAPER_EMPTY: u32 = 4u32;
pub const MCRO_ERROR_PAPER_JAM: u32 = 2u32;
pub const MCRO_ERROR_PAPER_PROBLEM: u32 = 3u32;
pub const MCRO_ERROR_USER_INTERVENTION: u32 = 6u32;
pub const MCRO_STATUS_OK: u32 = 1u32;
pub const MICR_READER: u32 = 1048576u32;
pub const MICR_READER_READY: u32 = 65536u32;
#[repr(C)]
pub struct MINIDRV_TRANSFER_CONTEXT {
    pub lSize: i32,
    pub lWidthInPixels: i32,
    pub lLines: i32,
    pub lDepth: i32,
    pub lXRes: i32,
    pub lYRes: i32,
    pub lCompression: i32,
    pub guidFormatID: ::windows_core::GUID,
    pub tymed: i32,
    pub hFile: isize,
    pub cbOffset: i32,
    pub lBufferSize: i32,
    pub lActiveBuffer: i32,
    pub lNumBuffers: i32,
    pub pBaseBuffer: *mut u8,
    pub pTransferBuffer: *mut u8,
    pub bTransferDataCB: ::win32_foundation::BOOL,
    pub bClassDrvAllocBuf: ::win32_foundation::BOOL,
    pub lClientAddress: isize,
    pub pIWiaMiniDrvCallBack: ::core::option::Option<IWiaMiniDrvCallBack>,
    pub lImageSize: i32,
    pub lHeaderSize: i32,
    pub lItemSize: i32,
    pub cbWidthInBytes: i32,
    pub lPage: i32,
    pub lCurIfdOffset: i32,
    pub lPrevIfdOffset: i32,
}
impl ::core::clone::Clone for MINIDRV_TRANSFER_CONTEXT {
    fn clone(&self) -> Self {
        Self {
            lSize: self.lSize,
            lWidthInPixels: self.lWidthInPixels,
            lLines: self.lLines,
            lDepth: self.lDepth,
            lXRes: self.lXRes,
            lYRes: self.lYRes,
            lCompression: self.lCompression,
            guidFormatID: self.guidFormatID,
            tymed: self.tymed,
            hFile: self.hFile,
            cbOffset: self.cbOffset,
            lBufferSize: self.lBufferSize,
            lActiveBuffer: self.lActiveBuffer,
            lNumBuffers: self.lNumBuffers,
            pBaseBuffer: self.pBaseBuffer,
            pTransferBuffer: self.pTransferBuffer,
            bTransferDataCB: self.bTransferDataCB,
            bClassDrvAllocBuf: self.bClassDrvAllocBuf,
            lClientAddress: self.lClientAddress,
            pIWiaMiniDrvCallBack: self.pIWiaMiniDrvCallBack.clone(),
            lImageSize: self.lImageSize,
            lHeaderSize: self.lHeaderSize,
            lItemSize: self.lItemSize,
            cbWidthInBytes: self.cbWidthInBytes,
            lPage: self.lPage,
            lCurIfdOffset: self.lCurIfdOffset,
            lPrevIfdOffset: self.lPrevIfdOffset,
        }
    }
}
impl ::core::fmt::Debug for MINIDRV_TRANSFER_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MINIDRV_TRANSFER_CONTEXT")
            .field("lSize", &self.lSize)
            .field("lWidthInPixels", &self.lWidthInPixels)
            .field("lLines", &self.lLines)
            .field("lDepth", &self.lDepth)
            .field("lXRes", &self.lXRes)
            .field("lYRes", &self.lYRes)
            .field("lCompression", &self.lCompression)
            .field("guidFormatID", &self.guidFormatID)
            .field("tymed", &self.tymed)
            .field("hFile", &self.hFile)
            .field("cbOffset", &self.cbOffset)
            .field("lBufferSize", &self.lBufferSize)
            .field("lActiveBuffer", &self.lActiveBuffer)
            .field("lNumBuffers", &self.lNumBuffers)
            .field("pBaseBuffer", &self.pBaseBuffer)
            .field("pTransferBuffer", &self.pTransferBuffer)
            .field("bTransferDataCB", &self.bTransferDataCB)
            .field("bClassDrvAllocBuf", &self.bClassDrvAllocBuf)
            .field("lClientAddress", &self.lClientAddress)
            .field("pIWiaMiniDrvCallBack", &self.pIWiaMiniDrvCallBack)
            .field("lImageSize", &self.lImageSize)
            .field("lHeaderSize", &self.lHeaderSize)
            .field("lItemSize", &self.lItemSize)
            .field("cbWidthInBytes", &self.cbWidthInBytes)
            .field("lPage", &self.lPage)
            .field("lCurIfdOffset", &self.lCurIfdOffset)
            .field("lPrevIfdOffset", &self.lPrevIfdOffset)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for MINIDRV_TRANSFER_CONTEXT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for MINIDRV_TRANSFER_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.lSize == other.lSize
            && self.lWidthInPixels == other.lWidthInPixels
            && self.lLines == other.lLines
            && self.lDepth == other.lDepth
            && self.lXRes == other.lXRes
            && self.lYRes == other.lYRes
            && self.lCompression == other.lCompression
            && self.guidFormatID == other.guidFormatID
            && self.tymed == other.tymed
            && self.hFile == other.hFile
            && self.cbOffset == other.cbOffset
            && self.lBufferSize == other.lBufferSize
            && self.lActiveBuffer == other.lActiveBuffer
            && self.lNumBuffers == other.lNumBuffers
            && self.pBaseBuffer == other.pBaseBuffer
            && self.pTransferBuffer == other.pTransferBuffer
            && self.bTransferDataCB == other.bTransferDataCB
            && self.bClassDrvAllocBuf == other.bClassDrvAllocBuf
            && self.lClientAddress == other.lClientAddress
            && self.pIWiaMiniDrvCallBack == other.pIWiaMiniDrvCallBack
            && self.lImageSize == other.lImageSize
            && self.lHeaderSize == other.lHeaderSize
            && self.lItemSize == other.lItemSize
            && self.cbWidthInBytes == other.cbWidthInBytes
            && self.lPage == other.lPage
            && self.lCurIfdOffset == other.lCurIfdOffset
            && self.lPrevIfdOffset == other.lPrevIfdOffset
    }
}
impl ::core::cmp::Eq for MINIDRV_TRANSFER_CONTEXT {}
impl ::core::default::Default for MINIDRV_TRANSFER_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const MIRRORED: u32 = 1u32;
pub const MULTIPLE_FEED: u32 = 512u32;
pub const NEXT_PAGE: u32 = 128u32;
pub const PAPER_JAM: u32 = 32u32;
pub const PATCH_CODE_READER: u32 = 524288u32;
pub const PATCH_CODE_READER_READY: u32 = 32768u32;
pub const PATH_COVER_UP: u32 = 16u32;
pub const PORTRAIT: u32 = 0u32;
pub const POWERMODE_BATTERY: u32 = 2u32;
pub const POWERMODE_LINE: u32 = 1u32;
pub const PREFEED: u32 = 256u32;
#[repr(C)]
pub struct RANGEVALUE {
    pub lMin: i32,
    pub lMax: i32,
    pub lStep: i32,
}
impl ::core::marker::Copy for RANGEVALUE {}
impl ::core::clone::Clone for RANGEVALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RANGEVALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RANGEVALUE").field("lMin", &self.lMin).field("lMax", &self.lMax).field("lStep", &self.lStep).finish()
    }
}
unsafe impl ::windows_core::Abi for RANGEVALUE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RANGEVALUE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RANGEVALUE>()) == 0 }
    }
}
impl ::core::cmp::Eq for RANGEVALUE {}
impl ::core::default::Default for RANGEVALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const RIGHT_JUSTIFIED: u32 = 2u32;
pub const ROT180: u32 = 2u32;
pub const ROT270: u32 = 3u32;
#[repr(C)]
pub struct SCANINFO {
    pub ADF: i32,
    pub TPA: i32,
    pub Endorser: i32,
    pub OpticalXResolution: i32,
    pub OpticalYResolution: i32,
    pub BedWidth: i32,
    pub BedHeight: i32,
    pub IntensityRange: RANGEVALUE,
    pub ContrastRange: RANGEVALUE,
    pub SupportedCompressionType: i32,
    pub SupportedDataTypes: i32,
    pub WidthPixels: i32,
    pub WidthBytes: i32,
    pub Lines: i32,
    pub DataType: i32,
    pub PixelBits: i32,
    pub Intensity: i32,
    pub Contrast: i32,
    pub Xresolution: i32,
    pub Yresolution: i32,
    pub Window: SCANWINDOW,
    pub DitherPattern: i32,
    pub Negative: i32,
    pub Mirror: i32,
    pub AutoBack: i32,
    pub ColorDitherPattern: i32,
    pub ToneMap: i32,
    pub Compression: i32,
    pub RawDataFormat: i32,
    pub RawPixelOrder: i32,
    pub bNeedDataAlignment: i32,
    pub DelayBetweenRead: i32,
    pub MaxBufferSize: i32,
    pub DeviceIOHandles: [::win32_foundation::HANDLE; 16],
    pub lReserved: [i32; 4],
    pub pMicroDriverContext: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SCANINFO {}
impl ::core::clone::Clone for SCANINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCANINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCANINFO")
            .field("ADF", &self.ADF)
            .field("TPA", &self.TPA)
            .field("Endorser", &self.Endorser)
            .field("OpticalXResolution", &self.OpticalXResolution)
            .field("OpticalYResolution", &self.OpticalYResolution)
            .field("BedWidth", &self.BedWidth)
            .field("BedHeight", &self.BedHeight)
            .field("IntensityRange", &self.IntensityRange)
            .field("ContrastRange", &self.ContrastRange)
            .field("SupportedCompressionType", &self.SupportedCompressionType)
            .field("SupportedDataTypes", &self.SupportedDataTypes)
            .field("WidthPixels", &self.WidthPixels)
            .field("WidthBytes", &self.WidthBytes)
            .field("Lines", &self.Lines)
            .field("DataType", &self.DataType)
            .field("PixelBits", &self.PixelBits)
            .field("Intensity", &self.Intensity)
            .field("Contrast", &self.Contrast)
            .field("Xresolution", &self.Xresolution)
            .field("Yresolution", &self.Yresolution)
            .field("Window", &self.Window)
            .field("DitherPattern", &self.DitherPattern)
            .field("Negative", &self.Negative)
            .field("Mirror", &self.Mirror)
            .field("AutoBack", &self.AutoBack)
            .field("ColorDitherPattern", &self.ColorDitherPattern)
            .field("ToneMap", &self.ToneMap)
            .field("Compression", &self.Compression)
            .field("RawDataFormat", &self.RawDataFormat)
            .field("RawPixelOrder", &self.RawPixelOrder)
            .field("bNeedDataAlignment", &self.bNeedDataAlignment)
            .field("DelayBetweenRead", &self.DelayBetweenRead)
            .field("MaxBufferSize", &self.MaxBufferSize)
            .field("DeviceIOHandles", &self.DeviceIOHandles)
            .field("lReserved", &self.lReserved)
            .field("pMicroDriverContext", &self.pMicroDriverContext)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for SCANINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCANINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCANINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCANINFO {}
impl ::core::default::Default for SCANINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const SCANMODE_FINALSCAN: u32 = 0u32;
pub const SCANMODE_PREVIEWSCAN: u32 = 1u32;
#[repr(C)]
pub struct SCANWINDOW {
    pub xPos: i32,
    pub yPos: i32,
    pub xExtent: i32,
    pub yExtent: i32,
}
impl ::core::marker::Copy for SCANWINDOW {}
impl ::core::clone::Clone for SCANWINDOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCANWINDOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCANWINDOW").field("xPos", &self.xPos).field("yPos", &self.yPos).field("xExtent", &self.xExtent).field("yExtent", &self.yExtent).finish()
    }
}
unsafe impl ::windows_core::Abi for SCANWINDOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCANWINDOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCANWINDOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCANWINDOW {}
impl ::core::default::Default for SCANWINDOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const SCAN_FINISHED: u32 = 30u32;
pub const SCAN_FIRST: u32 = 10u32;
pub const SCAN_NEXT: u32 = 20u32;
pub const SHELLEX_WIAUIEXTENSION_NAME: &str = "WiaDialogExtensionHandlers";
pub const STOR: u32 = 2048u32;
pub const STORAGE_FULL: u32 = 256u32;
pub const STORAGE_READY: u32 = 128u32;
pub const SUPPORT_BW: u32 = 2u32;
pub const SUPPORT_COLOR: u32 = 1u32;
pub const SUPPORT_GRAYSCALE: u32 = 4u32;
pub const TOP_JUSTIFIED: u32 = 0u32;
pub const TRANSPARENCY_DYNAMIC_FRAME_SUPPORT: u32 = 1u32;
pub const TRANSPARENCY_STATIC_FRAME_SUPPORT: u32 = 2u32;
#[repr(C)]
pub struct TWAIN_CAPABILITY {
    pub lSize: i32,
    pub lMSG: i32,
    pub lCapID: i32,
    pub lConType: i32,
    pub lRC: i32,
    pub lCC: i32,
    pub lDataSize: i32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for TWAIN_CAPABILITY {}
impl ::core::clone::Clone for TWAIN_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TWAIN_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TWAIN_CAPABILITY").field("lSize", &self.lSize).field("lMSG", &self.lMSG).field("lCapID", &self.lCapID).field("lConType", &self.lConType).field("lRC", &self.lRC).field("lCC", &self.lCC).field("lDataSize", &self.lDataSize).field("Data", &self.Data).finish()
    }
}
unsafe impl ::windows_core::Abi for TWAIN_CAPABILITY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TWAIN_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TWAIN_CAPABILITY>()) == 0 }
    }
}
impl ::core::cmp::Eq for TWAIN_CAPABILITY {}
impl ::core::default::Default for TWAIN_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const TYMED_CALLBACK: u32 = 128u32;
pub const TYMED_MULTIPAGE_CALLBACK: u32 = 512u32;
pub const TYMED_MULTIPAGE_FILE: u32 = 256u32;
#[repr(C)]
pub struct VAL {
    pub lVal: i32,
    pub dblVal: f64,
    pub pGuid: *mut ::windows_core::GUID,
    pub pScanInfo: *mut SCANINFO,
    pub handle: isize,
    pub ppButtonNames: *mut *mut u16,
    pub pHandle: *mut ::win32_foundation::HANDLE,
    pub lReserved: i32,
    pub szVal: [::win32_foundation::CHAR; 255],
}
impl ::core::marker::Copy for VAL {}
impl ::core::clone::Clone for VAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VAL").field("lVal", &self.lVal).field("dblVal", &self.dblVal).field("pGuid", &self.pGuid).field("pScanInfo", &self.pScanInfo).field("handle", &self.handle).field("ppButtonNames", &self.ppButtonNames).field("pHandle", &self.pHandle).field("lReserved", &self.lReserved).field("szVal", &self.szVal).finish()
    }
}
unsafe impl ::windows_core::Abi for VAL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VAL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VAL>()) == 0 }
    }
}
impl ::core::cmp::Eq for VAL {}
impl ::core::default::Default for VAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WHITEBALANCE_AUTO: u32 = 2u32;
pub const WHITEBALANCE_DAYLIGHT: u32 = 4u32;
pub const WHITEBALANCE_FLASH: u32 = 7u32;
pub const WHITEBALANCE_FLORESCENT: u32 = 5u32;
pub const WHITEBALANCE_MANUAL: u32 = 1u32;
pub const WHITEBALANCE_ONEPUSH_AUTO: u32 = 3u32;
pub const WHITEBALANCE_TUNGSTEN: u32 = 6u32;
#[repr(C)]
pub struct WIAS_CHANGED_VALUE_INFO {
    pub bChanged: ::win32_foundation::BOOL,
    pub vt: i32,
    pub Old: WIAS_CHANGED_VALUE_INFO_1,
    pub Current: WIAS_CHANGED_VALUE_INFO_0,
}
impl ::core::clone::Clone for WIAS_CHANGED_VALUE_INFO {
    fn clone(&self) -> Self {
        Self { bChanged: self.bChanged, vt: self.vt, Old: self.Old.clone(), Current: self.Current.clone() }
    }
}
unsafe impl ::windows_core::Abi for WIAS_CHANGED_VALUE_INFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for WIAS_CHANGED_VALUE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.bChanged == other.bChanged && self.vt == other.vt && self.Old == other.Old && self.Current == other.Current
    }
}
impl ::core::cmp::Eq for WIAS_CHANGED_VALUE_INFO {}
impl ::core::default::Default for WIAS_CHANGED_VALUE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union WIAS_CHANGED_VALUE_INFO_0 {
    pub lVal: i32,
    pub fltVal: f32,
    pub bstrVal: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>,
    pub guidVal: ::windows_core::GUID,
}
impl ::core::clone::Clone for WIAS_CHANGED_VALUE_INFO_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
unsafe impl ::windows_core::Abi for WIAS_CHANGED_VALUE_INFO_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for WIAS_CHANGED_VALUE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIAS_CHANGED_VALUE_INFO_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIAS_CHANGED_VALUE_INFO_0 {}
impl ::core::default::Default for WIAS_CHANGED_VALUE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union WIAS_CHANGED_VALUE_INFO_1 {
    pub lVal: i32,
    pub fltVal: f32,
    pub bstrVal: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>,
    pub guidVal: ::windows_core::GUID,
}
impl ::core::clone::Clone for WIAS_CHANGED_VALUE_INFO_1 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
unsafe impl ::windows_core::Abi for WIAS_CHANGED_VALUE_INFO_1 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for WIAS_CHANGED_VALUE_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIAS_CHANGED_VALUE_INFO_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIAS_CHANGED_VALUE_INFO_1 {}
impl ::core::default::Default for WIAS_CHANGED_VALUE_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WIAS_DOWN_SAMPLE_INFO {
    pub ulOriginalWidth: u32,
    pub ulOriginalHeight: u32,
    pub ulBitsPerPixel: u32,
    pub ulXRes: u32,
    pub ulYRes: u32,
    pub ulDownSampledWidth: u32,
    pub ulDownSampledHeight: u32,
    pub ulActualSize: u32,
    pub ulDestBufSize: u32,
    pub ulSrcBufSize: u32,
    pub pSrcBuffer: *mut u8,
    pub pDestBuffer: *mut u8,
}
impl ::core::marker::Copy for WIAS_DOWN_SAMPLE_INFO {}
impl ::core::clone::Clone for WIAS_DOWN_SAMPLE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIAS_DOWN_SAMPLE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIAS_DOWN_SAMPLE_INFO")
            .field("ulOriginalWidth", &self.ulOriginalWidth)
            .field("ulOriginalHeight", &self.ulOriginalHeight)
            .field("ulBitsPerPixel", &self.ulBitsPerPixel)
            .field("ulXRes", &self.ulXRes)
            .field("ulYRes", &self.ulYRes)
            .field("ulDownSampledWidth", &self.ulDownSampledWidth)
            .field("ulDownSampledHeight", &self.ulDownSampledHeight)
            .field("ulActualSize", &self.ulActualSize)
            .field("ulDestBufSize", &self.ulDestBufSize)
            .field("ulSrcBufSize", &self.ulSrcBufSize)
            .field("pSrcBuffer", &self.pSrcBuffer)
            .field("pDestBuffer", &self.pDestBuffer)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for WIAS_DOWN_SAMPLE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIAS_DOWN_SAMPLE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIAS_DOWN_SAMPLE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIAS_DOWN_SAMPLE_INFO {}
impl ::core::default::Default for WIAS_DOWN_SAMPLE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WIAS_ENDORSER_INFO {
    pub ulPageCount: u32,
    pub ulNumEndorserValues: u32,
    pub pEndorserValues: *mut WIAS_ENDORSER_VALUE,
}
impl ::core::marker::Copy for WIAS_ENDORSER_INFO {}
impl ::core::clone::Clone for WIAS_ENDORSER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIAS_ENDORSER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIAS_ENDORSER_INFO").field("ulPageCount", &self.ulPageCount).field("ulNumEndorserValues", &self.ulNumEndorserValues).field("pEndorserValues", &self.pEndorserValues).finish()
    }
}
unsafe impl ::windows_core::Abi for WIAS_ENDORSER_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIAS_ENDORSER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIAS_ENDORSER_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIAS_ENDORSER_INFO {}
impl ::core::default::Default for WIAS_ENDORSER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WIAS_ENDORSER_VALUE {
    pub wszTokenName: ::windows_core::PWSTR,
    pub wszValue: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for WIAS_ENDORSER_VALUE {}
impl ::core::clone::Clone for WIAS_ENDORSER_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIAS_ENDORSER_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIAS_ENDORSER_VALUE").field("wszTokenName", &self.wszTokenName).field("wszValue", &self.wszValue).finish()
    }
}
unsafe impl ::windows_core::Abi for WIAS_ENDORSER_VALUE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIAS_ENDORSER_VALUE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIAS_ENDORSER_VALUE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIAS_ENDORSER_VALUE {}
impl ::core::default::Default for WIAS_ENDORSER_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WIAU_DEBUG_TSTR: &str = "S";
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WIAVIDEO_STATE(pub i32);
pub const WIAVIDEO_NO_VIDEO: WIAVIDEO_STATE = WIAVIDEO_STATE(1i32);
pub const WIAVIDEO_CREATING_VIDEO: WIAVIDEO_STATE = WIAVIDEO_STATE(2i32);
pub const WIAVIDEO_VIDEO_CREATED: WIAVIDEO_STATE = WIAVIDEO_STATE(3i32);
pub const WIAVIDEO_VIDEO_PLAYING: WIAVIDEO_STATE = WIAVIDEO_STATE(4i32);
pub const WIAVIDEO_VIDEO_PAUSED: WIAVIDEO_STATE = WIAVIDEO_STATE(5i32);
pub const WIAVIDEO_DESTROYING_VIDEO: WIAVIDEO_STATE = WIAVIDEO_STATE(6i32);
impl ::core::marker::Copy for WIAVIDEO_STATE {}
impl ::core::clone::Clone for WIAVIDEO_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WIAVIDEO_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WIAVIDEO_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WIAVIDEO_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WIAVIDEO_STATE").field(&self.0).finish()
    }
}
pub const WIA_ACTION_EVENT: u32 = 2u32;
pub const WIA_ADVANCED_PREVIEW: u32 = 0u32;
pub const WIA_ALARM_BEEP1: u32 = 1u32;
pub const WIA_ALARM_BEEP10: u32 = 10u32;
pub const WIA_ALARM_BEEP2: u32 = 2u32;
pub const WIA_ALARM_BEEP3: u32 = 3u32;
pub const WIA_ALARM_BEEP4: u32 = 4u32;
pub const WIA_ALARM_BEEP5: u32 = 5u32;
pub const WIA_ALARM_BEEP6: u32 = 6u32;
pub const WIA_ALARM_BEEP7: u32 = 7u32;
pub const WIA_ALARM_BEEP8: u32 = 8u32;
pub const WIA_ALARM_BEEP9: u32 = 9u32;
pub const WIA_ALARM_NONE: u32 = 0u32;
pub const WIA_AUTO_CROP_DISABLED: u32 = 0u32;
pub const WIA_AUTO_CROP_MULTI: u32 = 2u32;
pub const WIA_AUTO_CROP_SINGLE: u32 = 1u32;
pub const WIA_AUTO_DESKEW_OFF: u32 = 1u32;
pub const WIA_AUTO_DESKEW_ON: u32 = 0u32;
#[repr(C)]
pub struct WIA_BARCODES {
    pub Tag: u32,
    pub Version: u32,
    pub Size: u32,
    pub Count: u32,
    pub Barcodes: [WIA_BARCODE_INFO; 1],
}
impl ::core::marker::Copy for WIA_BARCODES {}
impl ::core::clone::Clone for WIA_BARCODES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_BARCODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_BARCODES").field("Tag", &self.Tag).field("Version", &self.Version).field("Size", &self.Size).field("Count", &self.Count).field("Barcodes", &self.Barcodes).finish()
    }
}
unsafe impl ::windows_core::Abi for WIA_BARCODES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_BARCODES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_BARCODES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_BARCODES {}
impl ::core::default::Default for WIA_BARCODES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WIA_BARCODE_AUTO_SEARCH: u32 = 4u32;
pub const WIA_BARCODE_AZTEC: u32 = 36u32;
pub const WIA_BARCODE_CODABAR: u32 = 2u32;
pub const WIA_BARCODE_CODE128: u32 = 9u32;
pub const WIA_BARCODE_CODE128A: u32 = 10u32;
pub const WIA_BARCODE_CODE128B: u32 = 11u32;
pub const WIA_BARCODE_CODE128C: u32 = 12u32;
pub const WIA_BARCODE_CODE39: u32 = 5u32;
pub const WIA_BARCODE_CODE39_FULLASCII: u32 = 7u32;
pub const WIA_BARCODE_CODE39_MOD43: u32 = 6u32;
pub const WIA_BARCODE_CODE93: u32 = 8u32;
pub const WIA_BARCODE_CPCBINARY: u32 = 29u32;
pub const WIA_BARCODE_CUSTOMBASE: u32 = 32768u32;
pub const WIA_BARCODE_DATAMATRIX: u32 = 38u32;
pub const WIA_BARCODE_DATASTRIP: u32 = 39u32;
pub const WIA_BARCODE_EAN13: u32 = 17u32;
pub const WIA_BARCODE_EAN8: u32 = 16u32;
pub const WIA_BARCODE_EZCODE: u32 = 40u32;
pub const WIA_BARCODE_FIM: u32 = 30u32;
pub const WIA_BARCODE_GS1128: u32 = 13u32;
pub const WIA_BARCODE_GS1DATABAR: u32 = 14u32;
pub const WIA_BARCODE_HIGH_CAPACITY_COLOR: u32 = 26u32;
pub const WIA_BARCODE_HORIZONTAL_SEARCH: u32 = 0u32;
pub const WIA_BARCODE_HORIZONTAL_VERTICAL_SEARCH: u32 = 2u32;
#[repr(C)]
pub struct WIA_BARCODE_INFO {
    pub Size: u32,
    pub Type: u32,
    pub Page: u32,
    pub Confidence: u32,
    pub XOffset: u32,
    pub YOffset: u32,
    pub Rotation: u32,
    pub Length: u32,
    pub Text: [u16; 1],
}
impl ::core::marker::Copy for WIA_BARCODE_INFO {}
impl ::core::clone::Clone for WIA_BARCODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_BARCODE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_BARCODE_INFO").field("Size", &self.Size).field("Type", &self.Type).field("Page", &self.Page).field("Confidence", &self.Confidence).field("XOffset", &self.XOffset).field("YOffset", &self.YOffset).field("Rotation", &self.Rotation).field("Length", &self.Length).field("Text", &self.Text).finish()
    }
}
unsafe impl ::windows_core::Abi for WIA_BARCODE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_BARCODE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_BARCODE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_BARCODE_INFO {}
impl ::core::default::Default for WIA_BARCODE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WIA_BARCODE_INTELLIGENT_MAIL: u32 = 23u32;
pub const WIA_BARCODE_INTERLEAVED_2OF5: u32 = 4u32;
pub const WIA_BARCODE_ITF14: u32 = 15u32;
pub const WIA_BARCODE_JAN: u32 = 34u32;
pub const WIA_BARCODE_MAXICODE: u32 = 27u32;
pub const WIA_BARCODE_MSI: u32 = 33u32;
pub const WIA_BARCODE_NONINTERLEAVED_2OF5: u32 = 3u32;
pub const WIA_BARCODE_PDF417: u32 = 28u32;
pub const WIA_BARCODE_PHARMACODE: u32 = 31u32;
pub const WIA_BARCODE_PLANET: u32 = 22u32;
pub const WIA_BARCODE_PLESSEY: u32 = 32u32;
pub const WIA_BARCODE_POSTBAR: u32 = 24u32;
pub const WIA_BARCODE_POSTNETA: u32 = 18u32;
pub const WIA_BARCODE_POSTNETB: u32 = 19u32;
pub const WIA_BARCODE_POSTNETC: u32 = 20u32;
pub const WIA_BARCODE_POSTNET_DPBC: u32 = 21u32;
pub const WIA_BARCODE_QRCODE: u32 = 41u32;
pub const WIA_BARCODE_READER_AUTO: u32 = 1u32;
pub const WIA_BARCODE_READER_DISABLED: u32 = 0u32;
pub const WIA_BARCODE_READER_FEEDER_BACK: u32 = 4u32;
pub const WIA_BARCODE_READER_FEEDER_DUPLEX: u32 = 5u32;
pub const WIA_BARCODE_READER_FEEDER_FRONT: u32 = 3u32;
pub const WIA_BARCODE_READER_FLATBED: u32 = 2u32;
pub const WIA_BARCODE_RM4SCC: u32 = 25u32;
pub const WIA_BARCODE_SHOTCODE: u32 = 42u32;
pub const WIA_BARCODE_SMALLAZTEC: u32 = 37u32;
pub const WIA_BARCODE_SPARQCODE: u32 = 43u32;
pub const WIA_BARCODE_TELEPEN: u32 = 35u32;
pub const WIA_BARCODE_UPCA: u32 = 0u32;
pub const WIA_BARCODE_UPCE: u32 = 1u32;
pub const WIA_BARCODE_VERTICAL_HORIZONTAL_SEARCH: u32 = 3u32;
pub const WIA_BARCODE_VERTICAL_SEARCH: u32 = 1u32;
pub const WIA_BASIC_PREVIEW: u32 = 1u32;
pub const WIA_BLANK_PAGE_DETECTION_DISABLED: u32 = 0u32;
pub const WIA_BLANK_PAGE_DISCARD: u32 = 1u32;
pub const WIA_BLANK_PAGE_JOB_SEPARATOR: u32 = 2u32;
pub const WIA_CATEGORY_AUTO: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdefe5fd8_6c97_4dde_b11e_cb509b270e11);
pub const WIA_CATEGORY_BARCODE_READER: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x36e178a0_473f_494b_af8f_6c3f6d7486fc);
pub const WIA_CATEGORY_ENDORSER: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x47102cc3_127f_4771_adfc_991ab8ee1e97);
pub const WIA_CATEGORY_FEEDER: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfe131934_f84c_42ad_8da4_6129cddd7288);
pub const WIA_CATEGORY_FEEDER_BACK: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x61ca74d4_39db_42aa_89b1_8c19c9cd4c23);
pub const WIA_CATEGORY_FEEDER_FRONT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4823175c_3b28_487b_a7e6_eebc17614fd1);
pub const WIA_CATEGORY_FILM: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfcf65be7_3ce3_4473_af85_f5d37d21b68a);
pub const WIA_CATEGORY_FINISHED_FILE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xff2b77ca_cf84_432b_a735_3a130dde2a88);
pub const WIA_CATEGORY_FLATBED: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb607b1f_43f3_488b_855b_fb703ec342a6);
pub const WIA_CATEGORY_FOLDER: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc692a446_6f5a_481d_85bb_92e2e86fd30a);
pub const WIA_CATEGORY_IMPRINTER: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfc65016d_9202_43dd_91a7_64c2954cfb8b);
pub const WIA_CATEGORY_MICR_READER: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3b86c1ec_71bc_4645_b4d5_1b19da2be978);
pub const WIA_CATEGORY_PATCH_CODE_READER: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8faa1a6d_9c8a_42cd_98b3_ee9700cbc74f);
pub const WIA_CATEGORY_ROOT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf193526f_59b8_4a26_9888_e16e4f97ce10);
pub const WIA_CMD_BUILD_DEVICE_TREE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9cba5ce0_dbea_11d2_8416_00c04fa36145);
pub const WIA_CMD_CHANGE_DOCUMENT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x04e725b0_acae_11d2_a093_00c04f72dc3c);
pub const WIA_CMD_DELETE_ALL_ITEMS: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe208c170_acad_11d2_a093_00c04f72dc3c);
pub const WIA_CMD_DELETE_DEVICE_TREE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x73815942_dbea_11d2_8416_00c04fa36145);
pub const WIA_CMD_DIAGNOSTIC: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x10ff52f5_de04_4cf0_a5ad_691f8dce0141);
pub const WIA_CMD_FORMAT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc3a693aa_f788_4d34_a5b0_be7190759a24);
pub const WIA_CMD_PAUSE_FEEDER: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50985e4d_a5b2_4b71_9c95_6d7d7c469a43);
pub const WIA_CMD_START_FEEDER: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5a9df6c9_5f2d_4a39_9d6c_00456d047f00);
pub const WIA_CMD_STOP_FEEDER: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd847b06d_3905_459c_9509_9b29cdb691e7);
pub const WIA_CMD_SYNCHRONIZE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b26b7b2_acad_11d2_a093_00c04f72dc3c);
pub const WIA_CMD_TAKE_PICTURE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaf933cac_acad_11d2_a093_00c04f72dc3c);
pub const WIA_CMD_UNLOAD_DOCUMENT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1f3b3d8e_acae_11d2_a093_00c04f72dc3c);
pub const WIA_COLOR_DROP_BLUE: u32 = 3u32;
pub const WIA_COLOR_DROP_DISABLED: u32 = 0u32;
pub const WIA_COLOR_DROP_GREEN: u32 = 2u32;
pub const WIA_COLOR_DROP_RED: u32 = 1u32;
pub const WIA_COLOR_DROP_RGB: u32 = 4u32;
pub const WIA_COMPRESSION_AUTO: u32 = 100u32;
pub const WIA_COMPRESSION_BI_RLE4: u32 = 1u32;
pub const WIA_COMPRESSION_BI_RLE8: u32 = 2u32;
pub const WIA_COMPRESSION_G3: u32 = 3u32;
pub const WIA_COMPRESSION_G4: u32 = 4u32;
pub const WIA_COMPRESSION_JBIG: u32 = 6u32;
pub const WIA_COMPRESSION_JPEG: u32 = 5u32;
pub const WIA_COMPRESSION_JPEG2K: u32 = 7u32;
pub const WIA_COMPRESSION_NONE: u32 = 0u32;
pub const WIA_COMPRESSION_PNG: u32 = 8u32;
pub const WIA_DATA_AUTO: u32 = 100u32;
#[repr(C)]
pub struct WIA_DATA_CALLBACK_HEADER {
    pub lSize: i32,
    pub guidFormatID: ::windows_core::GUID,
    pub lBufferSize: i32,
    pub lPageCount: i32,
}
impl ::core::marker::Copy for WIA_DATA_CALLBACK_HEADER {}
impl ::core::clone::Clone for WIA_DATA_CALLBACK_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_DATA_CALLBACK_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_DATA_CALLBACK_HEADER").field("lSize", &self.lSize).field("guidFormatID", &self.guidFormatID).field("lBufferSize", &self.lBufferSize).field("lPageCount", &self.lPageCount).finish()
    }
}
unsafe impl ::windows_core::Abi for WIA_DATA_CALLBACK_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_DATA_CALLBACK_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_DATA_CALLBACK_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_DATA_CALLBACK_HEADER {}
impl ::core::default::Default for WIA_DATA_CALLBACK_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WIA_DATA_COLOR: u32 = 3u32;
pub const WIA_DATA_COLOR_DITHER: u32 = 5u32;
pub const WIA_DATA_COLOR_THRESHOLD: u32 = 4u32;
pub const WIA_DATA_DITHER: u32 = 1u32;
pub const WIA_DATA_GRAYSCALE: u32 = 2u32;
pub const WIA_DATA_RAW_BGR: u32 = 7u32;
pub const WIA_DATA_RAW_CMY: u32 = 10u32;
pub const WIA_DATA_RAW_CMYK: u32 = 11u32;
pub const WIA_DATA_RAW_RGB: u32 = 6u32;
pub const WIA_DATA_RAW_YUV: u32 = 8u32;
pub const WIA_DATA_RAW_YUVK: u32 = 9u32;
pub const WIA_DATA_THRESHOLD: u32 = 0u32;
#[repr(C)]
pub struct WIA_DATA_TRANSFER_INFO {
    pub ulSize: u32,
    pub ulSection: u32,
    pub ulBufferSize: u32,
    pub bDoubleBuffer: ::win32_foundation::BOOL,
    pub ulReserved1: u32,
    pub ulReserved2: u32,
    pub ulReserved3: u32,
}
impl ::core::marker::Copy for WIA_DATA_TRANSFER_INFO {}
impl ::core::clone::Clone for WIA_DATA_TRANSFER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_DATA_TRANSFER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_DATA_TRANSFER_INFO").field("ulSize", &self.ulSize).field("ulSection", &self.ulSection).field("ulBufferSize", &self.ulBufferSize).field("bDoubleBuffer", &self.bDoubleBuffer).field("ulReserved1", &self.ulReserved1).field("ulReserved2", &self.ulReserved2).field("ulReserved3", &self.ulReserved3).finish()
    }
}
unsafe impl ::windows_core::Abi for WIA_DATA_TRANSFER_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_DATA_TRANSFER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_DATA_TRANSFER_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_DATA_TRANSFER_INFO {}
impl ::core::default::Default for WIA_DATA_TRANSFER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WIA_DEPTH_AUTO: u32 = 0u32;
pub const WIA_DEVICE_COMMANDS: u32 = 1u32;
pub const WIA_DEVICE_CONNECTED: u32 = 1u32;
pub const WIA_DEVICE_DIALOG_SINGLE_IMAGE: u32 = 2u32;
pub const WIA_DEVICE_DIALOG_USE_COMMON_UI: u32 = 4u32;
pub const WIA_DEVICE_EVENTS: u32 = 2u32;
pub const WIA_DEVICE_NOT_CONNECTED: u32 = 0u32;
pub const WIA_DEVINFO_ENUM_ALL: u32 = 15u32;
pub const WIA_DEVINFO_ENUM_LOCAL: u32 = 16u32;
#[repr(C)]
pub struct WIA_DEV_CAP {
    pub guid: ::windows_core::GUID,
    pub ulFlags: u32,
    pub bstrName: ::win32_foundation::BSTR,
    pub bstrDescription: ::win32_foundation::BSTR,
    pub bstrIcon: ::win32_foundation::BSTR,
    pub bstrCommandline: ::win32_foundation::BSTR,
}
impl ::core::clone::Clone for WIA_DEV_CAP {
    fn clone(&self) -> Self {
        Self {
            guid: self.guid,
            ulFlags: self.ulFlags,
            bstrName: self.bstrName.clone(),
            bstrDescription: self.bstrDescription.clone(),
            bstrIcon: self.bstrIcon.clone(),
            bstrCommandline: self.bstrCommandline.clone(),
        }
    }
}
impl ::core::fmt::Debug for WIA_DEV_CAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_DEV_CAP").field("guid", &self.guid).field("ulFlags", &self.ulFlags).field("bstrName", &self.bstrName).field("bstrDescription", &self.bstrDescription).field("bstrIcon", &self.bstrIcon).field("bstrCommandline", &self.bstrCommandline).finish()
    }
}
unsafe impl ::windows_core::Abi for WIA_DEV_CAP {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for WIA_DEV_CAP {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid && self.ulFlags == other.ulFlags && self.bstrName == other.bstrName && self.bstrDescription == other.bstrDescription && self.bstrIcon == other.bstrIcon && self.bstrCommandline == other.bstrCommandline
    }
}
impl ::core::cmp::Eq for WIA_DEV_CAP {}
impl ::core::default::Default for WIA_DEV_CAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WIA_DEV_CAP_DRV {
    pub guid: *mut ::windows_core::GUID,
    pub ulFlags: u32,
    pub wszName: ::windows_core::PWSTR,
    pub wszDescription: ::windows_core::PWSTR,
    pub wszIcon: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for WIA_DEV_CAP_DRV {}
impl ::core::clone::Clone for WIA_DEV_CAP_DRV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_DEV_CAP_DRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_DEV_CAP_DRV").field("guid", &self.guid).field("ulFlags", &self.ulFlags).field("wszName", &self.wszName).field("wszDescription", &self.wszDescription).field("wszIcon", &self.wszIcon).finish()
    }
}
unsafe impl ::windows_core::Abi for WIA_DEV_CAP_DRV {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_DEV_CAP_DRV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_DEV_CAP_DRV>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_DEV_CAP_DRV {}
impl ::core::default::Default for WIA_DEV_CAP_DRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WIA_DIP_BAUDRATE: u32 = 12u32;
pub const WIA_DIP_BAUDRATE_STR: &str = "BaudRate";
pub const WIA_DIP_DEV_DESC: u32 = 4u32;
pub const WIA_DIP_DEV_DESC_STR: &str = "Description";
pub const WIA_DIP_DEV_ID: u32 = 2u32;
pub const WIA_DIP_DEV_ID_STR: &str = "Unique Device ID";
pub const WIA_DIP_DEV_NAME: u32 = 7u32;
pub const WIA_DIP_DEV_NAME_STR: &str = "Name";
pub const WIA_DIP_DEV_TYPE: u32 = 5u32;
pub const WIA_DIP_DEV_TYPE_STR: &str = "Type";
pub const WIA_DIP_DRIVER_VERSION: u32 = 15u32;
pub const WIA_DIP_DRIVER_VERSION_STR: &str = "Driver Version";
pub const WIA_DIP_FIRST: u32 = 2u32;
pub const WIA_DIP_HW_CONFIG: u32 = 11u32;
pub const WIA_DIP_HW_CONFIG_STR: &str = "Hardware Configuration";
pub const WIA_DIP_PNP_ID: u32 = 16u32;
pub const WIA_DIP_PNP_ID_STR: &str = "PnP ID String";
pub const WIA_DIP_PORT_NAME: u32 = 6u32;
pub const WIA_DIP_PORT_NAME_STR: &str = "Port";
pub const WIA_DIP_REMOTE_DEV_ID: u32 = 9u32;
pub const WIA_DIP_REMOTE_DEV_ID_STR: &str = "Remote Device ID";
pub const WIA_DIP_SERVER_NAME: u32 = 8u32;
pub const WIA_DIP_SERVER_NAME_STR: &str = "Server";
pub const WIA_DIP_STI_DRIVER_VERSION: u32 = 17u32;
pub const WIA_DIP_STI_DRIVER_VERSION_STR: &str = "STI Driver Version";
pub const WIA_DIP_STI_GEN_CAPABILITIES: u32 = 13u32;
pub const WIA_DIP_STI_GEN_CAPABILITIES_STR: &str = "STI Generic Capabilities";
pub const WIA_DIP_UI_CLSID: u32 = 10u32;
pub const WIA_DIP_UI_CLSID_STR: &str = "UI Class ID";
pub const WIA_DIP_VEND_DESC: u32 = 3u32;
pub const WIA_DIP_VEND_DESC_STR: &str = "Manufacturer";
pub const WIA_DIP_WIA_VERSION: u32 = 14u32;
pub const WIA_DIP_WIA_VERSION_STR: &str = "WIA Version";
#[repr(C)]
pub struct WIA_DITHER_PATTERN_DATA {
    pub lSize: i32,
    pub bstrPatternName: ::win32_foundation::BSTR,
    pub lPatternWidth: i32,
    pub lPatternLength: i32,
    pub cbPattern: i32,
    pub pbPattern: *mut u8,
}
impl ::core::clone::Clone for WIA_DITHER_PATTERN_DATA {
    fn clone(&self) -> Self {
        Self {
            lSize: self.lSize,
            bstrPatternName: self.bstrPatternName.clone(),
            lPatternWidth: self.lPatternWidth,
            lPatternLength: self.lPatternLength,
            cbPattern: self.cbPattern,
            pbPattern: self.pbPattern,
        }
    }
}
impl ::core::fmt::Debug for WIA_DITHER_PATTERN_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_DITHER_PATTERN_DATA").field("lSize", &self.lSize).field("bstrPatternName", &self.bstrPatternName).field("lPatternWidth", &self.lPatternWidth).field("lPatternLength", &self.lPatternLength).field("cbPattern", &self.cbPattern).field("pbPattern", &self.pbPattern).finish()
    }
}
unsafe impl ::windows_core::Abi for WIA_DITHER_PATTERN_DATA {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for WIA_DITHER_PATTERN_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.lSize == other.lSize && self.bstrPatternName == other.bstrPatternName && self.lPatternWidth == other.lPatternWidth && self.lPatternLength == other.lPatternLength && self.cbPattern == other.cbPattern && self.pbPattern == other.pbPattern
    }
}
impl ::core::cmp::Eq for WIA_DITHER_PATTERN_DATA {}
impl ::core::default::Default for WIA_DITHER_PATTERN_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WIA_DONT_SHOW_PREVIEW_CONTROL: u32 = 1u32;
pub const WIA_DONT_USE_SEGMENTATION_FILTER: u32 = 1u32;
pub const WIA_DPA_CONNECT_STATUS: u32 = 1027u32;
pub const WIA_DPA_CONNECT_STATUS_STR: &str = "Connect Status";
pub const WIA_DPA_DEVICE_TIME: u32 = 1028u32;
pub const WIA_DPA_DEVICE_TIME_STR: &str = "Device Time";
pub const WIA_DPA_FIRMWARE_VERSION: u32 = 1026u32;
pub const WIA_DPA_FIRMWARE_VERSION_STR: &str = "Firmware Version";
pub const WIA_DPC_ARTIST: u32 = 2091u32;
pub const WIA_DPC_ARTIST_STR: &str = "Artist";
pub const WIA_DPC_BATTERY_STATUS: u32 = 2065u32;
pub const WIA_DPC_BATTERY_STATUS_STR: &str = "Battery Status";
pub const WIA_DPC_BURST_INTERVAL: u32 = 2075u32;
pub const WIA_DPC_BURST_INTERVAL_STR: &str = "Burst Interval";
pub const WIA_DPC_BURST_NUMBER: u32 = 2076u32;
pub const WIA_DPC_BURST_NUMBER_STR: &str = "Burst Number";
pub const WIA_DPC_CAPTURE_DELAY: u32 = 2082u32;
pub const WIA_DPC_CAPTURE_DELAY_STR: &str = "Capture Delay";
pub const WIA_DPC_CAPTURE_MODE: u32 = 2081u32;
pub const WIA_DPC_CAPTURE_MODE_STR: &str = "Capture Mode";
pub const WIA_DPC_COMPRESSION_SETTING: u32 = 2071u32;
pub const WIA_DPC_COMPRESSION_SETTING_STR: &str = "Compression Setting";
pub const WIA_DPC_CONTRAST: u32 = 2080u32;
pub const WIA_DPC_CONTRAST_STR: &str = "Contrast";
pub const WIA_DPC_COPYRIGHT_INFO: u32 = 2092u32;
pub const WIA_DPC_COPYRIGHT_INFO_STR: &str = "Copyright Info";
pub const WIA_DPC_DIGITAL_ZOOM: u32 = 2078u32;
pub const WIA_DPC_DIGITAL_ZOOM_STR: &str = "Digital Zoom";
pub const WIA_DPC_DIMENSION: u32 = 2070u32;
pub const WIA_DPC_DIMENSION_STR: &str = "Dimension";
pub const WIA_DPC_EFFECT_MODE: u32 = 2077u32;
pub const WIA_DPC_EFFECT_MODE_STR: &str = "Effect Mode";
pub const WIA_DPC_EXPOSURE_COMP: u32 = 2053u32;
pub const WIA_DPC_EXPOSURE_COMP_STR: &str = "Exposure Compensation";
pub const WIA_DPC_EXPOSURE_INDEX: u32 = 2083u32;
pub const WIA_DPC_EXPOSURE_INDEX_STR: &str = "Exposure Index";
pub const WIA_DPC_EXPOSURE_METERING_MODE: u32 = 2084u32;
pub const WIA_DPC_EXPOSURE_METERING_MODE_STR: &str = "Exposure Metering Mode";
pub const WIA_DPC_EXPOSURE_MODE: u32 = 2052u32;
pub const WIA_DPC_EXPOSURE_MODE_STR: &str = "Exposure Mode";
pub const WIA_DPC_EXPOSURE_TIME: u32 = 2054u32;
pub const WIA_DPC_EXPOSURE_TIME_STR: &str = "Exposure Time";
pub const WIA_DPC_FLASH_MODE: u32 = 2056u32;
pub const WIA_DPC_FLASH_MODE_STR: &str = "Flash Mode";
pub const WIA_DPC_FNUMBER: u32 = 2055u32;
pub const WIA_DPC_FNUMBER_STR: &str = "F Number";
pub const WIA_DPC_FOCAL_LENGTH: u32 = 2087u32;
pub const WIA_DPC_FOCAL_LENGTH_STR: &str = "Focus Length";
pub const WIA_DPC_FOCUS_DISTANCE: u32 = 2086u32;
pub const WIA_DPC_FOCUS_DISTANCE_STR: &str = "Focus Distance";
pub const WIA_DPC_FOCUS_MANUAL_DIST: u32 = 2058u32;
pub const WIA_DPC_FOCUS_MANUAL_DIST_STR: &str = "Focus Manual Dist";
pub const WIA_DPC_FOCUS_METERING: u32 = 2072u32;
pub const WIA_DPC_FOCUS_METERING_MODE: u32 = 2085u32;
pub const WIA_DPC_FOCUS_METERING_MODE_STR: &str = "Focus Metering Mode";
pub const WIA_DPC_FOCUS_METERING_STR: &str = "Focus Metering Mode";
pub const WIA_DPC_FOCUS_MODE: u32 = 2057u32;
pub const WIA_DPC_FOCUS_MODE_STR: &str = "Focus Mode";
pub const WIA_DPC_PAN_POSITION: u32 = 2060u32;
pub const WIA_DPC_PAN_POSITION_STR: &str = "Pan Position";
pub const WIA_DPC_PICTURES_REMAINING: u32 = 2051u32;
pub const WIA_DPC_PICTURES_REMAINING_STR: &str = "Pictures Remaining";
pub const WIA_DPC_PICTURES_TAKEN: u32 = 2050u32;
pub const WIA_DPC_PICTURES_TAKEN_STR: &str = "Pictures Taken";
pub const WIA_DPC_PICT_HEIGHT: u32 = 2069u32;
pub const WIA_DPC_PICT_HEIGHT_STR: &str = "Picture Height";
pub const WIA_DPC_PICT_WIDTH: u32 = 2068u32;
pub const WIA_DPC_PICT_WIDTH_STR: &str = "Picture Width";
pub const WIA_DPC_POWER_MODE: u32 = 2064u32;
pub const WIA_DPC_POWER_MODE_STR: &str = "Power Mode";
pub const WIA_DPC_RGB_GAIN: u32 = 2088u32;
pub const WIA_DPC_RGB_GAIN_STR: &str = "RGB Gain";
pub const WIA_DPC_SHARPNESS: u32 = 2079u32;
pub const WIA_DPC_SHARPNESS_STR: &str = "Sharpness";
pub const WIA_DPC_THUMB_HEIGHT: u32 = 2067u32;
pub const WIA_DPC_THUMB_HEIGHT_STR: &str = "Thumbnail Height";
pub const WIA_DPC_THUMB_WIDTH: u32 = 2066u32;
pub const WIA_DPC_THUMB_WIDTH_STR: &str = "Thumbnail Width";
pub const WIA_DPC_TILT_POSITION: u32 = 2061u32;
pub const WIA_DPC_TILT_POSITION_STR: &str = "Tilt Position";
pub const WIA_DPC_TIMELAPSE_INTERVAL: u32 = 2073u32;
pub const WIA_DPC_TIMELAPSE_INTERVAL_STR: &str = "Timelapse Interval";
pub const WIA_DPC_TIMELAPSE_NUMBER: u32 = 2074u32;
pub const WIA_DPC_TIMELAPSE_NUMBER_STR: &str = "Timelapse Number";
pub const WIA_DPC_TIMER_MODE: u32 = 2062u32;
pub const WIA_DPC_TIMER_MODE_STR: &str = "Timer Mode";
pub const WIA_DPC_TIMER_VALUE: u32 = 2063u32;
pub const WIA_DPC_TIMER_VALUE_STR: &str = "Timer Value";
pub const WIA_DPC_UPLOAD_URL: u32 = 2090u32;
pub const WIA_DPC_UPLOAD_URL_STR: &str = "Upload URL";
pub const WIA_DPC_WHITE_BALANCE: u32 = 2089u32;
pub const WIA_DPC_WHITE_BALANCE_STR: &str = "White Balance";
pub const WIA_DPC_ZOOM_POSITION: u32 = 2059u32;
pub const WIA_DPC_ZOOM_POSITION_STR: &str = "Zoom Position";
pub const WIA_DPF_FIRST: u32 = 3330u32;
pub const WIA_DPF_MOUNT_POINT: u32 = 3330u32;
pub const WIA_DPF_MOUNT_POINT_STR: &str = "Directory mount point";
pub const WIA_DPS_DEVICE_ID: u32 = 3114u32;
pub const WIA_DPS_DEVICE_ID_STR: &str = "Device ID";
pub const WIA_DPS_DITHER_PATTERN_DATA: u32 = 3085u32;
pub const WIA_DPS_DITHER_PATTERN_DATA_STR: &str = "Dither Pattern Data";
pub const WIA_DPS_DITHER_SELECT: u32 = 3084u32;
pub const WIA_DPS_DITHER_SELECT_STR: &str = "Dither Select";
pub const WIA_DPS_DOCUMENT_HANDLING_CAPABILITIES: u32 = 3086u32;
pub const WIA_DPS_DOCUMENT_HANDLING_CAPABILITIES_STR: &str = "Document Handling Capabilities";
pub const WIA_DPS_DOCUMENT_HANDLING_CAPACITY: u32 = 3089u32;
pub const WIA_DPS_DOCUMENT_HANDLING_CAPACITY_STR: &str = "Document Handling Capacity";
pub const WIA_DPS_DOCUMENT_HANDLING_SELECT: u32 = 3088u32;
pub const WIA_DPS_DOCUMENT_HANDLING_SELECT_STR: &str = "Document Handling Select";
pub const WIA_DPS_DOCUMENT_HANDLING_STATUS: u32 = 3087u32;
pub const WIA_DPS_DOCUMENT_HANDLING_STATUS_STR: &str = "Document Handling Status";
pub const WIA_DPS_ENDORSER_CHARACTERS: u32 = 3092u32;
pub const WIA_DPS_ENDORSER_CHARACTERS_STR: &str = "Endorser Characters";
pub const WIA_DPS_ENDORSER_STRING: u32 = 3093u32;
pub const WIA_DPS_ENDORSER_STRING_STR: &str = "Endorser String";
pub const WIA_DPS_FILTER_SELECT: u32 = 3083u32;
pub const WIA_DPS_FILTER_SELECT_STR: &str = "Filter Select";
pub const WIA_DPS_FIRST: u32 = 3074u32;
pub const WIA_DPS_GLOBAL_IDENTITY: u32 = 3115u32;
pub const WIA_DPS_GLOBAL_IDENTITY_STR: &str = "Global Identity";
pub const WIA_DPS_HORIZONTAL_BED_REGISTRATION: u32 = 3079u32;
pub const WIA_DPS_HORIZONTAL_BED_REGISTRATION_STR: &str = "Horizontal Bed Registration";
pub const WIA_DPS_HORIZONTAL_BED_SIZE: u32 = 3074u32;
pub const WIA_DPS_HORIZONTAL_BED_SIZE_STR: &str = "Horizontal Bed Size";
pub const WIA_DPS_HORIZONTAL_SHEET_FEED_SIZE: u32 = 3076u32;
pub const WIA_DPS_HORIZONTAL_SHEET_FEED_SIZE_STR: &str = "Horizontal Sheet Feed Size";
pub const WIA_DPS_MAX_SCAN_TIME: u32 = 3095u32;
pub const WIA_DPS_MAX_SCAN_TIME_STR: &str = "Max Scan Time";
pub const WIA_DPS_MIN_HORIZONTAL_SHEET_FEED_SIZE: u32 = 3104u32;
pub const WIA_DPS_MIN_HORIZONTAL_SHEET_FEED_SIZE_STR: &str = "Minimum Horizontal Sheet Feed Size";
pub const WIA_DPS_MIN_VERTICAL_SHEET_FEED_SIZE: u32 = 3105u32;
pub const WIA_DPS_MIN_VERTICAL_SHEET_FEED_SIZE_STR: &str = "Minimum Vertical Sheet Feed Size";
pub const WIA_DPS_OPTICAL_XRES: u32 = 3090u32;
pub const WIA_DPS_OPTICAL_XRES_STR: &str = "Horizontal Optical Resolution";
pub const WIA_DPS_OPTICAL_YRES: u32 = 3091u32;
pub const WIA_DPS_OPTICAL_YRES_STR: &str = "Vertical Optical Resolution";
pub const WIA_DPS_PAD_COLOR: u32 = 3082u32;
pub const WIA_DPS_PAD_COLOR_STR: &str = "Pad Color";
pub const WIA_DPS_PAGES: u32 = 3096u32;
pub const WIA_DPS_PAGES_STR: &str = "Pages";
pub const WIA_DPS_PAGE_HEIGHT: u32 = 3099u32;
pub const WIA_DPS_PAGE_HEIGHT_STR: &str = "Page Height";
pub const WIA_DPS_PAGE_SIZE: u32 = 3097u32;
pub const WIA_DPS_PAGE_SIZE_STR: &str = "Page Size";
pub const WIA_DPS_PAGE_WIDTH: u32 = 3098u32;
pub const WIA_DPS_PAGE_WIDTH_STR: &str = "Page Width";
pub const WIA_DPS_PLATEN_COLOR: u32 = 3081u32;
pub const WIA_DPS_PLATEN_COLOR_STR: &str = "Platen Color";
pub const WIA_DPS_PREVIEW: u32 = 3100u32;
pub const WIA_DPS_PREVIEW_STR: &str = "Preview";
pub const WIA_DPS_SCAN_AHEAD_PAGES: u32 = 3094u32;
pub const WIA_DPS_SCAN_AHEAD_PAGES_STR: &str = "Scan Ahead Pages";
pub const WIA_DPS_SCAN_AVAILABLE_ITEM: u32 = 3116u32;
pub const WIA_DPS_SCAN_AVAILABLE_ITEM_STR: &str = "Scan Available Item";
pub const WIA_DPS_SERVICE_ID: u32 = 3113u32;
pub const WIA_DPS_SERVICE_ID_STR: &str = "Service ID";
pub const WIA_DPS_SHEET_FEEDER_REGISTRATION: u32 = 3078u32;
pub const WIA_DPS_SHEET_FEEDER_REGISTRATION_STR: &str = "Sheet Feeder Registration";
pub const WIA_DPS_SHOW_PREVIEW_CONTROL: u32 = 3103u32;
pub const WIA_DPS_SHOW_PREVIEW_CONTROL_STR: &str = "Show preview control";
pub const WIA_DPS_TRANSPARENCY: u32 = 3101u32;
pub const WIA_DPS_TRANSPARENCY_CAPABILITIES: u32 = 3106u32;
pub const WIA_DPS_TRANSPARENCY_CAPABILITIES_STR: &str = "Transparency Adapter Capabilities";
pub const WIA_DPS_TRANSPARENCY_SELECT: u32 = 3102u32;
pub const WIA_DPS_TRANSPARENCY_SELECT_STR: &str = "Transparency Adapter Select";
pub const WIA_DPS_TRANSPARENCY_STATUS: u32 = 3107u32;
pub const WIA_DPS_TRANSPARENCY_STATUS_STR: &str = "Transparency Adapter Status";
pub const WIA_DPS_TRANSPARENCY_STR: &str = "Transparency Adapter";
pub const WIA_DPS_USER_NAME: u32 = 3112u32;
pub const WIA_DPS_USER_NAME_STR: &str = "User Name";
pub const WIA_DPS_VERTICAL_BED_REGISTRATION: u32 = 3080u32;
pub const WIA_DPS_VERTICAL_BED_REGISTRATION_STR: &str = "Vertical Bed Registration";
pub const WIA_DPS_VERTICAL_BED_SIZE: u32 = 3075u32;
pub const WIA_DPS_VERTICAL_BED_SIZE_STR: &str = "Vertical Bed Size";
pub const WIA_DPS_VERTICAL_SHEET_FEED_SIZE: u32 = 3077u32;
pub const WIA_DPS_VERTICAL_SHEET_FEED_SIZE_STR: &str = "Vertical Sheet Feed Size";
pub const WIA_DPV_DSHOW_DEVICE_PATH: u32 = 3588u32;
pub const WIA_DPV_DSHOW_DEVICE_PATH_STR: &str = "Directshow Device Path";
pub const WIA_DPV_IMAGES_DIRECTORY: u32 = 3587u32;
pub const WIA_DPV_IMAGES_DIRECTORY_STR: &str = "Images Directory";
pub const WIA_DPV_LAST_PICTURE_TAKEN: u32 = 3586u32;
pub const WIA_DPV_LAST_PICTURE_TAKEN_STR: &str = "Last Picture Taken";
pub const WIA_ENDORSER_TOK_DATE: &str = "$DATE$";
pub const WIA_ENDORSER_TOK_DAY: &str = "$DAY$";
pub const WIA_ENDORSER_TOK_MONTH: &str = "$MONTH$";
pub const WIA_ENDORSER_TOK_PAGE_COUNT: &str = "$PAGE_COUNT$";
pub const WIA_ENDORSER_TOK_TIME: &str = "$TIME$";
pub const WIA_ENDORSER_TOK_YEAR: &str = "$YEAR$";
pub const WIA_ERROR_BUSY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145320954i32);
pub const WIA_ERROR_COVER_OPEN: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145320944i32);
pub const WIA_ERROR_DESTINATION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145320942i32);
pub const WIA_ERROR_DEVICE_COMMUNICATION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145320950i32);
pub const WIA_ERROR_DEVICE_LOCKED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145320947i32);
pub const WIA_ERROR_EXCEPTION_IN_DRIVER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145320946i32);
pub const WIA_ERROR_GENERAL_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145320959i32);
pub const WIA_ERROR_INCORRECT_HARDWARE_SETTING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145320948i32);
pub const WIA_ERROR_INVALID_COMMAND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145320949i32);
pub const WIA_ERROR_INVALID_DRIVER_RESPONSE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145320945i32);
pub const WIA_ERROR_ITEM_DELETED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145320951i32);
pub const WIA_ERROR_LAMP_OFF: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145320943i32);
pub const WIA_ERROR_MAXIMUM_PRINTER_ENDORSER_COUNTER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145320939i32);
pub const WIA_ERROR_MULTI_FEED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145320940i32);
pub const WIA_ERROR_NETWORK_RESERVATION_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145320941i32);
pub const WIA_ERROR_OFFLINE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145320955i32);
pub const WIA_ERROR_PAPER_EMPTY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145320957i32);
pub const WIA_ERROR_PAPER_JAM: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145320958i32);
pub const WIA_ERROR_PAPER_PROBLEM: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145320956i32);
pub const WIA_ERROR_USER_INTERVENTION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145320952i32);
pub const WIA_ERROR_WARMING_UP: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145320953i32);
pub const WIA_EVENT_CANCEL_IO: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc860f7b8_9ccd_41ea_bbbf_4dd09c5b1795);
pub const WIA_EVENT_COVER_CLOSED: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6714a1e6_e285_468c_9b8c_da7dc4cbaa05);
pub const WIA_EVENT_COVER_OPEN: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x19a12136_fa1c_4f66_900f_8f914ec74ec9);
pub const WIA_EVENT_DEVICE_CONNECTED: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa28bbade_64b6_11d2_a231_00c04fa31809);
pub const WIA_EVENT_DEVICE_CONNECTED_STR: &str = "Device Connected";
pub const WIA_EVENT_DEVICE_DISCONNECTED: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x143e4e83_6497_11d2_a231_00c04fa31809);
pub const WIA_EVENT_DEVICE_DISCONNECTED_STR: &str = "Device Disconnected";
pub const WIA_EVENT_DEVICE_NOT_READY: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd8962d7e_e4dc_4b4d_ba29_668a87f42e6f);
pub const WIA_EVENT_DEVICE_READY: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7523ec6c_988b_419e_9a0a_425ac31b37dc);
pub const WIA_EVENT_FEEDER_EMPTIED: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe70b4b82_6dda_46bb_8ff9_53ceb1a03e35);
pub const WIA_EVENT_FEEDER_LOADED: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcc8d701e_9aba_481d_bf74_78f763dc342a);
pub const WIA_EVENT_FLATBED_LID_CLOSED: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf879af0f_9b29_4283_ad95_d412164d39a9);
pub const WIA_EVENT_FLATBED_LID_OPEN: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xba0a0623_437d_4f03_a97d_7793b123113c);
pub const WIA_EVENT_HANDLER_NO_ACTION: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe0372b7d_e115_4525_bc55_b629e68c745a);
pub const WIA_EVENT_HANDLER_PROMPT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f4baad0_4d59_4fcd_b213_783ce7a92f22);
pub const WIA_EVENT_ITEM_CREATED: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4c8f4ef5_e14f_11d2_b326_00c04f68ce61);
pub const WIA_EVENT_ITEM_DELETED: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d22a559_e14f_11d2_b326_00c04f68ce61);
pub const WIA_EVENT_POWER_RESUME: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x618f153e_f686_4350_9634_4115a304830c);
pub const WIA_EVENT_POWER_SUSPEND: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa0922ff9_c3b4_411c_9e29_03a66993d2be);
pub const WIA_EVENT_SCAN_EMAIL_IMAGE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc686dcee_54f2_419e_9a27_2fc7f2e98f9e);
pub const WIA_EVENT_SCAN_FAX_IMAGE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc00eb793_8c6e_11d2_977a_0000f87a926f);
pub const WIA_EVENT_SCAN_FILM_IMAGE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b2b662c_6185_438c_b68b_e39ee25e71cb);
pub const WIA_EVENT_SCAN_IMAGE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa6c5a715_8c6e_11d2_977a_0000f87a926f);
pub const WIA_EVENT_SCAN_IMAGE2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfc4767c1_c8b3_48a2_9cfa_2e90cb3d3590);
pub const WIA_EVENT_SCAN_IMAGE3: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x154e27be_b617_4653_acc5_0fd7bd4c65ce);
pub const WIA_EVENT_SCAN_IMAGE4: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa65b704a_7f3c_4447_a75d_8a26dfca1fdf);
pub const WIA_EVENT_SCAN_OCR_IMAGE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9d095b89_37d6_4877_afed_62a297dc6dbe);
pub const WIA_EVENT_SCAN_PRINT_IMAGE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb441f425_8c6e_11d2_977a_0000f87a926f);
pub const WIA_EVENT_STI_PROXY: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd711f81f_1f0d_422d_8641_927d1b93e5e5);
pub const WIA_EVENT_STORAGE_CREATED: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x353308b2_fe73_46c8_895e_fa4551ccc85a);
pub const WIA_EVENT_STORAGE_DELETED: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e41e75e_9390_44c5_9a51_e47019e390cf);
pub const WIA_EVENT_TREE_UPDATED: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc9859b91_4ab2_4cd6_a1fc_582eec55e585);
pub const WIA_EVENT_VOLUME_INSERT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9638bbfd_d1bd_11d2_b31f_00c04f68ce61);
#[repr(C)]
pub struct WIA_EXTENDED_TRANSFER_INFO {
    pub ulSize: u32,
    pub ulMinBufferSize: u32,
    pub ulOptimalBufferSize: u32,
    pub ulMaxBufferSize: u32,
    pub ulNumBuffers: u32,
}
impl ::core::marker::Copy for WIA_EXTENDED_TRANSFER_INFO {}
impl ::core::clone::Clone for WIA_EXTENDED_TRANSFER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_EXTENDED_TRANSFER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_EXTENDED_TRANSFER_INFO").field("ulSize", &self.ulSize).field("ulMinBufferSize", &self.ulMinBufferSize).field("ulOptimalBufferSize", &self.ulOptimalBufferSize).field("ulMaxBufferSize", &self.ulMaxBufferSize).field("ulNumBuffers", &self.ulNumBuffers).finish()
    }
}
unsafe impl ::windows_core::Abi for WIA_EXTENDED_TRANSFER_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_EXTENDED_TRANSFER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_EXTENDED_TRANSFER_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_EXTENDED_TRANSFER_INFO {}
impl ::core::default::Default for WIA_EXTENDED_TRANSFER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WIA_FEEDER_CONTROL_AUTO: u32 = 0u32;
pub const WIA_FEEDER_CONTROL_MANUAL: u32 = 1u32;
pub const WIA_FILM_BW_NEGATIVE: u32 = 2u32;
pub const WIA_FILM_COLOR_NEGATIVE: u32 = 1u32;
pub const WIA_FILM_COLOR_SLIDE: u32 = 0u32;
pub const WIA_FINAL_SCAN: u32 = 0u32;
pub const WIA_FLAG_NOM: u32 = 0u32;
pub const WIA_FLAG_NUM_ELEMS: u32 = 2u32;
pub const WIA_FLAG_VALUES: u32 = 1u32;
#[repr(C)]
pub struct WIA_FORMAT_INFO {
    pub guidFormatID: ::windows_core::GUID,
    pub lTymed: i32,
}
impl ::core::marker::Copy for WIA_FORMAT_INFO {}
impl ::core::clone::Clone for WIA_FORMAT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_FORMAT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_FORMAT_INFO").field("guidFormatID", &self.guidFormatID).field("lTymed", &self.lTymed).finish()
    }
}
unsafe impl ::windows_core::Abi for WIA_FORMAT_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_FORMAT_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_FORMAT_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_FORMAT_INFO {}
impl ::core::default::Default for WIA_FORMAT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WIA_IMAGEPROC_FILTER_STR: &str = "ImageProcessingFilter";
pub const WIA_INTENT_BEST_PREVIEW: u32 = 262144u32;
pub const WIA_INTENT_IMAGE_TYPE_COLOR: u32 = 1u32;
pub const WIA_INTENT_IMAGE_TYPE_GRAYSCALE: u32 = 2u32;
pub const WIA_INTENT_IMAGE_TYPE_MASK: u32 = 15u32;
pub const WIA_INTENT_IMAGE_TYPE_TEXT: u32 = 4u32;
pub const WIA_INTENT_MAXIMIZE_QUALITY: u32 = 131072u32;
pub const WIA_INTENT_MINIMIZE_SIZE: u32 = 65536u32;
pub const WIA_INTENT_NONE: u32 = 0u32;
pub const WIA_INTENT_SIZE_MASK: u32 = 983040u32;
pub const WIA_IPA_ACCESS_RIGHTS: u32 = 4102u32;
pub const WIA_IPA_ACCESS_RIGHTS_STR: &str = "Access Rights";
pub const WIA_IPA_APP_COLOR_MAPPING: u32 = 4121u32;
pub const WIA_IPA_APP_COLOR_MAPPING_STR: &str = "Application Applies Color Mapping";
pub const WIA_IPA_BITS_PER_CHANNEL: u32 = 4110u32;
pub const WIA_IPA_BITS_PER_CHANNEL_STR: &str = "Bits Per Channel";
pub const WIA_IPA_BUFFER_SIZE: u32 = 4118u32;
pub const WIA_IPA_BUFFER_SIZE_STR: &str = "Buffer Size";
pub const WIA_IPA_BYTES_PER_LINE: u32 = 4113u32;
pub const WIA_IPA_BYTES_PER_LINE_STR: &str = "Bytes Per Line";
pub const WIA_IPA_CHANNELS_PER_PIXEL: u32 = 4109u32;
pub const WIA_IPA_CHANNELS_PER_PIXEL_STR: &str = "Channels Per Pixel";
pub const WIA_IPA_COLOR_PROFILE: u32 = 4117u32;
pub const WIA_IPA_COLOR_PROFILE_STR: &str = "Color Profiles";
pub const WIA_IPA_COMPRESSION: u32 = 4107u32;
pub const WIA_IPA_COMPRESSION_STR: &str = "Compression";
pub const WIA_IPA_DATATYPE: u32 = 4103u32;
pub const WIA_IPA_DATATYPE_STR: &str = "Data Type";
pub const WIA_IPA_DEPTH: u32 = 4104u32;
pub const WIA_IPA_DEPTH_STR: &str = "Bits Per Pixel";
pub const WIA_IPA_FILENAME_EXTENSION: u32 = 4123u32;
pub const WIA_IPA_FILENAME_EXTENSION_STR: &str = "Filename extension";
pub const WIA_IPA_FIRST: u32 = 4098u32;
pub const WIA_IPA_FORMAT: u32 = 4106u32;
pub const WIA_IPA_FORMAT_STR: &str = "Format";
pub const WIA_IPA_FULL_ITEM_NAME: u32 = 4099u32;
pub const WIA_IPA_FULL_ITEM_NAME_STR: &str = "Full Item Name";
pub const WIA_IPA_GAMMA_CURVES: u32 = 4115u32;
pub const WIA_IPA_GAMMA_CURVES_STR: &str = "Gamma Curves";
pub const WIA_IPA_ICM_PROFILE_NAME: u32 = 4120u32;
pub const WIA_IPA_ICM_PROFILE_NAME_STR: &str = "Color Profile Name";
pub const WIA_IPA_ITEMS_STORED: u32 = 4127u32;
pub const WIA_IPA_ITEMS_STORED_STR: &str = "Items Stored";
pub const WIA_IPA_ITEM_CATEGORY: u32 = 4125u32;
pub const WIA_IPA_ITEM_CATEGORY_STR: &str = "Item Category";
pub const WIA_IPA_ITEM_FLAGS: u32 = 4101u32;
pub const WIA_IPA_ITEM_FLAGS_STR: &str = "Item Flags";
pub const WIA_IPA_ITEM_NAME: u32 = 4098u32;
pub const WIA_IPA_ITEM_NAME_STR: &str = "Item Name";
pub const WIA_IPA_ITEM_SIZE: u32 = 4116u32;
pub const WIA_IPA_ITEM_SIZE_STR: &str = "Item Size";
pub const WIA_IPA_ITEM_TIME: u32 = 4100u32;
pub const WIA_IPA_ITEM_TIME_STR: &str = "Item Time Stamp";
pub const WIA_IPA_MIN_BUFFER_SIZE: u32 = 4118u32;
pub const WIA_IPA_MIN_BUFFER_SIZE_STR: &str = "Buffer Size";
pub const WIA_IPA_NUMBER_OF_LINES: u32 = 4114u32;
pub const WIA_IPA_NUMBER_OF_LINES_STR: &str = "Number of Lines";
pub const WIA_IPA_PIXELS_PER_LINE: u32 = 4112u32;
pub const WIA_IPA_PIXELS_PER_LINE_STR: &str = "Pixels Per Line";
pub const WIA_IPA_PLANAR: u32 = 4111u32;
pub const WIA_IPA_PLANAR_STR: &str = "Planar";
pub const WIA_IPA_PREFERRED_FORMAT: u32 = 4105u32;
pub const WIA_IPA_PREFERRED_FORMAT_STR: &str = "Preferred Format";
pub const WIA_IPA_PROP_STREAM_COMPAT_ID: u32 = 4122u32;
pub const WIA_IPA_PROP_STREAM_COMPAT_ID_STR: &str = "Stream Compatibility ID";
pub const WIA_IPA_RAW_BITS_PER_CHANNEL: u32 = 4128u32;
pub const WIA_IPA_RAW_BITS_PER_CHANNEL_STR: &str = "Raw Bits Per Channel";
pub const WIA_IPA_REGION_TYPE: u32 = 4119u32;
pub const WIA_IPA_REGION_TYPE_STR: &str = "Region Type";
pub const WIA_IPA_SUPPRESS_PROPERTY_PAGE: u32 = 4124u32;
pub const WIA_IPA_SUPPRESS_PROPERTY_PAGE_STR: &str = "Suppress a property page";
pub const WIA_IPA_TYMED: u32 = 4108u32;
pub const WIA_IPA_TYMED_STR: &str = "Media Type";
pub const WIA_IPA_UPLOAD_ITEM_SIZE: u32 = 4126u32;
pub const WIA_IPA_UPLOAD_ITEM_SIZE_STR: &str = "Upload Item Size";
pub const WIA_IPC_AUDIO_AVAILABLE: u32 = 5125u32;
pub const WIA_IPC_AUDIO_AVAILABLE_STR: &str = "Audio Available";
pub const WIA_IPC_AUDIO_DATA: u32 = 5127u32;
pub const WIA_IPC_AUDIO_DATA_FORMAT: u32 = 5126u32;
pub const WIA_IPC_AUDIO_DATA_FORMAT_STR: &str = "Audio Format";
pub const WIA_IPC_AUDIO_DATA_STR: &str = "Audio Data";
pub const WIA_IPC_FIRST: u32 = 5122u32;
pub const WIA_IPC_NUM_PICT_PER_ROW: u32 = 5128u32;
pub const WIA_IPC_NUM_PICT_PER_ROW_STR: &str = "Pictures per Row";
pub const WIA_IPC_SEQUENCE: u32 = 5129u32;
pub const WIA_IPC_SEQUENCE_STR: &str = "Sequence Number";
pub const WIA_IPC_THUMBNAIL: u32 = 5122u32;
pub const WIA_IPC_THUMBNAIL_STR: &str = "Thumbnail Data";
pub const WIA_IPC_THUMB_HEIGHT: u32 = 5124u32;
pub const WIA_IPC_THUMB_HEIGHT_STR: &str = "Thumbnail Height";
pub const WIA_IPC_THUMB_WIDTH: u32 = 5123u32;
pub const WIA_IPC_THUMB_WIDTH_STR: &str = "Thumbnail Width";
pub const WIA_IPC_TIMEDELAY: u32 = 5130u32;
pub const WIA_IPC_TIMEDELAY_STR: &str = "Time Delay";
pub const WIA_IPS_ALARM: u32 = 4185u32;
pub const WIA_IPS_ALARM_STR: &str = "Alarm";
pub const WIA_IPS_AUTO_CROP: u32 = 4170u32;
pub const WIA_IPS_AUTO_CROP_STR: &str = "Auto-Crop";
pub const WIA_IPS_AUTO_DESKEW: u32 = 3107u32;
pub const WIA_IPS_AUTO_DESKEW_STR: &str = "Automatic Deskew";
pub const WIA_IPS_BARCODE_READER: u32 = 4150u32;
pub const WIA_IPS_BARCODE_READER_STR: &str = "Barcode Reader";
pub const WIA_IPS_BARCODE_SEARCH_DIRECTION: u32 = 4152u32;
pub const WIA_IPS_BARCODE_SEARCH_DIRECTION_STR: &str = "Barcode Search Direction";
pub const WIA_IPS_BARCODE_SEARCH_TIMEOUT: u32 = 4154u32;
pub const WIA_IPS_BARCODE_SEARCH_TIMEOUT_STR: &str = "Barcode Search Timeout";
pub const WIA_IPS_BLANK_PAGES: u32 = 4167u32;
pub const WIA_IPS_BLANK_PAGES_SENSITIVITY: u32 = 4192u32;
pub const WIA_IPS_BLANK_PAGES_SENSITIVITY_STR: &str = "Blank Pages Sensitivity";
pub const WIA_IPS_BLANK_PAGES_STR: &str = "Blank Pages";
pub const WIA_IPS_BRIGHTNESS: u32 = 6154u32;
pub const WIA_IPS_BRIGHTNESS_STR: &str = "Brightness";
pub const WIA_IPS_COLOR_DROP: u32 = 4176u32;
pub const WIA_IPS_COLOR_DROP_BLUE: u32 = 4179u32;
pub const WIA_IPS_COLOR_DROP_BLUE_STR: &str = "Color Drop Blue";
pub const WIA_IPS_COLOR_DROP_GREEN: u32 = 4178u32;
pub const WIA_IPS_COLOR_DROP_GREEN_STR: &str = "Color Drop Green";
pub const WIA_IPS_COLOR_DROP_MULTI: u32 = 4191u32;
pub const WIA_IPS_COLOR_DROP_MULTI_STR: &str = "Color Drop Multiple";
pub const WIA_IPS_COLOR_DROP_RED: u32 = 4177u32;
pub const WIA_IPS_COLOR_DROP_RED_STR: &str = "Color Drop Red";
pub const WIA_IPS_COLOR_DROP_STR: &str = "Color Drop";
pub const WIA_IPS_CONTRAST: u32 = 6155u32;
pub const WIA_IPS_CONTRAST_STR: &str = "Contrast";
pub const WIA_IPS_CUR_INTENT: u32 = 6146u32;
pub const WIA_IPS_CUR_INTENT_STR: &str = "Current Intent";
pub const WIA_IPS_DESKEW_X: u32 = 6162u32;
pub const WIA_IPS_DESKEW_X_STR: &str = "DeskewX";
pub const WIA_IPS_DESKEW_Y: u32 = 6163u32;
pub const WIA_IPS_DESKEW_Y_STR: &str = "DeskewY";
pub const WIA_IPS_DOCUMENT_HANDLING_SELECT: u32 = 3088u32;
pub const WIA_IPS_DOCUMENT_HANDLING_SELECT_STR: &str = "Document Handling Select";
pub const WIA_IPS_ENABLED_BARCODE_TYPES: u32 = 4156u32;
pub const WIA_IPS_ENABLED_BARCODE_TYPES_STR: &str = "Enabled Barcode Types";
pub const WIA_IPS_ENABLED_PATCH_CODE_TYPES: u32 = 4163u32;
pub const WIA_IPS_ENABLED_PATCH_CODE_TYPES_STR: &str = "Enabled Path Code Types";
pub const WIA_IPS_FEEDER_CONTROL: u32 = 4182u32;
pub const WIA_IPS_FEEDER_CONTROL_STR: &str = "Feeder Control";
pub const WIA_IPS_FILM_NODE_NAME: u32 = 4129u32;
pub const WIA_IPS_FILM_NODE_NAME_STR: &str = "Film Node Name";
pub const WIA_IPS_FILM_SCAN_MODE: u32 = 3104u32;
pub const WIA_IPS_FILM_SCAN_MODE_STR: &str = "Film Scan Mode";
pub const WIA_IPS_FIRST: u32 = 6146u32;
pub const WIA_IPS_INVERT: u32 = 6160u32;
pub const WIA_IPS_INVERT_STR: &str = "Invert";
pub const WIA_IPS_JOB_SEPARATORS: u32 = 4165u32;
pub const WIA_IPS_JOB_SEPARATORS_STR: &str = "Job Separators";
pub const WIA_IPS_LAMP: u32 = 3105u32;
pub const WIA_IPS_LAMP_AUTO_OFF: u32 = 3106u32;
pub const WIA_IPS_LAMP_AUTO_OFF_STR: &str = "Lamp Auto Off";
pub const WIA_IPS_LAMP_STR: &str = "Lamp";
pub const WIA_IPS_LONG_DOCUMENT: u32 = 4166u32;
pub const WIA_IPS_LONG_DOCUMENT_STR: &str = "Long Document";
pub const WIA_IPS_MAXIMUM_BARCODES_PER_PAGE: u32 = 4151u32;
pub const WIA_IPS_MAXIMUM_BARCODES_PER_PAGE_STR: &str = "Maximum Barcodes Per Page";
pub const WIA_IPS_MAXIMUM_BARCODE_SEARCH_RETRIES: u32 = 4153u32;
pub const WIA_IPS_MAXIMUM_BARCODE_SEARCH_RETRIES_STR: &str = "Barcode Search Retries";
pub const WIA_IPS_MAX_HORIZONTAL_SIZE: u32 = 6165u32;
pub const WIA_IPS_MAX_HORIZONTAL_SIZE_STR: &str = "Maximum Horizontal Scan Size";
pub const WIA_IPS_MAX_VERTICAL_SIZE: u32 = 6166u32;
pub const WIA_IPS_MAX_VERTICAL_SIZE_STR: &str = "Maximum Vertical Scan Size";
pub const WIA_IPS_MICR_READER: u32 = 4164u32;
pub const WIA_IPS_MICR_READER_STR: &str = "MICR Reader";
pub const WIA_IPS_MIN_HORIZONTAL_SIZE: u32 = 6167u32;
pub const WIA_IPS_MIN_HORIZONTAL_SIZE_STR: &str = "Minimum Horizontal Scan Size";
pub const WIA_IPS_MIN_VERTICAL_SIZE: u32 = 6168u32;
pub const WIA_IPS_MIN_VERTICAL_SIZE_STR: &str = "Minimum Vertical Scan Size";
pub const WIA_IPS_MIRROR: u32 = 6158u32;
pub const WIA_IPS_MIRROR_STR: &str = "Mirror";
pub const WIA_IPS_MULTI_FEED: u32 = 4168u32;
pub const WIA_IPS_MULTI_FEED_DETECT_METHOD: u32 = 4193u32;
pub const WIA_IPS_MULTI_FEED_DETECT_METHOD_STR: &str = "Multi-Feed Detection Method";
pub const WIA_IPS_MULTI_FEED_SENSITIVITY: u32 = 4169u32;
pub const WIA_IPS_MULTI_FEED_SENSITIVITY_STR: &str = "Multi-Feed Sensitivity";
pub const WIA_IPS_MULTI_FEED_STR: &str = "Multi-Feed";
pub const WIA_IPS_OPTICAL_XRES: u32 = 3090u32;
pub const WIA_IPS_OPTICAL_XRES_STR: &str = "Horizontal Optical Resolution";
pub const WIA_IPS_OPTICAL_YRES: u32 = 3091u32;
pub const WIA_IPS_OPTICAL_YRES_STR: &str = "Vertical Optical Resolution";
pub const WIA_IPS_ORIENTATION: u32 = 6156u32;
pub const WIA_IPS_ORIENTATION_STR: &str = "Orientation";
pub const WIA_IPS_OVER_SCAN: u32 = 4171u32;
pub const WIA_IPS_OVER_SCAN_BOTTOM: u32 = 4175u32;
pub const WIA_IPS_OVER_SCAN_BOTTOM_STR: &str = "Overscan Bottom";
pub const WIA_IPS_OVER_SCAN_LEFT: u32 = 4172u32;
pub const WIA_IPS_OVER_SCAN_LEFT_STR: &str = "Overscan Left";
pub const WIA_IPS_OVER_SCAN_RIGHT: u32 = 4173u32;
pub const WIA_IPS_OVER_SCAN_RIGHT_STR: &str = "Overscan Right";
pub const WIA_IPS_OVER_SCAN_STR: &str = "Overscan";
pub const WIA_IPS_OVER_SCAN_TOP: u32 = 4174u32;
pub const WIA_IPS_OVER_SCAN_TOP_STR: &str = "Overscan Top";
pub const WIA_IPS_PAGES: u32 = 3096u32;
pub const WIA_IPS_PAGES_STR: &str = "Pages";
pub const WIA_IPS_PAGE_HEIGHT: u32 = 3099u32;
pub const WIA_IPS_PAGE_HEIGHT_STR: &str = "Page Height";
pub const WIA_IPS_PAGE_SIZE: u32 = 3097u32;
pub const WIA_IPS_PAGE_SIZE_STR: &str = "Page Size";
pub const WIA_IPS_PAGE_WIDTH: u32 = 3098u32;
pub const WIA_IPS_PAGE_WIDTH_STR: &str = "Page Width";
pub const WIA_IPS_PATCH_CODE_READER: u32 = 4157u32;
pub const WIA_IPS_PATCH_CODE_READER_STR: &str = "Patch Code Reader";
pub const WIA_IPS_PHOTOMETRIC_INTERP: u32 = 6153u32;
pub const WIA_IPS_PHOTOMETRIC_INTERP_STR: &str = "Photometric Interpretation";
pub const WIA_IPS_PREVIEW: u32 = 3100u32;
pub const WIA_IPS_PREVIEW_STR: &str = "Preview";
pub const WIA_IPS_PREVIEW_TYPE: u32 = 3111u32;
pub const WIA_IPS_PREVIEW_TYPE_STR: &str = "Preview Type";
pub const WIA_IPS_PRINTER_ENDORSER: u32 = 4130u32;
pub const WIA_IPS_PRINTER_ENDORSER_CHARACTER_ROTATION: u32 = 4187u32;
pub const WIA_IPS_PRINTER_ENDORSER_CHARACTER_ROTATION_STR: &str = "Printer/Endorser Character Rotation";
pub const WIA_IPS_PRINTER_ENDORSER_COUNTER: u32 = 4132u32;
pub const WIA_IPS_PRINTER_ENDORSER_COUNTER_DIGITS: u32 = 4190u32;
pub const WIA_IPS_PRINTER_ENDORSER_COUNTER_DIGITS_STR: &str = "Printer/Endorser Counter Digits";
pub const WIA_IPS_PRINTER_ENDORSER_COUNTER_STR: &str = "Printer/Endorser Counter";
pub const WIA_IPS_PRINTER_ENDORSER_FONT_TYPE: u32 = 4184u32;
pub const WIA_IPS_PRINTER_ENDORSER_FONT_TYPE_STR: &str = "Printer/Endorser Font Type";
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS: u32 = 4142u32;
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_DOWNLOAD: u32 = 4149u32;
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_DOWNLOAD_STR: &str = "Printer/Endorser Graphics Download";
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MAX_HEIGHT: u32 = 4147u32;
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MAX_HEIGHT_STR: &str = "Printer/Endorser Graphics Maximum Height";
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MAX_WIDTH: u32 = 4145u32;
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MAX_WIDTH_STR: &str = "Printer/Endorser Graphics Maximum Width";
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MIN_HEIGHT: u32 = 4146u32;
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MIN_HEIGHT_STR: &str = "Printer/Endorser Graphics Minimum Height";
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MIN_WIDTH: u32 = 4144u32;
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MIN_WIDTH_STR: &str = "Printer/Endorser Graphics Minimum Width";
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_POSITION: u32 = 4143u32;
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_POSITION_STR: &str = "Printer/Endorser Graphics Position";
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_STR: &str = "Printer/Endorser Graphics";
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_UPLOAD: u32 = 4148u32;
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_UPLOAD_STR: &str = "Printer/Endorser Graphics Upload";
pub const WIA_IPS_PRINTER_ENDORSER_INK: u32 = 4186u32;
pub const WIA_IPS_PRINTER_ENDORSER_INK_STR: &str = "Printer/Endorser Ink";
pub const WIA_IPS_PRINTER_ENDORSER_MAX_CHARACTERS: u32 = 4188u32;
pub const WIA_IPS_PRINTER_ENDORSER_MAX_CHARACTERS_STR: &str = "Printer/Endorser Maximum Characters";
pub const WIA_IPS_PRINTER_ENDORSER_MAX_GRAPHICS: u32 = 4189u32;
pub const WIA_IPS_PRINTER_ENDORSER_MAX_GRAPHICS_STR: &str = "Printer/Endorser Maximum Graphics";
pub const WIA_IPS_PRINTER_ENDORSER_NUM_LINES: u32 = 4136u32;
pub const WIA_IPS_PRINTER_ENDORSER_NUM_LINES_STR: &str = "Printer/Endorser Lines";
pub const WIA_IPS_PRINTER_ENDORSER_ORDER: u32 = 4131u32;
pub const WIA_IPS_PRINTER_ENDORSER_ORDER_STR: &str = "Printer/Endorser Order";
pub const WIA_IPS_PRINTER_ENDORSER_PADDING: u32 = 4183u32;
pub const WIA_IPS_PRINTER_ENDORSER_PADDING_STR: &str = "Printer/Endorser Padding";
pub const WIA_IPS_PRINTER_ENDORSER_STEP: u32 = 4133u32;
pub const WIA_IPS_PRINTER_ENDORSER_STEP_STR: &str = "Printer/Endorser Step";
pub const WIA_IPS_PRINTER_ENDORSER_STR: &str = "Printer/Endorser";
pub const WIA_IPS_PRINTER_ENDORSER_STRING: u32 = 4137u32;
pub const WIA_IPS_PRINTER_ENDORSER_STRING_STR: &str = "Printer/Endorser String";
pub const WIA_IPS_PRINTER_ENDORSER_TEXT_DOWNLOAD: u32 = 4141u32;
pub const WIA_IPS_PRINTER_ENDORSER_TEXT_DOWNLOAD_STR: &str = "Printer/Endorser Text Download";
pub const WIA_IPS_PRINTER_ENDORSER_TEXT_UPLOAD: u32 = 4140u32;
pub const WIA_IPS_PRINTER_ENDORSER_TEXT_UPLOAD_STR: &str = "Printer/Endorser Text Upload";
pub const WIA_IPS_PRINTER_ENDORSER_VALID_CHARACTERS: u32 = 4138u32;
pub const WIA_IPS_PRINTER_ENDORSER_VALID_CHARACTERS_STR: &str = "Printer/Endorser Valid Characters";
pub const WIA_IPS_PRINTER_ENDORSER_VALID_FORMAT_SPECIFIERS: u32 = 4139u32;
pub const WIA_IPS_PRINTER_ENDORSER_VALID_FORMAT_SPECIFIERS_STR: &str = "Printer/Endorser Valid Format Specifiers";
pub const WIA_IPS_PRINTER_ENDORSER_XOFFSET: u32 = 4134u32;
pub const WIA_IPS_PRINTER_ENDORSER_XOFFSET_STR: &str = "Printer/Endorser Horizontal Offset";
pub const WIA_IPS_PRINTER_ENDORSER_YOFFSET: u32 = 4135u32;
pub const WIA_IPS_PRINTER_ENDORSER_YOFFSET_STR: &str = "Printer/Endorser Vertical Offset";
pub const WIA_IPS_ROTATION: u32 = 6157u32;
pub const WIA_IPS_ROTATION_STR: &str = "Rotation";
pub const WIA_IPS_SCAN_AHEAD: u32 = 4180u32;
pub const WIA_IPS_SCAN_AHEAD_CAPACITY: u32 = 4181u32;
pub const WIA_IPS_SCAN_AHEAD_CAPACITY_STR: &str = "Scan Ahead Capacity";
pub const WIA_IPS_SCAN_AHEAD_STR: &str = "Scan Ahead";
pub const WIA_IPS_SEGMENTATION: u32 = 6164u32;
pub const WIA_IPS_SEGMENTATION_STR: &str = "Segmentation";
pub const WIA_IPS_SHEET_FEEDER_REGISTRATION: u32 = 3078u32;
pub const WIA_IPS_SHEET_FEEDER_REGISTRATION_STR: &str = "Sheet Feeder Registration";
pub const WIA_IPS_SHOW_PREVIEW_CONTROL: u32 = 3103u32;
pub const WIA_IPS_SHOW_PREVIEW_CONTROL_STR: &str = "Show preview control";
pub const WIA_IPS_SUPPORTED_BARCODE_TYPES: u32 = 4155u32;
pub const WIA_IPS_SUPPORTED_BARCODE_TYPES_STR: &str = "Supported Barcode Types";
pub const WIA_IPS_SUPPORTED_PATCH_CODE_TYPES: u32 = 4162u32;
pub const WIA_IPS_SUPPORTED_PATCH_CODE_TYPES_STR: &str = "Supported Patch Code Types";
pub const WIA_IPS_SUPPORTS_CHILD_ITEM_CREATION: u32 = 3108u32;
pub const WIA_IPS_SUPPORTS_CHILD_ITEM_CREATION_STR: &str = "Supports Child Item Creation";
pub const WIA_IPS_THRESHOLD: u32 = 6159u32;
pub const WIA_IPS_THRESHOLD_STR: &str = "Threshold";
pub const WIA_IPS_TRANSFER_CAPABILITIES: u32 = 6169u32;
pub const WIA_IPS_TRANSFER_CAPABILITIES_STR: &str = "Transfer Capabilities";
pub const WIA_IPS_WARM_UP_TIME: u32 = 6161u32;
pub const WIA_IPS_WARM_UP_TIME_STR: &str = "Lamp Warm up Time";
pub const WIA_IPS_XEXTENT: u32 = 6151u32;
pub const WIA_IPS_XEXTENT_STR: &str = "Horizontal Extent";
pub const WIA_IPS_XPOS: u32 = 6149u32;
pub const WIA_IPS_XPOS_STR: &str = "Horizontal Start Position";
pub const WIA_IPS_XRES: u32 = 6147u32;
pub const WIA_IPS_XRES_STR: &str = "Horizontal Resolution";
pub const WIA_IPS_XSCALING: u32 = 3109u32;
pub const WIA_IPS_XSCALING_STR: &str = "Horizontal Scaling";
pub const WIA_IPS_YEXTENT: u32 = 6152u32;
pub const WIA_IPS_YEXTENT_STR: &str = "Vertical Extent";
pub const WIA_IPS_YPOS: u32 = 6150u32;
pub const WIA_IPS_YPOS_STR: &str = "Vertical Start Position";
pub const WIA_IPS_YRES: u32 = 6148u32;
pub const WIA_IPS_YRES_STR: &str = "Vertical Resolution";
pub const WIA_IPS_YSCALING: u32 = 3110u32;
pub const WIA_IPS_YSCALING_STR: &str = "Vertical Scaling";
pub const WIA_IS_DEFAULT_HANDLER: u32 = 1u32;
pub const WIA_ITEM_CAN_BE_DELETED: u32 = 128u32;
pub const WIA_ITEM_READ: u32 = 1u32;
pub const WIA_ITEM_WRITE: u32 = 2u32;
pub const WIA_LAMP_OFF: u32 = 1u32;
pub const WIA_LAMP_ON: u32 = 0u32;
pub const WIA_LINE_ORDER_BOTTOM_TO_TOP: u32 = 2u32;
pub const WIA_LINE_ORDER_TOP_TO_BOTTOM: u32 = 1u32;
pub const WIA_LIST_COUNT: u32 = 0u32;
pub const WIA_LIST_NOM: u32 = 1u32;
pub const WIA_LIST_NUM_ELEMS: u32 = 2u32;
pub const WIA_LIST_VALUES: u32 = 2u32;
pub const WIA_LONG_DOCUMENT_DISABLED: u32 = 0u32;
pub const WIA_LONG_DOCUMENT_ENABLED: u32 = 1u32;
pub const WIA_LONG_DOCUMENT_SPLIT: u32 = 2u32;
pub const WIA_MAJOR_EVENT_DEVICE_CONNECT: u32 = 1u32;
pub const WIA_MAJOR_EVENT_DEVICE_DISCONNECT: u32 = 2u32;
pub const WIA_MAJOR_EVENT_PICTURE_DELETED: u32 = 4u32;
pub const WIA_MAJOR_EVENT_PICTURE_TAKEN: u32 = 3u32;
pub const WIA_MAX_CTX_SIZE: u32 = 16777216u32;
#[repr(C)]
pub struct WIA_MICR {
    pub Tag: u32,
    pub Version: u32,
    pub Size: u32,
    pub Placeholder: u16,
    pub Reserved: u16,
    pub Count: u32,
    pub Micr: [WIA_MICR_INFO; 1],
}
impl ::core::marker::Copy for WIA_MICR {}
impl ::core::clone::Clone for WIA_MICR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_MICR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_MICR").field("Tag", &self.Tag).field("Version", &self.Version).field("Size", &self.Size).field("Placeholder", &self.Placeholder).field("Reserved", &self.Reserved).field("Count", &self.Count).field("Micr", &self.Micr).finish()
    }
}
unsafe impl ::windows_core::Abi for WIA_MICR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_MICR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_MICR>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_MICR {}
impl ::core::default::Default for WIA_MICR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WIA_MICR_INFO {
    pub Size: u32,
    pub Page: u32,
    pub Length: u32,
    pub Text: [u16; 1],
}
impl ::core::marker::Copy for WIA_MICR_INFO {}
impl ::core::clone::Clone for WIA_MICR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_MICR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_MICR_INFO").field("Size", &self.Size).field("Page", &self.Page).field("Length", &self.Length).field("Text", &self.Text).finish()
    }
}
unsafe impl ::windows_core::Abi for WIA_MICR_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_MICR_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_MICR_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_MICR_INFO {}
impl ::core::default::Default for WIA_MICR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WIA_MICR_READER_AUTO: u32 = 1u32;
pub const WIA_MICR_READER_DISABLED: u32 = 0u32;
pub const WIA_MICR_READER_FEEDER_BACK: u32 = 4u32;
pub const WIA_MICR_READER_FEEDER_DUPLEX: u32 = 5u32;
pub const WIA_MICR_READER_FEEDER_FRONT: u32 = 3u32;
pub const WIA_MICR_READER_FLATBED: u32 = 2u32;
pub const WIA_MULTI_FEED_DETECT_CONTINUE: u32 = 3u32;
pub const WIA_MULTI_FEED_DETECT_DISABLED: u32 = 0u32;
pub const WIA_MULTI_FEED_DETECT_METHOD_LENGTH: u32 = 0u32;
pub const WIA_MULTI_FEED_DETECT_METHOD_OVERLAP: u32 = 1u32;
pub const WIA_MULTI_FEED_DETECT_STOP_ERROR: u32 = 1u32;
pub const WIA_MULTI_FEED_DETECT_STOP_SUCCESS: u32 = 2u32;
pub const WIA_NOTIFICATION_EVENT: u32 = 1u32;
pub const WIA_NUM_DIP: u32 = 16u32;
pub const WIA_NUM_IPC: u32 = 9u32;
pub const WIA_ORDER_BGR: u32 = 1u32;
pub const WIA_ORDER_RGB: u32 = 0u32;
pub const WIA_OVER_SCAN_ALL: u32 = 3u32;
pub const WIA_OVER_SCAN_DISABLED: u32 = 0u32;
pub const WIA_OVER_SCAN_LEFT_RIGHT: u32 = 2u32;
pub const WIA_OVER_SCAN_TOP_BOTTOM: u32 = 1u32;
pub const WIA_PACKED_PIXEL: u32 = 0u32;
pub const WIA_PAGE_A4: u32 = 0u32;
pub const WIA_PAGE_AUTO: u32 = 100u32;
pub const WIA_PAGE_BUSINESSCARD: u32 = 6u32;
pub const WIA_PAGE_CUSTOM: u32 = 2u32;
pub const WIA_PAGE_CUSTOM_BASE: u32 = 32768u32;
pub const WIA_PAGE_DIN_2B: u32 = 52u32;
pub const WIA_PAGE_DIN_4B: u32 = 53u32;
pub const WIA_PAGE_ISO_A0: u32 = 7u32;
pub const WIA_PAGE_ISO_A1: u32 = 8u32;
pub const WIA_PAGE_ISO_A10: u32 = 16u32;
pub const WIA_PAGE_ISO_A2: u32 = 9u32;
pub const WIA_PAGE_ISO_A3: u32 = 10u32;
pub const WIA_PAGE_ISO_A4: u32 = 0u32;
pub const WIA_PAGE_ISO_A5: u32 = 11u32;
pub const WIA_PAGE_ISO_A6: u32 = 12u32;
pub const WIA_PAGE_ISO_A7: u32 = 13u32;
pub const WIA_PAGE_ISO_A8: u32 = 14u32;
pub const WIA_PAGE_ISO_A9: u32 = 15u32;
pub const WIA_PAGE_ISO_B0: u32 = 17u32;
pub const WIA_PAGE_ISO_B1: u32 = 18u32;
pub const WIA_PAGE_ISO_B10: u32 = 27u32;
pub const WIA_PAGE_ISO_B2: u32 = 19u32;
pub const WIA_PAGE_ISO_B3: u32 = 20u32;
pub const WIA_PAGE_ISO_B4: u32 = 21u32;
pub const WIA_PAGE_ISO_B5: u32 = 22u32;
pub const WIA_PAGE_ISO_B6: u32 = 23u32;
pub const WIA_PAGE_ISO_B7: u32 = 24u32;
pub const WIA_PAGE_ISO_B8: u32 = 25u32;
pub const WIA_PAGE_ISO_B9: u32 = 26u32;
pub const WIA_PAGE_ISO_C0: u32 = 28u32;
pub const WIA_PAGE_ISO_C1: u32 = 29u32;
pub const WIA_PAGE_ISO_C10: u32 = 38u32;
pub const WIA_PAGE_ISO_C2: u32 = 30u32;
pub const WIA_PAGE_ISO_C3: u32 = 31u32;
pub const WIA_PAGE_ISO_C4: u32 = 32u32;
pub const WIA_PAGE_ISO_C5: u32 = 33u32;
pub const WIA_PAGE_ISO_C6: u32 = 34u32;
pub const WIA_PAGE_ISO_C7: u32 = 35u32;
pub const WIA_PAGE_ISO_C8: u32 = 36u32;
pub const WIA_PAGE_ISO_C9: u32 = 37u32;
pub const WIA_PAGE_JIS_2A: u32 = 50u32;
pub const WIA_PAGE_JIS_4A: u32 = 51u32;
pub const WIA_PAGE_JIS_B0: u32 = 39u32;
pub const WIA_PAGE_JIS_B1: u32 = 40u32;
pub const WIA_PAGE_JIS_B10: u32 = 49u32;
pub const WIA_PAGE_JIS_B2: u32 = 41u32;
pub const WIA_PAGE_JIS_B3: u32 = 42u32;
pub const WIA_PAGE_JIS_B4: u32 = 43u32;
pub const WIA_PAGE_JIS_B5: u32 = 44u32;
pub const WIA_PAGE_JIS_B6: u32 = 45u32;
pub const WIA_PAGE_JIS_B7: u32 = 46u32;
pub const WIA_PAGE_JIS_B8: u32 = 47u32;
pub const WIA_PAGE_JIS_B9: u32 = 48u32;
pub const WIA_PAGE_LETTER: u32 = 1u32;
pub const WIA_PAGE_USLEDGER: u32 = 4u32;
pub const WIA_PAGE_USLEGAL: u32 = 3u32;
pub const WIA_PAGE_USLETTER: u32 = 1u32;
pub const WIA_PAGE_USSTATEMENT: u32 = 5u32;
#[repr(C)]
pub struct WIA_PATCH_CODES {
    pub Tag: u32,
    pub Version: u32,
    pub Size: u32,
    pub Count: u32,
    pub PatchCodes: [WIA_PATCH_CODE_INFO; 1],
}
impl ::core::marker::Copy for WIA_PATCH_CODES {}
impl ::core::clone::Clone for WIA_PATCH_CODES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_PATCH_CODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_PATCH_CODES").field("Tag", &self.Tag).field("Version", &self.Version).field("Size", &self.Size).field("Count", &self.Count).field("PatchCodes", &self.PatchCodes).finish()
    }
}
unsafe impl ::windows_core::Abi for WIA_PATCH_CODES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_PATCH_CODES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_PATCH_CODES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_PATCH_CODES {}
impl ::core::default::Default for WIA_PATCH_CODES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WIA_PATCH_CODE_1: u32 = 1u32;
pub const WIA_PATCH_CODE_10: u32 = 10u32;
pub const WIA_PATCH_CODE_11: u32 = 11u32;
pub const WIA_PATCH_CODE_12: u32 = 12u32;
pub const WIA_PATCH_CODE_13: u32 = 13u32;
pub const WIA_PATCH_CODE_14: u32 = 14u32;
pub const WIA_PATCH_CODE_2: u32 = 2u32;
pub const WIA_PATCH_CODE_3: u32 = 3u32;
pub const WIA_PATCH_CODE_4: u32 = 4u32;
pub const WIA_PATCH_CODE_6: u32 = 6u32;
pub const WIA_PATCH_CODE_7: u32 = 7u32;
pub const WIA_PATCH_CODE_8: u32 = 8u32;
pub const WIA_PATCH_CODE_9: u32 = 9u32;
pub const WIA_PATCH_CODE_CUSTOM_BASE: u32 = 32768u32;
#[repr(C)]
pub struct WIA_PATCH_CODE_INFO {
    pub Type: u32,
}
impl ::core::marker::Copy for WIA_PATCH_CODE_INFO {}
impl ::core::clone::Clone for WIA_PATCH_CODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_PATCH_CODE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_PATCH_CODE_INFO").field("Type", &self.Type).finish()
    }
}
unsafe impl ::windows_core::Abi for WIA_PATCH_CODE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_PATCH_CODE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_PATCH_CODE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_PATCH_CODE_INFO {}
impl ::core::default::Default for WIA_PATCH_CODE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WIA_PATCH_CODE_READER_AUTO: u32 = 1u32;
pub const WIA_PATCH_CODE_READER_DISABLED: u32 = 0u32;
pub const WIA_PATCH_CODE_READER_FEEDER_BACK: u32 = 4u32;
pub const WIA_PATCH_CODE_READER_FEEDER_DUPLEX: u32 = 5u32;
pub const WIA_PATCH_CODE_READER_FEEDER_FRONT: u32 = 3u32;
pub const WIA_PATCH_CODE_READER_FLATBED: u32 = 2u32;
pub const WIA_PATCH_CODE_T: u32 = 5u32;
pub const WIA_PATCH_CODE_UNKNOWN: u32 = 0u32;
pub const WIA_PHOTO_WHITE_0: u32 = 1u32;
pub const WIA_PHOTO_WHITE_1: u32 = 0u32;
pub const WIA_PLANAR: u32 = 1u32;
pub const WIA_PREVIEW_SCAN: u32 = 1u32;
pub const WIA_PRINTER_ENDORSER_AFTER_SCAN: u32 = 1u32;
pub const WIA_PRINTER_ENDORSER_AUTO: u32 = 1u32;
pub const WIA_PRINTER_ENDORSER_BEFORE_SCAN: u32 = 0u32;
pub const WIA_PRINTER_ENDORSER_DIGITAL: u32 = 6u32;
pub const WIA_PRINTER_ENDORSER_DISABLED: u32 = 0u32;
pub const WIA_PRINTER_ENDORSER_FEEDER_BACK: u32 = 4u32;
pub const WIA_PRINTER_ENDORSER_FEEDER_DUPLEX: u32 = 5u32;
pub const WIA_PRINTER_ENDORSER_FEEDER_FRONT: u32 = 3u32;
pub const WIA_PRINTER_ENDORSER_FLATBED: u32 = 2u32;
pub const WIA_PRINTER_ENDORSER_GRAPHICS_BACKGROUND: u32 = 8u32;
pub const WIA_PRINTER_ENDORSER_GRAPHICS_BOTTOM: u32 = 3u32;
pub const WIA_PRINTER_ENDORSER_GRAPHICS_BOTTOM_LEFT: u32 = 6u32;
pub const WIA_PRINTER_ENDORSER_GRAPHICS_BOTTOM_RIGHT: u32 = 7u32;
pub const WIA_PRINTER_ENDORSER_GRAPHICS_DEVICE_DEFAULT: u32 = 9u32;
pub const WIA_PRINTER_ENDORSER_GRAPHICS_LEFT: u32 = 0u32;
pub const WIA_PRINTER_ENDORSER_GRAPHICS_RIGHT: u32 = 1u32;
pub const WIA_PRINTER_ENDORSER_GRAPHICS_TOP: u32 = 2u32;
pub const WIA_PRINTER_ENDORSER_GRAPHICS_TOP_LEFT: u32 = 4u32;
pub const WIA_PRINTER_ENDORSER_GRAPHICS_TOP_RIGHT: u32 = 5u32;
pub const WIA_PRINT_AM_PM: u32 = 9u32;
pub const WIA_PRINT_DATE: u32 = 0u32;
pub const WIA_PRINT_DAY: u32 = 3u32;
pub const WIA_PRINT_FONT_BOLD: u32 = 1u32;
pub const WIA_PRINT_FONT_EXTRA_BOLD: u32 = 2u32;
pub const WIA_PRINT_FONT_ITALIC: u32 = 5u32;
pub const WIA_PRINT_FONT_ITALIC_BOLD: u32 = 3u32;
pub const WIA_PRINT_FONT_ITALIC_EXTRA_BOLD: u32 = 4u32;
pub const WIA_PRINT_FONT_LARGE: u32 = 12u32;
pub const WIA_PRINT_FONT_LARGE_BOLD: u32 = 13u32;
pub const WIA_PRINT_FONT_LARGE_EXTRA_BOLD: u32 = 14u32;
pub const WIA_PRINT_FONT_LARGE_ITALIC: u32 = 17u32;
pub const WIA_PRINT_FONT_LARGE_ITALIC_BOLD: u32 = 15u32;
pub const WIA_PRINT_FONT_LARGE_ITALIC_EXTRA_BOLD: u32 = 16u32;
pub const WIA_PRINT_FONT_NORMAL: u32 = 0u32;
pub const WIA_PRINT_FONT_SMALL: u32 = 6u32;
pub const WIA_PRINT_FONT_SMALL_BOLD: u32 = 7u32;
pub const WIA_PRINT_FONT_SMALL_EXTRA_BOLD: u32 = 8u32;
pub const WIA_PRINT_FONT_SMALL_ITALIC: u32 = 11u32;
pub const WIA_PRINT_FONT_SMALL_ITALIC_BOLD: u32 = 9u32;
pub const WIA_PRINT_FONT_SMALL_ITALIC_EXTRA_BOLD: u32 = 10u32;
pub const WIA_PRINT_HOUR_12H: u32 = 8u32;
pub const WIA_PRINT_HOUR_24H: u32 = 7u32;
pub const WIA_PRINT_IMAGE: u32 = 13u32;
pub const WIA_PRINT_MILLISECOND: u32 = 14u32;
pub const WIA_PRINT_MINUTE: u32 = 10u32;
pub const WIA_PRINT_MONTH: u32 = 2u32;
pub const WIA_PRINT_MONTH_NAME: u32 = 15u32;
pub const WIA_PRINT_MONTH_SHORT: u32 = 16u32;
pub const WIA_PRINT_PADDING_BLANK: u32 = 2u32;
pub const WIA_PRINT_PADDING_NONE: u32 = 0u32;
pub const WIA_PRINT_PADDING_ZERO: u32 = 1u32;
pub const WIA_PRINT_PAGE_COUNT: u32 = 12u32;
pub const WIA_PRINT_SECOND: u32 = 11u32;
pub const WIA_PRINT_TIME_12H: u32 = 6u32;
pub const WIA_PRINT_TIME_24H: u32 = 5u32;
pub const WIA_PRINT_WEEK_DAY: u32 = 4u32;
pub const WIA_PRINT_WEEK_DAY_SHORT: u32 = 17u32;
pub const WIA_PRINT_YEAR: u32 = 1u32;
pub const WIA_PRIVATE_DEVPROP: u32 = 38914u32;
pub const WIA_PRIVATE_ITEMPROP: u32 = 71682u32;
#[repr(C)]
pub struct WIA_PROPERTY_CONTEXT {
    pub cProps: u32,
    pub pProps: *mut u32,
    pub pChanged: *mut ::win32_foundation::BOOL,
}
impl ::core::marker::Copy for WIA_PROPERTY_CONTEXT {}
impl ::core::clone::Clone for WIA_PROPERTY_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_PROPERTY_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_PROPERTY_CONTEXT").field("cProps", &self.cProps).field("pProps", &self.pProps).field("pChanged", &self.pChanged).finish()
    }
}
unsafe impl ::windows_core::Abi for WIA_PROPERTY_CONTEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_PROPERTY_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_PROPERTY_CONTEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_PROPERTY_CONTEXT {}
impl ::core::default::Default for WIA_PROPERTY_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WIA_PROPERTY_INFO {
    pub lAccessFlags: u32,
    pub vt: u16,
    pub ValidVal: WIA_PROPERTY_INFO_0,
}
impl ::core::clone::Clone for WIA_PROPERTY_INFO {
    fn clone(&self) -> Self {
        Self { lAccessFlags: self.lAccessFlags, vt: self.vt, ValidVal: self.ValidVal.clone() }
    }
}
unsafe impl ::windows_core::Abi for WIA_PROPERTY_INFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for WIA_PROPERTY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.lAccessFlags == other.lAccessFlags && self.vt == other.vt && self.ValidVal == other.ValidVal
    }
}
impl ::core::cmp::Eq for WIA_PROPERTY_INFO {}
impl ::core::default::Default for WIA_PROPERTY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union WIA_PROPERTY_INFO_0 {
    pub Range: WIA_PROPERTY_INFO_0_7,
    pub RangeFloat: WIA_PROPERTY_INFO_0_6,
    pub List: WIA_PROPERTY_INFO_0_4,
    pub ListFloat: WIA_PROPERTY_INFO_0_2,
    pub ListGuid: WIA_PROPERTY_INFO_0_3,
    pub ListBStr: ::core::mem::ManuallyDrop<WIA_PROPERTY_INFO_0_1>,
    pub Flag: WIA_PROPERTY_INFO_0_0,
    pub None: WIA_PROPERTY_INFO_0_5,
}
impl ::core::clone::Clone for WIA_PROPERTY_INFO_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
unsafe impl ::windows_core::Abi for WIA_PROPERTY_INFO_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for WIA_PROPERTY_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_PROPERTY_INFO_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_PROPERTY_INFO_0 {}
impl ::core::default::Default for WIA_PROPERTY_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WIA_PROPERTY_INFO_0_0 {
    pub Nom: i32,
    pub ValidBits: i32,
}
impl ::core::marker::Copy for WIA_PROPERTY_INFO_0_0 {}
impl ::core::clone::Clone for WIA_PROPERTY_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_PROPERTY_INFO_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_PROPERTY_INFO_0_0").field("Nom", &self.Nom).field("ValidBits", &self.ValidBits).finish()
    }
}
unsafe impl ::windows_core::Abi for WIA_PROPERTY_INFO_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_PROPERTY_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_PROPERTY_INFO_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_PROPERTY_INFO_0_0 {}
impl ::core::default::Default for WIA_PROPERTY_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WIA_PROPERTY_INFO_0_1 {
    pub cNumList: i32,
    pub Nom: ::win32_foundation::BSTR,
    pub pList: *mut ::win32_foundation::BSTR,
}
impl ::core::clone::Clone for WIA_PROPERTY_INFO_0_1 {
    fn clone(&self) -> Self {
        Self { cNumList: self.cNumList, Nom: self.Nom.clone(), pList: self.pList }
    }
}
impl ::core::fmt::Debug for WIA_PROPERTY_INFO_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_PROPERTY_INFO_0_1").field("cNumList", &self.cNumList).field("Nom", &self.Nom).field("pList", &self.pList).finish()
    }
}
unsafe impl ::windows_core::Abi for WIA_PROPERTY_INFO_0_1 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for WIA_PROPERTY_INFO_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.cNumList == other.cNumList && self.Nom == other.Nom && self.pList == other.pList
    }
}
impl ::core::cmp::Eq for WIA_PROPERTY_INFO_0_1 {}
impl ::core::default::Default for WIA_PROPERTY_INFO_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WIA_PROPERTY_INFO_0_2 {
    pub cNumList: i32,
    pub Nom: f64,
    pub pList: *mut u8,
}
impl ::core::marker::Copy for WIA_PROPERTY_INFO_0_2 {}
impl ::core::clone::Clone for WIA_PROPERTY_INFO_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_PROPERTY_INFO_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_PROPERTY_INFO_0_2").field("cNumList", &self.cNumList).field("Nom", &self.Nom).field("pList", &self.pList).finish()
    }
}
unsafe impl ::windows_core::Abi for WIA_PROPERTY_INFO_0_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_PROPERTY_INFO_0_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_PROPERTY_INFO_0_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_PROPERTY_INFO_0_2 {}
impl ::core::default::Default for WIA_PROPERTY_INFO_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WIA_PROPERTY_INFO_0_3 {
    pub cNumList: i32,
    pub Nom: ::windows_core::GUID,
    pub pList: *mut ::windows_core::GUID,
}
impl ::core::marker::Copy for WIA_PROPERTY_INFO_0_3 {}
impl ::core::clone::Clone for WIA_PROPERTY_INFO_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_PROPERTY_INFO_0_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_PROPERTY_INFO_0_3").field("cNumList", &self.cNumList).field("Nom", &self.Nom).field("pList", &self.pList).finish()
    }
}
unsafe impl ::windows_core::Abi for WIA_PROPERTY_INFO_0_3 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_PROPERTY_INFO_0_3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_PROPERTY_INFO_0_3>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_PROPERTY_INFO_0_3 {}
impl ::core::default::Default for WIA_PROPERTY_INFO_0_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WIA_PROPERTY_INFO_0_4 {
    pub cNumList: i32,
    pub Nom: i32,
    pub pList: *mut u8,
}
impl ::core::marker::Copy for WIA_PROPERTY_INFO_0_4 {}
impl ::core::clone::Clone for WIA_PROPERTY_INFO_0_4 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_PROPERTY_INFO_0_4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_PROPERTY_INFO_0_4").field("cNumList", &self.cNumList).field("Nom", &self.Nom).field("pList", &self.pList).finish()
    }
}
unsafe impl ::windows_core::Abi for WIA_PROPERTY_INFO_0_4 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_PROPERTY_INFO_0_4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_PROPERTY_INFO_0_4>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_PROPERTY_INFO_0_4 {}
impl ::core::default::Default for WIA_PROPERTY_INFO_0_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WIA_PROPERTY_INFO_0_5 {
    pub Dummy: i32,
}
impl ::core::marker::Copy for WIA_PROPERTY_INFO_0_5 {}
impl ::core::clone::Clone for WIA_PROPERTY_INFO_0_5 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_PROPERTY_INFO_0_5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_PROPERTY_INFO_0_5").field("Dummy", &self.Dummy).finish()
    }
}
unsafe impl ::windows_core::Abi for WIA_PROPERTY_INFO_0_5 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_PROPERTY_INFO_0_5 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_PROPERTY_INFO_0_5>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_PROPERTY_INFO_0_5 {}
impl ::core::default::Default for WIA_PROPERTY_INFO_0_5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WIA_PROPERTY_INFO_0_6 {
    pub Min: f64,
    pub Nom: f64,
    pub Max: f64,
    pub Inc: f64,
}
impl ::core::marker::Copy for WIA_PROPERTY_INFO_0_6 {}
impl ::core::clone::Clone for WIA_PROPERTY_INFO_0_6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_PROPERTY_INFO_0_6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_PROPERTY_INFO_0_6").field("Min", &self.Min).field("Nom", &self.Nom).field("Max", &self.Max).field("Inc", &self.Inc).finish()
    }
}
unsafe impl ::windows_core::Abi for WIA_PROPERTY_INFO_0_6 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_PROPERTY_INFO_0_6 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_PROPERTY_INFO_0_6>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_PROPERTY_INFO_0_6 {}
impl ::core::default::Default for WIA_PROPERTY_INFO_0_6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WIA_PROPERTY_INFO_0_7 {
    pub Min: i32,
    pub Nom: i32,
    pub Max: i32,
    pub Inc: i32,
}
impl ::core::marker::Copy for WIA_PROPERTY_INFO_0_7 {}
impl ::core::clone::Clone for WIA_PROPERTY_INFO_0_7 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_PROPERTY_INFO_0_7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_PROPERTY_INFO_0_7").field("Min", &self.Min).field("Nom", &self.Nom).field("Max", &self.Max).field("Inc", &self.Inc).finish()
    }
}
unsafe impl ::windows_core::Abi for WIA_PROPERTY_INFO_0_7 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_PROPERTY_INFO_0_7 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_PROPERTY_INFO_0_7>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_PROPERTY_INFO_0_7 {}
impl ::core::default::Default for WIA_PROPERTY_INFO_0_7 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WIA_PROPID_TO_NAME {
    pub propid: u32,
    pub pszName: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for WIA_PROPID_TO_NAME {}
impl ::core::clone::Clone for WIA_PROPID_TO_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_PROPID_TO_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_PROPID_TO_NAME").field("propid", &self.propid).field("pszName", &self.pszName).finish()
    }
}
unsafe impl ::windows_core::Abi for WIA_PROPID_TO_NAME {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_PROPID_TO_NAME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_PROPID_TO_NAME>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_PROPID_TO_NAME {}
impl ::core::default::Default for WIA_PROPID_TO_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WIA_PROPPAGE_CAMERA_ITEM_GENERAL: u32 = 2u32;
pub const WIA_PROPPAGE_DEVICE_GENERAL: u32 = 4u32;
pub const WIA_PROPPAGE_SCANNER_ITEM_GENERAL: u32 = 1u32;
pub const WIA_PROP_CACHEABLE: u32 = 65536u32;
pub const WIA_PROP_FLAG: u32 = 64u32;
pub const WIA_PROP_LIST: u32 = 32u32;
pub const WIA_PROP_NONE: u32 = 8u32;
pub const WIA_PROP_RANGE: u32 = 16u32;
pub const WIA_PROP_READ: u32 = 1u32;
pub const WIA_PROP_SYNC_REQUIRED: u32 = 4u32;
pub const WIA_PROP_WRITE: u32 = 2u32;
pub const WIA_RANGE_MAX: u32 = 2u32;
pub const WIA_RANGE_MIN: u32 = 0u32;
pub const WIA_RANGE_NOM: u32 = 1u32;
pub const WIA_RANGE_NUM_ELEMS: u32 = 4u32;
pub const WIA_RANGE_STEP: u32 = 3u32;
#[repr(C)]
pub struct WIA_RAW_HEADER {
    pub Tag: u32,
    pub Version: u32,
    pub HeaderSize: u32,
    pub XRes: u32,
    pub YRes: u32,
    pub XExtent: u32,
    pub YExtent: u32,
    pub BytesPerLine: u32,
    pub BitsPerPixel: u32,
    pub ChannelsPerPixel: u32,
    pub DataType: u32,
    pub BitsPerChannel: [u8; 8],
    pub Compression: u32,
    pub PhotometricInterp: u32,
    pub LineOrder: u32,
    pub RawDataOffset: u32,
    pub RawDataSize: u32,
    pub PaletteOffset: u32,
    pub PaletteSize: u32,
}
impl ::core::marker::Copy for WIA_RAW_HEADER {}
impl ::core::clone::Clone for WIA_RAW_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_RAW_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_RAW_HEADER")
            .field("Tag", &self.Tag)
            .field("Version", &self.Version)
            .field("HeaderSize", &self.HeaderSize)
            .field("XRes", &self.XRes)
            .field("YRes", &self.YRes)
            .field("XExtent", &self.XExtent)
            .field("YExtent", &self.YExtent)
            .field("BytesPerLine", &self.BytesPerLine)
            .field("BitsPerPixel", &self.BitsPerPixel)
            .field("ChannelsPerPixel", &self.ChannelsPerPixel)
            .field("DataType", &self.DataType)
            .field("BitsPerChannel", &self.BitsPerChannel)
            .field("Compression", &self.Compression)
            .field("PhotometricInterp", &self.PhotometricInterp)
            .field("LineOrder", &self.LineOrder)
            .field("RawDataOffset", &self.RawDataOffset)
            .field("RawDataSize", &self.RawDataSize)
            .field("PaletteOffset", &self.PaletteOffset)
            .field("PaletteSize", &self.PaletteSize)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for WIA_RAW_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_RAW_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_RAW_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_RAW_HEADER {}
impl ::core::default::Default for WIA_RAW_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WIA_REGISTER_EVENT_CALLBACK: u32 = 1u32;
pub const WIA_RESERVED_FOR_NEW_PROPS: u32 = 1024u32;
pub const WIA_SCAN_AHEAD_ALL: u32 = 0u32;
pub const WIA_SCAN_AHEAD_DISABLED: u32 = 0u32;
pub const WIA_SCAN_AHEAD_ENABLED: u32 = 1u32;
pub const WIA_SEGMENTATION_FILTER_STR: &str = "SegmentationFilter";
pub const WIA_SELECT_DEVICE_NODEFAULT: u32 = 1u32;
pub const WIA_SEPARATOR_DETECT_NOSCAN_CONTINUE: u32 = 3u32;
pub const WIA_SEPARATOR_DETECT_NOSCAN_STOP: u32 = 4u32;
pub const WIA_SEPARATOR_DETECT_SCAN_CONTINUE: u32 = 1u32;
pub const WIA_SEPARATOR_DETECT_SCAN_STOP: u32 = 2u32;
pub const WIA_SEPARATOR_DISABLED: u32 = 0u32;
pub const WIA_SET_DEFAULT_HANDLER: u32 = 4u32;
pub const WIA_SHOW_PREVIEW_CONTROL: u32 = 0u32;
pub const WIA_STATUS_CALIBRATING: ::windows_core::HRESULT = ::windows_core::HRESULT(2162691i32);
pub const WIA_STATUS_CLEAR: ::windows_core::HRESULT = ::windows_core::HRESULT(2162696i32);
pub const WIA_STATUS_END_OF_MEDIA: ::windows_core::HRESULT = ::windows_core::HRESULT(2162689i32);
pub const WIA_STATUS_NETWORK_DEVICE_RESERVED: ::windows_core::HRESULT = ::windows_core::HRESULT(2162695i32);
pub const WIA_STATUS_NOT_HANDLED: ::windows_core::HRESULT = ::windows_core::HRESULT(2162698i32);
pub const WIA_STATUS_RESERVING_NETWORK_DEVICE: ::windows_core::HRESULT = ::windows_core::HRESULT(2162694i32);
pub const WIA_STATUS_SKIP_ITEM: ::windows_core::HRESULT = ::windows_core::HRESULT(2162697i32);
pub const WIA_STATUS_WARMING_UP: ::windows_core::HRESULT = ::windows_core::HRESULT(2162690i32);
pub const WIA_S_CHANGE_DEVICE: ::windows_core::HRESULT = ::windows_core::HRESULT(2162699i32);
pub const WIA_S_NO_DEVICE_AVAILABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145320939i32);
pub const WIA_TRANSFER_ACQUIRE_CHILDREN: u32 = 1u32;
pub const WIA_TRANSFER_CHILDREN_SINGLE_SCAN: u32 = 1u32;
pub const WIA_TRANSFER_MSG_DEVICE_STATUS: u32 = 5u32;
pub const WIA_TRANSFER_MSG_END_OF_STREAM: u32 = 2u32;
pub const WIA_TRANSFER_MSG_END_OF_TRANSFER: u32 = 3u32;
pub const WIA_TRANSFER_MSG_NEW_PAGE: u32 = 6u32;
pub const WIA_TRANSFER_MSG_STATUS: u32 = 1u32;
pub const WIA_UNREGISTER_EVENT_CALLBACK: u32 = 2u32;
pub const WIA_USE_SEGMENTATION_FILTER: u32 = 0u32;
pub const WIA_WSD_FRIENDLY_NAME: u32 = 38920u32;
pub const WIA_WSD_FRIENDLY_NAME_STR: &str = "Friendly name";
pub const WIA_WSD_MANUFACTURER: u32 = 38914u32;
pub const WIA_WSD_MANUFACTURER_STR: &str = "Device manufacturer";
pub const WIA_WSD_MANUFACTURER_URL: u32 = 38915u32;
pub const WIA_WSD_MANUFACTURER_URL_STR: &str = "Manufacurer URL";
pub const WIA_WSD_MODEL_NAME: u32 = 38916u32;
pub const WIA_WSD_MODEL_NAME_STR: &str = "Model name";
pub const WIA_WSD_MODEL_NUMBER: u32 = 38917u32;
pub const WIA_WSD_MODEL_NUMBER_STR: &str = "Model number";
pub const WIA_WSD_MODEL_URL: u32 = 38918u32;
pub const WIA_WSD_MODEL_URL_STR: &str = "Model URL";
pub const WIA_WSD_PRESENTATION_URL: u32 = 38919u32;
pub const WIA_WSD_PRESENTATION_URL_STR: &str = "Presentation URL";
pub const WIA_WSD_SCAN_AVAILABLE_ITEM: u32 = 38922u32;
pub const WIA_WSD_SCAN_AVAILABLE_ITEM_STR: &str = "Scan Available Item";
pub const WIA_WSD_SERIAL_NUMBER: u32 = 38921u32;
pub const WIA_WSD_SERIAL_NUMBER_STR: &str = "Serial number";
pub const WiaAudFmt_AIFF: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x66e2bf4f_b6fc_443f_94c8_2f33c8a65aaf);
pub const WiaAudFmt_MP3: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0fbc71fb_43bf_49f2_9190_e6fecff37e54);
pub const WiaAudFmt_WAV: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf818e146_07af_40ff_ae55_be8f2c065dbe);
pub const WiaAudFmt_WMA: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd61d6413_8bc2_438f_93ad_21bd484db6a1);
pub const WiaDevMgr: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa1f4e726_8cf1_11d1_bf92_0060081ed811);
pub const WiaDevMgr2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb6c292bc_7c88_41ee_8b54_8ec92617e599);
pub const WiaImgFmt_ASF: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8d948ee9_d0aa_4a12_9d9a_9cc5de36199b);
pub const WiaImgFmt_AVI: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x32f8ca14_087c_4908_b7c4_6757fe7e90ab);
pub const WiaImgFmt_BMP: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb96b3cab_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_CIFF: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9821a8ab_3a7e_4215_94e0_d27a460c03b2);
pub const WiaImgFmt_CSV: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x355bda24_5a9f_4494_80dc_be752cecbc8c);
pub const WiaImgFmt_DPOF: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x369eeeab_a0e8_45ca_86a6_a83ce5697e28);
pub const WiaImgFmt_EMF: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb96b3cac_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_EXEC: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x485da097_141e_4aa5_bb3b_a5618d95d02b);
pub const WiaImgFmt_EXIF: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb96b3cb2_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_FLASHPIX: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb96b3cb4_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_GIF: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb96b3cb0_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_HTML: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc99a4e62_99de_4a94_acca_71956ac2977d);
pub const WiaImgFmt_ICO: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb96b3cb5_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_JBIG: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x41e8dd92_2f0a_43d4_8636_f1614ba11e46);
pub const WiaImgFmt_JBIG2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbb8e7e67_283c_4235_9e59_0b9bf94ca687);
pub const WiaImgFmt_JPEG: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb96b3cae_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_JPEG2K: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x344ee2b2_39db_4dde_8173_c4b75f8f1e49);
pub const WiaImgFmt_JPEG2KX: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43e14614_c80a_4850_baf3_4b152dc8da27);
pub const WiaImgFmt_MEMORYBMP: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb96b3caa_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_MPG: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecd757e4_d2ec_4f57_955d_bcf8a97c4e52);
pub const WiaImgFmt_OXPS: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c7b1240_c14d_4109_9755_04b89025153a);
pub const WiaImgFmt_PDFA: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9980bd5b_3463_43c7_bdca_3caa146f229f);
pub const WiaImgFmt_PHOTOCD: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb96b3cb3_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_PICT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa6bc85d8_6b3e_40ee_a95c_25d482e41adc);
pub const WiaImgFmt_PNG: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb96b3caf_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_RAW: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f120719_f1a8_4e07_9ade_9b64c63a3dcc);
pub const WiaImgFmt_RAWBAR: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xda63f833_d26e_451e_90d2_ea55a1365d62);
pub const WiaImgFmt_RAWMIC: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x22c4f058_0d88_409c_ac1c_eec12b0ea680);
pub const WiaImgFmt_RAWPAT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7760507c_5064_400c_9a17_575624d8824b);
pub const WiaImgFmt_RAWRGB: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbca48b55_f272_4371_b0f1_4a150d057bb4);
pub const WiaImgFmt_RTF: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x573dd6a3_4834_432d_a9b5_e198dd9e890d);
pub const WiaImgFmt_SCRIPT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfe7d6c53_2dac_446a_b0bd_d73e21e924c9);
pub const WiaImgFmt_TIFF: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb96b3cb1_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_TXT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfafd4d82_723f_421f_9318_30501ac44b59);
pub const WiaImgFmt_UNDEFINED: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb96b3ca9_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_UNICODE16: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1b7639b6_6357_47d1_9a07_12452dc073e9);
pub const WiaImgFmt_WMF: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb96b3cad_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_XML: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9171457_dac8_4884_b393_15b471d5f07e);
pub const WiaImgFmt_XMLBAR: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6235701c_3a98_484c_b2a8_fdffd87e6b16);
pub const WiaImgFmt_XMLMIC: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2d164c61_b9ae_4b23_8973_c7067e1fbd31);
pub const WiaImgFmt_XMLPAT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf8986f55_f052_460d_9523_3a7dfedbb33c);
pub const WiaImgFmt_XPS: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x700b4a0f_2011_411c_b430_d1e0b2e10b28);
pub const WiaItemTypeAnalyze: u32 = 16u32;
pub const WiaItemTypeAudio: u32 = 32u32;
pub const WiaItemTypeBurst: u32 = 2048u32;
pub const WiaItemTypeDeleted: u32 = 128u32;
pub const WiaItemTypeDevice: u32 = 64u32;
pub const WiaItemTypeDisconnected: u32 = 256u32;
pub const WiaItemTypeDocument: u32 = 262144u32;
pub const WiaItemTypeFile: u32 = 2u32;
pub const WiaItemTypeFolder: u32 = 4u32;
pub const WiaItemTypeFree: u32 = 0u32;
pub const WiaItemTypeGenerated: u32 = 16384u32;
pub const WiaItemTypeHPanorama: u32 = 512u32;
pub const WiaItemTypeHasAttachments: u32 = 32768u32;
pub const WiaItemTypeImage: u32 = 1u32;
pub const WiaItemTypeMask: u32 = 2148532223u32;
pub const WiaItemTypeProgrammableDataSource: u32 = 524288u32;
pub const WiaItemTypeRemoved: u32 = 2147483648u32;
pub const WiaItemTypeRoot: u32 = 8u32;
pub const WiaItemTypeStorage: u32 = 4096u32;
pub const WiaItemTypeTransfer: u32 = 8192u32;
pub const WiaItemTypeTwainCapabilityPassThrough: u32 = 131072u32;
pub const WiaItemTypeVPanorama: u32 = 1024u32;
pub const WiaItemTypeVideo: u32 = 65536u32;
pub const WiaLog: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa1e75357_881a_419e_83e2_bb16db197c68);
#[repr(C)]
pub struct WiaTransferParams {
    pub lMessage: i32,
    pub lPercentComplete: i32,
    pub ulTransferredBytes: u64,
    pub hrErrorStatus: ::windows_core::HRESULT,
}
impl ::core::marker::Copy for WiaTransferParams {}
impl ::core::clone::Clone for WiaTransferParams {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WiaTransferParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WiaTransferParams").field("lMessage", &self.lMessage).field("lPercentComplete", &self.lPercentComplete).field("ulTransferredBytes", &self.ulTransferredBytes).field("hrErrorStatus", &self.hrErrorStatus).finish()
    }
}
unsafe impl ::windows_core::Abi for WiaTransferParams {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WiaTransferParams {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WiaTransferParams>()) == 0 }
    }
}
impl ::core::cmp::Eq for WiaTransferParams {}
impl ::core::default::Default for WiaTransferParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WiaVideo: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3908c3cd_4478_4536_af2f_10c25d4ef89a);
pub const g_dwDebugFlags: u32 = 0u32;
