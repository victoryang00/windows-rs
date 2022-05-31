pub const CLSID_AudioFrameNativeFactory: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x16a0a3b9_9f65_4102_9367_2cda3a4f372a);
pub const CLSID_VideoFrameNativeFactory: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd194386a_04e3_4814_8100_b2b0ae6d78c7);
#[repr(transparent)]
pub struct IAudioFrameNative(::windows_core::IUnknown);
impl IAudioFrameNative {
    pub unsafe fn GetData<T: ::windows_core::Interface>(&self) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetData)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IAudioFrameNative> for ::windows_core::IUnknown {
    fn from(value: IAudioFrameNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioFrameNative> for ::windows_core::IUnknown {
    fn from(value: &IAudioFrameNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAudioFrameNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAudioFrameNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAudioFrameNative> for ::windows_core::IInspectable {
    fn from(value: IAudioFrameNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioFrameNative> for ::windows_core::IInspectable {
    fn from(value: &IAudioFrameNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IAudioFrameNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IAudioFrameNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioFrameNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioFrameNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioFrameNative {}
impl ::core::fmt::Debug for IAudioFrameNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioFrameNative").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAudioFrameNative {
    type Vtable = IAudioFrameNative_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x20be1e2e_930f_4746_9335_3c332f255093);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameNative_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAudioFrameNativeFactory(::windows_core::IUnknown);
impl IAudioFrameNativeFactory {
    #[cfg(feature = "win32-media")]
    pub unsafe fn CreateFromMFSample<'a, Param0: ::windows_core::IntoParam<'a, ::win32_media::MediaFoundation::IMFSample>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, T: ::windows_core::Interface>(&self, data: Param0, forcereadonly: Param1) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).CreateFromMFSample)(::windows_core::Interface::as_raw(self), data.into_param().abi(), forcereadonly.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IAudioFrameNativeFactory> for ::windows_core::IUnknown {
    fn from(value: IAudioFrameNativeFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioFrameNativeFactory> for ::windows_core::IUnknown {
    fn from(value: &IAudioFrameNativeFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAudioFrameNativeFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAudioFrameNativeFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAudioFrameNativeFactory> for ::windows_core::IInspectable {
    fn from(value: IAudioFrameNativeFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioFrameNativeFactory> for ::windows_core::IInspectable {
    fn from(value: &IAudioFrameNativeFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IAudioFrameNativeFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IAudioFrameNativeFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioFrameNativeFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioFrameNativeFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioFrameNativeFactory {}
impl ::core::fmt::Debug for IAudioFrameNativeFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioFrameNativeFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAudioFrameNativeFactory {
    type Vtable = IAudioFrameNativeFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7bd67cf8_bf7d_43e6_af8d_b170ee0c0110);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameNativeFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "win32-media")]
    pub CreateFromMFSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::windows_core::RawPtr, forcereadonly: ::win32_foundation::BOOL, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-media"))]
    CreateFromMFSample: usize,
}
#[repr(transparent)]
pub struct IVideoFrameNative(::windows_core::IUnknown);
impl IVideoFrameNative {
    pub unsafe fn GetData<T: ::windows_core::Interface>(&self) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetData)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn GetDevice<T: ::windows_core::Interface>(&self) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetDevice)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IVideoFrameNative> for ::windows_core::IUnknown {
    fn from(value: IVideoFrameNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVideoFrameNative> for ::windows_core::IUnknown {
    fn from(value: &IVideoFrameNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IVideoFrameNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IVideoFrameNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IVideoFrameNative> for ::windows_core::IInspectable {
    fn from(value: IVideoFrameNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVideoFrameNative> for ::windows_core::IInspectable {
    fn from(value: &IVideoFrameNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IVideoFrameNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IVideoFrameNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVideoFrameNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVideoFrameNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVideoFrameNative {}
impl ::core::fmt::Debug for IVideoFrameNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVideoFrameNative").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IVideoFrameNative {
    type Vtable = IVideoFrameNative_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x26ba702b_314a_4620_aaf6_7a51aa58fa18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoFrameNative_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IVideoFrameNativeFactory(::windows_core::IUnknown);
impl IVideoFrameNativeFactory {
    #[cfg(feature = "win32-media")]
    pub unsafe fn CreateFromMFSample<'a, Param0: ::windows_core::IntoParam<'a, ::win32_media::MediaFoundation::IMFSample>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param6: ::windows_core::IntoParam<'a, ::win32_media::MediaFoundation::IMFDXGIDeviceManager>, T: ::windows_core::Interface>(&self, data: Param0, subtype: *const ::windows_core::GUID, width: u32, height: u32, forcereadonly: Param4, mindisplayaperture: *const ::win32_media::MediaFoundation::MFVideoArea, device: Param6) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).CreateFromMFSample)(::windows_core::Interface::as_raw(self), data.into_param().abi(), ::core::mem::transmute(subtype), ::core::mem::transmute(width), ::core::mem::transmute(height), forcereadonly.into_param().abi(), ::core::mem::transmute(mindisplayaperture), device.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IVideoFrameNativeFactory> for ::windows_core::IUnknown {
    fn from(value: IVideoFrameNativeFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVideoFrameNativeFactory> for ::windows_core::IUnknown {
    fn from(value: &IVideoFrameNativeFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IVideoFrameNativeFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IVideoFrameNativeFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IVideoFrameNativeFactory> for ::windows_core::IInspectable {
    fn from(value: IVideoFrameNativeFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVideoFrameNativeFactory> for ::windows_core::IInspectable {
    fn from(value: &IVideoFrameNativeFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IVideoFrameNativeFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IVideoFrameNativeFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVideoFrameNativeFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVideoFrameNativeFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVideoFrameNativeFactory {}
impl ::core::fmt::Debug for IVideoFrameNativeFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVideoFrameNativeFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IVideoFrameNativeFactory {
    type Vtable = IVideoFrameNativeFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x69e3693e_8e1e_4e63_ac4c_7fdc21d9731d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoFrameNativeFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "win32-media")]
    pub CreateFromMFSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::windows_core::RawPtr, subtype: *const ::windows_core::GUID, width: u32, height: u32, forcereadonly: ::win32_foundation::BOOL, mindisplayaperture: *const ::win32_media::MediaFoundation::MFVideoArea, device: ::windows_core::RawPtr, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-media"))]
    CreateFromMFSample: usize,
}
