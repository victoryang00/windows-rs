#[doc(hidden)]
#[repr(transparent)]
pub struct IInstalledVoicesStatic(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInstalledVoicesStatic {
    type Vtable = IInstalledVoicesStatic_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d526ecc_7533_4c3f_85be_888c2baeebdc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInstalledVoicesStatic_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub AllVoices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    AllVoices: usize,
    pub DefaultVoice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInstalledVoicesStatic2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInstalledVoicesStatic2 {
    type Vtable = IInstalledVoicesStatic2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64255f2e_358d_4058_be9a_fd3fcb423530);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInstalledVoicesStatic2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TrySetDefaultVoiceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, voice: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechSynthesisStream(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechSynthesisStream {
    type Vtable = ISpeechSynthesisStream_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x83e46e93_244c_4622_ba0b_6229c4d0d65d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechSynthesisStream_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Markers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Markers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechSynthesizer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechSynthesizer {
    type Vtable = ISpeechSynthesizer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xce9f7c76_97f4_4ced_ad68_d51c458e45c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechSynthesizer_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SynthesizeTextToStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SynthesizeSsmlToStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ssml: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetVoice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Voice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechSynthesizer2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechSynthesizer2 {
    type Vtable = ISpeechSynthesizer2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa7c5ecb2_4339_4d6a_bbf8_c7a4f1544c2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechSynthesizer2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Options: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechSynthesizerOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechSynthesizerOptions {
    type Vtable = ISpeechSynthesizerOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa0e23871_cc3d_43c9_91b1_ee185324d83d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechSynthesizerOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IncludeWordBoundaryMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIncludeWordBoundaryMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IncludeSentenceBoundaryMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIncludeSentenceBoundaryMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechSynthesizerOptions2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechSynthesizerOptions2 {
    type Vtable = ISpeechSynthesizerOptions2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1cbef60e_119c_4bed_b118_d250c3a25793);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechSynthesizerOptions2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AudioVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetAudioVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub SpeakingRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetSpeakingRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub AudioPitch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetAudioPitch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechSynthesizerOptions3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechSynthesizerOptions3 {
    type Vtable = ISpeechSynthesizerOptions3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x401ed877_902c_4814_a582_a5d0c0769fa8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechSynthesizerOptions3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AppendedSilence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechAppendedSilence) -> ::windows_core::HRESULT,
    pub SetAppendedSilence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SpeechAppendedSilence) -> ::windows_core::HRESULT,
    pub PunctuationSilence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechPunctuationSilence) -> ::windows_core::HRESULT,
    pub SetPunctuationSilence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SpeechPunctuationSilence) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoiceInformation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVoiceInformation {
    type Vtable = IVoiceInformation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb127d6a4_1291_4604_aa9c_83134083352c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceInformation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Gender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VoiceGender) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SpeechAppendedSilence(pub i32);
impl SpeechAppendedSilence {
    pub const Default: Self = Self(0i32);
    pub const Min: Self = Self(1i32);
}
impl ::core::marker::Copy for SpeechAppendedSilence {}
impl ::core::clone::Clone for SpeechAppendedSilence {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechAppendedSilence {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SpeechAppendedSilence {
    type Abi = Self;
}
impl ::core::fmt::Debug for SpeechAppendedSilence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechAppendedSilence").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpeechAppendedSilence {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechSynthesis.SpeechAppendedSilence;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SpeechPunctuationSilence(pub i32);
impl SpeechPunctuationSilence {
    pub const Default: Self = Self(0i32);
    pub const Min: Self = Self(1i32);
}
impl ::core::marker::Copy for SpeechPunctuationSilence {}
impl ::core::clone::Clone for SpeechPunctuationSilence {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechPunctuationSilence {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SpeechPunctuationSilence {
    type Abi = Self;
}
impl ::core::fmt::Debug for SpeechPunctuationSilence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechPunctuationSilence").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpeechPunctuationSilence {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechSynthesis.SpeechPunctuationSilence;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SpeechSynthesisStream(::windows_core::IUnknown);
impl SpeechSynthesisStream {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn ContentType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IContentTypeProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ContentType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn ReadAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, buffer: Param0, count: u32, options: ::winrt_storage::Streams::InputStreamOptions) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IBuffer, u32>> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IInputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsync)(::windows_core::Interface::as_raw(this), buffer.into_param().abi(), count, options, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IBuffer, u32>>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn WriteAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, buffer: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WriteAsync)(::windows_core::Interface::as_raw(this), buffer.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn FlushAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FlushAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn Size(&self) -> ::windows_core::Result<u64> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn SetSize(&self, value: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows_core::Result<::winrt_storage::Streams::IInputStream> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetInputStreamAt)(::windows_core::Interface::as_raw(this), position, result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IInputStream>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows_core::Result<::winrt_storage::Streams::IOutputStream> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetOutputStreamAt)(::windows_core::Interface::as_raw(this), position, result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IOutputStream>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn Position(&self) -> ::windows_core::Result<u64> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn Seek(&self, position: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Seek)(::windows_core::Interface::as_raw(this), position).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CloneStream(&self) -> ::windows_core::Result<::winrt_storage::Streams::IRandomAccessStream> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CloneStream)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IRandomAccessStream>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CanRead(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanRead)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CanWrite(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanWrite)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Markers(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<super::IMediaMarker>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Markers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<super::IMediaMarker>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
    pub fn TimedMetadataTracks(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<super::Core::TimedMetadataTrack>> {
        let this = &::windows_core::Interface::cast::<super::Core::ITimedMetadataTrackProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TimedMetadataTracks)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<super::Core::TimedMetadataTrack>>(result__)
        }
    }
}
impl ::core::clone::Clone for SpeechSynthesisStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpeechSynthesisStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechSynthesisStream {}
impl ::core::fmt::Debug for SpeechSynthesisStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechSynthesisStream").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpeechSynthesisStream {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechSynthesis.SpeechSynthesisStream;{83e46e93-244c-4622-ba0b-6229c4d0d65d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpeechSynthesisStream {
    type Vtable = ISpeechSynthesisStream_Vtbl;
    const IID: ::windows_core::GUID = <ISpeechSynthesisStream as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpeechSynthesisStream {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.SpeechSynthesisStream";
}
impl ::core::convert::From<SpeechSynthesisStream> for ::windows_core::IUnknown {
    fn from(value: SpeechSynthesisStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechSynthesisStream> for ::windows_core::IUnknown {
    fn from(value: &SpeechSynthesisStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpeechSynthesisStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpeechSynthesisStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpeechSynthesisStream> for ::windows_core::IInspectable {
    fn from(value: SpeechSynthesisStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechSynthesisStream> for ::windows_core::IInspectable {
    fn from(value: &SpeechSynthesisStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpeechSynthesisStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpeechSynthesisStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SpeechSynthesisStream> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: SpeechSynthesisStream) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SpeechSynthesisStream> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &SpeechSynthesisStream) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for SpeechSynthesisStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &SpeechSynthesisStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::TryFrom<SpeechSynthesisStream> for ::winrt_storage::Streams::IContentTypeProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: SpeechSynthesisStream) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::TryFrom<&SpeechSynthesisStream> for ::winrt_storage::Streams::IContentTypeProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: &SpeechSynthesisStream) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IContentTypeProvider> for SpeechSynthesisStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IContentTypeProvider> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IContentTypeProvider> for &SpeechSynthesisStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IContentTypeProvider> {
        ::core::convert::TryInto::<::winrt_storage::Streams::IContentTypeProvider>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::TryFrom<SpeechSynthesisStream> for ::winrt_storage::Streams::IInputStream {
    type Error = ::windows_core::Error;
    fn try_from(value: SpeechSynthesisStream) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::TryFrom<&SpeechSynthesisStream> for ::winrt_storage::Streams::IInputStream {
    type Error = ::windows_core::Error;
    fn try_from(value: &SpeechSynthesisStream) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream> for SpeechSynthesisStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IInputStream> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream> for &SpeechSynthesisStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IInputStream> {
        ::core::convert::TryInto::<::winrt_storage::Streams::IInputStream>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::TryFrom<SpeechSynthesisStream> for ::winrt_storage::Streams::IOutputStream {
    type Error = ::windows_core::Error;
    fn try_from(value: SpeechSynthesisStream) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::TryFrom<&SpeechSynthesisStream> for ::winrt_storage::Streams::IOutputStream {
    type Error = ::windows_core::Error;
    fn try_from(value: &SpeechSynthesisStream) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IOutputStream> for SpeechSynthesisStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IOutputStream> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IOutputStream> for &SpeechSynthesisStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IOutputStream> {
        ::core::convert::TryInto::<::winrt_storage::Streams::IOutputStream>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::TryFrom<SpeechSynthesisStream> for ::winrt_storage::Streams::IRandomAccessStream {
    type Error = ::windows_core::Error;
    fn try_from(value: SpeechSynthesisStream) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::TryFrom<&SpeechSynthesisStream> for ::winrt_storage::Streams::IRandomAccessStream {
    type Error = ::windows_core::Error;
    fn try_from(value: &SpeechSynthesisStream) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream> for SpeechSynthesisStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IRandomAccessStream> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream> for &SpeechSynthesisStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IRandomAccessStream> {
        ::core::convert::TryInto::<::winrt_storage::Streams::IRandomAccessStream>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::TryFrom<SpeechSynthesisStream> for ::winrt_storage::Streams::IRandomAccessStreamWithContentType {
    type Error = ::windows_core::Error;
    fn try_from(value: SpeechSynthesisStream) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::TryFrom<&SpeechSynthesisStream> for ::winrt_storage::Streams::IRandomAccessStreamWithContentType {
    type Error = ::windows_core::Error;
    fn try_from(value: &SpeechSynthesisStream) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStreamWithContentType> for SpeechSynthesisStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IRandomAccessStreamWithContentType> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStreamWithContentType> for &SpeechSynthesisStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IRandomAccessStreamWithContentType> {
        ::core::convert::TryInto::<::winrt_storage::Streams::IRandomAccessStreamWithContentType>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::convert::TryFrom<SpeechSynthesisStream> for super::Core::ITimedMetadataTrackProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: SpeechSynthesisStream) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::convert::TryFrom<&SpeechSynthesisStream> for super::Core::ITimedMetadataTrackProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: &SpeechSynthesisStream) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-media")]
impl<'a> ::windows_core::IntoParam<'a, super::Core::ITimedMetadataTrackProvider> for SpeechSynthesisStream {
    fn into_param(self) -> ::windows_core::Param<'a, super::Core::ITimedMetadataTrackProvider> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-media")]
impl<'a> ::windows_core::IntoParam<'a, super::Core::ITimedMetadataTrackProvider> for &SpeechSynthesisStream {
    fn into_param(self) -> ::windows_core::Param<'a, super::Core::ITimedMetadataTrackProvider> {
        ::core::convert::TryInto::<super::Core::ITimedMetadataTrackProvider>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SpeechSynthesisStream {}
unsafe impl ::core::marker::Sync for SpeechSynthesisStream {}
#[repr(transparent)]
pub struct SpeechSynthesizer(::windows_core::IUnknown);
impl SpeechSynthesizer {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SpeechSynthesizer, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn AllVoices() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<VoiceInformation>> {
        Self::IInstalledVoicesStatic(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AllVoices)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<VoiceInformation>>(result__)
        })
    }
    pub fn DefaultVoice() -> ::windows_core::Result<VoiceInformation> {
        Self::IInstalledVoicesStatic(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultVoice)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<VoiceInformation>(result__)
        })
    }
    pub fn TrySetDefaultVoiceAsync<'a, Param0: ::windows_core::IntoParam<'a, VoiceInformation>>(voice: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        Self::IInstalledVoicesStatic2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TrySetDefaultVoiceAsync)(::windows_core::Interface::as_raw(this), voice.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn SynthesizeTextToStreamAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, text: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SpeechSynthesisStream>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SynthesizeTextToStreamAsync)(::windows_core::Interface::as_raw(this), text.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SpeechSynthesisStream>>(result__)
        }
    }
    pub fn SynthesizeSsmlToStreamAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, ssml: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SpeechSynthesisStream>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SynthesizeSsmlToStreamAsync)(::windows_core::Interface::as_raw(this), ssml.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SpeechSynthesisStream>>(result__)
        }
    }
    pub fn SetVoice<'a, Param0: ::windows_core::IntoParam<'a, VoiceInformation>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVoice)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Voice(&self) -> ::windows_core::Result<VoiceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Voice)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<VoiceInformation>(result__)
        }
    }
    pub fn Options(&self) -> ::windows_core::Result<SpeechSynthesizerOptions> {
        let this = &::windows_core::Interface::cast::<ISpeechSynthesizer2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Options)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpeechSynthesizerOptions>(result__)
        }
    }
    pub fn IInstalledVoicesStatic<R, F: FnOnce(&IInstalledVoicesStatic) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SpeechSynthesizer, IInstalledVoicesStatic> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IInstalledVoicesStatic2<R, F: FnOnce(&IInstalledVoicesStatic2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SpeechSynthesizer, IInstalledVoicesStatic2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpeechSynthesizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpeechSynthesizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechSynthesizer {}
impl ::core::fmt::Debug for SpeechSynthesizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechSynthesizer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpeechSynthesizer {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechSynthesis.SpeechSynthesizer;{ce9f7c76-97f4-4ced-ad68-d51c458e45c6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpeechSynthesizer {
    type Vtable = ISpeechSynthesizer_Vtbl;
    const IID: ::windows_core::GUID = <ISpeechSynthesizer as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpeechSynthesizer {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.SpeechSynthesizer";
}
impl ::core::convert::From<SpeechSynthesizer> for ::windows_core::IUnknown {
    fn from(value: SpeechSynthesizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechSynthesizer> for ::windows_core::IUnknown {
    fn from(value: &SpeechSynthesizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpeechSynthesizer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpeechSynthesizer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpeechSynthesizer> for ::windows_core::IInspectable {
    fn from(value: SpeechSynthesizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechSynthesizer> for ::windows_core::IInspectable {
    fn from(value: &SpeechSynthesizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpeechSynthesizer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpeechSynthesizer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SpeechSynthesizer> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: SpeechSynthesizer) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SpeechSynthesizer> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &SpeechSynthesizer) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for SpeechSynthesizer {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &SpeechSynthesizer {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SpeechSynthesizer {}
unsafe impl ::core::marker::Sync for SpeechSynthesizer {}
#[repr(transparent)]
pub struct SpeechSynthesizerOptions(::windows_core::IUnknown);
impl SpeechSynthesizerOptions {
    pub fn IncludeWordBoundaryMetadata(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IncludeWordBoundaryMetadata)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIncludeWordBoundaryMetadata(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIncludeWordBoundaryMetadata)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IncludeSentenceBoundaryMetadata(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IncludeSentenceBoundaryMetadata)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIncludeSentenceBoundaryMetadata(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIncludeSentenceBoundaryMetadata)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AudioVolume(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<ISpeechSynthesizerOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).AudioVolume)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetAudioVolume(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISpeechSynthesizerOptions2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAudioVolume)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SpeakingRate(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<ISpeechSynthesizerOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).SpeakingRate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetSpeakingRate(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISpeechSynthesizerOptions2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSpeakingRate)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AudioPitch(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<ISpeechSynthesizerOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).AudioPitch)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetAudioPitch(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISpeechSynthesizerOptions2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAudioPitch)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AppendedSilence(&self) -> ::windows_core::Result<SpeechAppendedSilence> {
        let this = &::windows_core::Interface::cast::<ISpeechSynthesizerOptions3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SpeechAppendedSilence>::zeroed();
            (::windows_core::Interface::vtable(this).AppendedSilence)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpeechAppendedSilence>(result__)
        }
    }
    pub fn SetAppendedSilence(&self, value: SpeechAppendedSilence) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISpeechSynthesizerOptions3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAppendedSilence)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PunctuationSilence(&self) -> ::windows_core::Result<SpeechPunctuationSilence> {
        let this = &::windows_core::Interface::cast::<ISpeechSynthesizerOptions3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SpeechPunctuationSilence>::zeroed();
            (::windows_core::Interface::vtable(this).PunctuationSilence)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpeechPunctuationSilence>(result__)
        }
    }
    pub fn SetPunctuationSilence(&self, value: SpeechPunctuationSilence) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISpeechSynthesizerOptions3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPunctuationSilence)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for SpeechSynthesizerOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpeechSynthesizerOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechSynthesizerOptions {}
impl ::core::fmt::Debug for SpeechSynthesizerOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechSynthesizerOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpeechSynthesizerOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechSynthesis.SpeechSynthesizerOptions;{a0e23871-cc3d-43c9-91b1-ee185324d83d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpeechSynthesizerOptions {
    type Vtable = ISpeechSynthesizerOptions_Vtbl;
    const IID: ::windows_core::GUID = <ISpeechSynthesizerOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpeechSynthesizerOptions {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.SpeechSynthesizerOptions";
}
impl ::core::convert::From<SpeechSynthesizerOptions> for ::windows_core::IUnknown {
    fn from(value: SpeechSynthesizerOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechSynthesizerOptions> for ::windows_core::IUnknown {
    fn from(value: &SpeechSynthesizerOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpeechSynthesizerOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpeechSynthesizerOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpeechSynthesizerOptions> for ::windows_core::IInspectable {
    fn from(value: SpeechSynthesizerOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechSynthesizerOptions> for ::windows_core::IInspectable {
    fn from(value: &SpeechSynthesizerOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpeechSynthesizerOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpeechSynthesizerOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpeechSynthesizerOptions {}
unsafe impl ::core::marker::Sync for SpeechSynthesizerOptions {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VoiceGender(pub i32);
impl VoiceGender {
    pub const Male: Self = Self(0i32);
    pub const Female: Self = Self(1i32);
}
impl ::core::marker::Copy for VoiceGender {}
impl ::core::clone::Clone for VoiceGender {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VoiceGender {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for VoiceGender {
    type Abi = Self;
}
impl ::core::fmt::Debug for VoiceGender {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceGender").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VoiceGender {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechSynthesis.VoiceGender;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct VoiceInformation(::windows_core::IUnknown);
impl VoiceInformation {
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Gender(&self) -> ::windows_core::Result<VoiceGender> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<VoiceGender>::zeroed();
            (::windows_core::Interface::vtable(this).Gender)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<VoiceGender>(result__)
        }
    }
}
impl ::core::clone::Clone for VoiceInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VoiceInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoiceInformation {}
impl ::core::fmt::Debug for VoiceInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VoiceInformation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechSynthesis.VoiceInformation;{b127d6a4-1291-4604-aa9c-83134083352c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for VoiceInformation {
    type Vtable = IVoiceInformation_Vtbl;
    const IID: ::windows_core::GUID = <IVoiceInformation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VoiceInformation {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.VoiceInformation";
}
impl ::core::convert::From<VoiceInformation> for ::windows_core::IUnknown {
    fn from(value: VoiceInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoiceInformation> for ::windows_core::IUnknown {
    fn from(value: &VoiceInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for VoiceInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a VoiceInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VoiceInformation> for ::windows_core::IInspectable {
    fn from(value: VoiceInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoiceInformation> for ::windows_core::IInspectable {
    fn from(value: &VoiceInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for VoiceInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a VoiceInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VoiceInformation {}
unsafe impl ::core::marker::Sync for VoiceInformation {}
