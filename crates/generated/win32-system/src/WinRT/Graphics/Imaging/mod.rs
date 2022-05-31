pub const CLSID_SoftwareBitmapNativeFactory: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x84e65691_8602_4a84_be46_708be9cd4b74);
#[repr(transparent)]
pub struct ISoftwareBitmapNative(::windows_core::IUnknown);
impl ISoftwareBitmapNative {
    pub unsafe fn GetData<T: ::windows_core::Interface>(&self) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetData)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<ISoftwareBitmapNative> for ::windows_core::IUnknown {
    fn from(value: ISoftwareBitmapNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISoftwareBitmapNative> for ::windows_core::IUnknown {
    fn from(value: &ISoftwareBitmapNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISoftwareBitmapNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISoftwareBitmapNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISoftwareBitmapNative> for ::windows_core::IInspectable {
    fn from(value: ISoftwareBitmapNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISoftwareBitmapNative> for ::windows_core::IInspectable {
    fn from(value: &ISoftwareBitmapNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ISoftwareBitmapNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ISoftwareBitmapNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISoftwareBitmapNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISoftwareBitmapNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISoftwareBitmapNative {}
impl ::core::fmt::Debug for ISoftwareBitmapNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISoftwareBitmapNative").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISoftwareBitmapNative {
    type Vtable = ISoftwareBitmapNative_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x94bc8415_04ea_4b2e_af13_4de95aa898eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapNative_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISoftwareBitmapNativeFactory(::windows_core::IUnknown);
impl ISoftwareBitmapNativeFactory {
    #[cfg(feature = "win32-graphics")]
    pub unsafe fn CreateFromWICBitmap<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Imaging::IWICBitmap>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, T: ::windows_core::Interface>(&self, data: Param0, forcereadonly: Param1) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).CreateFromWICBitmap)(::windows_core::Interface::as_raw(self), data.into_param().abi(), forcereadonly.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "win32-media")]
    pub unsafe fn CreateFromMF2DBuffer2<'a, Param0: ::windows_core::IntoParam<'a, ::win32_media::MediaFoundation::IMF2DBuffer2>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, T: ::windows_core::Interface>(&self, data: Param0, subtype: *const ::windows_core::GUID, width: u32, height: u32, forcereadonly: Param4, mindisplayaperture: *const ::win32_media::MediaFoundation::MFVideoArea) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).CreateFromMF2DBuffer2)(::windows_core::Interface::as_raw(self), data.into_param().abi(), ::core::mem::transmute(subtype), ::core::mem::transmute(width), ::core::mem::transmute(height), forcereadonly.into_param().abi(), ::core::mem::transmute(mindisplayaperture), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<ISoftwareBitmapNativeFactory> for ::windows_core::IUnknown {
    fn from(value: ISoftwareBitmapNativeFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISoftwareBitmapNativeFactory> for ::windows_core::IUnknown {
    fn from(value: &ISoftwareBitmapNativeFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISoftwareBitmapNativeFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISoftwareBitmapNativeFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISoftwareBitmapNativeFactory> for ::windows_core::IInspectable {
    fn from(value: ISoftwareBitmapNativeFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISoftwareBitmapNativeFactory> for ::windows_core::IInspectable {
    fn from(value: &ISoftwareBitmapNativeFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ISoftwareBitmapNativeFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ISoftwareBitmapNativeFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISoftwareBitmapNativeFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISoftwareBitmapNativeFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISoftwareBitmapNativeFactory {}
impl ::core::fmt::Debug for ISoftwareBitmapNativeFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISoftwareBitmapNativeFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISoftwareBitmapNativeFactory {
    type Vtable = ISoftwareBitmapNativeFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc3c181ec_2914_4791_af02_02d224a10b43);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapNativeFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "win32-graphics")]
    pub CreateFromWICBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::windows_core::RawPtr, forcereadonly: ::win32_foundation::BOOL, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-graphics"))]
    CreateFromWICBitmap: usize,
    #[cfg(feature = "win32-media")]
    pub CreateFromMF2DBuffer2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::windows_core::RawPtr, subtype: *const ::windows_core::GUID, width: u32, height: u32, forcereadonly: ::win32_foundation::BOOL, mindisplayaperture: *const ::win32_media::MediaFoundation::MFVideoArea, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-media"))]
    CreateFromMF2DBuffer2: usize,
}
