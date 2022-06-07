#[cfg(feature = "Gaming_Input_Custom")]
pub mod Custom;
#[cfg(feature = "Gaming_Input_ForceFeedback")]
pub mod ForceFeedback;
#[cfg(feature = "Gaming_Input_Preview")]
pub mod Preview;
pub type ArcadeStick = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Gaming_Input\"`*"]
#[repr(transparent)]
pub struct ArcadeStickButtons(pub u32);
impl ArcadeStickButtons {
    pub const None: Self = Self(0u32);
    pub const StickUp: Self = Self(1u32);
    pub const StickDown: Self = Self(2u32);
    pub const StickLeft: Self = Self(4u32);
    pub const StickRight: Self = Self(8u32);
    pub const Action1: Self = Self(16u32);
    pub const Action2: Self = Self(32u32);
    pub const Action3: Self = Self(64u32);
    pub const Action4: Self = Self(128u32);
    pub const Action5: Self = Self(256u32);
    pub const Action6: Self = Self(512u32);
    pub const Special1: Self = Self(1024u32);
    pub const Special2: Self = Self(2048u32);
}
impl ::core::marker::Copy for ArcadeStickButtons {}
impl ::core::clone::Clone for ArcadeStickButtons {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Gaming_Input\"`*"]
pub struct ArcadeStickReading {
    pub Timestamp: u64,
    pub Buttons: ArcadeStickButtons,
}
impl ::core::marker::Copy for ArcadeStickReading {}
impl ::core::clone::Clone for ArcadeStickReading {
    fn clone(&self) -> Self {
        *self
    }
}
pub type FlightStick = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Gaming_Input\"`*"]
#[repr(transparent)]
pub struct FlightStickButtons(pub u32);
impl FlightStickButtons {
    pub const None: Self = Self(0u32);
    pub const FirePrimary: Self = Self(1u32);
    pub const FireSecondary: Self = Self(2u32);
}
impl ::core::marker::Copy for FlightStickButtons {}
impl ::core::clone::Clone for FlightStickButtons {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Gaming_Input\"`*"]
pub struct FlightStickReading {
    pub Timestamp: u64,
    pub Buttons: FlightStickButtons,
    pub HatSwitch: GameControllerSwitchPosition,
    pub Roll: f64,
    pub Pitch: f64,
    pub Yaw: f64,
    pub Throttle: f64,
}
impl ::core::marker::Copy for FlightStickReading {}
impl ::core::clone::Clone for FlightStickReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Gaming_Input\"`*"]
#[repr(transparent)]
pub struct GameControllerButtonLabel(pub i32);
impl GameControllerButtonLabel {
    pub const None: Self = Self(0i32);
    pub const XboxBack: Self = Self(1i32);
    pub const XboxStart: Self = Self(2i32);
    pub const XboxMenu: Self = Self(3i32);
    pub const XboxView: Self = Self(4i32);
    pub const XboxUp: Self = Self(5i32);
    pub const XboxDown: Self = Self(6i32);
    pub const XboxLeft: Self = Self(7i32);
    pub const XboxRight: Self = Self(8i32);
    pub const XboxA: Self = Self(9i32);
    pub const XboxB: Self = Self(10i32);
    pub const XboxX: Self = Self(11i32);
    pub const XboxY: Self = Self(12i32);
    pub const XboxLeftBumper: Self = Self(13i32);
    pub const XboxLeftTrigger: Self = Self(14i32);
    pub const XboxLeftStickButton: Self = Self(15i32);
    pub const XboxRightBumper: Self = Self(16i32);
    pub const XboxRightTrigger: Self = Self(17i32);
    pub const XboxRightStickButton: Self = Self(18i32);
    pub const XboxPaddle1: Self = Self(19i32);
    pub const XboxPaddle2: Self = Self(20i32);
    pub const XboxPaddle3: Self = Self(21i32);
    pub const XboxPaddle4: Self = Self(22i32);
    pub const Mode: Self = Self(23i32);
    pub const Select: Self = Self(24i32);
    pub const Menu: Self = Self(25i32);
    pub const View: Self = Self(26i32);
    pub const Back: Self = Self(27i32);
    pub const Start: Self = Self(28i32);
    pub const Options: Self = Self(29i32);
    pub const Share: Self = Self(30i32);
    pub const Up: Self = Self(31i32);
    pub const Down: Self = Self(32i32);
    pub const Left: Self = Self(33i32);
    pub const Right: Self = Self(34i32);
    pub const LetterA: Self = Self(35i32);
    pub const LetterB: Self = Self(36i32);
    pub const LetterC: Self = Self(37i32);
    pub const LetterL: Self = Self(38i32);
    pub const LetterR: Self = Self(39i32);
    pub const LetterX: Self = Self(40i32);
    pub const LetterY: Self = Self(41i32);
    pub const LetterZ: Self = Self(42i32);
    pub const Cross: Self = Self(43i32);
    pub const Circle: Self = Self(44i32);
    pub const Square: Self = Self(45i32);
    pub const Triangle: Self = Self(46i32);
    pub const LeftBumper: Self = Self(47i32);
    pub const LeftTrigger: Self = Self(48i32);
    pub const LeftStickButton: Self = Self(49i32);
    pub const Left1: Self = Self(50i32);
    pub const Left2: Self = Self(51i32);
    pub const Left3: Self = Self(52i32);
    pub const RightBumper: Self = Self(53i32);
    pub const RightTrigger: Self = Self(54i32);
    pub const RightStickButton: Self = Self(55i32);
    pub const Right1: Self = Self(56i32);
    pub const Right2: Self = Self(57i32);
    pub const Right3: Self = Self(58i32);
    pub const Paddle1: Self = Self(59i32);
    pub const Paddle2: Self = Self(60i32);
    pub const Paddle3: Self = Self(61i32);
    pub const Paddle4: Self = Self(62i32);
    pub const Plus: Self = Self(63i32);
    pub const Minus: Self = Self(64i32);
    pub const DownLeftArrow: Self = Self(65i32);
    pub const DialLeft: Self = Self(66i32);
    pub const DialRight: Self = Self(67i32);
    pub const Suspension: Self = Self(68i32);
}
impl ::core::marker::Copy for GameControllerButtonLabel {}
impl ::core::clone::Clone for GameControllerButtonLabel {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Gaming_Input\"`*"]
#[repr(transparent)]
pub struct GameControllerSwitchKind(pub i32);
impl GameControllerSwitchKind {
    pub const TwoWay: Self = Self(0i32);
    pub const FourWay: Self = Self(1i32);
    pub const EightWay: Self = Self(2i32);
}
impl ::core::marker::Copy for GameControllerSwitchKind {}
impl ::core::clone::Clone for GameControllerSwitchKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Gaming_Input\"`*"]
#[repr(transparent)]
pub struct GameControllerSwitchPosition(pub i32);
impl GameControllerSwitchPosition {
    pub const Center: Self = Self(0i32);
    pub const Up: Self = Self(1i32);
    pub const UpRight: Self = Self(2i32);
    pub const Right: Self = Self(3i32);
    pub const DownRight: Self = Self(4i32);
    pub const Down: Self = Self(5i32);
    pub const DownLeft: Self = Self(6i32);
    pub const Left: Self = Self(7i32);
    pub const UpLeft: Self = Self(8i32);
}
impl ::core::marker::Copy for GameControllerSwitchPosition {}
impl ::core::clone::Clone for GameControllerSwitchPosition {
    fn clone(&self) -> Self {
        *self
    }
}
pub type Gamepad = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Gaming_Input\"`*"]
#[repr(transparent)]
pub struct GamepadButtons(pub u32);
impl GamepadButtons {
    pub const None: Self = Self(0u32);
    pub const Menu: Self = Self(1u32);
    pub const View: Self = Self(2u32);
    pub const A: Self = Self(4u32);
    pub const B: Self = Self(8u32);
    pub const X: Self = Self(16u32);
    pub const Y: Self = Self(32u32);
    pub const DPadUp: Self = Self(64u32);
    pub const DPadDown: Self = Self(128u32);
    pub const DPadLeft: Self = Self(256u32);
    pub const DPadRight: Self = Self(512u32);
    pub const LeftShoulder: Self = Self(1024u32);
    pub const RightShoulder: Self = Self(2048u32);
    pub const LeftThumbstick: Self = Self(4096u32);
    pub const RightThumbstick: Self = Self(8192u32);
    pub const Paddle1: Self = Self(16384u32);
    pub const Paddle2: Self = Self(32768u32);
    pub const Paddle3: Self = Self(65536u32);
    pub const Paddle4: Self = Self(131072u32);
}
impl ::core::marker::Copy for GamepadButtons {}
impl ::core::clone::Clone for GamepadButtons {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Gaming_Input\"`*"]
pub struct GamepadReading {
    pub Timestamp: u64,
    pub Buttons: GamepadButtons,
    pub LeftTrigger: f64,
    pub RightTrigger: f64,
    pub LeftThumbstickX: f64,
    pub LeftThumbstickY: f64,
    pub RightThumbstickX: f64,
    pub RightThumbstickY: f64,
}
impl ::core::marker::Copy for GamepadReading {}
impl ::core::clone::Clone for GamepadReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Gaming_Input\"`*"]
pub struct GamepadVibration {
    pub LeftMotor: f64,
    pub RightMotor: f64,
    pub LeftTrigger: f64,
    pub RightTrigger: f64,
}
impl ::core::marker::Copy for GamepadVibration {}
impl ::core::clone::Clone for GamepadVibration {
    fn clone(&self) -> Self {
        *self
    }
}
pub type Headset = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IArcadeStick {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetButtonLabel: unsafe extern "system" fn(this: *mut *mut Self, button: ArcadeStickButtons, result__: *mut GameControllerButtonLabel) -> ::windows_sys::core::HRESULT,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ArcadeStickReading) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IArcadeStick {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2974438301, data2: 48891, data3: 19585, data4: [128, 81, 21, 236, 243, 177, 48, 54] };
}
#[repr(C)]
pub struct IArcadeStickStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ArcadeStickAdded: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ArcadeStickAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveArcadeStickAdded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveArcadeStickAdded: usize,
    #[cfg(feature = "Foundation")]
    pub ArcadeStickRemoved: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ArcadeStickRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveArcadeStickRemoved: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveArcadeStickRemoved: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ArcadeSticks: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ArcadeSticks: usize,
}
impl ::windows_sys::core::Interface for IArcadeStickStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1547155656, data2: 14257, data3: 19160, data4: [148, 88, 32, 15, 26, 48, 1, 142] };
}
#[repr(C)]
pub struct IArcadeStickStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub FromGameController: unsafe extern "system" fn(this: *mut *mut Self, gamecontroller: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IArcadeStickStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1387648836, data2: 48006, data3: 17498, data4: [181, 156, 89, 111, 14, 42, 73, 223] };
}
#[repr(C)]
pub struct IFlightStick {
    pub base__: ::windows_sys::core::IInspectable,
    pub HatSwitchKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GameControllerSwitchKind) -> ::windows_sys::core::HRESULT,
    pub GetButtonLabel: unsafe extern "system" fn(this: *mut *mut Self, button: FlightStickButtons, result__: *mut GameControllerButtonLabel) -> ::windows_sys::core::HRESULT,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FlightStickReading) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFlightStick {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3030564892, data2: 47163, data3: 17497, data4: [161, 169, 151, 176, 60, 51, 218, 124] };
}
#[repr(C)]
pub struct IFlightStickStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FlightStickAdded: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FlightStickAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFlightStickAdded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFlightStickAdded: usize,
    #[cfg(feature = "Foundation")]
    pub FlightStickRemoved: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FlightStickRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFlightStickRemoved: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFlightStickRemoved: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FlightSticks: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FlightSticks: usize,
    pub FromGameController: unsafe extern "system" fn(this: *mut *mut Self, gamecontroller: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFlightStickStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1427411530, data2: 65228, data3: 17246, data4: [131, 220, 92, 236, 138, 24, 165, 32] };
}
#[repr(C)]
pub struct IGameController {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub HeadsetConnected: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HeadsetConnected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHeadsetConnected: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHeadsetConnected: usize,
    #[cfg(feature = "Foundation")]
    pub HeadsetDisconnected: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HeadsetDisconnected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHeadsetDisconnected: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHeadsetDisconnected: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub UserChanged: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    UserChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUserChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUserChanged: usize,
    pub Headset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsWireless: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
impl ::windows_sys::core::Interface for IGameController {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 464479522, data2: 24420, data3: 17093, data4: [130, 103, 185, 254, 34, 21, 191, 189] };
}
#[repr(C)]
pub struct IGameControllerBatteryInfo {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Power")]
    pub TryGetBatteryReport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Power"))]
    TryGetBatteryReport: usize,
}
impl ::windows_sys::core::Interface for IGameControllerBatteryInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3706504833, data2: 14691, data3: 19878, data4: [149, 93, 85, 63, 59, 111, 97, 97] };
}
#[repr(C)]
pub struct IGamepad {
    pub base__: ::windows_sys::core::IInspectable,
    pub Vibration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GamepadVibration) -> ::windows_sys::core::HRESULT,
    pub SetVibration: unsafe extern "system" fn(this: *mut *mut Self, value: GamepadVibration) -> ::windows_sys::core::HRESULT,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GamepadReading) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGamepad {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3162223676, data2: 2665, data3: 14595, data4: [158, 157, 165, 15, 134, 164, 93, 229] };
}
#[repr(C)]
pub struct IGamepad2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetButtonLabel: unsafe extern "system" fn(this: *mut *mut Self, button: GamepadButtons, result__: *mut GameControllerButtonLabel) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGamepad2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1008110013, data2: 22805, data3: 16965, data4: [176, 192, 200, 159, 174, 3, 8, 255] };
}
#[repr(C)]
pub struct IGamepadStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GamepadAdded: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GamepadAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGamepadAdded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGamepadAdded: usize,
    #[cfg(feature = "Foundation")]
    pub GamepadRemoved: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GamepadRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGamepadRemoved: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGamepadRemoved: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gamepads: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gamepads: usize,
}
impl ::windows_sys::core::Interface for IGamepadStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2344412457, data2: 54428, data3: 14825, data4: [149, 96, 228, 125, 222, 150, 183, 200] };
}
#[repr(C)]
pub struct IGamepadStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub FromGameController: unsafe extern "system" fn(this: *mut *mut Self, gamecontroller: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGamepadStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1114074565, data2: 2134, data3: 18372, data4: [146, 19, 179, 149, 80, 76, 58, 60] };
}
#[repr(C)]
pub struct IHeadset {
    pub base__: ::windows_sys::core::IInspectable,
    pub CaptureDeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RenderDeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHeadset {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1070683887, data2: 26917, data3: 16296, data4: [145, 129, 2, 156, 82, 35, 174, 59] };
}
#[repr(C)]
pub struct IRacingWheel {
    pub base__: ::windows_sys::core::IInspectable,
    pub HasClutch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub HasHandbrake: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub HasPatternShifter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub MaxPatternShifterGear: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MaxWheelAngle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Gaming_Input_ForceFeedback")]
    pub WheelMotor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Gaming_Input_ForceFeedback"))]
    WheelMotor: usize,
    pub GetButtonLabel: unsafe extern "system" fn(this: *mut *mut Self, button: RacingWheelButtons, result__: *mut GameControllerButtonLabel) -> ::windows_sys::core::HRESULT,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RacingWheelReading) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRacingWheel {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4115031407, data2: 57606, data3: 19586, data4: [169, 15, 85, 64, 18, 144, 75, 133] };
}
#[repr(C)]
pub struct IRacingWheelStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RacingWheelAdded: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RacingWheelAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRacingWheelAdded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRacingWheelAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RacingWheelRemoved: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RacingWheelRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRacingWheelRemoved: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRacingWheelRemoved: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RacingWheels: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RacingWheels: usize,
}
impl ::windows_sys::core::Interface for IRacingWheelStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 985738453, data2: 22555, data3: 18742, data4: [159, 148, 105, 241, 230, 81, 76, 125] };
}
#[repr(C)]
pub struct IRacingWheelStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub FromGameController: unsafe extern "system" fn(this: *mut *mut Self, gamecontroller: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRacingWheelStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3865492650, data2: 60925, data3: 17187, data4: [169, 246, 60, 56, 64, 72, 209, 237] };
}
#[repr(C)]
pub struct IRawGameController {
    pub base__: ::windows_sys::core::IInspectable,
    pub AxisCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ButtonCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Gaming_Input_ForceFeedback"))]
    pub ForceFeedbackMotors: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Gaming_Input_ForceFeedback")))]
    ForceFeedbackMotors: usize,
    pub HardwareProductId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub HardwareVendorId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub SwitchCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetButtonLabel: unsafe extern "system" fn(this: *mut *mut Self, buttonindex: i32, result__: *mut GameControllerButtonLabel) -> ::windows_sys::core::HRESULT,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut *mut Self, buttonArray_array_size: u32, buttonarray: *mut bool, switchArray_array_size: u32, switcharray: *mut GameControllerSwitchPosition, axisArray_array_size: u32, axisarray: *mut f64, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub GetSwitchKind: unsafe extern "system" fn(this: *mut *mut Self, switchindex: i32, result__: *mut GameControllerSwitchKind) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRawGameController {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2091740561, data2: 42977, data3: 20337, data4: [154, 120, 51, 233, 197, 223, 234, 98] };
}
#[repr(C)]
pub struct IRawGameController2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Devices_Haptics", feature = "Foundation_Collections"))]
    pub SimpleHapticsControllers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Haptics", feature = "Foundation_Collections")))]
    SimpleHapticsControllers: usize,
    pub NonRoamableId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRawGameController2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1136705589, data2: 47987, data3: 18262, data4: [167, 135, 62, 214, 190, 166, 23, 189] };
}
#[repr(C)]
pub struct IRawGameControllerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RawGameControllerAdded: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RawGameControllerAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRawGameControllerAdded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRawGameControllerAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RawGameControllerRemoved: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RawGameControllerRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRawGameControllerRemoved: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRawGameControllerRemoved: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RawGameControllers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RawGameControllers: usize,
    pub FromGameController: unsafe extern "system" fn(this: *mut *mut Self, gamecontroller: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRawGameControllerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3951888274, data2: 59738, data3: 19225, data4: [175, 199, 10, 89, 248, 191, 117, 158] };
}
#[repr(C)]
pub struct IUINavigationController {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UINavigationReading) -> ::windows_sys::core::HRESULT,
    pub GetOptionalButtonLabel: unsafe extern "system" fn(this: *mut *mut Self, button: OptionalUINavigationButtons, result__: *mut GameControllerButtonLabel) -> ::windows_sys::core::HRESULT,
    pub GetRequiredButtonLabel: unsafe extern "system" fn(this: *mut *mut Self, button: RequiredUINavigationButtons, result__: *mut GameControllerButtonLabel) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUINavigationController {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3853447133, data2: 62734, data3: 19029, data4: [140, 220, 211, 50, 41, 84, 129, 117] };
}
#[repr(C)]
pub struct IUINavigationControllerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub UINavigationControllerAdded: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UINavigationControllerAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUINavigationControllerAdded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUINavigationControllerAdded: usize,
    #[cfg(feature = "Foundation")]
    pub UINavigationControllerRemoved: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UINavigationControllerRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUINavigationControllerRemoved: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUINavigationControllerRemoved: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UINavigationControllers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UINavigationControllers: usize,
}
impl ::windows_sys::core::Interface for IUINavigationControllerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 789877514, data2: 63224, data3: 19016, data4: [141, 137, 148, 120, 108, 202, 12, 46] };
}
#[repr(C)]
pub struct IUINavigationControllerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub FromGameController: unsafe extern "system" fn(this: *mut *mut Self, gamecontroller: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUINavigationControllerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3771410659, data2: 45579, data3: 19211, data4: [158, 212, 243, 213, 60, 236, 13, 228] };
}
#[doc = "*Required features: `\"Gaming_Input\"`*"]
#[repr(transparent)]
pub struct OptionalUINavigationButtons(pub u32);
impl OptionalUINavigationButtons {
    pub const None: Self = Self(0u32);
    pub const Context1: Self = Self(1u32);
    pub const Context2: Self = Self(2u32);
    pub const Context3: Self = Self(4u32);
    pub const Context4: Self = Self(8u32);
    pub const PageUp: Self = Self(16u32);
    pub const PageDown: Self = Self(32u32);
    pub const PageLeft: Self = Self(64u32);
    pub const PageRight: Self = Self(128u32);
    pub const ScrollUp: Self = Self(256u32);
    pub const ScrollDown: Self = Self(512u32);
    pub const ScrollLeft: Self = Self(1024u32);
    pub const ScrollRight: Self = Self(2048u32);
}
impl ::core::marker::Copy for OptionalUINavigationButtons {}
impl ::core::clone::Clone for OptionalUINavigationButtons {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RacingWheel = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Gaming_Input\"`*"]
#[repr(transparent)]
pub struct RacingWheelButtons(pub u32);
impl RacingWheelButtons {
    pub const None: Self = Self(0u32);
    pub const PreviousGear: Self = Self(1u32);
    pub const NextGear: Self = Self(2u32);
    pub const DPadUp: Self = Self(4u32);
    pub const DPadDown: Self = Self(8u32);
    pub const DPadLeft: Self = Self(16u32);
    pub const DPadRight: Self = Self(32u32);
    pub const Button1: Self = Self(64u32);
    pub const Button2: Self = Self(128u32);
    pub const Button3: Self = Self(256u32);
    pub const Button4: Self = Self(512u32);
    pub const Button5: Self = Self(1024u32);
    pub const Button6: Self = Self(2048u32);
    pub const Button7: Self = Self(4096u32);
    pub const Button8: Self = Self(8192u32);
    pub const Button9: Self = Self(16384u32);
    pub const Button10: Self = Self(32768u32);
    pub const Button11: Self = Self(65536u32);
    pub const Button12: Self = Self(131072u32);
    pub const Button13: Self = Self(262144u32);
    pub const Button14: Self = Self(524288u32);
    pub const Button15: Self = Self(1048576u32);
    pub const Button16: Self = Self(2097152u32);
}
impl ::core::marker::Copy for RacingWheelButtons {}
impl ::core::clone::Clone for RacingWheelButtons {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Gaming_Input\"`*"]
pub struct RacingWheelReading {
    pub Timestamp: u64,
    pub Buttons: RacingWheelButtons,
    pub PatternShifterGear: i32,
    pub Wheel: f64,
    pub Throttle: f64,
    pub Brake: f64,
    pub Clutch: f64,
    pub Handbrake: f64,
}
impl ::core::marker::Copy for RacingWheelReading {}
impl ::core::clone::Clone for RacingWheelReading {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RawGameController = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Gaming_Input\"`*"]
#[repr(transparent)]
pub struct RequiredUINavigationButtons(pub u32);
impl RequiredUINavigationButtons {
    pub const None: Self = Self(0u32);
    pub const Menu: Self = Self(1u32);
    pub const View: Self = Self(2u32);
    pub const Accept: Self = Self(4u32);
    pub const Cancel: Self = Self(8u32);
    pub const Up: Self = Self(16u32);
    pub const Down: Self = Self(32u32);
    pub const Left: Self = Self(64u32);
    pub const Right: Self = Self(128u32);
}
impl ::core::marker::Copy for RequiredUINavigationButtons {}
impl ::core::clone::Clone for RequiredUINavigationButtons {
    fn clone(&self) -> Self {
        *self
    }
}
pub type UINavigationController = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"Gaming_Input\"`*"]
pub struct UINavigationReading {
    pub Timestamp: u64,
    pub RequiredButtons: RequiredUINavigationButtons,
    pub OptionalButtons: OptionalUINavigationButtons,
}
impl ::core::marker::Copy for UINavigationReading {}
impl ::core::clone::Clone for UINavigationReading {
    fn clone(&self) -> Self {
        *self
    }
}
