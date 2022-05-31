#[doc(hidden)]
#[repr(transparent)]
pub struct IImageScanner(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImageScanner {
    type Vtable = IImageScanner_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x53a88f78_5298_48a0_8da3_8087519665e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageScanner_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DefaultScanSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerScanSource) -> ::windows_core::HRESULT,
    pub IsScanSourceSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ImageScannerScanSource, result__: *mut bool) -> ::windows_core::HRESULT,
    pub FlatbedConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FeederConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AutoConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsPreviewSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scansource: ImageScannerScanSource, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ScanPreviewToStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scansource: ImageScannerScanSource, targetstream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ScanPreviewToStreamAsync: usize,
    #[cfg(feature = "Storage")]
    pub ScanFilesToFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scansource: ImageScannerScanSource, storagefolder: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    ScanFilesToFolderAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageScannerFeederConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImageScannerFeederConfiguration {
    type Vtable = IImageScannerFeederConfiguration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x74bdacee_fa97_4c17_8280_40e39c6dcc67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageScannerFeederConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CanAutoDetectPageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub AutoDetectPageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAutoDetectPageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Graphics_Printing")]
    pub PageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_graphics::Printing::PrintMediaSize) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    PageSize: usize,
    #[cfg(feature = "Graphics_Printing")]
    pub SetPageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_graphics::Printing::PrintMediaSize) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    SetPageSize: usize,
    #[cfg(feature = "Graphics_Printing")]
    pub PageOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_graphics::Printing::PrintOrientation) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    PageOrientation: usize,
    #[cfg(feature = "Graphics_Printing")]
    pub SetPageOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_graphics::Printing::PrintOrientation) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    SetPageOrientation: usize,
    pub PageSizeDimensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Size) -> ::windows_core::HRESULT,
    #[cfg(feature = "Graphics_Printing")]
    pub IsPageSizeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagesize: ::winrt_graphics::Printing::PrintMediaSize, pageorientation: ::winrt_graphics::Printing::PrintOrientation, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    IsPageSizeSupported: usize,
    pub MaxNumberOfPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetMaxNumberOfPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub CanScanDuplex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Duplex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetDuplex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CanScanAhead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ScanAhead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetScanAhead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IImageScannerFormatConfiguration(::windows_core::IUnknown);
impl IImageScannerFormatConfiguration {
    pub fn DefaultFormat(&self) -> ::windows_core::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerFormat>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerFormat>(result__)
        }
    }
    pub fn Format(&self) -> ::windows_core::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerFormat>::zeroed();
            (::windows_core::Interface::vtable(this).Format)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerFormat>(result__)
        }
    }
    pub fn SetFormat(&self, value: ImageScannerFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsFormatSupported(&self, value: ImageScannerFormat) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsFormatSupported)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::convert::From<IImageScannerFormatConfiguration> for ::windows_core::IUnknown {
    fn from(value: IImageScannerFormatConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IImageScannerFormatConfiguration> for ::windows_core::IUnknown {
    fn from(value: &IImageScannerFormatConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IImageScannerFormatConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IImageScannerFormatConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IImageScannerFormatConfiguration> for ::windows_core::IInspectable {
    fn from(value: IImageScannerFormatConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IImageScannerFormatConfiguration> for ::windows_core::IInspectable {
    fn from(value: &IImageScannerFormatConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IImageScannerFormatConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IImageScannerFormatConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IImageScannerFormatConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IImageScannerFormatConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IImageScannerFormatConfiguration {}
impl ::core::fmt::Debug for IImageScannerFormatConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IImageScannerFormatConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IImageScannerFormatConfiguration {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{ae275d11-dadf-4010-bf10-cca5c83dcbb0}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IImageScannerFormatConfiguration {
    type Vtable = IImageScannerFormatConfiguration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae275d11_dadf_4010_bf10_cca5c83dcbb0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageScannerFormatConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DefaultFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerFormat) -> ::windows_core::HRESULT,
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerFormat) -> ::windows_core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ImageScannerFormat) -> ::windows_core::HRESULT,
    pub IsFormatSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ImageScannerFormat, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageScannerPreviewResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImageScannerPreviewResult {
    type Vtable = IImageScannerPreviewResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08b7fe8e_8891_441d_be9c_176fa109c8bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageScannerPreviewResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerFormat) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageScannerScanResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImageScannerScanResult {
    type Vtable = IImageScannerScanResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc91624cd_9037_4e48_84c1_ac0975076bc5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageScannerScanResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub ScannedFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    ScannedFiles: usize,
}
#[repr(transparent)]
pub struct IImageScannerSourceConfiguration(::windows_core::IUnknown);
impl IImageScannerSourceConfiguration {
    pub fn MinScanArea(&self) -> ::windows_core::Result<::winrt_foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).MinScanArea)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Size>(result__)
        }
    }
    pub fn MaxScanArea(&self) -> ::windows_core::Result<::winrt_foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).MaxScanArea)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Size>(result__)
        }
    }
    pub fn SelectedScanRegion(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedScanRegion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    pub fn SetSelectedScanRegion<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedScanRegion)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn AutoCroppingMode(&self) -> ::windows_core::Result<ImageScannerAutoCroppingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerAutoCroppingMode>::zeroed();
            (::windows_core::Interface::vtable(this).AutoCroppingMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerAutoCroppingMode>(result__)
        }
    }
    pub fn SetAutoCroppingMode(&self, value: ImageScannerAutoCroppingMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAutoCroppingMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAutoCroppingModeSupported(&self, value: ImageScannerAutoCroppingMode) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAutoCroppingModeSupported)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn MinResolution(&self) -> ::windows_core::Result<ImageScannerResolution> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerResolution>::zeroed();
            (::windows_core::Interface::vtable(this).MinResolution)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerResolution>(result__)
        }
    }
    pub fn MaxResolution(&self) -> ::windows_core::Result<ImageScannerResolution> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerResolution>::zeroed();
            (::windows_core::Interface::vtable(this).MaxResolution)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerResolution>(result__)
        }
    }
    pub fn OpticalResolution(&self) -> ::windows_core::Result<ImageScannerResolution> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerResolution>::zeroed();
            (::windows_core::Interface::vtable(this).OpticalResolution)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerResolution>(result__)
        }
    }
    pub fn DesiredResolution(&self) -> ::windows_core::Result<ImageScannerResolution> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerResolution>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredResolution)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerResolution>(result__)
        }
    }
    pub fn SetDesiredResolution<'a, Param0: ::windows_core::IntoParam<'a, ImageScannerResolution>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredResolution)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ActualResolution(&self) -> ::windows_core::Result<ImageScannerResolution> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerResolution>::zeroed();
            (::windows_core::Interface::vtable(this).ActualResolution)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerResolution>(result__)
        }
    }
    pub fn DefaultColorMode(&self) -> ::windows_core::Result<ImageScannerColorMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerColorMode>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultColorMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerColorMode>(result__)
        }
    }
    pub fn ColorMode(&self) -> ::windows_core::Result<ImageScannerColorMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerColorMode>::zeroed();
            (::windows_core::Interface::vtable(this).ColorMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerColorMode>(result__)
        }
    }
    pub fn SetColorMode(&self, value: ImageScannerColorMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColorMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsColorModeSupported(&self, value: ImageScannerColorMode) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsColorModeSupported)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn MinBrightness(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).MinBrightness)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn MaxBrightness(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxBrightness)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn BrightnessStep(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).BrightnessStep)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn DefaultBrightness(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultBrightness)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Brightness(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Brightness)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetBrightness(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBrightness)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinContrast(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).MinContrast)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn MaxContrast(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxContrast)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn ContrastStep(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ContrastStep)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn DefaultContrast(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultContrast)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Contrast(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Contrast)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetContrast(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContrast)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DefaultFormat(&self) -> ::windows_core::Result<ImageScannerFormat> {
        let this = &::windows_core::Interface::cast::<IImageScannerFormatConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerFormat>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerFormat>(result__)
        }
    }
    pub fn Format(&self) -> ::windows_core::Result<ImageScannerFormat> {
        let this = &::windows_core::Interface::cast::<IImageScannerFormatConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerFormat>::zeroed();
            (::windows_core::Interface::vtable(this).Format)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerFormat>(result__)
        }
    }
    pub fn SetFormat(&self, value: ImageScannerFormat) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IImageScannerFormatConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsFormatSupported(&self, value: ImageScannerFormat) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IImageScannerFormatConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsFormatSupported)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::convert::From<IImageScannerSourceConfiguration> for ::windows_core::IUnknown {
    fn from(value: IImageScannerSourceConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IImageScannerSourceConfiguration> for ::windows_core::IUnknown {
    fn from(value: &IImageScannerSourceConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IImageScannerSourceConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IImageScannerSourceConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IImageScannerSourceConfiguration> for ::windows_core::IInspectable {
    fn from(value: IImageScannerSourceConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IImageScannerSourceConfiguration> for ::windows_core::IInspectable {
    fn from(value: &IImageScannerSourceConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IImageScannerSourceConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IImageScannerSourceConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IImageScannerSourceConfiguration> for IImageScannerFormatConfiguration {
    type Error = ::windows_core::Error;
    fn try_from(value: IImageScannerSourceConfiguration) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IImageScannerSourceConfiguration> for IImageScannerFormatConfiguration {
    type Error = ::windows_core::Error;
    fn try_from(value: &IImageScannerSourceConfiguration) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IImageScannerFormatConfiguration> for IImageScannerSourceConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, IImageScannerFormatConfiguration> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IImageScannerFormatConfiguration> for &IImageScannerSourceConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, IImageScannerFormatConfiguration> {
        ::core::convert::TryInto::<IImageScannerFormatConfiguration>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IImageScannerSourceConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IImageScannerSourceConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IImageScannerSourceConfiguration {}
impl ::core::fmt::Debug for IImageScannerSourceConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IImageScannerSourceConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IImageScannerSourceConfiguration {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{bfb50055-0b44-4c82-9e89-205f9c234e59}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IImageScannerSourceConfiguration {
    type Vtable = IImageScannerSourceConfiguration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbfb50055_0b44_4c82_9e89_205f9c234e59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageScannerSourceConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MinScanArea: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Size) -> ::windows_core::HRESULT,
    pub MaxScanArea: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Size) -> ::windows_core::HRESULT,
    pub SelectedScanRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    pub SetSelectedScanRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    pub AutoCroppingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerAutoCroppingMode) -> ::windows_core::HRESULT,
    pub SetAutoCroppingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ImageScannerAutoCroppingMode) -> ::windows_core::HRESULT,
    pub IsAutoCroppingModeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ImageScannerAutoCroppingMode, result__: *mut bool) -> ::windows_core::HRESULT,
    pub MinResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerResolution) -> ::windows_core::HRESULT,
    pub MaxResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerResolution) -> ::windows_core::HRESULT,
    pub OpticalResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerResolution) -> ::windows_core::HRESULT,
    pub DesiredResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerResolution) -> ::windows_core::HRESULT,
    pub SetDesiredResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ImageScannerResolution) -> ::windows_core::HRESULT,
    pub ActualResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerResolution) -> ::windows_core::HRESULT,
    pub DefaultColorMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerColorMode) -> ::windows_core::HRESULT,
    pub ColorMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ImageScannerColorMode) -> ::windows_core::HRESULT,
    pub SetColorMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ImageScannerColorMode) -> ::windows_core::HRESULT,
    pub IsColorModeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ImageScannerColorMode, result__: *mut bool) -> ::windows_core::HRESULT,
    pub MinBrightness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MaxBrightness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub BrightnessStep: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub DefaultBrightness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Brightness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetBrightness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub MinContrast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MaxContrast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub ContrastStep: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub DefaultContrast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Contrast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetContrast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageScannerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImageScannerStatics {
    type Vtable = IImageScannerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc57e70e_d804_4477_9fb5_b911b5473897);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageScannerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ImageScanner(::windows_core::IUnknown);
impl ImageScanner {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DefaultScanSource(&self) -> ::windows_core::Result<ImageScannerScanSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerScanSource>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultScanSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerScanSource>(result__)
        }
    }
    pub fn IsScanSourceSupported(&self, value: ImageScannerScanSource) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsScanSourceSupported)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn FlatbedConfiguration(&self) -> ::windows_core::Result<ImageScannerFlatbedConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FlatbedConfiguration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerFlatbedConfiguration>(result__)
        }
    }
    pub fn FeederConfiguration(&self) -> ::windows_core::Result<ImageScannerFeederConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FeederConfiguration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerFeederConfiguration>(result__)
        }
    }
    pub fn AutoConfiguration(&self) -> ::windows_core::Result<ImageScannerAutoConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AutoConfiguration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerAutoConfiguration>(result__)
        }
    }
    pub fn IsPreviewSupported(&self, scansource: ImageScannerScanSource) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPreviewSupported)(::windows_core::Interface::as_raw(this), scansource, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ScanPreviewToStreamAsync<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>>(&self, scansource: ImageScannerScanSource, targetstream: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ImageScannerPreviewResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ScanPreviewToStreamAsync)(::windows_core::Interface::as_raw(this), scansource, targetstream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ImageScannerPreviewResult>>(result__)
        }
    }
    #[cfg(feature = "Storage")]
    pub fn ScanFilesToFolderAsync<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::StorageFolder>>(&self, scansource: ImageScannerScanSource, storagefolder: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<ImageScannerScanResult, u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ScanFilesToFolderAsync)(::windows_core::Interface::as_raw(this), scansource, storagefolder.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<ImageScannerScanResult, u32>>(result__)
        }
    }
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ImageScanner>> {
        Self::IImageScannerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ImageScanner>>(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IImageScannerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IImageScannerStatics<R, F: FnOnce(&IImageScannerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ImageScanner, IImageScannerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ImageScanner {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ImageScanner {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ImageScanner {}
impl ::core::fmt::Debug for ImageScanner {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageScanner").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ImageScanner {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Scanners.ImageScanner;{53a88f78-5298-48a0-8da3-8087519665e0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ImageScanner {
    type Vtable = IImageScanner_Vtbl;
    const IID: ::windows_core::GUID = <IImageScanner as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ImageScanner {
    const NAME: &'static str = "Windows.Devices.Scanners.ImageScanner";
}
impl ::core::convert::From<ImageScanner> for ::windows_core::IUnknown {
    fn from(value: ImageScanner) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageScanner> for ::windows_core::IUnknown {
    fn from(value: &ImageScanner) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ImageScanner {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ImageScanner {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ImageScanner> for ::windows_core::IInspectable {
    fn from(value: ImageScanner) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageScanner> for ::windows_core::IInspectable {
    fn from(value: &ImageScanner) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ImageScanner {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ImageScanner {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ImageScanner {}
unsafe impl ::core::marker::Sync for ImageScanner {}
#[repr(transparent)]
pub struct ImageScannerAutoConfiguration(::windows_core::IUnknown);
impl ImageScannerAutoConfiguration {
    pub fn DefaultFormat(&self) -> ::windows_core::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerFormat>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerFormat>(result__)
        }
    }
    pub fn Format(&self) -> ::windows_core::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerFormat>::zeroed();
            (::windows_core::Interface::vtable(this).Format)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerFormat>(result__)
        }
    }
    pub fn SetFormat(&self, value: ImageScannerFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsFormatSupported(&self, value: ImageScannerFormat) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsFormatSupported)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for ImageScannerAutoConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ImageScannerAutoConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ImageScannerAutoConfiguration {}
impl ::core::fmt::Debug for ImageScannerAutoConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageScannerAutoConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ImageScannerAutoConfiguration {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Scanners.ImageScannerAutoConfiguration;{ae275d11-dadf-4010-bf10-cca5c83dcbb0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ImageScannerAutoConfiguration {
    type Vtable = IImageScannerFormatConfiguration_Vtbl;
    const IID: ::windows_core::GUID = <IImageScannerFormatConfiguration as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ImageScannerAutoConfiguration {
    const NAME: &'static str = "Windows.Devices.Scanners.ImageScannerAutoConfiguration";
}
impl ::core::convert::From<ImageScannerAutoConfiguration> for ::windows_core::IUnknown {
    fn from(value: ImageScannerAutoConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageScannerAutoConfiguration> for ::windows_core::IUnknown {
    fn from(value: &ImageScannerAutoConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ImageScannerAutoConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ImageScannerAutoConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ImageScannerAutoConfiguration> for ::windows_core::IInspectable {
    fn from(value: ImageScannerAutoConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageScannerAutoConfiguration> for ::windows_core::IInspectable {
    fn from(value: &ImageScannerAutoConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ImageScannerAutoConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ImageScannerAutoConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ImageScannerAutoConfiguration> for IImageScannerFormatConfiguration {
    type Error = ::windows_core::Error;
    fn try_from(value: ImageScannerAutoConfiguration) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ImageScannerAutoConfiguration> for IImageScannerFormatConfiguration {
    type Error = ::windows_core::Error;
    fn try_from(value: &ImageScannerAutoConfiguration) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IImageScannerFormatConfiguration> for ImageScannerAutoConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, IImageScannerFormatConfiguration> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IImageScannerFormatConfiguration> for &ImageScannerAutoConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, IImageScannerFormatConfiguration> {
        ::core::convert::TryInto::<IImageScannerFormatConfiguration>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ImageScannerAutoConfiguration {}
unsafe impl ::core::marker::Sync for ImageScannerAutoConfiguration {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ImageScannerAutoCroppingMode(pub i32);
impl ImageScannerAutoCroppingMode {
    pub const Disabled: Self = Self(0i32);
    pub const SingleRegion: Self = Self(1i32);
    pub const MultipleRegion: Self = Self(2i32);
}
impl ::core::marker::Copy for ImageScannerAutoCroppingMode {}
impl ::core::clone::Clone for ImageScannerAutoCroppingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ImageScannerAutoCroppingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ImageScannerAutoCroppingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for ImageScannerAutoCroppingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageScannerAutoCroppingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ImageScannerAutoCroppingMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Scanners.ImageScannerAutoCroppingMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ImageScannerColorMode(pub i32);
impl ImageScannerColorMode {
    pub const Color: Self = Self(0i32);
    pub const Grayscale: Self = Self(1i32);
    pub const Monochrome: Self = Self(2i32);
    pub const AutoColor: Self = Self(3i32);
}
impl ::core::marker::Copy for ImageScannerColorMode {}
impl ::core::clone::Clone for ImageScannerColorMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ImageScannerColorMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ImageScannerColorMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for ImageScannerColorMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageScannerColorMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ImageScannerColorMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Scanners.ImageScannerColorMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ImageScannerFeederConfiguration(::windows_core::IUnknown);
impl ImageScannerFeederConfiguration {
    pub fn CanAutoDetectPageSize(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanAutoDetectPageSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn AutoDetectPageSize(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AutoDetectPageSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAutoDetectPageSize(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAutoDetectPageSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Printing")]
    pub fn PageSize(&self) -> ::windows_core::Result<::winrt_graphics::Printing::PrintMediaSize> {
        let this = &::windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_graphics::Printing::PrintMediaSize>::zeroed();
            (::windows_core::Interface::vtable(this).PageSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::Printing::PrintMediaSize>(result__)
        }
    }
    #[cfg(feature = "Graphics_Printing")]
    pub fn SetPageSize(&self, value: ::winrt_graphics::Printing::PrintMediaSize) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPageSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Printing")]
    pub fn PageOrientation(&self) -> ::windows_core::Result<::winrt_graphics::Printing::PrintOrientation> {
        let this = &::windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_graphics::Printing::PrintOrientation>::zeroed();
            (::windows_core::Interface::vtable(this).PageOrientation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::Printing::PrintOrientation>(result__)
        }
    }
    #[cfg(feature = "Graphics_Printing")]
    pub fn SetPageOrientation(&self, value: ::winrt_graphics::Printing::PrintOrientation) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPageOrientation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PageSizeDimensions(&self) -> ::windows_core::Result<::winrt_foundation::Size> {
        let this = &::windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).PageSizeDimensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Graphics_Printing")]
    pub fn IsPageSizeSupported(&self, pagesize: ::winrt_graphics::Printing::PrintMediaSize, pageorientation: ::winrt_graphics::Printing::PrintOrientation) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPageSizeSupported)(::windows_core::Interface::as_raw(this), pagesize, pageorientation, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn MaxNumberOfPages(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxNumberOfPages)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetMaxNumberOfPages(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxNumberOfPages)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanScanDuplex(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanScanDuplex)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Duplex(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Duplex)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetDuplex(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDuplex)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanScanAhead(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanScanAhead)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ScanAhead(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ScanAhead)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetScanAhead(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetScanAhead)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DefaultFormat(&self) -> ::windows_core::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerFormat>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerFormat>(result__)
        }
    }
    pub fn Format(&self) -> ::windows_core::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerFormat>::zeroed();
            (::windows_core::Interface::vtable(this).Format)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerFormat>(result__)
        }
    }
    pub fn SetFormat(&self, value: ImageScannerFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsFormatSupported(&self, value: ImageScannerFormat) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsFormatSupported)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn MinScanArea(&self) -> ::windows_core::Result<::winrt_foundation::Size> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).MinScanArea)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Size>(result__)
        }
    }
    pub fn MaxScanArea(&self) -> ::windows_core::Result<::winrt_foundation::Size> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).MaxScanArea)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Size>(result__)
        }
    }
    pub fn SelectedScanRegion(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedScanRegion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    pub fn SetSelectedScanRegion<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedScanRegion)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn AutoCroppingMode(&self) -> ::windows_core::Result<ImageScannerAutoCroppingMode> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerAutoCroppingMode>::zeroed();
            (::windows_core::Interface::vtable(this).AutoCroppingMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerAutoCroppingMode>(result__)
        }
    }
    pub fn SetAutoCroppingMode(&self, value: ImageScannerAutoCroppingMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAutoCroppingMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAutoCroppingModeSupported(&self, value: ImageScannerAutoCroppingMode) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAutoCroppingModeSupported)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn MinResolution(&self) -> ::windows_core::Result<ImageScannerResolution> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerResolution>::zeroed();
            (::windows_core::Interface::vtable(this).MinResolution)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerResolution>(result__)
        }
    }
    pub fn MaxResolution(&self) -> ::windows_core::Result<ImageScannerResolution> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerResolution>::zeroed();
            (::windows_core::Interface::vtable(this).MaxResolution)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerResolution>(result__)
        }
    }
    pub fn OpticalResolution(&self) -> ::windows_core::Result<ImageScannerResolution> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerResolution>::zeroed();
            (::windows_core::Interface::vtable(this).OpticalResolution)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerResolution>(result__)
        }
    }
    pub fn DesiredResolution(&self) -> ::windows_core::Result<ImageScannerResolution> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerResolution>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredResolution)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerResolution>(result__)
        }
    }
    pub fn SetDesiredResolution<'a, Param0: ::windows_core::IntoParam<'a, ImageScannerResolution>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredResolution)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ActualResolution(&self) -> ::windows_core::Result<ImageScannerResolution> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerResolution>::zeroed();
            (::windows_core::Interface::vtable(this).ActualResolution)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerResolution>(result__)
        }
    }
    pub fn DefaultColorMode(&self) -> ::windows_core::Result<ImageScannerColorMode> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerColorMode>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultColorMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerColorMode>(result__)
        }
    }
    pub fn ColorMode(&self) -> ::windows_core::Result<ImageScannerColorMode> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerColorMode>::zeroed();
            (::windows_core::Interface::vtable(this).ColorMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerColorMode>(result__)
        }
    }
    pub fn SetColorMode(&self, value: ImageScannerColorMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetColorMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsColorModeSupported(&self, value: ImageScannerColorMode) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsColorModeSupported)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn MinBrightness(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).MinBrightness)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn MaxBrightness(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxBrightness)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn BrightnessStep(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).BrightnessStep)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn DefaultBrightness(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultBrightness)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Brightness(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Brightness)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetBrightness(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBrightness)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinContrast(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).MinContrast)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn MaxContrast(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxContrast)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn ContrastStep(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ContrastStep)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn DefaultContrast(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultContrast)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Contrast(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Contrast)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetContrast(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetContrast)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for ImageScannerFeederConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ImageScannerFeederConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ImageScannerFeederConfiguration {}
impl ::core::fmt::Debug for ImageScannerFeederConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageScannerFeederConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ImageScannerFeederConfiguration {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Scanners.ImageScannerFeederConfiguration;{ae275d11-dadf-4010-bf10-cca5c83dcbb0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ImageScannerFeederConfiguration {
    type Vtable = IImageScannerFormatConfiguration_Vtbl;
    const IID: ::windows_core::GUID = <IImageScannerFormatConfiguration as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ImageScannerFeederConfiguration {
    const NAME: &'static str = "Windows.Devices.Scanners.ImageScannerFeederConfiguration";
}
impl ::core::convert::From<ImageScannerFeederConfiguration> for ::windows_core::IUnknown {
    fn from(value: ImageScannerFeederConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageScannerFeederConfiguration> for ::windows_core::IUnknown {
    fn from(value: &ImageScannerFeederConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ImageScannerFeederConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ImageScannerFeederConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ImageScannerFeederConfiguration> for ::windows_core::IInspectable {
    fn from(value: ImageScannerFeederConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageScannerFeederConfiguration> for ::windows_core::IInspectable {
    fn from(value: &ImageScannerFeederConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ImageScannerFeederConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ImageScannerFeederConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ImageScannerFeederConfiguration> for IImageScannerFormatConfiguration {
    type Error = ::windows_core::Error;
    fn try_from(value: ImageScannerFeederConfiguration) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ImageScannerFeederConfiguration> for IImageScannerFormatConfiguration {
    type Error = ::windows_core::Error;
    fn try_from(value: &ImageScannerFeederConfiguration) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IImageScannerFormatConfiguration> for ImageScannerFeederConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, IImageScannerFormatConfiguration> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IImageScannerFormatConfiguration> for &ImageScannerFeederConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, IImageScannerFormatConfiguration> {
        ::core::convert::TryInto::<IImageScannerFormatConfiguration>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ImageScannerFeederConfiguration> for IImageScannerSourceConfiguration {
    type Error = ::windows_core::Error;
    fn try_from(value: ImageScannerFeederConfiguration) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ImageScannerFeederConfiguration> for IImageScannerSourceConfiguration {
    type Error = ::windows_core::Error;
    fn try_from(value: &ImageScannerFeederConfiguration) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IImageScannerSourceConfiguration> for ImageScannerFeederConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, IImageScannerSourceConfiguration> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IImageScannerSourceConfiguration> for &ImageScannerFeederConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, IImageScannerSourceConfiguration> {
        ::core::convert::TryInto::<IImageScannerSourceConfiguration>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ImageScannerFeederConfiguration {}
unsafe impl ::core::marker::Sync for ImageScannerFeederConfiguration {}
#[repr(transparent)]
pub struct ImageScannerFlatbedConfiguration(::windows_core::IUnknown);
impl ImageScannerFlatbedConfiguration {
    pub fn DefaultFormat(&self) -> ::windows_core::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerFormat>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerFormat>(result__)
        }
    }
    pub fn Format(&self) -> ::windows_core::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerFormat>::zeroed();
            (::windows_core::Interface::vtable(this).Format)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerFormat>(result__)
        }
    }
    pub fn SetFormat(&self, value: ImageScannerFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsFormatSupported(&self, value: ImageScannerFormat) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsFormatSupported)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn MinScanArea(&self) -> ::windows_core::Result<::winrt_foundation::Size> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).MinScanArea)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Size>(result__)
        }
    }
    pub fn MaxScanArea(&self) -> ::windows_core::Result<::winrt_foundation::Size> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).MaxScanArea)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Size>(result__)
        }
    }
    pub fn SelectedScanRegion(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedScanRegion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    pub fn SetSelectedScanRegion<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedScanRegion)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn AutoCroppingMode(&self) -> ::windows_core::Result<ImageScannerAutoCroppingMode> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerAutoCroppingMode>::zeroed();
            (::windows_core::Interface::vtable(this).AutoCroppingMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerAutoCroppingMode>(result__)
        }
    }
    pub fn SetAutoCroppingMode(&self, value: ImageScannerAutoCroppingMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAutoCroppingMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAutoCroppingModeSupported(&self, value: ImageScannerAutoCroppingMode) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAutoCroppingModeSupported)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn MinResolution(&self) -> ::windows_core::Result<ImageScannerResolution> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerResolution>::zeroed();
            (::windows_core::Interface::vtable(this).MinResolution)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerResolution>(result__)
        }
    }
    pub fn MaxResolution(&self) -> ::windows_core::Result<ImageScannerResolution> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerResolution>::zeroed();
            (::windows_core::Interface::vtable(this).MaxResolution)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerResolution>(result__)
        }
    }
    pub fn OpticalResolution(&self) -> ::windows_core::Result<ImageScannerResolution> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerResolution>::zeroed();
            (::windows_core::Interface::vtable(this).OpticalResolution)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerResolution>(result__)
        }
    }
    pub fn DesiredResolution(&self) -> ::windows_core::Result<ImageScannerResolution> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerResolution>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredResolution)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerResolution>(result__)
        }
    }
    pub fn SetDesiredResolution<'a, Param0: ::windows_core::IntoParam<'a, ImageScannerResolution>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredResolution)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ActualResolution(&self) -> ::windows_core::Result<ImageScannerResolution> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerResolution>::zeroed();
            (::windows_core::Interface::vtable(this).ActualResolution)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerResolution>(result__)
        }
    }
    pub fn DefaultColorMode(&self) -> ::windows_core::Result<ImageScannerColorMode> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerColorMode>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultColorMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerColorMode>(result__)
        }
    }
    pub fn ColorMode(&self) -> ::windows_core::Result<ImageScannerColorMode> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerColorMode>::zeroed();
            (::windows_core::Interface::vtable(this).ColorMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerColorMode>(result__)
        }
    }
    pub fn SetColorMode(&self, value: ImageScannerColorMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetColorMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsColorModeSupported(&self, value: ImageScannerColorMode) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsColorModeSupported)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn MinBrightness(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).MinBrightness)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn MaxBrightness(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxBrightness)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn BrightnessStep(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).BrightnessStep)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn DefaultBrightness(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultBrightness)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Brightness(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Brightness)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetBrightness(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBrightness)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinContrast(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).MinContrast)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn MaxContrast(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxContrast)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn ContrastStep(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ContrastStep)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn DefaultContrast(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultContrast)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Contrast(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Contrast)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetContrast(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetContrast)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for ImageScannerFlatbedConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ImageScannerFlatbedConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ImageScannerFlatbedConfiguration {}
impl ::core::fmt::Debug for ImageScannerFlatbedConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageScannerFlatbedConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ImageScannerFlatbedConfiguration {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Scanners.ImageScannerFlatbedConfiguration;{ae275d11-dadf-4010-bf10-cca5c83dcbb0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ImageScannerFlatbedConfiguration {
    type Vtable = IImageScannerFormatConfiguration_Vtbl;
    const IID: ::windows_core::GUID = <IImageScannerFormatConfiguration as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ImageScannerFlatbedConfiguration {
    const NAME: &'static str = "Windows.Devices.Scanners.ImageScannerFlatbedConfiguration";
}
impl ::core::convert::From<ImageScannerFlatbedConfiguration> for ::windows_core::IUnknown {
    fn from(value: ImageScannerFlatbedConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageScannerFlatbedConfiguration> for ::windows_core::IUnknown {
    fn from(value: &ImageScannerFlatbedConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ImageScannerFlatbedConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ImageScannerFlatbedConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ImageScannerFlatbedConfiguration> for ::windows_core::IInspectable {
    fn from(value: ImageScannerFlatbedConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageScannerFlatbedConfiguration> for ::windows_core::IInspectable {
    fn from(value: &ImageScannerFlatbedConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ImageScannerFlatbedConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ImageScannerFlatbedConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ImageScannerFlatbedConfiguration> for IImageScannerFormatConfiguration {
    type Error = ::windows_core::Error;
    fn try_from(value: ImageScannerFlatbedConfiguration) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ImageScannerFlatbedConfiguration> for IImageScannerFormatConfiguration {
    type Error = ::windows_core::Error;
    fn try_from(value: &ImageScannerFlatbedConfiguration) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IImageScannerFormatConfiguration> for ImageScannerFlatbedConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, IImageScannerFormatConfiguration> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IImageScannerFormatConfiguration> for &ImageScannerFlatbedConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, IImageScannerFormatConfiguration> {
        ::core::convert::TryInto::<IImageScannerFormatConfiguration>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ImageScannerFlatbedConfiguration> for IImageScannerSourceConfiguration {
    type Error = ::windows_core::Error;
    fn try_from(value: ImageScannerFlatbedConfiguration) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ImageScannerFlatbedConfiguration> for IImageScannerSourceConfiguration {
    type Error = ::windows_core::Error;
    fn try_from(value: &ImageScannerFlatbedConfiguration) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IImageScannerSourceConfiguration> for ImageScannerFlatbedConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, IImageScannerSourceConfiguration> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IImageScannerSourceConfiguration> for &ImageScannerFlatbedConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, IImageScannerSourceConfiguration> {
        ::core::convert::TryInto::<IImageScannerSourceConfiguration>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ImageScannerFlatbedConfiguration {}
unsafe impl ::core::marker::Sync for ImageScannerFlatbedConfiguration {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ImageScannerFormat(pub i32);
impl ImageScannerFormat {
    pub const Jpeg: Self = Self(0i32);
    pub const Png: Self = Self(1i32);
    pub const DeviceIndependentBitmap: Self = Self(2i32);
    pub const Tiff: Self = Self(3i32);
    pub const Xps: Self = Self(4i32);
    pub const OpenXps: Self = Self(5i32);
    pub const Pdf: Self = Self(6i32);
}
impl ::core::marker::Copy for ImageScannerFormat {}
impl ::core::clone::Clone for ImageScannerFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ImageScannerFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ImageScannerFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for ImageScannerFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageScannerFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ImageScannerFormat {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Scanners.ImageScannerFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ImageScannerPreviewResult(::windows_core::IUnknown);
impl ImageScannerPreviewResult {
    pub fn Succeeded(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Succeeded)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Format(&self) -> ::windows_core::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ImageScannerFormat>::zeroed();
            (::windows_core::Interface::vtable(this).Format)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageScannerFormat>(result__)
        }
    }
}
impl ::core::clone::Clone for ImageScannerPreviewResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ImageScannerPreviewResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ImageScannerPreviewResult {}
impl ::core::fmt::Debug for ImageScannerPreviewResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageScannerPreviewResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ImageScannerPreviewResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Scanners.ImageScannerPreviewResult;{08b7fe8e-8891-441d-be9c-176fa109c8bb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ImageScannerPreviewResult {
    type Vtable = IImageScannerPreviewResult_Vtbl;
    const IID: ::windows_core::GUID = <IImageScannerPreviewResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ImageScannerPreviewResult {
    const NAME: &'static str = "Windows.Devices.Scanners.ImageScannerPreviewResult";
}
impl ::core::convert::From<ImageScannerPreviewResult> for ::windows_core::IUnknown {
    fn from(value: ImageScannerPreviewResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageScannerPreviewResult> for ::windows_core::IUnknown {
    fn from(value: &ImageScannerPreviewResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ImageScannerPreviewResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ImageScannerPreviewResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ImageScannerPreviewResult> for ::windows_core::IInspectable {
    fn from(value: ImageScannerPreviewResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageScannerPreviewResult> for ::windows_core::IInspectable {
    fn from(value: &ImageScannerPreviewResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ImageScannerPreviewResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ImageScannerPreviewResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ImageScannerPreviewResult {}
unsafe impl ::core::marker::Sync for ImageScannerPreviewResult {}
#[repr(C)]
pub struct ImageScannerResolution {
    pub DpiX: f32,
    pub DpiY: f32,
}
impl ::core::marker::Copy for ImageScannerResolution {}
impl ::core::clone::Clone for ImageScannerResolution {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ImageScannerResolution {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ImageScannerResolution").field("DpiX", &self.DpiX).field("DpiY", &self.DpiY).finish()
    }
}
unsafe impl ::windows_core::Abi for ImageScannerResolution {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for ImageScannerResolution {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Devices.Scanners.ImageScannerResolution;f4;f4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for ImageScannerResolution {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ImageScannerResolution>()) == 0 }
    }
}
impl ::core::cmp::Eq for ImageScannerResolution {}
impl ::core::default::Default for ImageScannerResolution {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct ImageScannerScanResult(::windows_core::IUnknown);
impl ImageScannerScanResult {
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn ScannedFiles(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ScannedFiles)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_storage::StorageFile>>(result__)
        }
    }
}
impl ::core::clone::Clone for ImageScannerScanResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ImageScannerScanResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ImageScannerScanResult {}
impl ::core::fmt::Debug for ImageScannerScanResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageScannerScanResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ImageScannerScanResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Scanners.ImageScannerScanResult;{c91624cd-9037-4e48-84c1-ac0975076bc5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ImageScannerScanResult {
    type Vtable = IImageScannerScanResult_Vtbl;
    const IID: ::windows_core::GUID = <IImageScannerScanResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ImageScannerScanResult {
    const NAME: &'static str = "Windows.Devices.Scanners.ImageScannerScanResult";
}
impl ::core::convert::From<ImageScannerScanResult> for ::windows_core::IUnknown {
    fn from(value: ImageScannerScanResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageScannerScanResult> for ::windows_core::IUnknown {
    fn from(value: &ImageScannerScanResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ImageScannerScanResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ImageScannerScanResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ImageScannerScanResult> for ::windows_core::IInspectable {
    fn from(value: ImageScannerScanResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageScannerScanResult> for ::windows_core::IInspectable {
    fn from(value: &ImageScannerScanResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ImageScannerScanResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ImageScannerScanResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ImageScannerScanResult {}
unsafe impl ::core::marker::Sync for ImageScannerScanResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ImageScannerScanSource(pub i32);
impl ImageScannerScanSource {
    pub const Default: Self = Self(0i32);
    pub const Flatbed: Self = Self(1i32);
    pub const Feeder: Self = Self(2i32);
    pub const AutoConfigured: Self = Self(3i32);
}
impl ::core::marker::Copy for ImageScannerScanSource {}
impl ::core::clone::Clone for ImageScannerScanSource {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ImageScannerScanSource {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ImageScannerScanSource {
    type Abi = Self;
}
impl ::core::fmt::Debug for ImageScannerScanSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageScannerScanSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ImageScannerScanSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Scanners.ImageScannerScanSource;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
