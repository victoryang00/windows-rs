#[doc = "*Required features: `\"Phone_System_UserProfile_GameServices_Core\"`*"]
#[repr(transparent)]
pub struct GameServiceGameOutcome(pub i32);
impl GameServiceGameOutcome {
    pub const None: Self = Self(0i32);
    pub const Win: Self = Self(1i32);
    pub const Loss: Self = Self(2i32);
    pub const Tie: Self = Self(3i32);
}
impl ::core::marker::Copy for GameServiceGameOutcome {}
impl ::core::clone::Clone for GameServiceGameOutcome {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GameServicePropertyCollection = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Phone_System_UserProfile_GameServices_Core\"`*"]
#[repr(transparent)]
pub struct GameServiceScoreKind(pub i32);
impl GameServiceScoreKind {
    pub const Number: Self = Self(0i32);
    pub const Time: Self = Self(1i32);
}
impl ::core::marker::Copy for GameServiceScoreKind {}
impl ::core::clone::Clone for GameServiceScoreKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IGameService {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ServiceUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServiceUri: usize,
    #[cfg(feature = "Foundation")]
    pub GetGamerProfileAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetGamerProfileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetInstalledGameItemsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetInstalledGameItemsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetPartnerTokenAsync: unsafe extern "system" fn(this: *mut *mut Self, audienceuri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPartnerTokenAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetPrivilegesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPrivilegesAsync: usize,
    pub GrantAchievement: unsafe extern "system" fn(this: *mut *mut Self, achievementid: u32) -> ::windows_sys::core::HRESULT,
    pub GrantAvatarAward: unsafe extern "system" fn(this: *mut *mut Self, avatarawardid: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub PostResult: unsafe extern "system" fn(this: *mut *mut Self, gamevariant: u32, scorekind: GameServiceScoreKind, scorevalue: i64, gameoutcome: GameServiceGameOutcome, buffer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PostResult: usize,
}
#[repr(C)]
pub struct IGameService2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub NotifyPartnerTokenExpired: unsafe extern "system" fn(this: *mut *mut Self, audienceuri: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NotifyPartnerTokenExpired: usize,
    pub GetAuthenticationStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGameServicePropertyCollection {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetPropertyAsync: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPropertyAsync: usize,
}
