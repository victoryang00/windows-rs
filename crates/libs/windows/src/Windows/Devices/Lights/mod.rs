#[cfg(feature = "Devices_Lights_Effects")]
pub mod Effects;
#[doc(hidden)]
#[repr(transparent)]
pub struct ILamp(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILamp {
    type Vtable = ILamp_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x047d5b9a_ea45_4b2b_b1a2_14dff00bde7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILamp_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub BrightnessLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetBrightnessLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub IsColorSettable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub Color: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Color: usize,
    #[cfg(feature = "UI")]
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColor: usize,
    #[cfg(feature = "Foundation")]
    pub AvailabilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AvailabilityChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAvailabilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAvailabilityChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArray(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampArray {
    type Vtable = ILampArray_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7ace9787_c8a0_4e95_a1e0_d58676538649);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArray_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HardwareVendorId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub HardwareProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub HardwareVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub LampArrayKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LampArrayKind) -> ::windows_core::HRESULT,
    pub LampCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MinUpdateInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinUpdateInterval: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub BoundingBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    BoundingBox: usize,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub BrightnessLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetBrightnessLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SupportsVirtualKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub GetLampInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lampindex: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub GetIndicesForKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: super::super::System::VirtualKey, result_size__: *mut u32, result__: *mut *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetIndicesForKey: usize,
    pub GetIndicesForPurposes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, purposes: LampPurposes, result_size__: *mut u32, result__: *mut *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredcolor: super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColor: usize,
    #[cfg(feature = "UI")]
    pub SetColorForIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lampindex: i32, desiredcolor: super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColorForIndex: usize,
    #[cfg(feature = "UI")]
    pub SetSingleColorForIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredcolor: super::super::UI::Color, lampIndexes_array_size: u32, lampindexes: *const i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetSingleColorForIndices: usize,
    #[cfg(feature = "UI")]
    pub SetColorsForIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredColors_array_size: u32, desiredcolors: *const super::super::UI::Color, lampIndexes_array_size: u32, lampindexes: *const i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColorsForIndices: usize,
    #[cfg(all(feature = "System", feature = "UI"))]
    pub SetColorsForKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredcolor: super::super::UI::Color, key: super::super::System::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "System", feature = "UI")))]
    SetColorsForKey: usize,
    #[cfg(all(feature = "System", feature = "UI"))]
    pub SetColorsForKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredColors_array_size: u32, desiredcolors: *const super::super::UI::Color, keys_array_size: u32, keys: *const super::super::System::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "System", feature = "UI")))]
    SetColorsForKeys: usize,
    #[cfg(feature = "UI")]
    pub SetColorsForPurposes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredcolor: super::super::UI::Color, purposes: LampPurposes) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColorsForPurposes: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SendMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageid: i32, message: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SendMessageAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub RequestMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageid: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    RequestMessageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampArrayStatics {
    type Vtable = ILampArrayStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7bb8c98d_5fc1_452d_bb1f_4ad410d398ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampAvailabilityChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampAvailabilityChangedEventArgs {
    type Vtable = ILampAvailabilityChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4f6e3ded_07a2_499d_9260_67e304532ba4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampAvailabilityChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampInfo {
    type Vtable = ILampInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30bb521c_0acf_49da_8c10_150b9cf62713);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Index: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Purposes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LampPurposes) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    pub RedLevelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub GreenLevelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub BlueLevelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub GainLevelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub FixedColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI")))]
    FixedColor: usize,
    #[cfg(feature = "UI")]
    pub GetNearestSupportedColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredcolor: super::super::UI::Color, result__: *mut super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    GetNearestSupportedColor: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateLatency: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampStatics {
    type Vtable = ILampStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa822416c_8885_401e_b821_8e8b38a8e8ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
}
#[doc = "*Required features: `\"Devices_Lights\"`*"]
#[repr(transparent)]
pub struct Lamp(::windows_core::IUnknown);
impl Lamp {
    #[doc = "*Required features: `\"Devices_Lights\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`*"]
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`*"]
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`*"]
    pub fn BrightnessLevel(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).BrightnessLevel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`*"]
    pub fn SetBrightnessLevel(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBrightnessLevel)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`*"]
    pub fn IsColorSettable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsColorSettable)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`, `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn Color(&self) -> ::windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::UI::Color>::zeroed();
            (::windows_core::Interface::vtable(this).Color)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`, `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetColor<'a, Param0: ::windows_core::IntoParam<'a, super::super::UI::Color>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AvailabilityChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<Lamp, LampAvailabilityChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AvailabilityChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAvailabilityChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAvailabilityChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`*"]
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ILampStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Lights\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Lamp>> {
        Self::ILampStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<Lamp>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Lights\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Lamp>> {
        Self::ILampStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<Lamp>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILampStatics<R, F: FnOnce(&ILampStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Lamp, ILampStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Lamp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Lamp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Lamp {}
impl ::core::fmt::Debug for Lamp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Lamp").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Lamp {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Lamp;{047d5b9a-ea45-4b2b-b1a2-14dff00bde7b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Lamp {
    type Vtable = ILamp_Vtbl;
    const IID: ::windows_core::GUID = <ILamp as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Lamp {
    const NAME: &'static str = "Windows.Devices.Lights.Lamp";
}
impl ::core::convert::From<Lamp> for ::windows_core::IUnknown {
    fn from(value: Lamp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Lamp> for ::windows_core::IUnknown {
    fn from(value: &Lamp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Lamp {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Lamp {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Lamp> for ::windows_core::IInspectable {
    fn from(value: Lamp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Lamp> for ::windows_core::IInspectable {
    fn from(value: &Lamp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Lamp {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Lamp {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<Lamp> for super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: Lamp) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&Lamp> for super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &Lamp) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Foundation::IClosable> for Lamp {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Foundation::IClosable> for &Lamp {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for Lamp {}
unsafe impl ::core::marker::Sync for Lamp {}
#[doc = "*Required features: `\"Devices_Lights\"`*"]
#[repr(transparent)]
pub struct LampArray(::windows_core::IUnknown);
impl LampArray {
    #[doc = "*Required features: `\"Devices_Lights\"`*"]
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`*"]
    pub fn HardwareVendorId(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).HardwareVendorId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`*"]
    pub fn HardwareProductId(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).HardwareProductId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`*"]
    pub fn HardwareVersion(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).HardwareVersion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`*"]
    pub fn LampArrayKind(&self) -> ::windows_core::Result<LampArrayKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<LampArrayKind>::zeroed();
            (::windows_core::Interface::vtable(this).LampArrayKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LampArrayKind>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`*"]
    pub fn LampCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).LampCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MinUpdateInterval(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).MinUpdateInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn BoundingBox(&self) -> ::windows_core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).BoundingBox)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`*"]
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`*"]
    pub fn BrightnessLevel(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).BrightnessLevel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`*"]
    pub fn SetBrightnessLevel(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBrightnessLevel)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`*"]
    pub fn IsConnected(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsConnected)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`*"]
    pub fn SupportsVirtualKeys(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).SupportsVirtualKeys)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`*"]
    pub fn GetLampInfo(&self, lampindex: i32) -> ::windows_core::Result<LampInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetLampInfo)(::windows_core::Interface::as_raw(this), lampindex, result__.as_mut_ptr()).from_abi::<LampInfo>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetIndicesForKey(&self, key: super::super::System::VirtualKey) -> ::windows_core::Result<::windows_core::Array<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<i32>>::zeroed();
            (::windows_core::Interface::vtable(this).GetIndicesForKey)(::windows_core::Interface::as_raw(this), key, ::windows_core::Array::<i32>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`*"]
    pub fn GetIndicesForPurposes(&self, purposes: LampPurposes) -> ::windows_core::Result<::windows_core::Array<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<i32>>::zeroed();
            (::windows_core::Interface::vtable(this).GetIndicesForPurposes)(::windows_core::Interface::as_raw(this), purposes, ::windows_core::Array::<i32>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`, `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetColor<'a, Param0: ::windows_core::IntoParam<'a, super::super::UI::Color>>(&self, desiredcolor: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColor)(::windows_core::Interface::as_raw(this), desiredcolor.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`, `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetColorForIndex<'a, Param1: ::windows_core::IntoParam<'a, super::super::UI::Color>>(&self, lampindex: i32, desiredcolor: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColorForIndex)(::windows_core::Interface::as_raw(this), lampindex, desiredcolor.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`, `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetSingleColorForIndices<'a, Param0: ::windows_core::IntoParam<'a, super::super::UI::Color>>(&self, desiredcolor: Param0, lampindexes: &[i32]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSingleColorForIndices)(::windows_core::Interface::as_raw(this), desiredcolor.into_param().abi(), lampindexes.len() as u32, ::core::mem::transmute(lampindexes.as_ptr())).ok() }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`, `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetColorsForIndices(&self, desiredcolors: &[super::super::UI::Color], lampindexes: &[i32]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColorsForIndices)(::windows_core::Interface::as_raw(this), desiredcolors.len() as u32, ::core::mem::transmute(desiredcolors.as_ptr()), lampindexes.len() as u32, ::core::mem::transmute(lampindexes.as_ptr())).ok() }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`, `\"System\"`, `\"UI\"`*"]
    #[cfg(all(feature = "System", feature = "UI"))]
    pub fn SetColorsForKey<'a, Param0: ::windows_core::IntoParam<'a, super::super::UI::Color>>(&self, desiredcolor: Param0, key: super::super::System::VirtualKey) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColorsForKey)(::windows_core::Interface::as_raw(this), desiredcolor.into_param().abi(), key).ok() }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`, `\"System\"`, `\"UI\"`*"]
    #[cfg(all(feature = "System", feature = "UI"))]
    pub fn SetColorsForKeys(&self, desiredcolors: &[super::super::UI::Color], keys: &[super::super::System::VirtualKey]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColorsForKeys)(::windows_core::Interface::as_raw(this), desiredcolors.len() as u32, ::core::mem::transmute(desiredcolors.as_ptr()), keys.len() as u32, ::core::mem::transmute(keys.as_ptr())).ok() }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`, `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetColorsForPurposes<'a, Param0: ::windows_core::IntoParam<'a, super::super::UI::Color>>(&self, desiredcolor: Param0, purposes: LampPurposes) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColorsForPurposes)(::windows_core::Interface::as_raw(this), desiredcolor.into_param().abi(), purposes).ok() }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SendMessageAsync<'a, Param1: ::windows_core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, messageid: i32, message: Param1) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SendMessageAsync)(::windows_core::Interface::as_raw(this), messageid, message.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn RequestMessageAsync(&self, messageid: i32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestMessageAsync)(::windows_core::Interface::as_raw(this), messageid, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`*"]
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ILampArrayStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Lights\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<LampArray>> {
        Self::ILampArrayStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<LampArray>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILampArrayStatics<R, F: FnOnce(&ILampArrayStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LampArray, ILampArrayStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LampArray {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LampArray {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampArray {}
impl ::core::fmt::Debug for LampArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArray").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LampArray {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.LampArray;{7ace9787-c8a0-4e95-a1e0-d58676538649})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LampArray {
    type Vtable = ILampArray_Vtbl;
    const IID: ::windows_core::GUID = <ILampArray as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LampArray {
    const NAME: &'static str = "Windows.Devices.Lights.LampArray";
}
impl ::core::convert::From<LampArray> for ::windows_core::IUnknown {
    fn from(value: LampArray) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArray> for ::windows_core::IUnknown {
    fn from(value: &LampArray) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LampArray {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LampArray {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LampArray> for ::windows_core::IInspectable {
    fn from(value: LampArray) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArray> for ::windows_core::IInspectable {
    fn from(value: &LampArray) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LampArray {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LampArray {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LampArray {}
unsafe impl ::core::marker::Sync for LampArray {}
#[doc = "*Required features: `\"Devices_Lights\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LampArrayKind(pub i32);
impl LampArrayKind {
    pub const Undefined: Self = Self(0i32);
    pub const Keyboard: Self = Self(1i32);
    pub const Mouse: Self = Self(2i32);
    pub const GameController: Self = Self(3i32);
    pub const Peripheral: Self = Self(4i32);
    pub const Scene: Self = Self(5i32);
    pub const Notification: Self = Self(6i32);
    pub const Chassis: Self = Self(7i32);
    pub const Wearable: Self = Self(8i32);
    pub const Furniture: Self = Self(9i32);
    pub const Art: Self = Self(10i32);
}
impl ::core::marker::Copy for LampArrayKind {}
impl ::core::clone::Clone for LampArrayKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LampArrayKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LampArrayKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for LampArrayKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LampArrayKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Lights.LampArrayKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Lights\"`*"]
#[repr(transparent)]
pub struct LampAvailabilityChangedEventArgs(::windows_core::IUnknown);
impl LampAvailabilityChangedEventArgs {
    #[doc = "*Required features: `\"Devices_Lights\"`*"]
    pub fn IsAvailable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAvailable)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for LampAvailabilityChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LampAvailabilityChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampAvailabilityChangedEventArgs {}
impl ::core::fmt::Debug for LampAvailabilityChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampAvailabilityChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LampAvailabilityChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.LampAvailabilityChangedEventArgs;{4f6e3ded-07a2-499d-9260-67e304532ba4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LampAvailabilityChangedEventArgs {
    type Vtable = ILampAvailabilityChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ILampAvailabilityChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LampAvailabilityChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Lights.LampAvailabilityChangedEventArgs";
}
impl ::core::convert::From<LampAvailabilityChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: LampAvailabilityChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampAvailabilityChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &LampAvailabilityChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LampAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LampAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LampAvailabilityChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: LampAvailabilityChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampAvailabilityChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &LampAvailabilityChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LampAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LampAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LampAvailabilityChangedEventArgs {}
unsafe impl ::core::marker::Sync for LampAvailabilityChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Lights\"`*"]
#[repr(transparent)]
pub struct LampInfo(::windows_core::IUnknown);
impl LampInfo {
    #[doc = "*Required features: `\"Devices_Lights\"`*"]
    pub fn Index(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Index)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`*"]
    pub fn Purposes(&self) -> ::windows_core::Result<LampPurposes> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<LampPurposes>::zeroed();
            (::windows_core::Interface::vtable(this).Purposes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LampPurposes>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(&self) -> ::windows_core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`*"]
    pub fn RedLevelCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).RedLevelCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`*"]
    pub fn GreenLevelCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).GreenLevelCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`*"]
    pub fn BlueLevelCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).BlueLevelCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`*"]
    pub fn GainLevelCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).GainLevelCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`, `\"Foundation\"`, `\"UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub fn FixedColor(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FixedColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::UI::Color>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`, `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn GetNearestSupportedColor<'a, Param0: ::windows_core::IntoParam<'a, super::super::UI::Color>>(&self, desiredcolor: Param0) -> ::windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::UI::Color>::zeroed();
            (::windows_core::Interface::vtable(this).GetNearestSupportedColor)(::windows_core::Interface::as_raw(this), desiredcolor.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Lights\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateLatency(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateLatency)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for LampInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LampInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampInfo {}
impl ::core::fmt::Debug for LampInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LampInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.LampInfo;{30bb521c-0acf-49da-8c10-150b9cf62713})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LampInfo {
    type Vtable = ILampInfo_Vtbl;
    const IID: ::windows_core::GUID = <ILampInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LampInfo {
    const NAME: &'static str = "Windows.Devices.Lights.LampInfo";
}
impl ::core::convert::From<LampInfo> for ::windows_core::IUnknown {
    fn from(value: LampInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampInfo> for ::windows_core::IUnknown {
    fn from(value: &LampInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LampInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LampInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LampInfo> for ::windows_core::IInspectable {
    fn from(value: LampInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampInfo> for ::windows_core::IInspectable {
    fn from(value: &LampInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LampInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LampInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LampInfo {}
unsafe impl ::core::marker::Sync for LampInfo {}
#[doc = "*Required features: `\"Devices_Lights\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LampPurposes(pub u32);
impl LampPurposes {
    pub const Undefined: Self = Self(0u32);
    pub const Control: Self = Self(1u32);
    pub const Accent: Self = Self(2u32);
    pub const Branding: Self = Self(4u32);
    pub const Status: Self = Self(8u32);
    pub const Illumination: Self = Self(16u32);
    pub const Presentation: Self = Self(32u32);
}
impl ::core::marker::Copy for LampPurposes {}
impl ::core::clone::Clone for LampPurposes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LampPurposes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LampPurposes {
    type Abi = Self;
}
impl ::core::fmt::Debug for LampPurposes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampPurposes").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LampPurposes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LampPurposes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LampPurposes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LampPurposes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LampPurposes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for LampPurposes {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Lights.LampPurposes;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
