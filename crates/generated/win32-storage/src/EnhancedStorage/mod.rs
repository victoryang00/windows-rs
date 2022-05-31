#[repr(C)]
pub struct ACT_AUTHORIZATION_STATE {
    pub ulState: u32,
}
impl ::core::marker::Copy for ACT_AUTHORIZATION_STATE {}
impl ::core::clone::Clone for ACT_AUTHORIZATION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACT_AUTHORIZATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACT_AUTHORIZATION_STATE").field("ulState", &self.ulState).finish()
    }
}
unsafe impl ::windows_core::Abi for ACT_AUTHORIZATION_STATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACT_AUTHORIZATION_STATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACT_AUTHORIZATION_STATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACT_AUTHORIZATION_STATE {}
impl ::core::default::Default for ACT_AUTHORIZATION_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ACT_AUTHORIZATION_STATE_VALUE(pub i32);
pub const ACT_UNAUTHORIZED: ACT_AUTHORIZATION_STATE_VALUE = ACT_AUTHORIZATION_STATE_VALUE(0i32);
pub const ACT_AUTHORIZED: ACT_AUTHORIZATION_STATE_VALUE = ACT_AUTHORIZATION_STATE_VALUE(1i32);
impl ::core::marker::Copy for ACT_AUTHORIZATION_STATE_VALUE {}
impl ::core::clone::Clone for ACT_AUTHORIZATION_STATE_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ACT_AUTHORIZATION_STATE_VALUE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ACT_AUTHORIZATION_STATE_VALUE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ACT_AUTHORIZATION_STATE_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACT_AUTHORIZATION_STATE_VALUE").field(&self.0).finish()
    }
}
pub const ACT_AUTHORIZE_ON_RESUME: u32 = 1u32;
pub const ACT_AUTHORIZE_ON_SESSION_UNLOCK: u32 = 2u32;
pub const ACT_UNAUTHORIZE_ON_SESSION_LOCK: u32 = 2u32;
pub const ACT_UNAUTHORIZE_ON_SUSPEND: u32 = 1u32;
pub const APPUSERMODEL_STARTPINOPTION_DEFAULT: u32 = 0u32;
pub const APPUSERMODEL_STARTPINOPTION_NOPINONINSTALL: u32 = 1u32;
pub const APPUSERMODEL_STARTPINOPTION_USERPINNED: u32 = 2u32;
pub const AUDIO_CHANNELCOUNT_MONO: u32 = 1u32;
pub const AUDIO_CHANNELCOUNT_STEREO: u32 = 2u32;
pub const BLUETOOTH_ADDRESS_TYPE_PUBLIC: u32 = 0u32;
pub const BLUETOOTH_ADDRESS_TYPE_RANDOM: u32 = 1u32;
pub const BLUETOOTH_CACHED_MODE_UNCACHED: u32 = 1u32;
pub const BLUETOOTH_CACHE_MODE_CACHED: u32 = 0u32;
pub const CERT_CAPABILITY_ASYMMETRIC_KEY_CRYPTOGRAPHY: u32 = 2u32;
pub const CERT_CAPABILITY_CERTIFICATE_SUPPORT: u32 = 4u32;
pub const CERT_CAPABILITY_HASH_ALG: u32 = 1u32;
pub const CERT_CAPABILITY_OPTIONAL_FEATURES: u32 = 5u32;
pub const CERT_CAPABILITY_SIGNATURE_ALG: u32 = 3u32;
pub const CERT_MAX_CAPABILITY: u32 = 255u32;
pub const CERT_RSASSA_PSS_SHA1_OID: &str = "1.2.840.113549.1.1.10,1.3.14.3.2.26";
pub const CERT_RSASSA_PSS_SHA256_OID: &str = "1.2.840.113549.1.1.10,2.16.840.1.101.3.4.2.1";
pub const CERT_RSASSA_PSS_SHA384_OID: &str = "1.2.840.113549.1.1.10,2.16.840.1.101.3.4.2.2";
pub const CERT_RSASSA_PSS_SHA512_OID: &str = "1.2.840.113549.1.1.10,2.16.840.1.101.3.4.2.3";
pub const CERT_RSA_1024_OID: &str = "1.2.840.113549.1.1.1,1024";
pub const CERT_RSA_2048_OID: &str = "1.2.840.113549.1.1.1,2048";
pub const CERT_RSA_3072_OID: &str = "1.2.840.113549.1.1.1,3072";
pub const CERT_TYPE_ASCh: u32 = 3u32;
pub const CERT_TYPE_ASCm: u32 = 1u32;
pub const CERT_TYPE_EMPTY: u32 = 0u32;
pub const CERT_TYPE_HCh: u32 = 4u32;
pub const CERT_TYPE_PCp: u32 = 2u32;
pub const CERT_TYPE_SIGNER: u32 = 6u32;
pub const CERT_VALIDATION_POLICY_BASIC: u32 = 2u32;
pub const CERT_VALIDATION_POLICY_EXTENDED: u32 = 3u32;
pub const CERT_VALIDATION_POLICY_NONE: u32 = 1u32;
pub const CERT_VALIDATION_POLICY_RESERVED: u32 = 0u32;
pub const CREATOROPENWITHUIOPTION_HIDDEN: u32 = 0u32;
pub const CREATOROPENWITHUIOPTION_VISIBLE: u32 = 1u32;
pub const ENHANCED_STORAGE_AUTHN_STATE_AUTHENTICATED: u32 = 3u32;
pub const ENHANCED_STORAGE_AUTHN_STATE_AUTHENTICATION_DENIED: u32 = 2147483649u32;
pub const ENHANCED_STORAGE_AUTHN_STATE_DEVICE_ERROR: u32 = 2147483650u32;
pub const ENHANCED_STORAGE_AUTHN_STATE_NOT_AUTHENTICATED: u32 = 2u32;
pub const ENHANCED_STORAGE_AUTHN_STATE_NO_AUTHENTICATION_REQUIRED: u32 = 1u32;
pub const ENHANCED_STORAGE_AUTHN_STATE_UNKNOWN: u32 = 0u32;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_CAPABILITY_ASYMMETRIC_KEY_CRYPTOGRAPHY: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 4002u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_CAPABILITY_CERTIFICATE_EXTENSION_PARSING: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 4005u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_CAPABILITY_HASH_ALGS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 4001u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_CAPABILITY_RENDER_USER_DATA_UNUSABLE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 4004u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_CAPABILITY_SIGNING_ALGS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 4003u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_CERT_ADMIN_CERTIFICATE_AUTHENTICATION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 103u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_CERT_CREATE_CERTIFICATE_REQUEST: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 108u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_CERT_DEVICE_CERTIFICATE_AUTHENTICATION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 102u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_CERT_GET_ACT_FRIENDLY_NAME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 113u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_CERT_GET_CERTIFICATE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 106u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_CERT_GET_CERTIFICATE_COUNT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 105u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_CERT_GET_SILO_CAPABILITIES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 112u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_CERT_GET_SILO_CAPABILITY: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 111u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_CERT_GET_SILO_GUID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 114u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_CERT_HOST_CERTIFICATE_AUTHENTICATION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 101u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_CERT_INITIALIZE_TO_MANUFACTURER_STATE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 104u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_CERT_SET_CERTIFICATE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 107u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_CERT_UNAUTHENTICATION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 110u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_PASSWORD_AUTHORIZE_ACT_ACCESS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 203u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_PASSWORD_CHANGE_PASSWORD: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 209u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_PASSWORD_CONFIG_ADMINISTRATOR: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 206u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_PASSWORD_CREATE_USER: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 207u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_PASSWORD_DELETE_USER: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 208u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_PASSWORD_INITIALIZE_USER_PASSWORD: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 210u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_PASSWORD_QUERY_INFORMATION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 205u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_PASSWORD_START_INITIALIZE_TO_MANUFACTURER_STATE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 211u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_PASSWORD_UNAUTHORIZE_ACT_ACCESS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 204u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_SILO_ENUMERATE_SILOS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 11u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_SILO_GET_AUTHENTICATION_STATE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_COMMAND_SILO_IS_AUTHENTICATION_SILO: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 6u32 };
#[repr(C)]
pub struct ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION {
    pub CurrentAdminFailures: u8,
    pub CurrentUserFailures: u8,
    pub TotalUserAuthenticationCount: u32,
    pub TotalAdminAuthenticationCount: u32,
    pub FipsCompliant: ::win32_foundation::BOOL,
    pub SecurityIDAvailable: ::win32_foundation::BOOL,
    pub InitializeInProgress: ::win32_foundation::BOOL,
    pub ITMSArmed: ::win32_foundation::BOOL,
    pub ITMSArmable: ::win32_foundation::BOOL,
    pub UserCreated: ::win32_foundation::BOOL,
    pub ResetOnPORDefault: ::win32_foundation::BOOL,
    pub ResetOnPORCurrent: ::win32_foundation::BOOL,
    pub MaxAdminFailures: u8,
    pub MaxUserFailures: u8,
    pub TimeToCompleteInitialization: u32,
    pub TimeRemainingToCompleteInitialization: u32,
    pub MinTimeToAuthenticate: u32,
    pub MaxAdminPasswordSize: u8,
    pub MinAdminPasswordSize: u8,
    pub MaxAdminHintSize: u8,
    pub MaxUserPasswordSize: u8,
    pub MinUserPasswordSize: u8,
    pub MaxUserHintSize: u8,
    pub MaxUserNameSize: u8,
    pub MaxSiloNameSize: u8,
    pub MaxChallengeSize: u16,
}
impl ::core::marker::Copy for ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION {}
impl ::core::clone::Clone for ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION")
            .field("CurrentAdminFailures", &self.CurrentAdminFailures)
            .field("CurrentUserFailures", &self.CurrentUserFailures)
            .field("TotalUserAuthenticationCount", &self.TotalUserAuthenticationCount)
            .field("TotalAdminAuthenticationCount", &self.TotalAdminAuthenticationCount)
            .field("FipsCompliant", &self.FipsCompliant)
            .field("SecurityIDAvailable", &self.SecurityIDAvailable)
            .field("InitializeInProgress", &self.InitializeInProgress)
            .field("ITMSArmed", &self.ITMSArmed)
            .field("ITMSArmable", &self.ITMSArmable)
            .field("UserCreated", &self.UserCreated)
            .field("ResetOnPORDefault", &self.ResetOnPORDefault)
            .field("ResetOnPORCurrent", &self.ResetOnPORCurrent)
            .field("MaxAdminFailures", &self.MaxAdminFailures)
            .field("MaxUserFailures", &self.MaxUserFailures)
            .field("TimeToCompleteInitialization", &self.TimeToCompleteInitialization)
            .field("TimeRemainingToCompleteInitialization", &self.TimeRemainingToCompleteInitialization)
            .field("MinTimeToAuthenticate", &self.MinTimeToAuthenticate)
            .field("MaxAdminPasswordSize", &self.MaxAdminPasswordSize)
            .field("MinAdminPasswordSize", &self.MinAdminPasswordSize)
            .field("MaxAdminHintSize", &self.MaxAdminHintSize)
            .field("MaxUserPasswordSize", &self.MaxUserPasswordSize)
            .field("MinUserPasswordSize", &self.MinUserPasswordSize)
            .field("MaxUserHintSize", &self.MaxUserHintSize)
            .field("MaxUserNameSize", &self.MaxUserNameSize)
            .field("MaxSiloNameSize", &self.MaxSiloNameSize)
            .field("MaxChallengeSize", &self.MaxChallengeSize)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION {}
impl ::core::default::Default for ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_ADMIN_HINT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 2011u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_AUTHENTICATION_STATE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 1006u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_CERTIFICATE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 3009u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_CERTIFICATE_ACT_FRIENDLY_NAME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 3014u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_CERTIFICATE_CAPABILITY_TYPE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 3011u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_CERTIFICATE_INDEX: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 3003u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_CERTIFICATE_LENGTH: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 3008u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_CERTIFICATE_REQUEST: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 3010u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_CERTIFICATE_SILO_CAPABILITIES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 3013u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_CERTIFICATE_SILO_CAPABILITY: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 3012u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_CERTIFICATE_SILO_GUID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 3015u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_CERTIFICATE_TYPE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 3004u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_IS_AUTHENTICATION_SILO: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 1009u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_MAX_AUTH_FAILURES: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 2001u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_MAX_CERTIFICATE_COUNT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 3001u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_NEW_PASSWORD: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 2008u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_NEW_PASSWORD_INDICATOR: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 2007u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_NEXT_CERTIFICATE_INDEX: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 3006u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_NEXT_CERTIFICATE_OF_TYPE_INDEX: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 3007u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_OLD_PASSWORD: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 2005u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_PASSWORD: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 2004u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_PASSWORD_INDICATOR: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 2006u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_PASSWORD_SILO_INFO: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 2014u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_QUERY_SILO_RESULTS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 2017u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_QUERY_SILO_TYPE: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 2016u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_SECURITY_IDENTIFIER: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 2015u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_SIGNER_CERTIFICATE_INDEX: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 3016u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_SILO_FRIENDLYNAME_SPECIFIED: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 2013u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_SILO_NAME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 2012u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_STORED_CERTIFICATE_COUNT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 3002u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_TEMPORARY_UNAUTHENTICATION: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 1010u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_USER_HINT: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 2009u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_USER_NAME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 2010u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const ENHANCED_STORAGE_PROPERTY_VALIDATION_POLICY: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c), pid: 3005u32 };
pub const ES_AUTHN_ERROR_END: u32 = 1279u32;
pub const ES_AUTHN_ERROR_START: u32 = 1024u32;
pub const ES_GENERAL_ERROR_END: u32 = 1023u32;
pub const ES_GENERAL_ERROR_START: u32 = 512u32;
pub const ES_PW_SILO_ERROR_END: u32 = 4607u32;
pub const ES_PW_SILO_ERROR_START: u32 = 4352u32;
pub const ES_RESERVED_COM_ERROR_END: u32 = 511u32;
pub const ES_RESERVED_COM_ERROR_START: u32 = 0u32;
pub const ES_RESERVED_SILO_ERROR_END: u32 = 4095u32;
pub const ES_RESERVED_SILO_ERROR_START: u32 = 1280u32;
pub const ES_RESERVED_SILO_SPECIFIC_ERROR_END: u32 = 49151u32;
pub const ES_RESERVED_SILO_SPECIFIC_ERROR_START: u32 = 4608u32;
pub const ES_VENDOR_ERROR_END: u32 = 65535u32;
pub const ES_VENDOR_ERROR_START: u32 = 49152u32;
pub const EnhancedStorageACT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaf076a15_2ece_4ad4_bb21_29f040e176d8);
pub const EnhancedStorageSilo: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcb25220c_76c7_4fee_842b_f3383cd022bc);
pub const EnhancedStorageSiloAction: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x886d29dd_b506_466b_9fbf_b44ff383fb3f);
pub const EnumEnhancedStorageACT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfe841493_835c_4fa3_b6cc_b4b2d4719848);
pub const FACILITY_ENHANCED_STORAGE: u32 = 4u32;
pub const FILEOFFLINEAVAILABILITYSTATUS_COMPLETE: u32 = 2u32;
pub const FILEOFFLINEAVAILABILITYSTATUS_COMPLETE_PINNED: u32 = 3u32;
pub const FILEOFFLINEAVAILABILITYSTATUS_EXCLUDED: u32 = 4u32;
pub const FILEOFFLINEAVAILABILITYSTATUS_FOLDER_EMPTY: u32 = 5u32;
pub const FILEOFFLINEAVAILABILITYSTATUS_NOTAVAILABLEOFFLINE: u32 = 0u32;
pub const FILEOFFLINEAVAILABILITYSTATUS_PARTIAL: u32 = 1u32;
pub const FLAGSTATUS_COMPLETED: i32 = 1i32;
pub const FLAGSTATUS_FOLLOWUP: i32 = 2i32;
pub const FLAGSTATUS_NOTFLAGGED: i32 = 0i32;
pub const GUID_DEVINTERFACE_ENHANCED_STORAGE_SILO: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3897f6a4_fd35_4bc8_a0b7_5dbba36adafa);
#[repr(transparent)]
pub struct IEnhancedStorageACT(::windows_core::IUnknown);
impl IEnhancedStorageACT {
    pub unsafe fn Authorize(&self, hwndparent: u32, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Authorize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hwndparent), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn Unauthorize(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Unauthorize)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetAuthorizationState(&self) -> ::windows_core::Result<ACT_AUTHORIZATION_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::<ACT_AUTHORIZATION_STATE>::zeroed();
        (::windows_core::Interface::vtable(self).GetAuthorizationState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ACT_AUTHORIZATION_STATE>(result__)
    }
    pub unsafe fn GetMatchingVolume(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetMatchingVolume)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetUniqueIdentity(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetUniqueIdentity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetSilos(&self, pppienhancedstoragesilos: *mut *mut ::core::option::Option<IEnhancedStorageSilo>, pcenhancedstoragesilos: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSilos)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pppienhancedstoragesilos), ::core::mem::transmute(pcenhancedstoragesilos)).ok()
    }
}
impl ::core::convert::From<IEnhancedStorageACT> for ::windows_core::IUnknown {
    fn from(value: IEnhancedStorageACT) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnhancedStorageACT> for ::windows_core::IUnknown {
    fn from(value: &IEnhancedStorageACT) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnhancedStorageACT {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnhancedStorageACT {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnhancedStorageACT {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnhancedStorageACT {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnhancedStorageACT {}
impl ::core::fmt::Debug for IEnhancedStorageACT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnhancedStorageACT").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnhancedStorageACT {
    type Vtable = IEnhancedStorageACT_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6e7781f4_e0f2_4239_b976_a01abab52930);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnhancedStorageACT_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Authorize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: u32, dwflags: u32) -> ::windows_core::HRESULT,
    pub Unauthorize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetAuthorizationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut ACT_AUTHORIZATION_STATE) -> ::windows_core::HRESULT,
    pub GetMatchingVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszvolume: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetUniqueIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszidentity: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetSilos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppienhancedstoragesilos: *mut *mut ::windows_core::RawPtr, pcenhancedstoragesilos: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnhancedStorageACT2(::windows_core::IUnknown);
impl IEnhancedStorageACT2 {
    pub unsafe fn Authorize(&self, hwndparent: u32, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Authorize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hwndparent), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn Unauthorize(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Unauthorize)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetAuthorizationState(&self) -> ::windows_core::Result<ACT_AUTHORIZATION_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::<ACT_AUTHORIZATION_STATE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetAuthorizationState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ACT_AUTHORIZATION_STATE>(result__)
    }
    pub unsafe fn GetMatchingVolume(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetMatchingVolume)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetUniqueIdentity(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetUniqueIdentity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetSilos(&self, pppienhancedstoragesilos: *mut *mut ::core::option::Option<IEnhancedStorageSilo>, pcenhancedstoragesilos: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetSilos)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pppienhancedstoragesilos), ::core::mem::transmute(pcenhancedstoragesilos)).ok()
    }
    pub unsafe fn GetDeviceName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetDeviceName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn IsDeviceRemovable(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).IsDeviceRemovable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IEnhancedStorageACT2> for ::windows_core::IUnknown {
    fn from(value: IEnhancedStorageACT2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnhancedStorageACT2> for ::windows_core::IUnknown {
    fn from(value: &IEnhancedStorageACT2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnhancedStorageACT2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnhancedStorageACT2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IEnhancedStorageACT2> for IEnhancedStorageACT {
    fn from(value: IEnhancedStorageACT2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnhancedStorageACT2> for IEnhancedStorageACT {
    fn from(value: &IEnhancedStorageACT2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IEnhancedStorageACT> for IEnhancedStorageACT2 {
    fn into_param(self) -> ::windows_core::Param<'a, IEnhancedStorageACT> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IEnhancedStorageACT> for &'a IEnhancedStorageACT2 {
    fn into_param(self) -> ::windows_core::Param<'a, IEnhancedStorageACT> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnhancedStorageACT2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnhancedStorageACT2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnhancedStorageACT2 {}
impl ::core::fmt::Debug for IEnhancedStorageACT2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnhancedStorageACT2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnhancedStorageACT2 {
    type Vtable = IEnhancedStorageACT2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4da57d2e_8eb3_41f6_a07e_98b52b88242b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnhancedStorageACT2_Vtbl {
    pub base__: IEnhancedStorageACT_Vtbl,
    pub GetDeviceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszdevicename: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub IsDeviceRemovable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisdeviceremovable: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnhancedStorageACT3(::windows_core::IUnknown);
impl IEnhancedStorageACT3 {
    pub unsafe fn Authorize(&self, hwndparent: u32, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Authorize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hwndparent), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn Unauthorize(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Unauthorize)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetAuthorizationState(&self) -> ::windows_core::Result<ACT_AUTHORIZATION_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::<ACT_AUTHORIZATION_STATE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetAuthorizationState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ACT_AUTHORIZATION_STATE>(result__)
    }
    pub unsafe fn GetMatchingVolume(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetMatchingVolume)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetUniqueIdentity(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetUniqueIdentity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetSilos(&self, pppienhancedstoragesilos: *mut *mut ::core::option::Option<IEnhancedStorageSilo>, pcenhancedstoragesilos: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetSilos)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pppienhancedstoragesilos), ::core::mem::transmute(pcenhancedstoragesilos)).ok()
    }
    pub unsafe fn GetDeviceName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDeviceName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn IsDeviceRemovable(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsDeviceRemovable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn UnauthorizeEx(&self, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnauthorizeEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn IsQueueFrozen(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).IsQueueFrozen)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn GetShellExtSupport(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetShellExtSupport)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IEnhancedStorageACT3> for ::windows_core::IUnknown {
    fn from(value: IEnhancedStorageACT3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnhancedStorageACT3> for ::windows_core::IUnknown {
    fn from(value: &IEnhancedStorageACT3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnhancedStorageACT3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnhancedStorageACT3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IEnhancedStorageACT3> for IEnhancedStorageACT {
    fn from(value: IEnhancedStorageACT3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnhancedStorageACT3> for IEnhancedStorageACT {
    fn from(value: &IEnhancedStorageACT3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IEnhancedStorageACT> for IEnhancedStorageACT3 {
    fn into_param(self) -> ::windows_core::Param<'a, IEnhancedStorageACT> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IEnhancedStorageACT> for &'a IEnhancedStorageACT3 {
    fn into_param(self) -> ::windows_core::Param<'a, IEnhancedStorageACT> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IEnhancedStorageACT3> for IEnhancedStorageACT2 {
    fn from(value: IEnhancedStorageACT3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnhancedStorageACT3> for IEnhancedStorageACT2 {
    fn from(value: &IEnhancedStorageACT3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IEnhancedStorageACT2> for IEnhancedStorageACT3 {
    fn into_param(self) -> ::windows_core::Param<'a, IEnhancedStorageACT2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IEnhancedStorageACT2> for &'a IEnhancedStorageACT3 {
    fn into_param(self) -> ::windows_core::Param<'a, IEnhancedStorageACT2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnhancedStorageACT3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnhancedStorageACT3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnhancedStorageACT3 {}
impl ::core::fmt::Debug for IEnhancedStorageACT3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnhancedStorageACT3").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnhancedStorageACT3 {
    type Vtable = IEnhancedStorageACT3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x022150a1_113d_11df_bb61_001aa01bbc58);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnhancedStorageACT3_Vtbl {
    pub base__: IEnhancedStorageACT2_Vtbl,
    pub UnauthorizeEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT,
    pub IsQueueFrozen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisqueuefrozen: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetShellExtSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshellextsupport: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnhancedStorageSilo(::windows_core::IUnknown);
impl IEnhancedStorageSilo {
    pub unsafe fn GetInfo(&self) -> ::windows_core::Result<SILO_INFO> {
        let mut result__ = ::core::mem::MaybeUninit::<SILO_INFO>::zeroed();
        (::windows_core::Interface::vtable(self).GetInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<SILO_INFO>(result__)
    }
    pub unsafe fn GetActions(&self, pppienhancedstoragesiloactions: *mut *mut ::core::option::Option<IEnhancedStorageSiloAction>, pcenhancedstoragesiloactions: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetActions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pppienhancedstoragesiloactions), ::core::mem::transmute(pcenhancedstoragesiloactions)).ok()
    }
    pub unsafe fn SendCommand(&self, command: u8, pbcommandbuffer: &[u8], pbresponsebuffer: *mut u8, pcbresponsebuffer: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendCommand)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(command), ::core::mem::transmute(::windows_core::as_ptr_or_null(pbcommandbuffer)), pbcommandbuffer.len() as _, ::core::mem::transmute(pbresponsebuffer), ::core::mem::transmute(pcbresponsebuffer)).ok()
    }
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub unsafe fn GetPortableDevice(&self) -> ::windows_core::Result<::win32_devices::PortableDevices::IPortableDevice> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetPortableDevice)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_devices::PortableDevices::IPortableDevice>(result__)
    }
    pub unsafe fn GetDevicePath(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetDevicePath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IEnhancedStorageSilo> for ::windows_core::IUnknown {
    fn from(value: IEnhancedStorageSilo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnhancedStorageSilo> for ::windows_core::IUnknown {
    fn from(value: &IEnhancedStorageSilo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnhancedStorageSilo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnhancedStorageSilo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnhancedStorageSilo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnhancedStorageSilo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnhancedStorageSilo {}
impl ::core::fmt::Debug for IEnhancedStorageSilo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnhancedStorageSilo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnhancedStorageSilo {
    type Vtable = IEnhancedStorageSilo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5aef78c6_2242_4703_bf49_44b29357a359);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnhancedStorageSilo_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psiloinfo: *mut SILO_INFO) -> ::windows_core::HRESULT,
    pub GetActions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppienhancedstoragesiloactions: *mut *mut ::windows_core::RawPtr, pcenhancedstoragesiloactions: *mut u32) -> ::windows_core::HRESULT,
    pub SendCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, command: u8, pbcommandbuffer: *const u8, cbcommandbuffer: u32, pbresponsebuffer: *mut u8, pcbresponsebuffer: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    pub GetPortableDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiportabledevice: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Devices_PortableDevices"))]
    GetPortableDevice: usize,
    pub GetDevicePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszsilodevicepath: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnhancedStorageSiloAction(::windows_core::IUnknown);
impl IEnhancedStorageSiloAction {
    pub unsafe fn GetName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetDescription(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn Invoke(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Invoke)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IEnhancedStorageSiloAction> for ::windows_core::IUnknown {
    fn from(value: IEnhancedStorageSiloAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnhancedStorageSiloAction> for ::windows_core::IUnknown {
    fn from(value: &IEnhancedStorageSiloAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnhancedStorageSiloAction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnhancedStorageSiloAction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnhancedStorageSiloAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnhancedStorageSiloAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnhancedStorageSiloAction {}
impl ::core::fmt::Debug for IEnhancedStorageSiloAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnhancedStorageSiloAction").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnhancedStorageSiloAction {
    type Vtable = IEnhancedStorageSiloAction_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb6f7f311_206f_4ff8_9c4b_27efee77a86f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnhancedStorageSiloAction_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszactionname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszactiondescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumEnhancedStorageACT(::windows_core::IUnknown);
impl IEnumEnhancedStorageACT {
    pub unsafe fn GetACTs(&self, pppienhancedstorageacts: *mut *mut ::core::option::Option<IEnhancedStorageACT>, pcenhancedstorageacts: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetACTs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pppienhancedstorageacts), ::core::mem::transmute(pcenhancedstorageacts)).ok()
    }
    pub unsafe fn GetMatchingACT<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, szvolume: Param0) -> ::windows_core::Result<IEnhancedStorageACT> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetMatchingACT)(::windows_core::Interface::as_raw(self), szvolume.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnhancedStorageACT>(result__)
    }
}
impl ::core::convert::From<IEnumEnhancedStorageACT> for ::windows_core::IUnknown {
    fn from(value: IEnumEnhancedStorageACT) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumEnhancedStorageACT> for ::windows_core::IUnknown {
    fn from(value: &IEnumEnhancedStorageACT) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumEnhancedStorageACT {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumEnhancedStorageACT {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumEnhancedStorageACT {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumEnhancedStorageACT {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumEnhancedStorageACT {}
impl ::core::fmt::Debug for IEnumEnhancedStorageACT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumEnhancedStorageACT").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumEnhancedStorageACT {
    type Vtable = IEnumEnhancedStorageACT_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x09b224bd_1335_4631_a7ff_cfd3a92646d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumEnhancedStorageACT_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetACTs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppienhancedstorageacts: *mut *mut ::windows_core::RawPtr, pcenhancedstorageacts: *mut u32) -> ::windows_core::HRESULT,
    pub GetMatchingACT: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szvolume: ::windows_core::PCWSTR, ppienhancedstorageact: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
pub const IMPORTANCE_HIGH_MAX: i32 = 5i32;
pub const IMPORTANCE_HIGH_MIN: i32 = 5i32;
pub const IMPORTANCE_HIGH_SET: i32 = 5i32;
pub const IMPORTANCE_LOW_MAX: i32 = 1i32;
pub const IMPORTANCE_LOW_MIN: i32 = 0i32;
pub const IMPORTANCE_LOW_SET: i32 = 1i32;
pub const IMPORTANCE_NORMAL_MAX: i32 = 4i32;
pub const IMPORTANCE_NORMAL_MIN: i32 = 2i32;
pub const IMPORTANCE_NORMAL_SET: i32 = 3i32;
pub const ISDEFAULTSAVE_BOTH: u32 = 3u32;
pub const ISDEFAULTSAVE_NONE: u32 = 0u32;
pub const ISDEFAULTSAVE_NONOWNER: u32 = 2u32;
pub const ISDEFAULTSAVE_OWNER: u32 = 1u32;
pub const KIND_CALENDAR: &str = "calendar";
pub const KIND_COMMUNICATION: &str = "communication";
pub const KIND_CONTACT: &str = "contact";
pub const KIND_DOCUMENT: &str = "document";
pub const KIND_EMAIL: &str = "email";
pub const KIND_FEED: &str = "feed";
pub const KIND_FOLDER: &str = "folder";
pub const KIND_GAME: &str = "game";
pub const KIND_INSTANTMESSAGE: &str = "instantmessage";
pub const KIND_JOURNAL: &str = "journal";
pub const KIND_LINK: &str = "link";
pub const KIND_MOVIE: &str = "movie";
pub const KIND_MUSIC: &str = "music";
pub const KIND_NOTE: &str = "note";
pub const KIND_PICTURE: &str = "picture";
pub const KIND_PLAYLIST: &str = "playlist";
pub const KIND_PROGRAM: &str = "program";
pub const KIND_RECORDEDTV: &str = "recordedtv";
pub const KIND_SEARCHFOLDER: &str = "searchfolder";
pub const KIND_TASK: &str = "task";
pub const KIND_UNKNOWN: &str = "unknown";
pub const KIND_VIDEO: &str = "video";
pub const KIND_WEBHISTORY: &str = "webhistory";
pub const LAYOUTPATTERN_CVMFB_ALPHA: &str = "alpha";
pub const LAYOUTPATTERN_CVMFB_BETA: &str = "beta";
pub const LAYOUTPATTERN_CVMFB_DELTA: &str = "delta";
pub const LAYOUTPATTERN_CVMFB_GAMMA: &str = "gamma";
pub const LAYOUTPATTERN_CVMFS_ALPHA: &str = "alpha";
pub const LAYOUTPATTERN_CVMFS_BETA: &str = "beta";
pub const LAYOUTPATTERN_CVMFS_DELTA: &str = "delta";
pub const LAYOUTPATTERN_CVMFS_GAMMA: &str = "gamma";
pub const LINK_STATUS_BROKEN: i32 = 2i32;
pub const LINK_STATUS_RESOLVED: i32 = 1i32;
pub const OFFLINEAVAILABILITY_ALWAYS_AVAILABLE: u32 = 2u32;
pub const OFFLINEAVAILABILITY_AVAILABLE: u32 = 1u32;
pub const OFFLINEAVAILABILITY_NOT_AVAILABLE: u32 = 0u32;
pub const OFFLINESTATUS_OFFLINE: u32 = 1u32;
pub const OFFLINESTATUS_OFFLINE_ERROR: u32 = 4u32;
pub const OFFLINESTATUS_OFFLINE_FORCED: u32 = 2u32;
pub const OFFLINESTATUS_OFFLINE_ITEM_VERSION_CONFLICT: u32 = 5u32;
pub const OFFLINESTATUS_OFFLINE_SLOW: u32 = 3u32;
pub const OFFLINESTATUS_OFFLINE_SUSPENDED: u32 = 6u32;
pub const OFFLINESTATUS_ONLINE: u32 = 0u32;
pub const PHOTO_CONTRAST_HARD: u32 = 2u32;
pub const PHOTO_CONTRAST_NORMAL: u32 = 0u32;
pub const PHOTO_CONTRAST_SOFT: u32 = 1u32;
pub const PHOTO_EXPOSUREPROGRAM_ACTION: u32 = 6u32;
pub const PHOTO_EXPOSUREPROGRAM_APERTURE: u32 = 3u32;
pub const PHOTO_EXPOSUREPROGRAM_CREATIVE: u32 = 5u32;
pub const PHOTO_EXPOSUREPROGRAM_LANDSCAPE: u32 = 8u32;
pub const PHOTO_EXPOSUREPROGRAM_MANUAL: u32 = 1u32;
pub const PHOTO_EXPOSUREPROGRAM_NORMAL: u32 = 2u32;
pub const PHOTO_EXPOSUREPROGRAM_PORTRAIT: u32 = 7u32;
pub const PHOTO_EXPOSUREPROGRAM_SHUTTER: u32 = 4u32;
pub const PHOTO_EXPOSUREPROGRAM_UNKNOWN: u32 = 0u32;
pub const PHOTO_FLASH_FLASH: u32 = 1u32;
pub const PHOTO_FLASH_FLASH_AUTO: u32 = 25u32;
pub const PHOTO_FLASH_FLASH_AUTO_NORETURNLIGHT: u32 = 29u32;
pub const PHOTO_FLASH_FLASH_AUTO_REDEYE: u32 = 89u32;
pub const PHOTO_FLASH_FLASH_AUTO_REDEYE_NORETURNLIGHT: u32 = 93u32;
pub const PHOTO_FLASH_FLASH_AUTO_REDEYE_RETURNLIGHT: u32 = 95u32;
pub const PHOTO_FLASH_FLASH_AUTO_RETURNLIGHT: u32 = 31u32;
pub const PHOTO_FLASH_FLASH_COMPULSORY: u32 = 9u32;
pub const PHOTO_FLASH_FLASH_COMPULSORY_NORETURNLIGHT: u32 = 13u32;
pub const PHOTO_FLASH_FLASH_COMPULSORY_REDEYE: u32 = 73u32;
pub const PHOTO_FLASH_FLASH_COMPULSORY_REDEYE_NORETURNLIGHT: u32 = 77u32;
pub const PHOTO_FLASH_FLASH_COMPULSORY_REDEYE_RETURNLIGHT: u32 = 79u32;
pub const PHOTO_FLASH_FLASH_COMPULSORY_RETURNLIGHT: u32 = 15u32;
pub const PHOTO_FLASH_FLASH_REDEYE: u32 = 65u32;
pub const PHOTO_FLASH_FLASH_REDEYE_NORETURNLIGHT: u32 = 69u32;
pub const PHOTO_FLASH_FLASH_REDEYE_RETURNLIGHT: u32 = 71u32;
pub const PHOTO_FLASH_NOFUNCTION: u32 = 32u32;
pub const PHOTO_FLASH_NONE: u32 = 0u32;
pub const PHOTO_FLASH_NONE_AUTO: u32 = 24u32;
pub const PHOTO_FLASH_NONE_COMPULSORY: u32 = 16u32;
pub const PHOTO_FLASH_WITHOUTSTROBE: u32 = 5u32;
pub const PHOTO_FLASH_WITHSTROBE: u32 = 7u32;
pub const PHOTO_GAINCONTROL_HIGHGAINDOWN: f64 = 4f64;
pub const PHOTO_GAINCONTROL_HIGHGAINUP: f64 = 2f64;
pub const PHOTO_GAINCONTROL_LOWGAINDOWN: f64 = 3f64;
pub const PHOTO_GAINCONTROL_LOWGAINUP: f64 = 1f64;
pub const PHOTO_GAINCONTROL_NONE: f64 = 0f64;
pub const PHOTO_LIGHTSOURCE_D55: u32 = 20u32;
pub const PHOTO_LIGHTSOURCE_D65: u32 = 21u32;
pub const PHOTO_LIGHTSOURCE_D75: u32 = 22u32;
pub const PHOTO_LIGHTSOURCE_DAYLIGHT: u32 = 1u32;
pub const PHOTO_LIGHTSOURCE_FLUORESCENT: u32 = 2u32;
pub const PHOTO_LIGHTSOURCE_STANDARD_A: u32 = 17u32;
pub const PHOTO_LIGHTSOURCE_STANDARD_B: u32 = 18u32;
pub const PHOTO_LIGHTSOURCE_STANDARD_C: u32 = 19u32;
pub const PHOTO_LIGHTSOURCE_TUNGSTEN: u32 = 3u32;
pub const PHOTO_LIGHTSOURCE_UNKNOWN: u32 = 0u32;
pub const PHOTO_PROGRAMMODE_ACTION: u32 = 6u32;
pub const PHOTO_PROGRAMMODE_APERTURE: u32 = 3u32;
pub const PHOTO_PROGRAMMODE_CREATIVE: u32 = 5u32;
pub const PHOTO_PROGRAMMODE_LANDSCAPE: u32 = 8u32;
pub const PHOTO_PROGRAMMODE_MANUAL: u32 = 1u32;
pub const PHOTO_PROGRAMMODE_NORMAL: u32 = 2u32;
pub const PHOTO_PROGRAMMODE_NOTDEFINED: u32 = 0u32;
pub const PHOTO_PROGRAMMODE_PORTRAIT: u32 = 7u32;
pub const PHOTO_PROGRAMMODE_SHUTTER: u32 = 4u32;
pub const PHOTO_SATURATION_HIGH: u32 = 2u32;
pub const PHOTO_SATURATION_LOW: u32 = 1u32;
pub const PHOTO_SATURATION_NORMAL: u32 = 0u32;
pub const PHOTO_SHARPNESS_HARD: u32 = 2u32;
pub const PHOTO_SHARPNESS_NORMAL: u32 = 0u32;
pub const PHOTO_SHARPNESS_SOFT: u32 = 1u32;
pub const PHOTO_WHITEBALANCE_AUTO: u32 = 0u32;
pub const PHOTO_WHITEBALANCE_MANUAL: u32 = 1u32;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AcquisitionID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x65a98875_3c80_40ab_abbc_efdaf77dbee2), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Address_Country: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc07b4199_e1df_4493_b1e1_de5946fb58f8), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Address_CountryCode: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc07b4199_e1df_4493_b1e1_de5946fb58f8), pid: 101u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Address_Region: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc07b4199_e1df_4493_b1e1_de5946fb58f8), pid: 102u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Address_RegionCode: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc07b4199_e1df_4493_b1e1_de5946fb58f8), pid: 103u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Address_Town: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc07b4199_e1df_4493_b1e1_de5946fb58f8), pid: 104u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AppUserModel_ExcludeFromShowInNewInstall: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9f4c2855_9f79_4b39_a8d0_e1d42de1d5f3), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AppUserModel_ID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9f4c2855_9f79_4b39_a8d0_e1d42de1d5f3), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AppUserModel_IsDestListSeparator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9f4c2855_9f79_4b39_a8d0_e1d42de1d5f3), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AppUserModel_IsDualMode: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9f4c2855_9f79_4b39_a8d0_e1d42de1d5f3), pid: 11u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AppUserModel_PreventPinning: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9f4c2855_9f79_4b39_a8d0_e1d42de1d5f3), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AppUserModel_RelaunchCommand: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9f4c2855_9f79_4b39_a8d0_e1d42de1d5f3), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AppUserModel_RelaunchDisplayNameResource: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9f4c2855_9f79_4b39_a8d0_e1d42de1d5f3), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AppUserModel_RelaunchIconResource: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9f4c2855_9f79_4b39_a8d0_e1d42de1d5f3), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AppUserModel_SettingsCommand: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9f4c2855_9f79_4b39_a8d0_e1d42de1d5f3), pid: 38u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AppUserModel_StartPinOption: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9f4c2855_9f79_4b39_a8d0_e1d42de1d5f3), pid: 12u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AppUserModel_ToastActivatorCLSID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9f4c2855_9f79_4b39_a8d0_e1d42de1d5f3), pid: 26u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AppUserModel_UninstallCommand: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9f4c2855_9f79_4b39_a8d0_e1d42de1d5f3), pid: 37u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AppUserModel_VisualElementsManifestHintPath: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9f4c2855_9f79_4b39_a8d0_e1d42de1d5f3), pid: 31u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AppZoneIdentifier: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x502cfeab_47eb_459c_b960_e6d8728f7701), pid: 102u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ApplicationDefinedProperties: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xcdbfc167_337e_41d8_af7c_8c09205429c7), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ApplicationName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf29f85e0_4ff9_1068_ab91_08002b27b3d9), pid: 18u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Audio_ChannelCount: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440490_4c8b_11d1_8b70_080036b11a03), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Audio_Compression: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440490_4c8b_11d1_8b70_080036b11a03), pid: 10u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Audio_EncodingBitrate: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440490_4c8b_11d1_8b70_080036b11a03), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Audio_Format: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440490_4c8b_11d1_8b70_080036b11a03), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Audio_IsVariableBitRate: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe6822fee_8c17_4d62_823c_8e9cfcbd1d5c), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Audio_PeakValue: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2579e5d0_1116_4084_bd9a_9b4f7cb4df5e), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Audio_SampleRate: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440490_4c8b_11d1_8b70_080036b11a03), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Audio_SampleSize: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440490_4c8b_11d1_8b70_080036b11a03), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Audio_StreamName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440490_4c8b_11d1_8b70_080036b11a03), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Audio_StreamNumber: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440490_4c8b_11d1_8b70_080036b11a03), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Author: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf29f85e0_4ff9_1068_ab91_08002b27b3d9), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CachedFileUpdaterContentIdForConflictResolution: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfceff153_e839_4cf3_a9e7_ea22832094b8), pid: 114u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CachedFileUpdaterContentIdForStream: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfceff153_e839_4cf3_a9e7_ea22832094b8), pid: 113u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Calendar_Duration: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x293ca35a_09aa_4dd2_b180_1fe245728a52), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Calendar_IsOnline: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xbfee9149_e3e2_49a7_a862_c05988145cec), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Calendar_IsRecurring: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x315b9c8d_80a9_4ef9_ae16_8e746da51d70), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Calendar_Location: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf6272d18_cecc_40b1_b26a_3911717aa7bd), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Calendar_OptionalAttendeeAddresses: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd55bae5a_3892_417a_a649_c6ac5aaaeab3), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Calendar_OptionalAttendeeNames: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x09429607_582d_437f_84c3_de93a2b24c3c), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Calendar_OrganizerAddress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x744c8242_4df5_456c_ab9e_014efb9021e3), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Calendar_OrganizerName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xaaa660f9_9865_458e_b484_01bc7fe3973e), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Calendar_ReminderTime: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x72fc5ba4_24f9_4011_9f3f_add27afad818), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Calendar_RequiredAttendeeAddresses: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0ba7d6c3_568d_4159_ab91_781a91fb71e5), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Calendar_RequiredAttendeeNames: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb33af30b_f552_4584_936c_cb93e5cda29f), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Calendar_Resources: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f58a38_c54b_4c40_8696_97235980eae1), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Calendar_ResponseStatus: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x188c1f91_3c40_4132_9ec5_d8b03b72a8a2), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Calendar_ShowTimeAs: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5bf396d4_5eb2_466f_bde9_2fb3f2361d6e), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Calendar_ShowTimeAsText: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x53da57cf_62c0_45c4_81de_7610bcefd7f5), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Capacity: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9b174b35_40ff_11d2_a27e_00c04fc30871), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Category: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd5cdd502_2e9c_101b_9397_08002b2cf9ae), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Comment: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf29f85e0_4ff9_1068_ab91_08002b27b3d9), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Communication_AccountName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe3e0584c_b788_4a5a_bb20_7f5a44c9acdd), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Communication_DateItemExpires: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x428040ac_a177_4c8a_9760_f6f761227f9a), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Communication_Direction: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8e531030_b960_4346_ae0d_66bc9a86fb94), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Communication_FollowupIconIndex: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x83a6347e_6fe4_4f40_ba9c_c4865240d1f4), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Communication_HeaderItem: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc9c34f84_2241_4401_b607_bd20ed75ae7f), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Communication_PolicyTag: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xec0b4191_ab0b_4c66_90b6_c6637cdebbab), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Communication_SecurityFlags: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8619a4b6_9f4d_4429_8c0f_b996ca59e335), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Communication_Suffix: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x807b653a_9e91_43ef_8f97_11ce04ee20c5), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Communication_TaskStatus: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xbe1a72c6_9a1d_46b7_afe7_afaf8cef4999), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Communication_TaskStatusText: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa6744477_c237_475b_a075_54f34498292a), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Company: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd5cdd502_2e9c_101b_9397_08002b2cf9ae), pid: 15u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ComputerName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x28636aa6_953d_11d2_b5d6_00c04fd918d0), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Computer_DecoratedFreeSpace: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9b174b35_40ff_11d2_a27e_00c04fc30871), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_AccountPictureDynamicVideo: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0b8bb018_2725_4b44_92ba_7933aeb2dde7), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_AccountPictureLarge: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0b8bb018_2725_4b44_92ba_7933aeb2dde7), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_AccountPictureSmall: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0b8bb018_2725_4b44_92ba_7933aeb2dde7), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_Anniversary: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9ad5badb_cea7_4470_a03d_b84e51b9949e), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_AssistantName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xcd102c9c_5540_4a88_a6f6_64e4981c8cd1), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_AssistantTelephone: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9a93244d_a7ad_4ff8_9b99_45ee4cc09af6), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_Birthday: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x176dc63c_2688_4e89_8143_a347800f25e9), pid: 47u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x730fb6dd_cf7c_426b_a03f_bd166cc9ee24), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress1Country: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 119u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress1Locality: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 117u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress1PostalCode: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 120u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress1Region: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 118u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress1Street: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 116u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress2Country: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 124u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress2Locality: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 122u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress2PostalCode: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 125u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress2Region: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 123u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress2Street: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 121u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress3Country: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 129u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress3Locality: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 127u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress3PostalCode: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 130u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress3Region: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 128u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddress3Street: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 126u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddressCity: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x402b5934_ec5a_48c3_93e6_85e86a2d934e), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddressCountry: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb0b87314_fcf6_4feb_8dff_a50da6af561c), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddressPostOfficeBox: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xbc4e71ce_17f9_48d5_bee9_021df0ea5409), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddressPostalCode: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe1d4a09e_d758_4cd1_b6ec_34a8b5a73f80), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddressState: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x446f787f_10c4_41cb_a6c4_4d0343551597), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessAddressStreet: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xddd1460f_c0bf_4553_8ce4_10433c908fb0), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessEmailAddresses: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf271c659_7e5e_471f_ba25_7f77b286f836), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessFaxNumber: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x91eff6f3_2e27_42ca_933e_7c999fbe310b), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessHomePage: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x56310920_2491_4919_99ce_eadb06fafdb2), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_BusinessTelephone: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6a15e5a0_0a1e_4cd7_bb8c_d2f1b0c929bc), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_CallbackTelephone: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xbf53d1c3_49e0_4f7f_8567_5a821d8ac542), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_CarTelephone: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8fdc6dea_b929_412b_ba90_397a257465fe), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_Children: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd4729704_8ef1_43ef_9024_2bd381187fd5), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_CompanyMainTelephone: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8589e481_6040_473d_b171_7fa89c2708ed), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_ConnectedServiceDisplayName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x39b77f4f_a104_4863_b395_2db2ad8f7bc1), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_ConnectedServiceIdentities: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x80f41eb8_afc4_4208_aa5f_cce21a627281), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_ConnectedServiceName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb5c84c9e_5927_46b5_a3cc_933c21b78469), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_ConnectedServiceSupportedActions: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa19fb7a9_024b_4371_a8bf_4d29c3e4e9c9), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_DataSuppliers: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9660c283_fc3a_4a08_a096_eed3aac46da2), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_Department: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfc9f7306_ff8f_4d49_9fb6_3ffe5c0951ec), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_DisplayBusinessPhoneNumbers: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x364028da_d895_41fe_a584_302b1bb70a76), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_DisplayHomePhoneNumbers: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5068bcdf_d697_4d85_8c53_1f1cdab01763), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_DisplayMobilePhoneNumbers: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9cb0c358_9d7a_46b1_b466_dcc6f1a3d93d), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_DisplayOtherPhoneNumbers: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x03089873_8ee8_4191_bd60_d31f72b7900b), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_EmailAddress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf8fa7fa3_d12b_4785_8a4e_691a94f7a3e7), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_EmailAddress2: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x38965063_edc8_4268_8491_b7723172cf29), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_EmailAddress3: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x644d37b4_e1b3_4bad_b099_7e7c04966aca), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_EmailAddresses: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x84d8f337_981d_44b3_9615_c7596dba17e3), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_EmailName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xcc6f4f24_6083_4bd4_8754_674d0de87ab8), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_FileAsName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf1a24aa7_9ca7_40f6_89ec_97def9ffe8db), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_FirstName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x14977844_6b49_4aad_a714_a4513bf60460), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_FullName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x635e9051_50a5_4ba2_b9db_4ed056c77296), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_Gender: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x3c8cee58_d4f0_4cf9_b756_4e5d24447bcd), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_GenderValue: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x3c8cee58_d4f0_4cf9_b756_4e5d24447bcd), pid: 101u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_Hobbies: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5dc2253f_5e11_4adf_9cfe_910dd01e3e70), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x98f98354_617a_46b8_8560_5b1b64bf1f89), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress1Country: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 104u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress1Locality: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 102u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress1PostalCode: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 105u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress1Region: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 103u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress1Street: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 101u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress2Country: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 109u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress2Locality: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 107u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress2PostalCode: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 110u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress2Region: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 108u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress2Street: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 106u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress3Country: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 114u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress3Locality: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 112u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress3PostalCode: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 115u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress3Region: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 113u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddress3Street: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 111u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddressCity: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x176dc63c_2688_4e89_8143_a347800f25e9), pid: 65u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddressCountry: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x08a65aa1_f4c9_43dd_9ddf_a33d8e7ead85), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddressPostOfficeBox: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x7b9f6399_0a3f_4b12_89bd_4adc51c918af), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddressPostalCode: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8afcc170_8a46_4b53_9eee_90bae7151e62), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddressState: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc89a23d0_7d6d_4eb8_87d4_776a82d493e5), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeAddressStreet: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0adef160_db3f_4308_9a21_06237b16fa2a), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeEmailAddresses: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x56c90e9d_9d46_4963_886f_2e1cd9a694ef), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeFaxNumber: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x660e04d6_81ab_4977_a09f_82313113ab26), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_HomeTelephone: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x176dc63c_2688_4e89_8143_a347800f25e9), pid: 20u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_IMAddress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd68dbd8a_3374_4b81_9972_3ec30682db3d), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_Initials: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf3d8f40d_50cb_44a2_9718_40cb9119495d), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JA_CompanyNamePhonetic: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x897b3694_fe9e_43e6_8066_260f590c0100), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JA_FirstNamePhonetic: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x897b3694_fe9e_43e6_8066_260f590c0100), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JA_LastNamePhonetic: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x897b3694_fe9e_43e6_8066_260f590c0100), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo1CompanyAddress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f63dd8_22bd_4a5d_ba34_5cb0b9bdcb03), pid: 120u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo1CompanyName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f63dd8_22bd_4a5d_ba34_5cb0b9bdcb03), pid: 102u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo1Department: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f63dd8_22bd_4a5d_ba34_5cb0b9bdcb03), pid: 106u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo1Manager: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f63dd8_22bd_4a5d_ba34_5cb0b9bdcb03), pid: 105u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo1OfficeLocation: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f63dd8_22bd_4a5d_ba34_5cb0b9bdcb03), pid: 104u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo1Title: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f63dd8_22bd_4a5d_ba34_5cb0b9bdcb03), pid: 103u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo1YomiCompanyName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f63dd8_22bd_4a5d_ba34_5cb0b9bdcb03), pid: 101u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo2CompanyAddress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f63dd8_22bd_4a5d_ba34_5cb0b9bdcb03), pid: 121u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo2CompanyName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f63dd8_22bd_4a5d_ba34_5cb0b9bdcb03), pid: 108u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo2Department: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f63dd8_22bd_4a5d_ba34_5cb0b9bdcb03), pid: 113u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo2Manager: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f63dd8_22bd_4a5d_ba34_5cb0b9bdcb03), pid: 112u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo2OfficeLocation: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f63dd8_22bd_4a5d_ba34_5cb0b9bdcb03), pid: 110u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo2Title: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f63dd8_22bd_4a5d_ba34_5cb0b9bdcb03), pid: 109u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo2YomiCompanyName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f63dd8_22bd_4a5d_ba34_5cb0b9bdcb03), pid: 107u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo3CompanyAddress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f63dd8_22bd_4a5d_ba34_5cb0b9bdcb03), pid: 123u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo3CompanyName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f63dd8_22bd_4a5d_ba34_5cb0b9bdcb03), pid: 115u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo3Department: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f63dd8_22bd_4a5d_ba34_5cb0b9bdcb03), pid: 119u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo3Manager: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f63dd8_22bd_4a5d_ba34_5cb0b9bdcb03), pid: 118u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo3OfficeLocation: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f63dd8_22bd_4a5d_ba34_5cb0b9bdcb03), pid: 117u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo3Title: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f63dd8_22bd_4a5d_ba34_5cb0b9bdcb03), pid: 116u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobInfo3YomiCompanyName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f63dd8_22bd_4a5d_ba34_5cb0b9bdcb03), pid: 114u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_JobTitle: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x176dc63c_2688_4e89_8143_a347800f25e9), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_Label: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x97b0ad89_df49_49cc_834e_660974fd755b), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_LastName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8f367200_c270_457c_b1d4_e07c5bcd90c7), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_MailingAddress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc0ac206a_827e_4650_95ae_77e2bb74fcc9), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_MiddleName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x176dc63c_2688_4e89_8143_a347800f25e9), pid: 71u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_MobileTelephone: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x176dc63c_2688_4e89_8143_a347800f25e9), pid: 35u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_NickName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x176dc63c_2688_4e89_8143_a347800f25e9), pid: 74u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OfficeLocation: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x176dc63c_2688_4e89_8143_a347800f25e9), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x508161fa_313b_43d5_83a1_c1accf68622c), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress1Country: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 134u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress1Locality: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 132u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress1PostalCode: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 135u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress1Region: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 133u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress1Street: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 131u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress2Country: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 139u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress2Locality: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 137u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress2PostalCode: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 140u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress2Region: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 138u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress2Street: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 136u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress3Country: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 144u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress3Locality: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 142u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress3PostalCode: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 145u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress3Region: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 143u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddress3Street: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7b6f596_d678_4bc1_b05f_0203d27e8aa1), pid: 141u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddressCity: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6e682923_7f7b_4f0c_a337_cfca296687bf), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddressCountry: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8f167568_0aae_4322_8ed9_6055b7b0e398), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddressPostOfficeBox: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8b26ea41_058f_43f6_aecc_4035681ce977), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddressPostalCode: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x95c656c1_2abf_4148_9ed3_9ec602e3b7cd), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddressState: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x71b377d6_e570_425f_a170_809fae73e54e), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherAddressStreet: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xff962609_b7d6_4999_862d_95180d529aea), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_OtherEmailAddresses: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x11d6336b_38c4_4ec9_84d6_eb38d0b150af), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_PagerTelephone: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd6304e01_f8f5_4f45_8b15_d024a6296789), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_PersonalTitle: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x176dc63c_2688_4e89_8143_a347800f25e9), pid: 69u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_PhoneNumbersCanonical: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd042d2a1_927e_40b5_a503_6edbd42a517e), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_Prefix: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x176dc63c_2688_4e89_8143_a347800f25e9), pid: 75u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_PrimaryAddressCity: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc8ea94f0_a9e3_4969_a94b_9c62a95324e0), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_PrimaryAddressCountry: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe53d799d_0f3f_466e_b2ff_74634a3cb7a4), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_PrimaryAddressPostOfficeBox: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xde5ef3c7_46e1_484e_9999_62c5308394c1), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_PrimaryAddressPostalCode: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x18bbd425_ecfd_46ef_b612_7b4a6034eda0), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_PrimaryAddressState: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf1176dfe_7138_4640_8b4c_ae375dc70a6d), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_PrimaryAddressStreet: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x63c25b20_96be_488f_8788_c09c407ad812), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_PrimaryEmailAddress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x176dc63c_2688_4e89_8143_a347800f25e9), pid: 48u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_PrimaryTelephone: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x176dc63c_2688_4e89_8143_a347800f25e9), pid: 25u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_Profession: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x7268af55_1ce4_4f6e_a41f_b6e4ef10e4a9), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_SpouseName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9d2408b6_3167_422b_82b0_f583b7a7cfe3), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_Suffix: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x176dc63c_2688_4e89_8143_a347800f25e9), pid: 73u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_TTYTDDTelephone: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xaaf16bac_2b55_45e6_9f6d_415eb94910df), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_TelexNumber: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc554493c_c1f7_40c1_a76c_ef8c0614003e), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_WebPage: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe3e0584c_b788_4a5a_bb20_7f5a44c9acdd), pid: 18u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_Webpage2: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f63dd8_22bd_4a5d_ba34_5cb0b9bdcb03), pid: 124u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Contact_Webpage3: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00f63dd8_22bd_4a5d_ba34_5cb0b9bdcb03), pid: 125u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ContainedItems: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x28636aa6_953d_11d2_b5d6_00c04fd918d0), pid: 29u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ContentId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfceff153_e839_4cf3_a9e7_ea22832094b8), pid: 132u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ContentStatus: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd5cdd502_2e9c_101b_9397_08002b2cf9ae), pid: 27u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ContentType: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd5cdd502_2e9c_101b_9397_08002b2cf9ae), pid: 26u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ContentUri: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfceff153_e839_4cf3_a9e7_ea22832094b8), pid: 131u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Copyright: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 11u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CreatorAppId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc2ea046e_033c_4e91_bd5b_d4942f6bbe49), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CreatorOpenWithUIOptions: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc2ea046e_033c_4e91_bd5b_d4942f6bbe49), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DRM_DatePlayExpires: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xaeac19e4_89ae_4508_b9b7_bb867abee2ed), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DRM_DatePlayStarts: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xaeac19e4_89ae_4508_b9b7_bb867abee2ed), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DRM_Description: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xaeac19e4_89ae_4508_b9b7_bb867abee2ed), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DRM_IsDisabled: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xaeac19e4_89ae_4508_b9b7_bb867abee2ed), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DRM_IsProtected: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xaeac19e4_89ae_4508_b9b7_bb867abee2ed), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DRM_PlayCount: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xaeac19e4_89ae_4508_b9b7_bb867abee2ed), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DataObjectFormat: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1e81a3f8_a30f_4247_b9ee_1d0368a9425c), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DateAccessed: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb725f130_47ef_101a_a5f1_02608c9eebac), pid: 16u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DateAcquired: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2cbaa8f5_d81f_47ca_b17a_f8d822300131), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DateArchived: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x43f8d7b7_a444_4f87_9383_52271c9b915c), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DateCompleted: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x72fab781_acda_43e5_b155_b2434f85e678), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DateCreated: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb725f130_47ef_101a_a5f1_02608c9eebac), pid: 15u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DateImported: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x14b81da1_0135_4d31_96d9_6cbfc9671a99), pid: 18258u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DateModified: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb725f130_47ef_101a_a5f1_02608c9eebac), pid: 14u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DefaultSaveLocationDisplay: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5d76b67f_9b3d_44bb_b6ae_25da4f638a67), pid: 10u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DescriptionID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x28636aa6_953d_11d2_b5d6_00c04fd918d0), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Bluetooth_DeviceAddress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2bd67d8b_8beb_48d5_87e0_6cda3428040a), pid: 1u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Bluetooth_Flags: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2bd67d8b_8beb_48d5_87e0_6cda3428040a), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Bluetooth_LastConnectedTime: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2bd67d8b_8beb_48d5_87e0_6cda3428040a), pid: 11u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Bluetooth_Manufacturer: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2bd67d8b_8beb_48d5_87e0_6cda3428040a), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Bluetooth_ModelNumber: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2bd67d8b_8beb_48d5_87e0_6cda3428040a), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Bluetooth_ProductId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2bd67d8b_8beb_48d5_87e0_6cda3428040a), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Bluetooth_ProductVersion: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2bd67d8b_8beb_48d5_87e0_6cda3428040a), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Bluetooth_ServiceGuid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2bd67d8b_8beb_48d5_87e0_6cda3428040a), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Bluetooth_VendorId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2bd67d8b_8beb_48d5_87e0_6cda3428040a), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Bluetooth_VendorIdSource: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2bd67d8b_8beb_48d5_87e0_6cda3428040a), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Hid_IsReadOnly: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Hid_ProductId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Hid_UsageId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Hid_UsagePage: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Hid_VendorId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Hid_VersionNumber: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_PrinterDriverDirectory: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x847c66de_b8d6_4af9_abc3_6f4f926bc039), pid: 14u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_PrinterDriverName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xafc47170_14f5_498c_8f30_b0d19be449c6), pid: 11u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_PrinterEnumerationFlag: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa00742a1_cd8c_4b37_95ab_70755587767a), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_PrinterName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0a7b84ef_0c27_463f_84ef_06c5070001be), pid: 10u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_PrinterPortName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xeec7b761_6f94_41b1_949f_c729720dd13c), pid: 12u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Proximity_SupportsNfc: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfb3842cd_9e2a_4f83_8fcc_4b0761139ae9), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Serial_PortName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4c6bf15c_4c03_4aac_91f5_64c0f852bcf4), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Serial_UsbProductId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4c6bf15c_4c03_4aac_91f5_64c0f852bcf4), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Serial_UsbVendorId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4c6bf15c_4c03_4aac_91f5_64c0f852bcf4), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_WinUsb_DeviceInterfaceClasses: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x95e127b5_79cc_4e83_9c9e_8422187b3e0e), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_WinUsb_UsbClass: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x95e127b5_79cc_4e83_9c9e_8422187b3e0e), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_WinUsb_UsbProductId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x95e127b5_79cc_4e83_9c9e_8422187b3e0e), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_WinUsb_UsbProtocol: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x95e127b5_79cc_4e83_9c9e_8422187b3e0e), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_WinUsb_UsbSubClass: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x95e127b5_79cc_4e83_9c9e_8422187b3e0e), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_WinUsb_UsbVendorId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x95e127b5_79cc_4e83_9c9e_8422187b3e0e), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_PrinterURL: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0b48f35a_be6e_4f17_b108_3c4073d1669a), pid: 15u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_CanPair: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0bba1ede_7566_4f47_90ec_25fc567ced2a), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_Categories: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0bba1ede_7566_4f47_90ec_25fc567ced2a), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_Children: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0bba1ede_7566_4f47_90ec_25fc567ced2a), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_ContainerId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0bba1ede_7566_4f47_90ec_25fc567ced2a), pid: 12u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_DialProtocol_InstalledApplications: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6af55d45_38db_4495_acb0_d4728a3b8314), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_IsPaired: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0bba1ede_7566_4f47_90ec_25fc567ced2a), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_IsPresent: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0bba1ede_7566_4f47_90ec_25fc567ced2a), pid: 11u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_Manufacturer: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0bba1ede_7566_4f47_90ec_25fc567ced2a), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_ModelIds: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0bba1ede_7566_4f47_90ec_25fc567ced2a), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_ModelName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0bba1ede_7566_4f47_90ec_25fc567ced2a), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_ProtocolIds: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0bba1ede_7566_4f47_90ec_25fc567ced2a), pid: 13u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_SupportedUriSchemes: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6af55d45_38db_4495_acb0_d4728a3b8314), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_SupportsAudio: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6af55d45_38db_4495_acb0_d4728a3b8314), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_SupportsCapturing: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6af55d45_38db_4495_acb0_d4728a3b8314), pid: 11u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_SupportsImages: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6af55d45_38db_4495_acb0_d4728a3b8314), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_SupportsInformation: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6af55d45_38db_4495_acb0_d4728a3b8314), pid: 14u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_SupportsLimitedDiscovery: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6af55d45_38db_4495_acb0_d4728a3b8314), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_SupportsNetworking: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6af55d45_38db_4495_acb0_d4728a3b8314), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_SupportsObjectTransfer: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6af55d45_38db_4495_acb0_d4728a3b8314), pid: 12u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_SupportsPositioning: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6af55d45_38db_4495_acb0_d4728a3b8314), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_SupportsRendering: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6af55d45_38db_4495_acb0_d4728a3b8314), pid: 10u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_SupportsTelephony: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6af55d45_38db_4495_acb0_d4728a3b8314), pid: 13u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepContainer_SupportsVideo: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6af55d45_38db_4495_acb0_d4728a3b8314), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepService_AepId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc9c141a9_1b4c_4f17_a9d1_f298538cadb8), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepService_Bluetooth_CacheMode: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9744311e_7951_4b2e_b6f0_ecb293cac119), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepService_Bluetooth_ServiceGuid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa399aac7_c265_474e_b073_ffce57721716), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepService_Bluetooth_TargetDevice: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9744311e_7951_4b2e_b6f0_ecb293cac119), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepService_ContainerId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x71724756_3e74_4432_9b59_e7b2f668a593), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepService_FriendlyName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x71724756_3e74_4432_9b59_e7b2f668a593), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepService_IoT_ServiceInterfaces: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x79d94e82_4d79_45aa_821a_74858b4e4ca6), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepService_ParentAepIsPaired: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc9c141a9_1b4c_4f17_a9d1_f298538cadb8), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepService_ProtocolId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc9c141a9_1b4c_4f17_a9d1_f298538cadb8), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepService_ServiceClassId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x71724756_3e74_4432_9b59_e7b2f668a593), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AepService_ServiceId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc9c141a9_1b4c_4f17_a9d1_f298538cadb8), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_AepId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x3b2ce006_5e61_4fde_bab8_9b8aac9b26df), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Major: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5fbd34cd_561a_412e_ba98_478a6b0fef1d), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Minor: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5fbd34cd_561a_412e_ba98_478a6b0fef1d), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Services_Audio: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5fbd34cd_561a_412e_ba98_478a6b0fef1d), pid: 10u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Services_Capturing: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5fbd34cd_561a_412e_ba98_478a6b0fef1d), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Services_Information: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5fbd34cd_561a_412e_ba98_478a6b0fef1d), pid: 12u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Services_LimitedDiscovery: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5fbd34cd_561a_412e_ba98_478a6b0fef1d), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Services_Networking: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5fbd34cd_561a_412e_ba98_478a6b0fef1d), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Services_ObjectXfer: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5fbd34cd_561a_412e_ba98_478a6b0fef1d), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Services_Positioning: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5fbd34cd_561a_412e_ba98_478a6b0fef1d), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Services_Rendering: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5fbd34cd_561a_412e_ba98_478a6b0fef1d), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Services_Telephony: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5fbd34cd_561a_412e_ba98_478a6b0fef1d), pid: 11u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_LastSeenTime: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2bd67d8b_8beb_48d5_87e0_6cda3428040a), pid: 12u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Le_AddressType: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x995ef0b0_7eb3_4a8b_b9ce_068bb3f4af69), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Le_Appearance: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x995ef0b0_7eb3_4a8b_b9ce_068bb3f4af69), pid: 1u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Le_Appearance_Category: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x995ef0b0_7eb3_4a8b_b9ce_068bb3f4af69), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Le_Appearance_Subcategory: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x995ef0b0_7eb3_4a8b_b9ce_068bb3f4af69), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Bluetooth_Le_IsConnectable: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x995ef0b0_7eb3_4a8b_b9ce_068bb3f4af69), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_CanPair: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe7c3fb29_caa7_4f47_8c8b_be59b330d4c5), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Category: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa35996ab_11cf_4935_8b61_a6761081ecdf), pid: 17u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_ContainerId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe7c3fb29_caa7_4f47_8c8b_be59b330d4c5), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_DeviceAddress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa35996ab_11cf_4935_8b61_a6761081ecdf), pid: 12u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_IsConnected: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa35996ab_11cf_4935_8b61_a6761081ecdf), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_IsPaired: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa35996ab_11cf_4935_8b61_a6761081ecdf), pid: 16u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_IsPresent: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa35996ab_11cf_4935_8b61_a6761081ecdf), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_Manufacturer: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa35996ab_11cf_4935_8b61_a6761081ecdf), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_ModelId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa35996ab_11cf_4935_8b61_a6761081ecdf), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_ModelName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa35996ab_11cf_4935_8b61_a6761081ecdf), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_PointOfService_ConnectionTypes: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd4bf61b3_442e_4ada_882d_fa7b70c832d9), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_ProtocolId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x3b2ce006_5e61_4fde_bab8_9b8aac9b26df), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Aep_SignalStrength: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa35996ab_11cf_4935_8b61_a6761081ecdf), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AppPackageFamilyName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x51236583_0c4a_4fe8_b81f_166aec13f510), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AudioDevice_Microphone_IsFarField: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8943b373_388c_4395_b557_bc6dbaffafdb), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AudioDevice_Microphone_SensitivityInDbfs: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8943b373_388c_4395_b557_bc6dbaffafdb), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AudioDevice_Microphone_SensitivityInDbfs2: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8943b373_388c_4395_b557_bc6dbaffafdb), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AudioDevice_Microphone_SignalToNoiseRatioInDb: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8943b373_388c_4395_b557_bc6dbaffafdb), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AudioDevice_RawProcessingSupported: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8943b373_388c_4395_b557_bc6dbaffafdb), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_AudioDevice_SpeechProcessingSupported: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfb1de864_e06d_47f4_82a6_8a0aef44493c), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_BatteryLife: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x49cd1f76_5626_4b17_a4e8_18b4aa1a2213), pid: 10u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_BatteryPlusCharging: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x49cd1f76_5626_4b17_a4e8_18b4aa1a2213), pid: 22u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_BatteryPlusChargingText: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x49cd1f76_5626_4b17_a4e8_18b4aa1a2213), pid: 23u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Category: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 91u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_CategoryGroup: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 94u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_CategoryIds: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 90u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_CategoryPlural: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 92u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_ChallengeAep: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0774315e_b714_48ec_8de8_8125c077ac11), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_ChargingState: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x49cd1f76_5626_4b17_a4e8_18b4aa1a2213), pid: 11u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Children: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_ClassGuid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 10u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_CompatibleIds: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Connected: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 55u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_ContainerId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8c7ed206_3f8a_4827_b3ab_ae9e1faefc6c), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_DefaultTooltip: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x880f70a2_6082_47ac_8aab_a739d1a300c3), pid: 153u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_DevObjectType: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x13673f42_a3d6_49f6_b4da_ae46e0c5237c), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_DeviceCapabilities: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 17u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_DeviceCharacteristics: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 29u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_DeviceDescription1: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 81u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_DeviceDescription2: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 82u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_DeviceHasProblem: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x540b947e_8b40_45bc_a8a2_6a0b894cbda2), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_DeviceInstanceId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 256u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_DeviceManufacturer: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 13u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_DialProtocol_InstalledApplications: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6845cc72_1b71_48c3_af86_b09171a19b14), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_DiscoveryMethod: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 52u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Dnssd_Domain: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xbf79c0ab_bb74_4cee_b070_470b5ae202ea), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Dnssd_FullName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xbf79c0ab_bb74_4cee_b070_470b5ae202ea), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Dnssd_HostName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xbf79c0ab_bb74_4cee_b070_470b5ae202ea), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Dnssd_InstanceName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xbf79c0ab_bb74_4cee_b070_470b5ae202ea), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Dnssd_NetworkAdapterId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xbf79c0ab_bb74_4cee_b070_470b5ae202ea), pid: 11u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Dnssd_PortNumber: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xbf79c0ab_bb74_4cee_b070_470b5ae202ea), pid: 12u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Dnssd_Priority: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xbf79c0ab_bb74_4cee_b070_470b5ae202ea), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Dnssd_ServiceName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xbf79c0ab_bb74_4cee_b070_470b5ae202ea), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Dnssd_TextAttributes: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xbf79c0ab_bb74_4cee_b070_470b5ae202ea), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Dnssd_Ttl: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xbf79c0ab_bb74_4cee_b070_470b5ae202ea), pid: 10u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Dnssd_Weight: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xbf79c0ab_bb74_4cee_b070_470b5ae202ea), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_FriendlyName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12288u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_FunctionPaths: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd08dd4c0_3a9e_462e_8290_7b636b2576b9), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_GlyphIcon: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x51236583_0c4a_4fe8_b81f_166aec13f510), pid: 123u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_HardwareIds: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Icon: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 57u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_InLocalMachineContainer: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8c7ed206_3f8a_4827_b3ab_ae9e1faefc6c), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_InterfaceClassGuid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x026e516e_b814_414b_83cd_856d6fef4822), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_InterfaceEnabled: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x026e516e_b814_414b_83cd_856d6fef4822), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_InterfacePaths: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd08dd4c0_3a9e_462e_8290_7b636b2576b9), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_IpAddress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12297u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_IsDefault: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 86u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_IsNetworkConnected: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 85u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_IsShared: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 84u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_IsSoftwareInstalling: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x83da6326_97a6_4088_9453_a1923f573b29), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_LaunchDeviceStageFromExplorer: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 77u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_LocalMachine: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 70u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_LocationPaths: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 37u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Manufacturer: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8192u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_MetadataPath: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 71u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_MicrophoneArray_Geometry: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa1829ea2_27eb_459e_935d_b2fad7b07762), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_MissedCalls: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x49cd1f76_5626_4b17_a4e8_18b4aa1a2213), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_ModelId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_ModelName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8194u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_ModelNumber: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8195u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_NetworkName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x49cd1f76_5626_4b17_a4e8_18b4aa1a2213), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_NetworkType: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x49cd1f76_5626_4b17_a4e8_18b4aa1a2213), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_NetworkedTooltip: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x880f70a2_6082_47ac_8aab_a739d1a300c3), pid: 152u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_NewPictures: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x49cd1f76_5626_4b17_a4e8_18b4aa1a2213), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_NotWorkingProperly: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 83u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Notification: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x06704b0c_e830_4c81_9178_91e4e95a80a0), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_NotificationStore: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x06704b0c_e830_4c81_9178_91e4e95a80a0), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Notifications_LowBattery: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc4c07f2b_8524_4e66_ae3a_a6235f103beb), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Notifications_MissedCall: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6614ef48_4efe_4424_9eda_c79f404edf3e), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Notifications_NewMessage: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2be9260a_2012_4742_a555_f41b638b7dcb), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Notifications_NewVoicemail: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x59569556_0a08_4212_95b9_fae2ad6413db), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Notifications_StorageFull: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa0e00ee1_f0c7_4d41_b8e7_26a7bd8d38b0), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Notifications_StorageFullLinkText: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa0e00ee1_f0c7_4d41_b8e7_26a7bd8d38b0), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Paired: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 56u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Panel_PanelGroup: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8dbc9c86_97a9_4bff_9bc6_bfe95d3e6dad), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Panel_PanelId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8dbc9c86_97a9_4bff_9bc6_bfe95d3e6dad), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Parent: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_PhoneLineTransportDevice_Connected: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xaecf2fe8_1d00_4fee_8a6d_a70d719b772b), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_PhysicalDeviceLocation: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x540b947e_8b40_45bc_a8a2_6a0b894cbda2), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_PlaybackPositionPercent: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x3633de59_6825_4381_a49b_9f6ba13a1471), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_PlaybackState: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x3633de59_6825_4381_a49b_9f6ba13a1471), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_PlaybackTitle: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x3633de59_6825_4381_a49b_9f6ba13a1471), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Present: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x540b947e_8b40_45bc_a8a2_6a0b894cbda2), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_PresentationUrl: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8198u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_PrimaryCategory: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd08dd4c0_3a9e_462e_8290_7b636b2576b9), pid: 10u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_RemainingDuration: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x3633de59_6825_4381_a49b_9f6ba13a1471), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_RestrictedInterface: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x026e516e_b814_414b_83cd_856d6fef4822), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Roaming: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x49cd1f76_5626_4b17_a4e8_18b4aa1a2213), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_SafeRemovalRequired: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xafd97640_86a3_4210_b67c_289c41aabe55), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_SchematicName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x026e516e_b814_414b_83cd_856d6fef4822), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_ServiceAddress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16384u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_ServiceId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16385u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_SharedTooltip: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x880f70a2_6082_47ac_8aab_a739d1a300c3), pid: 151u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_SignalStrength: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x49cd1f76_5626_4b17_a4e8_18b4aa1a2213), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_SmartCards_ReaderKind: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd6b5b883_18bd_4b4d_b2ec_9e38affeda82), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Status: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd08dd4c0_3a9e_462e_8290_7b636b2576b9), pid: 259u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Status1: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd08dd4c0_3a9e_462e_8290_7b636b2576b9), pid: 257u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Status2: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd08dd4c0_3a9e_462e_8290_7b636b2576b9), pid: 258u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_StorageCapacity: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x49cd1f76_5626_4b17_a4e8_18b4aa1a2213), pid: 12u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_StorageFreeSpace: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x49cd1f76_5626_4b17_a4e8_18b4aa1a2213), pid: 13u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_StorageFreeSpacePercent: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x49cd1f76_5626_4b17_a4e8_18b4aa1a2213), pid: 14u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_TextMessages: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x49cd1f76_5626_4b17_a4e8_18b4aa1a2213), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Voicemail: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x49cd1f76_5626_4b17_a4e8_18b4aa1a2213), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirectServices_AdvertisementId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x31b37743_7c5e_4005_93e6_e953f92b82e9), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirectServices_RequestServiceInformation: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x31b37743_7c5e_4005_93e6_e953f92b82e9), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirectServices_ServiceAddress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x31b37743_7c5e_4005_93e6_e953f92b82e9), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirectServices_ServiceConfigMethods: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x31b37743_7c5e_4005_93e6_e953f92b82e9), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirectServices_ServiceInformation: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x31b37743_7c5e_4005_93e6_e953f92b82e9), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirectServices_ServiceName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x31b37743_7c5e_4005_93e6_e953f92b82e9), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirect_DeviceAddress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 13u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirect_GroupId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirect_InformationElements: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 12u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirect_InterfaceAddress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirect_InterfaceGuid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirect_IsConnected: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirect_IsLegacyDevice: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirect_IsMiracastLcpSupported: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirect_IsVisible: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirect_MiracastVersion: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirect_Services: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 10u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFiDirect_SupportedChannelList: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 11u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiFi_InterfaceGuid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef1167eb_cbfc_4341_a568_a7c91a68982c), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WiaDeviceType: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6bdd1fc6_810f_11d0_bec7_08002be2092f), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_WinPhone8CameraFlags: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb7b4d61c_5a64_4187_a52e_b1539f359099), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Devices_Wwan_InterfaceGuid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xff1167eb_cbfc_4341_a568_a7c91a68982c), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_ByteCount: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd5cdd502_2e9c_101b_9397_08002b2cf9ae), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_CharacterCount: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf29f85e0_4ff9_1068_ab91_08002b27b3d9), pid: 16u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_ClientID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x276d7bb0_5b34_4fb0_aa4b_158ed12a1809), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_Contributor: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf334115e_da1b_4509_9b3d_119504dc7abb), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_DateCreated: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf29f85e0_4ff9_1068_ab91_08002b27b3d9), pid: 12u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_DatePrinted: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf29f85e0_4ff9_1068_ab91_08002b27b3d9), pid: 11u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_DateSaved: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf29f85e0_4ff9_1068_ab91_08002b27b3d9), pid: 13u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_Division: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1e005ee6_bf27_428b_b01c_79676acd2870), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_DocumentID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe08805c8_e395_40df_80d2_54f0d6c43154), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_HiddenSlideCount: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd5cdd502_2e9c_101b_9397_08002b2cf9ae), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_LastAuthor: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf29f85e0_4ff9_1068_ab91_08002b27b3d9), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_LineCount: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd5cdd502_2e9c_101b_9397_08002b2cf9ae), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_Manager: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd5cdd502_2e9c_101b_9397_08002b2cf9ae), pid: 14u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_MultimediaClipCount: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd5cdd502_2e9c_101b_9397_08002b2cf9ae), pid: 10u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_NoteCount: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd5cdd502_2e9c_101b_9397_08002b2cf9ae), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_PageCount: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf29f85e0_4ff9_1068_ab91_08002b27b3d9), pid: 14u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_ParagraphCount: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd5cdd502_2e9c_101b_9397_08002b2cf9ae), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_PresentationFormat: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd5cdd502_2e9c_101b_9397_08002b2cf9ae), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_RevisionNumber: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf29f85e0_4ff9_1068_ab91_08002b27b3d9), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_Security: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf29f85e0_4ff9_1068_ab91_08002b27b3d9), pid: 19u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_SlideCount: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd5cdd502_2e9c_101b_9397_08002b2cf9ae), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_Template: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf29f85e0_4ff9_1068_ab91_08002b27b3d9), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_TotalEditingTime: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf29f85e0_4ff9_1068_ab91_08002b27b3d9), pid: 10u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_Version: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd5cdd502_2e9c_101b_9397_08002b2cf9ae), pid: 29u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Document_WordCount: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf29f85e0_4ff9_1068_ab91_08002b27b3d9), pid: 15u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DueDate: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x3f8472b5_e0af_4db2_8071_c53fe76ae7ce), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_EdgeGesture_DisableTouchWhenFullscreen: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x32ce38b2_2c9a_41b1_9bc5_b3784394aa44), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_EndDate: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc75faa05_96fd_49e7_9cb4_9f601082d553), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ExpandoProperties: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6fa20de6_d11c_4d9d_a154_64317628c12d), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FileAllocationSize: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb725f130_47ef_101a_a5f1_02608c9eebac), pid: 18u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FileAttributes: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb725f130_47ef_101a_a5f1_02608c9eebac), pid: 13u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FileCount: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x28636aa6_953d_11d2_b5d6_00c04fd918d0), pid: 12u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FileDescription: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cef7d53_fa64_11d1_a203_0000f81fedee), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FileExtension: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe4f10a3c_49e6_405d_8288_a23bd4eeaa6c), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FileFRN: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb725f130_47ef_101a_a5f1_02608c9eebac), pid: 21u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FileName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x41cf5ae0_f75a_4806_bd87_59c7d9248eb9), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FileOfflineAvailabilityStatus: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfceff153_e839_4cf3_a9e7_ea22832094b8), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FileOwner: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9b174b34_40ff_11d2_a27e_00c04fc30871), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FilePlaceholderStatus: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb2f9b9d6_fec4_4dd5_94d7_8957488c807b), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FileVersion: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cef7d53_fa64_11d1_a203_0000f81fedee), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FindData: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x28636aa6_953d_11d2_b5d6_00c04fd918d0), pid: 0u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FlagColor: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x67df94de_0ca7_4d6f_b792_053a3e4f03cf), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FlagColorText: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x45eae747_8e2a_40ae_8cbf_ca52aba6152a), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FlagStatus: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe3e0584c_b788_4a5a_bb20_7f5a44c9acdd), pid: 12u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FlagStatusText: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xdc54fd2e_189d_4871_aa01_08c2f57a4abc), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FolderKind: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfceff153_e839_4cf3_a9e7_ea22832094b8), pid: 101u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FolderNameDisplay: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb725f130_47ef_101a_a5f1_02608c9eebac), pid: 25u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FreeSpace: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9b174b35_40ff_11d2_a27e_00c04fc30871), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FullText: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1e3ee840_bc2b_476c_8237_2acd1a839b22), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_Altitude: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x827edb4f_5b73_44a7_891d_fdffabea35ca), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_AltitudeDenominator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78342dcb_e358_4145_ae9a_6bfe4e0f9f51), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_AltitudeNumerator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2dad1eb7_816d_40d3_9ec3_c9773be2aade), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_AltitudeRef: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x46ac629d_75ea_4515_867f_6dc4321c5844), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_AreaInformation: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x972e333e_ac7e_49f1_8adf_a70d07a9bcab), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DOP: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cf8fb02_1837_42f1_a697_a7017aa289b9), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DOPDenominator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa0be94c5_50ba_487b_bd35_0654be8881ed), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DOPNumerator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x47166b16_364f_4aa0_9f31_e2ab3df449c3), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_Date: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x3602c812_0f3b_45f0_85ad_603468d69423), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestBearing: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc66d4b3c_e888_47cc_b99f_9dca3ee34dea), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestBearingDenominator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x7abcf4f8_7c3f_4988_ac91_8d2c2e97eca5), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestBearingNumerator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xba3b1da9_86ee_4b5d_a2a4_a271a429f0cf), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestBearingRef: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9ab84393_2a0f_4b75_bb22_7279786977cb), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestDistance: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa93eae04_6804_4f24_ac81_09b266452118), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestDistanceDenominator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9bc2c99b_ac71_4127_9d1c_2596d0d7dcb7), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestDistanceNumerator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2bda47da_08c6_4fe1_80bc_a72fc517c5d0), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestDistanceRef: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xed4df2d3_8695_450b_856f_f5c1c53acb66), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestLatitude: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9d1d7cc5_5c39_451c_86b3_928e2d18cc47), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestLatitudeDenominator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x3a372292_7fca_49a7_99d5_e47bb2d4e7ab), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestLatitudeNumerator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xecf4b6f6_d5a6_433c_bb92_4076650fc890), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestLatitudeRef: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xcea820b9_ce61_4885_a128_005d9087c192), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestLongitude: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x47a96261_cb4c_4807_8ad3_40b9d9dbc6bc), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestLongitudeDenominator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x425d69e5_48ad_4900_8d80_6eb6b8d0ac86), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestLongitudeNumerator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa3250282_fb6d_48d5_9a89_dbcace75cccf), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_DestLongitudeRef: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x182c1ea6_7c1c_4083_ab4b_ac6c9f4ed128), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_Differential: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xaaf4ee25_bd3b_4dd7_bfc4_47f77bb00f6d), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_ImgDirection: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x16473c91_d017_4ed9_ba4d_b6baa55dbcf8), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_ImgDirectionDenominator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x10b24595_41a2_4e20_93c2_5761c1395f32), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_ImgDirectionNumerator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xdc5877c7_225f_45f7_bac7_e81334b6130a), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_ImgDirectionRef: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa4aaa5b7_1ad0_445f_811a_0f8f6e67f6b5), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_Latitude: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8727cfff_4868_4ec6_ad5b_81b98521d1ab), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_LatitudeDecimal: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0f55cde2_4f49_450d_92c1_dcd16301b1b7), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_LatitudeDenominator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x16e634ee_2bff_497b_bd8a_4341ad39eeb9), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_LatitudeNumerator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x7ddaaad1_ccc8_41ae_b750_b2cb8031aea2), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_LatitudeRef: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x029c0252_5b86_46c7_aca0_2769ffc8e3d4), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_Longitude: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc4c4dbb2_b593_466b_bbda_d03d27d5e43a), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_LongitudeDecimal: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4679c1b5_844d_4590_baf5_f322231f1b81), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_LongitudeDenominator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xbe6e176c_4534_4d2c_ace5_31dedac1606b), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_LongitudeNumerator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x02b0f689_a914_4e45_821d_1dda452ed2c4), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_LongitudeRef: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x33dcf22b_28d5_464c_8035_1ee9efd25278), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_MapDatum: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2ca2dae6_eddc_407d_bef1_773942abfa95), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_MeasureMode: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa015ed5d_aaea_4d58_8a86_3c586920ea0b), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_ProcessingMethod: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x59d49e61_840f_4aa9_a939_e2099b7f6399), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_Satellites: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x467ee575_1f25_4557_ad4e_b8b58b0d9c15), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_Speed: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xda5d0862_6e76_4e1b_babd_70021bd25494), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_SpeedDenominator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x7d122d5a_ae5e_4335_8841_d71e7ce72f53), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_SpeedNumerator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xacc9ce3d_c213_4942_8b48_6d0820f21c6d), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_SpeedRef: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xecf7f4c9_544f_4d6d_9d98_8ad79adaf453), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_Status: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x125491f4_818f_46b2_91b5_d537753617b2), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_Track: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x76c09943_7c33_49e3_9e7e_cdba872cfada), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_TrackDenominator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc8d1920c_01f6_40c0_ac86_2f3a4ad00770), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_TrackNumerator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x702926f4_44a6_43e1_ae71_45627116893b), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_TrackRef: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x35dbe6fe_44c3_4400_aaae_d2c799c407e8), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_GPS_VersionID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x22704da4_c6b2_4a99_8e56_f16df8c92599), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_HighKeywords: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf29f85e0_4ff9_1068_ab91_08002b27b3d9), pid: 24u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_History_SelectionCount: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1ce0d6bc_536c_4600_b0dd_7e0c66b350d5), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_History_TargetUrlHostName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1ce0d6bc_536c_4600_b0dd_7e0c66b350d5), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_History_VisitCount: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5cbf2787_48cf_4208_b90e_ee5e5d420294), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Identity: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa26f4afc_7346_4299_be47_eb1ae613139f), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IdentityProvider_Name: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb96eff7b_35ca_4a35_8607_29e3a54c46ea), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IdentityProvider_Picture: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2425166f_5642_4864_992f_98fd98f294c3), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Identity_Blob: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8c3b93a4_baed_1a83_9a32_102ee313f6eb), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Identity_DisplayName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x7d683fc9_d155_45a8_bb1f_89d19bcb792f), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Identity_InternetSid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6d6d5d49_265d_4688_9f4e_1fdd33e7cc83), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Identity_IsMeIdentity: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa4108708_09df_4377_9dfc_6d99986d5a67), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Identity_KeyProviderContext: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa26f4afc_7346_4299_be47_eb1ae613139f), pid: 17u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Identity_KeyProviderName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa26f4afc_7346_4299_be47_eb1ae613139f), pid: 16u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Identity_LogonStatusString: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf18dedf3_337f_42c0_9e03_cee08708a8c3), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Identity_PrimaryEmailAddress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfcc16823_baed_4f24_9b32_a0982117f7fa), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Identity_PrimarySid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2b1b801e_c0c1_4987_9ec5_72fa89814787), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Identity_ProviderData: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa8a74b92_361b_4e9a_b722_7c4a7330a312), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Identity_ProviderID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x74a7de49_fa11_4d3d_a006_db7e08675916), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Identity_QualifiedUserName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xda520e51_f4e9_4739_ac82_02e0a95c9030), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Identity_UniqueID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe55fc3b0_2b60_4220_918e_b21e8bf16016), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Identity_UserName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc4322503_78ca_49c6_9acc_a68e2afd7b6b), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ImageParsingName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd7750ee0_c6a4_48ec_b53e_b87b52e6d073), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Image_BitDepth: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6444048f_4c8b_11d1_8b70_080036b11a03), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Image_ColorSpace: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x14b81da1_0135_4d31_96d9_6cbfc9671a99), pid: 40961u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Image_CompressedBitsPerPixel: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x364b6fa9_37ab_482a_be2b_ae02f60d4318), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Image_CompressedBitsPerPixelDenominator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1f8844e1_24ad_4508_9dfd_5326a415ce02), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Image_CompressedBitsPerPixelNumerator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd21a7148_d32c_4624_8900_277210f79c0f), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Image_Compression: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x14b81da1_0135_4d31_96d9_6cbfc9671a99), pid: 259u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Image_CompressionText: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x3f08e66f_2f44_4bb9_a682_ac35d2562322), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Image_Dimensions: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6444048f_4c8b_11d1_8b70_080036b11a03), pid: 13u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Image_HorizontalResolution: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6444048f_4c8b_11d1_8b70_080036b11a03), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Image_HorizontalSize: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6444048f_4c8b_11d1_8b70_080036b11a03), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Image_ImageID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x10dabe05_32aa_4c29_bf1a_63e2d220587f), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Image_ResolutionUnit: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x19b51fa6_1f92_4a5c_ab48_7df0abd67444), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Image_VerticalResolution: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6444048f_4c8b_11d1_8b70_080036b11a03), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Image_VerticalSize: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6444048f_4c8b_11d1_8b70_080036b11a03), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Importance: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe3e0584c_b788_4a5a_bb20_7f5a44c9acdd), pid: 11u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ImportanceText: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa3b29791_7713_4e1d_bb40_17db85f01831), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_InfoTipText: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc9944a21_a406_48fe_8225_aec7e24c211b), pid: 17u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_InternalName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cef7d53_fa64_11d1_a203_0000f81fedee), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IsAttachment: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf23f425c_71a1_4fa8_922f_678ea4a60408), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IsDefaultNonOwnerSaveLocation: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5d76b67f_9b3d_44bb_b6ae_25da4f638a67), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IsDefaultSaveLocation: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5d76b67f_9b3d_44bb_b6ae_25da4f638a67), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IsDeleted: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5cda5fc8_33ee_4ff3_9094_ae7bd8868c4d), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IsEncrypted: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x90e5e14e_648b_4826_b2aa_acaf790e3513), pid: 10u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IsFlagged: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5da84765_e3ff_4278_86b0_a27967fbdd03), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IsFlaggedComplete: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa6f360d2_55f9_48de_b909_620e090a647c), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IsIncomplete: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x346c8bd1_2e6a_4c45_89a4_61b78e8e700f), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IsLocationSupported: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5d76b67f_9b3d_44bb_b6ae_25da4f638a67), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IsPinnedToNameSpaceTree: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5d76b67f_9b3d_44bb_b6ae_25da4f638a67), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IsRead: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe3e0584c_b788_4a5a_bb20_7f5a44c9acdd), pid: 10u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IsSearchOnlyItem: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5d76b67f_9b3d_44bb_b6ae_25da4f638a67), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IsSendToTarget: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x28636aa6_953d_11d2_b5d6_00c04fd918d0), pid: 33u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_IsShared: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef884c5b_2bfe_41bb_aae5_76eedf4f9902), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemAuthors: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd0a04f0a_462a_48a4_bb2f_3706e88dbd7d), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemClassType: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x048658ad_2db8_41a4_bbb6_ac1ef1207eb1), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemDate: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf7db74b4_4287_4103_afba_f1b13dcd75cf), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemFolderNameDisplay: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb725f130_47ef_101a_a5f1_02608c9eebac), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemFolderPathDisplay: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe3e0584c_b788_4a5a_bb20_7f5a44c9acdd), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemFolderPathDisplayNarrow: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xdabd30ed_0043_4789_a7f8_d013a4736622), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6b8da074_3b5c_43bc_886f_0a2cdce00b6f), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemNameDisplay: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb725f130_47ef_101a_a5f1_02608c9eebac), pid: 10u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemNameDisplayWithoutExtension: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb725f130_47ef_101a_a5f1_02608c9eebac), pid: 24u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemNamePrefix: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd7313ff1_a77a_401c_8c99_3dbdd68add36), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemNameSortOverride: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb725f130_47ef_101a_a5f1_02608c9eebac), pid: 23u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemParticipants: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd4d0aa16_9948_41a4_aa85_d97ff9646993), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemPathDisplay: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe3e0584c_b788_4a5a_bb20_7f5a44c9acdd), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemPathDisplayNarrow: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x28636aa6_953d_11d2_b5d6_00c04fd918d0), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemSubType: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x28636aa6_953d_11d2_b5d6_00c04fd918d0), pid: 37u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemType: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x28636aa6_953d_11d2_b5d6_00c04fd918d0), pid: 11u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemTypeText: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb725f130_47ef_101a_a5f1_02608c9eebac), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ItemUrl: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x49691c90_7e17_101a_a91c_08002b2ecda9), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Journal_Contacts: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xdea7c82c_1d89_4a66_9427_a4e3debabcb1), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Journal_EntryType: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x95beb1fc_326d_4644_b396_cd3ed90e6ddf), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Keywords: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf29f85e0_4ff9_1068_ab91_08002b27b3d9), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Kind: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1e3ee840_bc2b_476c_8237_2acd1a839b22), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_KindText: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf04bef95_c585_4197_a2b7_df46fdc9ee6d), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Language: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd5cdd502_2e9c_101b_9397_08002b2cf9ae), pid: 28u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_LastSyncError: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfceff153_e839_4cf3_a9e7_ea22832094b8), pid: 107u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_LastSyncWarning: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfceff153_e839_4cf3_a9e7_ea22832094b8), pid: 128u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_LastWriterPackageFamilyName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x502cfeab_47eb_459c_b960_e6d8728f7701), pid: 101u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_LayoutPattern_ContentViewModeForBrowse: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc9944a21_a406_48fe_8225_aec7e24c211b), pid: 500u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_LayoutPattern_ContentViewModeForSearch: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc9944a21_a406_48fe_8225_aec7e24c211b), pid: 501u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_LibraryLocationsCount: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x908696c7_8f87_44f2_80ed_a8c1c6894575), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Link_Arguments: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x436f2667_14e2_4feb_b30a_146c53b5b674), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Link_Comment: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb9b4b3fc_2b51_4a42_b5d8_324146afcf25), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Link_DateVisited: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5cbf2787_48cf_4208_b90e_ee5e5d420294), pid: 23u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Link_Description: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5cbf2787_48cf_4208_b90e_ee5e5d420294), pid: 21u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Link_FeedItemLocalId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8a2f99f9_3c37_465d_a8d7_69777a246d0c), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Link_Status: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb9b4b3fc_2b51_4a42_b5d8_324146afcf25), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Link_TargetExtension: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x7a7d76f4_b630_4bd7_95ff_37cc51a975c9), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Link_TargetParsingPath: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb9b4b3fc_2b51_4a42_b5d8_324146afcf25), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Link_TargetSFGAOFlags: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb9b4b3fc_2b51_4a42_b5d8_324146afcf25), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Link_TargetSFGAOFlagsStrings: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd6942081_d53b_443d_ad47_5e059d9cd27a), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Link_TargetUrl: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5cbf2787_48cf_4208_b90e_ee5e5d420294), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Link_TargetUrlHostName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8a2f99f9_3c37_465d_a8d7_69777a246d0c), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Link_TargetUrlPath: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8a2f99f9_3c37_465d_a8d7_69777a246d0c), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_LowKeywords: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf29f85e0_4ff9_1068_ab91_08002b27b3d9), pid: 25u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_MIMEType: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0b63e350_9ccc_11d0_bcdb_00805fccce04), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_AuthorUrl: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 32u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_AverageLevel: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x09edd5b6_b301_43c5_9990_d00302effd46), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_ClassPrimaryID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 13u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_ClassSecondaryID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 14u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_CollectionGroupID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 24u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_CollectionID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 25u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_ContentDistributor: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 18u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_ContentID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 26u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_CreatorApplication: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 27u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_CreatorApplicationVersion: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 28u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_DVDID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 15u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_DateEncoded: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2e4b640d_5019_46d8_8881_55414cc5caa0), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_DateReleased: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xde41cc29_6971_4290_b472_f59f2e2f31e2), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_DlnaProfileID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xcfa31b45_525d_4998_bb44_3f7d81542fa4), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_Duration: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440490_4c8b_11d1_8b70_080036b11a03), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_EncodedBy: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 36u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_EncodingSettings: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 37u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_EpisodeNumber: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_FrameCount: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6444048f_4c8b_11d1_8b70_080036b11a03), pid: 12u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_MCDI: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 16u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_MetadataContentProvider: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 17u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_Producer: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 22u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_PromotionUrl: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 33u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_ProtectionType: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 38u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_ProviderRating: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 39u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_ProviderStyle: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 40u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_Publisher: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 30u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_SeasonNumber: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 101u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_SeriesName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 42u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_SubTitle: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x56a3372e_ce9c_11d2_9f0e_006097c686f6), pid: 38u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_SubscriptionContentId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9aebae7a_9644_487d_a92c_657585ed751a), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_ThumbnailLargePath: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 47u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_ThumbnailLargeUri: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 48u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_ThumbnailSmallPath: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 49u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_ThumbnailSmallUri: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 50u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_UniqueFileIdentifier: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 35u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_UserNoAutoInfo: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 41u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_UserWebUrl: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 34u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_Writer: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 23u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Media_Year: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x56a3372e_ce9c_11d2_9f0e_006097c686f6), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_MediumKeywords: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf29f85e0_4ff9_1068_ab91_08002b27b3d9), pid: 26u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_AttachmentContents: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x3143bf7c_80a8_4854_8880_e2e40189bdd0), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_AttachmentNames: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe3e0584c_b788_4a5a_bb20_7f5a44c9acdd), pid: 21u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_BccAddress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe3e0584c_b788_4a5a_bb20_7f5a44c9acdd), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_BccName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe3e0584c_b788_4a5a_bb20_7f5a44c9acdd), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_CcAddress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe3e0584c_b788_4a5a_bb20_7f5a44c9acdd), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_CcName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe3e0584c_b788_4a5a_bb20_7f5a44c9acdd), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_ConversationID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xdc8f80bd_af1e_4289_85b6_3dfc1b493992), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_ConversationIndex: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xdc8f80bd_af1e_4289_85b6_3dfc1b493992), pid: 101u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_DateReceived: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe3e0584c_b788_4a5a_bb20_7f5a44c9acdd), pid: 20u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_DateSent: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe3e0584c_b788_4a5a_bb20_7f5a44c9acdd), pid: 19u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_Flags: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa82d9ee7_ca67_4312_965e_226bcea85023), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_FromAddress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe3e0584c_b788_4a5a_bb20_7f5a44c9acdd), pid: 13u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_FromName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe3e0584c_b788_4a5a_bb20_7f5a44c9acdd), pid: 14u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_HasAttachments: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9c1fcf74_2d97_41ba_b4ae_cb2e3661a6e4), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_IsFwdOrReply: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9a9bc088_4f6d_469e_9919_e705412040f9), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_MessageClass: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xcd9ed458_08ce_418f_a70e_f912c7bb9c5c), pid: 103u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_Participants: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1a9ba605_8e7c_4d11_ad7d_a50ada18ba1b), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_ProofInProgress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9098f33c_9a7d_48a8_8de5_2e1227a64e91), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_SenderAddress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0be1c8e7_1981_4676_ae14_fdd78f05a6e7), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_SenderName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0da41cfa_d224_4a18_ae2f_596158db4b3a), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_Store: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe3e0584c_b788_4a5a_bb20_7f5a44c9acdd), pid: 15u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_ToAddress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe3e0584c_b788_4a5a_bb20_7f5a44c9acdd), pid: 16u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_ToDoFlags: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1f856a9f_6900_4aba_9505_2d5f1b4d66cb), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_ToDoTitle: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xbccc8a3c_8cef_42e5_9b1c_c69079398bc7), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Message_ToName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe3e0584c_b788_4a5a_bb20_7f5a44c9acdd), pid: 17u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_MileageInformation: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfdf84370_031a_4add_9e91_0d775f1c6605), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_AlbumArtist: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x56a3372e_ce9c_11d2_9f0e_006097c686f6), pid: 13u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_AlbumArtistSortOverride: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf1fdb4af_f78c_466c_bb05_56e92db0b8ec), pid: 103u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_AlbumID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x56a3372e_ce9c_11d2_9f0e_006097c686f6), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_AlbumTitle: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x56a3372e_ce9c_11d2_9f0e_006097c686f6), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_AlbumTitleSortOverride: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x13eb7ffc_ec89_4346_b19d_ccc6f1784223), pid: 101u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_Artist: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x56a3372e_ce9c_11d2_9f0e_006097c686f6), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_ArtistSortOverride: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xdeeb2db5_0696_4ce0_94fe_a01f77a45fb5), pid: 102u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_BeatsPerMinute: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x56a3372e_ce9c_11d2_9f0e_006097c686f6), pid: 35u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_Composer: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 19u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_ComposerSortOverride: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00bc20a3_bd48_4085_872c_a88d77f5097e), pid: 105u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_Conductor: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x56a3372e_ce9c_11d2_9f0e_006097c686f6), pid: 36u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_ContentGroupDescription: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x56a3372e_ce9c_11d2_9f0e_006097c686f6), pid: 33u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_DiscNumber: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6afe7437_9bcd_49c7_80fe_4a5c65fa5874), pid: 104u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_DisplayArtist: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfd122953_fa93_4ef7_92c3_04c946b2f7c8), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_Genre: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x56a3372e_ce9c_11d2_9f0e_006097c686f6), pid: 11u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_InitialKey: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x56a3372e_ce9c_11d2_9f0e_006097c686f6), pid: 34u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_IsCompilation: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc449d5cb_9ea4_4809_82e8_af9d59ded6d1), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_Lyrics: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x56a3372e_ce9c_11d2_9f0e_006097c686f6), pid: 12u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_Mood: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x56a3372e_ce9c_11d2_9f0e_006097c686f6), pid: 39u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_PartOfSet: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x56a3372e_ce9c_11d2_9f0e_006097c686f6), pid: 37u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_Period: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 31u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_SynchronizedLyrics: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6b223b6a_162e_4aa9_b39f_05d678fc6d77), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Music_TrackNumber: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x56a3372e_ce9c_11d2_9f0e_006097c686f6), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_NamespaceCLSID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x28636aa6_953d_11d2_b5d6_00c04fd918d0), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Note_Color: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4776cafa_bce4_4cb1_a23e_265e76d8eb11), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Note_ColorText: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x46b4e8de_cdb2_440d_885c_1658eb65b914), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Null: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000), pid: 0u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_OfflineAvailability: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa94688b6_7d9f_4570_a648_e3dfc0ab2b3f), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_OfflineStatus: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6d24888f_4718_4bda_afed_ea0fb4386cd8), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_OriginalFileName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cef7d53_fa64_11d1_a203_0000f81fedee), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_OwnerSID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5d76b67f_9b3d_44bb_b6ae_25da4f638a67), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ParentalRating: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 21u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ParentalRatingReason: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x10984e0a_f9f2_4321_b7ef_baf195af4319), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ParentalRatingsOrganization: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa7fe0840_1344_46f0_8d37_52ed712a4bf9), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ParsingBindContext: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xdfb9a04d_362f_4ca3_b30b_0254b17b5b84), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ParsingName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x28636aa6_953d_11d2_b5d6_00c04fd918d0), pid: 24u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ParsingPath: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x28636aa6_953d_11d2_b5d6_00c04fd918d0), pid: 30u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PerceivedType: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x28636aa6_953d_11d2_b5d6_00c04fd918d0), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PercentFull: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9b174b35_40ff_11d2_a27e_00c04fc30871), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_Aperture: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x14b81da1_0135_4d31_96d9_6cbfc9671a99), pid: 37378u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ApertureDenominator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe1a9a38b_6685_46bd_875e_570dc7ad7320), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ApertureNumerator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0337ecec_39fb_4581_a0bd_4c4cc51e9914), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_Brightness: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1a701bf6_478c_4361_83ab_3701bb053c58), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_BrightnessDenominator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6ebe6946_2321_440a_90f0_c043efd32476), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_BrightnessNumerator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9e7d118f_b314_45a0_8cfb_d654b917c9e9), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_CameraManufacturer: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x14b81da1_0135_4d31_96d9_6cbfc9671a99), pid: 271u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_CameraModel: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x14b81da1_0135_4d31_96d9_6cbfc9671a99), pid: 272u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_CameraSerialNumber: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x14b81da1_0135_4d31_96d9_6cbfc9671a99), pid: 273u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_Contrast: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2a785ba9_8d23_4ded_82e6_60a350c86a10), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ContrastText: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x59dde9f2_5253_40ea_9a8b_479e96c6249a), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_DateTaken: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x14b81da1_0135_4d31_96d9_6cbfc9671a99), pid: 36867u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_DigitalZoom: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf85bf840_a925_4bc2_b0c4_8e36b598679e), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_DigitalZoomDenominator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x745baf0e_e5c1_4cfb_8a1b_d031a0a52393), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_DigitalZoomNumerator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x16cbb924_6500_473b_a5be_f1599bcbe413), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_EXIFVersion: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd35f743a_eb2e_47f2_a286_844132cb1427), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_Event: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x14b81da1_0135_4d31_96d9_6cbfc9671a99), pid: 18248u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ExposureBias: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x14b81da1_0135_4d31_96d9_6cbfc9671a99), pid: 37380u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ExposureBiasDenominator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xab205e50_04b7_461c_a18c_2f233836e627), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ExposureBiasNumerator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x738bf284_1d87_420b_92cf_5834bf6ef9ed), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ExposureIndex: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x967b5af8_995a_46ed_9e11_35b3c5b9782d), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ExposureIndexDenominator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x93112f89_c28b_492f_8a9d_4be2062cee8a), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ExposureIndexNumerator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xcdedcf30_8919_44df_8f4c_4eb2ffdb8d89), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ExposureProgram: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x14b81da1_0135_4d31_96d9_6cbfc9671a99), pid: 34850u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ExposureProgramText: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfec690b7_5f30_4646_ae47_4caafba884a3), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ExposureTime: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x14b81da1_0135_4d31_96d9_6cbfc9671a99), pid: 33434u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ExposureTimeDenominator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x55e98597_ad16_42e0_b624_21599a199838), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ExposureTimeNumerator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x257e44e2_9031_4323_ac38_85c552871b2e), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FNumber: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x14b81da1_0135_4d31_96d9_6cbfc9671a99), pid: 33437u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FNumberDenominator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe92a2496_223b_4463_a4e3_30eabba79d80), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FNumberNumerator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1b97738a_fdfc_462f_9d93_1957e08be90c), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_Flash: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x14b81da1_0135_4d31_96d9_6cbfc9671a99), pid: 37385u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FlashEnergy: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x14b81da1_0135_4d31_96d9_6cbfc9671a99), pid: 41483u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FlashEnergyDenominator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd7b61c70_6323_49cd_a5fc_c84277162c97), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FlashEnergyNumerator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfcad3d3d_0858_400f_aaa3_2f66cce2a6bc), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FlashManufacturer: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xaabaf6c9_e0c5_4719_8585_57b103e584fe), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FlashModel: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfe83bb35_4d1a_42e2_916b_06f3e1af719e), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FlashText: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6b8b68f6_200b_47ea_8d25_d8050f57339f), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FocalLength: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x14b81da1_0135_4d31_96d9_6cbfc9671a99), pid: 37386u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FocalLengthDenominator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x305bc615_dca1_44a5_9fd4_10c0ba79412e), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FocalLengthInFilm: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa0e74609_b84d_4f49_b860_462bd9971f98), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FocalLengthNumerator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x776b6b3b_1e3d_4b0c_9a0e_8fbaf2a8492a), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FocalPlaneXResolution: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xcfc08d97_c6f7_4484_89dd_ebef4356fe76), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FocalPlaneXResolutionDenominator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0933f3f5_4786_4f46_a8e8_d64dd37fa521), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FocalPlaneXResolutionNumerator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xdccb10af_b4e2_4b88_95f9_031b4d5ab490), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FocalPlaneYResolution: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4fffe4d0_914f_4ac4_8d6f_c9c61de169b1), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FocalPlaneYResolutionDenominator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1d6179a6_a876_4031_b013_3347b2b64dc8), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_FocalPlaneYResolutionNumerator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa2e541c5_4440_4ba8_867e_75cfc06828cd), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_GainControl: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfa304789_00c7_4d80_904a_1e4dcc7265aa), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_GainControlDenominator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x42864dfd_9da4_4f77_bded_4aad7b256735), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_GainControlNumerator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8e8ecf7c_b7b8_4eb8_a63f_0ee715c96f9e), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_GainControlText: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc06238b2_0bf9_4279_a723_25856715cb9d), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ISOSpeed: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x14b81da1_0135_4d31_96d9_6cbfc9671a99), pid: 34855u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_LensManufacturer: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe6ddcaf7_29c5_4f0a_9a68_d19412ec7090), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_LensModel: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe1277516_2b5f_4869_89b1_2e585bd38b7a), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_LightSource: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x14b81da1_0135_4d31_96d9_6cbfc9671a99), pid: 37384u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_MakerNote: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfa303353_b659_4052_85e9_bcac79549b84), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_MakerNoteOffset: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x813f4124_34e6_4d17_ab3e_6b1f3c2247a1), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_MaxAperture: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x08f6d7c2_e3f2_44fc_af1e_5aa5c81a2d3e), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_MaxApertureDenominator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc77724d4_601f_46c5_9b89_c53f93bceb77), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_MaxApertureNumerator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc107e191_a459_44c5_9ae6_b952ad4b906d), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_MeteringMode: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x14b81da1_0135_4d31_96d9_6cbfc9671a99), pid: 37383u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_MeteringModeText: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf628fd8c_7ba8_465a_a65b_c5aa79263a9e), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_Orientation: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x14b81da1_0135_4d31_96d9_6cbfc9671a99), pid: 274u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_OrientationText: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa9ea193c_c511_498a_a06b_58e2776dcc28), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_PeopleNames: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe8309b6e_084c_49b4_b1fc_90a80331b638), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_PhotometricInterpretation: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x341796f1_1df9_4b1c_a564_91bdefa43877), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_PhotometricInterpretationText: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x821437d6_9eab_4765_a589_3b1cbbd22a61), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ProgramMode: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6d217f6d_3f6a_4825_b470_5f03ca2fbe9b), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ProgramModeText: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x7fe3aa27_2648_42f3_89b0_454e5cb150c3), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_RelatedSoundFile: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x318a6b45_087f_4dc2_b8cc_05359551fc9e), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_Saturation: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x49237325_a95a_4f67_b211_816b2d45d2e0), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_SaturationText: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x61478c08_b600_4a84_bbe4_e99c45f0a072), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_Sharpness: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfc6976db_8349_4970_ae97_b3c5316a08f0), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_SharpnessText: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x51ec3f47_dd50_421d_8769_334f50424b1e), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ShutterSpeed: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x14b81da1_0135_4d31_96d9_6cbfc9671a99), pid: 37377u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ShutterSpeedDenominator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe13d8975_81c7_4948_ae3f_37cae11e8ff7), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_ShutterSpeedNumerator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x16ea4042_d6f4_4bca_8349_7c78d30fb333), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_SubjectDistance: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x14b81da1_0135_4d31_96d9_6cbfc9671a99), pid: 37382u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_SubjectDistanceDenominator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0c840a88_b043_466d_9766_d4b26da3fa77), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_SubjectDistanceNumerator: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8af4961c_f526_43e5_aa81_db768219178d), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_TagViewAggregate: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb812f15d_c2d8_4bbf_bacd_79744346113f), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_TranscodedForSync: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9a8ebb75_6458_4e82_bacb_35c0095b03bb), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_WhiteBalance: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xee3d3d8a_5381_4cfa_b13b_aaf66b5f4ec9), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Photo_WhiteBalanceText: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6336b95e_c7a7_426d_86fd_7ae3d39c84b4), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Priority: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9c1fcf74_2d97_41ba_b4ae_cb2e3661a6e4), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PriorityText: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd98be98b_b86b_4095_bf52_9d23b2e0a752), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Project: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x39a7f922_477c_48de_8bc8_b28441e342e3), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_Advanced: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x900a403b_097b_4b95_8ae2_071fdaeeb118), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_Audio: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2804d469_788f_48aa_8570_71b9c187e138), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_Calendar: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9973d2b5_bfd8_438a_ba94_5349b293181a), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_Camera: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xde00de32_547e_4981_ad4b_542f2e9007d8), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_Contact: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xdf975fd3_250a_4004_858f_34e29a3e37aa), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_Content: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd0dab0ba_368a_4050_a882_6c010fd19a4f), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_Description: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8969b275_9475_4e00_a887_ff93b8b41e44), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_FileSystem: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe3a7d2c1_80fc_4b40_8f34_30ea111bdc2e), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_GPS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf3713ada_90e3_4e11_aae5_fdc17685b9be), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_General: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xcc301630_b192_4c22_b372_9f4c6d338e07), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_Image: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe3690a87_0fa8_4a2a_9a9f_fce8827055ac), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_Media: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x61872cf7_6b5e_4b4b_ac2d_59da84459248), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_MediaAdvanced: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8859a284_de7e_4642_99ba_d431d044b1ec), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_Message: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x7fd7259d_16b4_4135_9f97_7c96ecd2fa9e), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_Music: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x68dd6094_7216_40f1_a029_43fe7127043f), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_Origin: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2598d2fb_5569_4367_95df_5cd3a177e1a5), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_PhotoAdvanced: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cb2bf5a_9ee7_4a86_8222_f01e07fdadaf), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_RecordedTV: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xe7b33238_6584_4170_a5c0_ac25efd9da56), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropGroup_Video: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xbebe0920_7671_4c54_a3eb_49fddfc191ee), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropList_ConflictPrompt: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc9944a21_a406_48fe_8225_aec7e24c211b), pid: 11u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropList_ContentViewModeForBrowse: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc9944a21_a406_48fe_8225_aec7e24c211b), pid: 13u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropList_ContentViewModeForSearch: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc9944a21_a406_48fe_8225_aec7e24c211b), pid: 14u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropList_ExtendedTileInfo: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc9944a21_a406_48fe_8225_aec7e24c211b), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropList_FileOperationPrompt: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc9944a21_a406_48fe_8225_aec7e24c211b), pid: 10u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropList_FullDetails: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc9944a21_a406_48fe_8225_aec7e24c211b), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropList_InfoTip: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc9944a21_a406_48fe_8225_aec7e24c211b), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropList_NonPersonal: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x49d1091f_082e_493f_b23f_d2308aa9668c), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropList_PreviewDetails: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc9944a21_a406_48fe_8225_aec7e24c211b), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropList_PreviewTitle: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc9944a21_a406_48fe_8225_aec7e24c211b), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropList_QuickTip: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc9944a21_a406_48fe_8225_aec7e24c211b), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropList_TileInfo: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xc9944a21_a406_48fe_8225_aec7e24c211b), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PropList_XPDetailsPanel: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf2275480_f782_4291_bd94_f13693513aec), pid: 0u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ProviderItemID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf21d9941_81f0_471a_adee_4e74b49217ed), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Rating: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RatingText: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x90197ca7_fd8f_4e8c_9da3_b57e1e609295), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_ChannelNumber: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6d748de2_8d38_4cc3_ac60_f009b057c557), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_Credits: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6d748de2_8d38_4cc3_ac60_f009b057c557), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_DateContentExpires: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6d748de2_8d38_4cc3_ac60_f009b057c557), pid: 15u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_EpisodeName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6d748de2_8d38_4cc3_ac60_f009b057c557), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_IsATSCContent: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6d748de2_8d38_4cc3_ac60_f009b057c557), pid: 16u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_IsClosedCaptioningAvailable: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6d748de2_8d38_4cc3_ac60_f009b057c557), pid: 12u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_IsDTVContent: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6d748de2_8d38_4cc3_ac60_f009b057c557), pid: 17u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_IsHDContent: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6d748de2_8d38_4cc3_ac60_f009b057c557), pid: 18u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_IsRepeatBroadcast: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6d748de2_8d38_4cc3_ac60_f009b057c557), pid: 13u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_IsSAP: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6d748de2_8d38_4cc3_ac60_f009b057c557), pid: 14u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_NetworkAffiliation: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x2c53c813_fb63_4e22_a1ab_0b331ca1e273), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_OriginalBroadcastDate: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4684fe97_8765_4842_9c13_f006447b178c), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_ProgramDescription: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6d748de2_8d38_4cc3_ac60_f009b057c557), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_RecordingTime: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa5477f61_7a82_4eca_9dde_98b69b2479b3), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_StationCallSign: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x6d748de2_8d38_4cc3_ac60_f009b057c557), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RecordedTV_StationName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x1b5439e7_eba1_4af8_bdd7_7af1d4549493), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_RemoteConflictingFile: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfceff153_e839_4cf3_a9e7_ea22832094b8), pid: 115u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SFGAOFlags: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x28636aa6_953d_11d2_b5d6_00c04fd918d0), pid: 25u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_AutoSummary: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x560c36c0_503a_11cf_baa1_00004c752a9a), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_ContainerHash: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xbceee283_35df_4d53_826a_f36a3eefc6be), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_Contents: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb725f130_47ef_101a_a5f1_02608c9eebac), pid: 19u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_EntryID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x49691c90_7e17_101a_a91c_08002b2ecda9), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_ExtendedProperties: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x7b03b546_fa4f_4a52_a2fe_03d5311e5865), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_GatherTime: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0b63e350_9ccc_11d0_bcdb_00805fccce04), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_HitCount: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x49691c90_7e17_101a_a91c_08002b2ecda9), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_IsClosedDirectory: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0b63e343_9ccc_11d0_bcdb_00805fccce04), pid: 23u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_IsFullyContained: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0b63e343_9ccc_11d0_bcdb_00805fccce04), pid: 24u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_QueryFocusedSummary: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x560c36c0_503a_11cf_baa1_00004c752a9a), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_QueryFocusedSummaryWithFallback: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x560c36c0_503a_11cf_baa1_00004c752a9a), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_QueryPropertyHits: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x49691c90_7e17_101a_a91c_08002b2ecda9), pid: 21u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_Rank: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x49691c90_7e17_101a_a91c_08002b2ecda9), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_Store: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa06992b3_8caf_4ed7_a547_b259e32ac9fc), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_UrlToIndex: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0b63e343_9ccc_11d0_bcdb_00805fccce04), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Search_UrlToIndexWithModificationTime: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0b63e343_9ccc_11d0_bcdb_00805fccce04), pid: 12u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Security_AllowedEnterpriseDataProtectionIdentities: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x38d43380_d418_4830_84d5_46935a81c5c6), pid: 32u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Security_EncryptionOwners: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5f5aff6a_37e5_4780_97ea_80c7565cf535), pid: 34u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Security_EncryptionOwnersDisplay: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xde621b8f_e125_43a3_a32d_5665446d632a), pid: 25u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Sensitivity: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf8d3f6ac_4874_42cb_be59_ab454b30716a), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SensitivityText: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd0c7f054_3f72_4725_8527_129a577cb269), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ShareUserRating: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 12u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SharedWith: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef884c5b_2bfe_41bb_aae5_76eedf4f9902), pid: 200u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SharingStatus: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xef884c5b_2bfe_41bb_aae5_76eedf4f9902), pid: 300u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Shell_OmitFromView: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xde35258c_c695_4cbc_b982_38b0ad24ced0), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Shell_SFGAOFlagsStrings: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd6942081_d53b_443d_ad47_5e059d9cd27a), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SimpleRating: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa09f084e_ad41_489f_8076_aa5be3082bca), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Size: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb725f130_47ef_101a_a5f1_02608c9eebac), pid: 12u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SoftwareUsed: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x14b81da1_0135_4d31_96d9_6cbfc9671a99), pid: 305u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Software_DateLastUsed: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x841e4f90_ff59_4d16_8947_e81bbffab36d), pid: 16u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Software_ProductName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cef7d53_fa64_11d1_a203_0000f81fedee), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SourceItem: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x668cdfa5_7a1b_4323_ae4b_e527393a1d81), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SourcePackageFamilyName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xffae9db7_1c8d_43ff_818c_84403aa3732d), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StartDate: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x48fd6ec8_8a12_4cdf_a03e_4ec5a511edde), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Status: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x000214a1_0000_0000_c000_000000000046), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StatusBarSelectedItemCount: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x26dc287c_6e3d_4bd3_b2b0_6a26ba2e346d), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StatusBarViewItemCount: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x26dc287c_6e3d_4bd3_b2b0_6a26ba2e346d), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StorageProviderCallerVersionInformation: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb2f9b9d6_fec4_4dd5_94d7_8957488c807b), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StorageProviderError: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfceff153_e839_4cf3_a9e7_ea22832094b8), pid: 109u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StorageProviderFileChecksum: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb2f9b9d6_fec4_4dd5_94d7_8957488c807b), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StorageProviderFileFlags: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb2f9b9d6_fec4_4dd5_94d7_8957488c807b), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StorageProviderFileHasConflict: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb2f9b9d6_fec4_4dd5_94d7_8957488c807b), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StorageProviderFileIdentifier: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb2f9b9d6_fec4_4dd5_94d7_8957488c807b), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StorageProviderFileRemoteUri: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfceff153_e839_4cf3_a9e7_ea22832094b8), pid: 112u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StorageProviderFileVersion: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb2f9b9d6_fec4_4dd5_94d7_8957488c807b), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StorageProviderFileVersionWaterline: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb2f9b9d6_fec4_4dd5_94d7_8957488c807b), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StorageProviderId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfceff153_e839_4cf3_a9e7_ea22832094b8), pid: 108u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StorageProviderShareStatuses: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfceff153_e839_4cf3_a9e7_ea22832094b8), pid: 111u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StorageProviderSharingStatus: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfceff153_e839_4cf3_a9e7_ea22832094b8), pid: 117u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_StorageProviderStatus: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfceff153_e839_4cf3_a9e7_ea22832094b8), pid: 110u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Storage_Portable: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4d1ebee8_0803_4774_9842_b77db50265e9), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Storage_RemovableMedia: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4d1ebee8_0803_4774_9842_b77db50265e9), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Storage_SystemCritical: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4d1ebee8_0803_4774_9842_b77db50265e9), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Subject: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf29f85e0_4ff9_1068_ab91_08002b27b3d9), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Supplemental_Album: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0c73b141_39d6_4653_a683_cab291eaf95b), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Supplemental_AlbumID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0c73b141_39d6_4653_a683_cab291eaf95b), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Supplemental_Location: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0c73b141_39d6_4653_a683_cab291eaf95b), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Supplemental_Person: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0c73b141_39d6_4653_a683_cab291eaf95b), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Supplemental_ResourceId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0c73b141_39d6_4653_a683_cab291eaf95b), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Supplemental_Tag: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0c73b141_39d6_4653_a683_cab291eaf95b), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SyncTransferStatus: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfceff153_e839_4cf3_a9e7_ea22832094b8), pid: 103u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Sync_Comments: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x7bd5533e_af15_44db_b8c8_bd6624e1d032), pid: 13u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Sync_ConflictDescription: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xce50c159_2fb8_41fd_be68_d3e042e274bc), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Sync_ConflictFirstLocation: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xce50c159_2fb8_41fd_be68_d3e042e274bc), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Sync_ConflictSecondLocation: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xce50c159_2fb8_41fd_be68_d3e042e274bc), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Sync_HandlerCollectionID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x7bd5533e_af15_44db_b8c8_bd6624e1d032), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Sync_HandlerID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x7bd5533e_af15_44db_b8c8_bd6624e1d032), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Sync_HandlerName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xce50c159_2fb8_41fd_be68_d3e042e274bc), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Sync_HandlerType: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x7bd5533e_af15_44db_b8c8_bd6624e1d032), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Sync_HandlerTypeLabel: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x7bd5533e_af15_44db_b8c8_bd6624e1d032), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Sync_ItemID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x7bd5533e_af15_44db_b8c8_bd6624e1d032), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Sync_ItemName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xce50c159_2fb8_41fd_be68_d3e042e274bc), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Sync_ProgressPercentage: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x7bd5533e_af15_44db_b8c8_bd6624e1d032), pid: 23u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Sync_State: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x7bd5533e_af15_44db_b8c8_bd6624e1d032), pid: 24u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Sync_Status: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x7bd5533e_af15_44db_b8c8_bd6624e1d032), pid: 10u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Task_BillingInformation: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd37d52c6_261c_4303_82b3_08b926ac6f12), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Task_CompletionStatus: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x084d8a0a_e6d5_40de_bf1f_c8820e7c877c), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Task_Owner: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x08c7cc5f_60f2_4494_ad75_55e3e0b5add0), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Thumbnail: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf29f85e0_4ff9_1068_ab91_08002b27b3d9), pid: 17u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ThumbnailCacheId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x446d16b1_8dad_4870_a748_402ea43d788c), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ThumbnailStream: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf29f85e0_4ff9_1068_ab91_08002b27b3d9), pid: 27u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Title: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf29f85e0_4ff9_1068_ab91_08002b27b3d9), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_TitleSortOverride: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xf0f7984d_222e_4ad2_82ab_1dd8ea40e57e), pid: 300u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_TotalFileSize: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x28636aa6_953d_11d2_b5d6_00c04fd918d0), pid: 14u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Trademarks: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x0cef7d53_fa64_11d1_a203_0000f81fedee), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_TransferOrder: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfceff153_e839_4cf3_a9e7_ea22832094b8), pid: 106u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_TransferPosition: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfceff153_e839_4cf3_a9e7_ea22832094b8), pid: 104u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_TransferSize: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xfceff153_e839_4cf3_a9e7_ea22832094b8), pid: 105u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_Compression: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440491_4c8b_11d1_8b70_080036b11a03), pid: 10u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_Director: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440492_4c8b_11d1_8b70_080036b11a03), pid: 20u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_EncodingBitrate: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440491_4c8b_11d1_8b70_080036b11a03), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_FourCC: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440491_4c8b_11d1_8b70_080036b11a03), pid: 44u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_FrameHeight: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440491_4c8b_11d1_8b70_080036b11a03), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_FrameRate: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440491_4c8b_11d1_8b70_080036b11a03), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_FrameWidth: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440491_4c8b_11d1_8b70_080036b11a03), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_HorizontalAspectRatio: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440491_4c8b_11d1_8b70_080036b11a03), pid: 42u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_IsSpherical: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440491_4c8b_11d1_8b70_080036b11a03), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_IsStereo: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440491_4c8b_11d1_8b70_080036b11a03), pid: 98u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_Orientation: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440491_4c8b_11d1_8b70_080036b11a03), pid: 99u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_SampleSize: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440491_4c8b_11d1_8b70_080036b11a03), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_StreamName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440491_4c8b_11d1_8b70_080036b11a03), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_StreamNumber: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440491_4c8b_11d1_8b70_080036b11a03), pid: 11u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_TotalBitrate: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440491_4c8b_11d1_8b70_080036b11a03), pid: 43u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_TranscodedForSync: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440491_4c8b_11d1_8b70_080036b11a03), pid: 46u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Video_VerticalAspectRatio: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x64440491_4c8b_11d1_8b70_080036b11a03), pid: 45u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_VolumeId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x446d16b1_8dad_4870_a748_402ea43d788c), pid: 104u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Volume_FileSystem: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9b174b35_40ff_11d2_a27e_00c04fc30871), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Volume_IsMappedDrive: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x149c0b69_2c2d_48fc_808f_d318d78c4636), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Volume_IsRoot: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x9b174b35_40ff_11d2_a27e_00c04fc30871), pid: 10u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_ZoneIdentifier: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x502cfeab_47eb_459c_b960_e6d8728f7701), pid: 100u32 };
pub const PLAYBACKSTATE_NOMEDIA: u32 = 7u32;
pub const PLAYBACKSTATE_PAUSED: u32 = 4u32;
pub const PLAYBACKSTATE_PLAYING: u32 = 2u32;
pub const PLAYBACKSTATE_RECORDING: u32 = 6u32;
pub const PLAYBACKSTATE_RECORDINGPAUSED: u32 = 5u32;
pub const PLAYBACKSTATE_STOPPED: u32 = 1u32;
pub const PLAYBACKSTATE_TRANSITIONING: u32 = 3u32;
pub const PLAYBACKSTATE_UNKNOWN: u32 = 0u32;
pub const RATING_FIVE_STARS_MAX: u32 = 99u32;
pub const RATING_FIVE_STARS_MIN: u32 = 88u32;
pub const RATING_FIVE_STARS_SET: u32 = 99u32;
pub const RATING_FOUR_STARS_MAX: u32 = 87u32;
pub const RATING_FOUR_STARS_MIN: u32 = 63u32;
pub const RATING_FOUR_STARS_SET: u32 = 75u32;
pub const RATING_ONE_STAR_MAX: u32 = 12u32;
pub const RATING_ONE_STAR_MIN: u32 = 1u32;
pub const RATING_ONE_STAR_SET: u32 = 1u32;
pub const RATING_THREE_STARS_MAX: u32 = 62u32;
pub const RATING_THREE_STARS_MIN: u32 = 38u32;
pub const RATING_THREE_STARS_SET: u32 = 50u32;
pub const RATING_TWO_STARS_MAX: u32 = 37u32;
pub const RATING_TWO_STARS_MIN: u32 = 13u32;
pub const RATING_TWO_STARS_SET: u32 = 25u32;
pub const SFGAOSTR_BROWSABLE: &str = "browsable";
pub const SFGAOSTR_FILEANC: &str = "fileanc";
pub const SFGAOSTR_FILESYS: &str = "filesys";
pub const SFGAOSTR_FOLDER: &str = "folder";
pub const SFGAOSTR_HIDDEN: &str = "hidden";
pub const SFGAOSTR_LINK: &str = "link";
pub const SFGAOSTR_NONENUM: &str = "nonenum";
pub const SFGAOSTR_PLACEHOLDER: &str = "placeholder";
pub const SFGAOSTR_STORAGEANC: &str = "storageanc";
pub const SFGAOSTR_STREAM: &str = "stream";
pub const SFGAOSTR_SUPERHIDDEN: &str = "superhidden";
pub const SFGAOSTR_SYSTEM: &str = "system";
pub const SHARINGSTATUS_NOTSHARED: u32 = 0u32;
pub const SHARINGSTATUS_PRIVATE: u32 = 2u32;
pub const SHARINGSTATUS_SHARED: u32 = 1u32;
#[repr(C)]
pub struct SILO_INFO {
    pub ulSTID: u32,
    pub SpecificationMajor: u8,
    pub SpecificationMinor: u8,
    pub ImplementationMajor: u8,
    pub ImplementationMinor: u8,
    pub r#type: u8,
    pub capabilities: u8,
}
impl ::core::marker::Copy for SILO_INFO {}
impl ::core::clone::Clone for SILO_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SILO_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SILO_INFO").field("ulSTID", &self.ulSTID).field("SpecificationMajor", &self.SpecificationMajor).field("SpecificationMinor", &self.SpecificationMinor).field("ImplementationMajor", &self.ImplementationMajor).field("ImplementationMinor", &self.ImplementationMinor).field("type", &self.r#type).field("capabilities", &self.capabilities).finish()
    }
}
unsafe impl ::windows_core::Abi for SILO_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SILO_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SILO_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for SILO_INFO {}
impl ::core::default::Default for SILO_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const STORAGE_PROVIDER_SHARE_STATUS_GROUP: &str = "Group";
pub const STORAGE_PROVIDER_SHARE_STATUS_OWNER: &str = "Owner";
pub const STORAGE_PROVIDER_SHARE_STATUS_PRIVATE: &str = "Private";
pub const STORAGE_PROVIDER_SHARE_STATUS_PUBLIC: &str = "Public";
pub const STORAGE_PROVIDER_SHARE_STATUS_SHARED: &str = "Shared";
pub const STORAGE_PROVIDER_SHARINGSTATUS_NOTSHARED: u32 = 0u32;
pub const STORAGE_PROVIDER_SHARINGSTATUS_PRIVATE: u32 = 2u32;
pub const STORAGE_PROVIDER_SHARINGSTATUS_PUBLIC: u32 = 3u32;
pub const STORAGE_PROVIDER_SHARINGSTATUS_PUBLIC_COOWNED: u32 = 7u32;
pub const STORAGE_PROVIDER_SHARINGSTATUS_PUBLIC_OWNED: u32 = 6u32;
pub const STORAGE_PROVIDER_SHARINGSTATUS_SHARED: u32 = 1u32;
pub const STORAGE_PROVIDER_SHARINGSTATUS_SHARED_COOWNED: u32 = 5u32;
pub const STORAGE_PROVIDER_SHARINGSTATUS_SHARED_OWNED: u32 = 4u32;
pub const SYNC_HANDLERTYPE_COMPUTERS: u32 = 5u32;
pub const SYNC_HANDLERTYPE_DEVICES: u32 = 2u32;
pub const SYNC_HANDLERTYPE_FOLDERS: u32 = 3u32;
pub const SYNC_HANDLERTYPE_OTHER: u32 = 0u32;
pub const SYNC_HANDLERTYPE_PROGRAMS: u32 = 1u32;
pub const SYNC_HANDLERTYPE_WEBSERVICES: u32 = 4u32;
pub const SYNC_STATE_ERROR: u32 = 3u32;
pub const SYNC_STATE_IDLE: u32 = 2u32;
pub const SYNC_STATE_NOTSETUP: u32 = 0u32;
pub const SYNC_STATE_PENDING: u32 = 4u32;
pub const SYNC_STATE_SYNCING: u32 = 5u32;
pub const SYNC_STATE_SYNCNOTRUN: u32 = 1u32;
pub const WPD_CATEGORY_ENHANCED_STORAGE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x91248166_b832_4ad4_baa4_7ca0b6b2798c);
