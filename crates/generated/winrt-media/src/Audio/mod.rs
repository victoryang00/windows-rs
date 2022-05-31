#[repr(transparent)]
pub struct AudioDeviceInputNode(::windows_core::IUnknown);
impl AudioDeviceInputNode {
    #[cfg(feature = "winrt-devices")]
    pub fn Device(&self) -> ::windows_core::Result<::winrt_devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Device)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Enumeration::DeviceInformation>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn OutgoingConnections(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = &::windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OutgoingConnections)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<AudioGraphConnection>>(result__)
        }
    }
    pub fn AddOutgoingConnection<'a, Param0: ::windows_core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AddOutgoingConnection)(::windows_core::Interface::as_raw(this), destination.into_param().abi()).ok() }
    }
    pub fn AddOutgoingConnectionWithGain<'a, Param0: ::windows_core::IntoParam<'a, IAudioNode>>(&self, destination: Param0, gain: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AddOutgoingConnectionWithGain)(::windows_core::Interface::as_raw(this), destination.into_param().abi(), gain).ok() }
    }
    pub fn RemoveOutgoingConnection<'a, Param0: ::windows_core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOutgoingConnection)(::windows_core::Interface::as_raw(this), destination.into_param().abi()).ok() }
    }
    pub fn Emitter(&self) -> ::windows_core::Result<AudioNodeEmitter> {
        let this = &::windows_core::Interface::cast::<IAudioInputNode2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Emitter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioNodeEmitter>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
    pub fn EffectDefinitions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EffectDefinitions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOutgoingGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OutgoingGain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn EncodingProperties(&self) -> ::windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EncodingProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ConsumeInput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetConsumeInput)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Reset)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).DisableEffectsByDefinition)(::windows_core::Interface::as_raw(this), definition.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).EnableEffectsByDefinition)(::windows_core::Interface::as_raw(this), definition.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for AudioDeviceInputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioDeviceInputNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioDeviceInputNode {}
impl ::core::fmt::Debug for AudioDeviceInputNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioDeviceInputNode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioDeviceInputNode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioDeviceInputNode;{b01b6be1-6f4e-49e2-ac01-559d62beb3a9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioDeviceInputNode {
    type Vtable = IAudioDeviceInputNode_Vtbl;
    const IID: ::windows_core::GUID = <IAudioDeviceInputNode as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioDeviceInputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioDeviceInputNode";
}
impl ::core::convert::From<AudioDeviceInputNode> for ::windows_core::IUnknown {
    fn from(value: AudioDeviceInputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioDeviceInputNode> for ::windows_core::IUnknown {
    fn from(value: &AudioDeviceInputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioDeviceInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioDeviceInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioDeviceInputNode> for ::windows_core::IInspectable {
    fn from(value: AudioDeviceInputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioDeviceInputNode> for ::windows_core::IInspectable {
    fn from(value: &AudioDeviceInputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioDeviceInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioDeviceInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AudioDeviceInputNode> for IAudioInputNode {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioDeviceInputNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioDeviceInputNode> for IAudioInputNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioDeviceInputNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioInputNode> for AudioDeviceInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioInputNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioInputNode> for &AudioDeviceInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioInputNode> {
        ::core::convert::TryInto::<IAudioInputNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioDeviceInputNode> for IAudioInputNode2 {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioDeviceInputNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioDeviceInputNode> for IAudioInputNode2 {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioDeviceInputNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioInputNode2> for AudioDeviceInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioInputNode2> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioInputNode2> for &AudioDeviceInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioInputNode2> {
        ::core::convert::TryInto::<IAudioInputNode2>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioDeviceInputNode> for IAudioNode {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioDeviceInputNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioDeviceInputNode> for IAudioNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioDeviceInputNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioNode> for AudioDeviceInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioNode> for &AudioDeviceInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioNode> {
        ::core::convert::TryInto::<IAudioNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioDeviceInputNode> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioDeviceInputNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioDeviceInputNode> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioDeviceInputNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for AudioDeviceInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &AudioDeviceInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioDeviceInputNode {}
unsafe impl ::core::marker::Sync for AudioDeviceInputNode {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioDeviceNodeCreationStatus(pub i32);
impl AudioDeviceNodeCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const DeviceNotAvailable: Self = Self(1i32);
    pub const FormatNotSupported: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
    pub const AccessDenied: Self = Self(4i32);
}
impl ::core::marker::Copy for AudioDeviceNodeCreationStatus {}
impl ::core::clone::Clone for AudioDeviceNodeCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioDeviceNodeCreationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AudioDeviceNodeCreationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioDeviceNodeCreationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioDeviceNodeCreationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioDeviceNodeCreationStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioDeviceNodeCreationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AudioDeviceOutputNode(::windows_core::IUnknown);
impl AudioDeviceOutputNode {
    #[cfg(feature = "winrt-devices")]
    pub fn Device(&self) -> ::windows_core::Result<::winrt_devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Device)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Enumeration::DeviceInformation>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
    pub fn EffectDefinitions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EffectDefinitions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOutgoingGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OutgoingGain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn EncodingProperties(&self) -> ::windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EncodingProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ConsumeInput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetConsumeInput)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Reset)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).DisableEffectsByDefinition)(::windows_core::Interface::as_raw(this), definition.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).EnableEffectsByDefinition)(::windows_core::Interface::as_raw(this), definition.into_param().abi()).ok() }
    }
    pub fn SetListener<'a, Param0: ::windows_core::IntoParam<'a, AudioNodeListener>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNodeWithListener>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetListener)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Listener(&self) -> ::windows_core::Result<AudioNodeListener> {
        let this = &::windows_core::Interface::cast::<IAudioNodeWithListener>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Listener)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioNodeListener>(result__)
        }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for AudioDeviceOutputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioDeviceOutputNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioDeviceOutputNode {}
impl ::core::fmt::Debug for AudioDeviceOutputNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioDeviceOutputNode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioDeviceOutputNode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioDeviceOutputNode;{362edbff-ff1c-4434-9e0f-bd2ef522ac82})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioDeviceOutputNode {
    type Vtable = IAudioDeviceOutputNode_Vtbl;
    const IID: ::windows_core::GUID = <IAudioDeviceOutputNode as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioDeviceOutputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioDeviceOutputNode";
}
impl ::core::convert::From<AudioDeviceOutputNode> for ::windows_core::IUnknown {
    fn from(value: AudioDeviceOutputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioDeviceOutputNode> for ::windows_core::IUnknown {
    fn from(value: &AudioDeviceOutputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioDeviceOutputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioDeviceOutputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioDeviceOutputNode> for ::windows_core::IInspectable {
    fn from(value: AudioDeviceOutputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioDeviceOutputNode> for ::windows_core::IInspectable {
    fn from(value: &AudioDeviceOutputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioDeviceOutputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioDeviceOutputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AudioDeviceOutputNode> for IAudioNode {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioDeviceOutputNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioDeviceOutputNode> for IAudioNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioDeviceOutputNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioNode> for AudioDeviceOutputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioNode> for &AudioDeviceOutputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioNode> {
        ::core::convert::TryInto::<IAudioNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioDeviceOutputNode> for IAudioNodeWithListener {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioDeviceOutputNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioDeviceOutputNode> for IAudioNodeWithListener {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioDeviceOutputNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioNodeWithListener> for AudioDeviceOutputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioNodeWithListener> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioNodeWithListener> for &AudioDeviceOutputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioNodeWithListener> {
        ::core::convert::TryInto::<IAudioNodeWithListener>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioDeviceOutputNode> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioDeviceOutputNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioDeviceOutputNode> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioDeviceOutputNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for AudioDeviceOutputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &AudioDeviceOutputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioDeviceOutputNode {}
unsafe impl ::core::marker::Sync for AudioDeviceOutputNode {}
#[repr(transparent)]
pub struct AudioFileInputNode(::windows_core::IUnknown);
impl AudioFileInputNode {
    pub fn SetPlaybackSpeedFactor(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPlaybackSpeedFactor)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PlaybackSpeedFactor(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackSpeedFactor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn Seek<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, position: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Seek)(::windows_core::Interface::as_raw(this), position.into_param().abi()).ok() }
    }
    pub fn StartTime(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetStartTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn EndTime(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EndTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetEndTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEndTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn LoopCount(&self) -> ::windows_core::Result<::winrt_foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LoopCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<i32>>(result__)
        }
    }
    pub fn SetLoopCount<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<i32>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLoopCount)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn SourceFile(&self) -> ::windows_core::Result<::winrt_storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SourceFile)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::StorageFile>(result__)
        }
    }
    pub fn FileCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AudioFileInputNode, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).FileCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveFileCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFileCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn OutgoingConnections(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = &::windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OutgoingConnections)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<AudioGraphConnection>>(result__)
        }
    }
    pub fn AddOutgoingConnection<'a, Param0: ::windows_core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AddOutgoingConnection)(::windows_core::Interface::as_raw(this), destination.into_param().abi()).ok() }
    }
    pub fn AddOutgoingConnectionWithGain<'a, Param0: ::windows_core::IntoParam<'a, IAudioNode>>(&self, destination: Param0, gain: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AddOutgoingConnectionWithGain)(::windows_core::Interface::as_raw(this), destination.into_param().abi(), gain).ok() }
    }
    pub fn RemoveOutgoingConnection<'a, Param0: ::windows_core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOutgoingConnection)(::windows_core::Interface::as_raw(this), destination.into_param().abi()).ok() }
    }
    pub fn Emitter(&self) -> ::windows_core::Result<AudioNodeEmitter> {
        let this = &::windows_core::Interface::cast::<IAudioInputNode2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Emitter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioNodeEmitter>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
    pub fn EffectDefinitions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EffectDefinitions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOutgoingGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OutgoingGain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn EncodingProperties(&self) -> ::windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EncodingProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ConsumeInput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetConsumeInput)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Reset)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).DisableEffectsByDefinition)(::windows_core::Interface::as_raw(this), definition.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).EnableEffectsByDefinition)(::windows_core::Interface::as_raw(this), definition.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for AudioFileInputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioFileInputNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioFileInputNode {}
impl ::core::fmt::Debug for AudioFileInputNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioFileInputNode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioFileInputNode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioFileInputNode;{905b67c8-6f65-4cd4-8890-4694843c276d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioFileInputNode {
    type Vtable = IAudioFileInputNode_Vtbl;
    const IID: ::windows_core::GUID = <IAudioFileInputNode as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioFileInputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioFileInputNode";
}
impl ::core::convert::From<AudioFileInputNode> for ::windows_core::IUnknown {
    fn from(value: AudioFileInputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioFileInputNode> for ::windows_core::IUnknown {
    fn from(value: &AudioFileInputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioFileInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioFileInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioFileInputNode> for ::windows_core::IInspectable {
    fn from(value: AudioFileInputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioFileInputNode> for ::windows_core::IInspectable {
    fn from(value: &AudioFileInputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioFileInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioFileInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AudioFileInputNode> for IAudioInputNode {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioFileInputNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioFileInputNode> for IAudioInputNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioFileInputNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioInputNode> for AudioFileInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioInputNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioInputNode> for &AudioFileInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioInputNode> {
        ::core::convert::TryInto::<IAudioInputNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioFileInputNode> for IAudioInputNode2 {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioFileInputNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioFileInputNode> for IAudioInputNode2 {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioFileInputNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioInputNode2> for AudioFileInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioInputNode2> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioInputNode2> for &AudioFileInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioInputNode2> {
        ::core::convert::TryInto::<IAudioInputNode2>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioFileInputNode> for IAudioNode {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioFileInputNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioFileInputNode> for IAudioNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioFileInputNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioNode> for AudioFileInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioNode> for &AudioFileInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioNode> {
        ::core::convert::TryInto::<IAudioNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioFileInputNode> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioFileInputNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioFileInputNode> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioFileInputNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for AudioFileInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &AudioFileInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioFileInputNode {}
unsafe impl ::core::marker::Sync for AudioFileInputNode {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioFileNodeCreationStatus(pub i32);
impl AudioFileNodeCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const FileNotFound: Self = Self(1i32);
    pub const InvalidFileType: Self = Self(2i32);
    pub const FormatNotSupported: Self = Self(3i32);
    pub const UnknownFailure: Self = Self(4i32);
}
impl ::core::marker::Copy for AudioFileNodeCreationStatus {}
impl ::core::clone::Clone for AudioFileNodeCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioFileNodeCreationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AudioFileNodeCreationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioFileNodeCreationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioFileNodeCreationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioFileNodeCreationStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioFileNodeCreationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AudioFileOutputNode(::windows_core::IUnknown);
impl AudioFileOutputNode {
    #[cfg(feature = "winrt-storage")]
    pub fn File(&self) -> ::windows_core::Result<::winrt_storage::IStorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).File)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::IStorageFile>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn FileEncodingProfile(&self) -> ::windows_core::Result<super::MediaProperties::MediaEncodingProfile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FileEncodingProfile)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::MediaEncodingProfile>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn FinalizeAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<super::Transcoding::TranscodeFailureReason>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FinalizeAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<super::Transcoding::TranscodeFailureReason>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
    pub fn EffectDefinitions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EffectDefinitions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOutgoingGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OutgoingGain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn EncodingProperties(&self) -> ::windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EncodingProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ConsumeInput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetConsumeInput)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Reset)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).DisableEffectsByDefinition)(::windows_core::Interface::as_raw(this), definition.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).EnableEffectsByDefinition)(::windows_core::Interface::as_raw(this), definition.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for AudioFileOutputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioFileOutputNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioFileOutputNode {}
impl ::core::fmt::Debug for AudioFileOutputNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioFileOutputNode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioFileOutputNode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioFileOutputNode;{50e01980-5166-4093-80f8-ada00089e9cf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioFileOutputNode {
    type Vtable = IAudioFileOutputNode_Vtbl;
    const IID: ::windows_core::GUID = <IAudioFileOutputNode as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioFileOutputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioFileOutputNode";
}
impl ::core::convert::From<AudioFileOutputNode> for ::windows_core::IUnknown {
    fn from(value: AudioFileOutputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioFileOutputNode> for ::windows_core::IUnknown {
    fn from(value: &AudioFileOutputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioFileOutputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioFileOutputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioFileOutputNode> for ::windows_core::IInspectable {
    fn from(value: AudioFileOutputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioFileOutputNode> for ::windows_core::IInspectable {
    fn from(value: &AudioFileOutputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioFileOutputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioFileOutputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AudioFileOutputNode> for IAudioNode {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioFileOutputNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioFileOutputNode> for IAudioNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioFileOutputNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioNode> for AudioFileOutputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioNode> for &AudioFileOutputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioNode> {
        ::core::convert::TryInto::<IAudioNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioFileOutputNode> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioFileOutputNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioFileOutputNode> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioFileOutputNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for AudioFileOutputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &AudioFileOutputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioFileOutputNode {}
unsafe impl ::core::marker::Sync for AudioFileOutputNode {}
#[repr(transparent)]
pub struct AudioFrameCompletedEventArgs(::windows_core::IUnknown);
impl AudioFrameCompletedEventArgs {
    pub fn Frame(&self) -> ::windows_core::Result<super::AudioFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Frame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::AudioFrame>(result__)
        }
    }
}
impl ::core::clone::Clone for AudioFrameCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioFrameCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioFrameCompletedEventArgs {}
impl ::core::fmt::Debug for AudioFrameCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioFrameCompletedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioFrameCompletedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioFrameCompletedEventArgs;{dc7c829e-0208-4504-a5a8-f0f268920a65})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioFrameCompletedEventArgs {
    type Vtable = IAudioFrameCompletedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAudioFrameCompletedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioFrameCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Audio.AudioFrameCompletedEventArgs";
}
impl ::core::convert::From<AudioFrameCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AudioFrameCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioFrameCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AudioFrameCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioFrameCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioFrameCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioFrameCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AudioFrameCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioFrameCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AudioFrameCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioFrameCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioFrameCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioFrameCompletedEventArgs {}
unsafe impl ::core::marker::Sync for AudioFrameCompletedEventArgs {}
#[repr(transparent)]
pub struct AudioFrameInputNode(::windows_core::IUnknown);
impl AudioFrameInputNode {
    pub fn SetPlaybackSpeedFactor(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPlaybackSpeedFactor)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PlaybackSpeedFactor(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackSpeedFactor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn AddFrame<'a, Param0: ::windows_core::IntoParam<'a, super::AudioFrame>>(&self, frame: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddFrame)(::windows_core::Interface::as_raw(this), frame.into_param().abi()).ok() }
    }
    pub fn DiscardQueuedFrames(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).DiscardQueuedFrames)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn QueuedSampleCount(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).QueuedSampleCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn AudioFrameCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AudioFrameInputNode, AudioFrameCompletedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AudioFrameCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAudioFrameCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAudioFrameCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn QuantumStarted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AudioFrameInputNode, FrameInputNodeQuantumStartedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).QuantumStarted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveQuantumStarted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveQuantumStarted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn OutgoingConnections(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = &::windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OutgoingConnections)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<AudioGraphConnection>>(result__)
        }
    }
    pub fn AddOutgoingConnection<'a, Param0: ::windows_core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AddOutgoingConnection)(::windows_core::Interface::as_raw(this), destination.into_param().abi()).ok() }
    }
    pub fn AddOutgoingConnectionWithGain<'a, Param0: ::windows_core::IntoParam<'a, IAudioNode>>(&self, destination: Param0, gain: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AddOutgoingConnectionWithGain)(::windows_core::Interface::as_raw(this), destination.into_param().abi(), gain).ok() }
    }
    pub fn RemoveOutgoingConnection<'a, Param0: ::windows_core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOutgoingConnection)(::windows_core::Interface::as_raw(this), destination.into_param().abi()).ok() }
    }
    pub fn Emitter(&self) -> ::windows_core::Result<AudioNodeEmitter> {
        let this = &::windows_core::Interface::cast::<IAudioInputNode2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Emitter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioNodeEmitter>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
    pub fn EffectDefinitions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EffectDefinitions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOutgoingGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OutgoingGain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn EncodingProperties(&self) -> ::windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EncodingProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ConsumeInput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetConsumeInput)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Reset)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).DisableEffectsByDefinition)(::windows_core::Interface::as_raw(this), definition.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).EnableEffectsByDefinition)(::windows_core::Interface::as_raw(this), definition.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for AudioFrameInputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioFrameInputNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioFrameInputNode {}
impl ::core::fmt::Debug for AudioFrameInputNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioFrameInputNode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioFrameInputNode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioFrameInputNode;{01b266c7-fd96-4ff5-a3c5-d27a9bf44237})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioFrameInputNode {
    type Vtable = IAudioFrameInputNode_Vtbl;
    const IID: ::windows_core::GUID = <IAudioFrameInputNode as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioFrameInputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioFrameInputNode";
}
impl ::core::convert::From<AudioFrameInputNode> for ::windows_core::IUnknown {
    fn from(value: AudioFrameInputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioFrameInputNode> for ::windows_core::IUnknown {
    fn from(value: &AudioFrameInputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioFrameInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioFrameInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioFrameInputNode> for ::windows_core::IInspectable {
    fn from(value: AudioFrameInputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioFrameInputNode> for ::windows_core::IInspectable {
    fn from(value: &AudioFrameInputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioFrameInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioFrameInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AudioFrameInputNode> for IAudioInputNode {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioFrameInputNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioFrameInputNode> for IAudioInputNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioFrameInputNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioInputNode> for AudioFrameInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioInputNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioInputNode> for &AudioFrameInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioInputNode> {
        ::core::convert::TryInto::<IAudioInputNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioFrameInputNode> for IAudioInputNode2 {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioFrameInputNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioFrameInputNode> for IAudioInputNode2 {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioFrameInputNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioInputNode2> for AudioFrameInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioInputNode2> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioInputNode2> for &AudioFrameInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioInputNode2> {
        ::core::convert::TryInto::<IAudioInputNode2>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioFrameInputNode> for IAudioNode {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioFrameInputNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioFrameInputNode> for IAudioNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioFrameInputNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioNode> for AudioFrameInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioNode> for &AudioFrameInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioNode> {
        ::core::convert::TryInto::<IAudioNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioFrameInputNode> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioFrameInputNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioFrameInputNode> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioFrameInputNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for AudioFrameInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &AudioFrameInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioFrameInputNode {}
unsafe impl ::core::marker::Sync for AudioFrameInputNode {}
#[repr(transparent)]
pub struct AudioFrameOutputNode(::windows_core::IUnknown);
impl AudioFrameOutputNode {
    pub fn GetFrame(&self) -> ::windows_core::Result<super::AudioFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::AudioFrame>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
    pub fn EffectDefinitions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EffectDefinitions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOutgoingGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OutgoingGain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn EncodingProperties(&self) -> ::windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EncodingProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ConsumeInput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetConsumeInput)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Reset)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).DisableEffectsByDefinition)(::windows_core::Interface::as_raw(this), definition.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).EnableEffectsByDefinition)(::windows_core::Interface::as_raw(this), definition.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for AudioFrameOutputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioFrameOutputNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioFrameOutputNode {}
impl ::core::fmt::Debug for AudioFrameOutputNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioFrameOutputNode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioFrameOutputNode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioFrameOutputNode;{b847371b-3299-45f5-88b3-c9d12a3f1cc8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioFrameOutputNode {
    type Vtable = IAudioFrameOutputNode_Vtbl;
    const IID: ::windows_core::GUID = <IAudioFrameOutputNode as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioFrameOutputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioFrameOutputNode";
}
impl ::core::convert::From<AudioFrameOutputNode> for ::windows_core::IUnknown {
    fn from(value: AudioFrameOutputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioFrameOutputNode> for ::windows_core::IUnknown {
    fn from(value: &AudioFrameOutputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioFrameOutputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioFrameOutputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioFrameOutputNode> for ::windows_core::IInspectable {
    fn from(value: AudioFrameOutputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioFrameOutputNode> for ::windows_core::IInspectable {
    fn from(value: &AudioFrameOutputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioFrameOutputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioFrameOutputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AudioFrameOutputNode> for IAudioNode {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioFrameOutputNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioFrameOutputNode> for IAudioNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioFrameOutputNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioNode> for AudioFrameOutputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioNode> for &AudioFrameOutputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioNode> {
        ::core::convert::TryInto::<IAudioNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioFrameOutputNode> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioFrameOutputNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioFrameOutputNode> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioFrameOutputNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for AudioFrameOutputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &AudioFrameOutputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioFrameOutputNode {}
unsafe impl ::core::marker::Sync for AudioFrameOutputNode {}
#[repr(transparent)]
pub struct AudioGraph(::windows_core::IUnknown);
impl AudioGraph {
    pub fn CreateFrameInputNode(&self) -> ::windows_core::Result<AudioFrameInputNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFrameInputNode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioFrameInputNode>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn CreateFrameInputNodeWithFormat<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>>(&self, encodingproperties: Param0) -> ::windows_core::Result<AudioFrameInputNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFrameInputNodeWithFormat)(::windows_core::Interface::as_raw(this), encodingproperties.into_param().abi(), result__.as_mut_ptr()).from_abi::<AudioFrameInputNode>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn CreateDeviceInputNodeAsync(&self, category: super::Capture::MediaCategory) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateDeviceInputNodeAsync)(::windows_core::Interface::as_raw(this), category, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-media", feature = "winrt-media"))]
    pub fn CreateDeviceInputNodeWithFormatAsync<'a, Param1: ::windows_core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>>(&self, category: super::Capture::MediaCategory, encodingproperties: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateDeviceInputNodeWithFormatAsync)(::windows_core::Interface::as_raw(this), category, encodingproperties.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-devices", feature = "winrt-media", feature = "winrt-media"))]
    pub fn CreateDeviceInputNodeWithFormatOnDeviceAsync<'a, Param1: ::windows_core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>, Param2: ::windows_core::IntoParam<'a, ::winrt_devices::Enumeration::DeviceInformation>>(&self, category: super::Capture::MediaCategory, encodingproperties: Param1, device: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateDeviceInputNodeWithFormatOnDeviceAsync)(::windows_core::Interface::as_raw(this), category, encodingproperties.into_param().abi(), device.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>>(result__)
        }
    }
    pub fn CreateFrameOutputNode(&self) -> ::windows_core::Result<AudioFrameOutputNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFrameOutputNode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioFrameOutputNode>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn CreateFrameOutputNodeWithFormat<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>>(&self, encodingproperties: Param0) -> ::windows_core::Result<AudioFrameOutputNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFrameOutputNodeWithFormat)(::windows_core::Interface::as_raw(this), encodingproperties.into_param().abi(), result__.as_mut_ptr()).from_abi::<AudioFrameOutputNode>(result__)
        }
    }
    pub fn CreateDeviceOutputNodeAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<CreateAudioDeviceOutputNodeResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateDeviceOutputNodeAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<CreateAudioDeviceOutputNodeResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CreateFileInputNodeAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFile>>(&self, file: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<CreateAudioFileInputNodeResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFileInputNodeAsync)(::windows_core::Interface::as_raw(this), file.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<CreateAudioFileInputNodeResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CreateFileOutputNodeAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFile>>(&self, file: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<CreateAudioFileOutputNodeResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFileOutputNodeAsync)(::windows_core::Interface::as_raw(this), file.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<CreateAudioFileOutputNodeResult>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-media", feature = "winrt-storage"))]
    pub fn CreateFileOutputNodeWithFileProfileAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFile>, Param1: ::windows_core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>>(&self, file: Param0, fileencodingprofile: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<CreateAudioFileOutputNodeResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFileOutputNodeWithFileProfileAsync)(::windows_core::Interface::as_raw(this), file.into_param().abi(), fileencodingprofile.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<CreateAudioFileOutputNodeResult>>(result__)
        }
    }
    pub fn CreateSubmixNode(&self) -> ::windows_core::Result<AudioSubmixNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateSubmixNode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioSubmixNode>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn CreateSubmixNodeWithFormat<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>>(&self, encodingproperties: Param0) -> ::windows_core::Result<AudioSubmixNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateSubmixNodeWithFormat)(::windows_core::Interface::as_raw(this), encodingproperties.into_param().abi(), result__.as_mut_ptr()).from_abi::<AudioSubmixNode>(result__)
        }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ResetAllNodes(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ResetAllNodes)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn QuantumStarted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AudioGraph, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).QuantumStarted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveQuantumStarted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveQuantumStarted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn QuantumProcessed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AudioGraph, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).QuantumProcessed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveQuantumProcessed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveQuantumProcessed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn UnrecoverableErrorOccurred<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AudioGraph, AudioGraphUnrecoverableErrorOccurredEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).UnrecoverableErrorOccurred)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveUnrecoverableErrorOccurred<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUnrecoverableErrorOccurred)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn CompletedQuantumCount(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).CompletedQuantumCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn EncodingProperties(&self) -> ::windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EncodingProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn LatencyInSamples(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).LatencyInSamples)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn PrimaryRenderDevice(&self) -> ::windows_core::Result<::winrt_devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PrimaryRenderDevice)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Enumeration::DeviceInformation>(result__)
        }
    }
    pub fn RenderDeviceAudioProcessing(&self) -> ::windows_core::Result<super::AudioProcessing> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::AudioProcessing>::zeroed();
            (::windows_core::Interface::vtable(this).RenderDeviceAudioProcessing)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::AudioProcessing>(result__)
        }
    }
    pub fn SamplesPerQuantum(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).SamplesPerQuantum)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn CreateFrameInputNodeWithFormatAndEmitter<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>, Param1: ::windows_core::IntoParam<'a, AudioNodeEmitter>>(&self, encodingproperties: Param0, emitter: Param1) -> ::windows_core::Result<AudioFrameInputNode> {
        let this = &::windows_core::Interface::cast::<IAudioGraph2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFrameInputNodeWithFormatAndEmitter)(::windows_core::Interface::as_raw(this), encodingproperties.into_param().abi(), emitter.into_param().abi(), result__.as_mut_ptr()).from_abi::<AudioFrameInputNode>(result__)
        }
    }
    #[cfg(all(feature = "winrt-devices", feature = "winrt-media", feature = "winrt-media"))]
    pub fn CreateDeviceInputNodeWithFormatAndEmitterOnDeviceAsync<'a, Param1: ::windows_core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>, Param2: ::windows_core::IntoParam<'a, ::winrt_devices::Enumeration::DeviceInformation>, Param3: ::windows_core::IntoParam<'a, AudioNodeEmitter>>(&self, category: super::Capture::MediaCategory, encodingproperties: Param1, device: Param2, emitter: Param3) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>> {
        let this = &::windows_core::Interface::cast::<IAudioGraph2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateDeviceInputNodeWithFormatAndEmitterOnDeviceAsync)(::windows_core::Interface::as_raw(this), category, encodingproperties.into_param().abi(), device.into_param().abi(), emitter.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CreateFileInputNodeWithEmitterAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFile>, Param1: ::windows_core::IntoParam<'a, AudioNodeEmitter>>(&self, file: Param0, emitter: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<CreateAudioFileInputNodeResult>> {
        let this = &::windows_core::Interface::cast::<IAudioGraph2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFileInputNodeWithEmitterAsync)(::windows_core::Interface::as_raw(this), file.into_param().abi(), emitter.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<CreateAudioFileInputNodeResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn CreateSubmixNodeWithFormatAndEmitter<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>, Param1: ::windows_core::IntoParam<'a, AudioNodeEmitter>>(&self, encodingproperties: Param0, emitter: Param1) -> ::windows_core::Result<AudioSubmixNode> {
        let this = &::windows_core::Interface::cast::<IAudioGraph2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateSubmixNodeWithFormatAndEmitter)(::windows_core::Interface::as_raw(this), encodingproperties.into_param().abi(), emitter.into_param().abi(), result__.as_mut_ptr()).from_abi::<AudioSubmixNode>(result__)
        }
    }
    pub fn CreateBatchUpdater(&self) -> ::windows_core::Result<AudioGraphBatchUpdater> {
        let this = &::windows_core::Interface::cast::<IAudioGraph2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateBatchUpdater)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioGraphBatchUpdater>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn CreateMediaSourceAudioInputNodeAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Core::MediaSource>>(&self, mediasource: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<CreateMediaSourceAudioInputNodeResult>> {
        let this = &::windows_core::Interface::cast::<IAudioGraph3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateMediaSourceAudioInputNodeAsync)(::windows_core::Interface::as_raw(this), mediasource.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<CreateMediaSourceAudioInputNodeResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn CreateMediaSourceAudioInputNodeWithEmitterAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Core::MediaSource>, Param1: ::windows_core::IntoParam<'a, AudioNodeEmitter>>(&self, mediasource: Param0, emitter: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<CreateMediaSourceAudioInputNodeResult>> {
        let this = &::windows_core::Interface::cast::<IAudioGraph3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateMediaSourceAudioInputNodeWithEmitterAsync)(::windows_core::Interface::as_raw(this), mediasource.into_param().abi(), emitter.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<CreateMediaSourceAudioInputNodeResult>>(result__)
        }
    }
    pub fn CreateAsync<'a, Param0: ::windows_core::IntoParam<'a, AudioGraphSettings>>(settings: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<CreateAudioGraphResult>> {
        Self::IAudioGraphStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAsync)(::windows_core::Interface::as_raw(this), settings.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<CreateAudioGraphResult>>(result__)
        })
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IAudioGraphStatics<R, F: FnOnce(&IAudioGraphStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AudioGraph, IAudioGraphStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AudioGraph {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioGraph {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioGraph {}
impl ::core::fmt::Debug for AudioGraph {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioGraph").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioGraph {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioGraph;{1ad46eed-e48c-4e14-9660-2c4f83e9cdd8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioGraph {
    type Vtable = IAudioGraph_Vtbl;
    const IID: ::windows_core::GUID = <IAudioGraph as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioGraph {
    const NAME: &'static str = "Windows.Media.Audio.AudioGraph";
}
impl ::core::convert::From<AudioGraph> for ::windows_core::IUnknown {
    fn from(value: AudioGraph) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioGraph> for ::windows_core::IUnknown {
    fn from(value: &AudioGraph) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioGraph {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioGraph {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioGraph> for ::windows_core::IInspectable {
    fn from(value: AudioGraph) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioGraph> for ::windows_core::IInspectable {
    fn from(value: &AudioGraph) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioGraph {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioGraph {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AudioGraph> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioGraph) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioGraph> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioGraph) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for AudioGraph {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &AudioGraph {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioGraph {}
unsafe impl ::core::marker::Sync for AudioGraph {}
#[repr(transparent)]
pub struct AudioGraphBatchUpdater(::windows_core::IUnknown);
impl AudioGraphBatchUpdater {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for AudioGraphBatchUpdater {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioGraphBatchUpdater {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioGraphBatchUpdater {}
impl ::core::fmt::Debug for AudioGraphBatchUpdater {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioGraphBatchUpdater").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioGraphBatchUpdater {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioGraphBatchUpdater;{30d5a829-7fa4-4026-83bb-d75bae4ea99e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioGraphBatchUpdater {
    type Vtable = ::winrt_foundation::IClosable_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_foundation::IClosable as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioGraphBatchUpdater {
    const NAME: &'static str = "Windows.Media.Audio.AudioGraphBatchUpdater";
}
impl ::core::convert::From<AudioGraphBatchUpdater> for ::windows_core::IUnknown {
    fn from(value: AudioGraphBatchUpdater) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioGraphBatchUpdater> for ::windows_core::IUnknown {
    fn from(value: &AudioGraphBatchUpdater) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioGraphBatchUpdater {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioGraphBatchUpdater {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioGraphBatchUpdater> for ::windows_core::IInspectable {
    fn from(value: AudioGraphBatchUpdater) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioGraphBatchUpdater> for ::windows_core::IInspectable {
    fn from(value: &AudioGraphBatchUpdater) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioGraphBatchUpdater {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioGraphBatchUpdater {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AudioGraphBatchUpdater> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioGraphBatchUpdater) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioGraphBatchUpdater> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioGraphBatchUpdater) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for AudioGraphBatchUpdater {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &AudioGraphBatchUpdater {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioGraphBatchUpdater {}
unsafe impl ::core::marker::Sync for AudioGraphBatchUpdater {}
#[repr(transparent)]
pub struct AudioGraphConnection(::windows_core::IUnknown);
impl AudioGraphConnection {
    pub fn Destination(&self) -> ::windows_core::Result<IAudioNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Destination)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IAudioNode>(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Gain(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Gain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for AudioGraphConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioGraphConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioGraphConnection {}
impl ::core::fmt::Debug for AudioGraphConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioGraphConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioGraphConnection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioGraphConnection;{763070ed-d04e-4fac-b233-600b42edd469})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioGraphConnection {
    type Vtable = IAudioGraphConnection_Vtbl;
    const IID: ::windows_core::GUID = <IAudioGraphConnection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioGraphConnection {
    const NAME: &'static str = "Windows.Media.Audio.AudioGraphConnection";
}
impl ::core::convert::From<AudioGraphConnection> for ::windows_core::IUnknown {
    fn from(value: AudioGraphConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioGraphConnection> for ::windows_core::IUnknown {
    fn from(value: &AudioGraphConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioGraphConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioGraphConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioGraphConnection> for ::windows_core::IInspectable {
    fn from(value: AudioGraphConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioGraphConnection> for ::windows_core::IInspectable {
    fn from(value: &AudioGraphConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioGraphConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioGraphConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioGraphConnection {}
unsafe impl ::core::marker::Sync for AudioGraphConnection {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioGraphCreationStatus(pub i32);
impl AudioGraphCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const DeviceNotAvailable: Self = Self(1i32);
    pub const FormatNotSupported: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for AudioGraphCreationStatus {}
impl ::core::clone::Clone for AudioGraphCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioGraphCreationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AudioGraphCreationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioGraphCreationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioGraphCreationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioGraphCreationStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioGraphCreationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AudioGraphSettings(::windows_core::IUnknown);
impl AudioGraphSettings {
    #[cfg(feature = "winrt-media")]
    pub fn EncodingProperties(&self) -> ::windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EncodingProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn SetEncodingProperties<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEncodingProperties)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn PrimaryRenderDevice(&self) -> ::windows_core::Result<::winrt_devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PrimaryRenderDevice)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Enumeration::DeviceInformation>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn SetPrimaryRenderDevice<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_devices::Enumeration::DeviceInformation>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPrimaryRenderDevice)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn QuantumSizeSelectionMode(&self) -> ::windows_core::Result<QuantumSizeSelectionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<QuantumSizeSelectionMode>::zeroed();
            (::windows_core::Interface::vtable(this).QuantumSizeSelectionMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<QuantumSizeSelectionMode>(result__)
        }
    }
    pub fn SetQuantumSizeSelectionMode(&self, value: QuantumSizeSelectionMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetQuantumSizeSelectionMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DesiredSamplesPerQuantum(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredSamplesPerQuantum)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetDesiredSamplesPerQuantum(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredSamplesPerQuantum)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn AudioRenderCategory(&self) -> ::windows_core::Result<super::Render::AudioRenderCategory> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Render::AudioRenderCategory>::zeroed();
            (::windows_core::Interface::vtable(this).AudioRenderCategory)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Render::AudioRenderCategory>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn SetAudioRenderCategory(&self, value: super::Render::AudioRenderCategory) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAudioRenderCategory)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DesiredRenderDeviceAudioProcessing(&self) -> ::windows_core::Result<super::AudioProcessing> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::AudioProcessing>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredRenderDeviceAudioProcessing)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::AudioProcessing>(result__)
        }
    }
    pub fn SetDesiredRenderDeviceAudioProcessing(&self, value: super::AudioProcessing) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredRenderDeviceAudioProcessing)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetMaxPlaybackSpeedFactor(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioGraphSettings2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxPlaybackSpeedFactor)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxPlaybackSpeedFactor(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<IAudioGraphSettings2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).MaxPlaybackSpeedFactor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn Create(audiorendercategory: super::Render::AudioRenderCategory) -> ::windows_core::Result<AudioGraphSettings> {
        Self::IAudioGraphSettingsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), audiorendercategory, result__.as_mut_ptr()).from_abi::<AudioGraphSettings>(result__)
        })
    }
    pub fn IAudioGraphSettingsFactory<R, F: FnOnce(&IAudioGraphSettingsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AudioGraphSettings, IAudioGraphSettingsFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AudioGraphSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioGraphSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioGraphSettings {}
impl ::core::fmt::Debug for AudioGraphSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioGraphSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioGraphSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioGraphSettings;{1d59647f-e6fe-4628-84f8-9d8bdba25785})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioGraphSettings {
    type Vtable = IAudioGraphSettings_Vtbl;
    const IID: ::windows_core::GUID = <IAudioGraphSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioGraphSettings {
    const NAME: &'static str = "Windows.Media.Audio.AudioGraphSettings";
}
impl ::core::convert::From<AudioGraphSettings> for ::windows_core::IUnknown {
    fn from(value: AudioGraphSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioGraphSettings> for ::windows_core::IUnknown {
    fn from(value: &AudioGraphSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioGraphSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioGraphSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioGraphSettings> for ::windows_core::IInspectable {
    fn from(value: AudioGraphSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioGraphSettings> for ::windows_core::IInspectable {
    fn from(value: &AudioGraphSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioGraphSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioGraphSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioGraphSettings {}
unsafe impl ::core::marker::Sync for AudioGraphSettings {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioGraphUnrecoverableError(pub i32);
impl AudioGraphUnrecoverableError {
    pub const None: Self = Self(0i32);
    pub const AudioDeviceLost: Self = Self(1i32);
    pub const AudioSessionDisconnected: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for AudioGraphUnrecoverableError {}
impl ::core::clone::Clone for AudioGraphUnrecoverableError {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioGraphUnrecoverableError {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AudioGraphUnrecoverableError {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioGraphUnrecoverableError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioGraphUnrecoverableError").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioGraphUnrecoverableError {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioGraphUnrecoverableError;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AudioGraphUnrecoverableErrorOccurredEventArgs(::windows_core::IUnknown);
impl AudioGraphUnrecoverableErrorOccurredEventArgs {
    pub fn Error(&self) -> ::windows_core::Result<AudioGraphUnrecoverableError> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AudioGraphUnrecoverableError>::zeroed();
            (::windows_core::Interface::vtable(this).Error)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioGraphUnrecoverableError>(result__)
        }
    }
}
impl ::core::clone::Clone for AudioGraphUnrecoverableErrorOccurredEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioGraphUnrecoverableErrorOccurredEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioGraphUnrecoverableErrorOccurredEventArgs {}
impl ::core::fmt::Debug for AudioGraphUnrecoverableErrorOccurredEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioGraphUnrecoverableErrorOccurredEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioGraphUnrecoverableErrorOccurredEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioGraphUnrecoverableErrorOccurredEventArgs;{c3d9cbe0-3ff6-4fb3-b262-50d435c55423})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioGraphUnrecoverableErrorOccurredEventArgs {
    type Vtable = IAudioGraphUnrecoverableErrorOccurredEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAudioGraphUnrecoverableErrorOccurredEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioGraphUnrecoverableErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.Media.Audio.AudioGraphUnrecoverableErrorOccurredEventArgs";
}
impl ::core::convert::From<AudioGraphUnrecoverableErrorOccurredEventArgs> for ::windows_core::IUnknown {
    fn from(value: AudioGraphUnrecoverableErrorOccurredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioGraphUnrecoverableErrorOccurredEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AudioGraphUnrecoverableErrorOccurredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioGraphUnrecoverableErrorOccurredEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioGraphUnrecoverableErrorOccurredEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioGraphUnrecoverableErrorOccurredEventArgs> for ::windows_core::IInspectable {
    fn from(value: AudioGraphUnrecoverableErrorOccurredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioGraphUnrecoverableErrorOccurredEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AudioGraphUnrecoverableErrorOccurredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioGraphUnrecoverableErrorOccurredEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioGraphUnrecoverableErrorOccurredEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioGraphUnrecoverableErrorOccurredEventArgs {}
unsafe impl ::core::marker::Sync for AudioGraphUnrecoverableErrorOccurredEventArgs {}
#[repr(transparent)]
pub struct AudioNodeEmitter(::windows_core::IUnknown);
impl AudioNodeEmitter {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AudioNodeEmitter, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetPosition<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPosition)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Direction(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).Direction)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetDirection<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDirection)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Shape(&self) -> ::windows_core::Result<AudioNodeEmitterShape> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Shape)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioNodeEmitterShape>(result__)
        }
    }
    pub fn DecayModel(&self) -> ::windows_core::Result<AudioNodeEmitterDecayModel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DecayModel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioNodeEmitterDecayModel>(result__)
        }
    }
    pub fn Gain(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Gain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DistanceScale(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DistanceScale)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetDistanceScale(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDistanceScale)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DopplerScale(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DopplerScale)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetDopplerScale(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDopplerScale)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn DopplerVelocity(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).DopplerVelocity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetDopplerVelocity<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDopplerVelocity)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsDopplerDisabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDopplerDisabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SpatialAudioModel(&self) -> ::windows_core::Result<SpatialAudioModel> {
        let this = &::windows_core::Interface::cast::<IAudioNodeEmitter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SpatialAudioModel>::zeroed();
            (::windows_core::Interface::vtable(this).SpatialAudioModel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialAudioModel>(result__)
        }
    }
    pub fn SetSpatialAudioModel(&self, value: SpatialAudioModel) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNodeEmitter2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSpatialAudioModel)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateAudioNodeEmitter<'a, Param0: ::windows_core::IntoParam<'a, AudioNodeEmitterShape>, Param1: ::windows_core::IntoParam<'a, AudioNodeEmitterDecayModel>>(shape: Param0, decaymodel: Param1, settings: AudioNodeEmitterSettings) -> ::windows_core::Result<AudioNodeEmitter> {
        Self::IAudioNodeEmitterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAudioNodeEmitter)(::windows_core::Interface::as_raw(this), shape.into_param().abi(), decaymodel.into_param().abi(), settings, result__.as_mut_ptr()).from_abi::<AudioNodeEmitter>(result__)
        })
    }
    pub fn IAudioNodeEmitterFactory<R, F: FnOnce(&IAudioNodeEmitterFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AudioNodeEmitter, IAudioNodeEmitterFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AudioNodeEmitter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioNodeEmitter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioNodeEmitter {}
impl ::core::fmt::Debug for AudioNodeEmitter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioNodeEmitter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioNodeEmitter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioNodeEmitter;{3676971d-880a-47b8-adf7-1323a9d965be})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioNodeEmitter {
    type Vtable = IAudioNodeEmitter_Vtbl;
    const IID: ::windows_core::GUID = <IAudioNodeEmitter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioNodeEmitter {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeEmitter";
}
impl ::core::convert::From<AudioNodeEmitter> for ::windows_core::IUnknown {
    fn from(value: AudioNodeEmitter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioNodeEmitter> for ::windows_core::IUnknown {
    fn from(value: &AudioNodeEmitter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioNodeEmitter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioNodeEmitter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioNodeEmitter> for ::windows_core::IInspectable {
    fn from(value: AudioNodeEmitter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioNodeEmitter> for ::windows_core::IInspectable {
    fn from(value: &AudioNodeEmitter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioNodeEmitter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioNodeEmitter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioNodeEmitter {}
unsafe impl ::core::marker::Sync for AudioNodeEmitter {}
#[repr(transparent)]
pub struct AudioNodeEmitterConeProperties(::windows_core::IUnknown);
impl AudioNodeEmitterConeProperties {
    pub fn InnerAngle(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).InnerAngle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn OuterAngle(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OuterAngle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn OuterAngleGain(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OuterAngleGain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for AudioNodeEmitterConeProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioNodeEmitterConeProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioNodeEmitterConeProperties {}
impl ::core::fmt::Debug for AudioNodeEmitterConeProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioNodeEmitterConeProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioNodeEmitterConeProperties {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioNodeEmitterConeProperties;{e99b2cee-02ca-4375-9326-0c6ae4bcdfb5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioNodeEmitterConeProperties {
    type Vtable = IAudioNodeEmitterConeProperties_Vtbl;
    const IID: ::windows_core::GUID = <IAudioNodeEmitterConeProperties as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioNodeEmitterConeProperties {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeEmitterConeProperties";
}
impl ::core::convert::From<AudioNodeEmitterConeProperties> for ::windows_core::IUnknown {
    fn from(value: AudioNodeEmitterConeProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioNodeEmitterConeProperties> for ::windows_core::IUnknown {
    fn from(value: &AudioNodeEmitterConeProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioNodeEmitterConeProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioNodeEmitterConeProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioNodeEmitterConeProperties> for ::windows_core::IInspectable {
    fn from(value: AudioNodeEmitterConeProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioNodeEmitterConeProperties> for ::windows_core::IInspectable {
    fn from(value: &AudioNodeEmitterConeProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioNodeEmitterConeProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioNodeEmitterConeProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioNodeEmitterConeProperties {}
unsafe impl ::core::marker::Sync for AudioNodeEmitterConeProperties {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioNodeEmitterDecayKind(pub i32);
impl AudioNodeEmitterDecayKind {
    pub const Natural: Self = Self(0i32);
    pub const Custom: Self = Self(1i32);
}
impl ::core::marker::Copy for AudioNodeEmitterDecayKind {}
impl ::core::clone::Clone for AudioNodeEmitterDecayKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioNodeEmitterDecayKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AudioNodeEmitterDecayKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioNodeEmitterDecayKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioNodeEmitterDecayKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioNodeEmitterDecayKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioNodeEmitterDecayKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AudioNodeEmitterDecayModel(::windows_core::IUnknown);
impl AudioNodeEmitterDecayModel {
    pub fn Kind(&self) -> ::windows_core::Result<AudioNodeEmitterDecayKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AudioNodeEmitterDecayKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioNodeEmitterDecayKind>(result__)
        }
    }
    pub fn MinGain(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).MinGain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn MaxGain(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).MaxGain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn NaturalProperties(&self) -> ::windows_core::Result<AudioNodeEmitterNaturalDecayModelProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NaturalProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioNodeEmitterNaturalDecayModelProperties>(result__)
        }
    }
    pub fn CreateNatural(mingain: f64, maxgain: f64, unitygaindistance: f64, cutoffdistance: f64) -> ::windows_core::Result<AudioNodeEmitterDecayModel> {
        Self::IAudioNodeEmitterDecayModelStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateNatural)(::windows_core::Interface::as_raw(this), mingain, maxgain, unitygaindistance, cutoffdistance, result__.as_mut_ptr()).from_abi::<AudioNodeEmitterDecayModel>(result__)
        })
    }
    pub fn CreateCustom(mingain: f64, maxgain: f64) -> ::windows_core::Result<AudioNodeEmitterDecayModel> {
        Self::IAudioNodeEmitterDecayModelStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateCustom)(::windows_core::Interface::as_raw(this), mingain, maxgain, result__.as_mut_ptr()).from_abi::<AudioNodeEmitterDecayModel>(result__)
        })
    }
    pub fn IAudioNodeEmitterDecayModelStatics<R, F: FnOnce(&IAudioNodeEmitterDecayModelStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AudioNodeEmitterDecayModel, IAudioNodeEmitterDecayModelStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AudioNodeEmitterDecayModel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioNodeEmitterDecayModel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioNodeEmitterDecayModel {}
impl ::core::fmt::Debug for AudioNodeEmitterDecayModel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioNodeEmitterDecayModel").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioNodeEmitterDecayModel {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioNodeEmitterDecayModel;{1d1d5af7-0d53-4fa9-bd84-d5816a86f3ff})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioNodeEmitterDecayModel {
    type Vtable = IAudioNodeEmitterDecayModel_Vtbl;
    const IID: ::windows_core::GUID = <IAudioNodeEmitterDecayModel as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioNodeEmitterDecayModel {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeEmitterDecayModel";
}
impl ::core::convert::From<AudioNodeEmitterDecayModel> for ::windows_core::IUnknown {
    fn from(value: AudioNodeEmitterDecayModel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioNodeEmitterDecayModel> for ::windows_core::IUnknown {
    fn from(value: &AudioNodeEmitterDecayModel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioNodeEmitterDecayModel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioNodeEmitterDecayModel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioNodeEmitterDecayModel> for ::windows_core::IInspectable {
    fn from(value: AudioNodeEmitterDecayModel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioNodeEmitterDecayModel> for ::windows_core::IInspectable {
    fn from(value: &AudioNodeEmitterDecayModel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioNodeEmitterDecayModel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioNodeEmitterDecayModel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioNodeEmitterDecayModel {}
unsafe impl ::core::marker::Sync for AudioNodeEmitterDecayModel {}
#[repr(transparent)]
pub struct AudioNodeEmitterNaturalDecayModelProperties(::windows_core::IUnknown);
impl AudioNodeEmitterNaturalDecayModelProperties {
    pub fn UnityGainDistance(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).UnityGainDistance)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn CutoffDistance(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).CutoffDistance)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for AudioNodeEmitterNaturalDecayModelProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioNodeEmitterNaturalDecayModelProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioNodeEmitterNaturalDecayModelProperties {}
impl ::core::fmt::Debug for AudioNodeEmitterNaturalDecayModelProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioNodeEmitterNaturalDecayModelProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioNodeEmitterNaturalDecayModelProperties {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioNodeEmitterNaturalDecayModelProperties;{48934bcf-cf2c-4efc-9331-75bd22df1f0c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioNodeEmitterNaturalDecayModelProperties {
    type Vtable = IAudioNodeEmitterNaturalDecayModelProperties_Vtbl;
    const IID: ::windows_core::GUID = <IAudioNodeEmitterNaturalDecayModelProperties as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioNodeEmitterNaturalDecayModelProperties {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeEmitterNaturalDecayModelProperties";
}
impl ::core::convert::From<AudioNodeEmitterNaturalDecayModelProperties> for ::windows_core::IUnknown {
    fn from(value: AudioNodeEmitterNaturalDecayModelProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioNodeEmitterNaturalDecayModelProperties> for ::windows_core::IUnknown {
    fn from(value: &AudioNodeEmitterNaturalDecayModelProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioNodeEmitterNaturalDecayModelProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioNodeEmitterNaturalDecayModelProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioNodeEmitterNaturalDecayModelProperties> for ::windows_core::IInspectable {
    fn from(value: AudioNodeEmitterNaturalDecayModelProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioNodeEmitterNaturalDecayModelProperties> for ::windows_core::IInspectable {
    fn from(value: &AudioNodeEmitterNaturalDecayModelProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioNodeEmitterNaturalDecayModelProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioNodeEmitterNaturalDecayModelProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioNodeEmitterNaturalDecayModelProperties {}
unsafe impl ::core::marker::Sync for AudioNodeEmitterNaturalDecayModelProperties {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioNodeEmitterSettings(pub u32);
impl AudioNodeEmitterSettings {
    pub const None: Self = Self(0u32);
    pub const DisableDoppler: Self = Self(1u32);
}
impl ::core::marker::Copy for AudioNodeEmitterSettings {}
impl ::core::clone::Clone for AudioNodeEmitterSettings {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioNodeEmitterSettings {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AudioNodeEmitterSettings {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioNodeEmitterSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioNodeEmitterSettings").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for AudioNodeEmitterSettings {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AudioNodeEmitterSettings {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AudioNodeEmitterSettings {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AudioNodeEmitterSettings {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AudioNodeEmitterSettings {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for AudioNodeEmitterSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioNodeEmitterSettings;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AudioNodeEmitterShape(::windows_core::IUnknown);
impl AudioNodeEmitterShape {
    pub fn Kind(&self) -> ::windows_core::Result<AudioNodeEmitterShapeKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AudioNodeEmitterShapeKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioNodeEmitterShapeKind>(result__)
        }
    }
    pub fn ConeProperties(&self) -> ::windows_core::Result<AudioNodeEmitterConeProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ConeProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioNodeEmitterConeProperties>(result__)
        }
    }
    pub fn CreateCone(innerangle: f64, outerangle: f64, outeranglegain: f64) -> ::windows_core::Result<AudioNodeEmitterShape> {
        Self::IAudioNodeEmitterShapeStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateCone)(::windows_core::Interface::as_raw(this), innerangle, outerangle, outeranglegain, result__.as_mut_ptr()).from_abi::<AudioNodeEmitterShape>(result__)
        })
    }
    pub fn CreateOmnidirectional() -> ::windows_core::Result<AudioNodeEmitterShape> {
        Self::IAudioNodeEmitterShapeStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateOmnidirectional)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioNodeEmitterShape>(result__)
        })
    }
    pub fn IAudioNodeEmitterShapeStatics<R, F: FnOnce(&IAudioNodeEmitterShapeStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AudioNodeEmitterShape, IAudioNodeEmitterShapeStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AudioNodeEmitterShape {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioNodeEmitterShape {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioNodeEmitterShape {}
impl ::core::fmt::Debug for AudioNodeEmitterShape {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioNodeEmitterShape").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioNodeEmitterShape {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioNodeEmitterShape;{ea0311c5-e73d-44bc-859c-45553bbc4828})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioNodeEmitterShape {
    type Vtable = IAudioNodeEmitterShape_Vtbl;
    const IID: ::windows_core::GUID = <IAudioNodeEmitterShape as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioNodeEmitterShape {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeEmitterShape";
}
impl ::core::convert::From<AudioNodeEmitterShape> for ::windows_core::IUnknown {
    fn from(value: AudioNodeEmitterShape) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioNodeEmitterShape> for ::windows_core::IUnknown {
    fn from(value: &AudioNodeEmitterShape) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioNodeEmitterShape {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioNodeEmitterShape {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioNodeEmitterShape> for ::windows_core::IInspectable {
    fn from(value: AudioNodeEmitterShape) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioNodeEmitterShape> for ::windows_core::IInspectable {
    fn from(value: &AudioNodeEmitterShape) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioNodeEmitterShape {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioNodeEmitterShape {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioNodeEmitterShape {}
unsafe impl ::core::marker::Sync for AudioNodeEmitterShape {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioNodeEmitterShapeKind(pub i32);
impl AudioNodeEmitterShapeKind {
    pub const Omnidirectional: Self = Self(0i32);
    pub const Cone: Self = Self(1i32);
}
impl ::core::marker::Copy for AudioNodeEmitterShapeKind {}
impl ::core::clone::Clone for AudioNodeEmitterShapeKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioNodeEmitterShapeKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AudioNodeEmitterShapeKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioNodeEmitterShapeKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioNodeEmitterShapeKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioNodeEmitterShapeKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioNodeEmitterShapeKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AudioNodeListener(::windows_core::IUnknown);
impl AudioNodeListener {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AudioNodeListener, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetPosition<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPosition)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Orientation(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Quaternion>::zeroed();
            (::windows_core::Interface::vtable(this).Orientation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Quaternion>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetOrientation<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Quaternion>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOrientation)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SpeedOfSound(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).SpeedOfSound)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetSpeedOfSound(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSpeedOfSound)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn DopplerVelocity(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).DopplerVelocity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetDopplerVelocity<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDopplerVelocity)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AudioNodeListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioNodeListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioNodeListener {}
impl ::core::fmt::Debug for AudioNodeListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioNodeListener").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioNodeListener {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioNodeListener;{d9722e16-0c0a-41da-b755-6c77835fb1eb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioNodeListener {
    type Vtable = IAudioNodeListener_Vtbl;
    const IID: ::windows_core::GUID = <IAudioNodeListener as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioNodeListener {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeListener";
}
impl ::core::convert::From<AudioNodeListener> for ::windows_core::IUnknown {
    fn from(value: AudioNodeListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioNodeListener> for ::windows_core::IUnknown {
    fn from(value: &AudioNodeListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioNodeListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioNodeListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioNodeListener> for ::windows_core::IInspectable {
    fn from(value: AudioNodeListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioNodeListener> for ::windows_core::IInspectable {
    fn from(value: &AudioNodeListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioNodeListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioNodeListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioNodeListener {}
unsafe impl ::core::marker::Sync for AudioNodeListener {}
#[repr(transparent)]
pub struct AudioPlaybackConnection(::windows_core::IUnknown);
impl AudioPlaybackConnection {
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn StartAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn State(&self) -> ::windows_core::Result<AudioPlaybackConnectionState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AudioPlaybackConnectionState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioPlaybackConnectionState>(result__)
        }
    }
    pub fn Open(&self) -> ::windows_core::Result<AudioPlaybackConnectionOpenResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Open)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioPlaybackConnectionOpenResult>(result__)
        }
    }
    pub fn OpenAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AudioPlaybackConnectionOpenResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AudioPlaybackConnectionOpenResult>>(result__)
        }
    }
    pub fn StateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AudioPlaybackConnection, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).StateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStateChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAudioPlaybackConnectionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn TryCreateFromId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(id: Param0) -> ::windows_core::Result<AudioPlaybackConnection> {
        Self::IAudioPlaybackConnectionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateFromId)(::windows_core::Interface::as_raw(this), id.into_param().abi(), result__.as_mut_ptr()).from_abi::<AudioPlaybackConnection>(result__)
        })
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IAudioPlaybackConnectionStatics<R, F: FnOnce(&IAudioPlaybackConnectionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AudioPlaybackConnection, IAudioPlaybackConnectionStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AudioPlaybackConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioPlaybackConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioPlaybackConnection {}
impl ::core::fmt::Debug for AudioPlaybackConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioPlaybackConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioPlaybackConnection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioPlaybackConnection;{1a4c1dea-cafc-50e7-8718-ea3f81cbfa51})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioPlaybackConnection {
    type Vtable = IAudioPlaybackConnection_Vtbl;
    const IID: ::windows_core::GUID = <IAudioPlaybackConnection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioPlaybackConnection {
    const NAME: &'static str = "Windows.Media.Audio.AudioPlaybackConnection";
}
impl ::core::convert::From<AudioPlaybackConnection> for ::windows_core::IUnknown {
    fn from(value: AudioPlaybackConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioPlaybackConnection> for ::windows_core::IUnknown {
    fn from(value: &AudioPlaybackConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioPlaybackConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioPlaybackConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioPlaybackConnection> for ::windows_core::IInspectable {
    fn from(value: AudioPlaybackConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioPlaybackConnection> for ::windows_core::IInspectable {
    fn from(value: &AudioPlaybackConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioPlaybackConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioPlaybackConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AudioPlaybackConnection> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioPlaybackConnection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioPlaybackConnection> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioPlaybackConnection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for AudioPlaybackConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &AudioPlaybackConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioPlaybackConnection {}
unsafe impl ::core::marker::Sync for AudioPlaybackConnection {}
#[repr(transparent)]
pub struct AudioPlaybackConnectionOpenResult(::windows_core::IUnknown);
impl AudioPlaybackConnectionOpenResult {
    pub fn Status(&self) -> ::windows_core::Result<AudioPlaybackConnectionOpenResultStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AudioPlaybackConnectionOpenResultStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioPlaybackConnectionOpenResultStatus>(result__)
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
impl ::core::clone::Clone for AudioPlaybackConnectionOpenResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioPlaybackConnectionOpenResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioPlaybackConnectionOpenResult {}
impl ::core::fmt::Debug for AudioPlaybackConnectionOpenResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioPlaybackConnectionOpenResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioPlaybackConnectionOpenResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioPlaybackConnectionOpenResult;{4e656aef-39f9-5fc9-a519-a5bbfd9fe921})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioPlaybackConnectionOpenResult {
    type Vtable = IAudioPlaybackConnectionOpenResult_Vtbl;
    const IID: ::windows_core::GUID = <IAudioPlaybackConnectionOpenResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioPlaybackConnectionOpenResult {
    const NAME: &'static str = "Windows.Media.Audio.AudioPlaybackConnectionOpenResult";
}
impl ::core::convert::From<AudioPlaybackConnectionOpenResult> for ::windows_core::IUnknown {
    fn from(value: AudioPlaybackConnectionOpenResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioPlaybackConnectionOpenResult> for ::windows_core::IUnknown {
    fn from(value: &AudioPlaybackConnectionOpenResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioPlaybackConnectionOpenResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioPlaybackConnectionOpenResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioPlaybackConnectionOpenResult> for ::windows_core::IInspectable {
    fn from(value: AudioPlaybackConnectionOpenResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioPlaybackConnectionOpenResult> for ::windows_core::IInspectable {
    fn from(value: &AudioPlaybackConnectionOpenResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioPlaybackConnectionOpenResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioPlaybackConnectionOpenResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioPlaybackConnectionOpenResult {}
unsafe impl ::core::marker::Sync for AudioPlaybackConnectionOpenResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioPlaybackConnectionOpenResultStatus(pub i32);
impl AudioPlaybackConnectionOpenResultStatus {
    pub const Success: Self = Self(0i32);
    pub const RequestTimedOut: Self = Self(1i32);
    pub const DeniedBySystem: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for AudioPlaybackConnectionOpenResultStatus {}
impl ::core::clone::Clone for AudioPlaybackConnectionOpenResultStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioPlaybackConnectionOpenResultStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AudioPlaybackConnectionOpenResultStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioPlaybackConnectionOpenResultStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioPlaybackConnectionOpenResultStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioPlaybackConnectionOpenResultStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioPlaybackConnectionOpenResultStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioPlaybackConnectionState(pub i32);
impl AudioPlaybackConnectionState {
    pub const Closed: Self = Self(0i32);
    pub const Opened: Self = Self(1i32);
}
impl ::core::marker::Copy for AudioPlaybackConnectionState {}
impl ::core::clone::Clone for AudioPlaybackConnectionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioPlaybackConnectionState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AudioPlaybackConnectionState {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioPlaybackConnectionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioPlaybackConnectionState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioPlaybackConnectionState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioPlaybackConnectionState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AudioStateMonitor(::windows_core::IUnknown);
impl AudioStateMonitor {
    pub fn SoundLevelChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AudioStateMonitor, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SoundLevelChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSoundLevelChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSoundLevelChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn SoundLevel(&self) -> ::windows_core::Result<super::SoundLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::SoundLevel>::zeroed();
            (::windows_core::Interface::vtable(this).SoundLevel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::SoundLevel>(result__)
        }
    }
    pub fn CreateForRenderMonitoring() -> ::windows_core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateForRenderMonitoring)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioStateMonitor>(result__)
        })
    }
    #[cfg(feature = "winrt-media")]
    pub fn CreateForRenderMonitoringWithCategory(category: super::Render::AudioRenderCategory) -> ::windows_core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateForRenderMonitoringWithCategory)(::windows_core::Interface::as_raw(this), category, result__.as_mut_ptr()).from_abi::<AudioStateMonitor>(result__)
        })
    }
    #[cfg(all(feature = "winrt-media", feature = "winrt-media"))]
    pub fn CreateForRenderMonitoringWithCategoryAndDeviceRole(category: super::Render::AudioRenderCategory, role: super::Devices::AudioDeviceRole) -> ::windows_core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateForRenderMonitoringWithCategoryAndDeviceRole)(::windows_core::Interface::as_raw(this), category, role, result__.as_mut_ptr()).from_abi::<AudioStateMonitor>(result__)
        })
    }
    #[cfg(feature = "winrt-media")]
    pub fn CreateForRenderMonitoringWithCategoryAndDeviceId<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(category: super::Render::AudioRenderCategory, deviceid: Param1) -> ::windows_core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateForRenderMonitoringWithCategoryAndDeviceId)(::windows_core::Interface::as_raw(this), category, deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<AudioStateMonitor>(result__)
        })
    }
    pub fn CreateForCaptureMonitoring() -> ::windows_core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateForCaptureMonitoring)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioStateMonitor>(result__)
        })
    }
    #[cfg(feature = "winrt-media")]
    pub fn CreateForCaptureMonitoringWithCategory(category: super::Capture::MediaCategory) -> ::windows_core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateForCaptureMonitoringWithCategory)(::windows_core::Interface::as_raw(this), category, result__.as_mut_ptr()).from_abi::<AudioStateMonitor>(result__)
        })
    }
    #[cfg(all(feature = "winrt-media", feature = "winrt-media"))]
    pub fn CreateForCaptureMonitoringWithCategoryAndDeviceRole(category: super::Capture::MediaCategory, role: super::Devices::AudioDeviceRole) -> ::windows_core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateForCaptureMonitoringWithCategoryAndDeviceRole)(::windows_core::Interface::as_raw(this), category, role, result__.as_mut_ptr()).from_abi::<AudioStateMonitor>(result__)
        })
    }
    #[cfg(feature = "winrt-media")]
    pub fn CreateForCaptureMonitoringWithCategoryAndDeviceId<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(category: super::Capture::MediaCategory, deviceid: Param1) -> ::windows_core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateForCaptureMonitoringWithCategoryAndDeviceId)(::windows_core::Interface::as_raw(this), category, deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<AudioStateMonitor>(result__)
        })
    }
    pub fn IAudioStateMonitorStatics<R, F: FnOnce(&IAudioStateMonitorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AudioStateMonitor, IAudioStateMonitorStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AudioStateMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioStateMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioStateMonitor {}
impl ::core::fmt::Debug for AudioStateMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioStateMonitor").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioStateMonitor {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioStateMonitor;{1d13d136-0199-4cdc-b84e-e72c2b581ece})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioStateMonitor {
    type Vtable = IAudioStateMonitor_Vtbl;
    const IID: ::windows_core::GUID = <IAudioStateMonitor as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioStateMonitor {
    const NAME: &'static str = "Windows.Media.Audio.AudioStateMonitor";
}
impl ::core::convert::From<AudioStateMonitor> for ::windows_core::IUnknown {
    fn from(value: AudioStateMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioStateMonitor> for ::windows_core::IUnknown {
    fn from(value: &AudioStateMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioStateMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioStateMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioStateMonitor> for ::windows_core::IInspectable {
    fn from(value: AudioStateMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioStateMonitor> for ::windows_core::IInspectable {
    fn from(value: &AudioStateMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioStateMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioStateMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioStateMonitor {}
unsafe impl ::core::marker::Sync for AudioStateMonitor {}
#[repr(transparent)]
pub struct AudioSubmixNode(::windows_core::IUnknown);
impl AudioSubmixNode {
    #[cfg(feature = "winrt-foundation")]
    pub fn OutgoingConnections(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OutgoingConnections)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<AudioGraphConnection>>(result__)
        }
    }
    pub fn AddOutgoingConnection<'a, Param0: ::windows_core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddOutgoingConnection)(::windows_core::Interface::as_raw(this), destination.into_param().abi()).ok() }
    }
    pub fn AddOutgoingConnectionWithGain<'a, Param0: ::windows_core::IntoParam<'a, IAudioNode>>(&self, destination: Param0, gain: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddOutgoingConnectionWithGain)(::windows_core::Interface::as_raw(this), destination.into_param().abi(), gain).ok() }
    }
    pub fn RemoveOutgoingConnection<'a, Param0: ::windows_core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOutgoingConnection)(::windows_core::Interface::as_raw(this), destination.into_param().abi()).ok() }
    }
    pub fn Emitter(&self) -> ::windows_core::Result<AudioNodeEmitter> {
        let this = &::windows_core::Interface::cast::<IAudioInputNode2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Emitter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioNodeEmitter>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
    pub fn EffectDefinitions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EffectDefinitions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOutgoingGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OutgoingGain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn EncodingProperties(&self) -> ::windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EncodingProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ConsumeInput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetConsumeInput)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Reset)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).DisableEffectsByDefinition)(::windows_core::Interface::as_raw(this), definition.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).EnableEffectsByDefinition)(::windows_core::Interface::as_raw(this), definition.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for AudioSubmixNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioSubmixNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioSubmixNode {}
impl ::core::fmt::Debug for AudioSubmixNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioSubmixNode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioSubmixNode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioSubmixNode;{d148005c-8428-4784-b7fd-a99d468c5d20})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioSubmixNode {
    type Vtable = IAudioInputNode_Vtbl;
    const IID: ::windows_core::GUID = <IAudioInputNode as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioSubmixNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioSubmixNode";
}
impl ::core::convert::From<AudioSubmixNode> for ::windows_core::IUnknown {
    fn from(value: AudioSubmixNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioSubmixNode> for ::windows_core::IUnknown {
    fn from(value: &AudioSubmixNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioSubmixNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioSubmixNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioSubmixNode> for ::windows_core::IInspectable {
    fn from(value: AudioSubmixNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioSubmixNode> for ::windows_core::IInspectable {
    fn from(value: &AudioSubmixNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioSubmixNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioSubmixNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AudioSubmixNode> for IAudioInputNode {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioSubmixNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioSubmixNode> for IAudioInputNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioSubmixNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioInputNode> for AudioSubmixNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioInputNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioInputNode> for &AudioSubmixNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioInputNode> {
        ::core::convert::TryInto::<IAudioInputNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioSubmixNode> for IAudioInputNode2 {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioSubmixNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioSubmixNode> for IAudioInputNode2 {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioSubmixNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioInputNode2> for AudioSubmixNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioInputNode2> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioInputNode2> for &AudioSubmixNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioInputNode2> {
        ::core::convert::TryInto::<IAudioInputNode2>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioSubmixNode> for IAudioNode {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioSubmixNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioSubmixNode> for IAudioNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioSubmixNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioNode> for AudioSubmixNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioNode> for &AudioSubmixNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioNode> {
        ::core::convert::TryInto::<IAudioNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioSubmixNode> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: AudioSubmixNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioSubmixNode> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &AudioSubmixNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for AudioSubmixNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &AudioSubmixNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioSubmixNode {}
unsafe impl ::core::marker::Sync for AudioSubmixNode {}
#[repr(transparent)]
pub struct CreateAudioDeviceInputNodeResult(::windows_core::IUnknown);
impl CreateAudioDeviceInputNodeResult {
    pub fn Status(&self) -> ::windows_core::Result<AudioDeviceNodeCreationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AudioDeviceNodeCreationStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioDeviceNodeCreationStatus>(result__)
        }
    }
    pub fn DeviceInputNode(&self) -> ::windows_core::Result<AudioDeviceInputNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceInputNode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioDeviceInputNode>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::Interface::cast::<ICreateAudioDeviceInputNodeResult2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for CreateAudioDeviceInputNodeResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CreateAudioDeviceInputNodeResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CreateAudioDeviceInputNodeResult {}
impl ::core::fmt::Debug for CreateAudioDeviceInputNodeResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreateAudioDeviceInputNodeResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CreateAudioDeviceInputNodeResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.CreateAudioDeviceInputNodeResult;{16eec7a8-1ca7-40ef-91a4-d346e0aa1bba})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CreateAudioDeviceInputNodeResult {
    type Vtable = ICreateAudioDeviceInputNodeResult_Vtbl;
    const IID: ::windows_core::GUID = <ICreateAudioDeviceInputNodeResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CreateAudioDeviceInputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateAudioDeviceInputNodeResult";
}
impl ::core::convert::From<CreateAudioDeviceInputNodeResult> for ::windows_core::IUnknown {
    fn from(value: CreateAudioDeviceInputNodeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CreateAudioDeviceInputNodeResult> for ::windows_core::IUnknown {
    fn from(value: &CreateAudioDeviceInputNodeResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CreateAudioDeviceInputNodeResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CreateAudioDeviceInputNodeResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CreateAudioDeviceInputNodeResult> for ::windows_core::IInspectable {
    fn from(value: CreateAudioDeviceInputNodeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CreateAudioDeviceInputNodeResult> for ::windows_core::IInspectable {
    fn from(value: &CreateAudioDeviceInputNodeResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CreateAudioDeviceInputNodeResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CreateAudioDeviceInputNodeResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CreateAudioDeviceInputNodeResult {}
unsafe impl ::core::marker::Sync for CreateAudioDeviceInputNodeResult {}
#[repr(transparent)]
pub struct CreateAudioDeviceOutputNodeResult(::windows_core::IUnknown);
impl CreateAudioDeviceOutputNodeResult {
    pub fn Status(&self) -> ::windows_core::Result<AudioDeviceNodeCreationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AudioDeviceNodeCreationStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioDeviceNodeCreationStatus>(result__)
        }
    }
    pub fn DeviceOutputNode(&self) -> ::windows_core::Result<AudioDeviceOutputNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceOutputNode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioDeviceOutputNode>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::Interface::cast::<ICreateAudioDeviceOutputNodeResult2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for CreateAudioDeviceOutputNodeResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CreateAudioDeviceOutputNodeResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CreateAudioDeviceOutputNodeResult {}
impl ::core::fmt::Debug for CreateAudioDeviceOutputNodeResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreateAudioDeviceOutputNodeResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CreateAudioDeviceOutputNodeResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.CreateAudioDeviceOutputNodeResult;{f7776d27-1d9a-47f7-9cd4-2859cc1b7bff})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CreateAudioDeviceOutputNodeResult {
    type Vtable = ICreateAudioDeviceOutputNodeResult_Vtbl;
    const IID: ::windows_core::GUID = <ICreateAudioDeviceOutputNodeResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CreateAudioDeviceOutputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateAudioDeviceOutputNodeResult";
}
impl ::core::convert::From<CreateAudioDeviceOutputNodeResult> for ::windows_core::IUnknown {
    fn from(value: CreateAudioDeviceOutputNodeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CreateAudioDeviceOutputNodeResult> for ::windows_core::IUnknown {
    fn from(value: &CreateAudioDeviceOutputNodeResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CreateAudioDeviceOutputNodeResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CreateAudioDeviceOutputNodeResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CreateAudioDeviceOutputNodeResult> for ::windows_core::IInspectable {
    fn from(value: CreateAudioDeviceOutputNodeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CreateAudioDeviceOutputNodeResult> for ::windows_core::IInspectable {
    fn from(value: &CreateAudioDeviceOutputNodeResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CreateAudioDeviceOutputNodeResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CreateAudioDeviceOutputNodeResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CreateAudioDeviceOutputNodeResult {}
unsafe impl ::core::marker::Sync for CreateAudioDeviceOutputNodeResult {}
#[repr(transparent)]
pub struct CreateAudioFileInputNodeResult(::windows_core::IUnknown);
impl CreateAudioFileInputNodeResult {
    pub fn Status(&self) -> ::windows_core::Result<AudioFileNodeCreationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AudioFileNodeCreationStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioFileNodeCreationStatus>(result__)
        }
    }
    pub fn FileInputNode(&self) -> ::windows_core::Result<AudioFileInputNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FileInputNode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioFileInputNode>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::Interface::cast::<ICreateAudioFileInputNodeResult2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for CreateAudioFileInputNodeResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CreateAudioFileInputNodeResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CreateAudioFileInputNodeResult {}
impl ::core::fmt::Debug for CreateAudioFileInputNodeResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreateAudioFileInputNodeResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CreateAudioFileInputNodeResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.CreateAudioFileInputNodeResult;{ce83d61c-e297-4c50-9ce7-1c7a69d6bd09})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CreateAudioFileInputNodeResult {
    type Vtable = ICreateAudioFileInputNodeResult_Vtbl;
    const IID: ::windows_core::GUID = <ICreateAudioFileInputNodeResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CreateAudioFileInputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateAudioFileInputNodeResult";
}
impl ::core::convert::From<CreateAudioFileInputNodeResult> for ::windows_core::IUnknown {
    fn from(value: CreateAudioFileInputNodeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CreateAudioFileInputNodeResult> for ::windows_core::IUnknown {
    fn from(value: &CreateAudioFileInputNodeResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CreateAudioFileInputNodeResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CreateAudioFileInputNodeResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CreateAudioFileInputNodeResult> for ::windows_core::IInspectable {
    fn from(value: CreateAudioFileInputNodeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CreateAudioFileInputNodeResult> for ::windows_core::IInspectable {
    fn from(value: &CreateAudioFileInputNodeResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CreateAudioFileInputNodeResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CreateAudioFileInputNodeResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CreateAudioFileInputNodeResult {}
unsafe impl ::core::marker::Sync for CreateAudioFileInputNodeResult {}
#[repr(transparent)]
pub struct CreateAudioFileOutputNodeResult(::windows_core::IUnknown);
impl CreateAudioFileOutputNodeResult {
    pub fn Status(&self) -> ::windows_core::Result<AudioFileNodeCreationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AudioFileNodeCreationStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioFileNodeCreationStatus>(result__)
        }
    }
    pub fn FileOutputNode(&self) -> ::windows_core::Result<AudioFileOutputNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FileOutputNode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioFileOutputNode>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::Interface::cast::<ICreateAudioFileOutputNodeResult2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for CreateAudioFileOutputNodeResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CreateAudioFileOutputNodeResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CreateAudioFileOutputNodeResult {}
impl ::core::fmt::Debug for CreateAudioFileOutputNodeResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreateAudioFileOutputNodeResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CreateAudioFileOutputNodeResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.CreateAudioFileOutputNodeResult;{47d6ba7b-e909-453f-866e-5540cda734ff})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CreateAudioFileOutputNodeResult {
    type Vtable = ICreateAudioFileOutputNodeResult_Vtbl;
    const IID: ::windows_core::GUID = <ICreateAudioFileOutputNodeResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CreateAudioFileOutputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateAudioFileOutputNodeResult";
}
impl ::core::convert::From<CreateAudioFileOutputNodeResult> for ::windows_core::IUnknown {
    fn from(value: CreateAudioFileOutputNodeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CreateAudioFileOutputNodeResult> for ::windows_core::IUnknown {
    fn from(value: &CreateAudioFileOutputNodeResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CreateAudioFileOutputNodeResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CreateAudioFileOutputNodeResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CreateAudioFileOutputNodeResult> for ::windows_core::IInspectable {
    fn from(value: CreateAudioFileOutputNodeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CreateAudioFileOutputNodeResult> for ::windows_core::IInspectable {
    fn from(value: &CreateAudioFileOutputNodeResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CreateAudioFileOutputNodeResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CreateAudioFileOutputNodeResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CreateAudioFileOutputNodeResult {}
unsafe impl ::core::marker::Sync for CreateAudioFileOutputNodeResult {}
#[repr(transparent)]
pub struct CreateAudioGraphResult(::windows_core::IUnknown);
impl CreateAudioGraphResult {
    pub fn Status(&self) -> ::windows_core::Result<AudioGraphCreationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AudioGraphCreationStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioGraphCreationStatus>(result__)
        }
    }
    pub fn Graph(&self) -> ::windows_core::Result<AudioGraph> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Graph)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioGraph>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::Interface::cast::<ICreateAudioGraphResult2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for CreateAudioGraphResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CreateAudioGraphResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CreateAudioGraphResult {}
impl ::core::fmt::Debug for CreateAudioGraphResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreateAudioGraphResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CreateAudioGraphResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.CreateAudioGraphResult;{5453ef7e-7bde-4b76-bb5d-48f79cfc8c0b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CreateAudioGraphResult {
    type Vtable = ICreateAudioGraphResult_Vtbl;
    const IID: ::windows_core::GUID = <ICreateAudioGraphResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CreateAudioGraphResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateAudioGraphResult";
}
impl ::core::convert::From<CreateAudioGraphResult> for ::windows_core::IUnknown {
    fn from(value: CreateAudioGraphResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CreateAudioGraphResult> for ::windows_core::IUnknown {
    fn from(value: &CreateAudioGraphResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CreateAudioGraphResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CreateAudioGraphResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CreateAudioGraphResult> for ::windows_core::IInspectable {
    fn from(value: CreateAudioGraphResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CreateAudioGraphResult> for ::windows_core::IInspectable {
    fn from(value: &CreateAudioGraphResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CreateAudioGraphResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CreateAudioGraphResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CreateAudioGraphResult {}
unsafe impl ::core::marker::Sync for CreateAudioGraphResult {}
#[repr(transparent)]
pub struct CreateMediaSourceAudioInputNodeResult(::windows_core::IUnknown);
impl CreateMediaSourceAudioInputNodeResult {
    pub fn Status(&self) -> ::windows_core::Result<MediaSourceAudioInputNodeCreationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaSourceAudioInputNodeCreationStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaSourceAudioInputNodeCreationStatus>(result__)
        }
    }
    pub fn Node(&self) -> ::windows_core::Result<MediaSourceAudioInputNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Node)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaSourceAudioInputNode>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::Interface::cast::<ICreateMediaSourceAudioInputNodeResult2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for CreateMediaSourceAudioInputNodeResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CreateMediaSourceAudioInputNodeResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CreateMediaSourceAudioInputNodeResult {}
impl ::core::fmt::Debug for CreateMediaSourceAudioInputNodeResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreateMediaSourceAudioInputNodeResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CreateMediaSourceAudioInputNodeResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.CreateMediaSourceAudioInputNodeResult;{46a658a3-53c0-4d59-9e51-cc1d1044a4c4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CreateMediaSourceAudioInputNodeResult {
    type Vtable = ICreateMediaSourceAudioInputNodeResult_Vtbl;
    const IID: ::windows_core::GUID = <ICreateMediaSourceAudioInputNodeResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CreateMediaSourceAudioInputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateMediaSourceAudioInputNodeResult";
}
impl ::core::convert::From<CreateMediaSourceAudioInputNodeResult> for ::windows_core::IUnknown {
    fn from(value: CreateMediaSourceAudioInputNodeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CreateMediaSourceAudioInputNodeResult> for ::windows_core::IUnknown {
    fn from(value: &CreateMediaSourceAudioInputNodeResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CreateMediaSourceAudioInputNodeResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CreateMediaSourceAudioInputNodeResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CreateMediaSourceAudioInputNodeResult> for ::windows_core::IInspectable {
    fn from(value: CreateMediaSourceAudioInputNodeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CreateMediaSourceAudioInputNodeResult> for ::windows_core::IInspectable {
    fn from(value: &CreateMediaSourceAudioInputNodeResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CreateMediaSourceAudioInputNodeResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CreateMediaSourceAudioInputNodeResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CreateMediaSourceAudioInputNodeResult {}
unsafe impl ::core::marker::Sync for CreateMediaSourceAudioInputNodeResult {}
#[repr(transparent)]
pub struct EchoEffectDefinition(::windows_core::IUnknown);
impl EchoEffectDefinition {
    #[cfg(feature = "winrt-media")]
    pub fn ActivatableClassId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatableClassId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IPropertySet> {
        let this = &::windows_core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IPropertySet>(result__)
        }
    }
    pub fn SetWetDryMix(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetWetDryMix)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WetDryMix(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).WetDryMix)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetFeedback(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFeedback)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Feedback(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Feedback)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetDelay(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDelay)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Delay(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Delay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, AudioGraph>>(audiograph: Param0) -> ::windows_core::Result<EchoEffectDefinition> {
        Self::IEchoEffectDefinitionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), audiograph.into_param().abi(), result__.as_mut_ptr()).from_abi::<EchoEffectDefinition>(result__)
        })
    }
    pub fn IEchoEffectDefinitionFactory<R, F: FnOnce(&IEchoEffectDefinitionFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EchoEffectDefinition, IEchoEffectDefinitionFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EchoEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EchoEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EchoEffectDefinition {}
impl ::core::fmt::Debug for EchoEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EchoEffectDefinition").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EchoEffectDefinition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.EchoEffectDefinition;{0e4d3faa-36b8-4c91-b9da-11f44a8a6610})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EchoEffectDefinition {
    type Vtable = IEchoEffectDefinition_Vtbl;
    const IID: ::windows_core::GUID = <IEchoEffectDefinition as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EchoEffectDefinition {
    const NAME: &'static str = "Windows.Media.Audio.EchoEffectDefinition";
}
impl ::core::convert::From<EchoEffectDefinition> for ::windows_core::IUnknown {
    fn from(value: EchoEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EchoEffectDefinition> for ::windows_core::IUnknown {
    fn from(value: &EchoEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EchoEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EchoEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EchoEffectDefinition> for ::windows_core::IInspectable {
    fn from(value: EchoEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EchoEffectDefinition> for ::windows_core::IInspectable {
    fn from(value: &EchoEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EchoEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EchoEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::convert::TryFrom<EchoEffectDefinition> for super::Effects::IAudioEffectDefinition {
    type Error = ::windows_core::Error;
    fn try_from(value: EchoEffectDefinition) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::convert::TryFrom<&EchoEffectDefinition> for super::Effects::IAudioEffectDefinition {
    type Error = ::windows_core::Error;
    fn try_from(value: &EchoEffectDefinition) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-media")]
impl<'a> ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition> for EchoEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, super::Effects::IAudioEffectDefinition> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-media")]
impl<'a> ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition> for &EchoEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, super::Effects::IAudioEffectDefinition> {
        ::core::convert::TryInto::<super::Effects::IAudioEffectDefinition>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for EchoEffectDefinition {}
unsafe impl ::core::marker::Sync for EchoEffectDefinition {}
#[repr(transparent)]
pub struct EqualizerBand(::windows_core::IUnknown);
impl EqualizerBand {
    pub fn Bandwidth(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Bandwidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetBandwidth(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBandwidth)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FrequencyCenter(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).FrequencyCenter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetFrequencyCenter(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFrequencyCenter)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Gain(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Gain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for EqualizerBand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EqualizerBand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EqualizerBand {}
impl ::core::fmt::Debug for EqualizerBand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EqualizerBand").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EqualizerBand {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.EqualizerBand;{c00a5a6a-262d-4b85-9bb7-43280b62ed0c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EqualizerBand {
    type Vtable = IEqualizerBand_Vtbl;
    const IID: ::windows_core::GUID = <IEqualizerBand as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EqualizerBand {
    const NAME: &'static str = "Windows.Media.Audio.EqualizerBand";
}
impl ::core::convert::From<EqualizerBand> for ::windows_core::IUnknown {
    fn from(value: EqualizerBand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EqualizerBand> for ::windows_core::IUnknown {
    fn from(value: &EqualizerBand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EqualizerBand {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EqualizerBand {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EqualizerBand> for ::windows_core::IInspectable {
    fn from(value: EqualizerBand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EqualizerBand> for ::windows_core::IInspectable {
    fn from(value: &EqualizerBand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EqualizerBand {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EqualizerBand {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EqualizerBand {}
unsafe impl ::core::marker::Sync for EqualizerBand {}
#[repr(transparent)]
pub struct EqualizerEffectDefinition(::windows_core::IUnknown);
impl EqualizerEffectDefinition {
    #[cfg(feature = "winrt-media")]
    pub fn ActivatableClassId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatableClassId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IPropertySet> {
        let this = &::windows_core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IPropertySet>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Bands(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<EqualizerBand>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Bands)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<EqualizerBand>>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, AudioGraph>>(audiograph: Param0) -> ::windows_core::Result<EqualizerEffectDefinition> {
        Self::IEqualizerEffectDefinitionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), audiograph.into_param().abi(), result__.as_mut_ptr()).from_abi::<EqualizerEffectDefinition>(result__)
        })
    }
    pub fn IEqualizerEffectDefinitionFactory<R, F: FnOnce(&IEqualizerEffectDefinitionFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EqualizerEffectDefinition, IEqualizerEffectDefinitionFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EqualizerEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EqualizerEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EqualizerEffectDefinition {}
impl ::core::fmt::Debug for EqualizerEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EqualizerEffectDefinition").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EqualizerEffectDefinition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.EqualizerEffectDefinition;{023f6f1f-83fe-449a-a822-c696442d16b0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EqualizerEffectDefinition {
    type Vtable = IEqualizerEffectDefinition_Vtbl;
    const IID: ::windows_core::GUID = <IEqualizerEffectDefinition as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EqualizerEffectDefinition {
    const NAME: &'static str = "Windows.Media.Audio.EqualizerEffectDefinition";
}
impl ::core::convert::From<EqualizerEffectDefinition> for ::windows_core::IUnknown {
    fn from(value: EqualizerEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EqualizerEffectDefinition> for ::windows_core::IUnknown {
    fn from(value: &EqualizerEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EqualizerEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EqualizerEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EqualizerEffectDefinition> for ::windows_core::IInspectable {
    fn from(value: EqualizerEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EqualizerEffectDefinition> for ::windows_core::IInspectable {
    fn from(value: &EqualizerEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EqualizerEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EqualizerEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::convert::TryFrom<EqualizerEffectDefinition> for super::Effects::IAudioEffectDefinition {
    type Error = ::windows_core::Error;
    fn try_from(value: EqualizerEffectDefinition) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::convert::TryFrom<&EqualizerEffectDefinition> for super::Effects::IAudioEffectDefinition {
    type Error = ::windows_core::Error;
    fn try_from(value: &EqualizerEffectDefinition) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-media")]
impl<'a> ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition> for EqualizerEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, super::Effects::IAudioEffectDefinition> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-media")]
impl<'a> ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition> for &EqualizerEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, super::Effects::IAudioEffectDefinition> {
        ::core::convert::TryInto::<super::Effects::IAudioEffectDefinition>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for EqualizerEffectDefinition {}
unsafe impl ::core::marker::Sync for EqualizerEffectDefinition {}
#[repr(transparent)]
pub struct FrameInputNodeQuantumStartedEventArgs(::windows_core::IUnknown);
impl FrameInputNodeQuantumStartedEventArgs {
    pub fn RequiredSamples(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).RequiredSamples)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
}
impl ::core::clone::Clone for FrameInputNodeQuantumStartedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameInputNodeQuantumStartedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameInputNodeQuantumStartedEventArgs {}
impl ::core::fmt::Debug for FrameInputNodeQuantumStartedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FrameInputNodeQuantumStartedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FrameInputNodeQuantumStartedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.FrameInputNodeQuantumStartedEventArgs;{3d9bd498-a306-4f06-bd9f-e9efc8226304})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FrameInputNodeQuantumStartedEventArgs {
    type Vtable = IFrameInputNodeQuantumStartedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IFrameInputNodeQuantumStartedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FrameInputNodeQuantumStartedEventArgs {
    const NAME: &'static str = "Windows.Media.Audio.FrameInputNodeQuantumStartedEventArgs";
}
impl ::core::convert::From<FrameInputNodeQuantumStartedEventArgs> for ::windows_core::IUnknown {
    fn from(value: FrameInputNodeQuantumStartedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameInputNodeQuantumStartedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &FrameInputNodeQuantumStartedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FrameInputNodeQuantumStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FrameInputNodeQuantumStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FrameInputNodeQuantumStartedEventArgs> for ::windows_core::IInspectable {
    fn from(value: FrameInputNodeQuantumStartedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameInputNodeQuantumStartedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &FrameInputNodeQuantumStartedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FrameInputNodeQuantumStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FrameInputNodeQuantumStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FrameInputNodeQuantumStartedEventArgs {}
unsafe impl ::core::marker::Sync for FrameInputNodeQuantumStartedEventArgs {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioDeviceInputNode(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioDeviceInputNode {
    type Vtable = IAudioDeviceInputNode_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb01b6be1_6f4e_49e2_ac01_559d62beb3a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceInputNode_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-devices")]
    pub Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    Device: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioDeviceOutputNode(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioDeviceOutputNode {
    type Vtable = IAudioDeviceOutputNode_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x362edbff_ff1c_4434_9e0f_bd2ef522ac82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceOutputNode_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-devices")]
    pub Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    Device: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioFileInputNode(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioFileInputNode {
    type Vtable = IAudioFileInputNode_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x905b67c8_6f65_4cd4_8890_4694843c276d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFileInputNode_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetPlaybackSpeedFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub PlaybackSpeedFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetEndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LoopCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetLoopCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub SourceFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    SourceFile: usize,
    pub FileCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveFileCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioFileOutputNode(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioFileOutputNode {
    type Vtable = IAudioFileOutputNode_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50e01980_5166_4093_80f8_ada00089e9cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFileOutputNode_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    File: usize,
    #[cfg(feature = "winrt-media")]
    pub FileEncodingProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    FileEncodingProfile: usize,
    #[cfg(feature = "winrt-media")]
    pub FinalizeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    FinalizeAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioFrameCompletedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioFrameCompletedEventArgs {
    type Vtable = IAudioFrameCompletedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdc7c829e_0208_4504_a5a8_f0f268920a65);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Frame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioFrameInputNode(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioFrameInputNode {
    type Vtable = IAudioFrameInputNode_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x01b266c7_fd96_4ff5_a3c5_d27a9bf44237);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameInputNode_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetPlaybackSpeedFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub PlaybackSpeedFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub AddFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frame: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DiscardQueuedFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub QueuedSampleCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub AudioFrameCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveAudioFrameCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub QuantumStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveQuantumStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioFrameOutputNode(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioFrameOutputNode {
    type Vtable = IAudioFrameOutputNode_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb847371b_3299_45f5_88b3_c9d12a3f1cc8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameOutputNode_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioGraph(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioGraph {
    type Vtable = IAudioGraph_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1ad46eed_e48c_4e14_9660_2c4f83e9cdd8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraph_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateFrameInputNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-media")]
    pub CreateFrameInputNodeWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    CreateFrameInputNodeWithFormat: usize,
    #[cfg(feature = "winrt-media")]
    pub CreateDeviceInputNodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::Capture::MediaCategory, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    CreateDeviceInputNodeAsync: usize,
    #[cfg(all(feature = "winrt-media", feature = "winrt-media"))]
    pub CreateDeviceInputNodeWithFormatAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::Capture::MediaCategory, encodingproperties: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-media", feature = "winrt-media")))]
    CreateDeviceInputNodeWithFormatAsync: usize,
    #[cfg(all(feature = "winrt-devices", feature = "winrt-media", feature = "winrt-media"))]
    pub CreateDeviceInputNodeWithFormatOnDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::Capture::MediaCategory, encodingproperties: ::windows_core::RawPtr, device: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-devices", feature = "winrt-media", feature = "winrt-media")))]
    CreateDeviceInputNodeWithFormatOnDeviceAsync: usize,
    pub CreateFrameOutputNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-media")]
    pub CreateFrameOutputNodeWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    CreateFrameOutputNodeWithFormat: usize,
    pub CreateDeviceOutputNodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub CreateFileInputNodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CreateFileInputNodeAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub CreateFileOutputNodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CreateFileOutputNodeAsync: usize,
    #[cfg(all(feature = "winrt-media", feature = "winrt-storage"))]
    pub CreateFileOutputNodeWithFileProfileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows_core::RawPtr, fileencodingprofile: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-media", feature = "winrt-storage")))]
    CreateFileOutputNodeWithFileProfileAsync: usize,
    pub CreateSubmixNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-media")]
    pub CreateSubmixNodeWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    CreateSubmixNodeWithFormat: usize,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ResetAllNodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub QuantumStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveQuantumStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub QuantumProcessed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveQuantumProcessed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub UnrecoverableErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveUnrecoverableErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub CompletedQuantumCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-media")]
    pub EncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    EncodingProperties: usize,
    pub LatencyInSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-devices")]
    pub PrimaryRenderDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    PrimaryRenderDevice: usize,
    pub RenderDeviceAudioProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::AudioProcessing) -> ::windows_core::HRESULT,
    pub SamplesPerQuantum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioGraph2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioGraph2 {
    type Vtable = IAudioGraph2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e4c3bd5_4fc1_45f6_a947_3cd38f4fd839);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraph2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-media")]
    pub CreateFrameInputNodeWithFormatAndEmitter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: ::windows_core::RawPtr, emitter: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    CreateFrameInputNodeWithFormatAndEmitter: usize,
    #[cfg(all(feature = "winrt-devices", feature = "winrt-media", feature = "winrt-media"))]
    pub CreateDeviceInputNodeWithFormatAndEmitterOnDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::Capture::MediaCategory, encodingproperties: ::windows_core::RawPtr, device: ::windows_core::RawPtr, emitter: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-devices", feature = "winrt-media", feature = "winrt-media")))]
    CreateDeviceInputNodeWithFormatAndEmitterOnDeviceAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub CreateFileInputNodeWithEmitterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows_core::RawPtr, emitter: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CreateFileInputNodeWithEmitterAsync: usize,
    #[cfg(feature = "winrt-media")]
    pub CreateSubmixNodeWithFormatAndEmitter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: ::windows_core::RawPtr, emitter: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    CreateSubmixNodeWithFormatAndEmitter: usize,
    pub CreateBatchUpdater: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioGraph3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioGraph3 {
    type Vtable = IAudioGraph3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xddcd25ae_1185_42a7_831d_6a9b0fc86820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraph3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-media")]
    pub CreateMediaSourceAudioInputNodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediasource: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    CreateMediaSourceAudioInputNodeAsync: usize,
    #[cfg(feature = "winrt-media")]
    pub CreateMediaSourceAudioInputNodeWithEmitterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediasource: ::windows_core::RawPtr, emitter: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    CreateMediaSourceAudioInputNodeWithEmitterAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioGraphConnection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioGraphConnection {
    type Vtable = IAudioGraphConnection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x763070ed_d04e_4fac_b233_600b42edd469);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraphConnection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Destination: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub Gain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioGraphSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioGraphSettings {
    type Vtable = IAudioGraphSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d59647f_e6fe_4628_84f8_9d8bdba25785);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraphSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-media")]
    pub EncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    EncodingProperties: usize,
    #[cfg(feature = "winrt-media")]
    pub SetEncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    SetEncodingProperties: usize,
    #[cfg(feature = "winrt-devices")]
    pub PrimaryRenderDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    PrimaryRenderDevice: usize,
    #[cfg(feature = "winrt-devices")]
    pub SetPrimaryRenderDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    SetPrimaryRenderDevice: usize,
    pub QuantumSizeSelectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut QuantumSizeSelectionMode) -> ::windows_core::HRESULT,
    pub SetQuantumSizeSelectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: QuantumSizeSelectionMode) -> ::windows_core::HRESULT,
    pub DesiredSamplesPerQuantum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetDesiredSamplesPerQuantum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-media")]
    pub AudioRenderCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Render::AudioRenderCategory) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    AudioRenderCategory: usize,
    #[cfg(feature = "winrt-media")]
    pub SetAudioRenderCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Render::AudioRenderCategory) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    SetAudioRenderCategory: usize,
    pub DesiredRenderDeviceAudioProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::AudioProcessing) -> ::windows_core::HRESULT,
    pub SetDesiredRenderDeviceAudioProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::AudioProcessing) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioGraphSettings2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioGraphSettings2 {
    type Vtable = IAudioGraphSettings2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72919787_4dab_46e3_b4c9_d8e1a2636062);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraphSettings2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetMaxPlaybackSpeedFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub MaxPlaybackSpeedFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioGraphSettingsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioGraphSettingsFactory {
    type Vtable = IAudioGraphSettingsFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa5d91cc6_c2eb_4a61_a214_1d66d75f83da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraphSettingsFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-media")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiorendercategory: super::Render::AudioRenderCategory, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioGraphStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioGraphStatics {
    type Vtable = IAudioGraphStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x76ec3132_e159_4ab7_a82a_17beb4b31e94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraphStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioGraphUnrecoverableErrorOccurredEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioGraphUnrecoverableErrorOccurredEventArgs {
    type Vtable = IAudioGraphUnrecoverableErrorOccurredEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc3d9cbe0_3ff6_4fb3_b262_50d435c55423);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraphUnrecoverableErrorOccurredEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioGraphUnrecoverableError) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAudioInputNode(::windows_core::IUnknown);
impl IAudioInputNode {
    #[cfg(feature = "winrt-foundation")]
    pub fn OutgoingConnections(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OutgoingConnections)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<AudioGraphConnection>>(result__)
        }
    }
    pub fn AddOutgoingConnection<'a, Param0: ::windows_core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddOutgoingConnection)(::windows_core::Interface::as_raw(this), destination.into_param().abi()).ok() }
    }
    pub fn AddOutgoingConnectionWithGain<'a, Param0: ::windows_core::IntoParam<'a, IAudioNode>>(&self, destination: Param0, gain: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddOutgoingConnectionWithGain)(::windows_core::Interface::as_raw(this), destination.into_param().abi(), gain).ok() }
    }
    pub fn RemoveOutgoingConnection<'a, Param0: ::windows_core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOutgoingConnection)(::windows_core::Interface::as_raw(this), destination.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
    pub fn EffectDefinitions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EffectDefinitions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOutgoingGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OutgoingGain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn EncodingProperties(&self) -> ::windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EncodingProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ConsumeInput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetConsumeInput)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Reset)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).DisableEffectsByDefinition)(::windows_core::Interface::as_raw(this), definition.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).EnableEffectsByDefinition)(::windows_core::Interface::as_raw(this), definition.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::convert::From<IAudioInputNode> for ::windows_core::IUnknown {
    fn from(value: IAudioInputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioInputNode> for ::windows_core::IUnknown {
    fn from(value: &IAudioInputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAudioInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAudioInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAudioInputNode> for ::windows_core::IInspectable {
    fn from(value: IAudioInputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioInputNode> for ::windows_core::IInspectable {
    fn from(value: &IAudioInputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IAudioInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IAudioInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IAudioInputNode> for IAudioNode {
    type Error = ::windows_core::Error;
    fn try_from(value: IAudioInputNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAudioInputNode> for IAudioNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &IAudioInputNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioNode> for IAudioInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioNode> for &IAudioInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioNode> {
        ::core::convert::TryInto::<IAudioNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<IAudioInputNode> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: IAudioInputNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAudioInputNode> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &IAudioInputNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for IAudioInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &IAudioInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IAudioInputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioInputNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioInputNode {}
impl ::core::fmt::Debug for IAudioInputNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioInputNode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IAudioInputNode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{d148005c-8428-4784-b7fd-a99d468c5d20}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IAudioInputNode {
    type Vtable = IAudioInputNode_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd148005c_8428_4784_b7fd_a99d468c5d20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioInputNode_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub OutgoingConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    OutgoingConnections: usize,
    pub AddOutgoingConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destination: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AddOutgoingConnectionWithGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destination: ::windows_core::RawPtr, gain: f64) -> ::windows_core::HRESULT,
    pub RemoveOutgoingConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destination: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAudioInputNode2(::windows_core::IUnknown);
impl IAudioInputNode2 {
    pub fn Emitter(&self) -> ::windows_core::Result<AudioNodeEmitter> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Emitter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioNodeEmitter>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn OutgoingConnections(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = &::windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OutgoingConnections)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<AudioGraphConnection>>(result__)
        }
    }
    pub fn AddOutgoingConnection<'a, Param0: ::windows_core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AddOutgoingConnection)(::windows_core::Interface::as_raw(this), destination.into_param().abi()).ok() }
    }
    pub fn AddOutgoingConnectionWithGain<'a, Param0: ::windows_core::IntoParam<'a, IAudioNode>>(&self, destination: Param0, gain: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AddOutgoingConnectionWithGain)(::windows_core::Interface::as_raw(this), destination.into_param().abi(), gain).ok() }
    }
    pub fn RemoveOutgoingConnection<'a, Param0: ::windows_core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOutgoingConnection)(::windows_core::Interface::as_raw(this), destination.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
    pub fn EffectDefinitions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EffectDefinitions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOutgoingGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OutgoingGain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn EncodingProperties(&self) -> ::windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EncodingProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ConsumeInput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetConsumeInput)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Reset)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).DisableEffectsByDefinition)(::windows_core::Interface::as_raw(this), definition.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).EnableEffectsByDefinition)(::windows_core::Interface::as_raw(this), definition.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::convert::From<IAudioInputNode2> for ::windows_core::IUnknown {
    fn from(value: IAudioInputNode2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioInputNode2> for ::windows_core::IUnknown {
    fn from(value: &IAudioInputNode2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAudioInputNode2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAudioInputNode2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAudioInputNode2> for ::windows_core::IInspectable {
    fn from(value: IAudioInputNode2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioInputNode2> for ::windows_core::IInspectable {
    fn from(value: &IAudioInputNode2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IAudioInputNode2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IAudioInputNode2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IAudioInputNode2> for IAudioInputNode {
    type Error = ::windows_core::Error;
    fn try_from(value: IAudioInputNode2) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAudioInputNode2> for IAudioInputNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &IAudioInputNode2) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioInputNode> for IAudioInputNode2 {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioInputNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioInputNode> for &IAudioInputNode2 {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioInputNode> {
        ::core::convert::TryInto::<IAudioInputNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<IAudioInputNode2> for IAudioNode {
    type Error = ::windows_core::Error;
    fn try_from(value: IAudioInputNode2) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAudioInputNode2> for IAudioNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &IAudioInputNode2) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioNode> for IAudioInputNode2 {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioNode> for &IAudioInputNode2 {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioNode> {
        ::core::convert::TryInto::<IAudioNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<IAudioInputNode2> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: IAudioInputNode2) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAudioInputNode2> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &IAudioInputNode2) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for IAudioInputNode2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &IAudioInputNode2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IAudioInputNode2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioInputNode2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioInputNode2 {}
impl ::core::fmt::Debug for IAudioInputNode2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioInputNode2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IAudioInputNode2 {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{905156b7-ca68-4c6d-a8bc-e3ee17fe3fd2}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IAudioInputNode2 {
    type Vtable = IAudioInputNode2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x905156b7_ca68_4c6d_a8bc_e3ee17fe3fd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioInputNode2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Emitter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAudioNode(::windows_core::IUnknown);
impl IAudioNode {
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
    pub fn EffectDefinitions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EffectDefinitions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOutgoingGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OutgoingGain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn EncodingProperties(&self) -> ::windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EncodingProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ConsumeInput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetConsumeInput)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Reset)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).DisableEffectsByDefinition)(::windows_core::Interface::as_raw(this), definition.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).EnableEffectsByDefinition)(::windows_core::Interface::as_raw(this), definition.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::convert::From<IAudioNode> for ::windows_core::IUnknown {
    fn from(value: IAudioNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioNode> for ::windows_core::IUnknown {
    fn from(value: &IAudioNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAudioNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAudioNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAudioNode> for ::windows_core::IInspectable {
    fn from(value: IAudioNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioNode> for ::windows_core::IInspectable {
    fn from(value: &IAudioNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IAudioNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IAudioNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IAudioNode> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: IAudioNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAudioNode> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &IAudioNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for IAudioNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &IAudioNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IAudioNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioNode {}
impl ::core::fmt::Debug for IAudioNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioNode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IAudioNode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{15389d7f-dbd8-4819-bf03-668e9357cd6d}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IAudioNode {
    type Vtable = IAudioNode_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x15389d7f_dbd8_4819_bf03_668e9357cd6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNode_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
    pub EffectDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-media")))]
    EffectDefinitions: usize,
    pub SetOutgoingGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub OutgoingGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-media")]
    pub EncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    EncodingProperties: usize,
    pub ConsumeInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetConsumeInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-media")]
    pub DisableEffectsByDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, definition: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    DisableEffectsByDefinition: usize,
    #[cfg(feature = "winrt-media")]
    pub EnableEffectsByDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, definition: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    EnableEffectsByDefinition: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioNodeEmitter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioNodeEmitter {
    type Vtable = IAudioNodeEmitter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3676971d_880a_47b8_adf7_1323a9d965be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Position: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetPosition: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Direction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Direction: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SetDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetDirection: usize,
    pub Shape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DecayModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Gain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub DistanceScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetDistanceScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub DopplerScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetDopplerScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub DopplerVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    DopplerVelocity: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SetDopplerVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetDopplerVelocity: usize,
    pub IsDopplerDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioNodeEmitter2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioNodeEmitter2 {
    type Vtable = IAudioNodeEmitter2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4ab6eecb_ec29_47f8_818c_b6b660a5aeb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitter2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SpatialAudioModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialAudioModel) -> ::windows_core::HRESULT,
    pub SetSpatialAudioModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SpatialAudioModel) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioNodeEmitterConeProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioNodeEmitterConeProperties {
    type Vtable = IAudioNodeEmitterConeProperties_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe99b2cee_02ca_4375_9326_0c6ae4bcdfb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterConeProperties_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InnerAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub OuterAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub OuterAngleGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioNodeEmitterDecayModel(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioNodeEmitterDecayModel {
    type Vtable = IAudioNodeEmitterDecayModel_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d1d5af7_0d53_4fa9_bd84_d5816a86f3ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterDecayModel_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioNodeEmitterDecayKind) -> ::windows_core::HRESULT,
    pub MinGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub MaxGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub NaturalProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioNodeEmitterDecayModelStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioNodeEmitterDecayModelStatics {
    type Vtable = IAudioNodeEmitterDecayModelStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc7787ca8_f178_462f_bc81_8dd5cbe5dae8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterDecayModelStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateNatural: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mingain: f64, maxgain: f64, unitygaindistance: f64, cutoffdistance: f64, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateCustom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mingain: f64, maxgain: f64, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioNodeEmitterFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioNodeEmitterFactory {
    type Vtable = IAudioNodeEmitterFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfdc8489a_6ad6_4ce4_b7f7_a99370df7ee9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateAudioNodeEmitter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows_core::RawPtr, decaymodel: ::windows_core::RawPtr, settings: AudioNodeEmitterSettings, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioNodeEmitterNaturalDecayModelProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioNodeEmitterNaturalDecayModelProperties {
    type Vtable = IAudioNodeEmitterNaturalDecayModelProperties_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x48934bcf_cf2c_4efc_9331_75bd22df1f0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterNaturalDecayModelProperties_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub UnityGainDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub CutoffDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioNodeEmitterShape(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioNodeEmitterShape {
    type Vtable = IAudioNodeEmitterShape_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea0311c5_e73d_44bc_859c_45553bbc4828);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterShape_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioNodeEmitterShapeKind) -> ::windows_core::HRESULT,
    pub ConeProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioNodeEmitterShapeStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioNodeEmitterShapeStatics {
    type Vtable = IAudioNodeEmitterShapeStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x57bb2771_ffa5_4b86_a779_e264aeb9145f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterShapeStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateCone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, innerangle: f64, outerangle: f64, outeranglegain: f64, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateOmnidirectional: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioNodeListener(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioNodeListener {
    type Vtable = IAudioNodeListener_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd9722e16_0c0a_41da_b755_6c77835fb1eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeListener_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Position: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetPosition: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Quaternion) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Orientation: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SetOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Numerics::Quaternion) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetOrientation: usize,
    pub SpeedOfSound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetSpeedOfSound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub DopplerVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    DopplerVelocity: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SetDopplerVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetDopplerVelocity: usize,
}
#[repr(transparent)]
pub struct IAudioNodeWithListener(::windows_core::IUnknown);
impl IAudioNodeWithListener {
    pub fn SetListener<'a, Param0: ::windows_core::IntoParam<'a, AudioNodeListener>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetListener)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Listener(&self) -> ::windows_core::Result<AudioNodeListener> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Listener)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioNodeListener>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
    pub fn EffectDefinitions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EffectDefinitions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOutgoingGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OutgoingGain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn EncodingProperties(&self) -> ::windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EncodingProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ConsumeInput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetConsumeInput)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Reset)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).DisableEffectsByDefinition)(::windows_core::Interface::as_raw(this), definition.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).EnableEffectsByDefinition)(::windows_core::Interface::as_raw(this), definition.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::convert::From<IAudioNodeWithListener> for ::windows_core::IUnknown {
    fn from(value: IAudioNodeWithListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioNodeWithListener> for ::windows_core::IUnknown {
    fn from(value: &IAudioNodeWithListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAudioNodeWithListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAudioNodeWithListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAudioNodeWithListener> for ::windows_core::IInspectable {
    fn from(value: IAudioNodeWithListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioNodeWithListener> for ::windows_core::IInspectable {
    fn from(value: &IAudioNodeWithListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IAudioNodeWithListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IAudioNodeWithListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IAudioNodeWithListener> for IAudioNode {
    type Error = ::windows_core::Error;
    fn try_from(value: IAudioNodeWithListener) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAudioNodeWithListener> for IAudioNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &IAudioNodeWithListener) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioNode> for IAudioNodeWithListener {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioNode> for &IAudioNodeWithListener {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioNode> {
        ::core::convert::TryInto::<IAudioNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<IAudioNodeWithListener> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: IAudioNodeWithListener) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAudioNodeWithListener> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &IAudioNodeWithListener) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for IAudioNodeWithListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &IAudioNodeWithListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IAudioNodeWithListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioNodeWithListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioNodeWithListener {}
impl ::core::fmt::Debug for IAudioNodeWithListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioNodeWithListener").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IAudioNodeWithListener {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{0e0f907c-79ff-4544-9eeb-01257b15105a}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IAudioNodeWithListener {
    type Vtable = IAudioNodeWithListener_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0e0f907c_79ff_4544_9eeb_01257b15105a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeWithListener_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetListener: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Listener: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioPlaybackConnection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioPlaybackConnection {
    type Vtable = IAudioPlaybackConnection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1a4c1dea_cafc_50e7_8718_ea3f81cbfa51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioPlaybackConnection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioPlaybackConnectionState) -> ::windows_core::HRESULT,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OpenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioPlaybackConnectionOpenResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioPlaybackConnectionOpenResult {
    type Vtable = IAudioPlaybackConnectionOpenResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e656aef_39f9_5fc9_a519_a5bbfd9fe921);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioPlaybackConnectionOpenResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioPlaybackConnectionOpenResultStatus) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioPlaybackConnectionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioPlaybackConnectionStatics {
    type Vtable = IAudioPlaybackConnectionStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe60963a2_69e6_5ffc_9e13_824a85213daf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioPlaybackConnectionStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TryCreateFromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioStateMonitor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioStateMonitor {
    type Vtable = IAudioStateMonitor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d13d136_0199_4cdc_b84e_e72c2b581ece);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStateMonitor_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SoundLevelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSoundLevelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SoundLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::SoundLevel) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioStateMonitorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioStateMonitorStatics {
    type Vtable = IAudioStateMonitorStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6374ea4c_1b3b_4001_94d9_dd225330fa40);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStateMonitorStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateForRenderMonitoring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-media")]
    pub CreateForRenderMonitoringWithCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::Render::AudioRenderCategory, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    CreateForRenderMonitoringWithCategory: usize,
    #[cfg(all(feature = "winrt-media", feature = "winrt-media"))]
    pub CreateForRenderMonitoringWithCategoryAndDeviceRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::Render::AudioRenderCategory, role: super::Devices::AudioDeviceRole, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-media", feature = "winrt-media")))]
    CreateForRenderMonitoringWithCategoryAndDeviceRole: usize,
    #[cfg(feature = "winrt-media")]
    pub CreateForRenderMonitoringWithCategoryAndDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::Render::AudioRenderCategory, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    CreateForRenderMonitoringWithCategoryAndDeviceId: usize,
    pub CreateForCaptureMonitoring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-media")]
    pub CreateForCaptureMonitoringWithCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::Capture::MediaCategory, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    CreateForCaptureMonitoringWithCategory: usize,
    #[cfg(all(feature = "winrt-media", feature = "winrt-media"))]
    pub CreateForCaptureMonitoringWithCategoryAndDeviceRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::Capture::MediaCategory, role: super::Devices::AudioDeviceRole, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-media", feature = "winrt-media")))]
    CreateForCaptureMonitoringWithCategoryAndDeviceRole: usize,
    #[cfg(feature = "winrt-media")]
    pub CreateForCaptureMonitoringWithCategoryAndDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::Capture::MediaCategory, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    CreateForCaptureMonitoringWithCategoryAndDeviceId: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateAudioDeviceInputNodeResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICreateAudioDeviceInputNodeResult {
    type Vtable = ICreateAudioDeviceInputNodeResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x16eec7a8_1ca7_40ef_91a4_d346e0aa1bba);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioDeviceInputNodeResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioDeviceNodeCreationStatus) -> ::windows_core::HRESULT,
    pub DeviceInputNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateAudioDeviceInputNodeResult2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICreateAudioDeviceInputNodeResult2 {
    type Vtable = ICreateAudioDeviceInputNodeResult2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x921c69ce_3f35_41c7_9622_79f608baedc2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioDeviceInputNodeResult2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateAudioDeviceOutputNodeResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICreateAudioDeviceOutputNodeResult {
    type Vtable = ICreateAudioDeviceOutputNodeResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf7776d27_1d9a_47f7_9cd4_2859cc1b7bff);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioDeviceOutputNodeResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioDeviceNodeCreationStatus) -> ::windows_core::HRESULT,
    pub DeviceOutputNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateAudioDeviceOutputNodeResult2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICreateAudioDeviceOutputNodeResult2 {
    type Vtable = ICreateAudioDeviceOutputNodeResult2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4864269f_bdce_4ab1_bd38_fbae93aedaca);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioDeviceOutputNodeResult2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateAudioFileInputNodeResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICreateAudioFileInputNodeResult {
    type Vtable = ICreateAudioFileInputNodeResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xce83d61c_e297_4c50_9ce7_1c7a69d6bd09);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioFileInputNodeResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioFileNodeCreationStatus) -> ::windows_core::HRESULT,
    pub FileInputNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateAudioFileInputNodeResult2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICreateAudioFileInputNodeResult2 {
    type Vtable = ICreateAudioFileInputNodeResult2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf9082020_3d80_4fe0_81c1_768fea7ca7e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioFileInputNodeResult2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateAudioFileOutputNodeResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICreateAudioFileOutputNodeResult {
    type Vtable = ICreateAudioFileOutputNodeResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x47d6ba7b_e909_453f_866e_5540cda734ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioFileOutputNodeResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioFileNodeCreationStatus) -> ::windows_core::HRESULT,
    pub FileOutputNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateAudioFileOutputNodeResult2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICreateAudioFileOutputNodeResult2 {
    type Vtable = ICreateAudioFileOutputNodeResult2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f01b50d_3318_47b3_a60a_1b492be7fc0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioFileOutputNodeResult2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateAudioGraphResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICreateAudioGraphResult {
    type Vtable = ICreateAudioGraphResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5453ef7e_7bde_4b76_bb5d_48f79cfc8c0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioGraphResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioGraphCreationStatus) -> ::windows_core::HRESULT,
    pub Graph: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateAudioGraphResult2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICreateAudioGraphResult2 {
    type Vtable = ICreateAudioGraphResult2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6d738dfc_88c6_4fcb_a534_85cedd4050a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioGraphResult2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateMediaSourceAudioInputNodeResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICreateMediaSourceAudioInputNodeResult {
    type Vtable = ICreateMediaSourceAudioInputNodeResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x46a658a3_53c0_4d59_9e51_cc1d1044a4c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateMediaSourceAudioInputNodeResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaSourceAudioInputNodeCreationStatus) -> ::windows_core::HRESULT,
    pub Node: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateMediaSourceAudioInputNodeResult2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICreateMediaSourceAudioInputNodeResult2 {
    type Vtable = ICreateMediaSourceAudioInputNodeResult2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x63514ce8_6a1a_49e3_97ec_28fd5be114e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateMediaSourceAudioInputNodeResult2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEchoEffectDefinition(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEchoEffectDefinition {
    type Vtable = IEchoEffectDefinition_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0e4d3faa_36b8_4c91_b9da_11f44a8a6610);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEchoEffectDefinition_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetWetDryMix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub WetDryMix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetFeedback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub Feedback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub Delay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEchoEffectDefinitionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEchoEffectDefinitionFactory {
    type Vtable = IEchoEffectDefinitionFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0d4e2257_aaf2_4e86_a54c_fb79db8f6c12);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEchoEffectDefinitionFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiograph: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEqualizerBand(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEqualizerBand {
    type Vtable = IEqualizerBand_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc00a5a6a_262d_4b85_9bb7_43280b62ed0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEqualizerBand_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Bandwidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetBandwidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub FrequencyCenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetFrequencyCenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub Gain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEqualizerEffectDefinition(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEqualizerEffectDefinition {
    type Vtable = IEqualizerEffectDefinition_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x023f6f1f_83fe_449a_a822_c696442d16b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEqualizerEffectDefinition_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Bands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Bands: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEqualizerEffectDefinitionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEqualizerEffectDefinitionFactory {
    type Vtable = IEqualizerEffectDefinitionFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd2876fc4_d410_4eb5_9e69_c9aa1277eaf0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEqualizerEffectDefinitionFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiograph: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameInputNodeQuantumStartedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameInputNodeQuantumStartedEventArgs {
    type Vtable = IFrameInputNodeQuantumStartedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3d9bd498_a306_4f06_bd9f_e9efc8226304);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameInputNodeQuantumStartedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequiredSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILimiterEffectDefinition(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILimiterEffectDefinition {
    type Vtable = ILimiterEffectDefinition_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6b755d19_2603_47ba_bdeb_39055e3486dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILimiterEffectDefinition_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetRelease: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub Release: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetLoudness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub Loudness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILimiterEffectDefinitionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILimiterEffectDefinitionFactory {
    type Vtable = ILimiterEffectDefinitionFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecbae6f1_61ff_45ef_b8f5_48659a57c72d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILimiterEffectDefinitionFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiograph: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSourceAudioInputNode(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaSourceAudioInputNode {
    type Vtable = IMediaSourceAudioInputNode_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x99d8983b_a88a_4041_8e4f_ddbac0c91fd3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceAudioInputNode_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetPlaybackSpeedFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub PlaybackSpeedFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetEndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LoopCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetLoopCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-media")]
    pub MediaSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    MediaSource: usize,
    pub MediaSourceCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveMediaSourceCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IReverbEffectDefinition(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IReverbEffectDefinition {
    type Vtable = IReverbEffectDefinition_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4606aa89_f563_4d0a_8f6e_f0cddff35d84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReverbEffectDefinition_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetWetDryMix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub WetDryMix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetReflectionsDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ReflectionsDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetReverbDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows_core::HRESULT,
    pub ReverbDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub SetRearDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows_core::HRESULT,
    pub RearDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub SetPositionLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows_core::HRESULT,
    pub PositionLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub SetPositionRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows_core::HRESULT,
    pub PositionRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub SetPositionMatrixLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows_core::HRESULT,
    pub PositionMatrixLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub SetPositionMatrixRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows_core::HRESULT,
    pub PositionMatrixRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub SetEarlyDiffusion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows_core::HRESULT,
    pub EarlyDiffusion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub SetLateDiffusion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows_core::HRESULT,
    pub LateDiffusion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub SetLowEQGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows_core::HRESULT,
    pub LowEQGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub SetLowEQCutoff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows_core::HRESULT,
    pub LowEQCutoff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub SetHighEQGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows_core::HRESULT,
    pub HighEQGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub SetHighEQCutoff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows_core::HRESULT,
    pub HighEQCutoff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub SetRoomFilterFreq: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub RoomFilterFreq: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetRoomFilterMain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub RoomFilterMain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetRoomFilterHF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub RoomFilterHF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetReflectionsGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub ReflectionsGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetReverbGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub ReverbGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetDecayTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub DecayTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetDensity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub Density: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetRoomSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub RoomSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetDisableLateField: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub DisableLateField: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IReverbEffectDefinitionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IReverbEffectDefinitionFactory {
    type Vtable = IReverbEffectDefinitionFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa7d5cbfe_100b_4ff0_9da6_dc4e05a759f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReverbEffectDefinitionFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiograph: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISetDefaultSpatialAudioFormatResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISetDefaultSpatialAudioFormatResult {
    type Vtable = ISetDefaultSpatialAudioFormatResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1c2aa511_1400_5e70_9ea9_ae151241e8ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISetDefaultSpatialAudioFormatResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SetDefaultSpatialAudioFormatStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAudioDeviceConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialAudioDeviceConfiguration {
    type Vtable = ISpatialAudioDeviceConfiguration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xee830034_61cf_5749_9da4_10f0fe028199);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioDeviceConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsSpatialAudioSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsSpatialAudioFormatSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subtype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ActiveSpatialAudioFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DefaultSpatialAudioFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDefaultSpatialAudioFormatAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subtype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ConfigurationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveConfigurationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAudioDeviceConfigurationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialAudioDeviceConfigurationStatics {
    type Vtable = ISpatialAudioDeviceConfigurationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3ec37f7b_936d_4e04_9728_2827d9f758c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioDeviceConfigurationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAudioFormatConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialAudioFormatConfiguration {
    type Vtable = ISpatialAudioFormatConfiguration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x32df09a8_50f0_5395_9923_7d44ca71ed6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioFormatConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReportLicenseChangedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subtype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReportConfigurationChangedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subtype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MixedRealityExclusiveModePolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MixedRealitySpatialAudioFormatPolicy) -> ::windows_core::HRESULT,
    pub SetMixedRealityExclusiveModePolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MixedRealitySpatialAudioFormatPolicy) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAudioFormatConfigurationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialAudioFormatConfigurationStatics {
    type Vtable = ISpatialAudioFormatConfigurationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2b5fef71_67c9_4e5f_a35b_41680711f8c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioFormatConfigurationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAudioFormatSubtypeStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialAudioFormatSubtypeStatics {
    type Vtable = ISpatialAudioFormatSubtypeStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb3de8a47_83ee_4266_a945_bedf507afeed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioFormatSubtypeStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub WindowsSonic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DolbyAtmosForHeadphones: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DolbyAtmosForHomeTheater: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DolbyAtmosForSpeakers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DTSHeadphoneX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DTSXUltra: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAudioFormatSubtypeStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialAudioFormatSubtypeStatics2 {
    type Vtable = ISpatialAudioFormatSubtypeStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4565e6cb_d95b_5621_b6af_0e8849c57c80);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioFormatSubtypeStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DTSXForHomeTheater: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct LimiterEffectDefinition(::windows_core::IUnknown);
impl LimiterEffectDefinition {
    #[cfg(feature = "winrt-media")]
    pub fn ActivatableClassId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatableClassId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IPropertySet> {
        let this = &::windows_core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IPropertySet>(result__)
        }
    }
    pub fn SetRelease(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRelease)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Release(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Release)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetLoudness(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLoudness)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Loudness(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Loudness)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, AudioGraph>>(audiograph: Param0) -> ::windows_core::Result<LimiterEffectDefinition> {
        Self::ILimiterEffectDefinitionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), audiograph.into_param().abi(), result__.as_mut_ptr()).from_abi::<LimiterEffectDefinition>(result__)
        })
    }
    pub fn ILimiterEffectDefinitionFactory<R, F: FnOnce(&ILimiterEffectDefinitionFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LimiterEffectDefinition, ILimiterEffectDefinitionFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LimiterEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LimiterEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LimiterEffectDefinition {}
impl ::core::fmt::Debug for LimiterEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LimiterEffectDefinition").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LimiterEffectDefinition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.LimiterEffectDefinition;{6b755d19-2603-47ba-bdeb-39055e3486dc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LimiterEffectDefinition {
    type Vtable = ILimiterEffectDefinition_Vtbl;
    const IID: ::windows_core::GUID = <ILimiterEffectDefinition as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LimiterEffectDefinition {
    const NAME: &'static str = "Windows.Media.Audio.LimiterEffectDefinition";
}
impl ::core::convert::From<LimiterEffectDefinition> for ::windows_core::IUnknown {
    fn from(value: LimiterEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LimiterEffectDefinition> for ::windows_core::IUnknown {
    fn from(value: &LimiterEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LimiterEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LimiterEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LimiterEffectDefinition> for ::windows_core::IInspectable {
    fn from(value: LimiterEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LimiterEffectDefinition> for ::windows_core::IInspectable {
    fn from(value: &LimiterEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LimiterEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LimiterEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::convert::TryFrom<LimiterEffectDefinition> for super::Effects::IAudioEffectDefinition {
    type Error = ::windows_core::Error;
    fn try_from(value: LimiterEffectDefinition) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::convert::TryFrom<&LimiterEffectDefinition> for super::Effects::IAudioEffectDefinition {
    type Error = ::windows_core::Error;
    fn try_from(value: &LimiterEffectDefinition) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-media")]
impl<'a> ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition> for LimiterEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, super::Effects::IAudioEffectDefinition> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-media")]
impl<'a> ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition> for &LimiterEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, super::Effects::IAudioEffectDefinition> {
        ::core::convert::TryInto::<super::Effects::IAudioEffectDefinition>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LimiterEffectDefinition {}
unsafe impl ::core::marker::Sync for LimiterEffectDefinition {}
#[repr(transparent)]
pub struct MediaSourceAudioInputNode(::windows_core::IUnknown);
impl MediaSourceAudioInputNode {
    #[cfg(feature = "winrt-foundation")]
    pub fn OutgoingConnections(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = &::windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OutgoingConnections)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<AudioGraphConnection>>(result__)
        }
    }
    pub fn AddOutgoingConnection<'a, Param0: ::windows_core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AddOutgoingConnection)(::windows_core::Interface::as_raw(this), destination.into_param().abi()).ok() }
    }
    pub fn AddOutgoingConnectionWithGain<'a, Param0: ::windows_core::IntoParam<'a, IAudioNode>>(&self, destination: Param0, gain: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AddOutgoingConnectionWithGain)(::windows_core::Interface::as_raw(this), destination.into_param().abi(), gain).ok() }
    }
    pub fn RemoveOutgoingConnection<'a, Param0: ::windows_core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOutgoingConnection)(::windows_core::Interface::as_raw(this), destination.into_param().abi()).ok() }
    }
    pub fn Emitter(&self) -> ::windows_core::Result<AudioNodeEmitter> {
        let this = &::windows_core::Interface::cast::<IAudioInputNode2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Emitter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioNodeEmitter>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
    pub fn EffectDefinitions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EffectDefinitions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOutgoingGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OutgoingGain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn EncodingProperties(&self) -> ::windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EncodingProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ConsumeInput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetConsumeInput)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Reset)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).DisableEffectsByDefinition)(::windows_core::Interface::as_raw(this), definition.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-media")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).EnableEffectsByDefinition)(::windows_core::Interface::as_raw(this), definition.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPlaybackSpeedFactor(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPlaybackSpeedFactor)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PlaybackSpeedFactor(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackSpeedFactor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn Seek<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, position: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Seek)(::windows_core::Interface::as_raw(this), position.into_param().abi()).ok() }
    }
    pub fn StartTime(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetStartTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn EndTime(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EndTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetEndTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEndTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn LoopCount(&self) -> ::windows_core::Result<::winrt_foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LoopCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<i32>>(result__)
        }
    }
    pub fn SetLoopCount<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<i32>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLoopCount)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn MediaSource(&self) -> ::windows_core::Result<super::Core::MediaSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MediaSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Core::MediaSource>(result__)
        }
    }
    pub fn MediaSourceCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaSourceAudioInputNode, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).MediaSourceCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveMediaSourceCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMediaSourceCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for MediaSourceAudioInputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaSourceAudioInputNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaSourceAudioInputNode {}
impl ::core::fmt::Debug for MediaSourceAudioInputNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaSourceAudioInputNode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaSourceAudioInputNode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.MediaSourceAudioInputNode;{99d8983b-a88a-4041-8e4f-ddbac0c91fd3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaSourceAudioInputNode {
    type Vtable = IMediaSourceAudioInputNode_Vtbl;
    const IID: ::windows_core::GUID = <IMediaSourceAudioInputNode as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaSourceAudioInputNode {
    const NAME: &'static str = "Windows.Media.Audio.MediaSourceAudioInputNode";
}
impl ::core::convert::From<MediaSourceAudioInputNode> for ::windows_core::IUnknown {
    fn from(value: MediaSourceAudioInputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaSourceAudioInputNode> for ::windows_core::IUnknown {
    fn from(value: &MediaSourceAudioInputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaSourceAudioInputNode> for ::windows_core::IInspectable {
    fn from(value: MediaSourceAudioInputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaSourceAudioInputNode> for ::windows_core::IInspectable {
    fn from(value: &MediaSourceAudioInputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MediaSourceAudioInputNode> for IAudioInputNode {
    type Error = ::windows_core::Error;
    fn try_from(value: MediaSourceAudioInputNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MediaSourceAudioInputNode> for IAudioInputNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &MediaSourceAudioInputNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioInputNode> for MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioInputNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioInputNode> for &MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioInputNode> {
        ::core::convert::TryInto::<IAudioInputNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<MediaSourceAudioInputNode> for IAudioInputNode2 {
    type Error = ::windows_core::Error;
    fn try_from(value: MediaSourceAudioInputNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MediaSourceAudioInputNode> for IAudioInputNode2 {
    type Error = ::windows_core::Error;
    fn try_from(value: &MediaSourceAudioInputNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioInputNode2> for MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioInputNode2> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioInputNode2> for &MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioInputNode2> {
        ::core::convert::TryInto::<IAudioInputNode2>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<MediaSourceAudioInputNode> for IAudioNode {
    type Error = ::windows_core::Error;
    fn try_from(value: MediaSourceAudioInputNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MediaSourceAudioInputNode> for IAudioNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &MediaSourceAudioInputNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioNode> for MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioNode> for &MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioNode> {
        ::core::convert::TryInto::<IAudioNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<MediaSourceAudioInputNode> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: MediaSourceAudioInputNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MediaSourceAudioInputNode> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &MediaSourceAudioInputNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MediaSourceAudioInputNode {}
unsafe impl ::core::marker::Sync for MediaSourceAudioInputNode {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaSourceAudioInputNodeCreationStatus(pub i32);
impl MediaSourceAudioInputNodeCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const FormatNotSupported: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaSourceAudioInputNodeCreationStatus {}
impl ::core::clone::Clone for MediaSourceAudioInputNodeCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaSourceAudioInputNodeCreationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaSourceAudioInputNodeCreationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaSourceAudioInputNodeCreationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaSourceAudioInputNodeCreationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaSourceAudioInputNodeCreationStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.MediaSourceAudioInputNodeCreationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MixedRealitySpatialAudioFormatPolicy(pub i32);
impl MixedRealitySpatialAudioFormatPolicy {
    pub const UseMixedRealityDefaultSpatialAudioFormat: Self = Self(0i32);
    pub const UseDeviceConfigurationDefaultSpatialAudioFormat: Self = Self(1i32);
}
impl ::core::marker::Copy for MixedRealitySpatialAudioFormatPolicy {}
impl ::core::clone::Clone for MixedRealitySpatialAudioFormatPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MixedRealitySpatialAudioFormatPolicy {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MixedRealitySpatialAudioFormatPolicy {
    type Abi = Self;
}
impl ::core::fmt::Debug for MixedRealitySpatialAudioFormatPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MixedRealitySpatialAudioFormatPolicy").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MixedRealitySpatialAudioFormatPolicy {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.MixedRealitySpatialAudioFormatPolicy;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct QuantumSizeSelectionMode(pub i32);
impl QuantumSizeSelectionMode {
    pub const SystemDefault: Self = Self(0i32);
    pub const LowestLatency: Self = Self(1i32);
    pub const ClosestToDesired: Self = Self(2i32);
}
impl ::core::marker::Copy for QuantumSizeSelectionMode {}
impl ::core::clone::Clone for QuantumSizeSelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for QuantumSizeSelectionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for QuantumSizeSelectionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for QuantumSizeSelectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QuantumSizeSelectionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for QuantumSizeSelectionMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.QuantumSizeSelectionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ReverbEffectDefinition(::windows_core::IUnknown);
impl ReverbEffectDefinition {
    #[cfg(feature = "winrt-media")]
    pub fn ActivatableClassId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatableClassId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-media"))]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IPropertySet> {
        let this = &::windows_core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IPropertySet>(result__)
        }
    }
    pub fn SetWetDryMix(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetWetDryMix)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WetDryMix(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).WetDryMix)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetReflectionsDelay(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReflectionsDelay)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReflectionsDelay(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ReflectionsDelay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetReverbDelay(&self, value: u8) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReverbDelay)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReverbDelay(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).ReverbDelay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        }
    }
    pub fn SetRearDelay(&self, value: u8) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRearDelay)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RearDelay(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).RearDelay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        }
    }
    pub fn SetPositionLeft(&self, value: u8) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPositionLeft)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PositionLeft(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).PositionLeft)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        }
    }
    pub fn SetPositionRight(&self, value: u8) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPositionRight)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PositionRight(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).PositionRight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        }
    }
    pub fn SetPositionMatrixLeft(&self, value: u8) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPositionMatrixLeft)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PositionMatrixLeft(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).PositionMatrixLeft)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        }
    }
    pub fn SetPositionMatrixRight(&self, value: u8) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPositionMatrixRight)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PositionMatrixRight(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).PositionMatrixRight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        }
    }
    pub fn SetEarlyDiffusion(&self, value: u8) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEarlyDiffusion)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EarlyDiffusion(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).EarlyDiffusion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        }
    }
    pub fn SetLateDiffusion(&self, value: u8) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLateDiffusion)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LateDiffusion(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).LateDiffusion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        }
    }
    pub fn SetLowEQGain(&self, value: u8) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLowEQGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LowEQGain(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).LowEQGain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        }
    }
    pub fn SetLowEQCutoff(&self, value: u8) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLowEQCutoff)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LowEQCutoff(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).LowEQCutoff)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        }
    }
    pub fn SetHighEQGain(&self, value: u8) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHighEQGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HighEQGain(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).HighEQGain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        }
    }
    pub fn SetHighEQCutoff(&self, value: u8) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHighEQCutoff)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HighEQCutoff(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).HighEQCutoff)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        }
    }
    pub fn SetRoomFilterFreq(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRoomFilterFreq)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RoomFilterFreq(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).RoomFilterFreq)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetRoomFilterMain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRoomFilterMain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RoomFilterMain(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).RoomFilterMain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetRoomFilterHF(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRoomFilterHF)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RoomFilterHF(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).RoomFilterHF)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetReflectionsGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReflectionsGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReflectionsGain(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ReflectionsGain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetReverbGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReverbGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReverbGain(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ReverbGain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetDecayTime(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDecayTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DecayTime(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DecayTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetDensity(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDensity)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Density(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Density)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetRoomSize(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRoomSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RoomSize(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).RoomSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetDisableLateField(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisableLateField)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DisableLateField(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).DisableLateField)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, AudioGraph>>(audiograph: Param0) -> ::windows_core::Result<ReverbEffectDefinition> {
        Self::IReverbEffectDefinitionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), audiograph.into_param().abi(), result__.as_mut_ptr()).from_abi::<ReverbEffectDefinition>(result__)
        })
    }
    pub fn IReverbEffectDefinitionFactory<R, F: FnOnce(&IReverbEffectDefinitionFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ReverbEffectDefinition, IReverbEffectDefinitionFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ReverbEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ReverbEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ReverbEffectDefinition {}
impl ::core::fmt::Debug for ReverbEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ReverbEffectDefinition").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ReverbEffectDefinition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.ReverbEffectDefinition;{4606aa89-f563-4d0a-8f6e-f0cddff35d84})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ReverbEffectDefinition {
    type Vtable = IReverbEffectDefinition_Vtbl;
    const IID: ::windows_core::GUID = <IReverbEffectDefinition as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ReverbEffectDefinition {
    const NAME: &'static str = "Windows.Media.Audio.ReverbEffectDefinition";
}
impl ::core::convert::From<ReverbEffectDefinition> for ::windows_core::IUnknown {
    fn from(value: ReverbEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ReverbEffectDefinition> for ::windows_core::IUnknown {
    fn from(value: &ReverbEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ReverbEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ReverbEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ReverbEffectDefinition> for ::windows_core::IInspectable {
    fn from(value: ReverbEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ReverbEffectDefinition> for ::windows_core::IInspectable {
    fn from(value: &ReverbEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ReverbEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ReverbEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::convert::TryFrom<ReverbEffectDefinition> for super::Effects::IAudioEffectDefinition {
    type Error = ::windows_core::Error;
    fn try_from(value: ReverbEffectDefinition) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-media")]
impl ::core::convert::TryFrom<&ReverbEffectDefinition> for super::Effects::IAudioEffectDefinition {
    type Error = ::windows_core::Error;
    fn try_from(value: &ReverbEffectDefinition) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-media")]
impl<'a> ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition> for ReverbEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, super::Effects::IAudioEffectDefinition> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-media")]
impl<'a> ::windows_core::IntoParam<'a, super::Effects::IAudioEffectDefinition> for &ReverbEffectDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, super::Effects::IAudioEffectDefinition> {
        ::core::convert::TryInto::<super::Effects::IAudioEffectDefinition>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ReverbEffectDefinition {}
unsafe impl ::core::marker::Sync for ReverbEffectDefinition {}
#[repr(transparent)]
pub struct SetDefaultSpatialAudioFormatResult(::windows_core::IUnknown);
impl SetDefaultSpatialAudioFormatResult {
    pub fn Status(&self) -> ::windows_core::Result<SetDefaultSpatialAudioFormatStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SetDefaultSpatialAudioFormatStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SetDefaultSpatialAudioFormatStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for SetDefaultSpatialAudioFormatResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SetDefaultSpatialAudioFormatResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SetDefaultSpatialAudioFormatResult {}
impl ::core::fmt::Debug for SetDefaultSpatialAudioFormatResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SetDefaultSpatialAudioFormatResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SetDefaultSpatialAudioFormatResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.SetDefaultSpatialAudioFormatResult;{1c2aa511-1400-5e70-9ea9-ae151241e8ea})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SetDefaultSpatialAudioFormatResult {
    type Vtable = ISetDefaultSpatialAudioFormatResult_Vtbl;
    const IID: ::windows_core::GUID = <ISetDefaultSpatialAudioFormatResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SetDefaultSpatialAudioFormatResult {
    const NAME: &'static str = "Windows.Media.Audio.SetDefaultSpatialAudioFormatResult";
}
impl ::core::convert::From<SetDefaultSpatialAudioFormatResult> for ::windows_core::IUnknown {
    fn from(value: SetDefaultSpatialAudioFormatResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SetDefaultSpatialAudioFormatResult> for ::windows_core::IUnknown {
    fn from(value: &SetDefaultSpatialAudioFormatResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SetDefaultSpatialAudioFormatResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SetDefaultSpatialAudioFormatResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SetDefaultSpatialAudioFormatResult> for ::windows_core::IInspectable {
    fn from(value: SetDefaultSpatialAudioFormatResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SetDefaultSpatialAudioFormatResult> for ::windows_core::IInspectable {
    fn from(value: &SetDefaultSpatialAudioFormatResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SetDefaultSpatialAudioFormatResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SetDefaultSpatialAudioFormatResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SetDefaultSpatialAudioFormatResult {}
unsafe impl ::core::marker::Sync for SetDefaultSpatialAudioFormatResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SetDefaultSpatialAudioFormatStatus(pub i32);
impl SetDefaultSpatialAudioFormatStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const AccessDenied: Self = Self(1i32);
    pub const LicenseExpired: Self = Self(2i32);
    pub const LicenseNotValidForAudioEndpoint: Self = Self(3i32);
    pub const NotSupportedOnAudioEndpoint: Self = Self(4i32);
    pub const UnknownError: Self = Self(5i32);
}
impl ::core::marker::Copy for SetDefaultSpatialAudioFormatStatus {}
impl ::core::clone::Clone for SetDefaultSpatialAudioFormatStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SetDefaultSpatialAudioFormatStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SetDefaultSpatialAudioFormatStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for SetDefaultSpatialAudioFormatStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SetDefaultSpatialAudioFormatStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SetDefaultSpatialAudioFormatStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.SetDefaultSpatialAudioFormatStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SpatialAudioDeviceConfiguration(::windows_core::IUnknown);
impl SpatialAudioDeviceConfiguration {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsSpatialAudioSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSpatialAudioSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsSpatialAudioFormatSupported<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, subtype: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSpatialAudioFormatSupported)(::windows_core::Interface::as_raw(this), subtype.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ActiveSpatialAudioFormat(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ActiveSpatialAudioFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DefaultSpatialAudioFormat(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultSpatialAudioFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetDefaultSpatialAudioFormatAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, subtype: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SetDefaultSpatialAudioFormatResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetDefaultSpatialAudioFormatAsync)(::windows_core::Interface::as_raw(this), subtype.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SetDefaultSpatialAudioFormatResult>>(result__)
        }
    }
    pub fn ConfigurationChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SpatialAudioDeviceConfiguration, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ConfigurationChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveConfigurationChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveConfigurationChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetForDeviceId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<SpatialAudioDeviceConfiguration> {
        Self::ISpatialAudioDeviceConfigurationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForDeviceId)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialAudioDeviceConfiguration>(result__)
        })
    }
    pub fn ISpatialAudioDeviceConfigurationStatics<R, F: FnOnce(&ISpatialAudioDeviceConfigurationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SpatialAudioDeviceConfiguration, ISpatialAudioDeviceConfigurationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpatialAudioDeviceConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialAudioDeviceConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialAudioDeviceConfiguration {}
impl ::core::fmt::Debug for SpatialAudioDeviceConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAudioDeviceConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialAudioDeviceConfiguration {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.SpatialAudioDeviceConfiguration;{ee830034-61cf-5749-9da4-10f0fe028199})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialAudioDeviceConfiguration {
    type Vtable = ISpatialAudioDeviceConfiguration_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialAudioDeviceConfiguration as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialAudioDeviceConfiguration {
    const NAME: &'static str = "Windows.Media.Audio.SpatialAudioDeviceConfiguration";
}
impl ::core::convert::From<SpatialAudioDeviceConfiguration> for ::windows_core::IUnknown {
    fn from(value: SpatialAudioDeviceConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialAudioDeviceConfiguration> for ::windows_core::IUnknown {
    fn from(value: &SpatialAudioDeviceConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialAudioDeviceConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialAudioDeviceConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialAudioDeviceConfiguration> for ::windows_core::IInspectable {
    fn from(value: SpatialAudioDeviceConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialAudioDeviceConfiguration> for ::windows_core::IInspectable {
    fn from(value: &SpatialAudioDeviceConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialAudioDeviceConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialAudioDeviceConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialAudioDeviceConfiguration {}
unsafe impl ::core::marker::Sync for SpatialAudioDeviceConfiguration {}
#[repr(transparent)]
pub struct SpatialAudioFormatConfiguration(::windows_core::IUnknown);
impl SpatialAudioFormatConfiguration {
    pub fn ReportLicenseChangedAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, subtype: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportLicenseChangedAsync)(::windows_core::Interface::as_raw(this), subtype.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ReportConfigurationChangedAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, subtype: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportConfigurationChangedAsync)(::windows_core::Interface::as_raw(this), subtype.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn MixedRealityExclusiveModePolicy(&self) -> ::windows_core::Result<MixedRealitySpatialAudioFormatPolicy> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MixedRealitySpatialAudioFormatPolicy>::zeroed();
            (::windows_core::Interface::vtable(this).MixedRealityExclusiveModePolicy)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MixedRealitySpatialAudioFormatPolicy>(result__)
        }
    }
    pub fn SetMixedRealityExclusiveModePolicy(&self, value: MixedRealitySpatialAudioFormatPolicy) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMixedRealityExclusiveModePolicy)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetDefault() -> ::windows_core::Result<SpatialAudioFormatConfiguration> {
        Self::ISpatialAudioFormatConfigurationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialAudioFormatConfiguration>(result__)
        })
    }
    pub fn ISpatialAudioFormatConfigurationStatics<R, F: FnOnce(&ISpatialAudioFormatConfigurationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SpatialAudioFormatConfiguration, ISpatialAudioFormatConfigurationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpatialAudioFormatConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialAudioFormatConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialAudioFormatConfiguration {}
impl ::core::fmt::Debug for SpatialAudioFormatConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAudioFormatConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialAudioFormatConfiguration {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.SpatialAudioFormatConfiguration;{32df09a8-50f0-5395-9923-7d44ca71ed6d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialAudioFormatConfiguration {
    type Vtable = ISpatialAudioFormatConfiguration_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialAudioFormatConfiguration as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialAudioFormatConfiguration {
    const NAME: &'static str = "Windows.Media.Audio.SpatialAudioFormatConfiguration";
}
impl ::core::convert::From<SpatialAudioFormatConfiguration> for ::windows_core::IUnknown {
    fn from(value: SpatialAudioFormatConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialAudioFormatConfiguration> for ::windows_core::IUnknown {
    fn from(value: &SpatialAudioFormatConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialAudioFormatConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialAudioFormatConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialAudioFormatConfiguration> for ::windows_core::IInspectable {
    fn from(value: SpatialAudioFormatConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialAudioFormatConfiguration> for ::windows_core::IInspectable {
    fn from(value: &SpatialAudioFormatConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialAudioFormatConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialAudioFormatConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialAudioFormatConfiguration {}
unsafe impl ::core::marker::Sync for SpatialAudioFormatConfiguration {}
pub struct SpatialAudioFormatSubtype;
impl SpatialAudioFormatSubtype {
    pub fn WindowsSonic() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).WindowsSonic)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn DolbyAtmosForHeadphones() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DolbyAtmosForHeadphones)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn DolbyAtmosForHomeTheater() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DolbyAtmosForHomeTheater)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn DolbyAtmosForSpeakers() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DolbyAtmosForSpeakers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn DTSHeadphoneX() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DTSHeadphoneX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn DTSXUltra() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DTSXUltra)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn DTSXForHomeTheater() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DTSXForHomeTheater)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn ISpatialAudioFormatSubtypeStatics<R, F: FnOnce(&ISpatialAudioFormatSubtypeStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SpatialAudioFormatSubtype, ISpatialAudioFormatSubtypeStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISpatialAudioFormatSubtypeStatics2<R, F: FnOnce(&ISpatialAudioFormatSubtypeStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SpatialAudioFormatSubtype, ISpatialAudioFormatSubtypeStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for SpatialAudioFormatSubtype {
    const NAME: &'static str = "Windows.Media.Audio.SpatialAudioFormatSubtype";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SpatialAudioModel(pub i32);
impl SpatialAudioModel {
    pub const ObjectBased: Self = Self(0i32);
    pub const FoldDown: Self = Self(1i32);
}
impl ::core::marker::Copy for SpatialAudioModel {}
impl ::core::clone::Clone for SpatialAudioModel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpatialAudioModel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SpatialAudioModel {
    type Abi = Self;
}
impl ::core::fmt::Debug for SpatialAudioModel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAudioModel").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialAudioModel {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.SpatialAudioModel;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
