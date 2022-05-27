pub const CLSID_DeviceIoControl: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x12d3e372_874b_457d_9fdf_73977778686c);
#[inline]
pub unsafe fn CreateDeviceAccessInstance<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(deviceinterfacepath: Param0, desiredaccess: u32) -> ::windows_core::Result<ICreateDeviceAccessAsync> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDeviceAccessInstance(deviceinterfacepath: ::windows_core::PCWSTR, desiredaccess: u32, createasync: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        CreateDeviceAccessInstance(deviceinterfacepath.into_param().abi(), ::core::mem::transmute(desiredaccess), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ICreateDeviceAccessAsync>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const DEV_PORT_1394: u32 = 8u32;
pub const DEV_PORT_ARTI: u32 = 7u32;
pub const DEV_PORT_COM1: u32 = 2u32;
pub const DEV_PORT_COM2: u32 = 3u32;
pub const DEV_PORT_COM3: u32 = 4u32;
pub const DEV_PORT_COM4: u32 = 5u32;
pub const DEV_PORT_DIAQ: u32 = 6u32;
pub const DEV_PORT_MAX: u32 = 9u32;
pub const DEV_PORT_MIN: u32 = 1u32;
pub const DEV_PORT_SIM: u32 = 1u32;
pub const DEV_PORT_USB: u32 = 9u32;
pub const ED_AUDIO_1: i32 = 1i32;
pub const ED_AUDIO_10: i32 = 512i32;
pub const ED_AUDIO_11: i32 = 1024i32;
pub const ED_AUDIO_12: i32 = 2048i32;
pub const ED_AUDIO_13: i32 = 4096i32;
pub const ED_AUDIO_14: i32 = 8192i32;
pub const ED_AUDIO_15: i32 = 16384i32;
pub const ED_AUDIO_16: i32 = 32768i32;
pub const ED_AUDIO_17: i32 = 65536i32;
pub const ED_AUDIO_18: i32 = 131072i32;
pub const ED_AUDIO_19: i32 = 262144i32;
pub const ED_AUDIO_2: i32 = 2i32;
pub const ED_AUDIO_20: i32 = 524288i32;
pub const ED_AUDIO_21: i32 = 1048576i32;
pub const ED_AUDIO_22: i32 = 2097152i32;
pub const ED_AUDIO_23: i32 = 4194304i32;
pub const ED_AUDIO_24: i32 = 8388608i32;
pub const ED_AUDIO_3: i32 = 4i32;
pub const ED_AUDIO_4: i32 = 8i32;
pub const ED_AUDIO_5: i32 = 16i32;
pub const ED_AUDIO_6: i32 = 32i32;
pub const ED_AUDIO_7: i32 = 64i32;
pub const ED_AUDIO_8: i32 = 128i32;
pub const ED_AUDIO_9: i32 = 256i32;
pub const ED_AUDIO_ALL: u32 = 268435456u32;
pub const ED_BASE: i32 = 4096i32;
pub const ED_BOTTOM: u32 = 4u32;
pub const ED_CENTER: u32 = 512u32;
pub const ED_LEFT: u32 = 256u32;
pub const ED_MIDDLE: u32 = 2u32;
pub const ED_RIGHT: u32 = 1024u32;
pub const ED_TOP: u32 = 1u32;
pub const ED_VIDEO: i32 = 33554432i32;
#[repr(transparent)]
pub struct ICreateDeviceAccessAsync(::windows_core::IUnknown);
impl ICreateDeviceAccessAsync {
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Wait(&self, timeout: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Wait)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(timeout)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetResult<T: ::windows_core::Interface>(&self) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetResult)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<ICreateDeviceAccessAsync> for ::windows_core::IUnknown {
    fn from(value: ICreateDeviceAccessAsync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICreateDeviceAccessAsync> for ::windows_core::IUnknown {
    fn from(value: &ICreateDeviceAccessAsync) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICreateDeviceAccessAsync {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICreateDeviceAccessAsync {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICreateDeviceAccessAsync {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICreateDeviceAccessAsync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICreateDeviceAccessAsync {}
impl ::core::fmt::Debug for ICreateDeviceAccessAsync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICreateDeviceAccessAsync").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICreateDeviceAccessAsync {
    type Vtable = ICreateDeviceAccessAsync_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3474628f_683d_42d2_abcb_db018c6503bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateDeviceAccessAsync_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Wait: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeout: u32) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, deviceaccess: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDeviceIoControl(::windows_core::IUnknown);
impl IDeviceIoControl {
    pub unsafe fn DeviceIoControlSync(&self, iocontrolcode: u32, inputbuffer: &[u8], outputbuffer: &mut [u8], bytesreturned: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeviceIoControlSync)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iocontrolcode), ::core::mem::transmute(::windows_core::as_ptr_or_null(inputbuffer)), inputbuffer.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(outputbuffer)), outputbuffer.len() as _, ::core::mem::transmute(bytesreturned)).ok()
    }
    pub unsafe fn DeviceIoControlAsync<'a, Param5: ::windows_core::IntoParam<'a, IDeviceRequestCompletionCallback>>(&self, iocontrolcode: u32, inputbuffer: &[u8], outputbuffer: &mut [u8], requestcompletioncallback: Param5, cancelcontext: *mut usize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeviceIoControlAsync)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iocontrolcode), ::core::mem::transmute(::windows_core::as_ptr_or_null(inputbuffer)), inputbuffer.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(outputbuffer)), outputbuffer.len() as _, requestcompletioncallback.into_param().abi(), ::core::mem::transmute(cancelcontext)).ok()
    }
    pub unsafe fn CancelOperation(&self, cancelcontext: usize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CancelOperation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cancelcontext)).ok()
    }
}
impl ::core::convert::From<IDeviceIoControl> for ::windows_core::IUnknown {
    fn from(value: IDeviceIoControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDeviceIoControl> for ::windows_core::IUnknown {
    fn from(value: &IDeviceIoControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDeviceIoControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDeviceIoControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDeviceIoControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDeviceIoControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDeviceIoControl {}
impl ::core::fmt::Debug for IDeviceIoControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDeviceIoControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDeviceIoControl {
    type Vtable = IDeviceIoControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9eefe161_23ab_4f18_9b49_991b586ae970);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceIoControl_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub DeviceIoControlSync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, bytesreturned: *mut u32) -> ::windows_core::HRESULT,
    pub DeviceIoControlAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, requestcompletioncallback: ::windows_core::RawPtr, cancelcontext: *mut usize) -> ::windows_core::HRESULT,
    pub CancelOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cancelcontext: usize) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDeviceRequestCompletionCallback(::windows_core::IUnknown);
impl IDeviceRequestCompletionCallback {
    pub unsafe fn Invoke(&self, requestresult: ::windows_core::HRESULT, bytesreturned: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Invoke)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(requestresult), ::core::mem::transmute(bytesreturned)).ok()
    }
}
impl ::core::convert::From<IDeviceRequestCompletionCallback> for ::windows_core::IUnknown {
    fn from(value: IDeviceRequestCompletionCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDeviceRequestCompletionCallback> for ::windows_core::IUnknown {
    fn from(value: &IDeviceRequestCompletionCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDeviceRequestCompletionCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDeviceRequestCompletionCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDeviceRequestCompletionCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDeviceRequestCompletionCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDeviceRequestCompletionCallback {}
impl ::core::fmt::Debug for IDeviceRequestCompletionCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDeviceRequestCompletionCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDeviceRequestCompletionCallback {
    type Vtable = IDeviceRequestCompletionCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x999bad24_9acd_45bb_8669_2a2fc0288b04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceRequestCompletionCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestresult: ::windows_core::HRESULT, bytesreturned: u32) -> ::windows_core::HRESULT,
}
