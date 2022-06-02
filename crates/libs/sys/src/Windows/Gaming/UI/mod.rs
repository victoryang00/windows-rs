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
#[repr(C)]
pub struct IGameChatMessageReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AppDisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SenderName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Origin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GameChatMessageOrigin) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGameChatOverlay {
    pub base__: ::windows_sys::core::IInspectable,
    pub DesiredPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GameChatOverlayPosition) -> ::windows_sys::core::HRESULT,
    pub SetDesiredPosition: unsafe extern "system" fn(this: *mut *mut Self, value: GameChatOverlayPosition) -> ::windows_sys::core::HRESULT,
    pub AddMessage: unsafe extern "system" fn(this: *mut *mut Self, sender: ::windows_sys::core::HSTRING, message: ::windows_sys::core::HSTRING, origin: GameChatMessageOrigin) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IGameChatOverlayStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
