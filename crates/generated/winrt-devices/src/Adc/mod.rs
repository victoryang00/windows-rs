#[cfg(feature = "Provider")]
pub mod Provider;
#[repr(transparent)]
pub struct AdcChannel(::windows_core::IUnknown);
impl AdcChannel {
    pub fn Controller(&self) -> ::windows_core::Result<AdcController> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Controller)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AdcController>(result__)
        }
    }
    pub fn ReadValue(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).ReadValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn ReadRatio(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ReadRatio)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for AdcChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdcChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdcChannel {}
impl ::core::fmt::Debug for AdcChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdcChannel").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AdcChannel {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Adc.AdcChannel;{040bf414-2588-4a56-abef-73a260acc60a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AdcChannel {
    type Vtable = IAdcChannel_Vtbl;
    const IID: ::windows_core::GUID = <IAdcChannel as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AdcChannel {
    const NAME: &'static str = "Windows.Devices.Adc.AdcChannel";
}
impl ::core::convert::From<AdcChannel> for ::windows_core::IUnknown {
    fn from(value: AdcChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdcChannel> for ::windows_core::IUnknown {
    fn from(value: &AdcChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AdcChannel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AdcChannel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdcChannel> for ::windows_core::IInspectable {
    fn from(value: AdcChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdcChannel> for ::windows_core::IInspectable {
    fn from(value: &AdcChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AdcChannel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AdcChannel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AdcChannel> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: AdcChannel) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AdcChannel> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &AdcChannel) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for AdcChannel {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &AdcChannel {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AdcChannel {}
unsafe impl ::core::marker::Sync for AdcChannel {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AdcChannelMode(pub i32);
impl AdcChannelMode {
    pub const SingleEnded: Self = Self(0i32);
    pub const Differential: Self = Self(1i32);
}
impl ::core::marker::Copy for AdcChannelMode {}
impl ::core::clone::Clone for AdcChannelMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AdcChannelMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AdcChannelMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for AdcChannelMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdcChannelMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AdcChannelMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Adc.AdcChannelMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AdcController(::windows_core::IUnknown);
impl AdcController {
    pub fn ChannelCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).ChannelCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn ResolutionInBits(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).ResolutionInBits)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn MinValue(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).MinValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn MaxValue(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn ChannelMode(&self) -> ::windows_core::Result<AdcChannelMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AdcChannelMode>::zeroed();
            (::windows_core::Interface::vtable(this).ChannelMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AdcChannelMode>(result__)
        }
    }
    pub fn SetChannelMode(&self, value: AdcChannelMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetChannelMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsChannelModeSupported(&self, channelmode: AdcChannelMode) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsChannelModeSupported)(::windows_core::Interface::as_raw(this), channelmode, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OpenChannel(&self, channelnumber: i32) -> ::windows_core::Result<AdcChannel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenChannel)(::windows_core::Interface::as_raw(this), channelnumber, result__.as_mut_ptr()).from_abi::<AdcChannel>(result__)
        }
    }
    #[cfg(all(feature = "winrt-devices", feature = "winrt-foundation"))]
    pub fn GetControllersAsync<'a, Param0: ::windows_core::IntoParam<'a, Provider::IAdcProvider>>(provider: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<AdcController>>> {
        Self::IAdcControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetControllersAsync)(::windows_core::Interface::as_raw(this), provider.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<AdcController>>>(result__)
        })
    }
    pub fn GetDefaultAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AdcController>> {
        Self::IAdcControllerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AdcController>>(result__)
        })
    }
    pub fn IAdcControllerStatics<R, F: FnOnce(&IAdcControllerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AdcController, IAdcControllerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAdcControllerStatics2<R, F: FnOnce(&IAdcControllerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AdcController, IAdcControllerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AdcController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdcController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdcController {}
impl ::core::fmt::Debug for AdcController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdcController").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AdcController {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Adc.AdcController;{2a76e4b0-a896-4219-86b6-ea8cdce98f56})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AdcController {
    type Vtable = IAdcController_Vtbl;
    const IID: ::windows_core::GUID = <IAdcController as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AdcController {
    const NAME: &'static str = "Windows.Devices.Adc.AdcController";
}
impl ::core::convert::From<AdcController> for ::windows_core::IUnknown {
    fn from(value: AdcController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdcController> for ::windows_core::IUnknown {
    fn from(value: &AdcController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AdcController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AdcController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdcController> for ::windows_core::IInspectable {
    fn from(value: AdcController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdcController> for ::windows_core::IInspectable {
    fn from(value: &AdcController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AdcController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AdcController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdcController {}
unsafe impl ::core::marker::Sync for AdcController {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdcChannel(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdcChannel {
    type Vtable = IAdcChannel_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x040bf414_2588_4a56_abef_73a260acc60a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdcChannel_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Controller: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReadValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub ReadRatio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdcController(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdcController {
    type Vtable = IAdcController_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a76e4b0_a896_4219_86b6_ea8cdce98f56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdcController_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ChannelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub ResolutionInBits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MinValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MaxValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub ChannelMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AdcChannelMode) -> ::windows_core::HRESULT,
    pub SetChannelMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AdcChannelMode) -> ::windows_core::HRESULT,
    pub IsChannelModeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channelmode: AdcChannelMode, result__: *mut bool) -> ::windows_core::HRESULT,
    pub OpenChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channelnumber: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdcControllerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdcControllerStatics {
    type Vtable = IAdcControllerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcce98e0c_01f8_4891_bc3b_be53ef279ca4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdcControllerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "winrt-devices", feature = "winrt-foundation"))]
    pub GetControllersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-devices", feature = "winrt-foundation")))]
    GetControllersAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdcControllerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdcControllerStatics2 {
    type Vtable = IAdcControllerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa2b93b1d_977b_4f5a_a5fe_a6abaffe6484);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdcControllerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
