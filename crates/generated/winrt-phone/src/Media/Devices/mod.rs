#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioRoutingEndpoint(pub i32);
impl AudioRoutingEndpoint {
    pub const Default: Self = Self(0i32);
    pub const Earpiece: Self = Self(1i32);
    pub const Speakerphone: Self = Self(2i32);
    pub const Bluetooth: Self = Self(3i32);
    pub const WiredHeadset: Self = Self(4i32);
    pub const WiredHeadsetSpeakerOnly: Self = Self(5i32);
    pub const BluetoothWithNoiseAndEchoCancellation: Self = Self(6i32);
    pub const BluetoothPreferred: Self = Self(7i32);
}
impl ::core::marker::Copy for AudioRoutingEndpoint {}
impl ::core::clone::Clone for AudioRoutingEndpoint {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioRoutingEndpoint {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AudioRoutingEndpoint {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioRoutingEndpoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioRoutingEndpoint").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioRoutingEndpoint {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Phone.Media.Devices.AudioRoutingEndpoint;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AudioRoutingManager(::windows_core::IUnknown);
impl AudioRoutingManager {
    pub fn GetAudioEndpoint(&self) -> ::windows_core::Result<AudioRoutingEndpoint> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AudioRoutingEndpoint>::zeroed();
            (::windows_core::Interface::vtable(this).GetAudioEndpoint)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioRoutingEndpoint>(result__)
        }
    }
    pub fn SetAudioEndpoint(&self, endpoint: AudioRoutingEndpoint) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAudioEndpoint)(::windows_core::Interface::as_raw(this), endpoint).ok() }
    }
    pub fn AudioEndpointChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AudioRoutingManager, ::windows_core::IInspectable>>>(&self, endpointchangehandler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AudioEndpointChanged)(::windows_core::Interface::as_raw(this), endpointchangehandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAudioEndpointChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAudioEndpointChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn AvailableAudioEndpoints(&self) -> ::windows_core::Result<AvailableAudioRoutingEndpoints> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AvailableAudioRoutingEndpoints>::zeroed();
            (::windows_core::Interface::vtable(this).AvailableAudioEndpoints)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AvailableAudioRoutingEndpoints>(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<AudioRoutingManager> {
        Self::IAudioRoutingManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioRoutingManager>(result__)
        })
    }
    pub fn IAudioRoutingManagerStatics<R, F: FnOnce(&IAudioRoutingManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AudioRoutingManager, IAudioRoutingManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AudioRoutingManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioRoutingManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioRoutingManager {}
impl ::core::fmt::Debug for AudioRoutingManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioRoutingManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioRoutingManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Phone.Media.Devices.AudioRoutingManager;{79340d20-71cc-4526-9f29-fc8d2486418b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioRoutingManager {
    type Vtable = IAudioRoutingManager_Vtbl;
    const IID: ::windows_core::GUID = <IAudioRoutingManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioRoutingManager {
    const NAME: &'static str = "Windows.Phone.Media.Devices.AudioRoutingManager";
}
impl ::core::convert::From<AudioRoutingManager> for ::windows_core::IUnknown {
    fn from(value: AudioRoutingManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioRoutingManager> for ::windows_core::IUnknown {
    fn from(value: &AudioRoutingManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioRoutingManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioRoutingManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioRoutingManager> for ::windows_core::IInspectable {
    fn from(value: AudioRoutingManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioRoutingManager> for ::windows_core::IInspectable {
    fn from(value: &AudioRoutingManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioRoutingManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioRoutingManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioRoutingManager {}
unsafe impl ::core::marker::Sync for AudioRoutingManager {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AvailableAudioRoutingEndpoints(pub u32);
impl AvailableAudioRoutingEndpoints {
    pub const None: Self = Self(0u32);
    pub const Earpiece: Self = Self(1u32);
    pub const Speakerphone: Self = Self(2u32);
    pub const Bluetooth: Self = Self(4u32);
}
impl ::core::marker::Copy for AvailableAudioRoutingEndpoints {}
impl ::core::clone::Clone for AvailableAudioRoutingEndpoints {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AvailableAudioRoutingEndpoints {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AvailableAudioRoutingEndpoints {
    type Abi = Self;
}
impl ::core::fmt::Debug for AvailableAudioRoutingEndpoints {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AvailableAudioRoutingEndpoints").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for AvailableAudioRoutingEndpoints {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AvailableAudioRoutingEndpoints {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AvailableAudioRoutingEndpoints {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AvailableAudioRoutingEndpoints {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AvailableAudioRoutingEndpoints {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for AvailableAudioRoutingEndpoints {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Phone.Media.Devices.AvailableAudioRoutingEndpoints;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioRoutingManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioRoutingManager {
    type Vtable = IAudioRoutingManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79340d20_71cc_4526_9f29_fc8d2486418b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioRoutingManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetAudioEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioRoutingEndpoint) -> ::windows_core::HRESULT,
    pub SetAudioEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpoint: AudioRoutingEndpoint) -> ::windows_core::HRESULT,
    pub AudioEndpointChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpointchangehandler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveAudioEndpointChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub AvailableAudioEndpoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AvailableAudioRoutingEndpoints) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioRoutingManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioRoutingManagerStatics {
    type Vtable = IAudioRoutingManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x977fb2a4_5590_4a6f_adde_6a3d0ad58250);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioRoutingManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
