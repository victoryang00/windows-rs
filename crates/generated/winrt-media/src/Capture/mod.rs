#[cfg(feature = "Core")]
pub mod Core;
#[cfg(feature = "Frames")]
pub mod Frames;
#[repr(transparent)]
pub struct AdvancedCapturedPhoto(::windows_core::IUnknown);
impl AdvancedCapturedPhoto {
    pub fn Frame(&self) -> ::windows_core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Frame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CapturedFrame>(result__)
        }
    }
    #[cfg(feature = "Media_Devices")]
    pub fn Mode(&self) -> ::windows_core::Result<super::Devices::AdvancedPhotoMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Devices::AdvancedPhotoMode>::zeroed();
            (::windows_core::Interface::vtable(this).Mode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Devices::AdvancedPhotoMode>(result__)
        }
    }
    pub fn Context(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Context)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn FrameBoundsRelativeToReferencePhoto(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::Rect>> {
        let this = &::windows_core::Interface::cast::<IAdvancedCapturedPhoto2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FrameBoundsRelativeToReferencePhoto)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::Rect>>(result__)
        }
    }
}
impl ::core::clone::Clone for AdvancedCapturedPhoto {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdvancedCapturedPhoto {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdvancedCapturedPhoto {}
impl ::core::fmt::Debug for AdvancedCapturedPhoto {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdvancedCapturedPhoto").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AdvancedCapturedPhoto {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AdvancedCapturedPhoto;{f072728b-b292-4491-9d41-99807a550bbf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AdvancedCapturedPhoto {
    type Vtable = IAdvancedCapturedPhoto_Vtbl;
    const IID: ::windows_core::GUID = <IAdvancedCapturedPhoto as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AdvancedCapturedPhoto {
    const NAME: &'static str = "Windows.Media.Capture.AdvancedCapturedPhoto";
}
impl ::core::convert::From<AdvancedCapturedPhoto> for ::windows_core::IUnknown {
    fn from(value: AdvancedCapturedPhoto) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdvancedCapturedPhoto> for ::windows_core::IUnknown {
    fn from(value: &AdvancedCapturedPhoto) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AdvancedCapturedPhoto {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AdvancedCapturedPhoto {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdvancedCapturedPhoto> for ::windows_core::IInspectable {
    fn from(value: AdvancedCapturedPhoto) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdvancedCapturedPhoto> for ::windows_core::IInspectable {
    fn from(value: &AdvancedCapturedPhoto) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AdvancedCapturedPhoto {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AdvancedCapturedPhoto {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdvancedCapturedPhoto {}
unsafe impl ::core::marker::Sync for AdvancedCapturedPhoto {}
#[repr(transparent)]
pub struct AdvancedPhotoCapture(::windows_core::IUnknown);
impl AdvancedPhotoCapture {
    pub fn CaptureAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AdvancedCapturedPhoto>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CaptureAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AdvancedCapturedPhoto>>(result__)
        }
    }
    pub fn CaptureWithContextAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, context: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AdvancedCapturedPhoto>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CaptureWithContextAsync)(::windows_core::Interface::as_raw(this), context.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AdvancedCapturedPhoto>>(result__)
        }
    }
    pub fn OptionalReferencePhotoCaptured<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AdvancedPhotoCapture, OptionalReferencePhotoCapturedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).OptionalReferencePhotoCaptured)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveOptionalReferencePhotoCaptured<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOptionalReferencePhotoCaptured)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn AllPhotosCaptured<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AdvancedPhotoCapture, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AllPhotosCaptured)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAllPhotosCaptured<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAllPhotosCaptured)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn FinishAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FinishAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for AdvancedPhotoCapture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdvancedPhotoCapture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdvancedPhotoCapture {}
impl ::core::fmt::Debug for AdvancedPhotoCapture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdvancedPhotoCapture").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AdvancedPhotoCapture {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AdvancedPhotoCapture;{83ffaafa-6667-44dc-973c-a6bce596aa0f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AdvancedPhotoCapture {
    type Vtable = IAdvancedPhotoCapture_Vtbl;
    const IID: ::windows_core::GUID = <IAdvancedPhotoCapture as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AdvancedPhotoCapture {
    const NAME: &'static str = "Windows.Media.Capture.AdvancedPhotoCapture";
}
impl ::core::convert::From<AdvancedPhotoCapture> for ::windows_core::IUnknown {
    fn from(value: AdvancedPhotoCapture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdvancedPhotoCapture> for ::windows_core::IUnknown {
    fn from(value: &AdvancedPhotoCapture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AdvancedPhotoCapture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AdvancedPhotoCapture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdvancedPhotoCapture> for ::windows_core::IInspectable {
    fn from(value: AdvancedPhotoCapture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdvancedPhotoCapture> for ::windows_core::IInspectable {
    fn from(value: &AdvancedPhotoCapture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AdvancedPhotoCapture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AdvancedPhotoCapture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdvancedPhotoCapture {}
unsafe impl ::core::marker::Sync for AdvancedPhotoCapture {}
#[repr(transparent)]
pub struct AppBroadcastBackgroundService(::windows_core::IUnknown);
impl AppBroadcastBackgroundService {
    pub fn SetPlugInState(&self, value: AppBroadcastPlugInState) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPlugInState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PlugInState(&self) -> ::windows_core::Result<AppBroadcastPlugInState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppBroadcastPlugInState>::zeroed();
            (::windows_core::Interface::vtable(this).PlugInState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastPlugInState>(result__)
        }
    }
    pub fn SetSignInInfo<'a, Param0: ::windows_core::IntoParam<'a, AppBroadcastBackgroundServiceSignInInfo>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSignInInfo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SignInInfo(&self) -> ::windows_core::Result<AppBroadcastBackgroundServiceSignInInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SignInInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastBackgroundServiceSignInInfo>(result__)
        }
    }
    pub fn SetStreamInfo<'a, Param0: ::windows_core::IntoParam<'a, AppBroadcastBackgroundServiceStreamInfo>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStreamInfo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StreamInfo(&self) -> ::windows_core::Result<AppBroadcastBackgroundServiceStreamInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StreamInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastBackgroundServiceStreamInfo>(result__)
        }
    }
    pub fn AppId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn BroadcastTitle(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).BroadcastTitle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetViewerCount(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetViewerCount)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ViewerCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ViewerCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn TerminateBroadcast(&self, reason: AppBroadcastTerminationReason, providerspecificreason: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).TerminateBroadcast)(::windows_core::Interface::as_raw(this), reason, providerspecificreason).ok() }
    }
    pub fn HeartbeatRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppBroadcastBackgroundService, AppBroadcastHeartbeatRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).HeartbeatRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveHeartbeatRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHeartbeatRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn TitleId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TitleId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetBroadcastTitle<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBroadcastTitle)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BroadcastLanguage(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).BroadcastLanguage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetBroadcastLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBroadcastLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BroadcastChannel(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).BroadcastChannel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetBroadcastChannel<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBroadcastChannel)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BroadcastTitleChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).BroadcastTitleChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveBroadcastTitleChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveBroadcastTitleChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn BroadcastLanguageChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).BroadcastLanguageChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveBroadcastLanguageChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveBroadcastLanguageChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn BroadcastChannelChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).BroadcastChannelChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveBroadcastChannelChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveBroadcastChannelChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AppBroadcastBackgroundService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastBackgroundService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastBackgroundService {}
impl ::core::fmt::Debug for AppBroadcastBackgroundService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastBackgroundService").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastBackgroundService {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastBackgroundService;{bad1e72a-fa94-46f9-95fc-d71511cda70b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastBackgroundService {
    type Vtable = IAppBroadcastBackgroundService_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastBackgroundService as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastBackgroundService {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastBackgroundService";
}
impl ::core::convert::From<AppBroadcastBackgroundService> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastBackgroundService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastBackgroundService> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastBackgroundService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastBackgroundService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastBackgroundService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastBackgroundService> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastBackgroundService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastBackgroundService> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastBackgroundService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastBackgroundService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastBackgroundService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct AppBroadcastBackgroundServiceSignInInfo(::windows_core::IUnknown);
impl AppBroadcastBackgroundServiceSignInInfo {
    pub fn SignInState(&self) -> ::windows_core::Result<AppBroadcastSignInState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppBroadcastSignInState>::zeroed();
            (::windows_core::Interface::vtable(this).SignInState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastSignInState>(result__)
        }
    }
    pub fn SetOAuthRequestUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOAuthRequestUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn OAuthRequestUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OAuthRequestUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetOAuthCallbackUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOAuthCallbackUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn OAuthCallbackUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OAuthCallbackUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Security_Authentication_Web")]
    pub fn AuthenticationResult(&self) -> ::windows_core::Result<::winrt_security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticationResult)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_security::Authentication::Web::WebAuthenticationResult>(result__)
        }
    }
    pub fn SetUserName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUserName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn UserName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).UserName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SignInStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppBroadcastBackgroundServiceSignInInfo, AppBroadcastSignInStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SignInStateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSignInStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSignInStateChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn UserNameChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppBroadcastBackgroundServiceSignInInfo, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IAppBroadcastBackgroundServiceSignInInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).UserNameChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveUserNameChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppBroadcastBackgroundServiceSignInInfo2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUserNameChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AppBroadcastBackgroundServiceSignInInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastBackgroundServiceSignInInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastBackgroundServiceSignInInfo {}
impl ::core::fmt::Debug for AppBroadcastBackgroundServiceSignInInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastBackgroundServiceSignInInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastBackgroundServiceSignInInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastBackgroundServiceSignInInfo;{5e735275-88c8-4eca-89ba-4825985db880})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastBackgroundServiceSignInInfo {
    type Vtable = IAppBroadcastBackgroundServiceSignInInfo_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastBackgroundServiceSignInInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastBackgroundServiceSignInInfo {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastBackgroundServiceSignInInfo";
}
impl ::core::convert::From<AppBroadcastBackgroundServiceSignInInfo> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastBackgroundServiceSignInInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastBackgroundServiceSignInInfo> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastBackgroundServiceSignInInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastBackgroundServiceSignInInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastBackgroundServiceSignInInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastBackgroundServiceSignInInfo> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastBackgroundServiceSignInInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastBackgroundServiceSignInInfo> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastBackgroundServiceSignInInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastBackgroundServiceSignInInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastBackgroundServiceSignInInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct AppBroadcastBackgroundServiceStreamInfo(::windows_core::IUnknown);
impl AppBroadcastBackgroundServiceStreamInfo {
    pub fn StreamState(&self) -> ::windows_core::Result<AppBroadcastStreamState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppBroadcastStreamState>::zeroed();
            (::windows_core::Interface::vtable(this).StreamState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastStreamState>(result__)
        }
    }
    pub fn SetDesiredVideoEncodingBitrate(&self, value: u64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredVideoEncodingBitrate)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DesiredVideoEncodingBitrate(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredVideoEncodingBitrate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SetBandwidthTestBitrate(&self, value: u64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBandwidthTestBitrate)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BandwidthTestBitrate(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).BandwidthTestBitrate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SetAudioCodec<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAudioCodec)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn AudioCodec(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AudioCodec)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn BroadcastStreamReader(&self) -> ::windows_core::Result<AppBroadcastStreamReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BroadcastStreamReader)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastStreamReader>(result__)
        }
    }
    pub fn StreamStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, AppBroadcastStreamStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).StreamStateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStreamStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStreamStateChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn VideoEncodingResolutionChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).VideoEncodingResolutionChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveVideoEncodingResolutionChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveVideoEncodingResolutionChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn VideoEncodingBitrateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).VideoEncodingBitrateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveVideoEncodingBitrateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveVideoEncodingBitrateChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ReportProblemWithStream(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppBroadcastBackgroundServiceStreamInfo2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReportProblemWithStream)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for AppBroadcastBackgroundServiceStreamInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastBackgroundServiceStreamInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastBackgroundServiceStreamInfo {}
impl ::core::fmt::Debug for AppBroadcastBackgroundServiceStreamInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastBackgroundServiceStreamInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastBackgroundServiceStreamInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastBackgroundServiceStreamInfo;{31dc02bc-990a-4904-aa96-fe364381f136})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastBackgroundServiceStreamInfo {
    type Vtable = IAppBroadcastBackgroundServiceStreamInfo_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastBackgroundServiceStreamInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastBackgroundServiceStreamInfo {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastBackgroundServiceStreamInfo";
}
impl ::core::convert::From<AppBroadcastBackgroundServiceStreamInfo> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastBackgroundServiceStreamInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastBackgroundServiceStreamInfo> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastBackgroundServiceStreamInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastBackgroundServiceStreamInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastBackgroundServiceStreamInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastBackgroundServiceStreamInfo> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastBackgroundServiceStreamInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastBackgroundServiceStreamInfo> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastBackgroundServiceStreamInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastBackgroundServiceStreamInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastBackgroundServiceStreamInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppBroadcastCameraCaptureState(pub i32);
impl AppBroadcastCameraCaptureState {
    pub const Stopped: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for AppBroadcastCameraCaptureState {}
impl ::core::clone::Clone for AppBroadcastCameraCaptureState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppBroadcastCameraCaptureState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppBroadcastCameraCaptureState {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppBroadcastCameraCaptureState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastCameraCaptureState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastCameraCaptureState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastCameraCaptureState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppBroadcastCameraCaptureStateChangedEventArgs(::windows_core::IUnknown);
impl AppBroadcastCameraCaptureStateChangedEventArgs {
    pub fn State(&self) -> ::windows_core::Result<AppBroadcastCameraCaptureState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppBroadcastCameraCaptureState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastCameraCaptureState>(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastCameraCaptureStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastCameraCaptureStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastCameraCaptureStateChangedEventArgs {}
impl ::core::fmt::Debug for AppBroadcastCameraCaptureStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastCameraCaptureStateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastCameraCaptureStateChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastCameraCaptureStateChangedEventArgs;{1e334cd0-b882-4b88-8692-05999aceb70f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastCameraCaptureStateChangedEventArgs {
    type Vtable = IAppBroadcastCameraCaptureStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastCameraCaptureStateChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastCameraCaptureStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastCameraCaptureStateChangedEventArgs";
}
impl ::core::convert::From<AppBroadcastCameraCaptureStateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastCameraCaptureStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastCameraCaptureStateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastCameraCaptureStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastCameraCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastCameraCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastCameraCaptureStateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastCameraCaptureStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastCameraCaptureStateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastCameraCaptureStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastCameraCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastCameraCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastCameraCaptureStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppBroadcastCameraCaptureStateChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppBroadcastCameraOverlayLocation(pub i32);
impl AppBroadcastCameraOverlayLocation {
    pub const TopLeft: Self = Self(0i32);
    pub const TopCenter: Self = Self(1i32);
    pub const TopRight: Self = Self(2i32);
    pub const MiddleLeft: Self = Self(3i32);
    pub const MiddleCenter: Self = Self(4i32);
    pub const MiddleRight: Self = Self(5i32);
    pub const BottomLeft: Self = Self(6i32);
    pub const BottomCenter: Self = Self(7i32);
    pub const BottomRight: Self = Self(8i32);
}
impl ::core::marker::Copy for AppBroadcastCameraOverlayLocation {}
impl ::core::clone::Clone for AppBroadcastCameraOverlayLocation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppBroadcastCameraOverlayLocation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppBroadcastCameraOverlayLocation {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppBroadcastCameraOverlayLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastCameraOverlayLocation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastCameraOverlayLocation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastCameraOverlayLocation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppBroadcastCameraOverlaySize(pub i32);
impl AppBroadcastCameraOverlaySize {
    pub const Small: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const Large: Self = Self(2i32);
}
impl ::core::marker::Copy for AppBroadcastCameraOverlaySize {}
impl ::core::clone::Clone for AppBroadcastCameraOverlaySize {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppBroadcastCameraOverlaySize {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppBroadcastCameraOverlaySize {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppBroadcastCameraOverlaySize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastCameraOverlaySize").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastCameraOverlaySize {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastCameraOverlaySize;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppBroadcastCaptureTargetType(pub i32);
impl AppBroadcastCaptureTargetType {
    pub const AppView: Self = Self(0i32);
    pub const EntireDisplay: Self = Self(1i32);
}
impl ::core::marker::Copy for AppBroadcastCaptureTargetType {}
impl ::core::clone::Clone for AppBroadcastCaptureTargetType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppBroadcastCaptureTargetType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppBroadcastCaptureTargetType {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppBroadcastCaptureTargetType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastCaptureTargetType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastCaptureTargetType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastCaptureTargetType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppBroadcastExitBroadcastModeReason(pub i32);
impl AppBroadcastExitBroadcastModeReason {
    pub const NormalExit: Self = Self(0i32);
    pub const UserCanceled: Self = Self(1i32);
    pub const AuthorizationFail: Self = Self(2i32);
    pub const ForegroundAppActivated: Self = Self(3i32);
}
impl ::core::marker::Copy for AppBroadcastExitBroadcastModeReason {}
impl ::core::clone::Clone for AppBroadcastExitBroadcastModeReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppBroadcastExitBroadcastModeReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppBroadcastExitBroadcastModeReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppBroadcastExitBroadcastModeReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastExitBroadcastModeReason").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastExitBroadcastModeReason {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastExitBroadcastModeReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppBroadcastGlobalSettings(::windows_core::IUnknown);
impl AppBroadcastGlobalSettings {
    pub fn IsBroadcastEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBroadcastEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDisabledByPolicy(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDisabledByPolicy)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsGpuConstrained(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsGpuConstrained)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn HasHardwareEncoder(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasHardwareEncoder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAudioCaptureEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsAudioCaptureEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAudioCaptureEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAudioCaptureEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsMicrophoneCaptureEnabledByDefault(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsMicrophoneCaptureEnabledByDefault)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsMicrophoneCaptureEnabledByDefault(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsMicrophoneCaptureEnabledByDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsEchoCancellationEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsEchoCancellationEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsEchoCancellationEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEchoCancellationEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetSystemAudioGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSystemAudioGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SystemAudioGain(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).SystemAudioGain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetMicrophoneGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMicrophoneGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MicrophoneGain(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).MicrophoneGain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetIsCameraCaptureEnabledByDefault(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsCameraCaptureEnabledByDefault)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsCameraCaptureEnabledByDefault(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCameraCaptureEnabledByDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetSelectedCameraId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedCameraId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectedCameraId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedCameraId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetCameraOverlayLocation(&self, value: AppBroadcastCameraOverlayLocation) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCameraOverlayLocation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CameraOverlayLocation(&self) -> ::windows_core::Result<AppBroadcastCameraOverlayLocation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppBroadcastCameraOverlayLocation>::zeroed();
            (::windows_core::Interface::vtable(this).CameraOverlayLocation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastCameraOverlayLocation>(result__)
        }
    }
    pub fn SetCameraOverlaySize(&self, value: AppBroadcastCameraOverlaySize) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCameraOverlaySize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CameraOverlaySize(&self) -> ::windows_core::Result<AppBroadcastCameraOverlaySize> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppBroadcastCameraOverlaySize>::zeroed();
            (::windows_core::Interface::vtable(this).CameraOverlaySize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastCameraOverlaySize>(result__)
        }
    }
    pub fn SetIsCursorImageCaptureEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsCursorImageCaptureEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsCursorImageCaptureEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCursorImageCaptureEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastGlobalSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastGlobalSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastGlobalSettings {}
impl ::core::fmt::Debug for AppBroadcastGlobalSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastGlobalSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastGlobalSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastGlobalSettings;{b2cb27a5-70fc-4e17-80bd-6ba0fd3ff3a0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastGlobalSettings {
    type Vtable = IAppBroadcastGlobalSettings_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastGlobalSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastGlobalSettings {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastGlobalSettings";
}
impl ::core::convert::From<AppBroadcastGlobalSettings> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastGlobalSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastGlobalSettings> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastGlobalSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastGlobalSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastGlobalSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastGlobalSettings> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastGlobalSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastGlobalSettings> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastGlobalSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastGlobalSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastGlobalSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct AppBroadcastHeartbeatRequestedEventArgs(::windows_core::IUnknown);
impl AppBroadcastHeartbeatRequestedEventArgs {
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastHeartbeatRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastHeartbeatRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastHeartbeatRequestedEventArgs {}
impl ::core::fmt::Debug for AppBroadcastHeartbeatRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastHeartbeatRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastHeartbeatRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastHeartbeatRequestedEventArgs;{cea54283-ee51-4dbf-9472-79a9ed4e2165})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastHeartbeatRequestedEventArgs {
    type Vtable = IAppBroadcastHeartbeatRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastHeartbeatRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastHeartbeatRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastHeartbeatRequestedEventArgs";
}
impl ::core::convert::From<AppBroadcastHeartbeatRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastHeartbeatRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastHeartbeatRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastHeartbeatRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastHeartbeatRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastHeartbeatRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastHeartbeatRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastHeartbeatRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastHeartbeatRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastHeartbeatRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastHeartbeatRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastHeartbeatRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
pub struct AppBroadcastManager;
impl AppBroadcastManager {
    pub fn GetGlobalSettings() -> ::windows_core::Result<AppBroadcastGlobalSettings> {
        Self::IAppBroadcastManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetGlobalSettings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastGlobalSettings>(result__)
        })
    }
    pub fn ApplyGlobalSettings<'a, Param0: ::windows_core::IntoParam<'a, AppBroadcastGlobalSettings>>(value: Param0) -> ::windows_core::Result<()> {
        Self::IAppBroadcastManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ApplyGlobalSettings)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() })
    }
    pub fn GetProviderSettings() -> ::windows_core::Result<AppBroadcastProviderSettings> {
        Self::IAppBroadcastManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetProviderSettings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastProviderSettings>(result__)
        })
    }
    pub fn ApplyProviderSettings<'a, Param0: ::windows_core::IntoParam<'a, AppBroadcastProviderSettings>>(value: Param0) -> ::windows_core::Result<()> {
        Self::IAppBroadcastManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ApplyProviderSettings)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() })
    }
    pub fn IAppBroadcastManagerStatics<R, F: FnOnce(&IAppBroadcastManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppBroadcastManager, IAppBroadcastManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for AppBroadcastManager {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastManager";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppBroadcastMicrophoneCaptureState(pub i32);
impl AppBroadcastMicrophoneCaptureState {
    pub const Stopped: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for AppBroadcastMicrophoneCaptureState {}
impl ::core::clone::Clone for AppBroadcastMicrophoneCaptureState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppBroadcastMicrophoneCaptureState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppBroadcastMicrophoneCaptureState {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppBroadcastMicrophoneCaptureState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastMicrophoneCaptureState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastMicrophoneCaptureState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastMicrophoneCaptureState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppBroadcastMicrophoneCaptureStateChangedEventArgs(::windows_core::IUnknown);
impl AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    pub fn State(&self) -> ::windows_core::Result<AppBroadcastMicrophoneCaptureState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppBroadcastMicrophoneCaptureState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastMicrophoneCaptureState>(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastMicrophoneCaptureStateChangedEventArgs {}
impl ::core::fmt::Debug for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastMicrophoneCaptureStateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastMicrophoneCaptureStateChangedEventArgs;{a86ad5e9-9440-4908-9d09-65b7e315d795})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    type Vtable = IAppBroadcastMicrophoneCaptureStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastMicrophoneCaptureStateChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastMicrophoneCaptureStateChangedEventArgs";
}
impl ::core::convert::From<AppBroadcastMicrophoneCaptureStateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastMicrophoneCaptureStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastMicrophoneCaptureStateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastMicrophoneCaptureStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastMicrophoneCaptureStateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastMicrophoneCaptureStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastMicrophoneCaptureStateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastMicrophoneCaptureStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastMicrophoneCaptureStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppBroadcastMicrophoneCaptureStateChangedEventArgs {}
#[repr(transparent)]
pub struct AppBroadcastPlugIn(::windows_core::IUnknown);
impl AppBroadcastPlugIn {
    pub fn AppId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ProviderSettings(&self) -> ::windows_core::Result<AppBroadcastProviderSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProviderSettings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastProviderSettings>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Logo(&self) -> ::windows_core::Result<::winrt_storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Logo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastPlugIn {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastPlugIn {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastPlugIn {}
impl ::core::fmt::Debug for AppBroadcastPlugIn {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastPlugIn").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastPlugIn {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPlugIn;{520c1e66-6513-4574-ac54-23b79729615b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastPlugIn {
    type Vtable = IAppBroadcastPlugIn_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastPlugIn as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastPlugIn {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPlugIn";
}
impl ::core::convert::From<AppBroadcastPlugIn> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastPlugIn) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPlugIn> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastPlugIn) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastPlugIn {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastPlugIn {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastPlugIn> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastPlugIn) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPlugIn> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastPlugIn) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastPlugIn {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastPlugIn {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastPlugIn {}
unsafe impl ::core::marker::Sync for AppBroadcastPlugIn {}
#[repr(transparent)]
pub struct AppBroadcastPlugInManager(::windows_core::IUnknown);
impl AppBroadcastPlugInManager {
    pub fn IsBroadcastProviderAvailable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBroadcastProviderAvailable)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn PlugInList(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<AppBroadcastPlugIn>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PlugInList)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<AppBroadcastPlugIn>>(result__)
        }
    }
    pub fn DefaultPlugIn(&self) -> ::windows_core::Result<AppBroadcastPlugIn> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultPlugIn)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastPlugIn>(result__)
        }
    }
    pub fn SetDefaultPlugIn<'a, Param0: ::windows_core::IntoParam<'a, AppBroadcastPlugIn>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDefaultPlugIn)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn GetDefault() -> ::windows_core::Result<AppBroadcastPlugInManager> {
        Self::IAppBroadcastPlugInManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastPlugInManager>(result__)
        })
    }
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>>(user: Param0) -> ::windows_core::Result<AppBroadcastPlugInManager> {
        Self::IAppBroadcastPlugInManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), result__.as_mut_ptr()).from_abi::<AppBroadcastPlugInManager>(result__)
        })
    }
    pub fn IAppBroadcastPlugInManagerStatics<R, F: FnOnce(&IAppBroadcastPlugInManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppBroadcastPlugInManager, IAppBroadcastPlugInManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppBroadcastPlugInManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastPlugInManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastPlugInManager {}
impl ::core::fmt::Debug for AppBroadcastPlugInManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastPlugInManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastPlugInManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPlugInManager;{e550d979-27a1-49a7-bbf4-d7a9e9d07668})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastPlugInManager {
    type Vtable = IAppBroadcastPlugInManager_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastPlugInManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastPlugInManager {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPlugInManager";
}
impl ::core::convert::From<AppBroadcastPlugInManager> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastPlugInManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPlugInManager> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastPlugInManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastPlugInManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastPlugInManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastPlugInManager> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastPlugInManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPlugInManager> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastPlugInManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastPlugInManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastPlugInManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastPlugInManager {}
unsafe impl ::core::marker::Sync for AppBroadcastPlugInManager {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppBroadcastPlugInState(pub i32);
impl AppBroadcastPlugInState {
    pub const Unknown: Self = Self(0i32);
    pub const Initialized: Self = Self(1i32);
    pub const MicrosoftSignInRequired: Self = Self(2i32);
    pub const OAuthSignInRequired: Self = Self(3i32);
    pub const ProviderSignInRequired: Self = Self(4i32);
    pub const InBandwidthTest: Self = Self(5i32);
    pub const ReadyToBroadcast: Self = Self(6i32);
}
impl ::core::marker::Copy for AppBroadcastPlugInState {}
impl ::core::clone::Clone for AppBroadcastPlugInState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppBroadcastPlugInState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppBroadcastPlugInState {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppBroadcastPlugInState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastPlugInState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastPlugInState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastPlugInState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppBroadcastPlugInStateChangedEventArgs(::windows_core::IUnknown);
impl AppBroadcastPlugInStateChangedEventArgs {
    pub fn PlugInState(&self) -> ::windows_core::Result<AppBroadcastPlugInState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppBroadcastPlugInState>::zeroed();
            (::windows_core::Interface::vtable(this).PlugInState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastPlugInState>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastPlugInStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastPlugInStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastPlugInStateChangedEventArgs {}
impl ::core::fmt::Debug for AppBroadcastPlugInStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastPlugInStateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastPlugInStateChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPlugInStateChangedEventArgs;{4881d0f2-abc5-4fc6-84b0-89370bb47212})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastPlugInStateChangedEventArgs {
    type Vtable = IAppBroadcastPlugInStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastPlugInStateChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastPlugInStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPlugInStateChangedEventArgs";
}
impl ::core::convert::From<AppBroadcastPlugInStateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastPlugInStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPlugInStateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastPlugInStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastPlugInStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastPlugInStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastPlugInStateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastPlugInStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPlugInStateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastPlugInStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastPlugInStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastPlugInStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastPlugInStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppBroadcastPlugInStateChangedEventArgs {}
#[repr(transparent)]
pub struct AppBroadcastPreview(::windows_core::IUnknown);
impl AppBroadcastPreview {
    pub fn StopPreview(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StopPreview)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn PreviewState(&self) -> ::windows_core::Result<AppBroadcastPreviewState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppBroadcastPreviewState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviewState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastPreviewState>(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u32>>(result__)
        }
    }
    pub fn PreviewStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppBroadcastPreview, AppBroadcastPreviewStateChangedEventArgs>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PreviewStateChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePreviewStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePreviewStateChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn PreviewStreamReader(&self) -> ::windows_core::Result<AppBroadcastPreviewStreamReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PreviewStreamReader)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastPreviewStreamReader>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastPreview {}
impl ::core::fmt::Debug for AppBroadcastPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastPreview").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastPreview {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPreview;{14b60f5a-6e4a-4b80-a14f-67ee77d153e7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastPreview {
    type Vtable = IAppBroadcastPreview_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastPreview as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastPreview {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPreview";
}
impl ::core::convert::From<AppBroadcastPreview> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPreview> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastPreview> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPreview> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastPreview {}
unsafe impl ::core::marker::Sync for AppBroadcastPreview {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppBroadcastPreviewState(pub i32);
impl AppBroadcastPreviewState {
    pub const Started: Self = Self(0i32);
    pub const Stopped: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for AppBroadcastPreviewState {}
impl ::core::clone::Clone for AppBroadcastPreviewState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppBroadcastPreviewState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppBroadcastPreviewState {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppBroadcastPreviewState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastPreviewState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastPreviewState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastPreviewState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppBroadcastPreviewStateChangedEventArgs(::windows_core::IUnknown);
impl AppBroadcastPreviewStateChangedEventArgs {
    pub fn PreviewState(&self) -> ::windows_core::Result<AppBroadcastPreviewState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppBroadcastPreviewState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviewState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastPreviewState>(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastPreviewStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastPreviewStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastPreviewStateChangedEventArgs {}
impl ::core::fmt::Debug for AppBroadcastPreviewStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastPreviewStateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastPreviewStateChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPreviewStateChangedEventArgs;{5a57f2de-8dea-4e86-90ad-03fc26b9653c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastPreviewStateChangedEventArgs {
    type Vtable = IAppBroadcastPreviewStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastPreviewStateChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastPreviewStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPreviewStateChangedEventArgs";
}
impl ::core::convert::From<AppBroadcastPreviewStateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastPreviewStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPreviewStateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastPreviewStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastPreviewStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastPreviewStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastPreviewStateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastPreviewStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPreviewStateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastPreviewStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastPreviewStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastPreviewStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastPreviewStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppBroadcastPreviewStateChangedEventArgs {}
#[repr(transparent)]
pub struct AppBroadcastPreviewStreamReader(::windows_core::IUnknown);
impl AppBroadcastPreviewStreamReader {
    pub fn VideoWidth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).VideoWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn VideoHeight(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).VideoHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn VideoStride(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).VideoStride)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn VideoBitmapPixelFormat(&self) -> ::windows_core::Result<::winrt_graphics::Imaging::BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_graphics::Imaging::BitmapPixelFormat>::zeroed();
            (::windows_core::Interface::vtable(this).VideoBitmapPixelFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::Imaging::BitmapPixelFormat>(result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn VideoBitmapAlphaMode(&self) -> ::windows_core::Result<::winrt_graphics::Imaging::BitmapAlphaMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_graphics::Imaging::BitmapAlphaMode>::zeroed();
            (::windows_core::Interface::vtable(this).VideoBitmapAlphaMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::Imaging::BitmapAlphaMode>(result__)
        }
    }
    pub fn TryGetNextVideoFrame(&self) -> ::windows_core::Result<AppBroadcastPreviewStreamVideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetNextVideoFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastPreviewStreamVideoFrame>(result__)
        }
    }
    pub fn VideoFrameArrived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppBroadcastPreviewStreamReader, ::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFrameArrived)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveVideoFrameArrived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveVideoFrameArrived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AppBroadcastPreviewStreamReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastPreviewStreamReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastPreviewStreamReader {}
impl ::core::fmt::Debug for AppBroadcastPreviewStreamReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastPreviewStreamReader").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastPreviewStreamReader {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPreviewStreamReader;{92228d50-db3f-40a8-8cd4-f4e371ddab37})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastPreviewStreamReader {
    type Vtable = IAppBroadcastPreviewStreamReader_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastPreviewStreamReader as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastPreviewStreamReader {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPreviewStreamReader";
}
impl ::core::convert::From<AppBroadcastPreviewStreamReader> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastPreviewStreamReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPreviewStreamReader> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastPreviewStreamReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastPreviewStreamReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastPreviewStreamReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastPreviewStreamReader> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastPreviewStreamReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPreviewStreamReader> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastPreviewStreamReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastPreviewStreamReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastPreviewStreamReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastPreviewStreamReader {}
unsafe impl ::core::marker::Sync for AppBroadcastPreviewStreamReader {}
#[repr(transparent)]
pub struct AppBroadcastPreviewStreamVideoFrame(::windows_core::IUnknown);
impl AppBroadcastPreviewStreamVideoFrame {
    pub fn VideoHeader(&self) -> ::windows_core::Result<AppBroadcastPreviewStreamVideoHeader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoHeader)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastPreviewStreamVideoHeader>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn VideoBuffer(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoBuffer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastPreviewStreamVideoFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastPreviewStreamVideoFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastPreviewStreamVideoFrame {}
impl ::core::fmt::Debug for AppBroadcastPreviewStreamVideoFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastPreviewStreamVideoFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastPreviewStreamVideoFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPreviewStreamVideoFrame;{010fbea1-94fe-4499-b8c0-8d244279fb12})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastPreviewStreamVideoFrame {
    type Vtable = IAppBroadcastPreviewStreamVideoFrame_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastPreviewStreamVideoFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastPreviewStreamVideoFrame {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPreviewStreamVideoFrame";
}
impl ::core::convert::From<AppBroadcastPreviewStreamVideoFrame> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastPreviewStreamVideoFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPreviewStreamVideoFrame> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastPreviewStreamVideoFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastPreviewStreamVideoFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastPreviewStreamVideoFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastPreviewStreamVideoFrame> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastPreviewStreamVideoFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPreviewStreamVideoFrame> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastPreviewStreamVideoFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastPreviewStreamVideoFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastPreviewStreamVideoFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastPreviewStreamVideoFrame {}
unsafe impl ::core::marker::Sync for AppBroadcastPreviewStreamVideoFrame {}
#[repr(transparent)]
pub struct AppBroadcastPreviewStreamVideoHeader(::windows_core::IUnknown);
impl AppBroadcastPreviewStreamVideoHeader {
    pub fn AbsoluteTimestamp(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).AbsoluteTimestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn RelativeTimestamp(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTimestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn FrameId(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).FrameId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastPreviewStreamVideoHeader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastPreviewStreamVideoHeader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastPreviewStreamVideoHeader {}
impl ::core::fmt::Debug for AppBroadcastPreviewStreamVideoHeader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastPreviewStreamVideoHeader").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastPreviewStreamVideoHeader {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPreviewStreamVideoHeader;{8bef6113-da84-4499-a7ab-87118cb4a157})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastPreviewStreamVideoHeader {
    type Vtable = IAppBroadcastPreviewStreamVideoHeader_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastPreviewStreamVideoHeader as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastPreviewStreamVideoHeader {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPreviewStreamVideoHeader";
}
impl ::core::convert::From<AppBroadcastPreviewStreamVideoHeader> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastPreviewStreamVideoHeader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPreviewStreamVideoHeader> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastPreviewStreamVideoHeader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastPreviewStreamVideoHeader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastPreviewStreamVideoHeader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastPreviewStreamVideoHeader> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastPreviewStreamVideoHeader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPreviewStreamVideoHeader> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastPreviewStreamVideoHeader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastPreviewStreamVideoHeader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastPreviewStreamVideoHeader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastPreviewStreamVideoHeader {}
unsafe impl ::core::marker::Sync for AppBroadcastPreviewStreamVideoHeader {}
#[repr(transparent)]
pub struct AppBroadcastProviderSettings(::windows_core::IUnknown);
impl AppBroadcastProviderSettings {
    pub fn SetDefaultBroadcastTitle<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDefaultBroadcastTitle)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DefaultBroadcastTitle(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultBroadcastTitle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetAudioEncodingBitrate(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAudioEncodingBitrate)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AudioEncodingBitrate(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).AudioEncodingBitrate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetCustomVideoEncodingBitrate(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCustomVideoEncodingBitrate)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CustomVideoEncodingBitrate(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).CustomVideoEncodingBitrate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetCustomVideoEncodingHeight(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCustomVideoEncodingHeight)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CustomVideoEncodingHeight(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).CustomVideoEncodingHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetCustomVideoEncodingWidth(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCustomVideoEncodingWidth)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CustomVideoEncodingWidth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).CustomVideoEncodingWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetVideoEncodingBitrateMode(&self, value: AppBroadcastVideoEncodingBitrateMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVideoEncodingBitrateMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VideoEncodingBitrateMode(&self) -> ::windows_core::Result<AppBroadcastVideoEncodingBitrateMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppBroadcastVideoEncodingBitrateMode>::zeroed();
            (::windows_core::Interface::vtable(this).VideoEncodingBitrateMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastVideoEncodingBitrateMode>(result__)
        }
    }
    pub fn SetVideoEncodingResolutionMode(&self, value: AppBroadcastVideoEncodingResolutionMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVideoEncodingResolutionMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VideoEncodingResolutionMode(&self) -> ::windows_core::Result<AppBroadcastVideoEncodingResolutionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppBroadcastVideoEncodingResolutionMode>::zeroed();
            (::windows_core::Interface::vtable(this).VideoEncodingResolutionMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastVideoEncodingResolutionMode>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastProviderSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastProviderSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastProviderSettings {}
impl ::core::fmt::Debug for AppBroadcastProviderSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastProviderSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastProviderSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastProviderSettings;{c30bdf62-9948-458f-ad50-aa06ec03da08})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastProviderSettings {
    type Vtable = IAppBroadcastProviderSettings_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastProviderSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastProviderSettings {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastProviderSettings";
}
impl ::core::convert::From<AppBroadcastProviderSettings> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastProviderSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastProviderSettings> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastProviderSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastProviderSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastProviderSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastProviderSettings> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastProviderSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastProviderSettings> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastProviderSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastProviderSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastProviderSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct AppBroadcastServices(::windows_core::IUnknown);
impl AppBroadcastServices {
    pub fn CaptureTargetType(&self) -> ::windows_core::Result<AppBroadcastCaptureTargetType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppBroadcastCaptureTargetType>::zeroed();
            (::windows_core::Interface::vtable(this).CaptureTargetType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastCaptureTargetType>(result__)
        }
    }
    pub fn SetCaptureTargetType(&self, value: AppBroadcastCaptureTargetType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCaptureTargetType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BroadcastTitle(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).BroadcastTitle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetBroadcastTitle<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBroadcastTitle)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BroadcastLanguage(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).BroadcastLanguage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetBroadcastLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBroadcastLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn UserName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).UserName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CanCapture(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanCapture)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn EnterBroadcastModeAsync<'a, Param0: ::windows_core::IntoParam<'a, AppBroadcastPlugIn>>(&self, plugin: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EnterBroadcastModeAsync)(::windows_core::Interface::as_raw(this), plugin.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<u32>>(result__)
        }
    }
    pub fn ExitBroadcastMode(&self, reason: AppBroadcastExitBroadcastModeReason) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ExitBroadcastMode)(::windows_core::Interface::as_raw(this), reason).ok() }
    }
    pub fn StartBroadcast(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StartBroadcast)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn PauseBroadcast(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PauseBroadcast)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ResumeBroadcast(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ResumeBroadcast)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn StartPreview<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Size>>(&self, desiredsize: Param0) -> ::windows_core::Result<AppBroadcastPreview> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartPreview)(::windows_core::Interface::as_raw(this), desiredsize.into_param().abi(), result__.as_mut_ptr()).from_abi::<AppBroadcastPreview>(result__)
        }
    }
    pub fn State(&self) -> ::windows_core::Result<AppBroadcastState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastState>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastServices {}
impl ::core::fmt::Debug for AppBroadcastServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastServices").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastServices {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastServices;{8660b4d6-969b-4e3c-ac3a-8b042ee4ee63})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastServices {
    type Vtable = IAppBroadcastServices_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastServices as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastServices {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastServices";
}
impl ::core::convert::From<AppBroadcastServices> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastServices> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastServices> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastServices> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastServices {}
unsafe impl ::core::marker::Sync for AppBroadcastServices {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppBroadcastSignInResult(pub i32);
impl AppBroadcastSignInResult {
    pub const Success: Self = Self(0i32);
    pub const AuthenticationFailed: Self = Self(1i32);
    pub const Unauthorized: Self = Self(2i32);
    pub const ServiceUnavailable: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
}
impl ::core::marker::Copy for AppBroadcastSignInResult {}
impl ::core::clone::Clone for AppBroadcastSignInResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppBroadcastSignInResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppBroadcastSignInResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppBroadcastSignInResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastSignInResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastSignInResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastSignInResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppBroadcastSignInState(pub i32);
impl AppBroadcastSignInState {
    pub const NotSignedIn: Self = Self(0i32);
    pub const MicrosoftSignInInProgress: Self = Self(1i32);
    pub const MicrosoftSignInComplete: Self = Self(2i32);
    pub const OAuthSignInInProgress: Self = Self(3i32);
    pub const OAuthSignInComplete: Self = Self(4i32);
}
impl ::core::marker::Copy for AppBroadcastSignInState {}
impl ::core::clone::Clone for AppBroadcastSignInState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppBroadcastSignInState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppBroadcastSignInState {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppBroadcastSignInState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastSignInState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastSignInState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastSignInState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppBroadcastSignInStateChangedEventArgs(::windows_core::IUnknown);
impl AppBroadcastSignInStateChangedEventArgs {
    pub fn SignInState(&self) -> ::windows_core::Result<AppBroadcastSignInState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppBroadcastSignInState>::zeroed();
            (::windows_core::Interface::vtable(this).SignInState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastSignInState>(result__)
        }
    }
    pub fn Result(&self) -> ::windows_core::Result<AppBroadcastSignInResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppBroadcastSignInResult>::zeroed();
            (::windows_core::Interface::vtable(this).Result)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastSignInResult>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastSignInStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastSignInStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastSignInStateChangedEventArgs {}
impl ::core::fmt::Debug for AppBroadcastSignInStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastSignInStateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastSignInStateChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastSignInStateChangedEventArgs;{02b692a4-5919-4a9e-8d5e-c9bb0dd3377a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastSignInStateChangedEventArgs {
    type Vtable = IAppBroadcastSignInStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastSignInStateChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastSignInStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastSignInStateChangedEventArgs";
}
impl ::core::convert::From<AppBroadcastSignInStateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastSignInStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastSignInStateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastSignInStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastSignInStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastSignInStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastSignInStateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastSignInStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastSignInStateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastSignInStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastSignInStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastSignInStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct AppBroadcastState(::windows_core::IUnknown);
impl AppBroadcastState {
    pub fn IsCaptureTargetRunning(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCaptureTargetRunning)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ViewerCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ViewerCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn ShouldCaptureMicrophone(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ShouldCaptureMicrophone)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetShouldCaptureMicrophone(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetShouldCaptureMicrophone)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RestartMicrophoneCapture(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RestartMicrophoneCapture)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ShouldCaptureCamera(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ShouldCaptureCamera)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetShouldCaptureCamera(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetShouldCaptureCamera)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RestartCameraCapture(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RestartCameraCapture)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn EncodedVideoSize(&self) -> ::windows_core::Result<::winrt_foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).EncodedVideoSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Size>(result__)
        }
    }
    pub fn MicrophoneCaptureState(&self) -> ::windows_core::Result<AppBroadcastMicrophoneCaptureState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppBroadcastMicrophoneCaptureState>::zeroed();
            (::windows_core::Interface::vtable(this).MicrophoneCaptureState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastMicrophoneCaptureState>(result__)
        }
    }
    pub fn MicrophoneCaptureError(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MicrophoneCaptureError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn CameraCaptureState(&self) -> ::windows_core::Result<AppBroadcastCameraCaptureState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppBroadcastCameraCaptureState>::zeroed();
            (::windows_core::Interface::vtable(this).CameraCaptureState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastCameraCaptureState>(result__)
        }
    }
    pub fn CameraCaptureError(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).CameraCaptureError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn StreamState(&self) -> ::windows_core::Result<AppBroadcastStreamState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppBroadcastStreamState>::zeroed();
            (::windows_core::Interface::vtable(this).StreamState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastStreamState>(result__)
        }
    }
    pub fn PlugInState(&self) -> ::windows_core::Result<AppBroadcastPlugInState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppBroadcastPlugInState>::zeroed();
            (::windows_core::Interface::vtable(this).PlugInState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastPlugInState>(result__)
        }
    }
    pub fn OAuthRequestUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OAuthRequestUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn OAuthCallbackUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OAuthCallbackUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Security_Authentication_Web")]
    pub fn AuthenticationResult(&self) -> ::windows_core::Result<::winrt_security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticationResult)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_security::Authentication::Web::WebAuthenticationResult>(result__)
        }
    }
    #[cfg(feature = "Security_Authentication_Web")]
    pub fn SetAuthenticationResult<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_security::Authentication::Web::WebAuthenticationResult>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAuthenticationResult)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SetSignInState(&self, value: AppBroadcastSignInState) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSignInState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SignInState(&self) -> ::windows_core::Result<AppBroadcastSignInState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppBroadcastSignInState>::zeroed();
            (::windows_core::Interface::vtable(this).SignInState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastSignInState>(result__)
        }
    }
    pub fn TerminationReason(&self) -> ::windows_core::Result<AppBroadcastTerminationReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppBroadcastTerminationReason>::zeroed();
            (::windows_core::Interface::vtable(this).TerminationReason)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastTerminationReason>(result__)
        }
    }
    pub fn TerminationReasonPlugInSpecific(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).TerminationReasonPlugInSpecific)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn ViewerCountChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppBroadcastState, AppBroadcastViewerCountChangedEventArgs>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ViewerCountChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveViewerCountChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveViewerCountChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn MicrophoneCaptureStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppBroadcastState, AppBroadcastMicrophoneCaptureStateChangedEventArgs>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).MicrophoneCaptureStateChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveMicrophoneCaptureStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMicrophoneCaptureStateChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn CameraCaptureStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppBroadcastState, AppBroadcastCameraCaptureStateChangedEventArgs>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CameraCaptureStateChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCameraCaptureStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCameraCaptureStateChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn PlugInStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppBroadcastState, AppBroadcastPlugInStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PlugInStateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePlugInStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePlugInStateChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn StreamStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppBroadcastState, AppBroadcastStreamStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).StreamStateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStreamStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStreamStateChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn CaptureTargetClosed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppBroadcastState, ::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CaptureTargetClosed)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCaptureTargetClosed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCaptureTargetClosed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AppBroadcastState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastState {}
impl ::core::fmt::Debug for AppBroadcastState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastState;{ee08056d-8099-4ddd-922e-c56dac58abfb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastState {
    type Vtable = IAppBroadcastState_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastState as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastState {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastState";
}
impl ::core::convert::From<AppBroadcastState> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastState> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastState {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastState {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastState> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastState> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastState {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastState {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastState {}
unsafe impl ::core::marker::Sync for AppBroadcastState {}
#[repr(transparent)]
pub struct AppBroadcastStreamAudioFrame(::windows_core::IUnknown);
impl AppBroadcastStreamAudioFrame {
    pub fn AudioHeader(&self) -> ::windows_core::Result<AppBroadcastStreamAudioHeader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AudioHeader)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastStreamAudioHeader>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn AudioBuffer(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AudioBuffer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastStreamAudioFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastStreamAudioFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastStreamAudioFrame {}
impl ::core::fmt::Debug for AppBroadcastStreamAudioFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastStreamAudioFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastStreamAudioFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastStreamAudioFrame;{efab4ac8-21ba-453f-8bb7-5e938a2e9a74})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastStreamAudioFrame {
    type Vtable = IAppBroadcastStreamAudioFrame_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastStreamAudioFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastStreamAudioFrame {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamAudioFrame";
}
impl ::core::convert::From<AppBroadcastStreamAudioFrame> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastStreamAudioFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastStreamAudioFrame> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastStreamAudioFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastStreamAudioFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastStreamAudioFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastStreamAudioFrame> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastStreamAudioFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastStreamAudioFrame> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastStreamAudioFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastStreamAudioFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastStreamAudioFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct AppBroadcastStreamAudioHeader(::windows_core::IUnknown);
impl AppBroadcastStreamAudioHeader {
    pub fn AbsoluteTimestamp(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).AbsoluteTimestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn RelativeTimestamp(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTimestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn HasDiscontinuity(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasDiscontinuity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn FrameId(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).FrameId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastStreamAudioHeader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastStreamAudioHeader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastStreamAudioHeader {}
impl ::core::fmt::Debug for AppBroadcastStreamAudioHeader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastStreamAudioHeader").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastStreamAudioHeader {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastStreamAudioHeader;{bf21a570-6b78-4216-9f07-5aff5256f1b7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastStreamAudioHeader {
    type Vtable = IAppBroadcastStreamAudioHeader_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastStreamAudioHeader as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastStreamAudioHeader {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamAudioHeader";
}
impl ::core::convert::From<AppBroadcastStreamAudioHeader> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastStreamAudioHeader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastStreamAudioHeader> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastStreamAudioHeader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastStreamAudioHeader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastStreamAudioHeader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastStreamAudioHeader> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastStreamAudioHeader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastStreamAudioHeader> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastStreamAudioHeader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastStreamAudioHeader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastStreamAudioHeader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct AppBroadcastStreamReader(::windows_core::IUnknown);
impl AppBroadcastStreamReader {
    pub fn AudioChannels(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).AudioChannels)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn AudioSampleRate(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).AudioSampleRate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn AudioAacSequence(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AudioAacSequence)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    pub fn AudioBitrate(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).AudioBitrate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn TryGetNextAudioFrame(&self) -> ::windows_core::Result<AppBroadcastStreamAudioFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetNextAudioFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastStreamAudioFrame>(result__)
        }
    }
    pub fn VideoWidth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).VideoWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn VideoHeight(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).VideoHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn VideoBitrate(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).VideoBitrate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn TryGetNextVideoFrame(&self) -> ::windows_core::Result<AppBroadcastStreamVideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetNextVideoFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastStreamVideoFrame>(result__)
        }
    }
    pub fn AudioFrameArrived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppBroadcastStreamReader, ::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AudioFrameArrived)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAudioFrameArrived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAudioFrameArrived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn VideoFrameArrived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppBroadcastStreamReader, ::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFrameArrived)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveVideoFrameArrived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveVideoFrameArrived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AppBroadcastStreamReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastStreamReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastStreamReader {}
impl ::core::fmt::Debug for AppBroadcastStreamReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastStreamReader").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastStreamReader {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastStreamReader;{b338bcf9-3364-4460-b5f1-3cc2796a8aa2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastStreamReader {
    type Vtable = IAppBroadcastStreamReader_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastStreamReader as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastStreamReader {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamReader";
}
impl ::core::convert::From<AppBroadcastStreamReader> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastStreamReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastStreamReader> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastStreamReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastStreamReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastStreamReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastStreamReader> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastStreamReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastStreamReader> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastStreamReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastStreamReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastStreamReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppBroadcastStreamState(pub i32);
impl AppBroadcastStreamState {
    pub const Initializing: Self = Self(0i32);
    pub const StreamReady: Self = Self(1i32);
    pub const Started: Self = Self(2i32);
    pub const Paused: Self = Self(3i32);
    pub const Terminated: Self = Self(4i32);
}
impl ::core::marker::Copy for AppBroadcastStreamState {}
impl ::core::clone::Clone for AppBroadcastStreamState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppBroadcastStreamState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppBroadcastStreamState {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppBroadcastStreamState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastStreamState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastStreamState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastStreamState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppBroadcastStreamStateChangedEventArgs(::windows_core::IUnknown);
impl AppBroadcastStreamStateChangedEventArgs {
    pub fn StreamState(&self) -> ::windows_core::Result<AppBroadcastStreamState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppBroadcastStreamState>::zeroed();
            (::windows_core::Interface::vtable(this).StreamState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastStreamState>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastStreamStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastStreamStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastStreamStateChangedEventArgs {}
impl ::core::fmt::Debug for AppBroadcastStreamStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastStreamStateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastStreamStateChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastStreamStateChangedEventArgs;{5108a733-d008-4a89-93be-58aed961374e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastStreamStateChangedEventArgs {
    type Vtable = IAppBroadcastStreamStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastStreamStateChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastStreamStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamStateChangedEventArgs";
}
impl ::core::convert::From<AppBroadcastStreamStateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastStreamStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastStreamStateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastStreamStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastStreamStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastStreamStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastStreamStateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastStreamStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastStreamStateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastStreamStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastStreamStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastStreamStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct AppBroadcastStreamVideoFrame(::windows_core::IUnknown);
impl AppBroadcastStreamVideoFrame {
    pub fn VideoHeader(&self) -> ::windows_core::Result<AppBroadcastStreamVideoHeader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoHeader)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastStreamVideoHeader>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn VideoBuffer(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoBuffer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastStreamVideoFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastStreamVideoFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastStreamVideoFrame {}
impl ::core::fmt::Debug for AppBroadcastStreamVideoFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastStreamVideoFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastStreamVideoFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastStreamVideoFrame;{0f97cf2b-c9e4-4e88-8194-d814cbd585d8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastStreamVideoFrame {
    type Vtable = IAppBroadcastStreamVideoFrame_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastStreamVideoFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastStreamVideoFrame {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamVideoFrame";
}
impl ::core::convert::From<AppBroadcastStreamVideoFrame> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastStreamVideoFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastStreamVideoFrame> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastStreamVideoFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastStreamVideoFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastStreamVideoFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastStreamVideoFrame> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastStreamVideoFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastStreamVideoFrame> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastStreamVideoFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastStreamVideoFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastStreamVideoFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct AppBroadcastStreamVideoHeader(::windows_core::IUnknown);
impl AppBroadcastStreamVideoHeader {
    pub fn AbsoluteTimestamp(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).AbsoluteTimestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn RelativeTimestamp(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTimestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn IsKeyFrame(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsKeyFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn HasDiscontinuity(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasDiscontinuity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn FrameId(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).FrameId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastStreamVideoHeader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastStreamVideoHeader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastStreamVideoHeader {}
impl ::core::fmt::Debug for AppBroadcastStreamVideoHeader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastStreamVideoHeader").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastStreamVideoHeader {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastStreamVideoHeader;{0b9ebece-7e32-432d-8ca2-36bf10b9f462})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastStreamVideoHeader {
    type Vtable = IAppBroadcastStreamVideoHeader_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastStreamVideoHeader as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastStreamVideoHeader {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamVideoHeader";
}
impl ::core::convert::From<AppBroadcastStreamVideoHeader> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastStreamVideoHeader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastStreamVideoHeader> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastStreamVideoHeader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastStreamVideoHeader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastStreamVideoHeader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastStreamVideoHeader> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastStreamVideoHeader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastStreamVideoHeader> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastStreamVideoHeader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastStreamVideoHeader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastStreamVideoHeader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppBroadcastTerminationReason(pub i32);
impl AppBroadcastTerminationReason {
    pub const NormalTermination: Self = Self(0i32);
    pub const LostConnectionToService: Self = Self(1i32);
    pub const NoNetworkConnectivity: Self = Self(2i32);
    pub const ServiceAbort: Self = Self(3i32);
    pub const ServiceError: Self = Self(4i32);
    pub const ServiceUnavailable: Self = Self(5i32);
    pub const InternalError: Self = Self(6i32);
    pub const UnsupportedFormat: Self = Self(7i32);
    pub const BackgroundTaskTerminated: Self = Self(8i32);
    pub const BackgroundTaskUnresponsive: Self = Self(9i32);
}
impl ::core::marker::Copy for AppBroadcastTerminationReason {}
impl ::core::clone::Clone for AppBroadcastTerminationReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppBroadcastTerminationReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppBroadcastTerminationReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppBroadcastTerminationReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastTerminationReason").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastTerminationReason {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastTerminationReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppBroadcastTriggerDetails(::windows_core::IUnknown);
impl AppBroadcastTriggerDetails {
    pub fn BackgroundService(&self) -> ::windows_core::Result<AppBroadcastBackgroundService> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BackgroundService)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastBackgroundService>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastTriggerDetails {}
impl ::core::fmt::Debug for AppBroadcastTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastTriggerDetails;{deebab35-ec5e-4d8f-b1c0-5da6e8c75638})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastTriggerDetails {
    type Vtable = IAppBroadcastTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastTriggerDetails {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastTriggerDetails";
}
impl ::core::convert::From<AppBroadcastTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppBroadcastVideoEncodingBitrateMode(pub i32);
impl AppBroadcastVideoEncodingBitrateMode {
    pub const Custom: Self = Self(0i32);
    pub const Auto: Self = Self(1i32);
}
impl ::core::marker::Copy for AppBroadcastVideoEncodingBitrateMode {}
impl ::core::clone::Clone for AppBroadcastVideoEncodingBitrateMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppBroadcastVideoEncodingBitrateMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppBroadcastVideoEncodingBitrateMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppBroadcastVideoEncodingBitrateMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastVideoEncodingBitrateMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastVideoEncodingBitrateMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastVideoEncodingBitrateMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppBroadcastVideoEncodingResolutionMode(pub i32);
impl AppBroadcastVideoEncodingResolutionMode {
    pub const Custom: Self = Self(0i32);
    pub const Auto: Self = Self(1i32);
}
impl ::core::marker::Copy for AppBroadcastVideoEncodingResolutionMode {}
impl ::core::clone::Clone for AppBroadcastVideoEncodingResolutionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppBroadcastVideoEncodingResolutionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppBroadcastVideoEncodingResolutionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppBroadcastVideoEncodingResolutionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastVideoEncodingResolutionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastVideoEncodingResolutionMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastVideoEncodingResolutionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppBroadcastViewerCountChangedEventArgs(::windows_core::IUnknown);
impl AppBroadcastViewerCountChangedEventArgs {
    pub fn ViewerCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ViewerCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastViewerCountChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastViewerCountChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastViewerCountChangedEventArgs {}
impl ::core::fmt::Debug for AppBroadcastViewerCountChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastViewerCountChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastViewerCountChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastViewerCountChangedEventArgs;{e6e11825-5401-4ade-8bd2-c14ecee6807d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastViewerCountChangedEventArgs {
    type Vtable = IAppBroadcastViewerCountChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastViewerCountChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastViewerCountChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastViewerCountChangedEventArgs";
}
impl ::core::convert::From<AppBroadcastViewerCountChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastViewerCountChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastViewerCountChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastViewerCountChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastViewerCountChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastViewerCountChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastViewerCountChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastViewerCountChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastViewerCountChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastViewerCountChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastViewerCountChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastViewerCountChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastViewerCountChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppBroadcastViewerCountChangedEventArgs {}
#[repr(transparent)]
pub struct AppCapture(::windows_core::IUnknown);
impl AppCapture {
    pub fn IsCapturingAudio(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCapturingAudio)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsCapturingVideo(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCapturingVideo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CapturingChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppCapture, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CapturingChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCapturingChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCapturingChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<AppCapture> {
        Self::IAppCaptureStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppCapture>(result__)
        })
    }
    pub fn SetAllowedAsync(allowed: bool) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        Self::IAppCaptureStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetAllowedAsync)(::windows_core::Interface::as_raw(this), allowed, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        })
    }
    pub fn IAppCaptureStatics<R, F: FnOnce(&IAppCaptureStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppCapture, IAppCaptureStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAppCaptureStatics2<R, F: FnOnce(&IAppCaptureStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppCapture, IAppCaptureStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppCapture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppCapture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppCapture {}
impl ::core::fmt::Debug for AppCapture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCapture").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppCapture {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCapture;{9749d453-a29a-45ed-8f29-22d09942cff7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppCapture {
    type Vtable = IAppCapture_Vtbl;
    const IID: ::windows_core::GUID = <IAppCapture as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppCapture {
    const NAME: &'static str = "Windows.Media.Capture.AppCapture";
}
impl ::core::convert::From<AppCapture> for ::windows_core::IUnknown {
    fn from(value: AppCapture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCapture> for ::windows_core::IUnknown {
    fn from(value: &AppCapture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppCapture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppCapture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppCapture> for ::windows_core::IInspectable {
    fn from(value: AppCapture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCapture> for ::windows_core::IInspectable {
    fn from(value: &AppCapture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppCapture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppCapture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct AppCaptureAlternateShortcutKeys(::windows_core::IUnknown);
impl AppCaptureAlternateShortcutKeys {
    #[cfg(feature = "System")]
    pub fn SetToggleGameBarKey(&self, value: ::winrt_system::VirtualKey) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetToggleGameBarKey)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleGameBarKey(&self) -> ::windows_core::Result<::winrt_system::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_system::VirtualKey>::zeroed();
            (::windows_core::Interface::vtable(this).ToggleGameBarKey)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleGameBarKeyModifiers(&self, value: ::winrt_system::VirtualKeyModifiers) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetToggleGameBarKeyModifiers)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleGameBarKeyModifiers(&self) -> ::windows_core::Result<::winrt_system::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_system::VirtualKeyModifiers>::zeroed();
            (::windows_core::Interface::vtable(this).ToggleGameBarKeyModifiers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::VirtualKeyModifiers>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetSaveHistoricalVideoKey(&self, value: ::winrt_system::VirtualKey) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSaveHistoricalVideoKey)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn SaveHistoricalVideoKey(&self) -> ::windows_core::Result<::winrt_system::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_system::VirtualKey>::zeroed();
            (::windows_core::Interface::vtable(this).SaveHistoricalVideoKey)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetSaveHistoricalVideoKeyModifiers(&self, value: ::winrt_system::VirtualKeyModifiers) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSaveHistoricalVideoKeyModifiers)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn SaveHistoricalVideoKeyModifiers(&self) -> ::windows_core::Result<::winrt_system::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_system::VirtualKeyModifiers>::zeroed();
            (::windows_core::Interface::vtable(this).SaveHistoricalVideoKeyModifiers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::VirtualKeyModifiers>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleRecordingKey(&self, value: ::winrt_system::VirtualKey) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetToggleRecordingKey)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleRecordingKey(&self) -> ::windows_core::Result<::winrt_system::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_system::VirtualKey>::zeroed();
            (::windows_core::Interface::vtable(this).ToggleRecordingKey)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleRecordingKeyModifiers(&self, value: ::winrt_system::VirtualKeyModifiers) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetToggleRecordingKeyModifiers)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleRecordingKeyModifiers(&self) -> ::windows_core::Result<::winrt_system::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_system::VirtualKeyModifiers>::zeroed();
            (::windows_core::Interface::vtable(this).ToggleRecordingKeyModifiers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::VirtualKeyModifiers>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetTakeScreenshotKey(&self, value: ::winrt_system::VirtualKey) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTakeScreenshotKey)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn TakeScreenshotKey(&self) -> ::windows_core::Result<::winrt_system::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_system::VirtualKey>::zeroed();
            (::windows_core::Interface::vtable(this).TakeScreenshotKey)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetTakeScreenshotKeyModifiers(&self, value: ::winrt_system::VirtualKeyModifiers) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTakeScreenshotKeyModifiers)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn TakeScreenshotKeyModifiers(&self) -> ::windows_core::Result<::winrt_system::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_system::VirtualKeyModifiers>::zeroed();
            (::windows_core::Interface::vtable(this).TakeScreenshotKeyModifiers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::VirtualKeyModifiers>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleRecordingIndicatorKey(&self, value: ::winrt_system::VirtualKey) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetToggleRecordingIndicatorKey)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleRecordingIndicatorKey(&self) -> ::windows_core::Result<::winrt_system::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_system::VirtualKey>::zeroed();
            (::windows_core::Interface::vtable(this).ToggleRecordingIndicatorKey)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleRecordingIndicatorKeyModifiers(&self, value: ::winrt_system::VirtualKeyModifiers) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetToggleRecordingIndicatorKeyModifiers)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleRecordingIndicatorKeyModifiers(&self) -> ::windows_core::Result<::winrt_system::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_system::VirtualKeyModifiers>::zeroed();
            (::windows_core::Interface::vtable(this).ToggleRecordingIndicatorKeyModifiers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::VirtualKeyModifiers>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleMicrophoneCaptureKey(&self, value: ::winrt_system::VirtualKey) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppCaptureAlternateShortcutKeys2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetToggleMicrophoneCaptureKey)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleMicrophoneCaptureKey(&self) -> ::windows_core::Result<::winrt_system::VirtualKey> {
        let this = &::windows_core::Interface::cast::<IAppCaptureAlternateShortcutKeys2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_system::VirtualKey>::zeroed();
            (::windows_core::Interface::vtable(this).ToggleMicrophoneCaptureKey)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleMicrophoneCaptureKeyModifiers(&self, value: ::winrt_system::VirtualKeyModifiers) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppCaptureAlternateShortcutKeys2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetToggleMicrophoneCaptureKeyModifiers)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleMicrophoneCaptureKeyModifiers(&self) -> ::windows_core::Result<::winrt_system::VirtualKeyModifiers> {
        let this = &::windows_core::Interface::cast::<IAppCaptureAlternateShortcutKeys2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_system::VirtualKeyModifiers>::zeroed();
            (::windows_core::Interface::vtable(this).ToggleMicrophoneCaptureKeyModifiers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::VirtualKeyModifiers>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleCameraCaptureKey(&self, value: ::winrt_system::VirtualKey) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetToggleCameraCaptureKey)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleCameraCaptureKey(&self) -> ::windows_core::Result<::winrt_system::VirtualKey> {
        let this = &::windows_core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_system::VirtualKey>::zeroed();
            (::windows_core::Interface::vtable(this).ToggleCameraCaptureKey)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleCameraCaptureKeyModifiers(&self, value: ::winrt_system::VirtualKeyModifiers) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetToggleCameraCaptureKeyModifiers)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleCameraCaptureKeyModifiers(&self) -> ::windows_core::Result<::winrt_system::VirtualKeyModifiers> {
        let this = &::windows_core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_system::VirtualKeyModifiers>::zeroed();
            (::windows_core::Interface::vtable(this).ToggleCameraCaptureKeyModifiers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::VirtualKeyModifiers>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleBroadcastKey(&self, value: ::winrt_system::VirtualKey) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetToggleBroadcastKey)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleBroadcastKey(&self) -> ::windows_core::Result<::winrt_system::VirtualKey> {
        let this = &::windows_core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_system::VirtualKey>::zeroed();
            (::windows_core::Interface::vtable(this).ToggleBroadcastKey)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleBroadcastKeyModifiers(&self, value: ::winrt_system::VirtualKeyModifiers) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetToggleBroadcastKeyModifiers)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleBroadcastKeyModifiers(&self) -> ::windows_core::Result<::winrt_system::VirtualKeyModifiers> {
        let this = &::windows_core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_system::VirtualKeyModifiers>::zeroed();
            (::windows_core::Interface::vtable(this).ToggleBroadcastKeyModifiers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::VirtualKeyModifiers>(result__)
        }
    }
}
impl ::core::clone::Clone for AppCaptureAlternateShortcutKeys {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppCaptureAlternateShortcutKeys {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppCaptureAlternateShortcutKeys {}
impl ::core::fmt::Debug for AppCaptureAlternateShortcutKeys {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureAlternateShortcutKeys").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppCaptureAlternateShortcutKeys {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureAlternateShortcutKeys;{19e8e0ef-236c-40f9-b38f-9b7dd65d1ccc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppCaptureAlternateShortcutKeys {
    type Vtable = IAppCaptureAlternateShortcutKeys_Vtbl;
    const IID: ::windows_core::GUID = <IAppCaptureAlternateShortcutKeys as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppCaptureAlternateShortcutKeys {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureAlternateShortcutKeys";
}
impl ::core::convert::From<AppCaptureAlternateShortcutKeys> for ::windows_core::IUnknown {
    fn from(value: AppCaptureAlternateShortcutKeys) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureAlternateShortcutKeys> for ::windows_core::IUnknown {
    fn from(value: &AppCaptureAlternateShortcutKeys) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppCaptureAlternateShortcutKeys {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppCaptureAlternateShortcutKeys {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppCaptureAlternateShortcutKeys> for ::windows_core::IInspectable {
    fn from(value: AppCaptureAlternateShortcutKeys) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureAlternateShortcutKeys> for ::windows_core::IInspectable {
    fn from(value: &AppCaptureAlternateShortcutKeys) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppCaptureAlternateShortcutKeys {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppCaptureAlternateShortcutKeys {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct AppCaptureDurationGeneratedEventArgs(::windows_core::IUnknown);
impl AppCaptureDurationGeneratedEventArgs {
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for AppCaptureDurationGeneratedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppCaptureDurationGeneratedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppCaptureDurationGeneratedEventArgs {}
impl ::core::fmt::Debug for AppCaptureDurationGeneratedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureDurationGeneratedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppCaptureDurationGeneratedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureDurationGeneratedEventArgs;{c1f5563b-ffa1-44c9-975f-27fbeb553b35})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppCaptureDurationGeneratedEventArgs {
    type Vtable = IAppCaptureDurationGeneratedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppCaptureDurationGeneratedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppCaptureDurationGeneratedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureDurationGeneratedEventArgs";
}
impl ::core::convert::From<AppCaptureDurationGeneratedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppCaptureDurationGeneratedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureDurationGeneratedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppCaptureDurationGeneratedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppCaptureDurationGeneratedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppCaptureDurationGeneratedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppCaptureDurationGeneratedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppCaptureDurationGeneratedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureDurationGeneratedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppCaptureDurationGeneratedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppCaptureDurationGeneratedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppCaptureDurationGeneratedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppCaptureDurationGeneratedEventArgs {}
unsafe impl ::core::marker::Sync for AppCaptureDurationGeneratedEventArgs {}
#[repr(transparent)]
pub struct AppCaptureFileGeneratedEventArgs(::windows_core::IUnknown);
impl AppCaptureFileGeneratedEventArgs {
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> ::windows_core::Result<::winrt_storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).File)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::StorageFile>(result__)
        }
    }
}
impl ::core::clone::Clone for AppCaptureFileGeneratedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppCaptureFileGeneratedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppCaptureFileGeneratedEventArgs {}
impl ::core::fmt::Debug for AppCaptureFileGeneratedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureFileGeneratedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppCaptureFileGeneratedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureFileGeneratedEventArgs;{4189fbf4-465e-45bf-907f-165b3fb23758})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppCaptureFileGeneratedEventArgs {
    type Vtable = IAppCaptureFileGeneratedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppCaptureFileGeneratedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppCaptureFileGeneratedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureFileGeneratedEventArgs";
}
impl ::core::convert::From<AppCaptureFileGeneratedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppCaptureFileGeneratedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureFileGeneratedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppCaptureFileGeneratedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppCaptureFileGeneratedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppCaptureFileGeneratedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppCaptureFileGeneratedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppCaptureFileGeneratedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureFileGeneratedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppCaptureFileGeneratedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppCaptureFileGeneratedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppCaptureFileGeneratedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppCaptureFileGeneratedEventArgs {}
unsafe impl ::core::marker::Sync for AppCaptureFileGeneratedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppCaptureHistoricalBufferLengthUnit(pub i32);
impl AppCaptureHistoricalBufferLengthUnit {
    pub const Megabytes: Self = Self(0i32);
    pub const Seconds: Self = Self(1i32);
}
impl ::core::marker::Copy for AppCaptureHistoricalBufferLengthUnit {}
impl ::core::clone::Clone for AppCaptureHistoricalBufferLengthUnit {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppCaptureHistoricalBufferLengthUnit {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppCaptureHistoricalBufferLengthUnit {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppCaptureHistoricalBufferLengthUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureHistoricalBufferLengthUnit").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppCaptureHistoricalBufferLengthUnit {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureHistoricalBufferLengthUnit;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct AppCaptureManager;
impl AppCaptureManager {
    pub fn GetCurrentSettings() -> ::windows_core::Result<AppCaptureSettings> {
        Self::IAppCaptureManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentSettings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppCaptureSettings>(result__)
        })
    }
    pub fn ApplySettings<'a, Param0: ::windows_core::IntoParam<'a, AppCaptureSettings>>(appcapturesettings: Param0) -> ::windows_core::Result<()> {
        Self::IAppCaptureManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ApplySettings)(::windows_core::Interface::as_raw(this), appcapturesettings.into_param().abi()).ok() })
    }
    pub fn IAppCaptureManagerStatics<R, F: FnOnce(&IAppCaptureManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppCaptureManager, IAppCaptureManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for AppCaptureManager {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureManager";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppCaptureMetadataPriority(pub i32);
impl AppCaptureMetadataPriority {
    pub const Informational: Self = Self(0i32);
    pub const Important: Self = Self(1i32);
}
impl ::core::marker::Copy for AppCaptureMetadataPriority {}
impl ::core::clone::Clone for AppCaptureMetadataPriority {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppCaptureMetadataPriority {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppCaptureMetadataPriority {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppCaptureMetadataPriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureMetadataPriority").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppCaptureMetadataPriority {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureMetadataPriority;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppCaptureMetadataWriter(::windows_core::IUnknown);
impl AppCaptureMetadataWriter {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppCaptureMetadataWriter, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn AddStringEvent<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0, value: Param1, priority: AppCaptureMetadataPriority) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddStringEvent)(::windows_core::Interface::as_raw(this), name.into_param().abi(), value.into_param().abi(), priority).ok() }
    }
    pub fn AddInt32Event<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0, value: i32, priority: AppCaptureMetadataPriority) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddInt32Event)(::windows_core::Interface::as_raw(this), name.into_param().abi(), value, priority).ok() }
    }
    pub fn AddDoubleEvent<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0, value: f64, priority: AppCaptureMetadataPriority) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddDoubleEvent)(::windows_core::Interface::as_raw(this), name.into_param().abi(), value, priority).ok() }
    }
    pub fn StartStringState<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0, value: Param1, priority: AppCaptureMetadataPriority) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StartStringState)(::windows_core::Interface::as_raw(this), name.into_param().abi(), value.into_param().abi(), priority).ok() }
    }
    pub fn StartInt32State<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0, value: i32, priority: AppCaptureMetadataPriority) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StartInt32State)(::windows_core::Interface::as_raw(this), name.into_param().abi(), value, priority).ok() }
    }
    pub fn StartDoubleState<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0, value: f64, priority: AppCaptureMetadataPriority) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StartDoubleState)(::windows_core::Interface::as_raw(this), name.into_param().abi(), value, priority).ok() }
    }
    pub fn StopState<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StopState)(::windows_core::Interface::as_raw(this), name.into_param().abi()).ok() }
    }
    pub fn StopAllStates(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StopAllStates)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn RemainingStorageBytesAvailable(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).RemainingStorageBytesAvailable)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn MetadataPurged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppCaptureMetadataWriter, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).MetadataPurged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveMetadataPurged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMetadataPurged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for AppCaptureMetadataWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppCaptureMetadataWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppCaptureMetadataWriter {}
impl ::core::fmt::Debug for AppCaptureMetadataWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureMetadataWriter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppCaptureMetadataWriter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureMetadataWriter;{e0ce4877-9aaf-46b4-ad31-6a60b441c780})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppCaptureMetadataWriter {
    type Vtable = IAppCaptureMetadataWriter_Vtbl;
    const IID: ::windows_core::GUID = <IAppCaptureMetadataWriter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppCaptureMetadataWriter {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureMetadataWriter";
}
impl ::core::convert::From<AppCaptureMetadataWriter> for ::windows_core::IUnknown {
    fn from(value: AppCaptureMetadataWriter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureMetadataWriter> for ::windows_core::IUnknown {
    fn from(value: &AppCaptureMetadataWriter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppCaptureMetadataWriter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppCaptureMetadataWriter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppCaptureMetadataWriter> for ::windows_core::IInspectable {
    fn from(value: AppCaptureMetadataWriter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureMetadataWriter> for ::windows_core::IInspectable {
    fn from(value: &AppCaptureMetadataWriter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppCaptureMetadataWriter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppCaptureMetadataWriter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AppCaptureMetadataWriter> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: AppCaptureMetadataWriter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppCaptureMetadataWriter> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &AppCaptureMetadataWriter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for AppCaptureMetadataWriter {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &AppCaptureMetadataWriter {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppCaptureMetadataWriter {}
unsafe impl ::core::marker::Sync for AppCaptureMetadataWriter {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppCaptureMicrophoneCaptureState(pub i32);
impl AppCaptureMicrophoneCaptureState {
    pub const Stopped: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for AppCaptureMicrophoneCaptureState {}
impl ::core::clone::Clone for AppCaptureMicrophoneCaptureState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppCaptureMicrophoneCaptureState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppCaptureMicrophoneCaptureState {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppCaptureMicrophoneCaptureState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureMicrophoneCaptureState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppCaptureMicrophoneCaptureState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureMicrophoneCaptureState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppCaptureMicrophoneCaptureStateChangedEventArgs(::windows_core::IUnknown);
impl AppCaptureMicrophoneCaptureStateChangedEventArgs {
    pub fn State(&self) -> ::windows_core::Result<AppCaptureMicrophoneCaptureState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppCaptureMicrophoneCaptureState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppCaptureMicrophoneCaptureState>(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppCaptureMicrophoneCaptureStateChangedEventArgs {}
impl ::core::fmt::Debug for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureMicrophoneCaptureStateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureMicrophoneCaptureStateChangedEventArgs;{324d249e-45bc-4c35-bc35-e469fc7a69e0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    type Vtable = IAppCaptureMicrophoneCaptureStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppCaptureMicrophoneCaptureStateChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureMicrophoneCaptureStateChangedEventArgs";
}
impl ::core::convert::From<AppCaptureMicrophoneCaptureStateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppCaptureMicrophoneCaptureStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureMicrophoneCaptureStateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppCaptureMicrophoneCaptureStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppCaptureMicrophoneCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppCaptureMicrophoneCaptureStateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppCaptureMicrophoneCaptureStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureMicrophoneCaptureStateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppCaptureMicrophoneCaptureStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppCaptureMicrophoneCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppCaptureMicrophoneCaptureStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppCaptureMicrophoneCaptureStateChangedEventArgs {}
#[repr(transparent)]
pub struct AppCaptureRecordOperation(::windows_core::IUnknown);
impl AppCaptureRecordOperation {
    pub fn StopRecording(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StopRecording)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<AppCaptureRecordingState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppCaptureRecordingState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppCaptureRecordingState>(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u32>>(result__)
        }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> ::windows_core::Result<::winrt_storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).File)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::StorageFile>(result__)
        }
    }
    pub fn IsFileTruncated(&self) -> ::windows_core::Result<::winrt_foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsFileTruncated)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<bool>>(result__)
        }
    }
    pub fn StateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureRecordingStateChangedEventArgs>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).StateChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStateChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn DurationGenerated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureDurationGeneratedEventArgs>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DurationGenerated)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDurationGenerated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDurationGenerated)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn FileGenerated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureFileGeneratedEventArgs>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).FileGenerated)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveFileGenerated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFileGenerated)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AppCaptureRecordOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppCaptureRecordOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppCaptureRecordOperation {}
impl ::core::fmt::Debug for AppCaptureRecordOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureRecordOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppCaptureRecordOperation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureRecordOperation;{c66020a9-1538-495c-9bbb-2ba870ec5861})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppCaptureRecordOperation {
    type Vtable = IAppCaptureRecordOperation_Vtbl;
    const IID: ::windows_core::GUID = <IAppCaptureRecordOperation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppCaptureRecordOperation {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureRecordOperation";
}
impl ::core::convert::From<AppCaptureRecordOperation> for ::windows_core::IUnknown {
    fn from(value: AppCaptureRecordOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureRecordOperation> for ::windows_core::IUnknown {
    fn from(value: &AppCaptureRecordOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppCaptureRecordOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppCaptureRecordOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppCaptureRecordOperation> for ::windows_core::IInspectable {
    fn from(value: AppCaptureRecordOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureRecordOperation> for ::windows_core::IInspectable {
    fn from(value: &AppCaptureRecordOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppCaptureRecordOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppCaptureRecordOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppCaptureRecordOperation {}
unsafe impl ::core::marker::Sync for AppCaptureRecordOperation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppCaptureRecordingState(pub i32);
impl AppCaptureRecordingState {
    pub const InProgress: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for AppCaptureRecordingState {}
impl ::core::clone::Clone for AppCaptureRecordingState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppCaptureRecordingState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppCaptureRecordingState {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppCaptureRecordingState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureRecordingState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppCaptureRecordingState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureRecordingState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppCaptureRecordingStateChangedEventArgs(::windows_core::IUnknown);
impl AppCaptureRecordingStateChangedEventArgs {
    pub fn State(&self) -> ::windows_core::Result<AppCaptureRecordingState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppCaptureRecordingState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppCaptureRecordingState>(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for AppCaptureRecordingStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppCaptureRecordingStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppCaptureRecordingStateChangedEventArgs {}
impl ::core::fmt::Debug for AppCaptureRecordingStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureRecordingStateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppCaptureRecordingStateChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureRecordingStateChangedEventArgs;{24fc8712-e305-490d-b415-6b1c9049736b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppCaptureRecordingStateChangedEventArgs {
    type Vtable = IAppCaptureRecordingStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppCaptureRecordingStateChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppCaptureRecordingStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureRecordingStateChangedEventArgs";
}
impl ::core::convert::From<AppCaptureRecordingStateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppCaptureRecordingStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureRecordingStateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppCaptureRecordingStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppCaptureRecordingStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppCaptureRecordingStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppCaptureRecordingStateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppCaptureRecordingStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureRecordingStateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppCaptureRecordingStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppCaptureRecordingStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppCaptureRecordingStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppCaptureRecordingStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppCaptureRecordingStateChangedEventArgs {}
#[repr(transparent)]
pub struct AppCaptureServices(::windows_core::IUnknown);
impl AppCaptureServices {
    pub fn Record(&self) -> ::windows_core::Result<AppCaptureRecordOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Record)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppCaptureRecordOperation>(result__)
        }
    }
    pub fn RecordTimeSpan<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, starttime: Param0, duration: Param1) -> ::windows_core::Result<AppCaptureRecordOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RecordTimeSpan)(::windows_core::Interface::as_raw(this), starttime.into_param().abi(), duration.into_param().abi(), result__.as_mut_ptr()).from_abi::<AppCaptureRecordOperation>(result__)
        }
    }
    pub fn CanCapture(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanCapture)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn State(&self) -> ::windows_core::Result<AppCaptureState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppCaptureState>(result__)
        }
    }
}
impl ::core::clone::Clone for AppCaptureServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppCaptureServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppCaptureServices {}
impl ::core::fmt::Debug for AppCaptureServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureServices").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppCaptureServices {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureServices;{44fec0b5-34f5-4f18-ae8c-b9123abbfc0d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppCaptureServices {
    type Vtable = IAppCaptureServices_Vtbl;
    const IID: ::windows_core::GUID = <IAppCaptureServices as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppCaptureServices {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureServices";
}
impl ::core::convert::From<AppCaptureServices> for ::windows_core::IUnknown {
    fn from(value: AppCaptureServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureServices> for ::windows_core::IUnknown {
    fn from(value: &AppCaptureServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppCaptureServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppCaptureServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppCaptureServices> for ::windows_core::IInspectable {
    fn from(value: AppCaptureServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureServices> for ::windows_core::IInspectable {
    fn from(value: &AppCaptureServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppCaptureServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppCaptureServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppCaptureServices {}
unsafe impl ::core::marker::Sync for AppCaptureServices {}
#[repr(transparent)]
pub struct AppCaptureSettings(::windows_core::IUnknown);
impl AppCaptureSettings {
    #[cfg(feature = "Storage")]
    pub fn SetAppCaptureDestinationFolder<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::StorageFolder>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAppCaptureDestinationFolder)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage")]
    pub fn AppCaptureDestinationFolder(&self) -> ::windows_core::Result<::winrt_storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppCaptureDestinationFolder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::StorageFolder>(result__)
        }
    }
    pub fn SetAudioEncodingBitrate(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAudioEncodingBitrate)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AudioEncodingBitrate(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).AudioEncodingBitrate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetIsAudioCaptureEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsAudioCaptureEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAudioCaptureEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAudioCaptureEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCustomVideoEncodingBitrate(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCustomVideoEncodingBitrate)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CustomVideoEncodingBitrate(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).CustomVideoEncodingBitrate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetCustomVideoEncodingHeight(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCustomVideoEncodingHeight)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CustomVideoEncodingHeight(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).CustomVideoEncodingHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetCustomVideoEncodingWidth(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCustomVideoEncodingWidth)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CustomVideoEncodingWidth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).CustomVideoEncodingWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetHistoricalBufferLength(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHistoricalBufferLength)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HistoricalBufferLength(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).HistoricalBufferLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetHistoricalBufferLengthUnit(&self, value: AppCaptureHistoricalBufferLengthUnit) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHistoricalBufferLengthUnit)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HistoricalBufferLengthUnit(&self) -> ::windows_core::Result<AppCaptureHistoricalBufferLengthUnit> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppCaptureHistoricalBufferLengthUnit>::zeroed();
            (::windows_core::Interface::vtable(this).HistoricalBufferLengthUnit)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppCaptureHistoricalBufferLengthUnit>(result__)
        }
    }
    pub fn SetIsHistoricalCaptureEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsHistoricalCaptureEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsHistoricalCaptureEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsHistoricalCaptureEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsHistoricalCaptureOnBatteryAllowed(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsHistoricalCaptureOnBatteryAllowed)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsHistoricalCaptureOnBatteryAllowed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsHistoricalCaptureOnBatteryAllowed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsHistoricalCaptureOnWirelessDisplayAllowed(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsHistoricalCaptureOnWirelessDisplayAllowed)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsHistoricalCaptureOnWirelessDisplayAllowed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsHistoricalCaptureOnWirelessDisplayAllowed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetMaximumRecordLength<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaximumRecordLength)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn MaximumRecordLength(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).MaximumRecordLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage")]
    pub fn SetScreenshotDestinationFolder<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::StorageFolder>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScreenshotDestinationFolder)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage")]
    pub fn ScreenshotDestinationFolder(&self) -> ::windows_core::Result<::winrt_storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ScreenshotDestinationFolder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::StorageFolder>(result__)
        }
    }
    pub fn SetVideoEncodingBitrateMode(&self, value: AppCaptureVideoEncodingBitrateMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVideoEncodingBitrateMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VideoEncodingBitrateMode(&self) -> ::windows_core::Result<AppCaptureVideoEncodingBitrateMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppCaptureVideoEncodingBitrateMode>::zeroed();
            (::windows_core::Interface::vtable(this).VideoEncodingBitrateMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppCaptureVideoEncodingBitrateMode>(result__)
        }
    }
    pub fn SetVideoEncodingResolutionMode(&self, value: AppCaptureVideoEncodingResolutionMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVideoEncodingResolutionMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VideoEncodingResolutionMode(&self) -> ::windows_core::Result<AppCaptureVideoEncodingResolutionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppCaptureVideoEncodingResolutionMode>::zeroed();
            (::windows_core::Interface::vtable(this).VideoEncodingResolutionMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppCaptureVideoEncodingResolutionMode>(result__)
        }
    }
    pub fn SetIsAppCaptureEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsAppCaptureEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAppCaptureEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAppCaptureEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsCpuConstrained(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCpuConstrained)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDisabledByPolicy(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDisabledByPolicy)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsMemoryConstrained(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsMemoryConstrained)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn HasHardwareEncoder(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasHardwareEncoder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsGpuConstrained(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppCaptureSettings2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsGpuConstrained)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn AlternateShortcutKeys(&self) -> ::windows_core::Result<AppCaptureAlternateShortcutKeys> {
        let this = &::windows_core::Interface::cast::<IAppCaptureSettings2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AlternateShortcutKeys)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppCaptureAlternateShortcutKeys>(result__)
        }
    }
    pub fn SetIsMicrophoneCaptureEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppCaptureSettings3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsMicrophoneCaptureEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsMicrophoneCaptureEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppCaptureSettings3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsMicrophoneCaptureEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsMicrophoneCaptureEnabledByDefault(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsMicrophoneCaptureEnabledByDefault)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsMicrophoneCaptureEnabledByDefault(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsMicrophoneCaptureEnabledByDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetSystemAudioGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSystemAudioGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SystemAudioGain(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).SystemAudioGain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetMicrophoneGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMicrophoneGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MicrophoneGain(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).MicrophoneGain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetVideoEncodingFrameRateMode(&self, value: AppCaptureVideoEncodingFrameRateMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetVideoEncodingFrameRateMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VideoEncodingFrameRateMode(&self) -> ::windows_core::Result<AppCaptureVideoEncodingFrameRateMode> {
        let this = &::windows_core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppCaptureVideoEncodingFrameRateMode>::zeroed();
            (::windows_core::Interface::vtable(this).VideoEncodingFrameRateMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppCaptureVideoEncodingFrameRateMode>(result__)
        }
    }
    pub fn SetIsEchoCancellationEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppCaptureSettings5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsEchoCancellationEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsEchoCancellationEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppCaptureSettings5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEchoCancellationEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsCursorImageCaptureEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppCaptureSettings5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsCursorImageCaptureEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsCursorImageCaptureEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppCaptureSettings5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCursorImageCaptureEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for AppCaptureSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppCaptureSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppCaptureSettings {}
impl ::core::fmt::Debug for AppCaptureSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppCaptureSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureSettings;{14683a86-8807-48d3-883a-970ee4532a39})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppCaptureSettings {
    type Vtable = IAppCaptureSettings_Vtbl;
    const IID: ::windows_core::GUID = <IAppCaptureSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureSettings";
}
impl ::core::convert::From<AppCaptureSettings> for ::windows_core::IUnknown {
    fn from(value: AppCaptureSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureSettings> for ::windows_core::IUnknown {
    fn from(value: &AppCaptureSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppCaptureSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppCaptureSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppCaptureSettings> for ::windows_core::IInspectable {
    fn from(value: AppCaptureSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureSettings> for ::windows_core::IInspectable {
    fn from(value: &AppCaptureSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppCaptureSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppCaptureSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct AppCaptureState(::windows_core::IUnknown);
impl AppCaptureState {
    pub fn IsTargetRunning(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsTargetRunning)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsHistoricalCaptureEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsHistoricalCaptureEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ShouldCaptureMicrophone(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ShouldCaptureMicrophone)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetShouldCaptureMicrophone(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetShouldCaptureMicrophone)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RestartMicrophoneCapture(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RestartMicrophoneCapture)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MicrophoneCaptureState(&self) -> ::windows_core::Result<AppCaptureMicrophoneCaptureState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppCaptureMicrophoneCaptureState>::zeroed();
            (::windows_core::Interface::vtable(this).MicrophoneCaptureState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppCaptureMicrophoneCaptureState>(result__)
        }
    }
    pub fn MicrophoneCaptureError(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MicrophoneCaptureError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn MicrophoneCaptureStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppCaptureState, AppCaptureMicrophoneCaptureStateChangedEventArgs>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).MicrophoneCaptureStateChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveMicrophoneCaptureStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMicrophoneCaptureStateChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn CaptureTargetClosed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppCaptureState, ::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CaptureTargetClosed)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCaptureTargetClosed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCaptureTargetClosed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AppCaptureState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppCaptureState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppCaptureState {}
impl ::core::fmt::Debug for AppCaptureState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppCaptureState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureState;{73134372-d4eb-44ce-9538-465f506ac4ea})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppCaptureState {
    type Vtable = IAppCaptureState_Vtbl;
    const IID: ::windows_core::GUID = <IAppCaptureState as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppCaptureState {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureState";
}
impl ::core::convert::From<AppCaptureState> for ::windows_core::IUnknown {
    fn from(value: AppCaptureState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureState> for ::windows_core::IUnknown {
    fn from(value: &AppCaptureState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppCaptureState {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppCaptureState {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppCaptureState> for ::windows_core::IInspectable {
    fn from(value: AppCaptureState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureState> for ::windows_core::IInspectable {
    fn from(value: &AppCaptureState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppCaptureState {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppCaptureState {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppCaptureState {}
unsafe impl ::core::marker::Sync for AppCaptureState {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppCaptureVideoEncodingBitrateMode(pub i32);
impl AppCaptureVideoEncodingBitrateMode {
    pub const Custom: Self = Self(0i32);
    pub const High: Self = Self(1i32);
    pub const Standard: Self = Self(2i32);
}
impl ::core::marker::Copy for AppCaptureVideoEncodingBitrateMode {}
impl ::core::clone::Clone for AppCaptureVideoEncodingBitrateMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppCaptureVideoEncodingBitrateMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppCaptureVideoEncodingBitrateMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppCaptureVideoEncodingBitrateMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureVideoEncodingBitrateMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppCaptureVideoEncodingBitrateMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureVideoEncodingBitrateMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppCaptureVideoEncodingFrameRateMode(pub i32);
impl AppCaptureVideoEncodingFrameRateMode {
    pub const Standard: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl ::core::marker::Copy for AppCaptureVideoEncodingFrameRateMode {}
impl ::core::clone::Clone for AppCaptureVideoEncodingFrameRateMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppCaptureVideoEncodingFrameRateMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppCaptureVideoEncodingFrameRateMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppCaptureVideoEncodingFrameRateMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureVideoEncodingFrameRateMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppCaptureVideoEncodingFrameRateMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureVideoEncodingFrameRateMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppCaptureVideoEncodingResolutionMode(pub i32);
impl AppCaptureVideoEncodingResolutionMode {
    pub const Custom: Self = Self(0i32);
    pub const High: Self = Self(1i32);
    pub const Standard: Self = Self(2i32);
}
impl ::core::marker::Copy for AppCaptureVideoEncodingResolutionMode {}
impl ::core::clone::Clone for AppCaptureVideoEncodingResolutionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppCaptureVideoEncodingResolutionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppCaptureVideoEncodingResolutionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppCaptureVideoEncodingResolutionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureVideoEncodingResolutionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppCaptureVideoEncodingResolutionMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureVideoEncodingResolutionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct CameraCaptureUI(::windows_core::IUnknown);
impl CameraCaptureUI {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CameraCaptureUI, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn PhotoSettings(&self) -> ::windows_core::Result<CameraCaptureUIPhotoCaptureSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PhotoSettings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CameraCaptureUIPhotoCaptureSettings>(result__)
        }
    }
    pub fn VideoSettings(&self) -> ::windows_core::Result<CameraCaptureUIVideoCaptureSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoSettings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CameraCaptureUIVideoCaptureSettings>(result__)
        }
    }
    #[cfg(feature = "Storage")]
    pub fn CaptureFileAsync(&self, mode: CameraCaptureUIMode) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CaptureFileAsync)(::windows_core::Interface::as_raw(this), mode, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_storage::StorageFile>>(result__)
        }
    }
}
impl ::core::clone::Clone for CameraCaptureUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CameraCaptureUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CameraCaptureUI {}
impl ::core::fmt::Debug for CameraCaptureUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraCaptureUI").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CameraCaptureUI {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.CameraCaptureUI;{48587540-6f93-4bb4-b8f3-e89e48948c91})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CameraCaptureUI {
    type Vtable = ICameraCaptureUI_Vtbl;
    const IID: ::windows_core::GUID = <ICameraCaptureUI as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CameraCaptureUI {
    const NAME: &'static str = "Windows.Media.Capture.CameraCaptureUI";
}
impl ::core::convert::From<CameraCaptureUI> for ::windows_core::IUnknown {
    fn from(value: CameraCaptureUI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraCaptureUI> for ::windows_core::IUnknown {
    fn from(value: &CameraCaptureUI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CameraCaptureUI {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CameraCaptureUI {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CameraCaptureUI> for ::windows_core::IInspectable {
    fn from(value: CameraCaptureUI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraCaptureUI> for ::windows_core::IInspectable {
    fn from(value: &CameraCaptureUI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CameraCaptureUI {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CameraCaptureUI {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CameraCaptureUIMaxPhotoResolution(pub i32);
impl CameraCaptureUIMaxPhotoResolution {
    pub const HighestAvailable: Self = Self(0i32);
    pub const VerySmallQvga: Self = Self(1i32);
    pub const SmallVga: Self = Self(2i32);
    pub const MediumXga: Self = Self(3i32);
    pub const Large3M: Self = Self(4i32);
    pub const VeryLarge5M: Self = Self(5i32);
}
impl ::core::marker::Copy for CameraCaptureUIMaxPhotoResolution {}
impl ::core::clone::Clone for CameraCaptureUIMaxPhotoResolution {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CameraCaptureUIMaxPhotoResolution {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CameraCaptureUIMaxPhotoResolution {
    type Abi = Self;
}
impl ::core::fmt::Debug for CameraCaptureUIMaxPhotoResolution {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraCaptureUIMaxPhotoResolution").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CameraCaptureUIMaxPhotoResolution {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.CameraCaptureUIMaxPhotoResolution;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CameraCaptureUIMaxVideoResolution(pub i32);
impl CameraCaptureUIMaxVideoResolution {
    pub const HighestAvailable: Self = Self(0i32);
    pub const LowDefinition: Self = Self(1i32);
    pub const StandardDefinition: Self = Self(2i32);
    pub const HighDefinition: Self = Self(3i32);
}
impl ::core::marker::Copy for CameraCaptureUIMaxVideoResolution {}
impl ::core::clone::Clone for CameraCaptureUIMaxVideoResolution {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CameraCaptureUIMaxVideoResolution {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CameraCaptureUIMaxVideoResolution {
    type Abi = Self;
}
impl ::core::fmt::Debug for CameraCaptureUIMaxVideoResolution {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraCaptureUIMaxVideoResolution").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CameraCaptureUIMaxVideoResolution {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.CameraCaptureUIMaxVideoResolution;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CameraCaptureUIMode(pub i32);
impl CameraCaptureUIMode {
    pub const PhotoOrVideo: Self = Self(0i32);
    pub const Photo: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl ::core::marker::Copy for CameraCaptureUIMode {}
impl ::core::clone::Clone for CameraCaptureUIMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CameraCaptureUIMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CameraCaptureUIMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for CameraCaptureUIMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraCaptureUIMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CameraCaptureUIMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.CameraCaptureUIMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct CameraCaptureUIPhotoCaptureSettings(::windows_core::IUnknown);
impl CameraCaptureUIPhotoCaptureSettings {
    pub fn Format(&self) -> ::windows_core::Result<CameraCaptureUIPhotoFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CameraCaptureUIPhotoFormat>::zeroed();
            (::windows_core::Interface::vtable(this).Format)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CameraCaptureUIPhotoFormat>(result__)
        }
    }
    pub fn SetFormat(&self, value: CameraCaptureUIPhotoFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxResolution(&self) -> ::windows_core::Result<CameraCaptureUIMaxPhotoResolution> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CameraCaptureUIMaxPhotoResolution>::zeroed();
            (::windows_core::Interface::vtable(this).MaxResolution)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CameraCaptureUIMaxPhotoResolution>(result__)
        }
    }
    pub fn SetMaxResolution(&self, value: CameraCaptureUIMaxPhotoResolution) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxResolution)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CroppedSizeInPixels(&self) -> ::windows_core::Result<::winrt_foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).CroppedSizeInPixels)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Size>(result__)
        }
    }
    pub fn SetCroppedSizeInPixels<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Size>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCroppedSizeInPixels)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CroppedAspectRatio(&self) -> ::windows_core::Result<::winrt_foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).CroppedAspectRatio)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Size>(result__)
        }
    }
    pub fn SetCroppedAspectRatio<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Size>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCroppedAspectRatio)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn AllowCropping(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AllowCropping)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowCropping(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowCropping)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for CameraCaptureUIPhotoCaptureSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CameraCaptureUIPhotoCaptureSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CameraCaptureUIPhotoCaptureSettings {}
impl ::core::fmt::Debug for CameraCaptureUIPhotoCaptureSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraCaptureUIPhotoCaptureSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CameraCaptureUIPhotoCaptureSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.CameraCaptureUIPhotoCaptureSettings;{b9f5be97-3472-46a8-8a9e-04ce42ccc97d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CameraCaptureUIPhotoCaptureSettings {
    type Vtable = ICameraCaptureUIPhotoCaptureSettings_Vtbl;
    const IID: ::windows_core::GUID = <ICameraCaptureUIPhotoCaptureSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CameraCaptureUIPhotoCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.CameraCaptureUIPhotoCaptureSettings";
}
impl ::core::convert::From<CameraCaptureUIPhotoCaptureSettings> for ::windows_core::IUnknown {
    fn from(value: CameraCaptureUIPhotoCaptureSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraCaptureUIPhotoCaptureSettings> for ::windows_core::IUnknown {
    fn from(value: &CameraCaptureUIPhotoCaptureSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CameraCaptureUIPhotoCaptureSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CameraCaptureUIPhotoCaptureSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CameraCaptureUIPhotoCaptureSettings> for ::windows_core::IInspectable {
    fn from(value: CameraCaptureUIPhotoCaptureSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraCaptureUIPhotoCaptureSettings> for ::windows_core::IInspectable {
    fn from(value: &CameraCaptureUIPhotoCaptureSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CameraCaptureUIPhotoCaptureSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CameraCaptureUIPhotoCaptureSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CameraCaptureUIPhotoCaptureSettings {}
unsafe impl ::core::marker::Sync for CameraCaptureUIPhotoCaptureSettings {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CameraCaptureUIPhotoFormat(pub i32);
impl CameraCaptureUIPhotoFormat {
    pub const Jpeg: Self = Self(0i32);
    pub const Png: Self = Self(1i32);
    pub const JpegXR: Self = Self(2i32);
}
impl ::core::marker::Copy for CameraCaptureUIPhotoFormat {}
impl ::core::clone::Clone for CameraCaptureUIPhotoFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CameraCaptureUIPhotoFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CameraCaptureUIPhotoFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for CameraCaptureUIPhotoFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraCaptureUIPhotoFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CameraCaptureUIPhotoFormat {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.CameraCaptureUIPhotoFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct CameraCaptureUIVideoCaptureSettings(::windows_core::IUnknown);
impl CameraCaptureUIVideoCaptureSettings {
    pub fn Format(&self) -> ::windows_core::Result<CameraCaptureUIVideoFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CameraCaptureUIVideoFormat>::zeroed();
            (::windows_core::Interface::vtable(this).Format)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CameraCaptureUIVideoFormat>(result__)
        }
    }
    pub fn SetFormat(&self, value: CameraCaptureUIVideoFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxResolution(&self) -> ::windows_core::Result<CameraCaptureUIMaxVideoResolution> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CameraCaptureUIMaxVideoResolution>::zeroed();
            (::windows_core::Interface::vtable(this).MaxResolution)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CameraCaptureUIMaxVideoResolution>(result__)
        }
    }
    pub fn SetMaxResolution(&self, value: CameraCaptureUIMaxVideoResolution) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxResolution)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxDurationInSeconds(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxDurationInSeconds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetMaxDurationInSeconds(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxDurationInSeconds)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowTrimming(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AllowTrimming)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowTrimming(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowTrimming)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for CameraCaptureUIVideoCaptureSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CameraCaptureUIVideoCaptureSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CameraCaptureUIVideoCaptureSettings {}
impl ::core::fmt::Debug for CameraCaptureUIVideoCaptureSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraCaptureUIVideoCaptureSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CameraCaptureUIVideoCaptureSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.CameraCaptureUIVideoCaptureSettings;{64e92d1f-a28d-425a-b84f-e568335ff24e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CameraCaptureUIVideoCaptureSettings {
    type Vtable = ICameraCaptureUIVideoCaptureSettings_Vtbl;
    const IID: ::windows_core::GUID = <ICameraCaptureUIVideoCaptureSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CameraCaptureUIVideoCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.CameraCaptureUIVideoCaptureSettings";
}
impl ::core::convert::From<CameraCaptureUIVideoCaptureSettings> for ::windows_core::IUnknown {
    fn from(value: CameraCaptureUIVideoCaptureSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraCaptureUIVideoCaptureSettings> for ::windows_core::IUnknown {
    fn from(value: &CameraCaptureUIVideoCaptureSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CameraCaptureUIVideoCaptureSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CameraCaptureUIVideoCaptureSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CameraCaptureUIVideoCaptureSettings> for ::windows_core::IInspectable {
    fn from(value: CameraCaptureUIVideoCaptureSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraCaptureUIVideoCaptureSettings> for ::windows_core::IInspectable {
    fn from(value: &CameraCaptureUIVideoCaptureSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CameraCaptureUIVideoCaptureSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CameraCaptureUIVideoCaptureSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CameraCaptureUIVideoCaptureSettings {}
unsafe impl ::core::marker::Sync for CameraCaptureUIVideoCaptureSettings {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CameraCaptureUIVideoFormat(pub i32);
impl CameraCaptureUIVideoFormat {
    pub const Mp4: Self = Self(0i32);
    pub const Wmv: Self = Self(1i32);
}
impl ::core::marker::Copy for CameraCaptureUIVideoFormat {}
impl ::core::clone::Clone for CameraCaptureUIVideoFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CameraCaptureUIVideoFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CameraCaptureUIVideoFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for CameraCaptureUIVideoFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraCaptureUIVideoFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CameraCaptureUIVideoFormat {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.CameraCaptureUIVideoFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct CameraOptionsUI;
impl CameraOptionsUI {
    pub fn Show<'a, Param0: ::windows_core::IntoParam<'a, MediaCapture>>(mediacapture: Param0) -> ::windows_core::Result<()> {
        Self::ICameraOptionsUIStatics(|this| unsafe { (::windows_core::Interface::vtable(this).Show)(::windows_core::Interface::as_raw(this), mediacapture.into_param().abi()).ok() })
    }
    pub fn ICameraOptionsUIStatics<R, F: FnOnce(&ICameraOptionsUIStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CameraOptionsUI, ICameraOptionsUIStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for CameraOptionsUI {
    const NAME: &'static str = "Windows.Media.Capture.CameraOptionsUI";
}
#[repr(transparent)]
pub struct CapturedFrame(::windows_core::IUnknown);
impl CapturedFrame {
    pub fn Width(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Width)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Height(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Height)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn ControlValues(&self) -> ::windows_core::Result<CapturedFrameControlValues> {
        let this = &::windows_core::Interface::cast::<ICapturedFrame2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ControlValues)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CapturedFrameControlValues>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub fn BitmapProperties(&self) -> ::windows_core::Result<::winrt_graphics::Imaging::BitmapPropertySet> {
        let this = &::windows_core::Interface::cast::<ICapturedFrame2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BitmapProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::Imaging::BitmapPropertySet>(result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SoftwareBitmap(&self) -> ::windows_core::Result<::winrt_graphics::Imaging::SoftwareBitmap> {
        let this = &::windows_core::Interface::cast::<ICapturedFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SoftwareBitmap)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::Imaging::SoftwareBitmap>(result__)
        }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ContentType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IContentTypeProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ContentType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ReadAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, buffer: Param0, count: u32, options: ::winrt_storage::Streams::InputStreamOptions) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IBuffer, u32>> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IInputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsync)(::windows_core::Interface::as_raw(this), buffer.into_param().abi(), count, options, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IBuffer, u32>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn WriteAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, buffer: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WriteAsync)(::windows_core::Interface::as_raw(this), buffer.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn FlushAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FlushAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Size(&self) -> ::windows_core::Result<u64> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSize(&self, value: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows_core::Result<::winrt_storage::Streams::IInputStream> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetInputStreamAt)(::windows_core::Interface::as_raw(this), position, result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IInputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows_core::Result<::winrt_storage::Streams::IOutputStream> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetOutputStreamAt)(::windows_core::Interface::as_raw(this), position, result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IOutputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Position(&self) -> ::windows_core::Result<u64> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Seek(&self, position: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Seek)(::windows_core::Interface::as_raw(this), position).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CloneStream(&self) -> ::windows_core::Result<::winrt_storage::Streams::IRandomAccessStream> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CloneStream)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IRandomAccessStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CanRead(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanRead)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CanWrite(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanWrite)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for CapturedFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CapturedFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CapturedFrame {}
impl ::core::fmt::Debug for CapturedFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CapturedFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CapturedFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.CapturedFrame;{1dd2de1f-571b-44d8-8e80-a08a1578766e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CapturedFrame {
    type Vtable = ICapturedFrame_Vtbl;
    const IID: ::windows_core::GUID = <ICapturedFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CapturedFrame {
    const NAME: &'static str = "Windows.Media.Capture.CapturedFrame";
}
impl ::core::convert::From<CapturedFrame> for ::windows_core::IUnknown {
    fn from(value: CapturedFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CapturedFrame> for ::windows_core::IUnknown {
    fn from(value: &CapturedFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CapturedFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CapturedFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CapturedFrame> for ::windows_core::IInspectable {
    fn from(value: CapturedFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CapturedFrame> for ::windows_core::IInspectable {
    fn from(value: &CapturedFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CapturedFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CapturedFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CapturedFrame> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: CapturedFrame) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CapturedFrame> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &CapturedFrame) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for CapturedFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &CapturedFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<CapturedFrame> for ::winrt_storage::Streams::IContentTypeProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: CapturedFrame) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&CapturedFrame> for ::winrt_storage::Streams::IContentTypeProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: &CapturedFrame) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IContentTypeProvider> for CapturedFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IContentTypeProvider> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IContentTypeProvider> for &CapturedFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IContentTypeProvider> {
        ::core::convert::TryInto::<::winrt_storage::Streams::IContentTypeProvider>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<CapturedFrame> for ::winrt_storage::Streams::IInputStream {
    type Error = ::windows_core::Error;
    fn try_from(value: CapturedFrame) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&CapturedFrame> for ::winrt_storage::Streams::IInputStream {
    type Error = ::windows_core::Error;
    fn try_from(value: &CapturedFrame) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream> for CapturedFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IInputStream> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream> for &CapturedFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IInputStream> {
        ::core::convert::TryInto::<::winrt_storage::Streams::IInputStream>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<CapturedFrame> for ::winrt_storage::Streams::IOutputStream {
    type Error = ::windows_core::Error;
    fn try_from(value: CapturedFrame) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&CapturedFrame> for ::winrt_storage::Streams::IOutputStream {
    type Error = ::windows_core::Error;
    fn try_from(value: &CapturedFrame) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IOutputStream> for CapturedFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IOutputStream> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IOutputStream> for &CapturedFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IOutputStream> {
        ::core::convert::TryInto::<::winrt_storage::Streams::IOutputStream>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<CapturedFrame> for ::winrt_storage::Streams::IRandomAccessStream {
    type Error = ::windows_core::Error;
    fn try_from(value: CapturedFrame) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&CapturedFrame> for ::winrt_storage::Streams::IRandomAccessStream {
    type Error = ::windows_core::Error;
    fn try_from(value: &CapturedFrame) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream> for CapturedFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IRandomAccessStream> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream> for &CapturedFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IRandomAccessStream> {
        ::core::convert::TryInto::<::winrt_storage::Streams::IRandomAccessStream>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<CapturedFrame> for ::winrt_storage::Streams::IRandomAccessStreamWithContentType {
    type Error = ::windows_core::Error;
    fn try_from(value: CapturedFrame) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&CapturedFrame> for ::winrt_storage::Streams::IRandomAccessStreamWithContentType {
    type Error = ::windows_core::Error;
    fn try_from(value: &CapturedFrame) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStreamWithContentType> for CapturedFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IRandomAccessStreamWithContentType> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStreamWithContentType> for &CapturedFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IRandomAccessStreamWithContentType> {
        ::core::convert::TryInto::<::winrt_storage::Streams::IRandomAccessStreamWithContentType>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CapturedFrame {}
unsafe impl ::core::marker::Sync for CapturedFrame {}
#[repr(transparent)]
pub struct CapturedFrameControlValues(::windows_core::IUnknown);
impl CapturedFrameControlValues {
    pub fn Exposure(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Exposure)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn ExposureCompensation(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExposureCompensation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f32>>(result__)
        }
    }
    pub fn IsoSpeed(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsoSpeed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u32>>(result__)
        }
    }
    pub fn Focus(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Focus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Media_Devices")]
    pub fn SceneMode(&self) -> ::windows_core::Result<::winrt_foundation::IReference<super::Devices::CaptureSceneMode>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SceneMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<super::Devices::CaptureSceneMode>>(result__)
        }
    }
    pub fn Flashed(&self) -> ::windows_core::Result<::winrt_foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Flashed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<bool>>(result__)
        }
    }
    pub fn FlashPowerPercent(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FlashPowerPercent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f32>>(result__)
        }
    }
    pub fn WhiteBalance(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WhiteBalance)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u32>>(result__)
        }
    }
    pub fn ZoomFactor(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ZoomFactor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f32>>(result__)
        }
    }
    #[cfg(feature = "Media_Devices")]
    pub fn FocusState(&self) -> ::windows_core::Result<::winrt_foundation::IReference<super::Devices::MediaCaptureFocusState>> {
        let this = &::windows_core::Interface::cast::<ICapturedFrameControlValues2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FocusState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<super::Devices::MediaCaptureFocusState>>(result__)
        }
    }
    pub fn IsoDigitalGain(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f64>> {
        let this = &::windows_core::Interface::cast::<ICapturedFrameControlValues2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsoDigitalGain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f64>>(result__)
        }
    }
    pub fn IsoAnalogGain(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f64>> {
        let this = &::windows_core::Interface::cast::<ICapturedFrameControlValues2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsoAnalogGain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SensorFrameRate(&self) -> ::windows_core::Result<super::MediaProperties::MediaRatio> {
        let this = &::windows_core::Interface::cast::<ICapturedFrameControlValues2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SensorFrameRate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::MediaRatio>(result__)
        }
    }
    pub fn WhiteBalanceGain(&self) -> ::windows_core::Result<::winrt_foundation::IReference<WhiteBalanceGain>> {
        let this = &::windows_core::Interface::cast::<ICapturedFrameControlValues2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WhiteBalanceGain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<WhiteBalanceGain>>(result__)
        }
    }
}
impl ::core::clone::Clone for CapturedFrameControlValues {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CapturedFrameControlValues {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CapturedFrameControlValues {}
impl ::core::fmt::Debug for CapturedFrameControlValues {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CapturedFrameControlValues").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CapturedFrameControlValues {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.CapturedFrameControlValues;{90c65b7f-4e0d-4ca4-882d-7a144fed0a90})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CapturedFrameControlValues {
    type Vtable = ICapturedFrameControlValues_Vtbl;
    const IID: ::windows_core::GUID = <ICapturedFrameControlValues as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CapturedFrameControlValues {
    const NAME: &'static str = "Windows.Media.Capture.CapturedFrameControlValues";
}
impl ::core::convert::From<CapturedFrameControlValues> for ::windows_core::IUnknown {
    fn from(value: CapturedFrameControlValues) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CapturedFrameControlValues> for ::windows_core::IUnknown {
    fn from(value: &CapturedFrameControlValues) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CapturedFrameControlValues {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CapturedFrameControlValues {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CapturedFrameControlValues> for ::windows_core::IInspectable {
    fn from(value: CapturedFrameControlValues) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CapturedFrameControlValues> for ::windows_core::IInspectable {
    fn from(value: &CapturedFrameControlValues) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CapturedFrameControlValues {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CapturedFrameControlValues {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CapturedFrameControlValues {}
unsafe impl ::core::marker::Sync for CapturedFrameControlValues {}
#[repr(transparent)]
pub struct CapturedPhoto(::windows_core::IUnknown);
impl CapturedPhoto {
    pub fn Frame(&self) -> ::windows_core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Frame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CapturedFrame>(result__)
        }
    }
    pub fn Thumbnail(&self) -> ::windows_core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Thumbnail)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CapturedFrame>(result__)
        }
    }
}
impl ::core::clone::Clone for CapturedPhoto {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CapturedPhoto {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CapturedPhoto {}
impl ::core::fmt::Debug for CapturedPhoto {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CapturedPhoto").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CapturedPhoto {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.CapturedPhoto;{b0ce7e5a-cfcc-4d6c-8ad1-0869208aca16})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CapturedPhoto {
    type Vtable = ICapturedPhoto_Vtbl;
    const IID: ::windows_core::GUID = <ICapturedPhoto as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CapturedPhoto {
    const NAME: &'static str = "Windows.Media.Capture.CapturedPhoto";
}
impl ::core::convert::From<CapturedPhoto> for ::windows_core::IUnknown {
    fn from(value: CapturedPhoto) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CapturedPhoto> for ::windows_core::IUnknown {
    fn from(value: &CapturedPhoto) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CapturedPhoto {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CapturedPhoto {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CapturedPhoto> for ::windows_core::IInspectable {
    fn from(value: CapturedPhoto) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CapturedPhoto> for ::windows_core::IInspectable {
    fn from(value: &CapturedPhoto) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CapturedPhoto {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CapturedPhoto {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CapturedPhoto {}
unsafe impl ::core::marker::Sync for CapturedPhoto {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ForegroundActivationArgument(pub i32);
impl ForegroundActivationArgument {
    pub const SignInRequired: Self = Self(0i32);
    pub const MoreSettings: Self = Self(1i32);
}
impl ::core::marker::Copy for ForegroundActivationArgument {}
impl ::core::clone::Clone for ForegroundActivationArgument {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ForegroundActivationArgument {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ForegroundActivationArgument {
    type Abi = Self;
}
impl ::core::fmt::Debug for ForegroundActivationArgument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ForegroundActivationArgument").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ForegroundActivationArgument {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.ForegroundActivationArgument;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GameBarCommand(pub i32);
impl GameBarCommand {
    pub const OpenGameBar: Self = Self(0i32);
    pub const RecordHistoricalBuffer: Self = Self(1i32);
    pub const ToggleStartStopRecord: Self = Self(2i32);
    pub const StartRecord: Self = Self(3i32);
    pub const StopRecord: Self = Self(4i32);
    pub const TakeScreenshot: Self = Self(5i32);
    pub const StartBroadcast: Self = Self(6i32);
    pub const StopBroadcast: Self = Self(7i32);
    pub const PauseBroadcast: Self = Self(8i32);
    pub const ResumeBroadcast: Self = Self(9i32);
    pub const ToggleStartStopBroadcast: Self = Self(10i32);
    pub const ToggleMicrophoneCapture: Self = Self(11i32);
    pub const ToggleCameraCapture: Self = Self(12i32);
    pub const ToggleRecordingIndicator: Self = Self(13i32);
}
impl ::core::marker::Copy for GameBarCommand {}
impl ::core::clone::Clone for GameBarCommand {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GameBarCommand {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GameBarCommand {
    type Abi = Self;
}
impl ::core::fmt::Debug for GameBarCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameBarCommand").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GameBarCommand {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.GameBarCommand;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GameBarCommandOrigin(pub i32);
impl GameBarCommandOrigin {
    pub const ShortcutKey: Self = Self(0i32);
    pub const Cortana: Self = Self(1i32);
    pub const AppCommand: Self = Self(2i32);
}
impl ::core::marker::Copy for GameBarCommandOrigin {}
impl ::core::clone::Clone for GameBarCommandOrigin {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GameBarCommandOrigin {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GameBarCommandOrigin {
    type Abi = Self;
}
impl ::core::fmt::Debug for GameBarCommandOrigin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameBarCommandOrigin").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GameBarCommandOrigin {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.GameBarCommandOrigin;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct GameBarServices(::windows_core::IUnknown);
impl GameBarServices {
    pub fn TargetCapturePolicy(&self) -> ::windows_core::Result<GameBarTargetCapturePolicy> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GameBarTargetCapturePolicy>::zeroed();
            (::windows_core::Interface::vtable(this).TargetCapturePolicy)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameBarTargetCapturePolicy>(result__)
        }
    }
    pub fn EnableCapture(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).EnableCapture)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn DisableCapture(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).DisableCapture)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn TargetInfo(&self) -> ::windows_core::Result<GameBarServicesTargetInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameBarServicesTargetInfo>(result__)
        }
    }
    pub fn SessionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SessionId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AppBroadcastServices(&self) -> ::windows_core::Result<AppBroadcastServices> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppBroadcastServices)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastServices>(result__)
        }
    }
    pub fn AppCaptureServices(&self) -> ::windows_core::Result<AppCaptureServices> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppCaptureServices)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppCaptureServices>(result__)
        }
    }
    pub fn CommandReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GameBarServices, GameBarServicesCommandEventArgs>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CommandReceived)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCommandReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCommandReceived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for GameBarServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameBarServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameBarServices {}
impl ::core::fmt::Debug for GameBarServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameBarServices").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GameBarServices {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.GameBarServices;{2dbead57-50a6-499e-8c6c-d330a7311796})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GameBarServices {
    type Vtable = IGameBarServices_Vtbl;
    const IID: ::windows_core::GUID = <IGameBarServices as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GameBarServices {
    const NAME: &'static str = "Windows.Media.Capture.GameBarServices";
}
impl ::core::convert::From<GameBarServices> for ::windows_core::IUnknown {
    fn from(value: GameBarServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameBarServices> for ::windows_core::IUnknown {
    fn from(value: &GameBarServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GameBarServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GameBarServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GameBarServices> for ::windows_core::IInspectable {
    fn from(value: GameBarServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameBarServices> for ::windows_core::IInspectable {
    fn from(value: &GameBarServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GameBarServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GameBarServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GameBarServices {}
unsafe impl ::core::marker::Sync for GameBarServices {}
#[repr(transparent)]
pub struct GameBarServicesCommandEventArgs(::windows_core::IUnknown);
impl GameBarServicesCommandEventArgs {
    pub fn Command(&self) -> ::windows_core::Result<GameBarCommand> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GameBarCommand>::zeroed();
            (::windows_core::Interface::vtable(this).Command)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameBarCommand>(result__)
        }
    }
    pub fn Origin(&self) -> ::windows_core::Result<GameBarCommandOrigin> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GameBarCommandOrigin>::zeroed();
            (::windows_core::Interface::vtable(this).Origin)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameBarCommandOrigin>(result__)
        }
    }
}
impl ::core::clone::Clone for GameBarServicesCommandEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameBarServicesCommandEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameBarServicesCommandEventArgs {}
impl ::core::fmt::Debug for GameBarServicesCommandEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameBarServicesCommandEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GameBarServicesCommandEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.GameBarServicesCommandEventArgs;{a74226b2-f176-4fcf-8fbb-cf698b2eb8e0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GameBarServicesCommandEventArgs {
    type Vtable = IGameBarServicesCommandEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IGameBarServicesCommandEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GameBarServicesCommandEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.GameBarServicesCommandEventArgs";
}
impl ::core::convert::From<GameBarServicesCommandEventArgs> for ::windows_core::IUnknown {
    fn from(value: GameBarServicesCommandEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameBarServicesCommandEventArgs> for ::windows_core::IUnknown {
    fn from(value: &GameBarServicesCommandEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GameBarServicesCommandEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GameBarServicesCommandEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GameBarServicesCommandEventArgs> for ::windows_core::IInspectable {
    fn from(value: GameBarServicesCommandEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameBarServicesCommandEventArgs> for ::windows_core::IInspectable {
    fn from(value: &GameBarServicesCommandEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GameBarServicesCommandEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GameBarServicesCommandEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GameBarServicesCommandEventArgs {}
unsafe impl ::core::marker::Sync for GameBarServicesCommandEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GameBarServicesDisplayMode(pub i32);
impl GameBarServicesDisplayMode {
    pub const Windowed: Self = Self(0i32);
    pub const FullScreenExclusive: Self = Self(1i32);
}
impl ::core::marker::Copy for GameBarServicesDisplayMode {}
impl ::core::clone::Clone for GameBarServicesDisplayMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GameBarServicesDisplayMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GameBarServicesDisplayMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for GameBarServicesDisplayMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameBarServicesDisplayMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GameBarServicesDisplayMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.GameBarServicesDisplayMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct GameBarServicesManager(::windows_core::IUnknown);
impl GameBarServicesManager {
    pub fn GameBarServicesCreated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GameBarServicesManager, GameBarServicesManagerGameBarServicesCreatedEventArgs>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).GameBarServicesCreated)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveGameBarServicesCreated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveGameBarServicesCreated)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetDefault() -> ::windows_core::Result<GameBarServicesManager> {
        Self::IGameBarServicesManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameBarServicesManager>(result__)
        })
    }
    pub fn IGameBarServicesManagerStatics<R, F: FnOnce(&IGameBarServicesManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GameBarServicesManager, IGameBarServicesManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GameBarServicesManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameBarServicesManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameBarServicesManager {}
impl ::core::fmt::Debug for GameBarServicesManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameBarServicesManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GameBarServicesManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.GameBarServicesManager;{3a4b9cfa-7f8b-4c60-9dbb-0bcd262dffc6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GameBarServicesManager {
    type Vtable = IGameBarServicesManager_Vtbl;
    const IID: ::windows_core::GUID = <IGameBarServicesManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GameBarServicesManager {
    const NAME: &'static str = "Windows.Media.Capture.GameBarServicesManager";
}
impl ::core::convert::From<GameBarServicesManager> for ::windows_core::IUnknown {
    fn from(value: GameBarServicesManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameBarServicesManager> for ::windows_core::IUnknown {
    fn from(value: &GameBarServicesManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GameBarServicesManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GameBarServicesManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GameBarServicesManager> for ::windows_core::IInspectable {
    fn from(value: GameBarServicesManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameBarServicesManager> for ::windows_core::IInspectable {
    fn from(value: &GameBarServicesManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GameBarServicesManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GameBarServicesManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GameBarServicesManager {}
unsafe impl ::core::marker::Sync for GameBarServicesManager {}
#[repr(transparent)]
pub struct GameBarServicesManagerGameBarServicesCreatedEventArgs(::windows_core::IUnknown);
impl GameBarServicesManagerGameBarServicesCreatedEventArgs {
    pub fn GameBarServices(&self) -> ::windows_core::Result<GameBarServices> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GameBarServices)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameBarServices>(result__)
        }
    }
}
impl ::core::clone::Clone for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameBarServicesManagerGameBarServicesCreatedEventArgs {}
impl ::core::fmt::Debug for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameBarServicesManagerGameBarServicesCreatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.GameBarServicesManagerGameBarServicesCreatedEventArgs;{ededbd9c-143e-49a3-a5ea-0b1995c8d46e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    type Vtable = IGameBarServicesManagerGameBarServicesCreatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IGameBarServicesManagerGameBarServicesCreatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.GameBarServicesManagerGameBarServicesCreatedEventArgs";
}
impl ::core::convert::From<GameBarServicesManagerGameBarServicesCreatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: GameBarServicesManagerGameBarServicesCreatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameBarServicesManagerGameBarServicesCreatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &GameBarServicesManagerGameBarServicesCreatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GameBarServicesManagerGameBarServicesCreatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GameBarServicesManagerGameBarServicesCreatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: GameBarServicesManagerGameBarServicesCreatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameBarServicesManagerGameBarServicesCreatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &GameBarServicesManagerGameBarServicesCreatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GameBarServicesManagerGameBarServicesCreatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GameBarServicesManagerGameBarServicesCreatedEventArgs {}
unsafe impl ::core::marker::Sync for GameBarServicesManagerGameBarServicesCreatedEventArgs {}
#[repr(transparent)]
pub struct GameBarServicesTargetInfo(::windows_core::IUnknown);
impl GameBarServicesTargetInfo {
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AppId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn TitleId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TitleId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DisplayMode(&self) -> ::windows_core::Result<GameBarServicesDisplayMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GameBarServicesDisplayMode>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameBarServicesDisplayMode>(result__)
        }
    }
}
impl ::core::clone::Clone for GameBarServicesTargetInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameBarServicesTargetInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameBarServicesTargetInfo {}
impl ::core::fmt::Debug for GameBarServicesTargetInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameBarServicesTargetInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GameBarServicesTargetInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.GameBarServicesTargetInfo;{b4202f92-1611-4e05-b6ef-dfd737ae33b0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GameBarServicesTargetInfo {
    type Vtable = IGameBarServicesTargetInfo_Vtbl;
    const IID: ::windows_core::GUID = <IGameBarServicesTargetInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GameBarServicesTargetInfo {
    const NAME: &'static str = "Windows.Media.Capture.GameBarServicesTargetInfo";
}
impl ::core::convert::From<GameBarServicesTargetInfo> for ::windows_core::IUnknown {
    fn from(value: GameBarServicesTargetInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameBarServicesTargetInfo> for ::windows_core::IUnknown {
    fn from(value: &GameBarServicesTargetInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GameBarServicesTargetInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GameBarServicesTargetInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GameBarServicesTargetInfo> for ::windows_core::IInspectable {
    fn from(value: GameBarServicesTargetInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameBarServicesTargetInfo> for ::windows_core::IInspectable {
    fn from(value: &GameBarServicesTargetInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GameBarServicesTargetInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GameBarServicesTargetInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GameBarServicesTargetInfo {}
unsafe impl ::core::marker::Sync for GameBarServicesTargetInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GameBarTargetCapturePolicy(pub i32);
impl GameBarTargetCapturePolicy {
    pub const EnabledBySystem: Self = Self(0i32);
    pub const EnabledByUser: Self = Self(1i32);
    pub const NotEnabled: Self = Self(2i32);
    pub const ProhibitedBySystem: Self = Self(3i32);
    pub const ProhibitedByPublisher: Self = Self(4i32);
}
impl ::core::marker::Copy for GameBarTargetCapturePolicy {}
impl ::core::clone::Clone for GameBarTargetCapturePolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GameBarTargetCapturePolicy {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GameBarTargetCapturePolicy {
    type Abi = Self;
}
impl ::core::fmt::Debug for GameBarTargetCapturePolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameBarTargetCapturePolicy").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GameBarTargetCapturePolicy {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.GameBarTargetCapturePolicy;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedCapturedPhoto(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdvancedCapturedPhoto {
    type Vtable = IAdvancedCapturedPhoto_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf072728b_b292_4491_9d41_99807a550bbf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedCapturedPhoto_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Frame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Media_Devices")]
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Devices::AdvancedPhotoMode) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    Mode: usize,
    pub Context: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedCapturedPhoto2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdvancedCapturedPhoto2 {
    type Vtable = IAdvancedCapturedPhoto2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x18cf6cd8_cffe_42d8_8104_017bb318f4a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedCapturedPhoto2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FrameBoundsRelativeToReferencePhoto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedPhotoCapture(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdvancedPhotoCapture {
    type Vtable = IAdvancedPhotoCapture_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x83ffaafa_6667_44dc_973c_a6bce596aa0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedPhotoCapture_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CaptureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CaptureWithContextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OptionalReferencePhotoCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveOptionalReferencePhotoCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub AllPhotosCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveAllPhotosCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub FinishAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastBackgroundService(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastBackgroundService {
    type Vtable = IAppBroadcastBackgroundService_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbad1e72a_fa94_46f9_95fc_d71511cda70b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundService_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetPlugInState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppBroadcastPlugInState) -> ::windows_core::HRESULT,
    pub PlugInState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastPlugInState) -> ::windows_core::HRESULT,
    pub SetSignInInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SignInInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetStreamInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StreamInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BroadcastTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetViewerCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ViewerCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub TerminateBroadcast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: AppBroadcastTerminationReason, providerspecificreason: u32) -> ::windows_core::HRESULT,
    pub HeartbeatRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveHeartbeatRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub TitleId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastBackgroundService2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastBackgroundService2 {
    type Vtable = IAppBroadcastBackgroundService2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfc8ccbbf_5549_4b87_959f_23ca401fd473);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundService2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetBroadcastTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BroadcastLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetBroadcastLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BroadcastChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetBroadcastChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BroadcastTitleChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveBroadcastTitleChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub BroadcastLanguageChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveBroadcastLanguageChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub BroadcastChannelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveBroadcastChannelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastBackgroundServiceSignInInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastBackgroundServiceSignInInfo {
    type Vtable = IAppBroadcastBackgroundServiceSignInInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e735275_88c8_4eca_89ba_4825985db880);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundServiceSignInInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SignInState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastSignInState) -> ::windows_core::HRESULT,
    pub SetOAuthRequestUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OAuthRequestUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetOAuthCallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OAuthCallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Security_Authentication_Web")]
    pub AuthenticationResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))]
    AuthenticationResult: usize,
    pub SetUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub UserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SignInStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSignInStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastBackgroundServiceSignInInfo2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastBackgroundServiceSignInInfo2 {
    type Vtable = IAppBroadcastBackgroundServiceSignInInfo2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9104285c_62cf_4a3c_a7ee_aeb507404645);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundServiceSignInInfo2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub UserNameChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveUserNameChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastBackgroundServiceStreamInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastBackgroundServiceStreamInfo {
    type Vtable = IAppBroadcastBackgroundServiceStreamInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x31dc02bc_990a_4904_aa96_fe364381f136);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundServiceStreamInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StreamState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastStreamState) -> ::windows_core::HRESULT,
    pub SetDesiredVideoEncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64) -> ::windows_core::HRESULT,
    pub DesiredVideoEncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub SetBandwidthTestBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64) -> ::windows_core::HRESULT,
    pub BandwidthTestBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub SetAudioCodec: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioCodec: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BroadcastStreamReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StreamStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStreamStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub VideoEncodingResolutionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveVideoEncodingResolutionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub VideoEncodingBitrateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveVideoEncodingBitrateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastBackgroundServiceStreamInfo2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastBackgroundServiceStreamInfo2 {
    type Vtable = IAppBroadcastBackgroundServiceStreamInfo2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbd1e9f6d_94dc_4fce_9541_a9f129596334);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundServiceStreamInfo2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReportProblemWithStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastCameraCaptureStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastCameraCaptureStateChangedEventArgs {
    type Vtable = IAppBroadcastCameraCaptureStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1e334cd0_b882_4b88_8692_05999aceb70f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastCameraCaptureStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastCameraCaptureState) -> ::windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastGlobalSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastGlobalSettings {
    type Vtable = IAppBroadcastGlobalSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb2cb27a5_70fc_4e17_80bd_6ba0fd3ff3a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastGlobalSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsBroadcastEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsDisabledByPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsGpuConstrained: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HasHardwareEncoder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsAudioCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsAudioCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsMicrophoneCaptureEnabledByDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsMicrophoneCaptureEnabledByDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsEchoCancellationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsEchoCancellationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetSystemAudioGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub SystemAudioGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetMicrophoneGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub MicrophoneGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetIsCameraCaptureEnabledByDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsCameraCaptureEnabledByDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetSelectedCameraId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SelectedCameraId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCameraOverlayLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppBroadcastCameraOverlayLocation) -> ::windows_core::HRESULT,
    pub CameraOverlayLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastCameraOverlayLocation) -> ::windows_core::HRESULT,
    pub SetCameraOverlaySize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppBroadcastCameraOverlaySize) -> ::windows_core::HRESULT,
    pub CameraOverlaySize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastCameraOverlaySize) -> ::windows_core::HRESULT,
    pub SetIsCursorImageCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsCursorImageCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastHeartbeatRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastHeartbeatRequestedEventArgs {
    type Vtable = IAppBroadcastHeartbeatRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcea54283_ee51_4dbf_9472_79a9ed4e2165);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastHeartbeatRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastManagerStatics {
    type Vtable = IAppBroadcastManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x364e018b_1e4e_411f_ab3e_92959844c156);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetGlobalSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ApplyGlobalSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetProviderSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ApplyProviderSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastMicrophoneCaptureStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastMicrophoneCaptureStateChangedEventArgs {
    type Vtable = IAppBroadcastMicrophoneCaptureStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa86ad5e9_9440_4908_9d09_65b7e315d795);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastMicrophoneCaptureStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastMicrophoneCaptureState) -> ::windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastPlugIn(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastPlugIn {
    type Vtable = IAppBroadcastPlugIn_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x520c1e66_6513_4574_ac54_23b79729615b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPlugIn_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ProviderSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Logo: usize,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastPlugInManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastPlugInManager {
    type Vtable = IAppBroadcastPlugInManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe550d979_27a1_49a7_bbf4_d7a9e9d07668);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPlugInManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsBroadcastProviderAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PlugInList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PlugInList: usize,
    pub DefaultPlugIn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetDefaultPlugIn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastPlugInManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastPlugInManagerStatics {
    type Vtable = IAppBroadcastPlugInManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf2645c20_5c76_4cdc_9364_82fe9eb6534d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPlugInManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastPlugInStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastPlugInStateChangedEventArgs {
    type Vtable = IAppBroadcastPlugInStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4881d0f2_abc5_4fc6_84b0_89370bb47212);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPlugInStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PlugInState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastPlugInState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastPreview(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastPreview {
    type Vtable = IAppBroadcastPreview_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x14b60f5a_6e4a_4b80_a14f_67ee77d153e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPreview_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StopPreview: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PreviewState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastPreviewState) -> ::windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PreviewStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePreviewStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub PreviewStreamReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastPreviewStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastPreviewStateChangedEventArgs {
    type Vtable = IAppBroadcastPreviewStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5a57f2de_8dea_4e86_90ad_03fc26b9653c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPreviewStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PreviewState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastPreviewState) -> ::windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastPreviewStreamReader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastPreviewStreamReader {
    type Vtable = IAppBroadcastPreviewStreamReader_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x92228d50_db3f_40a8_8cd4_f4e371ddab37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPreviewStreamReader_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub VideoWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub VideoHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub VideoStride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")]
    pub VideoBitmapPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_graphics::Imaging::BitmapPixelFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    VideoBitmapPixelFormat: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub VideoBitmapAlphaMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_graphics::Imaging::BitmapAlphaMode) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    VideoBitmapAlphaMode: usize,
    pub TryGetNextVideoFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub VideoFrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveVideoFrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastPreviewStreamVideoFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastPreviewStreamVideoFrame {
    type Vtable = IAppBroadcastPreviewStreamVideoFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x010fbea1_94fe_4499_b8c0_8d244279fb12);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPreviewStreamVideoFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub VideoHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub VideoBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    VideoBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastPreviewStreamVideoHeader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastPreviewStreamVideoHeader {
    type Vtable = IAppBroadcastPreviewStreamVideoHeader_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8bef6113_da84_4499_a7ab_87118cb4a157);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPreviewStreamVideoHeader_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AbsoluteTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub RelativeTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub FrameId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastProviderSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastProviderSettings {
    type Vtable = IAppBroadcastProviderSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc30bdf62_9948_458f_ad50_aa06ec03da08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastProviderSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetDefaultBroadcastTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DefaultBroadcastTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAudioEncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub AudioEncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetCustomVideoEncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub CustomVideoEncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetCustomVideoEncodingHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub CustomVideoEncodingHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetCustomVideoEncodingWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub CustomVideoEncodingWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetVideoEncodingBitrateMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppBroadcastVideoEncodingBitrateMode) -> ::windows_core::HRESULT,
    pub VideoEncodingBitrateMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastVideoEncodingBitrateMode) -> ::windows_core::HRESULT,
    pub SetVideoEncodingResolutionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppBroadcastVideoEncodingResolutionMode) -> ::windows_core::HRESULT,
    pub VideoEncodingResolutionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastVideoEncodingResolutionMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastServices(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastServices {
    type Vtable = IAppBroadcastServices_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8660b4d6_969b_4e3c_ac3a_8b042ee4ee63);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastServices_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CaptureTargetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastCaptureTargetType) -> ::windows_core::HRESULT,
    pub SetCaptureTargetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppBroadcastCaptureTargetType) -> ::windows_core::HRESULT,
    pub BroadcastTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetBroadcastTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BroadcastLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetBroadcastLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub UserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CanCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub EnterBroadcastModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plugin: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExitBroadcastMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: AppBroadcastExitBroadcastModeReason) -> ::windows_core::HRESULT,
    pub StartBroadcast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PauseBroadcast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ResumeBroadcast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StartPreview: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredsize: ::winrt_foundation::Size, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastSignInStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastSignInStateChangedEventArgs {
    type Vtable = IAppBroadcastSignInStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x02b692a4_5919_4a9e_8d5e_c9bb0dd3377a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastSignInStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SignInState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastSignInState) -> ::windows_core::HRESULT,
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastSignInResult) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastState(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastState {
    type Vtable = IAppBroadcastState_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xee08056d_8099_4ddd_922e_c56dac58abfb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastState_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsCaptureTargetRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ViewerCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub ShouldCaptureMicrophone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetShouldCaptureMicrophone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub RestartMicrophoneCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ShouldCaptureCamera: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetShouldCaptureCamera: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub RestartCameraCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EncodedVideoSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Size) -> ::windows_core::HRESULT,
    pub MicrophoneCaptureState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastMicrophoneCaptureState) -> ::windows_core::HRESULT,
    pub MicrophoneCaptureError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub CameraCaptureState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastCameraCaptureState) -> ::windows_core::HRESULT,
    pub CameraCaptureError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub StreamState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastStreamState) -> ::windows_core::HRESULT,
    pub PlugInState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastPlugInState) -> ::windows_core::HRESULT,
    pub OAuthRequestUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OAuthCallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Security_Authentication_Web")]
    pub AuthenticationResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))]
    AuthenticationResult: usize,
    #[cfg(feature = "Security_Authentication_Web")]
    pub SetAuthenticationResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))]
    SetAuthenticationResult: usize,
    pub SetSignInState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppBroadcastSignInState) -> ::windows_core::HRESULT,
    pub SignInState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastSignInState) -> ::windows_core::HRESULT,
    pub TerminationReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastTerminationReason) -> ::windows_core::HRESULT,
    pub TerminationReasonPlugInSpecific: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub ViewerCountChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveViewerCountChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub MicrophoneCaptureStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveMicrophoneCaptureStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub CameraCaptureStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCameraCaptureStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub PlugInStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePlugInStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub StreamStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStreamStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub CaptureTargetClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCaptureTargetClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastStreamAudioFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastStreamAudioFrame {
    type Vtable = IAppBroadcastStreamAudioFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xefab4ac8_21ba_453f_8bb7_5e938a2e9a74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastStreamAudioFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AudioHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub AudioBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AudioBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastStreamAudioHeader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastStreamAudioHeader {
    type Vtable = IAppBroadcastStreamAudioHeader_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbf21a570_6b78_4216_9f07_5aff5256f1b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastStreamAudioHeader_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AbsoluteTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub RelativeTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub HasDiscontinuity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub FrameId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastStreamReader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastStreamReader {
    type Vtable = IAppBroadcastStreamReader_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb338bcf9_3364_4460_b5f1_3cc2796a8aa2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastStreamReader_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AudioChannels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub AudioSampleRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub AudioAacSequence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AudioAacSequence: usize,
    pub AudioBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub TryGetNextAudioFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub VideoWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub VideoHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub VideoBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub TryGetNextVideoFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AudioFrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveAudioFrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub VideoFrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveVideoFrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastStreamStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastStreamStateChangedEventArgs {
    type Vtable = IAppBroadcastStreamStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5108a733_d008_4a89_93be_58aed961374e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastStreamStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StreamState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastStreamState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastStreamVideoFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastStreamVideoFrame {
    type Vtable = IAppBroadcastStreamVideoFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0f97cf2b_c9e4_4e88_8194_d814cbd585d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastStreamVideoFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub VideoHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub VideoBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    VideoBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastStreamVideoHeader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastStreamVideoHeader {
    type Vtable = IAppBroadcastStreamVideoHeader_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0b9ebece_7e32_432d_8ca2_36bf10b9f462);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastStreamVideoHeader_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AbsoluteTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub RelativeTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub IsKeyFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HasDiscontinuity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub FrameId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastTriggerDetails {
    type Vtable = IAppBroadcastTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdeebab35_ec5e_4d8f_b1c0_5da6e8c75638);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub BackgroundService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastViewerCountChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastViewerCountChangedEventArgs {
    type Vtable = IAppBroadcastViewerCountChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe6e11825_5401_4ade_8bd2_c14ecee6807d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastViewerCountChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ViewerCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCapture(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCapture {
    type Vtable = IAppCapture_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9749d453_a29a_45ed_8f29_22d09942cff7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCapture_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsCapturingAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsCapturingVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CapturingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCapturingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureAlternateShortcutKeys(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureAlternateShortcutKeys {
    type Vtable = IAppCaptureAlternateShortcutKeys_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x19e8e0ef_236c_40f9_b38f_9b7dd65d1ccc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureAlternateShortcutKeys_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub SetToggleGameBarKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_system::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleGameBarKey: usize,
    #[cfg(feature = "System")]
    pub ToggleGameBarKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_system::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleGameBarKey: usize,
    #[cfg(feature = "System")]
    pub SetToggleGameBarKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_system::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleGameBarKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub ToggleGameBarKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_system::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleGameBarKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub SetSaveHistoricalVideoKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_system::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetSaveHistoricalVideoKey: usize,
    #[cfg(feature = "System")]
    pub SaveHistoricalVideoKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_system::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SaveHistoricalVideoKey: usize,
    #[cfg(feature = "System")]
    pub SetSaveHistoricalVideoKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_system::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetSaveHistoricalVideoKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub SaveHistoricalVideoKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_system::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SaveHistoricalVideoKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub SetToggleRecordingKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_system::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleRecordingKey: usize,
    #[cfg(feature = "System")]
    pub ToggleRecordingKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_system::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleRecordingKey: usize,
    #[cfg(feature = "System")]
    pub SetToggleRecordingKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_system::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleRecordingKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub ToggleRecordingKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_system::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleRecordingKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub SetTakeScreenshotKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_system::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetTakeScreenshotKey: usize,
    #[cfg(feature = "System")]
    pub TakeScreenshotKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_system::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    TakeScreenshotKey: usize,
    #[cfg(feature = "System")]
    pub SetTakeScreenshotKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_system::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetTakeScreenshotKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub TakeScreenshotKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_system::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    TakeScreenshotKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub SetToggleRecordingIndicatorKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_system::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleRecordingIndicatorKey: usize,
    #[cfg(feature = "System")]
    pub ToggleRecordingIndicatorKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_system::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleRecordingIndicatorKey: usize,
    #[cfg(feature = "System")]
    pub SetToggleRecordingIndicatorKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_system::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleRecordingIndicatorKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub ToggleRecordingIndicatorKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_system::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleRecordingIndicatorKeyModifiers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureAlternateShortcutKeys2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureAlternateShortcutKeys2 {
    type Vtable = IAppCaptureAlternateShortcutKeys2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc3669090_dd17_47f0_95e5_ce42286cf338);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureAlternateShortcutKeys2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub SetToggleMicrophoneCaptureKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_system::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleMicrophoneCaptureKey: usize,
    #[cfg(feature = "System")]
    pub ToggleMicrophoneCaptureKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_system::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleMicrophoneCaptureKey: usize,
    #[cfg(feature = "System")]
    pub SetToggleMicrophoneCaptureKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_system::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleMicrophoneCaptureKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub ToggleMicrophoneCaptureKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_system::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleMicrophoneCaptureKeyModifiers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureAlternateShortcutKeys3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureAlternateShortcutKeys3 {
    type Vtable = IAppCaptureAlternateShortcutKeys3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7b81448c_418e_469c_a49a_45b597c826b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureAlternateShortcutKeys3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub SetToggleCameraCaptureKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_system::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleCameraCaptureKey: usize,
    #[cfg(feature = "System")]
    pub ToggleCameraCaptureKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_system::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleCameraCaptureKey: usize,
    #[cfg(feature = "System")]
    pub SetToggleCameraCaptureKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_system::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleCameraCaptureKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub ToggleCameraCaptureKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_system::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleCameraCaptureKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub SetToggleBroadcastKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_system::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleBroadcastKey: usize,
    #[cfg(feature = "System")]
    pub ToggleBroadcastKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_system::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleBroadcastKey: usize,
    #[cfg(feature = "System")]
    pub SetToggleBroadcastKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_system::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleBroadcastKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub ToggleBroadcastKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_system::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleBroadcastKeyModifiers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureDurationGeneratedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureDurationGeneratedEventArgs {
    type Vtable = IAppCaptureDurationGeneratedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc1f5563b_ffa1_44c9_975f_27fbeb553b35);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureDurationGeneratedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureFileGeneratedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureFileGeneratedEventArgs {
    type Vtable = IAppCaptureFileGeneratedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4189fbf4_465e_45bf_907f_165b3fb23758);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureFileGeneratedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureManagerStatics {
    type Vtable = IAppCaptureManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d9e3ea7_6282_4735_8d4e_aa45f90f6723);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetCurrentSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ApplySettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appcapturesettings: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureMetadataWriter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureMetadataWriter {
    type Vtable = IAppCaptureMetadataWriter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe0ce4877_9aaf_46b4_ad31_6a60b441c780);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureMetadataWriter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AddStringEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, priority: AppCaptureMetadataPriority) -> ::windows_core::HRESULT,
    pub AddInt32Event: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, value: i32, priority: AppCaptureMetadataPriority) -> ::windows_core::HRESULT,
    pub AddDoubleEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, value: f64, priority: AppCaptureMetadataPriority) -> ::windows_core::HRESULT,
    pub StartStringState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, priority: AppCaptureMetadataPriority) -> ::windows_core::HRESULT,
    pub StartInt32State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, value: i32, priority: AppCaptureMetadataPriority) -> ::windows_core::HRESULT,
    pub StartDoubleState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, value: f64, priority: AppCaptureMetadataPriority) -> ::windows_core::HRESULT,
    pub StopState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub StopAllStates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemainingStorageBytesAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub MetadataPurged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveMetadataPurged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureMicrophoneCaptureStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureMicrophoneCaptureStateChangedEventArgs {
    type Vtable = IAppCaptureMicrophoneCaptureStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x324d249e_45bc_4c35_bc35_e469fc7a69e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureMicrophoneCaptureStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureMicrophoneCaptureState) -> ::windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureRecordOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureRecordOperation {
    type Vtable = IAppCaptureRecordOperation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc66020a9_1538_495c_9bbb_2ba870ec5861);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureRecordOperation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StopRecording: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureRecordingState) -> ::windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
    pub IsFileTruncated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub DurationGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDurationGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub FileGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveFileGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureRecordingStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureRecordingStateChangedEventArgs {
    type Vtable = IAppCaptureRecordingStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x24fc8712_e305_490d_b415_6b1c9049736b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureRecordingStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureRecordingState) -> ::windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureServices(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureServices {
    type Vtable = IAppCaptureServices_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x44fec0b5_34f5_4f18_ae8c_b9123abbfc0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureServices_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Record: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RecordTimeSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: ::winrt_foundation::DateTime, duration: ::winrt_foundation::TimeSpan, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CanCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureSettings {
    type Vtable = IAppCaptureSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x14683a86_8807_48d3_883a_970ee4532a39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage")]
    pub SetAppCaptureDestinationFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SetAppCaptureDestinationFolder: usize,
    #[cfg(feature = "Storage")]
    pub AppCaptureDestinationFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    AppCaptureDestinationFolder: usize,
    pub SetAudioEncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub AudioEncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetIsAudioCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsAudioCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCustomVideoEncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub CustomVideoEncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetCustomVideoEncodingHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub CustomVideoEncodingHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetCustomVideoEncodingWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub CustomVideoEncodingWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetHistoricalBufferLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub HistoricalBufferLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetHistoricalBufferLengthUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppCaptureHistoricalBufferLengthUnit) -> ::windows_core::HRESULT,
    pub HistoricalBufferLengthUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureHistoricalBufferLengthUnit) -> ::windows_core::HRESULT,
    pub SetIsHistoricalCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsHistoricalCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsHistoricalCaptureOnBatteryAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsHistoricalCaptureOnBatteryAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsHistoricalCaptureOnWirelessDisplayAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsHistoricalCaptureOnWirelessDisplayAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetMaximumRecordLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub MaximumRecordLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub SetScreenshotDestinationFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SetScreenshotDestinationFolder: usize,
    #[cfg(feature = "Storage")]
    pub ScreenshotDestinationFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    ScreenshotDestinationFolder: usize,
    pub SetVideoEncodingBitrateMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppCaptureVideoEncodingBitrateMode) -> ::windows_core::HRESULT,
    pub VideoEncodingBitrateMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureVideoEncodingBitrateMode) -> ::windows_core::HRESULT,
    pub SetVideoEncodingResolutionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppCaptureVideoEncodingResolutionMode) -> ::windows_core::HRESULT,
    pub VideoEncodingResolutionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureVideoEncodingResolutionMode) -> ::windows_core::HRESULT,
    pub SetIsAppCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsAppCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsCpuConstrained: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsDisabledByPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsMemoryConstrained: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HasHardwareEncoder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureSettings2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureSettings2 {
    type Vtable = IAppCaptureSettings2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfcb8cee7_e26b_476f_9b1a_ec342d2a8fde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureSettings2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsGpuConstrained: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub AlternateShortcutKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureSettings3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureSettings3 {
    type Vtable = IAppCaptureSettings3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa93502fe_88c2_42d6_aaaa_40feffd75aec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureSettings3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetIsMicrophoneCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsMicrophoneCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureSettings4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureSettings4 {
    type Vtable = IAppCaptureSettings4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x07c2774c_1a81_482f_a244_049d95f25b0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureSettings4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetIsMicrophoneCaptureEnabledByDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsMicrophoneCaptureEnabledByDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetSystemAudioGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub SystemAudioGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetMicrophoneGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub MicrophoneGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetVideoEncodingFrameRateMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppCaptureVideoEncodingFrameRateMode) -> ::windows_core::HRESULT,
    pub VideoEncodingFrameRateMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureVideoEncodingFrameRateMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureSettings5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureSettings5 {
    type Vtable = IAppCaptureSettings5_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x18894522_b0e8_4ba0_8f13_3eaa5fa4013b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureSettings5_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetIsEchoCancellationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsEchoCancellationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsCursorImageCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsCursorImageCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureState(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureState {
    type Vtable = IAppCaptureState_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x73134372_d4eb_44ce_9538_465f506ac4ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureState_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsTargetRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsHistoricalCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ShouldCaptureMicrophone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetShouldCaptureMicrophone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub RestartMicrophoneCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MicrophoneCaptureState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureMicrophoneCaptureState) -> ::windows_core::HRESULT,
    pub MicrophoneCaptureError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub MicrophoneCaptureStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveMicrophoneCaptureStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub CaptureTargetClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCaptureTargetClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureStatics {
    type Vtable = IAppCaptureStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf922dd6c_0a7e_4e74_8b20_9c1f902d08a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureStatics2 {
    type Vtable = IAppCaptureStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb2d881d4_836c_4da4_afd7_facc041e1cf3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetAllowedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allowed: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraCaptureUI(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICameraCaptureUI {
    type Vtable = ICameraCaptureUI_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x48587540_6f93_4bb4_b8f3_e89e48948c91);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraCaptureUI_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PhotoSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub VideoSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub CaptureFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: CameraCaptureUIMode, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    CaptureFileAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraCaptureUIPhotoCaptureSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICameraCaptureUIPhotoCaptureSettings {
    type Vtable = ICameraCaptureUIPhotoCaptureSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9f5be97_3472_46a8_8a9e_04ce42ccc97d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraCaptureUIPhotoCaptureSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CameraCaptureUIPhotoFormat) -> ::windows_core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CameraCaptureUIPhotoFormat) -> ::windows_core::HRESULT,
    pub MaxResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CameraCaptureUIMaxPhotoResolution) -> ::windows_core::HRESULT,
    pub SetMaxResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CameraCaptureUIMaxPhotoResolution) -> ::windows_core::HRESULT,
    pub CroppedSizeInPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Size) -> ::windows_core::HRESULT,
    pub SetCroppedSizeInPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Size) -> ::windows_core::HRESULT,
    pub CroppedAspectRatio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Size) -> ::windows_core::HRESULT,
    pub SetCroppedAspectRatio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Size) -> ::windows_core::HRESULT,
    pub AllowCropping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowCropping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraCaptureUIVideoCaptureSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICameraCaptureUIVideoCaptureSettings {
    type Vtable = ICameraCaptureUIVideoCaptureSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64e92d1f_a28d_425a_b84f_e568335ff24e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraCaptureUIVideoCaptureSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CameraCaptureUIVideoFormat) -> ::windows_core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CameraCaptureUIVideoFormat) -> ::windows_core::HRESULT,
    pub MaxResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CameraCaptureUIMaxVideoResolution) -> ::windows_core::HRESULT,
    pub SetMaxResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CameraCaptureUIMaxVideoResolution) -> ::windows_core::HRESULT,
    pub MaxDurationInSeconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetMaxDurationInSeconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub AllowTrimming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowTrimming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraOptionsUIStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICameraOptionsUIStatics {
    type Vtable = ICameraOptionsUIStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3b0d5e34_3906_4b7d_946c_7bde844499ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraOptionsUIStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediacapture: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICapturedFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICapturedFrame {
    type Vtable = ICapturedFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1dd2de1f_571b_44d8_8e80_a08a1578766e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICapturedFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICapturedFrame2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICapturedFrame2 {
    type Vtable = ICapturedFrame2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x543fa6d1_bd78_4866_adda_24314bc65dea);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICapturedFrame2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ControlValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub BitmapProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))]
    BitmapProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICapturedFrameControlValues(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICapturedFrameControlValues {
    type Vtable = ICapturedFrameControlValues_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x90c65b7f_4e0d_4ca4_882d_7a144fed0a90);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICapturedFrameControlValues_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Exposure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExposureCompensation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsoSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Focus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Media_Devices")]
    pub SceneMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    SceneMode: usize,
    pub Flashed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FlashPowerPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub WhiteBalance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ZoomFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICapturedFrameControlValues2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICapturedFrameControlValues2 {
    type Vtable = ICapturedFrameControlValues2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x500b2b88_06d2_4aa7_a7db_d37af73321d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICapturedFrameControlValues2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Media_Devices")]
    pub FocusState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    FocusState: usize,
    pub IsoDigitalGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsoAnalogGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub SensorFrameRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SensorFrameRate: usize,
    pub WhiteBalanceGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICapturedFrameWithSoftwareBitmap(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICapturedFrameWithSoftwareBitmap {
    type Vtable = ICapturedFrameWithSoftwareBitmap_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb58e8b6e_8503_49b5_9e86_897d26a3ff3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICapturedFrameWithSoftwareBitmap_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub SoftwareBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SoftwareBitmap: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICapturedPhoto(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICapturedPhoto {
    type Vtable = ICapturedPhoto_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb0ce7e5a_cfcc_4d6c_8ad1_0869208aca16);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICapturedPhoto_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Frame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameBarServices(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameBarServices {
    type Vtable = IGameBarServices_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2dbead57_50a6_499e_8c6c_d330a7311796);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarServices_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TargetCapturePolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameBarTargetCapturePolicy) -> ::windows_core::HRESULT,
    pub EnableCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DisableCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TargetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AppBroadcastServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AppCaptureServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CommandReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCommandReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameBarServicesCommandEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameBarServicesCommandEventArgs {
    type Vtable = IGameBarServicesCommandEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa74226b2_f176_4fcf_8fbb_cf698b2eb8e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarServicesCommandEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Command: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameBarCommand) -> ::windows_core::HRESULT,
    pub Origin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameBarCommandOrigin) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameBarServicesManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameBarServicesManager {
    type Vtable = IGameBarServicesManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3a4b9cfa_7f8b_4c60_9dbb_0bcd262dffc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarServicesManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GameBarServicesCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveGameBarServicesCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameBarServicesManagerGameBarServicesCreatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameBarServicesManagerGameBarServicesCreatedEventArgs {
    type Vtable = IGameBarServicesManagerGameBarServicesCreatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xededbd9c_143e_49a3_a5ea_0b1995c8d46e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarServicesManagerGameBarServicesCreatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GameBarServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameBarServicesManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameBarServicesManagerStatics {
    type Vtable = IGameBarServicesManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x34c1b616_ff25_4792_98f2_d3753f15ac13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarServicesManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameBarServicesTargetInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameBarServicesTargetInfo {
    type Vtable = IGameBarServicesTargetInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb4202f92_1611_4e05_b6ef_dfd737ae33b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarServicesTargetInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TitleId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameBarServicesDisplayMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLagMediaRecording(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILowLagMediaRecording {
    type Vtable = ILowLagMediaRecording_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x41c8baf7_ff3f_49f0_a477_f195e3ce5108);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagMediaRecording_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StopAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FinishAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLagMediaRecording2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILowLagMediaRecording2 {
    type Vtable = ILowLagMediaRecording2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6369c758_5644_41e2_97af_8ef56a25e225);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagMediaRecording2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Media_Devices")]
    pub PauseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    PauseAsync: usize,
    pub ResumeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLagMediaRecording3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILowLagMediaRecording3 {
    type Vtable = ILowLagMediaRecording3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c33ab12_48f7_47da_b41e_90880a5fe0ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagMediaRecording3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Media_Devices")]
    pub PauseWithResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    PauseWithResultAsync: usize,
    pub StopWithResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLagPhotoCapture(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILowLagPhotoCapture {
    type Vtable = ILowLagPhotoCapture_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa37251b7_6b44_473d_8f24_f703d6c0ec44);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagPhotoCapture_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CaptureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FinishAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLagPhotoSequenceCapture(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILowLagPhotoSequenceCapture {
    type Vtable = ILowLagPhotoSequenceCapture_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7cc346bb_b9a9_4c91_8ffa_287e9c668669);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagPhotoSequenceCapture_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StopAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FinishAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PhotoCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePhotoCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCapture(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCapture {
    type Vtable = IMediaCapture_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc61afbb4_fb10_4a34_ac18_ca80d9c8e7ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InitializeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub InitializeWithSettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediacaptureinitializationsettings: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Media_MediaProperties", feature = "Storage"))]
    pub StartRecordToStorageFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: ::windows_core::RawPtr, file: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_MediaProperties", feature = "Storage")))]
    StartRecordToStorageFileAsync: usize,
    #[cfg(all(feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub StartRecordToStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: ::windows_core::RawPtr, stream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_MediaProperties", feature = "Storage_Streams")))]
    StartRecordToStreamAsync: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub StartRecordToCustomSinkAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: ::windows_core::RawPtr, custommediasink: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    StartRecordToCustomSinkAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub StartRecordToCustomSinkIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: ::windows_core::RawPtr, customsinkactivationid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, customsinksettings: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))]
    StartRecordToCustomSinkIdAsync: usize,
    pub StopRecordAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Media_MediaProperties", feature = "Storage"))]
    pub CapturePhotoToStorageFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: ::windows_core::RawPtr, file: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_MediaProperties", feature = "Storage")))]
    CapturePhotoToStorageFileAsync: usize,
    #[cfg(all(feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub CapturePhotoToStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: ::windows_core::RawPtr, stream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_MediaProperties", feature = "Storage_Streams")))]
    CapturePhotoToStreamAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub AddEffectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediastreamtype: MediaStreamType, effectactivationid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, effectsettings: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    AddEffectAsync: usize,
    pub ClearEffectsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediastreamtype: MediaStreamType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetEncoderProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediastreamtype: MediaStreamType, propertyid: ::windows_core::GUID, propertyvalue: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetEncoderProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediastreamtype: MediaStreamType, propertyid: ::windows_core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Failed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, erroreventhandler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RecordLimitationExceeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recordlimitationexceededeventhandler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveRecordLimitationExceeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub MediaCaptureSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Media_Devices")]
    pub AudioDeviceController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    AudioDeviceController: usize,
    #[cfg(feature = "Media_Devices")]
    pub VideoDeviceController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    VideoDeviceController: usize,
    pub SetPreviewMirroring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub GetPreviewMirroring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetPreviewRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: VideoRotation) -> ::windows_core::HRESULT,
    pub GetPreviewRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VideoRotation) -> ::windows_core::HRESULT,
    pub SetRecordRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: VideoRotation) -> ::windows_core::HRESULT,
    pub GetRecordRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VideoRotation) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCapture2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCapture2 {
    type Vtable = IMediaCapture2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9cc68260_7da1_4043_b652_21b8878daff9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Media_MediaProperties", feature = "Storage"))]
    pub PrepareLowLagRecordToStorageFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: ::windows_core::RawPtr, file: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_MediaProperties", feature = "Storage")))]
    PrepareLowLagRecordToStorageFileAsync: usize,
    #[cfg(all(feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub PrepareLowLagRecordToStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: ::windows_core::RawPtr, stream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_MediaProperties", feature = "Storage_Streams")))]
    PrepareLowLagRecordToStreamAsync: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub PrepareLowLagRecordToCustomSinkAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: ::windows_core::RawPtr, custommediasink: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    PrepareLowLagRecordToCustomSinkAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub PrepareLowLagRecordToCustomSinkIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: ::windows_core::RawPtr, customsinkactivationid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, customsinksettings: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))]
    PrepareLowLagRecordToCustomSinkIdAsync: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub PrepareLowLagPhotoCaptureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    PrepareLowLagPhotoCaptureAsync: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub PrepareLowLagPhotoSequenceCaptureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    PrepareLowLagPhotoSequenceCaptureAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub SetEncodingPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediastreamtype: MediaStreamType, mediaencodingproperties: ::windows_core::RawPtr, encoderproperties: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))]
    SetEncodingPropertiesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCapture3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCapture3 {
    type Vtable = IMediaCapture3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd4136f30_1564_466e_bc0a_af94e02ab016);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Media_Capture_Core", feature = "Media_MediaProperties"))]
    pub PrepareVariablePhotoSequenceCaptureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_Capture_Core", feature = "Media_MediaProperties")))]
    PrepareVariablePhotoSequenceCaptureAsync: usize,
    pub FocusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveFocusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub PhotoConfirmationCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePhotoConfirmationCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCapture4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCapture4 {
    type Vtable = IMediaCapture4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbacd6fd6_fb08_4947_aea2_ce14eff0ce13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Media_Effects")]
    pub AddAudioEffectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, definition: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Effects"))]
    AddAudioEffectAsync: usize,
    #[cfg(feature = "Media_Effects")]
    pub AddVideoEffectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, definition: ::windows_core::RawPtr, mediastreamtype: MediaStreamType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Effects"))]
    AddVideoEffectAsync: usize,
    #[cfg(feature = "Media_Devices")]
    pub PauseRecordAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    PauseRecordAsync: usize,
    pub ResumeRecordAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CameraStreamStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCameraStreamStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "Media_Devices")]
    pub CameraStreamState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Devices::CameraStreamState) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    CameraStreamState: usize,
    pub GetPreviewFrameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetPreviewFrameCopyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destination: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ThermalStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveThermalStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ThermalStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaCaptureThermalStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub PrepareAdvancedPhotoCaptureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    PrepareAdvancedPhotoCaptureAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCapture5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCapture5 {
    type Vtable = IMediaCapture5_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xda787c22_3a9b_4720_a71e_97900a316e5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture5_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RemoveEffectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effect: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Media_Devices")]
    pub PauseRecordWithResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    PauseRecordWithResultAsync: usize,
    pub StopRecordWithResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    pub FrameSources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames")))]
    FrameSources: usize,
    #[cfg(feature = "Media_Capture_Frames")]
    pub CreateFrameReaderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputsource: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Capture_Frames"))]
    CreateFrameReaderAsync: usize,
    #[cfg(feature = "Media_Capture_Frames")]
    pub CreateFrameReaderWithSubtypeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputsource: ::windows_core::RawPtr, outputsubtype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Capture_Frames"))]
    CreateFrameReaderWithSubtypeAsync: usize,
    #[cfg(all(feature = "Graphics_Imaging", feature = "Media_Capture_Frames"))]
    pub CreateFrameReaderWithSubtypeAndSizeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputsource: ::windows_core::RawPtr, outputsubtype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, outputsize: ::winrt_graphics::Imaging::BitmapSize, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Imaging", feature = "Media_Capture_Frames")))]
    CreateFrameReaderWithSubtypeAndSizeAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCapture6(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCapture6 {
    type Vtable = IMediaCapture6_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x228948bd_4b20_4bb1_9fd6_a583212a1012);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture6_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CaptureDeviceExclusiveControlStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCaptureDeviceExclusiveControlStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    pub CreateMultiSourceFrameReaderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputsources: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames")))]
    CreateMultiSourceFrameReaderAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCapture7(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCapture7 {
    type Vtable = IMediaCapture7_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9169f102_8888_541a_95bc_24e4d462542a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture7_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "UI_WindowManagement")]
    pub CreateRelativePanelWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capturemode: StreamingCaptureMode, displayregion: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))]
    CreateRelativePanelWatcher: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    type Vtable = IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9d2f920d_a588_43c6_89d6_5ad322af006a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaCaptureDeviceExclusiveControlStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureFailedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureFailedEventArgs {
    type Vtable = IMediaCaptureFailedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x80fde3f4_54c4_42c0_8d19_cea1a87ca18b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureFailedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Code: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureFocusChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureFocusChangedEventArgs {
    type Vtable = IMediaCaptureFocusChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x81e1bc7f_2277_493e_abee_d3f44ff98c04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureFocusChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Media_Devices")]
    pub FocusState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Devices::MediaCaptureFocusState) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    FocusState: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureInitializationSettings {
    type Vtable = IMediaCaptureInitializationSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9782ba70_ea65_4900_9356_8ca887726884);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetAudioDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetVideoDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetStreamingCaptureMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StreamingCaptureMode) -> ::windows_core::HRESULT,
    pub StreamingCaptureMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StreamingCaptureMode) -> ::windows_core::HRESULT,
    pub SetPhotoCaptureSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PhotoCaptureSource) -> ::windows_core::HRESULT,
    pub PhotoCaptureSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhotoCaptureSource) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureInitializationSettings2 {
    type Vtable = IMediaCaptureInitializationSettings2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x404e0626_c9dc_43e9_aee4_e6bf1b57b44c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetMediaCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaCategory) -> ::windows_core::HRESULT,
    pub MediaCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaCategory) -> ::windows_core::HRESULT,
    pub SetAudioProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::AudioProcessing) -> ::windows_core::HRESULT,
    pub AudioProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::AudioProcessing) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureInitializationSettings3 {
    type Vtable = IMediaCaptureInitializationSettings3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4160519d_be48_4730_8104_0cf6e9e97948);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Media_Core")]
    pub SetAudioSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    SetAudioSource: usize,
    #[cfg(feature = "Media_Core")]
    pub AudioSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    AudioSource: usize,
    #[cfg(feature = "Media_Core")]
    pub SetVideoSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    SetVideoSource: usize,
    #[cfg(feature = "Media_Core")]
    pub VideoSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    VideoSource: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureInitializationSettings4 {
    type Vtable = IMediaCaptureInitializationSettings4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf502a537_4cb7_4d28_95ed_4f9f012e0518);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub VideoProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetVideoProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PreviewMediaDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetPreviewMediaDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RecordMediaDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetRecordMediaDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PhotoMediaDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetPhotoMediaDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureInitializationSettings5 {
    type Vtable = IMediaCaptureInitializationSettings5_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd5a2e3b8_2626_4e94_b7b3_5308a0f64b1a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings5_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Media_Capture_Frames")]
    pub SourceGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Capture_Frames"))]
    SourceGroup: usize,
    #[cfg(feature = "Media_Capture_Frames")]
    pub SetSourceGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Capture_Frames"))]
    SetSourceGroup: usize,
    pub SharingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaCaptureSharingMode) -> ::windows_core::HRESULT,
    pub SetSharingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaCaptureSharingMode) -> ::windows_core::HRESULT,
    pub MemoryPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaCaptureMemoryPreference) -> ::windows_core::HRESULT,
    pub SetMemoryPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaCaptureMemoryPreference) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings6(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureInitializationSettings6 {
    type Vtable = IMediaCaptureInitializationSettings6_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb2e26b47_3db1_4d33_ab63_0ffa09056585);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings6_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AlwaysPlaySystemShutterSound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAlwaysPlaySystemShutterSound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings7(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureInitializationSettings7 {
    type Vtable = IMediaCaptureInitializationSettings7_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x41546967_f58a_5d82_9ef4_ed572fb5e34e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings7_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Security_Credentials")]
    pub DeviceUriPasswordCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    DeviceUriPasswordCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetDeviceUriPasswordCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetDeviceUriPasswordCredential: usize,
    pub DeviceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetDeviceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCapturePauseResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCapturePauseResult {
    type Vtable = IMediaCapturePauseResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaec47ca3_4477_4b04_a06f_2c1c5182fe9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapturePauseResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LastFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RecordDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureRelativePanelWatcher(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureRelativePanelWatcher {
    type Vtable = IMediaCaptureRelativePanelWatcher_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d896566_04be_5b89_b30e_bd34a9f12db0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureRelativePanelWatcher_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub RelativePanel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_devices::Enumeration::Panel) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    RelativePanel: usize,
    pub Changed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureSettings {
    type Vtable = IMediaCaptureSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d83aafe_6d45_4477_8dc4_ac5bc01c4091);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AudioDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub StreamingCaptureMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StreamingCaptureMode) -> ::windows_core::HRESULT,
    pub PhotoCaptureSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhotoCaptureSource) -> ::windows_core::HRESULT,
    pub VideoDeviceCharacteristic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VideoDeviceCharacteristic) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureSettings2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureSettings2 {
    type Vtable = IMediaCaptureSettings2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f9e7cfb_fa9f_4b13_9cbe_5ab94f1f3493);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureSettings2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ConcurrentRecordAndPhotoSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ConcurrentRecordAndPhotoSequenceSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CameraSoundRequiredForRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Horizontal35mmEquivalentFocalLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PitchOffsetDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Vertical35mmEquivalentFocalLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MediaCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaCategory) -> ::windows_core::HRESULT,
    pub AudioProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::AudioProcessing) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureSettings3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureSettings3 {
    type Vtable = IMediaCaptureSettings3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x303c67c2_8058_4b1b_b877_8c2ef3528440);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureSettings3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3D11Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3D11Device: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureStatics {
    type Vtable = IMediaCaptureStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xacef81ff_99ed_4645_965e_1925cfc63834);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsVideoProfileSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, videodeviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllVideoProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, videodeviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllVideoProfiles: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindConcurrentProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, videodeviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindConcurrentProfiles: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindKnownVideoProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, videodeviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, name: KnownVideoProfile, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindKnownVideoProfiles: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureStopResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureStopResult {
    type Vtable = IMediaCaptureStopResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf9db6a2a_a092_4ad1_97d4_f201f9d082db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureStopResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LastFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RecordDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureVideoPreview(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureVideoPreview {
    type Vtable = IMediaCaptureVideoPreview_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27727073_549e_447f_a20a_4f03c479d8c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureVideoPreview_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StartPreviewAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub StartPreviewToCustomSinkAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: ::windows_core::RawPtr, custommediasink: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    StartPreviewToCustomSinkAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub StartPreviewToCustomSinkIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: ::windows_core::RawPtr, customsinkactivationid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, customsinksettings: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))]
    StartPreviewToCustomSinkIdAsync: usize,
    pub StopPreviewAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureVideoProfile(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureVideoProfile {
    type Vtable = IMediaCaptureVideoProfile_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x21a073bf_a3ee_4ecf_9ef6_50b0bc4e1305);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureVideoProfile_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedPreviewMediaDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedPreviewMediaDescription: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedRecordMediaDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedRecordMediaDescription: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedPhotoMediaDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedPhotoMediaDescription: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConcurrency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConcurrency: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureVideoProfile2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureVideoProfile2 {
    type Vtable = IMediaCaptureVideoProfile2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x97ddc95f_94ce_468f_9316_fc5bc2638f6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureVideoProfile2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    pub FrameSourceInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames")))]
    FrameSourceInfos: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureVideoProfileMediaDescription(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureVideoProfileMediaDescription {
    type Vtable = IMediaCaptureVideoProfileMediaDescription_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8012afef_b691_49ff_83f2_c1e76eaaea1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureVideoProfileMediaDescription_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub FrameRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub IsVariablePhotoSequenceSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsVariablePhotoSequenceSupported: usize,
    #[cfg(feature = "deprecated")]
    pub IsHdrVideoSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsHdrVideoSupported: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureVideoProfileMediaDescription2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureVideoProfileMediaDescription2 {
    type Vtable = IMediaCaptureVideoProfileMediaDescription2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc6a6ef13_322d_413a_b85a_68a88e02f4e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureVideoProfileMediaDescription2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Subtype: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOptionalReferencePhotoCapturedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOptionalReferencePhotoCapturedEventArgs {
    type Vtable = IOptionalReferencePhotoCapturedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x470f88b3_1e6d_4051_9c8b_f1d85af047b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOptionalReferencePhotoCapturedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Frame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Context: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoCapturedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhotoCapturedEventArgs {
    type Vtable = IPhotoCapturedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x373bfbc1_984e_4ff0_bf85_1c00aabc5a45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoCapturedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Frame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CaptureTimeOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoConfirmationCapturedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhotoConfirmationCapturedEventArgs {
    type Vtable = IPhotoConfirmationCapturedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab473672_c28a_4827_8f8d_3636d3beb51e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoConfirmationCapturedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Frame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CaptureTimeOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScreenCapture(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScreenCapture {
    type Vtable = IScreenCapture_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x89179ef7_cd12_4e0e_a6d4_5b3de98b2e9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScreenCapture_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Media_Core")]
    pub AudioSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    AudioSource: usize,
    #[cfg(feature = "Media_Core")]
    pub VideoSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    VideoSource: usize,
    pub IsAudioSuspended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsVideoSuspended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SourceSuspensionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSourceSuspensionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScreenCaptureStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScreenCaptureStatics {
    type Vtable = IScreenCaptureStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc898c3b0_c8a5_11e2_8b8b_0800200c9a66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScreenCaptureStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISourceSuspensionChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISourceSuspensionChangedEventArgs {
    type Vtable = ISourceSuspensionChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2ece7b5e_d49b_4394_bc32_f97d6cedec1c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISourceSuspensionChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsAudioSuspended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsVideoSuspended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoStreamConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoStreamConfiguration {
    type Vtable = IVideoStreamConfiguration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd8770a6f_4390_4b5e_ad3e_0f8af0963490);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoStreamConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Media_MediaProperties")]
    pub InputProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    InputProperties: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub OutputProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    OutputProperties: usize,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct KnownVideoProfile(pub i32);
impl KnownVideoProfile {
    pub const VideoRecording: Self = Self(0i32);
    pub const HighQualityPhoto: Self = Self(1i32);
    pub const BalancedVideoAndPhoto: Self = Self(2i32);
    pub const VideoConferencing: Self = Self(3i32);
    pub const PhotoSequence: Self = Self(4i32);
    pub const HighFrameRate: Self = Self(5i32);
    pub const VariablePhotoSequence: Self = Self(6i32);
    pub const HdrWithWcgVideo: Self = Self(7i32);
    pub const HdrWithWcgPhoto: Self = Self(8i32);
    pub const VideoHdr8: Self = Self(9i32);
    pub const CompressedCamera: Self = Self(10i32);
}
impl ::core::marker::Copy for KnownVideoProfile {}
impl ::core::clone::Clone for KnownVideoProfile {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KnownVideoProfile {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for KnownVideoProfile {
    type Abi = Self;
}
impl ::core::fmt::Debug for KnownVideoProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KnownVideoProfile").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for KnownVideoProfile {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.KnownVideoProfile;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct LowLagMediaRecording(::windows_core::IUnknown);
impl LowLagMediaRecording {
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
    #[cfg(feature = "Media_Devices")]
    pub fn PauseAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<ILowLagMediaRecording2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PauseAsync)(::windows_core::Interface::as_raw(this), behavior, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ResumeAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<ILowLagMediaRecording2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResumeAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Media_Devices")]
    pub fn PauseWithResultAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<MediaCapturePauseResult>> {
        let this = &::windows_core::Interface::cast::<ILowLagMediaRecording3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PauseWithResultAsync)(::windows_core::Interface::as_raw(this), behavior, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<MediaCapturePauseResult>>(result__)
        }
    }
    pub fn StopWithResultAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<MediaCaptureStopResult>> {
        let this = &::windows_core::Interface::cast::<ILowLagMediaRecording3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StopWithResultAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<MediaCaptureStopResult>>(result__)
        }
    }
}
impl ::core::clone::Clone for LowLagMediaRecording {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LowLagMediaRecording {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LowLagMediaRecording {}
impl ::core::fmt::Debug for LowLagMediaRecording {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LowLagMediaRecording").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LowLagMediaRecording {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.LowLagMediaRecording;{41c8baf7-ff3f-49f0-a477-f195e3ce5108})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LowLagMediaRecording {
    type Vtable = ILowLagMediaRecording_Vtbl;
    const IID: ::windows_core::GUID = <ILowLagMediaRecording as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LowLagMediaRecording {
    const NAME: &'static str = "Windows.Media.Capture.LowLagMediaRecording";
}
impl ::core::convert::From<LowLagMediaRecording> for ::windows_core::IUnknown {
    fn from(value: LowLagMediaRecording) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LowLagMediaRecording> for ::windows_core::IUnknown {
    fn from(value: &LowLagMediaRecording) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LowLagMediaRecording {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LowLagMediaRecording {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LowLagMediaRecording> for ::windows_core::IInspectable {
    fn from(value: LowLagMediaRecording) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LowLagMediaRecording> for ::windows_core::IInspectable {
    fn from(value: &LowLagMediaRecording) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LowLagMediaRecording {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LowLagMediaRecording {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct LowLagPhotoCapture(::windows_core::IUnknown);
impl LowLagPhotoCapture {
    pub fn CaptureAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<CapturedPhoto>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CaptureAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<CapturedPhoto>>(result__)
        }
    }
    pub fn FinishAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FinishAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for LowLagPhotoCapture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LowLagPhotoCapture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LowLagPhotoCapture {}
impl ::core::fmt::Debug for LowLagPhotoCapture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LowLagPhotoCapture").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LowLagPhotoCapture {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.LowLagPhotoCapture;{a37251b7-6b44-473d-8f24-f703d6c0ec44})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LowLagPhotoCapture {
    type Vtable = ILowLagPhotoCapture_Vtbl;
    const IID: ::windows_core::GUID = <ILowLagPhotoCapture as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LowLagPhotoCapture {
    const NAME: &'static str = "Windows.Media.Capture.LowLagPhotoCapture";
}
impl ::core::convert::From<LowLagPhotoCapture> for ::windows_core::IUnknown {
    fn from(value: LowLagPhotoCapture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LowLagPhotoCapture> for ::windows_core::IUnknown {
    fn from(value: &LowLagPhotoCapture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LowLagPhotoCapture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LowLagPhotoCapture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LowLagPhotoCapture> for ::windows_core::IInspectable {
    fn from(value: LowLagPhotoCapture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LowLagPhotoCapture> for ::windows_core::IInspectable {
    fn from(value: &LowLagPhotoCapture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LowLagPhotoCapture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LowLagPhotoCapture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct LowLagPhotoSequenceCapture(::windows_core::IUnknown);
impl LowLagPhotoSequenceCapture {
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
    pub fn PhotoCaptured<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<LowLagPhotoSequenceCapture, PhotoCapturedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
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
}
impl ::core::clone::Clone for LowLagPhotoSequenceCapture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LowLagPhotoSequenceCapture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LowLagPhotoSequenceCapture {}
impl ::core::fmt::Debug for LowLagPhotoSequenceCapture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LowLagPhotoSequenceCapture").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LowLagPhotoSequenceCapture {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.LowLagPhotoSequenceCapture;{7cc346bb-b9a9-4c91-8ffa-287e9c668669})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LowLagPhotoSequenceCapture {
    type Vtable = ILowLagPhotoSequenceCapture_Vtbl;
    const IID: ::windows_core::GUID = <ILowLagPhotoSequenceCapture as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LowLagPhotoSequenceCapture {
    const NAME: &'static str = "Windows.Media.Capture.LowLagPhotoSequenceCapture";
}
impl ::core::convert::From<LowLagPhotoSequenceCapture> for ::windows_core::IUnknown {
    fn from(value: LowLagPhotoSequenceCapture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LowLagPhotoSequenceCapture> for ::windows_core::IUnknown {
    fn from(value: &LowLagPhotoSequenceCapture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LowLagPhotoSequenceCapture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LowLagPhotoSequenceCapture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LowLagPhotoSequenceCapture> for ::windows_core::IInspectable {
    fn from(value: LowLagPhotoSequenceCapture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LowLagPhotoSequenceCapture> for ::windows_core::IInspectable {
    fn from(value: &LowLagPhotoSequenceCapture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LowLagPhotoSequenceCapture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LowLagPhotoSequenceCapture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct MediaCapture(::windows_core::IUnknown);
impl MediaCapture {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MediaCapture, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn InitializeAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InitializeAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn InitializeWithSettingsAsync<'a, Param0: ::windows_core::IntoParam<'a, MediaCaptureInitializationSettings>>(&self, mediacaptureinitializationsettings: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InitializeWithSettingsAsync)(::windows_core::Interface::as_raw(this), mediacaptureinitializationsettings.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Media_MediaProperties", feature = "Storage"))]
    pub fn StartRecordToStorageFileAsync<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFile>>(&self, encodingprofile: Param0, file: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartRecordToStorageFileAsync)(::windows_core::Interface::as_raw(this), encodingprofile.into_param().abi(), file.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub fn StartRecordToStreamAsync<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>>(&self, encodingprofile: Param0, stream: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartRecordToStreamAsync)(::windows_core::Interface::as_raw(this), encodingprofile.into_param().abi(), stream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn StartRecordToCustomSinkAsync<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows_core::IntoParam<'a, super::IMediaExtension>>(&self, encodingprofile: Param0, custommediasink: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartRecordToCustomSinkAsync)(::windows_core::Interface::as_raw(this), encodingprofile.into_param().abi(), custommediasink.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub fn StartRecordToCustomSinkIdAsync<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IPropertySet>>(&self, encodingprofile: Param0, customsinkactivationid: Param1, customsinksettings: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartRecordToCustomSinkIdAsync)(::windows_core::Interface::as_raw(this), encodingprofile.into_param().abi(), customsinkactivationid.into_param().abi(), customsinksettings.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn StopRecordAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StopRecordAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Media_MediaProperties", feature = "Storage"))]
    pub fn CapturePhotoToStorageFileAsync<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProperties::ImageEncodingProperties>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFile>>(&self, r#type: Param0, file: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CapturePhotoToStorageFileAsync)(::windows_core::Interface::as_raw(this), r#type.into_param().abi(), file.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub fn CapturePhotoToStreamAsync<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProperties::ImageEncodingProperties>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>>(&self, r#type: Param0, stream: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CapturePhotoToStreamAsync)(::windows_core::Interface::as_raw(this), r#type.into_param().abi(), stream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn AddEffectAsync<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IPropertySet>>(&self, mediastreamtype: MediaStreamType, effectactivationid: Param1, effectsettings: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AddEffectAsync)(::windows_core::Interface::as_raw(this), mediastreamtype, effectactivationid.into_param().abi(), effectsettings.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ClearEffectsAsync(&self, mediastreamtype: MediaStreamType) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ClearEffectsAsync)(::windows_core::Interface::as_raw(this), mediastreamtype, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn SetEncoderProperty<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param2: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, mediastreamtype: MediaStreamType, propertyid: Param1, propertyvalue: Param2) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEncoderProperty)(::windows_core::Interface::as_raw(this), mediastreamtype, propertyid.into_param().abi(), propertyvalue.into_param().abi()).ok() }
    }
    pub fn GetEncoderProperty<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, mediastreamtype: MediaStreamType, propertyid: Param1) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).GetEncoderProperty)(::windows_core::Interface::as_raw(this), mediastreamtype, propertyid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Failed<'a, Param0: ::windows_core::IntoParam<'a, MediaCaptureFailedEventHandler>>(&self, erroreventhandler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Failed)(::windows_core::Interface::as_raw(this), erroreventhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveFailed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFailed)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
    pub fn RecordLimitationExceeded<'a, Param0: ::windows_core::IntoParam<'a, RecordLimitationExceededEventHandler>>(&self, recordlimitationexceededeventhandler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).RecordLimitationExceeded)(::windows_core::Interface::as_raw(this), recordlimitationexceededeventhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveRecordLimitationExceeded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRecordLimitationExceeded)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
    pub fn MediaCaptureSettings(&self) -> ::windows_core::Result<MediaCaptureSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MediaCaptureSettings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaCaptureSettings>(result__)
        }
    }
    #[cfg(feature = "Media_Devices")]
    pub fn AudioDeviceController(&self) -> ::windows_core::Result<super::Devices::AudioDeviceController> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AudioDeviceController)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Devices::AudioDeviceController>(result__)
        }
    }
    #[cfg(feature = "Media_Devices")]
    pub fn VideoDeviceController(&self) -> ::windows_core::Result<super::Devices::VideoDeviceController> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoDeviceController)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Devices::VideoDeviceController>(result__)
        }
    }
    pub fn SetPreviewMirroring(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPreviewMirroring)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetPreviewMirroring(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).GetPreviewMirroring)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetPreviewRotation(&self, value: VideoRotation) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPreviewRotation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetPreviewRotation(&self) -> ::windows_core::Result<VideoRotation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<VideoRotation>::zeroed();
            (::windows_core::Interface::vtable(this).GetPreviewRotation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<VideoRotation>(result__)
        }
    }
    pub fn SetRecordRotation(&self, value: VideoRotation) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRecordRotation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetRecordRotation(&self) -> ::windows_core::Result<VideoRotation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<VideoRotation>::zeroed();
            (::windows_core::Interface::vtable(this).GetRecordRotation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<VideoRotation>(result__)
        }
    }
    #[cfg(all(feature = "Media_MediaProperties", feature = "Storage"))]
    pub fn PrepareLowLagRecordToStorageFileAsync<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFile>>(&self, encodingprofile: Param0, file: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<LowLagMediaRecording>> {
        let this = &::windows_core::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PrepareLowLagRecordToStorageFileAsync)(::windows_core::Interface::as_raw(this), encodingprofile.into_param().abi(), file.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<LowLagMediaRecording>>(result__)
        }
    }
    #[cfg(all(feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub fn PrepareLowLagRecordToStreamAsync<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>>(&self, encodingprofile: Param0, stream: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<LowLagMediaRecording>> {
        let this = &::windows_core::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PrepareLowLagRecordToStreamAsync)(::windows_core::Interface::as_raw(this), encodingprofile.into_param().abi(), stream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<LowLagMediaRecording>>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn PrepareLowLagRecordToCustomSinkAsync<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows_core::IntoParam<'a, super::IMediaExtension>>(&self, encodingprofile: Param0, custommediasink: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<LowLagMediaRecording>> {
        let this = &::windows_core::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PrepareLowLagRecordToCustomSinkAsync)(::windows_core::Interface::as_raw(this), encodingprofile.into_param().abi(), custommediasink.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<LowLagMediaRecording>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub fn PrepareLowLagRecordToCustomSinkIdAsync<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IPropertySet>>(&self, encodingprofile: Param0, customsinkactivationid: Param1, customsinksettings: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<LowLagMediaRecording>> {
        let this = &::windows_core::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PrepareLowLagRecordToCustomSinkIdAsync)(::windows_core::Interface::as_raw(this), encodingprofile.into_param().abi(), customsinkactivationid.into_param().abi(), customsinksettings.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<LowLagMediaRecording>>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn PrepareLowLagPhotoCaptureAsync<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProperties::ImageEncodingProperties>>(&self, r#type: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<LowLagPhotoCapture>> {
        let this = &::windows_core::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PrepareLowLagPhotoCaptureAsync)(::windows_core::Interface::as_raw(this), r#type.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<LowLagPhotoCapture>>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn PrepareLowLagPhotoSequenceCaptureAsync<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProperties::ImageEncodingProperties>>(&self, r#type: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<LowLagPhotoSequenceCapture>> {
        let this = &::windows_core::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PrepareLowLagPhotoSequenceCaptureAsync)(::windows_core::Interface::as_raw(this), r#type.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<LowLagPhotoSequenceCapture>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub fn SetEncodingPropertiesAsync<'a, Param1: ::windows_core::IntoParam<'a, super::MediaProperties::IMediaEncodingProperties>, Param2: ::windows_core::IntoParam<'a, super::MediaProperties::MediaPropertySet>>(&self, mediastreamtype: MediaStreamType, mediaencodingproperties: Param1, encoderproperties: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetEncodingPropertiesAsync)(::windows_core::Interface::as_raw(this), mediastreamtype, mediaencodingproperties.into_param().abi(), encoderproperties.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Media_Capture_Core", feature = "Media_MediaProperties"))]
    pub fn PrepareVariablePhotoSequenceCaptureAsync<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProperties::ImageEncodingProperties>>(&self, r#type: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<Core::VariablePhotoSequenceCapture>> {
        let this = &::windows_core::Interface::cast::<IMediaCapture3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PrepareVariablePhotoSequenceCaptureAsync)(::windows_core::Interface::as_raw(this), r#type.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<Core::VariablePhotoSequenceCapture>>(result__)
        }
    }
    pub fn FocusChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaCapture, MediaCaptureFocusChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IMediaCapture3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).FocusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveFocusChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCapture3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFocusChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn PhotoConfirmationCaptured<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaCapture, PhotoConfirmationCapturedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IMediaCapture3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PhotoConfirmationCaptured)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePhotoConfirmationCaptured<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCapture3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemovePhotoConfirmationCaptured)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn AddAudioEffectAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<super::IMediaExtension>> {
        let this = &::windows_core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AddAudioEffectAsync)(::windows_core::Interface::as_raw(this), definition.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<super::IMediaExtension>>(result__)
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn AddVideoEffectAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Effects::IVideoEffectDefinition>>(&self, definition: Param0, mediastreamtype: MediaStreamType) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<super::IMediaExtension>> {
        let this = &::windows_core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AddVideoEffectAsync)(::windows_core::Interface::as_raw(this), definition.into_param().abi(), mediastreamtype, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<super::IMediaExtension>>(result__)
        }
    }
    #[cfg(feature = "Media_Devices")]
    pub fn PauseRecordAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PauseRecordAsync)(::windows_core::Interface::as_raw(this), behavior, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ResumeRecordAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResumeRecordAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn CameraStreamStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaCapture, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CameraStreamStateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCameraStreamStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCameraStreamStateChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Devices")]
    pub fn CameraStreamState(&self) -> ::windows_core::Result<super::Devices::CameraStreamState> {
        let this = &::windows_core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Devices::CameraStreamState>::zeroed();
            (::windows_core::Interface::vtable(this).CameraStreamState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Devices::CameraStreamState>(result__)
        }
    }
    pub fn GetPreviewFrameAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<super::VideoFrame>> {
        let this = &::windows_core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPreviewFrameAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<super::VideoFrame>>(result__)
        }
    }
    pub fn GetPreviewFrameCopyAsync<'a, Param0: ::windows_core::IntoParam<'a, super::VideoFrame>>(&self, destination: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<super::VideoFrame>> {
        let this = &::windows_core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPreviewFrameCopyAsync)(::windows_core::Interface::as_raw(this), destination.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<super::VideoFrame>>(result__)
        }
    }
    pub fn ThermalStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaCapture, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ThermalStatusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveThermalStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveThermalStatusChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ThermalStatus(&self) -> ::windows_core::Result<MediaCaptureThermalStatus> {
        let this = &::windows_core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaCaptureThermalStatus>::zeroed();
            (::windows_core::Interface::vtable(this).ThermalStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaCaptureThermalStatus>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn PrepareAdvancedPhotoCaptureAsync<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProperties::ImageEncodingProperties>>(&self, encodingproperties: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AdvancedPhotoCapture>> {
        let this = &::windows_core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PrepareAdvancedPhotoCaptureAsync)(::windows_core::Interface::as_raw(this), encodingproperties.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AdvancedPhotoCapture>>(result__)
        }
    }
    pub fn RemoveEffectAsync<'a, Param0: ::windows_core::IntoParam<'a, super::IMediaExtension>>(&self, effect: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveEffectAsync)(::windows_core::Interface::as_raw(this), effect.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Media_Devices")]
    pub fn PauseRecordWithResultAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<MediaCapturePauseResult>> {
        let this = &::windows_core::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PauseRecordWithResultAsync)(::windows_core::Interface::as_raw(this), behavior, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<MediaCapturePauseResult>>(result__)
        }
    }
    pub fn StopRecordWithResultAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<MediaCaptureStopResult>> {
        let this = &::windows_core::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StopRecordWithResultAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<MediaCaptureStopResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    pub fn FrameSources(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, Frames::MediaFrameSource>> {
        let this = &::windows_core::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FrameSources)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, Frames::MediaFrameSource>>(result__)
        }
    }
    #[cfg(feature = "Media_Capture_Frames")]
    pub fn CreateFrameReaderAsync<'a, Param0: ::windows_core::IntoParam<'a, Frames::MediaFrameSource>>(&self, inputsource: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<Frames::MediaFrameReader>> {
        let this = &::windows_core::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFrameReaderAsync)(::windows_core::Interface::as_raw(this), inputsource.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<Frames::MediaFrameReader>>(result__)
        }
    }
    #[cfg(feature = "Media_Capture_Frames")]
    pub fn CreateFrameReaderWithSubtypeAsync<'a, Param0: ::windows_core::IntoParam<'a, Frames::MediaFrameSource>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, inputsource: Param0, outputsubtype: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<Frames::MediaFrameReader>> {
        let this = &::windows_core::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFrameReaderWithSubtypeAsync)(::windows_core::Interface::as_raw(this), inputsource.into_param().abi(), outputsubtype.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<Frames::MediaFrameReader>>(result__)
        }
    }
    #[cfg(all(feature = "Graphics_Imaging", feature = "Media_Capture_Frames"))]
    pub fn CreateFrameReaderWithSubtypeAndSizeAsync<'a, Param0: ::windows_core::IntoParam<'a, Frames::MediaFrameSource>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapSize>>(&self, inputsource: Param0, outputsubtype: Param1, outputsize: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<Frames::MediaFrameReader>> {
        let this = &::windows_core::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFrameReaderWithSubtypeAndSizeAsync)(::windows_core::Interface::as_raw(this), inputsource.into_param().abi(), outputsubtype.into_param().abi(), outputsize.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<Frames::MediaFrameReader>>(result__)
        }
    }
    pub fn CaptureDeviceExclusiveControlStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaCapture, MediaCaptureDeviceExclusiveControlStatusChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IMediaCapture6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CaptureDeviceExclusiveControlStatusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCaptureDeviceExclusiveControlStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCapture6>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCaptureDeviceExclusiveControlStatusChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    pub fn CreateMultiSourceFrameReaderAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<Frames::MediaFrameSource>>>(&self, inputsources: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<Frames::MultiSourceMediaFrameReader>> {
        let this = &::windows_core::Interface::cast::<IMediaCapture6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateMultiSourceFrameReaderAsync)(::windows_core::Interface::as_raw(this), inputsources.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<Frames::MultiSourceMediaFrameReader>>(result__)
        }
    }
    #[cfg(feature = "UI_WindowManagement")]
    pub fn CreateRelativePanelWatcher<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_ui::WindowManagement::DisplayRegion>>(&self, capturemode: StreamingCaptureMode, displayregion: Param1) -> ::windows_core::Result<MediaCaptureRelativePanelWatcher> {
        let this = &::windows_core::Interface::cast::<IMediaCapture7>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateRelativePanelWatcher)(::windows_core::Interface::as_raw(this), capturemode, displayregion.into_param().abi(), result__.as_mut_ptr()).from_abi::<MediaCaptureRelativePanelWatcher>(result__)
        }
    }
    pub fn IsVideoProfileSupported<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(videodeviceid: Param0) -> ::windows_core::Result<bool> {
        Self::IMediaCaptureStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsVideoProfileSupported)(::windows_core::Interface::as_raw(this), videodeviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllVideoProfiles<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(videodeviceid: Param0) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<MediaCaptureVideoProfile>> {
        Self::IMediaCaptureStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAllVideoProfiles)(::windows_core::Interface::as_raw(this), videodeviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<MediaCaptureVideoProfile>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindConcurrentProfiles<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(videodeviceid: Param0) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<MediaCaptureVideoProfile>> {
        Self::IMediaCaptureStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindConcurrentProfiles)(::windows_core::Interface::as_raw(this), videodeviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<MediaCaptureVideoProfile>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindKnownVideoProfiles<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(videodeviceid: Param0, name: KnownVideoProfile) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<MediaCaptureVideoProfile>> {
        Self::IMediaCaptureStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindKnownVideoProfiles)(::windows_core::Interface::as_raw(this), videodeviceid.into_param().abi(), name, result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<MediaCaptureVideoProfile>>(result__)
        })
    }
    pub fn StartPreviewAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureVideoPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartPreviewAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn StartPreviewToCustomSinkAsync<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows_core::IntoParam<'a, super::IMediaExtension>>(&self, encodingprofile: Param0, custommediasink: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureVideoPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartPreviewToCustomSinkAsync)(::windows_core::Interface::as_raw(this), encodingprofile.into_param().abi(), custommediasink.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub fn StartPreviewToCustomSinkIdAsync<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IPropertySet>>(&self, encodingprofile: Param0, customsinkactivationid: Param1, customsinksettings: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureVideoPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartPreviewToCustomSinkIdAsync)(::windows_core::Interface::as_raw(this), encodingprofile.into_param().abi(), customsinkactivationid.into_param().abi(), customsinksettings.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn StopPreviewAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureVideoPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StopPreviewAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn IMediaCaptureStatics<R, F: FnOnce(&IMediaCaptureStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MediaCapture, IMediaCaptureStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MediaCapture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaCapture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaCapture {}
impl ::core::fmt::Debug for MediaCapture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCapture").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaCapture {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCapture;{c61afbb4-fb10-4a34-ac18-ca80d9c8e7ee})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaCapture {
    type Vtable = IMediaCapture_Vtbl;
    const IID: ::windows_core::GUID = <IMediaCapture as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaCapture {
    const NAME: &'static str = "Windows.Media.Capture.MediaCapture";
}
impl ::core::convert::From<MediaCapture> for ::windows_core::IUnknown {
    fn from(value: MediaCapture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCapture> for ::windows_core::IUnknown {
    fn from(value: &MediaCapture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaCapture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaCapture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaCapture> for ::windows_core::IInspectable {
    fn from(value: MediaCapture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCapture> for ::windows_core::IInspectable {
    fn from(value: &MediaCapture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaCapture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaCapture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MediaCapture> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: MediaCapture) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MediaCapture> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &MediaCapture) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for MediaCapture {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &MediaCapture {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaCaptureDeviceExclusiveControlStatus(pub i32);
impl MediaCaptureDeviceExclusiveControlStatus {
    pub const ExclusiveControlAvailable: Self = Self(0i32);
    pub const SharedReadOnlyAvailable: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaCaptureDeviceExclusiveControlStatus {}
impl ::core::clone::Clone for MediaCaptureDeviceExclusiveControlStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaCaptureDeviceExclusiveControlStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaCaptureDeviceExclusiveControlStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaCaptureDeviceExclusiveControlStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureDeviceExclusiveControlStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaCaptureDeviceExclusiveControlStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCaptureDeviceExclusiveControlStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MediaCaptureDeviceExclusiveControlStatusChangedEventArgs(::windows_core::IUnknown);
impl MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<MediaCaptureDeviceExclusiveControlStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaCaptureDeviceExclusiveControlStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaCaptureDeviceExclusiveControlStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {}
impl ::core::fmt::Debug for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureDeviceExclusiveControlStatusChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureDeviceExclusiveControlStatusChangedEventArgs;{9d2f920d-a588-43c6-89d6-5ad322af006a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    type Vtable = IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureDeviceExclusiveControlStatusChangedEventArgs";
}
impl ::core::convert::From<MediaCaptureDeviceExclusiveControlStatusChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaCaptureDeviceExclusiveControlStatusChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureDeviceExclusiveControlStatusChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaCaptureDeviceExclusiveControlStatusChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaCaptureDeviceExclusiveControlStatusChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaCaptureDeviceExclusiveControlStatusChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureDeviceExclusiveControlStatusChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaCaptureDeviceExclusiveControlStatusChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {}
#[repr(transparent)]
pub struct MediaCaptureFailedEventArgs(::windows_core::IUnknown);
impl MediaCaptureFailedEventArgs {
    pub fn Message(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Message)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Code(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Code)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaCaptureFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaCaptureFailedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaCaptureFailedEventArgs {}
impl ::core::fmt::Debug for MediaCaptureFailedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureFailedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaCaptureFailedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureFailedEventArgs;{80fde3f4-54c4-42c0-8d19-cea1a87ca18b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaCaptureFailedEventArgs {
    type Vtable = IMediaCaptureFailedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaCaptureFailedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaCaptureFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureFailedEventArgs";
}
impl ::core::convert::From<MediaCaptureFailedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaCaptureFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureFailedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaCaptureFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaCaptureFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaCaptureFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaCaptureFailedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaCaptureFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureFailedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaCaptureFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaCaptureFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaCaptureFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct MediaCaptureFailedEventHandler(pub ::windows_core::IUnknown);
impl MediaCaptureFailedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<MediaCapture>, &::core::option::Option<MediaCaptureFailedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = MediaCaptureFailedEventHandlerBox::<F> { vtable: &MediaCaptureFailedEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, MediaCapture>, Param1: ::windows_core::IntoParam<'a, MediaCaptureFailedEventArgs>>(&self, sender: Param0, erroreventargs: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), erroreventargs.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct MediaCaptureFailedEventHandlerBox<F: FnMut(&::core::option::Option<MediaCapture>, &::core::option::Option<MediaCaptureFailedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const MediaCaptureFailedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<MediaCapture>, &::core::option::Option<MediaCaptureFailedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> MediaCaptureFailedEventHandlerBox<F> {
    const VTABLE: MediaCaptureFailedEventHandler_Vtbl = MediaCaptureFailedEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<MediaCaptureFailedEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows_core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: ::windows_core::RawPtr, erroreventargs: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&erroreventargs)).into()
    }
}
impl ::core::clone::Clone for MediaCaptureFailedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaCaptureFailedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaCaptureFailedEventHandler {}
impl ::core::fmt::Debug for MediaCaptureFailedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureFailedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for MediaCaptureFailedEventHandler {
    type Vtable = MediaCaptureFailedEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2014effb_5cd8_4f08_a314_0d360da59f14);
}
unsafe impl ::windows_core::RuntimeType for MediaCaptureFailedEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{2014effb-5cd8-4f08-a314-0d360da59f14}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct MediaCaptureFailedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows_core::RawPtr, erroreventargs: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct MediaCaptureFocusChangedEventArgs(::windows_core::IUnknown);
impl MediaCaptureFocusChangedEventArgs {
    #[cfg(feature = "Media_Devices")]
    pub fn FocusState(&self) -> ::windows_core::Result<super::Devices::MediaCaptureFocusState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Devices::MediaCaptureFocusState>::zeroed();
            (::windows_core::Interface::vtable(this).FocusState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Devices::MediaCaptureFocusState>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaCaptureFocusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaCaptureFocusChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaCaptureFocusChangedEventArgs {}
impl ::core::fmt::Debug for MediaCaptureFocusChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureFocusChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaCaptureFocusChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureFocusChangedEventArgs;{81e1bc7f-2277-493e-abee-d3f44ff98c04})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaCaptureFocusChangedEventArgs {
    type Vtable = IMediaCaptureFocusChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaCaptureFocusChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaCaptureFocusChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureFocusChangedEventArgs";
}
impl ::core::convert::From<MediaCaptureFocusChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaCaptureFocusChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureFocusChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaCaptureFocusChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaCaptureFocusChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaCaptureFocusChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaCaptureFocusChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaCaptureFocusChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureFocusChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaCaptureFocusChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaCaptureFocusChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaCaptureFocusChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaCaptureFocusChangedEventArgs {}
unsafe impl ::core::marker::Sync for MediaCaptureFocusChangedEventArgs {}
#[repr(transparent)]
pub struct MediaCaptureInitializationSettings(::windows_core::IUnknown);
impl MediaCaptureInitializationSettings {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MediaCaptureInitializationSettings, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn SetAudioDeviceId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAudioDeviceId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn AudioDeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AudioDeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetVideoDeviceId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVideoDeviceId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn VideoDeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoDeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetStreamingCaptureMode(&self, value: StreamingCaptureMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStreamingCaptureMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StreamingCaptureMode(&self) -> ::windows_core::Result<StreamingCaptureMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<StreamingCaptureMode>::zeroed();
            (::windows_core::Interface::vtable(this).StreamingCaptureMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StreamingCaptureMode>(result__)
        }
    }
    pub fn SetPhotoCaptureSource(&self, value: PhotoCaptureSource) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPhotoCaptureSource)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PhotoCaptureSource(&self) -> ::windows_core::Result<PhotoCaptureSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhotoCaptureSource>::zeroed();
            (::windows_core::Interface::vtable(this).PhotoCaptureSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoCaptureSource>(result__)
        }
    }
    pub fn SetMediaCategory(&self, value: MediaCategory) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureInitializationSettings2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMediaCategory)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MediaCategory(&self) -> ::windows_core::Result<MediaCategory> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureInitializationSettings2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaCategory>::zeroed();
            (::windows_core::Interface::vtable(this).MediaCategory)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaCategory>(result__)
        }
    }
    pub fn SetAudioProcessing(&self, value: super::AudioProcessing) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureInitializationSettings2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAudioProcessing)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AudioProcessing(&self) -> ::windows_core::Result<super::AudioProcessing> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureInitializationSettings2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::AudioProcessing>::zeroed();
            (::windows_core::Interface::vtable(this).AudioProcessing)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::AudioProcessing>(result__)
        }
    }
    #[cfg(feature = "Media_Core")]
    pub fn SetAudioSource<'a, Param0: ::windows_core::IntoParam<'a, super::Core::IMediaSource>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureInitializationSettings3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAudioSource)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Core")]
    pub fn AudioSource(&self) -> ::windows_core::Result<super::Core::IMediaSource> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureInitializationSettings3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AudioSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Core::IMediaSource>(result__)
        }
    }
    #[cfg(feature = "Media_Core")]
    pub fn SetVideoSource<'a, Param0: ::windows_core::IntoParam<'a, super::Core::IMediaSource>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureInitializationSettings3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetVideoSource)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Core")]
    pub fn VideoSource(&self) -> ::windows_core::Result<super::Core::IMediaSource> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureInitializationSettings3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Core::IMediaSource>(result__)
        }
    }
    pub fn VideoProfile(&self) -> ::windows_core::Result<MediaCaptureVideoProfile> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoProfile)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaCaptureVideoProfile>(result__)
        }
    }
    pub fn SetVideoProfile<'a, Param0: ::windows_core::IntoParam<'a, MediaCaptureVideoProfile>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetVideoProfile)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn PreviewMediaDescription(&self) -> ::windows_core::Result<MediaCaptureVideoProfileMediaDescription> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PreviewMediaDescription)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaCaptureVideoProfileMediaDescription>(result__)
        }
    }
    pub fn SetPreviewMediaDescription<'a, Param0: ::windows_core::IntoParam<'a, MediaCaptureVideoProfileMediaDescription>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPreviewMediaDescription)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RecordMediaDescription(&self) -> ::windows_core::Result<MediaCaptureVideoProfileMediaDescription> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RecordMediaDescription)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaCaptureVideoProfileMediaDescription>(result__)
        }
    }
    pub fn SetRecordMediaDescription<'a, Param0: ::windows_core::IntoParam<'a, MediaCaptureVideoProfileMediaDescription>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRecordMediaDescription)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn PhotoMediaDescription(&self) -> ::windows_core::Result<MediaCaptureVideoProfileMediaDescription> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PhotoMediaDescription)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaCaptureVideoProfileMediaDescription>(result__)
        }
    }
    pub fn SetPhotoMediaDescription<'a, Param0: ::windows_core::IntoParam<'a, MediaCaptureVideoProfileMediaDescription>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPhotoMediaDescription)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Capture_Frames")]
    pub fn SourceGroup(&self) -> ::windows_core::Result<Frames::MediaFrameSourceGroup> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SourceGroup)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Frames::MediaFrameSourceGroup>(result__)
        }
    }
    #[cfg(feature = "Media_Capture_Frames")]
    pub fn SetSourceGroup<'a, Param0: ::windows_core::IntoParam<'a, Frames::MediaFrameSourceGroup>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSourceGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SharingMode(&self) -> ::windows_core::Result<MediaCaptureSharingMode> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaCaptureSharingMode>::zeroed();
            (::windows_core::Interface::vtable(this).SharingMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaCaptureSharingMode>(result__)
        }
    }
    pub fn SetSharingMode(&self, value: MediaCaptureSharingMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSharingMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MemoryPreference(&self) -> ::windows_core::Result<MediaCaptureMemoryPreference> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaCaptureMemoryPreference>::zeroed();
            (::windows_core::Interface::vtable(this).MemoryPreference)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaCaptureMemoryPreference>(result__)
        }
    }
    pub fn SetMemoryPreference(&self, value: MediaCaptureMemoryPreference) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMemoryPreference)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AlwaysPlaySystemShutterSound(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureInitializationSettings6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AlwaysPlaySystemShutterSound)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAlwaysPlaySystemShutterSound(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureInitializationSettings6>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAlwaysPlaySystemShutterSound)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn DeviceUriPasswordCredential(&self) -> ::windows_core::Result<::winrt_security::Credentials::PasswordCredential> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureInitializationSettings7>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceUriPasswordCredential)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn SetDeviceUriPasswordCredential<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureInitializationSettings7>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDeviceUriPasswordCredential)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DeviceUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureInitializationSettings7>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetDeviceUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureInitializationSettings7>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDeviceUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for MediaCaptureInitializationSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaCaptureInitializationSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaCaptureInitializationSettings {}
impl ::core::fmt::Debug for MediaCaptureInitializationSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureInitializationSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaCaptureInitializationSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureInitializationSettings;{9782ba70-ea65-4900-9356-8ca887726884})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaCaptureInitializationSettings {
    type Vtable = IMediaCaptureInitializationSettings_Vtbl;
    const IID: ::windows_core::GUID = <IMediaCaptureInitializationSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaCaptureInitializationSettings {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureInitializationSettings";
}
impl ::core::convert::From<MediaCaptureInitializationSettings> for ::windows_core::IUnknown {
    fn from(value: MediaCaptureInitializationSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureInitializationSettings> for ::windows_core::IUnknown {
    fn from(value: &MediaCaptureInitializationSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaCaptureInitializationSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaCaptureInitializationSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaCaptureInitializationSettings> for ::windows_core::IInspectable {
    fn from(value: MediaCaptureInitializationSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureInitializationSettings> for ::windows_core::IInspectable {
    fn from(value: &MediaCaptureInitializationSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaCaptureInitializationSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaCaptureInitializationSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaCaptureInitializationSettings {}
unsafe impl ::core::marker::Sync for MediaCaptureInitializationSettings {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaCaptureMemoryPreference(pub i32);
impl MediaCaptureMemoryPreference {
    pub const Auto: Self = Self(0i32);
    pub const Cpu: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaCaptureMemoryPreference {}
impl ::core::clone::Clone for MediaCaptureMemoryPreference {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaCaptureMemoryPreference {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaCaptureMemoryPreference {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaCaptureMemoryPreference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureMemoryPreference").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaCaptureMemoryPreference {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCaptureMemoryPreference;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MediaCapturePauseResult(::windows_core::IUnknown);
impl MediaCapturePauseResult {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn LastFrame(&self) -> ::windows_core::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LastFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::VideoFrame>(result__)
        }
    }
    pub fn RecordDuration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).RecordDuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaCapturePauseResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaCapturePauseResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaCapturePauseResult {}
impl ::core::fmt::Debug for MediaCapturePauseResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCapturePauseResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaCapturePauseResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCapturePauseResult;{aec47ca3-4477-4b04-a06f-2c1c5182fe9d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaCapturePauseResult {
    type Vtable = IMediaCapturePauseResult_Vtbl;
    const IID: ::windows_core::GUID = <IMediaCapturePauseResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaCapturePauseResult {
    const NAME: &'static str = "Windows.Media.Capture.MediaCapturePauseResult";
}
impl ::core::convert::From<MediaCapturePauseResult> for ::windows_core::IUnknown {
    fn from(value: MediaCapturePauseResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCapturePauseResult> for ::windows_core::IUnknown {
    fn from(value: &MediaCapturePauseResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaCapturePauseResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaCapturePauseResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaCapturePauseResult> for ::windows_core::IInspectable {
    fn from(value: MediaCapturePauseResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCapturePauseResult> for ::windows_core::IInspectable {
    fn from(value: &MediaCapturePauseResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaCapturePauseResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaCapturePauseResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MediaCapturePauseResult> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: MediaCapturePauseResult) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MediaCapturePauseResult> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &MediaCapturePauseResult) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for MediaCapturePauseResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &MediaCapturePauseResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[repr(transparent)]
pub struct MediaCaptureRelativePanelWatcher(::windows_core::IUnknown);
impl MediaCaptureRelativePanelWatcher {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn RelativePanel(&self) -> ::windows_core::Result<::winrt_devices::Enumeration::Panel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_devices::Enumeration::Panel>::zeroed();
            (::windows_core::Interface::vtable(this).RelativePanel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Enumeration::Panel>(result__)
        }
    }
    pub fn Changed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaCaptureRelativePanelWatcher, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Changed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for MediaCaptureRelativePanelWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaCaptureRelativePanelWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaCaptureRelativePanelWatcher {}
impl ::core::fmt::Debug for MediaCaptureRelativePanelWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureRelativePanelWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaCaptureRelativePanelWatcher {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureRelativePanelWatcher;{7d896566-04be-5b89-b30e-bd34a9f12db0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaCaptureRelativePanelWatcher {
    type Vtable = IMediaCaptureRelativePanelWatcher_Vtbl;
    const IID: ::windows_core::GUID = <IMediaCaptureRelativePanelWatcher as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaCaptureRelativePanelWatcher {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureRelativePanelWatcher";
}
impl ::core::convert::From<MediaCaptureRelativePanelWatcher> for ::windows_core::IUnknown {
    fn from(value: MediaCaptureRelativePanelWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureRelativePanelWatcher> for ::windows_core::IUnknown {
    fn from(value: &MediaCaptureRelativePanelWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaCaptureRelativePanelWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaCaptureRelativePanelWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaCaptureRelativePanelWatcher> for ::windows_core::IInspectable {
    fn from(value: MediaCaptureRelativePanelWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureRelativePanelWatcher> for ::windows_core::IInspectable {
    fn from(value: &MediaCaptureRelativePanelWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaCaptureRelativePanelWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaCaptureRelativePanelWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MediaCaptureRelativePanelWatcher> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: MediaCaptureRelativePanelWatcher) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MediaCaptureRelativePanelWatcher> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &MediaCaptureRelativePanelWatcher) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for MediaCaptureRelativePanelWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &MediaCaptureRelativePanelWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MediaCaptureRelativePanelWatcher {}
unsafe impl ::core::marker::Sync for MediaCaptureRelativePanelWatcher {}
#[repr(transparent)]
pub struct MediaCaptureSettings(::windows_core::IUnknown);
impl MediaCaptureSettings {
    pub fn AudioDeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AudioDeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn VideoDeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoDeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn StreamingCaptureMode(&self) -> ::windows_core::Result<StreamingCaptureMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<StreamingCaptureMode>::zeroed();
            (::windows_core::Interface::vtable(this).StreamingCaptureMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StreamingCaptureMode>(result__)
        }
    }
    pub fn PhotoCaptureSource(&self) -> ::windows_core::Result<PhotoCaptureSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhotoCaptureSource>::zeroed();
            (::windows_core::Interface::vtable(this).PhotoCaptureSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoCaptureSource>(result__)
        }
    }
    pub fn VideoDeviceCharacteristic(&self) -> ::windows_core::Result<VideoDeviceCharacteristic> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<VideoDeviceCharacteristic>::zeroed();
            (::windows_core::Interface::vtable(this).VideoDeviceCharacteristic)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<VideoDeviceCharacteristic>(result__)
        }
    }
    pub fn ConcurrentRecordAndPhotoSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ConcurrentRecordAndPhotoSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ConcurrentRecordAndPhotoSequenceSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ConcurrentRecordAndPhotoSequenceSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CameraSoundRequiredForRegion(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CameraSoundRequiredForRegion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Horizontal35mmEquivalentFocalLength(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u32>> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Horizontal35mmEquivalentFocalLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u32>>(result__)
        }
    }
    pub fn PitchOffsetDegrees(&self) -> ::windows_core::Result<::winrt_foundation::IReference<i32>> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PitchOffsetDegrees)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<i32>>(result__)
        }
    }
    pub fn Vertical35mmEquivalentFocalLength(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u32>> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Vertical35mmEquivalentFocalLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u32>>(result__)
        }
    }
    pub fn MediaCategory(&self) -> ::windows_core::Result<MediaCategory> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaCategory>::zeroed();
            (::windows_core::Interface::vtable(this).MediaCategory)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaCategory>(result__)
        }
    }
    pub fn AudioProcessing(&self) -> ::windows_core::Result<super::AudioProcessing> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::AudioProcessing>::zeroed();
            (::windows_core::Interface::vtable(this).AudioProcessing)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::AudioProcessing>(result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Direct3D11Device(&self) -> ::windows_core::Result<::winrt_graphics::DirectX::Direct3D11::IDirect3DDevice> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureSettings3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Direct3D11Device)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::DirectX::Direct3D11::IDirect3DDevice>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaCaptureSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaCaptureSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaCaptureSettings {}
impl ::core::fmt::Debug for MediaCaptureSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaCaptureSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureSettings;{1d83aafe-6d45-4477-8dc4-ac5bc01c4091})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaCaptureSettings {
    type Vtable = IMediaCaptureSettings_Vtbl;
    const IID: ::windows_core::GUID = <IMediaCaptureSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureSettings";
}
impl ::core::convert::From<MediaCaptureSettings> for ::windows_core::IUnknown {
    fn from(value: MediaCaptureSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureSettings> for ::windows_core::IUnknown {
    fn from(value: &MediaCaptureSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaCaptureSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaCaptureSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaCaptureSettings> for ::windows_core::IInspectable {
    fn from(value: MediaCaptureSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureSettings> for ::windows_core::IInspectable {
    fn from(value: &MediaCaptureSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaCaptureSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaCaptureSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaCaptureSharingMode(pub i32);
impl MediaCaptureSharingMode {
    pub const ExclusiveControl: Self = Self(0i32);
    pub const SharedReadOnly: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaCaptureSharingMode {}
impl ::core::clone::Clone for MediaCaptureSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaCaptureSharingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaCaptureSharingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaCaptureSharingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureSharingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaCaptureSharingMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCaptureSharingMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MediaCaptureStopResult(::windows_core::IUnknown);
impl MediaCaptureStopResult {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn LastFrame(&self) -> ::windows_core::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LastFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::VideoFrame>(result__)
        }
    }
    pub fn RecordDuration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).RecordDuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaCaptureStopResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaCaptureStopResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaCaptureStopResult {}
impl ::core::fmt::Debug for MediaCaptureStopResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureStopResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaCaptureStopResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureStopResult;{f9db6a2a-a092-4ad1-97d4-f201f9d082db})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaCaptureStopResult {
    type Vtable = IMediaCaptureStopResult_Vtbl;
    const IID: ::windows_core::GUID = <IMediaCaptureStopResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaCaptureStopResult {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureStopResult";
}
impl ::core::convert::From<MediaCaptureStopResult> for ::windows_core::IUnknown {
    fn from(value: MediaCaptureStopResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureStopResult> for ::windows_core::IUnknown {
    fn from(value: &MediaCaptureStopResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaCaptureStopResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaCaptureStopResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaCaptureStopResult> for ::windows_core::IInspectable {
    fn from(value: MediaCaptureStopResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureStopResult> for ::windows_core::IInspectable {
    fn from(value: &MediaCaptureStopResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaCaptureStopResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaCaptureStopResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MediaCaptureStopResult> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: MediaCaptureStopResult) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MediaCaptureStopResult> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &MediaCaptureStopResult) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for MediaCaptureStopResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &MediaCaptureStopResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaCaptureThermalStatus(pub i32);
impl MediaCaptureThermalStatus {
    pub const Normal: Self = Self(0i32);
    pub const Overheated: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaCaptureThermalStatus {}
impl ::core::clone::Clone for MediaCaptureThermalStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaCaptureThermalStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaCaptureThermalStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaCaptureThermalStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureThermalStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaCaptureThermalStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCaptureThermalStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MediaCaptureVideoProfile(::windows_core::IUnknown);
impl MediaCaptureVideoProfile {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn VideoDeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoDeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedPreviewMediaDescription(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedPreviewMediaDescription)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedRecordMediaDescription(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedRecordMediaDescription)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedPhotoMediaDescription(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedPhotoMediaDescription)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConcurrency(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<MediaCaptureVideoProfile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetConcurrency)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<MediaCaptureVideoProfile>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    pub fn FrameSourceInfos(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<Frames::MediaFrameSourceInfo>> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureVideoProfile2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FrameSourceInfos)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<Frames::MediaFrameSourceInfo>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::GUID, ::windows_core::IInspectable>> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureVideoProfile2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::GUID, ::windows_core::IInspectable>>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaCaptureVideoProfile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaCaptureVideoProfile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaCaptureVideoProfile {}
impl ::core::fmt::Debug for MediaCaptureVideoProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureVideoProfile").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaCaptureVideoProfile {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureVideoProfile;{21a073bf-a3ee-4ecf-9ef6-50b0bc4e1305})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaCaptureVideoProfile {
    type Vtable = IMediaCaptureVideoProfile_Vtbl;
    const IID: ::windows_core::GUID = <IMediaCaptureVideoProfile as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaCaptureVideoProfile {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureVideoProfile";
}
impl ::core::convert::From<MediaCaptureVideoProfile> for ::windows_core::IUnknown {
    fn from(value: MediaCaptureVideoProfile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureVideoProfile> for ::windows_core::IUnknown {
    fn from(value: &MediaCaptureVideoProfile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaCaptureVideoProfile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaCaptureVideoProfile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaCaptureVideoProfile> for ::windows_core::IInspectable {
    fn from(value: MediaCaptureVideoProfile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureVideoProfile> for ::windows_core::IInspectable {
    fn from(value: &MediaCaptureVideoProfile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaCaptureVideoProfile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaCaptureVideoProfile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaCaptureVideoProfile {}
unsafe impl ::core::marker::Sync for MediaCaptureVideoProfile {}
#[repr(transparent)]
pub struct MediaCaptureVideoProfileMediaDescription(::windows_core::IUnknown);
impl MediaCaptureVideoProfileMediaDescription {
    pub fn Width(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Width)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Height(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Height)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn FrameRate(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).FrameRate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn IsVariablePhotoSequenceSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsVariablePhotoSequenceSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn IsHdrVideoSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsHdrVideoSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Subtype(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureVideoProfileMediaDescription2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Subtype)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::GUID, ::windows_core::IInspectable>> {
        let this = &::windows_core::Interface::cast::<IMediaCaptureVideoProfileMediaDescription2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::GUID, ::windows_core::IInspectable>>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaCaptureVideoProfileMediaDescription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaCaptureVideoProfileMediaDescription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaCaptureVideoProfileMediaDescription {}
impl ::core::fmt::Debug for MediaCaptureVideoProfileMediaDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureVideoProfileMediaDescription").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaCaptureVideoProfileMediaDescription {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureVideoProfileMediaDescription;{8012afef-b691-49ff-83f2-c1e76eaaea1b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaCaptureVideoProfileMediaDescription {
    type Vtable = IMediaCaptureVideoProfileMediaDescription_Vtbl;
    const IID: ::windows_core::GUID = <IMediaCaptureVideoProfileMediaDescription as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaCaptureVideoProfileMediaDescription {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureVideoProfileMediaDescription";
}
impl ::core::convert::From<MediaCaptureVideoProfileMediaDescription> for ::windows_core::IUnknown {
    fn from(value: MediaCaptureVideoProfileMediaDescription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureVideoProfileMediaDescription> for ::windows_core::IUnknown {
    fn from(value: &MediaCaptureVideoProfileMediaDescription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaCaptureVideoProfileMediaDescription {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaCaptureVideoProfileMediaDescription {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaCaptureVideoProfileMediaDescription> for ::windows_core::IInspectable {
    fn from(value: MediaCaptureVideoProfileMediaDescription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureVideoProfileMediaDescription> for ::windows_core::IInspectable {
    fn from(value: &MediaCaptureVideoProfileMediaDescription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaCaptureVideoProfileMediaDescription {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaCaptureVideoProfileMediaDescription {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaCaptureVideoProfileMediaDescription {}
unsafe impl ::core::marker::Sync for MediaCaptureVideoProfileMediaDescription {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaCategory(pub i32);
impl MediaCategory {
    pub const Other: Self = Self(0i32);
    pub const Communications: Self = Self(1i32);
    pub const Media: Self = Self(2i32);
    pub const GameChat: Self = Self(3i32);
    pub const Speech: Self = Self(4i32);
    pub const FarFieldSpeech: Self = Self(5i32);
    pub const UniformSpeech: Self = Self(6i32);
    pub const VoiceTyping: Self = Self(7i32);
}
impl ::core::marker::Copy for MediaCategory {}
impl ::core::clone::Clone for MediaCategory {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaCategory {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaCategory {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCategory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaCategory {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCategory;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaStreamType(pub i32);
impl MediaStreamType {
    pub const VideoPreview: Self = Self(0i32);
    pub const VideoRecord: Self = Self(1i32);
    pub const Audio: Self = Self(2i32);
    pub const Photo: Self = Self(3i32);
    pub const Metadata: Self = Self(4i32);
}
impl ::core::marker::Copy for MediaStreamType {}
impl ::core::clone::Clone for MediaStreamType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaStreamType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaStreamType {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaStreamType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaStreamType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaStreamType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct OptionalReferencePhotoCapturedEventArgs(::windows_core::IUnknown);
impl OptionalReferencePhotoCapturedEventArgs {
    pub fn Frame(&self) -> ::windows_core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Frame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CapturedFrame>(result__)
        }
    }
    pub fn Context(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Context)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for OptionalReferencePhotoCapturedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OptionalReferencePhotoCapturedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OptionalReferencePhotoCapturedEventArgs {}
impl ::core::fmt::Debug for OptionalReferencePhotoCapturedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OptionalReferencePhotoCapturedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for OptionalReferencePhotoCapturedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.OptionalReferencePhotoCapturedEventArgs;{470f88b3-1e6d-4051-9c8b-f1d85af047b7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for OptionalReferencePhotoCapturedEventArgs {
    type Vtable = IOptionalReferencePhotoCapturedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IOptionalReferencePhotoCapturedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for OptionalReferencePhotoCapturedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.OptionalReferencePhotoCapturedEventArgs";
}
impl ::core::convert::From<OptionalReferencePhotoCapturedEventArgs> for ::windows_core::IUnknown {
    fn from(value: OptionalReferencePhotoCapturedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OptionalReferencePhotoCapturedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &OptionalReferencePhotoCapturedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for OptionalReferencePhotoCapturedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a OptionalReferencePhotoCapturedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OptionalReferencePhotoCapturedEventArgs> for ::windows_core::IInspectable {
    fn from(value: OptionalReferencePhotoCapturedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OptionalReferencePhotoCapturedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &OptionalReferencePhotoCapturedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for OptionalReferencePhotoCapturedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a OptionalReferencePhotoCapturedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OptionalReferencePhotoCapturedEventArgs {}
unsafe impl ::core::marker::Sync for OptionalReferencePhotoCapturedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhotoCaptureSource(pub i32);
impl PhotoCaptureSource {
    pub const Auto: Self = Self(0i32);
    pub const VideoPreview: Self = Self(1i32);
    pub const Photo: Self = Self(2i32);
}
impl ::core::marker::Copy for PhotoCaptureSource {}
impl ::core::clone::Clone for PhotoCaptureSource {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhotoCaptureSource {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PhotoCaptureSource {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhotoCaptureSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoCaptureSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhotoCaptureSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.PhotoCaptureSource;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PhotoCapturedEventArgs(::windows_core::IUnknown);
impl PhotoCapturedEventArgs {
    pub fn Frame(&self) -> ::windows_core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Frame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CapturedFrame>(result__)
        }
    }
    pub fn Thumbnail(&self) -> ::windows_core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Thumbnail)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CapturedFrame>(result__)
        }
    }
    pub fn CaptureTimeOffset(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).CaptureTimeOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for PhotoCapturedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhotoCapturedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhotoCapturedEventArgs {}
impl ::core::fmt::Debug for PhotoCapturedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoCapturedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhotoCapturedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.PhotoCapturedEventArgs;{373bfbc1-984e-4ff0-bf85-1c00aabc5a45})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PhotoCapturedEventArgs {
    type Vtable = IPhotoCapturedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPhotoCapturedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PhotoCapturedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.PhotoCapturedEventArgs";
}
impl ::core::convert::From<PhotoCapturedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PhotoCapturedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoCapturedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PhotoCapturedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PhotoCapturedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PhotoCapturedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhotoCapturedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PhotoCapturedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoCapturedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PhotoCapturedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PhotoCapturedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PhotoCapturedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhotoCapturedEventArgs {}
unsafe impl ::core::marker::Sync for PhotoCapturedEventArgs {}
#[repr(transparent)]
pub struct PhotoConfirmationCapturedEventArgs(::windows_core::IUnknown);
impl PhotoConfirmationCapturedEventArgs {
    pub fn Frame(&self) -> ::windows_core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Frame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CapturedFrame>(result__)
        }
    }
    pub fn CaptureTimeOffset(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).CaptureTimeOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for PhotoConfirmationCapturedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhotoConfirmationCapturedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhotoConfirmationCapturedEventArgs {}
impl ::core::fmt::Debug for PhotoConfirmationCapturedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoConfirmationCapturedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhotoConfirmationCapturedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.PhotoConfirmationCapturedEventArgs;{ab473672-c28a-4827-8f8d-3636d3beb51e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PhotoConfirmationCapturedEventArgs {
    type Vtable = IPhotoConfirmationCapturedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPhotoConfirmationCapturedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PhotoConfirmationCapturedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.PhotoConfirmationCapturedEventArgs";
}
impl ::core::convert::From<PhotoConfirmationCapturedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PhotoConfirmationCapturedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoConfirmationCapturedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PhotoConfirmationCapturedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PhotoConfirmationCapturedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PhotoConfirmationCapturedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhotoConfirmationCapturedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PhotoConfirmationCapturedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoConfirmationCapturedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PhotoConfirmationCapturedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PhotoConfirmationCapturedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PhotoConfirmationCapturedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhotoConfirmationCapturedEventArgs {}
unsafe impl ::core::marker::Sync for PhotoConfirmationCapturedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PowerlineFrequency(pub i32);
impl PowerlineFrequency {
    pub const Disabled: Self = Self(0i32);
    pub const FiftyHertz: Self = Self(1i32);
    pub const SixtyHertz: Self = Self(2i32);
    pub const Auto: Self = Self(3i32);
}
impl ::core::marker::Copy for PowerlineFrequency {}
impl ::core::clone::Clone for PowerlineFrequency {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PowerlineFrequency {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PowerlineFrequency {
    type Abi = Self;
}
impl ::core::fmt::Debug for PowerlineFrequency {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PowerlineFrequency").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PowerlineFrequency {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.PowerlineFrequency;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct RecordLimitationExceededEventHandler(pub ::windows_core::IUnknown);
impl RecordLimitationExceededEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<MediaCapture>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = RecordLimitationExceededEventHandlerBox::<F> { vtable: &RecordLimitationExceededEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, MediaCapture>>(&self, sender: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct RecordLimitationExceededEventHandlerBox<F: FnMut(&::core::option::Option<MediaCapture>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const RecordLimitationExceededEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<MediaCapture>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> RecordLimitationExceededEventHandlerBox<F> {
    const VTABLE: RecordLimitationExceededEventHandler_Vtbl = RecordLimitationExceededEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<RecordLimitationExceededEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows_core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender)).into()
    }
}
impl ::core::clone::Clone for RecordLimitationExceededEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RecordLimitationExceededEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RecordLimitationExceededEventHandler {}
impl ::core::fmt::Debug for RecordLimitationExceededEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RecordLimitationExceededEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for RecordLimitationExceededEventHandler {
    type Vtable = RecordLimitationExceededEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3fae8f2e_4fe1_4ffd_aaba_e1f1337d4e53);
}
unsafe impl ::windows_core::RuntimeType for RecordLimitationExceededEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{3fae8f2e-4fe1-4ffd-aaba-e1f1337d4e53}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct RecordLimitationExceededEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ScreenCapture(::windows_core::IUnknown);
impl ScreenCapture {
    #[cfg(feature = "Media_Core")]
    pub fn AudioSource(&self) -> ::windows_core::Result<super::Core::IMediaSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AudioSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Core::IMediaSource>(result__)
        }
    }
    #[cfg(feature = "Media_Core")]
    pub fn VideoSource(&self) -> ::windows_core::Result<super::Core::IMediaSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Core::IMediaSource>(result__)
        }
    }
    pub fn IsAudioSuspended(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAudioSuspended)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsVideoSuspended(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsVideoSuspended)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SourceSuspensionChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ScreenCapture, SourceSuspensionChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SourceSuspensionChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSourceSuspensionChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSourceSuspensionChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<ScreenCapture> {
        Self::IScreenCaptureStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ScreenCapture>(result__)
        })
    }
    pub fn IScreenCaptureStatics<R, F: FnOnce(&IScreenCaptureStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ScreenCapture, IScreenCaptureStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ScreenCapture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ScreenCapture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ScreenCapture {}
impl ::core::fmt::Debug for ScreenCapture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScreenCapture").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ScreenCapture {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.ScreenCapture;{89179ef7-cd12-4e0e-a6d4-5b3de98b2e9b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ScreenCapture {
    type Vtable = IScreenCapture_Vtbl;
    const IID: ::windows_core::GUID = <IScreenCapture as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ScreenCapture {
    const NAME: &'static str = "Windows.Media.Capture.ScreenCapture";
}
impl ::core::convert::From<ScreenCapture> for ::windows_core::IUnknown {
    fn from(value: ScreenCapture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScreenCapture> for ::windows_core::IUnknown {
    fn from(value: &ScreenCapture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ScreenCapture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ScreenCapture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ScreenCapture> for ::windows_core::IInspectable {
    fn from(value: ScreenCapture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScreenCapture> for ::windows_core::IInspectable {
    fn from(value: &ScreenCapture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ScreenCapture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ScreenCapture {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ScreenCapture {}
unsafe impl ::core::marker::Sync for ScreenCapture {}
#[repr(transparent)]
pub struct SourceSuspensionChangedEventArgs(::windows_core::IUnknown);
impl SourceSuspensionChangedEventArgs {
    pub fn IsAudioSuspended(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAudioSuspended)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsVideoSuspended(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsVideoSuspended)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for SourceSuspensionChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SourceSuspensionChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SourceSuspensionChangedEventArgs {}
impl ::core::fmt::Debug for SourceSuspensionChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SourceSuspensionChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SourceSuspensionChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.SourceSuspensionChangedEventArgs;{2ece7b5e-d49b-4394-bc32-f97d6cedec1c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SourceSuspensionChangedEventArgs {
    type Vtable = ISourceSuspensionChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISourceSuspensionChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SourceSuspensionChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.SourceSuspensionChangedEventArgs";
}
impl ::core::convert::From<SourceSuspensionChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SourceSuspensionChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SourceSuspensionChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SourceSuspensionChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SourceSuspensionChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SourceSuspensionChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SourceSuspensionChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SourceSuspensionChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SourceSuspensionChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SourceSuspensionChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SourceSuspensionChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SourceSuspensionChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SourceSuspensionChangedEventArgs {}
unsafe impl ::core::marker::Sync for SourceSuspensionChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StreamingCaptureMode(pub i32);
impl StreamingCaptureMode {
    pub const AudioAndVideo: Self = Self(0i32);
    pub const Audio: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl ::core::marker::Copy for StreamingCaptureMode {}
impl ::core::clone::Clone for StreamingCaptureMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StreamingCaptureMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for StreamingCaptureMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for StreamingCaptureMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StreamingCaptureMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StreamingCaptureMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.StreamingCaptureMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VideoDeviceCharacteristic(pub i32);
impl VideoDeviceCharacteristic {
    pub const AllStreamsIndependent: Self = Self(0i32);
    pub const PreviewRecordStreamsIdentical: Self = Self(1i32);
    pub const PreviewPhotoStreamsIdentical: Self = Self(2i32);
    pub const RecordPhotoStreamsIdentical: Self = Self(3i32);
    pub const AllStreamsIdentical: Self = Self(4i32);
}
impl ::core::marker::Copy for VideoDeviceCharacteristic {}
impl ::core::clone::Clone for VideoDeviceCharacteristic {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VideoDeviceCharacteristic {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for VideoDeviceCharacteristic {
    type Abi = Self;
}
impl ::core::fmt::Debug for VideoDeviceCharacteristic {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoDeviceCharacteristic").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VideoDeviceCharacteristic {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.VideoDeviceCharacteristic;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VideoRotation(pub i32);
impl VideoRotation {
    pub const None: Self = Self(0i32);
    pub const Clockwise90Degrees: Self = Self(1i32);
    pub const Clockwise180Degrees: Self = Self(2i32);
    pub const Clockwise270Degrees: Self = Self(3i32);
}
impl ::core::marker::Copy for VideoRotation {}
impl ::core::clone::Clone for VideoRotation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VideoRotation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for VideoRotation {
    type Abi = Self;
}
impl ::core::fmt::Debug for VideoRotation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoRotation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VideoRotation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.VideoRotation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct VideoStreamConfiguration(::windows_core::IUnknown);
impl VideoStreamConfiguration {
    #[cfg(feature = "Media_MediaProperties")]
    pub fn InputProperties(&self) -> ::windows_core::Result<super::MediaProperties::VideoEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InputProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::VideoEncodingProperties>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn OutputProperties(&self) -> ::windows_core::Result<super::MediaProperties::VideoEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OutputProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::VideoEncodingProperties>(result__)
        }
    }
}
impl ::core::clone::Clone for VideoStreamConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VideoStreamConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoStreamConfiguration {}
impl ::core::fmt::Debug for VideoStreamConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoStreamConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VideoStreamConfiguration {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.VideoStreamConfiguration;{d8770a6f-4390-4b5e-ad3e-0f8af0963490})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for VideoStreamConfiguration {
    type Vtable = IVideoStreamConfiguration_Vtbl;
    const IID: ::windows_core::GUID = <IVideoStreamConfiguration as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VideoStreamConfiguration {
    const NAME: &'static str = "Windows.Media.Capture.VideoStreamConfiguration";
}
impl ::core::convert::From<VideoStreamConfiguration> for ::windows_core::IUnknown {
    fn from(value: VideoStreamConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoStreamConfiguration> for ::windows_core::IUnknown {
    fn from(value: &VideoStreamConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for VideoStreamConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a VideoStreamConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VideoStreamConfiguration> for ::windows_core::IInspectable {
    fn from(value: VideoStreamConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoStreamConfiguration> for ::windows_core::IInspectable {
    fn from(value: &VideoStreamConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for VideoStreamConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a VideoStreamConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VideoStreamConfiguration {}
unsafe impl ::core::marker::Sync for VideoStreamConfiguration {}
#[repr(C)]
pub struct WhiteBalanceGain {
    pub R: f64,
    pub G: f64,
    pub B: f64,
}
impl ::core::marker::Copy for WhiteBalanceGain {}
impl ::core::clone::Clone for WhiteBalanceGain {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WhiteBalanceGain {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WhiteBalanceGain").field("R", &self.R).field("G", &self.G).field("B", &self.B).finish()
    }
}
unsafe impl ::windows_core::Abi for WhiteBalanceGain {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for WhiteBalanceGain {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Media.Capture.WhiteBalanceGain;f8;f8;f8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for WhiteBalanceGain {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WhiteBalanceGain>()) == 0 }
    }
}
impl ::core::cmp::Eq for WhiteBalanceGain {}
impl ::core::default::Default for WhiteBalanceGain {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
