
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#[cfg(feature = "AppBroadcasting")]
pub mod AppBroadcasting;
#[cfg(feature = "AppRecording")]
pub mod AppRecording;
#[cfg(feature = "Audio")]
pub mod Audio;
#[cfg(feature = "Capture")]
pub mod Capture;
#[cfg(feature = "Casting")]
pub mod Casting;
#[cfg(feature = "ClosedCaptioning")]
pub mod ClosedCaptioning;
#[cfg(feature = "ContentRestrictions")]
pub mod ContentRestrictions;
#[cfg(feature = "Control")]
pub mod Control;
#[cfg(feature = "Core")]
pub mod Core;
#[cfg(feature = "Devices")]
pub mod Devices;
#[cfg(feature = "DialProtocol")]
pub mod DialProtocol;
#[cfg(feature = "Editing")]
pub mod Editing;
#[cfg(feature = "Effects")]
pub mod Effects;
#[cfg(feature = "FaceAnalysis")]
pub mod FaceAnalysis;
#[cfg(feature = "Import")]
pub mod Import;
#[cfg(feature = "MediaProperties")]
pub mod MediaProperties;
#[cfg(feature = "Miracast")]
pub mod Miracast;
#[cfg(feature = "Ocr")]
pub mod Ocr;
#[cfg(feature = "PlayTo")]
pub mod PlayTo;
#[cfg(feature = "Playback")]
pub mod Playback;
#[cfg(feature = "Playlists")]
pub mod Playlists;
#[cfg(feature = "Protection")]
pub mod Protection;
#[cfg(feature = "Render")]
pub mod Render;
#[cfg(feature = "SpeechRecognition")]
pub mod SpeechRecognition;
#[cfg(feature = "SpeechSynthesis")]
pub mod SpeechSynthesis;
#[cfg(feature = "Streaming")]
pub mod Streaming;
#[cfg(feature = "Transcoding")]
pub mod Transcoding;
#[repr(transparent)]
pub struct AudioBuffer(::windows_core::IUnknown);
impl AudioBuffer {
    pub fn Capacity(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Capacity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Length(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Length)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetLength(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLength)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CreateReference(&self) -> ::windows_core::Result<::winrt_foundation::IMemoryBufferReference> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateReference)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IMemoryBufferReference>(result__)
        }
    }
}
impl ::core::clone::Clone for AudioBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioBuffer {}
impl ::core::fmt::Debug for AudioBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioBuffer {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.AudioBuffer;{35175827-724b-4c6a-b130-f6537f9ae0d0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioBuffer {
    type Vtable = IAudioBuffer_Vtbl;
    const IID: ::windows_core::GUID = <IAudioBuffer as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioBuffer {
    const NAME: &'static str = "Windows.Media.AudioBuffer";
}
impl ::core::convert::From<AudioBuffer> for ::windows_core::IUnknown {
    fn from(value: AudioBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioBuffer> for ::windows_core::IUnknown {
    fn from(value: &AudioBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioBuffer> for ::windows_core::IInspectable {
    fn from(value: AudioBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioBuffer> for ::windows_core::IInspectable {
    fn from(value: &AudioBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AudioBuffer> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioBuffer) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioBuffer> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioBuffer) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for AudioBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &AudioBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioBuffer> for ::winrt_foundation::IMemoryBuffer {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioBuffer) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioBuffer> for ::winrt_foundation::IMemoryBuffer {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioBuffer) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IMemoryBuffer> for AudioBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IMemoryBuffer> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IMemoryBuffer> for &AudioBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IMemoryBuffer> {
        ::core::convert::TryInto::<::winrt_foundation::IMemoryBuffer>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioBuffer {}
unsafe impl ::core::marker::Sync for AudioBuffer {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioBufferAccessMode(pub i32);
impl AudioBufferAccessMode {
    pub const Read: Self = Self(0i32);
    pub const ReadWrite: Self = Self(1i32);
    pub const Write: Self = Self(2i32);
}
impl ::core::marker::Copy for AudioBufferAccessMode {}
impl ::core::clone::Clone for AudioBufferAccessMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioBufferAccessMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AudioBufferAccessMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioBufferAccessMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioBufferAccessMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioBufferAccessMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.AudioBufferAccessMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AudioFrame(::windows_core::IUnknown);
impl AudioFrame {
    pub fn LockBuffer(&self, mode: AudioBufferAccessMode) -> ::windows_core::Result<AudioBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LockBuffer)(::windows_core::Interface::as_raw(this), mode, result__.as_mut_ptr()).from_abi::<AudioBuffer>(result__)
        }
    }
    pub fn Create(capacity: u32) -> ::windows_core::Result<AudioFrame> {
        Self::IAudioFrameFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), capacity, result__.as_mut_ptr()).from_abi::<AudioFrame>(result__)
        })
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsReadOnly(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsReadOnly)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetRelativeTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRelativeTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RelativeTime(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetSystemRelativeTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSystemRelativeTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SystemRelativeTime(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SystemRelativeTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetDuration<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetIsDiscontinuous(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDiscontinuous)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDiscontinuous(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDiscontinuous)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IPropertySet> {
        let this = &::windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IPropertySet>(result__)
        }
    }
    pub fn IAudioFrameFactory<R, F: FnOnce(&IAudioFrameFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AudioFrame, IAudioFrameFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AudioFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioFrame {}
impl ::core::fmt::Debug for AudioFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.AudioFrame;{e36ac304-aab2-4277-9ed0-43cedf8e29c6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioFrame {
    type Vtable = IAudioFrame_Vtbl;
    const IID: ::windows_core::GUID = <IAudioFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioFrame {
    const NAME: &'static str = "Windows.Media.AudioFrame";
}
impl ::core::convert::From<AudioFrame> for ::windows_core::IUnknown {
    fn from(value: AudioFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioFrame> for ::windows_core::IUnknown {
    fn from(value: &AudioFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioFrame> for ::windows_core::IInspectable {
    fn from(value: AudioFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioFrame> for ::windows_core::IInspectable {
    fn from(value: &AudioFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AudioFrame> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioFrame) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioFrame> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioFrame) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for AudioFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &AudioFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioFrame> for IMediaFrame {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioFrame) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioFrame> for IMediaFrame {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioFrame) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaFrame> for AudioFrame {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaFrame> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaFrame> for &AudioFrame {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaFrame> {
        ::core::convert::TryInto::<IMediaFrame>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioFrame {}
unsafe impl ::core::marker::Sync for AudioFrame {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioProcessing(pub i32);
impl AudioProcessing {
    pub const Default: Self = Self(0i32);
    pub const Raw: Self = Self(1i32);
}
impl ::core::marker::Copy for AudioProcessing {}
impl ::core::clone::Clone for AudioProcessing {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioProcessing {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AudioProcessing {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioProcessing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioProcessing").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioProcessing {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.AudioProcessing;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AutoRepeatModeChangeRequestedEventArgs(::windows_core::IUnknown);
impl AutoRepeatModeChangeRequestedEventArgs {
    pub fn RequestedAutoRepeatMode(&self) -> ::windows_core::Result<MediaPlaybackAutoRepeatMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaPlaybackAutoRepeatMode>::zeroed();
            (::windows_core::Interface::vtable(this).RequestedAutoRepeatMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackAutoRepeatMode>(result__)
        }
    }
}
impl ::core::clone::Clone for AutoRepeatModeChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AutoRepeatModeChangeRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutoRepeatModeChangeRequestedEventArgs {}
impl ::core::fmt::Debug for AutoRepeatModeChangeRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutoRepeatModeChangeRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AutoRepeatModeChangeRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.AutoRepeatModeChangeRequestedEventArgs;{ea137efa-d852-438e-882b-c990109a78f4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AutoRepeatModeChangeRequestedEventArgs {
    type Vtable = IAutoRepeatModeChangeRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAutoRepeatModeChangeRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AutoRepeatModeChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.AutoRepeatModeChangeRequestedEventArgs";
}
impl ::core::convert::From<AutoRepeatModeChangeRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AutoRepeatModeChangeRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutoRepeatModeChangeRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AutoRepeatModeChangeRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AutoRepeatModeChangeRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AutoRepeatModeChangeRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AutoRepeatModeChangeRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AutoRepeatModeChangeRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutoRepeatModeChangeRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AutoRepeatModeChangeRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AutoRepeatModeChangeRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AutoRepeatModeChangeRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AutoRepeatModeChangeRequestedEventArgs {}
unsafe impl ::core::marker::Sync for AutoRepeatModeChangeRequestedEventArgs {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioBuffer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioBuffer {
    type Vtable = IAudioBuffer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x35175827_724b_4c6a_b130_f6537f9ae0d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioBuffer_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Capacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioFrame {
    type Vtable = IAudioFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe36ac304_aab2_4277_9ed0_43cedf8e29c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LockBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: AudioBufferAccessMode, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioFrameFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioFrameFactory {
    type Vtable = IAudioFrameFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x91a90ade_2422_40a6_b9ad_30d02404317d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capacity: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutoRepeatModeChangeRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutoRepeatModeChangeRequestedEventArgs {
    type Vtable = IAutoRepeatModeChangeRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea137efa_d852_438e_882b_c990109a78f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutoRepeatModeChangeRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestedAutoRepeatMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackAutoRepeatMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageDisplayProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImageDisplayProperties {
    type Vtable = IImageDisplayProperties_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd0bc7ef_54e7_411f_9933_f0e98b0a96d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageDisplayProperties_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSubtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IMediaControl(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IMediaControl {
    type Vtable = IMediaControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x98f1fbe1_7a8d_42cb_b6fe_8fe698264f13);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IMediaControl_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub SoundLevelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SoundLevelChanged: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveSoundLevelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveSoundLevelChanged: usize,
    #[cfg(feature = "deprecated")]
    pub PlayPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PlayPressed: usize,
    #[cfg(feature = "deprecated")]
    pub RemovePlayPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemovePlayPressed: usize,
    #[cfg(feature = "deprecated")]
    pub PausePressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PausePressed: usize,
    #[cfg(feature = "deprecated")]
    pub RemovePausePressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemovePausePressed: usize,
    #[cfg(feature = "deprecated")]
    pub StopPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    StopPressed: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveStopPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveStopPressed: usize,
    #[cfg(feature = "deprecated")]
    pub PlayPauseTogglePressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PlayPauseTogglePressed: usize,
    #[cfg(feature = "deprecated")]
    pub RemovePlayPauseTogglePressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemovePlayPauseTogglePressed: usize,
    #[cfg(feature = "deprecated")]
    pub RecordPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RecordPressed: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveRecordPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveRecordPressed: usize,
    #[cfg(feature = "deprecated")]
    pub NextTrackPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    NextTrackPressed: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveNextTrackPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveNextTrackPressed: usize,
    #[cfg(feature = "deprecated")]
    pub PreviousTrackPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PreviousTrackPressed: usize,
    #[cfg(feature = "deprecated")]
    pub RemovePreviousTrackPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemovePreviousTrackPressed: usize,
    #[cfg(feature = "deprecated")]
    pub FastForwardPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FastForwardPressed: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveFastForwardPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveFastForwardPressed: usize,
    #[cfg(feature = "deprecated")]
    pub RewindPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RewindPressed: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveRewindPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveRewindPressed: usize,
    #[cfg(feature = "deprecated")]
    pub ChannelUpPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ChannelUpPressed: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveChannelUpPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveChannelUpPressed: usize,
    #[cfg(feature = "deprecated")]
    pub ChannelDownPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ChannelDownPressed: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveChannelDownPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveChannelDownPressed: usize,
    #[cfg(feature = "deprecated")]
    pub SoundLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SoundLevel) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SoundLevel: usize,
    #[cfg(feature = "deprecated")]
    pub SetTrackName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetTrackName: usize,
    #[cfg(feature = "deprecated")]
    pub TrackName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TrackName: usize,
    #[cfg(feature = "deprecated")]
    pub SetArtistName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetArtistName: usize,
    #[cfg(feature = "deprecated")]
    pub ArtistName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ArtistName: usize,
    #[cfg(feature = "deprecated")]
    pub SetIsPlaying: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetIsPlaying: usize,
    #[cfg(feature = "deprecated")]
    pub IsPlaying: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsPlaying: usize,
    #[cfg(feature = "deprecated")]
    pub SetAlbumArt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetAlbumArt: usize,
    #[cfg(feature = "deprecated")]
    pub AlbumArt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AlbumArt: usize,
}
#[repr(transparent)]
pub struct IMediaExtension(::windows_core::IUnknown);
impl IMediaExtension {
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetProperties<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IPropertySet>>(&self, configuration: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetProperties)(::windows_core::Interface::as_raw(this), configuration.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IMediaExtension> for ::windows_core::IUnknown {
    fn from(value: IMediaExtension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaExtension> for ::windows_core::IUnknown {
    fn from(value: &IMediaExtension) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMediaExtension {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMediaExtension {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMediaExtension> for ::windows_core::IInspectable {
    fn from(value: IMediaExtension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaExtension> for ::windows_core::IInspectable {
    fn from(value: &IMediaExtension) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IMediaExtension {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IMediaExtension {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMediaExtension {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMediaExtension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaExtension {}
impl ::core::fmt::Debug for IMediaExtension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaExtension").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IMediaExtension {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{07915118-45df-442b-8a3f-f7826a6370ab}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IMediaExtension {
    type Vtable = IMediaExtension_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x07915118_45df_442b_8a3f_f7826a6370ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaExtension_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaExtensionManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaExtensionManager {
    type Vtable = IMediaExtensionManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4a25eaf5_242d_4dfb_97f4_69b7c42576ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaExtensionManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RegisterSchemeHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, scheme: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterSchemeHandlerWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, scheme: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, configuration: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterSchemeHandlerWithSettings: usize,
    pub RegisterByteStreamHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, fileextension: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, mimetype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterByteStreamHandlerWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, fileextension: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, mimetype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, configuration: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterByteStreamHandlerWithSettings: usize,
    pub RegisterAudioDecoder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, inputsubtype: ::windows_core::GUID, outputsubtype: ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterAudioDecoderWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, inputsubtype: ::windows_core::GUID, outputsubtype: ::windows_core::GUID, configuration: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterAudioDecoderWithSettings: usize,
    pub RegisterAudioEncoder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, inputsubtype: ::windows_core::GUID, outputsubtype: ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterAudioEncoderWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, inputsubtype: ::windows_core::GUID, outputsubtype: ::windows_core::GUID, configuration: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterAudioEncoderWithSettings: usize,
    pub RegisterVideoDecoder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, inputsubtype: ::windows_core::GUID, outputsubtype: ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterVideoDecoderWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, inputsubtype: ::windows_core::GUID, outputsubtype: ::windows_core::GUID, configuration: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterVideoDecoderWithSettings: usize,
    pub RegisterVideoEncoder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, inputsubtype: ::windows_core::GUID, outputsubtype: ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterVideoEncoderWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, inputsubtype: ::windows_core::GUID, outputsubtype: ::windows_core::GUID, configuration: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterVideoEncoderWithSettings: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaExtensionManager2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaExtensionManager2 {
    type Vtable = IMediaExtensionManager2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5bcebf47_4043_4fed_acaf_54ec29dfb1f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaExtensionManager2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_AppService")]
    pub RegisterMediaExtensionForAppService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extension: ::windows_core::RawPtr, connection: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_AppService"))]
    RegisterMediaExtensionForAppService: usize,
}
#[repr(transparent)]
pub struct IMediaFrame(::windows_core::IUnknown);
impl IMediaFrame {
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsReadOnly(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsReadOnly)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetRelativeTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRelativeTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RelativeTime(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetSystemRelativeTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSystemRelativeTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SystemRelativeTime(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SystemRelativeTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetDuration<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetIsDiscontinuous(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDiscontinuous)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDiscontinuous(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDiscontinuous)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IPropertySet>(result__)
        }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::convert::From<IMediaFrame> for ::windows_core::IUnknown {
    fn from(value: IMediaFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaFrame> for ::windows_core::IUnknown {
    fn from(value: &IMediaFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMediaFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMediaFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMediaFrame> for ::windows_core::IInspectable {
    fn from(value: IMediaFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaFrame> for ::windows_core::IInspectable {
    fn from(value: &IMediaFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IMediaFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IMediaFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IMediaFrame> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: IMediaFrame) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IMediaFrame> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &IMediaFrame) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for IMediaFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &IMediaFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IMediaFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMediaFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaFrame {}
impl ::core::fmt::Debug for IMediaFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IMediaFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{bfb52f8c-5943-47d8-8e10-05308aa5fbd0}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IMediaFrame {
    type Vtable = IMediaFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbfb52f8c_5943_47d8_8e10_05308aa5fbd0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetRelativeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RelativeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetSystemRelativeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SystemRelativeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetIsDiscontinuous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsDiscontinuous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ExtendedProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExtendedProperties: usize,
}
#[repr(transparent)]
pub struct IMediaMarker(::windows_core::IUnknown);
impl IMediaMarker {
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
}
impl ::core::convert::From<IMediaMarker> for ::windows_core::IUnknown {
    fn from(value: IMediaMarker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaMarker> for ::windows_core::IUnknown {
    fn from(value: &IMediaMarker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMediaMarker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMediaMarker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMediaMarker> for ::windows_core::IInspectable {
    fn from(value: IMediaMarker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaMarker> for ::windows_core::IInspectable {
    fn from(value: &IMediaMarker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IMediaMarker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IMediaMarker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMediaMarker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMediaMarker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaMarker {}
impl ::core::fmt::Debug for IMediaMarker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaMarker").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IMediaMarker {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{1803def8-dca5-4b6f-9c20-e3d3c0643625}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IMediaMarker {
    type Vtable = IMediaMarker_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1803def8_dca5_4b6f_9c20_e3d3c0643625);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaMarker_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Time: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub MediaMarkerType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaMarkerTypesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaMarkerTypesStatics {
    type Vtable = IMediaMarkerTypesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbb198040_482f_4743_8832_45853821ece0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaMarkerTypesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Bookmark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IMediaMarkers(::windows_core::IUnknown);
impl IMediaMarkers {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Markers(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<IMediaMarker>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Markers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<IMediaMarker>>(result__)
        }
    }
}
impl ::core::convert::From<IMediaMarkers> for ::windows_core::IUnknown {
    fn from(value: IMediaMarkers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaMarkers> for ::windows_core::IUnknown {
    fn from(value: &IMediaMarkers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMediaMarkers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMediaMarkers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMediaMarkers> for ::windows_core::IInspectable {
    fn from(value: IMediaMarkers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaMarkers> for ::windows_core::IInspectable {
    fn from(value: &IMediaMarkers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IMediaMarkers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IMediaMarkers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMediaMarkers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMediaMarkers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaMarkers {}
impl ::core::fmt::Debug for IMediaMarkers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaMarkers").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IMediaMarkers {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{afeab189-f8dd-466e-aa10-920b52353fdf}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IMediaMarkers {
    type Vtable = IMediaMarkers_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xafeab189_f8dd_466e_aa10_920b52353fdf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaMarkers_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Markers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Markers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaProcessingTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaProcessingTriggerDetails {
    type Vtable = IMediaProcessingTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeb8564ac_a351_4f4e_b4f0_9bf2408993db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaProcessingTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Arguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Arguments: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaTimelineController(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaTimelineController {
    type Vtable = IMediaTimelineController_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8ed361f3_0b78_4360_bf71_0c841999ea1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaTimelineController_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub ClockRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetClockRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaTimelineControllerState) -> ::windows_core::HRESULT,
    pub PositionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, positionchangedeventhandler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePositionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statechangedeventhandler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaTimelineController2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaTimelineController2 {
    type Vtable = IMediaTimelineController2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef74ea38_9e72_4df9_8355_6e90c81bbadd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaTimelineController2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsLoopingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsLoopingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Failed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Ended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaTimelineControllerFailedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaTimelineControllerFailedEventArgs {
    type Vtable = IMediaTimelineControllerFailedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8821f81d_3e77_43fb_be26_4fc87a044834);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaTimelineControllerFailedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMusicDisplayProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMusicDisplayProperties {
    type Vtable = IMusicDisplayProperties_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6bbf0c59_d0a0_4d26_92a0_f978e1d18e7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMusicDisplayProperties_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AlbumArtist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAlbumArtist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Artist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetArtist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMusicDisplayProperties2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMusicDisplayProperties2 {
    type Vtable = IMusicDisplayProperties2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00368462_97d3_44b9_b00f_008afcefaf18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMusicDisplayProperties2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AlbumTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAlbumTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TrackNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetTrackNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Genres: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Genres: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMusicDisplayProperties3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMusicDisplayProperties3 {
    type Vtable = IMusicDisplayProperties3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4db51ac1_0681_4e8c_9401_b8159d9eefc7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMusicDisplayProperties3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AlbumTrackCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetAlbumTrackCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaybackPositionChangeRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlaybackPositionChangeRequestedEventArgs {
    type Vtable = IPlaybackPositionChangeRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb4493f88_eb28_4961_9c14_335e44f3e125);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackPositionChangeRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestedPlaybackPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaybackRateChangeRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlaybackRateChangeRequestedEventArgs {
    type Vtable = IPlaybackRateChangeRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2ce2c41f_3cd6_4f77_9ba7_eb27c26a2140);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackRateChangeRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestedPlaybackRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShuffleEnabledChangeRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IShuffleEnabledChangeRequestedEventArgs {
    type Vtable = IShuffleEnabledChangeRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x49b593fe_4fd0_4666_a314_c0e01940d302);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShuffleEnabledChangeRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestedShuffleEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemMediaTransportControls(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemMediaTransportControls {
    type Vtable = ISystemMediaTransportControls_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x99fa3ff4_1742_42a6_902e_087d41f965ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControls_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PlaybackStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackStatus) -> ::windows_core::HRESULT,
    pub SetPlaybackStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaPlaybackStatus) -> ::windows_core::HRESULT,
    pub DisplayUpdater: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SoundLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SoundLevel) -> ::windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsPlayEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsPlayEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsStopEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsStopEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsPauseEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsPauseEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsRecordEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsRecordEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsFastForwardEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsFastForwardEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsRewindEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsRewindEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsPreviousEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsPreviousEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsNextEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsNextEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsChannelUpEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsChannelUpEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsChannelDownEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsChannelDownEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub PropertyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePropertyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemMediaTransportControls2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemMediaTransportControls2 {
    type Vtable = ISystemMediaTransportControls2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea98d2f6_7f3c_4af2_a586_72889808efb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControls2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AutoRepeatMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackAutoRepeatMode) -> ::windows_core::HRESULT,
    pub SetAutoRepeatMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaPlaybackAutoRepeatMode) -> ::windows_core::HRESULT,
    pub ShuffleEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetShuffleEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub PlaybackRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetPlaybackRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub UpdateTimelineProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timelineproperties: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PlaybackPositionChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePlaybackPositionChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub PlaybackRateChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePlaybackRateChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ShuffleEnabledChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveShuffleEnabledChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub AutoRepeatModeChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveAutoRepeatModeChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemMediaTransportControlsButtonPressedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemMediaTransportControlsButtonPressedEventArgs {
    type Vtable = ISystemMediaTransportControlsButtonPressedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb7f47116_a56f_4dc8_9e11_92031f4a87c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControlsButtonPressedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Button: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SystemMediaTransportControlsButton) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemMediaTransportControlsDisplayUpdater(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemMediaTransportControlsDisplayUpdater {
    type Vtable = ISystemMediaTransportControlsDisplayUpdater_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8abbc53e_fa55_4ecf_ad8e_c984e5dd1550);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControlsDisplayUpdater_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackType) -> ::windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaPlaybackType) -> ::windows_core::HRESULT,
    pub AppMediaId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAppMediaId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetThumbnail: usize,
    pub MusicProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub VideoProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ImageProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub CopyFromFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: MediaPlaybackType, source: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    CopyFromFileAsync: usize,
    pub ClearAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemMediaTransportControlsPropertyChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemMediaTransportControlsPropertyChangedEventArgs {
    type Vtable = ISystemMediaTransportControlsPropertyChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd0ca0936_339b_4cb3_8eeb_737607f56e08);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControlsPropertyChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SystemMediaTransportControlsProperty) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemMediaTransportControlsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemMediaTransportControlsStatics {
    type Vtable = ISystemMediaTransportControlsStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43ba380a_eca4_4832_91ab_d415fae484c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControlsStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemMediaTransportControlsTimelineProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemMediaTransportControlsTimelineProperties {
    type Vtable = ISystemMediaTransportControlsTimelineProperties_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5125316a_c3a2_475b_8507_93534dc88f15);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControlsTimelineProperties_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub EndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetEndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub MinSeekTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetMinSeekTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub MaxSeekTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetMaxSeekTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoDisplayProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoDisplayProperties {
    type Vtable = IVideoDisplayProperties_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5609fdb1_5d2d_4872_8170_45dee5bc2f5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoDisplayProperties_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSubtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoDisplayProperties2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoDisplayProperties2 {
    type Vtable = IVideoDisplayProperties2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb410e1ce_ab52_41ab_a486_cc10fab152f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoDisplayProperties2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Genres: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Genres: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoEffectsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoEffectsStatics {
    type Vtable = IVideoEffectsStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1fcda5e8_baf1_4521_980c_3bcebb44cf38);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEffectsStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub VideoStabilization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoFrame {
    type Vtable = IVideoFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0cc06625_90fc_4c92_bd95_7ded21819d1c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub SoftwareBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SoftwareBitmap: usize,
    pub CopyToAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frame: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3DSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3DSurface: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoFrame2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoFrame2 {
    type Vtable = IVideoFrame2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3837840d_336c_4366_8d46_060798736c5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoFrame2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub CopyToWithBoundsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frame: ::windows_core::RawPtr, sourcebounds: ::windows_core::RawPtr, destinationbounds: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    CopyToWithBoundsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoFrameFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoFrameFactory {
    type Vtable = IVideoFrameFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x014b6d69_2228_4c92_92ff_50c380d3e776);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoFrameFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: ::winrt_graphics::Imaging::BitmapPixelFormat, width: i32, height: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    Create: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub CreateWithAlpha: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: ::winrt_graphics::Imaging::BitmapPixelFormat, width: i32, height: i32, alpha: ::winrt_graphics::Imaging::BitmapAlphaMode, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    CreateWithAlpha: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoFrameStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoFrameStatics {
    type Vtable = IVideoFrameStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab2a556f_6111_4b33_8ec3_2b209a02e17a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoFrameStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Graphics_DirectX")]
    pub CreateAsDirect3D11SurfaceBacked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: ::winrt_graphics::DirectX::DirectXPixelFormat, width: i32, height: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    CreateAsDirect3D11SurfaceBacked: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateAsDirect3D11SurfaceBackedWithDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: ::winrt_graphics::DirectX::DirectXPixelFormat, width: i32, height: i32, device: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateAsDirect3D11SurfaceBackedWithDevice: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub CreateWithSoftwareBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    CreateWithSoftwareBitmap: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateWithDirect3D11Surface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, surface: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateWithDirect3D11Surface: usize,
}
#[repr(transparent)]
pub struct ImageDisplayProperties(::windows_core::IUnknown);
impl ImageDisplayProperties {
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetTitle<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTitle)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Subtitle(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Subtitle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetSubtitle<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSubtitle)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for ImageDisplayProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ImageDisplayProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ImageDisplayProperties {}
impl ::core::fmt::Debug for ImageDisplayProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageDisplayProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ImageDisplayProperties {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.ImageDisplayProperties;{cd0bc7ef-54e7-411f-9933-f0e98b0a96d2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ImageDisplayProperties {
    type Vtable = IImageDisplayProperties_Vtbl;
    const IID: ::windows_core::GUID = <IImageDisplayProperties as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ImageDisplayProperties {
    const NAME: &'static str = "Windows.Media.ImageDisplayProperties";
}
impl ::core::convert::From<ImageDisplayProperties> for ::windows_core::IUnknown {
    fn from(value: ImageDisplayProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageDisplayProperties> for ::windows_core::IUnknown {
    fn from(value: &ImageDisplayProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ImageDisplayProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ImageDisplayProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ImageDisplayProperties> for ::windows_core::IInspectable {
    fn from(value: ImageDisplayProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageDisplayProperties> for ::windows_core::IInspectable {
    fn from(value: &ImageDisplayProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ImageDisplayProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ImageDisplayProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ImageDisplayProperties {}
unsafe impl ::core::marker::Sync for ImageDisplayProperties {}
#[cfg(feature = "deprecated")]
pub struct MediaControl;
#[cfg(feature = "deprecated")]
impl MediaControl {
    #[cfg(feature = "deprecated")]
    pub fn SoundLevelChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SoundLevelChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveSoundLevelChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveSoundLevelChanged)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn PlayPressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PlayPressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemovePlayPressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).RemovePlayPressed)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn PausePressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PausePressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemovePausePressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).RemovePausePressed)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn StopPressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).StopPressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveStopPressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveStopPressed)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn PlayPauseTogglePressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PlayPauseTogglePressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemovePlayPauseTogglePressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).RemovePlayPauseTogglePressed)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn RecordPressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).RecordPressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveRecordPressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveRecordPressed)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn NextTrackPressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).NextTrackPressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveNextTrackPressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveNextTrackPressed)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn PreviousTrackPressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousTrackPressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemovePreviousTrackPressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).RemovePreviousTrackPressed)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn FastForwardPressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).FastForwardPressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveFastForwardPressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveFastForwardPressed)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn RewindPressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).RewindPressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveRewindPressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveRewindPressed)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn ChannelUpPressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ChannelUpPressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveChannelUpPressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveChannelUpPressed)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn ChannelDownPressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ChannelDownPressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveChannelDownPressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveChannelDownPressed)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn SoundLevel() -> ::windows_core::Result<SoundLevel> {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SoundLevel>::zeroed();
            (::windows_core::Interface::vtable(this).SoundLevel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SoundLevel>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn SetTrackName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(value: Param0) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).SetTrackName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn TrackName() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TrackName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn SetArtistName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(value: Param0) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).SetArtistName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn ArtistName() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ArtistName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn SetIsPlaying(value: bool) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).SetIsPlaying)(::windows_core::Interface::as_raw(this), value).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn IsPlaying() -> ::windows_core::Result<bool> {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPlaying)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn SetAlbumArt<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(value: Param0) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).SetAlbumArt)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn AlbumArt() -> ::windows_core::Result<::winrt_foundation::Uri> {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AlbumArt)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn IMediaControl<R, F: FnOnce(&IMediaControl) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MediaControl, IMediaControl> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for MediaControl {
    const NAME: &'static str = "Windows.Media.MediaControl";
}
#[repr(transparent)]
pub struct MediaExtensionManager(::windows_core::IUnknown);
impl MediaExtensionManager {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MediaExtensionManager, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn RegisterSchemeHandler<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, activatableclassid: Param0, scheme: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RegisterSchemeHandler)(::windows_core::Interface::as_raw(this), activatableclassid.into_param().abi(), scheme.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterSchemeHandlerWithSettings<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IPropertySet>>(&self, activatableclassid: Param0, scheme: Param1, configuration: Param2) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RegisterSchemeHandlerWithSettings)(::windows_core::Interface::as_raw(this), activatableclassid.into_param().abi(), scheme.into_param().abi(), configuration.into_param().abi()).ok() }
    }
    pub fn RegisterByteStreamHandler<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, activatableclassid: Param0, fileextension: Param1, mimetype: Param2) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RegisterByteStreamHandler)(::windows_core::Interface::as_raw(this), activatableclassid.into_param().abi(), fileextension.into_param().abi(), mimetype.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterByteStreamHandlerWithSettings<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IPropertySet>>(&self, activatableclassid: Param0, fileextension: Param1, mimetype: Param2, configuration: Param3) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RegisterByteStreamHandlerWithSettings)(::windows_core::Interface::as_raw(this), activatableclassid.into_param().abi(), fileextension.into_param().abi(), mimetype.into_param().abi(), configuration.into_param().abi()).ok() }
    }
    pub fn RegisterAudioDecoder<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param2: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, activatableclassid: Param0, inputsubtype: Param1, outputsubtype: Param2) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RegisterAudioDecoder)(::windows_core::Interface::as_raw(this), activatableclassid.into_param().abi(), inputsubtype.into_param().abi(), outputsubtype.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterAudioDecoderWithSettings<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param2: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param3: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IPropertySet>>(&self, activatableclassid: Param0, inputsubtype: Param1, outputsubtype: Param2, configuration: Param3) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RegisterAudioDecoderWithSettings)(::windows_core::Interface::as_raw(this), activatableclassid.into_param().abi(), inputsubtype.into_param().abi(), outputsubtype.into_param().abi(), configuration.into_param().abi()).ok() }
    }
    pub fn RegisterAudioEncoder<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param2: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, activatableclassid: Param0, inputsubtype: Param1, outputsubtype: Param2) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RegisterAudioEncoder)(::windows_core::Interface::as_raw(this), activatableclassid.into_param().abi(), inputsubtype.into_param().abi(), outputsubtype.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterAudioEncoderWithSettings<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param2: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param3: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IPropertySet>>(&self, activatableclassid: Param0, inputsubtype: Param1, outputsubtype: Param2, configuration: Param3) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RegisterAudioEncoderWithSettings)(::windows_core::Interface::as_raw(this), activatableclassid.into_param().abi(), inputsubtype.into_param().abi(), outputsubtype.into_param().abi(), configuration.into_param().abi()).ok() }
    }
    pub fn RegisterVideoDecoder<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param2: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, activatableclassid: Param0, inputsubtype: Param1, outputsubtype: Param2) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RegisterVideoDecoder)(::windows_core::Interface::as_raw(this), activatableclassid.into_param().abi(), inputsubtype.into_param().abi(), outputsubtype.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterVideoDecoderWithSettings<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param2: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param3: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IPropertySet>>(&self, activatableclassid: Param0, inputsubtype: Param1, outputsubtype: Param2, configuration: Param3) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RegisterVideoDecoderWithSettings)(::windows_core::Interface::as_raw(this), activatableclassid.into_param().abi(), inputsubtype.into_param().abi(), outputsubtype.into_param().abi(), configuration.into_param().abi()).ok() }
    }
    pub fn RegisterVideoEncoder<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param2: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, activatableclassid: Param0, inputsubtype: Param1, outputsubtype: Param2) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RegisterVideoEncoder)(::windows_core::Interface::as_raw(this), activatableclassid.into_param().abi(), inputsubtype.into_param().abi(), outputsubtype.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterVideoEncoderWithSettings<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param2: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param3: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IPropertySet>>(&self, activatableclassid: Param0, inputsubtype: Param1, outputsubtype: Param2, configuration: Param3) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RegisterVideoEncoderWithSettings)(::windows_core::Interface::as_raw(this), activatableclassid.into_param().abi(), inputsubtype.into_param().abi(), outputsubtype.into_param().abi(), configuration.into_param().abi()).ok() }
    }
    #[cfg(feature = "ApplicationModel_AppService")]
    pub fn RegisterMediaExtensionForAppService<'a, Param0: ::windows_core::IntoParam<'a, IMediaExtension>, Param1: ::windows_core::IntoParam<'a, ::winrt_applicationmodel::AppService::AppServiceConnection>>(&self, extension: Param0, connection: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaExtensionManager2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RegisterMediaExtensionForAppService)(::windows_core::Interface::as_raw(this), extension.into_param().abi(), connection.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for MediaExtensionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaExtensionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaExtensionManager {}
impl ::core::fmt::Debug for MediaExtensionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaExtensionManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaExtensionManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.MediaExtensionManager;{4a25eaf5-242d-4dfb-97f4-69b7c42576ff})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaExtensionManager {
    type Vtable = IMediaExtensionManager_Vtbl;
    const IID: ::windows_core::GUID = <IMediaExtensionManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaExtensionManager {
    const NAME: &'static str = "Windows.Media.MediaExtensionManager";
}
impl ::core::convert::From<MediaExtensionManager> for ::windows_core::IUnknown {
    fn from(value: MediaExtensionManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaExtensionManager> for ::windows_core::IUnknown {
    fn from(value: &MediaExtensionManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaExtensionManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaExtensionManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaExtensionManager> for ::windows_core::IInspectable {
    fn from(value: MediaExtensionManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaExtensionManager> for ::windows_core::IInspectable {
    fn from(value: &MediaExtensionManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaExtensionManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaExtensionManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaExtensionManager {}
unsafe impl ::core::marker::Sync for MediaExtensionManager {}
pub struct MediaMarkerTypes;
impl MediaMarkerTypes {
    pub fn Bookmark() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaMarkerTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Bookmark)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IMediaMarkerTypesStatics<R, F: FnOnce(&IMediaMarkerTypesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MediaMarkerTypes, IMediaMarkerTypesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for MediaMarkerTypes {
    const NAME: &'static str = "Windows.Media.MediaMarkerTypes";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaPlaybackAutoRepeatMode(pub i32);
impl MediaPlaybackAutoRepeatMode {
    pub const None: Self = Self(0i32);
    pub const Track: Self = Self(1i32);
    pub const List: Self = Self(2i32);
}
impl ::core::marker::Copy for MediaPlaybackAutoRepeatMode {}
impl ::core::clone::Clone for MediaPlaybackAutoRepeatMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaPlaybackAutoRepeatMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaPlaybackAutoRepeatMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaPlaybackAutoRepeatMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackAutoRepeatMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlaybackAutoRepeatMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.MediaPlaybackAutoRepeatMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaPlaybackStatus(pub i32);
impl MediaPlaybackStatus {
    pub const Closed: Self = Self(0i32);
    pub const Changing: Self = Self(1i32);
    pub const Stopped: Self = Self(2i32);
    pub const Playing: Self = Self(3i32);
    pub const Paused: Self = Self(4i32);
}
impl ::core::marker::Copy for MediaPlaybackStatus {}
impl ::core::clone::Clone for MediaPlaybackStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaPlaybackStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaPlaybackStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaPlaybackStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlaybackStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.MediaPlaybackStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaPlaybackType(pub i32);
impl MediaPlaybackType {
    pub const Unknown: Self = Self(0i32);
    pub const Music: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
    pub const Image: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaPlaybackType {}
impl ::core::clone::Clone for MediaPlaybackType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaPlaybackType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaPlaybackType {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaPlaybackType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaPlaybackType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.MediaPlaybackType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MediaProcessingTriggerDetails(::windows_core::IUnknown);
impl MediaProcessingTriggerDetails {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Arguments(&self) -> ::windows_core::Result<::winrt_foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::ValueSet>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaProcessingTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaProcessingTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaProcessingTriggerDetails {}
impl ::core::fmt::Debug for MediaProcessingTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaProcessingTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaProcessingTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.MediaProcessingTriggerDetails;{eb8564ac-a351-4f4e-b4f0-9bf2408993db})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaProcessingTriggerDetails {
    type Vtable = IMediaProcessingTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IMediaProcessingTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaProcessingTriggerDetails {
    const NAME: &'static str = "Windows.Media.MediaProcessingTriggerDetails";
}
impl ::core::convert::From<MediaProcessingTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: MediaProcessingTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaProcessingTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &MediaProcessingTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaProcessingTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaProcessingTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaProcessingTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: MediaProcessingTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaProcessingTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &MediaProcessingTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaProcessingTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaProcessingTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaProcessingTriggerDetails {}
unsafe impl ::core::marker::Sync for MediaProcessingTriggerDetails {}
#[repr(C)]
pub struct MediaTimeRange {
    pub Start: ::winrt_foundation::TimeSpan,
    pub End: ::winrt_foundation::TimeSpan,
}
impl ::core::marker::Copy for MediaTimeRange {}
impl ::core::clone::Clone for MediaTimeRange {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MediaTimeRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MediaTimeRange").field("Start", &self.Start).field("End", &self.End).finish()
    }
}
unsafe impl ::windows_core::Abi for MediaTimeRange {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for MediaTimeRange {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Media.MediaTimeRange;struct(Windows.Foundation.TimeSpan;i8);struct(Windows.Foundation.TimeSpan;i8))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for MediaTimeRange {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MediaTimeRange>()) == 0 }
    }
}
impl ::core::cmp::Eq for MediaTimeRange {}
impl ::core::default::Default for MediaTimeRange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct MediaTimelineController(::windows_core::IUnknown);
impl MediaTimelineController {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MediaTimelineController, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Resume(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Resume)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Pause(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Pause)(::windows_core::Interface::as_raw(this)).ok() }
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
    pub fn ClockRate(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ClockRate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetClockRate(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetClockRate)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<MediaTimelineControllerState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaTimelineControllerState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaTimelineControllerState>(result__)
        }
    }
    pub fn PositionChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaTimelineController, ::windows_core::IInspectable>>>(&self, positionchangedeventhandler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PositionChanged)(::windows_core::Interface::as_raw(this), positionchangedeventhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePositionChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePositionChanged)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
    pub fn StateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaTimelineController, ::windows_core::IInspectable>>>(&self, statechangedeventhandler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).StateChanged)(::windows_core::Interface::as_raw(this), statechangedeventhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStateChanged)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetDuration<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsLoopingEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsLoopingEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsLoopingEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsLoopingEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Failed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaTimelineController, MediaTimelineControllerFailedEventArgs>>>(&self, eventhandler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Failed)(::windows_core::Interface::as_raw(this), eventhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveFailed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFailed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Ended<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaTimelineController, ::windows_core::IInspectable>>>(&self, eventhandler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Ended)(::windows_core::Interface::as_raw(this), eventhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveEnded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEnded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for MediaTimelineController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaTimelineController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaTimelineController {}
impl ::core::fmt::Debug for MediaTimelineController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaTimelineController").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaTimelineController {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.MediaTimelineController;{8ed361f3-0b78-4360-bf71-0c841999ea1b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaTimelineController {
    type Vtable = IMediaTimelineController_Vtbl;
    const IID: ::windows_core::GUID = <IMediaTimelineController as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaTimelineController {
    const NAME: &'static str = "Windows.Media.MediaTimelineController";
}
impl ::core::convert::From<MediaTimelineController> for ::windows_core::IUnknown {
    fn from(value: MediaTimelineController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaTimelineController> for ::windows_core::IUnknown {
    fn from(value: &MediaTimelineController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaTimelineController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaTimelineController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaTimelineController> for ::windows_core::IInspectable {
    fn from(value: MediaTimelineController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaTimelineController> for ::windows_core::IInspectable {
    fn from(value: &MediaTimelineController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaTimelineController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaTimelineController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaTimelineController {}
unsafe impl ::core::marker::Sync for MediaTimelineController {}
#[repr(transparent)]
pub struct MediaTimelineControllerFailedEventArgs(::windows_core::IUnknown);
impl MediaTimelineControllerFailedEventArgs {
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaTimelineControllerFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaTimelineControllerFailedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaTimelineControllerFailedEventArgs {}
impl ::core::fmt::Debug for MediaTimelineControllerFailedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaTimelineControllerFailedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaTimelineControllerFailedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.MediaTimelineControllerFailedEventArgs;{8821f81d-3e77-43fb-be26-4fc87a044834})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaTimelineControllerFailedEventArgs {
    type Vtable = IMediaTimelineControllerFailedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaTimelineControllerFailedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaTimelineControllerFailedEventArgs {
    const NAME: &'static str = "Windows.Media.MediaTimelineControllerFailedEventArgs";
}
impl ::core::convert::From<MediaTimelineControllerFailedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaTimelineControllerFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaTimelineControllerFailedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaTimelineControllerFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaTimelineControllerFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaTimelineControllerFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaTimelineControllerFailedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaTimelineControllerFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaTimelineControllerFailedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaTimelineControllerFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaTimelineControllerFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaTimelineControllerFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaTimelineControllerFailedEventArgs {}
unsafe impl ::core::marker::Sync for MediaTimelineControllerFailedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaTimelineControllerState(pub i32);
impl MediaTimelineControllerState {
    pub const Paused: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const Stalled: Self = Self(2i32);
    pub const Error: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaTimelineControllerState {}
impl ::core::clone::Clone for MediaTimelineControllerState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaTimelineControllerState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaTimelineControllerState {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaTimelineControllerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaTimelineControllerState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaTimelineControllerState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.MediaTimelineControllerState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MusicDisplayProperties(::windows_core::IUnknown);
impl MusicDisplayProperties {
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetTitle<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTitle)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn AlbumArtist(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AlbumArtist)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetAlbumArtist<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAlbumArtist)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Artist(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Artist)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetArtist<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetArtist)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn AlbumTitle(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMusicDisplayProperties2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AlbumTitle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetAlbumTitle<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMusicDisplayProperties2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAlbumTitle)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn TrackNumber(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IMusicDisplayProperties2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).TrackNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetTrackNumber(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMusicDisplayProperties2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTrackNumber)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Genres(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = &::windows_core::Interface::cast::<IMusicDisplayProperties2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Genres)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn AlbumTrackCount(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IMusicDisplayProperties3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).AlbumTrackCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetAlbumTrackCount(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMusicDisplayProperties3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAlbumTrackCount)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for MusicDisplayProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MusicDisplayProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MusicDisplayProperties {}
impl ::core::fmt::Debug for MusicDisplayProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MusicDisplayProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MusicDisplayProperties {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.MusicDisplayProperties;{6bbf0c59-d0a0-4d26-92a0-f978e1d18e7b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MusicDisplayProperties {
    type Vtable = IMusicDisplayProperties_Vtbl;
    const IID: ::windows_core::GUID = <IMusicDisplayProperties as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MusicDisplayProperties {
    const NAME: &'static str = "Windows.Media.MusicDisplayProperties";
}
impl ::core::convert::From<MusicDisplayProperties> for ::windows_core::IUnknown {
    fn from(value: MusicDisplayProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MusicDisplayProperties> for ::windows_core::IUnknown {
    fn from(value: &MusicDisplayProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MusicDisplayProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MusicDisplayProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MusicDisplayProperties> for ::windows_core::IInspectable {
    fn from(value: MusicDisplayProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MusicDisplayProperties> for ::windows_core::IInspectable {
    fn from(value: &MusicDisplayProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MusicDisplayProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MusicDisplayProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MusicDisplayProperties {}
unsafe impl ::core::marker::Sync for MusicDisplayProperties {}
#[repr(transparent)]
pub struct PlaybackPositionChangeRequestedEventArgs(::windows_core::IUnknown);
impl PlaybackPositionChangeRequestedEventArgs {
    pub fn RequestedPlaybackPosition(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).RequestedPlaybackPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for PlaybackPositionChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlaybackPositionChangeRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlaybackPositionChangeRequestedEventArgs {}
impl ::core::fmt::Debug for PlaybackPositionChangeRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaybackPositionChangeRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlaybackPositionChangeRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.PlaybackPositionChangeRequestedEventArgs;{b4493f88-eb28-4961-9c14-335e44f3e125})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PlaybackPositionChangeRequestedEventArgs {
    type Vtable = IPlaybackPositionChangeRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPlaybackPositionChangeRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PlaybackPositionChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlaybackPositionChangeRequestedEventArgs";
}
impl ::core::convert::From<PlaybackPositionChangeRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PlaybackPositionChangeRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlaybackPositionChangeRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PlaybackPositionChangeRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PlaybackPositionChangeRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PlaybackPositionChangeRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlaybackPositionChangeRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PlaybackPositionChangeRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlaybackPositionChangeRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PlaybackPositionChangeRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PlaybackPositionChangeRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PlaybackPositionChangeRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PlaybackPositionChangeRequestedEventArgs {}
unsafe impl ::core::marker::Sync for PlaybackPositionChangeRequestedEventArgs {}
#[repr(transparent)]
pub struct PlaybackRateChangeRequestedEventArgs(::windows_core::IUnknown);
impl PlaybackRateChangeRequestedEventArgs {
    pub fn RequestedPlaybackRate(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).RequestedPlaybackRate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for PlaybackRateChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlaybackRateChangeRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlaybackRateChangeRequestedEventArgs {}
impl ::core::fmt::Debug for PlaybackRateChangeRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaybackRateChangeRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlaybackRateChangeRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.PlaybackRateChangeRequestedEventArgs;{2ce2c41f-3cd6-4f77-9ba7-eb27c26a2140})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PlaybackRateChangeRequestedEventArgs {
    type Vtable = IPlaybackRateChangeRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPlaybackRateChangeRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PlaybackRateChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlaybackRateChangeRequestedEventArgs";
}
impl ::core::convert::From<PlaybackRateChangeRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PlaybackRateChangeRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlaybackRateChangeRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PlaybackRateChangeRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PlaybackRateChangeRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PlaybackRateChangeRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlaybackRateChangeRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PlaybackRateChangeRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlaybackRateChangeRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PlaybackRateChangeRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PlaybackRateChangeRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PlaybackRateChangeRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PlaybackRateChangeRequestedEventArgs {}
unsafe impl ::core::marker::Sync for PlaybackRateChangeRequestedEventArgs {}
#[repr(transparent)]
pub struct ShuffleEnabledChangeRequestedEventArgs(::windows_core::IUnknown);
impl ShuffleEnabledChangeRequestedEventArgs {
    pub fn RequestedShuffleEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).RequestedShuffleEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for ShuffleEnabledChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ShuffleEnabledChangeRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ShuffleEnabledChangeRequestedEventArgs {}
impl ::core::fmt::Debug for ShuffleEnabledChangeRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShuffleEnabledChangeRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ShuffleEnabledChangeRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.ShuffleEnabledChangeRequestedEventArgs;{49b593fe-4fd0-4666-a314-c0e01940d302})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ShuffleEnabledChangeRequestedEventArgs {
    type Vtable = IShuffleEnabledChangeRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IShuffleEnabledChangeRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ShuffleEnabledChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.ShuffleEnabledChangeRequestedEventArgs";
}
impl ::core::convert::From<ShuffleEnabledChangeRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ShuffleEnabledChangeRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ShuffleEnabledChangeRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ShuffleEnabledChangeRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ShuffleEnabledChangeRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ShuffleEnabledChangeRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ShuffleEnabledChangeRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ShuffleEnabledChangeRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ShuffleEnabledChangeRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ShuffleEnabledChangeRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ShuffleEnabledChangeRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ShuffleEnabledChangeRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ShuffleEnabledChangeRequestedEventArgs {}
unsafe impl ::core::marker::Sync for ShuffleEnabledChangeRequestedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SoundLevel(pub i32);
impl SoundLevel {
    pub const Muted: Self = Self(0i32);
    pub const Low: Self = Self(1i32);
    pub const Full: Self = Self(2i32);
}
impl ::core::marker::Copy for SoundLevel {}
impl ::core::clone::Clone for SoundLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SoundLevel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SoundLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for SoundLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SoundLevel").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SoundLevel {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.SoundLevel;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SystemMediaTransportControls(::windows_core::IUnknown);
impl SystemMediaTransportControls {
    pub fn PlaybackStatus(&self) -> ::windows_core::Result<MediaPlaybackStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaPlaybackStatus>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackStatus>(result__)
        }
    }
    pub fn SetPlaybackStatus(&self, value: MediaPlaybackStatus) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPlaybackStatus)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DisplayUpdater(&self) -> ::windows_core::Result<SystemMediaTransportControlsDisplayUpdater> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayUpdater)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SystemMediaTransportControlsDisplayUpdater>(result__)
        }
    }
    pub fn SoundLevel(&self) -> ::windows_core::Result<SoundLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SoundLevel>::zeroed();
            (::windows_core::Interface::vtable(this).SoundLevel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SoundLevel>(result__)
        }
    }
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
    pub fn IsPlayEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPlayEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsPlayEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsPlayEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsStopEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsStopEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsStopEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsStopEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsPauseEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPauseEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsPauseEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsPauseEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsRecordEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsRecordEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsRecordEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsRecordEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsFastForwardEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsFastForwardEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsFastForwardEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsFastForwardEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsRewindEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsRewindEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsRewindEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsRewindEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsPreviousEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPreviousEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsPreviousEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsPreviousEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsNextEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsNextEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsNextEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsNextEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsChannelUpEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsChannelUpEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsChannelUpEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsChannelUpEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsChannelDownEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsChannelDownEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsChannelDownEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsChannelDownEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ButtonPressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SystemMediaTransportControls, SystemMediaTransportControlsButtonPressedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ButtonPressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveButtonPressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveButtonPressed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn PropertyChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SystemMediaTransportControls, SystemMediaTransportControlsPropertyChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PropertyChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePropertyChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePropertyChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn AutoRepeatMode(&self) -> ::windows_core::Result<MediaPlaybackAutoRepeatMode> {
        let this = &::windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaPlaybackAutoRepeatMode>::zeroed();
            (::windows_core::Interface::vtable(this).AutoRepeatMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackAutoRepeatMode>(result__)
        }
    }
    pub fn SetAutoRepeatMode(&self, value: MediaPlaybackAutoRepeatMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAutoRepeatMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShuffleEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ShuffleEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetShuffleEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetShuffleEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PlaybackRate(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackRate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetPlaybackRate(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPlaybackRate)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UpdateTimelineProperties<'a, Param0: ::windows_core::IntoParam<'a, SystemMediaTransportControlsTimelineProperties>>(&self, timelineproperties: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).UpdateTimelineProperties)(::windows_core::Interface::as_raw(this), timelineproperties.into_param().abi()).ok() }
    }
    pub fn PlaybackPositionChangeRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SystemMediaTransportControls, PlaybackPositionChangeRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackPositionChangeRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePlaybackPositionChangeRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemovePlaybackPositionChangeRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn PlaybackRateChangeRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SystemMediaTransportControls, PlaybackRateChangeRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackRateChangeRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePlaybackRateChangeRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemovePlaybackRateChangeRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ShuffleEnabledChangeRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SystemMediaTransportControls, ShuffleEnabledChangeRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ShuffleEnabledChangeRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveShuffleEnabledChangeRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveShuffleEnabledChangeRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn AutoRepeatModeChangeRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SystemMediaTransportControls, AutoRepeatModeChangeRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AutoRepeatModeChangeRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAutoRepeatModeChangeRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAutoRepeatModeChangeRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<SystemMediaTransportControls> {
        Self::ISystemMediaTransportControlsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SystemMediaTransportControls>(result__)
        })
    }
    pub fn ISystemMediaTransportControlsStatics<R, F: FnOnce(&ISystemMediaTransportControlsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SystemMediaTransportControls, ISystemMediaTransportControlsStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SystemMediaTransportControls {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemMediaTransportControls {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemMediaTransportControls {}
impl ::core::fmt::Debug for SystemMediaTransportControls {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemMediaTransportControls").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SystemMediaTransportControls {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.SystemMediaTransportControls;{99fa3ff4-1742-42a6-902e-087d41f965ec})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SystemMediaTransportControls {
    type Vtable = ISystemMediaTransportControls_Vtbl;
    const IID: ::windows_core::GUID = <ISystemMediaTransportControls as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SystemMediaTransportControls {
    const NAME: &'static str = "Windows.Media.SystemMediaTransportControls";
}
impl ::core::convert::From<SystemMediaTransportControls> for ::windows_core::IUnknown {
    fn from(value: SystemMediaTransportControls) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemMediaTransportControls> for ::windows_core::IUnknown {
    fn from(value: &SystemMediaTransportControls) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SystemMediaTransportControls {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SystemMediaTransportControls {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemMediaTransportControls> for ::windows_core::IInspectable {
    fn from(value: SystemMediaTransportControls) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemMediaTransportControls> for ::windows_core::IInspectable {
    fn from(value: &SystemMediaTransportControls) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SystemMediaTransportControls {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SystemMediaTransportControls {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemMediaTransportControls {}
unsafe impl ::core::marker::Sync for SystemMediaTransportControls {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SystemMediaTransportControlsButton(pub i32);
impl SystemMediaTransportControlsButton {
    pub const Play: Self = Self(0i32);
    pub const Pause: Self = Self(1i32);
    pub const Stop: Self = Self(2i32);
    pub const Record: Self = Self(3i32);
    pub const FastForward: Self = Self(4i32);
    pub const Rewind: Self = Self(5i32);
    pub const Next: Self = Self(6i32);
    pub const Previous: Self = Self(7i32);
    pub const ChannelUp: Self = Self(8i32);
    pub const ChannelDown: Self = Self(9i32);
}
impl ::core::marker::Copy for SystemMediaTransportControlsButton {}
impl ::core::clone::Clone for SystemMediaTransportControlsButton {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SystemMediaTransportControlsButton {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SystemMediaTransportControlsButton {
    type Abi = Self;
}
impl ::core::fmt::Debug for SystemMediaTransportControlsButton {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemMediaTransportControlsButton").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SystemMediaTransportControlsButton {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.SystemMediaTransportControlsButton;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SystemMediaTransportControlsButtonPressedEventArgs(::windows_core::IUnknown);
impl SystemMediaTransportControlsButtonPressedEventArgs {
    pub fn Button(&self) -> ::windows_core::Result<SystemMediaTransportControlsButton> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SystemMediaTransportControlsButton>::zeroed();
            (::windows_core::Interface::vtable(this).Button)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SystemMediaTransportControlsButton>(result__)
        }
    }
}
impl ::core::clone::Clone for SystemMediaTransportControlsButtonPressedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemMediaTransportControlsButtonPressedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemMediaTransportControlsButtonPressedEventArgs {}
impl ::core::fmt::Debug for SystemMediaTransportControlsButtonPressedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemMediaTransportControlsButtonPressedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SystemMediaTransportControlsButtonPressedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.SystemMediaTransportControlsButtonPressedEventArgs;{b7f47116-a56f-4dc8-9e11-92031f4a87c2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SystemMediaTransportControlsButtonPressedEventArgs {
    type Vtable = ISystemMediaTransportControlsButtonPressedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISystemMediaTransportControlsButtonPressedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SystemMediaTransportControlsButtonPressedEventArgs {
    const NAME: &'static str = "Windows.Media.SystemMediaTransportControlsButtonPressedEventArgs";
}
impl ::core::convert::From<SystemMediaTransportControlsButtonPressedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SystemMediaTransportControlsButtonPressedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemMediaTransportControlsButtonPressedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SystemMediaTransportControlsButtonPressedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SystemMediaTransportControlsButtonPressedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SystemMediaTransportControlsButtonPressedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemMediaTransportControlsButtonPressedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SystemMediaTransportControlsButtonPressedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemMediaTransportControlsButtonPressedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SystemMediaTransportControlsButtonPressedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SystemMediaTransportControlsButtonPressedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SystemMediaTransportControlsButtonPressedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemMediaTransportControlsButtonPressedEventArgs {}
unsafe impl ::core::marker::Sync for SystemMediaTransportControlsButtonPressedEventArgs {}
#[repr(transparent)]
pub struct SystemMediaTransportControlsDisplayUpdater(::windows_core::IUnknown);
impl SystemMediaTransportControlsDisplayUpdater {
    pub fn Type(&self) -> ::windows_core::Result<MediaPlaybackType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaPlaybackType>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaPlaybackType>(result__)
        }
    }
    pub fn SetType(&self, value: MediaPlaybackType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AppMediaId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppMediaId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetAppMediaId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAppMediaId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows_core::Result<::winrt_storage::Streams::RandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Thumbnail)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::RandomAccessStreamReference>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetThumbnail<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::RandomAccessStreamReference>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetThumbnail)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn MusicProperties(&self) -> ::windows_core::Result<MusicDisplayProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MusicProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MusicDisplayProperties>(result__)
        }
    }
    pub fn VideoProperties(&self) -> ::windows_core::Result<VideoDisplayProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<VideoDisplayProperties>(result__)
        }
    }
    pub fn ImageProperties(&self) -> ::windows_core::Result<ImageDisplayProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImageProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ImageDisplayProperties>(result__)
        }
    }
    #[cfg(feature = "Storage")]
    pub fn CopyFromFileAsync<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::StorageFile>>(&self, r#type: MediaPlaybackType, source: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CopyFromFileAsync)(::windows_core::Interface::as_raw(this), r#type, source.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn ClearAll(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ClearAll)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Update(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Update)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for SystemMediaTransportControlsDisplayUpdater {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemMediaTransportControlsDisplayUpdater {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemMediaTransportControlsDisplayUpdater {}
impl ::core::fmt::Debug for SystemMediaTransportControlsDisplayUpdater {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemMediaTransportControlsDisplayUpdater").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SystemMediaTransportControlsDisplayUpdater {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.SystemMediaTransportControlsDisplayUpdater;{8abbc53e-fa55-4ecf-ad8e-c984e5dd1550})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SystemMediaTransportControlsDisplayUpdater {
    type Vtable = ISystemMediaTransportControlsDisplayUpdater_Vtbl;
    const IID: ::windows_core::GUID = <ISystemMediaTransportControlsDisplayUpdater as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SystemMediaTransportControlsDisplayUpdater {
    const NAME: &'static str = "Windows.Media.SystemMediaTransportControlsDisplayUpdater";
}
impl ::core::convert::From<SystemMediaTransportControlsDisplayUpdater> for ::windows_core::IUnknown {
    fn from(value: SystemMediaTransportControlsDisplayUpdater) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemMediaTransportControlsDisplayUpdater> for ::windows_core::IUnknown {
    fn from(value: &SystemMediaTransportControlsDisplayUpdater) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SystemMediaTransportControlsDisplayUpdater {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SystemMediaTransportControlsDisplayUpdater {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemMediaTransportControlsDisplayUpdater> for ::windows_core::IInspectable {
    fn from(value: SystemMediaTransportControlsDisplayUpdater) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemMediaTransportControlsDisplayUpdater> for ::windows_core::IInspectable {
    fn from(value: &SystemMediaTransportControlsDisplayUpdater) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SystemMediaTransportControlsDisplayUpdater {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SystemMediaTransportControlsDisplayUpdater {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemMediaTransportControlsDisplayUpdater {}
unsafe impl ::core::marker::Sync for SystemMediaTransportControlsDisplayUpdater {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SystemMediaTransportControlsProperty(pub i32);
impl SystemMediaTransportControlsProperty {
    pub const SoundLevel: Self = Self(0i32);
}
impl ::core::marker::Copy for SystemMediaTransportControlsProperty {}
impl ::core::clone::Clone for SystemMediaTransportControlsProperty {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SystemMediaTransportControlsProperty {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SystemMediaTransportControlsProperty {
    type Abi = Self;
}
impl ::core::fmt::Debug for SystemMediaTransportControlsProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemMediaTransportControlsProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SystemMediaTransportControlsProperty {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.SystemMediaTransportControlsProperty;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SystemMediaTransportControlsPropertyChangedEventArgs(::windows_core::IUnknown);
impl SystemMediaTransportControlsPropertyChangedEventArgs {
    pub fn Property(&self) -> ::windows_core::Result<SystemMediaTransportControlsProperty> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SystemMediaTransportControlsProperty>::zeroed();
            (::windows_core::Interface::vtable(this).Property)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SystemMediaTransportControlsProperty>(result__)
        }
    }
}
impl ::core::clone::Clone for SystemMediaTransportControlsPropertyChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemMediaTransportControlsPropertyChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemMediaTransportControlsPropertyChangedEventArgs {}
impl ::core::fmt::Debug for SystemMediaTransportControlsPropertyChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemMediaTransportControlsPropertyChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SystemMediaTransportControlsPropertyChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.SystemMediaTransportControlsPropertyChangedEventArgs;{d0ca0936-339b-4cb3-8eeb-737607f56e08})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SystemMediaTransportControlsPropertyChangedEventArgs {
    type Vtable = ISystemMediaTransportControlsPropertyChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISystemMediaTransportControlsPropertyChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SystemMediaTransportControlsPropertyChangedEventArgs {
    const NAME: &'static str = "Windows.Media.SystemMediaTransportControlsPropertyChangedEventArgs";
}
impl ::core::convert::From<SystemMediaTransportControlsPropertyChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SystemMediaTransportControlsPropertyChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemMediaTransportControlsPropertyChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SystemMediaTransportControlsPropertyChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SystemMediaTransportControlsPropertyChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SystemMediaTransportControlsPropertyChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemMediaTransportControlsPropertyChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SystemMediaTransportControlsPropertyChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemMediaTransportControlsPropertyChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SystemMediaTransportControlsPropertyChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SystemMediaTransportControlsPropertyChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SystemMediaTransportControlsPropertyChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemMediaTransportControlsPropertyChangedEventArgs {}
unsafe impl ::core::marker::Sync for SystemMediaTransportControlsPropertyChangedEventArgs {}
#[repr(transparent)]
pub struct SystemMediaTransportControlsTimelineProperties(::windows_core::IUnknown);
impl SystemMediaTransportControlsTimelineProperties {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SystemMediaTransportControlsTimelineProperties, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn StartTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetStartTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn EndTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).EndTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetEndTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEndTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn MinSeekTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).MinSeekTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetMinSeekTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMinSeekTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn MaxSeekTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).MaxSeekTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetMaxSeekTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxSeekTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
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
}
impl ::core::clone::Clone for SystemMediaTransportControlsTimelineProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemMediaTransportControlsTimelineProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemMediaTransportControlsTimelineProperties {}
impl ::core::fmt::Debug for SystemMediaTransportControlsTimelineProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemMediaTransportControlsTimelineProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SystemMediaTransportControlsTimelineProperties {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.SystemMediaTransportControlsTimelineProperties;{5125316a-c3a2-475b-8507-93534dc88f15})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SystemMediaTransportControlsTimelineProperties {
    type Vtable = ISystemMediaTransportControlsTimelineProperties_Vtbl;
    const IID: ::windows_core::GUID = <ISystemMediaTransportControlsTimelineProperties as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SystemMediaTransportControlsTimelineProperties {
    const NAME: &'static str = "Windows.Media.SystemMediaTransportControlsTimelineProperties";
}
impl ::core::convert::From<SystemMediaTransportControlsTimelineProperties> for ::windows_core::IUnknown {
    fn from(value: SystemMediaTransportControlsTimelineProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemMediaTransportControlsTimelineProperties> for ::windows_core::IUnknown {
    fn from(value: &SystemMediaTransportControlsTimelineProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SystemMediaTransportControlsTimelineProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SystemMediaTransportControlsTimelineProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemMediaTransportControlsTimelineProperties> for ::windows_core::IInspectable {
    fn from(value: SystemMediaTransportControlsTimelineProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemMediaTransportControlsTimelineProperties> for ::windows_core::IInspectable {
    fn from(value: &SystemMediaTransportControlsTimelineProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SystemMediaTransportControlsTimelineProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SystemMediaTransportControlsTimelineProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemMediaTransportControlsTimelineProperties {}
unsafe impl ::core::marker::Sync for SystemMediaTransportControlsTimelineProperties {}
#[repr(transparent)]
pub struct VideoDisplayProperties(::windows_core::IUnknown);
impl VideoDisplayProperties {
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetTitle<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTitle)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Subtitle(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Subtitle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetSubtitle<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSubtitle)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Genres(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = &::windows_core::Interface::cast::<IVideoDisplayProperties2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Genres)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
}
impl ::core::clone::Clone for VideoDisplayProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VideoDisplayProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoDisplayProperties {}
impl ::core::fmt::Debug for VideoDisplayProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoDisplayProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VideoDisplayProperties {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.VideoDisplayProperties;{5609fdb1-5d2d-4872-8170-45dee5bc2f5c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for VideoDisplayProperties {
    type Vtable = IVideoDisplayProperties_Vtbl;
    const IID: ::windows_core::GUID = <IVideoDisplayProperties as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VideoDisplayProperties {
    const NAME: &'static str = "Windows.Media.VideoDisplayProperties";
}
impl ::core::convert::From<VideoDisplayProperties> for ::windows_core::IUnknown {
    fn from(value: VideoDisplayProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoDisplayProperties> for ::windows_core::IUnknown {
    fn from(value: &VideoDisplayProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for VideoDisplayProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a VideoDisplayProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VideoDisplayProperties> for ::windows_core::IInspectable {
    fn from(value: VideoDisplayProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoDisplayProperties> for ::windows_core::IInspectable {
    fn from(value: &VideoDisplayProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for VideoDisplayProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a VideoDisplayProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VideoDisplayProperties {}
unsafe impl ::core::marker::Sync for VideoDisplayProperties {}
pub struct VideoEffects;
impl VideoEffects {
    pub fn VideoStabilization() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IVideoEffectsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoStabilization)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IVideoEffectsStatics<R, F: FnOnce(&IVideoEffectsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<VideoEffects, IVideoEffectsStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for VideoEffects {
    const NAME: &'static str = "Windows.Media.VideoEffects";
}
#[repr(transparent)]
pub struct VideoFrame(::windows_core::IUnknown);
impl VideoFrame {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsReadOnly(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsReadOnly)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetRelativeTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRelativeTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RelativeTime(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetSystemRelativeTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSystemRelativeTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SystemRelativeTime(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SystemRelativeTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetDuration<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetIsDiscontinuous(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDiscontinuous)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDiscontinuous(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDiscontinuous)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IPropertySet> {
        let this = &::windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IPropertySet>(result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SoftwareBitmap(&self) -> ::windows_core::Result<::winrt_graphics::Imaging::SoftwareBitmap> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SoftwareBitmap)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::Imaging::SoftwareBitmap>(result__)
        }
    }
    pub fn CopyToAsync<'a, Param0: ::windows_core::IntoParam<'a, VideoFrame>>(&self, frame: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CopyToAsync)(::windows_core::Interface::as_raw(this), frame.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Direct3DSurface(&self) -> ::windows_core::Result<::winrt_graphics::DirectX::Direct3D11::IDirect3DSurface> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Direct3DSurface)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::DirectX::Direct3D11::IDirect3DSurface>(result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn CopyToWithBoundsAsync<'a, Param0: ::windows_core::IntoParam<'a, VideoFrame>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_graphics::Imaging::BitmapBounds>>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_graphics::Imaging::BitmapBounds>>>(&self, frame: Param0, sourcebounds: Param1, destinationbounds: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IVideoFrame2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CopyToWithBoundsAsync)(::windows_core::Interface::as_raw(this), frame.into_param().abi(), sourcebounds.into_param().abi(), destinationbounds.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn Create(format: ::winrt_graphics::Imaging::BitmapPixelFormat, width: i32, height: i32) -> ::windows_core::Result<VideoFrame> {
        Self::IVideoFrameFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), format, width, height, result__.as_mut_ptr()).from_abi::<VideoFrame>(result__)
        })
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn CreateWithAlpha(format: ::winrt_graphics::Imaging::BitmapPixelFormat, width: i32, height: i32, alpha: ::winrt_graphics::Imaging::BitmapAlphaMode) -> ::windows_core::Result<VideoFrame> {
        Self::IVideoFrameFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithAlpha)(::windows_core::Interface::as_raw(this), format, width, height, alpha, result__.as_mut_ptr()).from_abi::<VideoFrame>(result__)
        })
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn CreateAsDirect3D11SurfaceBacked(format: ::winrt_graphics::DirectX::DirectXPixelFormat, width: i32, height: i32) -> ::windows_core::Result<VideoFrame> {
        Self::IVideoFrameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAsDirect3D11SurfaceBacked)(::windows_core::Interface::as_raw(this), format, width, height, result__.as_mut_ptr()).from_abi::<VideoFrame>(result__)
        })
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CreateAsDirect3D11SurfaceBackedWithDevice<'a, Param3: ::windows_core::IntoParam<'a, ::winrt_graphics::DirectX::Direct3D11::IDirect3DDevice>>(format: ::winrt_graphics::DirectX::DirectXPixelFormat, width: i32, height: i32, device: Param3) -> ::windows_core::Result<VideoFrame> {
        Self::IVideoFrameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAsDirect3D11SurfaceBackedWithDevice)(::windows_core::Interface::as_raw(this), format, width, height, device.into_param().abi(), result__.as_mut_ptr()).from_abi::<VideoFrame>(result__)
        })
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn CreateWithSoftwareBitmap<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::SoftwareBitmap>>(bitmap: Param0) -> ::windows_core::Result<VideoFrame> {
        Self::IVideoFrameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithSoftwareBitmap)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi(), result__.as_mut_ptr()).from_abi::<VideoFrame>(result__)
        })
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CreateWithDirect3D11Surface<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::DirectX::Direct3D11::IDirect3DSurface>>(surface: Param0) -> ::windows_core::Result<VideoFrame> {
        Self::IVideoFrameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithDirect3D11Surface)(::windows_core::Interface::as_raw(this), surface.into_param().abi(), result__.as_mut_ptr()).from_abi::<VideoFrame>(result__)
        })
    }
    pub fn IVideoFrameFactory<R, F: FnOnce(&IVideoFrameFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<VideoFrame, IVideoFrameFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IVideoFrameStatics<R, F: FnOnce(&IVideoFrameStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<VideoFrame, IVideoFrameStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for VideoFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VideoFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoFrame {}
impl ::core::fmt::Debug for VideoFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VideoFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.VideoFrame;{0cc06625-90fc-4c92-bd95-7ded21819d1c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for VideoFrame {
    type Vtable = IVideoFrame_Vtbl;
    const IID: ::windows_core::GUID = <IVideoFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VideoFrame {
    const NAME: &'static str = "Windows.Media.VideoFrame";
}
impl ::core::convert::From<VideoFrame> for ::windows_core::IUnknown {
    fn from(value: VideoFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoFrame> for ::windows_core::IUnknown {
    fn from(value: &VideoFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for VideoFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a VideoFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VideoFrame> for ::windows_core::IInspectable {
    fn from(value: VideoFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoFrame> for ::windows_core::IInspectable {
    fn from(value: &VideoFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for VideoFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a VideoFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<VideoFrame> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: VideoFrame) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VideoFrame> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &VideoFrame) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for VideoFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &VideoFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<VideoFrame> for IMediaFrame {
    type Error = ::windows_core::Error;
    fn try_from(value: VideoFrame) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VideoFrame> for IMediaFrame {
    type Error = ::windows_core::Error;
    fn try_from(value: &VideoFrame) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaFrame> for VideoFrame {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaFrame> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IMediaFrame> for &VideoFrame {
    fn into_param(self) -> ::windows_core::Param<'a, IMediaFrame> {
        ::core::convert::TryInto::<IMediaFrame>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for VideoFrame {}
unsafe impl ::core::marker::Sync for VideoFrame {}
