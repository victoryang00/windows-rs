#[doc = "*Required features: `\"Gaming_UI\"`*"]
#[repr(transparent)]
pub struct GameChatMessageOrigin(pub i32);
impl GameChatMessageOrigin {
    pub const Voice: Self = Self(0i32);
    pub const Text: Self = Self(1i32);
}
impl ::core::marker::Copy for GameChatMessageOrigin {}
impl ::core::clone::Clone for GameChatMessageOrigin {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GameChatMessageReceivedEventArgs = *mut ::core::ffi::c_void;
pub type GameChatOverlay = *mut ::core::ffi::c_void;
pub type GameChatOverlayMessageSource = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Gaming_UI\"`*"]
#[repr(transparent)]
pub struct GameChatOverlayPosition(pub i32);
impl GameChatOverlayPosition {
    pub const BottomCenter: Self = Self(0i32);
    pub const BottomLeft: Self = Self(1i32);
    pub const BottomRight: Self = Self(2i32);
    pub const MiddleRight: Self = Self(3i32);
    pub const MiddleLeft: Self = Self(4i32);
    pub const TopCenter: Self = Self(5i32);
    pub const TopLeft: Self = Self(6i32);
    pub const TopRight: Self = Self(7i32);
}
impl ::core::marker::Copy for GameChatOverlayPosition {}
impl ::core::clone::Clone for GameChatOverlayPosition {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GameUIProviderActivatedEventArgs = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IGameBarStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub VisibilityChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VisibilityChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVisibilityChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVisibilityChanged: usize,
    #[cfg(feature = "Foundation")]
    pub IsInputRedirectedChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsInputRedirectedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsInputRedirectedChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsInputRedirectedChanged: usize,
    pub Visible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsInputRedirected: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGameBarStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 498705042, data2: 52344, data3: 16755, data4: [190, 69, 182, 30, 103, 40, 62, 167] };
}
#[repr(C)]
pub struct IGameChatMessageReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AppDisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SenderName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Origin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GameChatMessageOrigin) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGameChatMessageReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2726429169, data2: 16313, data3: 20034, data4: [164, 3, 122, 252, 226, 2, 59, 30] };
}
#[repr(C)]
pub struct IGameChatOverlay {
    pub base__: ::windows_sys::core::IInspectable,
    pub DesiredPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GameChatOverlayPosition) -> ::windows_sys::core::HRESULT,
    pub SetDesiredPosition: unsafe extern "system" fn(this: *mut *mut Self, value: GameChatOverlayPosition) -> ::windows_sys::core::HRESULT,
    pub AddMessage: unsafe extern "system" fn(this: *mut *mut Self, sender: ::windows_sys::core::HSTRING, message: ::windows_sys::core::HSTRING, origin: GameChatMessageOrigin) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGameChatOverlay {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4224075877, data2: 63228, data3: 19016, data4: [174, 7, 3, 172, 110, 212, 55, 4] };
}
#[repr(C)]
pub struct IGameChatOverlayMessageSource {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub MessageReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MessageReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMessageReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMessageReceived: usize,
    #[cfg(feature = "Foundation")]
    pub SetDelayBeforeClosingAfterMessageReceived: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDelayBeforeClosingAfterMessageReceived: usize,
}
impl ::windows_sys::core::Interface for IGameChatOverlayMessageSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 504853399, data2: 23035, data3: 20303, data4: [142, 154, 128, 172, 248, 23, 116, 60] };
}
#[repr(C)]
pub struct IGameChatOverlayStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGameChatOverlayStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2309813780, data2: 30823, data3: 18935, data4: [150, 135, 37, 217, 219, 244, 68, 209] };
}
#[repr(C)]
pub struct IGameUIProviderActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GameUIArgs: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GameUIArgs: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ReportCompleted: unsafe extern "system" fn(this: *mut *mut Self, results: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReportCompleted: usize,
}
impl ::windows_sys::core::Interface for IGameUIProviderActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2813534270, data2: 51959, data3: 19949, data4: [187, 210, 71, 222, 67, 187, 109, 213] };
}
