pub struct GameService;
impl GameService {
    pub fn ServiceUri() -> ::windows_core::Result<::winrt_foundation::Uri> {
        Self::IGameService(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        })
    }
    pub fn GetGamerProfileAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<GameServicePropertyCollection>> {
        Self::IGameService(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetGamerProfileAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<GameServicePropertyCollection>>(result__)
        })
    }
    pub fn GetInstalledGameItemsAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<GameServicePropertyCollection>> {
        Self::IGameService(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetInstalledGameItemsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<GameServicePropertyCollection>>(result__)
        })
    }
    pub fn GetPartnerTokenAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(audienceuri: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::IGameService(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPartnerTokenAsync)(::windows_core::Interface::as_raw(this), audienceuri.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        })
    }
    pub fn GetPrivilegesAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::IGameService(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPrivilegesAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        })
    }
    pub fn GrantAchievement(achievementid: u32) -> ::windows_core::Result<()> {
        Self::IGameService(|this| unsafe { (::windows_core::Interface::vtable(this).GrantAchievement)(::windows_core::Interface::as_raw(this), achievementid).ok() })
    }
    pub fn GrantAvatarAward(avatarawardid: u32) -> ::windows_core::Result<()> {
        Self::IGameService(|this| unsafe { (::windows_core::Interface::vtable(this).GrantAvatarAward)(::windows_core::Interface::as_raw(this), avatarawardid).ok() })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn PostResult<'a, Param4: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(gamevariant: u32, scorekind: GameServiceScoreKind, scorevalue: i64, gameoutcome: GameServiceGameOutcome, buffer: Param4) -> ::windows_core::Result<()> {
        Self::IGameService(|this| unsafe { (::windows_core::Interface::vtable(this).PostResult)(::windows_core::Interface::as_raw(this), gamevariant, scorekind, scorevalue, gameoutcome, buffer.into_param().abi()).ok() })
    }
    pub fn NotifyPartnerTokenExpired<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(audienceuri: Param0) -> ::windows_core::Result<()> {
        Self::IGameService2(|this| unsafe { (::windows_core::Interface::vtable(this).NotifyPartnerTokenExpired)(::windows_core::Interface::as_raw(this), audienceuri.into_param().abi()).ok() })
    }
    pub fn GetAuthenticationStatus() -> ::windows_core::Result<u32> {
        Self::IGameService2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetAuthenticationStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn IGameService<R, F: FnOnce(&IGameService) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GameService, IGameService> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGameService2<R, F: FnOnce(&IGameService2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GameService, IGameService2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for GameService {
    const NAME: &'static str = "Windows.Phone.System.UserProfile.GameServices.Core.GameService";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for GameServiceGameOutcome {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GameServiceGameOutcome {
    type Abi = Self;
}
impl ::core::fmt::Debug for GameServiceGameOutcome {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameServiceGameOutcome").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GameServiceGameOutcome {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Phone.System.UserProfile.GameServices.Core.GameServiceGameOutcome;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct GameServicePropertyCollection(::windows_core::IUnknown);
impl GameServicePropertyCollection {
    pub fn GetPropertyAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPropertyAsync)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::IInspectable>>(result__)
        }
    }
}
impl ::core::clone::Clone for GameServicePropertyCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameServicePropertyCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameServicePropertyCollection {}
impl ::core::fmt::Debug for GameServicePropertyCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameServicePropertyCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GameServicePropertyCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Phone.System.UserProfile.GameServices.Core.GameServicePropertyCollection;{07e57fc8-debb-4609-9cc8-529d16bc2bd9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GameServicePropertyCollection {
    type Vtable = IGameServicePropertyCollection_Vtbl;
    const IID: ::windows_core::GUID = <IGameServicePropertyCollection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GameServicePropertyCollection {
    const NAME: &'static str = "Windows.Phone.System.UserProfile.GameServices.Core.GameServicePropertyCollection";
}
impl ::core::convert::From<GameServicePropertyCollection> for ::windows_core::IUnknown {
    fn from(value: GameServicePropertyCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameServicePropertyCollection> for ::windows_core::IUnknown {
    fn from(value: &GameServicePropertyCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GameServicePropertyCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GameServicePropertyCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GameServicePropertyCollection> for ::windows_core::IInspectable {
    fn from(value: GameServicePropertyCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameServicePropertyCollection> for ::windows_core::IInspectable {
    fn from(value: &GameServicePropertyCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GameServicePropertyCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GameServicePropertyCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GameServicePropertyCollection {}
unsafe impl ::core::marker::Sync for GameServicePropertyCollection {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for GameServiceScoreKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GameServiceScoreKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for GameServiceScoreKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameServiceScoreKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GameServiceScoreKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Phone.System.UserProfile.GameServices.Core.GameServiceScoreKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameService(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameService {
    type Vtable = IGameService_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2e2d5098_48a9_4efc_afd6_8e6da09003fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameService_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ServiceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetGamerProfileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetInstalledGameItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetPartnerTokenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audienceuri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetPrivilegesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GrantAchievement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, achievementid: u32) -> ::windows_core::HRESULT,
    pub GrantAvatarAward: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, avatarawardid: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub PostResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gamevariant: u32, scorekind: GameServiceScoreKind, scorevalue: i64, gameoutcome: GameServiceGameOutcome, buffer: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    PostResult: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameService2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameService2 {
    type Vtable = IGameService2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd2364ef6_ea17_4be5_8d8a_c860885e051f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameService2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub NotifyPartnerTokenExpired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audienceuri: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetAuthenticationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameServicePropertyCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameServicePropertyCollection {
    type Vtable = IGameServicePropertyCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x07e57fc8_debb_4609_9cc8_529d16bc2bd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameServicePropertyCollection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetPropertyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
