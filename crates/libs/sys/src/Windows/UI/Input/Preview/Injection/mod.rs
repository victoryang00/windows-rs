#[repr(C)]
pub struct IInjectedInputGamepadInfo {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Gaming_Input")]
    pub Buttons: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Gaming::Input::GamepadButtons) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Gaming_Input"))]
    Buttons: usize,
    #[cfg(feature = "Gaming_Input")]
    pub SetButtons: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::super::Gaming::Input::GamepadButtons) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Gaming_Input"))]
    SetButtons: usize,
    pub LeftThumbstickX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetLeftThumbstickX: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub LeftThumbstickY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetLeftThumbstickY: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub LeftTrigger: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetLeftTrigger: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub RightThumbstickX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetRightThumbstickX: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub RightThumbstickY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetRightThumbstickY: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub RightTrigger: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetRightTrigger: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInjectedInputGamepadInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 548313663, data2: 57105, data3: 17778, data4: [169, 171, 215, 91, 138, 94, 72, 173] };
}
#[repr(C)]
pub struct IInjectedInputGamepadInfoFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Gaming_Input")]
    pub CreateInstanceFromGamepadReading: unsafe extern "system" fn(this: *mut *mut Self, reading: super::super::super::super::Gaming::Input::GamepadReading, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Gaming_Input"))]
    CreateInstanceFromGamepadReading: usize,
}
impl ::windows_sys::core::Interface for IInjectedInputGamepadInfoFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1499031670, data2: 27705, data3: 20164, data4: [139, 42, 41, 239, 125, 225, 138, 202] };
}
#[repr(C)]
pub struct IInjectedInputKeyboardInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub KeyOptions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InjectedInputKeyOptions) -> ::windows_sys::core::HRESULT,
    pub SetKeyOptions: unsafe extern "system" fn(this: *mut *mut Self, value: InjectedInputKeyOptions) -> ::windows_sys::core::HRESULT,
    pub ScanCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub SetScanCode: unsafe extern "system" fn(this: *mut *mut Self, value: u16) -> ::windows_sys::core::HRESULT,
    pub VirtualKey: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub SetVirtualKey: unsafe extern "system" fn(this: *mut *mut Self, value: u16) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInjectedInputKeyboardInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1262932288, data2: 11114, data3: 24570, data4: [126, 174, 189, 7, 123, 5, 42, 205] };
}
#[repr(C)]
pub struct IInjectedInputMouseInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub MouseOptions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InjectedInputMouseOptions) -> ::windows_sys::core::HRESULT,
    pub SetMouseOptions: unsafe extern "system" fn(this: *mut *mut Self, value: InjectedInputMouseOptions) -> ::windows_sys::core::HRESULT,
    pub MouseData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMouseData: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub DeltaY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetDeltaY: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub DeltaX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetDeltaX: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub TimeOffsetInMilliseconds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetTimeOffsetInMilliseconds: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInjectedInputMouseInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2532666987, data2: 58490, data3: 23796, data4: [65, 141, 138, 95, 185, 103, 12, 125] };
}
#[repr(C)]
pub struct IInjectedInputPenInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub PointerInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InjectedInputPointerInfo) -> ::windows_sys::core::HRESULT,
    pub SetPointerInfo: unsafe extern "system" fn(this: *mut *mut Self, value: InjectedInputPointerInfo) -> ::windows_sys::core::HRESULT,
    pub PenButtons: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InjectedInputPenButtons) -> ::windows_sys::core::HRESULT,
    pub SetPenButtons: unsafe extern "system" fn(this: *mut *mut Self, value: InjectedInputPenButtons) -> ::windows_sys::core::HRESULT,
    pub PenParameters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InjectedInputPenParameters) -> ::windows_sys::core::HRESULT,
    pub SetPenParameters: unsafe extern "system" fn(this: *mut *mut Self, value: InjectedInputPenParameters) -> ::windows_sys::core::HRESULT,
    pub Pressure: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetPressure: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Rotation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub TiltX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetTiltX: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub TiltY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetTiltY: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInjectedInputPenInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1799400707, data2: 51742, data3: 21799, data4: [126, 2, 40, 40, 84, 11, 177, 212] };
}
#[repr(C)]
pub struct IInjectedInputTouchInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub Contact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InjectedInputRectangle) -> ::windows_sys::core::HRESULT,
    pub SetContact: unsafe extern "system" fn(this: *mut *mut Self, value: InjectedInputRectangle) -> ::windows_sys::core::HRESULT,
    pub Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub PointerInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InjectedInputPointerInfo) -> ::windows_sys::core::HRESULT,
    pub SetPointerInfo: unsafe extern "system" fn(this: *mut *mut Self, value: InjectedInputPointerInfo) -> ::windows_sys::core::HRESULT,
    pub Pressure: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetPressure: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub TouchParameters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InjectedInputTouchParameters) -> ::windows_sys::core::HRESULT,
    pub SetTouchParameters: unsafe extern "system" fn(this: *mut *mut Self, value: InjectedInputTouchParameters) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInjectedInputTouchInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 575656415, data2: 17384, data3: 24309, data4: [81, 10, 105, 202, 140, 155, 76, 40] };
}
#[repr(C)]
pub struct IInputInjector {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub InjectKeyboardInput: unsafe extern "system" fn(this: *mut *mut Self, input: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InjectKeyboardInput: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InjectMouseInput: unsafe extern "system" fn(this: *mut *mut Self, input: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InjectMouseInput: usize,
    pub InitializeTouchInjection: unsafe extern "system" fn(this: *mut *mut Self, visualmode: InjectedInputVisualizationMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub InjectTouchInput: unsafe extern "system" fn(this: *mut *mut Self, input: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InjectTouchInput: usize,
    pub UninitializeTouchInjection: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub InitializePenInjection: unsafe extern "system" fn(this: *mut *mut Self, visualmode: InjectedInputVisualizationMode) -> ::windows_sys::core::HRESULT,
    pub InjectPenInput: unsafe extern "system" fn(this: *mut *mut Self, input: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UninitializePenInjection: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub InjectShortcut: unsafe extern "system" fn(this: *mut *mut Self, shortcut: InjectedInputShortcut) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInputInjector {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2395107204, data2: 2818, data3: 19410, data4: [173, 122, 61, 70, 88, 190, 62, 24] };
}
#[repr(C)]
pub struct IInputInjector2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub InitializeGamepadInjection: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub InjectGamepadInput: unsafe extern "system" fn(this: *mut *mut Self, input: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UninitializeGamepadInjection: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInputInjector2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2390397021, data2: 5203, data3: 17319, data4: [155, 203, 6, 214, 215, 179, 5, 247] };
}
#[repr(C)]
pub struct IInputInjectorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryCreate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInputInjectorStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3735972163, data2: 29698, data3: 16705, data4: [165, 198, 12, 1, 170, 87, 177, 106] };
}
#[repr(C)]
pub struct IInputInjectorStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryCreateForAppBroadcastOnly: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInputInjectorStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2765830395, data2: 56716, data3: 16719, data4: [149, 234, 248, 126, 244, 192, 174, 108] };
}
#[doc = "*Required features: `\"UI_Input_Preview_Injection\"`*"]
#[repr(transparent)]
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
pub type InjectedInputGamepadInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Input_Preview_Injection\"`*"]
#[repr(transparent)]
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
pub type InjectedInputKeyboardInfo = *mut ::core::ffi::c_void;
pub type InjectedInputMouseInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Input_Preview_Injection\"`*"]
#[repr(transparent)]
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
#[doc = "*Required features: `\"UI_Input_Preview_Injection\"`*"]
#[repr(transparent)]
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
pub type InjectedInputPenInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Input_Preview_Injection\"`*"]
#[repr(transparent)]
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
#[repr(C)]
#[doc = "*Required features: `\"UI_Input_Preview_Injection\"`*"]
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
#[repr(C)]
#[doc = "*Required features: `\"UI_Input_Preview_Injection\"`*"]
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
#[doc = "*Required features: `\"UI_Input_Preview_Injection\"`*"]
#[repr(transparent)]
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
#[repr(C)]
#[doc = "*Required features: `\"UI_Input_Preview_Injection\"`*"]
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
#[doc = "*Required features: `\"UI_Input_Preview_Injection\"`*"]
#[repr(transparent)]
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
pub type InjectedInputTouchInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Input_Preview_Injection\"`*"]
#[repr(transparent)]
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
#[doc = "*Required features: `\"UI_Input_Preview_Injection\"`*"]
#[repr(transparent)]
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
pub type InputInjector = *mut ::core::ffi::c_void;
