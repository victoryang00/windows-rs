#[cfg(feature = "Devices_Input_Preview")]
pub mod Preview;
#[repr(C)]
pub struct IKeyboardCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub KeyboardPresent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMouseCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub MousePresent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub VerticalWheelPresent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub HorizontalWheelPresent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SwapButtons: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub NumberOfButtons: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMouseDevice {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub MouseMoved: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MouseMoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMouseMoved: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMouseMoved: usize,
}
#[repr(C)]
pub struct IMouseDeviceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMouseEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub MouseDelta: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MouseDelta) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPenButtonListener {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsSupportedChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsSupportedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsSupportedChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsSupportedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub TailButtonClicked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TailButtonClicked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTailButtonClicked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTailButtonClicked: usize,
    #[cfg(feature = "Foundation")]
    pub TailButtonDoubleClicked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TailButtonDoubleClicked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTailButtonDoubleClicked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTailButtonDoubleClicked: usize,
    #[cfg(feature = "Foundation")]
    pub TailButtonLongPressed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TailButtonLongPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTailButtonLongPressed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTailButtonLongPressed: usize,
}
#[repr(C)]
pub struct IPenButtonListenerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPenDevice {
    pub base__: ::windows_sys::core::IInspectable,
    pub PenId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPenDevice2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
#[repr(C)]
pub struct IPenDeviceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetFromPointerId: unsafe extern "system" fn(this: *mut *mut Self, pointerid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPenDockListener {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsSupportedChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsSupportedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsSupportedChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsSupportedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub Docked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Docked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDocked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDocked: usize,
    #[cfg(feature = "Foundation")]
    pub Undocked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Undocked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUndocked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUndocked: usize,
}
#[repr(C)]
pub struct IPenDockListenerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPenDockedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IPenTailButtonClickedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IPenTailButtonDoubleClickedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IPenTailButtonLongPressedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IPenUndockedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IPointerDevice {
    pub base__: ::windows_sys::core::IInspectable,
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PointerDeviceType) -> ::windows_sys::core::HRESULT,
    pub IsIntegrated: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub MaxContacts: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PhysicalDeviceRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PhysicalDeviceRect: usize,
    #[cfg(feature = "Foundation")]
    pub ScreenRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScreenRect: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedUsages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedUsages: usize,
}
#[repr(C)]
pub struct IPointerDevice2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxPointersWithZDistance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPointerDeviceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetPointerDevice: unsafe extern "system" fn(this: *mut *mut Self, pointerid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPointerDevices: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPointerDevices: usize,
}
#[repr(C)]
pub struct ITouchCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub TouchPresent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Contacts: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
pub type KeyboardCapabilities = *mut ::core::ffi::c_void;
pub type MouseCapabilities = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"Devices_Input\"`*"]
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
pub type MouseDevice = *mut ::core::ffi::c_void;
pub type MouseEventArgs = *mut ::core::ffi::c_void;
pub type PenButtonListener = *mut ::core::ffi::c_void;
pub type PenDevice = *mut ::core::ffi::c_void;
pub type PenDockListener = *mut ::core::ffi::c_void;
pub type PenDockedEventArgs = *mut ::core::ffi::c_void;
pub type PenTailButtonClickedEventArgs = *mut ::core::ffi::c_void;
pub type PenTailButtonDoubleClickedEventArgs = *mut ::core::ffi::c_void;
pub type PenTailButtonLongPressedEventArgs = *mut ::core::ffi::c_void;
pub type PenUndockedEventArgs = *mut ::core::ffi::c_void;
pub type PointerDevice = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Input\"`*"]
#[repr(transparent)]
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
#[repr(C)]
#[doc = "*Required features: `\"Devices_Input\"`*"]
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
pub type TouchCapabilities = *mut ::core::ffi::c_void;
