#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DEVICE_SELECTION_DEVICE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DST_UNKNOWN_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DST_WPD_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DST_WIA_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DST_STI_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DSF_TWAIN_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DST_FS_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DST_DV_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(6i32);
impl ::core::marker::Copy for DEVICE_SELECTION_DEVICE_TYPE {}
impl ::core::clone::Clone for DEVICE_SELECTION_DEVICE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEVICE_SELECTION_DEVICE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DEVICE_SELECTION_DEVICE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DEVICE_SELECTION_DEVICE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVICE_SELECTION_DEVICE_TYPE").field(&self.0).finish()
    }
}
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
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ERROR_ADVISE_MESSAGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_ERROR_SKIPRETRYCANCEL: ERROR_ADVISE_MESSAGE_TYPE = ERROR_ADVISE_MESSAGE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_ERROR_RETRYCANCEL: ERROR_ADVISE_MESSAGE_TYPE = ERROR_ADVISE_MESSAGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_ERROR_YESNO: ERROR_ADVISE_MESSAGE_TYPE = ERROR_ADVISE_MESSAGE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_ERROR_OK: ERROR_ADVISE_MESSAGE_TYPE = ERROR_ADVISE_MESSAGE_TYPE(3i32);
impl ::core::marker::Copy for ERROR_ADVISE_MESSAGE_TYPE {}
impl ::core::clone::Clone for ERROR_ADVISE_MESSAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ERROR_ADVISE_MESSAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ERROR_ADVISE_MESSAGE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ERROR_ADVISE_MESSAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ERROR_ADVISE_MESSAGE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ERROR_ADVISE_RESULT(pub i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_RESULT_YES: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(0i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_RESULT_NO: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(1i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_RESULT_OK: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(2i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_RESULT_SKIP: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(3i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_RESULT_SKIP_ALL: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(4i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_RESULT_RETRY: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(5i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_RESULT_ABORT: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(6i32);
impl ::core::marker::Copy for ERROR_ADVISE_RESULT {}
impl ::core::clone::Clone for ERROR_ADVISE_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ERROR_ADVISE_RESULT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ERROR_ADVISE_RESULT {
    type Abi = Self;
}
impl ::core::fmt::Debug for ERROR_ADVISE_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ERROR_ADVISE_RESULT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
pub struct IPhotoAcquire(::windows_core::IUnknown);
impl IPhotoAcquire {
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn CreatePhotoSource<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszdevice: Param0) -> ::windows_core::Result<IPhotoAcquireSource> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreatePhotoSource)(::windows_core::Interface::as_raw(self), pszdevice.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPhotoAcquireSource>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Acquire<'a, Param0: ::windows_core::IntoParam<'a, IPhotoAcquireSource>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows_core::IntoParam<'a, super::super::Foundation::HWND>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param4: ::windows_core::IntoParam<'a, IPhotoAcquireProgressCB>>(&self, pphotoacquiresource: Param0, fshowprogress: Param1, hwndparent: Param2, pszapplicationname: Param3, pphotoacquireprogresscb: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Acquire)(::windows_core::Interface::as_raw(self), pphotoacquiresource.into_param().abi(), fshowprogress.into_param().abi(), hwndparent.into_param().abi(), pszapplicationname.into_param().abi(), pphotoacquireprogresscb.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumResults(&self) -> ::windows_core::Result<super::super::System::Com::IEnumString> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumResults)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IEnumString>(result__)
    }
}
impl ::core::convert::From<IPhotoAcquire> for ::windows_core::IUnknown {
    fn from(value: IPhotoAcquire) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPhotoAcquire> for ::windows_core::IUnknown {
    fn from(value: &IPhotoAcquire) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPhotoAcquire {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPhotoAcquire {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPhotoAcquire {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPhotoAcquire {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPhotoAcquire {}
impl ::core::fmt::Debug for IPhotoAcquire {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPhotoAcquire").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPhotoAcquire {
    type Vtable = IPhotoAcquire_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00f23353_e31b_4955_a8ad_ca5ebf31e2ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquire_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CreatePhotoSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdevice: ::windows_core::PCWSTR, ppphotoacquiresource: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Acquire: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pphotoacquiresource: ::windows_core::RawPtr, fshowprogress: super::super::Foundation::BOOL, hwndparent: super::super::Foundation::HWND, pszapplicationname: ::windows_core::PCWSTR, pphotoacquireprogresscb: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Acquire: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumfilepaths: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumResults: usize,
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
pub struct IPhotoAcquireDeviceSelectionDialog(::windows_core::IUnknown);
impl IPhotoAcquireDeviceSelectionDialog {
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn SetTitle<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, psztitle: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTitle)(::windows_core::Interface::as_raw(self), psztitle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn SetSubmitButtonText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszsubmitbuttontext: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSubmitButtonText)(::windows_core::Interface::as_raw(self), pszsubmitbuttontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoModal<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0, dwdeviceflags: u32, pbstrdeviceid: *mut super::super::Foundation::BSTR, pndevicetype: *mut DEVICE_SELECTION_DEVICE_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DoModal)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), ::core::mem::transmute(dwdeviceflags), ::core::mem::transmute(pbstrdeviceid), ::core::mem::transmute(pndevicetype)).ok()
    }
}
impl ::core::convert::From<IPhotoAcquireDeviceSelectionDialog> for ::windows_core::IUnknown {
    fn from(value: IPhotoAcquireDeviceSelectionDialog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPhotoAcquireDeviceSelectionDialog> for ::windows_core::IUnknown {
    fn from(value: &IPhotoAcquireDeviceSelectionDialog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPhotoAcquireDeviceSelectionDialog {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPhotoAcquireDeviceSelectionDialog {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPhotoAcquireDeviceSelectionDialog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPhotoAcquireDeviceSelectionDialog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPhotoAcquireDeviceSelectionDialog {}
impl ::core::fmt::Debug for IPhotoAcquireDeviceSelectionDialog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPhotoAcquireDeviceSelectionDialog").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPhotoAcquireDeviceSelectionDialog {
    type Vtable = IPhotoAcquireDeviceSelectionDialog_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00f28837_55dd_4f37_aaf5_6855a9640467);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquireDeviceSelectionDialog_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztitle: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SetSubmitButtonText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubmitbuttontext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DoModal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwdeviceflags: u32, pbstrdeviceid: *mut super::super::Foundation::BSTR, pndevicetype: *mut DEVICE_SELECTION_DEVICE_TYPE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DoModal: usize,
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
pub struct IPhotoAcquireItem(::windows_core::IUnknown);
impl IPhotoAcquireItem {
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetItemName(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetItemName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetThumbnail<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::SIZE>>(&self, sizethumbnail: Param0) -> ::windows_core::Result<super::super::Graphics::Gdi::HBITMAP> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Graphics::Gdi::HBITMAP>::zeroed();
        (::windows_core::Interface::vtable(self).GetThumbnail)(::windows_core::Interface::as_raw(self), sizethumbnail.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Graphics::Gdi::HBITMAP>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetProperty(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn SetProperty(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(pv)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanDelete(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).CanDelete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn GetSubItemCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetSubItemCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn GetSubItemAt(&self, nitemindex: u32) -> ::windows_core::Result<IPhotoAcquireItem> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSubItemAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(nitemindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPhotoAcquireItem>(result__)
    }
}
impl ::core::convert::From<IPhotoAcquireItem> for ::windows_core::IUnknown {
    fn from(value: IPhotoAcquireItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPhotoAcquireItem> for ::windows_core::IUnknown {
    fn from(value: &IPhotoAcquireItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPhotoAcquireItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPhotoAcquireItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPhotoAcquireItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPhotoAcquireItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPhotoAcquireItem {}
impl ::core::fmt::Debug for IPhotoAcquireItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPhotoAcquireItem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPhotoAcquireItem {
    type Vtable = IPhotoAcquireItem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00f21c97_28bf_4c02_b842_5e4e90139a30);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquireItem_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetItemName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstritemname: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetItemName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub GetThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sizethumbnail: super::super::Foundation::SIZE, phbmpthumbnail: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    GetThumbnail: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    SetProperty: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CanDelete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfcandelete: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CanDelete: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSubItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pncount: *mut u32) -> ::windows_core::HRESULT,
    pub GetSubItemAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nitemindex: u32, ppphotoacquireitem: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
pub struct IPhotoAcquireOptionsDialog(::windows_core::IUnknown);
impl IPhotoAcquireOptionsDialog {
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszregistryroot: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), pszregistryroot.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0) -> ::windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::HWND>::zeroed();
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn Destroy(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Destroy)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoModal<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0, ppnreturncode: *mut isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DoModal)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), ::core::mem::transmute(ppnreturncode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn SaveData(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SaveData)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IPhotoAcquireOptionsDialog> for ::windows_core::IUnknown {
    fn from(value: IPhotoAcquireOptionsDialog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPhotoAcquireOptionsDialog> for ::windows_core::IUnknown {
    fn from(value: &IPhotoAcquireOptionsDialog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPhotoAcquireOptionsDialog {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPhotoAcquireOptionsDialog {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPhotoAcquireOptionsDialog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPhotoAcquireOptionsDialog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPhotoAcquireOptionsDialog {}
impl ::core::fmt::Debug for IPhotoAcquireOptionsDialog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPhotoAcquireOptionsDialog").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPhotoAcquireOptionsDialog {
    type Vtable = IPhotoAcquireOptionsDialog_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00f2b3ee_bf64_47ee_89f4_4dedd79643f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquireOptionsDialog_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszregistryroot: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, phwnddialog: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Create: usize,
    pub Destroy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DoModal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppnreturncode: *mut isize) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DoModal: usize,
    pub SaveData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
pub struct IPhotoAcquirePlugin(::windows_core::IUnknown);
impl IPhotoAcquirePlugin {
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows_core::IntoParam<'a, IPhotoAcquireSource>, Param1: ::windows_core::IntoParam<'a, IPhotoAcquireProgressCB>>(&self, pphotoacquiresource: Param0, pphotoacquireprogresscb: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), pphotoacquiresource.into_param().abi(), pphotoacquireprogresscb.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_System_Com\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn ProcessItem<'a, Param1: ::windows_core::IntoParam<'a, IPhotoAcquireItem>, Param2: ::windows_core::IntoParam<'a, super::super::System::Com::IStream>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param4: ::windows_core::IntoParam<'a, super::super::UI::Shell::PropertiesSystem::IPropertyStore>>(&self, dwacquirestage: u32, pphotoacquireitem: Param1, poriginalitemstream: Param2, pszfinalfilename: Param3, ppropertystore: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ProcessItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwacquirestage), pphotoacquireitem.into_param().abi(), poriginalitemstream.into_param().abi(), pszfinalfilename.into_param().abi(), ppropertystore.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn TransferComplete(&self, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TransferComplete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hr)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayConfigureDialog<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisplayConfigureDialog)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IPhotoAcquirePlugin> for ::windows_core::IUnknown {
    fn from(value: IPhotoAcquirePlugin) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPhotoAcquirePlugin> for ::windows_core::IUnknown {
    fn from(value: &IPhotoAcquirePlugin) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPhotoAcquirePlugin {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPhotoAcquirePlugin {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPhotoAcquirePlugin {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPhotoAcquirePlugin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPhotoAcquirePlugin {}
impl ::core::fmt::Debug for IPhotoAcquirePlugin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPhotoAcquirePlugin").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPhotoAcquirePlugin {
    type Vtable = IPhotoAcquirePlugin_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00f2dceb_ecb8_4f77_8e47_e7a987c83dd0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquirePlugin_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pphotoacquiresource: ::windows_core::RawPtr, pphotoacquireprogresscb: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub ProcessItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwacquirestage: u32, pphotoacquireitem: ::windows_core::RawPtr, poriginalitemstream: ::windows_core::RawPtr, pszfinalfilename: ::windows_core::PCWSTR, ppropertystore: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem")))]
    ProcessItem: usize,
    pub TransferComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayConfigureDialog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayConfigureDialog: usize,
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
pub struct IPhotoAcquireProgressCB(::windows_core::IUnknown);
impl IPhotoAcquireProgressCB {
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Cancelled(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).Cancelled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn StartEnumeration<'a, Param0: ::windows_core::IntoParam<'a, IPhotoAcquireSource>>(&self, pphotoacquiresource: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StartEnumeration)(::windows_core::Interface::as_raw(self), pphotoacquiresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn FoundItem<'a, Param0: ::windows_core::IntoParam<'a, IPhotoAcquireItem>>(&self, pphotoacquireitem: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FoundItem)(::windows_core::Interface::as_raw(self), pphotoacquireitem.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn EndEnumeration(&self, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndEnumeration)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hr)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn StartTransfer<'a, Param0: ::windows_core::IntoParam<'a, IPhotoAcquireSource>>(&self, pphotoacquiresource: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StartTransfer)(::windows_core::Interface::as_raw(self), pphotoacquiresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn StartItemTransfer<'a, Param1: ::windows_core::IntoParam<'a, IPhotoAcquireItem>>(&self, nitemindex: u32, pphotoacquireitem: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StartItemTransfer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(nitemindex), pphotoacquireitem.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn DirectoryCreated<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszdirectory: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DirectoryCreated)(::windows_core::Interface::as_raw(self), pszdirectory.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateTransferPercent<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, foverall: Param0, npercent: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UpdateTransferPercent)(::windows_core::Interface::as_raw(self), foverall.into_param().abi(), ::core::mem::transmute(npercent)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn EndItemTransfer<'a, Param1: ::windows_core::IntoParam<'a, IPhotoAcquireItem>>(&self, nitemindex: u32, pphotoacquireitem: Param1, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndItemTransfer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(nitemindex), pphotoacquireitem.into_param().abi(), ::core::mem::transmute(hr)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn EndTransfer(&self, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndTransfer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hr)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn StartDelete<'a, Param0: ::windows_core::IntoParam<'a, IPhotoAcquireSource>>(&self, pphotoacquiresource: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StartDelete)(::windows_core::Interface::as_raw(self), pphotoacquiresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn StartItemDelete<'a, Param1: ::windows_core::IntoParam<'a, IPhotoAcquireItem>>(&self, nitemindex: u32, pphotoacquireitem: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StartItemDelete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(nitemindex), pphotoacquireitem.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn UpdateDeletePercent(&self, npercent: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UpdateDeletePercent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(npercent)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn EndItemDelete<'a, Param1: ::windows_core::IntoParam<'a, IPhotoAcquireItem>>(&self, nitemindex: u32, pphotoacquireitem: Param1, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndItemDelete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(nitemindex), pphotoacquireitem.into_param().abi(), ::core::mem::transmute(hr)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn EndDelete(&self, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndDelete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hr)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn EndSession(&self, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndSession)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hr)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeleteAfterAcquire(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetDeleteAfterAcquire)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn ErrorAdvise<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, hr: ::windows_core::HRESULT, pszerrormessage: Param1, nmessagetype: ERROR_ADVISE_MESSAGE_TYPE) -> ::windows_core::Result<ERROR_ADVISE_RESULT> {
        let mut result__ = ::core::mem::MaybeUninit::<ERROR_ADVISE_RESULT>::zeroed();
        (::windows_core::Interface::vtable(self).ErrorAdvise)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hr), pszerrormessage.into_param().abi(), ::core::mem::transmute(nmessagetype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ERROR_ADVISE_RESULT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetUserInput<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, riidtype: *const ::windows_core::GUID, punknown: Param1, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetUserInput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riidtype), punknown.into_param().abi(), ::core::mem::transmute(ppropvarresult), ::core::mem::transmute(ppropvardefault)).ok()
    }
}
impl ::core::convert::From<IPhotoAcquireProgressCB> for ::windows_core::IUnknown {
    fn from(value: IPhotoAcquireProgressCB) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPhotoAcquireProgressCB> for ::windows_core::IUnknown {
    fn from(value: &IPhotoAcquireProgressCB) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPhotoAcquireProgressCB {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPhotoAcquireProgressCB {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPhotoAcquireProgressCB {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPhotoAcquireProgressCB {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPhotoAcquireProgressCB {}
impl ::core::fmt::Debug for IPhotoAcquireProgressCB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPhotoAcquireProgressCB").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPhotoAcquireProgressCB {
    type Vtable = IPhotoAcquireProgressCB_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00f2ce1e_935e_4248_892c_130f32c45cb4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquireProgressCB_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Cancelled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfcancelled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Cancelled: usize,
    pub StartEnumeration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pphotoacquiresource: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FoundItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pphotoacquireitem: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EndEnumeration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub StartTransfer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pphotoacquiresource: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StartItemTransfer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nitemindex: u32, pphotoacquireitem: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DirectoryCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdirectory: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UpdateTransferPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, foverall: super::super::Foundation::BOOL, npercent: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UpdateTransferPercent: usize,
    pub EndItemTransfer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nitemindex: u32, pphotoacquireitem: ::windows_core::RawPtr, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub EndTransfer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub StartDelete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pphotoacquiresource: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StartItemDelete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nitemindex: u32, pphotoacquireitem: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UpdateDeletePercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, npercent: u32) -> ::windows_core::HRESULT,
    pub EndItemDelete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nitemindex: u32, pphotoacquireitem: ::windows_core::RawPtr, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub EndDelete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub EndSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDeleteAfterAcquire: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfdeleteafteracquire: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDeleteAfterAcquire: usize,
    pub ErrorAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT, pszerrormessage: ::windows_core::PCWSTR, nmessagetype: ERROR_ADVISE_MESSAGE_TYPE, pnerroradviseresult: *mut ERROR_ADVISE_RESULT) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetUserInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riidtype: *const ::windows_core::GUID, punknown: *mut ::core::ffi::c_void, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetUserInput: usize,
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
pub struct IPhotoAcquireSettings(::windows_core::IUnknown);
impl IPhotoAcquireSettings {
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn InitializeFromRegistry<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszregistrykey: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InitializeFromRegistry)(::windows_core::Interface::as_raw(self), pszregistrykey.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn SetFlags(&self, dwphotoacquireflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwphotoacquireflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn SetOutputFilenameTemplate<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, psztemplate: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOutputFilenameTemplate)(::windows_core::Interface::as_raw(self), psztemplate.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn SetSequencePaddingWidth(&self, dwwidth: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSequencePaddingWidth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwwidth)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSequenceZeroPadding<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fzeropad: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSequenceZeroPadding)(::windows_core::Interface::as_raw(self), fzeropad.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn SetGroupTag<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszgrouptag: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGroupTag)(::windows_core::Interface::as_raw(self), pszgrouptag.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAcquisitionTime(&self, pftacquisitiontime: *const super::super::Foundation::FILETIME) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAcquisitionTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pftacquisitiontime)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn GetFlags(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOutputFilenameTemplate(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetOutputFilenameTemplate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn GetSequencePaddingWidth(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetSequencePaddingWidth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSequenceZeroPadding(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetSequenceZeroPadding)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGroupTag(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetGroupTag)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAcquisitionTime(&self) -> ::windows_core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::FILETIME>::zeroed();
        (::windows_core::Interface::vtable(self).GetAcquisitionTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
}
impl ::core::convert::From<IPhotoAcquireSettings> for ::windows_core::IUnknown {
    fn from(value: IPhotoAcquireSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPhotoAcquireSettings> for ::windows_core::IUnknown {
    fn from(value: &IPhotoAcquireSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPhotoAcquireSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPhotoAcquireSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPhotoAcquireSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPhotoAcquireSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPhotoAcquireSettings {}
impl ::core::fmt::Debug for IPhotoAcquireSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPhotoAcquireSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPhotoAcquireSettings {
    type Vtable = IPhotoAcquireSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00f2b868_dd67_487c_9553_049240767e91);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquireSettings_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub InitializeFromRegistry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszregistrykey: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwphotoacquireflags: u32) -> ::windows_core::HRESULT,
    pub SetOutputFilenameTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztemplate: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SetSequencePaddingWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwwidth: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSequenceZeroPadding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fzeropad: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSequenceZeroPadding: usize,
    pub SetGroupTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszgrouptag: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAcquisitionTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pftacquisitiontime: *const super::super::Foundation::FILETIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAcquisitionTime: usize,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwphotoacquireflags: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetOutputFilenameTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtemplate: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetOutputFilenameTemplate: usize,
    pub GetSequencePaddingWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwwidth: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSequenceZeroPadding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfzeropad: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSequenceZeroPadding: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetGroupTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrgrouptag: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetGroupTag: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAcquisitionTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pftacquisitiontime: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAcquisitionTime: usize,
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
pub struct IPhotoAcquireSource(::windows_core::IUnknown);
impl IPhotoAcquireSource {
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFriendlyName(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetFriendlyName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetDeviceIcons(&self, nsize: u32, phlargeicon: *mut super::super::UI::WindowsAndMessaging::HICON, phsmallicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDeviceIcons)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(nsize), ::core::mem::transmute(phlargeicon), ::core::mem::transmute(phsmallicon)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitializeItemList<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows_core::IntoParam<'a, IPhotoAcquireProgressCB>>(&self, fforceenumeration: Param0, pphotoacquireprogresscb: Param1, pnitemcount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InitializeItemList)(::windows_core::Interface::as_raw(self), fforceenumeration.into_param().abi(), pphotoacquireprogresscb.into_param().abi(), ::core::mem::transmute(pnitemcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn GetItemCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetItemCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn GetItemAt(&self, nindex: u32) -> ::windows_core::Result<IPhotoAcquireItem> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetItemAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(nindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPhotoAcquireItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn GetPhotoAcquireSettings(&self) -> ::windows_core::Result<IPhotoAcquireSettings> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetPhotoAcquireSettings)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPhotoAcquireSettings>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceId(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetDeviceId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn BindToObject(&self, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BindToObject)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
}
impl ::core::convert::From<IPhotoAcquireSource> for ::windows_core::IUnknown {
    fn from(value: IPhotoAcquireSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPhotoAcquireSource> for ::windows_core::IUnknown {
    fn from(value: &IPhotoAcquireSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPhotoAcquireSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPhotoAcquireSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPhotoAcquireSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPhotoAcquireSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPhotoAcquireSource {}
impl ::core::fmt::Debug for IPhotoAcquireSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPhotoAcquireSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPhotoAcquireSource {
    type Vtable = IPhotoAcquireSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00f2c703_8613_4282_a53b_6ec59c5883ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquireSource_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfriendlyname: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFriendlyName: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetDeviceIcons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nsize: u32, phlargeicon: *mut super::super::UI::WindowsAndMessaging::HICON, phsmallicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetDeviceIcons: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeItemList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fforceenumeration: super::super::Foundation::BOOL, pphotoacquireprogresscb: ::windows_core::RawPtr, pnitemcount: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeItemList: usize,
    pub GetItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnitemcount: *mut u32) -> ::windows_core::HRESULT,
    pub GetItemAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, ppphotoacquireitem: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetPhotoAcquireSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppphotoacquiresettings: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdeviceid: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDeviceId: usize,
    pub BindToObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
pub struct IPhotoProgressActionCB(::windows_core::IUnknown);
impl IPhotoProgressActionCB {
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoAction<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DoAction)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IPhotoProgressActionCB> for ::windows_core::IUnknown {
    fn from(value: IPhotoProgressActionCB) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPhotoProgressActionCB> for ::windows_core::IUnknown {
    fn from(value: &IPhotoProgressActionCB) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPhotoProgressActionCB {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPhotoProgressActionCB {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPhotoProgressActionCB {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPhotoProgressActionCB {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPhotoProgressActionCB {}
impl ::core::fmt::Debug for IPhotoProgressActionCB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPhotoProgressActionCB").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPhotoProgressActionCB {
    type Vtable = IPhotoProgressActionCB_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00f242d0_b206_4e7d_b4c1_4755bcbb9c9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoProgressActionCB_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub DoAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DoAction: usize,
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
pub struct IPhotoProgressDialog(::windows_core::IUnknown);
impl IPhotoProgressDialog {
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self) -> ::windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::HWND>::zeroed();
        (::windows_core::Interface::vtable(self).GetWindow)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn Destroy(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Destroy)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn SetTitle<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, psztitle: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTitle)(::windows_core::Interface::as_raw(self), psztitle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowCheckbox<'a, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fshow: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ShowCheckbox)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ncheckboxid), fshow.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn SetCheckboxText<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtext: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCheckboxText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ncheckboxid), pszcheckboxtext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCheckboxCheck<'a, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fchecked: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCheckboxCheck)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ncheckboxid), fchecked.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn SetCheckboxTooltip<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtooltiptext: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCheckboxTooltip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ncheckboxid), pszcheckboxtooltiptext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCheckboxChecked(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).IsCheckboxChecked)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ncheckboxid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn SetCaption<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, psztitle: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCaption)(::windows_core::Interface::as_raw(self), psztitle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn SetImage<'a, Param1: ::windows_core::IntoParam<'a, super::super::UI::WindowsAndMessaging::HICON>, Param2: ::windows_core::IntoParam<'a, super::super::Graphics::Gdi::HBITMAP>>(&self, nimagetype: PROGRESS_DIALOG_IMAGE_TYPE, hicon: Param1, hbitmap: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetImage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(nimagetype), hicon.into_param().abi(), hbitmap.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn SetPercentComplete(&self, npercent: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPercentComplete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(npercent)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn SetProgressText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszprogresstext: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProgressText)(::windows_core::Interface::as_raw(self), pszprogresstext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn SetActionLinkCallback<'a, Param0: ::windows_core::IntoParam<'a, IPhotoProgressActionCB>>(&self, pphotoprogressactioncb: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetActionLinkCallback)(::windows_core::Interface::as_raw(self), pphotoprogressactioncb.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn SetActionLinkText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszcaption: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetActionLinkText)(::windows_core::Interface::as_raw(self), pszcaption.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowActionLink<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fshow: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ShowActionLink)(::windows_core::Interface::as_raw(self), fshow.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCancelled(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).IsCancelled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetUserInput<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, riidtype: *const ::windows_core::GUID, punknown: Param1, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetUserInput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riidtype), punknown.into_param().abi(), ::core::mem::transmute(ppropvarresult), ::core::mem::transmute(ppropvardefault)).ok()
    }
}
impl ::core::convert::From<IPhotoProgressDialog> for ::windows_core::IUnknown {
    fn from(value: IPhotoProgressDialog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPhotoProgressDialog> for ::windows_core::IUnknown {
    fn from(value: &IPhotoProgressDialog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPhotoProgressDialog {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPhotoProgressDialog {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPhotoProgressDialog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPhotoProgressDialog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPhotoProgressDialog {}
impl ::core::fmt::Debug for IPhotoProgressDialog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPhotoProgressDialog").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPhotoProgressDialog {
    type Vtable = IPhotoProgressDialog_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00f246f9_0750_4f08_9381_2cd8e906a4ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoProgressDialog_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Create: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwndprogressdialog: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWindow: usize,
    pub Destroy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztitle: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowCheckbox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fshow: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowCheckbox: usize,
    pub SetCheckboxText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCheckboxCheck: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fchecked: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCheckboxCheck: usize,
    pub SetCheckboxTooltip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtooltiptext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsCheckboxChecked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pfchecked: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsCheckboxChecked: usize,
    pub SetCaption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztitle: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub SetImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nimagetype: PROGRESS_DIALOG_IMAGE_TYPE, hicon: super::super::UI::WindowsAndMessaging::HICON, hbitmap: super::super::Graphics::Gdi::HBITMAP) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging")))]
    SetImage: usize,
    pub SetPercentComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, npercent: i32) -> ::windows_core::HRESULT,
    pub SetProgressText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszprogresstext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SetActionLinkCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pphotoprogressactioncb: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetActionLinkText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcaption: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowActionLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowActionLink: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsCancelled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfcancelled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsCancelled: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetUserInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riidtype: *const ::windows_core::GUID, punknown: *mut ::core::ffi::c_void, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetUserInput: usize,
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
pub struct IUserInputString(::windows_core::IUnknown);
impl IUserInputString {
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSubmitButtonText(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetSubmitButtonText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPrompt(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetPrompt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStringId(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetStringId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn GetStringType(&self) -> ::windows_core::Result<USER_INPUT_STRING_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<USER_INPUT_STRING_TYPE>::zeroed();
        (::windows_core::Interface::vtable(self).GetStringType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<USER_INPUT_STRING_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTooltipText(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetTooltipText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn GetMaxLength(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetMaxLength)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDefault(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetDefault)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
    pub unsafe fn GetMruCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetMruCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMruEntryAt(&self, nindex: u32) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetMruEntryAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(nindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn GetImage(&self, nsize: u32, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP, phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetImage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(nsize), ::core::mem::transmute(phbitmap), ::core::mem::transmute(phicon)).ok()
    }
}
impl ::core::convert::From<IUserInputString> for ::windows_core::IUnknown {
    fn from(value: IUserInputString) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserInputString> for ::windows_core::IUnknown {
    fn from(value: &IUserInputString) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUserInputString {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUserInputString {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUserInputString {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUserInputString {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUserInputString {}
impl ::core::fmt::Debug for IUserInputString {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUserInputString").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUserInputString {
    type Vtable = IUserInputString_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00f243a1_205b_45ba_ae26_abbc53aa7a6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserInputString_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSubmitButtonText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubmitbuttontext: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSubmitButtonText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPrompt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprompttitle: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPrompt: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetStringId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrstringid: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetStringId: usize,
    pub GetStringType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnstringtype: *mut USER_INPUT_STRING_TYPE) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTooltipText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtooltiptext: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTooltipText: usize,
    pub GetMaxLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchmaxlength: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdefault: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDefault: usize,
    pub GetMruCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnmrucount: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMruEntryAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, pbstrmruentry: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMruEntryAt: usize,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub GetImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nsize: u32, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP, phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows_core::HRESULT,
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
pub const PHOTOACQ_ERROR_RESTART_REQUIRED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147180543i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_IMPORT_VIDEO_AS_MULTIPLE_FILES: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_NO_GALLERY_LAUNCH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_RUN_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_CameraSequenceNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 7u32 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_DuplicateDetectionID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 10u32 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_FinalFilename: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_GroupTag: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 4u32 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_IntermediateFile: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 8u32 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_OriginalFilename: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 6u32 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_RelativePathname: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_SkipImport: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 9u32 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_TransferResult: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 5u32 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROGRESS_DIALOG_CHECKBOX_ID(pub i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PROGRESS_DIALOG_CHECKBOX_ID_DEFAULT: PROGRESS_DIALOG_CHECKBOX_ID = PROGRESS_DIALOG_CHECKBOX_ID(0i32);
impl ::core::marker::Copy for PROGRESS_DIALOG_CHECKBOX_ID {}
impl ::core::clone::Clone for PROGRESS_DIALOG_CHECKBOX_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROGRESS_DIALOG_CHECKBOX_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PROGRESS_DIALOG_CHECKBOX_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROGRESS_DIALOG_CHECKBOX_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROGRESS_DIALOG_CHECKBOX_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROGRESS_DIALOG_IMAGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PROGRESS_DIALOG_ICON_SMALL: PROGRESS_DIALOG_IMAGE_TYPE = PROGRESS_DIALOG_IMAGE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PROGRESS_DIALOG_ICON_LARGE: PROGRESS_DIALOG_IMAGE_TYPE = PROGRESS_DIALOG_IMAGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PROGRESS_DIALOG_ICON_THUMBNAIL: PROGRESS_DIALOG_IMAGE_TYPE = PROGRESS_DIALOG_IMAGE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PROGRESS_DIALOG_BITMAP_THUMBNAIL: PROGRESS_DIALOG_IMAGE_TYPE = PROGRESS_DIALOG_IMAGE_TYPE(3i32);
impl ::core::marker::Copy for PROGRESS_DIALOG_IMAGE_TYPE {}
impl ::core::clone::Clone for PROGRESS_DIALOG_IMAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROGRESS_DIALOG_IMAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PROGRESS_DIALOG_IMAGE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROGRESS_DIALOG_IMAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROGRESS_DIALOG_IMAGE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PROGRESS_INDETERMINATE: i32 = -1i32;
pub const PhotoAcquire: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00f26e02_e9f2_4a9f_9fdd_5a962fb26a98);
pub const PhotoAcquireAutoPlayDropTarget: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00f20eb5_8fd6_4d9d_b75e_36801766c8f1);
pub const PhotoAcquireAutoPlayHWEventHandler: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00f2b433_44e4_4d88_b2b0_2698a0a91dba);
pub const PhotoAcquireDeviceSelectionDialog: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00f29a34_b8a1_482c_bcf8_3ac7b0fe8f62);
pub const PhotoAcquireOptionsDialog: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00f210a1_62f0_438b_9f7e_9618d72a1831);
pub const PhotoProgressDialog: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00f24ca0_748f_4e8a_894f_0e0357c6799f);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct USER_INPUT_STRING_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const USER_INPUT_DEFAULT: USER_INPUT_STRING_TYPE = USER_INPUT_STRING_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const USER_INPUT_PATH_ELEMENT: USER_INPUT_STRING_TYPE = USER_INPUT_STRING_TYPE(1i32);
impl ::core::marker::Copy for USER_INPUT_STRING_TYPE {}
impl ::core::clone::Clone for USER_INPUT_STRING_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for USER_INPUT_STRING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for USER_INPUT_STRING_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for USER_INPUT_STRING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USER_INPUT_STRING_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
