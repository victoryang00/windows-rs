#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AutoLoadedDisplayPropertyKind(pub i32);
impl AutoLoadedDisplayPropertyKind {
    pub const None: Self = Self(0i32);
    pub const MusicOrVideo: Self = Self(1i32);
    pub const Music: Self = Self(2i32);
    pub const Video: Self = Self(3i32);
}
impl ::core::marker::Copy for AutoLoadedDisplayPropertyKind {}
impl ::core::clone::Clone for AutoLoadedDisplayPropertyKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutoLoadedDisplayPropertyKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AutoLoadedDisplayPropertyKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for AutoLoadedDisplayPropertyKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutoLoadedDisplayPropertyKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AutoLoadedDisplayPropertyKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.AutoLoadedDisplayPropertyKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "winrt-")]
pub struct BackgroundMediaPlayer;
#[cfg(feature = "winrt-")]
impl BackgroundMediaPlayer {
    #[cfg(feature = "winrt-")]
    pub fn Current() -> ::windows_core::Result<MediaPlayer> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Current)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlayer>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn MessageReceivedFromBackground<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<MediaPlayerDataReceivedEventArgs>>>(value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).MessageReceivedFromBackground)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveMessageReceivedFromBackground<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveMessageReceivedFromBackground)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "winrt-")]
    pub fn MessageReceivedFromForeground<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<MediaPlayerDataReceivedEventArgs>>>(value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).MessageReceivedFromForeground)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveMessageReceivedFromForeground<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveMessageReceivedFromForeground)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn SendMessageToBackground<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::ValueSet>>(value: Param0) -> ::windows_core::Result<()> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SendMessageToBackground)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() })
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn SendMessageToForeground<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::ValueSet>>(value: Param0) -> ::windows_core::Result<()> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SendMessageToForeground)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() })
    }
    #[cfg(feature = "winrt-")]
    pub fn IsMediaPlaying() -> ::windows_core::Result<bool> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsMediaPlaying)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn Shutdown() -> ::windows_core::Result<()> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).Shutdown)(::windows_core::Interface::as_raw(this)).ok() })
    }
    #[cfg(feature = "winrt-")]
    pub fn IBackgroundMediaPlayerStatics<R, F: FnOnce(&IBackgroundMediaPlayerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BackgroundMediaPlayer, IBackgroundMediaPlayerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for BackgroundMediaPlayer {
    const NAME: &'static str = "Windows.Media.Playback.BackgroundMediaPlayer";
}
#[repr(transparent)]
pub struct CurrentMediaPlaybackItemChangedEventArgs(::windows_core::IUnknown);
impl CurrentMediaPlaybackItemChangedEventArgs {
    pub fn NewItem(&self) -> ::windows_core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NewItem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackItem>(result__)
        }
    }
    pub fn OldItem(&self) -> ::windows_core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OldItem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackItem>(result__)
        }
    }
    pub fn Reason(&self) -> ::windows_core::Result<MediaPlaybackItemChangedReason> {
        let this = &::windows_core::Interface::cast::<ICurrentMediaPlaybackItemChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaPlaybackItemChangedReason>::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackItemChangedReason>(result__)
        }
    }
}
impl ::core::clone::Clone for CurrentMediaPlaybackItemChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CurrentMediaPlaybackItemChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CurrentMediaPlaybackItemChangedEventArgs {}
impl ::core::fmt::Debug for CurrentMediaPlaybackItemChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CurrentMediaPlaybackItemChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CurrentMediaPlaybackItemChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.CurrentMediaPlaybackItemChangedEventArgs;{1743a892-5c43-4a15-967a-572d2d0f26c6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CurrentMediaPlaybackItemChangedEventArgs {
    type Vtable = ICurrentMediaPlaybackItemChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ICurrentMediaPlaybackItemChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CurrentMediaPlaybackItemChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.CurrentMediaPlaybackItemChangedEventArgs";
}
impl ::core::convert::From<CurrentMediaPlaybackItemChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: CurrentMediaPlaybackItemChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CurrentMediaPlaybackItemChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &CurrentMediaPlaybackItemChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CurrentMediaPlaybackItemChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CurrentMediaPlaybackItemChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CurrentMediaPlaybackItemChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: CurrentMediaPlaybackItemChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CurrentMediaPlaybackItemChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &CurrentMediaPlaybackItemChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CurrentMediaPlaybackItemChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CurrentMediaPlaybackItemChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CurrentMediaPlaybackItemChangedEventArgs {}
unsafe impl ::core::marker::Sync for CurrentMediaPlaybackItemChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FailedMediaStreamKind(pub i32);
impl FailedMediaStreamKind {
    pub const Unknown: Self = Self(0i32);
    pub const Audio: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl ::core::marker::Copy for FailedMediaStreamKind {}
impl ::core::clone::Clone for FailedMediaStreamKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FailedMediaStreamKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FailedMediaStreamKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for FailedMediaStreamKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FailedMediaStreamKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FailedMediaStreamKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.FailedMediaStreamKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IBackgroundMediaPlayerStatics(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IBackgroundMediaPlayerStatics {
    type Vtable = IBackgroundMediaPlayerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x856ddbc1_55f7_471f_a0f2_68ac4c904592);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundMediaPlayerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Current: usize,
    #[cfg(feature = "winrt-")]
    pub MessageReceivedFromBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    MessageReceivedFromBackground: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveMessageReceivedFromBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveMessageReceivedFromBackground: usize,
    #[cfg(feature = "winrt-")]
    pub MessageReceivedFromForeground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    MessageReceivedFromForeground: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveMessageReceivedFromForeground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveMessageReceivedFromForeground: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub SendMessageToBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    SendMessageToBackground: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub SendMessageToForeground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    SendMessageToForeground: usize,
    #[cfg(feature = "winrt-")]
    pub IsMediaPlaying: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    IsMediaPlaying: usize,
    #[cfg(feature = "winrt-")]
    pub Shutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Shutdown: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentMediaPlaybackItemChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrentMediaPlaybackItemChangedEventArgs {
    type Vtable = ICurrentMediaPlaybackItemChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1743a892_5c43_4a15_967a_572d2d0f26c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentMediaPlaybackItemChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub NewItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OldItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentMediaPlaybackItemChangedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrentMediaPlaybackItemChangedEventArgs2 {
    type Vtable = ICurrentMediaPlaybackItemChangedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d80a51e_996e_40a9_be48_e66ec90b2b7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentMediaPlaybackItemChangedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackItemChangedReason) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaBreak(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaBreak {
    type Vtable = IMediaBreak_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x714be270_0def_4ebc_a489_6b34930e1558);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBreak_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PlaybackList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PresentationPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub InsertionMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaBreakInsertionMethod) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub CustomProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    CustomProperties: usize,
    pub CanStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaBreakEndedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaBreakEndedEventArgs {
    type Vtable = IMediaBreakEndedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x32b93276_1c5d_4fee_8732_236dc3a88580);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBreakEndedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MediaBreak: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaBreakFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaBreakFactory {
    type Vtable = IMediaBreakFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4516e002_18e0_4079_8b5f_d33495c15d2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBreakFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, insertionmethod: MediaBreakInsertionMethod, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWithPresentationPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, insertionmethod: MediaBreakInsertionMethod, presentationposition: ::winrt_foundation::TimeSpan, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaBreakManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaBreakManager {
    type Vtable = IMediaBreakManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa854ddb1_feb4_4d9b_9d97_0fdbe58e5e39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBreakManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub BreaksSeekedOver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveBreaksSeekedOver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub BreakStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveBreakStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub BreakEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveBreakEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub BreakSkipped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveBreakSkipped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub CurrentBreak: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PlaybackSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PlayBreak: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SkipCurrentBreak: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaBreakSchedule(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaBreakSchedule {
    type Vtable = IMediaBreakSchedule_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa19a5813_98b6_41d8_83da_f971d22b7bba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBreakSchedule_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ScheduleChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveScheduleChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub InsertMidrollBreak: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediabreak: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveMidrollBreak: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediabreak: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub MidrollBreaks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    MidrollBreaks: usize,
    pub SetPrerollBreak: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PrerollBreak: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetPostrollBreak: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PostrollBreak: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PlaybackItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaBreakSeekedOverEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaBreakSeekedOverEventArgs {
    type Vtable = IMediaBreakSeekedOverEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe5aa6746_0606_4492_b9d3_c3c8fde0a4ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBreakSeekedOverEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub SeekedOverBreaks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SeekedOverBreaks: usize,
    pub OldPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub NewPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaBreakSkippedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaBreakSkippedEventArgs {
    type Vtable = IMediaBreakSkippedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6ee94c05_2f54_4a3e_a3ab_24c3b270b4a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBreakSkippedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MediaBreak: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaBreakStartedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaBreakStartedEventArgs {
    type Vtable = IMediaBreakStartedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa87efe71_dfd4_454a_956e_0a4a648395f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBreakStartedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MediaBreak: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IMediaEnginePlaybackSource(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl IMediaEnginePlaybackSource {
    #[cfg(feature = "winrt-")]
    pub fn CurrentItem(&self) -> ::windows_core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentItem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackItem>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetPlaybackSource<'a, Param0: ::windows_core::IntoParam<'a, IMediaPlaybackSource>>(&self, source: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPlaybackSource)(::windows_core::Interface::as_raw(this), source.into_param().abi()).ok() }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<IMediaEnginePlaybackSource> for ::windows_core::IUnknown {
    fn from(value: IMediaEnginePlaybackSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&IMediaEnginePlaybackSource> for ::windows_core::IUnknown {
    fn from(value: &IMediaEnginePlaybackSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMediaEnginePlaybackSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMediaEnginePlaybackSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<IMediaEnginePlaybackSource> for ::windows_core::IInspectable {
    fn from(value: IMediaEnginePlaybackSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&IMediaEnginePlaybackSource> for ::windows_core::IInspectable {
    fn from(value: &IMediaEnginePlaybackSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IMediaEnginePlaybackSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IMediaEnginePlaybackSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for IMediaEnginePlaybackSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for IMediaEnginePlaybackSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for IMediaEnginePlaybackSource {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for IMediaEnginePlaybackSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaEnginePlaybackSource").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for IMediaEnginePlaybackSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{5c1d0ba7-3856-48b9-8dc6-244bf107bf8c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IMediaEnginePlaybackSource {
    type Vtable = IMediaEnginePlaybackSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c1d0ba7_3856_48b9_8dc6_244bf107bf8c);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEnginePlaybackSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub CurrentItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CurrentItem: usize,
    #[cfg(feature = "winrt-")]
    pub SetPlaybackSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetPlaybackSource: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaItemDisplayProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaItemDisplayProperties {
    type Vtable = IMediaItemDisplayProperties_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1e3c1b48_7097_4384_a217_c1291dfa8c16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaItemDisplayProperties_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::MediaPlaybackType) -> ::windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::MediaPlaybackType) -> ::windows_core::HRESULT,
    pub MusicProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub VideoProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    Thumbnail: usize,
    #[cfg(feature = "winrt-storage")]
    pub SetThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    SetThumbnail: usize,
    pub ClearAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackCommandManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackCommandManager {
    type Vtable = IMediaPlaybackCommandManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5acee5a6_5cb6_4a5a_8521_cc86b1c1ed37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub MediaPlayer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PlayBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PauseBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub NextBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PreviousBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FastForwardBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RewindBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShuffleBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AutoRepeatModeBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PositionBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RateBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PlayReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePlayReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub PauseReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePauseReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub NextReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveNextReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub PreviousReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePreviousReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub FastForwardReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveFastForwardReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RewindReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveRewindReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ShuffleReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveShuffleReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub AutoRepeatModeReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveAutoRepeatModeReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub PositionReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePositionReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RateReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveRateReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3d6f4f23_5230_4411_a0e9_bad94c2a045c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub AutoRepeatMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::MediaPlaybackAutoRepeatMode) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerCommandBehavior(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackCommandManagerCommandBehavior {
    type Vtable = IMediaPlaybackCommandManagerCommandBehavior_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x786c1e78_ce78_4a10_afd6_843fcbb90c2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerCommandBehavior_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CommandManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub EnablingRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaCommandEnablingRule) -> ::windows_core::HRESULT,
    pub SetEnablingRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaCommandEnablingRule) -> ::windows_core::HRESULT,
    pub IsEnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveIsEnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerFastForwardReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerFastForwardReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30f064d9_b491_4d0a_bc21_3098bd1332e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerFastForwardReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerNextReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackCommandManagerNextReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerNextReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe1504433_a2b0_45d4_b9de_5f42ac14a839);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerNextReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerPauseReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackCommandManagerPauseReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerPauseReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5ceccd1c_c25c_4221_b16c_c3c98ce012d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerPauseReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerPlayReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackCommandManagerPlayReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerPlayReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9af0004e_578b_4c56_a006_16159d888a48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerPlayReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerPositionReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackCommandManagerPositionReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerPositionReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5591a754_d627_4bdd_a90d_86a015b24902);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerPositionReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerPreviousReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackCommandManagerPreviousReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerPreviousReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x525e3081_4632_4f76_99b1_d771623f6287);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerPreviousReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerRateReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackCommandManagerRateReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerRateReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x18ea3939_4a16_4169_8b05_3eb9f5ff78eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerRateReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub PlaybackRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerRewindReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackCommandManagerRewindReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerRewindReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f085947_a3c0_425d_aaef_97ba7898b141);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerRewindReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerShuffleReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackCommandManagerShuffleReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerShuffleReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50a05cef_63ee_4a96_b7b5_fee08b9ff90c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerShuffleReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsShuffleRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackItem {
    type Vtable = IMediaPlaybackItem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x047097d2_e4af_48ab_b283_6929e674ece2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackItem_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub AudioTracksChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    AudioTracksChanged: usize,
    pub RemoveAudioTracksChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub VideoTracksChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    VideoTracksChanged: usize,
    pub RemoveVideoTracksChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub TimedMetadataTracksChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    TimedMetadataTracksChanged: usize,
    pub RemoveTimedMetadataTracksChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-media")]
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    Source: usize,
    #[cfg(feature = "winrt-foundation")]
    pub AudioTracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    AudioTracks: usize,
    #[cfg(feature = "winrt-foundation")]
    pub VideoTracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    VideoTracks: usize,
    #[cfg(feature = "winrt-foundation")]
    pub TimedMetadataTracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    TimedMetadataTracks: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackItem2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackItem2 {
    type Vtable = IMediaPlaybackItem2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd859d171_d7ef_4b81_ac1f_f40493cbb091);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackItem2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub BreakSchedule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub DurationLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CanSkip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanSkip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub GetDisplayProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ApplyDisplayProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackItem3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackItem3 {
    type Vtable = IMediaPlaybackItem3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0d328220_b80a_4d09_9ff8_f87094a1c831);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackItem3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsDisabledInPlaybackList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsDisabledInPlaybackList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub TotalDownloadProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub AutoLoadedDisplayProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AutoLoadedDisplayPropertyKind) -> ::windows_core::HRESULT,
    pub SetAutoLoadedDisplayProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AutoLoadedDisplayPropertyKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackItemError(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackItemError {
    type Vtable = IMediaPlaybackItemError_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x69fbef2b_dcd6_4df9_a450_dbf4c6f1c2c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackItemError_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackItemErrorCode) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackItemFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackItemFactory {
    type Vtable = IMediaPlaybackItemFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7133fce1_1769_4ff9_a7c1_38d2c4d42360);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackItemFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-media")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackItemFactory2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackItemFactory2 {
    type Vtable = IMediaPlaybackItemFactory2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd77cdf3a_b947_4972_b35d_adfb931a71e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackItemFactory2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-media")]
    pub CreateWithStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows_core::RawPtr, starttime: ::winrt_foundation::TimeSpan, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    CreateWithStartTime: usize,
    #[cfg(feature = "winrt-media")]
    pub CreateWithStartTimeAndDurationLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows_core::RawPtr, starttime: ::winrt_foundation::TimeSpan, durationlimit: ::winrt_foundation::TimeSpan, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    CreateWithStartTimeAndDurationLimit: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackItemFailedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackItemFailedEventArgs {
    type Vtable = IMediaPlaybackItemFailedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7703134a_e9a7_47c3_862c_c656d30683d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackItemFailedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackItemOpenedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackItemOpenedEventArgs {
    type Vtable = IMediaPlaybackItemOpenedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcbd9bd82_3037_4fbe_ae8f_39fc39edf4ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackItemOpenedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackItemStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackItemStatics {
    type Vtable = IMediaPlaybackItemStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4b1be7f4_4345_403c_8a67_f5de91df4c86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackItemStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-media")]
    pub FindFromMediaSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    FindFromMediaSource: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackList(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackList {
    type Vtable = IMediaPlaybackList_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7f77ee9c_dc42_4e26_a98d_7850df8ec925);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackList_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ItemFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveItemFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub CurrentItemChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCurrentItemChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ItemOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveItemOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Items: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Items: usize,
    pub AutoRepeatEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAutoRepeatEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ShuffleEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetShuffleEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CurrentItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CurrentItemIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MovePrevious: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MoveTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemindex: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackList2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackList2 {
    type Vtable = IMediaPlaybackList2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0e09b478_600a_4274_a14b_0b6723d0f48b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackList2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MaxPrefetchTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetMaxPrefetchTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StartingItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetStartingItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub ShuffledItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ShuffledItems: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SetShuffledItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetShuffledItems: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackList3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackList3 {
    type Vtable = IMediaPlaybackList3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdd24bba9_bc47_4463_aa90_c18b7e5ffde1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackList3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MaxPlayedItemsToKeepOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetMaxPlayedItemsToKeepOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackSession(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackSession {
    type Vtable = IMediaPlaybackSession_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc32b683d_0407_41ba_8946_8b345a5a5435);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackSession_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PlaybackStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePlaybackStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub PlaybackRateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePlaybackRateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SeekCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSeekCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub BufferingStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveBufferingStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub BufferingEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveBufferingEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub BufferingProgressChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveBufferingProgressChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub DownloadProgressChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDownloadProgressChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub NaturalDurationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveNaturalDurationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub PositionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePositionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub NaturalVideoSizeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveNaturalVideoSizeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub MediaPlayer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub NaturalDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub PlaybackState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackState) -> ::windows_core::HRESULT,
    pub CanSeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CanPause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsProtected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub PlaybackRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetPlaybackRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub BufferingProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub DownloadProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub NaturalVideoHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub NaturalVideoWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub NormalizedSourceRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    pub SetNormalizedSourceRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-media")]
    pub StereoscopicVideoPackingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::StereoscopicVideoPackingMode) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    StereoscopicVideoPackingMode: usize,
    #[cfg(feature = "winrt-media")]
    pub SetStereoscopicVideoPackingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::MediaProperties::StereoscopicVideoPackingMode) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    SetStereoscopicVideoPackingMode: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackSession2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackSession2 {
    type Vtable = IMediaPlaybackSession2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf8ba7c79_1fc8_4097_ad70_c0fa18cc0050);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackSession2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub BufferedRangesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveBufferedRangesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub PlayedRangesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePlayedRangesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SeekableRangesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSeekableRangesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SupportedPlaybackRatesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSupportedPlaybackRatesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SphericalVideoProjection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsMirroring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsMirroring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub GetBufferedRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetBufferedRanges: usize,
    #[cfg(feature = "winrt-foundation")]
    pub GetPlayedRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetPlayedRanges: usize,
    #[cfg(feature = "winrt-foundation")]
    pub GetSeekableRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetSeekableRanges: usize,
    pub IsSupportedPlaybackRateRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rate1: f64, rate2: f64, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackSession3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackSession3 {
    type Vtable = IMediaPlaybackSession3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7ba2b41a_a3e2_405f_b77b_a4812c238b66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackSession3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-media")]
    pub PlaybackRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::MediaRotation) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    PlaybackRotation: usize,
    #[cfg(feature = "winrt-media")]
    pub SetPlaybackRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::MediaProperties::MediaRotation) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    SetPlaybackRotation: usize,
    pub GetOutputDegradationPolicyState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackSessionBufferingStartedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackSessionBufferingStartedEventArgs {
    type Vtable = IMediaPlaybackSessionBufferingStartedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd6aafed_74e2_43b5_b115_76236c33791a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackSessionBufferingStartedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsPlaybackInterruption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackSessionOutputDegradationPolicyState(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackSessionOutputDegradationPolicyState {
    type Vtable = IMediaPlaybackSessionOutputDegradationPolicyState_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x558e727d_f633_49f9_965a_abaa1db709be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackSessionOutputDegradationPolicyState_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub VideoConstrictionReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackSessionVideoConstrictionReason) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IMediaPlaybackSource(::windows_core::IUnknown);
impl IMediaPlaybackSource {}
impl ::core::convert::From<IMediaPlaybackSource> for ::windows_core::IUnknown {
    fn from(value: IMediaPlaybackSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaPlaybackSource> for ::windows_core::IUnknown {
    fn from(value: &IMediaPlaybackSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMediaPlaybackSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMediaPlaybackSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMediaPlaybackSource> for ::windows_core::IInspectable {
    fn from(value: IMediaPlaybackSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaPlaybackSource> for ::windows_core::IInspectable {
    fn from(value: &IMediaPlaybackSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IMediaPlaybackSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IMediaPlaybackSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMediaPlaybackSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMediaPlaybackSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaPlaybackSource {}
impl ::core::fmt::Debug for IMediaPlaybackSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaPlaybackSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IMediaPlaybackSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{ef9dc2bc-9317-4696-b051-2bad643177b5}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IMediaPlaybackSource {
    type Vtable = IMediaPlaybackSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef9dc2bc_9317_4696_b051_2bad643177b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackSphericalVideoProjection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackSphericalVideoProjection {
    type Vtable = IMediaPlaybackSphericalVideoProjection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd405b37c_6f0e_4661_b8ee_d487ba9752d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackSphericalVideoProjection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-media")]
    pub FrameFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::SphericalVideoFrameFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    FrameFormat: usize,
    #[cfg(feature = "winrt-media")]
    pub SetFrameFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::MediaProperties::SphericalVideoFrameFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    SetFrameFormat: usize,
    pub HorizontalFieldOfViewInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetHorizontalFieldOfViewInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub ViewOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Quaternion) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ViewOrientation: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SetViewOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Numerics::Quaternion) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetViewOrientation: usize,
    pub ProjectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SphericalVideoProjectionMode) -> ::windows_core::HRESULT,
    pub SetProjectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SphericalVideoProjectionMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackTimedMetadataTrackList(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlaybackTimedMetadataTrackList {
    type Vtable = IMediaPlaybackTimedMetadataTrackList_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72b41319_bbfb_46a3_9372_9c9c744b9438);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackTimedMetadataTrackList_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub PresentationModeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    PresentationModeChanged: usize,
    pub RemovePresentationModeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub GetPresentationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut TimedMetadataTrackPresentationMode) -> ::windows_core::HRESULT,
    pub SetPresentationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: TimedMetadataTrackPresentationMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlayer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlayer {
    type Vtable = IMediaPlayer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x381a83cb_6fff_499b_8d64_2885dfc1249e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayer_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AutoPlay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAutoPlay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-")]
    pub NaturalDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    NaturalDuration: usize,
    #[cfg(feature = "winrt-")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Position: usize,
    #[cfg(feature = "winrt-")]
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetPosition: usize,
    #[cfg(feature = "winrt-")]
    pub BufferingProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    BufferingProgress: usize,
    #[cfg(feature = "winrt-")]
    pub CurrentState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaPlayerState) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CurrentState: usize,
    #[cfg(feature = "winrt-")]
    pub CanSeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CanSeek: usize,
    #[cfg(feature = "winrt-")]
    pub CanPause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CanPause: usize,
    pub IsLoopingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsLoopingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-")]
    pub IsProtected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    IsProtected: usize,
    pub IsMuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsMuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-")]
    pub PlaybackRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    PlaybackRate: usize,
    #[cfg(feature = "winrt-")]
    pub SetPlaybackRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetPlaybackRate: usize,
    pub Volume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-")]
    pub PlaybackMediaMarkers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    PlaybackMediaMarkers: usize,
    pub MediaOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveMediaOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub MediaEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveMediaEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub MediaFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveMediaFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-")]
    pub CurrentStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CurrentStateChanged: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveCurrentStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveCurrentStateChanged: usize,
    #[cfg(feature = "winrt-")]
    pub PlaybackMediaMarkerReached: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    PlaybackMediaMarkerReached: usize,
    #[cfg(feature = "winrt-")]
    pub RemovePlaybackMediaMarkerReached: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemovePlaybackMediaMarkerReached: usize,
    #[cfg(feature = "winrt-")]
    pub MediaPlayerRateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    MediaPlayerRateChanged: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveMediaPlayerRateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveMediaPlayerRateChanged: usize,
    pub VolumeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveVolumeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-")]
    pub SeekCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SeekCompleted: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveSeekCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveSeekCompleted: usize,
    #[cfg(feature = "winrt-")]
    pub BufferingStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    BufferingStarted: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveBufferingStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveBufferingStarted: usize,
    #[cfg(feature = "winrt-")]
    pub BufferingEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    BufferingEnded: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveBufferingEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveBufferingEnded: usize,
    pub Play: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-")]
    pub SetUriSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetUriSource: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlayer2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlayer2 {
    type Vtable = IMediaPlayer2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3c841218_2123_4fc5_9082_2f883f77bdf5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayer2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SystemMediaTransportControls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AudioCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaPlayerAudioCategory) -> ::windows_core::HRESULT,
    pub SetAudioCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaPlayerAudioCategory) -> ::windows_core::HRESULT,
    pub AudioDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaPlayerAudioDeviceType) -> ::windows_core::HRESULT,
    pub SetAudioDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaPlayerAudioDeviceType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlayer3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlayer3 {
    type Vtable = IMediaPlayer3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xee0660da_031b_4feb_bd9b_92e0a0a8d299);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayer3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsMutedChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveIsMutedChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SourceChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSourceChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub AudioBalance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetAudioBalance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub RealTimePlayback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetRealTimePlayback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub StereoscopicVideoRenderMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StereoscopicVideoRenderMode) -> ::windows_core::HRESULT,
    pub SetStereoscopicVideoRenderMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StereoscopicVideoRenderMode) -> ::windows_core::HRESULT,
    pub BreakManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CommandManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-devices")]
    pub AudioDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    AudioDevice: usize,
    #[cfg(feature = "winrt-devices")]
    pub SetAudioDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    SetAudioDevice: usize,
    pub TimelineController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetTimelineController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TimelineControllerPositionOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetTimelineControllerPositionOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub PlaybackSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StepForwardOneFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StepBackwardOneFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-media")]
    pub GetAsCastingSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    GetAsCastingSource: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlayer4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlayer4 {
    type Vtable = IMediaPlayer4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x80035db0_7448_4770_afcf_2a57450914c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayer4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetSurfaceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: ::winrt_foundation::Size) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub GetSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    GetSurface: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlayer5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlayer5 {
    type Vtable = IMediaPlayer5_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcfe537fd_f86a_4446_bf4d_c8e792b7b4b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayer5_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub VideoFrameAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveVideoFrameAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub IsVideoFrameServerEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsVideoFrameServerEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-graphics")]
    pub CopyFrameToVideoSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destination: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    CopyFrameToVideoSurface: usize,
    #[cfg(feature = "winrt-graphics")]
    pub CopyFrameToVideoSurfaceWithTargetRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destination: ::windows_core::RawPtr, targetrectangle: ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    CopyFrameToVideoSurfaceWithTargetRectangle: usize,
    #[cfg(feature = "winrt-graphics")]
    pub CopyFrameToStereoscopicVideoSurfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destinationlefteye: ::windows_core::RawPtr, destinationrighteye: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    CopyFrameToStereoscopicVideoSurfaces: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlayer6(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlayer6 {
    type Vtable = IMediaPlayer6_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe0caa086_ae65_414c_b010_8bc55f00e692);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayer6_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SubtitleFrameChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSubtitleFrameChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-graphics")]
    pub RenderSubtitlesToSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destination: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    RenderSubtitlesToSurface: usize,
    #[cfg(feature = "winrt-graphics")]
    pub RenderSubtitlesToSurfaceWithTargetRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destination: ::windows_core::RawPtr, targetrectangle: ::winrt_foundation::Rect, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    RenderSubtitlesToSurfaceWithTargetRectangle: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlayer7(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlayer7 {
    type Vtable = IMediaPlayer7_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d1dc478_4500_4531_b3f4_777a71491f7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayer7_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-media")]
    pub AudioStateMonitor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    AudioStateMonitor: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlayerDataReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlayerDataReceivedEventArgs {
    type Vtable = IMediaPlayerDataReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc75a9405_c801_412a_835b_83fc0e622a8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayerDataReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Data: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlayerEffects(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlayerEffects {
    type Vtable = IMediaPlayerEffects_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x85a1deda_cab6_4cc0_8be3_6035f4de2591);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayerEffects_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub AddAudioEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, effectoptional: bool, configuration: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    AddAudioEffect: usize,
    pub RemoveAllEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlayerEffects2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlayerEffects2 {
    type Vtable = IMediaPlayerEffects2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfa419a79_1bbe_46c5_ae1f_8ee69fb3c2c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayerEffects2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub AddVideoEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, effectoptional: bool, effectconfiguration: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    AddVideoEffect: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlayerFailedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlayerFailedEventArgs {
    type Vtable = IMediaPlayerFailedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2744e9b9_a7e3_4f16_bac4_7914ebc08301);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayerFailedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaPlayerError) -> ::windows_core::HRESULT,
    pub ExtendedErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub ErrorMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlayerRateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlayerRateChangedEventArgs {
    type Vtable = IMediaPlayerRateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x40600d58_3b61_4bb2_989f_fc65608b6cab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayerRateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub NewRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlayerSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlayerSource {
    type Vtable = IMediaPlayerSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbd4f8897_1423_4c3e_82c5_0fb1af94f715);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayerSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-media")]
    pub ProtectionManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    ProtectionManager: usize,
    #[cfg(feature = "winrt-media")]
    pub SetProtectionManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    SetProtectionManager: usize,
    #[cfg(all(feature = "winrt-storage", feature = "winrt-"))]
    pub SetFileSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-storage", feature = "winrt-")))]
    SetFileSource: usize,
    #[cfg(all(feature = "winrt-storage", feature = "winrt-"))]
    pub SetStreamSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-storage", feature = "winrt-")))]
    SetStreamSource: usize,
    #[cfg(all(feature = "winrt-media", feature = "winrt-"))]
    pub SetMediaSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-media", feature = "winrt-")))]
    SetMediaSource: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlayerSource2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlayerSource2 {
    type Vtable = IMediaPlayerSource2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x82449b9f_7322_4c0b_b03b_3e69a48260c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayerSource2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlayerSurface(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPlayerSurface {
    type Vtable = IMediaPlayerSurface_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0ed653bc_b736_49c3_830b_764a3845313a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayerSurface_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-ui")]
    pub CompositionSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    CompositionSurface: usize,
    #[cfg(feature = "winrt-ui")]
    pub Compositor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    Compositor: usize,
    pub MediaPlayer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaybackMediaMarker(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlaybackMediaMarker {
    type Vtable = IPlaybackMediaMarker_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc4d22f5c_3c1c_4444_b6b9_778b0422d41a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackMediaMarker_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Time: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub MediaMarkerType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaybackMediaMarkerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlaybackMediaMarkerFactory {
    type Vtable = IPlaybackMediaMarkerFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8c530a78_e0ae_4e1a_a8c8_e23f982a937b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackMediaMarkerFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateFromTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan, mediamarkettype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, text: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaybackMediaMarkerReachedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlaybackMediaMarkerReachedEventArgs {
    type Vtable = IPlaybackMediaMarkerReachedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x578cd1b9_90e2_4e60_abc4_8740b01f6196);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackMediaMarkerReachedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PlaybackMediaMarker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaybackMediaMarkerSequence(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlaybackMediaMarkerSequence {
    type Vtable = IPlaybackMediaMarkerSequence_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf2810cee_638b_46cf_8817_1d111fe9d8c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackMediaMarkerSequence_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Insert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedMetadataPresentationModeChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedMetadataPresentationModeChangedEventArgs {
    type Vtable = ITimedMetadataPresentationModeChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd1636099_65df_45ae_8cef_dc0b53fdc2bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataPresentationModeChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-media")]
    pub Track: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    Track: usize,
    pub OldPresentationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedMetadataTrackPresentationMode) -> ::windows_core::HRESULT,
    pub NewPresentationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedMetadataTrackPresentationMode) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct MediaBreak(::windows_core::IUnknown);
impl MediaBreak {
    pub fn PlaybackList(&self) -> ::windows_core::Result<MediaPlaybackList> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackList)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackList>(result__)
        }
    }
    pub fn PresentationPosition(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PresentationPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn InsertionMethod(&self) -> ::windows_core::Result<MediaBreakInsertionMethod> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaBreakInsertionMethod>::zeroed();
            (::windows_core::Interface::vtable(this).InsertionMethod)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaBreakInsertionMethod>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn CustomProperties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CustomProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::ValueSet>(result__)
        }
    }
    pub fn CanStart(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanStart)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanStart(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCanStart)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create(insertionmethod: MediaBreakInsertionMethod) -> ::windows_core::Result<MediaBreak> {
        Self::IMediaBreakFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), insertionmethod, result__.as_mut_ptr()).from_abi::<MediaBreak>(result__)
        })
    }
    pub fn CreateWithPresentationPosition<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(insertionmethod: MediaBreakInsertionMethod, presentationposition: Param1) -> ::windows_core::Result<MediaBreak> {
        Self::IMediaBreakFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithPresentationPosition)(::windows_core::Interface::as_raw(this), insertionmethod, presentationposition.into_param().abi(), result__.as_mut_ptr()).from_abi::<MediaBreak>(result__)
        })
    }
    pub fn IMediaBreakFactory<R, F: FnOnce(&IMediaBreakFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MediaBreak, IMediaBreakFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MediaBreak {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaBreak {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaBreak {}
impl ::core::fmt::Debug for MediaBreak {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaBreak").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaBreak {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaBreak;{714be270-0def-4ebc-a489-6b34930e1558})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaBreak {
    type Vtable = IMediaBreak_Vtbl;
    const IID: ::windows_core::GUID = <IMediaBreak as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaBreak {
    const NAME: &'static str = "Windows.Media.Playback.MediaBreak";
}
impl ::core::convert::From<MediaBreak> for ::windows_core::IUnknown {
    fn from(value: MediaBreak) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaBreak> for ::windows_core::IUnknown {
    fn from(value: &MediaBreak) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaBreak {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaBreak {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaBreak> for ::windows_core::IInspectable {
    fn from(value: MediaBreak) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaBreak> for ::windows_core::IInspectable {
    fn from(value: &MediaBreak) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaBreak {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaBreak {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaBreak {}
unsafe impl ::core::marker::Sync for MediaBreak {}
#[repr(transparent)]
pub struct MediaBreakEndedEventArgs(::windows_core::IUnknown);
impl MediaBreakEndedEventArgs {
    pub fn MediaBreak(&self) -> ::windows_core::Result<MediaBreak> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MediaBreak)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaBreak>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaBreakEndedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaBreakEndedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaBreakEndedEventArgs {}
impl ::core::fmt::Debug for MediaBreakEndedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaBreakEndedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaBreakEndedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaBreakEndedEventArgs;{32b93276-1c5d-4fee-8732-236dc3a88580})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaBreakEndedEventArgs {
    type Vtable = IMediaBreakEndedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaBreakEndedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaBreakEndedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaBreakEndedEventArgs";
}
impl ::core::convert::From<MediaBreakEndedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaBreakEndedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaBreakEndedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaBreakEndedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaBreakEndedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaBreakEndedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaBreakEndedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaBreakEndedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaBreakEndedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaBreakEndedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaBreakEndedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaBreakEndedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaBreakEndedEventArgs {}
unsafe impl ::core::marker::Sync for MediaBreakEndedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaBreakInsertionMethod(pub i32);
impl MediaBreakInsertionMethod {
    pub const Interrupt: Self = Self(0i32);
    pub const Replace: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaBreakInsertionMethod {}
impl ::core::clone::Clone for MediaBreakInsertionMethod {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaBreakInsertionMethod {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaBreakInsertionMethod {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaBreakInsertionMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaBreakInsertionMethod").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaBreakInsertionMethod {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaBreakInsertionMethod;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MediaBreakManager(::windows_core::IUnknown);
impl MediaBreakManager {
    pub fn BreaksSeekedOver<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaBreakManager, MediaBreakSeekedOverEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).BreaksSeekedOver)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveBreaksSeekedOver<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveBreaksSeekedOver)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn BreakStarted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaBreakManager, MediaBreakStartedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).BreakStarted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveBreakStarted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveBreakStarted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn BreakEnded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaBreakManager, MediaBreakEndedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).BreakEnded)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveBreakEnded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveBreakEnded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn BreakSkipped<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaBreakManager, MediaBreakSkippedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).BreakSkipped)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveBreakSkipped<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveBreakSkipped)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn CurrentBreak(&self) -> ::windows_core::Result<MediaBreak> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentBreak)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaBreak>(result__)
        }
    }
    pub fn PlaybackSession(&self) -> ::windows_core::Result<MediaPlaybackSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackSession)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackSession>(result__)
        }
    }
    pub fn PlayBreak<'a, Param0: ::windows_core::IntoParam<'a, MediaBreak>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PlayBreak)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SkipCurrentBreak(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SkipCurrentBreak)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for MediaBreakManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaBreakManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaBreakManager {}
impl ::core::fmt::Debug for MediaBreakManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaBreakManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaBreakManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaBreakManager;{a854ddb1-feb4-4d9b-9d97-0fdbe58e5e39})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaBreakManager {
    type Vtable = IMediaBreakManager_Vtbl;
    const IID: ::windows_core::GUID = <IMediaBreakManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaBreakManager {
    const NAME: &'static str = "Windows.Media.Playback.MediaBreakManager";
}
impl ::core::convert::From<MediaBreakManager> for ::windows_core::IUnknown {
    fn from(value: MediaBreakManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaBreakManager> for ::windows_core::IUnknown {
    fn from(value: &MediaBreakManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaBreakManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaBreakManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaBreakManager> for ::windows_core::IInspectable {
    fn from(value: MediaBreakManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaBreakManager> for ::windows_core::IInspectable {
    fn from(value: &MediaBreakManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaBreakManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaBreakManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaBreakManager {}
unsafe impl ::core::marker::Sync for MediaBreakManager {}
#[repr(transparent)]
pub struct MediaBreakSchedule(::windows_core::IUnknown);
impl MediaBreakSchedule {
    pub fn ScheduleChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaBreakSchedule, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ScheduleChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveScheduleChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveScheduleChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn InsertMidrollBreak<'a, Param0: ::windows_core::IntoParam<'a, MediaBreak>>(&self, mediabreak: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InsertMidrollBreak)(::windows_core::Interface::as_raw(this), mediabreak.into_param().abi()).ok() }
    }
    pub fn RemoveMidrollBreak<'a, Param0: ::windows_core::IntoParam<'a, MediaBreak>>(&self, mediabreak: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMidrollBreak)(::windows_core::Interface::as_raw(this), mediabreak.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn MidrollBreaks(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<MediaBreak>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MidrollBreaks)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<MediaBreak>>(result__)
        }
    }
    pub fn SetPrerollBreak<'a, Param0: ::windows_core::IntoParam<'a, MediaBreak>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPrerollBreak)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn PrerollBreak(&self) -> ::windows_core::Result<MediaBreak> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PrerollBreak)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaBreak>(result__)
        }
    }
    pub fn SetPostrollBreak<'a, Param0: ::windows_core::IntoParam<'a, MediaBreak>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPostrollBreak)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn PostrollBreak(&self) -> ::windows_core::Result<MediaBreak> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PostrollBreak)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaBreak>(result__)
        }
    }
    pub fn PlaybackItem(&self) -> ::windows_core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackItem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackItem>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaBreakSchedule {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaBreakSchedule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaBreakSchedule {}
impl ::core::fmt::Debug for MediaBreakSchedule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaBreakSchedule").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaBreakSchedule {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaBreakSchedule;{a19a5813-98b6-41d8-83da-f971d22b7bba})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaBreakSchedule {
    type Vtable = IMediaBreakSchedule_Vtbl;
    const IID: ::windows_core::GUID = <IMediaBreakSchedule as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaBreakSchedule {
    const NAME: &'static str = "Windows.Media.Playback.MediaBreakSchedule";
}
impl ::core::convert::From<MediaBreakSchedule> for ::windows_core::IUnknown {
    fn from(value: MediaBreakSchedule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaBreakSchedule> for ::windows_core::IUnknown {
    fn from(value: &MediaBreakSchedule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaBreakSchedule {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaBreakSchedule {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaBreakSchedule> for ::windows_core::IInspectable {
    fn from(value: MediaBreakSchedule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaBreakSchedule> for ::windows_core::IInspectable {
    fn from(value: &MediaBreakSchedule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaBreakSchedule {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaBreakSchedule {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaBreakSchedule {}
unsafe impl ::core::marker::Sync for MediaBreakSchedule {}
#[repr(transparent)]
pub struct MediaBreakSeekedOverEventArgs(::windows_core::IUnknown);
impl MediaBreakSeekedOverEventArgs {
    #[cfg(feature = "winrt-foundation")]
    pub fn SeekedOverBreaks(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<MediaBreak>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SeekedOverBreaks)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<MediaBreak>>(result__)
        }
    }
    pub fn OldPosition(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).OldPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn NewPosition(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).NewPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaBreakSeekedOverEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaBreakSeekedOverEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaBreakSeekedOverEventArgs {}
impl ::core::fmt::Debug for MediaBreakSeekedOverEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaBreakSeekedOverEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaBreakSeekedOverEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaBreakSeekedOverEventArgs;{e5aa6746-0606-4492-b9d3-c3c8fde0a4ea})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaBreakSeekedOverEventArgs {
    type Vtable = IMediaBreakSeekedOverEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaBreakSeekedOverEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaBreakSeekedOverEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaBreakSeekedOverEventArgs";
}
impl ::core::convert::From<MediaBreakSeekedOverEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaBreakSeekedOverEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaBreakSeekedOverEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaBreakSeekedOverEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaBreakSeekedOverEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaBreakSeekedOverEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaBreakSeekedOverEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaBreakSeekedOverEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaBreakSeekedOverEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaBreakSeekedOverEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaBreakSeekedOverEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaBreakSeekedOverEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaBreakSeekedOverEventArgs {}
unsafe impl ::core::marker::Sync for MediaBreakSeekedOverEventArgs {}
#[repr(transparent)]
pub struct MediaBreakSkippedEventArgs(::windows_core::IUnknown);
impl MediaBreakSkippedEventArgs {
    pub fn MediaBreak(&self) -> ::windows_core::Result<MediaBreak> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MediaBreak)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaBreak>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaBreakSkippedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaBreakSkippedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaBreakSkippedEventArgs {}
impl ::core::fmt::Debug for MediaBreakSkippedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaBreakSkippedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaBreakSkippedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaBreakSkippedEventArgs;{6ee94c05-2f54-4a3e-a3ab-24c3b270b4a3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaBreakSkippedEventArgs {
    type Vtable = IMediaBreakSkippedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaBreakSkippedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaBreakSkippedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaBreakSkippedEventArgs";
}
impl ::core::convert::From<MediaBreakSkippedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaBreakSkippedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaBreakSkippedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaBreakSkippedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaBreakSkippedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaBreakSkippedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaBreakSkippedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaBreakSkippedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaBreakSkippedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaBreakSkippedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaBreakSkippedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaBreakSkippedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaBreakSkippedEventArgs {}
unsafe impl ::core::marker::Sync for MediaBreakSkippedEventArgs {}
#[repr(transparent)]
pub struct MediaBreakStartedEventArgs(::windows_core::IUnknown);
impl MediaBreakStartedEventArgs {
    pub fn MediaBreak(&self) -> ::windows_core::Result<MediaBreak> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MediaBreak)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaBreak>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaBreakStartedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaBreakStartedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaBreakStartedEventArgs {}
impl ::core::fmt::Debug for MediaBreakStartedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaBreakStartedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaBreakStartedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaBreakStartedEventArgs;{a87efe71-dfd4-454a-956e-0a4a648395f8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaBreakStartedEventArgs {
    type Vtable = IMediaBreakStartedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaBreakStartedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaBreakStartedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaBreakStartedEventArgs";
}
impl ::core::convert::From<MediaBreakStartedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaBreakStartedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaBreakStartedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaBreakStartedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaBreakStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaBreakStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaBreakStartedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaBreakStartedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaBreakStartedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaBreakStartedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaBreakStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaBreakStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaBreakStartedEventArgs {}
unsafe impl ::core::marker::Sync for MediaBreakStartedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaCommandEnablingRule(pub i32);
impl MediaCommandEnablingRule {
    pub const Auto: Self = Self(0i32);
    pub const Always: Self = Self(1i32);
    pub const Never: Self = Self(2i32);
}
impl ::core::marker::Copy for MediaCommandEnablingRule {}
impl ::core::clone::Clone for MediaCommandEnablingRule {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaCommandEnablingRule {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaCommandEnablingRule {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaCommandEnablingRule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCommandEnablingRule").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaCommandEnablingRule {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaCommandEnablingRule;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MediaItemDisplayProperties(::windows_core::IUnknown);
impl MediaItemDisplayProperties {
    pub fn Type(&self) -> ::windows_core::Result<super::MediaPlaybackType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::MediaPlaybackType>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaPlaybackType>(result__)
        }
    }
    pub fn SetType(&self, value: super::MediaPlaybackType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MusicProperties(&self) -> ::windows_core::Result<super::MusicDisplayProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MusicProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MusicDisplayProperties>(result__)
        }
    }
    pub fn VideoProperties(&self) -> ::windows_core::Result<super::VideoDisplayProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::VideoDisplayProperties>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn Thumbnail(&self) -> ::windows_core::Result<::winrt_storage::Streams::RandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Thumbnail)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::RandomAccessStreamReference>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn SetThumbnail<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::RandomAccessStreamReference>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetThumbnail)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ClearAll(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ClearAll)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for MediaItemDisplayProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaItemDisplayProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaItemDisplayProperties {}
impl ::core::fmt::Debug for MediaItemDisplayProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaItemDisplayProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaItemDisplayProperties {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaItemDisplayProperties;{1e3c1b48-7097-4384-a217-c1291dfa8c16})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaItemDisplayProperties {
    type Vtable = IMediaItemDisplayProperties_Vtbl;
    const IID: ::windows_core::GUID = <IMediaItemDisplayProperties as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaItemDisplayProperties {
    const NAME: &'static str = "Windows.Media.Playback.MediaItemDisplayProperties";
}
impl ::core::convert::From<MediaItemDisplayProperties> for ::windows_core::IUnknown {
    fn from(value: MediaItemDisplayProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaItemDisplayProperties> for ::windows_core::IUnknown {
    fn from(value: &MediaItemDisplayProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaItemDisplayProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaItemDisplayProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaItemDisplayProperties> for ::windows_core::IInspectable {
    fn from(value: MediaItemDisplayProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaItemDisplayProperties> for ::windows_core::IInspectable {
    fn from(value: &MediaItemDisplayProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaItemDisplayProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaItemDisplayProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaItemDisplayProperties {}
unsafe impl ::core::marker::Sync for MediaItemDisplayProperties {}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct MediaPlaybackAudioTrackList(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl MediaPlaybackAudioTrackList {
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<super::Core::AudioTrack>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<super::Core::AudioTrack>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<super::Core::AudioTrack>>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn SelectedIndexChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<super::Core::ISingleSelectMediaTrackList, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedIndexChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn RemoveSelectedIndexChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSelectedIndexChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn SetSelectedIndex(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedIndex)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn SelectedIndex(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedIndex)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<super::Core::AudioTrack> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<super::Core::AudioTrack>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, super::Core::AudioTrack>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<super::Core::AudioTrack>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for MediaPlaybackAudioTrackList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for MediaPlaybackAudioTrackList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for MediaPlaybackAudioTrackList {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for MediaPlaybackAudioTrackList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackAudioTrackList").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for MediaPlaybackAudioTrackList {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackAudioTrackList;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.Media.Core.AudioTrack;{03e1fafc-c931-491a-b46b-c10ee8c256b7})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for MediaPlaybackAudioTrackList {
    type Vtable = ::winrt_foundation::Collections::IVectorView_Vtbl<super::Core::AudioTrack>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IVectorView<super::Core::AudioTrack> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for MediaPlaybackAudioTrackList {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackAudioTrackList";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for MediaPlaybackAudioTrackList {
    type Item = super::Core::AudioTrack;
    type IntoIter = ::winrt_foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &MediaPlaybackAudioTrackList {
    type Item = super::Core::AudioTrack;
    type IntoIter = ::winrt_foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<MediaPlaybackAudioTrackList> for ::windows_core::IUnknown {
    fn from(value: MediaPlaybackAudioTrackList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&MediaPlaybackAudioTrackList> for ::windows_core::IUnknown {
    fn from(value: &MediaPlaybackAudioTrackList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaPlaybackAudioTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaPlaybackAudioTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<MediaPlaybackAudioTrackList> for ::windows_core::IInspectable {
    fn from(value: MediaPlaybackAudioTrackList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&MediaPlaybackAudioTrackList> for ::windows_core::IInspectable {
    fn from(value: &MediaPlaybackAudioTrackList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaPlaybackAudioTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaPlaybackAudioTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl ::core::convert::TryFrom<MediaPlaybackAudioTrackList> for ::winrt_foundation::Collections::IIterable<super::Core::AudioTrack> {
    type Error = ::windows_core::Error;
    fn try_from(value: MediaPlaybackAudioTrackList) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl ::core::convert::TryFrom<&MediaPlaybackAudioTrackList> for ::winrt_foundation::Collections::IIterable<super::Core::AudioTrack> {
    type Error = ::windows_core::Error;
    fn try_from(value: &MediaPlaybackAudioTrackList) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<super::Core::AudioTrack>> for MediaPlaybackAudioTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<super::Core::AudioTrack>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<super::Core::AudioTrack>> for &MediaPlaybackAudioTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<super::Core::AudioTrack>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<super::Core::AudioTrack>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl ::core::convert::TryFrom<MediaPlaybackAudioTrackList> for super::Core::ISingleSelectMediaTrackList {
    type Error = ::windows_core::Error;
    fn try_from(value: MediaPlaybackAudioTrackList) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl ::core::convert::TryFrom<&MediaPlaybackAudioTrackList> for super::Core::ISingleSelectMediaTrackList {
    type Error = ::windows_core::Error;
    fn try_from(value: &MediaPlaybackAudioTrackList) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl<'a> ::windows_core::IntoParam<'a, super::Core::ISingleSelectMediaTrackList> for MediaPlaybackAudioTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, super::Core::ISingleSelectMediaTrackList> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl<'a> ::windows_core::IntoParam<'a, super::Core::ISingleSelectMediaTrackList> for &MediaPlaybackAudioTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, super::Core::ISingleSelectMediaTrackList> {
        ::core::convert::TryInto::<super::Core::ISingleSelectMediaTrackList>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl ::core::convert::TryFrom<MediaPlaybackAudioTrackList> for ::winrt_foundation::Collections::IVectorView<super::Core::AudioTrack> {
    type Error = ::windows_core::Error;
    fn try_from(value: MediaPlaybackAudioTrackList) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl ::core::convert::TryFrom<&MediaPlaybackAudioTrackList> for ::winrt_foundation::Collections::IVectorView<super::Core::AudioTrack> {
    type Error = ::windows_core::Error;
    fn try_from(value: &MediaPlaybackAudioTrackList) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<super::Core::AudioTrack>> for MediaPlaybackAudioTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVectorView<super::Core::AudioTrack>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<super::Core::AudioTrack>> for &MediaPlaybackAudioTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVectorView<super::Core::AudioTrack>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVectorView<super::Core::AudioTrack>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Send for MediaPlaybackAudioTrackList {}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Sync for MediaPlaybackAudioTrackList {}
#[repr(transparent)]
pub struct MediaPlaybackCommandManager(::windows_core::IUnknown);
impl MediaPlaybackCommandManager {
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MediaPlayer(&self) -> ::windows_core::Result<MediaPlayer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MediaPlayer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlayer>(result__)
        }
    }
    pub fn PlayBehavior(&self) -> ::windows_core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PlayBehavior)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackCommandManagerCommandBehavior>(result__)
        }
    }
    pub fn PauseBehavior(&self) -> ::windows_core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PauseBehavior)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackCommandManagerCommandBehavior>(result__)
        }
    }
    pub fn NextBehavior(&self) -> ::windows_core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NextBehavior)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackCommandManagerCommandBehavior>(result__)
        }
    }
    pub fn PreviousBehavior(&self) -> ::windows_core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousBehavior)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackCommandManagerCommandBehavior>(result__)
        }
    }
    pub fn FastForwardBehavior(&self) -> ::windows_core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FastForwardBehavior)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackCommandManagerCommandBehavior>(result__)
        }
    }
    pub fn RewindBehavior(&self) -> ::windows_core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RewindBehavior)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackCommandManagerCommandBehavior>(result__)
        }
    }
    pub fn ShuffleBehavior(&self) -> ::windows_core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShuffleBehavior)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackCommandManagerCommandBehavior>(result__)
        }
    }
    pub fn AutoRepeatModeBehavior(&self) -> ::windows_core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AutoRepeatModeBehavior)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackCommandManagerCommandBehavior>(result__)
        }
    }
    pub fn PositionBehavior(&self) -> ::windows_core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PositionBehavior)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackCommandManagerCommandBehavior>(result__)
        }
    }
    pub fn RateBehavior(&self) -> ::windows_core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RateBehavior)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackCommandManagerCommandBehavior>(result__)
        }
    }
    pub fn PlayReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPlayReceivedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PlayReceived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePlayReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePlayReceived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn PauseReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPauseReceivedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PauseReceived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePauseReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePauseReceived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn NextReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerNextReceivedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).NextReceived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveNextReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveNextReceived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn PreviousReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPreviousReceivedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousReceived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePreviousReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePreviousReceived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn FastForwardReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerFastForwardReceivedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).FastForwardReceived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveFastForwardReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFastForwardReceived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn RewindReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerRewindReceivedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).RewindReceived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveRewindReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRewindReceived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ShuffleReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerShuffleReceivedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ShuffleReceived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveShuffleReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveShuffleReceived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn AutoRepeatModeReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AutoRepeatModeReceived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAutoRepeatModeReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAutoRepeatModeReceived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn PositionReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPositionReceivedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PositionReceived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePositionReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePositionReceived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn RateReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerRateReceivedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).RateReceived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveRateReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRateReceived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for MediaPlaybackCommandManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackCommandManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackCommandManager {}
impl ::core::fmt::Debug for MediaPlaybackCommandManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackCommandManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlaybackCommandManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManager;{5acee5a6-5cb6-4a5a-8521-cc86b1c1ed37})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaPlaybackCommandManager {
    type Vtable = IMediaPlaybackCommandManager_Vtbl;
    const IID: ::windows_core::GUID = <IMediaPlaybackCommandManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaPlaybackCommandManager {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManager";
}
impl ::core::convert::From<MediaPlaybackCommandManager> for ::windows_core::IUnknown {
    fn from(value: MediaPlaybackCommandManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManager> for ::windows_core::IUnknown {
    fn from(value: &MediaPlaybackCommandManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaPlaybackCommandManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaPlaybackCommandManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaPlaybackCommandManager> for ::windows_core::IInspectable {
    fn from(value: MediaPlaybackCommandManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManager> for ::windows_core::IInspectable {
    fn from(value: &MediaPlaybackCommandManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaPlaybackCommandManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaPlaybackCommandManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackCommandManager {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManager {}
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs(::windows_core::IUnknown);
impl MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AutoRepeatMode(&self) -> ::windows_core::Result<super::MediaPlaybackAutoRepeatMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::MediaPlaybackAutoRepeatMode>::zeroed();
            (::windows_core::Interface::vtable(this).AutoRepeatMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaPlaybackAutoRepeatMode>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {}
impl ::core::fmt::Debug for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs;{3d6f4f23-5230-4411-a0e9-bad94c2a045c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs";
}
impl ::core::convert::From<MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {}
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerCommandBehavior(::windows_core::IUnknown);
impl MediaPlaybackCommandManagerCommandBehavior {
    pub fn CommandManager(&self) -> ::windows_core::Result<MediaPlaybackCommandManager> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CommandManager)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackCommandManager>(result__)
        }
    }
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn EnablingRule(&self) -> ::windows_core::Result<MediaCommandEnablingRule> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaCommandEnablingRule>::zeroed();
            (::windows_core::Interface::vtable(this).EnablingRule)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaCommandEnablingRule>(result__)
        }
    }
    pub fn SetEnablingRule(&self, value: MediaCommandEnablingRule) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnablingRule)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsEnabledChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackCommandManagerCommandBehavior, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabledChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveIsEnabledChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveIsEnabledChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for MediaPlaybackCommandManagerCommandBehavior {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackCommandManagerCommandBehavior {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackCommandManagerCommandBehavior {}
impl ::core::fmt::Debug for MediaPlaybackCommandManagerCommandBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackCommandManagerCommandBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlaybackCommandManagerCommandBehavior {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerCommandBehavior;{786c1e78-ce78-4a10-afd6-843fcbb90c2e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaPlaybackCommandManagerCommandBehavior {
    type Vtable = IMediaPlaybackCommandManagerCommandBehavior_Vtbl;
    const IID: ::windows_core::GUID = <IMediaPlaybackCommandManagerCommandBehavior as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaPlaybackCommandManagerCommandBehavior {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerCommandBehavior";
}
impl ::core::convert::From<MediaPlaybackCommandManagerCommandBehavior> for ::windows_core::IUnknown {
    fn from(value: MediaPlaybackCommandManagerCommandBehavior) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerCommandBehavior> for ::windows_core::IUnknown {
    fn from(value: &MediaPlaybackCommandManagerCommandBehavior) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaPlaybackCommandManagerCommandBehavior {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaPlaybackCommandManagerCommandBehavior {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaPlaybackCommandManagerCommandBehavior> for ::windows_core::IInspectable {
    fn from(value: MediaPlaybackCommandManagerCommandBehavior) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerCommandBehavior> for ::windows_core::IInspectable {
    fn from(value: &MediaPlaybackCommandManagerCommandBehavior) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaPlaybackCommandManagerCommandBehavior {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaPlaybackCommandManagerCommandBehavior {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerCommandBehavior {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerCommandBehavior {}
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerFastForwardReceivedEventArgs(::windows_core::IUnknown);
impl MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {}
impl ::core::fmt::Debug for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackCommandManagerFastForwardReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerFastForwardReceivedEventArgs;{30f064d9-b491-4d0a-bc21-3098bd1332e9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerFastForwardReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaPlaybackCommandManagerFastForwardReceivedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerFastForwardReceivedEventArgs";
}
impl ::core::convert::From<MediaPlaybackCommandManagerFastForwardReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaPlaybackCommandManagerFastForwardReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerFastForwardReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaPlaybackCommandManagerFastForwardReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaPlaybackCommandManagerFastForwardReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaPlaybackCommandManagerFastForwardReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerFastForwardReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaPlaybackCommandManagerFastForwardReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {}
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerNextReceivedEventArgs(::windows_core::IUnknown);
impl MediaPlaybackCommandManagerNextReceivedEventArgs {
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackCommandManagerNextReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackCommandManagerNextReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackCommandManagerNextReceivedEventArgs {}
impl ::core::fmt::Debug for MediaPlaybackCommandManagerNextReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackCommandManagerNextReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlaybackCommandManagerNextReceivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerNextReceivedEventArgs;{e1504433-a2b0-45d4-b9de-5f42ac14a839})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaPlaybackCommandManagerNextReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerNextReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaPlaybackCommandManagerNextReceivedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaPlaybackCommandManagerNextReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerNextReceivedEventArgs";
}
impl ::core::convert::From<MediaPlaybackCommandManagerNextReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaPlaybackCommandManagerNextReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerNextReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaPlaybackCommandManagerNextReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaPlaybackCommandManagerNextReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaPlaybackCommandManagerNextReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaPlaybackCommandManagerNextReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaPlaybackCommandManagerNextReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerNextReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaPlaybackCommandManagerNextReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaPlaybackCommandManagerNextReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaPlaybackCommandManagerNextReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerNextReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerNextReceivedEventArgs {}
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerPauseReceivedEventArgs(::windows_core::IUnknown);
impl MediaPlaybackCommandManagerPauseReceivedEventArgs {
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackCommandManagerPauseReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackCommandManagerPauseReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackCommandManagerPauseReceivedEventArgs {}
impl ::core::fmt::Debug for MediaPlaybackCommandManagerPauseReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackCommandManagerPauseReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlaybackCommandManagerPauseReceivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerPauseReceivedEventArgs;{5ceccd1c-c25c-4221-b16c-c3c98ce012d6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaPlaybackCommandManagerPauseReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerPauseReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaPlaybackCommandManagerPauseReceivedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaPlaybackCommandManagerPauseReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerPauseReceivedEventArgs";
}
impl ::core::convert::From<MediaPlaybackCommandManagerPauseReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaPlaybackCommandManagerPauseReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerPauseReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaPlaybackCommandManagerPauseReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaPlaybackCommandManagerPauseReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaPlaybackCommandManagerPauseReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaPlaybackCommandManagerPauseReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaPlaybackCommandManagerPauseReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerPauseReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaPlaybackCommandManagerPauseReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaPlaybackCommandManagerPauseReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaPlaybackCommandManagerPauseReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerPauseReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerPauseReceivedEventArgs {}
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerPlayReceivedEventArgs(::windows_core::IUnknown);
impl MediaPlaybackCommandManagerPlayReceivedEventArgs {
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackCommandManagerPlayReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackCommandManagerPlayReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackCommandManagerPlayReceivedEventArgs {}
impl ::core::fmt::Debug for MediaPlaybackCommandManagerPlayReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackCommandManagerPlayReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlaybackCommandManagerPlayReceivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerPlayReceivedEventArgs;{9af0004e-578b-4c56-a006-16159d888a48})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaPlaybackCommandManagerPlayReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerPlayReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaPlaybackCommandManagerPlayReceivedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaPlaybackCommandManagerPlayReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerPlayReceivedEventArgs";
}
impl ::core::convert::From<MediaPlaybackCommandManagerPlayReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaPlaybackCommandManagerPlayReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerPlayReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaPlaybackCommandManagerPlayReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaPlaybackCommandManagerPlayReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaPlaybackCommandManagerPlayReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaPlaybackCommandManagerPlayReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaPlaybackCommandManagerPlayReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerPlayReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaPlaybackCommandManagerPlayReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaPlaybackCommandManagerPlayReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaPlaybackCommandManagerPlayReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerPlayReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerPlayReceivedEventArgs {}
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerPositionReceivedEventArgs(::windows_core::IUnknown);
impl MediaPlaybackCommandManagerPositionReceivedEventArgs {
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackCommandManagerPositionReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackCommandManagerPositionReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackCommandManagerPositionReceivedEventArgs {}
impl ::core::fmt::Debug for MediaPlaybackCommandManagerPositionReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackCommandManagerPositionReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlaybackCommandManagerPositionReceivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerPositionReceivedEventArgs;{5591a754-d627-4bdd-a90d-86a015b24902})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaPlaybackCommandManagerPositionReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerPositionReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaPlaybackCommandManagerPositionReceivedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaPlaybackCommandManagerPositionReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerPositionReceivedEventArgs";
}
impl ::core::convert::From<MediaPlaybackCommandManagerPositionReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaPlaybackCommandManagerPositionReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerPositionReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaPlaybackCommandManagerPositionReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaPlaybackCommandManagerPositionReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaPlaybackCommandManagerPositionReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaPlaybackCommandManagerPositionReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaPlaybackCommandManagerPositionReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerPositionReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaPlaybackCommandManagerPositionReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaPlaybackCommandManagerPositionReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaPlaybackCommandManagerPositionReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerPositionReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerPositionReceivedEventArgs {}
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerPreviousReceivedEventArgs(::windows_core::IUnknown);
impl MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackCommandManagerPreviousReceivedEventArgs {}
impl ::core::fmt::Debug for MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackCommandManagerPreviousReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerPreviousReceivedEventArgs;{525e3081-4632-4f76-99b1-d771623f6287})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerPreviousReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaPlaybackCommandManagerPreviousReceivedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerPreviousReceivedEventArgs";
}
impl ::core::convert::From<MediaPlaybackCommandManagerPreviousReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaPlaybackCommandManagerPreviousReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerPreviousReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaPlaybackCommandManagerPreviousReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaPlaybackCommandManagerPreviousReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaPlaybackCommandManagerPreviousReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerPreviousReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaPlaybackCommandManagerPreviousReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerPreviousReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerPreviousReceivedEventArgs {}
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerRateReceivedEventArgs(::windows_core::IUnknown);
impl MediaPlaybackCommandManagerRateReceivedEventArgs {
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PlaybackRate(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackRate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackCommandManagerRateReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackCommandManagerRateReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackCommandManagerRateReceivedEventArgs {}
impl ::core::fmt::Debug for MediaPlaybackCommandManagerRateReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackCommandManagerRateReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlaybackCommandManagerRateReceivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerRateReceivedEventArgs;{18ea3939-4a16-4169-8b05-3eb9f5ff78eb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaPlaybackCommandManagerRateReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerRateReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaPlaybackCommandManagerRateReceivedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaPlaybackCommandManagerRateReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerRateReceivedEventArgs";
}
impl ::core::convert::From<MediaPlaybackCommandManagerRateReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaPlaybackCommandManagerRateReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerRateReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaPlaybackCommandManagerRateReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaPlaybackCommandManagerRateReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaPlaybackCommandManagerRateReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaPlaybackCommandManagerRateReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaPlaybackCommandManagerRateReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerRateReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaPlaybackCommandManagerRateReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaPlaybackCommandManagerRateReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaPlaybackCommandManagerRateReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerRateReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerRateReceivedEventArgs {}
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerRewindReceivedEventArgs(::windows_core::IUnknown);
impl MediaPlaybackCommandManagerRewindReceivedEventArgs {
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackCommandManagerRewindReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackCommandManagerRewindReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackCommandManagerRewindReceivedEventArgs {}
impl ::core::fmt::Debug for MediaPlaybackCommandManagerRewindReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackCommandManagerRewindReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlaybackCommandManagerRewindReceivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerRewindReceivedEventArgs;{9f085947-a3c0-425d-aaef-97ba7898b141})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaPlaybackCommandManagerRewindReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerRewindReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaPlaybackCommandManagerRewindReceivedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaPlaybackCommandManagerRewindReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerRewindReceivedEventArgs";
}
impl ::core::convert::From<MediaPlaybackCommandManagerRewindReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaPlaybackCommandManagerRewindReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerRewindReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaPlaybackCommandManagerRewindReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaPlaybackCommandManagerRewindReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaPlaybackCommandManagerRewindReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaPlaybackCommandManagerRewindReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaPlaybackCommandManagerRewindReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerRewindReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaPlaybackCommandManagerRewindReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaPlaybackCommandManagerRewindReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaPlaybackCommandManagerRewindReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerRewindReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerRewindReceivedEventArgs {}
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerShuffleReceivedEventArgs(::windows_core::IUnknown);
impl MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsShuffleRequested(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsShuffleRequested)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackCommandManagerShuffleReceivedEventArgs {}
impl ::core::fmt::Debug for MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackCommandManagerShuffleReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerShuffleReceivedEventArgs;{50a05cef-63ee-4a96-b7b5-fee08b9ff90c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerShuffleReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaPlaybackCommandManagerShuffleReceivedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerShuffleReceivedEventArgs";
}
impl ::core::convert::From<MediaPlaybackCommandManagerShuffleReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaPlaybackCommandManagerShuffleReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerShuffleReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaPlaybackCommandManagerShuffleReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaPlaybackCommandManagerShuffleReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaPlaybackCommandManagerShuffleReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackCommandManagerShuffleReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaPlaybackCommandManagerShuffleReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerShuffleReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerShuffleReceivedEventArgs {}
#[repr(transparent)]
pub struct MediaPlaybackItem(::windows_core::IUnknown);
impl MediaPlaybackItem {
    #[cfg(feature = "winrt-foundation")]
    pub fn AudioTracksChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackItem, ::winrt_foundation::Collections::IVectorChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AudioTracksChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAudioTracksChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAudioTracksChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn VideoTracksChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackItem, ::winrt_foundation::Collections::IVectorChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).VideoTracksChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveVideoTracksChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveVideoTracksChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn TimedMetadataTracksChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackItem, ::winrt_foundation::Collections::IVectorChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).TimedMetadataTracksChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveTimedMetadataTracksChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveTimedMetadataTracksChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn Source(&self) -> ::windows_core::Result<super::Core::MediaSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Source)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Core::MediaSource>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn AudioTracks(&self) -> ::windows_core::Result<MediaPlaybackAudioTrackList> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AudioTracks)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackAudioTrackList>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn VideoTracks(&self) -> ::windows_core::Result<MediaPlaybackVideoTrackList> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoTracks)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackVideoTrackList>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn TimedMetadataTracks(&self) -> ::windows_core::Result<MediaPlaybackTimedMetadataTrackList> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TimedMetadataTracks)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackTimedMetadataTrackList>(result__)
        }
    }
    pub fn BreakSchedule(&self) -> ::windows_core::Result<MediaBreakSchedule> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackItem2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BreakSchedule)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaBreakSchedule>(result__)
        }
    }
    pub fn StartTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackItem2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn DurationLimit(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackItem2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DurationLimit)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn CanSkip(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackItem2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanSkip)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanSkip(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackItem2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCanSkip)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetDisplayProperties(&self) -> ::windows_core::Result<MediaItemDisplayProperties> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackItem2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDisplayProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaItemDisplayProperties>(result__)
        }
    }
    pub fn ApplyDisplayProperties<'a, Param0: ::windows_core::IntoParam<'a, MediaItemDisplayProperties>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackItem2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ApplyDisplayProperties)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsDisabledInPlaybackList(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackItem3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDisabledInPlaybackList)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsDisabledInPlaybackList(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackItem3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDisabledInPlaybackList)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TotalDownloadProgress(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackItem3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).TotalDownloadProgress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn AutoLoadedDisplayProperties(&self) -> ::windows_core::Result<AutoLoadedDisplayPropertyKind> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackItem3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AutoLoadedDisplayPropertyKind>::zeroed();
            (::windows_core::Interface::vtable(this).AutoLoadedDisplayProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutoLoadedDisplayPropertyKind>(result__)
        }
    }
    pub fn SetAutoLoadedDisplayProperties(&self, value: AutoLoadedDisplayPropertyKind) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackItem3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAutoLoadedDisplayProperties)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::Core::MediaSource>>(source: Param0) -> ::windows_core::Result<MediaPlaybackItem> {
        Self::IMediaPlaybackItemFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), source.into_param().abi(), result__.as_mut_ptr()).from_abi::<MediaPlaybackItem>(result__)
        })
    }
    #[cfg(feature = "winrt-media")]
    pub fn CreateWithStartTime<'a, Param0: ::windows_core::IntoParam<'a, super::Core::MediaSource>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(source: Param0, starttime: Param1) -> ::windows_core::Result<MediaPlaybackItem> {
        Self::IMediaPlaybackItemFactory2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithStartTime)(::windows_core::Interface::as_raw(this), source.into_param().abi(), starttime.into_param().abi(), result__.as_mut_ptr()).from_abi::<MediaPlaybackItem>(result__)
        })
    }
    #[cfg(feature = "winrt-media")]
    pub fn CreateWithStartTimeAndDurationLimit<'a, Param0: ::windows_core::IntoParam<'a, super::Core::MediaSource>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(source: Param0, starttime: Param1, durationlimit: Param2) -> ::windows_core::Result<MediaPlaybackItem> {
        Self::IMediaPlaybackItemFactory2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithStartTimeAndDurationLimit)(::windows_core::Interface::as_raw(this), source.into_param().abi(), starttime.into_param().abi(), durationlimit.into_param().abi(), result__.as_mut_ptr()).from_abi::<MediaPlaybackItem>(result__)
        })
    }
    #[cfg(feature = "winrt-media")]
    pub fn FindFromMediaSource<'a, Param0: ::windows_core::IntoParam<'a, super::Core::MediaSource>>(source: Param0) -> ::windows_core::Result<MediaPlaybackItem> {
        Self::IMediaPlaybackItemStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindFromMediaSource)(::windows_core::Interface::as_raw(this), source.into_param().abi(), result__.as_mut_ptr()).from_abi::<MediaPlaybackItem>(result__)
        })
    }
    pub fn IMediaPlaybackItemFactory<R, F: FnOnce(&IMediaPlaybackItemFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MediaPlaybackItem, IMediaPlaybackItemFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMediaPlaybackItemFactory2<R, F: FnOnce(&IMediaPlaybackItemFactory2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MediaPlaybackItem, IMediaPlaybackItemFactory2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMediaPlaybackItemStatics<R, F: FnOnce(&IMediaPlaybackItemStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MediaPlaybackItem, IMediaPlaybackItemStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MediaPlaybackItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackItem {}
impl ::core::fmt::Debug for MediaPlaybackItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackItem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlaybackItem {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackItem;{047097d2-e4af-48ab-b283-6929e674ece2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaPlaybackItem {
    type Vtable = IMediaPlaybackItem_Vtbl;
    const IID: ::windows_core::GUID = <IMediaPlaybackItem as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaPlaybackItem {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackItem";
}
impl ::core::convert::From<MediaPlaybackItem> for ::windows_core::IUnknown {
    fn from(value: MediaPlaybackItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackItem> for ::windows_core::IUnknown {
    fn from(value: &MediaPlaybackItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaPlaybackItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaPlaybackItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaPlaybackItem> for ::windows_core::IInspectable {
    fn from(value: MediaPlaybackItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackItem> for ::windows_core::IInspectable {
    fn from(value: &MediaPlaybackItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaPlaybackItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaPlaybackItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MediaPlaybackItem> for IMediaPlaybackSource {
    type Error = ::windows_core::Error;
    fn try_from(value: MediaPlaybackItem) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MediaPlaybackItem> for IMediaPlaybackSource {
    type Error = ::windows_core::Error;
    fn try_from(value: &MediaPlaybackItem) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaPlaybackSource> for MediaPlaybackItem {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaPlaybackSource> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaPlaybackSource> for &MediaPlaybackItem {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaPlaybackSource> {
        ::core::convert::TryInto::<IMediaPlaybackSource>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackItem {}
unsafe impl ::core::marker::Sync for MediaPlaybackItem {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaPlaybackItemChangedReason(pub i32);
impl MediaPlaybackItemChangedReason {
    pub const InitialItem: Self = Self(0i32);
    pub const EndOfStream: Self = Self(1i32);
    pub const Error: Self = Self(2i32);
    pub const AppRequested: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaPlaybackItemChangedReason {}
impl ::core::clone::Clone for MediaPlaybackItemChangedReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaPlaybackItemChangedReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaPlaybackItemChangedReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaPlaybackItemChangedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackItemChangedReason").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlaybackItemChangedReason {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlaybackItemChangedReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MediaPlaybackItemError(::windows_core::IUnknown);
impl MediaPlaybackItemError {
    pub fn ErrorCode(&self) -> ::windows_core::Result<MediaPlaybackItemErrorCode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaPlaybackItemErrorCode>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackItemErrorCode>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackItemError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackItemError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackItemError {}
impl ::core::fmt::Debug for MediaPlaybackItemError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackItemError").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlaybackItemError {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackItemError;{69fbef2b-dcd6-4df9-a450-dbf4c6f1c2c2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaPlaybackItemError {
    type Vtable = IMediaPlaybackItemError_Vtbl;
    const IID: ::windows_core::GUID = <IMediaPlaybackItemError as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaPlaybackItemError {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackItemError";
}
impl ::core::convert::From<MediaPlaybackItemError> for ::windows_core::IUnknown {
    fn from(value: MediaPlaybackItemError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackItemError> for ::windows_core::IUnknown {
    fn from(value: &MediaPlaybackItemError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaPlaybackItemError {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaPlaybackItemError {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaPlaybackItemError> for ::windows_core::IInspectable {
    fn from(value: MediaPlaybackItemError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackItemError> for ::windows_core::IInspectable {
    fn from(value: &MediaPlaybackItemError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaPlaybackItemError {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaPlaybackItemError {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackItemError {}
unsafe impl ::core::marker::Sync for MediaPlaybackItemError {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaPlaybackItemErrorCode(pub i32);
impl MediaPlaybackItemErrorCode {
    pub const None: Self = Self(0i32);
    pub const Aborted: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const DecodeError: Self = Self(3i32);
    pub const SourceNotSupportedError: Self = Self(4i32);
    pub const EncryptionError: Self = Self(5i32);
}
impl ::core::marker::Copy for MediaPlaybackItemErrorCode {}
impl ::core::clone::Clone for MediaPlaybackItemErrorCode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaPlaybackItemErrorCode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaPlaybackItemErrorCode {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaPlaybackItemErrorCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackItemErrorCode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlaybackItemErrorCode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlaybackItemErrorCode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MediaPlaybackItemFailedEventArgs(::windows_core::IUnknown);
impl MediaPlaybackItemFailedEventArgs {
    pub fn Item(&self) -> ::windows_core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Item)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackItem>(result__)
        }
    }
    pub fn Error(&self) -> ::windows_core::Result<MediaPlaybackItemError> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Error)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackItemError>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackItemFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackItemFailedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackItemFailedEventArgs {}
impl ::core::fmt::Debug for MediaPlaybackItemFailedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackItemFailedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlaybackItemFailedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackItemFailedEventArgs;{7703134a-e9a7-47c3-862c-c656d30683d4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaPlaybackItemFailedEventArgs {
    type Vtable = IMediaPlaybackItemFailedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaPlaybackItemFailedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaPlaybackItemFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackItemFailedEventArgs";
}
impl ::core::convert::From<MediaPlaybackItemFailedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaPlaybackItemFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackItemFailedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaPlaybackItemFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaPlaybackItemFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaPlaybackItemFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaPlaybackItemFailedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaPlaybackItemFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackItemFailedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaPlaybackItemFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaPlaybackItemFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaPlaybackItemFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackItemFailedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackItemFailedEventArgs {}
#[repr(transparent)]
pub struct MediaPlaybackItemOpenedEventArgs(::windows_core::IUnknown);
impl MediaPlaybackItemOpenedEventArgs {
    pub fn Item(&self) -> ::windows_core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Item)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackItem>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackItemOpenedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackItemOpenedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackItemOpenedEventArgs {}
impl ::core::fmt::Debug for MediaPlaybackItemOpenedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackItemOpenedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlaybackItemOpenedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackItemOpenedEventArgs;{cbd9bd82-3037-4fbe-ae8f-39fc39edf4ef})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaPlaybackItemOpenedEventArgs {
    type Vtable = IMediaPlaybackItemOpenedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaPlaybackItemOpenedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaPlaybackItemOpenedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackItemOpenedEventArgs";
}
impl ::core::convert::From<MediaPlaybackItemOpenedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaPlaybackItemOpenedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackItemOpenedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaPlaybackItemOpenedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaPlaybackItemOpenedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaPlaybackItemOpenedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaPlaybackItemOpenedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaPlaybackItemOpenedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackItemOpenedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaPlaybackItemOpenedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaPlaybackItemOpenedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaPlaybackItemOpenedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackItemOpenedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackItemOpenedEventArgs {}
#[repr(transparent)]
pub struct MediaPlaybackList(::windows_core::IUnknown);
impl MediaPlaybackList {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MediaPlaybackList, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ItemFailed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackList, MediaPlaybackItemFailedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ItemFailed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveItemFailed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveItemFailed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn CurrentItemChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackList, CurrentMediaPlaybackItemChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentItemChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCurrentItemChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCurrentItemChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ItemOpened<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackList, MediaPlaybackItemOpenedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ItemOpened)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveItemOpened<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveItemOpened)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Items(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IObservableVector<MediaPlaybackItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Items)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IObservableVector<MediaPlaybackItem>>(result__)
        }
    }
    pub fn AutoRepeatEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AutoRepeatEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAutoRepeatEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAutoRepeatEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShuffleEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ShuffleEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetShuffleEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetShuffleEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CurrentItem(&self) -> ::windows_core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentItem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackItem>(result__)
        }
    }
    pub fn CurrentItemIndex(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentItemIndex)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn MoveNext(&self) -> ::windows_core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MoveNext)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackItem>(result__)
        }
    }
    pub fn MovePrevious(&self) -> ::windows_core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MovePrevious)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackItem>(result__)
        }
    }
    pub fn MoveTo(&self, itemindex: u32) -> ::windows_core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MoveTo)(::windows_core::Interface::as_raw(this), itemindex, result__.as_mut_ptr()).from_abi::<MediaPlaybackItem>(result__)
        }
    }
    pub fn MaxPrefetchTime(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackList2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MaxPrefetchTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetMaxPrefetchTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackList2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxPrefetchTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartingItem(&self) -> ::windows_core::Result<MediaPlaybackItem> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackList2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartingItem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackItem>(result__)
        }
    }
    pub fn SetStartingItem<'a, Param0: ::windows_core::IntoParam<'a, MediaPlaybackItem>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackList2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartingItem)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ShuffledItems(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<MediaPlaybackItem>> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackList2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShuffledItems)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<MediaPlaybackItem>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetShuffledItems<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<MediaPlaybackItem>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackList2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetShuffledItems)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn MaxPlayedItemsToKeepOpen(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u32>> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackList3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MaxPlayedItemsToKeepOpen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u32>>(result__)
        }
    }
    pub fn SetMaxPlayedItemsToKeepOpen<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<u32>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackList3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxPlayedItemsToKeepOpen)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for MediaPlaybackList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackList {}
impl ::core::fmt::Debug for MediaPlaybackList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackList").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlaybackList {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackList;{7f77ee9c-dc42-4e26-a98d-7850df8ec925})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaPlaybackList {
    type Vtable = IMediaPlaybackList_Vtbl;
    const IID: ::windows_core::GUID = <IMediaPlaybackList as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaPlaybackList {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackList";
}
impl ::core::convert::From<MediaPlaybackList> for ::windows_core::IUnknown {
    fn from(value: MediaPlaybackList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackList> for ::windows_core::IUnknown {
    fn from(value: &MediaPlaybackList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaPlaybackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaPlaybackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaPlaybackList> for ::windows_core::IInspectable {
    fn from(value: MediaPlaybackList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackList> for ::windows_core::IInspectable {
    fn from(value: &MediaPlaybackList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaPlaybackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaPlaybackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MediaPlaybackList> for IMediaPlaybackSource {
    type Error = ::windows_core::Error;
    fn try_from(value: MediaPlaybackList) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MediaPlaybackList> for IMediaPlaybackSource {
    type Error = ::windows_core::Error;
    fn try_from(value: &MediaPlaybackList) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaPlaybackSource> for MediaPlaybackList {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaPlaybackSource> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaPlaybackSource> for &MediaPlaybackList {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaPlaybackSource> {
        ::core::convert::TryInto::<IMediaPlaybackSource>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackList {}
unsafe impl ::core::marker::Sync for MediaPlaybackList {}
#[repr(transparent)]
pub struct MediaPlaybackSession(::windows_core::IUnknown);
impl MediaPlaybackSession {
    pub fn PlaybackStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackSession, ::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackStateChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePlaybackStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePlaybackStateChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn PlaybackRateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackSession, ::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackRateChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePlaybackRateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePlaybackRateChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn SeekCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackSession, ::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SeekCompleted)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSeekCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSeekCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn BufferingStarted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackSession, ::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).BufferingStarted)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveBufferingStarted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveBufferingStarted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn BufferingEnded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackSession, ::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).BufferingEnded)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveBufferingEnded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveBufferingEnded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn BufferingProgressChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackSession, ::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).BufferingProgressChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveBufferingProgressChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveBufferingProgressChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn DownloadProgressChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackSession, ::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DownloadProgressChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDownloadProgressChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDownloadProgressChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn NaturalDurationChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackSession, ::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).NaturalDurationChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveNaturalDurationChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveNaturalDurationChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn PositionChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackSession, ::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PositionChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePositionChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePositionChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn NaturalVideoSizeChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackSession, ::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).NaturalVideoSizeChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveNaturalVideoSizeChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveNaturalVideoSizeChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn MediaPlayer(&self) -> ::windows_core::Result<MediaPlayer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MediaPlayer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlayer>(result__)
        }
    }
    pub fn NaturalDuration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).NaturalDuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetPosition<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPosition)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn PlaybackState(&self) -> ::windows_core::Result<MediaPlaybackState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaPlaybackState>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackState>(result__)
        }
    }
    pub fn CanSeek(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanSeek)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CanPause(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanPause)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsProtected(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsProtected)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn PlaybackRate(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackRate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetPlaybackRate(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPlaybackRate)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BufferingProgress(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).BufferingProgress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn DownloadProgress(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DownloadProgress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn NaturalVideoHeight(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).NaturalVideoHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn NaturalVideoWidth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).NaturalVideoWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn NormalizedSourceRect(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).NormalizedSourceRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    pub fn SetNormalizedSourceRect<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNormalizedSourceRect)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn StereoscopicVideoPackingMode(&self) -> ::windows_core::Result<super::MediaProperties::StereoscopicVideoPackingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::MediaProperties::StereoscopicVideoPackingMode>::zeroed();
            (::windows_core::Interface::vtable(this).StereoscopicVideoPackingMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::StereoscopicVideoPackingMode>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn SetStereoscopicVideoPackingMode(&self, value: super::MediaProperties::StereoscopicVideoPackingMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStereoscopicVideoPackingMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BufferedRangesChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackSession, ::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).BufferedRangesChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveBufferedRangesChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveBufferedRangesChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn PlayedRangesChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackSession, ::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PlayedRangesChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePlayedRangesChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemovePlayedRangesChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn SeekableRangesChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackSession, ::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SeekableRangesChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSeekableRangesChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSeekableRangesChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn SupportedPlaybackRatesChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackSession, ::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedPlaybackRatesChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSupportedPlaybackRatesChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSupportedPlaybackRatesChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn SphericalVideoProjection(&self) -> ::windows_core::Result<MediaPlaybackSphericalVideoProjection> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SphericalVideoProjection)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackSphericalVideoProjection>(result__)
        }
    }
    pub fn IsMirroring(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsMirroring)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsMirroring(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsMirroring)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetBufferedRanges(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<super::MediaTimeRange>> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetBufferedRanges)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<super::MediaTimeRange>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetPlayedRanges(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<super::MediaTimeRange>> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPlayedRanges)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<super::MediaTimeRange>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetSeekableRanges(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<super::MediaTimeRange>> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSeekableRanges)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<super::MediaTimeRange>>(result__)
        }
    }
    pub fn IsSupportedPlaybackRateRange(&self, rate1: f64, rate2: f64) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSupportedPlaybackRateRange)(::windows_core::Interface::as_raw(this), rate1, rate2, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn PlaybackRotation(&self) -> ::windows_core::Result<super::MediaProperties::MediaRotation> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackSession3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::MediaProperties::MediaRotation>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackRotation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::MediaRotation>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn SetPlaybackRotation(&self, value: super::MediaProperties::MediaRotation) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackSession3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPlaybackRotation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetOutputDegradationPolicyState(&self) -> ::windows_core::Result<MediaPlaybackSessionOutputDegradationPolicyState> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackSession3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetOutputDegradationPolicyState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackSessionOutputDegradationPolicyState>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackSession {}
impl ::core::fmt::Debug for MediaPlaybackSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackSession").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlaybackSession {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackSession;{c32b683d-0407-41ba-8946-8b345a5a5435})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaPlaybackSession {
    type Vtable = IMediaPlaybackSession_Vtbl;
    const IID: ::windows_core::GUID = <IMediaPlaybackSession as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaPlaybackSession {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackSession";
}
impl ::core::convert::From<MediaPlaybackSession> for ::windows_core::IUnknown {
    fn from(value: MediaPlaybackSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackSession> for ::windows_core::IUnknown {
    fn from(value: &MediaPlaybackSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaPlaybackSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaPlaybackSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaPlaybackSession> for ::windows_core::IInspectable {
    fn from(value: MediaPlaybackSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackSession> for ::windows_core::IInspectable {
    fn from(value: &MediaPlaybackSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaPlaybackSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaPlaybackSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackSession {}
unsafe impl ::core::marker::Sync for MediaPlaybackSession {}
#[repr(transparent)]
pub struct MediaPlaybackSessionBufferingStartedEventArgs(::windows_core::IUnknown);
impl MediaPlaybackSessionBufferingStartedEventArgs {
    pub fn IsPlaybackInterruption(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPlaybackInterruption)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackSessionBufferingStartedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackSessionBufferingStartedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackSessionBufferingStartedEventArgs {}
impl ::core::fmt::Debug for MediaPlaybackSessionBufferingStartedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackSessionBufferingStartedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlaybackSessionBufferingStartedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackSessionBufferingStartedEventArgs;{cd6aafed-74e2-43b5-b115-76236c33791a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaPlaybackSessionBufferingStartedEventArgs {
    type Vtable = IMediaPlaybackSessionBufferingStartedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaPlaybackSessionBufferingStartedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaPlaybackSessionBufferingStartedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackSessionBufferingStartedEventArgs";
}
impl ::core::convert::From<MediaPlaybackSessionBufferingStartedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaPlaybackSessionBufferingStartedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackSessionBufferingStartedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaPlaybackSessionBufferingStartedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaPlaybackSessionBufferingStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaPlaybackSessionBufferingStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaPlaybackSessionBufferingStartedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaPlaybackSessionBufferingStartedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackSessionBufferingStartedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaPlaybackSessionBufferingStartedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaPlaybackSessionBufferingStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaPlaybackSessionBufferingStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackSessionBufferingStartedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackSessionBufferingStartedEventArgs {}
#[repr(transparent)]
pub struct MediaPlaybackSessionOutputDegradationPolicyState(::windows_core::IUnknown);
impl MediaPlaybackSessionOutputDegradationPolicyState {
    pub fn VideoConstrictionReason(&self) -> ::windows_core::Result<MediaPlaybackSessionVideoConstrictionReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaPlaybackSessionVideoConstrictionReason>::zeroed();
            (::windows_core::Interface::vtable(this).VideoConstrictionReason)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackSessionVideoConstrictionReason>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackSessionOutputDegradationPolicyState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackSessionOutputDegradationPolicyState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackSessionOutputDegradationPolicyState {}
impl ::core::fmt::Debug for MediaPlaybackSessionOutputDegradationPolicyState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackSessionOutputDegradationPolicyState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlaybackSessionOutputDegradationPolicyState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackSessionOutputDegradationPolicyState;{558e727d-f633-49f9-965a-abaa1db709be})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaPlaybackSessionOutputDegradationPolicyState {
    type Vtable = IMediaPlaybackSessionOutputDegradationPolicyState_Vtbl;
    const IID: ::windows_core::GUID = <IMediaPlaybackSessionOutputDegradationPolicyState as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaPlaybackSessionOutputDegradationPolicyState {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackSessionOutputDegradationPolicyState";
}
impl ::core::convert::From<MediaPlaybackSessionOutputDegradationPolicyState> for ::windows_core::IUnknown {
    fn from(value: MediaPlaybackSessionOutputDegradationPolicyState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackSessionOutputDegradationPolicyState> for ::windows_core::IUnknown {
    fn from(value: &MediaPlaybackSessionOutputDegradationPolicyState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaPlaybackSessionOutputDegradationPolicyState {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaPlaybackSessionOutputDegradationPolicyState {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaPlaybackSessionOutputDegradationPolicyState> for ::windows_core::IInspectable {
    fn from(value: MediaPlaybackSessionOutputDegradationPolicyState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackSessionOutputDegradationPolicyState> for ::windows_core::IInspectable {
    fn from(value: &MediaPlaybackSessionOutputDegradationPolicyState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaPlaybackSessionOutputDegradationPolicyState {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaPlaybackSessionOutputDegradationPolicyState {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackSessionOutputDegradationPolicyState {}
unsafe impl ::core::marker::Sync for MediaPlaybackSessionOutputDegradationPolicyState {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaPlaybackSessionVideoConstrictionReason(pub i32);
impl MediaPlaybackSessionVideoConstrictionReason {
    pub const None: Self = Self(0i32);
    pub const VirtualMachine: Self = Self(1i32);
    pub const UnsupportedDisplayAdapter: Self = Self(2i32);
    pub const UnsignedDriver: Self = Self(3i32);
    pub const FrameServerEnabled: Self = Self(4i32);
    pub const OutputProtectionFailed: Self = Self(5i32);
    pub const Unknown: Self = Self(6i32);
}
impl ::core::marker::Copy for MediaPlaybackSessionVideoConstrictionReason {}
impl ::core::clone::Clone for MediaPlaybackSessionVideoConstrictionReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaPlaybackSessionVideoConstrictionReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaPlaybackSessionVideoConstrictionReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaPlaybackSessionVideoConstrictionReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackSessionVideoConstrictionReason").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlaybackSessionVideoConstrictionReason {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlaybackSessionVideoConstrictionReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MediaPlaybackSphericalVideoProjection(::windows_core::IUnknown);
impl MediaPlaybackSphericalVideoProjection {
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn FrameFormat(&self) -> ::windows_core::Result<super::MediaProperties::SphericalVideoFrameFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::MediaProperties::SphericalVideoFrameFormat>::zeroed();
            (::windows_core::Interface::vtable(this).FrameFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::SphericalVideoFrameFormat>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn SetFrameFormat(&self, value: super::MediaProperties::SphericalVideoFrameFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFrameFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HorizontalFieldOfViewInDegrees(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).HorizontalFieldOfViewInDegrees)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetHorizontalFieldOfViewInDegrees(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHorizontalFieldOfViewInDegrees)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ViewOrientation(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Quaternion>::zeroed();
            (::windows_core::Interface::vtable(this).ViewOrientation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Quaternion>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetViewOrientation<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Quaternion>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetViewOrientation)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ProjectionMode(&self) -> ::windows_core::Result<SphericalVideoProjectionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SphericalVideoProjectionMode>::zeroed();
            (::windows_core::Interface::vtable(this).ProjectionMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SphericalVideoProjectionMode>(result__)
        }
    }
    pub fn SetProjectionMode(&self, value: SphericalVideoProjectionMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetProjectionMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for MediaPlaybackSphericalVideoProjection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackSphericalVideoProjection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackSphericalVideoProjection {}
impl ::core::fmt::Debug for MediaPlaybackSphericalVideoProjection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackSphericalVideoProjection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlaybackSphericalVideoProjection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackSphericalVideoProjection;{d405b37c-6f0e-4661-b8ee-d487ba9752d5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaPlaybackSphericalVideoProjection {
    type Vtable = IMediaPlaybackSphericalVideoProjection_Vtbl;
    const IID: ::windows_core::GUID = <IMediaPlaybackSphericalVideoProjection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaPlaybackSphericalVideoProjection {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackSphericalVideoProjection";
}
impl ::core::convert::From<MediaPlaybackSphericalVideoProjection> for ::windows_core::IUnknown {
    fn from(value: MediaPlaybackSphericalVideoProjection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackSphericalVideoProjection> for ::windows_core::IUnknown {
    fn from(value: &MediaPlaybackSphericalVideoProjection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaPlaybackSphericalVideoProjection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaPlaybackSphericalVideoProjection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaPlaybackSphericalVideoProjection> for ::windows_core::IInspectable {
    fn from(value: MediaPlaybackSphericalVideoProjection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlaybackSphericalVideoProjection> for ::windows_core::IInspectable {
    fn from(value: &MediaPlaybackSphericalVideoProjection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaPlaybackSphericalVideoProjection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaPlaybackSphericalVideoProjection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackSphericalVideoProjection {}
unsafe impl ::core::marker::Sync for MediaPlaybackSphericalVideoProjection {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaPlaybackState(pub i32);
impl MediaPlaybackState {
    pub const None: Self = Self(0i32);
    pub const Opening: Self = Self(1i32);
    pub const Buffering: Self = Self(2i32);
    pub const Playing: Self = Self(3i32);
    pub const Paused: Self = Self(4i32);
}
impl ::core::marker::Copy for MediaPlaybackState {}
impl ::core::clone::Clone for MediaPlaybackState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaPlaybackState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaPlaybackState {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaPlaybackState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlaybackState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlaybackState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct MediaPlaybackTimedMetadataTrackList(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl MediaPlaybackTimedMetadataTrackList {
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<super::Core::TimedMetadataTrack>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<super::Core::TimedMetadataTrack>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<super::Core::TimedMetadataTrack>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn PresentationModeChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlaybackTimedMetadataTrackList, TimedMetadataPresentationModeChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackTimedMetadataTrackList>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PresentationModeChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePresentationModeChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackTimedMetadataTrackList>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemovePresentationModeChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetPresentationMode(&self, index: u32) -> ::windows_core::Result<TimedMetadataTrackPresentationMode> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackTimedMetadataTrackList>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TimedMetadataTrackPresentationMode>::zeroed();
            (::windows_core::Interface::vtable(this).GetPresentationMode)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<TimedMetadataTrackPresentationMode>(result__)
        }
    }
    pub fn SetPresentationMode(&self, index: u32, value: TimedMetadataTrackPresentationMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlaybackTimedMetadataTrackList>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPresentationMode)(::windows_core::Interface::as_raw(this), index, value).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<super::Core::TimedMetadataTrack> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<super::Core::TimedMetadataTrack>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, super::Core::TimedMetadataTrack>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<super::Core::TimedMetadataTrack>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for MediaPlaybackTimedMetadataTrackList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for MediaPlaybackTimedMetadataTrackList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for MediaPlaybackTimedMetadataTrackList {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for MediaPlaybackTimedMetadataTrackList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackTimedMetadataTrackList").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for MediaPlaybackTimedMetadataTrackList {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackTimedMetadataTrackList;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.Media.Core.TimedMetadataTrack;{9e6aed9e-f67a-49a9-b330-cf03b0e9cf07})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for MediaPlaybackTimedMetadataTrackList {
    type Vtable = ::winrt_foundation::Collections::IVectorView_Vtbl<super::Core::TimedMetadataTrack>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IVectorView<super::Core::TimedMetadataTrack> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for MediaPlaybackTimedMetadataTrackList {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackTimedMetadataTrackList";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for MediaPlaybackTimedMetadataTrackList {
    type Item = super::Core::TimedMetadataTrack;
    type IntoIter = ::winrt_foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &MediaPlaybackTimedMetadataTrackList {
    type Item = super::Core::TimedMetadataTrack;
    type IntoIter = ::winrt_foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<MediaPlaybackTimedMetadataTrackList> for ::windows_core::IUnknown {
    fn from(value: MediaPlaybackTimedMetadataTrackList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&MediaPlaybackTimedMetadataTrackList> for ::windows_core::IUnknown {
    fn from(value: &MediaPlaybackTimedMetadataTrackList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaPlaybackTimedMetadataTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaPlaybackTimedMetadataTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<MediaPlaybackTimedMetadataTrackList> for ::windows_core::IInspectable {
    fn from(value: MediaPlaybackTimedMetadataTrackList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&MediaPlaybackTimedMetadataTrackList> for ::windows_core::IInspectable {
    fn from(value: &MediaPlaybackTimedMetadataTrackList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaPlaybackTimedMetadataTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaPlaybackTimedMetadataTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl ::core::convert::TryFrom<MediaPlaybackTimedMetadataTrackList> for ::winrt_foundation::Collections::IIterable<super::Core::TimedMetadataTrack> {
    type Error = ::windows_core::Error;
    fn try_from(value: MediaPlaybackTimedMetadataTrackList) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl ::core::convert::TryFrom<&MediaPlaybackTimedMetadataTrackList> for ::winrt_foundation::Collections::IIterable<super::Core::TimedMetadataTrack> {
    type Error = ::windows_core::Error;
    fn try_from(value: &MediaPlaybackTimedMetadataTrackList) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<super::Core::TimedMetadataTrack>> for MediaPlaybackTimedMetadataTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<super::Core::TimedMetadataTrack>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<super::Core::TimedMetadataTrack>> for &MediaPlaybackTimedMetadataTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<super::Core::TimedMetadataTrack>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<super::Core::TimedMetadataTrack>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl ::core::convert::TryFrom<MediaPlaybackTimedMetadataTrackList> for ::winrt_foundation::Collections::IVectorView<super::Core::TimedMetadataTrack> {
    type Error = ::windows_core::Error;
    fn try_from(value: MediaPlaybackTimedMetadataTrackList) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl ::core::convert::TryFrom<&MediaPlaybackTimedMetadataTrackList> for ::winrt_foundation::Collections::IVectorView<super::Core::TimedMetadataTrack> {
    type Error = ::windows_core::Error;
    fn try_from(value: &MediaPlaybackTimedMetadataTrackList) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<super::Core::TimedMetadataTrack>> for MediaPlaybackTimedMetadataTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVectorView<super::Core::TimedMetadataTrack>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<super::Core::TimedMetadataTrack>> for &MediaPlaybackTimedMetadataTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVectorView<super::Core::TimedMetadataTrack>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVectorView<super::Core::TimedMetadataTrack>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Send for MediaPlaybackTimedMetadataTrackList {}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Sync for MediaPlaybackTimedMetadataTrackList {}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct MediaPlaybackVideoTrackList(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl MediaPlaybackVideoTrackList {
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<super::Core::VideoTrack>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<super::Core::VideoTrack>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<super::Core::VideoTrack>>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn SelectedIndexChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<super::Core::ISingleSelectMediaTrackList, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedIndexChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn RemoveSelectedIndexChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSelectedIndexChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn SetSelectedIndex(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedIndex)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn SelectedIndex(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedIndex)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<super::Core::VideoTrack> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<super::Core::VideoTrack>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, super::Core::VideoTrack>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<super::Core::VideoTrack>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for MediaPlaybackVideoTrackList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for MediaPlaybackVideoTrackList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for MediaPlaybackVideoTrackList {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for MediaPlaybackVideoTrackList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackVideoTrackList").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for MediaPlaybackVideoTrackList {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackVideoTrackList;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.Media.Core.VideoTrack;{03e1fafc-c931-491a-b46b-c10ee8c256b7})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for MediaPlaybackVideoTrackList {
    type Vtable = ::winrt_foundation::Collections::IVectorView_Vtbl<super::Core::VideoTrack>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IVectorView<super::Core::VideoTrack> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for MediaPlaybackVideoTrackList {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackVideoTrackList";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for MediaPlaybackVideoTrackList {
    type Item = super::Core::VideoTrack;
    type IntoIter = ::winrt_foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &MediaPlaybackVideoTrackList {
    type Item = super::Core::VideoTrack;
    type IntoIter = ::winrt_foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<MediaPlaybackVideoTrackList> for ::windows_core::IUnknown {
    fn from(value: MediaPlaybackVideoTrackList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&MediaPlaybackVideoTrackList> for ::windows_core::IUnknown {
    fn from(value: &MediaPlaybackVideoTrackList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaPlaybackVideoTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaPlaybackVideoTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<MediaPlaybackVideoTrackList> for ::windows_core::IInspectable {
    fn from(value: MediaPlaybackVideoTrackList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&MediaPlaybackVideoTrackList> for ::windows_core::IInspectable {
    fn from(value: &MediaPlaybackVideoTrackList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaPlaybackVideoTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaPlaybackVideoTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl ::core::convert::TryFrom<MediaPlaybackVideoTrackList> for ::winrt_foundation::Collections::IIterable<super::Core::VideoTrack> {
    type Error = ::windows_core::Error;
    fn try_from(value: MediaPlaybackVideoTrackList) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl ::core::convert::TryFrom<&MediaPlaybackVideoTrackList> for ::winrt_foundation::Collections::IIterable<super::Core::VideoTrack> {
    type Error = ::windows_core::Error;
    fn try_from(value: &MediaPlaybackVideoTrackList) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<super::Core::VideoTrack>> for MediaPlaybackVideoTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<super::Core::VideoTrack>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<super::Core::VideoTrack>> for &MediaPlaybackVideoTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<super::Core::VideoTrack>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<super::Core::VideoTrack>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl ::core::convert::TryFrom<MediaPlaybackVideoTrackList> for super::Core::ISingleSelectMediaTrackList {
    type Error = ::windows_core::Error;
    fn try_from(value: MediaPlaybackVideoTrackList) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl ::core::convert::TryFrom<&MediaPlaybackVideoTrackList> for super::Core::ISingleSelectMediaTrackList {
    type Error = ::windows_core::Error;
    fn try_from(value: &MediaPlaybackVideoTrackList) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl<'a> ::windows_core::IntoParam<'a, super::Core::ISingleSelectMediaTrackList> for MediaPlaybackVideoTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, super::Core::ISingleSelectMediaTrackList> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl<'a> ::windows_core::IntoParam<'a, super::Core::ISingleSelectMediaTrackList> for &MediaPlaybackVideoTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, super::Core::ISingleSelectMediaTrackList> {
        ::core::convert::TryInto::<super::Core::ISingleSelectMediaTrackList>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl ::core::convert::TryFrom<MediaPlaybackVideoTrackList> for ::winrt_foundation::Collections::IVectorView<super::Core::VideoTrack> {
    type Error = ::windows_core::Error;
    fn try_from(value: MediaPlaybackVideoTrackList) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl ::core::convert::TryFrom<&MediaPlaybackVideoTrackList> for ::winrt_foundation::Collections::IVectorView<super::Core::VideoTrack> {
    type Error = ::windows_core::Error;
    fn try_from(value: &MediaPlaybackVideoTrackList) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<super::Core::VideoTrack>> for MediaPlaybackVideoTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVectorView<super::Core::VideoTrack>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<super::Core::VideoTrack>> for &MediaPlaybackVideoTrackList {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVectorView<super::Core::VideoTrack>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVectorView<super::Core::VideoTrack>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Send for MediaPlaybackVideoTrackList {}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Sync for MediaPlaybackVideoTrackList {}
#[repr(transparent)]
pub struct MediaPlayer(::windows_core::IUnknown);
impl MediaPlayer {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MediaPlayer, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn AutoPlay(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AutoPlay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAutoPlay(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAutoPlay)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn NaturalDuration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).NaturalDuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetPosition<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPosition)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn BufferingProgress(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).BufferingProgress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn CurrentState(&self) -> ::windows_core::Result<MediaPlayerState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaPlayerState>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlayerState>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn CanSeek(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanSeek)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn CanPause(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanPause)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsLoopingEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsLoopingEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsLoopingEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsLoopingEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn IsProtected(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsProtected)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsMuted(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsMuted)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsMuted(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsMuted)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn PlaybackRate(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackRate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetPlaybackRate(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPlaybackRate)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Volume(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Volume)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetVolume(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVolume)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn PlaybackMediaMarkers(&self) -> ::windows_core::Result<PlaybackMediaMarkerSequence> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackMediaMarkers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PlaybackMediaMarkerSequence>(result__)
        }
    }
    pub fn MediaOpened<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlayer, ::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).MediaOpened)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveMediaOpened<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMediaOpened)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn MediaEnded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlayer, ::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).MediaEnded)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveMediaEnded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMediaEnded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn MediaFailed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlayer, MediaPlayerFailedEventArgs>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).MediaFailed)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveMediaFailed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMediaFailed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn CurrentStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlayer, ::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentStateChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveCurrentStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCurrentStateChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn PlaybackMediaMarkerReached<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlayer, PlaybackMediaMarkerReachedEventArgs>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackMediaMarkerReached)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemovePlaybackMediaMarkerReached<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePlaybackMediaMarkerReached)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn MediaPlayerRateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlayer, MediaPlayerRateChangedEventArgs>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).MediaPlayerRateChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveMediaPlayerRateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMediaPlayerRateChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn VolumeChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlayer, ::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).VolumeChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveVolumeChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveVolumeChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn SeekCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlayer, ::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SeekCompleted)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveSeekCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSeekCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn BufferingStarted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlayer, ::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).BufferingStarted)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveBufferingStarted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveBufferingStarted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn BufferingEnded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlayer, ::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).BufferingEnded)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveBufferingEnded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveBufferingEnded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Play(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Play)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Pause(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Pause)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetUriSource<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUriSource)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SystemMediaTransportControls(&self) -> ::windows_core::Result<super::SystemMediaTransportControls> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SystemMediaTransportControls)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::SystemMediaTransportControls>(result__)
        }
    }
    pub fn AudioCategory(&self) -> ::windows_core::Result<MediaPlayerAudioCategory> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaPlayerAudioCategory>::zeroed();
            (::windows_core::Interface::vtable(this).AudioCategory)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlayerAudioCategory>(result__)
        }
    }
    pub fn SetAudioCategory(&self, value: MediaPlayerAudioCategory) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAudioCategory)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AudioDeviceType(&self) -> ::windows_core::Result<MediaPlayerAudioDeviceType> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaPlayerAudioDeviceType>::zeroed();
            (::windows_core::Interface::vtable(this).AudioDeviceType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlayerAudioDeviceType>(result__)
        }
    }
    pub fn SetAudioDeviceType(&self, value: MediaPlayerAudioDeviceType) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAudioDeviceType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsMutedChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlayer, ::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).IsMutedChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveIsMutedChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveIsMutedChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn SourceChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlayer, ::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SourceChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSourceChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSourceChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn AudioBalance(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).AudioBalance)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetAudioBalance(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAudioBalance)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RealTimePlayback(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).RealTimePlayback)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetRealTimePlayback(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRealTimePlayback)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StereoscopicVideoRenderMode(&self) -> ::windows_core::Result<StereoscopicVideoRenderMode> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<StereoscopicVideoRenderMode>::zeroed();
            (::windows_core::Interface::vtable(this).StereoscopicVideoRenderMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StereoscopicVideoRenderMode>(result__)
        }
    }
    pub fn SetStereoscopicVideoRenderMode(&self, value: StereoscopicVideoRenderMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStereoscopicVideoRenderMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BreakManager(&self) -> ::windows_core::Result<MediaBreakManager> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BreakManager)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaBreakManager>(result__)
        }
    }
    pub fn CommandManager(&self) -> ::windows_core::Result<MediaPlaybackCommandManager> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CommandManager)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackCommandManager>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn AudioDevice(&self) -> ::windows_core::Result<::winrt_devices::Enumeration::DeviceInformation> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AudioDevice)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Enumeration::DeviceInformation>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn SetAudioDevice<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_devices::Enumeration::DeviceInformation>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAudioDevice)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn TimelineController(&self) -> ::windows_core::Result<super::MediaTimelineController> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TimelineController)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaTimelineController>(result__)
        }
    }
    pub fn SetTimelineController<'a, Param0: ::windows_core::IntoParam<'a, super::MediaTimelineController>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTimelineController)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn TimelineControllerPositionOffset(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).TimelineControllerPositionOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetTimelineControllerPositionOffset<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTimelineControllerPositionOffset)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn PlaybackSession(&self) -> ::windows_core::Result<MediaPlaybackSession> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackSession)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackSession>(result__)
        }
    }
    pub fn StepForwardOneFrame(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StepForwardOneFrame)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn StepBackwardOneFrame(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StepBackwardOneFrame)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn GetAsCastingSource(&self) -> ::windows_core::Result<super::Casting::CastingSource> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAsCastingSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Casting::CastingSource>(result__)
        }
    }
    pub fn SetSurfaceSize<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Size>>(&self, size: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSurfaceSize)(::windows_core::Interface::as_raw(this), size.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn GetSurface<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_ui::Composition::Compositor>>(&self, compositor: Param0) -> ::windows_core::Result<MediaPlayerSurface> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSurface)(::windows_core::Interface::as_raw(this), compositor.into_param().abi(), result__.as_mut_ptr()).from_abi::<MediaPlayerSurface>(result__)
        }
    }
    pub fn VideoFrameAvailable<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlayer, ::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFrameAvailable)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveVideoFrameAvailable<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveVideoFrameAvailable)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn IsVideoFrameServerEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsVideoFrameServerEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsVideoFrameServerEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsVideoFrameServerEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn CopyFrameToVideoSurface<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::DirectX::Direct3D11::IDirect3DSurface>>(&self, destination: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).CopyFrameToVideoSurface)(::windows_core::Interface::as_raw(this), destination.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn CopyFrameToVideoSurfaceWithTargetRectangle<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::DirectX::Direct3D11::IDirect3DSurface>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, destination: Param0, targetrectangle: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).CopyFrameToVideoSurfaceWithTargetRectangle)(::windows_core::Interface::as_raw(this), destination.into_param().abi(), targetrectangle.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn CopyFrameToStereoscopicVideoSurfaces<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::DirectX::Direct3D11::IDirect3DSurface>, Param1: ::windows_core::IntoParam<'a, ::winrt_graphics::DirectX::Direct3D11::IDirect3DSurface>>(&self, destinationlefteye: Param0, destinationrighteye: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).CopyFrameToStereoscopicVideoSurfaces)(::windows_core::Interface::as_raw(this), destinationlefteye.into_param().abi(), destinationrighteye.into_param().abi()).ok() }
    }
    pub fn SubtitleFrameChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaPlayer, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SubtitleFrameChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSubtitleFrameChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer6>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSubtitleFrameChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn RenderSubtitlesToSurface<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::DirectX::Direct3D11::IDirect3DSurface>>(&self, destination: Param0) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).RenderSubtitlesToSurface)(::windows_core::Interface::as_raw(this), destination.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn RenderSubtitlesToSurfaceWithTargetRectangle<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::DirectX::Direct3D11::IDirect3DSurface>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, destination: Param0, targetrectangle: Param1) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).RenderSubtitlesToSurfaceWithTargetRectangle)(::windows_core::Interface::as_raw(this), destination.into_param().abi(), targetrectangle.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn AudioStateMonitor(&self) -> ::windows_core::Result<super::Audio::AudioStateMonitor> {
        let this = &::windows_core::Interface::cast::<IMediaPlayer7>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AudioStateMonitor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Audio::AudioStateMonitor>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn AddAudioEffect<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IPropertySet>>(&self, activatableclassid: Param0, effectoptional: bool, configuration: Param2) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlayerEffects>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AddAudioEffect)(::windows_core::Interface::as_raw(this), activatableclassid.into_param().abi(), effectoptional, configuration.into_param().abi()).ok() }
    }
    pub fn RemoveAllEffects(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlayerEffects>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAllEffects)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn AddVideoEffect<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IPropertySet>>(&self, activatableclassid: Param0, effectoptional: bool, effectconfiguration: Param2) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlayerEffects2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AddVideoEffect)(::windows_core::Interface::as_raw(this), activatableclassid.into_param().abi(), effectoptional, effectconfiguration.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn ProtectionManager(&self) -> ::windows_core::Result<super::Protection::MediaProtectionManager> {
        let this = &::windows_core::Interface::cast::<IMediaPlayerSource>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionManager)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Protection::MediaProtectionManager>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn SetProtectionManager<'a, Param0: ::windows_core::IntoParam<'a, super::Protection::MediaProtectionManager>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlayerSource>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetProtectionManager)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "winrt-storage", feature = "winrt-"))]
    pub fn SetFileSource<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFile>>(&self, file: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlayerSource>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetFileSource)(::windows_core::Interface::as_raw(this), file.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "winrt-storage", feature = "winrt-"))]
    pub fn SetStreamSource<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>>(&self, stream: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlayerSource>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStreamSource)(::windows_core::Interface::as_raw(this), stream.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "winrt-media", feature = "winrt-"))]
    pub fn SetMediaSource<'a, Param0: ::windows_core::IntoParam<'a, super::Core::IMediaSource>>(&self, source: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlayerSource>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMediaSource)(::windows_core::Interface::as_raw(this), source.into_param().abi()).ok() }
    }
    pub fn Source(&self) -> ::windows_core::Result<IMediaPlaybackSource> {
        let this = &::windows_core::Interface::cast::<IMediaPlayerSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Source)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IMediaPlaybackSource>(result__)
        }
    }
    pub fn SetSource<'a, Param0: ::windows_core::IntoParam<'a, IMediaPlaybackSource>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaPlayerSource2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSource)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for MediaPlayer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlayer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlayer {}
impl ::core::fmt::Debug for MediaPlayer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlayer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlayer {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlayer;{381a83cb-6fff-499b-8d64-2885dfc1249e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaPlayer {
    type Vtable = IMediaPlayer_Vtbl;
    const IID: ::windows_core::GUID = <IMediaPlayer as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaPlayer {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlayer";
}
impl ::core::convert::From<MediaPlayer> for ::windows_core::IUnknown {
    fn from(value: MediaPlayer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlayer> for ::windows_core::IUnknown {
    fn from(value: &MediaPlayer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaPlayer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaPlayer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaPlayer> for ::windows_core::IInspectable {
    fn from(value: MediaPlayer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlayer> for ::windows_core::IInspectable {
    fn from(value: &MediaPlayer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaPlayer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaPlayer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MediaPlayer> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: MediaPlayer) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MediaPlayer> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &MediaPlayer) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for MediaPlayer {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &MediaPlayer {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MediaPlayer {}
unsafe impl ::core::marker::Sync for MediaPlayer {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaPlayerAudioCategory(pub i32);
impl MediaPlayerAudioCategory {
    pub const Other: Self = Self(0i32);
    pub const Communications: Self = Self(3i32);
    pub const Alerts: Self = Self(4i32);
    pub const SoundEffects: Self = Self(5i32);
    pub const GameEffects: Self = Self(6i32);
    pub const GameMedia: Self = Self(7i32);
    pub const GameChat: Self = Self(8i32);
    pub const Speech: Self = Self(9i32);
    pub const Movie: Self = Self(10i32);
    pub const Media: Self = Self(11i32);
}
impl ::core::marker::Copy for MediaPlayerAudioCategory {}
impl ::core::clone::Clone for MediaPlayerAudioCategory {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaPlayerAudioCategory {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaPlayerAudioCategory {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaPlayerAudioCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlayerAudioCategory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlayerAudioCategory {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlayerAudioCategory;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaPlayerAudioDeviceType(pub i32);
impl MediaPlayerAudioDeviceType {
    pub const Console: Self = Self(0i32);
    pub const Multimedia: Self = Self(1i32);
    pub const Communications: Self = Self(2i32);
}
impl ::core::marker::Copy for MediaPlayerAudioDeviceType {}
impl ::core::clone::Clone for MediaPlayerAudioDeviceType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaPlayerAudioDeviceType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaPlayerAudioDeviceType {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaPlayerAudioDeviceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlayerAudioDeviceType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlayerAudioDeviceType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlayerAudioDeviceType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MediaPlayerDataReceivedEventArgs(::windows_core::IUnknown);
impl MediaPlayerDataReceivedEventArgs {
    #[cfg(feature = "winrt-foundation")]
    pub fn Data(&self) -> ::windows_core::Result<::winrt_foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::ValueSet>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlayerDataReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlayerDataReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlayerDataReceivedEventArgs {}
impl ::core::fmt::Debug for MediaPlayerDataReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlayerDataReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlayerDataReceivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlayerDataReceivedEventArgs;{c75a9405-c801-412a-835b-83fc0e622a8e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaPlayerDataReceivedEventArgs {
    type Vtable = IMediaPlayerDataReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaPlayerDataReceivedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaPlayerDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlayerDataReceivedEventArgs";
}
impl ::core::convert::From<MediaPlayerDataReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaPlayerDataReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlayerDataReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaPlayerDataReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaPlayerDataReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaPlayerDataReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaPlayerDataReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaPlayerDataReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlayerDataReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaPlayerDataReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaPlayerDataReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaPlayerDataReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaPlayerDataReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlayerDataReceivedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaPlayerError(pub i32);
impl MediaPlayerError {
    pub const Unknown: Self = Self(0i32);
    pub const Aborted: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const DecodingError: Self = Self(3i32);
    pub const SourceNotSupported: Self = Self(4i32);
}
impl ::core::marker::Copy for MediaPlayerError {}
impl ::core::clone::Clone for MediaPlayerError {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaPlayerError {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaPlayerError {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaPlayerError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlayerError").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlayerError {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlayerError;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MediaPlayerFailedEventArgs(::windows_core::IUnknown);
impl MediaPlayerFailedEventArgs {
    pub fn Error(&self) -> ::windows_core::Result<MediaPlayerError> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaPlayerError>::zeroed();
            (::windows_core::Interface::vtable(this).Error)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlayerError>(result__)
        }
    }
    pub fn ExtendedErrorCode(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn ErrorMessage(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlayerFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlayerFailedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlayerFailedEventArgs {}
impl ::core::fmt::Debug for MediaPlayerFailedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlayerFailedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlayerFailedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlayerFailedEventArgs;{2744e9b9-a7e3-4f16-bac4-7914ebc08301})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaPlayerFailedEventArgs {
    type Vtable = IMediaPlayerFailedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaPlayerFailedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaPlayerFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlayerFailedEventArgs";
}
impl ::core::convert::From<MediaPlayerFailedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaPlayerFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlayerFailedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaPlayerFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaPlayerFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaPlayerFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaPlayerFailedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaPlayerFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlayerFailedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaPlayerFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaPlayerFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaPlayerFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaPlayerFailedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlayerFailedEventArgs {}
#[repr(transparent)]
pub struct MediaPlayerRateChangedEventArgs(::windows_core::IUnknown);
impl MediaPlayerRateChangedEventArgs {
    pub fn NewRate(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).NewRate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlayerRateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlayerRateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlayerRateChangedEventArgs {}
impl ::core::fmt::Debug for MediaPlayerRateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlayerRateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlayerRateChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlayerRateChangedEventArgs;{40600d58-3b61-4bb2-989f-fc65608b6cab})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaPlayerRateChangedEventArgs {
    type Vtable = IMediaPlayerRateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaPlayerRateChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaPlayerRateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlayerRateChangedEventArgs";
}
impl ::core::convert::From<MediaPlayerRateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaPlayerRateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlayerRateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaPlayerRateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaPlayerRateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaPlayerRateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaPlayerRateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaPlayerRateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlayerRateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaPlayerRateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaPlayerRateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaPlayerRateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaPlayerRateChangedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlayerRateChangedEventArgs {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaPlayerState(pub i32);
#[cfg(feature = "winrt-")]
impl MediaPlayerState {
    pub const Closed: Self = Self(0i32);
    pub const Opening: Self = Self(1i32);
    pub const Buffering: Self = Self(2i32);
    pub const Playing: Self = Self(3i32);
    pub const Paused: Self = Self(4i32);
    pub const Stopped: Self = Self(5i32);
}
#[cfg(feature = "winrt-")]
impl ::core::marker::Copy for MediaPlayerState {}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for MediaPlayerState {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "winrt-")]
impl ::core::default::Default for MediaPlayerState {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Abi for MediaPlayerState {
    type Abi = Self;
}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for MediaPlayerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlayerState").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for MediaPlayerState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlayerState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MediaPlayerSurface(::windows_core::IUnknown);
impl MediaPlayerSurface {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn CompositionSurface(&self) -> ::windows_core::Result<::winrt_ui::Composition::ICompositionSurface> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CompositionSurface)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Composition::ICompositionSurface>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Compositor(&self) -> ::windows_core::Result<::winrt_ui::Composition::Compositor> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Composition::Compositor>(result__)
        }
    }
    pub fn MediaPlayer(&self) -> ::windows_core::Result<MediaPlayer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MediaPlayer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlayer>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlayerSurface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlayerSurface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlayerSurface {}
impl ::core::fmt::Debug for MediaPlayerSurface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlayerSurface").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlayerSurface {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlayerSurface;{0ed653bc-b736-49c3-830b-764a3845313a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaPlayerSurface {
    type Vtable = IMediaPlayerSurface_Vtbl;
    const IID: ::windows_core::GUID = <IMediaPlayerSurface as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaPlayerSurface {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlayerSurface";
}
impl ::core::convert::From<MediaPlayerSurface> for ::windows_core::IUnknown {
    fn from(value: MediaPlayerSurface) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlayerSurface> for ::windows_core::IUnknown {
    fn from(value: &MediaPlayerSurface) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaPlayerSurface {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaPlayerSurface {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaPlayerSurface> for ::windows_core::IInspectable {
    fn from(value: MediaPlayerSurface) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaPlayerSurface> for ::windows_core::IInspectable {
    fn from(value: &MediaPlayerSurface) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaPlayerSurface {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaPlayerSurface {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MediaPlayerSurface> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: MediaPlayerSurface) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MediaPlayerSurface> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &MediaPlayerSurface) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for MediaPlayerSurface {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &MediaPlayerSurface {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MediaPlayerSurface {}
unsafe impl ::core::marker::Sync for MediaPlayerSurface {}
#[repr(transparent)]
pub struct PlaybackMediaMarker(::windows_core::IUnknown);
impl PlaybackMediaMarker {
    pub fn Time(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Time)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn MediaMarkerType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).MediaMarkerType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CreateFromTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(value: Param0) -> ::windows_core::Result<PlaybackMediaMarker> {
        Self::IPlaybackMediaMarkerFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromTime)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<PlaybackMediaMarker>(result__)
        })
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(value: Param0, mediamarkettype: Param1, text: Param2) -> ::windows_core::Result<PlaybackMediaMarker> {
        Self::IPlaybackMediaMarkerFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), value.into_param().abi(), mediamarkettype.into_param().abi(), text.into_param().abi(), result__.as_mut_ptr()).from_abi::<PlaybackMediaMarker>(result__)
        })
    }
    pub fn IPlaybackMediaMarkerFactory<R, F: FnOnce(&IPlaybackMediaMarkerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PlaybackMediaMarker, IPlaybackMediaMarkerFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PlaybackMediaMarker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlaybackMediaMarker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlaybackMediaMarker {}
impl ::core::fmt::Debug for PlaybackMediaMarker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaybackMediaMarker").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlaybackMediaMarker {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.PlaybackMediaMarker;{c4d22f5c-3c1c-4444-b6b9-778b0422d41a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PlaybackMediaMarker {
    type Vtable = IPlaybackMediaMarker_Vtbl;
    const IID: ::windows_core::GUID = <IPlaybackMediaMarker as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PlaybackMediaMarker {
    const NAME: &'static str = "Windows.Media.Playback.PlaybackMediaMarker";
}
impl ::core::convert::From<PlaybackMediaMarker> for ::windows_core::IUnknown {
    fn from(value: PlaybackMediaMarker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlaybackMediaMarker> for ::windows_core::IUnknown {
    fn from(value: &PlaybackMediaMarker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PlaybackMediaMarker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PlaybackMediaMarker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlaybackMediaMarker> for ::windows_core::IInspectable {
    fn from(value: PlaybackMediaMarker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlaybackMediaMarker> for ::windows_core::IInspectable {
    fn from(value: &PlaybackMediaMarker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PlaybackMediaMarker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PlaybackMediaMarker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PlaybackMediaMarker {}
unsafe impl ::core::marker::Sync for PlaybackMediaMarker {}
#[repr(transparent)]
pub struct PlaybackMediaMarkerReachedEventArgs(::windows_core::IUnknown);
impl PlaybackMediaMarkerReachedEventArgs {
    pub fn PlaybackMediaMarker(&self) -> ::windows_core::Result<PlaybackMediaMarker> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackMediaMarker)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PlaybackMediaMarker>(result__)
        }
    }
}
impl ::core::clone::Clone for PlaybackMediaMarkerReachedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlaybackMediaMarkerReachedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlaybackMediaMarkerReachedEventArgs {}
impl ::core::fmt::Debug for PlaybackMediaMarkerReachedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaybackMediaMarkerReachedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlaybackMediaMarkerReachedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.PlaybackMediaMarkerReachedEventArgs;{578cd1b9-90e2-4e60-abc4-8740b01f6196})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PlaybackMediaMarkerReachedEventArgs {
    type Vtable = IPlaybackMediaMarkerReachedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPlaybackMediaMarkerReachedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PlaybackMediaMarkerReachedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.PlaybackMediaMarkerReachedEventArgs";
}
impl ::core::convert::From<PlaybackMediaMarkerReachedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PlaybackMediaMarkerReachedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlaybackMediaMarkerReachedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PlaybackMediaMarkerReachedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PlaybackMediaMarkerReachedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PlaybackMediaMarkerReachedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlaybackMediaMarkerReachedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PlaybackMediaMarkerReachedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlaybackMediaMarkerReachedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PlaybackMediaMarkerReachedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PlaybackMediaMarkerReachedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PlaybackMediaMarkerReachedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PlaybackMediaMarkerReachedEventArgs {}
unsafe impl ::core::marker::Sync for PlaybackMediaMarkerReachedEventArgs {}
#[repr(transparent)]
pub struct PlaybackMediaMarkerSequence(::windows_core::IUnknown);
impl PlaybackMediaMarkerSequence {
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<PlaybackMediaMarker>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<PlaybackMediaMarker>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<PlaybackMediaMarker>>(result__)
        }
    }
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Insert<'a, Param0: ::windows_core::IntoParam<'a, PlaybackMediaMarker>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Insert)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for PlaybackMediaMarkerSequence {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlaybackMediaMarkerSequence {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlaybackMediaMarkerSequence {}
impl ::core::fmt::Debug for PlaybackMediaMarkerSequence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaybackMediaMarkerSequence").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlaybackMediaMarkerSequence {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.PlaybackMediaMarkerSequence;{f2810cee-638b-46cf-8817-1d111fe9d8c4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PlaybackMediaMarkerSequence {
    type Vtable = IPlaybackMediaMarkerSequence_Vtbl;
    const IID: ::windows_core::GUID = <IPlaybackMediaMarkerSequence as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PlaybackMediaMarkerSequence {
    const NAME: &'static str = "Windows.Media.Playback.PlaybackMediaMarkerSequence";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for PlaybackMediaMarkerSequence {
    type Item = PlaybackMediaMarker;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &PlaybackMediaMarkerSequence {
    type Item = PlaybackMediaMarker;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl ::core::convert::From<PlaybackMediaMarkerSequence> for ::windows_core::IUnknown {
    fn from(value: PlaybackMediaMarkerSequence) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlaybackMediaMarkerSequence> for ::windows_core::IUnknown {
    fn from(value: &PlaybackMediaMarkerSequence) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PlaybackMediaMarkerSequence {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PlaybackMediaMarkerSequence {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlaybackMediaMarkerSequence> for ::windows_core::IInspectable {
    fn from(value: PlaybackMediaMarkerSequence) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlaybackMediaMarkerSequence> for ::windows_core::IInspectable {
    fn from(value: &PlaybackMediaMarkerSequence) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PlaybackMediaMarkerSequence {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PlaybackMediaMarkerSequence {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<PlaybackMediaMarkerSequence> for ::winrt_foundation::Collections::IIterable<PlaybackMediaMarker> {
    type Error = ::windows_core::Error;
    fn try_from(value: PlaybackMediaMarkerSequence) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&PlaybackMediaMarkerSequence> for ::winrt_foundation::Collections::IIterable<PlaybackMediaMarker> {
    type Error = ::windows_core::Error;
    fn try_from(value: &PlaybackMediaMarkerSequence) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<PlaybackMediaMarker>> for PlaybackMediaMarkerSequence {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<PlaybackMediaMarker>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<PlaybackMediaMarker>> for &PlaybackMediaMarkerSequence {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<PlaybackMediaMarker>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<PlaybackMediaMarker>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PlaybackMediaMarkerSequence {}
unsafe impl ::core::marker::Sync for PlaybackMediaMarkerSequence {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SphericalVideoProjectionMode(pub i32);
impl SphericalVideoProjectionMode {
    pub const Spherical: Self = Self(0i32);
    pub const Flat: Self = Self(1i32);
}
impl ::core::marker::Copy for SphericalVideoProjectionMode {}
impl ::core::clone::Clone for SphericalVideoProjectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SphericalVideoProjectionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SphericalVideoProjectionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for SphericalVideoProjectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SphericalVideoProjectionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SphericalVideoProjectionMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.SphericalVideoProjectionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StereoscopicVideoRenderMode(pub i32);
impl StereoscopicVideoRenderMode {
    pub const Mono: Self = Self(0i32);
    pub const Stereo: Self = Self(1i32);
}
impl ::core::marker::Copy for StereoscopicVideoRenderMode {}
impl ::core::clone::Clone for StereoscopicVideoRenderMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StereoscopicVideoRenderMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for StereoscopicVideoRenderMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for StereoscopicVideoRenderMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StereoscopicVideoRenderMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StereoscopicVideoRenderMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.StereoscopicVideoRenderMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct TimedMetadataPresentationModeChangedEventArgs(::windows_core::IUnknown);
impl TimedMetadataPresentationModeChangedEventArgs {
    #[cfg(feature = "winrt-media")]
    pub fn Track(&self) -> ::windows_core::Result<super::Core::TimedMetadataTrack> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Track)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Core::TimedMetadataTrack>(result__)
        }
    }
    pub fn OldPresentationMode(&self) -> ::windows_core::Result<TimedMetadataTrackPresentationMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TimedMetadataTrackPresentationMode>::zeroed();
            (::windows_core::Interface::vtable(this).OldPresentationMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedMetadataTrackPresentationMode>(result__)
        }
    }
    pub fn NewPresentationMode(&self) -> ::windows_core::Result<TimedMetadataTrackPresentationMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<TimedMetadataTrackPresentationMode>::zeroed();
            (::windows_core::Interface::vtable(this).NewPresentationMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimedMetadataTrackPresentationMode>(result__)
        }
    }
}
impl ::core::clone::Clone for TimedMetadataPresentationModeChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimedMetadataPresentationModeChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedMetadataPresentationModeChangedEventArgs {}
impl ::core::fmt::Debug for TimedMetadataPresentationModeChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedMetadataPresentationModeChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedMetadataPresentationModeChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.TimedMetadataPresentationModeChangedEventArgs;{d1636099-65df-45ae-8cef-dc0b53fdc2bb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TimedMetadataPresentationModeChangedEventArgs {
    type Vtable = ITimedMetadataPresentationModeChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ITimedMetadataPresentationModeChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimedMetadataPresentationModeChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.TimedMetadataPresentationModeChangedEventArgs";
}
impl ::core::convert::From<TimedMetadataPresentationModeChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: TimedMetadataPresentationModeChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedMetadataPresentationModeChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &TimedMetadataPresentationModeChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TimedMetadataPresentationModeChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TimedMetadataPresentationModeChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimedMetadataPresentationModeChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: TimedMetadataPresentationModeChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedMetadataPresentationModeChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &TimedMetadataPresentationModeChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TimedMetadataPresentationModeChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TimedMetadataPresentationModeChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TimedMetadataPresentationModeChangedEventArgs {}
unsafe impl ::core::marker::Sync for TimedMetadataPresentationModeChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TimedMetadataTrackPresentationMode(pub i32);
impl TimedMetadataTrackPresentationMode {
    pub const Disabled: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
    pub const ApplicationPresented: Self = Self(2i32);
    pub const PlatformPresented: Self = Self(3i32);
}
impl ::core::marker::Copy for TimedMetadataTrackPresentationMode {}
impl ::core::clone::Clone for TimedMetadataTrackPresentationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedMetadataTrackPresentationMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TimedMetadataTrackPresentationMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for TimedMetadataTrackPresentationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedMetadataTrackPresentationMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TimedMetadataTrackPresentationMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.TimedMetadataTrackPresentationMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
