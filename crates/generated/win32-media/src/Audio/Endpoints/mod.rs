#[repr(C)]
pub struct AUDIO_ENDPOINT_SHARED_CREATE_PARAMS {
    pub u32Size: u32,
    pub u32TSSessionId: u32,
    pub targetEndpointConnectorType: EndpointConnectorType,
    pub wfxDeviceFormat: super::WAVEFORMATEX,
}
impl ::core::marker::Copy for AUDIO_ENDPOINT_SHARED_CREATE_PARAMS {}
impl ::core::clone::Clone for AUDIO_ENDPOINT_SHARED_CREATE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for AUDIO_ENDPOINT_SHARED_CREATE_PARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUDIO_ENDPOINT_SHARED_CREATE_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUDIO_ENDPOINT_SHARED_CREATE_PARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUDIO_ENDPOINT_SHARED_CREATE_PARAMS {}
impl ::core::default::Default for AUDIO_ENDPOINT_SHARED_CREATE_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DEVINTERFACE_AUDIOENDPOINTPLUGIN: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f2f7b66_65ac_4fa6_8ae4_123c78b89313);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_AudioEndpointPlugin2_FactoryCLSID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x12d83bd7_cf12_46be_8540_812710d3021c), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_AudioEndpointPlugin_DataFlow: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x12d83bd7_cf12_46be_8540_812710d3021c), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_AudioEndpointPlugin_FactoryCLSID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x12d83bd7_cf12_46be_8540_812710d3021c), pid: 1u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_AudioEndpointPlugin_PnPInterface: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x12d83bd7_cf12_46be_8540_812710d3021c), pid: 3u32 };
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EndpointConnectorType(pub i32);
pub const eHostProcessConnector: EndpointConnectorType = EndpointConnectorType(0i32);
pub const eOffloadConnector: EndpointConnectorType = EndpointConnectorType(1i32);
pub const eLoopbackConnector: EndpointConnectorType = EndpointConnectorType(2i32);
pub const eKeywordDetectorConnector: EndpointConnectorType = EndpointConnectorType(3i32);
pub const eConnectorCount: EndpointConnectorType = EndpointConnectorType(4i32);
impl ::core::marker::Copy for EndpointConnectorType {}
impl ::core::clone::Clone for EndpointConnectorType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EndpointConnectorType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EndpointConnectorType {
    type Abi = Self;
}
impl ::core::fmt::Debug for EndpointConnectorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EndpointConnectorType").field(&self.0).finish()
    }
}
#[repr(transparent)]
pub struct IAudioEndpointFormatControl(::windows_core::IUnknown);
impl IAudioEndpointFormatControl {
    pub unsafe fn ResetToDefault(&self, resetflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ResetToDefault)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(resetflags)).ok()
    }
}
impl ::core::convert::From<IAudioEndpointFormatControl> for ::windows_core::IUnknown {
    fn from(value: IAudioEndpointFormatControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioEndpointFormatControl> for ::windows_core::IUnknown {
    fn from(value: &IAudioEndpointFormatControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAudioEndpointFormatControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAudioEndpointFormatControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioEndpointFormatControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioEndpointFormatControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEndpointFormatControl {}
impl ::core::fmt::Debug for IAudioEndpointFormatControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEndpointFormatControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAudioEndpointFormatControl {
    type Vtable = IAudioEndpointFormatControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x784cfd40_9f89_456e_a1a6_873b006a664e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointFormatControl_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ResetToDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resetflags: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAudioEndpointLastBufferControl(::windows_core::IUnknown);
impl IAudioEndpointLastBufferControl {
    pub unsafe fn IsLastBufferControlSupported(&self) -> ::win32_foundation::BOOL {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).IsLastBufferControlSupported)(::windows_core::Interface::as_raw(self)))
    }
    #[cfg(feature = "Win32_Media_Audio_Apo")]
    pub unsafe fn ReleaseOutputDataPointerForLastBuffer(&self, pconnectionproperty: *const super::Apo::APO_CONNECTION_PROPERTY) {
        (::windows_core::Interface::vtable(self).ReleaseOutputDataPointerForLastBuffer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pconnectionproperty))
    }
}
impl ::core::convert::From<IAudioEndpointLastBufferControl> for ::windows_core::IUnknown {
    fn from(value: IAudioEndpointLastBufferControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioEndpointLastBufferControl> for ::windows_core::IUnknown {
    fn from(value: &IAudioEndpointLastBufferControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAudioEndpointLastBufferControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAudioEndpointLastBufferControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioEndpointLastBufferControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioEndpointLastBufferControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEndpointLastBufferControl {}
impl ::core::fmt::Debug for IAudioEndpointLastBufferControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEndpointLastBufferControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAudioEndpointLastBufferControl {
    type Vtable = IAudioEndpointLastBufferControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf8520dd3_8f9d_4437_9861_62f584c33dd6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointLastBufferControl_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub IsLastBufferControlSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL,
    #[cfg(feature = "Win32_Media_Audio_Apo")]
    pub ReleaseOutputDataPointerForLastBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnectionproperty: *const super::Apo::APO_CONNECTION_PROPERTY),
    #[cfg(not(feature = "Win32_Media_Audio_Apo"))]
    ReleaseOutputDataPointerForLastBuffer: usize,
}
#[repr(transparent)]
pub struct IAudioEndpointOffloadStreamMeter(::windows_core::IUnknown);
impl IAudioEndpointOffloadStreamMeter {
    pub unsafe fn GetMeterChannelCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetMeterChannelCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMeteringData(&self, u32channelcount: u32) -> ::windows_core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
        (::windows_core::Interface::vtable(self).GetMeteringData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(u32channelcount), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
}
impl ::core::convert::From<IAudioEndpointOffloadStreamMeter> for ::windows_core::IUnknown {
    fn from(value: IAudioEndpointOffloadStreamMeter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioEndpointOffloadStreamMeter> for ::windows_core::IUnknown {
    fn from(value: &IAudioEndpointOffloadStreamMeter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAudioEndpointOffloadStreamMeter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAudioEndpointOffloadStreamMeter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioEndpointOffloadStreamMeter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioEndpointOffloadStreamMeter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEndpointOffloadStreamMeter {}
impl ::core::fmt::Debug for IAudioEndpointOffloadStreamMeter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEndpointOffloadStreamMeter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAudioEndpointOffloadStreamMeter {
    type Vtable = IAudioEndpointOffloadStreamMeter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe1546dce_9dd1_418b_9ab2_348ced161c86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointOffloadStreamMeter_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetMeterChannelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pu32channelcount: *mut u32) -> ::windows_core::HRESULT,
    pub GetMeteringData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, u32channelcount: u32, pf32peakvalues: *mut f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAudioEndpointOffloadStreamMute(::windows_core::IUnknown);
impl IAudioEndpointOffloadStreamMute {
    pub unsafe fn SetMute(&self, bmuted: u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMute)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(bmuted)).ok()
    }
    pub unsafe fn GetMute(&self) -> ::windows_core::Result<u8> {
        let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
        (::windows_core::Interface::vtable(self).GetMute)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u8>(result__)
    }
}
impl ::core::convert::From<IAudioEndpointOffloadStreamMute> for ::windows_core::IUnknown {
    fn from(value: IAudioEndpointOffloadStreamMute) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioEndpointOffloadStreamMute> for ::windows_core::IUnknown {
    fn from(value: &IAudioEndpointOffloadStreamMute) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAudioEndpointOffloadStreamMute {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAudioEndpointOffloadStreamMute {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioEndpointOffloadStreamMute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioEndpointOffloadStreamMute {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEndpointOffloadStreamMute {}
impl ::core::fmt::Debug for IAudioEndpointOffloadStreamMute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEndpointOffloadStreamMute").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAudioEndpointOffloadStreamMute {
    type Vtable = IAudioEndpointOffloadStreamMute_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdfe21355_5ec2_40e0_8d6b_710ac3c00249);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointOffloadStreamMute_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetMute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmuted: u8) -> ::windows_core::HRESULT,
    pub GetMute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbmuted: *mut u8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAudioEndpointOffloadStreamVolume(::windows_core::IUnknown);
impl IAudioEndpointOffloadStreamVolume {
    pub unsafe fn GetVolumeChannelCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetVolumeChannelCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Media_KernelStreaming")]
    pub unsafe fn SetChannelVolumes(&self, u32channelcount: u32, pf32volumes: *const f32, u32curvetype: super::super::KernelStreaming::AUDIO_CURVE_TYPE, pcurveduration: *const i64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetChannelVolumes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(u32channelcount), ::core::mem::transmute(pf32volumes), ::core::mem::transmute(u32curvetype), ::core::mem::transmute(pcurveduration)).ok()
    }
    pub unsafe fn GetChannelVolumes(&self, u32channelcount: u32) -> ::windows_core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
        (::windows_core::Interface::vtable(self).GetChannelVolumes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(u32channelcount), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
}
impl ::core::convert::From<IAudioEndpointOffloadStreamVolume> for ::windows_core::IUnknown {
    fn from(value: IAudioEndpointOffloadStreamVolume) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioEndpointOffloadStreamVolume> for ::windows_core::IUnknown {
    fn from(value: &IAudioEndpointOffloadStreamVolume) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAudioEndpointOffloadStreamVolume {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAudioEndpointOffloadStreamVolume {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioEndpointOffloadStreamVolume {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioEndpointOffloadStreamVolume {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEndpointOffloadStreamVolume {}
impl ::core::fmt::Debug for IAudioEndpointOffloadStreamVolume {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEndpointOffloadStreamVolume").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAudioEndpointOffloadStreamVolume {
    type Vtable = IAudioEndpointOffloadStreamVolume_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64f1dd49_71ca_4281_8672_3a9eddd1d0b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointOffloadStreamVolume_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetVolumeChannelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pu32channelcount: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_KernelStreaming")]
    pub SetChannelVolumes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, u32channelcount: u32, pf32volumes: *const f32, u32curvetype: super::super::KernelStreaming::AUDIO_CURVE_TYPE, pcurveduration: *const i64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_KernelStreaming"))]
    SetChannelVolumes: usize,
    pub GetChannelVolumes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, u32channelcount: u32, pf32volumes: *mut f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAudioEndpointVolume(::windows_core::IUnknown);
impl IAudioEndpointVolume {
    pub unsafe fn RegisterControlChangeNotify<'a, Param0: ::windows_core::IntoParam<'a, IAudioEndpointVolumeCallback>>(&self, pnotify: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterControlChangeNotify)(::windows_core::Interface::as_raw(self), pnotify.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterControlChangeNotify<'a, Param0: ::windows_core::IntoParam<'a, IAudioEndpointVolumeCallback>>(&self, pnotify: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnregisterControlChangeNotify)(::windows_core::Interface::as_raw(self), pnotify.into_param().abi()).ok()
    }
    pub unsafe fn GetChannelCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetChannelCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMasterVolumeLevel(&self, fleveldb: f32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMasterVolumeLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn SetMasterVolumeLevelScalar(&self, flevel: f32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMasterVolumeLevelScalar)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(flevel), ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn GetMasterVolumeLevel(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
        (::windows_core::Interface::vtable(self).GetMasterVolumeLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn GetMasterVolumeLevelScalar(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
        (::windows_core::Interface::vtable(self).GetMasterVolumeLevelScalar)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetChannelVolumeLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetChannelVolumeLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn SetChannelVolumeLevelScalar(&self, nchannel: u32, flevel: f32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetChannelVolumeLevelScalar)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(flevel), ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn GetChannelVolumeLevel(&self, nchannel: u32) -> ::windows_core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
        (::windows_core::Interface::vtable(self).GetChannelVolumeLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn GetChannelVolumeLevelScalar(&self, nchannel: u32) -> ::windows_core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
        (::windows_core::Interface::vtable(self).GetChannelVolumeLevelScalar)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetMute<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bmute: Param0, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMute)(::windows_core::Interface::as_raw(self), bmute.into_param().abi(), ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn GetMute(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetMute)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn GetVolumeStepInfo(&self, pnstep: *mut u32, pnstepcount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetVolumeStepInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pnstep), ::core::mem::transmute(pnstepcount)).ok()
    }
    pub unsafe fn VolumeStepUp(&self, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).VolumeStepUp)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn VolumeStepDown(&self, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).VolumeStepDown)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn QueryHardwareSupport(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).QueryHardwareSupport)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetVolumeRange(&self, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetVolumeRange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pflvolumemindb), ::core::mem::transmute(pflvolumemaxdb), ::core::mem::transmute(pflvolumeincrementdb)).ok()
    }
}
impl ::core::convert::From<IAudioEndpointVolume> for ::windows_core::IUnknown {
    fn from(value: IAudioEndpointVolume) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioEndpointVolume> for ::windows_core::IUnknown {
    fn from(value: &IAudioEndpointVolume) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAudioEndpointVolume {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAudioEndpointVolume {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioEndpointVolume {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioEndpointVolume {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEndpointVolume {}
impl ::core::fmt::Debug for IAudioEndpointVolume {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEndpointVolume").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAudioEndpointVolume {
    type Vtable = IAudioEndpointVolume_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5cdf2c82_841e_4546_9722_0cf74078229a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointVolume_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub RegisterControlChangeNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnotify: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UnregisterControlChangeNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnotify: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetChannelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnchannelcount: *mut u32) -> ::windows_core::HRESULT,
    pub SetMasterVolumeLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fleveldb: f32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetMasterVolumeLevelScalar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flevel: f32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetMasterVolumeLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfleveldb: *mut f32) -> ::windows_core::HRESULT,
    pub GetMasterVolumeLevelScalar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflevel: *mut f32) -> ::windows_core::HRESULT,
    pub SetChannelVolumeLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetChannelVolumeLevelScalar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nchannel: u32, flevel: f32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetChannelVolumeLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nchannel: u32, pfleveldb: *mut f32) -> ::windows_core::HRESULT,
    pub GetChannelVolumeLevelScalar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nchannel: u32, pflevel: *mut f32) -> ::windows_core::HRESULT,
    pub SetMute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmute: ::win32_foundation::BOOL, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetMute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbmute: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetVolumeStepInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnstep: *mut u32, pnstepcount: *mut u32) -> ::windows_core::HRESULT,
    pub VolumeStepUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub VolumeStepDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub QueryHardwareSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwhardwaresupportmask: *mut u32) -> ::windows_core::HRESULT,
    pub GetVolumeRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAudioEndpointVolumeCallback(::windows_core::IUnknown);
impl IAudioEndpointVolumeCallback {
    pub unsafe fn OnNotify(&self, pnotify: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnNotify)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pnotify)).ok()
    }
}
impl ::core::convert::From<IAudioEndpointVolumeCallback> for ::windows_core::IUnknown {
    fn from(value: IAudioEndpointVolumeCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioEndpointVolumeCallback> for ::windows_core::IUnknown {
    fn from(value: &IAudioEndpointVolumeCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAudioEndpointVolumeCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAudioEndpointVolumeCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioEndpointVolumeCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioEndpointVolumeCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEndpointVolumeCallback {}
impl ::core::fmt::Debug for IAudioEndpointVolumeCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEndpointVolumeCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAudioEndpointVolumeCallback {
    type Vtable = IAudioEndpointVolumeCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x657804fa_d6ad_4496_8a60_352752af4f89);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointVolumeCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnotify: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAudioEndpointVolumeEx(::windows_core::IUnknown);
impl IAudioEndpointVolumeEx {
    pub unsafe fn RegisterControlChangeNotify<'a, Param0: ::windows_core::IntoParam<'a, IAudioEndpointVolumeCallback>>(&self, pnotify: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RegisterControlChangeNotify)(::windows_core::Interface::as_raw(self), pnotify.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterControlChangeNotify<'a, Param0: ::windows_core::IntoParam<'a, IAudioEndpointVolumeCallback>>(&self, pnotify: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.UnregisterControlChangeNotify)(::windows_core::Interface::as_raw(self), pnotify.into_param().abi()).ok()
    }
    pub unsafe fn GetChannelCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetChannelCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMasterVolumeLevel(&self, fleveldb: f32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetMasterVolumeLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn SetMasterVolumeLevelScalar(&self, flevel: f32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetMasterVolumeLevelScalar)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(flevel), ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn GetMasterVolumeLevel(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetMasterVolumeLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn GetMasterVolumeLevelScalar(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetMasterVolumeLevelScalar)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetChannelVolumeLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetChannelVolumeLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn SetChannelVolumeLevelScalar(&self, nchannel: u32, flevel: f32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetChannelVolumeLevelScalar)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(flevel), ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn GetChannelVolumeLevel(&self, nchannel: u32) -> ::windows_core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetChannelVolumeLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn GetChannelVolumeLevelScalar(&self, nchannel: u32) -> ::windows_core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetChannelVolumeLevelScalar)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetMute<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bmute: Param0, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetMute)(::windows_core::Interface::as_raw(self), bmute.into_param().abi(), ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn GetMute(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetMute)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn GetVolumeStepInfo(&self, pnstep: *mut u32, pnstepcount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetVolumeStepInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pnstep), ::core::mem::transmute(pnstepcount)).ok()
    }
    pub unsafe fn VolumeStepUp(&self, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.VolumeStepUp)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn VolumeStepDown(&self, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.VolumeStepDown)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn QueryHardwareSupport(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.QueryHardwareSupport)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetVolumeRange(&self, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetVolumeRange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pflvolumemindb), ::core::mem::transmute(pflvolumemaxdb), ::core::mem::transmute(pflvolumeincrementdb)).ok()
    }
    pub unsafe fn GetVolumeRangeChannel(&self, ichannel: u32, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetVolumeRangeChannel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ichannel), ::core::mem::transmute(pflvolumemindb), ::core::mem::transmute(pflvolumemaxdb), ::core::mem::transmute(pflvolumeincrementdb)).ok()
    }
}
impl ::core::convert::From<IAudioEndpointVolumeEx> for ::windows_core::IUnknown {
    fn from(value: IAudioEndpointVolumeEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioEndpointVolumeEx> for ::windows_core::IUnknown {
    fn from(value: &IAudioEndpointVolumeEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAudioEndpointVolumeEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAudioEndpointVolumeEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAudioEndpointVolumeEx> for IAudioEndpointVolume {
    fn from(value: IAudioEndpointVolumeEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioEndpointVolumeEx> for IAudioEndpointVolume {
    fn from(value: &IAudioEndpointVolumeEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioEndpointVolume> for IAudioEndpointVolumeEx {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioEndpointVolume> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioEndpointVolume> for &'a IAudioEndpointVolumeEx {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioEndpointVolume> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioEndpointVolumeEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioEndpointVolumeEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEndpointVolumeEx {}
impl ::core::fmt::Debug for IAudioEndpointVolumeEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEndpointVolumeEx").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAudioEndpointVolumeEx {
    type Vtable = IAudioEndpointVolumeEx_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x66e11784_f695_4f28_a505_a7080081a78f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointVolumeEx_Vtbl {
    pub base__: IAudioEndpointVolume_Vtbl,
    pub GetVolumeRangeChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ichannel: u32, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAudioLfxControl(::windows_core::IUnknown);
impl IAudioLfxControl {
    pub unsafe fn SetLocalEffectsState<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, benabled: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLocalEffectsState)(::windows_core::Interface::as_raw(self), benabled.into_param().abi()).ok()
    }
    pub unsafe fn GetLocalEffectsState(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetLocalEffectsState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IAudioLfxControl> for ::windows_core::IUnknown {
    fn from(value: IAudioLfxControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioLfxControl> for ::windows_core::IUnknown {
    fn from(value: &IAudioLfxControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAudioLfxControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAudioLfxControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioLfxControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioLfxControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioLfxControl {}
impl ::core::fmt::Debug for IAudioLfxControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioLfxControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAudioLfxControl {
    type Vtable = IAudioLfxControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x076a6922_d802_4f83_baf6_409d9ca11bfe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioLfxControl_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetLocalEffectsState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benabled: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetLocalEffectsState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAudioMeterInformation(::windows_core::IUnknown);
impl IAudioMeterInformation {
    pub unsafe fn GetPeakValue(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
        (::windows_core::Interface::vtable(self).GetPeakValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn GetMeteringChannelCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetMeteringChannelCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetChannelsPeakValues(&self, afpeakvalues: &mut [f32]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetChannelsPeakValues)(::windows_core::Interface::as_raw(self), afpeakvalues.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(afpeakvalues))).ok()
    }
    pub unsafe fn QueryHardwareSupport(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).QueryHardwareSupport)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IAudioMeterInformation> for ::windows_core::IUnknown {
    fn from(value: IAudioMeterInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioMeterInformation> for ::windows_core::IUnknown {
    fn from(value: &IAudioMeterInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAudioMeterInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAudioMeterInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioMeterInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioMeterInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioMeterInformation {}
impl ::core::fmt::Debug for IAudioMeterInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioMeterInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAudioMeterInformation {
    type Vtable = IAudioMeterInformation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc02216f6_8c67_4b5b_9d00_d008e73e0064);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioMeterInformation_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetPeakValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfpeak: *mut f32) -> ::windows_core::HRESULT,
    pub GetMeteringChannelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnchannelcount: *mut u32) -> ::windows_core::HRESULT,
    pub GetChannelsPeakValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, u32channelcount: u32, afpeakvalues: *mut f32) -> ::windows_core::HRESULT,
    pub QueryHardwareSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwhardwaresupportmask: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IHardwareAudioEngineBase(::windows_core::IUnknown);
impl IHardwareAudioEngineBase {
    pub unsafe fn GetAvailableOffloadConnectorCount<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, _pwstrdeviceid: Param0, _uconnectorid: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetAvailableOffloadConnectorCount)(::windows_core::Interface::as_raw(self), _pwstrdeviceid.into_param().abi(), ::core::mem::transmute(_uconnectorid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetEngineFormat<'a, Param0: ::windows_core::IntoParam<'a, super::IMMDevice>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pdevice: Param0, _brequestdeviceformat: Param1, _ppwfxformat: *mut *mut super::WAVEFORMATEX) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetEngineFormat)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), _brequestdeviceformat.into_param().abi(), ::core::mem::transmute(_ppwfxformat)).ok()
    }
    pub unsafe fn SetEngineDeviceFormat<'a, Param0: ::windows_core::IntoParam<'a, super::IMMDevice>>(&self, pdevice: Param0, _pwfxformat: *mut super::WAVEFORMATEX) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEngineDeviceFormat)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), ::core::mem::transmute(_pwfxformat)).ok()
    }
    pub unsafe fn SetGfxState<'a, Param0: ::windows_core::IntoParam<'a, super::IMMDevice>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pdevice: Param0, _benable: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGfxState)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), _benable.into_param().abi()).ok()
    }
    pub unsafe fn GetGfxState<'a, Param0: ::windows_core::IntoParam<'a, super::IMMDevice>>(&self, pdevice: Param0) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetGfxState)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IHardwareAudioEngineBase> for ::windows_core::IUnknown {
    fn from(value: IHardwareAudioEngineBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHardwareAudioEngineBase> for ::windows_core::IUnknown {
    fn from(value: &IHardwareAudioEngineBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IHardwareAudioEngineBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IHardwareAudioEngineBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IHardwareAudioEngineBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IHardwareAudioEngineBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHardwareAudioEngineBase {}
impl ::core::fmt::Debug for IHardwareAudioEngineBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHardwareAudioEngineBase").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IHardwareAudioEngineBase {
    type Vtable = IHardwareAudioEngineBase_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeddce3e4_f3c1_453a_b461_223563cbd886);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHardwareAudioEngineBase_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetAvailableOffloadConnectorCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, _pwstrdeviceid: ::windows_core::PCWSTR, _uconnectorid: u32, _pavailableconnectorinstancecount: *mut u32) -> ::windows_core::HRESULT,
    pub GetEngineFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: ::windows_core::RawPtr, _brequestdeviceformat: ::win32_foundation::BOOL, _ppwfxformat: *mut *mut super::WAVEFORMATEX) -> ::windows_core::HRESULT,
    pub SetEngineDeviceFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: ::windows_core::RawPtr, _pwfxformat: *mut super::WAVEFORMATEX) -> ::windows_core::HRESULT,
    pub SetGfxState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: ::windows_core::RawPtr, _benable: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetGfxState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: ::windows_core::RawPtr, _pbenable: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
