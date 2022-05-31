#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DeviceAccountAuthenticationType(pub i32);
impl DeviceAccountAuthenticationType {
    pub const Basic: Self = Self(0i32);
    pub const OAuth: Self = Self(1i32);
    pub const SingleSignOn: Self = Self(2i32);
}
impl ::core::marker::Copy for DeviceAccountAuthenticationType {}
impl ::core::clone::Clone for DeviceAccountAuthenticationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeviceAccountAuthenticationType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DeviceAccountAuthenticationType {
    type Abi = Self;
}
impl ::core::fmt::Debug for DeviceAccountAuthenticationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceAccountAuthenticationType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeviceAccountAuthenticationType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountAuthenticationType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct DeviceAccountConfiguration(::windows_core::IUnknown);
impl DeviceAccountConfiguration {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DeviceAccountConfiguration, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn AccountName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AccountName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetAccountName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAccountName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DeviceAccountTypeId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceAccountTypeId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetDeviceAccountTypeId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDeviceAccountTypeId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ServerType(&self) -> ::windows_core::Result<DeviceAccountServerType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DeviceAccountServerType>::zeroed();
            (::windows_core::Interface::vtable(this).ServerType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DeviceAccountServerType>(result__)
        }
    }
    pub fn SetServerType(&self, value: DeviceAccountServerType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetServerType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EmailAddress(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).EmailAddress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetEmailAddress<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEmailAddress)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Domain(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Domain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetDomain<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDomain)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn EmailSyncEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).EmailSyncEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetEmailSyncEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEmailSyncEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ContactsSyncEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ContactsSyncEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetContactsSyncEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContactsSyncEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CalendarSyncEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CalendarSyncEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCalendarSyncEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCalendarSyncEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IncomingServerAddress(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).IncomingServerAddress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetIncomingServerAddress<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIncomingServerAddress)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IncomingServerPort(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).IncomingServerPort)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetIncomingServerPort(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIncomingServerPort)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IncomingServerRequiresSsl(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IncomingServerRequiresSsl)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIncomingServerRequiresSsl(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIncomingServerRequiresSsl)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IncomingServerUsername(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).IncomingServerUsername)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetIncomingServerUsername<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIncomingServerUsername)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn OutgoingServerAddress(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).OutgoingServerAddress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetOutgoingServerAddress<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOutgoingServerAddress)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn OutgoingServerPort(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).OutgoingServerPort)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetOutgoingServerPort(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOutgoingServerPort)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingServerRequiresSsl(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).OutgoingServerRequiresSsl)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetOutgoingServerRequiresSsl(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOutgoingServerRequiresSsl)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingServerUsername(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).OutgoingServerUsername)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetOutgoingServerUsername<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOutgoingServerUsername)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-security")]
    pub fn IncomingServerCredential(&self) -> ::windows_core::Result<::winrt_security::Credentials::PasswordCredential> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IncomingServerCredential)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(feature = "winrt-security")]
    pub fn SetIncomingServerCredential<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIncomingServerCredential)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-security")]
    pub fn OutgoingServerCredential(&self) -> ::windows_core::Result<::winrt_security::Credentials::PasswordCredential> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OutgoingServerCredential)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(feature = "winrt-security")]
    pub fn SetOutgoingServerCredential<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOutgoingServerCredential)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn OAuthRefreshToken(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).OAuthRefreshToken)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetOAuthRefreshToken<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOAuthRefreshToken)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsExternallyManaged(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsExternallyManaged)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsExternallyManaged(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsExternallyManaged)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AccountIconId(&self) -> ::windows_core::Result<DeviceAccountIconId> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DeviceAccountIconId>::zeroed();
            (::windows_core::Interface::vtable(this).AccountIconId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DeviceAccountIconId>(result__)
        }
    }
    pub fn SetAccountIconId(&self, value: DeviceAccountIconId) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAccountIconId)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AuthenticationType(&self) -> ::windows_core::Result<DeviceAccountAuthenticationType> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DeviceAccountAuthenticationType>::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticationType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DeviceAccountAuthenticationType>(result__)
        }
    }
    pub fn SetAuthenticationType(&self, value: DeviceAccountAuthenticationType) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAuthenticationType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsSsoAuthenticationSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSsoAuthenticationSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SsoAccountId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SsoAccountId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetSsoAccountId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSsoAccountId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn AlwaysDownloadFullMessage(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AlwaysDownloadFullMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAlwaysDownloadFullMessage(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAlwaysDownloadFullMessage)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DoesPolicyAllowMailSync(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).DoesPolicyAllowMailSync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SyncScheduleKind(&self) -> ::windows_core::Result<DeviceAccountSyncScheduleKind> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DeviceAccountSyncScheduleKind>::zeroed();
            (::windows_core::Interface::vtable(this).SyncScheduleKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DeviceAccountSyncScheduleKind>(result__)
        }
    }
    pub fn SetSyncScheduleKind(&self, value: DeviceAccountSyncScheduleKind) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSyncScheduleKind)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MailAgeFilter(&self) -> ::windows_core::Result<DeviceAccountMailAgeFilter> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DeviceAccountMailAgeFilter>::zeroed();
            (::windows_core::Interface::vtable(this).MailAgeFilter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DeviceAccountMailAgeFilter>(result__)
        }
    }
    pub fn SetMailAgeFilter(&self, value: DeviceAccountMailAgeFilter) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMailAgeFilter)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsClientAuthenticationCertificateRequired(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsClientAuthenticationCertificateRequired)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsClientAuthenticationCertificateRequired(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsClientAuthenticationCertificateRequired)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AutoSelectAuthenticationCertificate(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AutoSelectAuthenticationCertificate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAutoSelectAuthenticationCertificate(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAutoSelectAuthenticationCertificate)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AuthenticationCertificateId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticationCertificateId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetAuthenticationCertificateId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAuthenticationCertificateId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CardDavSyncScheduleKind(&self) -> ::windows_core::Result<DeviceAccountSyncScheduleKind> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DeviceAccountSyncScheduleKind>::zeroed();
            (::windows_core::Interface::vtable(this).CardDavSyncScheduleKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DeviceAccountSyncScheduleKind>(result__)
        }
    }
    pub fn SetCardDavSyncScheduleKind(&self, value: DeviceAccountSyncScheduleKind) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCardDavSyncScheduleKind)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CalDavSyncScheduleKind(&self) -> ::windows_core::Result<DeviceAccountSyncScheduleKind> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DeviceAccountSyncScheduleKind>::zeroed();
            (::windows_core::Interface::vtable(this).CalDavSyncScheduleKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DeviceAccountSyncScheduleKind>(result__)
        }
    }
    pub fn SetCalDavSyncScheduleKind(&self, value: DeviceAccountSyncScheduleKind) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCalDavSyncScheduleKind)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CardDavServerUrl(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CardDavServerUrl)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetCardDavServerUrl<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCardDavServerUrl)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CardDavRequiresSsl(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CardDavRequiresSsl)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCardDavRequiresSsl(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCardDavRequiresSsl)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CalDavServerUrl(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CalDavServerUrl)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetCalDavServerUrl<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCalDavServerUrl)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CalDavRequiresSsl(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CalDavRequiresSsl)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCalDavRequiresSsl(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCalDavRequiresSsl)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WasModifiedByUser(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).WasModifiedByUser)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetWasModifiedByUser(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetWasModifiedByUser)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WasIncomingServerCertificateHashConfirmed(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).WasIncomingServerCertificateHashConfirmed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetWasIncomingServerCertificateHashConfirmed(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetWasIncomingServerCertificateHashConfirmed)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IncomingServerCertificateHash(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).IncomingServerCertificateHash)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetIncomingServerCertificateHash<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIncomingServerCertificateHash)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsOutgoingServerAuthenticationRequired(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsOutgoingServerAuthenticationRequired)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsOutgoingServerAuthenticationRequired(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsOutgoingServerAuthenticationRequired)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsOutgoingServerAuthenticationEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsOutgoingServerAuthenticationEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsOutgoingServerAuthenticationEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsOutgoingServerAuthenticationEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WasOutgoingServerCertificateHashConfirmed(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).WasOutgoingServerCertificateHashConfirmed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetWasOutgoingServerCertificateHashConfirmed(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetWasOutgoingServerCertificateHashConfirmed)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingServerCertificateHash(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).OutgoingServerCertificateHash)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetOutgoingServerCertificateHash<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOutgoingServerCertificateHash)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsSyncScheduleManagedBySystem(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSyncScheduleManagedBySystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsSyncScheduleManagedBySystem(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsSyncScheduleManagedBySystem)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for DeviceAccountConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeviceAccountConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceAccountConfiguration {}
impl ::core::fmt::Debug for DeviceAccountConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceAccountConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeviceAccountConfiguration {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountConfiguration;{ad0123a3-fbdc-4d1b-be43-5a27ea4a1b63})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DeviceAccountConfiguration {
    type Vtable = IDeviceAccountConfiguration_Vtbl;
    const IID: ::windows_core::GUID = <IDeviceAccountConfiguration as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DeviceAccountConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountConfiguration";
}
impl ::core::convert::From<DeviceAccountConfiguration> for ::windows_core::IUnknown {
    fn from(value: DeviceAccountConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceAccountConfiguration> for ::windows_core::IUnknown {
    fn from(value: &DeviceAccountConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DeviceAccountConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DeviceAccountConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DeviceAccountConfiguration> for ::windows_core::IInspectable {
    fn from(value: DeviceAccountConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceAccountConfiguration> for ::windows_core::IInspectable {
    fn from(value: &DeviceAccountConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DeviceAccountConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DeviceAccountConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DeviceAccountConfiguration {}
unsafe impl ::core::marker::Sync for DeviceAccountConfiguration {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DeviceAccountIconId(pub i32);
impl DeviceAccountIconId {
    pub const Exchange: Self = Self(0i32);
    pub const Msa: Self = Self(1i32);
    pub const Outlook: Self = Self(2i32);
    pub const Generic: Self = Self(3i32);
}
impl ::core::marker::Copy for DeviceAccountIconId {}
impl ::core::clone::Clone for DeviceAccountIconId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeviceAccountIconId {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DeviceAccountIconId {
    type Abi = Self;
}
impl ::core::fmt::Debug for DeviceAccountIconId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceAccountIconId").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeviceAccountIconId {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountIconId;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DeviceAccountMailAgeFilter(pub i32);
impl DeviceAccountMailAgeFilter {
    pub const All: Self = Self(0i32);
    pub const Last1Day: Self = Self(1i32);
    pub const Last3Days: Self = Self(2i32);
    pub const Last7Days: Self = Self(3i32);
    pub const Last14Days: Self = Self(4i32);
    pub const Last30Days: Self = Self(5i32);
    pub const Last90Days: Self = Self(6i32);
}
impl ::core::marker::Copy for DeviceAccountMailAgeFilter {}
impl ::core::clone::Clone for DeviceAccountMailAgeFilter {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeviceAccountMailAgeFilter {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DeviceAccountMailAgeFilter {
    type Abi = Self;
}
impl ::core::fmt::Debug for DeviceAccountMailAgeFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceAccountMailAgeFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeviceAccountMailAgeFilter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountMailAgeFilter;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DeviceAccountServerType(pub i32);
impl DeviceAccountServerType {
    pub const Exchange: Self = Self(0i32);
    pub const Pop: Self = Self(1i32);
    pub const Imap: Self = Self(2i32);
}
impl ::core::marker::Copy for DeviceAccountServerType {}
impl ::core::clone::Clone for DeviceAccountServerType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeviceAccountServerType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DeviceAccountServerType {
    type Abi = Self;
}
impl ::core::fmt::Debug for DeviceAccountServerType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceAccountServerType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeviceAccountServerType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountServerType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DeviceAccountSyncScheduleKind(pub i32);
impl DeviceAccountSyncScheduleKind {
    pub const Manual: Self = Self(0i32);
    pub const Every15Minutes: Self = Self(1i32);
    pub const Every30Minutes: Self = Self(2i32);
    pub const Every60Minutes: Self = Self(3i32);
    pub const Every2Hours: Self = Self(4i32);
    pub const Daily: Self = Self(5i32);
    pub const AsItemsArrive: Self = Self(6i32);
}
impl ::core::marker::Copy for DeviceAccountSyncScheduleKind {}
impl ::core::clone::Clone for DeviceAccountSyncScheduleKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeviceAccountSyncScheduleKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DeviceAccountSyncScheduleKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for DeviceAccountSyncScheduleKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceAccountSyncScheduleKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeviceAccountSyncScheduleKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountSyncScheduleKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceAccountConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceAccountConfiguration {
    type Vtable = IDeviceAccountConfiguration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xad0123a3_fbdc_4d1b_be43_5a27ea4a1b63);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceAccountConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AccountName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAccountName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DeviceAccountTypeId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDeviceAccountTypeId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ServerType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccountServerType) -> ::windows_core::HRESULT,
    pub SetServerType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: DeviceAccountServerType) -> ::windows_core::HRESULT,
    pub EmailAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetEmailAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Domain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub EmailSyncEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetEmailSyncEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ContactsSyncEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetContactsSyncEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CalendarSyncEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCalendarSyncEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IncomingServerAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetIncomingServerAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IncomingServerPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetIncomingServerPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub IncomingServerRequiresSsl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIncomingServerRequiresSsl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IncomingServerUsername: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetIncomingServerUsername: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub OutgoingServerAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetOutgoingServerAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub OutgoingServerPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetOutgoingServerPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub OutgoingServerRequiresSsl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetOutgoingServerRequiresSsl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub OutgoingServerUsername: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetOutgoingServerUsername: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceAccountConfiguration2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceAccountConfiguration2 {
    type Vtable = IDeviceAccountConfiguration2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf2b2e5a6_728d_4a4a_8945_2bf8580136de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceAccountConfiguration2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-security")]
    pub IncomingServerCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-security"))]
    IncomingServerCredential: usize,
    #[cfg(feature = "winrt-security")]
    pub SetIncomingServerCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-security"))]
    SetIncomingServerCredential: usize,
    #[cfg(feature = "winrt-security")]
    pub OutgoingServerCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-security"))]
    OutgoingServerCredential: usize,
    #[cfg(feature = "winrt-security")]
    pub SetOutgoingServerCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-security"))]
    SetOutgoingServerCredential: usize,
    pub OAuthRefreshToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetOAuthRefreshToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsExternallyManaged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsExternallyManaged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub AccountIconId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccountIconId) -> ::windows_core::HRESULT,
    pub SetAccountIconId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: DeviceAccountIconId) -> ::windows_core::HRESULT,
    pub AuthenticationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccountAuthenticationType) -> ::windows_core::HRESULT,
    pub SetAuthenticationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: DeviceAccountAuthenticationType) -> ::windows_core::HRESULT,
    pub IsSsoAuthenticationSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SsoAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSsoAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AlwaysDownloadFullMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAlwaysDownloadFullMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub DoesPolicyAllowMailSync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SyncScheduleKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccountSyncScheduleKind) -> ::windows_core::HRESULT,
    pub SetSyncScheduleKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: DeviceAccountSyncScheduleKind) -> ::windows_core::HRESULT,
    pub MailAgeFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccountMailAgeFilter) -> ::windows_core::HRESULT,
    pub SetMailAgeFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: DeviceAccountMailAgeFilter) -> ::windows_core::HRESULT,
    pub IsClientAuthenticationCertificateRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsClientAuthenticationCertificateRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub AutoSelectAuthenticationCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAutoSelectAuthenticationCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub AuthenticationCertificateId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAuthenticationCertificateId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CardDavSyncScheduleKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccountSyncScheduleKind) -> ::windows_core::HRESULT,
    pub SetCardDavSyncScheduleKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: DeviceAccountSyncScheduleKind) -> ::windows_core::HRESULT,
    pub CalDavSyncScheduleKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccountSyncScheduleKind) -> ::windows_core::HRESULT,
    pub SetCalDavSyncScheduleKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: DeviceAccountSyncScheduleKind) -> ::windows_core::HRESULT,
    pub CardDavServerUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCardDavServerUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CardDavRequiresSsl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCardDavRequiresSsl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CalDavServerUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCalDavServerUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CalDavRequiresSsl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCalDavRequiresSsl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub WasModifiedByUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetWasModifiedByUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub WasIncomingServerCertificateHashConfirmed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetWasIncomingServerCertificateHashConfirmed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IncomingServerCertificateHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetIncomingServerCertificateHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsOutgoingServerAuthenticationRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsOutgoingServerAuthenticationRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsOutgoingServerAuthenticationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsOutgoingServerAuthenticationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub WasOutgoingServerCertificateHashConfirmed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetWasOutgoingServerCertificateHashConfirmed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub OutgoingServerCertificateHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetOutgoingServerCertificateHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsSyncScheduleManagedBySystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsSyncScheduleManagedBySystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountSystemAccessManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataAccountSystemAccessManagerStatics {
    type Vtable = IUserDataAccountSystemAccessManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9d6b11b9_cbe5_45f5_822b_c267b81dbdb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountSystemAccessManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub AddAndShowDeviceAccountsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accounts: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    AddAndShowDeviceAccountsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountSystemAccessManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataAccountSystemAccessManagerStatics2 {
    type Vtable = IUserDataAccountSystemAccessManagerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x943f854d_4b4e_439f_83d3_979b27c05ac7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountSystemAccessManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SuppressLocalAccountWithAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userdataaccountid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateDeviceAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, account: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeleteDeviceAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accountid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeviceAccountConfigurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accountid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
pub struct UserDataAccountSystemAccessManager;
impl UserDataAccountSystemAccessManager {
    #[cfg(feature = "winrt-foundation")]
    pub fn AddAndShowDeviceAccountsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<DeviceAccountConfiguration>>>(accounts: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>> {
        Self::IUserDataAccountSystemAccessManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AddAndShowDeviceAccountsAsync)(::windows_core::Interface::as_raw(this), accounts.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>>(result__)
        })
    }
    pub fn SuppressLocalAccountWithAccountAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(userdataaccountid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        Self::IUserDataAccountSystemAccessManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SuppressLocalAccountWithAccountAsync)(::windows_core::Interface::as_raw(this), userdataaccountid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        })
    }
    pub fn CreateDeviceAccountAsync<'a, Param0: ::windows_core::IntoParam<'a, DeviceAccountConfiguration>>(account: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::IUserDataAccountSystemAccessManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateDeviceAccountAsync)(::windows_core::Interface::as_raw(this), account.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        })
    }
    pub fn DeleteDeviceAccountAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(accountid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        Self::IUserDataAccountSystemAccessManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteDeviceAccountAsync)(::windows_core::Interface::as_raw(this), accountid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        })
    }
    pub fn GetDeviceAccountConfigurationAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(accountid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DeviceAccountConfiguration>> {
        Self::IUserDataAccountSystemAccessManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceAccountConfigurationAsync)(::windows_core::Interface::as_raw(this), accountid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DeviceAccountConfiguration>>(result__)
        })
    }
    pub fn IUserDataAccountSystemAccessManagerStatics<R, F: FnOnce(&IUserDataAccountSystemAccessManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UserDataAccountSystemAccessManager, IUserDataAccountSystemAccessManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IUserDataAccountSystemAccessManagerStatics2<R, F: FnOnce(&IUserDataAccountSystemAccessManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UserDataAccountSystemAccessManager, IUserDataAccountSystemAccessManagerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for UserDataAccountSystemAccessManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.SystemAccess.UserDataAccountSystemAccessManager";
}
