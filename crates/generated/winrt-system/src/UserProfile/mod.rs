#[cfg(feature = "winrt-")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AccountPictureKind(pub i32);
#[cfg(feature = "winrt-")]
impl AccountPictureKind {
    pub const SmallImage: Self = Self(0i32);
    pub const LargeImage: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
#[cfg(feature = "winrt-")]
impl ::core::marker::Copy for AccountPictureKind {}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for AccountPictureKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "winrt-")]
impl ::core::default::Default for AccountPictureKind {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Abi for AccountPictureKind {
    type Abi = Self;
}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for AccountPictureKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccountPictureKind").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for AccountPictureKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.UserProfile.AccountPictureKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct AdvertisingManager;
impl AdvertisingManager {
    pub fn AdvertisingId() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAdvertisingManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AdvertisingId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn GetForUser<'a, Param0: ::windows_core::IntoParam<'a, super::User>>(user: Param0) -> ::windows_core::Result<AdvertisingManagerForUser> {
        Self::IAdvertisingManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), result__.as_mut_ptr()).from_abi::<AdvertisingManagerForUser>(result__)
        })
    }
    pub fn IAdvertisingManagerStatics<R, F: FnOnce(&IAdvertisingManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AdvertisingManager, IAdvertisingManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAdvertisingManagerStatics2<R, F: FnOnce(&IAdvertisingManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AdvertisingManager, IAdvertisingManagerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for AdvertisingManager {
    const NAME: &'static str = "Windows.System.UserProfile.AdvertisingManager";
}
#[repr(transparent)]
pub struct AdvertisingManagerForUser(::windows_core::IUnknown);
impl AdvertisingManagerForUser {
    pub fn AdvertisingId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AdvertisingId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn User(&self) -> ::windows_core::Result<super::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::User>(result__)
        }
    }
}
impl ::core::clone::Clone for AdvertisingManagerForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdvertisingManagerForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdvertisingManagerForUser {}
impl ::core::fmt::Debug for AdvertisingManagerForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdvertisingManagerForUser").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AdvertisingManagerForUser {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.UserProfile.AdvertisingManagerForUser;{928bf3d0-cf7c-4ab0-a7dc-6dc5bcd44252})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AdvertisingManagerForUser {
    type Vtable = IAdvertisingManagerForUser_Vtbl;
    const IID: ::windows_core::GUID = <IAdvertisingManagerForUser as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AdvertisingManagerForUser {
    const NAME: &'static str = "Windows.System.UserProfile.AdvertisingManagerForUser";
}
impl ::core::convert::From<AdvertisingManagerForUser> for ::windows_core::IUnknown {
    fn from(value: AdvertisingManagerForUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdvertisingManagerForUser> for ::windows_core::IUnknown {
    fn from(value: &AdvertisingManagerForUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AdvertisingManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AdvertisingManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdvertisingManagerForUser> for ::windows_core::IInspectable {
    fn from(value: AdvertisingManagerForUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdvertisingManagerForUser> for ::windows_core::IInspectable {
    fn from(value: &AdvertisingManagerForUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AdvertisingManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AdvertisingManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdvertisingManagerForUser {}
unsafe impl ::core::marker::Sync for AdvertisingManagerForUser {}
#[repr(transparent)]
pub struct AssignedAccessSettings(::windows_core::IUnknown);
impl AssignedAccessSettings {
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsSingleAppKioskMode(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSingleAppKioskMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn User(&self) -> ::windows_core::Result<super::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::User>(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<AssignedAccessSettings> {
        Self::IAssignedAccessSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AssignedAccessSettings>(result__)
        })
    }
    pub fn GetForUser<'a, Param0: ::windows_core::IntoParam<'a, super::User>>(user: Param0) -> ::windows_core::Result<AssignedAccessSettings> {
        Self::IAssignedAccessSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), result__.as_mut_ptr()).from_abi::<AssignedAccessSettings>(result__)
        })
    }
    pub fn IAssignedAccessSettingsStatics<R, F: FnOnce(&IAssignedAccessSettingsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AssignedAccessSettings, IAssignedAccessSettingsStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AssignedAccessSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AssignedAccessSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AssignedAccessSettings {}
impl ::core::fmt::Debug for AssignedAccessSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AssignedAccessSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AssignedAccessSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.UserProfile.AssignedAccessSettings;{1bc57f1c-e971-5757-b8e0-512f8b8c46d2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AssignedAccessSettings {
    type Vtable = IAssignedAccessSettings_Vtbl;
    const IID: ::windows_core::GUID = <IAssignedAccessSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AssignedAccessSettings {
    const NAME: &'static str = "Windows.System.UserProfile.AssignedAccessSettings";
}
impl ::core::convert::From<AssignedAccessSettings> for ::windows_core::IUnknown {
    fn from(value: AssignedAccessSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AssignedAccessSettings> for ::windows_core::IUnknown {
    fn from(value: &AssignedAccessSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AssignedAccessSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AssignedAccessSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AssignedAccessSettings> for ::windows_core::IInspectable {
    fn from(value: AssignedAccessSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AssignedAccessSettings> for ::windows_core::IInspectable {
    fn from(value: &AssignedAccessSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AssignedAccessSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AssignedAccessSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AssignedAccessSettings {}
unsafe impl ::core::marker::Sync for AssignedAccessSettings {}
#[repr(transparent)]
pub struct DiagnosticsSettings(::windows_core::IUnknown);
impl DiagnosticsSettings {
    pub fn CanUseDiagnosticsToTailorExperiences(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanUseDiagnosticsToTailorExperiences)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn User(&self) -> ::windows_core::Result<super::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::User>(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<DiagnosticsSettings> {
        Self::IDiagnosticsSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DiagnosticsSettings>(result__)
        })
    }
    pub fn GetForUser<'a, Param0: ::windows_core::IntoParam<'a, super::User>>(user: Param0) -> ::windows_core::Result<DiagnosticsSettings> {
        Self::IDiagnosticsSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), result__.as_mut_ptr()).from_abi::<DiagnosticsSettings>(result__)
        })
    }
    pub fn IDiagnosticsSettingsStatics<R, F: FnOnce(&IDiagnosticsSettingsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DiagnosticsSettings, IDiagnosticsSettingsStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DiagnosticsSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DiagnosticsSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DiagnosticsSettings {}
impl ::core::fmt::Debug for DiagnosticsSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DiagnosticsSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DiagnosticsSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.UserProfile.DiagnosticsSettings;{e5e9eccd-2711-44e0-973c-491d78048d24})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DiagnosticsSettings {
    type Vtable = IDiagnosticsSettings_Vtbl;
    const IID: ::windows_core::GUID = <IDiagnosticsSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DiagnosticsSettings {
    const NAME: &'static str = "Windows.System.UserProfile.DiagnosticsSettings";
}
impl ::core::convert::From<DiagnosticsSettings> for ::windows_core::IUnknown {
    fn from(value: DiagnosticsSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DiagnosticsSettings> for ::windows_core::IUnknown {
    fn from(value: &DiagnosticsSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DiagnosticsSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DiagnosticsSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DiagnosticsSettings> for ::windows_core::IInspectable {
    fn from(value: DiagnosticsSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DiagnosticsSettings> for ::windows_core::IInspectable {
    fn from(value: &DiagnosticsSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DiagnosticsSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DiagnosticsSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DiagnosticsSettings {}
unsafe impl ::core::marker::Sync for DiagnosticsSettings {}
#[repr(transparent)]
pub struct FirstSignInSettings(::windows_core::IUnknown);
impl FirstSignInSettings {
    pub fn GetDefault() -> ::windows_core::Result<FirstSignInSettings> {
        Self::IFirstSignInSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FirstSignInSettings>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Lookup<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), key.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn HasKey<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), key.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Split(&self, first: &mut ::core::option::Option<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>, second: &mut ::core::option::Option<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Split)(::windows_core::Interface::as_raw(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
    pub fn IFirstSignInSettingsStatics<R, F: FnOnce(&IFirstSignInSettingsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FirstSignInSettings, IFirstSignInSettingsStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for FirstSignInSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FirstSignInSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FirstSignInSettings {}
impl ::core::fmt::Debug for FirstSignInSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FirstSignInSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FirstSignInSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.UserProfile.FirstSignInSettings;{3e945153-3a5e-452e-a601-f5baad2a4870})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FirstSignInSettings {
    type Vtable = IFirstSignInSettings_Vtbl;
    const IID: ::windows_core::GUID = <IFirstSignInSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FirstSignInSettings {
    const NAME: &'static str = "Windows.System.UserProfile.FirstSignInSettings";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for FirstSignInSettings {
    type Item = ::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &FirstSignInSettings {
    type Item = ::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl ::core::convert::From<FirstSignInSettings> for ::windows_core::IUnknown {
    fn from(value: FirstSignInSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FirstSignInSettings> for ::windows_core::IUnknown {
    fn from(value: &FirstSignInSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FirstSignInSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FirstSignInSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FirstSignInSettings> for ::windows_core::IInspectable {
    fn from(value: FirstSignInSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FirstSignInSettings> for ::windows_core::IInspectable {
    fn from(value: &FirstSignInSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FirstSignInSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FirstSignInSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<FirstSignInSettings> for ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>> {
    type Error = ::windows_core::Error;
    fn try_from(value: FirstSignInSettings) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&FirstSignInSettings> for ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>> {
    type Error = ::windows_core::Error;
    fn try_from(value: &FirstSignInSettings) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>> for FirstSignInSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>> for &FirstSignInSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<FirstSignInSettings> for ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable> {
    type Error = ::windows_core::Error;
    fn try_from(value: FirstSignInSettings) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&FirstSignInSettings> for ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable> {
    type Error = ::windows_core::Error;
    fn try_from(value: &FirstSignInSettings) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> for FirstSignInSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> for &FirstSignInSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for FirstSignInSettings {}
unsafe impl ::core::marker::Sync for FirstSignInSettings {}
pub struct GlobalizationPreferences;
impl GlobalizationPreferences {
    #[cfg(feature = "winrt-foundation")]
    pub fn Calendars() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        Self::IGlobalizationPreferencesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Calendars)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Clocks() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        Self::IGlobalizationPreferencesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Clocks)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Currencies() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        Self::IGlobalizationPreferencesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Currencies)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Languages() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        Self::IGlobalizationPreferencesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Languages)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        })
    }
    pub fn HomeGeographicRegion() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IGlobalizationPreferencesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).HomeGeographicRegion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-globalization")]
    pub fn WeekStartsOn() -> ::windows_core::Result<::winrt_globalization::DayOfWeek> {
        Self::IGlobalizationPreferencesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_globalization::DayOfWeek>::zeroed();
            (::windows_core::Interface::vtable(this).WeekStartsOn)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_globalization::DayOfWeek>(result__)
        })
    }
    pub fn TrySetHomeGeographicRegion<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(region: Param0) -> ::windows_core::Result<bool> {
        Self::IGlobalizationPreferencesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TrySetHomeGeographicRegion)(::windows_core::Interface::as_raw(this), region.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn TrySetLanguages<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(languagetags: Param0) -> ::windows_core::Result<bool> {
        Self::IGlobalizationPreferencesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TrySetLanguages)(::windows_core::Interface::as_raw(this), languagetags.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn GetForUser<'a, Param0: ::windows_core::IntoParam<'a, super::User>>(user: Param0) -> ::windows_core::Result<GlobalizationPreferencesForUser> {
        Self::IGlobalizationPreferencesStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), result__.as_mut_ptr()).from_abi::<GlobalizationPreferencesForUser>(result__)
        })
    }
    pub fn IGlobalizationPreferencesStatics<R, F: FnOnce(&IGlobalizationPreferencesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GlobalizationPreferences, IGlobalizationPreferencesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGlobalizationPreferencesStatics2<R, F: FnOnce(&IGlobalizationPreferencesStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GlobalizationPreferences, IGlobalizationPreferencesStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGlobalizationPreferencesStatics3<R, F: FnOnce(&IGlobalizationPreferencesStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GlobalizationPreferences, IGlobalizationPreferencesStatics3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for GlobalizationPreferences {
    const NAME: &'static str = "Windows.System.UserProfile.GlobalizationPreferences";
}
#[repr(transparent)]
pub struct GlobalizationPreferencesForUser(::windows_core::IUnknown);
impl GlobalizationPreferencesForUser {
    pub fn User(&self) -> ::windows_core::Result<super::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::User>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Calendars(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Calendars)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Clocks(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Clocks)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Currencies(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Currencies)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Languages(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Languages)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn HomeGeographicRegion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).HomeGeographicRegion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-globalization")]
    pub fn WeekStartsOn(&self) -> ::windows_core::Result<::winrt_globalization::DayOfWeek> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_globalization::DayOfWeek>::zeroed();
            (::windows_core::Interface::vtable(this).WeekStartsOn)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_globalization::DayOfWeek>(result__)
        }
    }
}
impl ::core::clone::Clone for GlobalizationPreferencesForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GlobalizationPreferencesForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GlobalizationPreferencesForUser {}
impl ::core::fmt::Debug for GlobalizationPreferencesForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GlobalizationPreferencesForUser").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GlobalizationPreferencesForUser {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.UserProfile.GlobalizationPreferencesForUser;{150f0795-4f6e-40ba-a010-e27d81bda7f5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GlobalizationPreferencesForUser {
    type Vtable = IGlobalizationPreferencesForUser_Vtbl;
    const IID: ::windows_core::GUID = <IGlobalizationPreferencesForUser as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GlobalizationPreferencesForUser {
    const NAME: &'static str = "Windows.System.UserProfile.GlobalizationPreferencesForUser";
}
impl ::core::convert::From<GlobalizationPreferencesForUser> for ::windows_core::IUnknown {
    fn from(value: GlobalizationPreferencesForUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GlobalizationPreferencesForUser> for ::windows_core::IUnknown {
    fn from(value: &GlobalizationPreferencesForUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GlobalizationPreferencesForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GlobalizationPreferencesForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GlobalizationPreferencesForUser> for ::windows_core::IInspectable {
    fn from(value: GlobalizationPreferencesForUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GlobalizationPreferencesForUser> for ::windows_core::IInspectable {
    fn from(value: &GlobalizationPreferencesForUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GlobalizationPreferencesForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GlobalizationPreferencesForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GlobalizationPreferencesForUser {}
unsafe impl ::core::marker::Sync for GlobalizationPreferencesForUser {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvertisingManagerForUser(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdvertisingManagerForUser {
    type Vtable = IAdvertisingManagerForUser_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x928bf3d0_cf7c_4ab0_a7dc_6dc5bcd44252);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvertisingManagerForUser_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AdvertisingId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvertisingManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdvertisingManagerStatics {
    type Vtable = IAdvertisingManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xadd3468c_a273_48cb_b346_3544522d5581);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvertisingManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AdvertisingId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvertisingManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdvertisingManagerStatics2 {
    type Vtable = IAdvertisingManagerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdd0947af_1a6d_46b0_95bc_f3f9d6beb9fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvertisingManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAssignedAccessSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAssignedAccessSettings {
    type Vtable = IAssignedAccessSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1bc57f1c_e971_5757_b8e0_512f8b8c46d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAssignedAccessSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsSingleAppKioskMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAssignedAccessSettingsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAssignedAccessSettingsStatics {
    type Vtable = IAssignedAccessSettingsStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x34a81d0d_8a29_5ef3_a7be_618e6ac3bd01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAssignedAccessSettingsStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDiagnosticsSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDiagnosticsSettings {
    type Vtable = IDiagnosticsSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe5e9eccd_2711_44e0_973c_491d78048d24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiagnosticsSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CanUseDiagnosticsToTailorExperiences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDiagnosticsSettingsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDiagnosticsSettingsStatics {
    type Vtable = IDiagnosticsSettingsStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72d2e80f_5390_4793_990b_3ccc7d6ac9c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiagnosticsSettingsStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFirstSignInSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFirstSignInSettings {
    type Vtable = IFirstSignInSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3e945153_3a5e_452e_a601_f5baad2a4870);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFirstSignInSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFirstSignInSettingsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFirstSignInSettingsStatics {
    type Vtable = IFirstSignInSettingsStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1ce18f0f_1c41_4ea0_b7a2_6f0c1c7e8438);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFirstSignInSettingsStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalizationPreferencesForUser(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGlobalizationPreferencesForUser {
    type Vtable = IGlobalizationPreferencesForUser_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x150f0795_4f6e_40ba_a010_e27d81bda7f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalizationPreferencesForUser_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Calendars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Calendars: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Clocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Clocks: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Currencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Currencies: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Languages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Languages: usize,
    pub HomeGeographicRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-globalization")]
    pub WeekStartsOn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_globalization::DayOfWeek) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-globalization"))]
    WeekStartsOn: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalizationPreferencesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGlobalizationPreferencesStatics {
    type Vtable = IGlobalizationPreferencesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x01bf4326_ed37_4e96_b0e9_c1340d1ea158);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalizationPreferencesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Calendars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Calendars: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Clocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Clocks: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Currencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Currencies: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Languages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Languages: usize,
    pub HomeGeographicRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-globalization")]
    pub WeekStartsOn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_globalization::DayOfWeek) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-globalization"))]
    WeekStartsOn: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalizationPreferencesStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGlobalizationPreferencesStatics2 {
    type Vtable = IGlobalizationPreferencesStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfcce85f1_4300_4cd0_9cac_1a8e7b7e18f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalizationPreferencesStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TrySetHomeGeographicRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, region: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub TrySetLanguages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languagetags: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    TrySetLanguages: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalizationPreferencesStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGlobalizationPreferencesStatics3 {
    type Vtable = IGlobalizationPreferencesStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1e059733_35f5_40d8_b9e8_aef3ef856fce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalizationPreferencesStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILockScreenImageFeedStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILockScreenImageFeedStatics {
    type Vtable = ILockScreenImageFeedStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c0d73f6_03a9_41a6_9b01_495251ff51d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenImageFeedStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestSetImageFeedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, syndicationfeeduri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryRemoveImageFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILockScreenStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILockScreenStatics {
    type Vtable = ILockScreenStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3ee9d3ad_b607_40ae_b426_7631d9821269);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OriginalImageFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub GetImageStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    GetImageStream: usize,
    #[cfg(feature = "winrt-storage")]
    pub SetImageFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    SetImageFileAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub SetImageStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    SetImageStreamAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IUserInformationStatics(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IUserInformationStatics {
    type Vtable = IUserInformationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x77f3a910_48fa_489c_934e_2ae85ba8f772);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IUserInformationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub AccountPictureChangeEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    AccountPictureChangeEnabled: usize,
    #[cfg(feature = "winrt-")]
    pub NameAccessAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    NameAccessAllowed: usize,
    #[cfg(all(feature = "winrt-storage", feature = "winrt-"))]
    pub GetAccountPicture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: AccountPictureKind, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-storage", feature = "winrt-")))]
    GetAccountPicture: usize,
    #[cfg(all(feature = "winrt-storage", feature = "winrt-"))]
    pub SetAccountPictureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-storage", feature = "winrt-")))]
    SetAccountPictureAsync: usize,
    #[cfg(all(feature = "winrt-storage", feature = "winrt-"))]
    pub SetAccountPicturesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smallimage: ::windows_core::RawPtr, largeimage: ::windows_core::RawPtr, video: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-storage", feature = "winrt-")))]
    SetAccountPicturesAsync: usize,
    #[cfg(all(feature = "winrt-storage", feature = "winrt-"))]
    pub SetAccountPictureFromStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-storage", feature = "winrt-")))]
    SetAccountPictureFromStreamAsync: usize,
    #[cfg(all(feature = "winrt-storage", feature = "winrt-"))]
    pub SetAccountPicturesFromStreamsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smallimage: ::windows_core::RawPtr, largeimage: ::windows_core::RawPtr, video: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-storage", feature = "winrt-")))]
    SetAccountPicturesFromStreamsAsync: usize,
    #[cfg(feature = "winrt-")]
    pub AccountPictureChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changehandler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    AccountPictureChanged: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveAccountPictureChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveAccountPictureChanged: usize,
    #[cfg(feature = "winrt-")]
    pub GetDisplayNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    GetDisplayNameAsync: usize,
    #[cfg(feature = "winrt-")]
    pub GetFirstNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    GetFirstNameAsync: usize,
    #[cfg(feature = "winrt-")]
    pub GetLastNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    GetLastNameAsync: usize,
    #[cfg(feature = "winrt-")]
    pub GetPrincipalNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    GetPrincipalNameAsync: usize,
    #[cfg(feature = "winrt-")]
    pub GetSessionInitiationProtocolUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    GetSessionInitiationProtocolUriAsync: usize,
    #[cfg(feature = "winrt-")]
    pub GetDomainNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    GetDomainNameAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserProfilePersonalizationSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserProfilePersonalizationSettings {
    type Vtable = IUserProfilePersonalizationSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8ceddab4_7998_46d5_8dd3_184f1c5f9ab9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserProfilePersonalizationSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub TrySetLockScreenImageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefile: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    TrySetLockScreenImageAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub TrySetWallpaperImageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefile: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    TrySetWallpaperImageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserProfilePersonalizationSettingsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserProfilePersonalizationSettingsStatics {
    type Vtable = IUserProfilePersonalizationSettingsStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x91acb841_5037_454b_9883_bb772d08dd16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserProfilePersonalizationSettingsStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
pub struct LockScreen;
impl LockScreen {
    pub fn RequestSetImageFeedAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(syndicationfeeduri: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SetImageFeedResult>> {
        Self::ILockScreenImageFeedStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestSetImageFeedAsync)(::windows_core::Interface::as_raw(this), syndicationfeeduri.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SetImageFeedResult>>(result__)
        })
    }
    pub fn TryRemoveImageFeed() -> ::windows_core::Result<bool> {
        Self::ILockScreenImageFeedStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryRemoveImageFeed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn OriginalImageFile() -> ::windows_core::Result<::winrt_foundation::Uri> {
        Self::ILockScreenStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OriginalImageFile)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn GetImageStream() -> ::windows_core::Result<::winrt_storage::Streams::IRandomAccessStream> {
        Self::ILockScreenStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetImageStream)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IRandomAccessStream>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn SetImageFileAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFile>>(value: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        Self::ILockScreenStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetImageFileAsync)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn SetImageStreamAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>>(value: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        Self::ILockScreenStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetImageStreamAsync)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        })
    }
    pub fn ILockScreenImageFeedStatics<R, F: FnOnce(&ILockScreenImageFeedStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LockScreen, ILockScreenImageFeedStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ILockScreenStatics<R, F: FnOnce(&ILockScreenStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LockScreen, ILockScreenStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for LockScreen {
    const NAME: &'static str = "Windows.System.UserProfile.LockScreen";
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SetAccountPictureResult(pub i32);
#[cfg(feature = "winrt-")]
impl SetAccountPictureResult {
    pub const Success: Self = Self(0i32);
    pub const ChangeDisabled: Self = Self(1i32);
    pub const LargeOrDynamicError: Self = Self(2i32);
    pub const VideoFrameSizeError: Self = Self(3i32);
    pub const FileSizeError: Self = Self(4i32);
    pub const Failure: Self = Self(5i32);
}
#[cfg(feature = "winrt-")]
impl ::core::marker::Copy for SetAccountPictureResult {}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SetAccountPictureResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "winrt-")]
impl ::core::default::Default for SetAccountPictureResult {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Abi for SetAccountPictureResult {
    type Abi = Self;
}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SetAccountPictureResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SetAccountPictureResult").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SetAccountPictureResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.UserProfile.SetAccountPictureResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SetImageFeedResult(pub i32);
impl SetImageFeedResult {
    pub const Success: Self = Self(0i32);
    pub const ChangeDisabled: Self = Self(1i32);
    pub const UserCanceled: Self = Self(2i32);
}
impl ::core::marker::Copy for SetImageFeedResult {}
impl ::core::clone::Clone for SetImageFeedResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SetImageFeedResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SetImageFeedResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for SetImageFeedResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SetImageFeedResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SetImageFeedResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.UserProfile.SetImageFeedResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "winrt-")]
pub struct UserInformation;
#[cfg(feature = "winrt-")]
impl UserInformation {
    #[cfg(feature = "winrt-")]
    pub fn AccountPictureChangeEnabled() -> ::windows_core::Result<bool> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AccountPictureChangeEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn NameAccessAllowed() -> ::windows_core::Result<bool> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).NameAccessAllowed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[cfg(all(feature = "winrt-storage", feature = "winrt-"))]
    pub fn GetAccountPicture(kind: AccountPictureKind) -> ::windows_core::Result<::winrt_storage::IStorageFile> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAccountPicture)(::windows_core::Interface::as_raw(this), kind, result__.as_mut_ptr()).from_abi::<::winrt_storage::IStorageFile>(result__)
        })
    }
    #[cfg(all(feature = "winrt-storage", feature = "winrt-"))]
    pub fn SetAccountPictureAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFile>>(image: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SetAccountPictureResult>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetAccountPictureAsync)(::windows_core::Interface::as_raw(this), image.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SetAccountPictureResult>>(result__)
        })
    }
    #[cfg(all(feature = "winrt-storage", feature = "winrt-"))]
    pub fn SetAccountPicturesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFile>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFile>, Param2: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFile>>(smallimage: Param0, largeimage: Param1, video: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SetAccountPictureResult>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetAccountPicturesAsync)(::windows_core::Interface::as_raw(this), smallimage.into_param().abi(), largeimage.into_param().abi(), video.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SetAccountPictureResult>>(result__)
        })
    }
    #[cfg(all(feature = "winrt-storage", feature = "winrt-"))]
    pub fn SetAccountPictureFromStreamAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>>(image: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SetAccountPictureResult>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetAccountPictureFromStreamAsync)(::windows_core::Interface::as_raw(this), image.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SetAccountPictureResult>>(result__)
        })
    }
    #[cfg(all(feature = "winrt-storage", feature = "winrt-"))]
    pub fn SetAccountPicturesFromStreamsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>, Param2: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>>(smallimage: Param0, largeimage: Param1, video: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SetAccountPictureResult>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetAccountPicturesFromStreamsAsync)(::windows_core::Interface::as_raw(this), smallimage.into_param().abi(), largeimage.into_param().abi(), video.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SetAccountPictureResult>>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn AccountPictureChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(changehandler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AccountPictureChanged)(::windows_core::Interface::as_raw(this), changehandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveAccountPictureChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IUserInformationStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveAccountPictureChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "winrt-")]
    pub fn GetDisplayNameAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDisplayNameAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn GetFirstNameAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFirstNameAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn GetLastNameAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetLastNameAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn GetPrincipalNameAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPrincipalNameAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn GetSessionInitiationProtocolUriAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Uri>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSessionInitiationProtocolUriAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Uri>>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn GetDomainNameAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDomainNameAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn IUserInformationStatics<R, F: FnOnce(&IUserInformationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UserInformation, IUserInformationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for UserInformation {
    const NAME: &'static str = "Windows.System.UserProfile.UserInformation";
}
#[repr(transparent)]
pub struct UserProfilePersonalizationSettings(::windows_core::IUnknown);
impl UserProfilePersonalizationSettings {
    #[cfg(feature = "winrt-storage")]
    pub fn TrySetLockScreenImageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::StorageFile>>(&self, imagefile: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TrySetLockScreenImageAsync)(::windows_core::Interface::as_raw(this), imagefile.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn TrySetWallpaperImageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::StorageFile>>(&self, imagefile: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TrySetWallpaperImageAsync)(::windows_core::Interface::as_raw(this), imagefile.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn Current() -> ::windows_core::Result<UserProfilePersonalizationSettings> {
        Self::IUserProfilePersonalizationSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Current)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserProfilePersonalizationSettings>(result__)
        })
    }
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::IUserProfilePersonalizationSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IUserProfilePersonalizationSettingsStatics<R, F: FnOnce(&IUserProfilePersonalizationSettingsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UserProfilePersonalizationSettings, IUserProfilePersonalizationSettingsStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UserProfilePersonalizationSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserProfilePersonalizationSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserProfilePersonalizationSettings {}
impl ::core::fmt::Debug for UserProfilePersonalizationSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserProfilePersonalizationSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserProfilePersonalizationSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.UserProfile.UserProfilePersonalizationSettings;{8ceddab4-7998-46d5-8dd3-184f1c5f9ab9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserProfilePersonalizationSettings {
    type Vtable = IUserProfilePersonalizationSettings_Vtbl;
    const IID: ::windows_core::GUID = <IUserProfilePersonalizationSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserProfilePersonalizationSettings {
    const NAME: &'static str = "Windows.System.UserProfile.UserProfilePersonalizationSettings";
}
impl ::core::convert::From<UserProfilePersonalizationSettings> for ::windows_core::IUnknown {
    fn from(value: UserProfilePersonalizationSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserProfilePersonalizationSettings> for ::windows_core::IUnknown {
    fn from(value: &UserProfilePersonalizationSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserProfilePersonalizationSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserProfilePersonalizationSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserProfilePersonalizationSettings> for ::windows_core::IInspectable {
    fn from(value: UserProfilePersonalizationSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserProfilePersonalizationSettings> for ::windows_core::IInspectable {
    fn from(value: &UserProfilePersonalizationSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserProfilePersonalizationSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserProfilePersonalizationSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserProfilePersonalizationSettings {}
unsafe impl ::core::marker::Sync for UserProfilePersonalizationSettings {}
