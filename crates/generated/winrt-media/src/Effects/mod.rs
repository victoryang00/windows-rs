#[repr(transparent)]
pub struct AudioCaptureEffectsManager(::windows_core::IUnknown);
impl AudioCaptureEffectsManager {
    pub fn AudioCaptureEffectsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AudioCaptureEffectsManager, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AudioCaptureEffectsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAudioCaptureEffectsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAudioCaptureEffectsChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAudioCaptureEffects(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<AudioEffect>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAudioCaptureEffects)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<AudioEffect>>(result__)
        }
    }
}
impl ::core::clone::Clone for AudioCaptureEffectsManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioCaptureEffectsManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioCaptureEffectsManager {}
impl ::core::fmt::Debug for AudioCaptureEffectsManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioCaptureEffectsManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioCaptureEffectsManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.AudioCaptureEffectsManager;{8f85c271-038d-4393-8298-540110608eef})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioCaptureEffectsManager {
    type Vtable = IAudioCaptureEffectsManager_Vtbl;
    const IID: ::windows_core::GUID = <IAudioCaptureEffectsManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioCaptureEffectsManager {
    const NAME: &'static str = "Windows.Media.Effects.AudioCaptureEffectsManager";
}
impl ::core::convert::From<AudioCaptureEffectsManager> for ::windows_core::IUnknown {
    fn from(value: AudioCaptureEffectsManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioCaptureEffectsManager> for ::windows_core::IUnknown {
    fn from(value: &AudioCaptureEffectsManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioCaptureEffectsManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioCaptureEffectsManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioCaptureEffectsManager> for ::windows_core::IInspectable {
    fn from(value: AudioCaptureEffectsManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioCaptureEffectsManager> for ::windows_core::IInspectable {
    fn from(value: &AudioCaptureEffectsManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioCaptureEffectsManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioCaptureEffectsManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioCaptureEffectsManager {}
unsafe impl ::core::marker::Sync for AudioCaptureEffectsManager {}
#[repr(transparent)]
pub struct AudioEffect(::windows_core::IUnknown);
impl AudioEffect {
    pub fn AudioEffectType(&self) -> ::windows_core::Result<AudioEffectType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AudioEffectType>::zeroed();
            (::windows_core::Interface::vtable(this).AudioEffectType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioEffectType>(result__)
        }
    }
}
impl ::core::clone::Clone for AudioEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioEffect {}
impl ::core::fmt::Debug for AudioEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioEffect {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.AudioEffect;{34aafa51-9207-4055-be93-6e5734a86ae4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioEffect {
    type Vtable = IAudioEffect_Vtbl;
    const IID: ::windows_core::GUID = <IAudioEffect as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioEffect {
    const NAME: &'static str = "Windows.Media.Effects.AudioEffect";
}
impl ::core::convert::From<AudioEffect> for ::windows_core::IUnknown {
    fn from(value: AudioEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioEffect> for ::windows_core::IUnknown {
    fn from(value: &AudioEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioEffect> for ::windows_core::IInspectable {
    fn from(value: AudioEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioEffect> for ::windows_core::IInspectable {
    fn from(value: &AudioEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioEffect {}
unsafe impl ::core::marker::Sync for AudioEffect {}
#[repr(transparent)]
pub struct AudioEffectDefinition(::windows_core::IUnknown);
impl AudioEffectDefinition {
    pub fn ActivatableClassId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatableClassId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IPropertySet>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(activatableclassid: Param0) -> ::windows_core::Result<AudioEffectDefinition> {
        Self::IAudioEffectDefinitionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), activatableclassid.into_param().abi(), result__.as_mut_ptr()).from_abi::<AudioEffectDefinition>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithProperties<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IPropertySet>>(activatableclassid: Param0, props: Param1) -> ::windows_core::Result<AudioEffectDefinition> {
        Self::IAudioEffectDefinitionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithProperties)(::windows_core::Interface::as_raw(this), activatableclassid.into_param().abi(), props.into_param().abi(), result__.as_mut_ptr()).from_abi::<AudioEffectDefinition>(result__)
        })
    }
    pub fn IAudioEffectDefinitionFactory<R, F: FnOnce(&IAudioEffectDefinitionFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AudioEffectDefinition, IAudioEffectDefinitionFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AudioEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioEffectDefinition {}
impl ::core::fmt::Debug for AudioEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioEffectDefinition").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioEffectDefinition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.AudioEffectDefinition;{e4d7f974-7d80-4f73-9089-e31c9db9c294})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioEffectDefinition {
    type Vtable = IAudioEffectDefinition_Vtbl;
    const IID: ::windows_core::GUID = <IAudioEffectDefinition as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.AudioEffectDefinition";
}
impl ::core::convert::From<AudioEffectDefinition> for ::windows_core::IUnknown {
    fn from(value: AudioEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioEffectDefinition> for ::windows_core::IUnknown {
    fn from(value: &AudioEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioEffectDefinition> for ::windows_core::IInspectable {
    fn from(value: AudioEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioEffectDefinition> for ::windows_core::IInspectable {
    fn from(value: &AudioEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AudioEffectDefinition> for IAudioEffectDefinition {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioEffectDefinition) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioEffectDefinition> for IAudioEffectDefinition {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioEffectDefinition) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioEffectDefinition> for AudioEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioEffectDefinition> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioEffectDefinition> for &AudioEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioEffectDefinition> {
        ::core::convert::TryInto::<IAudioEffectDefinition>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioEffectDefinition {}
unsafe impl ::core::marker::Sync for AudioEffectDefinition {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioEffectType(pub i32);
impl AudioEffectType {
    pub const Other: Self = Self(0i32);
    pub const AcousticEchoCancellation: Self = Self(1i32);
    pub const NoiseSuppression: Self = Self(2i32);
    pub const AutomaticGainControl: Self = Self(3i32);
    pub const BeamForming: Self = Self(4i32);
    pub const ConstantToneRemoval: Self = Self(5i32);
    pub const Equalizer: Self = Self(6i32);
    pub const LoudnessEqualizer: Self = Self(7i32);
    pub const BassBoost: Self = Self(8i32);
    pub const VirtualSurround: Self = Self(9i32);
    pub const VirtualHeadphones: Self = Self(10i32);
    pub const SpeakerFill: Self = Self(11i32);
    pub const RoomCorrection: Self = Self(12i32);
    pub const BassManagement: Self = Self(13i32);
    pub const EnvironmentalEffects: Self = Self(14i32);
    pub const SpeakerProtection: Self = Self(15i32);
    pub const SpeakerCompensation: Self = Self(16i32);
    pub const DynamicRangeCompression: Self = Self(17i32);
    pub const FarFieldBeamForming: Self = Self(18i32);
    pub const DeepNoiseSuppression: Self = Self(19i32);
}
impl ::core::marker::Copy for AudioEffectType {}
impl ::core::clone::Clone for AudioEffectType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioEffectType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AudioEffectType {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioEffectType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioEffectType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioEffectType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Effects.AudioEffectType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct AudioEffectsManager;
impl AudioEffectsManager {
    #[cfg(feature = "Media_Render")]
    pub fn CreateAudioRenderEffectsManager<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0, category: super::Render::AudioRenderCategory) -> ::windows_core::Result<AudioRenderEffectsManager> {
        Self::IAudioEffectsManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAudioRenderEffectsManager)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), category, result__.as_mut_ptr()).from_abi::<AudioRenderEffectsManager>(result__)
        })
    }
    #[cfg(feature = "Media_Render")]
    pub fn CreateAudioRenderEffectsManagerWithMode<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0, category: super::Render::AudioRenderCategory, mode: super::AudioProcessing) -> ::windows_core::Result<AudioRenderEffectsManager> {
        Self::IAudioEffectsManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAudioRenderEffectsManagerWithMode)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), category, mode, result__.as_mut_ptr()).from_abi::<AudioRenderEffectsManager>(result__)
        })
    }
    #[cfg(feature = "Media_Capture")]
    pub fn CreateAudioCaptureEffectsManager<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0, category: super::Capture::MediaCategory) -> ::windows_core::Result<AudioCaptureEffectsManager> {
        Self::IAudioEffectsManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAudioCaptureEffectsManager)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), category, result__.as_mut_ptr()).from_abi::<AudioCaptureEffectsManager>(result__)
        })
    }
    #[cfg(feature = "Media_Capture")]
    pub fn CreateAudioCaptureEffectsManagerWithMode<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0, category: super::Capture::MediaCategory, mode: super::AudioProcessing) -> ::windows_core::Result<AudioCaptureEffectsManager> {
        Self::IAudioEffectsManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAudioCaptureEffectsManagerWithMode)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), category, mode, result__.as_mut_ptr()).from_abi::<AudioCaptureEffectsManager>(result__)
        })
    }
    pub fn IAudioEffectsManagerStatics<R, F: FnOnce(&IAudioEffectsManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AudioEffectsManager, IAudioEffectsManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for AudioEffectsManager {
    const NAME: &'static str = "Windows.Media.Effects.AudioEffectsManager";
}
#[repr(transparent)]
pub struct AudioRenderEffectsManager(::windows_core::IUnknown);
impl AudioRenderEffectsManager {
    pub fn AudioRenderEffectsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AudioRenderEffectsManager, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AudioRenderEffectsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAudioRenderEffectsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAudioRenderEffectsChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAudioRenderEffects(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<AudioEffect>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAudioRenderEffects)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<AudioEffect>>(result__)
        }
    }
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn EffectsProviderThumbnail(&self) -> ::windows_core::Result<::winrt_storage::Streams::IRandomAccessStreamWithContentType> {
        let this = &::windows_core::Interface::cast::<IAudioRenderEffectsManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EffectsProviderThumbnail)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IRandomAccessStreamWithContentType>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn EffectsProviderSettingsLabel(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAudioRenderEffectsManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).EffectsProviderSettingsLabel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ShowSettingsUI(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioRenderEffectsManager2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ShowSettingsUI)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for AudioRenderEffectsManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioRenderEffectsManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioRenderEffectsManager {}
impl ::core::fmt::Debug for AudioRenderEffectsManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioRenderEffectsManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioRenderEffectsManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.AudioRenderEffectsManager;{4dc98966-8751-42b2-bfcb-39ca7864bd47})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioRenderEffectsManager {
    type Vtable = IAudioRenderEffectsManager_Vtbl;
    const IID: ::windows_core::GUID = <IAudioRenderEffectsManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioRenderEffectsManager {
    const NAME: &'static str = "Windows.Media.Effects.AudioRenderEffectsManager";
}
impl ::core::convert::From<AudioRenderEffectsManager> for ::windows_core::IUnknown {
    fn from(value: AudioRenderEffectsManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioRenderEffectsManager> for ::windows_core::IUnknown {
    fn from(value: &AudioRenderEffectsManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioRenderEffectsManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioRenderEffectsManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioRenderEffectsManager> for ::windows_core::IInspectable {
    fn from(value: AudioRenderEffectsManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioRenderEffectsManager> for ::windows_core::IInspectable {
    fn from(value: &AudioRenderEffectsManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioRenderEffectsManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioRenderEffectsManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioRenderEffectsManager {}
unsafe impl ::core::marker::Sync for AudioRenderEffectsManager {}
#[repr(transparent)]
pub struct CompositeVideoFrameContext(::windows_core::IUnknown);
impl CompositeVideoFrameContext {
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11"))]
    pub fn SurfacesToOverlay(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_graphics::DirectX::Direct3D11::IDirect3DSurface>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SurfacesToOverlay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_graphics::DirectX::Direct3D11::IDirect3DSurface>>(result__)
        }
    }
    pub fn BackgroundFrame(&self) -> ::windows_core::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BackgroundFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::VideoFrame>(result__)
        }
    }
    pub fn OutputFrame(&self) -> ::windows_core::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OutputFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::VideoFrame>(result__)
        }
    }
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_Editing"))]
    pub fn GetOverlayForSurface<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::DirectX::Direct3D11::IDirect3DSurface>>(&self, surfacetooverlay: Param0) -> ::windows_core::Result<super::Editing::MediaOverlay> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetOverlayForSurface)(::windows_core::Interface::as_raw(this), surfacetooverlay.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Editing::MediaOverlay>(result__)
        }
    }
}
impl ::core::clone::Clone for CompositeVideoFrameContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CompositeVideoFrameContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompositeVideoFrameContext {}
impl ::core::fmt::Debug for CompositeVideoFrameContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompositeVideoFrameContext").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CompositeVideoFrameContext {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.CompositeVideoFrameContext;{6c30024b-f514-4278-a5f7-b9188049d110})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CompositeVideoFrameContext {
    type Vtable = ICompositeVideoFrameContext_Vtbl;
    const IID: ::windows_core::GUID = <ICompositeVideoFrameContext as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CompositeVideoFrameContext {
    const NAME: &'static str = "Windows.Media.Effects.CompositeVideoFrameContext";
}
impl ::core::convert::From<CompositeVideoFrameContext> for ::windows_core::IUnknown {
    fn from(value: CompositeVideoFrameContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompositeVideoFrameContext> for ::windows_core::IUnknown {
    fn from(value: &CompositeVideoFrameContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CompositeVideoFrameContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CompositeVideoFrameContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CompositeVideoFrameContext> for ::windows_core::IInspectable {
    fn from(value: CompositeVideoFrameContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompositeVideoFrameContext> for ::windows_core::IInspectable {
    fn from(value: &CompositeVideoFrameContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CompositeVideoFrameContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CompositeVideoFrameContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CompositeVideoFrameContext {}
unsafe impl ::core::marker::Sync for CompositeVideoFrameContext {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioCaptureEffectsManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioCaptureEffectsManager {
    type Vtable = IAudioCaptureEffectsManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8f85c271_038d_4393_8298_540110608eef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioCaptureEffectsManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AudioCaptureEffectsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveAudioCaptureEffectsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAudioCaptureEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAudioCaptureEffects: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioEffect(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioEffect {
    type Vtable = IAudioEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x34aafa51_9207_4055_be93_6e5734a86ae4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEffect_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AudioEffectType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioEffectType) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAudioEffectDefinition(::windows_core::IUnknown);
impl IAudioEffectDefinition {
    pub fn ActivatableClassId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatableClassId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IPropertySet>(result__)
        }
    }
}
impl ::core::convert::From<IAudioEffectDefinition> for ::windows_core::IUnknown {
    fn from(value: IAudioEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioEffectDefinition> for ::windows_core::IUnknown {
    fn from(value: &IAudioEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAudioEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAudioEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAudioEffectDefinition> for ::windows_core::IInspectable {
    fn from(value: IAudioEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioEffectDefinition> for ::windows_core::IInspectable {
    fn from(value: &IAudioEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IAudioEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IAudioEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEffectDefinition {}
impl ::core::fmt::Debug for IAudioEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEffectDefinition").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IAudioEffectDefinition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{e4d7f974-7d80-4f73-9089-e31c9db9c294}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IAudioEffectDefinition {
    type Vtable = IAudioEffectDefinition_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe4d7f974_7d80_4f73_9089_e31c9db9c294);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEffectDefinition_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ActivatableClassId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioEffectDefinitionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioEffectDefinitionFactory {
    type Vtable = IAudioEffectDefinitionFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8e1da646_e705_45ed_8a2b_fc4e4f405a97);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEffectDefinitionFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, props: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioEffectsManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioEffectsManagerStatics {
    type Vtable = IAudioEffectsManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x66406c04_86fa_47cc_a315_f489d8c3fe10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEffectsManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Media_Render")]
    pub CreateAudioRenderEffectsManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, category: super::Render::AudioRenderCategory, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Render"))]
    CreateAudioRenderEffectsManager: usize,
    #[cfg(feature = "Media_Render")]
    pub CreateAudioRenderEffectsManagerWithMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, category: super::Render::AudioRenderCategory, mode: super::AudioProcessing, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Render"))]
    CreateAudioRenderEffectsManagerWithMode: usize,
    #[cfg(feature = "Media_Capture")]
    pub CreateAudioCaptureEffectsManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, category: super::Capture::MediaCategory, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    CreateAudioCaptureEffectsManager: usize,
    #[cfg(feature = "Media_Capture")]
    pub CreateAudioCaptureEffectsManagerWithMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, category: super::Capture::MediaCategory, mode: super::AudioProcessing, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    CreateAudioCaptureEffectsManagerWithMode: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioRenderEffectsManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioRenderEffectsManager {
    type Vtable = IAudioRenderEffectsManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4dc98966_8751_42b2_bfcb_39ca7864bd47);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioRenderEffectsManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AudioRenderEffectsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveAudioRenderEffectsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAudioRenderEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAudioRenderEffects: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAudioRenderEffectsManager2(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAudioRenderEffectsManager2 {
    type Vtable = IAudioRenderEffectsManager2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa844cd09_5ecc_44b3_bb4e_1db07287139c);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAudioRenderEffectsManager2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub EffectsProviderThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    EffectsProviderThumbnail: usize,
    #[cfg(feature = "deprecated")]
    pub EffectsProviderSettingsLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    EffectsProviderSettingsLabel: usize,
    #[cfg(feature = "deprecated")]
    pub ShowSettingsUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ShowSettingsUI: usize,
}
#[repr(transparent)]
pub struct IBasicAudioEffect(::windows_core::IUnknown);
impl IBasicAudioEffect {
    pub fn UseInputFrameForOutput(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).UseInputFrameForOutput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub fn SupportedEncodingProperties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<super::MediaProperties::AudioEncodingProperties>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedEncodingProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<super::MediaProperties::AudioEncodingProperties>>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetEncodingProperties<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>>(&self, encodingproperties: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEncodingProperties)(::windows_core::Interface::as_raw(this), encodingproperties.into_param().abi()).ok() }
    }
    pub fn ProcessFrame<'a, Param0: ::windows_core::IntoParam<'a, ProcessAudioFrameContext>>(&self, context: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ProcessFrame)(::windows_core::Interface::as_raw(this), context.into_param().abi()).ok() }
    }
    pub fn Close(&self, reason: MediaEffectClosedReason) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this), reason).ok() }
    }
    pub fn DiscardQueuedFrames(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).DiscardQueuedFrames)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetProperties<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IPropertySet>>(&self, configuration: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IMediaExtension>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetProperties)(::windows_core::Interface::as_raw(this), configuration.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IBasicAudioEffect> for ::windows_core::IUnknown {
    fn from(value: IBasicAudioEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBasicAudioEffect> for ::windows_core::IUnknown {
    fn from(value: &IBasicAudioEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IBasicAudioEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IBasicAudioEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBasicAudioEffect> for ::windows_core::IInspectable {
    fn from(value: IBasicAudioEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBasicAudioEffect> for ::windows_core::IInspectable {
    fn from(value: &IBasicAudioEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IBasicAudioEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IBasicAudioEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IBasicAudioEffect> for super::IMediaExtension {
    type Error = ::windows_core::Error;
    fn try_from(value: IBasicAudioEffect) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IBasicAudioEffect> for super::IMediaExtension {
    type Error = ::windows_core::Error;
    fn try_from(value: &IBasicAudioEffect) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaExtension> for IBasicAudioEffect {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaExtension> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaExtension> for &IBasicAudioEffect {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaExtension> {
        ::core::convert::TryInto::<super::IMediaExtension>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IBasicAudioEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBasicAudioEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBasicAudioEffect {}
impl ::core::fmt::Debug for IBasicAudioEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBasicAudioEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IBasicAudioEffect {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{8c062c53-6bc0-48b8-a99a-4b41550f1359}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IBasicAudioEffect {
    type Vtable = IBasicAudioEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8c062c53_6bc0_48b8_a99a_4b41550f1359);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBasicAudioEffect_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub UseInputFrameForOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub SupportedEncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))]
    SupportedEncodingProperties: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetEncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetEncodingProperties: usize,
    pub ProcessFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: MediaEffectClosedReason) -> ::windows_core::HRESULT,
    pub DiscardQueuedFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IBasicVideoEffect(::windows_core::IUnknown);
impl IBasicVideoEffect {
    pub fn IsReadOnly(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsReadOnly)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SupportedMemoryTypes(&self) -> ::windows_core::Result<MediaMemoryTypes> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaMemoryTypes>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedMemoryTypes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaMemoryTypes>(result__)
        }
    }
    pub fn TimeIndependent(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TimeIndependent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub fn SupportedEncodingProperties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<super::MediaProperties::VideoEncodingProperties>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedEncodingProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<super::MediaProperties::VideoEncodingProperties>>(result__)
        }
    }
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
    pub fn SetEncodingProperties<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProperties::VideoEncodingProperties>, Param1: ::windows_core::IntoParam<'a, ::winrt_graphics::DirectX::Direct3D11::IDirect3DDevice>>(&self, encodingproperties: Param0, device: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEncodingProperties)(::windows_core::Interface::as_raw(this), encodingproperties.into_param().abi(), device.into_param().abi()).ok() }
    }
    pub fn ProcessFrame<'a, Param0: ::windows_core::IntoParam<'a, ProcessVideoFrameContext>>(&self, context: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ProcessFrame)(::windows_core::Interface::as_raw(this), context.into_param().abi()).ok() }
    }
    pub fn Close(&self, reason: MediaEffectClosedReason) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this), reason).ok() }
    }
    pub fn DiscardQueuedFrames(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).DiscardQueuedFrames)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetProperties<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IPropertySet>>(&self, configuration: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IMediaExtension>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetProperties)(::windows_core::Interface::as_raw(this), configuration.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IBasicVideoEffect> for ::windows_core::IUnknown {
    fn from(value: IBasicVideoEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBasicVideoEffect> for ::windows_core::IUnknown {
    fn from(value: &IBasicVideoEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IBasicVideoEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IBasicVideoEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBasicVideoEffect> for ::windows_core::IInspectable {
    fn from(value: IBasicVideoEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBasicVideoEffect> for ::windows_core::IInspectable {
    fn from(value: &IBasicVideoEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IBasicVideoEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IBasicVideoEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IBasicVideoEffect> for super::IMediaExtension {
    type Error = ::windows_core::Error;
    fn try_from(value: IBasicVideoEffect) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IBasicVideoEffect> for super::IMediaExtension {
    type Error = ::windows_core::Error;
    fn try_from(value: &IBasicVideoEffect) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaExtension> for IBasicVideoEffect {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaExtension> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaExtension> for &IBasicVideoEffect {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaExtension> {
        ::core::convert::TryInto::<super::IMediaExtension>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IBasicVideoEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBasicVideoEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBasicVideoEffect {}
impl ::core::fmt::Debug for IBasicVideoEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBasicVideoEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IBasicVideoEffect {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{8262c7ef-b360-40be-949b-2ff42ff35693}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IBasicVideoEffect {
    type Vtable = IBasicVideoEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8262c7ef_b360_40be_949b_2ff42ff35693);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBasicVideoEffect_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SupportedMemoryTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaMemoryTypes) -> ::windows_core::HRESULT,
    pub TimeIndependent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub SupportedEncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))]
    SupportedEncodingProperties: usize,
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
    pub SetEncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: ::windows_core::RawPtr, device: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties")))]
    SetEncodingProperties: usize,
    pub ProcessFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: MediaEffectClosedReason) -> ::windows_core::HRESULT,
    pub DiscardQueuedFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositeVideoFrameContext(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompositeVideoFrameContext {
    type Vtable = ICompositeVideoFrameContext_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6c30024b_f514_4278_a5f7_b9188049d110);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositeVideoFrameContext_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11"))]
    pub SurfacesToOverlay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11")))]
    SurfacesToOverlay: usize,
    pub BackgroundFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OutputFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_Editing"))]
    pub GetOverlayForSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, surfacetooverlay: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_Editing")))]
    GetOverlayForSurface: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProcessAudioFrameContext(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProcessAudioFrameContext {
    type Vtable = IProcessAudioFrameContext_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4cd92946_1222_4a27_a586_fb3e20273255);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessAudioFrameContext_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InputFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OutputFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProcessVideoFrameContext(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProcessVideoFrameContext {
    type Vtable = IProcessVideoFrameContext_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x276f0e2b_6461_401e_ba78_0fdad6114eec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessVideoFrameContext_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InputFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OutputFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISlowMotionEffectDefinition(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISlowMotionEffectDefinition {
    type Vtable = ISlowMotionEffectDefinition_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x35053cd0_176c_4763_82c4_1b02dbe31737);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISlowMotionEffectDefinition_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TimeStretchRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetTimeStretchRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IVideoCompositor(::windows_core::IUnknown);
impl IVideoCompositor {
    pub fn TimeIndependent(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TimeIndependent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
    pub fn SetEncodingProperties<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProperties::VideoEncodingProperties>, Param1: ::windows_core::IntoParam<'a, ::winrt_graphics::DirectX::Direct3D11::IDirect3DDevice>>(&self, backgroundproperties: Param0, device: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEncodingProperties)(::windows_core::Interface::as_raw(this), backgroundproperties.into_param().abi(), device.into_param().abi()).ok() }
    }
    pub fn CompositeFrame<'a, Param0: ::windows_core::IntoParam<'a, CompositeVideoFrameContext>>(&self, context: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).CompositeFrame)(::windows_core::Interface::as_raw(this), context.into_param().abi()).ok() }
    }
    pub fn Close(&self, reason: MediaEffectClosedReason) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this), reason).ok() }
    }
    pub fn DiscardQueuedFrames(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).DiscardQueuedFrames)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetProperties<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IPropertySet>>(&self, configuration: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IMediaExtension>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetProperties)(::windows_core::Interface::as_raw(this), configuration.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IVideoCompositor> for ::windows_core::IUnknown {
    fn from(value: IVideoCompositor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVideoCompositor> for ::windows_core::IUnknown {
    fn from(value: &IVideoCompositor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IVideoCompositor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IVideoCompositor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IVideoCompositor> for ::windows_core::IInspectable {
    fn from(value: IVideoCompositor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVideoCompositor> for ::windows_core::IInspectable {
    fn from(value: &IVideoCompositor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IVideoCompositor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IVideoCompositor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IVideoCompositor> for super::IMediaExtension {
    type Error = ::windows_core::Error;
    fn try_from(value: IVideoCompositor) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IVideoCompositor> for super::IMediaExtension {
    type Error = ::windows_core::Error;
    fn try_from(value: &IVideoCompositor) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaExtension> for IVideoCompositor {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaExtension> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaExtension> for &IVideoCompositor {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaExtension> {
        ::core::convert::TryInto::<super::IMediaExtension>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IVideoCompositor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVideoCompositor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVideoCompositor {}
impl ::core::fmt::Debug for IVideoCompositor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVideoCompositor").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IVideoCompositor {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{8510b43e-420c-420f-96c7-7c98bba1fc55}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IVideoCompositor {
    type Vtable = IVideoCompositor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8510b43e_420c_420f_96c7_7c98bba1fc55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoCompositor_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TimeIndependent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
    pub SetEncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, backgroundproperties: ::windows_core::RawPtr, device: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties")))]
    SetEncodingProperties: usize,
    pub CompositeFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: MediaEffectClosedReason) -> ::windows_core::HRESULT,
    pub DiscardQueuedFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IVideoCompositorDefinition(::windows_core::IUnknown);
impl IVideoCompositorDefinition {
    pub fn ActivatableClassId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatableClassId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IPropertySet>(result__)
        }
    }
}
impl ::core::convert::From<IVideoCompositorDefinition> for ::windows_core::IUnknown {
    fn from(value: IVideoCompositorDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVideoCompositorDefinition> for ::windows_core::IUnknown {
    fn from(value: &IVideoCompositorDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IVideoCompositorDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IVideoCompositorDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IVideoCompositorDefinition> for ::windows_core::IInspectable {
    fn from(value: IVideoCompositorDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVideoCompositorDefinition> for ::windows_core::IInspectable {
    fn from(value: &IVideoCompositorDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IVideoCompositorDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IVideoCompositorDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVideoCompositorDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVideoCompositorDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVideoCompositorDefinition {}
impl ::core::fmt::Debug for IVideoCompositorDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVideoCompositorDefinition").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IVideoCompositorDefinition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{7946b8d0-2010-4ae3-9ab2-2cef42edd4d2}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IVideoCompositorDefinition {
    type Vtable = IVideoCompositorDefinition_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7946b8d0_2010_4ae3_9ab2_2cef42edd4d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoCompositorDefinition_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ActivatableClassId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoCompositorDefinitionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoCompositorDefinitionFactory {
    type Vtable = IVideoCompositorDefinitionFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4366fd10_68b8_4d52_89b6_02a968cca899);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoCompositorDefinitionFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, props: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithProperties: usize,
}
#[repr(transparent)]
pub struct IVideoEffectDefinition(::windows_core::IUnknown);
impl IVideoEffectDefinition {
    pub fn ActivatableClassId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatableClassId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IPropertySet>(result__)
        }
    }
}
impl ::core::convert::From<IVideoEffectDefinition> for ::windows_core::IUnknown {
    fn from(value: IVideoEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVideoEffectDefinition> for ::windows_core::IUnknown {
    fn from(value: &IVideoEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IVideoEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IVideoEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IVideoEffectDefinition> for ::windows_core::IInspectable {
    fn from(value: IVideoEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVideoEffectDefinition> for ::windows_core::IInspectable {
    fn from(value: &IVideoEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IVideoEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IVideoEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVideoEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVideoEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVideoEffectDefinition {}
impl ::core::fmt::Debug for IVideoEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVideoEffectDefinition").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IVideoEffectDefinition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{39f38cf0-8d0f-4f3e-84fc-2d46a5297943}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IVideoEffectDefinition {
    type Vtable = IVideoEffectDefinition_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x39f38cf0_8d0f_4f3e_84fc_2d46a5297943);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEffectDefinition_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ActivatableClassId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoEffectDefinitionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoEffectDefinitionFactory {
    type Vtable = IVideoEffectDefinitionFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x81439b4e_6e33_428f_9d21_b5aafef7617c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEffectDefinitionFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, props: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoTransformEffectDefinition(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoTransformEffectDefinition {
    type Vtable = IVideoTransformEffectDefinition_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9664bb6a_1ea6_4aa6_8074_abe8851ecae2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoTransformEffectDefinition_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "UI")]
    pub PaddingColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    PaddingColor: usize,
    #[cfg(feature = "UI")]
    pub SetPaddingColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetPaddingColor: usize,
    pub OutputSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Size) -> ::windows_core::HRESULT,
    pub SetOutputSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Size) -> ::windows_core::HRESULT,
    pub CropRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    pub SetCropRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub Rotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::MediaRotation) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    Rotation: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::MediaProperties::MediaRotation) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetRotation: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub Mirror: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::MediaMirroringOptions) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    Mirror: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetMirror: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::MediaProperties::MediaMirroringOptions) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetMirror: usize,
    #[cfg(feature = "Media_Transcoding")]
    pub SetProcessingAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Transcoding::MediaVideoProcessingAlgorithm) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Transcoding"))]
    SetProcessingAlgorithm: usize,
    #[cfg(feature = "Media_Transcoding")]
    pub ProcessingAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Transcoding::MediaVideoProcessingAlgorithm) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Transcoding"))]
    ProcessingAlgorithm: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoTransformEffectDefinition2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoTransformEffectDefinition2 {
    type Vtable = IVideoTransformEffectDefinition2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf0a8089f_66c8_4694_9fd9_1136abf7444a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoTransformEffectDefinition2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SphericalProjection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoTransformSphericalProjection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoTransformSphericalProjection {
    type Vtable = IVideoTransformSphericalProjection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcf4401f0_9bf2_4c39_9f41_e022514a8468);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoTransformSphericalProjection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub FrameFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::SphericalVideoFrameFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    FrameFormat: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetFrameFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::MediaProperties::SphericalVideoFrameFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetFrameFormat: usize,
    #[cfg(feature = "Media_Playback")]
    pub ProjectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Playback::SphericalVideoProjectionMode) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))]
    ProjectionMode: usize,
    #[cfg(feature = "Media_Playback")]
    pub SetProjectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Playback::SphericalVideoProjectionMode) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))]
    SetProjectionMode: usize,
    pub HorizontalFieldOfViewInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetHorizontalFieldOfViewInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub ViewOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Quaternion) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ViewOrientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetViewOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Numerics::Quaternion) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetViewOrientation: usize,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaEffectClosedReason(pub i32);
impl MediaEffectClosedReason {
    pub const Done: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const UnsupportedEncodingFormat: Self = Self(2i32);
    pub const EffectCurrentlyUnloaded: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaEffectClosedReason {}
impl ::core::clone::Clone for MediaEffectClosedReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaEffectClosedReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaEffectClosedReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaEffectClosedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaEffectClosedReason").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaEffectClosedReason {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Effects.MediaEffectClosedReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaMemoryTypes(pub i32);
impl MediaMemoryTypes {
    pub const Gpu: Self = Self(0i32);
    pub const Cpu: Self = Self(1i32);
    pub const GpuAndCpu: Self = Self(2i32);
}
impl ::core::marker::Copy for MediaMemoryTypes {}
impl ::core::clone::Clone for MediaMemoryTypes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaMemoryTypes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaMemoryTypes {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaMemoryTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaMemoryTypes").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaMemoryTypes {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Effects.MediaMemoryTypes;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ProcessAudioFrameContext(::windows_core::IUnknown);
impl ProcessAudioFrameContext {
    pub fn InputFrame(&self) -> ::windows_core::Result<super::AudioFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InputFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::AudioFrame>(result__)
        }
    }
    pub fn OutputFrame(&self) -> ::windows_core::Result<super::AudioFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OutputFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::AudioFrame>(result__)
        }
    }
}
impl ::core::clone::Clone for ProcessAudioFrameContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProcessAudioFrameContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProcessAudioFrameContext {}
impl ::core::fmt::Debug for ProcessAudioFrameContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessAudioFrameContext").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProcessAudioFrameContext {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.ProcessAudioFrameContext;{4cd92946-1222-4a27-a586-fb3e20273255})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProcessAudioFrameContext {
    type Vtable = IProcessAudioFrameContext_Vtbl;
    const IID: ::windows_core::GUID = <IProcessAudioFrameContext as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProcessAudioFrameContext {
    const NAME: &'static str = "Windows.Media.Effects.ProcessAudioFrameContext";
}
impl ::core::convert::From<ProcessAudioFrameContext> for ::windows_core::IUnknown {
    fn from(value: ProcessAudioFrameContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessAudioFrameContext> for ::windows_core::IUnknown {
    fn from(value: &ProcessAudioFrameContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProcessAudioFrameContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProcessAudioFrameContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProcessAudioFrameContext> for ::windows_core::IInspectable {
    fn from(value: ProcessAudioFrameContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessAudioFrameContext> for ::windows_core::IInspectable {
    fn from(value: &ProcessAudioFrameContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProcessAudioFrameContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProcessAudioFrameContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProcessAudioFrameContext {}
unsafe impl ::core::marker::Sync for ProcessAudioFrameContext {}
#[repr(transparent)]
pub struct ProcessVideoFrameContext(::windows_core::IUnknown);
impl ProcessVideoFrameContext {
    pub fn InputFrame(&self) -> ::windows_core::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InputFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::VideoFrame>(result__)
        }
    }
    pub fn OutputFrame(&self) -> ::windows_core::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OutputFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::VideoFrame>(result__)
        }
    }
}
impl ::core::clone::Clone for ProcessVideoFrameContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProcessVideoFrameContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProcessVideoFrameContext {}
impl ::core::fmt::Debug for ProcessVideoFrameContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessVideoFrameContext").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProcessVideoFrameContext {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.ProcessVideoFrameContext;{276f0e2b-6461-401e-ba78-0fdad6114eec})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProcessVideoFrameContext {
    type Vtable = IProcessVideoFrameContext_Vtbl;
    const IID: ::windows_core::GUID = <IProcessVideoFrameContext as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProcessVideoFrameContext {
    const NAME: &'static str = "Windows.Media.Effects.ProcessVideoFrameContext";
}
impl ::core::convert::From<ProcessVideoFrameContext> for ::windows_core::IUnknown {
    fn from(value: ProcessVideoFrameContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessVideoFrameContext> for ::windows_core::IUnknown {
    fn from(value: &ProcessVideoFrameContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProcessVideoFrameContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProcessVideoFrameContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProcessVideoFrameContext> for ::windows_core::IInspectable {
    fn from(value: ProcessVideoFrameContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessVideoFrameContext> for ::windows_core::IInspectable {
    fn from(value: &ProcessVideoFrameContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProcessVideoFrameContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProcessVideoFrameContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProcessVideoFrameContext {}
unsafe impl ::core::marker::Sync for ProcessVideoFrameContext {}
#[repr(transparent)]
pub struct SlowMotionEffectDefinition(::windows_core::IUnknown);
impl SlowMotionEffectDefinition {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SlowMotionEffectDefinition, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TimeStretchRate(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).TimeStretchRate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetTimeStretchRate(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTimeStretchRate)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ActivatableClassId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IVideoEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatableClassId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IPropertySet> {
        let this = &::windows_core::Interface::cast::<IVideoEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IPropertySet>(result__)
        }
    }
}
impl ::core::clone::Clone for SlowMotionEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SlowMotionEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SlowMotionEffectDefinition {}
impl ::core::fmt::Debug for SlowMotionEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SlowMotionEffectDefinition").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SlowMotionEffectDefinition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.SlowMotionEffectDefinition;{35053cd0-176c-4763-82c4-1b02dbe31737})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SlowMotionEffectDefinition {
    type Vtable = ISlowMotionEffectDefinition_Vtbl;
    const IID: ::windows_core::GUID = <ISlowMotionEffectDefinition as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SlowMotionEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.SlowMotionEffectDefinition";
}
impl ::core::convert::From<SlowMotionEffectDefinition> for ::windows_core::IUnknown {
    fn from(value: SlowMotionEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SlowMotionEffectDefinition> for ::windows_core::IUnknown {
    fn from(value: &SlowMotionEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SlowMotionEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SlowMotionEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SlowMotionEffectDefinition> for ::windows_core::IInspectable {
    fn from(value: SlowMotionEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SlowMotionEffectDefinition> for ::windows_core::IInspectable {
    fn from(value: &SlowMotionEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SlowMotionEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SlowMotionEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SlowMotionEffectDefinition> for IVideoEffectDefinition {
    type Error = ::windows_core::Error;
    fn try_from(value: SlowMotionEffectDefinition) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SlowMotionEffectDefinition> for IVideoEffectDefinition {
    type Error = ::windows_core::Error;
    fn try_from(value: &SlowMotionEffectDefinition) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IVideoEffectDefinition> for SlowMotionEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, IVideoEffectDefinition> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IVideoEffectDefinition> for &SlowMotionEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, IVideoEffectDefinition> {
        ::core::convert::TryInto::<IVideoEffectDefinition>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SlowMotionEffectDefinition {}
unsafe impl ::core::marker::Sync for SlowMotionEffectDefinition {}
#[repr(transparent)]
pub struct VideoCompositorDefinition(::windows_core::IUnknown);
impl VideoCompositorDefinition {
    pub fn ActivatableClassId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatableClassId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IPropertySet>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(activatableclassid: Param0) -> ::windows_core::Result<VideoCompositorDefinition> {
        Self::IVideoCompositorDefinitionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), activatableclassid.into_param().abi(), result__.as_mut_ptr()).from_abi::<VideoCompositorDefinition>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithProperties<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IPropertySet>>(activatableclassid: Param0, props: Param1) -> ::windows_core::Result<VideoCompositorDefinition> {
        Self::IVideoCompositorDefinitionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithProperties)(::windows_core::Interface::as_raw(this), activatableclassid.into_param().abi(), props.into_param().abi(), result__.as_mut_ptr()).from_abi::<VideoCompositorDefinition>(result__)
        })
    }
    pub fn IVideoCompositorDefinitionFactory<R, F: FnOnce(&IVideoCompositorDefinitionFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<VideoCompositorDefinition, IVideoCompositorDefinitionFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for VideoCompositorDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VideoCompositorDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoCompositorDefinition {}
impl ::core::fmt::Debug for VideoCompositorDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoCompositorDefinition").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VideoCompositorDefinition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.VideoCompositorDefinition;{7946b8d0-2010-4ae3-9ab2-2cef42edd4d2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for VideoCompositorDefinition {
    type Vtable = IVideoCompositorDefinition_Vtbl;
    const IID: ::windows_core::GUID = <IVideoCompositorDefinition as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VideoCompositorDefinition {
    const NAME: &'static str = "Windows.Media.Effects.VideoCompositorDefinition";
}
impl ::core::convert::From<VideoCompositorDefinition> for ::windows_core::IUnknown {
    fn from(value: VideoCompositorDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoCompositorDefinition> for ::windows_core::IUnknown {
    fn from(value: &VideoCompositorDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for VideoCompositorDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a VideoCompositorDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VideoCompositorDefinition> for ::windows_core::IInspectable {
    fn from(value: VideoCompositorDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoCompositorDefinition> for ::windows_core::IInspectable {
    fn from(value: &VideoCompositorDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for VideoCompositorDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a VideoCompositorDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<VideoCompositorDefinition> for IVideoCompositorDefinition {
    type Error = ::windows_core::Error;
    fn try_from(value: VideoCompositorDefinition) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VideoCompositorDefinition> for IVideoCompositorDefinition {
    type Error = ::windows_core::Error;
    fn try_from(value: &VideoCompositorDefinition) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IVideoCompositorDefinition> for VideoCompositorDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, IVideoCompositorDefinition> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IVideoCompositorDefinition> for &VideoCompositorDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, IVideoCompositorDefinition> {
        ::core::convert::TryInto::<IVideoCompositorDefinition>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for VideoCompositorDefinition {}
unsafe impl ::core::marker::Sync for VideoCompositorDefinition {}
#[repr(transparent)]
pub struct VideoEffectDefinition(::windows_core::IUnknown);
impl VideoEffectDefinition {
    pub fn ActivatableClassId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatableClassId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IPropertySet>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(activatableclassid: Param0) -> ::windows_core::Result<VideoEffectDefinition> {
        Self::IVideoEffectDefinitionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), activatableclassid.into_param().abi(), result__.as_mut_ptr()).from_abi::<VideoEffectDefinition>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithProperties<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IPropertySet>>(activatableclassid: Param0, props: Param1) -> ::windows_core::Result<VideoEffectDefinition> {
        Self::IVideoEffectDefinitionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithProperties)(::windows_core::Interface::as_raw(this), activatableclassid.into_param().abi(), props.into_param().abi(), result__.as_mut_ptr()).from_abi::<VideoEffectDefinition>(result__)
        })
    }
    pub fn IVideoEffectDefinitionFactory<R, F: FnOnce(&IVideoEffectDefinitionFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<VideoEffectDefinition, IVideoEffectDefinitionFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for VideoEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VideoEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoEffectDefinition {}
impl ::core::fmt::Debug for VideoEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoEffectDefinition").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VideoEffectDefinition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.VideoEffectDefinition;{39f38cf0-8d0f-4f3e-84fc-2d46a5297943})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for VideoEffectDefinition {
    type Vtable = IVideoEffectDefinition_Vtbl;
    const IID: ::windows_core::GUID = <IVideoEffectDefinition as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VideoEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.VideoEffectDefinition";
}
impl ::core::convert::From<VideoEffectDefinition> for ::windows_core::IUnknown {
    fn from(value: VideoEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoEffectDefinition> for ::windows_core::IUnknown {
    fn from(value: &VideoEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for VideoEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a VideoEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VideoEffectDefinition> for ::windows_core::IInspectable {
    fn from(value: VideoEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoEffectDefinition> for ::windows_core::IInspectable {
    fn from(value: &VideoEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for VideoEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a VideoEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<VideoEffectDefinition> for IVideoEffectDefinition {
    type Error = ::windows_core::Error;
    fn try_from(value: VideoEffectDefinition) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VideoEffectDefinition> for IVideoEffectDefinition {
    type Error = ::windows_core::Error;
    fn try_from(value: &VideoEffectDefinition) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IVideoEffectDefinition> for VideoEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, IVideoEffectDefinition> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IVideoEffectDefinition> for &VideoEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, IVideoEffectDefinition> {
        ::core::convert::TryInto::<IVideoEffectDefinition>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for VideoEffectDefinition {}
unsafe impl ::core::marker::Sync for VideoEffectDefinition {}
#[repr(transparent)]
pub struct VideoTransformEffectDefinition(::windows_core::IUnknown);
impl VideoTransformEffectDefinition {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<VideoTransformEffectDefinition, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ActivatableClassId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatableClassId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IPropertySet>(result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn PaddingColor(&self) -> ::windows_core::Result<::winrt_ui::Color> {
        let this = &::windows_core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_ui::Color>::zeroed();
            (::windows_core::Interface::vtable(this).PaddingColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Color>(result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn SetPaddingColor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_ui::Color>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPaddingColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn OutputSize(&self) -> ::windows_core::Result<::winrt_foundation::Size> {
        let this = &::windows_core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).OutputSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Size>(result__)
        }
    }
    pub fn SetOutputSize<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Size>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOutputSize)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CropRectangle(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = &::windows_core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).CropRectangle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    pub fn SetCropRectangle<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCropRectangle)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn Rotation(&self) -> ::windows_core::Result<super::MediaProperties::MediaRotation> {
        let this = &::windows_core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::MediaProperties::MediaRotation>::zeroed();
            (::windows_core::Interface::vtable(this).Rotation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::MediaRotation>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetRotation(&self, value: super::MediaProperties::MediaRotation) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRotation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn Mirror(&self) -> ::windows_core::Result<super::MediaProperties::MediaMirroringOptions> {
        let this = &::windows_core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::MediaProperties::MediaMirroringOptions>::zeroed();
            (::windows_core::Interface::vtable(this).Mirror)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::MediaMirroringOptions>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetMirror(&self, value: super::MediaProperties::MediaMirroringOptions) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMirror)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Media_Transcoding")]
    pub fn SetProcessingAlgorithm(&self, value: super::Transcoding::MediaVideoProcessingAlgorithm) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetProcessingAlgorithm)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Media_Transcoding")]
    pub fn ProcessingAlgorithm(&self) -> ::windows_core::Result<super::Transcoding::MediaVideoProcessingAlgorithm> {
        let this = &::windows_core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Transcoding::MediaVideoProcessingAlgorithm>::zeroed();
            (::windows_core::Interface::vtable(this).ProcessingAlgorithm)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Transcoding::MediaVideoProcessingAlgorithm>(result__)
        }
    }
    pub fn SphericalProjection(&self) -> ::windows_core::Result<VideoTransformSphericalProjection> {
        let this = &::windows_core::Interface::cast::<IVideoTransformEffectDefinition2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SphericalProjection)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<VideoTransformSphericalProjection>(result__)
        }
    }
}
impl ::core::clone::Clone for VideoTransformEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VideoTransformEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoTransformEffectDefinition {}
impl ::core::fmt::Debug for VideoTransformEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoTransformEffectDefinition").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VideoTransformEffectDefinition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.VideoTransformEffectDefinition;{39f38cf0-8d0f-4f3e-84fc-2d46a5297943})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for VideoTransformEffectDefinition {
    type Vtable = IVideoEffectDefinition_Vtbl;
    const IID: ::windows_core::GUID = <IVideoEffectDefinition as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VideoTransformEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.VideoTransformEffectDefinition";
}
impl ::core::convert::From<VideoTransformEffectDefinition> for ::windows_core::IUnknown {
    fn from(value: VideoTransformEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoTransformEffectDefinition> for ::windows_core::IUnknown {
    fn from(value: &VideoTransformEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for VideoTransformEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a VideoTransformEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VideoTransformEffectDefinition> for ::windows_core::IInspectable {
    fn from(value: VideoTransformEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoTransformEffectDefinition> for ::windows_core::IInspectable {
    fn from(value: &VideoTransformEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for VideoTransformEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a VideoTransformEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<VideoTransformEffectDefinition> for IVideoEffectDefinition {
    type Error = ::windows_core::Error;
    fn try_from(value: VideoTransformEffectDefinition) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VideoTransformEffectDefinition> for IVideoEffectDefinition {
    type Error = ::windows_core::Error;
    fn try_from(value: &VideoTransformEffectDefinition) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IVideoEffectDefinition> for VideoTransformEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, IVideoEffectDefinition> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IVideoEffectDefinition> for &VideoTransformEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, IVideoEffectDefinition> {
        ::core::convert::TryInto::<IVideoEffectDefinition>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for VideoTransformEffectDefinition {}
unsafe impl ::core::marker::Sync for VideoTransformEffectDefinition {}
#[repr(transparent)]
pub struct VideoTransformSphericalProjection(::windows_core::IUnknown);
impl VideoTransformSphericalProjection {
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
    #[cfg(feature = "Media_MediaProperties")]
    pub fn FrameFormat(&self) -> ::windows_core::Result<super::MediaProperties::SphericalVideoFrameFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::MediaProperties::SphericalVideoFrameFormat>::zeroed();
            (::windows_core::Interface::vtable(this).FrameFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::SphericalVideoFrameFormat>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetFrameFormat(&self, value: super::MediaProperties::SphericalVideoFrameFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFrameFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Media_Playback")]
    pub fn ProjectionMode(&self) -> ::windows_core::Result<super::Playback::SphericalVideoProjectionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Playback::SphericalVideoProjectionMode>::zeroed();
            (::windows_core::Interface::vtable(this).ProjectionMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Playback::SphericalVideoProjectionMode>(result__)
        }
    }
    #[cfg(feature = "Media_Playback")]
    pub fn SetProjectionMode(&self, value: super::Playback::SphericalVideoProjectionMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetProjectionMode)(::windows_core::Interface::as_raw(this), value).ok() }
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
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ViewOrientation(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Quaternion>::zeroed();
            (::windows_core::Interface::vtable(this).ViewOrientation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Quaternion>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetViewOrientation<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Quaternion>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetViewOrientation)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for VideoTransformSphericalProjection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VideoTransformSphericalProjection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoTransformSphericalProjection {}
impl ::core::fmt::Debug for VideoTransformSphericalProjection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoTransformSphericalProjection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VideoTransformSphericalProjection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.VideoTransformSphericalProjection;{cf4401f0-9bf2-4c39-9f41-e022514a8468})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for VideoTransformSphericalProjection {
    type Vtable = IVideoTransformSphericalProjection_Vtbl;
    const IID: ::windows_core::GUID = <IVideoTransformSphericalProjection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VideoTransformSphericalProjection {
    const NAME: &'static str = "Windows.Media.Effects.VideoTransformSphericalProjection";
}
impl ::core::convert::From<VideoTransformSphericalProjection> for ::windows_core::IUnknown {
    fn from(value: VideoTransformSphericalProjection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoTransformSphericalProjection> for ::windows_core::IUnknown {
    fn from(value: &VideoTransformSphericalProjection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for VideoTransformSphericalProjection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a VideoTransformSphericalProjection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VideoTransformSphericalProjection> for ::windows_core::IInspectable {
    fn from(value: VideoTransformSphericalProjection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoTransformSphericalProjection> for ::windows_core::IInspectable {
    fn from(value: &VideoTransformSphericalProjection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for VideoTransformSphericalProjection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a VideoTransformSphericalProjection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VideoTransformSphericalProjection {}
unsafe impl ::core::marker::Sync for VideoTransformSphericalProjection {}
