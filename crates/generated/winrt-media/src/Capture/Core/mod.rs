#[doc(hidden)]
#[repr(transparent)]
pub struct IVariablePhotoCapturedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVariablePhotoCapturedEventArgs {
    type Vtable = IVariablePhotoCapturedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd1eb4c5c_1b53_4e4a_8b5c_db7887ac949b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVariablePhotoCapturedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Frame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CaptureTimeOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub UsedFrameControllerIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CapturedFrameControlValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVariablePhotoSequenceCapture(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVariablePhotoSequenceCapture {
    type Vtable = IVariablePhotoSequenceCapture_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd0112d1d_031e_4041_a6d6_bd742476a8ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVariablePhotoSequenceCapture_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StopAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FinishAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PhotoCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePhotoCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVariablePhotoSequenceCapture2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVariablePhotoSequenceCapture2 {
    type Vtable = IVariablePhotoSequenceCapture2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfe2c62bc_50b0_43e3_917c_e3b92798942f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVariablePhotoSequenceCapture2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub UpdateSettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct VariablePhotoCapturedEventArgs(::windows_core::IUnknown);
impl VariablePhotoCapturedEventArgs {
    pub fn Frame(&self) -> ::windows_core::Result<super::CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Frame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::CapturedFrame>(result__)
        }
    }
    pub fn CaptureTimeOffset(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).CaptureTimeOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn UsedFrameControllerIndex(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UsedFrameControllerIndex)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u32>>(result__)
        }
    }
    pub fn CapturedFrameControlValues(&self) -> ::windows_core::Result<super::CapturedFrameControlValues> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CapturedFrameControlValues)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::CapturedFrameControlValues>(result__)
        }
    }
}
impl ::core::clone::Clone for VariablePhotoCapturedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VariablePhotoCapturedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VariablePhotoCapturedEventArgs {}
impl ::core::fmt::Debug for VariablePhotoCapturedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VariablePhotoCapturedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VariablePhotoCapturedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Core.VariablePhotoCapturedEventArgs;{d1eb4c5c-1b53-4e4a-8b5c-db7887ac949b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for VariablePhotoCapturedEventArgs {
    type Vtable = IVariablePhotoCapturedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IVariablePhotoCapturedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VariablePhotoCapturedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.Core.VariablePhotoCapturedEventArgs";
}
impl ::core::convert::From<VariablePhotoCapturedEventArgs> for ::windows_core::IUnknown {
    fn from(value: VariablePhotoCapturedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VariablePhotoCapturedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &VariablePhotoCapturedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for VariablePhotoCapturedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a VariablePhotoCapturedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VariablePhotoCapturedEventArgs> for ::windows_core::IInspectable {
    fn from(value: VariablePhotoCapturedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VariablePhotoCapturedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &VariablePhotoCapturedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for VariablePhotoCapturedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a VariablePhotoCapturedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VariablePhotoCapturedEventArgs {}
unsafe impl ::core::marker::Sync for VariablePhotoCapturedEventArgs {}
#[repr(transparent)]
pub struct VariablePhotoSequenceCapture(::windows_core::IUnknown);
impl VariablePhotoSequenceCapture {
    pub fn StartAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn StopAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StopAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn FinishAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FinishAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn PhotoCaptured<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<VariablePhotoSequenceCapture, VariablePhotoCapturedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PhotoCaptured)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePhotoCaptured<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePhotoCaptured)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Stopped<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<VariablePhotoSequenceCapture, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Stopped)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStopped<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStopped)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn UpdateSettingsAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IVariablePhotoSequenceCapture2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateSettingsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for VariablePhotoSequenceCapture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VariablePhotoSequenceCapture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VariablePhotoSequenceCapture {}
impl ::core::fmt::Debug for VariablePhotoSequenceCapture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VariablePhotoSequenceCapture").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VariablePhotoSequenceCapture {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Core.VariablePhotoSequenceCapture;{d0112d1d-031e-4041-a6d6-bd742476a8ee})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for VariablePhotoSequenceCapture {
    type Vtable = IVariablePhotoSequenceCapture_Vtbl;
    const IID: ::windows_core::GUID = <IVariablePhotoSequenceCapture as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VariablePhotoSequenceCapture {
    const NAME: &'static str = "Windows.Media.Capture.Core.VariablePhotoSequenceCapture";
}
impl ::core::convert::From<VariablePhotoSequenceCapture> for ::windows_core::IUnknown {
    fn from(value: VariablePhotoSequenceCapture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VariablePhotoSequenceCapture> for ::windows_core::IUnknown {
    fn from(value: &VariablePhotoSequenceCapture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for VariablePhotoSequenceCapture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a VariablePhotoSequenceCapture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VariablePhotoSequenceCapture> for ::windows_core::IInspectable {
    fn from(value: VariablePhotoSequenceCapture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VariablePhotoSequenceCapture> for ::windows_core::IInspectable {
    fn from(value: &VariablePhotoSequenceCapture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for VariablePhotoSequenceCapture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a VariablePhotoSequenceCapture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
