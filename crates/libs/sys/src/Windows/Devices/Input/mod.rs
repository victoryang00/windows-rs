#[cfg(feature = "Devices_Input_Preview")]
pub mod Preview;
#[repr(C)]
pub struct IKeyboardCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub KeyboardPresent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IKeyboardCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 977247062, data2: 26520, data3: 19388, data4: [131, 62, 15, 52, 177, 124, 101, 255] };
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
impl ::windows_sys::core::Interface for IMouseCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3164987427, data2: 32217, data3: 19307, data4: [154, 146, 85, 212, 60, 179, 143, 115] };
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
impl ::windows_sys::core::Interface for IMouseDevice {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2297295960, data2: 62152, data3: 18932, data4: [190, 31, 194, 86, 179, 136, 188, 17] };
}
#[repr(C)]
pub struct IMouseDeviceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMouseDeviceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1212846149, data2: 28016, data3: 18907, data4: [142, 104, 70, 255, 189, 23, 211, 141] };
}
#[repr(C)]
pub struct IMouseEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub MouseDelta: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MouseDelta) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMouseEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4129663581, data2: 9044, data3: 19655, data4: [146, 48, 150, 148, 28, 150, 159, 222] };
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
impl ::windows_sys::core::Interface for IPenButtonListener {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2185610102, data2: 7907, data3: 21495, data4: [177, 247, 131, 52, 161, 111, 40, 21] };
}
#[repr(C)]
pub struct IPenButtonListenerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPenButtonListenerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 430482820, data2: 34351, data3: 24425, data4: [191, 234, 5, 246, 88, 79, 19, 63] };
}
#[repr(C)]
pub struct IPenDevice {
    pub base__: ::windows_sys::core::IInspectable,
    pub PenId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPenDevice {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 830828218, data2: 42808, data3: 23180, data4: [184, 246, 249, 126, 246, 141, 24, 239] };
}
#[repr(C)]
pub struct IPenDevice2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
impl ::windows_sys::core::Interface for IPenDevice2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 34067239, data2: 32696, data3: 21862, data4: [140, 52, 248, 52, 32, 55, 183, 249] };
}
#[repr(C)]
pub struct IPenDeviceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetFromPointerId: unsafe extern "system" fn(this: *mut *mut Self, pointerid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPenDeviceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2650521089, data2: 2406, data3: 20864, data4: [188, 180, 184, 80, 96, 227, 148, 121] };
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
impl ::windows_sys::core::Interface for IPenDockListener {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1973374352, data2: 7616, data3: 21963, data4: [173, 24, 185, 16, 20, 86, 245, 146] };
}
#[repr(C)]
pub struct IPenDockListenerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPenDockListenerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3401014938, data2: 22, data3: 23666, data4: [150, 158, 169, 126, 17, 153, 42, 147] };
}
#[repr(C)]
pub struct IPenDockedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IPenDockedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4248991686, data2: 51811, data3: 23886, data4: [158, 211, 162, 138, 84, 82, 28, 140] };
}
#[repr(C)]
pub struct IPenTailButtonClickedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IPenTailButtonClickedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1563408310, data2: 27347, data3: 23870, data4: [171, 41, 5, 234, 36, 16, 227, 144] };
}
#[repr(C)]
pub struct IPenTailButtonDoubleClickedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IPenTailButtonDoubleClickedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2221089186, data2: 24970, data3: 21624, data4: [176, 76, 179, 88, 35, 29, 164, 167] };
}
#[repr(C)]
pub struct IPenTailButtonLongPressedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IPenTailButtonLongPressedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4085014638, data2: 50698, data3: 24386, data4: [184, 24, 165, 49, 18, 64, 108, 19] };
}
#[repr(C)]
pub struct IPenUndockedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IPenUndockedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3436220752, data2: 9755, data3: 23014, data4: [165, 212, 193, 150, 76, 208, 63, 235] };
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
impl ::windows_sys::core::Interface for IPointerDevice {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2479471356, data2: 60363, data3: 18046, data4: [130, 198, 39, 111, 234, 227, 107, 90] };
}
#[repr(C)]
pub struct IPointerDevice2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxPointersWithZDistance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPointerDevice2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4171682464, data2: 50308, data3: 18591, data4: [174, 62, 48, 210, 238, 31, 253, 62] };
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
impl ::windows_sys::core::Interface for IPointerDeviceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3635976865, data2: 53702, data3: 16750, data4: [189, 141, 87, 144, 145, 77, 197, 99] };
}
#[repr(C)]
pub struct ITouchCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub TouchPresent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Contacts: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITouchCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 551376377, data2: 5105, data3: 18120, data4: [146, 133, 44, 5, 250, 62, 218, 111] };
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
