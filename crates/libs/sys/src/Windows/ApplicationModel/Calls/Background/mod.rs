#[repr(C)]
pub struct IPhoneCallBlockedTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LineId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub CallBlockedReason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneCallBlockedReason) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneCallBlockedTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2762379426, data2: 58561, data3: 17023, data4: [134, 78, 228, 112, 71, 125, 219, 103] };
}
#[repr(C)]
pub struct IPhoneCallOriginDataRequestTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequestId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneCallOriginDataRequestTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1855675199, data2: 50507, data3: 20098, data4: [76, 201, 227, 41, 164, 24, 69, 146] };
}
#[repr(C)]
pub struct IPhoneIncomingCallDismissedTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub LineId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DismissalTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DismissalTime: usize,
    pub TextReplyMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneIncomingCallDismissedReason) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneIncomingCallDismissedTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3134390902, data2: 33718, data3: 22322, data4: [156, 56, 12, 32, 101, 70, 25, 106] };
}
#[repr(C)]
pub struct IPhoneIncomingCallNotificationTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub LineId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub CallId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneIncomingCallNotificationTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 722362436, data2: 39730, data3: 23874, data4: [130, 34, 210, 129, 46, 57, 251, 33] };
}
#[repr(C)]
pub struct IPhoneLineChangedTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub LineId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub ChangeType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneLineChangeKind) -> ::windows_sys::core::HRESULT,
    pub HasLinePropertyChanged: unsafe extern "system" fn(this: *mut *mut Self, lineproperty: PhoneLineProperties, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneLineChangedTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3335725543, data2: 53533, data3: 16600, data4: [178, 183, 228, 10, 1, 214, 98, 73] };
}
#[repr(C)]
pub struct IPhoneNewVoicemailMessageTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub LineId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub VoicemailCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub OperatorMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneNewVoicemailMessageTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 329826331, data2: 47153, data3: 18643, data4: [139, 169, 141, 34, 166, 88, 13, 207] };
}
#[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
#[repr(transparent)]
pub struct PhoneCallBlockedReason(pub i32);
impl PhoneCallBlockedReason {
    pub const InCallBlockingList: Self = Self(0i32);
    pub const PrivateNumber: Self = Self(1i32);
    pub const UnknownNumber: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneCallBlockedReason {}
impl ::core::clone::Clone for PhoneCallBlockedReason {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PhoneCallBlockedTriggerDetails = *mut ::core::ffi::c_void;
pub type PhoneCallOriginDataRequestTriggerDetails = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
#[repr(transparent)]
pub struct PhoneIncomingCallDismissedReason(pub i32);
impl PhoneIncomingCallDismissedReason {
    pub const Unknown: Self = Self(0i32);
    pub const CallRejected: Self = Self(1i32);
    pub const TextReply: Self = Self(2i32);
    pub const ConnectionLost: Self = Self(3i32);
}
impl ::core::marker::Copy for PhoneIncomingCallDismissedReason {}
impl ::core::clone::Clone for PhoneIncomingCallDismissedReason {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PhoneIncomingCallDismissedTriggerDetails = *mut ::core::ffi::c_void;
pub type PhoneIncomingCallNotificationTriggerDetails = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
#[repr(transparent)]
pub struct PhoneLineChangeKind(pub i32);
impl PhoneLineChangeKind {
    pub const Added: Self = Self(0i32);
    pub const Removed: Self = Self(1i32);
    pub const PropertiesChanged: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneLineChangeKind {}
impl ::core::clone::Clone for PhoneLineChangeKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PhoneLineChangedTriggerDetails = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
#[repr(transparent)]
pub struct PhoneLineProperties(pub u32);
impl PhoneLineProperties {
    pub const None: Self = Self(0u32);
    pub const BrandingOptions: Self = Self(1u32);
    pub const CanDial: Self = Self(2u32);
    pub const CellularDetails: Self = Self(4u32);
    pub const DisplayColor: Self = Self(8u32);
    pub const DisplayName: Self = Self(16u32);
    pub const NetworkName: Self = Self(32u32);
    pub const NetworkState: Self = Self(64u32);
    pub const Transport: Self = Self(128u32);
    pub const Voicemail: Self = Self(256u32);
}
impl ::core::marker::Copy for PhoneLineProperties {}
impl ::core::clone::Clone for PhoneLineProperties {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PhoneNewVoicemailMessageTriggerDetails = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
#[repr(transparent)]
pub struct PhoneTriggerType(pub i32);
impl PhoneTriggerType {
    pub const NewVoicemailMessage: Self = Self(0i32);
    pub const CallHistoryChanged: Self = Self(1i32);
    pub const LineChanged: Self = Self(2i32);
    pub const AirplaneModeDisabledForEmergencyCall: Self = Self(3i32);
    pub const CallOriginDataRequest: Self = Self(4i32);
    pub const CallBlocked: Self = Self(5i32);
    pub const IncomingCallDismissed: Self = Self(6i32);
    pub const IncomingCallNotification: Self = Self(7i32);
}
impl ::core::marker::Copy for PhoneTriggerType {}
impl ::core::clone::Clone for PhoneTriggerType {
    fn clone(&self) -> Self {
        *self
    }
}
