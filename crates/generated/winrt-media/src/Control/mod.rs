#[repr(transparent)]
pub struct CurrentSessionChangedEventArgs(::windows_core::IUnknown);
impl CurrentSessionChangedEventArgs {}
impl ::core::clone::Clone for CurrentSessionChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CurrentSessionChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CurrentSessionChangedEventArgs {}
impl ::core::fmt::Debug for CurrentSessionChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CurrentSessionChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CurrentSessionChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Control.CurrentSessionChangedEventArgs;{6969cb39-0bfa-5fe0-8d73-09cc5e5408e1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CurrentSessionChangedEventArgs {
    type Vtable = ICurrentSessionChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ICurrentSessionChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CurrentSessionChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Control.CurrentSessionChangedEventArgs";
}
impl ::core::convert::From<CurrentSessionChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: CurrentSessionChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CurrentSessionChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &CurrentSessionChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CurrentSessionChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CurrentSessionChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CurrentSessionChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: CurrentSessionChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CurrentSessionChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &CurrentSessionChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CurrentSessionChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CurrentSessionChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CurrentSessionChangedEventArgs {}
unsafe impl ::core::marker::Sync for CurrentSessionChangedEventArgs {}
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSession(::windows_core::IUnknown);
impl GlobalSystemMediaTransportControlsSession {
    pub fn SourceAppUserModelId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SourceAppUserModelId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn TryGetMediaPropertiesAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<GlobalSystemMediaTransportControlsSessionMediaProperties>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetMediaPropertiesAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<GlobalSystemMediaTransportControlsSessionMediaProperties>>(result__)
        }
    }
    pub fn GetTimelineProperties(&self) -> ::windows_core::Result<GlobalSystemMediaTransportControlsSessionTimelineProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetTimelineProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GlobalSystemMediaTransportControlsSessionTimelineProperties>(result__)
        }
    }
    pub fn GetPlaybackInfo(&self) -> ::windows_core::Result<GlobalSystemMediaTransportControlsSessionPlaybackInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPlaybackInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GlobalSystemMediaTransportControlsSessionPlaybackInfo>(result__)
        }
    }
    pub fn TryPlayAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryPlayAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TryPauseAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryPauseAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TryStopAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryStopAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TryRecordAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryRecordAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TryFastForwardAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryFastForwardAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TryRewindAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryRewindAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TrySkipNextAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TrySkipNextAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TrySkipPreviousAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TrySkipPreviousAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TryChangeChannelUpAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryChangeChannelUpAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TryChangeChannelDownAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryChangeChannelDownAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TryTogglePlayPauseAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryTogglePlayPauseAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TryChangeAutoRepeatModeAsync(&self, requestedautorepeatmode: super::MediaPlaybackAutoRepeatMode) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryChangeAutoRepeatModeAsync)(::windows_core::Interface::as_raw(this), requestedautorepeatmode, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TryChangePlaybackRateAsync(&self, requestedplaybackrate: f64) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryChangePlaybackRateAsync)(::windows_core::Interface::as_raw(this), requestedplaybackrate, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TryChangeShuffleActiveAsync(&self, requestedshufflestate: bool) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryChangeShuffleActiveAsync)(::windows_core::Interface::as_raw(this), requestedshufflestate, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TryChangePlaybackPositionAsync(&self, requestedplaybackposition: i64) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryChangePlaybackPositionAsync)(::windows_core::Interface::as_raw(this), requestedplaybackposition, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TimelinePropertiesChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSession, TimelinePropertiesChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).TimelinePropertiesChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveTimelinePropertiesChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveTimelinePropertiesChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn PlaybackInfoChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSession, PlaybackInfoChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackInfoChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePlaybackInfoChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePlaybackInfoChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn MediaPropertiesChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSession, MediaPropertiesChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).MediaPropertiesChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveMediaPropertiesChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMediaPropertiesChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for GlobalSystemMediaTransportControlsSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GlobalSystemMediaTransportControlsSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GlobalSystemMediaTransportControlsSession {}
impl ::core::fmt::Debug for GlobalSystemMediaTransportControlsSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GlobalSystemMediaTransportControlsSession").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GlobalSystemMediaTransportControlsSession {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Control.GlobalSystemMediaTransportControlsSession;{7148c835-9b14-5ae2-ab85-dc9b1c14e1a8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GlobalSystemMediaTransportControlsSession {
    type Vtable = IGlobalSystemMediaTransportControlsSession_Vtbl;
    const IID: ::windows_core::GUID = <IGlobalSystemMediaTransportControlsSession as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GlobalSystemMediaTransportControlsSession {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSession";
}
impl ::core::convert::From<GlobalSystemMediaTransportControlsSession> for ::windows_core::IUnknown {
    fn from(value: GlobalSystemMediaTransportControlsSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GlobalSystemMediaTransportControlsSession> for ::windows_core::IUnknown {
    fn from(value: &GlobalSystemMediaTransportControlsSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GlobalSystemMediaTransportControlsSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GlobalSystemMediaTransportControlsSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GlobalSystemMediaTransportControlsSession> for ::windows_core::IInspectable {
    fn from(value: GlobalSystemMediaTransportControlsSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GlobalSystemMediaTransportControlsSession> for ::windows_core::IInspectable {
    fn from(value: &GlobalSystemMediaTransportControlsSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GlobalSystemMediaTransportControlsSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GlobalSystemMediaTransportControlsSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GlobalSystemMediaTransportControlsSession {}
unsafe impl ::core::marker::Sync for GlobalSystemMediaTransportControlsSession {}
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSessionManager(::windows_core::IUnknown);
impl GlobalSystemMediaTransportControlsSessionManager {
    pub fn GetCurrentSession(&self) -> ::windows_core::Result<GlobalSystemMediaTransportControlsSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentSession)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GlobalSystemMediaTransportControlsSession>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetSessions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<GlobalSystemMediaTransportControlsSession>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSessions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<GlobalSystemMediaTransportControlsSession>>(result__)
        }
    }
    pub fn CurrentSessionChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSessionManager, CurrentSessionChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentSessionChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCurrentSessionChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCurrentSessionChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn SessionsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSessionManager, SessionsChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SessionsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSessionsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSessionsChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn RequestAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<GlobalSystemMediaTransportControlsSessionManager>> {
        Self::IGlobalSystemMediaTransportControlsSessionManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<GlobalSystemMediaTransportControlsSessionManager>>(result__)
        })
    }
    pub fn IGlobalSystemMediaTransportControlsSessionManagerStatics<R, F: FnOnce(&IGlobalSystemMediaTransportControlsSessionManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GlobalSystemMediaTransportControlsSessionManager, IGlobalSystemMediaTransportControlsSessionManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GlobalSystemMediaTransportControlsSessionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GlobalSystemMediaTransportControlsSessionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GlobalSystemMediaTransportControlsSessionManager {}
impl ::core::fmt::Debug for GlobalSystemMediaTransportControlsSessionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GlobalSystemMediaTransportControlsSessionManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GlobalSystemMediaTransportControlsSessionManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Control.GlobalSystemMediaTransportControlsSessionManager;{cace8eac-e86e-504a-ab31-5ff8ff1bce49})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GlobalSystemMediaTransportControlsSessionManager {
    type Vtable = IGlobalSystemMediaTransportControlsSessionManager_Vtbl;
    const IID: ::windows_core::GUID = <IGlobalSystemMediaTransportControlsSessionManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GlobalSystemMediaTransportControlsSessionManager {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionManager";
}
impl ::core::convert::From<GlobalSystemMediaTransportControlsSessionManager> for ::windows_core::IUnknown {
    fn from(value: GlobalSystemMediaTransportControlsSessionManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GlobalSystemMediaTransportControlsSessionManager> for ::windows_core::IUnknown {
    fn from(value: &GlobalSystemMediaTransportControlsSessionManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GlobalSystemMediaTransportControlsSessionManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GlobalSystemMediaTransportControlsSessionManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GlobalSystemMediaTransportControlsSessionManager> for ::windows_core::IInspectable {
    fn from(value: GlobalSystemMediaTransportControlsSessionManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GlobalSystemMediaTransportControlsSessionManager> for ::windows_core::IInspectable {
    fn from(value: &GlobalSystemMediaTransportControlsSessionManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GlobalSystemMediaTransportControlsSessionManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GlobalSystemMediaTransportControlsSessionManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GlobalSystemMediaTransportControlsSessionManager {}
unsafe impl ::core::marker::Sync for GlobalSystemMediaTransportControlsSessionManager {}
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSessionMediaProperties(::windows_core::IUnknown);
impl GlobalSystemMediaTransportControlsSessionMediaProperties {
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Subtitle(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Subtitle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AlbumArtist(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AlbumArtist)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Artist(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Artist)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AlbumTitle(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AlbumTitle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn TrackNumber(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).TrackNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Genres(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Genres)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn AlbumTrackCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).AlbumTrackCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn PlaybackType(&self) -> ::windows_core::Result<::winrt_foundation::IReference<super::MediaPlaybackType>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<super::MediaPlaybackType>>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn Thumbnail(&self) -> ::windows_core::Result<::winrt_storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Thumbnail)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
}
impl ::core::clone::Clone for GlobalSystemMediaTransportControlsSessionMediaProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GlobalSystemMediaTransportControlsSessionMediaProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GlobalSystemMediaTransportControlsSessionMediaProperties {}
impl ::core::fmt::Debug for GlobalSystemMediaTransportControlsSessionMediaProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GlobalSystemMediaTransportControlsSessionMediaProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GlobalSystemMediaTransportControlsSessionMediaProperties {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Control.GlobalSystemMediaTransportControlsSessionMediaProperties;{68856cf6-adb4-54b2-ac16-05837907acb6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GlobalSystemMediaTransportControlsSessionMediaProperties {
    type Vtable = IGlobalSystemMediaTransportControlsSessionMediaProperties_Vtbl;
    const IID: ::windows_core::GUID = <IGlobalSystemMediaTransportControlsSessionMediaProperties as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GlobalSystemMediaTransportControlsSessionMediaProperties {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionMediaProperties";
}
impl ::core::convert::From<GlobalSystemMediaTransportControlsSessionMediaProperties> for ::windows_core::IUnknown {
    fn from(value: GlobalSystemMediaTransportControlsSessionMediaProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GlobalSystemMediaTransportControlsSessionMediaProperties> for ::windows_core::IUnknown {
    fn from(value: &GlobalSystemMediaTransportControlsSessionMediaProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GlobalSystemMediaTransportControlsSessionMediaProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GlobalSystemMediaTransportControlsSessionMediaProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GlobalSystemMediaTransportControlsSessionMediaProperties> for ::windows_core::IInspectable {
    fn from(value: GlobalSystemMediaTransportControlsSessionMediaProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GlobalSystemMediaTransportControlsSessionMediaProperties> for ::windows_core::IInspectable {
    fn from(value: &GlobalSystemMediaTransportControlsSessionMediaProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GlobalSystemMediaTransportControlsSessionMediaProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GlobalSystemMediaTransportControlsSessionMediaProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GlobalSystemMediaTransportControlsSessionMediaProperties {}
unsafe impl ::core::marker::Sync for GlobalSystemMediaTransportControlsSessionMediaProperties {}
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSessionPlaybackControls(::windows_core::IUnknown);
impl GlobalSystemMediaTransportControlsSessionPlaybackControls {
    pub fn IsPlayEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPlayEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPauseEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPauseEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsStopEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsStopEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsRecordEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsRecordEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsFastForwardEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsFastForwardEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsRewindEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsRewindEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsNextEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsNextEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPreviousEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPreviousEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsChannelUpEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsChannelUpEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsChannelDownEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsChannelDownEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPlayPauseToggleEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPlayPauseToggleEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsShuffleEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsShuffleEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsRepeatEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsRepeatEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPlaybackRateEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPlaybackRateEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPlaybackPositionEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPlaybackPositionEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GlobalSystemMediaTransportControlsSessionPlaybackControls {}
impl ::core::fmt::Debug for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GlobalSystemMediaTransportControlsSessionPlaybackControls").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackControls;{6501a3e6-bc7a-503a-bb1b-68f158f3fb03})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    type Vtable = IGlobalSystemMediaTransportControlsSessionPlaybackControls_Vtbl;
    const IID: ::windows_core::GUID = <IGlobalSystemMediaTransportControlsSessionPlaybackControls as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackControls";
}
impl ::core::convert::From<GlobalSystemMediaTransportControlsSessionPlaybackControls> for ::windows_core::IUnknown {
    fn from(value: GlobalSystemMediaTransportControlsSessionPlaybackControls) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GlobalSystemMediaTransportControlsSessionPlaybackControls> for ::windows_core::IUnknown {
    fn from(value: &GlobalSystemMediaTransportControlsSessionPlaybackControls) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GlobalSystemMediaTransportControlsSessionPlaybackControls {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GlobalSystemMediaTransportControlsSessionPlaybackControls> for ::windows_core::IInspectable {
    fn from(value: GlobalSystemMediaTransportControlsSessionPlaybackControls) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GlobalSystemMediaTransportControlsSessionPlaybackControls> for ::windows_core::IInspectable {
    fn from(value: &GlobalSystemMediaTransportControlsSessionPlaybackControls) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GlobalSystemMediaTransportControlsSessionPlaybackControls {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GlobalSystemMediaTransportControlsSessionPlaybackControls {}
unsafe impl ::core::marker::Sync for GlobalSystemMediaTransportControlsSessionPlaybackControls {}
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSessionPlaybackInfo(::windows_core::IUnknown);
impl GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    pub fn Controls(&self) -> ::windows_core::Result<GlobalSystemMediaTransportControlsSessionPlaybackControls> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Controls)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GlobalSystemMediaTransportControlsSessionPlaybackControls>(result__)
        }
    }
    pub fn PlaybackStatus(&self) -> ::windows_core::Result<GlobalSystemMediaTransportControlsSessionPlaybackStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GlobalSystemMediaTransportControlsSessionPlaybackStatus>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GlobalSystemMediaTransportControlsSessionPlaybackStatus>(result__)
        }
    }
    pub fn PlaybackType(&self) -> ::windows_core::Result<::winrt_foundation::IReference<super::MediaPlaybackType>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<super::MediaPlaybackType>>(result__)
        }
    }
    pub fn AutoRepeatMode(&self) -> ::windows_core::Result<::winrt_foundation::IReference<super::MediaPlaybackAutoRepeatMode>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AutoRepeatMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<super::MediaPlaybackAutoRepeatMode>>(result__)
        }
    }
    pub fn PlaybackRate(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackRate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f64>>(result__)
        }
    }
    pub fn IsShuffleActive(&self) -> ::windows_core::Result<::winrt_foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsShuffleActive)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<bool>>(result__)
        }
    }
}
impl ::core::clone::Clone for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GlobalSystemMediaTransportControlsSessionPlaybackInfo {}
impl ::core::fmt::Debug for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GlobalSystemMediaTransportControlsSessionPlaybackInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackInfo;{94b4b6cf-e8ba-51ad-87a7-c10ade106127})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    type Vtable = IGlobalSystemMediaTransportControlsSessionPlaybackInfo_Vtbl;
    const IID: ::windows_core::GUID = <IGlobalSystemMediaTransportControlsSessionPlaybackInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackInfo";
}
impl ::core::convert::From<GlobalSystemMediaTransportControlsSessionPlaybackInfo> for ::windows_core::IUnknown {
    fn from(value: GlobalSystemMediaTransportControlsSessionPlaybackInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GlobalSystemMediaTransportControlsSessionPlaybackInfo> for ::windows_core::IUnknown {
    fn from(value: &GlobalSystemMediaTransportControlsSessionPlaybackInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GlobalSystemMediaTransportControlsSessionPlaybackInfo> for ::windows_core::IInspectable {
    fn from(value: GlobalSystemMediaTransportControlsSessionPlaybackInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GlobalSystemMediaTransportControlsSessionPlaybackInfo> for ::windows_core::IInspectable {
    fn from(value: &GlobalSystemMediaTransportControlsSessionPlaybackInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GlobalSystemMediaTransportControlsSessionPlaybackInfo {}
unsafe impl ::core::marker::Sync for GlobalSystemMediaTransportControlsSessionPlaybackInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GlobalSystemMediaTransportControlsSessionPlaybackStatus(pub i32);
impl GlobalSystemMediaTransportControlsSessionPlaybackStatus {
    pub const Closed: Self = Self(0i32);
    pub const Opened: Self = Self(1i32);
    pub const Changing: Self = Self(2i32);
    pub const Stopped: Self = Self(3i32);
    pub const Playing: Self = Self(4i32);
    pub const Paused: Self = Self(5i32);
}
impl ::core::marker::Copy for GlobalSystemMediaTransportControlsSessionPlaybackStatus {}
impl ::core::clone::Clone for GlobalSystemMediaTransportControlsSessionPlaybackStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GlobalSystemMediaTransportControlsSessionPlaybackStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GlobalSystemMediaTransportControlsSessionPlaybackStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for GlobalSystemMediaTransportControlsSessionPlaybackStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GlobalSystemMediaTransportControlsSessionPlaybackStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GlobalSystemMediaTransportControlsSessionPlaybackStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSessionTimelineProperties(::windows_core::IUnknown);
impl GlobalSystemMediaTransportControlsSessionTimelineProperties {
    pub fn StartTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn EndTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).EndTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn MinSeekTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).MinSeekTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn MaxSeekTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).MaxSeekTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn LastUpdatedTime(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).LastUpdatedTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
}
impl ::core::clone::Clone for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GlobalSystemMediaTransportControlsSessionTimelineProperties {}
impl ::core::fmt::Debug for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GlobalSystemMediaTransportControlsSessionTimelineProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Control.GlobalSystemMediaTransportControlsSessionTimelineProperties;{ede34136-6f25-588d-8ecf-ea5b6735aaa5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    type Vtable = IGlobalSystemMediaTransportControlsSessionTimelineProperties_Vtbl;
    const IID: ::windows_core::GUID = <IGlobalSystemMediaTransportControlsSessionTimelineProperties as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionTimelineProperties";
}
impl ::core::convert::From<GlobalSystemMediaTransportControlsSessionTimelineProperties> for ::windows_core::IUnknown {
    fn from(value: GlobalSystemMediaTransportControlsSessionTimelineProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GlobalSystemMediaTransportControlsSessionTimelineProperties> for ::windows_core::IUnknown {
    fn from(value: &GlobalSystemMediaTransportControlsSessionTimelineProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GlobalSystemMediaTransportControlsSessionTimelineProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GlobalSystemMediaTransportControlsSessionTimelineProperties> for ::windows_core::IInspectable {
    fn from(value: GlobalSystemMediaTransportControlsSessionTimelineProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GlobalSystemMediaTransportControlsSessionTimelineProperties> for ::windows_core::IInspectable {
    fn from(value: &GlobalSystemMediaTransportControlsSessionTimelineProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GlobalSystemMediaTransportControlsSessionTimelineProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GlobalSystemMediaTransportControlsSessionTimelineProperties {}
unsafe impl ::core::marker::Sync for GlobalSystemMediaTransportControlsSessionTimelineProperties {}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentSessionChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrentSessionChangedEventArgs {
    type Vtable = ICurrentSessionChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6969cb39_0bfa_5fe0_8d73_09cc5e5408e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentSessionChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSession(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGlobalSystemMediaTransportControlsSession {
    type Vtable = IGlobalSystemMediaTransportControlsSession_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7148c835_9b14_5ae2_ab85_dc9b1c14e1a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSession_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SourceAppUserModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TryGetMediaPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetTimelineProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetPlaybackInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryPlayAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryPauseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryStopAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryRecordAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryFastForwardAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryRewindAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TrySkipNextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TrySkipPreviousAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryChangeChannelUpAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryChangeChannelDownAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryTogglePlayPauseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryChangeAutoRepeatModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedautorepeatmode: super::MediaPlaybackAutoRepeatMode, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryChangePlaybackRateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedplaybackrate: f64, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryChangeShuffleActiveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedshufflestate: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryChangePlaybackPositionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedplaybackposition: i64, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TimelinePropertiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveTimelinePropertiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub PlaybackInfoChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePlaybackInfoChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub MediaPropertiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveMediaPropertiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSessionManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGlobalSystemMediaTransportControlsSessionManager {
    type Vtable = IGlobalSystemMediaTransportControlsSessionManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcace8eac_e86e_504a_ab31_5ff8ff1bce49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetCurrentSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub GetSessions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetSessions: usize,
    pub CurrentSessionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCurrentSessionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SessionsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSessionsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSessionManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGlobalSystemMediaTransportControlsSessionManagerStatics {
    type Vtable = IGlobalSystemMediaTransportControlsSessionManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2050c4ee_11a0_57de_aed7_c97c70338245);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSessionMediaProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGlobalSystemMediaTransportControlsSessionMediaProperties {
    type Vtable = IGlobalSystemMediaTransportControlsSessionMediaProperties_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x68856cf6_adb4_54b2_ac16_05837907acb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionMediaProperties_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AlbumArtist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Artist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AlbumTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TrackNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Genres: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Genres: usize,
    pub AlbumTrackCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub PlaybackType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    Thumbnail: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSessionPlaybackControls(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGlobalSystemMediaTransportControlsSessionPlaybackControls {
    type Vtable = IGlobalSystemMediaTransportControlsSessionPlaybackControls_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6501a3e6_bc7a_503a_bb1b_68f158f3fb03);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionPlaybackControls_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsPlayEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsPauseEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsStopEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsRecordEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsFastForwardEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsRewindEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsNextEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsPreviousEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsChannelUpEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsChannelDownEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsPlayPauseToggleEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsShuffleEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsRepeatEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsPlaybackRateEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsPlaybackPositionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSessionPlaybackInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGlobalSystemMediaTransportControlsSessionPlaybackInfo {
    type Vtable = IGlobalSystemMediaTransportControlsSessionPlaybackInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x94b4b6cf_e8ba_51ad_87a7_c10ade106127);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionPlaybackInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Controls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PlaybackStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GlobalSystemMediaTransportControlsSessionPlaybackStatus) -> ::windows_core::HRESULT,
    pub PlaybackType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AutoRepeatMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PlaybackRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsShuffleActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSessionTimelineProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGlobalSystemMediaTransportControlsSessionTimelineProperties {
    type Vtable = IGlobalSystemMediaTransportControlsSessionTimelineProperties_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xede34136_6f25_588d_8ecf_ea5b6735aaa5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionTimelineProperties_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub EndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub MinSeekTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub MaxSeekTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub LastUpdatedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPropertiesChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPropertiesChangedEventArgs {
    type Vtable = IMediaPropertiesChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d3741cb_adf0_5cef_91ba_cfabcdd77678);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPropertiesChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaybackInfoChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlaybackInfoChangedEventArgs {
    type Vtable = IPlaybackInfoChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x786756c2_bc0d_50a5_8807_054291fef139);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackInfoChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISessionsChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISessionsChangedEventArgs {
    type Vtable = ISessionsChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbbf0cd32_42c4_5a58_b317_f34bbfbd26e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISessionsChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimelinePropertiesChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimelinePropertiesChangedEventArgs {
    type Vtable = ITimelinePropertiesChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x29033a2f_c923_5a77_bcaf_055ff415ad32);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimelinePropertiesChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[repr(transparent)]
pub struct MediaPropertiesChangedEventArgs(::windows_core::IUnknown);
impl MediaPropertiesChangedEventArgs {}
impl ::core::clone::Clone for MediaPropertiesChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPropertiesChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPropertiesChangedEventArgs {}
impl ::core::fmt::Debug for MediaPropertiesChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPropertiesChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPropertiesChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Control.MediaPropertiesChangedEventArgs;{7d3741cb-adf0-5cef-91ba-cfabcdd77678})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaPropertiesChangedEventArgs {
    type Vtable = IMediaPropertiesChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaPropertiesChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaPropertiesChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Control.MediaPropertiesChangedEventArgs";
}
impl ::core::convert::From<MediaPropertiesChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaPropertiesChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPropertiesChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaPropertiesChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaPropertiesChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaPropertiesChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaPropertiesChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaPropertiesChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPropertiesChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaPropertiesChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaPropertiesChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaPropertiesChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaPropertiesChangedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPropertiesChangedEventArgs {}
#[repr(transparent)]
pub struct PlaybackInfoChangedEventArgs(::windows_core::IUnknown);
impl PlaybackInfoChangedEventArgs {}
impl ::core::clone::Clone for PlaybackInfoChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlaybackInfoChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlaybackInfoChangedEventArgs {}
impl ::core::fmt::Debug for PlaybackInfoChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaybackInfoChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlaybackInfoChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Control.PlaybackInfoChangedEventArgs;{786756c2-bc0d-50a5-8807-054291fef139})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PlaybackInfoChangedEventArgs {
    type Vtable = IPlaybackInfoChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPlaybackInfoChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PlaybackInfoChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Control.PlaybackInfoChangedEventArgs";
}
impl ::core::convert::From<PlaybackInfoChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PlaybackInfoChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlaybackInfoChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PlaybackInfoChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PlaybackInfoChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PlaybackInfoChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlaybackInfoChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PlaybackInfoChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlaybackInfoChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PlaybackInfoChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PlaybackInfoChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PlaybackInfoChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PlaybackInfoChangedEventArgs {}
unsafe impl ::core::marker::Sync for PlaybackInfoChangedEventArgs {}
#[repr(transparent)]
pub struct SessionsChangedEventArgs(::windows_core::IUnknown);
impl SessionsChangedEventArgs {}
impl ::core::clone::Clone for SessionsChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SessionsChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SessionsChangedEventArgs {}
impl ::core::fmt::Debug for SessionsChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SessionsChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SessionsChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Control.SessionsChangedEventArgs;{bbf0cd32-42c4-5a58-b317-f34bbfbd26e0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SessionsChangedEventArgs {
    type Vtable = ISessionsChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISessionsChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SessionsChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Control.SessionsChangedEventArgs";
}
impl ::core::convert::From<SessionsChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SessionsChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SessionsChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SessionsChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SessionsChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SessionsChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SessionsChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SessionsChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SessionsChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SessionsChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SessionsChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SessionsChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SessionsChangedEventArgs {}
unsafe impl ::core::marker::Sync for SessionsChangedEventArgs {}
#[repr(transparent)]
pub struct TimelinePropertiesChangedEventArgs(::windows_core::IUnknown);
impl TimelinePropertiesChangedEventArgs {}
impl ::core::clone::Clone for TimelinePropertiesChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimelinePropertiesChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimelinePropertiesChangedEventArgs {}
impl ::core::fmt::Debug for TimelinePropertiesChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimelinePropertiesChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimelinePropertiesChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Control.TimelinePropertiesChangedEventArgs;{29033a2f-c923-5a77-bcaf-055ff415ad32})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TimelinePropertiesChangedEventArgs {
    type Vtable = ITimelinePropertiesChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ITimelinePropertiesChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimelinePropertiesChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Control.TimelinePropertiesChangedEventArgs";
}
impl ::core::convert::From<TimelinePropertiesChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: TimelinePropertiesChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimelinePropertiesChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &TimelinePropertiesChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TimelinePropertiesChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TimelinePropertiesChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimelinePropertiesChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: TimelinePropertiesChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimelinePropertiesChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &TimelinePropertiesChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TimelinePropertiesChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TimelinePropertiesChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TimelinePropertiesChangedEventArgs {}
unsafe impl ::core::marker::Sync for TimelinePropertiesChangedEventArgs {}
