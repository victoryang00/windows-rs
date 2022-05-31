#[cfg(feature = "Preview")]
pub mod Preview;
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyboardCapabilities(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyboardCapabilities {
    type Vtable = IKeyboardCapabilities_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3a3f9b56_6798_4bbc_833e_0f34b17c65ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardCapabilities_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub KeyboardPresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMouseCapabilities(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMouseCapabilities {
    type Vtable = IMouseCapabilities_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbca5e023_7dd9_4b6b_9a92_55d43cb38f73);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMouseCapabilities_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MousePresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub VerticalWheelPresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub HorizontalWheelPresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SwapButtons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub NumberOfButtons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMouseDevice(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMouseDevice {
    type Vtable = IMouseDevice_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x88edf458_f2c8_49f4_be1f_c256b388bc11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMouseDevice_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MouseMoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveMouseMoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMouseDeviceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMouseDeviceStatics {
    type Vtable = IMouseDeviceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x484a9045_6d70_49db_8e68_46ffbd17d38d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMouseDeviceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMouseEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMouseEventArgs {
    type Vtable = IMouseEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf625aa5d_2354_4cc7_9230_96941c969fde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMouseEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MouseDelta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MouseDelta) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenButtonListener(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPenButtonListener {
    type Vtable = IPenButtonListener_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8245c376_1ee3_53f7_b1f7_8334a16f2815);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenButtonListener_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsSupportedChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveIsSupportedChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub TailButtonClicked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveTailButtonClicked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub TailButtonDoubleClicked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveTailButtonDoubleClicked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub TailButtonLongPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveTailButtonLongPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenButtonListenerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPenButtonListenerStatics {
    type Vtable = IPenButtonListenerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x19a8a584_862f_5f69_bfea_05f6584f133f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenButtonListenerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenDevice(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPenDevice {
    type Vtable = IPenDevice_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x31856eba_a738_5a8c_b8f6_f97ef68d18ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenDevice_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PenId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenDevice2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPenDevice2 {
    type Vtable = IPenDevice2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0207d327_7fb8_5566_8c34_f8342037b7f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenDevice2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenDeviceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPenDeviceStatics {
    type Vtable = IPenDeviceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9dfbbe01_0966_5180_bcb4_b85060e39479);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenDeviceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetFromPointerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenDockListener(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPenDockListener {
    type Vtable = IPenDockListener_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x759f4d90_1dc0_55cb_ad18_b9101456f592);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenDockListener_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsSupportedChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveIsSupportedChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Docked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Undocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveUndocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenDockListenerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPenDockListenerStatics {
    type Vtable = IPenDockListenerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcab75e9a_0016_5c72_969e_a97e11992a93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenDockListenerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenDockedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPenDockedEventArgs {
    type Vtable = IPenDockedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfd4277c6_ca63_5d4e_9ed3_a28a54521c8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenDockedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenTailButtonClickedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPenTailButtonClickedEventArgs {
    type Vtable = IPenTailButtonClickedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d2fb7b6_6ad3_5d3e_ab29_05ea2410e390);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenTailButtonClickedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenTailButtonDoubleClickedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPenTailButtonDoubleClickedEventArgs {
    type Vtable = IPenTailButtonDoubleClickedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x846321a2_618a_5478_b04c_b358231da4a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenTailButtonDoubleClickedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenTailButtonLongPressedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPenTailButtonLongPressedEventArgs {
    type Vtable = IPenTailButtonLongPressedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf37c606e_c60a_5f42_b818_a53112406c13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenTailButtonLongPressedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenUndockedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPenUndockedEventArgs {
    type Vtable = IPenUndockedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xccd09150_261b_59e6_a5d4_c1964cd03feb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenUndockedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerDevice(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointerDevice {
    type Vtable = IPointerDevice_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x93c9bafc_ebcb_467e_82c6_276feae36b5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerDevice_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PointerDeviceType) -> ::windows_core::HRESULT,
    pub IsIntegrated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub MaxContacts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub PhysicalDeviceRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    pub ScreenRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedUsages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedUsages: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerDevice2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointerDevice2 {
    type Vtable = IPointerDevice2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf8a6d2a0_c484_489f_ae3e_30d2ee1ffd3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerDevice2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MaxPointersWithZDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerDeviceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointerDeviceStatics {
    type Vtable = IPointerDeviceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd8b89aa1_d1c6_416e_bd8d_5790914dc563);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerDeviceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetPointerDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPointerDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPointerDevices: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITouchCapabilities(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITouchCapabilities {
    type Vtable = ITouchCapabilities_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x20dd55f9_13f1_46c8_9285_2c05fa3eda6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITouchCapabilities_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TouchPresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Contacts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct KeyboardCapabilities(::windows_core::IUnknown);
impl KeyboardCapabilities {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<KeyboardCapabilities, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn KeyboardPresent(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).KeyboardPresent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
}
impl ::core::clone::Clone for KeyboardCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for KeyboardCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyboardCapabilities {}
impl ::core::fmt::Debug for KeyboardCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyboardCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for KeyboardCapabilities {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.KeyboardCapabilities;{3a3f9b56-6798-4bbc-833e-0f34b17c65ff})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for KeyboardCapabilities {
    type Vtable = IKeyboardCapabilities_Vtbl;
    const IID: ::windows_core::GUID = <IKeyboardCapabilities as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for KeyboardCapabilities {
    const NAME: &'static str = "Windows.Devices.Input.KeyboardCapabilities";
}
impl ::core::convert::From<KeyboardCapabilities> for ::windows_core::IUnknown {
    fn from(value: KeyboardCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyboardCapabilities> for ::windows_core::IUnknown {
    fn from(value: &KeyboardCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for KeyboardCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a KeyboardCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<KeyboardCapabilities> for ::windows_core::IInspectable {
    fn from(value: KeyboardCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyboardCapabilities> for ::windows_core::IInspectable {
    fn from(value: &KeyboardCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for KeyboardCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a KeyboardCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for KeyboardCapabilities {}
unsafe impl ::core::marker::Sync for KeyboardCapabilities {}
#[repr(transparent)]
pub struct MouseCapabilities(::windows_core::IUnknown);
impl MouseCapabilities {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MouseCapabilities, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn MousePresent(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).MousePresent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn VerticalWheelPresent(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).VerticalWheelPresent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn HorizontalWheelPresent(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).HorizontalWheelPresent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SwapButtons(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).SwapButtons)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn NumberOfButtons(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).NumberOfButtons)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for MouseCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MouseCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MouseCapabilities {}
impl ::core::fmt::Debug for MouseCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MouseCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MouseCapabilities {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.MouseCapabilities;{bca5e023-7dd9-4b6b-9a92-55d43cb38f73})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MouseCapabilities {
    type Vtable = IMouseCapabilities_Vtbl;
    const IID: ::windows_core::GUID = <IMouseCapabilities as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MouseCapabilities {
    const NAME: &'static str = "Windows.Devices.Input.MouseCapabilities";
}
impl ::core::convert::From<MouseCapabilities> for ::windows_core::IUnknown {
    fn from(value: MouseCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MouseCapabilities> for ::windows_core::IUnknown {
    fn from(value: &MouseCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MouseCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MouseCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MouseCapabilities> for ::windows_core::IInspectable {
    fn from(value: MouseCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MouseCapabilities> for ::windows_core::IInspectable {
    fn from(value: &MouseCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MouseCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MouseCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MouseCapabilities {}
unsafe impl ::core::marker::Sync for MouseCapabilities {}
#[repr(C)]
pub struct MouseDelta {
    pub X: i32,
    pub Y: i32,
}
impl ::core::marker::Copy for MouseDelta {}
impl ::core::clone::Clone for MouseDelta {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MouseDelta {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MouseDelta").field("X", &self.X).field("Y", &self.Y).finish()
    }
}
unsafe impl ::windows_core::Abi for MouseDelta {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for MouseDelta {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Devices.Input.MouseDelta;i4;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for MouseDelta {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MouseDelta>()) == 0 }
    }
}
impl ::core::cmp::Eq for MouseDelta {}
impl ::core::default::Default for MouseDelta {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct MouseDevice(::windows_core::IUnknown);
impl MouseDevice {
    pub fn MouseMoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<MouseDevice, MouseEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).MouseMoved)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveMouseMoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMouseMoved)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<MouseDevice> {
        Self::IMouseDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MouseDevice>(result__)
        })
    }
    pub fn IMouseDeviceStatics<R, F: FnOnce(&IMouseDeviceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MouseDevice, IMouseDeviceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MouseDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MouseDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MouseDevice {}
impl ::core::fmt::Debug for MouseDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MouseDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MouseDevice {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.MouseDevice;{88edf458-f2c8-49f4-be1f-c256b388bc11})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MouseDevice {
    type Vtable = IMouseDevice_Vtbl;
    const IID: ::windows_core::GUID = <IMouseDevice as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MouseDevice {
    const NAME: &'static str = "Windows.Devices.Input.MouseDevice";
}
impl ::core::convert::From<MouseDevice> for ::windows_core::IUnknown {
    fn from(value: MouseDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MouseDevice> for ::windows_core::IUnknown {
    fn from(value: &MouseDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MouseDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MouseDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MouseDevice> for ::windows_core::IInspectable {
    fn from(value: MouseDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MouseDevice> for ::windows_core::IInspectable {
    fn from(value: &MouseDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MouseDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MouseDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct MouseEventArgs(::windows_core::IUnknown);
impl MouseEventArgs {
    pub fn MouseDelta(&self) -> ::windows_core::Result<MouseDelta> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MouseDelta>::zeroed();
            (::windows_core::Interface::vtable(this).MouseDelta)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MouseDelta>(result__)
        }
    }
}
impl ::core::clone::Clone for MouseEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MouseEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MouseEventArgs {}
impl ::core::fmt::Debug for MouseEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MouseEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MouseEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.MouseEventArgs;{f625aa5d-2354-4cc7-9230-96941c969fde})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MouseEventArgs {
    type Vtable = IMouseEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMouseEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MouseEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.MouseEventArgs";
}
impl ::core::convert::From<MouseEventArgs> for ::windows_core::IUnknown {
    fn from(value: MouseEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MouseEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MouseEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MouseEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MouseEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MouseEventArgs> for ::windows_core::IInspectable {
    fn from(value: MouseEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MouseEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MouseEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MouseEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MouseEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct PenButtonListener(::windows_core::IUnknown);
impl PenButtonListener {
    pub fn IsSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsSupportedChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PenButtonListener, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).IsSupportedChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveIsSupportedChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveIsSupportedChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn TailButtonClicked<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PenButtonListener, PenTailButtonClickedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).TailButtonClicked)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveTailButtonClicked<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveTailButtonClicked)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn TailButtonDoubleClicked<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PenButtonListener, PenTailButtonDoubleClickedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).TailButtonDoubleClicked)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveTailButtonDoubleClicked<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveTailButtonDoubleClicked)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn TailButtonLongPressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PenButtonListener, PenTailButtonLongPressedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).TailButtonLongPressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveTailButtonLongPressed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveTailButtonLongPressed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetDefault() -> ::windows_core::Result<PenButtonListener> {
        Self::IPenButtonListenerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PenButtonListener>(result__)
        })
    }
    pub fn IPenButtonListenerStatics<R, F: FnOnce(&IPenButtonListenerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PenButtonListener, IPenButtonListenerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PenButtonListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PenButtonListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PenButtonListener {}
impl ::core::fmt::Debug for PenButtonListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenButtonListener").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PenButtonListener {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.PenButtonListener;{8245c376-1ee3-53f7-b1f7-8334a16f2815})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PenButtonListener {
    type Vtable = IPenButtonListener_Vtbl;
    const IID: ::windows_core::GUID = <IPenButtonListener as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PenButtonListener {
    const NAME: &'static str = "Windows.Devices.Input.PenButtonListener";
}
impl ::core::convert::From<PenButtonListener> for ::windows_core::IUnknown {
    fn from(value: PenButtonListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PenButtonListener> for ::windows_core::IUnknown {
    fn from(value: &PenButtonListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PenButtonListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PenButtonListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PenButtonListener> for ::windows_core::IInspectable {
    fn from(value: PenButtonListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PenButtonListener> for ::windows_core::IInspectable {
    fn from(value: &PenButtonListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PenButtonListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PenButtonListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PenButtonListener {}
unsafe impl ::core::marker::Sync for PenButtonListener {}
#[repr(transparent)]
pub struct PenDevice(::windows_core::IUnknown);
impl PenDevice {
    pub fn PenId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).PenId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    #[cfg(feature = "Devices_Haptics")]
    pub fn SimpleHapticsController(&self) -> ::windows_core::Result<super::Haptics::SimpleHapticsController> {
        let this = &::windows_core::Interface::cast::<IPenDevice2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SimpleHapticsController)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Haptics::SimpleHapticsController>(result__)
        }
    }
    pub fn GetFromPointerId(pointerid: u32) -> ::windows_core::Result<PenDevice> {
        Self::IPenDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFromPointerId)(::windows_core::Interface::as_raw(this), pointerid, result__.as_mut_ptr()).from_abi::<PenDevice>(result__)
        })
    }
    pub fn IPenDeviceStatics<R, F: FnOnce(&IPenDeviceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PenDevice, IPenDeviceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PenDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PenDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PenDevice {}
impl ::core::fmt::Debug for PenDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PenDevice {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.PenDevice;{31856eba-a738-5a8c-b8f6-f97ef68d18ef})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PenDevice {
    type Vtable = IPenDevice_Vtbl;
    const IID: ::windows_core::GUID = <IPenDevice as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PenDevice {
    const NAME: &'static str = "Windows.Devices.Input.PenDevice";
}
impl ::core::convert::From<PenDevice> for ::windows_core::IUnknown {
    fn from(value: PenDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PenDevice> for ::windows_core::IUnknown {
    fn from(value: &PenDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PenDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PenDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PenDevice> for ::windows_core::IInspectable {
    fn from(value: PenDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PenDevice> for ::windows_core::IInspectable {
    fn from(value: &PenDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PenDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PenDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PenDevice {}
unsafe impl ::core::marker::Sync for PenDevice {}
#[repr(transparent)]
pub struct PenDockListener(::windows_core::IUnknown);
impl PenDockListener {
    pub fn IsSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsSupportedChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PenDockListener, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).IsSupportedChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveIsSupportedChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveIsSupportedChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Docked<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PenDockListener, PenDockedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Docked)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDocked<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDocked)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Undocked<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PenDockListener, PenUndockedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Undocked)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveUndocked<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUndocked)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetDefault() -> ::windows_core::Result<PenDockListener> {
        Self::IPenDockListenerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PenDockListener>(result__)
        })
    }
    pub fn IPenDockListenerStatics<R, F: FnOnce(&IPenDockListenerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PenDockListener, IPenDockListenerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PenDockListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PenDockListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PenDockListener {}
impl ::core::fmt::Debug for PenDockListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenDockListener").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PenDockListener {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.PenDockListener;{759f4d90-1dc0-55cb-ad18-b9101456f592})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PenDockListener {
    type Vtable = IPenDockListener_Vtbl;
    const IID: ::windows_core::GUID = <IPenDockListener as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PenDockListener {
    const NAME: &'static str = "Windows.Devices.Input.PenDockListener";
}
impl ::core::convert::From<PenDockListener> for ::windows_core::IUnknown {
    fn from(value: PenDockListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PenDockListener> for ::windows_core::IUnknown {
    fn from(value: &PenDockListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PenDockListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PenDockListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PenDockListener> for ::windows_core::IInspectable {
    fn from(value: PenDockListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PenDockListener> for ::windows_core::IInspectable {
    fn from(value: &PenDockListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PenDockListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PenDockListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PenDockListener {}
unsafe impl ::core::marker::Sync for PenDockListener {}
#[repr(transparent)]
pub struct PenDockedEventArgs(::windows_core::IUnknown);
impl PenDockedEventArgs {}
impl ::core::clone::Clone for PenDockedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PenDockedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PenDockedEventArgs {}
impl ::core::fmt::Debug for PenDockedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenDockedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PenDockedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.PenDockedEventArgs;{fd4277c6-ca63-5d4e-9ed3-a28a54521c8c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PenDockedEventArgs {
    type Vtable = IPenDockedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPenDockedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PenDockedEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.PenDockedEventArgs";
}
impl ::core::convert::From<PenDockedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PenDockedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PenDockedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PenDockedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PenDockedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PenDockedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PenDockedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PenDockedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PenDockedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PenDockedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PenDockedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PenDockedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PenDockedEventArgs {}
unsafe impl ::core::marker::Sync for PenDockedEventArgs {}
#[repr(transparent)]
pub struct PenTailButtonClickedEventArgs(::windows_core::IUnknown);
impl PenTailButtonClickedEventArgs {}
impl ::core::clone::Clone for PenTailButtonClickedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PenTailButtonClickedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PenTailButtonClickedEventArgs {}
impl ::core::fmt::Debug for PenTailButtonClickedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenTailButtonClickedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PenTailButtonClickedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.PenTailButtonClickedEventArgs;{5d2fb7b6-6ad3-5d3e-ab29-05ea2410e390})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PenTailButtonClickedEventArgs {
    type Vtable = IPenTailButtonClickedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPenTailButtonClickedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PenTailButtonClickedEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.PenTailButtonClickedEventArgs";
}
impl ::core::convert::From<PenTailButtonClickedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PenTailButtonClickedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PenTailButtonClickedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PenTailButtonClickedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PenTailButtonClickedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PenTailButtonClickedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PenTailButtonClickedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PenTailButtonClickedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PenTailButtonClickedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PenTailButtonClickedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PenTailButtonClickedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PenTailButtonClickedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PenTailButtonClickedEventArgs {}
unsafe impl ::core::marker::Sync for PenTailButtonClickedEventArgs {}
#[repr(transparent)]
pub struct PenTailButtonDoubleClickedEventArgs(::windows_core::IUnknown);
impl PenTailButtonDoubleClickedEventArgs {}
impl ::core::clone::Clone for PenTailButtonDoubleClickedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PenTailButtonDoubleClickedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PenTailButtonDoubleClickedEventArgs {}
impl ::core::fmt::Debug for PenTailButtonDoubleClickedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenTailButtonDoubleClickedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PenTailButtonDoubleClickedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.PenTailButtonDoubleClickedEventArgs;{846321a2-618a-5478-b04c-b358231da4a7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PenTailButtonDoubleClickedEventArgs {
    type Vtable = IPenTailButtonDoubleClickedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPenTailButtonDoubleClickedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PenTailButtonDoubleClickedEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.PenTailButtonDoubleClickedEventArgs";
}
impl ::core::convert::From<PenTailButtonDoubleClickedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PenTailButtonDoubleClickedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PenTailButtonDoubleClickedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PenTailButtonDoubleClickedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PenTailButtonDoubleClickedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PenTailButtonDoubleClickedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PenTailButtonDoubleClickedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PenTailButtonDoubleClickedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PenTailButtonDoubleClickedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PenTailButtonDoubleClickedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PenTailButtonDoubleClickedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PenTailButtonDoubleClickedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PenTailButtonDoubleClickedEventArgs {}
unsafe impl ::core::marker::Sync for PenTailButtonDoubleClickedEventArgs {}
#[repr(transparent)]
pub struct PenTailButtonLongPressedEventArgs(::windows_core::IUnknown);
impl PenTailButtonLongPressedEventArgs {}
impl ::core::clone::Clone for PenTailButtonLongPressedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PenTailButtonLongPressedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PenTailButtonLongPressedEventArgs {}
impl ::core::fmt::Debug for PenTailButtonLongPressedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenTailButtonLongPressedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PenTailButtonLongPressedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.PenTailButtonLongPressedEventArgs;{f37c606e-c60a-5f42-b818-a53112406c13})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PenTailButtonLongPressedEventArgs {
    type Vtable = IPenTailButtonLongPressedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPenTailButtonLongPressedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PenTailButtonLongPressedEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.PenTailButtonLongPressedEventArgs";
}
impl ::core::convert::From<PenTailButtonLongPressedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PenTailButtonLongPressedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PenTailButtonLongPressedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PenTailButtonLongPressedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PenTailButtonLongPressedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PenTailButtonLongPressedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PenTailButtonLongPressedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PenTailButtonLongPressedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PenTailButtonLongPressedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PenTailButtonLongPressedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PenTailButtonLongPressedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PenTailButtonLongPressedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PenTailButtonLongPressedEventArgs {}
unsafe impl ::core::marker::Sync for PenTailButtonLongPressedEventArgs {}
#[repr(transparent)]
pub struct PenUndockedEventArgs(::windows_core::IUnknown);
impl PenUndockedEventArgs {}
impl ::core::clone::Clone for PenUndockedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PenUndockedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PenUndockedEventArgs {}
impl ::core::fmt::Debug for PenUndockedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenUndockedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PenUndockedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.PenUndockedEventArgs;{ccd09150-261b-59e6-a5d4-c1964cd03feb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PenUndockedEventArgs {
    type Vtable = IPenUndockedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPenUndockedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PenUndockedEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.PenUndockedEventArgs";
}
impl ::core::convert::From<PenUndockedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PenUndockedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PenUndockedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PenUndockedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PenUndockedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PenUndockedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PenUndockedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PenUndockedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PenUndockedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PenUndockedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PenUndockedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PenUndockedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PenUndockedEventArgs {}
unsafe impl ::core::marker::Sync for PenUndockedEventArgs {}
#[repr(transparent)]
pub struct PointerDevice(::windows_core::IUnknown);
impl PointerDevice {
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PointerDeviceType>::zeroed();
            (::windows_core::Interface::vtable(this).PointerDeviceType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PointerDeviceType>(result__)
        }
    }
    pub fn IsIntegrated(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsIntegrated)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn MaxContacts(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxContacts)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn PhysicalDeviceRect(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).PhysicalDeviceRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    pub fn ScreenRect(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).ScreenRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedUsages(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<PointerDeviceUsage>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedUsages)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<PointerDeviceUsage>>(result__)
        }
    }
    pub fn MaxPointersWithZDistance(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IPointerDevice2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxPointersWithZDistance)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn GetPointerDevice(pointerid: u32) -> ::windows_core::Result<PointerDevice> {
        Self::IPointerDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPointerDevice)(::windows_core::Interface::as_raw(this), pointerid, result__.as_mut_ptr()).from_abi::<PointerDevice>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPointerDevices() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<PointerDevice>> {
        Self::IPointerDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPointerDevices)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<PointerDevice>>(result__)
        })
    }
    pub fn IPointerDeviceStatics<R, F: FnOnce(&IPointerDeviceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PointerDevice, IPointerDeviceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PointerDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PointerDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PointerDevice {}
impl ::core::fmt::Debug for PointerDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PointerDevice {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.PointerDevice;{93c9bafc-ebcb-467e-82c6-276feae36b5a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PointerDevice {
    type Vtable = IPointerDevice_Vtbl;
    const IID: ::windows_core::GUID = <IPointerDevice as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PointerDevice {
    const NAME: &'static str = "Windows.Devices.Input.PointerDevice";
}
impl ::core::convert::From<PointerDevice> for ::windows_core::IUnknown {
    fn from(value: PointerDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerDevice> for ::windows_core::IUnknown {
    fn from(value: &PointerDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PointerDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PointerDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointerDevice> for ::windows_core::IInspectable {
    fn from(value: PointerDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerDevice> for ::windows_core::IInspectable {
    fn from(value: &PointerDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PointerDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PointerDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PointerDeviceType(pub i32);
impl PointerDeviceType {
    pub const Touch: Self = Self(0i32);
    pub const Pen: Self = Self(1i32);
    pub const Mouse: Self = Self(2i32);
}
impl ::core::marker::Copy for PointerDeviceType {}
impl ::core::clone::Clone for PointerDeviceType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PointerDeviceType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PointerDeviceType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PointerDeviceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerDeviceType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PointerDeviceType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Input.PointerDeviceType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
pub struct PointerDeviceUsage {
    pub UsagePage: u32,
    pub Usage: u32,
    pub MinLogical: i32,
    pub MaxLogical: i32,
    pub MinPhysical: i32,
    pub MaxPhysical: i32,
    pub Unit: u32,
    pub PhysicalMultiplier: f32,
}
impl ::core::marker::Copy for PointerDeviceUsage {}
impl ::core::clone::Clone for PointerDeviceUsage {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PointerDeviceUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PointerDeviceUsage").field("UsagePage", &self.UsagePage).field("Usage", &self.Usage).field("MinLogical", &self.MinLogical).field("MaxLogical", &self.MaxLogical).field("MinPhysical", &self.MinPhysical).field("MaxPhysical", &self.MaxPhysical).field("Unit", &self.Unit).field("PhysicalMultiplier", &self.PhysicalMultiplier).finish()
    }
}
unsafe impl ::windows_core::Abi for PointerDeviceUsage {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for PointerDeviceUsage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Devices.Input.PointerDeviceUsage;u4;u4;i4;i4;i4;i4;u4;f4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for PointerDeviceUsage {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PointerDeviceUsage>()) == 0 }
    }
}
impl ::core::cmp::Eq for PointerDeviceUsage {}
impl ::core::default::Default for PointerDeviceUsage {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct TouchCapabilities(::windows_core::IUnknown);
impl TouchCapabilities {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TouchCapabilities, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TouchPresent(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).TouchPresent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Contacts(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Contacts)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for TouchCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TouchCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TouchCapabilities {}
impl ::core::fmt::Debug for TouchCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TouchCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TouchCapabilities {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.TouchCapabilities;{20dd55f9-13f1-46c8-9285-2c05fa3eda6f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TouchCapabilities {
    type Vtable = ITouchCapabilities_Vtbl;
    const IID: ::windows_core::GUID = <ITouchCapabilities as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TouchCapabilities {
    const NAME: &'static str = "Windows.Devices.Input.TouchCapabilities";
}
impl ::core::convert::From<TouchCapabilities> for ::windows_core::IUnknown {
    fn from(value: TouchCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TouchCapabilities> for ::windows_core::IUnknown {
    fn from(value: &TouchCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TouchCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TouchCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TouchCapabilities> for ::windows_core::IInspectable {
    fn from(value: TouchCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TouchCapabilities> for ::windows_core::IInspectable {
    fn from(value: &TouchCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TouchCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TouchCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TouchCapabilities {}
unsafe impl ::core::marker::Sync for TouchCapabilities {}
