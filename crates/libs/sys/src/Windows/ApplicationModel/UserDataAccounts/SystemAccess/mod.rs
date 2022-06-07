#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
#[repr(transparent)]
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
pub type DeviceAccountConfiguration = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
#[repr(transparent)]
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
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
#[repr(transparent)]
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
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
#[repr(transparent)]
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
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
#[repr(transparent)]
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
#[repr(C)]
pub struct IDeviceAccountConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    pub AccountName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetAccountName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DeviceAccountTypeId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDeviceAccountTypeId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ServerType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DeviceAccountServerType) -> ::windows_sys::core::HRESULT,
    pub SetServerType: unsafe extern "system" fn(this: *mut *mut Self, value: DeviceAccountServerType) -> ::windows_sys::core::HRESULT,
    pub EmailAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetEmailAddress: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Domain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDomain: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EmailSyncEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetEmailSyncEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ContactsSyncEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetContactsSyncEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CalendarSyncEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCalendarSyncEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IncomingServerAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetIncomingServerAddress: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IncomingServerPort: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetIncomingServerPort: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub IncomingServerRequiresSsl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIncomingServerRequiresSsl: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IncomingServerUsername: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetIncomingServerUsername: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub OutgoingServerAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetOutgoingServerAddress: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub OutgoingServerPort: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetOutgoingServerPort: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub OutgoingServerRequiresSsl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetOutgoingServerRequiresSsl: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub OutgoingServerUsername: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetOutgoingServerUsername: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeviceAccountConfiguration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2902533027, data2: 64476, data3: 19739, data4: [190, 67, 90, 39, 234, 74, 27, 99] };
}
#[repr(C)]
pub struct IDeviceAccountConfiguration2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Credentials")]
    pub IncomingServerCredential: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    IncomingServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetIncomingServerCredential: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetIncomingServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub OutgoingServerCredential: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    OutgoingServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetOutgoingServerCredential: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetOutgoingServerCredential: usize,
    pub OAuthRefreshToken: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetOAuthRefreshToken: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsExternallyManaged: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsExternallyManaged: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AccountIconId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DeviceAccountIconId) -> ::windows_sys::core::HRESULT,
    pub SetAccountIconId: unsafe extern "system" fn(this: *mut *mut Self, value: DeviceAccountIconId) -> ::windows_sys::core::HRESULT,
    pub AuthenticationType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DeviceAccountAuthenticationType) -> ::windows_sys::core::HRESULT,
    pub SetAuthenticationType: unsafe extern "system" fn(this: *mut *mut Self, value: DeviceAccountAuthenticationType) -> ::windows_sys::core::HRESULT,
    pub IsSsoAuthenticationSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SsoAccountId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSsoAccountId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AlwaysDownloadFullMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAlwaysDownloadFullMessage: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub DoesPolicyAllowMailSync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SyncScheduleKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DeviceAccountSyncScheduleKind) -> ::windows_sys::core::HRESULT,
    pub SetSyncScheduleKind: unsafe extern "system" fn(this: *mut *mut Self, value: DeviceAccountSyncScheduleKind) -> ::windows_sys::core::HRESULT,
    pub MailAgeFilter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DeviceAccountMailAgeFilter) -> ::windows_sys::core::HRESULT,
    pub SetMailAgeFilter: unsafe extern "system" fn(this: *mut *mut Self, value: DeviceAccountMailAgeFilter) -> ::windows_sys::core::HRESULT,
    pub IsClientAuthenticationCertificateRequired: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsClientAuthenticationCertificateRequired: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AutoSelectAuthenticationCertificate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAutoSelectAuthenticationCertificate: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AuthenticationCertificateId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetAuthenticationCertificateId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CardDavSyncScheduleKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DeviceAccountSyncScheduleKind) -> ::windows_sys::core::HRESULT,
    pub SetCardDavSyncScheduleKind: unsafe extern "system" fn(this: *mut *mut Self, value: DeviceAccountSyncScheduleKind) -> ::windows_sys::core::HRESULT,
    pub CalDavSyncScheduleKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DeviceAccountSyncScheduleKind) -> ::windows_sys::core::HRESULT,
    pub SetCalDavSyncScheduleKind: unsafe extern "system" fn(this: *mut *mut Self, value: DeviceAccountSyncScheduleKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CardDavServerUrl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CardDavServerUrl: usize,
    #[cfg(feature = "Foundation")]
    pub SetCardDavServerUrl: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCardDavServerUrl: usize,
    pub CardDavRequiresSsl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCardDavRequiresSsl: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CalDavServerUrl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CalDavServerUrl: usize,
    #[cfg(feature = "Foundation")]
    pub SetCalDavServerUrl: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCalDavServerUrl: usize,
    pub CalDavRequiresSsl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCalDavRequiresSsl: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub WasModifiedByUser: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetWasModifiedByUser: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub WasIncomingServerCertificateHashConfirmed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetWasIncomingServerCertificateHashConfirmed: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IncomingServerCertificateHash: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetIncomingServerCertificateHash: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsOutgoingServerAuthenticationRequired: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsOutgoingServerAuthenticationRequired: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsOutgoingServerAuthenticationEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsOutgoingServerAuthenticationEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub WasOutgoingServerCertificateHashConfirmed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetWasOutgoingServerCertificateHashConfirmed: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub OutgoingServerCertificateHash: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetOutgoingServerCertificateHash: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsSyncScheduleManagedBySystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsSyncScheduleManagedBySystem: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeviceAccountConfiguration2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4071810470, data2: 29325, data3: 19018, data4: [137, 69, 43, 248, 88, 1, 54, 222] };
}
#[repr(C)]
pub struct IUserDataAccountSystemAccessManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AddAndShowDeviceAccountsAsync: unsafe extern "system" fn(this: *mut *mut Self, accounts: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddAndShowDeviceAccountsAsync: usize,
}
impl ::windows_sys::core::Interface for IUserDataAccountSystemAccessManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2641039801, data2: 52197, data3: 17909, data4: [130, 43, 194, 103, 184, 29, 189, 182] };
}
#[repr(C)]
pub struct IUserDataAccountSystemAccessManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SuppressLocalAccountWithAccountAsync: unsafe extern "system" fn(this: *mut *mut Self, userdataaccountid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SuppressLocalAccountWithAccountAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateDeviceAccountAsync: unsafe extern "system" fn(this: *mut *mut Self, account: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateDeviceAccountAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteDeviceAccountAsync: unsafe extern "system" fn(this: *mut *mut Self, accountid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteDeviceAccountAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDeviceAccountConfigurationAsync: unsafe extern "system" fn(this: *mut *mut Self, accountid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeviceAccountConfigurationAsync: usize,
}
impl ::windows_sys::core::Interface for IUserDataAccountSystemAccessManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2487190861, data2: 19278, data3: 17311, data4: [131, 211, 151, 155, 39, 192, 90, 199] };
}
