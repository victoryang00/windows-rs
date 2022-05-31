#[doc(hidden)]
#[repr(transparent)]
pub struct IInjectedInputGamepadInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInjectedInputGamepadInfo {
    type Vtable = IInjectedInputGamepadInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x20ae9a3f_df11_4572_a9ab_d75b8a5e48ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInjectedInputGamepadInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-gaming")]
    pub Buttons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_gaming::Input::GamepadButtons) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-gaming"))]
    Buttons: usize,
    #[cfg(feature = "winrt-gaming")]
    pub SetButtons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_gaming::Input::GamepadButtons) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-gaming"))]
    SetButtons: usize,
    pub LeftThumbstickX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetLeftThumbstickX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub LeftThumbstickY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetLeftThumbstickY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub LeftTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetLeftTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub RightThumbstickX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetRightThumbstickX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub RightThumbstickY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetRightThumbstickY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub RightTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetRightTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInjectedInputGamepadInfoFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInjectedInputGamepadInfoFactory {
    type Vtable = IInjectedInputGamepadInfoFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x59596876_6c39_4ec4_8b2a_29ef7de18aca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInjectedInputGamepadInfoFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-gaming")]
    pub CreateInstanceFromGamepadReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reading: ::winrt_gaming::Input::GamepadReading, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-gaming"))]
    CreateInstanceFromGamepadReading: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInjectedInputKeyboardInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInjectedInputKeyboardInfo {
    type Vtable = IInjectedInputKeyboardInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4b46d140_2b6a_5ffa_7eae_bd077b052acd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInjectedInputKeyboardInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub KeyOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputKeyOptions) -> ::windows_core::HRESULT,
    pub SetKeyOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InjectedInputKeyOptions) -> ::windows_core::HRESULT,
    pub ScanCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub SetScanCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u16) -> ::windows_core::HRESULT,
    pub VirtualKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub SetVirtualKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u16) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInjectedInputMouseInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInjectedInputMouseInfo {
    type Vtable = IInjectedInputMouseInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96f56e6b_e47a_5cf4_418d_8a5fb9670c7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInjectedInputMouseInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MouseOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputMouseOptions) -> ::windows_core::HRESULT,
    pub SetMouseOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InjectedInputMouseOptions) -> ::windows_core::HRESULT,
    pub MouseData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetMouseData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub DeltaY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetDeltaY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub DeltaX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetDeltaX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub TimeOffsetInMilliseconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetTimeOffsetInMilliseconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInjectedInputPenInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInjectedInputPenInfo {
    type Vtable = IInjectedInputPenInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6b40ad03_ca1e_5527_7e02_2828540bb1d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInjectedInputPenInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PointerInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputPointerInfo) -> ::windows_core::HRESULT,
    pub SetPointerInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InjectedInputPointerInfo) -> ::windows_core::HRESULT,
    pub PenButtons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputPenButtons) -> ::windows_core::HRESULT,
    pub SetPenButtons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InjectedInputPenButtons) -> ::windows_core::HRESULT,
    pub PenParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputPenParameters) -> ::windows_core::HRESULT,
    pub SetPenParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InjectedInputPenParameters) -> ::windows_core::HRESULT,
    pub Pressure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetPressure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub Rotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub TiltX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetTiltX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub TiltY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetTiltY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInjectedInputTouchInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInjectedInputTouchInfo {
    type Vtable = IInjectedInputTouchInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x224fd1df_43e8_5ef5_510a_69ca8c9b4c28);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInjectedInputTouchInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputRectangle) -> ::windows_core::HRESULT,
    pub SetContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InjectedInputRectangle) -> ::windows_core::HRESULT,
    pub Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub PointerInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputPointerInfo) -> ::windows_core::HRESULT,
    pub SetPointerInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InjectedInputPointerInfo) -> ::windows_core::HRESULT,
    pub Pressure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetPressure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub TouchParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputTouchParameters) -> ::windows_core::HRESULT,
    pub SetTouchParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InjectedInputTouchParameters) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputInjector(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputInjector {
    type Vtable = IInputInjector_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8ec26f84_0b02_4bd2_ad7a_3d4658be3e18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputInjector_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub InjectKeyboardInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    InjectKeyboardInput: usize,
    #[cfg(feature = "winrt-foundation")]
    pub InjectMouseInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    InjectMouseInput: usize,
    pub InitializeTouchInjection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visualmode: InjectedInputVisualizationMode) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub InjectTouchInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    InjectTouchInput: usize,
    pub UninitializeTouchInjection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InitializePenInjection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visualmode: InjectedInputVisualizationMode) -> ::windows_core::HRESULT,
    pub InjectPenInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UninitializePenInjection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InjectShortcut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shortcut: InjectedInputShortcut) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputInjector2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputInjector2 {
    type Vtable = IInputInjector2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8e7a905d_1453_43a7_9bcb_06d6d7b305f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputInjector2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InitializeGamepadInjection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InjectGamepadInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UninitializeGamepadInjection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputInjectorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputInjectorStatics {
    type Vtable = IInputInjectorStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdeae6943_7402_4141_a5c6_0c01aa57b16a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputInjectorStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TryCreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputInjectorStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputInjectorStatics2 {
    type Vtable = IInputInjectorStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa4db38fb_dd8c_414f_95ea_f87ef4c0ae6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputInjectorStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TryCreateForAppBroadcastOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InjectedInputButtonChangeKind(pub i32);
impl InjectedInputButtonChangeKind {
    pub const None: Self = Self(0i32);
    pub const FirstButtonDown: Self = Self(1i32);
    pub const FirstButtonUp: Self = Self(2i32);
    pub const SecondButtonDown: Self = Self(3i32);
    pub const SecondButtonUp: Self = Self(4i32);
    pub const ThirdButtonDown: Self = Self(5i32);
    pub const ThirdButtonUp: Self = Self(6i32);
    pub const FourthButtonDown: Self = Self(7i32);
    pub const FourthButtonUp: Self = Self(8i32);
    pub const FifthButtonDown: Self = Self(9i32);
    pub const FifthButtonUp: Self = Self(10i32);
}
impl ::core::marker::Copy for InjectedInputButtonChangeKind {}
impl ::core::clone::Clone for InjectedInputButtonChangeKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InjectedInputButtonChangeKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InjectedInputButtonChangeKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for InjectedInputButtonChangeKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InjectedInputButtonChangeKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InjectedInputButtonChangeKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputButtonChangeKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct InjectedInputGamepadInfo(::windows_core::IUnknown);
impl InjectedInputGamepadInfo {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<InjectedInputGamepadInfo, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-gaming")]
    pub fn Buttons(&self) -> ::windows_core::Result<::winrt_gaming::Input::GamepadButtons> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_gaming::Input::GamepadButtons>::zeroed();
            (::windows_core::Interface::vtable(this).Buttons)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_gaming::Input::GamepadButtons>(result__)
        }
    }
    #[cfg(feature = "winrt-gaming")]
    pub fn SetButtons(&self, value: ::winrt_gaming::Input::GamepadButtons) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetButtons)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LeftThumbstickX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).LeftThumbstickX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetLeftThumbstickX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLeftThumbstickX)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LeftThumbstickY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).LeftThumbstickY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetLeftThumbstickY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLeftThumbstickY)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LeftTrigger(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).LeftTrigger)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetLeftTrigger(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLeftTrigger)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RightThumbstickX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).RightThumbstickX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetRightThumbstickX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRightThumbstickX)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RightThumbstickY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).RightThumbstickY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetRightThumbstickY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRightThumbstickY)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RightTrigger(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).RightTrigger)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetRightTrigger(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRightTrigger)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-gaming")]
    pub fn CreateInstanceFromGamepadReading<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_gaming::Input::GamepadReading>>(reading: Param0) -> ::windows_core::Result<InjectedInputGamepadInfo> {
        Self::IInjectedInputGamepadInfoFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceFromGamepadReading)(::windows_core::Interface::as_raw(this), reading.into_param().abi(), result__.as_mut_ptr()).from_abi::<InjectedInputGamepadInfo>(result__)
        })
    }
    pub fn IInjectedInputGamepadInfoFactory<R, F: FnOnce(&IInjectedInputGamepadInfoFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<InjectedInputGamepadInfo, IInjectedInputGamepadInfoFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for InjectedInputGamepadInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InjectedInputGamepadInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InjectedInputGamepadInfo {}
impl ::core::fmt::Debug for InjectedInputGamepadInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InjectedInputGamepadInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InjectedInputGamepadInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Preview.Injection.InjectedInputGamepadInfo;{20ae9a3f-df11-4572-a9ab-d75b8a5e48ad})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InjectedInputGamepadInfo {
    type Vtable = IInjectedInputGamepadInfo_Vtbl;
    const IID: ::windows_core::GUID = <IInjectedInputGamepadInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InjectedInputGamepadInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.InjectedInputGamepadInfo";
}
impl ::core::convert::From<InjectedInputGamepadInfo> for ::windows_core::IUnknown {
    fn from(value: InjectedInputGamepadInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InjectedInputGamepadInfo> for ::windows_core::IUnknown {
    fn from(value: &InjectedInputGamepadInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InjectedInputGamepadInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InjectedInputGamepadInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InjectedInputGamepadInfo> for ::windows_core::IInspectable {
    fn from(value: InjectedInputGamepadInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InjectedInputGamepadInfo> for ::windows_core::IInspectable {
    fn from(value: &InjectedInputGamepadInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InjectedInputGamepadInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InjectedInputGamepadInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InjectedInputKeyOptions(pub u32);
impl InjectedInputKeyOptions {
    pub const None: Self = Self(0u32);
    pub const ExtendedKey: Self = Self(1u32);
    pub const KeyUp: Self = Self(2u32);
    pub const ScanCode: Self = Self(8u32);
    pub const Unicode: Self = Self(4u32);
}
impl ::core::marker::Copy for InjectedInputKeyOptions {}
impl ::core::clone::Clone for InjectedInputKeyOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InjectedInputKeyOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InjectedInputKeyOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for InjectedInputKeyOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InjectedInputKeyOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for InjectedInputKeyOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for InjectedInputKeyOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for InjectedInputKeyOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for InjectedInputKeyOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for InjectedInputKeyOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for InjectedInputKeyOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputKeyOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct InjectedInputKeyboardInfo(::windows_core::IUnknown);
impl InjectedInputKeyboardInfo {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<InjectedInputKeyboardInfo, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn KeyOptions(&self) -> ::windows_core::Result<InjectedInputKeyOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InjectedInputKeyOptions>::zeroed();
            (::windows_core::Interface::vtable(this).KeyOptions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InjectedInputKeyOptions>(result__)
        }
    }
    pub fn SetKeyOptions(&self, value: InjectedInputKeyOptions) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetKeyOptions)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScanCode(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).ScanCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn SetScanCode(&self, value: u16) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScanCode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VirtualKey(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).VirtualKey)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn SetVirtualKey(&self, value: u16) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVirtualKey)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for InjectedInputKeyboardInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InjectedInputKeyboardInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InjectedInputKeyboardInfo {}
impl ::core::fmt::Debug for InjectedInputKeyboardInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InjectedInputKeyboardInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InjectedInputKeyboardInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Preview.Injection.InjectedInputKeyboardInfo;{4b46d140-2b6a-5ffa-7eae-bd077b052acd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InjectedInputKeyboardInfo {
    type Vtable = IInjectedInputKeyboardInfo_Vtbl;
    const IID: ::windows_core::GUID = <IInjectedInputKeyboardInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InjectedInputKeyboardInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.InjectedInputKeyboardInfo";
}
impl ::core::convert::From<InjectedInputKeyboardInfo> for ::windows_core::IUnknown {
    fn from(value: InjectedInputKeyboardInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InjectedInputKeyboardInfo> for ::windows_core::IUnknown {
    fn from(value: &InjectedInputKeyboardInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InjectedInputKeyboardInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InjectedInputKeyboardInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InjectedInputKeyboardInfo> for ::windows_core::IInspectable {
    fn from(value: InjectedInputKeyboardInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InjectedInputKeyboardInfo> for ::windows_core::IInspectable {
    fn from(value: &InjectedInputKeyboardInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InjectedInputKeyboardInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InjectedInputKeyboardInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct InjectedInputMouseInfo(::windows_core::IUnknown);
impl InjectedInputMouseInfo {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<InjectedInputMouseInfo, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn MouseOptions(&self) -> ::windows_core::Result<InjectedInputMouseOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InjectedInputMouseOptions>::zeroed();
            (::windows_core::Interface::vtable(this).MouseOptions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InjectedInputMouseOptions>(result__)
        }
    }
    pub fn SetMouseOptions(&self, value: InjectedInputMouseOptions) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMouseOptions)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MouseData(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MouseData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetMouseData(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMouseData)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DeltaY(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).DeltaY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetDeltaY(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDeltaY)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DeltaX(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).DeltaX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetDeltaX(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDeltaX)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TimeOffsetInMilliseconds(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).TimeOffsetInMilliseconds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetTimeOffsetInMilliseconds(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTimeOffsetInMilliseconds)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for InjectedInputMouseInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InjectedInputMouseInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InjectedInputMouseInfo {}
impl ::core::fmt::Debug for InjectedInputMouseInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InjectedInputMouseInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InjectedInputMouseInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Preview.Injection.InjectedInputMouseInfo;{96f56e6b-e47a-5cf4-418d-8a5fb9670c7d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InjectedInputMouseInfo {
    type Vtable = IInjectedInputMouseInfo_Vtbl;
    const IID: ::windows_core::GUID = <IInjectedInputMouseInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InjectedInputMouseInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.InjectedInputMouseInfo";
}
impl ::core::convert::From<InjectedInputMouseInfo> for ::windows_core::IUnknown {
    fn from(value: InjectedInputMouseInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InjectedInputMouseInfo> for ::windows_core::IUnknown {
    fn from(value: &InjectedInputMouseInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InjectedInputMouseInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InjectedInputMouseInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InjectedInputMouseInfo> for ::windows_core::IInspectable {
    fn from(value: InjectedInputMouseInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InjectedInputMouseInfo> for ::windows_core::IInspectable {
    fn from(value: &InjectedInputMouseInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InjectedInputMouseInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InjectedInputMouseInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InjectedInputMouseOptions(pub u32);
impl InjectedInputMouseOptions {
    pub const None: Self = Self(0u32);
    pub const Move: Self = Self(1u32);
    pub const LeftDown: Self = Self(2u32);
    pub const LeftUp: Self = Self(4u32);
    pub const RightDown: Self = Self(8u32);
    pub const RightUp: Self = Self(16u32);
    pub const MiddleDown: Self = Self(32u32);
    pub const MiddleUp: Self = Self(64u32);
    pub const XDown: Self = Self(128u32);
    pub const XUp: Self = Self(256u32);
    pub const Wheel: Self = Self(2048u32);
    pub const HWheel: Self = Self(4096u32);
    pub const MoveNoCoalesce: Self = Self(8192u32);
    pub const VirtualDesk: Self = Self(16384u32);
    pub const Absolute: Self = Self(32768u32);
}
impl ::core::marker::Copy for InjectedInputMouseOptions {}
impl ::core::clone::Clone for InjectedInputMouseOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InjectedInputMouseOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InjectedInputMouseOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for InjectedInputMouseOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InjectedInputMouseOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for InjectedInputMouseOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for InjectedInputMouseOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for InjectedInputMouseOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for InjectedInputMouseOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for InjectedInputMouseOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for InjectedInputMouseOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputMouseOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InjectedInputPenButtons(pub u32);
impl InjectedInputPenButtons {
    pub const None: Self = Self(0u32);
    pub const Barrel: Self = Self(1u32);
    pub const Inverted: Self = Self(2u32);
    pub const Eraser: Self = Self(4u32);
}
impl ::core::marker::Copy for InjectedInputPenButtons {}
impl ::core::clone::Clone for InjectedInputPenButtons {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InjectedInputPenButtons {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InjectedInputPenButtons {
    type Abi = Self;
}
impl ::core::fmt::Debug for InjectedInputPenButtons {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InjectedInputPenButtons").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for InjectedInputPenButtons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for InjectedInputPenButtons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for InjectedInputPenButtons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for InjectedInputPenButtons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for InjectedInputPenButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for InjectedInputPenButtons {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputPenButtons;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct InjectedInputPenInfo(::windows_core::IUnknown);
impl InjectedInputPenInfo {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<InjectedInputPenInfo, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn PointerInfo(&self) -> ::windows_core::Result<InjectedInputPointerInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InjectedInputPointerInfo>::zeroed();
            (::windows_core::Interface::vtable(this).PointerInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InjectedInputPointerInfo>(result__)
        }
    }
    pub fn SetPointerInfo<'a, Param0: ::windows_core::IntoParam<'a, InjectedInputPointerInfo>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPointerInfo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn PenButtons(&self) -> ::windows_core::Result<InjectedInputPenButtons> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InjectedInputPenButtons>::zeroed();
            (::windows_core::Interface::vtable(this).PenButtons)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InjectedInputPenButtons>(result__)
        }
    }
    pub fn SetPenButtons(&self, value: InjectedInputPenButtons) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPenButtons)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PenParameters(&self) -> ::windows_core::Result<InjectedInputPenParameters> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InjectedInputPenParameters>::zeroed();
            (::windows_core::Interface::vtable(this).PenParameters)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InjectedInputPenParameters>(result__)
        }
    }
    pub fn SetPenParameters(&self, value: InjectedInputPenParameters) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPenParameters)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Pressure(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Pressure)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetPressure(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPressure)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Rotation(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Rotation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetRotation(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRotation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TiltX(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).TiltX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetTiltX(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTiltX)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TiltY(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).TiltY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetTiltY(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTiltY)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for InjectedInputPenInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InjectedInputPenInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InjectedInputPenInfo {}
impl ::core::fmt::Debug for InjectedInputPenInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InjectedInputPenInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InjectedInputPenInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Preview.Injection.InjectedInputPenInfo;{6b40ad03-ca1e-5527-7e02-2828540bb1d4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InjectedInputPenInfo {
    type Vtable = IInjectedInputPenInfo_Vtbl;
    const IID: ::windows_core::GUID = <IInjectedInputPenInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InjectedInputPenInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.InjectedInputPenInfo";
}
impl ::core::convert::From<InjectedInputPenInfo> for ::windows_core::IUnknown {
    fn from(value: InjectedInputPenInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InjectedInputPenInfo> for ::windows_core::IUnknown {
    fn from(value: &InjectedInputPenInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InjectedInputPenInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InjectedInputPenInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InjectedInputPenInfo> for ::windows_core::IInspectable {
    fn from(value: InjectedInputPenInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InjectedInputPenInfo> for ::windows_core::IInspectable {
    fn from(value: &InjectedInputPenInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InjectedInputPenInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InjectedInputPenInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InjectedInputPenParameters(pub u32);
impl InjectedInputPenParameters {
    pub const None: Self = Self(0u32);
    pub const Pressure: Self = Self(1u32);
    pub const Rotation: Self = Self(2u32);
    pub const TiltX: Self = Self(4u32);
    pub const TiltY: Self = Self(8u32);
}
impl ::core::marker::Copy for InjectedInputPenParameters {}
impl ::core::clone::Clone for InjectedInputPenParameters {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InjectedInputPenParameters {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InjectedInputPenParameters {
    type Abi = Self;
}
impl ::core::fmt::Debug for InjectedInputPenParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InjectedInputPenParameters").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for InjectedInputPenParameters {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for InjectedInputPenParameters {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for InjectedInputPenParameters {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for InjectedInputPenParameters {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for InjectedInputPenParameters {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for InjectedInputPenParameters {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputPenParameters;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
pub struct InjectedInputPoint {
    pub PositionX: i32,
    pub PositionY: i32,
}
impl ::core::marker::Copy for InjectedInputPoint {}
impl ::core::clone::Clone for InjectedInputPoint {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for InjectedInputPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("InjectedInputPoint").field("PositionX", &self.PositionX).field("PositionY", &self.PositionY).finish()
    }
}
unsafe impl ::windows_core::Abi for InjectedInputPoint {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for InjectedInputPoint {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.UI.Input.Preview.Injection.InjectedInputPoint;i4;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for InjectedInputPoint {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<InjectedInputPoint>()) == 0 }
    }
}
impl ::core::cmp::Eq for InjectedInputPoint {}
impl ::core::default::Default for InjectedInputPoint {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct InjectedInputPointerInfo {
    pub PointerId: u32,
    pub PointerOptions: InjectedInputPointerOptions,
    pub PixelLocation: InjectedInputPoint,
    pub TimeOffsetInMilliseconds: u32,
    pub PerformanceCount: u64,
}
impl ::core::marker::Copy for InjectedInputPointerInfo {}
impl ::core::clone::Clone for InjectedInputPointerInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for InjectedInputPointerInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("InjectedInputPointerInfo").field("PointerId", &self.PointerId).field("PointerOptions", &self.PointerOptions).field("PixelLocation", &self.PixelLocation).field("TimeOffsetInMilliseconds", &self.TimeOffsetInMilliseconds).field("PerformanceCount", &self.PerformanceCount).finish()
    }
}
unsafe impl ::windows_core::Abi for InjectedInputPointerInfo {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for InjectedInputPointerInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.UI.Input.Preview.Injection.InjectedInputPointerInfo;u4;enum(Windows.UI.Input.Preview.Injection.InjectedInputPointerOptions;u4);struct(Windows.UI.Input.Preview.Injection.InjectedInputPoint;i4;i4);u4;u8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for InjectedInputPointerInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<InjectedInputPointerInfo>()) == 0 }
    }
}
impl ::core::cmp::Eq for InjectedInputPointerInfo {}
impl ::core::default::Default for InjectedInputPointerInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InjectedInputPointerOptions(pub u32);
impl InjectedInputPointerOptions {
    pub const None: Self = Self(0u32);
    pub const New: Self = Self(1u32);
    pub const InRange: Self = Self(2u32);
    pub const InContact: Self = Self(4u32);
    pub const FirstButton: Self = Self(16u32);
    pub const SecondButton: Self = Self(32u32);
    pub const Primary: Self = Self(8192u32);
    pub const Confidence: Self = Self(16384u32);
    pub const Canceled: Self = Self(32768u32);
    pub const PointerDown: Self = Self(65536u32);
    pub const Update: Self = Self(131072u32);
    pub const PointerUp: Self = Self(262144u32);
    pub const CaptureChanged: Self = Self(2097152u32);
}
impl ::core::marker::Copy for InjectedInputPointerOptions {}
impl ::core::clone::Clone for InjectedInputPointerOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InjectedInputPointerOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InjectedInputPointerOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for InjectedInputPointerOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InjectedInputPointerOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for InjectedInputPointerOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for InjectedInputPointerOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for InjectedInputPointerOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for InjectedInputPointerOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for InjectedInputPointerOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for InjectedInputPointerOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputPointerOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
pub struct InjectedInputRectangle {
    pub Left: i32,
    pub Top: i32,
    pub Bottom: i32,
    pub Right: i32,
}
impl ::core::marker::Copy for InjectedInputRectangle {}
impl ::core::clone::Clone for InjectedInputRectangle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for InjectedInputRectangle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("InjectedInputRectangle").field("Left", &self.Left).field("Top", &self.Top).field("Bottom", &self.Bottom).field("Right", &self.Right).finish()
    }
}
unsafe impl ::windows_core::Abi for InjectedInputRectangle {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for InjectedInputRectangle {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.UI.Input.Preview.Injection.InjectedInputRectangle;i4;i4;i4;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for InjectedInputRectangle {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<InjectedInputRectangle>()) == 0 }
    }
}
impl ::core::cmp::Eq for InjectedInputRectangle {}
impl ::core::default::Default for InjectedInputRectangle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InjectedInputShortcut(pub i32);
impl InjectedInputShortcut {
    pub const Back: Self = Self(0i32);
    pub const Start: Self = Self(1i32);
    pub const Search: Self = Self(2i32);
}
impl ::core::marker::Copy for InjectedInputShortcut {}
impl ::core::clone::Clone for InjectedInputShortcut {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InjectedInputShortcut {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InjectedInputShortcut {
    type Abi = Self;
}
impl ::core::fmt::Debug for InjectedInputShortcut {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InjectedInputShortcut").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InjectedInputShortcut {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputShortcut;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct InjectedInputTouchInfo(::windows_core::IUnknown);
impl InjectedInputTouchInfo {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<InjectedInputTouchInfo, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Contact(&self) -> ::windows_core::Result<InjectedInputRectangle> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InjectedInputRectangle>::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InjectedInputRectangle>(result__)
        }
    }
    pub fn SetContact<'a, Param0: ::windows_core::IntoParam<'a, InjectedInputRectangle>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContact)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Orientation(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Orientation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetOrientation(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOrientation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PointerInfo(&self) -> ::windows_core::Result<InjectedInputPointerInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InjectedInputPointerInfo>::zeroed();
            (::windows_core::Interface::vtable(this).PointerInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InjectedInputPointerInfo>(result__)
        }
    }
    pub fn SetPointerInfo<'a, Param0: ::windows_core::IntoParam<'a, InjectedInputPointerInfo>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPointerInfo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Pressure(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Pressure)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetPressure(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPressure)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TouchParameters(&self) -> ::windows_core::Result<InjectedInputTouchParameters> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InjectedInputTouchParameters>::zeroed();
            (::windows_core::Interface::vtable(this).TouchParameters)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InjectedInputTouchParameters>(result__)
        }
    }
    pub fn SetTouchParameters(&self, value: InjectedInputTouchParameters) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTouchParameters)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for InjectedInputTouchInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InjectedInputTouchInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InjectedInputTouchInfo {}
impl ::core::fmt::Debug for InjectedInputTouchInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InjectedInputTouchInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InjectedInputTouchInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Preview.Injection.InjectedInputTouchInfo;{224fd1df-43e8-5ef5-510a-69ca8c9b4c28})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InjectedInputTouchInfo {
    type Vtable = IInjectedInputTouchInfo_Vtbl;
    const IID: ::windows_core::GUID = <IInjectedInputTouchInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InjectedInputTouchInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.InjectedInputTouchInfo";
}
impl ::core::convert::From<InjectedInputTouchInfo> for ::windows_core::IUnknown {
    fn from(value: InjectedInputTouchInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InjectedInputTouchInfo> for ::windows_core::IUnknown {
    fn from(value: &InjectedInputTouchInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InjectedInputTouchInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InjectedInputTouchInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InjectedInputTouchInfo> for ::windows_core::IInspectable {
    fn from(value: InjectedInputTouchInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InjectedInputTouchInfo> for ::windows_core::IInspectable {
    fn from(value: &InjectedInputTouchInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InjectedInputTouchInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InjectedInputTouchInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InjectedInputTouchParameters(pub u32);
impl InjectedInputTouchParameters {
    pub const None: Self = Self(0u32);
    pub const Contact: Self = Self(1u32);
    pub const Orientation: Self = Self(2u32);
    pub const Pressure: Self = Self(4u32);
}
impl ::core::marker::Copy for InjectedInputTouchParameters {}
impl ::core::clone::Clone for InjectedInputTouchParameters {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InjectedInputTouchParameters {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InjectedInputTouchParameters {
    type Abi = Self;
}
impl ::core::fmt::Debug for InjectedInputTouchParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InjectedInputTouchParameters").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for InjectedInputTouchParameters {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for InjectedInputTouchParameters {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for InjectedInputTouchParameters {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for InjectedInputTouchParameters {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for InjectedInputTouchParameters {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for InjectedInputTouchParameters {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputTouchParameters;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InjectedInputVisualizationMode(pub i32);
impl InjectedInputVisualizationMode {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
    pub const Indirect: Self = Self(2i32);
}
impl ::core::marker::Copy for InjectedInputVisualizationMode {}
impl ::core::clone::Clone for InjectedInputVisualizationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InjectedInputVisualizationMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InjectedInputVisualizationMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for InjectedInputVisualizationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InjectedInputVisualizationMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InjectedInputVisualizationMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputVisualizationMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct InputInjector(::windows_core::IUnknown);
impl InputInjector {
    #[cfg(feature = "winrt-foundation")]
    pub fn InjectKeyboardInput<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<InjectedInputKeyboardInfo>>>(&self, input: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InjectKeyboardInput)(::windows_core::Interface::as_raw(this), input.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn InjectMouseInput<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<InjectedInputMouseInfo>>>(&self, input: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InjectMouseInput)(::windows_core::Interface::as_raw(this), input.into_param().abi()).ok() }
    }
    pub fn InitializeTouchInjection(&self, visualmode: InjectedInputVisualizationMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InitializeTouchInjection)(::windows_core::Interface::as_raw(this), visualmode).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn InjectTouchInput<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<InjectedInputTouchInfo>>>(&self, input: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InjectTouchInput)(::windows_core::Interface::as_raw(this), input.into_param().abi()).ok() }
    }
    pub fn UninitializeTouchInjection(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).UninitializeTouchInjection)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn InitializePenInjection(&self, visualmode: InjectedInputVisualizationMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InitializePenInjection)(::windows_core::Interface::as_raw(this), visualmode).ok() }
    }
    pub fn InjectPenInput<'a, Param0: ::windows_core::IntoParam<'a, InjectedInputPenInfo>>(&self, input: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InjectPenInput)(::windows_core::Interface::as_raw(this), input.into_param().abi()).ok() }
    }
    pub fn UninitializePenInjection(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).UninitializePenInjection)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn InjectShortcut(&self, shortcut: InjectedInputShortcut) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InjectShortcut)(::windows_core::Interface::as_raw(this), shortcut).ok() }
    }
    pub fn InitializeGamepadInjection(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IInputInjector2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).InitializeGamepadInjection)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn InjectGamepadInput<'a, Param0: ::windows_core::IntoParam<'a, InjectedInputGamepadInfo>>(&self, input: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IInputInjector2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).InjectGamepadInput)(::windows_core::Interface::as_raw(this), input.into_param().abi()).ok() }
    }
    pub fn UninitializeGamepadInjection(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IInputInjector2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).UninitializeGamepadInjection)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn TryCreate() -> ::windows_core::Result<InputInjector> {
        Self::IInputInjectorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryCreate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InputInjector>(result__)
        })
    }
    pub fn TryCreateForAppBroadcastOnly() -> ::windows_core::Result<InputInjector> {
        Self::IInputInjectorStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateForAppBroadcastOnly)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InputInjector>(result__)
        })
    }
    pub fn IInputInjectorStatics<R, F: FnOnce(&IInputInjectorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<InputInjector, IInputInjectorStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IInputInjectorStatics2<R, F: FnOnce(&IInputInjectorStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<InputInjector, IInputInjectorStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for InputInjector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InputInjector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InputInjector {}
impl ::core::fmt::Debug for InputInjector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputInjector").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InputInjector {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Preview.Injection.InputInjector;{8ec26f84-0b02-4bd2-ad7a-3d4658be3e18})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InputInjector {
    type Vtable = IInputInjector_Vtbl;
    const IID: ::windows_core::GUID = <IInputInjector as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InputInjector {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.InputInjector";
}
impl ::core::convert::From<InputInjector> for ::windows_core::IUnknown {
    fn from(value: InputInjector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputInjector> for ::windows_core::IUnknown {
    fn from(value: &InputInjector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InputInjector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InputInjector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InputInjector> for ::windows_core::IInspectable {
    fn from(value: InputInjector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputInjector> for ::windows_core::IInspectable {
    fn from(value: &InputInjector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InputInjector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InputInjector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
