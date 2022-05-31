#[repr(transparent)]
pub struct AudioMediaFrame(::windows_core::IUnknown);
impl AudioMediaFrame {
    pub fn FrameReference(&self) -> ::windows_core::Result<MediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FrameReference)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaFrameReference>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn AudioEncodingProperties(&self) -> ::windows_core::Result<super::super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AudioEncodingProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn GetAudioFrame(&self) -> ::windows_core::Result<super::super::AudioFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAudioFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::AudioFrame>(result__)
        }
    }
}
impl ::core::clone::Clone for AudioMediaFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioMediaFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioMediaFrame {}
impl ::core::fmt::Debug for AudioMediaFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioMediaFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AudioMediaFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.AudioMediaFrame;{a3a9feff-8021-441b-9a46-e7f0137b7981})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AudioMediaFrame {
    type Vtable = IAudioMediaFrame_Vtbl;
    const IID: ::windows_core::GUID = <IAudioMediaFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioMediaFrame {
    const NAME: &'static str = "Windows.Media.Capture.Frames.AudioMediaFrame";
}
impl ::core::convert::From<AudioMediaFrame> for ::windows_core::IUnknown {
    fn from(value: AudioMediaFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioMediaFrame> for ::windows_core::IUnknown {
    fn from(value: &AudioMediaFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AudioMediaFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AudioMediaFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioMediaFrame> for ::windows_core::IInspectable {
    fn from(value: AudioMediaFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioMediaFrame> for ::windows_core::IInspectable {
    fn from(value: &AudioMediaFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AudioMediaFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AudioMediaFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioMediaFrame {}
unsafe impl ::core::marker::Sync for AudioMediaFrame {}
#[repr(transparent)]
pub struct BufferMediaFrame(::windows_core::IUnknown);
impl BufferMediaFrame {
    pub fn FrameReference(&self) -> ::windows_core::Result<MediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FrameReference)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaFrameReference>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Buffer(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Buffer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for BufferMediaFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BufferMediaFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BufferMediaFrame {}
impl ::core::fmt::Debug for BufferMediaFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BufferMediaFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BufferMediaFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.BufferMediaFrame;{b5b153c7-9b84-4062-b79c-a365b2596854})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BufferMediaFrame {
    type Vtable = IBufferMediaFrame_Vtbl;
    const IID: ::windows_core::GUID = <IBufferMediaFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BufferMediaFrame {
    const NAME: &'static str = "Windows.Media.Capture.Frames.BufferMediaFrame";
}
impl ::core::convert::From<BufferMediaFrame> for ::windows_core::IUnknown {
    fn from(value: BufferMediaFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BufferMediaFrame> for ::windows_core::IUnknown {
    fn from(value: &BufferMediaFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BufferMediaFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BufferMediaFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BufferMediaFrame> for ::windows_core::IInspectable {
    fn from(value: BufferMediaFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BufferMediaFrame> for ::windows_core::IInspectable {
    fn from(value: &BufferMediaFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BufferMediaFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BufferMediaFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BufferMediaFrame {}
unsafe impl ::core::marker::Sync for BufferMediaFrame {}
#[repr(transparent)]
pub struct DepthMediaFrame(::windows_core::IUnknown);
impl DepthMediaFrame {
    pub fn FrameReference(&self) -> ::windows_core::Result<MediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FrameReference)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaFrameReference>(result__)
        }
    }
    pub fn VideoMediaFrame(&self) -> ::windows_core::Result<VideoMediaFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoMediaFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<VideoMediaFrame>(result__)
        }
    }
    pub fn DepthFormat(&self) -> ::windows_core::Result<DepthMediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DepthFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DepthMediaFrameFormat>(result__)
        }
    }
    #[cfg(all(feature = "Media_Devices_Core", feature = "Perception_Spatial"))]
    pub fn TryCreateCoordinateMapper<'a, Param0: ::windows_core::IntoParam<'a, super::super::Devices::Core::CameraIntrinsics>, Param1: ::windows_core::IntoParam<'a, ::winrt_perception::Spatial::SpatialCoordinateSystem>>(&self, cameraintrinsics: Param0, coordinatesystem: Param1) -> ::windows_core::Result<super::super::Devices::Core::DepthCorrelatedCoordinateMapper> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateCoordinateMapper)(::windows_core::Interface::as_raw(this), cameraintrinsics.into_param().abi(), coordinatesystem.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Devices::Core::DepthCorrelatedCoordinateMapper>(result__)
        }
    }
    pub fn MaxReliableDepth(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IDepthMediaFrame2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxReliableDepth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn MinReliableDepth(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IDepthMediaFrame2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MinReliableDepth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for DepthMediaFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DepthMediaFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DepthMediaFrame {}
impl ::core::fmt::Debug for DepthMediaFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DepthMediaFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DepthMediaFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.DepthMediaFrame;{47135e4f-8549-45c0-925b-80d35efdb10a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DepthMediaFrame {
    type Vtable = IDepthMediaFrame_Vtbl;
    const IID: ::windows_core::GUID = <IDepthMediaFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DepthMediaFrame {
    const NAME: &'static str = "Windows.Media.Capture.Frames.DepthMediaFrame";
}
impl ::core::convert::From<DepthMediaFrame> for ::windows_core::IUnknown {
    fn from(value: DepthMediaFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DepthMediaFrame> for ::windows_core::IUnknown {
    fn from(value: &DepthMediaFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DepthMediaFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DepthMediaFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DepthMediaFrame> for ::windows_core::IInspectable {
    fn from(value: DepthMediaFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DepthMediaFrame> for ::windows_core::IInspectable {
    fn from(value: &DepthMediaFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DepthMediaFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DepthMediaFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DepthMediaFrame {}
unsafe impl ::core::marker::Sync for DepthMediaFrame {}
#[repr(transparent)]
pub struct DepthMediaFrameFormat(::windows_core::IUnknown);
impl DepthMediaFrameFormat {
    pub fn VideoFormat(&self) -> ::windows_core::Result<VideoMediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<VideoMediaFrameFormat>(result__)
        }
    }
    pub fn DepthScaleInMeters(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DepthScaleInMeters)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for DepthMediaFrameFormat {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DepthMediaFrameFormat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DepthMediaFrameFormat {}
impl ::core::fmt::Debug for DepthMediaFrameFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DepthMediaFrameFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DepthMediaFrameFormat {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.DepthMediaFrameFormat;{c312cf40-d729-453e-8780-2e04f140d28e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DepthMediaFrameFormat {
    type Vtable = IDepthMediaFrameFormat_Vtbl;
    const IID: ::windows_core::GUID = <IDepthMediaFrameFormat as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DepthMediaFrameFormat {
    const NAME: &'static str = "Windows.Media.Capture.Frames.DepthMediaFrameFormat";
}
impl ::core::convert::From<DepthMediaFrameFormat> for ::windows_core::IUnknown {
    fn from(value: DepthMediaFrameFormat) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DepthMediaFrameFormat> for ::windows_core::IUnknown {
    fn from(value: &DepthMediaFrameFormat) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DepthMediaFrameFormat {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DepthMediaFrameFormat {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DepthMediaFrameFormat> for ::windows_core::IInspectable {
    fn from(value: DepthMediaFrameFormat) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DepthMediaFrameFormat> for ::windows_core::IInspectable {
    fn from(value: &DepthMediaFrameFormat) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DepthMediaFrameFormat {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DepthMediaFrameFormat {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DepthMediaFrameFormat {}
unsafe impl ::core::marker::Sync for DepthMediaFrameFormat {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioMediaFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioMediaFrame {
    type Vtable = IAudioMediaFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa3a9feff_8021_441b_9a46_e7f0137b7981);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioMediaFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FrameReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub AudioEncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    AudioEncodingProperties: usize,
    pub GetAudioFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBufferMediaFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBufferMediaFrame {
    type Vtable = IBufferMediaFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb5b153c7_9b84_4062_b79c_a365b2596854);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBufferMediaFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FrameReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Buffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Buffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDepthMediaFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDepthMediaFrame {
    type Vtable = IDepthMediaFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x47135e4f_8549_45c0_925b_80d35efdb10a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDepthMediaFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FrameReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub VideoMediaFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DepthFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Media_Devices_Core", feature = "Perception_Spatial"))]
    pub TryCreateCoordinateMapper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cameraintrinsics: ::windows_core::RawPtr, coordinatesystem: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_Devices_Core", feature = "Perception_Spatial")))]
    TryCreateCoordinateMapper: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDepthMediaFrame2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDepthMediaFrame2 {
    type Vtable = IDepthMediaFrame2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6cca473d_c4a4_4176_b0cd_33eae3b35aa3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDepthMediaFrame2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MaxReliableDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub MinReliableDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDepthMediaFrameFormat(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDepthMediaFrameFormat {
    type Vtable = IDepthMediaFrameFormat_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc312cf40_d729_453e_8780_2e04f140d28e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDepthMediaFrameFormat_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub VideoFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DepthScaleInMeters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInfraredMediaFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInfraredMediaFrame {
    type Vtable = IInfraredMediaFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3fd13503_004b_4f0e_91ac_465299b41658);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInfraredMediaFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FrameReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub VideoMediaFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsIlluminated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameArrivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaFrameArrivedEventArgs {
    type Vtable = IMediaFrameArrivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0b430add_a490_4435_ada1_9affd55239f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameArrivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameFormat(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaFrameFormat {
    type Vtable = IMediaFrameFormat_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x71902b4e_b279_4a97_a9db_bd5a2fb78f39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameFormat_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MajorType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Subtype: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub FrameRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    FrameRate: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub VideoFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameFormat2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaFrameFormat2 {
    type Vtable = IMediaFrameFormat2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x63856340_5e87_4c10_86d1_6df097a6c6a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameFormat2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Media_MediaProperties")]
    pub AudioEncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    AudioEncodingProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameReader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaFrameReader {
    type Vtable = IMediaFrameReader_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe4c94395_2028_48ed_90b0_d1c1b162e24c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameReader_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveFrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub TryAcquireLatestFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StopAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameReader2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaFrameReader2 {
    type Vtable = IMediaFrameReader2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x871127b3_8531_4050_87cc_a13733cf3e9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameReader2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetAcquisitionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaFrameReaderAcquisitionMode) -> ::windows_core::HRESULT,
    pub AcquisitionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaFrameReaderAcquisitionMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameReference(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaFrameReference {
    type Vtable = IMediaFrameReference_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf6b88641_f0dc_4044_8dc9_961cedd05bad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameReference_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SourceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaFrameSourceKind) -> ::windows_core::HRESULT,
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SystemRelativeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub BufferMediaFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub VideoMediaFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub CoordinateSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    CoordinateSystem: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameReference2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaFrameReference2 {
    type Vtable = IMediaFrameReference2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xddbc3ecc_d5b2_49ef_836a_947d989b80c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameReference2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AudioMediaFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaFrameSource {
    type Vtable = IMediaFrameSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd6782953_90db_46a8_8add_2aa884a8d253);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Info: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Controller: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedFormats: usize,
    pub CurrentFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetFormatAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FormatChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveFormatChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "Media_Devices_Core")]
    pub TryGetCameraIntrinsics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices_Core"))]
    TryGetCameraIntrinsics: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameSourceController(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaFrameSourceController {
    type Vtable = IMediaFrameSourceController_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6d076635_316d_4b8f_b7b6_eeb04a8c6525);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameSourceController_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetPropertyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetPropertyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, propertyvalue: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Media_Devices")]
    pub VideoDeviceController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    VideoDeviceController: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameSourceController2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaFrameSourceController2 {
    type Vtable = IMediaFrameSourceController2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xefc49fd4_fcf2_4a03_b4e4_ac9628739bee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameSourceController2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetPropertyByExtendedIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extendedPropertyId_array_size: u32, extendedpropertyid: *const u8, maxpropertyvaluesize: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetPropertyByExtendedIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extendedPropertyId_array_size: u32, extendedpropertyid: *const u8, propertyValue_array_size: u32, propertyvalue: *const u8, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameSourceController3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaFrameSourceController3 {
    type Vtable = IMediaFrameSourceController3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1f0cf815_2464_4651_b1e8_4a82dbdb54de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameSourceController3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Media_Devices")]
    pub AudioDeviceController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    AudioDeviceController: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameSourceGetPropertyResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaFrameSourceGetPropertyResult {
    type Vtable = IMediaFrameSourceGetPropertyResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x088616c2_3a64_4bd5_bd2b_e7c898d2f37a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameSourceGetPropertyResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaFrameSourceGetPropertyStatus) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameSourceGroup(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaFrameSourceGroup {
    type Vtable = IMediaFrameSourceGroup_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7f605b87_4832_4b5f_ae3d_412faab37d34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameSourceGroup_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SourceInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SourceInfos: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameSourceGroupStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaFrameSourceGroupStatics {
    type Vtable = IMediaFrameSourceGroupStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1c48bfc5_436f_4508_94cf_d5d8b7326445);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameSourceGroupStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameSourceInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaFrameSourceInfo {
    type Vtable = IMediaFrameSourceInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x87bdc9cd_4601_408f_91cf_038318cd0af3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameSourceInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MediaStreamType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::MediaStreamType) -> ::windows_core::HRESULT,
    pub SourceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaFrameSourceKind) -> ::windows_core::HRESULT,
    pub SourceGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceInformation: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(feature = "Perception_Spatial")]
    pub CoordinateSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    CoordinateSystem: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameSourceInfo2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaFrameSourceInfo2 {
    type Vtable = IMediaFrameSourceInfo2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x195a7855_6457_42c6_a769_19b65bd32e6e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameSourceInfo2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ProfileId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub VideoProfileMediaDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    VideoProfileMediaDescription: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaFrameSourceInfo3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaFrameSourceInfo3 {
    type Vtable = IMediaFrameSourceInfo3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xca824ab6_66ea_5885_a2b6_26c0eeec3c7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrameSourceInfo3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Devices_Enumeration", feature = "UI_WindowManagement"))]
    pub GetRelativePanel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayregion: ::windows_core::RawPtr, result__: *mut ::winrt_devices::Enumeration::Panel) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "UI_WindowManagement")))]
    GetRelativePanel: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMultiSourceMediaFrameArrivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMultiSourceMediaFrameArrivedEventArgs {
    type Vtable = IMultiSourceMediaFrameArrivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x63115e01_cf51_48fd_aab0_6d693eb48127);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiSourceMediaFrameArrivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMultiSourceMediaFrameReader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMultiSourceMediaFrameReader {
    type Vtable = IMultiSourceMediaFrameReader_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8d144402_f763_488d_98f2_b437bcf075e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiSourceMediaFrameReader_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveFrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub TryAcquireLatestFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StopAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMultiSourceMediaFrameReader2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMultiSourceMediaFrameReader2 {
    type Vtable = IMultiSourceMediaFrameReader2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef5c8abd_fc5c_4c6b_9d81_3cb9cc637c26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiSourceMediaFrameReader2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetAcquisitionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaFrameReaderAcquisitionMode) -> ::windows_core::HRESULT,
    pub AcquisitionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaFrameReaderAcquisitionMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMultiSourceMediaFrameReference(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMultiSourceMediaFrameReference {
    type Vtable = IMultiSourceMediaFrameReference_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x21964b1a_7fe2_44d6_92e5_298e6d2810e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiSourceMediaFrameReference_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TryGetFrameReferenceBySourceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoMediaFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoMediaFrame {
    type Vtable = IVideoMediaFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00dd4ccb_32bd_4fe1_a013_7cc13cf5dbcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoMediaFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FrameReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub VideoFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")]
    pub SoftwareBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SoftwareBitmap: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3DSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3DSurface: usize,
    #[cfg(feature = "Media_Devices_Core")]
    pub CameraIntrinsics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices_Core"))]
    CameraIntrinsics: usize,
    pub InfraredMediaFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DepthMediaFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetVideoFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoMediaFrameFormat(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoMediaFrameFormat {
    type Vtable = IVideoMediaFrameFormat_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x46027fc0_d71b_45c7_8f14_6d9a0ae604e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoMediaFrameFormat_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MediaFrameFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DepthFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct InfraredMediaFrame(::windows_core::IUnknown);
impl InfraredMediaFrame {
    pub fn FrameReference(&self) -> ::windows_core::Result<MediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FrameReference)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaFrameReference>(result__)
        }
    }
    pub fn VideoMediaFrame(&self) -> ::windows_core::Result<VideoMediaFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoMediaFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<VideoMediaFrame>(result__)
        }
    }
    pub fn IsIlluminated(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsIlluminated)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for InfraredMediaFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InfraredMediaFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InfraredMediaFrame {}
impl ::core::fmt::Debug for InfraredMediaFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InfraredMediaFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InfraredMediaFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.InfraredMediaFrame;{3fd13503-004b-4f0e-91ac-465299b41658})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InfraredMediaFrame {
    type Vtable = IInfraredMediaFrame_Vtbl;
    const IID: ::windows_core::GUID = <IInfraredMediaFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InfraredMediaFrame {
    const NAME: &'static str = "Windows.Media.Capture.Frames.InfraredMediaFrame";
}
impl ::core::convert::From<InfraredMediaFrame> for ::windows_core::IUnknown {
    fn from(value: InfraredMediaFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InfraredMediaFrame> for ::windows_core::IUnknown {
    fn from(value: &InfraredMediaFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InfraredMediaFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InfraredMediaFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InfraredMediaFrame> for ::windows_core::IInspectable {
    fn from(value: InfraredMediaFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InfraredMediaFrame> for ::windows_core::IInspectable {
    fn from(value: &InfraredMediaFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InfraredMediaFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InfraredMediaFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InfraredMediaFrame {}
unsafe impl ::core::marker::Sync for InfraredMediaFrame {}
#[repr(transparent)]
pub struct MediaFrameArrivedEventArgs(::windows_core::IUnknown);
impl MediaFrameArrivedEventArgs {}
impl ::core::clone::Clone for MediaFrameArrivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaFrameArrivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaFrameArrivedEventArgs {}
impl ::core::fmt::Debug for MediaFrameArrivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaFrameArrivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaFrameArrivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MediaFrameArrivedEventArgs;{0b430add-a490-4435-ada1-9affd55239f7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaFrameArrivedEventArgs {
    type Vtable = IMediaFrameArrivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaFrameArrivedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaFrameArrivedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameArrivedEventArgs";
}
impl ::core::convert::From<MediaFrameArrivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MediaFrameArrivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameArrivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MediaFrameArrivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaFrameArrivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaFrameArrivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaFrameArrivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MediaFrameArrivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameArrivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MediaFrameArrivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaFrameArrivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaFrameArrivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaFrameArrivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaFrameArrivedEventArgs {}
#[repr(transparent)]
pub struct MediaFrameFormat(::windows_core::IUnknown);
impl MediaFrameFormat {
    pub fn MajorType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).MajorType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Subtype(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Subtype)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn FrameRate(&self) -> ::windows_core::Result<super::super::MediaProperties::MediaRatio> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FrameRate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::MediaProperties::MediaRatio>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::GUID, ::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::GUID, ::windows_core::IInspectable>>(result__)
        }
    }
    pub fn VideoFormat(&self) -> ::windows_core::Result<VideoMediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<VideoMediaFrameFormat>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn AudioEncodingProperties(&self) -> ::windows_core::Result<super::super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows_core::Interface::cast::<IMediaFrameFormat2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AudioEncodingProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaFrameFormat {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaFrameFormat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaFrameFormat {}
impl ::core::fmt::Debug for MediaFrameFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaFrameFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaFrameFormat {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MediaFrameFormat;{71902b4e-b279-4a97-a9db-bd5a2fb78f39})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaFrameFormat {
    type Vtable = IMediaFrameFormat_Vtbl;
    const IID: ::windows_core::GUID = <IMediaFrameFormat as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaFrameFormat {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameFormat";
}
impl ::core::convert::From<MediaFrameFormat> for ::windows_core::IUnknown {
    fn from(value: MediaFrameFormat) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameFormat> for ::windows_core::IUnknown {
    fn from(value: &MediaFrameFormat) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaFrameFormat {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaFrameFormat {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaFrameFormat> for ::windows_core::IInspectable {
    fn from(value: MediaFrameFormat) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameFormat> for ::windows_core::IInspectable {
    fn from(value: &MediaFrameFormat) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaFrameFormat {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaFrameFormat {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaFrameFormat {}
unsafe impl ::core::marker::Sync for MediaFrameFormat {}
#[repr(transparent)]
pub struct MediaFrameReader(::windows_core::IUnknown);
impl MediaFrameReader {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn FrameArrived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaFrameReader, MediaFrameArrivedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).FrameArrived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveFrameArrived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFrameArrived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn TryAcquireLatestFrame(&self) -> ::windows_core::Result<MediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryAcquireLatestFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaFrameReference>(result__)
        }
    }
    pub fn StartAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<MediaFrameReaderStartStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<MediaFrameReaderStartStatus>>(result__)
        }
    }
    pub fn StopAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StopAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn SetAcquisitionMode(&self, value: MediaFrameReaderAcquisitionMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaFrameReader2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAcquisitionMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AcquisitionMode(&self) -> ::windows_core::Result<MediaFrameReaderAcquisitionMode> {
        let this = &::windows_core::Interface::cast::<IMediaFrameReader2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaFrameReaderAcquisitionMode>::zeroed();
            (::windows_core::Interface::vtable(this).AcquisitionMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaFrameReaderAcquisitionMode>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaFrameReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaFrameReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaFrameReader {}
impl ::core::fmt::Debug for MediaFrameReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaFrameReader").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaFrameReader {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MediaFrameReader;{e4c94395-2028-48ed-90b0-d1c1b162e24c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaFrameReader {
    type Vtable = IMediaFrameReader_Vtbl;
    const IID: ::windows_core::GUID = <IMediaFrameReader as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaFrameReader {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameReader";
}
impl ::core::convert::From<MediaFrameReader> for ::windows_core::IUnknown {
    fn from(value: MediaFrameReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameReader> for ::windows_core::IUnknown {
    fn from(value: &MediaFrameReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaFrameReader> for ::windows_core::IInspectable {
    fn from(value: MediaFrameReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameReader> for ::windows_core::IInspectable {
    fn from(value: &MediaFrameReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MediaFrameReader> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: MediaFrameReader) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MediaFrameReader> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &MediaFrameReader) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for MediaFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &MediaFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MediaFrameReader {}
unsafe impl ::core::marker::Sync for MediaFrameReader {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaFrameReaderAcquisitionMode(pub i32);
impl MediaFrameReaderAcquisitionMode {
    pub const Realtime: Self = Self(0i32);
    pub const Buffered: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaFrameReaderAcquisitionMode {}
impl ::core::clone::Clone for MediaFrameReaderAcquisitionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaFrameReaderAcquisitionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaFrameReaderAcquisitionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaFrameReaderAcquisitionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaFrameReaderAcquisitionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaFrameReaderAcquisitionMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.Frames.MediaFrameReaderAcquisitionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaFrameReaderStartStatus(pub i32);
impl MediaFrameReaderStartStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const DeviceNotAvailable: Self = Self(2i32);
    pub const OutputFormatNotSupported: Self = Self(3i32);
    pub const ExclusiveControlNotAvailable: Self = Self(4i32);
}
impl ::core::marker::Copy for MediaFrameReaderStartStatus {}
impl ::core::clone::Clone for MediaFrameReaderStartStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaFrameReaderStartStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaFrameReaderStartStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaFrameReaderStartStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaFrameReaderStartStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaFrameReaderStartStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.Frames.MediaFrameReaderStartStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MediaFrameReference(::windows_core::IUnknown);
impl MediaFrameReference {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SourceKind(&self) -> ::windows_core::Result<MediaFrameSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaFrameSourceKind>::zeroed();
            (::windows_core::Interface::vtable(this).SourceKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaFrameSourceKind>(result__)
        }
    }
    pub fn Format(&self) -> ::windows_core::Result<MediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Format)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaFrameFormat>(result__)
        }
    }
    pub fn SystemRelativeTime(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SystemRelativeTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::GUID, ::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::GUID, ::windows_core::IInspectable>>(result__)
        }
    }
    pub fn BufferMediaFrame(&self) -> ::windows_core::Result<BufferMediaFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BufferMediaFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BufferMediaFrame>(result__)
        }
    }
    pub fn VideoMediaFrame(&self) -> ::windows_core::Result<VideoMediaFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoMediaFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<VideoMediaFrame>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    pub fn CoordinateSystem(&self) -> ::windows_core::Result<::winrt_perception::Spatial::SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CoordinateSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_perception::Spatial::SpatialCoordinateSystem>(result__)
        }
    }
    pub fn AudioMediaFrame(&self) -> ::windows_core::Result<AudioMediaFrame> {
        let this = &::windows_core::Interface::cast::<IMediaFrameReference2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AudioMediaFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AudioMediaFrame>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaFrameReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaFrameReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaFrameReference {}
impl ::core::fmt::Debug for MediaFrameReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaFrameReference").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaFrameReference {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MediaFrameReference;{f6b88641-f0dc-4044-8dc9-961cedd05bad})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaFrameReference {
    type Vtable = IMediaFrameReference_Vtbl;
    const IID: ::windows_core::GUID = <IMediaFrameReference as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaFrameReference {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameReference";
}
impl ::core::convert::From<MediaFrameReference> for ::windows_core::IUnknown {
    fn from(value: MediaFrameReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameReference> for ::windows_core::IUnknown {
    fn from(value: &MediaFrameReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaFrameReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaFrameReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaFrameReference> for ::windows_core::IInspectable {
    fn from(value: MediaFrameReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameReference> for ::windows_core::IInspectable {
    fn from(value: &MediaFrameReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaFrameReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaFrameReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MediaFrameReference> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: MediaFrameReference) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MediaFrameReference> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &MediaFrameReference) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for MediaFrameReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &MediaFrameReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MediaFrameReference {}
unsafe impl ::core::marker::Sync for MediaFrameReference {}
#[repr(transparent)]
pub struct MediaFrameSource(::windows_core::IUnknown);
impl MediaFrameSource {
    pub fn Info(&self) -> ::windows_core::Result<MediaFrameSourceInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Info)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaFrameSourceInfo>(result__)
        }
    }
    pub fn Controller(&self) -> ::windows_core::Result<MediaFrameSourceController> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Controller)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaFrameSourceController>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedFormats(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<MediaFrameFormat>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedFormats)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<MediaFrameFormat>>(result__)
        }
    }
    pub fn CurrentFormat(&self) -> ::windows_core::Result<MediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaFrameFormat>(result__)
        }
    }
    pub fn SetFormatAsync<'a, Param0: ::windows_core::IntoParam<'a, MediaFrameFormat>>(&self, format: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetFormatAsync)(::windows_core::Interface::as_raw(this), format.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn FormatChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MediaFrameSource, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).FormatChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveFormatChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFormatChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Devices_Core")]
    pub fn TryGetCameraIntrinsics<'a, Param0: ::windows_core::IntoParam<'a, MediaFrameFormat>>(&self, format: Param0) -> ::windows_core::Result<super::super::Devices::Core::CameraIntrinsics> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetCameraIntrinsics)(::windows_core::Interface::as_raw(this), format.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Devices::Core::CameraIntrinsics>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaFrameSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaFrameSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaFrameSource {}
impl ::core::fmt::Debug for MediaFrameSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaFrameSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaFrameSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MediaFrameSource;{d6782953-90db-46a8-8add-2aa884a8d253})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaFrameSource {
    type Vtable = IMediaFrameSource_Vtbl;
    const IID: ::windows_core::GUID = <IMediaFrameSource as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaFrameSource {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameSource";
}
impl ::core::convert::From<MediaFrameSource> for ::windows_core::IUnknown {
    fn from(value: MediaFrameSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameSource> for ::windows_core::IUnknown {
    fn from(value: &MediaFrameSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaFrameSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaFrameSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaFrameSource> for ::windows_core::IInspectable {
    fn from(value: MediaFrameSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameSource> for ::windows_core::IInspectable {
    fn from(value: &MediaFrameSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaFrameSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaFrameSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaFrameSource {}
unsafe impl ::core::marker::Sync for MediaFrameSource {}
#[repr(transparent)]
pub struct MediaFrameSourceController(::windows_core::IUnknown);
impl MediaFrameSourceController {
    pub fn GetPropertyAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<MediaFrameSourceGetPropertyResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPropertyAsync)(::windows_core::Interface::as_raw(this), propertyid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<MediaFrameSourceGetPropertyResult>>(result__)
        }
    }
    pub fn SetPropertyAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, propertyid: Param0, propertyvalue: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<MediaFrameSourceSetPropertyStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetPropertyAsync)(::windows_core::Interface::as_raw(this), propertyid.into_param().abi(), propertyvalue.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<MediaFrameSourceSetPropertyStatus>>(result__)
        }
    }
    #[cfg(feature = "Media_Devices")]
    pub fn VideoDeviceController(&self) -> ::windows_core::Result<super::super::Devices::VideoDeviceController> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoDeviceController)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::VideoDeviceController>(result__)
        }
    }
    pub fn GetPropertyByExtendedIdAsync<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<u32>>>(&self, extendedpropertyid: &[u8], maxpropertyvaluesize: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<MediaFrameSourceGetPropertyResult>> {
        let this = &::windows_core::Interface::cast::<IMediaFrameSourceController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPropertyByExtendedIdAsync)(::windows_core::Interface::as_raw(this), extendedpropertyid.len() as u32, ::core::mem::transmute(extendedpropertyid.as_ptr()), maxpropertyvaluesize.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<MediaFrameSourceGetPropertyResult>>(result__)
        }
    }
    pub fn SetPropertyByExtendedIdAsync(&self, extendedpropertyid: &[u8], propertyvalue: &[u8]) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<MediaFrameSourceSetPropertyStatus>> {
        let this = &::windows_core::Interface::cast::<IMediaFrameSourceController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetPropertyByExtendedIdAsync)(::windows_core::Interface::as_raw(this), extendedpropertyid.len() as u32, ::core::mem::transmute(extendedpropertyid.as_ptr()), propertyvalue.len() as u32, ::core::mem::transmute(propertyvalue.as_ptr()), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<MediaFrameSourceSetPropertyStatus>>(result__)
        }
    }
    #[cfg(feature = "Media_Devices")]
    pub fn AudioDeviceController(&self) -> ::windows_core::Result<super::super::Devices::AudioDeviceController> {
        let this = &::windows_core::Interface::cast::<IMediaFrameSourceController3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AudioDeviceController)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::AudioDeviceController>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaFrameSourceController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaFrameSourceController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaFrameSourceController {}
impl ::core::fmt::Debug for MediaFrameSourceController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaFrameSourceController").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaFrameSourceController {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MediaFrameSourceController;{6d076635-316d-4b8f-b7b6-eeb04a8c6525})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaFrameSourceController {
    type Vtable = IMediaFrameSourceController_Vtbl;
    const IID: ::windows_core::GUID = <IMediaFrameSourceController as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaFrameSourceController {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameSourceController";
}
impl ::core::convert::From<MediaFrameSourceController> for ::windows_core::IUnknown {
    fn from(value: MediaFrameSourceController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameSourceController> for ::windows_core::IUnknown {
    fn from(value: &MediaFrameSourceController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaFrameSourceController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaFrameSourceController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaFrameSourceController> for ::windows_core::IInspectable {
    fn from(value: MediaFrameSourceController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameSourceController> for ::windows_core::IInspectable {
    fn from(value: &MediaFrameSourceController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaFrameSourceController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaFrameSourceController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaFrameSourceController {}
unsafe impl ::core::marker::Sync for MediaFrameSourceController {}
#[repr(transparent)]
pub struct MediaFrameSourceGetPropertyResult(::windows_core::IUnknown);
impl MediaFrameSourceGetPropertyResult {
    pub fn Status(&self) -> ::windows_core::Result<MediaFrameSourceGetPropertyStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaFrameSourceGetPropertyStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaFrameSourceGetPropertyStatus>(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaFrameSourceGetPropertyResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaFrameSourceGetPropertyResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaFrameSourceGetPropertyResult {}
impl ::core::fmt::Debug for MediaFrameSourceGetPropertyResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaFrameSourceGetPropertyResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaFrameSourceGetPropertyResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MediaFrameSourceGetPropertyResult;{088616c2-3a64-4bd5-bd2b-e7c898d2f37a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaFrameSourceGetPropertyResult {
    type Vtable = IMediaFrameSourceGetPropertyResult_Vtbl;
    const IID: ::windows_core::GUID = <IMediaFrameSourceGetPropertyResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaFrameSourceGetPropertyResult {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameSourceGetPropertyResult";
}
impl ::core::convert::From<MediaFrameSourceGetPropertyResult> for ::windows_core::IUnknown {
    fn from(value: MediaFrameSourceGetPropertyResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameSourceGetPropertyResult> for ::windows_core::IUnknown {
    fn from(value: &MediaFrameSourceGetPropertyResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaFrameSourceGetPropertyResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaFrameSourceGetPropertyResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaFrameSourceGetPropertyResult> for ::windows_core::IInspectable {
    fn from(value: MediaFrameSourceGetPropertyResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameSourceGetPropertyResult> for ::windows_core::IInspectable {
    fn from(value: &MediaFrameSourceGetPropertyResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaFrameSourceGetPropertyResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaFrameSourceGetPropertyResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaFrameSourceGetPropertyResult {}
unsafe impl ::core::marker::Sync for MediaFrameSourceGetPropertyResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaFrameSourceGetPropertyStatus(pub i32);
impl MediaFrameSourceGetPropertyStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const NotSupported: Self = Self(2i32);
    pub const DeviceNotAvailable: Self = Self(3i32);
    pub const MaxPropertyValueSizeTooSmall: Self = Self(4i32);
    pub const MaxPropertyValueSizeRequired: Self = Self(5i32);
}
impl ::core::marker::Copy for MediaFrameSourceGetPropertyStatus {}
impl ::core::clone::Clone for MediaFrameSourceGetPropertyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaFrameSourceGetPropertyStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaFrameSourceGetPropertyStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaFrameSourceGetPropertyStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaFrameSourceGetPropertyStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaFrameSourceGetPropertyStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.Frames.MediaFrameSourceGetPropertyStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MediaFrameSourceGroup(::windows_core::IUnknown);
impl MediaFrameSourceGroup {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SourceInfos(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<MediaFrameSourceInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SourceInfos)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<MediaFrameSourceInfo>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<MediaFrameSourceGroup>>> {
        Self::IMediaFrameSourceGroupStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAllAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<MediaFrameSourceGroup>>>(result__)
        })
    }
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(id: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<MediaFrameSourceGroup>> {
        Self::IMediaFrameSourceGroupStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), id.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<MediaFrameSourceGroup>>(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaFrameSourceGroupStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IMediaFrameSourceGroupStatics<R, F: FnOnce(&IMediaFrameSourceGroupStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MediaFrameSourceGroup, IMediaFrameSourceGroupStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MediaFrameSourceGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaFrameSourceGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaFrameSourceGroup {}
impl ::core::fmt::Debug for MediaFrameSourceGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaFrameSourceGroup").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaFrameSourceGroup {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MediaFrameSourceGroup;{7f605b87-4832-4b5f-ae3d-412faab37d34})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaFrameSourceGroup {
    type Vtable = IMediaFrameSourceGroup_Vtbl;
    const IID: ::windows_core::GUID = <IMediaFrameSourceGroup as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaFrameSourceGroup {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameSourceGroup";
}
impl ::core::convert::From<MediaFrameSourceGroup> for ::windows_core::IUnknown {
    fn from(value: MediaFrameSourceGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameSourceGroup> for ::windows_core::IUnknown {
    fn from(value: &MediaFrameSourceGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaFrameSourceGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaFrameSourceGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaFrameSourceGroup> for ::windows_core::IInspectable {
    fn from(value: MediaFrameSourceGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameSourceGroup> for ::windows_core::IInspectable {
    fn from(value: &MediaFrameSourceGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaFrameSourceGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaFrameSourceGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaFrameSourceGroup {}
unsafe impl ::core::marker::Sync for MediaFrameSourceGroup {}
#[repr(transparent)]
pub struct MediaFrameSourceInfo(::windows_core::IUnknown);
impl MediaFrameSourceInfo {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn MediaStreamType(&self) -> ::windows_core::Result<super::MediaStreamType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::MediaStreamType>::zeroed();
            (::windows_core::Interface::vtable(this).MediaStreamType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::MediaStreamType>(result__)
        }
    }
    pub fn SourceKind(&self) -> ::windows_core::Result<MediaFrameSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaFrameSourceKind>::zeroed();
            (::windows_core::Interface::vtable(this).SourceKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaFrameSourceKind>(result__)
        }
    }
    pub fn SourceGroup(&self) -> ::windows_core::Result<MediaFrameSourceGroup> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SourceGroup)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaFrameSourceGroup>(result__)
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceInformation(&self) -> ::windows_core::Result<::winrt_devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceInformation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Enumeration::DeviceInformation>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::GUID, ::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::GUID, ::windows_core::IInspectable>>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    pub fn CoordinateSystem(&self) -> ::windows_core::Result<::winrt_perception::Spatial::SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CoordinateSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_perception::Spatial::SpatialCoordinateSystem>(result__)
        }
    }
    pub fn ProfileId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaFrameSourceInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ProfileId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn VideoProfileMediaDescription(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<super::MediaCaptureVideoProfileMediaDescription>> {
        let this = &::windows_core::Interface::cast::<IMediaFrameSourceInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoProfileMediaDescription)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<super::MediaCaptureVideoProfileMediaDescription>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Enumeration", feature = "UI_WindowManagement"))]
    pub fn GetRelativePanel<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_ui::WindowManagement::DisplayRegion>>(&self, displayregion: Param0) -> ::windows_core::Result<::winrt_devices::Enumeration::Panel> {
        let this = &::windows_core::Interface::cast::<IMediaFrameSourceInfo3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_devices::Enumeration::Panel>::zeroed();
            (::windows_core::Interface::vtable(this).GetRelativePanel)(::windows_core::Interface::as_raw(this), displayregion.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_devices::Enumeration::Panel>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaFrameSourceInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaFrameSourceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaFrameSourceInfo {}
impl ::core::fmt::Debug for MediaFrameSourceInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaFrameSourceInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaFrameSourceInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MediaFrameSourceInfo;{87bdc9cd-4601-408f-91cf-038318cd0af3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaFrameSourceInfo {
    type Vtable = IMediaFrameSourceInfo_Vtbl;
    const IID: ::windows_core::GUID = <IMediaFrameSourceInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaFrameSourceInfo {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameSourceInfo";
}
impl ::core::convert::From<MediaFrameSourceInfo> for ::windows_core::IUnknown {
    fn from(value: MediaFrameSourceInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameSourceInfo> for ::windows_core::IUnknown {
    fn from(value: &MediaFrameSourceInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaFrameSourceInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaFrameSourceInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaFrameSourceInfo> for ::windows_core::IInspectable {
    fn from(value: MediaFrameSourceInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaFrameSourceInfo> for ::windows_core::IInspectable {
    fn from(value: &MediaFrameSourceInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaFrameSourceInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaFrameSourceInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaFrameSourceInfo {}
unsafe impl ::core::marker::Sync for MediaFrameSourceInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaFrameSourceKind(pub i32);
impl MediaFrameSourceKind {
    pub const Custom: Self = Self(0i32);
    pub const Color: Self = Self(1i32);
    pub const Infrared: Self = Self(2i32);
    pub const Depth: Self = Self(3i32);
    pub const Audio: Self = Self(4i32);
    pub const Image: Self = Self(5i32);
    pub const Metadata: Self = Self(6i32);
}
impl ::core::marker::Copy for MediaFrameSourceKind {}
impl ::core::clone::Clone for MediaFrameSourceKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaFrameSourceKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaFrameSourceKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaFrameSourceKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaFrameSourceKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaFrameSourceKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.Frames.MediaFrameSourceKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaFrameSourceSetPropertyStatus(pub i32);
impl MediaFrameSourceSetPropertyStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const NotSupported: Self = Self(2i32);
    pub const InvalidValue: Self = Self(3i32);
    pub const DeviceNotAvailable: Self = Self(4i32);
    pub const NotInControl: Self = Self(5i32);
}
impl ::core::marker::Copy for MediaFrameSourceSetPropertyStatus {}
impl ::core::clone::Clone for MediaFrameSourceSetPropertyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaFrameSourceSetPropertyStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MediaFrameSourceSetPropertyStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaFrameSourceSetPropertyStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaFrameSourceSetPropertyStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaFrameSourceSetPropertyStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.Frames.MediaFrameSourceSetPropertyStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MultiSourceMediaFrameArrivedEventArgs(::windows_core::IUnknown);
impl MultiSourceMediaFrameArrivedEventArgs {}
impl ::core::clone::Clone for MultiSourceMediaFrameArrivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MultiSourceMediaFrameArrivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MultiSourceMediaFrameArrivedEventArgs {}
impl ::core::fmt::Debug for MultiSourceMediaFrameArrivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MultiSourceMediaFrameArrivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MultiSourceMediaFrameArrivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MultiSourceMediaFrameArrivedEventArgs;{63115e01-cf51-48fd-aab0-6d693eb48127})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MultiSourceMediaFrameArrivedEventArgs {
    type Vtable = IMultiSourceMediaFrameArrivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMultiSourceMediaFrameArrivedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MultiSourceMediaFrameArrivedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MultiSourceMediaFrameArrivedEventArgs";
}
impl ::core::convert::From<MultiSourceMediaFrameArrivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MultiSourceMediaFrameArrivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MultiSourceMediaFrameArrivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MultiSourceMediaFrameArrivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MultiSourceMediaFrameArrivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MultiSourceMediaFrameArrivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MultiSourceMediaFrameArrivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MultiSourceMediaFrameArrivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MultiSourceMediaFrameArrivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MultiSourceMediaFrameArrivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MultiSourceMediaFrameArrivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MultiSourceMediaFrameArrivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MultiSourceMediaFrameArrivedEventArgs {}
unsafe impl ::core::marker::Sync for MultiSourceMediaFrameArrivedEventArgs {}
#[repr(transparent)]
pub struct MultiSourceMediaFrameReader(::windows_core::IUnknown);
impl MultiSourceMediaFrameReader {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn FrameArrived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MultiSourceMediaFrameReader, MultiSourceMediaFrameArrivedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).FrameArrived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveFrameArrived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFrameArrived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn TryAcquireLatestFrame(&self) -> ::windows_core::Result<MultiSourceMediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryAcquireLatestFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MultiSourceMediaFrameReference>(result__)
        }
    }
    pub fn StartAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<MultiSourceMediaFrameReaderStartStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<MultiSourceMediaFrameReaderStartStatus>>(result__)
        }
    }
    pub fn StopAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StopAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn SetAcquisitionMode(&self, value: MediaFrameReaderAcquisitionMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMultiSourceMediaFrameReader2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAcquisitionMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AcquisitionMode(&self) -> ::windows_core::Result<MediaFrameReaderAcquisitionMode> {
        let this = &::windows_core::Interface::cast::<IMultiSourceMediaFrameReader2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MediaFrameReaderAcquisitionMode>::zeroed();
            (::windows_core::Interface::vtable(this).AcquisitionMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaFrameReaderAcquisitionMode>(result__)
        }
    }
}
impl ::core::clone::Clone for MultiSourceMediaFrameReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MultiSourceMediaFrameReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MultiSourceMediaFrameReader {}
impl ::core::fmt::Debug for MultiSourceMediaFrameReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MultiSourceMediaFrameReader").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MultiSourceMediaFrameReader {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MultiSourceMediaFrameReader;{8d144402-f763-488d-98f2-b437bcf075e7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MultiSourceMediaFrameReader {
    type Vtable = IMultiSourceMediaFrameReader_Vtbl;
    const IID: ::windows_core::GUID = <IMultiSourceMediaFrameReader as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MultiSourceMediaFrameReader {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MultiSourceMediaFrameReader";
}
impl ::core::convert::From<MultiSourceMediaFrameReader> for ::windows_core::IUnknown {
    fn from(value: MultiSourceMediaFrameReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MultiSourceMediaFrameReader> for ::windows_core::IUnknown {
    fn from(value: &MultiSourceMediaFrameReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MultiSourceMediaFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MultiSourceMediaFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MultiSourceMediaFrameReader> for ::windows_core::IInspectable {
    fn from(value: MultiSourceMediaFrameReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MultiSourceMediaFrameReader> for ::windows_core::IInspectable {
    fn from(value: &MultiSourceMediaFrameReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MultiSourceMediaFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MultiSourceMediaFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MultiSourceMediaFrameReader> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: MultiSourceMediaFrameReader) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MultiSourceMediaFrameReader> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &MultiSourceMediaFrameReader) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for MultiSourceMediaFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &MultiSourceMediaFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MultiSourceMediaFrameReader {}
unsafe impl ::core::marker::Sync for MultiSourceMediaFrameReader {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MultiSourceMediaFrameReaderStartStatus(pub i32);
impl MultiSourceMediaFrameReaderStartStatus {
    pub const Success: Self = Self(0i32);
    pub const NotSupported: Self = Self(1i32);
    pub const InsufficientResources: Self = Self(2i32);
    pub const DeviceNotAvailable: Self = Self(3i32);
    pub const UnknownFailure: Self = Self(4i32);
}
impl ::core::marker::Copy for MultiSourceMediaFrameReaderStartStatus {}
impl ::core::clone::Clone for MultiSourceMediaFrameReaderStartStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MultiSourceMediaFrameReaderStartStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MultiSourceMediaFrameReaderStartStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MultiSourceMediaFrameReaderStartStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MultiSourceMediaFrameReaderStartStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MultiSourceMediaFrameReaderStartStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.Frames.MultiSourceMediaFrameReaderStartStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MultiSourceMediaFrameReference(::windows_core::IUnknown);
impl MultiSourceMediaFrameReference {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn TryGetFrameReferenceBySourceId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, sourceid: Param0) -> ::windows_core::Result<MediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetFrameReferenceBySourceId)(::windows_core::Interface::as_raw(this), sourceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<MediaFrameReference>(result__)
        }
    }
}
impl ::core::clone::Clone for MultiSourceMediaFrameReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MultiSourceMediaFrameReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MultiSourceMediaFrameReference {}
impl ::core::fmt::Debug for MultiSourceMediaFrameReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MultiSourceMediaFrameReference").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MultiSourceMediaFrameReference {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.MultiSourceMediaFrameReference;{21964b1a-7fe2-44d6-92e5-298e6d2810e9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MultiSourceMediaFrameReference {
    type Vtable = IMultiSourceMediaFrameReference_Vtbl;
    const IID: ::windows_core::GUID = <IMultiSourceMediaFrameReference as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MultiSourceMediaFrameReference {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MultiSourceMediaFrameReference";
}
impl ::core::convert::From<MultiSourceMediaFrameReference> for ::windows_core::IUnknown {
    fn from(value: MultiSourceMediaFrameReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MultiSourceMediaFrameReference> for ::windows_core::IUnknown {
    fn from(value: &MultiSourceMediaFrameReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MultiSourceMediaFrameReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MultiSourceMediaFrameReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MultiSourceMediaFrameReference> for ::windows_core::IInspectable {
    fn from(value: MultiSourceMediaFrameReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MultiSourceMediaFrameReference> for ::windows_core::IInspectable {
    fn from(value: &MultiSourceMediaFrameReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MultiSourceMediaFrameReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MultiSourceMediaFrameReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MultiSourceMediaFrameReference> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: MultiSourceMediaFrameReference) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MultiSourceMediaFrameReference> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &MultiSourceMediaFrameReference) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for MultiSourceMediaFrameReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &MultiSourceMediaFrameReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MultiSourceMediaFrameReference {}
unsafe impl ::core::marker::Sync for MultiSourceMediaFrameReference {}
#[repr(transparent)]
pub struct VideoMediaFrame(::windows_core::IUnknown);
impl VideoMediaFrame {
    pub fn FrameReference(&self) -> ::windows_core::Result<MediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FrameReference)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaFrameReference>(result__)
        }
    }
    pub fn VideoFormat(&self) -> ::windows_core::Result<VideoMediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<VideoMediaFrameFormat>(result__)
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
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Direct3DSurface(&self) -> ::windows_core::Result<::winrt_graphics::DirectX::Direct3D11::IDirect3DSurface> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Direct3DSurface)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::DirectX::Direct3D11::IDirect3DSurface>(result__)
        }
    }
    #[cfg(feature = "Media_Devices_Core")]
    pub fn CameraIntrinsics(&self) -> ::windows_core::Result<super::super::Devices::Core::CameraIntrinsics> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CameraIntrinsics)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Core::CameraIntrinsics>(result__)
        }
    }
    pub fn InfraredMediaFrame(&self) -> ::windows_core::Result<InfraredMediaFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InfraredMediaFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InfraredMediaFrame>(result__)
        }
    }
    pub fn DepthMediaFrame(&self) -> ::windows_core::Result<DepthMediaFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DepthMediaFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DepthMediaFrame>(result__)
        }
    }
    pub fn GetVideoFrame(&self) -> ::windows_core::Result<super::super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetVideoFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::VideoFrame>(result__)
        }
    }
}
impl ::core::clone::Clone for VideoMediaFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VideoMediaFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoMediaFrame {}
impl ::core::fmt::Debug for VideoMediaFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoMediaFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VideoMediaFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.VideoMediaFrame;{00dd4ccb-32bd-4fe1-a013-7cc13cf5dbcf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for VideoMediaFrame {
    type Vtable = IVideoMediaFrame_Vtbl;
    const IID: ::windows_core::GUID = <IVideoMediaFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VideoMediaFrame {
    const NAME: &'static str = "Windows.Media.Capture.Frames.VideoMediaFrame";
}
impl ::core::convert::From<VideoMediaFrame> for ::windows_core::IUnknown {
    fn from(value: VideoMediaFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoMediaFrame> for ::windows_core::IUnknown {
    fn from(value: &VideoMediaFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for VideoMediaFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a VideoMediaFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VideoMediaFrame> for ::windows_core::IInspectable {
    fn from(value: VideoMediaFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoMediaFrame> for ::windows_core::IInspectable {
    fn from(value: &VideoMediaFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for VideoMediaFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a VideoMediaFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VideoMediaFrame {}
unsafe impl ::core::marker::Sync for VideoMediaFrame {}
#[repr(transparent)]
pub struct VideoMediaFrameFormat(::windows_core::IUnknown);
impl VideoMediaFrameFormat {
    pub fn MediaFrameFormat(&self) -> ::windows_core::Result<MediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MediaFrameFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaFrameFormat>(result__)
        }
    }
    pub fn DepthFormat(&self) -> ::windows_core::Result<DepthMediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DepthFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DepthMediaFrameFormat>(result__)
        }
    }
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
}
impl ::core::clone::Clone for VideoMediaFrameFormat {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VideoMediaFrameFormat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoMediaFrameFormat {}
impl ::core::fmt::Debug for VideoMediaFrameFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoMediaFrameFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VideoMediaFrameFormat {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Frames.VideoMediaFrameFormat;{46027fc0-d71b-45c7-8f14-6d9a0ae604e4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for VideoMediaFrameFormat {
    type Vtable = IVideoMediaFrameFormat_Vtbl;
    const IID: ::windows_core::GUID = <IVideoMediaFrameFormat as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VideoMediaFrameFormat {
    const NAME: &'static str = "Windows.Media.Capture.Frames.VideoMediaFrameFormat";
}
impl ::core::convert::From<VideoMediaFrameFormat> for ::windows_core::IUnknown {
    fn from(value: VideoMediaFrameFormat) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoMediaFrameFormat> for ::windows_core::IUnknown {
    fn from(value: &VideoMediaFrameFormat) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for VideoMediaFrameFormat {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a VideoMediaFrameFormat {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VideoMediaFrameFormat> for ::windows_core::IInspectable {
    fn from(value: VideoMediaFrameFormat) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoMediaFrameFormat> for ::windows_core::IInspectable {
    fn from(value: &VideoMediaFrameFormat) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for VideoMediaFrameFormat {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a VideoMediaFrameFormat {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VideoMediaFrameFormat {}
unsafe impl ::core::marker::Sync for VideoMediaFrameFormat {}
